import { Commitment, Connection, Keypair } from "@solana/web3.js";
import wallet from "../wallet.json";
import { createMint } from "@solana/spl-token";

const keypair=Keypair.fromSecretKey(new Uint8Array(wallet));
const confirmed:Commitment="confirmed";

const connection=new Connection("https://api.devnet.solana.com",confirmed);


async function splinit(){
    try {
        const mint=await createMint(
            connection,
            keypair,
            keypair.publicKey,
            keypair.publicKey,
            9

        )

        console.log("mint created success",mint.toBase58());
    } catch (error) {
        console.log("error in creating mint ",error);
    }
}

splinit();