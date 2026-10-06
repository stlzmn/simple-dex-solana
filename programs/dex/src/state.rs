use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct BidOffer {
    pub who: Pubkey,
    pub what: Pubkey,
    pub for_what: Pubkey,
    pub amount: u64,
    pub quantity: u64,
}

#[account]
#[derive(InitSpace)]
pub struct AskOffer {
    pub who: Pubkey,
    pub what: Pubkey,
    pub for_what: Pubkey,
    pub amount: u64,
    pub quantity: u64,
}

#[account]
#[derive(InitSpace)]
pub struct AskOrderbook {
    pub id: u64,
}

#[account]
#[derive(InitSpace)]
pub struct BidOrderbook {
    pub id: u64,
}
