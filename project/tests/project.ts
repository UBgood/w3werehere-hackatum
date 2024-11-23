import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Project } from "../target/types/project";
import { PublicKey } from "@solana/web3.js";

describe("project", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 
  const program = anchor.workspace.Project as Program<Project>;
 
  // Generate a new keypair to use as the address the project account
  //const projectAccount = new Keypair();

  const [projectPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("project")],
    program.programId,
  );
 
  // it("Is initialized!", async () => {
  //   // Invoke the initialize instruction
  //   const transactionSignature = await program.methods
  //     .initialize()
  //     .accounts({
  //       project: projectAccount.publicKey,
  //     })
  //     .signers([projectAccount]) // include project keypair as additional signer
  //     .rpc({ skipPreflight: true });
 
  //   // Fetch the project account data
  //   const accountData = await program.account.project.fetch(
  //     projectAccount.publicKey,
  //   );
 
  //   console.log(`Transaction Signature: ${transactionSignature}`);
  //   console.log(`Count: ${accountData.count}`);
  // });

  it("Is initialized!", async () => {
    try {
      const txSig = await program.methods
        .initialize()
        .accounts({
          project: projectPDAPDA,
        })
        .rpc();
 
      const accountData = await program.account.project.fetch(projectPDA);
      console.log(`Transaction Signature: ${txSig}`);
      console.log(`Count: ${accountData.count}`);
    } catch (error) {
      // If PDA Account already created, then we expect an error
      console.log(error);
    }
  });
 
  it("Increment", async () => {
    // Invoke the increment instruction
    const transactionSignature = await program.methods
      .increment()
      .accounts({
        project: projectPDA,
      })
      .rpc();
 
    // Fetch the project account data
    const accountData = await program.account.project.fetch(
      projectPDA
    );
 
    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});