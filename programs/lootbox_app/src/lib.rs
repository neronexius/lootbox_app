use anchor_lang::prelude::*;

declare_id!("wPZy7zBjiXQ8w7Q2hQDXzQamv434Q3Xouzjpu5WCPZC");

#[program]
pub mod lootbox_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
