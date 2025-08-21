
import {AnchorProvider, Program} from "@coral-xyz/anchor"
import {Cluster, PublicKey} from "@solana/web3.js"
import {Amm} from "../components/amm"
import idl from "../components/amm.json"


 



export const PROJECT_PROGRAM_ID=new PublicKey(idl.address);

export function getAmmProgram(provider:AnchorProvider) {
    return new Program(idl as Amm,provider)
}




export function getAmmProgramID(cluster:Cluster){
    switch (cluster) {
        case "devnet":
        case "testnet" :

        return new PublicKey("F2YJSaZaQW5M8ntHAd2ShgD9VzRGgEyVqnkvXmYURBc4");
        case "mainnet-beta" :
            default :
            return PROJECT_PROGRAM_ID
    }
}

