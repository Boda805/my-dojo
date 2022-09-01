use crate::error::ErrorCode;
use crate::context::*;

use anchor_lang::prelude::*;
use anchor_spl::token;

pub mod context;
pub mod error;
pub mod state;

declare_id!("Ak1zXQBNXUwe95PvEAaYpyMSDaN4uz6jShpdztuzFXoJ");

#[program]
pub mod my_dojo {
    use mpl_token_metadata::instruction::{create_metadata_accounts_v2};
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



