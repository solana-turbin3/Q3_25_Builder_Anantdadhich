import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js" 
import Wallet from "./dev-wallet.json"  

const keypair=Keypair.fromSecretKey(new Uint8Array(Wallet)) 


const connection=new Connection("https://api.devnet.solana.com");


(async ()=>{
    try {
        const transactionhash=await connection.requestAirdrop(keypair.publicKey,2*LAMPORTS_PER_SOL) ;
        console.log(`Success check out TX here: https://explorer.solana.com/tx/${transactionhash}?cluster=devnet`);
        

    } catch (error) {
        console.error(`Oops, something went wrong: ${error}`)
    }
})();