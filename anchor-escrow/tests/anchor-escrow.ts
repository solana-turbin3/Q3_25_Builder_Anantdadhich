import * as anchor from "@coral-xyz/anchor";
import { Program ,BN} from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction} from "@solana/web3.js"
import {createAssociatedTokenAccountIdempotentInstruction, createInitializeMint2Instruction, createMintToInstruction, getAssociatedTokenAddressSync, getMinimumBalanceForRentExemptMint, MINT_SIZE, TOKEN_2022_PROGRAM_ID} from "@solana/spl-token"
import {randomBytes} from  "crypto"

describe("anchor-escrow",() => {
    anchor.setProvider(anchor.AnchorProvider.env()) 

    const provider=anchor.getProvider(); 

    const connection=provider.connection; 

    const program=anchor.workspace.AnchorEscrow as Program<AnchorEscrow> 

    const token_program=TOKEN_2022_PROGRAM_ID; 
     

    const confirm=async(signature:string):Promise<string> => {
       const block=await connection.getLatestBlockhash(); 
       await connection.confirmTransaction({
        signature,
        ...block
       }) 

       return signature 
    } 

     const log=async(signature:string):Promise<string> => {
      console.log(
        `your transaction signature : https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
      ) ; 
      return signature 
     } 

      const seed=new BN(randomBytes(8));

      const [maker,taker,mintA,mintB]=Array.from({length:4},()=>  
       Keypair.generate() 
      ) ;

      const [makerAtaA, makerAtaB] = [mintA, mintB].map((mint) =>
        getAssociatedTokenAddressSync(mint.publicKey, maker.publicKey, false, token_program)
      );
      const [takerAtaA, takerAtaB] = [mintA, mintB].map((mint) =>
        getAssociatedTokenAddressSync(mint.publicKey, taker.publicKey, false, token_program)
      );

      const escrow=PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"),maker.publicKey.toBuffer(),seed.toArrayLike(Buffer,"le",8)],
        program.programId
      )[0]; 


      const vault=getAssociatedTokenAddressSync(mintA.publicKey,escrow,true,token_program)
      
      const accounts={
        maker:maker.publicKey,
        taker:taker.publicKey,
        mintA:mintA.publicKey,
        mintB:mintB.publicKey, 
        makerAtaA,
        makerAtaB, 
        takerAtaA, 
        takerAtaB, 
        escrow,
        vault, 
        token_program 
      }   


      it("Airdrop and creaate mints",async ()=> {
        let lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction();
    tx.instructions = [
      ...[maker, taker].map((account) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: account.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      ),
      ...[mintA, mintB].map((mint) =>
        SystemProgram.createAccount({
          fromPubkey: provider.publicKey,
          newAccountPubkey: mint.publicKey,
          lamports,
          space: MINT_SIZE,
          programId: token_program,
        })
      ),
      ...[
        { mint: mintA.publicKey, authority: maker.publicKey, ata: makerAtaA },
        { mint: mintB.publicKey, authority: taker.publicKey, ata: takerAtaB },
      ]
      .flatMap((x) => [
        createInitializeMint2Instruction(x.mint, 6, x.authority, null, token_program),
        createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, x.ata, x.authority, x.mint, token_program),
        createMintToInstruction(x.mint, x.ata, x.authority, 1e9, undefined, token_program),
      ])
    ];

    await provider.sendAndConfirm(tx, [mintA, mintB, maker, taker]).then(log);
  });

  it("Make", async () => {
    await program.methods
      .make(seed, new BN(1e6)) // Only two arguments
      .accounts({ ...accounts })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log);
  });

  xit("Refund", async () => {
    await program.methods.take()
      .accounts({ ...accounts })
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Take", async () => {
    try {
    await program.methods
      .take()
      .accounts({  ...accounts })
      .signers([taker])
      .rpc()
      .then(confirm)
      .then(log);
    } catch(e) {
      console.log(e);
      throw(e)
    }
  });
  


})