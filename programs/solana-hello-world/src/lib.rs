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
pub struct CreateMessage<'info> {
    #[account(init, payer = author, space = 1000)]
    pub message: Account<'info, Message>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
		#[account(mut)]
    pub message: Account<'info, Message>,
		#[account(mut)]
    pub author: Signer<'info>,
}

#[account]
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}
