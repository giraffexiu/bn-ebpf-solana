// Cached result for function: sub_1000004b0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: c44145921d22cb1aa337bf4149620131.rs

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
    ConstraintMut,
    #[msg("A constraint was violated")]
    ConstraintOwner,
    #[msg("The program expected this account to be already initialized")]
    AccountNotInitialized,
    #[msg("The program expected this account to be mutable")]
    AccountNotMutable,
    #[msg("The program expected this account to be a signer")]
    AccountNotSigner,
    #[msg("The program expected this account to be writable")]
    AccountNotWritable,
    #[msg("The program expected this account to be executable")]
    AccountNotExecutable,
    #[msg("The program expected this account to be a program")]
    AccountNotProgram,
    #[msg("The program expected this account to be a system account")]
    AccountNotSystem,
    #[msg("The program expected this account to be a token account")]
    AccountNotToken,
    #[msg("The program expected this account to be a mint account")]
    AccountNotMint,
    #[msg("The program expected this account to be a associated token account")]
    AccountNotAssociatedToken,
    #[msg("The program expected this account to be a program data account")]
    AccountNotProgramData,
    #[msg("The program expected this account to be a token program")]
    AccountNotTokenProgram,
    #[msg("The program expected this account to be a system program")]
    AccountNotSystemProgram,
    #[msg("The program expected this account to be a rent sysvar")]
    AccountNotRentSysvar,
    #[msg("The program expected this account to be a clock sysvar")]
    AccountNotClockSysvar,
    #[msg("The program expected this account to be a epoch schedule sysvar")]
    AccountNotEpochScheduleSysvar,
    #[msg("The program expected this account to be a instructions sysvar")]
    AccountNotInstructionsSysvar,
    #[msg("The program expected this account to be a recent blockhashes sysvar")]
    AccountNotRecentBlockhashesSysvar,
    #[msg("The program expected this account to be a rewards sysvar")]
    AccountNotRewardsSysvar,
    #[msg("The program expected this account to be a slot hashes sysvar")]
    AccountNotSlotHashesSysvar,
    #[msg("The program expected this account to be a slot history sysvar")]
    AccountNotSlotHistorySysvar,
    #[msg("The program expected this account to be a stake history sysvar")]
    AccountNotStakeHistorySysvar,
    #[msg("The program expected this account to be a sysvar")]
    AccountNotSysvar,
    #[msg("The program expected this account to be a native mint")]
    AccountNotNativeMint,
    #[msg("The program expected this account to be a native token account")]
    AccountNotNativeTokenAccount,
    #[msg("The program expected this account to be a native token mint")]
    AccountNotNativeTokenMint,
    #[msg("The program expected this account to be a native token program")]
    AccountNotNativeTokenProgram,
    #[msg("The program expected this account to be a native token program data