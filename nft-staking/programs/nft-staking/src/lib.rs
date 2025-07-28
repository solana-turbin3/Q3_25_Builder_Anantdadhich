
#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*; 
pub mod state;

pub mod errors;
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

    pub fn init_stake(ctx:Context<InitStake>)->Result<()>{
      ctx.accounts.stake(&ctx.bumps)
    }

    pub fn init_unstake(ctx:Context<InitUnstake>)->Result<()>{
        ctx.accounts.unstake()
    }

    pub fn init_claim(ctx:Context<InitClaim>)->Result<()> {

        ctx.accounts.claim()
    }

    
}

