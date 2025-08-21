"use client"
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import {
    useQuery,
    useMutation,
    useQueryClient as useTanstackQueryClient,
} from '@tanstack/react-query';
import { getAmmProgram } from "@/lib/anchorexport";
import { useAnchorProvider } from "../solana/solana-provider";
import { useMemo } from "react";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAccount, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";




export function useAmmProgram() {
    const wallet = useAnchorWallet();
    const { connection } = useConnection();
    const tanstackQueryClient = useTanstackQueryClient();

    const provider = useAnchorProvider()
    const program = useMemo(() => {
        if (!provider) return null;
        return getAmmProgram(provider);
    }, [provider]);

    // Fetch pool state using React Query
    const usePoolState = (configPda:any) => {
        return useQuery({
            queryKey: ['poolState', configPda?.toBase58()],
            queryFn: async () => {
                if (!program || !configPda) return null;
                try {
                    //@ts-ignore
                    const state = await program.account.config.fetch(configPda);
                    const [lpMint] = PublicKey.findProgramAddressSync(
                        [Buffer.from("lp"), configPda.toBuffer()],
                        program.programId
                    );
                    const vaultX = await getAssociatedTokenAddress(state.mintX, configPda, true);
                    const vaultY = await getAssociatedTokenAddress(state.mintY, configPda, true);
                    
                    const vaultXInfo = await getAccount(connection, vaultX);
                    const vaultYInfo = await getAccount(connection, vaultY);

                    return { ...state, vaultXAmount: vaultXInfo.amount, vaultYAmount: vaultYInfo.amount, lpMint };
                } catch (error) {
                    console.error("Failed to fetch pool state:", error);
                    return null;
                }
            },
            enabled: !!program && !!configPda,
        });
    };

    // Initialize Pool Mutation
    const initializePoolMutation = useMutation({
        mutationFn: async ({ seed, fee, mintX, mintY }:any) => {
            if (!program || !provider || !wallet) throw new Error("Program or provider not initialized");

            const [configPda] = PublicKey.findProgramAddressSync(
                [Buffer.from("config"), seed.toBuffer("le", 8)],
                program.programId
            );
            const [mintLp] = PublicKey.findProgramAddressSync(
                [Buffer.from("lp"), configPda.toBuffer()],
                program.programId
            );
            const vaultX = await getAssociatedTokenAddress(mintX, configPda, true);
            const vaultY = await getAssociatedTokenAddress(mintY, configPda, true);

            const tx = await program.methods
                .init(seed, fee, null)
                .accounts({
                    admin: wallet.publicKey,
                    mintX,
                    mintY,
                    mintLp,
                    vaultX,
                    vaultY,
                    config: configPda,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .rpc();
            
            return { tx, configPda };
        },
        onSuccess: () => {
            tanstackQueryClient.invalidateQueries({ queryKey: ['poolState'] });
        },
    });

    // Deposit Liquidity Mutation
    const depositLiquidityMutation = useMutation({
        mutationFn: async ({ configPda, depositAmount, maxX, maxY }:any) => {
            if (!program || !provider || !wallet) throw new Error("Program or provider not initialized");
             //@ts-ignore
            const poolState = await program.account.config.fetch(configPda);
            const [mintLp] = PublicKey.findProgramAddressSync(
                [Buffer.from("lp"), configPda.toBuffer()],
                program.programId
            );
            const vaultX = await getAssociatedTokenAddress(poolState.mintX, configPda, true);
            const vaultY = await getAssociatedTokenAddress(poolState.mintY, configPda, true);

            const userAtaX = await getAssociatedTokenAddress(poolState.mintX, wallet.publicKey);
            const userAtaY = await getAssociatedTokenAddress(poolState.mintY, wallet.publicKey);
            const userAtaLp = await getAssociatedTokenAddress(mintLp, wallet.publicKey, true);

            const tx = await program.methods
                .deposit(depositAmount, maxX, maxY)
                .accounts({
                    user: wallet.publicKey,
                    userAtaX,
                    userAtaY,
                    userAtaLp,
                    mintX: poolState.mintX,
                    mintY: poolState.mintY,
                    mintLp,
                    vaultX,
                    vaultY,
                    config: configPda,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .rpc();

            return tx;
        },
        onSuccess: (data, variables) => {
            tanstackQueryClient.invalidateQueries({ queryKey: ['poolState', variables.configPda.toBase58()] });
        },
    });

    // Swap Mutation
    const swapMutation = useMutation({
        mutationFn: async ({ configPda, isX, amountIn, minOut }:any) => {
            if (!program || !provider || !wallet) throw new Error("Program or provider not initialized");
             //@ts-ignore
            const poolState = await program.account.config.fetch(configPda);
            const vaultX = await getAssociatedTokenAddress(poolState.mintX, configPda, true);
            const vaultY = await getAssociatedTokenAddress(poolState.mintY, configPda, true);
            const userAtaX = await getAssociatedTokenAddress(poolState.mintX, wallet.publicKey);
            const userAtaY = await getAssociatedTokenAddress(poolState.mintY, wallet.publicKey);
            
            const tx = await program.methods
                .swap(isX, amountIn, minOut)
                .accounts({
                    signer: wallet.publicKey,
                    userAtaX,
                    userAtaY,
                    mintX: poolState.mintX,
                    mintY: poolState.mintY,
                    vaultX,
                    vaultY,
                    config: configPda,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .rpc();

            return tx;
        },
        onSuccess: (data, variables) => {
            tanstackQueryClient.invalidateQueries({ queryKey: ['poolState', variables.configPda.toBase58()] });
        },
    });

    return { usePoolState, initializePoolMutation, depositLiquidityMutation, swapMutation };
}

