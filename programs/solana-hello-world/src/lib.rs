use anchor_lang::prelude::*;

declare_id!("AL4A5LDecS2GeFqpg6w1WWUcjV4qhXZzuDAF4sMjb3jW");

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}
