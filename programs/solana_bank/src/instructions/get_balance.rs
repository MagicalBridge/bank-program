use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct GetBalance<'info> {
    #[account(seeds = [b"user", user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<GetBalance>) -> Result<u64> {
    let balance = ctx.accounts.user_account.balance;
    msg!(
        "Queried balance for user {}: {} lamports",
        ctx.accounts.user.key(),
        balance
    );
    Ok(balance)
} 