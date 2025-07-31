// Cached result for function: sub_100002ba0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: d9e74b8a4919899704382e7f58fb5964.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let account_info_iter = &mut accounts.iter();
            let _payer_account = next_account_info(account_info_iter)?;
            let _program_account = next_account_info(account_info_iter)?;
            let _system_program = next_account_info(account_info_iter)?;
            // No specific logic for Initialize in the provided decompilation,
            // likely just account validation and setup.
            Ok(())
        }
        MyInstruction::Increment => {
            let account_info_iter = &mut accounts.iter();
            let program_account = next_account_info(account_info_iter)?;

            let mut program_data = program_account.try_borrow_mut_data()?;
            let mut counter_data = Counter::try_from_slice(&program_data)?;
            counter_data.count = counter_data.count.checked_add(1).ok_or(ProgramError::ArithmeticOverflow)?;
            counter_data.serialize(&mut &mut program_data[..])?;
            Ok(())
        }
        MyInstruction::Decrement => {
            let account_info_iter = &mut accounts.iter();
            let program_account = next_account_info(account_info_iter)?;

            let mut program_data = program_account.try_borrow_mut_data()?;
            let mut counter_data = Counter::try_from_slice(&program_data)?;
            counter_data.count = counter_data.count.checked_sub(1).ok_or(ProgramError::ArithmeticOverflow)?;
            counter_data.serialize(&mut &mut program_data[..])?;
            Ok(())
        }
        MyInstruction::Set { value } => {
            let account_info_iter = &mut accounts.iter();
            let program_account = next_account_info(account_info_iter)?;

            let mut program_data = program_account.try_borrow_mut_data()?;
            let mut counter_data = Counter::try_from_slice(&program_data)?;
            counter_data.count = value;
            counter_data.serialize(&mut &mut program_data[..])?;
            Ok(())
        }
    }
}