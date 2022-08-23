use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not the owner of the provided dojo")]
    NotDojoOwner,
}