use anchor_lang::prelude::*;

#[account]
pub struct Bank {
    pub owner: Pubkey,
    pub total_balance: u64,
}

#[account]
pub struct UserAccount {
    pub balance: u64,
} 