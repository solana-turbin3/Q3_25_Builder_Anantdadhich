import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as spl from "@solana/spl-token"

describe("amm", () => {
  // Configure the client to use the local cluster.
  const provider=anchor.AnchorProvider.env()
  anchor.setProvider(provider) 



  const program = anchor.workspace.amm as Program<Amm>;

  const seed=new anchor.BN(1); 

  const fee=30;
  const user=Keypair.generate(); 
   let usertokenx:PublicKey , usertokeny:PublicKey ; 

   let tokenmintx:PublicKey , tokenminty:PublicKey ; 

   let tokenvaultx:PublicKey ,tokenvaulty:PublicKey ; 

   let config:PublicKey; 
   let tokenmintlp:PublicKey ;
   let usermintlp:PublicKey ; 

    before(async()=>{
      const airdropsignature=await provider.connection.requestAirdrop(user.publicKey, 

        2*anchor.web3.LAMPORTS_PER_SOL 
      );
      await provider.connection.confirmTransaction(airdropsignature); 

      tokenmintx=await spl.createMint(
        provider.connection,
        user,
        user.publicKey,
        null,
        6
        
      ); 

      tokenminty=await spl.createMint(
        provider.connection,
        user,
        user.publicKey,
        null,
        6
      );


      await spl.createAssociatedTokenAccount(
        provider.connection, 
        user,
        tokenmintx,
        user.publicKey
      );

      await spl.createAssociatedTokenAccount(
        provider.connection,
        user,
        tokenminty, 
         user.publicKey
      );

      usertokenx=spl.getAssociatedTokenAddressSync(tokenmintx,user.publicKey); 
      usertokeny=spl.getAssociatedTokenAddressSync(tokenminty,user.publicKey);

      await spl.mintTo(
        provider.connection,
        user,
        tokenmintx,
        usertokenx,
        user,
        200000000000

      )
      await spl.mintTo(
        provider.connection, 
        user,
        tokenminty,
        usertokeny,
        user,
        200000000000

      )

       config=PublicKey.findProgramAddressSync(
        [Buffer.from("config"),seed.toArrayLike(Buffer,"le",8)],
        program.programId
       )[0];

       tokenvaultx=spl.getAssociatedTokenAddressSync(tokenmintx,config,true);
       tokenvaulty=spl.getAssociatedTokenAddressSync(tokenminty,config,true);

       tokenmintlp=PublicKey.findProgramAddressSync(
        [Buffer.from("lp"),config.toBuffer()],
        program.programId
       )[0]; 

       usermintlp=spl.getAssociatedTokenAddressSync(tokenmintlp,user.publicKey);

    });




  it("Is initialized!", async () => {
    const tx = await program.methods.init(fee, seed, null).accountsPartial({
      signer: user.publicKey,
      mintX: tokenmintx,
      mintY: tokenminty,
      mintLp:tokenmintlp,
      vaultX: tokenvaultx,
      vaultY: tokenvaulty,
      config: config,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);
    console.log(" amm initialized successfully");
  });


  it("deposit tokens",async ()=> {
    const tx=await program.methods.deposit( new anchor.BN(2000000),
    new anchor.BN(1000000),
    new anchor.BN(1000000)).accountsPartial({
      signer:user.publicKey ,
      mintX:tokenmintx,
      mintY:tokenminty,
      mintLp:tokenmintlp,
      config,
      vaultX:tokenvaultx,
      vaultY:tokenvaulty,
      userX:usertokenx,
      userY:usertokeny ,
      userLp:usermintlp ,
      tokenProgram:spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram:spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram:anchor.web3.SystemProgram.programId
    }).signers([user]).rpc()

    console.log("your transaction signature",tx); 

    console.log(
      "token_x_vault balance should be 1000000",
      await provider.connection.getTokenAccountBalance(tokenvaultx)
    );
    console.log(
      "token_y_vault balance should be 1000000",
      await provider.connection.getTokenAccountBalance(tokenvaulty)
    );
    console.log(
      "user_lp balance should be 2000000",
      await provider.connection.getTokenAccountBalance(usermintlp)
    );
  })

  it("swap tokenx for tokeny",async ()=> {
     const tx=await program.methods.swap(true,new anchor.BN(2345),new anchor.BN(1)).accountsPartial({
       signer:user.publicKey ,
       mintX:tokenmintx,
       mintY:tokenminty ,
       vaultX:tokenvaultx ,
       vaultY:tokenvaulty ,
       config, 
       userX:usertokenx ,
       userY:usertokeny ,
       tokenProgram:spl.TOKEN_PROGRAM_ID ,
       associatedTokenProgram:spl.ASSOCIATED_TOKEN_PROGRAM_ID ,
       systemProgram:anchor.web3.SystemProgram.programId 
     }).signers([user]).rpc();


     console.log("Your transaction signature", tx);
     console.log(
       "token_x_vault balance should be ---->",
       await provider.connection.getTokenAccountBalance(tokenvaultx)
     );
     console.log(
       "token_Y_vault balance should be ---->",
       await provider.connection.getTokenAccountBalance(tokenvaulty)
     );
     console.log(
       "user_token_x balance should be ----->",
       await provider.connection.getTokenAccountBalance(usertokenx)
     );
  })

    it("withdraw tokens ",async ()=> {
      const tx=await program.methods.withdraw(
        new anchor.BN(2000000), 
        new anchor.BN(1000000), 
        new anchor.BN(1000000)  
      )
      .accountsPartial({
        user:user.publicKey ,
        mintX:tokenmintx,
        mintY:tokenminty,
        mintLp:tokenmintlp,
        config,
        vaultX:tokenvaultx,
        vaultY:tokenvaulty,
        userX:usertokenx,
        userY:usertokeny ,
        userLp:usermintlp ,
        tokenProgram:spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram:spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram:anchor.web3.SystemProgram.programId
      })
      .signers([user])
      .rpc();


      console.log("Your transaction signature", tx);
      console.log(
        "token_x_vault balance should be ---->",
        await provider.connection.getTokenAccountBalance(tokenvaultx)
      );
      console.log(
        "token_Y_vault balance should be ---->",
        await provider.connection.getTokenAccountBalance(tokenvaulty)
      );
      console.log(
        "user_token_x balance should be ----->",
        await provider.connection.getTokenAccountBalance(usertokenx)
      );
    })
   
});
