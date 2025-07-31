// Cached result for function: sub_100002dc0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 6f62f2bb31d2aeba165cb3fe0b68862a.rs

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[repr(C)]
pub struct SolParameters {
    pub ka: *mut AccountInfo<'static>,
    pub ka_num: u64,
    pub data: *const u8,
    pub data_len: u64,
    pub program_id: *const Pubkey,
}

#[repr(C)]
pub struct FixedAccountInfo {
    pub accounts: [AccountInfo<'static>; 32],
}

#[repr(C)]
pub struct FixedAccountMeta {
    pub metas: [SolAccountMeta; 0x100],
}

#[repr(C)]
pub struct SolAccountMeta {
    pub pubkey: *mut Pubkey,
    pub is_writable: bool,
    pub is_signer: bool,
}

#[repr(C)]
pub struct SolInstruction {
    pub program_id: *mut Pubkey,
    pub accounts: *mut FixedAccountMeta,
    pub account_len: u64,
    pub data: *mut u8,
    pub data_len: u64,
}

#[repr(C)]
pub struct SolSignerSeed {
    pub addr: *const u8,
    pub len: u64,
}

#[repr(C)]
pub struct SolSignerSeeds {
    pub addr: *const SolSignerSeed,
    pub len: u64,
}

#[repr(C)]
pub struct SolBytes {
    pub addr: *const u8,
    pub len: u64,
}

#[repr(C)]
pub struct SolPubkey {
    pub x: [u8; 32],
}

#[repr(C)]
pub struct SolAccountInfo {
    pub key: *mut SolPubkey,
    pub lamports: *mut u64,
    pub data_len: u64,
    pub data: *mut u8,
    pub owner: *mut SolPubkey,
    pub rent_epoch: u64,
    pub is_signer: bool,
    pub is_writable: bool,
    pub executable: bool,
}

#[repr(C)]
pub struct InstructionData {
    pub instruction_discriminator: u8,
    pub amount: u64,
}

#[no_mangle]
pub unsafe extern "C" fn sub_100002dc0(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction_discriminator = *instruction_data.get_unchecked(0);
    let instruction_data_ptr = instruction_data.as_ptr();

    if instruction_discriminator == 0 {
        // Process `Initialize` instruction
        let accounts_iter = &mut accounts.iter();
        let _payer_account = next_account_info(accounts_iter)?;
        let _vault_account = next_account_info(accounts_iter)?;
        let _system_program = next_account_info(accounts_iter)?;

        // No specific logic for Initialize in this snippet, usually involves account creation/initialization
        Ok(())
    } else if instruction_discriminator == 1 {
        // Process `Deposit` instruction
        let accounts_iter = &mut accounts.iter();
        let _depositor_account = next_account_info(accounts_iter)?;
        let vault_account = next_account_info(