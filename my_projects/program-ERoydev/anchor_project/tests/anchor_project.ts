import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorProject } from "../target/types/anchor_project";

import * as token from "@solana/spl-token";
import { assert } from "chai";
import { BN } from "bn.js";
import { Keypair, PublicKey } from "@solana/web3.js";

// =========// Helper function to airdrop SOL to a given address
async function airdrop(connection: any, address: any, amount = 10000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}

describe("Liquidity Pool Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorProject as Program<AnchorProject>;

  let userWallet: anchor.web3.Keypair;
  let USDC_token_authority: anchor.web3.Keypair;
  let SOL_token_authority: anchor.web3.Keypair;

  let USDC_token_mint: anchor.web3.PublicKey;
  let SOL_token_mint: anchor.web3.PublicKey;

  let user_token_a_ata: token.Account;
  let user_token_b_ata: token.Account;

  const to_mint = new anchor.BN(100); // Initial mint for testing each token with 100 units

  beforeEach(async () => {
    userWallet = anchor.web3.Keypair.generate();
    USDC_token_authority = anchor.web3.Keypair.generate();
    SOL_token_authority = anchor.web3.Keypair.generate();

    await airdrop(provider.connection, userWallet.publicKey);
    await airdrop(provider.connection, USDC_token_authority.publicKey);
    await airdrop(provider.connection, SOL_token_authority.publicKey);

    // 1. Create Mint Accounts for Token A and Token B
    USDC_token_mint = await token.createMint(
      provider.connection,
      USDC_token_authority,
      USDC_token_authority.publicKey,
      null,
      9 // Decimals for token A
    );

    SOL_token_mint = await token.createMint(
      provider.connection,
      SOL_token_authority,
      SOL_token_authority.publicKey,
      null,
      9 // Decimals for token B
    );

    // 1.1 Configs
    let token_a_amount = new anchor.BN(30);
    let token_b_amount = new anchor.BN(40);

    // 2. Create User ATA
    user_token_a_ata = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      userWallet,
      USDC_token_mint,
      userWallet.publicKey
    );

    user_token_b_ata = await token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      userWallet,
      SOL_token_mint,
      userWallet.publicKey
    );  

    await token.mintTo(provider.connection, userWallet, USDC_token_mint, user_token_a_ata.address, USDC_token_authority, to_mint.toNumber());
    await token.mintTo(provider.connection, userWallet, SOL_token_mint, user_token_b_ata.address, SOL_token_authority, to_mint.toNumber());

  });

  it("Initialize liquidity pool, should be successful", async () => {
    let [poolPda, poolBump] = getPoolPda(USDC_token_mint, SOL_token_mint, program.programId);
    let [lpMintPda, lpMintBump] = getLpMintPda(poolPda, program.programId);

    let [escrowTokenAAccountPda, escrowTokenABump] = getEscrowTokenAAccountPda(poolPda, program.programId);
    let [escrowTokenBAccountPda, escrowTokenBBump] = getEscrowTokenBAccountPda(poolPda, program.programId);
  
    await program.methods
      .initializeLiquidityPool()
      .accounts({
        creator: userWallet.publicKey,
        // @ts-ignore
        pool: poolPda,
        lpMint: lpMintPda,
        escrowTokenAAccount: escrowTokenAAccountPda,
        escrowTokenBAccount: escrowTokenBAccountPda,
        tokenAMint: USDC_token_mint,
        tokenBMint: SOL_token_mint,  
        tokenProgram: token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: token.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userWallet])
      .rpc({skipPreflight: true});

    // Fetch the pool account to verify initialization
    const poolAccount = await program.account.pool.fetch(poolPda); 
    assert.equal(poolAccount.tokenAMint.toBase58(), USDC_token_mint.toBase58(), "Token A mint does not match");
    assert.equal(poolAccount.tokenBMint.toBase58(), SOL_token_mint.toBase58(), "Token B mint does not match");
    assert.equal(poolAccount.escrowTokenAAccount.toBase58(), escrowTokenAAccountPda.toBase58(), "Escrow Token A account does not match");
    assert.equal(poolAccount.escrowTokenBAccount.toBase58(), escrowTokenBAccountPda.toBase58(), "Escrow Token B account does not match");
    assert.equal(poolAccount.reserveA.toString(), new BN(0).toString(), "Reserve A does not match");
    assert.equal(poolAccount.reserveB.toString(), new BN(0).toString(), "Reserve B does not match");

    const escrowAAccountInfo = await token.getAccount(
      provider.connection,
      escrowTokenAAccountPda
    );
    assert.equal(escrowAAccountInfo.amount.toString(), new BN(0).toString(), "Escrow Token A balance does not match expected");

    const escrowBAccountInfo = await token.getAccount(
      provider.connection,
      escrowTokenBAccountPda
    );
    assert.equal(escrowBAccountInfo.amount.toString(), new BN(0).toString(), "Escrow Token B balance does not match expected");
  });

  it("Add liquidity, should be successful", async () => {
    let [poolPda, poolBump] = getPoolPda(USDC_token_mint, SOL_token_mint, program.programId);
    let [lpMintPda, lpMintBump] = getLpMintPda(poolPda, program.programId);

    let [escrowTokenAAccountPda, escrowTokenABump] = getEscrowTokenAAccountPda(poolPda, program.programId);
    let [escrowTokenBAccountPda, escrowTokenBBump] = getEscrowTokenBAccountPda(poolPda, program.programId);

    // Initialize the liquidity pool first
    await program.methods
      .initializeLiquidityPool()
      .accounts({
        creator: userWallet.publicKey,
        // @ts-ignore
        pool: poolPda,
        lpMint: lpMintPda,
        escrowTokenAAccount: escrowTokenAAccountPda,
        escrowTokenBAccount: escrowTokenBAccountPda,
        tokenAMint: USDC_token_mint,
        tokenBMint: SOL_token_mint,
        tokenProgram: token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: token.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userWallet])
      .rpc();

    // Add liquidity to the pool
    const amountA = new BN(10); // Amount of Token A to add
    const amountB = new BN(20); // Amount of Token B to add

    // Derive the ATA address for the user and token mints
     const userSendTokenAAccountAta = token.getAssociatedTokenAddressSync(
      USDC_token_mint,           // token_b_mint
      userWallet.publicKey      // provider
    );

    const userSendTokenBAccountAta = token.getAssociatedTokenAddressSync(
      SOL_token_mint,           // token_b_mint
      userWallet.publicKey      // provider
    );

    let [lpProviderPda, lpProviderBump] = getLpProviderPda(poolPda, userWallet.publicKey, program.programId);

    const lpUserReceiveAta = token.getAssociatedTokenAddressSync(
      lpMintPda,            // The LP mint PDA
      userWallet.publicKey  // The user's wallet (provider)
    );

    await program.methods
      .addLiquidity(amountA, amountB)
      .accounts({
        provider: userWallet.publicKey,
        pool: poolPda,
        // @ts-ignore
        userSendTokenAAccountAta: userSendTokenAAccountAta, // User's ATA for Token A
        userSendTokenBAccountAta: userSendTokenBAccountAta, // User's ATA
        escrowTokenAAccount: escrowTokenAAccountPda,
        escrowTokenBAccount: escrowTokenBAccountPda,
        lpProvider: lpProviderPda,
        lpUserReceiveAta: lpUserReceiveAta, // User's ATA for LP tokens
        lpMint: lpMintPda,
        tokenAMint: USDC_token_mint,
        tokenBMint: SOL_token_mint,
        associatedTokenProgram: token.ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userWallet])
      .rpc();

    // // Fetch the updated pool account to verify liquidity addition
    const poolAccount = await program.account.pool.fetch(poolPda);
    
    assert.equal(poolAccount.reserveA.toString(), amountA.toString(), "Reserve A does not match after adding liquidity");
    assert.equal(poolAccount.reserveB.toString(), amountB.toString(), "Reserve B does not match after adding liquidity");
    
    const escrowAAccountInfo = await token.getAccount(
      provider.connection,
      escrowTokenAAccountPda
    );
    assert.equal(escrowAAccountInfo.amount.toString(), amountA.toString(), "Escrow Token A balance does not match expected");
    
    const escrowBAccountInfo = await token.getAccount(
      provider.connection,
      escrowTokenBAccountPda
    );
    assert.equal(escrowBAccountInfo.amount.toString(), amountB.toString(), "Escrow Token B balance does not match expected");

    let expectedLp = getAmountInitialLpTokensToMint(amountA, amountB);
  
    // Verify the LP Provider account
    const lpProviderAccount = await program.account.lpProvider.fetch(lpProviderPda);
    assert.equal(lpProviderAccount.user.toBase58(), userWallet.publicKey.toBase58(), "LP Provider user does not match");
    assert.equal(lpProviderAccount.tokenAProvided.toString(), amountA.toString(), "LP Provider Token A provided does not match");
    assert.equal(lpProviderAccount.tokenBProvided.toString(), amountB.toString(), "LP Provider Token B provided does not match");
    assert.equal(lpProviderAccount.lpTokensOwned.toString(), expectedLp.toString(), "LP Provider LP tokens owned does not match");
  })


});

