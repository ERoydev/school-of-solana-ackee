import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplEscrow } from "../target/types/spl_escrow";
import { assert } from "chai";
import { createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("spl-escrow: initialize_exchange", () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SplEscrow as Program<SplEscrow>;

  // Setup test accounts and mints
  const aliceWallet = anchor.web3.Keypair.generate();
  const bobWallet = anchor.web3.Keypair.generate();

  // Mint Accounts Initialization
  const aToBMint = await createMint(
    provider.connection,
    aliceWallet, // Keypair paying for the ming
    aliceWallet.publicKey, // Mint authority
    null, // Freeze authority
    6 // Decimals
  )

  // Associated Token Accounts (ATAs) Initialization
  const aliceATA = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    aliceWallet, // Payer
    aToBMint, // Mint
    aliceWallet.publicKey // Owner
  )

  it("Initializes escrow and transfers tokens", async () => {


    // Assume mint accounts are already created and funded
    const aToBMint = /* PublicKey of a_to_b_mint */;
    const bToAMint = /* PublicKey of b_to_a_mint */;
    const sideASendTokenAccount = /* ATA for sideA and aToBMint */;
    const escrowTokenAccount = /* PDA for escrow token account */;
    const escrow = /* PDA for escrow data account */;
    const aToBAmount = new anchor.BN(1000);
    const bToAAmount = new anchor.BN(500);

    // Call the instruction
    await program.methods
      .initializeExchange(aToBAmount, bToAAmount, sideB.publicKey)
      .accounts({
        sideA: sideA.publicKey,
        escrow,
        sideASendTokenAccountAta: sideASendTokenAccount,
        escrowTokenAccount,
        aToBMint,
        bToAMint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([sideA])
      .rpc();

    // Assert escrow state
    const escrowAccount = await program.account.escrow.fetch(escrow);
    assert.equal(escrowAccount.sideA.toBase58(), sideA.publicKey.toBase58());
    assert.equal(escrowAccount.sideB.toBase58(), sideB.publicKey.toBase58());
    assert.equal(escrowAccount.aToBAmount.toString(), aToBAmount.toString());
    assert.equal(escrowAccount.bToAAmount.toString(), bToAAmount.toString());

    // Assert token transfer
    // (fetch balances and check that escrowTokenAccount received tokens)
  });

  // Add more tests for edge cases, e.g. invalid mints, insufficient balance, etc.
});