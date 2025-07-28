use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::state::{StakeAccount, StateConfig, UserAccount};
use crate::errors::Error;
#[derive(Accounts)]
pub struct InitStake<'info> {
   #[account(mut)]
   pub signer:Signer<'info> ,
  #[account(                //program owned account that holds the golbal setting of pda 
    seeds=[b"config"],       //its pda
    bump         
  )]
   pub config:Account<'info,StateConfig>,
   #[account(
    mut,                      //user data is mutable because we update the stakecount
    seeds=[b"user_data",signer.key().as_ref()],    //pda dervied from user data and the user public key 
    bump=user_data.bump           
   )]
   pub user_data:Account<'info,UserAccount>,
    #[account(
        init,
        payer=signer,
        space=8 + StakeAccount::INIT_SPACE,
        seeds=[b"stake_data",config.key().as_ref(),nft_mint.key().as_ref()],
        bump
    )]
   pub stake_data:Account<'info,StakeAccount>,

    pub nft_mint:Account<'info,Mint>,
     #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program=metadata_program.key(),
        bump, 
        constraint=nft_metadata.collection.as_ref().unwrap().key.as_ref()==nft_collection.key().as_ref(),
        constraint=nft_metadata.collection.as_ref().unwrap().verified

     )]
    pub nft_metadata:Account<'info,MetadataAccount>,
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
    pub nft_edition:Account<'info,Mint>,   //this provr that nft is unique token not fungible one
   
    pub nft_collection:Account<'info,MasterEditionAccount>,
     #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=signer
     )]
    pub user_ata_for_nft:Account<'info,TokenAccount>,  //it holds our nft 

    pub metadata_program:Program<'info,Metadata>,

    pub token_program:Program<'info,Token>,

    pub system_program:Program<'info,System>

}

impl <'info> InitStake <'info> {
    pub fn stake(&mut self,bumps:&InitStakeBumps)->Result<()> {
           // Check staked account  here we are checking that user already staked the maximum amount of staked
            require!(self.user_data.staked_count < self.config.max_stake ,
              Error::MaxStakesReached
            );
            
 
          //lets delgate the account
        let accounts=Approve{
            to:self.user_ata_for_nft.to_account_info(),
            delegate:self.stake_data.to_account_info(), 
            authority:self.signer.to_account_info() 
        }; 
        let ctx=CpiContext::new(self.token_program.to_account_info(),accounts);
        approve(ctx, 1)?;

        //freeze nft token account 

        let stake_data_seeds=&[
            b"stake_data".as_ref(),
            self.config.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[bumps.stake_data]
        ]; 

        let signer_seeds=&[&stake_data_seeds[..]];

        let metadata_program=&self.metadata_program.to_account_info(); 

        let delegate=&self.stake_data.to_account_info(); 

        let token_account=&self.user_ata_for_nft.to_account_info(); 

        let edition=&self.nft_edition.to_account_info();

        let mint=&self.nft_mint.to_account_info();

        let token_program=&self.token_program.to_account_info();

        FreezeDelegatedAccountCpi::new(metadata_program, FreezeDelegatedAccountCpiAccounts{
            delegate,
            token_account,
            edition,
            mint,
            token_program
        },
    ).invoke_signed(signer_seeds)?;


    self.stake_data.set_inner(StakeAccount{
        owner:self.user_data.key(),
        mint:self.nft_mint.key(),
        staked_at:Clock::get()?.unix_timestamp,
        bump:bumps.stake_data
        
    });

        

            
        Ok(())
    }
}