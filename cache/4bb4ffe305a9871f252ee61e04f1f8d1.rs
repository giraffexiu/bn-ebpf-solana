// Cached result for function: sub_1000018a0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 4bb4ffe305a9871f252ee61e04f1f8d1.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let account_iter = &mut accounts.iter();
            let my_account = next_account_info(account_iter)?;
            let rent = next_account_info(account_iter)?;

            let mut my_account_data = my_account.try_borrow_mut_data()?;
            let mut my_account_state = MyAccount::try_from_slice(&my_account_data)?;

            if my_account_state.initialized {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            my_account_state.initialized = true;
            my_account_state.serialize(&mut &mut my_account_data[..])?;

            Ok(())
        }
        MyInstruction::Increment => {
            let account_iter = &mut accounts.iter();
            let my_account = next_account_info(account_iter)?;

            let mut my_account_data = my_account.try_borrow_mut_data()?;
            let mut my_account_state = MyAccount::try_from_slice(&my_account_data)?;

            if !my_account_state.initialized {
                return Err(ProgramError::UninitializedAccount);
            }

            my_account_state.counter += 1;
            my_account_state.serialize(&mut &mut my_account_data[..])?;

            Ok(())
        }
        MyInstruction::Decrement => {
            let account_iter = &mut accounts.iter();
            let my_account = next_account_info(account_iter)?;

            let mut my_account_data = my_account.try_borrow_mut_data()?;
            let mut my_account_state = MyAccount::try_from_slice(&my_account_data)?;

            if !my_account_state.initialized {
                return Err(ProgramError::UninitializedAccount);
            }

            my_account_state.counter -= 1;
            my_account_state.serialize(&mut &mut my_account_data[..])?;

            Ok(())
        }
        MyInstruction::Update { value } => {
            let account_iter = &mut accounts.iter();
            let my_account = next_account_info(account_iter)?;

            let mut my_account_data = my_account.try_borrow_mut_data()?;
            let mut my_account_state = MyAccount::try_from_slice(&my_account_data)?;

            if !my_account_state.initialized {
                return Err(ProgramError::UninitializedAccount);
            }

            my_account_state.counter = value;
            my_account_state.serialize(&mut &mut my_account_data[..])?;

            Ok(())
        }
    }
}