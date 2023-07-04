use anchor_lang::prelude::*;

#[account]
pub struct Prediction {
    player: Pubkey,
    result: u8,
    amount: u64,
    bump: u8
}

impl Prediction {
    pub const LEN:usize = 8 + 32 + 1 + 8 + 1;
}