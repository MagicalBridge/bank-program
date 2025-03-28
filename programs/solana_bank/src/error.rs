use anchor_lang::prelude::*;

#[error_code]
pub enum BankError {
    #[msg("Deposit amount must be at least 0.01 SOL")]
    DepositTooSmall,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
} 