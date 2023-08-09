use crate::*;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        close = user, 
        seeds = [b"lootbox".as_ref(), user.key().as_ref()],
        bump,
        constraint = lootbox_point.claimed == ClaimStatus::Claimed
    )]
    pub lootbox_point: Account<'info, LootboxPointer>,
    #[account(mut)]
    pub user: Signer<'info>
}