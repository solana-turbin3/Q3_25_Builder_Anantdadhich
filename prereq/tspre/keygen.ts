import {Keypair} from "@solana/web3.js"

let key=Keypair.generate(); 
console.log(`You've generated a new Solana wallet:${key.publicKey.toBase58()}`) 


console.log(`[${key.secretKey}]`)