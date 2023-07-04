use std::{collections::BTreeMap};

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{state::{Game, Prediction, Vault}, errors::PredictionError};

#[derive(Accounts)]
pub struct FinalizeGame<'info> {
    #[account(
        seeds = [
            creator.key().as_ref(), 
            game.id.to_le_bytes().as_ref()
        ],
        bump = game.bump
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}