use std::{collections::BTreeMap};

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{state::{Game, Prediction, Vault}, errors::PredictionError};


#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer=owner, space=Vault::LEN)]
    pub vault_state: Account<'info, Vault>,
    /// CHECK
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub vault_auth : UncheckedAccount<'info>,
    #[account(seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount <'info>,
    pub system_program: Program<'info, System>,
}