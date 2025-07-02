import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import wallet from "../wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
const commitment:Commitment="confirmed";

const keypair=Keypair.fromSecretKey(new Uint8Array(wallet)) 

const connection=new Connection("https://api.devnet.solana.com",commitment);


const mint=new PublicKey("FoGvzPbCYLrkwbhLDM3UmzTx6SjFYweBtyVntwmtE2H4");
const to=new PublicKey("75RKxPxsXtL7KffGijaGAYWE86VGE8hBJWnraXULR39U");
const token_decimals=1_000_000;

(async()=>{
    try {
         const from_ata=await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
         )   

         console.log(`from ata `,from_ata.address.toBase58())

         const to_ata=await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
         )

         console.log(`to ata`,to_ata.address.toBase58())

      
          
         const transaction=await transfer(
            connection,
            keypair,
            from_ata.address,
            to_ata.address,
            keypair.publicKey,
            10*token_decimals
         )

         console.log(`the traansaction is ${transaction}`)  // BUagqTyo4B8XYWoUpb2kaZqsGTVwjvcBdssKeqDjawwHvCvb6uVppgwiwC3E7SB5yESUwmD1AuJxfWAn3ig7En6
    } catch (error) {
         console.log("failed",error)
    }
})()