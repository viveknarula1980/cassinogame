use anchor_lang::prelude::*;

declare_id!("SLoTpRoj3c7j41ZPrPZhxYvjDUDU9ZDkGdMFm3Meme");

#[program]
pub mod memeslot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn spin(ctx: Context<Spin>, bet_amount: u64) -> Result<()> {
        require!(bet_amount > 0, ErrorCode::InvalidBet);

        let random_seed = Clock::get()?.unix_timestamp;
        let symbol_result = (random_seed % 5) as u8; // 0 to 4 symbols

        msg!("You spun a {:?}", symbol_result);

        if symbol_result == 3 {
            msg!("You won with matching symbols! 🎉");
        } else {
            msg!("Try again! 😢");
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
        seeds = [b"memeslot-state"],
        bump
    )]
    pub state: Account<'info, SlotState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Spin<'info> {
    #[account(
        mut,
        seeds = [b"memeslot-state"],
        bump = state.bump
    )]
    pub state: Account<'info, SlotState>,
}

#[account]
pub struct SlotState {
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Bet amount must be greater than 0.")]
    InvalidBet,
}
