use anchor_lang::prelude::*;

declare_id!("C6AC67E3kLs1iK7Wj7G7AzVFEkjyf14uXuBFFt6V49mp");

#[program]
pub mod tally {
    use super::*;

    /// Creates a tally owned by `authority` that never counts past `limit`.
    pub fn initialize(ctx: Context<Initialize>, limit: u64) -> Result<()> {
        let tally = &mut ctx.accounts.tally;
        tally.authority = ctx.accounts.authority.key();
        tally.count = 0;
        tally.limit = limit;
        msg!("tally created with limit {}", limit);
        Ok(())
    }

    /// Adds `amount` and returns the new count.
    pub fn increment(ctx: Context<Update>, amount: u64) -> Result<u64> {
        let tally = &mut ctx.accounts.tally;
        let count = tally
            .count
            .checked_add(amount)
            .ok_or(TallyError::Overflow)?;
        require!(count <= tally.limit, TallyError::LimitExceeded);
        tally.count = count;
        msg!("count is now {}", count);
        Ok(count)
    }

    /// Sets the count back to zero.
    pub fn reset(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.tally.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + Tally::INIT_SPACE)]
    pub tally: Account<'info, Tally>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority)]
    pub tally: Account<'info, Tally>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Tally {
    pub authority: Pubkey,
    pub count: u64,
    pub limit: u64,
}

#[error_code]
pub enum TallyError {
    #[msg("The tally would pass its limit")]
    LimitExceeded,
    #[msg("The count overflowed")]
    Overflow,
}
