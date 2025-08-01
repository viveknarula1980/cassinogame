use anchor_lang::prelude::*;

declare_id!("HrYFHpYUqqEbVvgvPWQdbBnUhDYUgtWykbfTbdqYimuL");    // program id for deploy

#[program]
pub mod mines {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn reveal_tile(ctx: Context<RevealTile>) -> Result<()> {
        let clock = Clock::get()?;
        let is_mine = clock.unix_timestamp % 5 == 0;

        msg!("🟫 You clicked a tile...");

        if is_mine {
            msg!("💥 Boom! You hit a mine. Game over.");
        } else {
            msg!("✅ Safe! You can keep playing.");
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
        seeds = [b"mines-state"],
        bump
    )]
    pub state: Account<'info, MinesState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevealTile<'info> {
    #[account(
        mut,
        seeds = [b"mines-state"],
        bump = state.bump
    )]
    pub state: Account<'info, MinesState>,
}

#[account]
pub struct MinesState {
    pub bump: u8,
}
