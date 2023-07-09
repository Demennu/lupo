use anchor_lang::prelude::*;

#[account]
pub struct Global {
    dao_vault: Pubkey,
}

impl Global {
    pub const LEN: usize = 8 + 32;
}
