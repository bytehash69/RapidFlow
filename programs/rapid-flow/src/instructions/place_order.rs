#![allow(warnings)]
use std::env::var;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::ErrorCode, *};

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market", market.base_mint.key().as_ref(), market.quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"bids", market.key().as_ref()],
        bump
    )]
    pub bids: Account<'info, OrderBook>,

    #[account(
        mut,
        seeds = [b"asks", market.key().as_ref()],
        bump
    )]
    pub asks: Account<'info, OrderBook>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + OpenOrders::INIT_SPACE,
        seeds = [b"user_open_orders", market.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub user_open_orders: Account<'info, OpenOrders>,

    #[account(
        mut,
        associated_token::mint = market.base_mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = market.quote_mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = market.base_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_base_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = market.quote_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_quote_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> PlaceOrder<'info> {
    pub fn place_order(&mut self, is_bid: bool, price: u64, size: u64) -> Result<()> {
        let clock = Clock::get()?;

        let amount = if is_bid { price * size } else { size };

        if self.user_open_orders.owner == Pubkey::default() {
            self.user_open_orders.owner = self.signer.key();
            self.user_open_orders.market = self.market.key();
            self.user_open_orders.base_free = 0;
            self.user_open_orders.base_locked = 0;
            self.user_open_orders.quote_free = 0;
            self.user_open_orders.quote_locked = 0;
        }

        let cpi_ctx = if is_bid {
            let cpi_accounts = Transfer {
                authority: self.signer.to_account_info(),
                from: self.user_quote_vault.to_account_info(),
                to: self.quote_vault.to_account_info(),
            };
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
        } else {
            let cpi_accounts = Transfer {
                authority: self.signer.to_account_info(),
                from: self.user_base_vault.to_account_info(),
                to: self.base_vault.to_account_info(),
            };
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
        };

        if is_bid {
            self.user_open_orders.quote_locked = self
                .user_open_orders
                .quote_locked
                .checked_add(amount)
                .ok_or(ErrorCode::MathOverflow)?;
        } else {
            self.user_open_orders.base_locked = self
                .user_open_orders
                .base_locked
                .checked_add(amount)
                .ok_or(ErrorCode::MathOverflow)?;
        }

        // Fixed matching logic for bid orders (buying)
        if is_bid {
            let asks = &mut self.asks;
            let mut i = 0;

            while i < asks.orders.len() && size > 0 {
                let ask_order = &asks.orders[i];

                if price >= ask_order.price && size == ask_order.size {
                    self.user_open_orders.quote_locked = self
                        .user_open_orders
                        .quote_locked
                        .checked_sub(price * size)
                        .ok_or(ErrorCode::InsufficientFunds)?;

                    self.user_open_orders.base_free = self
                        .user_open_orders
                        .base_free
                        .checked_add(size)
                        .ok_or(ErrorCode::MathOverflow)?;

                    asks.orders.remove(i);
                    break; // Order fully matched, exit loop
                } else {
                    i += 1; // Move to next order
                }
            }

            // If no match found, add bid to order book
            if size > 0 {
                self.bids.orders.push(Order {
                    order_id: clock.unix_timestamp as u128,
                    owner: self.signer.key(),
                    price,
                    size,
                    timestamp: clock.unix_timestamp,
                });
            }
        } else {
            // Fixed matching logic for ask orders (selling)
            let bids = &mut self.bids;
            let mut i = 0;

            while i < bids.orders.len() && size > 0 {
                let bid_order = &bids.orders[i];

                if price <= bid_order.price && size == bid_order.size {
                    self.user_open_orders.base_locked = self
                        .user_open_orders
                        .base_locked
                        .checked_sub(size)
                        .ok_or(ErrorCode::InsufficientFunds)?;

                    self.user_open_orders.quote_free = self
                        .user_open_orders
                        .quote_free
                        .checked_add(price * size)
                        .ok_or(ErrorCode::MathOverflow)?;

                    bids.orders.remove(i);
                    break;
                } else {
                    i += 1;
                }
            }

            // If no match found, add ask to order book
            if size > 0 {
                self.asks.orders.push(Order {
                    order_id: clock.unix_timestamp as u128,
                    owner: self.signer.key(),
                    price,
                    size,
                    timestamp: clock.unix_timestamp,
                });
            }
        }

        transfer(cpi_ctx, amount);

        Ok(())
    }
}
