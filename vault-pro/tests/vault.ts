import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { BN } from "@coral-xyz/anchor";
describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vault as Program<Vault>;

  const provider=anchor.getProvider();
  const connection=provider.connection ;

  const confirm=async(signature:string):Promise<string> =>{
    const block=await connection.getLatestBlockhash();
    await connection.confirmTransaction( {
      signature,
      ...block , }
    )
    return signature
  }

   const log=async (signature:string):Promise<string> =>{
    console.log(
      `your transaction is here https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    ) 
    return signature 
   }   
   it("is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("deposit",async ()=>{
      const tx=await program.methods.deposit(new BN(1_000_000))
      .accounts({
        signer:provider.publicKey!
      })
      .rpc().then(confirm).then(log)
  })

  it("withdraw",async ()=>{
    const tx=await program.methods.withdraw(new BN(1_000_000)).accounts({
      signer:provider.publicKey!
    }).rpc().then(confirm).then(log)
  })
   
  
   


 
});
