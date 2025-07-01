import {
    Transaction,
    SystemProgram,
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
  } from "@solana/web3.js";
  
  import Wallet from "./Turbin3-wallet.json";

  const from = Keypair.fromSecretKey(new Uint8Array(Wallet));
  
  const to = new PublicKey("87eaezi5Nou5d5MFH2DStENzWZ6iHNroDHZSbSca4RDu");
  
  const connection = new Connection("https://api.devnet.solana.com");
  
  (async () => {
    try {
      const balance = await connection.getBalance(from.publicKey);
  
      const transaction = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance,
        })
      );
  
      transaction.recentBlockhash = (
        await connection.getLatestBlockhash("confirmed")
      ).blockhash;
  
      transaction.feePayer = from.publicKey;
  
      const fee =
        (
          await connection.getFeeForMessage(
            transaction.compileMessage(),
            "confirmed"
          )
        )?.value || 0;
  
      if (balance <= fee) {
        console.error("Insufficient balance!!!");
      }
  
      transaction.instructions.pop();
  
      transaction.add(
        SystemProgram.transfer({
          fromPubkey: from.publicKey,
          toPubkey: to,
          lamports: balance - fee,
        })
      );
  
      const signature = await sendAndConfirmTransaction(connection, transaction, [
        from,
      ]);
  
      console.log(
        `Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`
      );
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  })();