describe("Token Minting Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.anchorProject as Program<AnchorProject>;


  it("Initialize Mint Account, should be successful", async () => {
    const payer = provider.wallet;
    const mint = Keypair.generate();

    // Derive PDA for mint_authority
    const [mintAuthority, mintAuthorityBump] = await PublicKey.findProgramAddress(
      [
        Buffer.from("mint_authority"),
        payer.publicKey.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Derive PDA for token_metadata
    const [tokenMetadataPda, tokenMetadataBump] = await PublicKey.findProgramAddress(
      [
        Buffer.from("token_metadata"),
        payer.publicKey.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      program.programId
    );

    const logoUrl = "https://cdn-icons-png.flaticon.com/512/17978/17978725.png";

    // Call initialize
    await program.methods
      .initializeMintAccount("TestToken", "TTK", logoUrl, new anchor.BN(100000))
      .accounts({
        payer: payer.publicKey,
        mint: mint.publicKey,
        // @ts-ignore
        mintAuthority: mintAuthority,
        tokenMetadata: tokenMetadataPda,
        tokenProgram: token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([mint])
      .rpc();

    // Fetch and assert token metadata 
    const metadataAccount = await program.account.tokenMetadata.fetch(tokenMetadataPda);
    assert.equal(metadataAccount.name, "TestToken");
    assert.equal(metadataAccount.symbol, "TTK");
    assert.equal(metadataAccount.mint.toBase58(), mint.publicKey.toBase58());
    assert.equal(metadataAccount.creator.toBase58(), payer.publicKey.toBase58());
    assert.equal(metadataAccount.decimals, 9);
    assert.equal(metadataAccount.supply.toNumber(), 100000);
  });

});

// ====================== PDA DERIVATION FUNCTION HELPERS

function getLpMintPda(pool_pk: anchor.web3.PublicKey, programID: anchor.web3.PublicKey): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("lp-mint"), 
      pool_pk.toBuffer()
    ],
    programID
  );
}

function getPoolPda(token_a_mint: anchor.web3.PublicKey, token_b_mint: anchor.web3.PublicKey, programID: anchor.web3.PublicKey): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("liquidity_pool"), 
      token_a_mint.toBuffer(), 
      token_b_mint.toBuffer()
    ],
    programID
  );
}


function getEscrowTokenAAccountPda(pool_pk: anchor.web3.PublicKey, programID: anchor.web3.PublicKey): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow-a"), 
      pool_pk.toBuffer()
    ],
    programID
  );
}

function getLpProviderPda(pool_pk: anchor.web3.PublicKey, creator_pk: anchor.web3.PublicKey, programID: anchor.web3.PublicKey): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("lp-provider"), 
      creator_pk.toBuffer(),
      pool_pk.toBuffer()
    ],
    programID
  );
}

function getEscrowTokenBAccountPda(pool_pk: anchor.web3.PublicKey, programID: anchor.web3.PublicKey): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow-b"), 
      pool_pk.toBuffer()
    ],
    programID
  );
}

// Util function to calculate initial LP tokens to mint based on the provided amounts of Token A and Token B
function getAmountInitialLpTokensToMint(depositA, depositB) {
  // Usage: let expectedLp = getAmountInitialLpTokensToMint(amountA, amountB);
  const total = BigInt(depositA) * BigInt(depositB);
  const amountLpToMint = Math.sqrt(Number(total));
  return Math.floor(amountLpToMint); // Match Rust's as u64 truncation
}