use anchor_lang::prelude::*;

declare_id!("5jtwmP1bNjvEAu23qerh2WCzpULsqZJuGX5cFATbZBHS");

#[program]
pub mod carrot_loyalty_alpha {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_brand(
        ctx: Context<EnrollNewBrand>,
        brand_name: String,
        logo_link: String,
    ) -> ProgramResult {
        let brand = &mut ctx.accounts.brand;
        let brand_address = &mut ctx.accounts.brand_address;

        brand.brand_name = brand_name;
        brand.logo_link = logo_link;
        brand.brand_address = *brand_address.key;
        Ok(())
    }

    pub fn create_loyalty(
        ctx: Context<CreateNewLoyalty>,
        brand_address: Pubkey,
        brand_name: String,
        loyalty_score: u64,
        loyalty_level: u64,
    ) -> ProgramResult {
        let loyalty = &mut ctx.accounts.loyalty;
        let authority_consumer = &mut ctx.accounts.authority_consumer;

        // let brand_address = &mut ctx.accounts.brand_address;
        // let consumer_address = &mut ctx.accounts.consumer_address;

        loyalty.brand_address = brand_address;
        loyalty.consumer_address = *authority_consumer.key;
        loyalty.brand_name = brand_name;
        loyalty.loyalty_score = loyalty_score;
        loyalty.loyalty_level = loyalty_level;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided brand name should be 75 characters long maximum.")]
    BrandNameTooLong,
}

// Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_BRAND_NAME_LENGTH: usize = 75 * 4; // 75 chars max.
const MAX_LOGO_LINK_LENGTH: usize = 75 * 4; // 75 chars max.
const LOYALTY_SCORE_LENGTH: usize = 8;
const LOYALTY_LEVEL_LENGTH: usize = 8;

#[derive(Accounts)]
pub struct EnrollNewBrand<'info> {
    #[account(init, payer = brand_address, space = Brand::LEN)]
    pub brand: Account<'info, Brand>,
    #[account(mut)]
    pub brand_address: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 1. Defining the structure of brand account
#[account]
pub struct Brand {
    pub brand_address: Pubkey,
    pub brand_name: String,
    pub logo_link: String,
}

// implementation block for size calculation
impl Brand {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Brand Address.
        + STRING_LENGTH_PREFIX + MAX_BRAND_NAME_LENGTH // brand name string
        + STRING_LENGTH_PREFIX + MAX_LOGO_LINK_LENGTH;
}

#[derive(Accounts)]
pub struct CreateNewLoyalty<'info> {
    #[account(init, payer = authority_consumer, space = Loyalty::LEN)]
    pub loyalty: Account<'info, Loyalty>,
    #[account(mut)]
    pub authority_consumer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Loyalty {
    pub consumer_address: Pubkey,
    pub brand_address: Pubkey,
    pub brand_name: String,
    pub loyalty_score: u64,
    pub loyalty_level: u64,
}

// implementation block for size calculation
impl Loyalty {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Brand address
        + PUBLIC_KEY_LENGTH // Consumer address.
        + STRING_LENGTH_PREFIX + MAX_BRAND_NAME_LENGTH  // brand name string
        + LOYALTY_SCORE_LENGTH // loyalty score.
        + LOYALTY_LEVEL_LENGTH; // loyalty level.
}
