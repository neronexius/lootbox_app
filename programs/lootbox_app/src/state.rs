use anchor_lang::prelude::*;

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

