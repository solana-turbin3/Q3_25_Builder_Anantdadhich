use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub staked_count:u8 ,
    pub points:u32, 
    pub bump:u8 
} 
