
use std::{collections::BTreeMap};

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{state::{Game, Prediction, Vault}, errors::PredictionError};

#[derive(Accounts)]
#[instruction(result: u8)]

pub struct Claim<'info> {

    #[account(
        seeds = [
            creator.key().as_ref(),
            game.id.to_le_bytes().as_ref()
        ],
        payer = creator,
        bump = game.bump
        
    )]

    pub game: Account<'info, Game>,
    ///CHECK: This is safe
    pub creator: UncheckedAccount<'info>,

    #[account(associated_token::mint = token_mint, associated_token::authority = creator)]
    pub creator_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        seeds = [
            game.key().as_ref(),
            player.key().as_ref(),
            &[result]
        ],
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
    pub player: Signer<'info>
}

impl<'info> Claim<'info> {
    pub fn transfer_to_player(
        &self,
        amount: u64
    ) -> Result<()> {
        
        let cpi_accounts = Transfer {
            from: self.creator_ata.to_account_info(),
            to: self.prediction.player.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
}