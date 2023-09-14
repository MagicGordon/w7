use anchor_lang::{
    prelude::*,
    system_program,
};
use anchor_spl::token;


declare_id!("FHYHp1HoGK1DT7k31XtD1eejDjqXUify5aeDE6j4TpkM");

#[program]
pub mod w7 {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, icon: String) -> Result<()> {

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.auth_account.to_account_info(),
                    to: ctx.accounts.mint_account.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;

        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.auth_account.key(),
            Some(&ctx.accounts.auth_account.key()),
        )?;

        let (gen_ext_mint_key, _) = Pubkey::find_program_address(
            &[
                &ctx.accounts.token_program.key.to_bytes(),
                &ctx.accounts.mint_account.key.to_bytes(),
            ],
            ctx.program_id,
        );
        if gen_ext_mint_key != ctx.accounts.ext_mint_account.key() {
            msg!("Error: ext_mint_account address does not match seed derivation");
            return err!(MyError::InvalidSeeds);
        }

        let ext_mint_account = &mut ctx.accounts.ext_mint_account;
        ext_mint_account.mint = ctx.accounts.mint_account.key();
        ext_mint_account.name = name;
        ext_mint_account.symbol = symbol;
        ext_mint_account.icon = icon;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(
        init,
        payer = auth_account,
        space = 1024,
        seeds = [
            token_program.key().as_ref(),
            mint_account.key().as_ref(),
        ],
        bump
    )]
    pub ext_mint_account: Account<'info, ExtMint>,
    #[account(mut)]
    pub auth_account: Signer<'info>,
    #[account(mut)]
    pub mint_account: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct ExtMint {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub icon: String,
}

#[error_code]
pub enum MyError {
    #[msg("ext_mint_account address does not match seed derivation")]
    InvalidSeeds
}