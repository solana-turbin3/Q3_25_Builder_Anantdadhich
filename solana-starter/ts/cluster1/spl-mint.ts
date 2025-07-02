import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import wallet from "../wallet.json"
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";


const commitment:Commitment="confirmed";

const keypair=Keypair.fromSecretKey(new Uint8Array(wallet)) 

const connection=new Connection("https://api.devnet.solana.com",commitment);


const token_decimals=1_000_000;

const mint=new PublicKey("FoGvzPbCYLrkwbhLDM3UmzTx6SjFYweBtyVntwmtE2H4");  

(async ()=>{
     try {
         //create an ata  
         const ata=await getOrCreateAssociatedTokenAccount(    ///7GrK71FEeXqEeCZRisr9zzKfCPkeXb72PZjqicT73Rxn
            connection,
            keypair,
            mint,
            keypair.publicKey,
            

         );

         console.log(`the ata is ${ata.address.toBase58()}`)   

         //let mint it into the ata
          
         const minttr=await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair.publicKey,
            100*token_decimals
         )

         console.log(`the transaction ${minttr}`)      ///3sqjqtLxf1wm9z1AWZRyVSjdXeDa9vfteee6ZKtypWEDu2A5oyZnftYGSnunhU1nNF4PnFagLSSejRmTtTzyexJ
     } catch (error) {
        console.log("the erorr in creating mint",error); 
     }
})()   

  
  