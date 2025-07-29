use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct  Listing {
    //seller puublic key who list nft
    pub seller:Pubkey,

    pub  mint:Pubkey,
   //price of the 
    pub price:u64 ,
      ///pda bump 
    pub bump:u8 ,
    //listng is currently active or not
    pub is_active:bool 
}