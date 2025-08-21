use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = mint_x,
        associated_token::authority = signer
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = mint_y,
        associated_token::authority = signer
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);

      
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        let p = if is_x {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };

        let res = curve.swap(p, amount, min).map_err(AmmError::from)?;

        require!(res.deposit != 0, AmmError::InvalidAmount);
        require!(res.withdraw != 0, AmmError::InvalidAmount);

        self.deposit_tokens(is_x, res.deposit)?;
        self.withdraw_tokens(!is_x, res.withdraw)?;

        Ok(())
    }

    
    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = if is_x {
            (
                self.user_ata_x.to_account_info(),
                self.vault_x.to_account_info(),
            )
        } else {
            (
                self.user_ata_y.to_account_info(),
                self.vault_y.to_account_info(),
            )
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = Transfer {
            from,
            to,
            authority: self.signer.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, accounts);

        transfer(cpi_context, amount)
    }

   
    pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
       
        let (from, to) = if is_x {
            (
                self.vault_x.to_account_info(),
                self.user_ata_x.to_account_info(),
            )
        } else {
            (
                self.vault_y.to_account_info(),
                self.user_ata_y.to_account_info(),
            )
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info(),
        };

      
        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, accounts, signer_seeds);
        transfer(cpi_context, amount)
    }
}
