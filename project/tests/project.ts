import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Project } from "../target/types/project";
import { Keypair } from "@solana/web3.js";

describe("project", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 
  const program = anchor.workspace.Project as Program<Project>;
 
  // Generate a new keypair to use as the address the project account
  const projectAccount = new Keypair();
 
  it("Is initialized!", async () => {
    // Invoke the initialize instruction
    const transactionSignature = await program.methods
      .initialize()
      .accounts({
        project: projectAccount.publicKey,
      })
      .signers([projectAccount]) // include project keypair as additional signer
      .rpc({ skipPreflight: true });
 
    // Fetch the project account data
    const accountData = await program.account.project.fetch(
      projectAccount.publicKey,
    );
 
    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
 
  it("Increment", async () => {
    // Invoke the increment instruction
    const transactionSignature = await program.methods
      .increment()
      .accounts({
        project: projectAccount.publicKey,
      })
      .rpc();
 
    // Fetch the project account data
    const accountData = await program.account.project.fetch(
      projectAccount.publicKey,
    );
 
    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});