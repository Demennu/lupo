use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub vault_bump: u8,
    pub auth_bump: u8,
    pub amount: u64,
    pub score: u8,
}

impl Vault {
    const LEN: usize =8 + 32 + 1 + 1 + 1;
}