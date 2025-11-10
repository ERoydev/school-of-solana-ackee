// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import AnchorProjectIDL from '../target/idl/anchor_project.json'
import type { AnchorProject } from '../target/types/anchor_project'

// Re-export the generated IDL and type
export { AnchorProject, AnchorProjectIDL }

// The programId is imported from the program IDL.
export const ANCHOR_PROJECT_PROGRAM_ID = new PublicKey(AnchorProjectIDL.address)

// This is a helper function to get the Anchor program.
export function getAnchorProjectProgram(provider: AnchorProvider, address?: PublicKey): Program<AnchorProject> {
  return new Program({ ...AnchorProjectIDL, address: address ? address.toBase58() : AnchorProjectIDL.address } as AnchorProject, provider)
}

// This is a helper function to get the program ID for the Counter program depending on the cluster.
export function getAnchorProjectProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Counter program on devnet and testnet.
      return new PublicKey(AnchorProjectIDL.address) // This should take the ProgramID of my deployed program 
    case 'mainnet-beta':
    default:
      return ANCHOR_PROJECT_PROGRAM_ID
  }
}
