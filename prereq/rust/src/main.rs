
#[cfg(test)]
mod tests {
   use solana_sdk::{  blake3::hash, instruction::{AccountMeta, Instruction}, message::Message, signature::{ read_keypair_file, Keypair, Signer}, transaction::Transaction};
   use solana_client::{ nonblocking::rpc_client, rpc_client::RpcClient};
   use solana_program::{pubkey:: Pubkey,system_instruction::transfer};
   use std::str::FromStr;
   use solana_sdk::system_program;
   #[test]
   
   fn keygen(){
      let kp=Keypair::new(); 
      print!("You generated a wallet {}",kp.pubkey().to_string());
      print!("");
      print!("to save your wallet copy and paste the following into json filew  ");
      print!("{:?}",kp.to_bytes());
   }

    
   fn airdrop(){
   const RPC_URL:&str="https://api.devnet.solana.com";
    let keypair=read_keypair_file("dev-wallet.json").expect("couldnt find wallet file"); 
    let client=RpcClient::new(RPC_URL);
    //now we are going to airdrop the 2 devent sol tokens  
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(sig)=>{
         print!("Success check your transaction here");
         println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
        }Err(err)=>{
         print!("Airdrop failed{}",err);
        }
    }


   }


 
   
   fn transfersol(){
    const RPC_URL:&str="https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    let keypair=read_keypair_file("dev-wallet.json").expect("couldnt find wallet"); 
    let pubkey=keypair.pubkey();
    let messgae_bytes=b"i verify my solana keypair";
    let sig=keypair.sign_message(messgae_bytes); 
    let signed_hashed=hash(sig.as_ref()); 
   
   
    //now we verify the signature using the public key 
    match  sig.verify(&pubkey.to_bytes(), &signed_hashed.to_bytes()) {
        true => print!("signature verified"),
        false => print!("signature not verifief")
    }

    let to_pubkey=Pubkey::from_str("87eaezi5Nou5d5MFH2DStENzWZ6iHNroDHZSbSca4RDu").unwrap(); 

    let rpc_client=RpcClient::new(RPC_URL); 
    let balance=rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance"); 

    let recentblockhash=rpc_client.get_latest_blockhash().expect("failed to get latest blockhash");

    let message=Message::new_with_blockhash(&[transfer(&keypair.pubkey(),&to_pubkey,balance)],
   Some(&keypair.pubkey()),
   &recentblockhash
);
  
  let fees=rpc_client.get_fee_for_message(&message).expect("failed to get fee calculator");

    //build recent blockhash 
 
    //now we create the transaction  
    let transaction=Transaction::new_signed_with_payer(
      &[transfer(&keypair.pubkey(),&to_pubkey,balance-fees)],
      Some(&keypair.pubkey()),
      &vec![&keypair],
      recentblockhash
    );
     let signature=rpc_client.send_and_confirm_transaction(&transaction).expect("failed to send transaction"); 
     println!(

      "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet"
      ,signature
      );

   }



   fn submit_rs(){
      const RPC_URL:&str="https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
      let rpc_client=RpcClient::new(RPC_URL); 

      let signer=read_keypair_file("dev-wallet.json").expect("not find wallet"); 

      let mint=Keypair::new() ;

      let turbin3_prereq_program=Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();

      let collection =Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

      let mpl_core_program =Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();

      let system_program = system_program::id();


      let signer_publickey=signer.pubkey();
      let seeds=&[b"prereqs",signer_publickey.as_ref()]; 

      let (prereq_pda,_bump)=Pubkey::find_program_address(seeds, &turbin3_prereq_program); 
      let data = vec![77, 124, 82, 163, 21, 133, 181, 206];


      let authority_seeds = &[b"collection", collection.as_ref()];
      let (authority, _) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

      let accounts = vec![
AccountMeta::new(signer.pubkey(), true),
AccountMeta::new(prereq_pda, false), 
AccountMeta::new(mint.pubkey(), true),
AccountMeta::new(collection, false), 
AccountMeta::new_readonly(authority, false), 
AccountMeta::new_readonly(mpl_core_program, false),
AccountMeta::new_readonly(system_program, false)
      ];  


      let blockhash=rpc_client.get_latest_blockhash().expect("failed to get recent blockhash");


      let instruction=Instruction{
         program_id:turbin3_prereq_program,
         accounts,
         data

      };


      let transaction=Transaction::new_signed_with_payer(
         &[instruction],
         Some(&signer.pubkey()),
         &[&signer,&mint],
         blockhash
      );

      let signature=rpc_client.send_and_confirm_transaction(&transaction).expect("transaction failed");

      println!(
         "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet"
         ,
         signature
         );



}

}