use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer:Signer<'info> ,
    #[account(
        init,
        payer=signer,
        space=8 + UserAccount::INIT_SPACE,
        seeds=[b"user_data",signer.key().as_ref()],
        bump
    )]
    pub user_data:Account<'info,UserAccount>,

    pub system_program:Program<'info,System>
}

impl<'info> InitUser<'info> {
    pub fn init_user(&mut self,bumps:&InitUserBumps)->Result<()>{
          self.user_data.set_inner(UserAccount{
          points:0,
          staked_count:0, 
          bump:bumps.user_data
          });
        Ok(())
    }
}