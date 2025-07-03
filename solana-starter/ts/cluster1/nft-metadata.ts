import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "../wallet.json" 
import { createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";


const umi=createUmi("https://api.devnet.solana.com") 
const keypair=umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet))
const signer=createSignerFromKeypair(umi,keypair) 
umi.use(irysUploader())
umi.use(signerIdentity(signer)) ;



//https://gateway.irys.xyz/HkkhNtGQLXLKWx1aaXQT6BZc2PhpAH29CJp6PHsVV3Lu 

(async()=>{
      const image="https://gateway.irys.xyz/HkkhNtGQLXLKWx1aaXQT6BZc2PhpAH29CJp6PHsVV3Lu" ;

     
       try {
        const metadata = {
            name: "GENERUG",
            symbol: "RUG",
            description: "This is an random image uploaded by the adtech in the Tubin3 rug day",
            image: image,
            attributes: [
                {trait_type: 'random', value: 'yes'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        }
        
        const myurii=await umi.uploader.uploadJson(metadata).catch((err)=>{
            throw new Error(err)
        })
        console.log("the my uri for metadata",myurii  )  
       } catch (error) {
         console.log("error ",error) 
       }
})()