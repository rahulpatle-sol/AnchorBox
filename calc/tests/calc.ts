import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calc } from "../target/types/calc";

describe("calc", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Calc as Program<Calc>;
  const calculator = anchor.web3.Keypair.generate();

  it("Initialize", async () => {
    await program.methods
      .initialize("Hello Calculator!")
      .accounts({
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([calculator])
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    console.log("✅ Initialized!");
    console.log("Greeting:", account.greetings);
    console.log("Result:", account.result.toString());
  });

  it("Add: 15 + 10", async () => {
    await program.methods
      .add(new anchor.BN(15), new anchor.BN(10))
      .accounts({ calculator: calculator.publicKey })
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    console.log("➕ 15 + 10 =", account.result.toString());
  });

  it("Subtract: 50 - 20", async () => {
    await program.methods
      .sub(new anchor.BN(50), new anchor.BN(20))
      .accounts({ calculator: calculator.publicKey })
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    console.log("➖ 50 - 20 =", account.result.toString());
  });

  it("Multiply: 8 * 7", async () => {
    await program.methods
      .mult(new anchor.BN(8), new anchor.BN(7))
      .accounts({ calculator: calculator.publicKey })
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    console.log("✖️ 8 * 7 =", account.result.toString());
  });

  it("Divide: 100 / 4", async () => {
    await program.methods
      .div(new anchor.BN(100), new anchor.BN(4))
      .accounts({ calculator: calculator.publicKey })
      .rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    console.log("➗ 100 / 4 =", account.result.toString());
    console.log("Remainder:", account.remainder.toString());
  });

  it("Divide by zero (should fail)", async () => {
    try {
      await program.methods
        .div(new anchor.BN(10), new anchor.BN(0))
        .accounts({ calculator: calculator.publicKey })
        .rpc();
      
      console.log("❌ Should have failed!");
    } catch (err) {
      console.log("✅ Correctly failed: Cannot divide by zero");
    }
  });
});