use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::state::StateConfig;

#[derive(Accounts)]
pub struct  InitConfig<'info> {
    #[account(mut)]
  pub signer:Signer<'info> ,
   #[account(
    init, 
    payer=signer,
    space=8+StateConfig::INIT_SPACE,
    seeds=[b"config"],
    bump
   )]
  pub config:Account<'info,StateConfig>  ,
  #[account(
    init,
    payer=signer,
    mint::decimals=6,
    mint::authority=config, 
    mint::token_program=token_program, 
    seeds=[b"rewards_mint",config.key().as_ref()],
    bump
  )]
  pub rewards_mint:InterfaceAccount<'info,Mint>,

  pub token_program:Interface<'info,TokenInterface>,

  pub system_program:Program<'info,System>

}

impl<'info> InitConfig <'info> {
  pub fn init_config(&mut self,points_per_stake:u8,max_stake:u8,freeze_period:u32,bumps:&InitConfigBumps)->Result<()> {
         self.config.set_inner(StateConfig{
          points_per_stake,
          max_stake,
          freeze_period,
          rewards_mint_bump:bumps.rewards_mint,
          bump:bumps.config
         });
    Ok(())
  }
}