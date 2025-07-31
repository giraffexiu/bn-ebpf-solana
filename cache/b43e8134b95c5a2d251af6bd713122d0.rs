// Cached result for function: sub_100000368
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: b43e8134b95c5a2d251af6bd713122d0.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let accounts_iter = &mut accounts.iter();
            let _program_account = next_account_info(accounts_iter)?;
            let _system_program = next_account_info(accounts_iter)?;
            let _rent = next_account_info(accounts_iter)?;
            let _token_program = next_account_info(accounts_iter)?;
            let _associated_token_program = next_account_info(accounts_iter)?;
            let _spl_token_2022_program = next_account_info(accounts_iter)?;
            let _spl_associated_token_2022_program = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_2 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_3 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_4 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_5 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_6 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_7 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_8 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_9 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_10 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_11 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_12 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_13 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_14 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_15 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_16 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_17 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_18 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_19 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_20 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_21 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_22 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_23 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_24 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_25 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_26 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_27 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_28 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_29 = next_account_info(accounts_iter)?;
            let _mpl_token_metadata_program_3