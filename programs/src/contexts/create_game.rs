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

impl<'info> CreateGame<'info> {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>, seed: u64, amount: u64) -> Result<()> {
        let game = &mut self.game;
        game.creator = *self.creator.key;
        game.creator_token = *self.creator_token.to_account_info().key;
        game.seed = seed;
        game.auth_bump = *bumps.get("auth").unwrap();
        game.vault_bump = *bumps.get("vault").unwrap();
        game.amount = amount;
    }
}
