import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Todo } from "../target/types/todo";
import { PublicKey } from "@solana/web3.js";

describe("todo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Todo as Program<Todo>;

  it("Create Todo", async () => {
    const user = provider.wallet.publicKey;
    const todoId = new anchor.BN(1);
    
    // Derive PDA
    const [todoPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("todo"),
        user.toBuffer(),
        todoId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .createTodo("Learn Anchor", todoId)
      .accounts({
        todo: todoPda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const todoAccount = await program.account.todo.fetch(todoPda);
    console.log("✅ Created:", todoAccount.title);
    console.log("Done:", todoAccount.isDone);
    console.log("ID:", todoAccount.id.toString());
  });

  it("Update Todo", async () => {
    const user = provider.wallet.publicKey;
    const todoId = new anchor.BN(1);
    
    const [todoPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("todo"),
        user.toBuffer(),
        todoId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .updateTodo("Master Anchor!")
      .accounts({
        todo: todoPda,
        user: user,
      })
      .rpc();

    const todoAccount = await program.account.todo.fetch(todoPda);
    console.log("✏️ Updated:", todoAccount.title);
  });

  it("Mark as Done", async () => {
    const user = provider.wallet.publicKey;
    const todoId = new anchor.BN(1);
    
    const [todoPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("todo"),
        user.toBuffer(),
        todoId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .markTodoAsDone()
      .accounts({
        todo: todoPda,
        user: user,
      })
      .rpc();

    const todoAccount = await program.account.todo.fetch(todoPda);
    console.log("✅ Marked done:", todoAccount.isDone);
  });

  it("Delete Todo", async () => {
    const user = provider.wallet.publicKey;
    const todoId = new anchor.BN(1);
    
    const [todoPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("todo"),
        user.toBuffer(),
        todoId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    await program.methods
      .deleteTodo()
      .accounts({
        todo: todoPda,
        user: user,
      })
      .rpc();

    console.log("🗑️ Todo deleted and account closed!");
    
    // Try to fetch (should fail)
    try {
      await program.account.todo.fetch(todoPda);
      console.log("❌ Should have been deleted!");
    } catch (err) {
      console.log("✅ Account successfully closed!");
    }
  });
});