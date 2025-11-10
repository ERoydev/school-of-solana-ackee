// Mapped in tsconfig.json to get the exported function from my anchor_project/src
import { getAnchorProjectProgram, getAnchorProjectProgramId } from '@project/anchor';

import { Cluster, Keypair, PublicKey, Transaction } from '@solana/web3.js'
import { useMemo } from 'react'

import { useCluster } from '@/components/cluster/cluster-data-access'
import { useAnchorProvider } from '@/components/solana/use-anchor-provider'

// Queries
import { useMutation } from '@tanstack/react-query'
import { useTransactionToast } from '@/components/use-transaction-toast';
import { toast } from 'sonner'
import { createAssociatedTokenAccountInstruction, getAssociatedTokenAddress } from '@solana/spl-token';
import { BN } from 'bn.js';

export function useTokenMinter() {
    const { cluster } = useCluster()

    const transactionToast = useTransactionToast()

    const provider = useAnchorProvider()
    // This takes my program ID from the anchor project exported code
    const programId = useMemo(() => getAnchorProjectProgramId(cluster.network as Cluster), [cluster])
    const program = useMemo(() => getAnchorProjectProgram(provider, programId), [provider, programId])

    // Create the user's associated token account (ATA) for the new mint
    const create_mint_holder_ata = useMutation({
        mutationFn: async (mintKeypair: Keypair) => {
            const ata = await getAssociatedTokenAddress(
                mintKeypair.publicKey,
                provider.wallet.publicKey
            );

            // Return the instruction only, DO NOT send
            return createAssociatedTokenAccountInstruction(
                provider.wallet.publicKey, // payer
                ata,                       // ATA
                provider.wallet.publicKey, // owner
                mintKeypair.publicKey      // mint
            );
        },
        onSuccess: (instruction) => {
            // instruction is a TransactionInstruction, not a string
            console.log("Got ATA instruction:", instruction);
        },
        onError: () => toast.error("Failed to create ATA instruction"),
    });

    // Initialize the mint account
    const initialize_mint = useMutation({

        mutationFn: async ({
                mintKeypair,
                name,
                symbol,
                uri,
                supply
            }: {
                mintKeypair: Keypair;
                name: string;
                symbol: string;
                uri: string;
                supply: string;
            }) => {
            
            const tokenSupply = new BN(parseInt(supply) * (10 ** 9)); // Assuming 9 decimals
            
            const [mintAuthorityPda, _mintAuthorityBump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from("mint_authority"),
                    provider.wallet.publicKey.toBuffer(),
                    mintKeypair.publicKey.toBuffer()
                ],
                program.programId
            );
        
            const [tokenMetadataPda, _tokenMetadataBump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from("token_metadata"),
                    provider.publicKey.toBuffer(),
                    mintKeypair.publicKey.toBuffer(),
                ],
                program.programId
            );

            return await program.methods
                .initializeMintAccount(name, symbol, uri, tokenSupply) 
                .accounts({ 
                    payer: provider.wallet.publicKey,
                    mint: mintKeypair.publicKey,
                    mintAuthority: mintAuthorityPda,
                    tokenMetadata: tokenMetadataPda,
                    // No need to pass tokenProgram or systemProgram
                })
                .signers([mintKeypair])
                .instruction();
        },
        onSuccess: async (signature) => transactionToast(signature.toString()),
        onError: () => toast.error('Failed to initialize mint account')
    });

    // Initial minting when creating the token
    const initial_token_supply = useMutation({
        mutationFn: async ({ mintKeypair, ata, amount }: { mintKeypair: Keypair; ata: Keypair; amount: string }) => {
            const mintAmount = new BN(parseInt(amount) * (10 ** 9)); 

            const [mintAuthorityPda, _mintAuthorityBump] = await PublicKey.findProgramAddress(
                [
                    Buffer.from("mint_authority"),
                    provider.wallet.publicKey.toBuffer(),
                    mintKeypair.publicKey.toBuffer()
                ],
                program.programId
            );

            console.log("Mint Authority PDA:", mintAuthorityPda.toBase58());

            return await program.methods
                .mintTokens(mintAmount)
                .accounts({
                    signer: provider.wallet.publicKey,
                    mint: mintKeypair.publicKey,
                    to: ata.publicKey,
                    mintAuthority: mintAuthorityPda,
                })
                .signers([mintKeypair, ata]) // Cast to Keypair to satisfy types
                .instruction();
        },
        onSuccess: async (signature) => transactionToast(signature.toString()),
        onError: () => toast.error('Failed to mint tokens')
    })

    // For minting more tokens after the initial supply
    const mint_more_tokens = useMutation({
        mutationFn: async ({ mintPublicKey, ata, amount }: { mintPublicKey: PublicKey; ata: Keypair; amount: string }) => {
            const mintAmount = new BN(parseInt(amount) * (10 ** 9)); 

            return await program.methods
                .mintTokens(mintAmount)
                .accounts({
                    mint: mintPublicKey,
                    to: ata.publicKey,
                    authority: provider.wallet.publicKey,
                })
                .instruction();
        },
        onSuccess: async (signature) => transactionToast(signature.toString()),
        onError: () => toast.error('Failed to mint tokens')
    })



    return { program, programId, provider, initialize_mint, create_mint_holder_ata, initial_token_supply, mint_more_tokens }
}
