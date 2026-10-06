use crate::error::ErrorCode;
use crate::state::AskOffer;
use crate::{BidOffer, BidOrderbook, BID_ORDERBOOK_SEED, BID_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

pub fn handle_bid(ctx: Context<Bid>, quantity: u64, amount: u64) -> Result<()> {
    ctx.accounts.bid_offer.who = ctx.accounts.signer.key();
    ctx.accounts.bid_offer.what = ctx.accounts.bid_mint.key();
    ctx.accounts.bid_offer.for_what = ctx.accounts.for_mint.key();
    ctx.accounts.bid_offer.quantity = quantity;
    ctx.accounts.bid_offer.amount = amount;

    ctx.accounts
        .bid_orderbook
        .id
        .checked_add(1)
        .ok_or(ErrorCode::AskOrderbookOverflow)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub bid_mint: InterfaceAccount<'info, Mint>,
    pub for_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = AskOffer::INIT_SPACE,
        seeds = [BID_SEED, bid_mint.key().as_ref(), for_mint.key().as_ref(), bid_orderbook.id.to_le_bytes().as_ref()],
        bump
    )]
    pub bid_offer: Account<'info, BidOffer>,

    #[account(
        mut,
        seeds = [BID_ORDERBOOK_SEED],
        bump
    )]
    pub bid_orderbook: Account<'info, BidOrderbook>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
