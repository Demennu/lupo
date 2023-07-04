use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint, Transfer as SPLTransfer}, associated_token::AssociatedToken};


// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("JB8tSGwspD1rjBRiE9G6yuoSgwmSnpPSL2XcyLoeegn6");

mod errors;
mod state;
mod contexts;

use contexts::*;
use state::*;
use errors::*;

#[program]
mod hello_anchor {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault.score = 0;
        ctx.accounts.vault.amount = 0;
        ctx.accounts.vault.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.vault.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.vault.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>, title: String, id: u64, rate: u64) -> Result<()> {
        require!(title.len() < 33);
        ctx.accounts.game.id = id;
        ctx.accounts.game.bump = *ctx.bumps.get("game").unwrap();
        ctx.accounts.game.result = 0xff; // 0 = TIE, 1 = WIN, 2 = LOSS, 255 = unresolved
        ctx.accounts.title = title;
        ctx.accounts.game.rate = 0;

        // inizialize vault
        ctx.accounts.game.vault_tie = ctx.accounts.vault;
        ctx.accounts.game.vault_win  = ctx.accounts.vault;
        ctx.accounts.game.vault_lose  = ctx.accounts.vault;
        Ok(())
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>, result: u8) -> Result<()> {
        require!(result < 3);
        ctx.accounts.game.result = result;
        // should move this to an external function that return a vault type depending on the result
        let winning_amount = if result == 0 {
            ctx.accounts.game.vault_tie.amount 
        }
        else if result == 1 {
            ctx.accounts.game.vault_win.amount
        }
        else {
            ctx.accounts.game.vault_lose.amount
        };
        // How to grab amount into a vault?
        // Should we use tokens instead of native sol?
        let total_deposited = ctx.accounts.game.vault_tie.amount + ctx.accounts.game.vault_win.amount + ctx.accounts.game.vault_lose.amount;
        let fee = total_deposited * 0.03; // TODO how to use decimal into solana?
        ctx.accounts.game.rate = total_deposited - fee / winning_amount;

        Ok(())
    }

    pub fn make_prediction(ctx: Context<MakePrediction>, result: u8, amount: u64) {
        require!(*ctx.accounts.game.result == 0xff);
        let current_bet = *ctx.accounts.prediction.amount;
        ctx.accounts.prediction.amount = current_bet.checked_add(amount).ok_or(PredictionError::Overflow)?;
        ctx.accounts.prediction.result = result;
        ctx.accounts.prediction.bump = *ctx.bumps.get("prediction").unwrap();
        ctx.accounts.prediction.player = *ctx.accounts.player.key;

        ctx.accounts.prediction.player.transfer_to_vault(amount)
        Ok(())
    }
/* 

    pub fn claim(ctx: Context<Claim>, result: u8) -> Result<()> {

        require!(*ctx.accounts.game.result == result);
        let total_claim = ctx.accounts.prediction.amount * ctx.accounts.game.rate;
        ctx.accounts.prediction.player.transfer_to_vault(total_claim);

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, result: u8) -> Result<()> {
        require!(*ctx.accounts.game.result == result);
        ctx.accounts.deposit_to_player()?;
        ctx.accounts.empty_vault_to_taker()?;
        ctx.accounts.close_vault()
    }
    
    */
}