use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::CloseAccount,
    token_interface::{
        close_account, transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::state::EscrowState;

#[derive(Accounts)]
pub struct Taker<'info> {
#[account(mut)]
pub taker:Signer<'info> ,

pub mint_a:InterfaceAccount<'info,Mint>,

pub mint_b:InterfaceAccount<'info,Mint>, 

#[account(
    mut,
    associated_token::mint=escrow.mint_a,
    associated_token::authority=taker
)]
pub taker_ata_a:InterfaceAccount<'info,TokenAccount>,
#[account(
    mut,
    associated_token::mint=escrow.mint_b,
    associated_token::authority=taker
)]

pub taker_ata_b:InterfaceAccount<'info,TokenAccount>,

#[account(
    mut,
    associated_token::mint=escrow.mint_a,
    associated_token::authority=escrow.maker
)]
pub maker_ata_b:InterfaceAccount<'info,TokenAccount> ,

#[account(
    mut, 
    seeds=[b"escrow",escrow.maker.as_ref(),escrow.seed.to_le_bytes().as_ref()], 
    bump=escrow.bump
)]
pub escrow:Account<'info,EscrowState> ,

#[account(
    mut,
    associated_token::mint=escrow.mint_a,
    associated_token::authority=escrow
  
)]
pub vault:InterfaceAccount<'info,TokenAccount> ,


pub token_program:Interface<'info,TokenInterface> ,

pub associated_token_program:Program<'info,AssociatedToken>,

pub system_program:Program<'info,System>

}

impl <'info> Taker <'info>   {
     
     pub fn deposit(&mut self)->Result<()> {
          let tranfer_program=self.token_program.to_account_info(); 

          let cpi_transfer=TransferChecked{
            from:self.taker_ata_b.to_account_info(),
            mint:self.mint_b.to_account_info(),
            to:self.maker_ata_b.to_account_info(), 
            authority:self.taker.to_account_info()
          }; 

        let cpi_ctx=CpiContext::new(tranfer_program,cpi_transfer); 
        transfer_checked(cpi_ctx, self.escrow.taker, self.mint_b.decimals)?;


        Ok(())
     }

     pub fn release(&mut self )->Result<()> {
       let transfer_program=self.token_program.to_account_info(); 


       let cpi_transfer=TransferChecked {
        from:self.vault.to_account_info(),
       mint:self.mint_a.to_account_info() ,
       to:self.taker_ata_a.to_account_info(), 
       authority:self.escrow.to_account_info()
       }; 
       

       let seed_bytes=self.escrow.seed.to_be_bytes(); 
        
        let seeds= &[
            b"escrow", 
            self.escrow.maker.as_ref(),
            seed_bytes.as_ref(),
            &[self.escrow.bump]
        ]; 

        let signer_seeds= [&seeds[..]];


       let cpi_context=CpiContext::new_with_signer(transfer_program, cpi_transfer, &signer_seeds) ;

         transfer_checked(cpi_context, self.escrow.taker, self.mint_a.decimals)?;
 
        Ok(())
     }
     

      pub fn close(&mut self)->Result<()> {
        let transfer_program=self.token_program.to_account_info(); 


        let cpi_accounts=CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.taker.to_account_info(),
            authority:self.escrow.to_account_info()
        }; 
        let seed_bytes=self.escrow.seed.to_be_bytes(); 
        
        let seeds= &[
            b"escrow", 
            self.escrow.maker.as_ref(),
            seed_bytes.as_ref(),
            &[self.escrow.bump]
        ]; 

        let signer_seeds= [&seeds[..]];
        
        let cpi_ctx=CpiContext::new_with_signer(transfer_program, cpi_accounts, &signer_seeds);
        close_account(cpi_ctx)?;
         


        Ok(())
      }

}