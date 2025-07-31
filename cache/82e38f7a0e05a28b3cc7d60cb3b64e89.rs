// Cached result for function: sub_100002ce0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 82e38f7a0e05a28b3cc7d60cb3b64e89.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: [anchor_lang::prelude::AccountMeta; 2] = [
        anchor_lang::prelude::AccountMeta {
            pubkey: Pubkey::new_from_array([
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1,
            ]),
            is_signer: true,
            is_writable: true,
        },
        anchor_lang::prelude::AccountMeta {
            pubkey: Pubkey::new_from_array([
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2,
            ]),
            is_signer: false,
            is_writable: true,
        },
    ];
    pub const DATA: [u8; 8] = [105, 110, 105, 116, 105, 97, 108, 105]; // "initialize"
}

pub struct MyAccount {
    pub data: u64,
}

impl anchor_lang::AccountSerialize for MyAccount {}
impl anchor_lang::AccountDeserialize for MyAccount {}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let accounts_len = accounts.len();
    if accounts_len < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let initializer_account_info = &accounts[0];
    let my_account_info = &accounts[1];

    if !initializer_account_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !my_account_info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if the account is owned by the system program (i.e., uninitialized)
    if my_account_info.owner != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId); // Or a more specific error like AccountAlreadyInitialized
    }

    // Allocate space for MyAccount data (8 bytes for u64)
    let account_data_len = 8;
    let lamports_required = solana_program::rent::Rent::get()?.minimum_balance(account_data_len);

    // Create the account using the system program
    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            initializer_account_info.key,
            my_account_info.key,
            lamports_required,
            account_data_len as u64,
            program_id,
        ),
        &[
            initializer_account_info.clone(),
            my_account_info.clone(),
            solana_program::program::get_system_program_account().clone(),
        ],
    )?;

    // Deserialize the account data
    let mut my_account_data =
        MyAccount::try_from_slice(&my_account_info.data.borrow())?;

    // Set the data field (e.g., to 0 or some initial