use anchor_lang::prelude::*;

declare_id!("H9sR4GCub6wU637U8bw74Znab2zBxcJiiKjuFWt5hzSv");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
