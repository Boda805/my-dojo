use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_metadata_accounts_v2};


declare_id!("vrQ91QxPGytFvwMbnjj1DRwMnauD1EQYRVFuYDbhz3J");

#[program]
pub mod my_dojo {
    use super::*;

    pub fn add_dojo(
        ctx: Context<AddDojo>,
        name: String,
        location: String,
        description: String,
    ) -> Result<()> {
        let my_dojo = &mut ctx.accounts.my_dojo;
        // if name.as_bytes().len() > 200 {
        //     // proper error handling omitted for brevity
        //     panic!();
        // }
        my_dojo.name = name;
        my_dojo.location = location;
        my_dojo.description = description;

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
        // creator_key: Pubkey,
        uri: String,
        name: String,
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
        // let creator = vec![
        //     mpl_token_metadata::state::Creator {
        //         address: creator_key,
        //         verified: false,
        //         share: 100,
        //     },
        //     mpl_token_metadata::state::Creator {
        //         address: ctx.accounts.mint_authority.key(),
        //         verified: false,
        //         share: 0,
        //     },
        // ];
        // msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("symb");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                name + " - Black Belt",
                symbol,
                uri,
                None,
                1,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created");

        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct AddDojo<'info> {
    #[account(mut)]
    dojo_owner: Signer<'info>,
    /*
    space: 8 discriminator + 4 name length + 200 name + 4 location length +
           200 location + 4 description length + 200 description + 1 bump
    */
    #[account(
        init,
        payer = dojo_owner,
        space = 8 + 4 + 200 + 4 + 200 + 4 + 200 + 1,
        seeds = [b"my-dojo", dojo_owner.key().as_ref()], 
        bump
    )]
    pub my_dojo: Account<'info, MyDojo>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintBlackBelt<'info> {
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
}

#[account]
pub struct MyDojo {
    name: String,
    location: String,
    description: String,
    bump: u8,
}