
#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub use instructions::*;

declare_id!("3c4nU4FYxbk7shmt9YjArmoaT997MC1dB1FnFvURSHmS");

#[program]
pub mod anchor_dice {
    use super::*;

   
}


