use std::convert::TryInto;
use crate::error::EscrowError::InvalidInstruction;
use solana_program::program_error::ProgramError;

pub enum EscrowInstruction {
    
    /// 因为要在初始化里转移临时代币账号所有权，所以需要原owner签名，并且原owner也是初始化者
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
    
        /// The amount party A expects to receive of token Y
        amount: u64
    }
}

impl EscrowInstruction {
    

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        
        Ok(match tag {
    
            0 => Self::InitEscrow {
    
                amount: Self::unpack_amount(rest)?,
            },
            //注意这里的用法，InvalidInstruction转化为ProgramError时，使用了into
          	//因为我们在error.rs中已经实现了那个from，系统会自动帮我们实现into
            _ => return Err(InvalidInstruction.into()),
        })
    }

    //这里学习Input 转化为u64
    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
    
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}