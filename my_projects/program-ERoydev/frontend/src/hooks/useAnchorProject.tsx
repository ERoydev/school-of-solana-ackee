// Mapped in tsconfig.json to get the exported function from my anchor_project/src
import { getAnchorProjectProgram, getAnchorProjectProgramId } from '@project/anchor';

import { Cluster, Keypair, Transaction } from '@solana/web3.js'
import { useMemo } from 'react'

import { useCluster } from '@/components/cluster/cluster-data-access'
import { useAnchorProvider } from '@/components/solana/use-anchor-provider'

// Queries
import { useMutation, useQuery } from '@tanstack/react-query'
import { useTransactionToast } from '@/components/use-transaction-toast';
import { toast } from 'sonner'
import { createAssociatedTokenAccount, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { deriveTokenMetadataPDAs } from '@/utils/deriveTokenMetadatasPDAs';
import { fetchTokenMetadataAccounts } from '@/utils/fetchTokenMetadataAccounts';

export function useAnchorProgram() {
    const { cluster } = useCluster()

    const transactionToast = useTransactionToast()

    const provider = useAnchorProvider()
    // This takes my program ID from the anchor project exported code
    const programId = useMemo(() => getAnchorProjectProgramId(cluster.network as Cluster), [cluster])
    const program = useMemo(() => getAnchorProjectProgram(provider, programId), [provider, programId])

    // 
    const userTokenAccountsQuery = useQuery({
        queryKey: ['userTokenAccounts', provider.wallet?.publicKey?.toBase58()],
        queryFn: async () => {
            if (!provider.wallet?.publicKey) return [];
                // Fetch all Token accounts (including ATAs) for the connected wallet
                const tokenAccounts = await provider.connection.getParsedTokenAccountsByOwner(
                provider.wallet.publicKey,
                { programId: TOKEN_PROGRAM_ID }
            );

            // Fetch all PDA's for TokenMetadata accounts linked to those token accounts
            const pdas = await deriveTokenMetadataPDAs(tokenAccounts.value, program.programId, provider.wallet.publicKey);
            // All TokenMetadata accounts data fetched from the blockchain
                const metadataAccounts = await Promise.all(
                    pdas.map(async (pda) => {
                        try {
                        const data = await program.account.tokenMetadata.fetch(pda);
                        return { pda, ...data };
                        } catch {
                        return null;
                        }
                    })
                );

            return metadataAccounts.filter(Boolean);
        },
        enabled: !!provider.wallet?.publicKey,
    });

    // Fetch all SPL token accounts (including ATAs) for the connected wallet
    // const allTokens = useQuery({

    // })

    /*
        useQuery is a React hook from React Query that:
            - Runs an asynchronous function (queryFn) to fetch data (e.g., accounts from the blockchain).
            - Caches the result and keeps it updated.
            - Provides loading, error, and data states for easy UI management.
    
    */

    // Example: Fetching all accounts from the program

    // const poolAccounts = useQuery({
    //     queryKey: ['anchor_project', 'pool', { cluster }],
    //     queryFn: () => program.account.pool.all(),
    // })

    // const lpProviderAccounts = useQuery({
    //     queryKey: ['anchor_project', 'lpProvider', { cluster }],
    //     queryFn: () => program.account.lpProvider.all(),
    // })


    /*
        The useMutation hook from React Query is used for handling actions that change data—like creating, updating, or deleting records.

            - It runs an asynchronous function (mutationFn) when you trigger the mutation (e.g., by clicking a button).
            - It tracks the mutation’s state: loading, success, or error.
            - You can define callbacks (onSuccess, onError) to react to the result, such as showing notifications or refreshing data.
    */

    //Example: Initializing a new account in the program

    return { program, programId, provider, userTokenAccountsQuery}
}


// =============================== Example of usage of this hook in a component =======================================

// export function useCounterProgramAccount({ account }: { account: PublicKey }) {
//   const { cluster } = useCluster()
//   const transactionToast = useTransactionToast()
//   const { program, accounts } = useCounterProgram()

//   const accountQuery = useQuery({
//     queryKey: ['counter', 'fetch', { cluster, account }],
//     queryFn: () => program.account.counter.fetch(account),
//   })

//   const closeMutation = useMutation({
//     mutationKey: ['counter', 'close', { cluster, account }],
//     mutationFn: () => program.methods.close().accounts({ counter: account }).rpc(),
//     onSuccess: async (tx) => {
//       transactionToast(tx)
//       await accounts.refetch()
//     },
//   })

//   const decrementMutation = useMutation({
//     mutationKey: ['counter', 'decrement', { cluster, account }],
//     mutationFn: () => program.methods.decrement().accounts({ counter: account }).rpc(),
//     onSuccess: async (tx) => {
//       transactionToast(tx)
//       await accountQuery.refetch()
//     },
//   })

//   const incrementMutation = useMutation({
//     mutationKey: ['counter', 'increment', { cluster, account }],
//     mutationFn: () => program.methods.increment().accounts({ counter: account }).rpc(),
//     onSuccess: async (tx) => {
//       transactionToast(tx)
//       await accountQuery.refetch()
//     },
//   })

//   const setMutation = useMutation({
//     mutationKey: ['counter', 'set', { cluster, account }],
//     mutationFn: (value: number) => program.methods.set(value).accounts({ counter: account }).rpc(),
//     onSuccess: async (tx) => {
//       transactionToast(tx)
//       await accountQuery.refetch()
//     },
//   })

//   return {
//     accountQuery,
//     closeMutation,
//     decrementMutation,
//     incrementMutation,
//     setMutation,
//   }
// }
