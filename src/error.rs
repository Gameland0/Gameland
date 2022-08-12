use thiserror::Error;
use solana_program::program_error::ProgramError;
#[error("NotRentExempt")]
NotRentExempt,

#[derive(Error,Debug,Copy,Clone)]
pub enum EscrowError {
    
    // Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
    
    fn from(e: EscrowError) -> Self {
    
        ProgramError::Custom(e as u32)
    }
}