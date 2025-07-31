// Cached result for function: sub_1000027a8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 279e24c3ae0af2fff57f371f4fcde56b.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw,
    #[msg("The given account is not owned by the executing program")]
    ConstraintOwner,
    #[msg("The given account is not a program account")]
    ConstraintExecutable,
    #[msg("The given account is not a signer")]
    ConstraintSigner,
    #[msg("The given account is not writable")]
    ConstraintWritable,
    #[msg("The given account is not initialized")]
    ConstraintZero,
    #[msg("The given account is already initialized")]
    ConstraintClose,
    #[msg("The given account is not of the correct type")]
    ConstraintType,
    #[msg("The given account has a data size that does not match the given constraint")]
    ConstraintDataLen,
    #[msg("The given account does not have the correct address")]
    ConstraintAddress,
    #[msg("The given account is not a mint account")]
    ConstraintMint,
    #[msg("The given account is not a associated token account")]
    ConstraintAssociatedToken,
    #[msg("The given account is not a token account")]
    ConstraintToken,
    #[msg("The given account is not a token program")]
    ConstraintTokenProgram,
    #[msg("The given account is not a system program")]
    ConstraintSystemProgram,
    #[msg("The given account is not a rent sysvar")]
    ConstraintRent,
    #[msg("The given account is not a associated token program")]
    ConstraintAssociatedTokenProgram,
    #[msg("The given account is not a wrapped sol mint")]
    ConstraintWrappedSolMint,
    #[msg("The given account is not a native token mint")]
    ConstraintNativeTokenMint,
    #[msg("The given account is not a program data account")]
    ConstraintProgramData,
    #[msg("The given account is not a program account")]
    ConstraintProgram,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccount,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountData,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountDataLen,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountOwner,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountExecutable,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountSigner,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountWritable,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountRentEpoch,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountLamports,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsInitialized,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsSigner,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsWritable,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsExecutable,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsRentEpoch,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsLamports,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsDataLen,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsOwner,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsProgram,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsProgramData,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsProgramAccount,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsProgramAccountData,
    #[msg("The given account is not a program account")]
    ConstraintProgramAccountIsProgram