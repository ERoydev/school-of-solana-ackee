import React, { useState } from "react";
import { Button } from "../ui/button";
import { Keypair, Transaction } from "@solana/web3.js";
import { useTokenMinter } from "@/hooks/useTokenMinter";
import { useAnchorProvider } from "../solana/use-anchor-provider";
import { useTransactionToast } from "../use-transaction-toast";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { useNavigate } from "react-router";

export default function MintTokens() {
    // Form state variables
    const [tokenName, setTokenName] = useState("");
    const [tokenSymbol, setTokenSymbol] = useState("");
    const [tokenSupply, setTokenSupply] = useState("");
    const [logoUri, setLogoUri] = useState("");

    const [minted, setMinted] = useState(false);
    const { initialize_mint, create_mint_holder_ata, initial_token_supply } = useTokenMinter();
    const transactionToast = useTransactionToast()

    const provider = useAnchorProvider()

    const navigate = useNavigate();

    
    const handleMint = async () => {
        try {
            // Generate a new Keypair for the mint
            const mintKeypair = Keypair.generate();
    
            // Initialize mint acccount
            const initMintIx = await initialize_mint.mutateAsync({ mintKeypair: mintKeypair, name: tokenName, symbol: tokenSymbol, uri: logoUri, supply: tokenSupply });
    
            // Create user's ATA that will hold the minted tokens
            const createAtaIx = await create_mint_holder_ata.mutateAsync(mintKeypair);
    
            // Compute user's associated token account
            const userATA = await getAssociatedTokenAddress(
                mintKeypair.publicKey,
                provider.wallet.publicKey
            );
    
            // Build mint instruction
            const mintAmount = tokenSupply.toString()// Assuming 9 decimals
            const mintIx = await initial_token_supply.mutateAsync({ mintKeypair, ata: { publicKey: userATA } as Keypair, amount: mintAmount });
    
            // Bundle all these operations into a single transaction
            const tx = new Transaction().add(initMintIx, createAtaIx, mintIx);
    
            // Send transaction
            const signature = await provider.sendAndConfirm(tx, [mintKeypair]);
            transactionToast(signature);
    
            setMinted(true);

            navigate("/");

        } catch (error) {
            console.error("Minting failed:", error);
        }
    };

    return (
        <div className="flex min-h-screen justify-center pt-16 p-4">
            <div className="w-full max-w-xl bg-gray-900 rounded-2xl window-border window-shadow p-12 flex flex-col gap-8">
                <h2 className="text-2xl md:text-3xl font-bold text-cyan-400 mb-2 text-center">Mint New Token</h2>
                <p className="text-gray-400 text-center mb-4">Create your own token and mint supply to your wallet.</p>

                <div className="flex flex-col gap-4">
                    <input
                        type="text"
                        placeholder="Token Name"
                        value={tokenName}
                        onChange={e => setTokenName(e.target.value)}
                        className="w-full rounded-lg bg-gray-800 px-4 py-2 text-cyan-200 border-2 border-cyan-400/30 shadow focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all duration-200 placeholder:text-gray-500 outline-none"
                    />
                    <input
                        type="text"
                        placeholder="Token Symbol (e.g. USDC)"
                        value={tokenSymbol}
                        onChange={e => setTokenSymbol(e.target.value)}
                        className="w-full rounded-lg bg-gray-800 px-4 py-2 text-cyan-200 border-2 border-cyan-400/30 shadow focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all duration-200 placeholder:text-gray-500 outline-none"
                    />
                    <input
                        type="number"
                        min={1}
                        placeholder="Initial Token Supply"
                        value={tokenSupply}
                        onChange={e => setTokenSupply(e.target.value)}
                        className="w-full rounded-lg bg-gray-800 px-4 py-2 text-cyan-200 border-2 border-cyan-400/30 shadow focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all duration-200 placeholder:text-gray-500 outline-none"
                    />
                    <input
                        id="logo-uri"
                        type="text"
                        value={logoUri}
                        onChange={e => setLogoUri(e.target.value)}
                        placeholder="Enter logo URI"
                        className="w-full rounded-lg bg-gray-800 px-4 py-2 text-cyan-200 border-2 border-cyan-400/30 shadow focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all duration-200 placeholder:text-gray-500 outline-none"
                        />
                    {logoUri && <img src={logoUri} alt="Logo preview" style={{ maxWidth: 100 }} />}
                </div>
            
            <Button
                className="w-full mt-4 px-6 py-3 rounded-xl bg-cyan-400 text-black font-bold text-lg shadow hover:bg-cyan-300 transition"
                onClick={handleMint}
                disabled={!tokenName || !tokenSymbol || !tokenSupply}
                >
                Mint Token
            </Button>

            {minted && (
                <div className="mt-4 text-green-400 text-center font-semibold">Token minted successfully!</div>
            )}

            </div>
        </div>
    );
}
