use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Token, TransferChecked},
    token_interface::{Mint, TokenAccount},
};
use crate::{errors::MarketplaceErrors};

use crate::state::{Listing, MarketPlace};
#[derive(Accounts)]
pub struct InitPurchaseNft<'info> {
#[account(mut)]
 pub buyer:Signer<'info> ,
 #[account(mut)]
 pub seller:Signer<'info>,

 pub nft:InterfaceAccount<'info,Mint> ,
 #[account(
    mut,
     seeds=[
        b"listing",
        marketplace.key().as_ref(),
        seller.key().as_ref(),
        nft.key().as_ref()
     ],
     bump
 )]
 pub listing:Account<'info,Listing>,
#[account(
    mut,
    associated_token::mint=nft,
    associated_token::authority=listing
)]
 pub listing_token_account:InterfaceAccount<'info,TokenAccount>,
 #[account(
     init_if_needed,
     payer=buyer,
    associated_token::mint=nft,
    associated_token::authority=buyer
 )]
 pub buyer_token_account:InterfaceAccount<'info,TokenAccount>,
#[account(
    seeds=[b"marketplace"],
    bump=marketplace.bump
)]
 pub marketplace:Account<'info,MarketPlace>,
 #[account(
    seeds=[b"treasury",marketplace.key().as_ref()],
    bump
   )]
 pub treausry:SystemAccount<'info>  , 

 pub token_program:Program<'info,Token>,
 pub associated_token_program:Program<'info,AssociatedToken>,
 pub system_program:Program<'info,System>
}

impl <'info> InitPurchaseNft<'info> {

    pub fn transfer_nft(&mut self )->Result<()>{
      


      require!(self.listing.is_active && self.listing.seller == self.seller.key(),
    MarketplaceErrors::ListingNotActive
);
 
       let marketplace=self.marketplace.key();
       let seller=self.seller.key();
       let nft=self.nft.key(); 

       let listing_seeds:&[&[u8]]=&[
          b"listing",
          marketplace.as_ref(),
          seller.as_ref(),
          nft.as_ref(),
          &[self.listing.bump]
       ];

       let signer_seeds=&[listing_seeds];

       let token_program=self.token_program.to_account_info(); 
       let accounts=TransferChecked{
        from:self.listing_token_account.to_account_info(),
        mint:self.nft.to_account_info(),
        to:self.buyer_token_account.to_account_info(),
        authority:self.listing.to_account_info()
       };

       let ctx=CpiContext::new_with_signer(token_program, accounts, signer_seeds);

       transfer_checked(ctx, 1, self.nft.decimals)



       

    }



    pub fn transfer_sol(&mut self) ->Result<()>{
   let fee_lamports=(self.marketplace.fee_percentage as u64).checked_mul(self.listing.price).
   ok_or(MarketplaceErrors::MathOverflow)?.checked_div(100).ok_or(MarketplaceErrors::MathOverflow)?;


   let seller_lamports=self.listing.price.checked_sub(fee_lamports).ok_or(MarketplaceErrors::MathOverflow)?;


   let ctx=CpiContext::new(self.token_program.to_account_info(), Transfer{
    from:self.buyer.to_account_info(),
    to:self.treausry.to_account_info()
   });


       transfer(ctx, seller_lamports)?;
       Ok(())
    }

    pub fn delist_nft(&mut self) -> Result<()> {
   
        self.listing.is_active = false;
        Ok(())
    }
}