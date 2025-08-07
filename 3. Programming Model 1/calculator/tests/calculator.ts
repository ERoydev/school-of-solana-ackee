import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { expect } from 'chai';

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Referencing the program - Abstraction that allows us to call methods of our SOL program.
  const program = anchor.workspace.calculator as Program<Calculator>;
  // const programProvider = program.provider as anchor.AnchorProvider;

  // Generating a keypairs (wallet/accounts) that will be used to interact with the program
  const wallet = anchor.web3.Keypair.generate();
  const data_account = anchor.web3.Keypair.generate();

  it("Creating Calculator Instance", async () => {
    // I need to airdrop some SOL to the wallet that will be interacting with the program (signer in my case)
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(wallet.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );

    const tx = await program.methods
      .create("Hello World!")
      .accounts({
        // Supply all of the accounts that are used in context for this instruction
        calculator: data_account.publicKey, // calculator account
        signer: wallet.publicKey, // ref the public key of the wallet that is interacting with the test
      })
      .signers([wallet, data_account]) // Signers required to sign this instruction
      .rpc(); // rpc() is the method that sends the transaction to the network (in my case this should be the local cluster)

    console.log("Your transaction signature: ", tx);

    // We fetch the data account and read if the string is actually in the account
    const account = await program.account.calculator.fetch(data_account.publicKey)
    expect(account.greeting).to.eql("Hello World!")
  });

  // Test addition
  it("Test addition", async () => {
    let x = new anchor.BN(5); // Since Rust expects i32 and i don't have such DataType in JavaScript i use BN that handles this serialization
    let y = new anchor.BN(7);
    let expected_result = new anchor.BN(12);

    const tx = await program.methods
      .add(x, y)
      .accounts({
        calculator: data_account.publicKey
      })
      .rpc();

    const account = await program.account.calculator.fetch(data_account.publicKey)
    expect(account.result).to.eql(expected_result)
  })

  // Test subtraction
  it("Test Subtraction", async () => {
    let x = new anchor.BN(10); // Since Rust expects i32 and i don't have such DataType in JavaScript i use BN that handles this serialization
    let y = new anchor.BN(3);
    let expected_result = new anchor.BN(7);

    const tx = await program.methods
      .subtraction(x, y)
      .accounts({
        calculator: data_account.publicKey
      })
      .rpc();

    const account = await program.account.calculator.fetch(data_account.publicKey)
    expect(account.result).to.eql(expected_result)
  })

});
