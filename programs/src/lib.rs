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
        unimplemented!()
    }

    pub fn create_game(ctx: Context<CreateGame>, title: String, seed: u64, id: u64) -> Result<()> {
        require!(title.len() < 33);
        ctx.accounts.game.id = id;
        ctx.accounts.game.bump = *ctx.bumps.get("game").unwrap();
        ctx.accounts.game.result = 0xff; // 0 = TIE, 1 = WIN, 2 = LOSS, 255 = unresolved
        ctx.accounts.title = title;
        ctx.accounts.game.rate = 0;

        ctx.accounts.game.vault_lose = ctx.accounts.init(&ctx.bumps, seed)?;
        ctx.accounts.game.vault_win = ctx.accounts.init(&ctx.bumps, seed)?;
        ctx.accounts.game.vault_tie = ctx.accounts.init(&ctx.bumps, seed)?;
        Ok(())
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>, result: u8) -> Result<()> {
        require!(result < 3);
        //require signer to be the DAO contract
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

        let amount_tie = ctx.accounts.game.vault_tie.amount;
        let amount_lose = ctx.accounts.game.vault_lose.amount;

        ctx.accounts.transfer_to_vault_tie(amount_tie)
        ctx.accounts.transfer_to_vault_lose(amount_lose)

        // Should we use tokens instead of native sol?
        let total_deposited = ctx.accounts.game.vault_win.amount;
        let fee = total_deposited * 0.03; // TODO how to use decimal into solana?
        ctx.accounts.game.rate = total_deposited - fee / winning_amount;

        // transfer token to DAO
        ctx.accounts.transfer_to_vault_DAO(fee)

        Ok(())
    }

    pub fn make_prediction(ctx: Context<MakePrediction>, amount: u64) {
        require!(*ctx.accounts.game.result == 0xff);
        let current_bet = *ctx.accounts.prediction.amount;
        ctx.accounts.prediction.amount = current_bet.checked_add(amount).ok_or(PredictionError::Overflow)?;
        ctx.accounts.prediction.result = result;
        ctx.accounts.prediction.bump = *ctx.bumps.get("prediction").unwrap();
        ctx.accounts.prediction.player = *ctx.accounts.player.key;

        ctx.accounts.transfer_to_vault(amount)
        Ok(())
    }

    pub fn claim(ctx: Contect<Claim>) -> Result<()> {

        require!(*ctx.accounts.prediction.result == ctx.accounts.game.result);
        // check if user already claimed
        let winning_amount = ctx.accounts.game.rate * ctx.accounts.prediction.amount;

        // update variable saying that the user already claimed

        ctx.accounts.prediction.player.transfer_to_player(winning_amount)


        Ok(())
    }
}
