use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8,
        seeds = [b"bank"],
        bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Initializing bank contract");
    let bank = &mut ctx.accounts.bank;
    bank.owner = ctx.accounts.owner.key();
    bank.total_balance = 0;
    Ok(())
} 