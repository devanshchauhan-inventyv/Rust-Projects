import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CheckingAccountsAnchor } from "../target/types/checking_accounts_anchor";
import { it } from "mocha";

describe("Checking Accounts Validation in Anchor ", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .checkingAccountsAnchor as Program<CheckingAccountsAnchor>;
  const provider = anchor.getProvider();
  const wallet = provider.wallet as anchor.Wallet;
  const accountToCreate = anchor.web3.Keypair.generate();
  const accountToChange = anchor.web3.Keypair.generate();

  it("Create a account owned by our program", async () => {
    const instruction = await anchor.web3.SystemProgram.createAccount({
      fromPubkey: wallet.payer.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: anchor.web3.LAMPORTS_PER_SOL,
      programId: program.programId,
      space: 0,
    });

    const transaction = new anchor.web3.Transaction().add(instruction);

    await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      transaction,
      [wallet.payer, accountToChange]
    );
  });

  it("Checking accounts", async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer: wallet.publicKey,
        accountsToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
      })
      .signers([accountToCreate])
      .rpc();
  });
});
