import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCloseAccountProgram } from "../target/types/anchor_close_account_program";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { assert } from "chai";
import { it } from "mocha";

describe("Anchor :Close an account program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorCloseAccountProgram as Program<AnchorCloseAccountProgram>;

  const wallet = anchor.getProvider().wallet as anchor.Wallet;

  const seeds =[Buffer.from("USER"),wallet.publicKey.toBuffer()];

  const [pdaDerivedPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(seeds,program.programId);

  it("Create an Account", async () => {
    // Add your test here.
    const userDetails = { name: "Devansh Chauhan", id: 7 };
    const tx = await program.methods.createUser(userDetails).accounts({
      payer:wallet.publicKey,
      
    }).rpc();
    console.log("Your transaction signature", tx);


    const userAccount = await program.account.userState.fetch(pdaDerivedPublicKey);

    console.log(`Fetched the newly created user account ${JSON.stringify(userAccount)}`);

    assert.equal(userAccount.name,userDetails.name,"Name did not match");
    assert.equal(userAccount.payerKey.toBase58(),wallet.publicKey.toBase58(),`Payer did not match with ${wallet.publicKey}`);
    assert.equal(userAccount.id,userDetails.id,"ID did not match");


  });


  it("Close an account", async () => {
    const closeTx = await program.methods.closeUser().accounts({
      payer:wallet.publicKey,
    }).rpc();

    console.log(`Close transaction Sig ${closeTx}`);

    const closedUserAccount = await program.account.userState.fetchNullable(pdaDerivedPublicKey);

    console.log(`CloseAccount we should not be able to fetch ${closedUserAccount}`);

    assert.equal(closedUserAccount,null);

  });
});