import { PublicKey } from "@solana/web3.js";

const TOKEN_METADATA_SEED = "token_metadata";

export const deriveTokenMetadataPDAs = async (userTokenAccounts: any, programId: any, userPubkey: PublicKey) => {
  return Promise.all(
    userTokenAccounts.map(async (acc) => {
      const mintPubkey = new PublicKey(acc.account.data.parsed.info.mint);
      const [pda] = await PublicKey.findProgramAddress(
        [
          Buffer.from(TOKEN_METADATA_SEED),
          userPubkey.toBuffer(),
          mintPubkey.toBuffer(),
        ],
        programId
      );
      return pda;
    })
  );
};