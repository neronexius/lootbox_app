use crate::*;

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


impl StakeNft<'_>{
    pub fn stake_nft(ctx:&mut Context<StakeNft>) -> Result<()>{
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
}