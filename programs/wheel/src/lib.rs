use anchor_lang::prelude::*;

declare_id!("WheelSpinGame11111111111111111111111111111111");

#[program]
pub mod wheel {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn spin_wheel(ctx: Context<Spin>) -> Result<()> {
        let clock = Clock::get()?;
        let outcome = (clock.unix_timestamp % 8) as u8; // 8 segments: 0–7

        msg!("🌀 Wheel spun! Landed on segment: {}", outcome);

        // Optional: Reward logic or UI mapping for outcome
        match outcome {
            0 => msg!("🎉 Jackpot!"),
            1..=2 => msg!("🥈 Second prize!"),
            3..=5 => msg!("🥉 Small prize!"),
            _ => msg!("💤 No prize, try again!"),
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
        seeds = [b"wheel-state"],
        bump
    )]
    pub state: Account<'info, WheelState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spin<'info> {
    #[account(
        mut,
        seeds = [b"wheel-state"],
        bump = state.bump
    )]
    pub state: Account<'info, WheelState>,
}

#[account]
pub struct WheelState {
    pub bump: u8,
}
