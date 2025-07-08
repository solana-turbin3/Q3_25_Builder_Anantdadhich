#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*,  system_program::{transfer, Transfer}};


declare_id!("6AFW1Yorfnh2DCVkdNuhwAKkJx6cug83Dg4mpbpdPhrB");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct  VaultState {
    pub vault_bump:u8,
    pub state_bump:u8
}


#[derive(Accounts)]
pub struct Initialize<'info> {
     #[account(
        mut
     )]   
    pub signer:Signer<'info> ,
    #[account(
        init,
        payer=signer,
        space=8 + VaultState::INIT_SPACE ,
        seeds=[signer.key().as_ref()],
        bump
       
    )]
    pub vault_state:Account<'info,VaultState> , 
    
    #[account(
        seeds=[vault_state.key().as_ref()],
        bump
    )]
    pub vaultt:SystemAccount<'info>,  

    pub system_program:Program<'info,System>

         

}

impl<'info> Initialize<'info> {
    pub fn   initialize(&mut self , bumps:InitializeBumps)->Result<()> {
         self.vault_state.vault_bump=bumps.vaultt;
         self.vault_state.state_bump=bumps.vault_state;
        Ok(())
    }
}

pub fn initialize(ctx:Context<Initialize>)->Result<()>{
     ctx.accounts.vault_state.vault_bump=ctx.bumps.vaultt;
     ctx.accounts.vault_state.state_bump=ctx.bumps.vault_state; 
    Ok(())
}




#[derive(Accounts)]
pub struct Payment<'info> {
   #[account(
    mut
   )]
  pub signer:Signer<'info> ,
  #[
    account(
        mut,
        seeds=[b"seeds",signer.key().as_ref()],
        bump=vault_state.state_bump
    )
  ]
  pub vault_state:Account<'info,VaultState> ,
   #[account(
    mut,
    seeds=[vault_state.key().as_ref()],
    bump=vault_state.vault_bump
   )] 
  pub vault:SystemAccount<'info> ,

  pub system_program:Program<'info,System> 
}


impl<'info> Payment<'info> {

    pub fn deposit( &mut self , amount:u64)->Result<()> {
        let system_programs=self.system_program.to_account_info();

         let accounts=Transfer{
            from:self.signer.to_account_info(),
            to:self.vault.to_account_info()
         };

          let cpi_context=CpiContext::new(system_programs, accounts);

          transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self , amount:u64)->Result<()> {
      let system_programs=self.system_program.to_account_info() ;

      let accounts=Transfer {
        from:self.vault.to_account_info(),
        to:self.signer.to_account_info() 
      }; 


    

       let seeds=&[
         b"state",
         self.vault_state.to_account_info().key.as_ref(),
         &[
            self.vault_state.vault_bump
         ]
       ]; 

       let signer_seeds = &[&seeds[..]];

       let cpi_context=CpiContext::new_with_signer(system_programs, accounts, signer_seeds) ;

       transfer(cpi_context, amount)?;
        Ok(())
    }
} 