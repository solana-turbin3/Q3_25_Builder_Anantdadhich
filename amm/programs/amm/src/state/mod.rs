use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmmState {
    pub seed:u64, 
    pub fee:u16,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey, 
    pub mint_lp_bump:u8, 
    pub authority:Option<Pubkey>,
    pub locked:bool,
    pub bump:u8
}