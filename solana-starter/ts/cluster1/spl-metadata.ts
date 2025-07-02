

import wallet from "../wallet.json"
import {  createSignerFromKeypair, publicKey, signerIdentity } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createMetadataAccountV3, CreateMetadataAccountV3InstructionAccounts, CreateMetadataAccountV3InstructionArgs, CreateMetadataAccountV3InstructionData, DataV2Args } from "@metaplex-foundation/mpl-token-metadata";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";



const mint =publicKey("FoGvzPbCYLrkwbhLDM3UmzTx6SjFYweBtyVntwmtE2H4") 

 const umi=createUmi("https://api.devnet.solana.com");
 const keypair=umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet)) 
 const signer=createSignerFromKeypair(umi,keypair); 

 umi.use(signerIdentity(createSignerFromKeypair(umi,keypair))) ;

 (async ()=>   {
     try {
           let accounts:CreateMetadataAccountV3InstructionAccounts={
            mint,
            mintAuthority:signer
           }
          
           let data:DataV2Args={
           name:"tehie",
           symbol:"$tec",
           uri:"https://media.gettyimages.com/id/155419717/photo/peacock-feather.jpg?s=1024x1024&w=gi&k=20&c=2g-ry_XRqZwdTtFljutguf_ozjWy-xOqQFG7lnxc76M=",
           sellerFeeBasisPoints:0,
           collection:null,
           creators:null,
           uses:null
           
           }

          let args:CreateMetadataAccountV3InstructionArgs={
               data,
               isMutable:true,
               collectionDetails:null
          }

          let transaction=createMetadataAccountV3(
            umi,{
              ...accounts,
              ...args
            }
          )
          let res=await transaction.sendAndConfirm(umi)
          console.log(`the transaction sig `,bs58.encode(res.signature))  ;   //4nrZcxcYFMi19DxpmEeWM4tmaSoN3SejpZcmfRxRpL4b9XvWxhNFtmGeStDYVxGLWmBxQhpW4ndoDb3z86Qx3hsy


     } catch (error) {
        console.log("the error in the splmetadata",error)
     }
 })()

