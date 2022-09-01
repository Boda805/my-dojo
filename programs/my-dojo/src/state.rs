use anchor_lang::prelude::*;

#[account]
pub struct MintAuthorityPda {}

#[account]
pub struct MyDojo {
    pub name: String,
    pub location: String,
    pub description: String,
    pub owner: Pubkey,
    pub bump: u8,
}