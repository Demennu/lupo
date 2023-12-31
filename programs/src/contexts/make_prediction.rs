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
    pub vault: SystemAccount<'info>
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub player_ata: Account<'info, TokenAccount>,
}


impl<'info> MakePrediction<'info> {
    pub fn transfer_to_vault(
        &self,
        amount: u64
    ) -> Result<()> {

        let vault = if prediction.result == 0 {
            vault_tie
        }
        else if prediction.result == 1 {
            vault_win
        }
        else {
            vault_lose
        };

        let cpi_accounts = Transfer {
            from: self.creator_ata.to_account_info(),
            to: self.accounts.game.vault.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
}