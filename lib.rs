use getrandom::{register_custom_getrandom, Error};

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

/// Custom implementation of getrandom that fills the buffer with zeros.
fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    // Fill the buffer with zeros or any other deterministic value.
    buf.fill(0);
    Ok(())
}

// Register the custom getrandom function.
register_custom_getrandom!(custom_getrandom);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    msg!("Hello, world!");
    Ok(())
}