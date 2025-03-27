import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaBank } from "../target/types/solana_bank";
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe("solana_bank", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaBank as Program<SolanaBank>;
  const provider = anchor.getProvider();

  // 生成银行PDA地址
  const [bankPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("bank")],
    program.programId
  );

  it("Initialize bank", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        bank: bankPDA,
        owner: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    console.log("Bank initialized with tx:", tx);

    // 验证初始化结果
    const bankAccount = await program.account.bank.fetch(bankPDA);
    expect(bankAccount.owner.toString()).to.equal(provider.publicKey.toString());
    expect(bankAccount.totalBalance.toString()).to.equal("0");
  });


  it("Deposit funds", async () => {
    // 生成用户账户PDA
    const [userAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), provider.publicKey.toBuffer()],
      program.programId
    );

    const depositAmount = new anchor.BN(1_000_000_000); // 1 SOL
    
    const tx = await program.methods
      .deposit(depositAmount)
      .accounts({
        bank: bankPDA,
        userAccount: userAccountPDA,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit tx:", tx);

    // 验证存款结果
    const userAccount = await program.account.userAccount.fetch(userAccountPDA);
    expect(userAccount.balance.toString()).to.equal(depositAmount.toString());
  });

  it("Get balance", async () => {
    const [userAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), provider.publicKey.toBuffer()],
      program.programId
    );

    const balance = await program.methods
      .getBalance()
      .accounts({
        userAccount: userAccountPDA,
        user: provider.publicKey,
      })
      .view();

    console.log("User balance:", balance.toString());
    // 验证余额结果
    expect(balance.toString()).to.equal("1000000000");
  });

  it("Withdraw funds", async () => {
    const [userAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), provider.publicKey.toBuffer()],
      program.programId
    );

    const withdrawAmount = new anchor.BN(500_000_000); // 0.5 SOL

    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        bank: bankPDA,
        userAccount: userAccountPDA,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw tx:", tx);

    // 验证取款结果
    const userAccount = await program.account.userAccount.fetch(userAccountPDA);
    expect(userAccount.balance.toString()).to.equal("500000000");
  });

  it("Should fail with insufficient funds", async () => {
    const [userAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), provider.publicKey.toBuffer()],
      program.programId
    );

    const withdrawAmount = new anchor.BN(1_000_000_000); // 尝试取出1 SOL（超过余额）

    try {
      await program.methods
        .withdraw(withdrawAmount)
        .accounts({
          bank: bankPDA,
          userAccount: userAccountPDA,
          user: provider.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      expect.fail("应该失败但没有失败");
    } catch (error) {
      expect(error.toString()).to.include("Insufficient funds for withdrawal");
    }
  });
});
