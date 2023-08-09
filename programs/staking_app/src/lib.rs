use anchor_lang::prelude::*;
use anchor_spl::token::{
    Token, Mint, MintTo, mint_to,TokenAccount
};
use anchor_spl::associated_token::AssociatedToken;

use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};
use anchor_spl::token::{approve, Approve, revoke, Revoke};
use solana_program::program::invoke_signed;

mod error;
mod state;
mod instructions;

use error::*;
use state::*;
use instructions::*;

declare_id!("EisUaNgBLuBWxFRRDH5scNoXBrpp5rbTcomm6bcRBzNB");

#[program]
pub mod staking_app {


    use super::*;

    pub fn initialize_mint(_ctx: Context<InitializeMint>) -> Result<()>{
        Ok(())
    }

    pub fn stake_nft(mut ctx: Context<StakeNft>) -> Result<()> {
        StakeNft::stake_nft(&mut ctx)
    }

    pub fn redeem_token(mut ctx: Context<RedeemToken>) -> Result<()> {
        RedeemToken::redeem_token(&mut ctx)
    }

    pub fn unstake_nft(mut ctx: Context<UnstakeNft>) -> Result<()>{
        UnstakeNft::unstake_nft(&mut ctx)
    }

}










