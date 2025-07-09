use anchor_lang::prelude::*; 

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
 pub seeds:u64 ,
 pub maker:Pubkey,
 pub mint_a:Pubkey,
 pub mint_b:Pubkey,
 pub taker:u64,
 pub bump:u8
}