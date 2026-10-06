use crate::constants::{ASK_ORDERBOOK_SEED, ASK_SEED};
use crate::error::ErrorCode;
use crate::state::AskOffer;
use crate::AskOrderbook;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

pub fn handle_ask(ctx: Context<Ask>, quantity: u64, amount: u64) -> Result<()> {
    ctx.accounts.ask_offer.who = ctx.accounts.signer.key();
    ctx.accounts.ask_offer.what = ctx.accounts.ask_mint.key();
    ctx.accounts.ask_offer.for_what = ctx.accounts.for_mint.key();
    ctx.accounts.ask_offer.quantity = quantity;
    ctx.accounts.ask_offer.amount = amount;

    ctx.accounts
        .ask_orderbook
        .id
        .checked_add(1)
        .ok_or(ErrorCode::AskOrderbookOverflow)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Ask<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub ask_mint: InterfaceAccount<'info, Mint>,
    pub for_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = AskOffer::INIT_SPACE,
        seeds = [ASK_SEED, ask_mint.key().as_ref(), for_mint.key().as_ref(), ask_orderbook.id.to_le_bytes().as_ref()],
        bump
    )]
    pub ask_offer: Account<'info, AskOffer>,

    #[account(
        mut,
        seeds = [ASK_ORDERBOOK_SEED],
        bump
    )]
    pub ask_orderbook: Account<'info, AskOrderbook>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
