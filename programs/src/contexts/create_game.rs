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
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [
            b"vault_lose",
            game.key().as_ref(),
            ID.as_ref(),
            mint.key().as_ref(),
        ]
    )]
    pub vault_lose: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = creator,
        seeds = [
            b"vault_win",
            game.key().as_ref(),
            ID.as_ref(),
            mint.key().as_ref(),
        ]
    )]
    pub vault_win: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = creator,
        seeds = [
            b"vault_tie",
            game.key().as_ref(),
            ID.as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub vault_tie: Account<'info, TokenAccount>,
    pub usdc_mint: Accunt<'info, Mint>,

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
