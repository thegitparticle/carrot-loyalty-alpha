use anchor_lang::prelude::*;

declare_id!("5jtwmP1bNjvEAu23qerh2WCzpULsqZJuGX5cFATbZBHS");

#[program]
pub mod carrot_loyalty_alpha {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn add_brand(
        ctx: Context<AddNewBrand>,
        loyalty_score: u64,
        loyalty_level: u64,
    ) -> ProgramResult {
        let brand = &mut ctx.accounts.brand;
        let consumer = &mut ctx.accounts.consumer;

        brand.loyalty_score = loyalty_score;
        brand.loyalty_level = loyalty_level;
        brand.consumer = *consumer.key;

        Ok(())
    }
}

// define and manage error statements.
#[error_code]
pub enum ErrorCode {
    #[msg("The provided brand name should be 75 characters long maximum.")]
    BrandNameTooLong,
}

#[derive(Accounts)]
pub struct AddNewBrand<'info> {
    #[account(init, payer = author, space = Brand::LEN)]
    pub brand: Account<'info, Brand>,
    #[account(mut)]
    pub consumer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 1. Defining the structure of tweet account
#[account]
pub struct Brand {
    pub consumer: Pubkey,
    pub loyalty_score: u64,
    pub loyalty_level: u64,
}

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_BRAND_NAME_LENGTH: usize = 75 * 4; // 75 chars max.
const LOYALTY_SCORE_LENGTH: usize = 8;
const LOYALTY_LEVEL_LENGTH: usize = 8;

// 3. implementation block for size calculation
impl Brand {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Consumer.
        + STRING_LENGTH_PREFIX + MAX_BRAND_NAME_LENGTH  // brand name string
        + LOYALTY_SCORE_LENGTH // loyalty score.
        + LOYALTY_LEVEL_LENGTH; // loyalty level.
}
