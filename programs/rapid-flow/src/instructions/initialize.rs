use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::*;

pub const MAX_ORDERS: usize = 128;
pub const ORDER_BOOK_SPACE: usize = 8 + 32 + 1 + 4 + (MAX_ORDERS * 72);

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Market::INIT_SPACE,
        seeds = [b"market", base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = signer,
        space = ORDER_BOOK_SPACE,
        seeds = [b"bids", market.key().as_ref()],
        bump
    )]
    pub bids: Account<'info, OrderBook>,

    #[account(
        init,
        payer = signer,
        space = ORDER_BOOK_SPACE,
        seeds = [b"asks", market.key().as_ref()],
        bump
    )]
    pub asks: Account<'info, OrderBook>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = base_mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub base_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = quote_mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub quote_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
