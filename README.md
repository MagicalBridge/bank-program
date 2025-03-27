# Solana Bank Smart Contract

A decentralized banking application built on the Solana blockchain using the Anchor framework. This project implements a secure and efficient banking system that allows users to deposit, withdraw, and check their SOL balances.

## Features

- **Deposit SOL**: Users can deposit SOL tokens into their bank account
- **Withdraw SOL**: Users can withdraw their SOL tokens from their bank account
- **Balance Check**: Users can query their current balance
- **Minimum Deposit**: Enforces a minimum deposit of 0.01 SOL
- **Secure Transactions**: All transactions are secured by Solana's blockchain
- **Program Derived Addresses**: Uses PDAs for secure account management

## Prerequisites

- Rust and Cargo
- Solana CLI
- Anchor Framework
- Node.js and Yarn

## Installation

1. Clone the repository:
```bash
git clone https://github.com/MagicalBridge/bank-program
cd solana_bank
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

## Usage

### Deploy the Program

```bash
anchor deploy
```

### Run Tests

```bash
anchor test
```

## Program Structure

The program consists of the following main components:

- `initialize`: Initializes the bank contract
- `deposit`: Allows users to deposit SOL tokens
- `withdraw`: Allows users to withdraw their SOL tokens
- `get_balance`: Queries the user's current balance

## Account Structure

### Bank Account
- Owner: The public key of the bank owner
- Total Balance: The total amount of SOL held by the bank

### User Account
- Balance: The amount of SOL owned by the user

## Error Handling

The program includes error handling for:
- Insufficient funds for withdrawal
- Deposits below the minimum amount (0.01 SOL)

## Security Features

- Program Derived Addresses (PDAs) for secure account management
- Signer verification for all transactions
- Proper account validation and access control
- Safe arithmetic operations using checked_add and checked_sub

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 