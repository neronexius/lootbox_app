use anchor_lang::prelude::*;
use anchor_spl::token::{
    Token, Mint, MintTo, mint_to,TokenAccount
};
use anchor_spl::associated_token::{
    AssociatedToken
};

use mpl_token_metadata::{
    instruction::Delegate,
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};
use anchor_spl::token::{approve, Approve, revoke, Revoke};
use solana_program::{program::invoke_signed};

declare_id!("EisUaNgBLuBWxFRRDH5scNoXBrpp5rbTcomm6bcRBzNB");

#[program]
pub mod staking_app {


    use super::*;
    pub fn initialize_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Mint initialized!");
        Ok(())
    }

    pub fn stake_nft(ctx:Context<StakeNft>) -> Result<()>{
        require!(ctx.accounts.staking_pda.staking_status == StakingStatus::Unstaked, 
        StakeError::AlreadyStaked);

        msg!("Delegating Authority to Program authority");
        approve(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Approve{
                    to: ctx.accounts.nft_token_acc.to_account_info(),
                    delegate: ctx.accounts.program_authority.to_account_info(),
                    authority: ctx.accounts.user.to_account_info()
                })
            , 1
        )?;

        msg!("Freezing Account");
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.meta_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_token_acc.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key()    
            ),
            &[
                ctx.accounts.meta_program.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_token_acc.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info()
            ],
            &[&[b"authority", &[*ctx.bumps.get("program_authority").unwrap()]]]
        )?;

        msg!("Updating Staking PDA");

        let clock = Clock::get()?;

        ctx.accounts.staking_pda.is_initialized = true; 
        ctx.accounts.staking_pda.last_redeem_time = clock.unix_timestamp;
        ctx.accounts.staking_pda.start_stake_time = clock.unix_timestamp;
        ctx.accounts.staking_pda.nft_token = ctx.accounts.nft_token_acc.key();
        ctx.accounts.staking_pda.user = ctx.accounts.user.key();
        ctx.accounts.staking_pda.staking_status = StakingStatus::Staked;
        
        Ok(())
    }

    pub fn redeem_token(ctx: Context<RedeemToken>) -> Result<()> {
        require!(ctx.accounts.staking_pda.staking_status == StakingStatus::Staked,
        StakeError::InvalidStakeState
        );

        require!(ctx.accounts.staking_pda.is_initialized == true, 
        StakeError::InvalidStakeState
        );

        msg!("Calculating amount...");

        let clock = Clock::get()?;

        let amount  = clock.unix_timestamp.checked_sub(ctx.accounts.staking_pda.last_redeem_time).unwrap();
        
        msg!("Minting Amount to user {}", amount.to_string());
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { 
                    mint: ctx.accounts.mint.to_account_info(), 
                    to: ctx.accounts.user_ata.to_account_info(), 
                    authority: ctx.accounts.mint.to_account_info()
                },
                &[&["mint".as_ref(), &[*ctx.bumps.get("mint").unwrap()]]]
            )
            ,amount.try_into().unwrap()
        )?;

        msg!("Updating last redeem");

        ctx.accounts.staking_pda.last_redeem_time = clock.unix_timestamp;

        Ok(())
    }

    pub fn unstake_nft(ctx:Context<UnstakeNft>) -> Result<()> {
        msg!("Thaw Nft token account");
        
        invoke_signed(
            &thaw_delegated_account(
                ctx.accounts.meta_program.key(),
                ctx.accounts.program_authority.key(), 
                ctx.accounts.nft_token_acc.key(), 
                ctx.accounts.nft_edition.key(), 
                ctx.accounts.nft_mint.key()
            ), 
            &[
                ctx.accounts.meta_program.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
                ctx.accounts.nft_token_acc.to_account_info(),
                ctx.accounts.nft_edition.to_account_info()
            ],
            &[&[b"authority", &[*ctx.bumps.get("program_authority").unwrap()]]]
        )?;

        msg!("Revoking delegate");
        revoke(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Revoke {
                    source: ctx.accounts.nft_token_acc.to_account_info(), 
                    authority: ctx.accounts.user.to_account_info()
                })
        )?;

        msg!("Calculating amount...");

        let clock = Clock::get()?;

        let amount  = clock.unix_timestamp.checked_sub(ctx.accounts.staking_pda.last_redeem_time).unwrap();
        
        msg!("Minting Amount to user {}", amount.to_string());
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo { 
                    mint: ctx.accounts.mint.to_account_info(), 
                    to: ctx.accounts.user_ata.to_account_info(), 
                    authority: ctx.accounts.mint.to_account_info()
                },
                &[&["mint".as_ref(), &[*ctx.bumps.get("mint").unwrap()]]]
            )
            ,amount.try_into().unwrap()
        )?;

        ctx.accounts.staking_pda.last_redeem_time = clock.unix_timestamp;
        ctx.accounts.staking_pda.staking_status = StakingStatus::Unstaked;

        msg!("Done");

        Ok(())
    }
}

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


#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(
        init_if_needed,
        seeds = [user.key().as_ref(), nft_token_acc.key().as_ref()],
        bump,
        space = 8 + 1 + 32 + 32 + 8 + 8 + 1,
        payer = user
    )]
    pub staking_pda: Account<'info, StakingPda>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub nft_token_acc: Account<'info, TokenAccount>,

    pub nft_mint: Account<'info, Mint>,

    ///CHECK: Check Manual
    #[account(
        mut,
        seeds = [b"authority".as_ref()],
        bump
    )]
    pub program_authority: UncheckedAccount<'info>,

    ///CHECK: Check Manual
    #[account(
        owner = MetadataTokenId
    )]
    pub nft_edition: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    ///CHECK: Check Manual
    pub meta_program: Program<'info, Metadata>
}

#[derive(Accounts)]
pub struct RedeemToken<'info> {
    #[account(
        init_if_needed, 
        associated_token::mint = mint,
        associated_token::authority = user,
        payer = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [user.key().as_ref(), nft_token_acc.key().as_ref()],
        bump,
    )]
    pub staking_pda: Account<'info, StakingPda>,

    #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub nft_token_acc: Account<'info, TokenAccount>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"mint".as_ref()],
        bump,
    )]
    pub mint:Account<'info,Mint> ,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    #[account(
        mut,
        seeds=[user.key().as_ref(), nft_token_acc.key().as_ref()],
        bump
    )]
    pub staking_pda: Account<'info, StakingPda>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub nft_token_acc:Account<'info, TokenAccount>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        owner = MetadataTokenId
    )]
    ///CHECK: check manual
    pub nft_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = user,
        payer = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"mint".as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,


    #[account(
        mut,
        seeds = [b"authority".as_ref()],
        bump
    )]
    ///CHECK: Check Manual
    pub program_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub meta_program: Program<'info, Metadata> 
}


#[account]
pub struct StakingPda {
    pub is_initialized: bool, //1
    pub user: Pubkey, //32
    pub nft_token: Pubkey, //32
    pub start_stake_time: i64, //8
    pub last_redeem_time: i64, //8
    pub staking_status: StakingStatus //1
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum StakingStatus{
    Unstaked,
    Staked,
}

impl Default for StakingStatus {
    fn default() -> Self {
        StakingStatus::Unstaked
    }
}

#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata {
    fn id() -> Pubkey {
        MetadataTokenId
    }
}

#[error_code]
pub enum StakeError {
    #[msg("NFT already staked")]
    AlreadyStaked,

    #[msg("State account is uninitialized")]
    UninitializedAccount,

    #[msg("Stake state is invalid")]
    InvalidStakeState,
}