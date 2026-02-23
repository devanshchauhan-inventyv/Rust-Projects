import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCounterProgram } from "../target/types/anchor_counter_program";

describe("anchor_counter_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorCounterProgram as Program<AnchorCounterProgram>;

  it("Is Counter program initialized!", async () => {
    const newAccountKp = new anchor.web3.Keypair();
    const tx = await program.methods.initializeCounter(new anchor.BN(40)).accounts({
      counter: newAccountKp.publicKey,
      signer: anchor.getProvider().publicKey,
    }).signers([newAccountKp]).rpc({
      commitment: 'confirmed'
    });
    console.log("Your transaction signature", tx);

    const account = await program.account.myCounter.fetch(newAccountKp.publicKey);
    console.log("Counter data:", account.data.toString());
  });

  it("Is Counter incremented!", async () => {
    const newAccountKp = new anchor.web3.Keypair();
    await program.methods.initializeCounter(new anchor.BN(40)).accounts({
      counter: newAccountKp.publicKey,
      signer: anchor.getProvider().publicKey,
    }).signers([newAccountKp]).rpc({
      commitment: 'confirmed'
    });

    await program.methods.incrementCounter().accounts({
      counter: newAccountKp.publicKey,
    }).rpc({
      commitment: 'confirmed'
    });

    const account = await program.account.myCounter.fetch(newAccountKp.publicKey);
    console.log("Counter data:", account.data.toString());
  });
});
