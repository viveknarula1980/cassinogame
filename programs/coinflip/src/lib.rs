use anchor_lang::prelude::*;

declare_id!("Gq7ehciAVAWTqP9kgVkWejeokxkfqdyQenirboYX3tJs");

#[program]
pub mod coinflip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn flip_coin(ctx: Context<Flip>, guess: bool) -> Result<()> {
        let result = Clock::get()?.unix_timestamp % 2 == 0;
        if result == guess {
            msg!("You guessed right! 🎉");
        } else {
            msg!("Wrong guess. 😢");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8, // 8 bytes for discriminator + 8 for GameState
        seeds = [b"state"],
        bump
    )]
    pub state: Account<'info, GameState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, GameState>,
}

#[account]
pub struct GameState {
    pub bump: u8,
}