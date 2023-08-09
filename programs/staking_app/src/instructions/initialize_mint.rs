use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};


#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init_if_needed,
        seeds = [b"mint".as_ref()],
        bump,
        payer = initializer,
        mint::decimals = 2, 
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut
    )]
    pub initializer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, 
}