// Cached result for function: sub_100000e90
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: bccbc07b24b99ea7821966f1eb895c54.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: [AccountMeta; 2] = [
        AccountMeta::new_writable(Pubkey::new_from_array([0; 32]), true), // [0] `counter`
        AccountMeta::new_readonly(Pubkey::new_from_array([0; 32]), true), // [1] `payer`
    ];
    pub const IDENT: [u8; 8] = [173, 100, 107, 243, 12, 187, 104, 161];

    pub fn instruction(
        &self,
        accounts: &[AccountInfo],
    ) -> Result<Instruction, ProgramError> {
        let account_metas: Vec<AccountMeta> = accounts
            .iter()
            .map(|account| AccountMeta {
                pubkey: *account.key,
                is_signer: account.is_signer,
                is_writable: account.is_writable,
            })
            .collect();

        Ok(Instruction {
            program_id: crate::id(),
            accounts: account_metas,
            data: self.data(),
        })
    }

    pub fn data(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8);
        buf.extend_from_slice(&Self::IDENT);
        buf
    }
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_counter = &accounts[0];
    let account_info_payer = &accounts[1];

    // Check if the counter account is a signer
    if !account_info_counter.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if the payer account is a signer
    if !account_info_payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if the counter account is writable
    if !account_info_counter.is_writable {
        return Err(ProgramError::InvalidArgument);
    }

    // Check if the payer account is writable
    if !account_info_payer.is_writable {
        return Err(ProgramError::InvalidArgument);
    }

    // Check if the counter account is owned by the current program
    if account_info_counter.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // Check if the counter account's data length is 8 bytes
    if account_info_counter.data_len != 8 {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if the counter account's data is all zeros (uninitialized)
    let counter_data = account_info_counter.data.borrow();
    if counter_data != [0; 8] {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Initialize the counter to 0
    counter_data.copy_from_slice(&0u64.to_le_bytes());

    Ok(())
}