use anchor_lang::{prelude::*, solana_program::stake::config::Config};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount}, token_interface::TokenInterface};
use crate::state::*;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct  Initialize<'info> {
    #[account(mut)]
   pub signer:Signer<'info> ,
    #[account(
        init,
        payer=signer,
        space=8+ AmmState::INIT_SPACE, 
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump,
    )]
   pub  config:Account<'info,AmmState>, 
    
    
    pub  mint_x:InterfaceAccount<'info,Mint>, 

    pub mint_y:InterfaceAccount<'info,Mint>, 


    pub mint_lp:InterfaceAccount<'info,Mint>, 
    pub vault_x:Account<'info,TokenAccount>, 
    pub vault_y:Account<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program:Program<'info,System>   

    }



