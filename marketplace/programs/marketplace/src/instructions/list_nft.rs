

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{transfer_checked, Token, TransferChecked},
    token_interface::{Mint, TokenAccount},
};
use crate::{errors::MarketplaceErrors};

use crate::state::{Listing, MarketPlace};

#[derive(Accounts)]

pub struct InitLIstNFT<'info> {
     #[account(mut)]
    pub seller:Signer<'info> , 

    pub nft:InterfaceAccount<'info,Mint> ,
      #[account(
        seeds=[b"marketplace"],
        bump=marketplace.bump
      )]
    pub  marketplace:Account<'info,MarketPlace> ,
    #[account(
        init,
        payer=seller ,
        space=8+Listing::INIT_SPACE,
        seeds=[
            b"listing",
            marketplace.key().as_ref(),
            seller.key().as_ref(),
            nft.key().as_ref()
        ],
        bump
    )]
    pub listing:Account<'info,Listing> , 
    
      #[account(
        init,
        payer=seller,
        associated_token::mint=nft,
        associated_token::authority=listing
      )]
    pub listing_token_account:InterfaceAccount<'info,TokenAccount>, 
    
    #[account(
        mut,
        associated_token::mint=nft,
        associated_token::authority=seller,
        constraint =seller_token_account.owner == seller.key()
    )]
    pub seller_token_account:InterfaceAccount<'info,TokenAccount>,
    
    pub collection_mint:InterfaceAccount<'info,Mint>, 
    
     #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft.key().as_ref()
        ],
        seeds::program=metadata_program.key(),
        bump,
        constraint=metadata.collection.as_ref().unwrap().key.as_ref() ==collection_mint.key().as_ref(),
        constraint=metadata.collection.as_ref().unwrap().verified ==true 

     )]
    pub metadata:Account<'info,MetadataAccount>, 
     
     #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft.key().as_ref(),
            b"edition"
        ],
        seeds::program=metadata_program.key(),
        bump
     )]
     
    pub master_edition:Account<'info,MasterEditionAccount>,

    pub token_program:Program<'info,Token>,
    pub metadata_program:Program<'info,Metadata> ,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
 

 
}

impl <'info> InitLIstNFT  <'info > {

    pub fn transfer(&mut self)->Result<()> {
                 let cpi_context=CpiContext::new(self.token_program.to_account_info(), TransferChecked{
                    from:self.seller_token_account.to_account_info(),
                    mint:self.nft.to_account_info(),
                    to:self.listing_token_account.to_account_info(),
                    authority:self.seller.to_account_info()
                 }); 
                 transfer_checked(cpi_context, 1, self.nft.decimals)
        
    }

    pub fn initalise_listing(&mut self,price:u64,bumps:InitLIstNFTBumps)->Result<()>{

        require!(price > 0 , MarketplaceErrors::InvalidPrice);
       
       self.listing.set_inner(Listing{
        seller:self.seller.key(),
        mint:self.nft.key(),
        price,
        bump:bumps.listing,
        is_active:true
       });

        Ok(())
    }
}