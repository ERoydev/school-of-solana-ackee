import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramA } from "../target/types/program_a";
import { ProgramB } from "../target/types/program_b";
import { assert } from "chai";

const PDA_ACCOUNT_SEED = "ackee";

describe("program-a", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programA = anchor.workspace.programA as Program<ProgramA>;
  const programB = anchor.workspace.programB as Program<ProgramB>; // Logically i should create a new programB instance to use it in the test. since i have CPI call to program B in the program A.

  let signer = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    let connection = programA.provider.connection;    
    const [pda_address, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        // Here i specify all the seeds that I used in the program
        Buffer.from(PDA_ACCOUNT_SEED), 
        signer.publicKey.toBuffer()
      ], programA.programId // programId is part of derivation process its Program Derived Address after all.
    );
    
    await airdrop(connection, pda_address, 500_000_000_000);

    const tx = await programA.methods.initialize()
      .accounts({
        pdaAccount: pda_address,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId, 
        programB: programB.programId // CPI call to program B 
      })
      .signers([signer])
      .rpc();
    
    console.log("Your transaction signature", tx);

    // I use .getAccountInfo to fetch the account info and i can whech whatever i want then 
    let pda_address_info = await connection.getAccountInfo(pda_address);
    let signer_info = await connection.getAccountInfo(signer.publicKey);

    assert.equal(pda_address_info.lamports, 499999000000);
    assert.equal(signer_info.lamports, 1_000_000);
  });
});


export async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000 // 0.5 SOL`
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}