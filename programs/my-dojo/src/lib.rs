use anchor_lang::prelude::*;

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
        msg!("testing logs 1!");
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

#[account]
pub struct MyDojo {
    name: String,
    location: String,
    description: String,
    bump: u8,
}
