pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("62DpUrFdPDYigC4PJ2iU34Ce6NqVzasrd33nTLc8Wmjj");

#[program]
pub mod dex {
    use super::*;
}
