import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "../wallet.json"
import { createSignerFromKeypair, generateSigner, percentAmount, signerIdentity } from "@metaplex-foundation/umi";
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import base58 from "bs58";


const umi=createUmi("https://api.devnet.solana.com") 
const keypair=umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet)) 
const signer=createSignerFromKeypair(umi,keypair) ;

umi.use(mplTokenMetadata())
umi.use(signerIdentity(signer)) ;

const mint=generateSigner(umi) ;

(async ()=>{
    let tx=await createNft(
        umi,{
            mint,
            name:"ADTECH", 
            symbol:"$tech",
            uri:"https://gateway.irys.xyz/9DvTeHwhd8Kd4KmnmeXZyhr6jT54EKUvkpYMduYnSVhH",
            sellerFeeBasisPoints:percentAmount(40)
            
        }
    )
    let result=await tx.sendAndConfirm(umi) 
    const signature=base58.encode(result.signature) ;
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)
    console.log("Mint Address: ", mint.publicKey);
})()


