import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaErrorsLecture5 } from "../target/types/solana_errors_lecture_5";
import { expect, assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("solana-errors-lecture-5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;

  const program = anchor.workspace.solanaErrorsLecture5 as Program<SolanaErrorsLecture5>;
  
  // Create keypair for data Account
  const user = anchor.web3.Keypair.generate();
  // const dataAccount = anchor.web3.Keypair.generate();
  // Since i use PDA i need to use another way of signing

  // Get the PDA using programId and the static seed string used
  const [dataPDA] = PublicKey.findProgramAddressSync([Buffer.from("data")], program.programId)

  before("prepare", async() => {
    await airdrop(anchor.getProvider().connection, user.publicKey)
  })

  it("Cannot initialize with incorrect data account!", async () => {

    console.log("user balance = " + await connection.getBalance(user.publicKey))

    let expected_counter = new anchor.BN(0);

    const bad_data_account = Keypair.generate(); // Not PDA as expected

    try {
      const tx = await program.methods
        .initialize(2)
        .accounts({
          user: user.publicKey,
          data: bad_data_account.publicKey,
        })
        .signers([user])
        .rpc();
        assert.fail();

    } catch(_err) {
      const err = anchor.AnchorError.parse(_err.logs);
      assert.strictEqual(err.error.errorCode.code, "ConstraintSeeds");
    }

  });

  it("Is initialized!", async () => {

    console.log("user balance = " + await connection.getBalance(user.publicKey))

    let expected_counter = new anchor.BN(0);

    const tx = await program.methods
      .initialize(10)
      .accounts({
        user: user.publicKey,
        data: dataPDA,
      })
      .signers([user])
      .rpc({commitment: "confirmed"});
      // .rpc({skipPreflight: true}); -> This will skip preflight checks before sending the transaction, i will recieve different errors and logs in .anchor/program-logs


    let dataAccount = await program.account.myData.fetch(dataPDA); // i Fetch account via the PDA
    assert.deepEqual(dataAccount.authority, user.publicKey)
    assert.strictEqual(dataAccount.counter, 0)
  });
});

async function airdrop(connection: any, address: any, amount = 100_000_000) {
  const signature = await connection.requestAirdrop(address, amount);
  await connection.confirmTransaction(signature, 'confirmed')
}