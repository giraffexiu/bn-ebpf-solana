// Cached result for function: sub_100002358
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 35cb52fe15f06b42eceb7c4df3e47d8c.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS_LEN: usize = 3;
    pub const DATA_LEN: usize = 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeArgs {
    pub value: u64,
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let _initializer_account = next_account_info(accounts_iter)?;
    let state_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let instruction_data_slice: &[u8] = instruction_data;
    let initialize_args: InitializeArgs =
        InitializeArgs::try_from_slice(instruction_data_slice)?;

    let value = initialize_args.value;

    let rent = Rent::get()?;
    let space = 8; // Size of u64 for the value
    let lamports = rent.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            _initializer_account.key,
            state_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[_initializer_account.clone(), state_account.clone(), system_program.clone()],
    )?;

    state_account.data.borrow_mut()[0..8].copy_from_slice(&value.to_le_bytes());

    Ok(())
}