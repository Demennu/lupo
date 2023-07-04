use std::{collections::BTreeMap};

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{state::{Game, Prediction, Vault}, errors::PredictionError};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateGame<'info> {
    #[account(
        init, 
        seeds = [
            creator.key().as_ref(), 
            id.to_le_bytes().as_ref()
        ], 
        payer = creator, 
        bump, 
        space = Game::LEN
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
