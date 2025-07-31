// Cached result for function: sub_100000430
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: dcd06359cf7731826583bd484dcf3355.rs

pub enum ErrorCode {
    #[msg("Invalid instruction data")]
    InvalidInstructionData = 0x1770, // 6000
    #[msg("Invalid account data")]
    InvalidAccountData = 0x1771, // 6001
    #[msg("Invalid account owner")]
    InvalidAccountOwner = 0x1772, // 6002
    #[msg("Account not writable")]
    AccountNotWritable = 0x1773, // 6003
    #[msg("Account not signer")]
    AccountNotSigner = 0x1774, // 6004
}

pub fn process_instruction(
    program_id: &SolPubkey,
    accounts: &mut [SolAccountInfo],
    instruction_data: &[u8],
) -> u64 {
    if instruction_data.len() < 8 {
        return ErrorCode::InvalidInstructionData as u64;
    }

    let instruction_discriminator = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    match instruction_discriminator {
        0x1c37b9875e536102 => { // Initialize
            if accounts.len() < 2 {
                return ErrorCode::InvalidAccountData as u64;
            }

            let account_0 = &mut accounts[0]; // Payer
            let account_1 = &mut accounts[1]; // Config

            if account_0.owner != program_id {
                return ErrorCode::InvalidAccountOwner as u64;
            }
            if !account_0.is_writable {
                return ErrorCode::AccountNotWritable as u64;
            }
            if !account_0.is_signer {
                return ErrorCode::AccountNotSigner as u64;
            }

            if account_1.owner != program_id {
                return ErrorCode::InvalidAccountOwner as u64;
            }
            if !account_1.is_writable {
                return ErrorCode::AccountNotWritable as u64;
            }

            // Deserialize instruction data
            // Assuming instruction_data[8..] contains the data for Initialize
            // This part would typically involve a custom deserialization logic
            // For now, we'll just acknowledge its presence.
            let _initialize_data = &instruction_data[8..];

            // Placeholder for actual initialization logic
            // This would involve writing data to account_1 (Config)
            // and potentially transferring lamports from account_0 (Payer)
            // For example:
            // let config_data_len = account_1.data_len;
            // let config_data = account_1.data;
            // config_data[0..8].copy_from_slice(&0xDEADBEEF_u64.to_le_bytes()); // Example write

            0 // Success
        }
        // Add more instruction discriminators and their corresponding logic here
        _ => ErrorCode::InvalidInstructionData as u64, // Unknown instruction
    }
}

#[no_mangle]
pub extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let sol_parameters: &mut SolParameters = unsafe { &mut *(input as *mut SolParameters) };

    let program_id