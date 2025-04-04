import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { expect } from 'chai';

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Referencing the program - Abstraction that allows us to call methods of our SOL program.
  const program = anchor.workspace.calculator as Program<Calculator>;
  const programProvider = program.provider as anchor.AnchorProvider;

  // Generating a keypair for our Calculator account
  const calculatorPair = anchor.web3.Keypair.generate();

  const text = "Limechain School of Solana"

  it("Creating Calculator Instance", async () => {
    const tx = await program.methods
      .create(text)
      .accounts({
        // Supply all of the accounts that are used in context for this instruction
        calculator: calculatorPair.publicKey, // calculator account
        signer: programProvider.wallet.publicKey, // ref the public key of the wallet that is interacting with the test
      })
      .signers([calculatorPair]) //Singers required to sign this instruction
      .rpc();

    console.log("Your transaction signature: ", tx);

    // We fetch the account and red if the string is actually in the account
    const account = await program.account.calculator.fetch(calculatorPair.publicKey)
    expect(account.greeting).to.eql(text)
  });

  // Test addition
  it("Test addition", async () => {
    let x = new anchor.BN(5); // Since Rust expects i32 and i dont have such DT in JavaScript i use BN that handles this serialization
    let y = new anchor.BN(7);
    let expected_result = new anchor.BN(12);

    const tx = await program.methods
      .add(x, y)
      .accounts({
        calculator: calculatorPair.publicKey
      })
      .rpc();

    const account = await program.account.calculator.fetch(calculatorPair.publicKey)
    expect(account.result).to.eql(expected_result)
  })

  // Test subtraction
  it("Test Subtraction", async () => {
    let x = new anchor.BN(10); // Since Rust expects i32 and i dont have such DT in JavaScript i use BN that handles this serialization
    let y = new anchor.BN(3);
    let expected_result = new anchor.BN(7);

    const tx = await program.methods
      .subtraction(x, y)
      .accounts({
        calculator: calculatorPair.publicKey
      })
      .rpc();

    const account = await program.account.calculator.fetch(calculatorPair.publicKey)
    expect(account.result).to.eql(expected_result)
  })

});
