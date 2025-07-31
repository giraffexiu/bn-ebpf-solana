// Cached result for function: sub_100001918
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: ba27f1a25981e2ff921b670d1e77e19d.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TokenInstruction::unpack(instruction_data)?;

    match instruction {
        TokenInstruction::InitializeMint {
            decimals,
            mint_authority,
            freeze_authority,
        } => initialize_mint(
            program_id,
            &accounts[0],
            &accounts[1],
            decimals,
            &mint_authority,
            freeze_authority.as_ref(),
        ),
        TokenInstruction::InitializeAccount { owner } => {
            initialize_account(program_id, &accounts[0], &accounts[1], &owner)
        }
        TokenInstruction::InitializeMultisig { m } => {
            initialize_multisig(program_id, &accounts[0], &accounts[1..], m)
        }
        TokenInstruction::Transfer { amount } => {
            transfer(program_id, &accounts[0], &accounts[1], &accounts[2..], amount)
        }
        TokenInstruction::Approve { amount } => {
            approve(program_id, &accounts[0], &accounts[1], &accounts[2..], amount)
        }
        TokenInstruction::Revoke => revoke(program_id, &accounts[0], &accounts[1..]),
        TokenInstruction::SetAuthority {
            new_authority,
            authority_type,
        } => set_authority(
            program_id,
            &accounts[0],
            &accounts[1..],
            authority_type,
            new_authority.as_ref(),
        ),
        TokenInstruction::MintTo { amount } => {
            mint_to(program_id, &accounts[0], &accounts[1], &accounts[2..], amount)
        }
        TokenInstruction::Burn { amount } => {
            burn(program_id, &accounts[0], &accounts[1], &accounts[2..], amount)
        }
        TokenInstruction::CloseAccount => close_account(program_id, &accounts[0], &accounts[1], &accounts[2..]),
        TokenInstruction::FreezeAccount => freeze_account(program_id, &accounts[0], &accounts[1], &accounts[2..]),
        TokenInstruction::ThawAccount => thaw_account(program_id, &accounts[0], &accounts[1], &accounts[2..]),
        TokenInstruction::TransferChecked { amount, decimals } => transfer_checked(
            program_id,
            &accounts[0],
            &accounts[1],
            &accounts[2],
            &accounts[3..],
            amount,
            decimals,
        ),
        TokenInstruction::ApproveChecked { amount, decimals } => approve_checked(
            program_id,
            &accounts[0],
            &accounts[1],
            &accounts[2],
            &accounts[3..],
            amount,
            decimals,
        ),
        TokenInstruction::MintToChecked { amount, decimals } => mint_to_checked(
            program_id,
            &accounts[0],
            &accounts[1],
            &accounts[2..],
            amount,
            decimals,
        ),
        TokenInstruction::BurnChecked { amount, decimals } => burn_checked(
            program_id,
            &accounts[0],
            &accounts[1],
            &accounts[2],
            &accounts[3..],
            amount,
            decimals,
        ),
        TokenInstruction::InitializeAccount2 { owner } => {
            initialize_account2(program_id, &accounts[0], &accounts[1], &owner)
        }
        TokenInstruction::SyncNative => sync_native(program_id, &accounts[0]),
        TokenInstruction::InitializeMint2 {
            decimals,
            mint_authority,
            freeze_authority,
        } => initialize_mint2(
            program_id,
            &accounts[0],
            decimals,
            &mint_authority,
            freeze_authority.as_ref(),
        ),
    }
}