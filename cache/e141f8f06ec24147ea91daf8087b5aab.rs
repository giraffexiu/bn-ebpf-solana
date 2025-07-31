// Cached result for function: sub_100000e30
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: e141f8f06ec24147ea91daf8087b5aab.rs

pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub lamports: &'a mut u64,
    pub data: &'a mut [u8],
    pub owner: &'a Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

pub struct Pubkey([u8; 32]);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), u64> {
    if instruction_data.len() < 8 {
        return Err(1); // Instruction data too short
    }

    let discriminator = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

    match discriminator {
        0x56710657c38c238b => { // Initialize
            if accounts.len() < 2 {
                return Err(1); // Not enough accounts
            }
            let account_0 = &accounts[0]; // Payer
            let account_1 = &accounts[1]; // Counter

            if !account_0.is_signer {
                return Err(1); // Payer must be a signer
            }
            if !account_0.is_writable {
                return Err(1); // Payer must be writable
            }
            if account_1.owner != program_id {
                return Err(1); // Counter account must be owned by the program
            }
            if account_1.data.len() != 8 {
                return Err(1); // Counter account data length must be 8 bytes
            }

            // Initialize counter to 0
            account_1.data[0..8].copy_from_slice(&0u64.to_le_bytes());
            Ok(())
        },
        0x1f062973179836f6 => { // Increment
            if accounts.len() < 1 {
                return Err(1); // Not enough accounts
            }
            let account_0 = &accounts[0]; // Counter

            if account_0.owner != program_id {
                return Err(1); // Counter account must be owned by the program
            }
            if account_0.data.len() != 8 {
                return Err(1); // Counter account data length must be 8 bytes
            }
            if !account_0.is_writable {
                return Err(1); // Counter account must be writable
            }

            let mut counter_value = u64::from_le_bytes(account_0.data[0..8].try_into().unwrap());
            counter_value = counter_value.checked_add(1).ok_or(1)?; // Increment counter, check for overflow
            account_0.data[0..8].copy_from_slice(&counter_value.to_le_bytes());
            Ok(())
        },
        _ => Err(1), // Unknown instruction discriminator
    }
}