use anchor_lang::prelude::*;

declare_id!("5jtwmP1bNjvEAu23qerh2WCzpULsqZJuGX5cFATbZBHS");

#[program]
pub mod carrot_loyalty_alpha {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
