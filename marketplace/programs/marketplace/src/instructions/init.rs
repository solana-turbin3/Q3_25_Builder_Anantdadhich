use anchor_lang::prelude::*;

use crate::{state::MarketPlace}; 
use crate::{errors::MarketplaceErrors};
#[derive(Accounts)]
pub struct InitMarketPlace<'info>  {
     #[account(mut )]
  pub admin:Signer<'info>   ,

    #[account(
        init,
        payer=admin, 
        space=8+ MarketPlace::INIT_SPACE,
        seeds=[b"marketpalce"],
        bump

    )]
  pub marketplace:Account<'info,MarketPlace>,
   #[account(
    seeds=[b"treasury",marketplace.key().as_ref()],
    bump
   )]
  pub treasury: SystemAccount<'info> ,
  pub system_program:Program<'info,System>
 
}

impl <'info> InitMarketPlace<'info> {
    pub fn   marketpalce(&mut self,fee_percentage:u8, bumps:InitMarketPlaceBumps)->Result<()> {
              
            require!( 
                fee_percentage <=100 ,
                MarketplaceErrors::InvalidFeePercentage
            );
        

          self.marketplace.set_inner(MarketPlace  {
            admin:self.admin.key() ,
            fee_percentage,
            bump:bumps.marketplace,
            treasury_bump:bumps.treasury

          });
        Ok(())
    }
}