use crate::*;

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