use crate::*;

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
impl UnstakeNft<'_> {
    pub fn unstake_nft(ctx: &mut Context<UnstakeNft>) -> Result<()> {
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