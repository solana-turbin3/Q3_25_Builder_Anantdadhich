import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress,
  getAccount
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import BN from "bn.js";

describe("AMM Program Tests", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.amm as Program<Amm>;
  const admin = provider.wallet;
  const connection = provider.connection;


  const seed = new BN(Math.floor(Math.random() * 100000));
  
 
  const fee = 30;
  
  
  let mintX: PublicKey;
  let mintY: PublicKey;
  let mintLp: PublicKey;
  let vaultX: PublicKey;
  let vaultY: PublicKey;
  let userAtaX: PublicKey;
  let userAtaY: PublicKey;
  let userAtaLp: PublicKey;
  let configPda: PublicKey;
  
 
  before(async () => {
    
    mintX = await createMint(connection, admin.payer, admin.publicKey, null, 6);
    mintY = await createMint(connection, admin.payer, admin.publicKey, null, 6);
    
   
    const ataX = await getOrCreateAssociatedTokenAccount(connection, admin.payer, mintX, admin.publicKey);
    const ataY = await getOrCreateAssociatedTokenAccount(connection, admin.payer, mintY, admin.publicKey);
    userAtaX = ataX.address;
    userAtaY = ataY.address;

  
    await mintTo(connection, admin.payer, mintX, userAtaX, admin.publicKey, 100_000_000);
    await mintTo(connection, admin.payer, mintY, userAtaY, admin.publicKey, 100_000_000);

   
    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config"), seed.toBuffer("le", 8)],
      program.programId
    );
    
   
    [mintLp] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), configPda.toBuffer()],
      program.programId
    );
  });
  
  it("Initializes the liquidity pool", async () => {
   
    vaultX = await getAssociatedTokenAddress(mintX, configPda, true);
    vaultY = await getAssociatedTokenAddress(mintY, configPda, true);
    

    await program.methods
      .init(seed,fee,null) 
      .accountsPartial({
        admin: admin.publicKey,
        mintX: mintX,
        mintY: mintY,
        mintLp: mintLp,
        vaultX: vaultX,
        vaultY: vaultY,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      
    
    const configAccount = await program.account.config.fetch(configPda);
    assert.equal(configAccount.seed.toString(), seed.toString());
    assert.equal(configAccount.fee, fee);
    assert.deepEqual(configAccount.mintX, mintX);
    assert.deepEqual(configAccount.mintY, mintY);
  });
  
  it("Deposits liquidity", async () => {
    
    const ataLp = await getOrCreateAssociatedTokenAccount(connection, admin.payer, mintLp, admin.publicKey, true);
    userAtaLp = ataLp.address;

    const depositAmount = new BN(10_000_000); 
    const maxX = new BN(50_000_000);
    const maxY = new BN(50_000_000); 
    
    
    await program.methods
      .deposit(depositAmount, maxX, maxY)
      .accountsPartial({
        user: admin.publicKey,
        userAtaX: userAtaX, 
        userAtaY: userAtaY,
        userAtaLp: userAtaLp,
        mintX: mintX,
        mintY: mintY,
        mintLp: mintLp,
        vaultX: vaultX,
        vaultY: vaultY,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      
   
    const lpBalance = await getAccount(connection, userAtaLp);
    assert.equal(lpBalance.amount.toString(), depositAmount.toString());
  });
  
  it("Swaps token X for token Y", async () => {
    const amountIn = new BN(1_000_000); 
    const minOut = new BN(1); 
    
    const userAtaYBefore = await getAccount(connection, userAtaY);
    
   
    await program.methods
      .swap(true, amountIn, minOut) 
      .accountsPartial({
        signer: admin.publicKey, 
        userAtaX: userAtaX,
        userAtaY: userAtaY,
        mintX: mintX,
        mintY: mintY,
        vaultX: vaultX,
        vaultY: vaultY,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      
   
    const userAtaYAfter = await getAccount(connection, userAtaY);
    assert.isTrue(userAtaYAfter.amount > userAtaYBefore.amount, "User Y balance should increase");
  });
  
  it("Withdraws liquidity", async () => {
    const withdrawAmount = new BN(5_000_000); 
    const minX = new BN(1);
    const minY = new BN(1);
    
    const userLpBefore = await getAccount(connection, userAtaLp);
    
 
    await program.methods
      .withdraw(withdrawAmount, minX, minY)
      .accountsPartial({
        user: admin.publicKey,
        userAtaX: userAtaX,
        userAtaY: userAtaY,
        userAtaLp: userAtaLp,
        mintX: mintX,
        mintY: mintY,
        mintLp: mintLp,
        vaultX: vaultX,
        vaultY: vaultY,
        config: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      
  
    const userLpAfter = await getAccount(connection, userAtaLp);
    const expectedLpAfter = Number(userLpBefore.amount) - Number(withdrawAmount);
    assert.equal(userLpAfter.amount.toString(), expectedLpAfter.toString(), "LP tokens should be burned");
  });
});
