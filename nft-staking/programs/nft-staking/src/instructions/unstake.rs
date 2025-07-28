use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata,
    },
    token::{revoke, Mint, Revoke, Token, TokenAccount},
};
use crate::state::{StakeAccount, StateConfig, UserAccount};
use crate::errors::Error;
#[derive(Accounts)]
pub struct InitUnstake<'info>  {
#[account(mut)] 
pub signer:Signer<'info> ,
 #[account(
    seeds=[b"config"],
    bump
 )]
 pub config:Account<'info,StateConfig>,

 #[account(
    mut,
    seeds=[b"user_data",signer.key().as_ref()],
    bump=user_data.bump
 )]
 pub user_data:Account<'info,UserAccount> ,
 #[account(
    mut,
    close=signer,
    seeds=[b"stake_data",config.key().as_ref(),nft_mint.key().as_ref()],
    bump
 )]

 pub stake_data:Account<'info,StakeAccount>, 

 pub nft_mint:Account<'info,Mint>,
 #[account(
    seeds=[
        b"metadata",
        metadata_program.key().as_ref(),
        nft_mint.key().as_ref(),
        b"edition"
    ],
    seeds::program=metadata_program.key(),
    bump
)]
 pub nft_edition:Account<'info,MasterEditionAccount>,
 #[account(
    mut,
    associated_token::mint=nft_mint,
    associated_token::authority=signer
 )]
 pub user_ata_for_nft:Account<'info,TokenAccount> ,
 

 pub metadata_program:Program<'info,Metadata>,
 pub token_program:Program<'info,Token>,
 pub system_program:Program<'info,System>
}

impl<'info> InitUnstake<'info> {
    pub fn unstake(&mut self)->Result<()> {

        let days_elapsed=(Clock::get()?.unix_timestamp - self.stake_data.staked_at )/(60 * 60 *24); 

        require!(
            days_elapsed >= self.config.freeze_period.into(),
            Error::FreezePeriodNotOver
        )    ; 

        let staked_data_seeds=&[
            b"stake_data".as_ref(),
            self.config.to_account_info().key.as_ref() ,
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.stake_data.bump]
        ]; 

        let signer_seeds=&[&staked_data_seeds[..]]; 

        let metadata_program=&self.metadata_program.to_account_info(); 
        let delegate=&self.stake_data.to_account_info(); 
        let token_account=&self.user_ata_for_nft.to_account_info(); 
        let edition=&self.nft_edition.to_account_info();
        let mint=&self.nft_mint.to_account_info(); 
        let token_program=&self.token_program.to_account_info(); 


        ThawDelegatedAccountCpi::new(
            metadata_program, 
         ThawDelegatedAccountCpiAccounts{
            delegate,
            token_account,
            edition,
            mint,
            token_program
         },
        ).invoke_signed(signer_seeds)?;

        let accounts=Revoke{
            source:self.user_ata_for_nft.to_account_info(),
            authority:self.signer.to_account_info() 
        };

        let ctx=CpiContext::new(self.token_program.to_account_info(),accounts);
        revoke(ctx)?;
         //update the userdata 

         self.user_data.staked_count -= 1;
          self.user_data.points += days_elapsed as u32 * self.config.points_per_stake as u32;
     
        Ok(())
    }
}