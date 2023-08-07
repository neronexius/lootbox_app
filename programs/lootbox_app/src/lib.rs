use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{AssociatedToken},
    token::{
        burn, Burn, TokenAccount, Token, Mint, MintTo, mint_to
    }
};

declare_id!("wPZy7zBjiXQ8w7Q2hQDXzQamv434Q3Xouzjpu5WCPZC");

#[program]
pub mod lootbox_app {
    use anchor_spl::token::mint_to;

    use super::*;

    pub fn open_lootbox(ctx: Context<OpenLootbox>) -> Result<()> {
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

    pub fn redeem_lootbox(ctx: Context<RedeemMint>) -> Result<()> {
        require!(ctx.accounts.lootbox_pointer.claimed == ClaimStatus::Unclaimed,
            LootError::LootBoxClaimed
        );

        msg!("Minting Specific Token for user");

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo{
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_mint_ata.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info()
                },
                &[&[b"mint_authority", &[*ctx.bumps.get("mint_authority").unwrap()]]])
            , 1
        )?;

        msg!("Changing State to Claimed");

        ctx.accounts.lootbox_pointer.claimed = ClaimStatus::Claimed;

        

        Ok(())
    }
}

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

#[derive(Accounts)]
pub struct RedeemMint<'info> {
    #[account(
        mut,
        seeds = [b"lootbox".as_ref(), user.key().as_ref()],
        bump,
        constraint = lootbox_pointer.is_initialized
    )]
    pub lootbox_pointer: Account<'info, LootboxPointer>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = user,
        payer = user
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lootbox_pointer.mint == mint.key()
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [b"mint_authority".as_ref()],
        bump,
    )]
    ///CHECK MANUAL: Check 
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>

}

#[account]
pub struct LootboxPointer {
    pub is_initialized: bool , // 1 
    pub mint: Pubkey, //32
    pub claimed: ClaimStatus, //1
} 

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum ClaimStatus{
    Unclaimed,
    Claimed
}


#[error_code]
enum LootError {
    #[msg("Box has already been claimed")]
    LootBoxClaimed,

    #[msg("Account already has been initialized, no opening until redeem")]
    LootBoxInitialized
}

