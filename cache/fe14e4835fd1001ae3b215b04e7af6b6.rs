// Cached result for function: sub_100001790
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: fe14e4835fd1001ae3b215b04e7af6b6.rs

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MyInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MyInstruction::Initialize(args) => {
            initialize(program_id, accounts, args.value)
        }
        MyInstruction::Increment => {
            increment(program_id, accounts)
        }
        MyInstruction::Decrement => {
            decrement(program_id, accounts)
        }
        MyInstruction::Update(args) => {
            update(program_id, accounts, args.value)
        }
        MyInstruction::SetCloseAuthority => {
            set_close_authority(program_id, accounts)
        }
        MyInstruction::Close => {
            close(program_id, accounts)
        }
    }
}