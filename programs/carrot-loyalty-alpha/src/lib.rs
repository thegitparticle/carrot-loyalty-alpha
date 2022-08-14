use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

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
        // brand_details: Brand,
    ) -> ProgramResult {
        let loyalty = &mut ctx.accounts.loyalty;
        let consumer_address = &mut ctx.accounts.consumer_address;

        loyalty.brand_address = brand_address;
        loyalty.consumer_address = *consumer_address.key;
        loyalty.brand_name = brand_name;
        loyalty.loyalty_score = loyalty_score;
        loyalty.loyalty_level = loyalty_level;

        // let mintNftContext: MintNFT = {};

        // mint_nft(
        //     { accounts: {
        //         mintAuthority.publicKey,
        //         mint: mintKey.publicKey,
        //         tokenAccount: NftTokenAccount,
        //         tokenProgram: TOKEN_PROGRAM_ID,
        //         metadata: metadataAddress,
        //         tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        //         payer: wallet.publicKey,
        //         systemProgram: SystemProgram.programId,
        //         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        //         masterEdition: masterEdition,
        //       } },
        //     *consumer_address.key,
        //     brand_details.logo_link,
        //     brand_name,
        // );

        Ok(())
    }

    pub fn update_loyalty(ctx: Context<UpdateLoyalty>, loyalty_score_change: u64) -> ProgramResult {
        let loyalty = &mut ctx.accounts.loyalty;

        let current_score = loyalty.loyalty_score;
        let new_score = current_score + loyalty_score_change;

        loyalty.loyalty_score = new_score;

        if new_score < 144 {
            loyalty.loyalty_level = 1;
        } else if new_score >= 233 && new_score < 377 {
            loyalty.loyalty_level = 2;
        } else if new_score >= 377 && new_score < 610 {
            loyalty.loyalty_level = 3;
        } else if new_score >= 610 && new_score < 987 {
            loyalty.loyalty_level = 4;
        } else if new_score >= 987 && new_score < 1597 {
            loyalty.loyalty_level = 5;
        } else if new_score >= 1597 && new_score < 2584 {
            loyalty.loyalty_level = 6;
        } else if new_score >= 2584 && new_score < 4181 {
            loyalty.loyalty_level = 7;
        } else if new_score >= 4181 && new_score < 6765 {
            loyalty.loyalty_level = 8;
        } else if new_score >= 6765 && new_score < 10946 {
            loyalty.loyalty_level = 9;
        } else if new_score >= 10946 {
            loyalty.loyalty_level = 10;
        } else {
            loyalty.loyalty_level = 1
        }

        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        msg!("Initializing Mint NFT");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("symb");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                uri,
                Some(creator),
                1,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");
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
    #[account(init, payer = consumer_address, space = Loyalty::LEN)]
    pub loyalty: Account<'info, Loyalty>,
    #[account(mut)]
    pub consumer_address: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateLoyalty<'info> {
    #[account(mut, has_one = consumer_address)]
    pub loyalty: Account<'info, Loyalty>,
    pub consumer_address: Signer<'info>,
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

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}
