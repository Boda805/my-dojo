use crate::state::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

#[derive(Accounts)]
pub struct AddDojo<'info> {
    #[account(mut)]
    pub dojo_owner: Signer<'info>,
    /*
    space:   8  discriminator 
             4  name length
           200  name
             4  location length
           200  location
             4  description length
           200  description
            32  owner address
        +    1  bump
        ---------------------------
        =  653  bytes
    */
    #[account(
        init,
        payer = dojo_owner,
        space = 8 + 4 + 200 + 4 + 200 + 4 + 200 + 32 + 1,
        seeds = [b"my-dojo", dojo_owner.key().as_ref()], 
        bump
    )]
    pub my_dojo: Account<'info, MyDojo>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintBlackBelt<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_account: Account<'info, token::Mint>,
    #[account(
        init, 
        payer = payer,
        space = 8 + 32,
        seeds = [
            b"mint_authority_", 
            mint_account.key().as_ref(),
        ],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account()]
    pub my_dojo: Account<'info, MyDojo>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}