use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Name too long")]
    NameTooLong,

    #[msg("Location too long")]
    LocationTooLong,
    
    #[msg("Description too long")]
    DescriptionTooLong,

    #[msg("You are not the owner of the provided dojo")]
    NotDojoOwner,
}