use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::*;

#[derive(Accounts)]
#[instruction(is_bid: bool)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(
        init_if_needed, 
        payer = signer,
        space = 8 + OpenOrders::INIT_SPACE,
        seeds = [b"open", market.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub open_orders: Account<'info, OpenOrders>,

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
        mut,
        constraint = user_token_account.owner == signer.key(),
        constraint = user_token_account.mint == if is_bid { market.quote_mint } else { market.base_mint }
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault.key() == if is_bid { market.quote_vault } else { market.base_vault }
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
