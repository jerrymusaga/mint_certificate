use anchor_lang::prelude::*;
use anchor_spl::
    {
        associated_token::AssociatedToken,
        metadata::{create_metadata_accounts_v3},
        token::{self, Mint, TokenAccount, Token, MintTo}
    };



declare_id!("23vxUF22Q7pU5BJtFGwD23xZnF2dSfdCHQjuUbVize5J");

#[program]
pub mod mint_nft_collection {

    use super::*;

    pub fn mint_certificate(ctx: Context<MintCertificate>,name: String, symbol: String, uri: String) -> Result<()> {
        // Check if the user has already minted an NFT in this collection
        require!(!ctx.accounts.has_minted.to_account_info().owner.eq(&crate::ID), ErrorCode::AlreadyMinted);

        // Create the mint account
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0, // decimals
            ctx.accounts.authority.key,
            Some(ctx.accounts.authority.key),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCertificate<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub has_minted: Account<'info, HasMinted>,
}

#[account]
pub struct HasMinted {
    pub has_minted: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already minted an NFT in this collection")]
    AlreadyMinted,
}
