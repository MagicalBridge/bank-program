use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("ditw8dH7D93kotkJgokM6WLbJHNdrbK9fJfLR74NJ7h");

#[program]
pub mod solana_bank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing bank contract");
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.owner.key();
        bank.total_balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
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

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
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

    pub fn get_balance(ctx: Context<GetBalance>) -> Result<u64> {
        let balance = ctx.accounts.user_account.balance;
        msg!(
            "Queried balance for user {}: {} lamports",
            ctx.accounts.user.key(),
            balance
        );
        Ok(balance)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
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

#[derive(Accounts)]
pub struct GetBalance<'info> {
    #[account(seeds = [b"user", user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    pub user: Signer<'info>,
}

#[account]
pub struct Bank {
    pub owner: Pubkey,
    pub total_balance: u64,
}

#[account]
pub struct UserAccount {
    pub balance: u64,
}

#[error_code]
pub enum BankError {
    #[msg("Deposit amount must be at least 0.01 SOL")]
    DepositTooSmall,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
}
