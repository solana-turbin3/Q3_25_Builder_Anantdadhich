import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "../wallet.json"

const confirmed:Commitment="confirmed";

const connection=new Connection("https://api.devnet.solana.com",confirmed);
const keypair=Keypair.fromSecretKey(new Uint8Array(wallet));

async function airdrop(){
   try {
    const signature=await connection.requestAirdrop(keypair.publicKey,2*LAMPORTS_PER_SOL);
    console.log(`Success check out TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`)

   } catch (error) {
      console.log("error in airdrop",error)
   }

}
airdrop();