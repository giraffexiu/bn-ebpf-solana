// Cached result for function: sub_100000518
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: e4e9046a2a26eb99cfc6550a4ade124d.rs

pub enum ErrorCode {
    InvalidInstruction = 0,
}

pub struct ProgramState {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub token_account: Pubkey,
}

impl ProgramState {
    pub const LEN: usize = 1 + 32 + 32 + 32; // is_initialized + authority + mint + token_account
}

pub enum Instruction {
    Initialize {
        authority: Pubkey,
        mint: Pubkey,
        token_account: Pubkey,
    },
    Deposit {
        amount: u64,
    },
    Withdraw {
        amount: u64,
    },
}

impl Instruction {
    pub const INITIALIZE_DISCRIMINATOR: u8 = 0;
    pub const DEPOSIT_DISCRIMINATOR: u8 = 1;
    pub const WITHDRAW_DISCRIMINATOR: u8 = 2;
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), u64> {
    if instruction_data.is_empty() {
        return Err(ErrorCode::InvalidInstruction as u64);
    }

    let instruction_discriminator = instruction_data[0];
    let instruction_payload = &instruction_data[1..];

    match instruction_discriminator {
        Instruction::INITIALIZE_DISCRIMINATOR => {
            if instruction_payload.len() != 32 + 32 + 32 {
                return Err(ErrorCode::InvalidInstruction as u64);
            }

            let authority_bytes: [u8; 32] = instruction_payload[0..32].try_into().unwrap();
            let mint_bytes: [u8; 32] = instruction_payload[32..64].try_into().unwrap();
            let token_account_bytes: [u8; 32] = instruction_payload[64..96].try_into().unwrap();

            let authority = Pubkey::new_from_array(authority_bytes);
            let mint = Pubkey::new_from_array(mint_bytes);
            let token_account = Pubkey::new_from_array(token_account_bytes);

            process_initialize(
                program_id,
                accounts,
                authority,
                mint,
                token_account,
            )
        }
        Instruction::DEPOSIT_DISCRIMINATOR => {
            if instruction_payload.len() != 8 {
                return Err(ErrorCode::InvalidInstruction as u64);
            }
            let amount = u64::from_le_bytes(instruction_payload[0..8].try_into().unwrap());
            process_deposit(program_id, accounts, amount)
        }
        Instruction::WITHDRAW_DISCRIMINATOR => {
            if instruction_payload.len() != 8 {
                return Err(ErrorCode::InvalidInstruction as u64);
            }
            let amount = u64::from_le_bytes(instruction_payload[0..8].try_into().unwrap());
            process_withdraw(program_id, accounts, amount)
        }
        _ => Err(ErrorCode::InvalidInstruction as u64),
    }
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
) -> Result<(), u64> {
    // Account validation and logic for initialization
    // This is a placeholder for the