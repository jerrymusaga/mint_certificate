use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::{
    instructions::{CreateMasterEditionV3, CreateMetadataAccountV3, VerifyCollection},
    types::{Collection, Creator, DataV2},
    accounts::Metadata
};
declare_id!("23vxUF22Q7pU5BJtFGwD23xZnF2dSfdCHQjuUbVize5J");

#[program]
pub mod mint_nft_collection {

    use super::*;

    pub fn mint_certificate(ctx: Context<MintCertificate>,name: String, symbol: String, uri: String) -> Result<()> {
       Ok(())
    }
}


#[derive(Accounts)]
pub struct MintCertificate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: This is the authority (deployer) who pays for the minting
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = collection_mint,
        associated_token::authority = user
    )]
    pub user_minted_nft: Account<'info, TokenAccount>,
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    /// CHECK: This account is created by the mpl-token-metadata program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This account is created by the mpl-token-metadata program
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    /// CHECK: This is the metadata account of the collection
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: This is the master edition account of the collection
    pub collection_master_edition: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    // pub token_metadata_program: Program<'info, mpl_token_metadata::ID>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already minted an NFT in this collection")]
    AlreadyMinted,
}