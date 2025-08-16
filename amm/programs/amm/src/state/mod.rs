use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed:u64, 
    pub fee:u16,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey, 
    pub lp_bump:u8, 
    pub authority:Option<Pubkey>,
    pub locked:bool,
    pub config_bump:u8
}