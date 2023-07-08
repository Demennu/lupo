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
    #[account(
        mut,
        associated_token::mint = creator_token,
        associated_token::authority = creator
    )]

    pub creator_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeGame<'info> {
    pub fn transfer_to_vault_tie(
        &self,
        amount: u64
    ) -> Result<()> {

        let cpi_accounts = Transfer {
            from: self.creator_ata.to_account_info(),
            to: self.accounts.game.vault_win.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }

    pub fn transfer_to_vault_lose(
        &self,
        amount: u64
    ) -> Result<()> {

        let cpi_accounts = Transfer {
            from: self.creator_ata.to_account_info(),
            to: self.accounts.game.vault_win.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }

    /*
    pub fn transfer_to_vault_DAO(
        &self,
        amount: u64
    ) -> Result<()> {

        let cpi_accounts = Transfer {
            from: self.creator_ata.to_account_info(),
            //vault DAO???
            to: self.accounts.game.vault_lose.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
     */
}