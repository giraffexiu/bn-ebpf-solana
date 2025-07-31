// Cached result for function: sub_100001730
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 1c157a799a19ad05a1f49651f93ff79a.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw,
    #[msg("A constraint was violated")]
    ConstraintHasOne,
    #[msg("A constraint was violated")]
    ConstraintSeeds,
    #[msg("A constraint was violated")]
    ConstraintTokenMint,
    #[msg("A constraint was violated")]
    ConstraintTokenOwner,
    #[msg("A constraint was violated")]
    ConstraintAssociated,
    #[msg("A constraint was violated")]
    ConstraintRentExempt,
    #[msg("A constraint was violated")]
    ConstraintSigner,
    #[msg("A constraint was violated")]
    ConstraintWritable,
    #[msg("A constraint was violated")]
    ConstraintExecutable,
    #[msg("A constraint was violated")]
    ConstraintState,
    #[msg("A constraint was violated")]
    ConstraintClosed,
    #[msg("A constraint was violated")]
    ConstraintZeroCopy,
    #[msg("A constraint was violated")]
    ConstraintOwner,
    #[msg("Program ID was not as expected")]
    ConstraintProgramId,
    #[msg("The given account is not a program account")]
    AccountNotProgram,
    #[msg("The given account is not owned by the executing program")]
    AccountNotOwnedByProgram,
    #[msg("Program failed to create account with provided seed")]
    InvalidProgramId,
    #[msg("The given account is not a program account")]
    InvalidAccountLen,
    #[msg("The given account is not a program account")]
    InvalidAccountData,
    #[msg("The given account is not a program account")]
    InvalidAccountDiscriminator,
    #[msg("The given account is not a program account")]
    InvalidAccountAddress,
    #[msg("The given account is not a program account")]
    InvalidAccountOwner,
    #[msg("The given account is not a program account")]
    InvalidAccountInfo,
    #[msg("The given account is not a program account")]
    InvalidAccountKey,
    #[msg("The given account is not a program account")]
    InvalidAccountClose,
    #[msg("The given account is not a program account")]
    InvalidAccountRentExemption,
    #[msg("The given account is not a program account")]
    InvalidAccountSpace,
    #[msg("The given account is not a program account")]
    InvalidAccountMut,
    #[msg("The given account is not a program account")]
    InvalidAccountNotProgram,
    #[msg("The given account is not a program account")]
    InvalidAccountNotOwnedByProgram,
    #[msg("The given account is not a program account")]
    InvalidAccountNotSigner,
    #[msg("The given account is not a program account")]
    InvalidAccountNotWritable,
    #[msg("The given account is not a program account")]
    InvalidAccountNotExecutable,
    #[msg("The given account is not a program account")]
    InvalidAccountNotClosed,
    #[msg("The given account is not a program account")]
    InvalidAccountNotAssociated,
    #[msg("The given account is not a program account")]
    InvalidAccountNotRentExempt,
    #[msg("The given account is not a program account")]
    InvalidAccountNotTokenMint,
    #[msg("The given account is not a program account")]
    InvalidAccountNotTokenOwner,
    #[msg("The given account is not a program account")]
    InvalidAccountNotAssociatedTokenAccount,
    #[msg("The given account is not a program account")]
    InvalidAccountNotSystemAccount,
    #[msg("The given account is not a program account")]
    InvalidAccountNotUninitialized,
    #[msg("The given account is not a program account")]
    InvalidAccountNotProgramOwned,
    #[msg("The given account is not a program account")]
    InvalidAccountNotProgramOwnedByProgram,
    #[msg("The given account is not a program account")]
    InvalidAccountNotProgramOwnedBySystem,
    #[msg("The given account is not a program account")]
    InvalidAccountNotProgramOwnedByToken,
    #[msg("The given account is not a program account")]
    InvalidAccountNotProgramOwnedByAssociatedToken,
    #[msg("The given account is not a program account")]
    InvalidAccount