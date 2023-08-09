use crate::*;

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


impl RedeemToken<'_> {
    pub fn redeem_token(ctx: &mut Context<RedeemToken>) -> Result<()> {
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
}