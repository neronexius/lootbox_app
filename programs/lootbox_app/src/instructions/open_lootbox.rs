use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
    Token, TokenAccount, Mint, MintTo, mint_to
    },
    associated_token::AssociatedToken
};
use crate::*;


#[derive(Accounts)]
pub struct OpenLootbox<'info> {
    #[account(
        init_if_needed, 
        seeds = [b"lootbox".as_ref(), user.key().as_ref()],
        bump, 
        payer = user,
        space = 8 + 32 + 1 + 1
    )]
    pub lootbox_pointer: Account<'info, LootboxPointer>,

    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>, 

    #[account(
        mut,
        address = "7Gf4SiU1p9mfPt2icJti4rPut4mJX893CTuhTXS4GcMi".parse::<Pubkey>().unwrap()
    )]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken> 
}

impl OpenLootbox<'_> {
    pub fn open_lootbox(ctx: &mut Context<Self>) -> Result<()> {
        require!(ctx.accounts.lootbox_pointer.claimed == ClaimStatus::Unclaimed, 
            LootError::LootBoxClaimed
        );

        require!(!ctx.accounts.lootbox_pointer.is_initialized, 
            LootError::LootBoxInitialized
        );

        msg!("Burning necessary tokens to open ");

        burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn { 
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.user_ata.to_account_info(), 
                    authority: ctx.accounts.user.to_account_info()
                }
            ),
            10
        )?;

        //random command to point to the selected mint
        

        let clock = Clock::get().unwrap();

        let random = clock.unix_timestamp % 5 ;

        let selected_key = match random {
            0 => "4PSN9HMKSqZjE1Lu2WTQdVPmRpwNnmehcmNdb7FdSuf2".parse::<Pubkey>().unwrap(),
            1 => "2spgRLUpLFZokN8gz7fTtTt1g3cYNYA3GtTL8CRMqFZ4".parse::<Pubkey>().unwrap(),
            2 => "BoehhqNcXWZUHAUXpEiaY2Tw2SBe7DHk3yvsrTftb6eY".parse::<Pubkey>().unwrap(),
            3 => "DtrvvTTD9CPbgEgqUenp4Doe5Tz3ocKvDoCciadwyZbY".parse::<Pubkey>().unwrap(),
            4 => "4iy5iTrhFnPfMEAkGmT8rT69UA4UR23P3R75p8ze2FyV".parse::<Pubkey>().unwrap(),
            _ => "4iy5iTrhFnPfMEAkGmT8rT69UA4UR23P3R75p8ze2FyV".parse::<Pubkey>().unwrap(),
        };

        ctx.accounts.lootbox_pointer.claimed = ClaimStatus::Unclaimed;
        ctx.accounts.lootbox_pointer.mint = selected_key;
        ctx.accounts.lootbox_pointer.is_initialized = true;
        
        Ok(())
    }
}