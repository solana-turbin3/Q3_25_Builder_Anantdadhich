
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "../wallet.json" 
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

const umi=createUmi("https://api.devnet.solana.com") 
const keypair=umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet)) 
const signer=createSignerFromKeypair(umi,keypair) ;

umi.use(irysUploader())
umi.use(signerIdentity(signer)) ;


(async()=>{
     try {
        const image=await readFile("");

        const file=createGenericFile(image,'generug.png',{
          tags:[
              {
                  name:"Content-Type",value:"image/png"
               }
          ]
        }); 
  
         const [myuri]=await umi.uploader.upload([file]) ;
         console.log("my urib ",myuri)
     } catch (error) {
        console.log("error in ",error)
     }

       

})()