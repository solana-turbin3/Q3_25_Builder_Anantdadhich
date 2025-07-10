#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
use self::instructions::*;
declare_id!("ALbp7uEeYNcMeWpzDCisUoDuwVESgtYKnyqTkMpUAS5A");

#[program]
pub mod anchor_escrow {




    use super::*;

    pub fn make(ctx:Context<Maker>,seed:u64,recieve_amonut:u64)->Result<()>{
          ctx.accounts.maker(seed,recieve_amonut,&ctx.bumps)?;
           ctx.accounts.deposit(recieve_amonut)?; 
        Ok(())
    }
     
     pub fn   take(ctx:Context<Taker>)->Result<()> {
     ctx.accounts.deposit()?;
      ctx.accounts.release()?;
      ctx.accounts.close()?;
        Ok(())
     }
}


