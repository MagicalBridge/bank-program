use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"bank"], bump)]
    pub bank: Account<'info, Bank>,
    #[account(mut, seeds = [b"user", user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    msg!(
        "Processing withdrawal of {} lamports for user: {}",
        amount,
        ctx.accounts.user.key()
    );
    require!(user_account.balance >= amount, BankError::InsufficientFunds);

    let old_balance = user_account.balance;
    let old_bank_balance = ctx.accounts.bank.total_balance;

    **ctx
        .accounts
        .bank
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx
        .accounts
        .user
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;

    user_account.balance = user_account.balance.checked_sub(amount).unwrap();

    let bank = &mut ctx.accounts.bank;
    bank.total_balance = bank.total_balance.checked_sub(amount).unwrap();
    msg!(
        "Withdrawal successful. User balance: {} -> {}, Bank total: {} -> {}",
        old_balance,
        user_account.balance,
        old_bank_balance,
        bank.total_balance
    );

    Ok(())
} 