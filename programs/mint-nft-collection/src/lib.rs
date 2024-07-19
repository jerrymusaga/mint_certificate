use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, MintTo};

declare_id!("23vxUF22Q7pU5BJtFGwD23xZnF2dSfdCHQjuUbVize5J");

#[program]
pub mod mint_nft_collection {
    use super::*;

    pub fn mint_certificate(ctx: Context<MintCertificate>, uri: String) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCertificate<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}
