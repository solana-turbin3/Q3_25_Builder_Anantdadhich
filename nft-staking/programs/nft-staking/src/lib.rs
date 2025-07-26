
#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*; 
pub mod state;

pub mod error;
declare_id!("H9sR4GCub6wU637U8bw74Znab2zBxcJiiKjuFWt5hzSv");

#[program]
pub mod nft_staking {
   
    use super::*;

    pub fn init_config_state(ctx: Context<InitConfig>,points_per_stake:u8 ,max_stake:u8,freeze_period:u32) -> Result<()> {
          ctx.accounts.init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
       
    }

    pub fn init_useraccount(ctx:Context<InitUser>)->Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }
}

