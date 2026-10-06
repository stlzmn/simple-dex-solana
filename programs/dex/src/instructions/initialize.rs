use crate::state::{AskOrderbook, BidOrderbook};
use anchor_lang::prelude::*;

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = AskOrderbook::INIT_SPACE,
        seeds = [b"ask_counter"],
        bump,
    )]
    pub ask_counter: Account<'info, AskOrderbook>,
    #[account(
        init,
        payer = signer,
        space = BidOrderbook::INIT_SPACE,
        seeds = [b"bid_counter"],
        bump,
    )]
    pub bid_counter: Account<'info, BidOrderbook>,

    pub system_program: Program<'info, System>,
}
