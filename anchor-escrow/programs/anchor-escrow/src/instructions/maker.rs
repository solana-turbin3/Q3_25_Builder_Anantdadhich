use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

use crate::state::EscrowState;


#[derive(Accounts)]
#[instruction(seeds:u8)]
pub struct Maker<'info> {
     #[account(
        mut
     )]
    pub maker:Signer<'info> , 

    pub mint_a:InterfaceAccount<'info,Mint> ,
    pub mint_b:InterfaceAccount<'info,Mint> ,

    #[account(
        mut ,
        associated_token::mint=mint_a ,
        associated_token::authority=maker 
    )]
    pub maker_ata_a:InterfaceAccount<'info,TokenAccount >, 
    #[account(
        init,
        payer=maker,
        space=8 +EscrowState::INIT_SPACE,
        seeds=[b"escrow",maker.key.as_ref(),seeds.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow:Account<'info,EscrowState>, 

    #[account(
        init,
        payer=maker, 
        associated_token::mint=mint_a,
        associated_token::authority=escrow
          
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface> ,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System> 
}


impl<'info> Maker <'info> {
     pub fn maker(&mut self ,seed:u64,  recive_amount:u64 , bumps:&MakerBumps)->Result<()>{
             self.escrow.set_inner(EscrowState  {
                seed, 
                maker:self.maker.key(),
                mint_a:self.mint_a.key(),
                mint_b:self.mint_b.key(),
                taker:recive_amount ,
                bump:bumps.escrow
             });
        Ok( ())
     }

     pub fn deposit(&mut self,amount:u64) -> Result<()> {
            let cpi_program=self.token_program.to_account_info(); 

          let cpi_tranfer=TransferChecked {
            from:self.maker_ata_a.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.vault.to_account_info() ,
            authority:self.maker.to_account_info()
          } ; 

          let cpi_context=CpiContext::new(cpi_program,cpi_tranfer);
          
          transfer_checked(cpi_context, amount, self.mint_a.decimals)?;

        Ok(())
     }
     
}