import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

describe("counter", () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  
  // Generate a new keypair for the counter account
  const counter = anchor.web3.Keypair.generate();

  it("Initializes the counter", async () => {
    console.log("🚀 Initializing counter...");
    
    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counter])
      .rpc();

    console.log("✅ Initialize transaction:", tx);

    // Fetch the counter account
    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    console.log("📊 Counter value:", counterAccount.count.toString());
    expect(counterAccount.count.toString()).to.equal("0");
  });

  it("Increments the counter", async () => {
    console.log("➕ Incrementing counter...");
    
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    console.log("✅ Increment transaction:", tx);

    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    console.log("📊 Counter value:", counterAccount.count.toString());
    expect(counterAccount.count.toString()).to.equal("1");
  });

  it("Increments again", async () => {
    console.log("➕ Incrementing counter again...");
    
    await program.methods
      .increment()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    console.log("📊 Counter value:", counterAccount.count.toString());
    expect(counterAccount.count.toString()).to.equal("2");
  });

  it("Decrements the counter", async () => {
    console.log("➖ Decrementing counter...");
    
    await program.methods
      .decrement()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    console.log("📊 Counter value:", counterAccount.count.toString());
    expect(counterAccount.count.toString()).to.equal("1");
  });

  it("Resets the counter", async () => {
    console.log("🔄 Resetting counter...");
    
    await program.methods
      .reset()
      .accounts({
        counter: counter.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counter.fetch(
      counter.publicKey
    );

    console.log("📊 Counter value:", counterAccount.count.toString());
    expect(counterAccount.count.toString()).to.equal("0");
  });
});