use anchor_lang::prelude::*;

pub static BLACK_BELT_URI: &'static str = "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA";

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