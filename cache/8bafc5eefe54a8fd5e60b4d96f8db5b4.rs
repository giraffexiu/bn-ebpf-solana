// Cached result for function: sub_100000fc8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 8bafc5eefe54a8fd5e60b4d96f8db5b4.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize => {
            let account_info_iter = &mut accounts.iter();
            let mut remaining_accounts = account_info_iter.clone();

            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;
            let _ = next_account_info(account_info_iter)?;

            let remaining_accounts_len = remaining_accounts.count();
            if remaining_accounts_len != 0 {
                return Err(ProgramError::NotEnoughAccountKeys);
            }

            Ok(())
        }
        MyInstruction::AnotherInstruction { value } => {
            // Placeholder for another instruction logic
            msg!("AnotherInstruction called with value: {}", value);
            Ok(())
        }
    }
}

pub enum MyInstruction {
    Initialize,
    AnotherInstruction { value: u64 },
}

impl borsh::de::BorshDeserialize for MyInstruction {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let discriminant: u8 = borsh::BorshDeserialize::deserialize_reader(reader)?;
        match discriminant {
            0 => Ok(MyInstruction::Initialize),
            1 => {
                let value = borsh::BorshDeserialize::deserialize_reader(reader)?;
                Ok(MyInstruction::AnotherInstruction { value })
            }
            _ => Err(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "Invalid discriminant value",
            )),
        }
    }
}

impl borsh::ser::BorshSerialize for MyInstruction {
    fn serialize<W: borsh::io