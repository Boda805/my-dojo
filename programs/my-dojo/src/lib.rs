use anchor_lang::{prelude::*};
use anchor_spl::{token::{self, TokenAccount}};
use mpl_token_metadata::instruction::{create_metadata_accounts_v2};

use crate::error::ErrorCode;
pub mod error;

declare_id!("Ak1zXQBNXUwe95PvEAaYpyMSDaN4uz6jShpdztuzFXoJ");

#[program]
pub mod my_dojo {
    use anchor_lang::solana_program::program::invoke_signed;

    use super::*;

    pub fn add_dojo(
        ctx: Context<AddDojo>,
        name: String,
        location: String,
        description: String,
    ) -> Result<()> {
        let my_dojo = &mut ctx.accounts.my_dojo;

        if name.as_bytes().len() > 200 {
            return Err(ErrorCode::NameTooLong.into())
        }
        if location.as_bytes().len() > 200 {
            return Err(ErrorCode::LocationTooLong.into())
        }
        if description.as_bytes().len() > 200 {
            return Err(ErrorCode::DescriptionTooLong.into())
        }
        
        my_dojo.name = name;
        my_dojo.location = location;
        my_dojo.description = description;
        my_dojo.owner = ctx.accounts.dojo_owner.key();

        /*
        Causing "Error: failed to send transaction: Transaction simulation
        failed: Error processing Instruction 0: Program failed to complete"
        */
        // my_dojo.bump = *ctx.bumps.get("my-dojo").unwrap();

        let (_, first_bump) = Pubkey::find_program_address(
            &[b"my-dojo", ctx.accounts.dojo_owner.key.as_ref()],
            ctx.program_id,
        );

        my_dojo.bump = first_bump;
        
        Ok(())
    }

    pub fn mint_black_belt(
        ctx: Context<MintBlackBelt>,
        metadata_uri: String,
        metadata_name: String,
        mint_authority_pda_bump: u8,
    ) -> Result<()> {
        let my_dojo = &ctx.accounts.my_dojo;

        if ctx.accounts.payer.key() != my_dojo.owner {
            return Err(ErrorCode::NotDojoOwner.into())
        }

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint_authority_", 
                    ctx.accounts.mint_account.key().as_ref(),
                    &[mint_authority_pda_bump],
                ]]
            ),
            1,
        )?;
        msg!("Token Minted!");

        msg!("Creating metadata account...");
        
        invoke_signed(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata_account.key(),
                ctx.accounts.mint_account.key(), 
                ctx.accounts.mint_authority.key(), 
                ctx.accounts.payer.key(),  
                ctx.accounts.mint_authority.key(),
                metadata_name,
                String::from("BELT"),
                metadata_uri, 
                None, 
                0,                           
                true,     
                false,   
                None,   
                None,
            ),
            &[
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.mint_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
            &[&[
                b"mint_authority_", 
                ctx.accounts.mint_account.key().as_ref(),
                &[mint_authority_pda_bump],
            ]]
        )?;

        msg!("Token mint created successfully.");

        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct AddDojo<'info> {
    #[account(mut)]
    dojo_owner: Signer<'info>,
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

#[account]
pub struct MintAuthorityPda {}

#[account]
pub struct MyDojo {
    name: String,
    location: String,
    description: String,
    owner: Pubkey,
    bump: u8,
}