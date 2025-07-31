// Cached result for function: sub_1000012e0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: eaf0ce2ab72b447722ffd0708c72051f.rs

pub struct Initialize;

impl Initialize {
    pub const ACCOUNTS: [AccountMeta; 2] = [
        AccountMeta::new_writable(Pubkey::new_from_array([0; 32]), true), // [0] `payer`
        AccountMeta::new_writable(Pubkey::new_from_array([0; 32]), false), // [1] `counter`
    ];
    pub const IDENT: [u8; 8] = [173, 108, 237, 240, 166, 109, 101, 120];

    pub fn instruction(&self) -> Instruction {
        Instruction {
            program_id: crate::ID,
            accounts: Initialize::ACCOUNTS.to_vec(),
            data: InstructionData::new_with_discriminator(Initialize::IDENT, &self).unwrap(),
        }
    }
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account_info = next_account_info(account_info_iter)?;
    let counter_account_info = next_account_info(account_info_iter)?;

    if !payer_account_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = Rent::get()?;
    let space = 8 + 8; // Discriminator (8 bytes) + count (u64, 8 bytes)
    let lamports = rent.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            payer_account_info.key,
            counter_account_info.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            payer_account_info.clone(),
            counter_account_info.clone(),
            system_program::id().to_account_info(),
        ],
    )?;

    let mut counter_data = counter_account_info.try_borrow_mut_data()?;
    let mut counter_account = Counter::try_from_slice(&counter_data)?;
    counter_account.count = 0;
    counter_account.serialize(&mut &mut counter_data[..])?;

    Ok(())
}