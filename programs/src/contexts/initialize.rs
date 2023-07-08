use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::PredictionError,
    state::{Game, Prediction, Vault},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer=owner, space=Vault::LEN)]
    pub vault_state: Account<'info, Vault>,
    /// CHECK
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub vault_auth: UncheckedAccount<'info>,
    #[account(init, seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}
