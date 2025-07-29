#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub use instructions::*;
pub mod errors;
declare_id!("F89MvWFXs2uPXV3WZ5kfvL3r2ns6pWA6J23fUthG36KQ");

#[program]
pub mod marketplace {
    use super::*;
     

     pub fn init_marketplace(ctx:Context<InitMarketPlace>,fee_percentage:u8)->Result<()>{
        ctx.accounts.marketpalce(fee_percentage, ctx.bumps)?;
        Ok(())
     }

     pub fn init_list_nft(ctx:Context<InitLIstNFT>,price:u64)->Result<()>{
       ctx.accounts.initalise_listing(price,ctx.bumps)?;
       ctx.accounts.transfer()
       
     }
     
     pub fn init_delist(ctx:Context<InitDelist>)->Result<()>{
        ctx.accounts.transfer_back_nft()
     }

     pub fn init_purchasenft(ctx:Context<InitPurchaseNft>)->Result<()>{
        ctx.accounts.transfer_nft()?;
        ctx.accounts.transfer_sol()?;
        ctx.accounts.delist_nft()
     }
    
}

