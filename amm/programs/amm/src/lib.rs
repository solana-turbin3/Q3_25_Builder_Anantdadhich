
#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;


 mod instructions;
use instructions::*;
 mod state;
 mod error;
declare_id!("DgSehEwjsxkk7njabexKFhFQxBkngNyFaeDmRfKnsYsV");

#[program]
pub mod amm {
    use super::*;

    pub fn init(ctx: Context<Init>, fee: u16, seed: u64, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed,fee, authority, ctx.bumps)
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

