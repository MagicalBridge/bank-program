use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"bank"], bump)]
    pub bank: Account<'info, Bank>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    const MIN_DEPOSIT: u64 = 10_000_000; // 0.01 SOL
    msg!(
        "Processing deposit of {} lamports from user: {}",
        amount,
        ctx.accounts.user.key()
    );
    require!(amount >= MIN_DEPOSIT, BankError::DepositTooSmall);

    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.bank.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.bank.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let user_account = &mut ctx.accounts.user_account;
    let old_balance = user_account.balance;
    user_account.balance = user_account.balance.checked_add(amount).unwrap();

    let bank = &mut ctx.accounts.bank;
    bank.total_balance = bank.total_balance.checked_add(amount).unwrap();
    msg!(
        "Deposit successful. User balance: {} -> {}, Bank total: {}",
        old_balance,
        user_account.balance,
        bank.total_balance
    );

    Ok(())
} 