import { 
    Connection, 
    Keypair, 
    PublicKey, 
    Transaction, 
    TransactionInstruction,
    sendAndConfirmTransaction,
    AccountMeta
} from "@solana/web3.js";
import { 
    TOKEN_2022_PROGRAM_ID,
    createTransferCheckedInstruction
} from "@solana/spl-token";
import fs from "fs";


(async () => {
    try {
        // Create a connection to cluster
        const connection = new Connection("https://api.devnet.solana.com", "confirmed");
        
        // Load payer keypair from file
        const payerSecretKey = Uint8Array.from(JSON.parse(fs.readFileSync("/Users/emilemilovroydev/phantomWallet.json", "utf8")));
        const payer = Keypair.fromSecretKey(payerSecretKey);
        
        console.log("Payer address:", payer.publicKey.toString());

        connection.getLatestBlockhash().then(console.log);

        // Define addresses
        const senderATA = new PublicKey("7v3HcGrxjvxzC68CtR8Gqb727BE6Kcqmw7oy5yBhVNrd"); // I create each ATA using spl-token create-account <token_address> --owner ./sender.json
        const receiverATA = new PublicKey("6LpmSB5yF7tpM9Wjxai5ZBLRX2rBUVyVTbojfXjFxXLm"); // Receiver ATA
        const mint = new PublicKey("9cSXtFxpe64QyArPdEZPJLQLvRYrJ1GZDoNaGGhVW2gv"); // Mint Token address
        const hookProgram = new PublicKey("GLJosyGfzpEH1YRpek8rDnSBK8NG8vD3V7At3LwZCp43"); // Your deployed program

        // Create the transfer instruction using SPL Token-2022
        const transferIx = createTransferCheckedInstruction(
            senderATA,           // source
            mint,                // mint
            receiverATA,         // destination
            payer.publicKey,     // authority
            1_000_000,                  // amount
            9,                   // decimals
            [],                  // multiSigners
            TOKEN_2022_PROGRAM_ID
        );
        
        // Add the hook program as an additional account (read-only)
        transferIx.keys.push({
            pubkey: hookProgram,
            isSigner: false,
            isWritable: false
        });
        
        console.log("Transfer instruction created with", transferIx.keys.length, "accounts");
        
        // Create and send transaction
        const transaction = new Transaction().add(transferIx);
        
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [payer],
            { commitment: "confirmed" }
        );
        
        console.log("Transaction successful!");
        console.log("Signature:", signature);
        
    } catch (e) {
        console.error("Error:", e);
    }
})();
