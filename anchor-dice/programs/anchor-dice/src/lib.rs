
#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;
pub use instructions::*;

declare_id!("3c4nU4FYxbk7shmt9YjArmoaT997MC1dB1FnFvURSHmS");

#[program]
pub mod anchor_dice {


    use super::*;
    pub fn initialize(ctx: Context<Init>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, roll: u8, amount: u64) -> Result<()> {
        ctx.accounts.create_bet(&ctx.bumps, seed, roll, amount)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx:Context<Resolvebet>,sig:Vec<u8>) -> Result<()> {
     ctx.accounts.verify_ed25519_signature(&sig)?;
     ctx.accounts.resolve_bet(&ctx.bumps, &sig)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refundbet(&ctx.bumps)
    }
   
}


