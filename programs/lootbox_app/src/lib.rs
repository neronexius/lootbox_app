use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{AssociatedToken},
    token::{
        burn, Burn, TokenAccount, Token, Mint, MintTo, mint_to
    }
};

mod instructions;
mod error;
mod state;

use instructions::*;
use state::*;
use error::*;

declare_id!("wPZy7zBjiXQ8w7Q2hQDXzQamv434Q3Xouzjpu5WCPZC");

#[program]
pub mod lootbox_app {

    use super::*;

    pub fn open_lootbox(mut ctx:Context<OpenLootbox>) -> Result<()> {
        OpenLootbox::open_lootbox(&mut ctx)
    }

    pub fn redeem_lootbox(mut ctx: Context<RedeemMint>) -> Result<()> {
        RedeemMint::redeem_lootbox(&mut ctx)
    }
}

