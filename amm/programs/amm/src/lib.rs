
#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;


 mod instructions;
use instructions::*;
 mod state;
 mod error;
declare_id!("F2YJSaZaQW5M8ntHAd2ShgD9VzRGgEyVqnkvXmYURBc4");

#[program]
pub mod amm {
    use super::*;

    pub fn init(ctx: Context<Init>, seed:u64,fee:u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed,fee ,authority, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }
}

