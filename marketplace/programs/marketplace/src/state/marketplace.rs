use anchor_lang::prelude::*; 

#[account]
#[derive(InitSpace)]
pub struct  MarketPlace{
    pub admin:Pubkey ,
     //how much fee oon each sle taken from sale price
    pub fee_percentage:u8 ,

    pub bump:u8,   
    //pda bump seed for treasury  account
    pub treasury_bump:u8 
}