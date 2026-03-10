import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountDataAnchor } from "../target/types/account_data_anchor";
import { it } from "mocha";
import { assert } from "chai";

describe("anchor", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.account_data_anchor as Program<AccountDataAnchor>;

  const addressInfoAccountKp = anchor.web3.Keypair.generate();
  const addressInfo = {
    name: "Devansh Chauhan",
    houseNumber: 7,
    street: "Sitaram-Baug",
    city: "Surendranagar"
  };

  it("Create the address info account", async () => {
    // Add your test here.
    const tx = await program.methods.createAddressAcc(addressInfo.name, addressInfo.houseNumber, addressInfo.street, addressInfo.city).accounts({
      payer: payer.publicKey,
      addressInfo: addressInfoAccountKp.publicKey
    }).signers([addressInfoAccountKp]).rpc();

    console.log("Your transaction signature", tx);

  });

  it("Reads the new account's data", async () => {
    const createdAddressInfo = await program.account.addressInfo.fetch(addressInfoAccountKp.publicKey);

    assert.strictEqual(createdAddressInfo.name, addressInfo.name);
    assert.strictEqual(createdAddressInfo.houseNumber, addressInfo.houseNumber);
    assert.strictEqual(createdAddressInfo.street, addressInfo.street);
    assert.strictEqual(createdAddressInfo.city, addressInfo.city);

    console.log(`Name     : ${createdAddressInfo.name}`);
		console.log(`House Num: ${createdAddressInfo.houseNumber}`);
		console.log(`Street   : ${createdAddressInfo.street}`);
		console.log(`City     : ${createdAddressInfo.city}`);
  });
});
