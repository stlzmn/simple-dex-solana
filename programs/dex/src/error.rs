use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Too much ask offerts")]
    AskOrderbookOverflow,
    #[msg("Too much bid offerts")]
    BidOrderbookOverflow,
}
