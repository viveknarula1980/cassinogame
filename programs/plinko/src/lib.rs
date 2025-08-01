use anchor_lang::prelude::*;

declare_id!("PlinkoGame1111111111111111111111111111111111");

#[program]
pub mod plinko {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn drop_ball(ctx: Context<DropBall>) -> Result<()> {
        let clock = Clock::get()?;
        let randomness = (clock.unix_timestamp % 3) as u8;

        msg!("🔻 Dropping the ball...");

        match randomness {
            0 => msg!("🟢 Ball landed in the LEFT bucket – Small reward!"),
            1 => msg!("🟢 Ball landed in the CENTER bucket – Medium reward!"),
            2 => msg!("🟢 Ball landed in the RIGHT bucket – Jackpot!"),
            _ => msg!("❌ Unexpected outcome."),
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8,
        seeds = [b"plinko-state"],
        bump
    )]
    pub state: Account<'info, PlinkoState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DropBall<'info> {
    #[account(
        mut,
        seeds = [b"plinko-state"],
        bump = state.bump
    )]
    pub state: Account<'info, PlinkoState>,
}

#[account]
pub struct PlinkoState {
    pub bump: u8,
}
