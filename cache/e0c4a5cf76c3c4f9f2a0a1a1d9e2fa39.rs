// Cached result for function: sub_100001908
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: e0c4a5cf76c3c4f9f2a0a1a1d9e2fa39.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw,
    #[msg("An owner constraint was violated")]
    ConstraintOwner,
    #[msg("A close constraint was violated")]
    ConstraintClose,
    #[msg("An address constraint was violated")]
    ConstraintAddress,
    #[msg("An associated token account constraint was violated")]
    ConstraintAssociated,
    #[msg("A token mint constraint was violated")]
    ConstraintTokenMint,
    #[msg("A token account constraint was violated")]
    ConstraintTokenAccount,
    #[msg("A signer constraint was violated")]
    ConstraintSigner,
    #[msg("A solana_program::system_program::ID constraint was violated")]
    ConstraintSystem,
    #[msg("A solana_program::token::ID constraint was violated")]
    ConstraintToken,
    #[msg("A solana_program::rent::ID constraint was violated")]
    ConstraintRent,
    #[msg("A constraint was violated")]
    ConstraintSeeds,
    #[msg("A constraint was violated")]
    ConstraintExecutable,
    #[msg("A constraint was violated")]
    ConstraintState,
    #[msg("A constraint was violated")]
    ConstraintZeroCopy,
    #[msg("A constraint was violated")]
    ConstraintMut,
    #[msg("A constraint was violated")]
    ConstraintHasOne,
    #[msg("A constraint was violated")]
    ConstraintForRentExemption,
    #[msg("A constraint was violated")]
    ConstraintLiteral,
    #[msg("The program expected this account to be already initialized")]
    AccountNotInitialized,
    #[msg("The program expected this account to be mutable")]
    AccountNotMutable,
    #[msg("The program expected this account to be a signer")]
    AccountNotSigner,
    #[msg("The program expected this account to be writable")]
    AccountNotWritable,
    #[msg("The given account is not owned by the executing program")]
    AccountNotProgramOwned,
    #[msg("The given account is not a program account")]
    InvalidProgramId,
    #[msg("The given account is not a token account")]
    InvalidTokenAccount,
    #[msg("The given account is not a token mint")]
    InvalidTokenMint,
    #[msg("The given account is not a system account")]
    InvalidSystemAccount,
    #[msg("The given account is not a rent sysvar")]
    InvalidRentSysvar,
    #[msg("The given account is not a associated token account")]
    InvalidAssociatedTokenAccount,
    #[msg("The given account is not a program data account")]
    InvalidProgramDataAccount,
    #[msg("The given account is not a program account")]
    InvalidProgramAccount,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountData,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountOwner,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountDiscriminator,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountSize,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountClose,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountHasOne,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountForRentExemption,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountLiteral,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountRaw,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountOwnerMismatch,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountAddressMismatch,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountExecutableMismatch,
    #[msg("The given account is not a program account")]
    InvalidProgramAccountStateMismatch,
    #[msg("The given account is not