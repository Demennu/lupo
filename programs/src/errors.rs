use anchor_lang::error_code;

#[error_code]
pub enum PredictionError {
    #[msg("Overflow")]
    Overflow
}
