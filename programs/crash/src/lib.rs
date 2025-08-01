use anchor_lang::prelude::*;

declare_id!("HrYFHpYUqqEbVvgvPWQdbBnUhDYUgtWykbfTbdqYimuL");   // program id for deploy

#[program]
pub mod crash {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn play_crash(ctx: Context<PlayCrash>, target_multiplier: u64) -> Result<()> {
        require!(target_multiplier >= 100, CrashError::InvalidMultiplier); // At least 1.00x

        let clock = Clock::get()?;
        let simulated_crash_point = ((clock.unix_timestamp % 500) + 100) as u64; // Simulate 1.00x to 6.00x

        msg!("🎯 You targeted: {}x", target_multiplier as f64 / 100.0);
        msg!("🚀 Crash point: {}x", simulated_crash_point as f64 / 100.0);

        if target_multiplier <= simulated_crash_point {
            msg!("✅ You won! You cashed out before the crash.");
        } else {
            msg!("💥 Crashed before your cashout. You lost.");
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
        seeds = [b"crash-state"],
        bump
    )]
    pub state: Account<'info, CrashState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlayCrash<'info> {
    #[account(
        mut,
        seeds = [b"crash-state"],
        bump = state.bump
    )]
    pub state: Account<'info, CrashState>,
}

#[account]
pub struct CrashState {
    pub bump: u8,
}

#[error_code]
pub enum CrashError {
    #[msg("Multiplier must be at least 1.00x (100)")]
    InvalidMultiplier,
}
