// Cached result for function: sub_100001928
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 7556492b715f41aac329331f1c8f2f8d.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(instruction_data)?;
    match instruction {
        Instruction::Initialize => {
            let account_info_iter = &mut accounts.iter();
            let mut remaining_accounts = account_info_iter.clone();

            let mint_account = next_account_info(account_info_iter)?;
            let rent_sysvar = next_account_info(account_info_iter)?;
            let token_program = next_account_info(account_info_iter)?;
            let system_program = next_account_info(account_info_iter)?;
            let payer_account = next_account_info(account_info_iter)?;

            let cpi_accounts = [mint_account.clone(), rent_sysvar.clone()];
            let cpi_program = token_program.clone();
            let ix_data = spl_token::instruction::initialize_mint(
                &token_program.key,
                &mint_account.key,
                &payer_account.key,
                None,
                0,
            )?;
            invoke(
                &ix_data,
                &[
                    mint_account.clone(),
                    rent_sysvar.clone(),
                    token_program.clone(),
                ],
            )?;
        }
        Instruction::MintTo { amount } => {
            let account_info_iter = &mut accounts.iter();
            let mut remaining_accounts = account_info_iter.clone();

            let mint_account = next_account_info(account_info_iter)?;
            let destination_account = next_account_info(account_info_iter)?;
            let authority_account = next_account_info(account_info_iter)?;
            let token_program = next_account_info(account_info_iter)?;

            let cpi_accounts = [
                mint_account.clone(),
                destination_account.clone(),
                authority_account.clone(),
            ];
            let cpi_program = token_program.clone();
            let ix_data = spl_token::instruction::mint_to(
                &token_program.key,
                &mint_account.key,
                &destination_account.key,
                &authority_account.key,
                &[],
                amount,
            )?;
            invoke(
                &ix_data,
                &[
                    mint_account.clone(),
                    destination_account.clone(),
                    authority_account.clone(),
                    token_program.clone(),
                ],
            )?;
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Instruction {
    Initialize,
    MintTo { amount: u64 },
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => Instruction::Initialize,
            1 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(ProgramError::InvalidInstructionData)?;
                Instruction::MintTo { amount }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

pub mod spl_token {
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
    };

    pub fn initialize_mint(
        token_program_id: &Pubkey,
        mint_pubkey: &Pubkey,
        mint_authority_pubkey: &Pubkey,
        freeze_authority_pubkey: Option<&Pubkey>,
        decimals: u8,