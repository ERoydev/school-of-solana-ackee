// import { getAnchorProjectProgram, getAnchorProjectProgramId } from '@project/anchor'

// import { useConnection } from '@solana/wallet-adapter-react'
// import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
// import { useMutation, useQuery } from '@tanstack/react-query'
// import { useMemo } from 'react'
// import { useCluster } from '@/components/cluster/cluster-data-access'
// import { useAnchorProvider } from '@/components/solana/use-anchor-provider'
// import { useTransactionToast } from '@/components/use-transaction-toast'
// import { toast } from 'sonner'

// // Custom hook
// export function useCounterProgram() {
//   const { connection } = useConnection()
//   const { cluster } = useCluster()
//   const transactionToast = useTransactionToast()
  
//   const provider = useAnchorProvider()
//   const programId = useMemo(() => getAnchorProjectProgramId(cluster.network as Cluster), [cluster])
//   const program = useMemo(() => getAnchorProjectProgram(provider, programId), [provider, programId])

//   const accounts = useQuery({
//     queryKey: ['counter', 'all', { cluster }],
//     queryFn: () => program.account.counter.all(),
//   })

//   const getProgramAccount = useQuery({
//     queryKey: ['get-program-account', { cluster }],
//     queryFn: () => connection.getParsedAccountInfo(programId),
//   })

//   const initialize = useMutation({
//     mutationKey: ['counter', 'initialize', { cluster }],
//     mutationFn: (keypair: Keypair) =>
//       program.methods.initialize().accounts({ counter: keypair.publicKey }).signers([keypair]).rpc(),
//     onSuccess: async (signature) => {
//       transactionToast(signature)
//       await accounts.refetch()
//     },
//     onError: () => {
//       toast.error('Failed to initialize account')
//     },
//   })

//   return {
//     program,
//     programId,
//     accounts,
//     getProgramAccount,
//     initialize,
//   }
// }