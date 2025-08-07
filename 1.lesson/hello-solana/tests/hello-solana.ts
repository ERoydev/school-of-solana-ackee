import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("hello-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Create a reference to my program so i can call methods
  const program = anchor.workspace.HelloSolana as Program<HelloSolana>;

  // .Keypair.generate() => Creates random private & public key (private key owned by a user)
  const signer = anchor.web3.Keypair.generate(); // Generate new solana wallet with no data, owned by Solana program(have public, private key)
  const data_account = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // I need to AirDrop some lamports in order for signer to have lamports to pay for data_account rent
    await program.provider.connection.confirmTransaction(await program.provider.connection.requestAirdrop(signer.publicKey, 100*anchor.web3.LAMPORTS_PER_SOL), "confirmed");


    // Add your test here.
    

    const tx = await program.methods.initialize("Hello Solana").accounts({
      signer: signer.publicKey,
      dataAccount: data_account.publicKey,
    }).signers(
      [signer, data_account] // These two account must sign in order to initialize and i specify it here
    ).rpc();

    console.log("Your transaction signature", tx);

    const dataAccount = await program.account.dataAccount.fetch(data_account.publicKey); // I fetch the account so i can perform tests on it

    console.log("Data Account: ", dataAccount);
  });
});
