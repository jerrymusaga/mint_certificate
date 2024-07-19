use anchor_lang::prelude::*;

declare_id!("23vxUF22Q7pU5BJtFGwD23xZnF2dSfdCHQjuUbVize5J");

#[program]
pub mod mint_nft_collection {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
