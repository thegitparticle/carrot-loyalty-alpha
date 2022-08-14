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
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided brand name should be 75 characters long maximum.")]
    BrandNameTooLong,
}

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

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_BRAND_NAME_LENGTH: usize = 75 * 4; // 75 chars max.
const MAX_LOGO_LINK_LENGTH: usize = 75 * 4; // 75 chars max.

// 3. implementation block for size calculation
impl Brand {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Brand Address.
        + STRING_LENGTH_PREFIX + MAX_BRAND_NAME_LENGTH // brand name string
        + STRING_LENGTH_PREFIX + MAX_LOGO_LINK_LENGTH;
}
