// Cached result for function: sub_100001bd8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 3d609b9f52523329b84867eadfcabed9.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw,
    #[msg("An owner constraint was violated")]
    ConstraintOwner,
    #[msg("A close constraint was violated")]
    ConstraintClose,
    #[msg("An address constraint was violated")]
    ConstraintAddress,
    #[msg("A zero copy constraint was violated")]
    ConstraintZeroCopy,
    #[msg("A associated token account constraint was violated")]
    ConstraintAssociated,
    #[msg("A token mint constraint was violated")]
    ConstraintTokenMint,
    #[msg("A token account constraint was violated")]
    ConstraintTokenAccount,
    #[msg("A signer constraint was violated")]
    ConstraintSigner,
    #[msg("An account has not been initialized")]
    AccountNotInitialized,
    #[msg("The program expected this account to be already initialized")]
    AccountAlreadyInitialized,
    #[msg("The program expected the account to be mutable")]
    AccountMutable,
    #[msg("The program expected the account to be a signer")]
    AccountNotSigner,
    #[msg("The program expected the account to be writable")]
    AccountNotWritable,
    #[msg("The given account is not owned by the executing program")]
    AccountNotProgramOwned,
    #[msg("Program ID was not as expected")]
    InvalidProgramId,
    #[msg("The given account is not a program account")]
    InvalidProgramAccount,
    #[msg("The given account is not a system account")]
    InvalidSystemAccount,
    #[msg("The given account is not a sysvar account")]
    InvalidSysvar,
    #[msg("The given account is not a token account")]
    InvalidTokenAccount,
    #[msg("The given account is not a token mint")]
    InvalidTokenMint,
    #[msg("The given account is not a associated token account")]
    InvalidAssociatedTokenAccount,
    #[msg("The given account is not a rent sysvar")]
    InvalidRentSysvar,
    #[msg("The given account is not a clock sysvar")]
    InvalidClockSysvar,
    #[msg("The given account is not a epoch schedule sysvar")]
    InvalidEpochScheduleSysvar,
    #[msg("The given account is not a instructions sysvar")]
    InvalidInstructionsSysvar,
    #[msg("The given account is not a recent blockhashes sysvar")]
    InvalidRecentBlockhashesSysvar,
    #[msg("The given account is not a rewards sysvar")]
    InvalidRewardsSysvar,
    #[msg("The given account is not a stake history sysvar")]
    InvalidStakeHistorySysvar,
    #[msg("The given account is not a anchor discriminator")]
    InvalidDiscriminator,
    #[msg("The given account has a data size different from what was expected")]
    DiscriminatorMismatch,
    #[msg("The given account has a data size different from what was expected")]
    AccountDiscriminatorMismatch,
    #[msg("The given account is not a program data account")]
    InvalidProgramDataAccount,
    #[msg("The given account is not a BPF upgradeable loader account")]
    InvalidUpgradeableLoader,
    #[msg("The given account is not a state account")]
    InvalidState,
    #[msg("The given account is not a delegate")]
    NoDelegate,
    #[msg("The given account has no owner")]
    NoOwner,
    #[msg("The given account has no data")]
    NoData,
    #[msg("The given account has no lamports")]
    NoLamports,
    #[msg("The given account has no rent epoch")]
    NoRentEpoch,
    #[msg("The given account has no executable")]
    NoExecutable,
    #[msg("The given account has no rent exemption")]
    NotRentExempt,
    #[msg("The given account has no space")]
    NoSpace,
    #[msg("The given account has no data length")]
    NoDataLength,
    #[msg("The given account has no owner program")]
    NoOwnerProgram,
    #[msg("The given account has no program data")]
    No