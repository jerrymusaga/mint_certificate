use anchor_lang::prelude::*;
use anchor_spl::token::{self, mint_to, Mint, TokenAccount, Token, MintTo};

declare_id!("23vxUF22Q7pU5BJtFGwD23xZnF2dSfdCHQjuUbVize5J");

#[program]
pub mod mint_nft_collection {
    use super::*;

    pub fn mint_certificate(ctx: Context<MintCertificate>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
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
