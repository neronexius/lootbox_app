use anchor_lang::prelude::*;

#[error_code]
pub enum LootError {
    #[msg("Box has already been claimed")]
    LootBoxClaimed,

    #[msg("Account already has been initialized, no opening until redeem")]
    LootBoxInitialized
}
