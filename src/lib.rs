use solana_program::{
    account_info::AccountInfo, // Module for accessing account metadata and ownership information.
    entrypoint, // Provides the macro to define the entry point of the Solana program.
    entrypoint::ProgramResult, // A type alias for Result<(), ProgramError>, indicating success or failure.
    msg, // A macro for logging messages to the Solana runtime.
    pubkey::Pubkey, // Provides functionality for handling public keys.
};

// This macro defines the entry point for the Solana program.
// `process_instruction` is the function that the Solana runtime calls to process incoming instructions.
entrypoint!(process_instruction);

// The main processing function for the program.
// - `program_id`: The public key of the program (used to identify it).
// - `accounts`: A slice containing account information required by the instruction.
// - `instruction_data`: The raw input data for the instruction.
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Splits the `instruction_data` slice into two parts:
    // - `key`: The first byte of the slice, treated as a key or identifier.
    // - `rem`: The remaining part of the slice after the first byte.
    let (key, rem) = instruction_data.split_first().unwrap();

    // Extracts an 8-byte slice from `rem` (starting from the beginning) and attempts to convert it to a `u64`.
    // - `get(0..8)`: Retrieves a slice containing the first 8 bytes (if available).
    // - `try_into`: Tries to convert the slice into a fixed-size array of 8 bytes.
    // - `u64::from_le_bytes`: Converts the 8-byte array into a `u64` using little-endian byte order.
    // - If any step fails, it defaults to 0 using `unwrap_or(0)`.
    let value: u64 = rem
        .get(0..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .unwrap_or(0);

    // Logs the extracted `value` to the Solana runtime, making it visible for debugging.
    msg!("value {:?}", value);

    // Returns `Ok(())` to indicate the program executed successfully.
    Ok(())
}
