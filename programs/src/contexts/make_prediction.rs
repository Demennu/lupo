
use std::{collections::BTreeMap};

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{state::{Game, Prediction, Vault}, errors::PredictionError};

#[derive(Accounts)]
#[instruction(result: u8)]

pub struct MakePrediction<'info> {

    #[account(
        seeds = [
            creator.key().as_ref(),
            game.id.to_le_bytes().as_ref()
        ],
        bump = game.bump
    )]

    pub game: Account<'info, Game>,
    ///CHECK: This is safe
    pub creator: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        seeds = [
            game.key().as_ref(),
            player.key().as_ref(),
            &[result]
        ],
        payer = player, 
        bump, 
        space = Prediction::LEN
    )]

    pub prediction: Account<'info, Prediction>,
    #[account(
        seeds = [
            game.key().as_ref(),
            &[result]
        ],
        bump
    )]

    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut, associated_token::mint = token_mint, associated_token::authority = player)]
    pub player_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> MakePrediction<'info> {
    pub fn transfer_to_vault(
        &self,
        amount: u64,
        result: u8
    ) -> Result<()> {

        prediction.result = result;
        prediction.amount = amount;

        let vault = if result == 0 {
            game.vault_tie
        }
        else if result == 1 {
            game.vault_win
        }
        else {
            game.vault_lose
        };
        
        let cpi_accounts = Transfer {
            from: self.player_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.player.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
}