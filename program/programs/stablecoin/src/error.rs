use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("Below min health factor")]
    BelowMinHealthFactor,

    #[msg("Cannot liquidate a healthy account")]
    AboveMinHealthFactor,

}
