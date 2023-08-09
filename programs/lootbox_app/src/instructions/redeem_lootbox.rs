use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
    Token, TokenAccount, Mint, MintTo, mint_to
    },
    associated_token::AssociatedToken
};
use crate::*;

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

impl RedeemMint<'_> {
    pub fn redeem_lootbox(ctx: &mut Context<RedeemMint>) -> Result<()> {
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
