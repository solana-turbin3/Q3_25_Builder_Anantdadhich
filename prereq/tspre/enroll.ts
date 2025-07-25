import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Idl } from "@coral-xyz/anchor"
import { IDL } from "./programs/Turbin3 _prereq";
import wallet from "./Turbin3-wallet.json"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";


const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const keypair=Keypair.fromSecretKey(new Uint8Array(wallet)) 

const connection=new Connection("https://api.devnet.solana.com") ;

const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"});


const program:Program<Idl>=new Program(IDL as Idl , provider)


const account_seeds=[Buffer.from("prereqs"),keypair.publicKey.toBuffer()]

const [account_key,_bump]=PublicKey.findProgramAddressSync(
    account_seeds,
    program.programId
);
const mintCollection = new
PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

const minttx=Keypair.generate();

const [authority_key, _bump_authority] = PublicKey.findProgramAddressSync(
    [
        Buffer.from("collection"),
        mintCollection.toBuffer()
    ],
    program.programId
);





/*
//intitialize 
(async()=>{
    try {
        const txhash=await program.methods.initialize("Anantdadhich").accountsPartial({
            user:keypair.publicKey,
            account:account_key,
            system_program:SYSTEM_PROGRAM_ID
        }).signers([keypair]).rpc()
        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${txhash}?cluster=devnet`)

    } catch (error) {
        console.error(`Oops, something went wrong: ${error}`);
    }
})();
*/
(async()=>{
try {
    const txhash=await program.methods.submitTs().accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: minttx.publicKey,
        collection: mintCollection,
        authority: authority_key,
        mpl_core_program:MPL_CORE_PROGRAM_ID,
        system_program:SYSTEM_PROGRAM_ID
    }).signers([keypair,minttx]).rpc();

    console.log(`Success! submit Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
} catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
}
})()