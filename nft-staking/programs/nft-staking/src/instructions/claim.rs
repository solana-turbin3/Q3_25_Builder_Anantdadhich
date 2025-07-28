use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount},
    token_interface::{mint_to, Mint, MintTo},
};


use crate::state::{StateConfig, UserAccount};

#[derive(Accounts)]
pub struct  InitClaim<'info> {
 #[account(mut)]
 pub signer:Signer<'info> , 
  #[account(
    seeds=[b"config"],
    bump=config.bump
  )]
 pub config:Account<'info,StateConfig>  ,
  #[account(
    mut,
    seeds=[b"user_data",signer.key.as_ref()],
    bump=user_data.bump
  )]
 pub user_data:Account<'info,UserAccount>,
  #[account(
    seeds=[b"reward_mint",config.key().as_ref()],
    bump=config.rewards_mint_bump
  )]
 pub reward_mint:InterfaceAccount<'info,Mint> , 
  #[account(
    init_if_needed,
    payer=signer,
    associated_token::mint=reward_mint, 
    associated_token::authority=signer
  )]
 pub user_ata_for_reward:Account<'info, TokenAccount> ,

 pub token_program:Program<'info,Token> ,

 pub associated_token_program:Program<'info,AssociatedToken> ,

 pub system_program:Program<'info,System> 

}


impl <'info> InitClaim<'info> {
    pub fn claim(&mut self)->Result<()> {
           
         let amount=self.user_data.points as u64 *10u64.pow(self.reward_mint.decimals.into()); 
         let config_seeds=&[b"config".as_ref(),&[self.config.bump]]; 
         let signer_seeds=&[&config_seeds[..]];


         let accounts=MintTo{
            mint:self.reward_mint.to_account_info() ,
            to:self.user_ata_for_reward.to_account_info(), 
            authority:self.config.to_account_info() 
         };  

         let ctx=CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

         mint_to(ctx, amount)?;
          
          self.user_data.points =0 ;
        Ok(())
    }
}