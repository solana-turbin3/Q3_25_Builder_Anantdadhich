"use client"
import { useState } from 'react';
import { PublicKey } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';
import { useWallet } from '@solana/wallet-adapter-react';
import { useAmmProgram } from './data';
import { AmmUi } from './ui';

// This component is the main feature entry point.
// It connects the data logic from the hook to the UI components.
export function AmmFeature() {
    const { publicKey } = useWallet();
    const { usePoolState, initializePoolMutation, depositLiquidityMutation, swapMutation } = useAmmProgram();

    // State for the UI
    const [seed, setSeed] = useState('');
    const [mintX, setMintX] = useState('');
    const [mintY, setMintY] = useState('');
    const [configPda, setConfigPda] = useState(null);
    const [txSignature, setTxSignature] = useState('');
    const [depositAmount, setDepositAmount] = useState('');
    const [swapAmount, setSwapAmount] = useState('');
    const [swapDirection, setSwapDirection] = useState('x_to_y');

    const { data: poolState } = usePoolState(configPda);

    const handleInitialize = async () => {
        if (!seed || !mintX || !mintY) {
            alert("Please provide Seed, Mint X, and Mint Y");
            return;
        }
        try {
            const seedBN = new BN(seed);
            const mintXPubkey = new PublicKey(mintX);
            const mintYPubkey = new PublicKey(mintY);
            
            const result = await initializePoolMutation.mutateAsync({ seed: seedBN, fee: 30, mintX: mintXPubkey, mintY: mintYPubkey });
            //@ts-ignore
            setConfigPda(result.configPda);
            setTxSignature(result.tx);
            alert(`Pool initialized! Config PDA: ${result.configPda.toBase58()}`);
        } catch (error) {
            console.error(error);
        
        }
    };

    const handleDeposit = async () => {
        if (!configPda || !depositAmount) {
            alert("Please initialize a pool and enter a deposit amount.");
            return;
        }
        try {
            const depositBN = new BN(depositAmount).mul(new BN(10 ** 6)); // Assuming 6 decimals
            const result = await depositLiquidityMutation.mutateAsync({
                configPda,
                depositAmount: depositBN,
                maxX: depositBN, // Simplified for example
                maxY: depositBN, // Simplified for example
            });
            setTxSignature(result);
            alert("Deposit successful!");
        } catch (error) {
            console.error(error);
           
        }
    };

    const handleSwap = async () => {
        if (!configPda || !swapAmount) {
            alert("Please initialize a pool and enter a swap amount.");
            return;
        }
        try {
            const isX = swapDirection === 'x_to_y';
            const amountIn = new BN(swapAmount).mul(new BN(10 ** 6)); // Assuming 6 decimals
            const result = await swapMutation.mutateAsync({
                configPda,
                isX,
                amountIn,
                minOut: new BN(1), // Allow any amount out for simplicity
            });
            setTxSignature(result);
            alert("Swap successful!");
        } catch (error) {
            console.error(error);
          
        }
    };

    if (!publicKey) {
        return <div className="text-center text-lg">Please connect your wallet to continue.</div>
    }

    return (
        <AmmUi
            seed={seed}
            setSeed={setSeed}
            mintX={mintX}
            setMintX={setMintX}
            mintY={mintY}
            setMintY={setMintY}
            handleInitialize={handleInitialize}
            initializeLoading={initializePoolMutation.isPending}
            configPda={configPda}
            depositAmount={depositAmount}
            setDepositAmount={setDepositAmount}
            handleDeposit={handleDeposit}
            depositLoading={depositLiquidityMutation.isPending}
            swapAmount={swapAmount}
            setSwapAmount={setSwapAmount}
            swapDirection={swapDirection}
            setSwapDirection={setSwapDirection}
            handleSwap={handleSwap}
            swapLoading={swapMutation.isPending}
            poolState={poolState}
            txSignature={txSignature}
        />
    );
}
