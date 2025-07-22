use anchor_lang::prelude::*;


pub mod instructions;
pub mod state;
declare_id!("DgSehEwjsxkk7njabexKFhFQxBkngNyFaeDmRfKnsYsV");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
