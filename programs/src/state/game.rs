use anchor_lang::prelude::*;

#[account]
pub struct Game {
    id: u64,
    result: u8,
    bump: u8,
    title: String,
}

impl Game {
    pub const LEN:usize = 8 + 8 + 32 + 1 + 1;
}
