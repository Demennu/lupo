use anchor_lang::prelude::*;


// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("JB8tSGwspD1rjBRiE9G6yuoSgwmSnpPSL2XcyLoeegn6");

#[program]
mod hello_anchor {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault_state.score = 0;
        ctx.accounts.vault_state.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.vault_state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.vault_state.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>, title: String, id: u64, rate: u64) -> Result<()> {
        require!(title.len() < 33);
        ctx.accounts.game.id = id;
        ctx.accounts.game.bump = *ctx.bumps.get("game").unwrap();
        ctx.accounts.game.result = 0xff; // 0 = TIE, 1 = WIN, 2 = LOSS, 255 = unresolved
        ctx.accounts.title = title;
        ctx.accounts.game.rate = 0;
        ctx.accounts.game.vault_tie = ctx.accounts.vault.result;
        ctx.accounts.game.vault_win  = ctx.accounts.vault.result;
        ctx.accounts.game.vault_lose  = ctx.accounts.vault.result;
        Ok(())
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>, result: u8) -> Result<()> {
        require!(result < 3);
        ctx.accounts.game.result = result;
        // should move this to an external function that return a vault type depending on the result
        let winning_amount = if result == 0 {
            ctx.accounts.vault_tie.vault_state.score 
        }
        else if result == 1 {
            ctx.accounts.vault_win.vault_state.score 
        }
        else {
            ctx.accounts.vault_lose.vault_state.score 
        };
        // How to grab amount into a vault?
        // Should we use tokens instead of native sol?
        let total_deposited = ctx.accounts.vault_tie.vault_state.score + ctx.accounts.vault_win.vault_state.score + ctx.accounts.vault_lose.vault_state.score;
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

        let vault = if result == 0 {
            ctx.accounts.vault_tie
        }
        else if result == 1 {
            ctx.accounts.vault_win
        }
        else {
            ctx.accounts.vault_lose
        };
        
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SPLTransfer{
                //player_ata????
                from: ctx.accounts.prediction.player_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.prediction.player.to_account_info(),
            },
        );

        anchor_spl::token::transfer(cpi, amount)?;
        // what is this score?
        ctx.accounts.vault_state.score += 1;
        
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, amount: u64) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer=owner, space=Vault::LEN)]
    pub vault_state: Account<'info, Vault>,
    /// CHECK
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub vault_auth : UncheckedAccount<'info>,
    #[account(seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount <'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub vault_bump: u8,
    pub auth_bump: u8,
    pub score: u8,
}

impl Vault {
    const LEN: usize =8 + 32 + 1 + 1 + 1;
}

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
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    id: u64,
    result: u8,
    bump: u8,
    title: String,
}

impl Game {
    pub const LEN:usize = 8 + 8 + 32 + 1 + 1;
}

#[account]
pub struct Prediction {
    player: Pubkey,
    result: u8,
    amount: u64,
    bump: u8
}

impl Prediction {
    pub const LEN:usize = 8 + 32 + 1 + 8 + 1;
}

#[error_code]
pub enum PredictionError {
    #[msg("Overflow")]
    Overflow
}