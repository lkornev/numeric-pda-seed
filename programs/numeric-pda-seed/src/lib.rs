use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod numeric_pda_seed {
    use super::*;

    pub fn create(ctx: Context<Create>, seed_key: u8, bump: u8) -> Result<()> {
        let account_numeric_seed = &mut ctx.accounts.account_numeric_seed;
        account_numeric_seed.bump = bump;

        Ok(())
    }

    pub fn access(ctx: Context<Access>, seed_key: u8) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed_key: u8)]
pub struct Create<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 1, // 8 discriminator + 1 bump
        seeds = [&[seed_key]], 
        bump
    )]
    pub account_numeric_seed: Account<'info, AccountNumericSeed>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(seed_key: u8)]
pub struct Access<'info> {
    #[account(
        seeds = [&[seed_key]], 
        bump = account_numeric_seed.bump
    )]
    pub account_numeric_seed: Account<'info, AccountNumericSeed>,
}

#[account]
pub struct AccountNumericSeed {
    bump: u8,
}
