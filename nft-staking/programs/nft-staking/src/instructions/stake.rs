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

#[derive(Accounts)]
pub struct InitStake<'info> {
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

    pub nft_metadata:Account<'info,MetadataAccount>,

    pub nft_edition:Account<'info,Mint>,

    pub nft_collection:Account<'info,MasterEditionAccount>,

    pub user_ata_for_nft:Account<'info,TokenAccount>,

    pub metadata_program:Program<'info,Metadata>,

    pub token_program:Program<'info,Token>,

    pub system_program:Program<'info,System>

}