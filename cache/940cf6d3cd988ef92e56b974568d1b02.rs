// Cached result for function: sub_1000012d0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 940cf6d3cd988ef92e56b974568d1b02.rs

pub enum ErrorCode {
    #[msg("Invalid account data")]
    InvalidAccountData,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Invalid program ID")]
    InvalidProgramId,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Account not writable")]
    AccountNotWritable,
    #[msg("Account not signer")]
    AccountNotSigner,
    #[msg("Account not rent exempt")]
    AccountNotRentExempt,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account not empty")]
    AccountNotEmpty,
    #[msg("Account not closed")]
    AccountNotClosed,
    #[msg("Account not mutable")]
    AccountNotMutable,
    #[msg("Account not a program")]
    AccountNotProgram,
    #[msg("Account not a system account")]
    AccountNotSystemAccount,
    #[msg("Account not a token account")]
    AccountNotTokenAccount,
    #[msg("Account not a mint account")]
    AccountNotMintAccount,
    #[msg("Account not a associated token account")]
    AccountNotAssociatedTokenAccount,
    #[msg("Account not a spl token program")]
    AccountNotSplTokenProgram,
    #[msg("Account not a system program")]
    AccountNotSystemProgram,
    #[msg("Account not a rent sysvar")]
    AccountNotRentSysvar,
    #[msg("Account not a clock sysvar")]
    AccountNotClockSysvar,
    #[msg("Account not a epoch schedule sysvar")]
    AccountNotEpochScheduleSysvar,
    #[msg("Account not a recent blockhashes sysvar")]
    AccountNotRecentBlockhashesSysvar,
    #[msg("Account not a fees sysvar")]
    AccountNotFeesSysvar,
    #[msg("Account not a rewards sysvar")]
    AccountNotRewardsSysvar,
    #[msg("Account not a slot hashes sysvar")]
    AccountNotSlotHashesSysvar,
    #[msg("Account not a slot history sysvar")]
    AccountNotSlotHistorySysvar,
    #[msg("Account not a stake history sysvar")]
    AccountNotStakeHistorySysvar,
    #[msg("Account not a stake pool sysvar")]
    AccountNotStakePoolSysvar,
    #[msg("Account not a stake config sysvar")]
    AccountNotStakeConfigSysvar,
    #[msg("Account not a stake program")]
    AccountNotStakeProgram,
    #[msg("Account not a vote program")]
    AccountNotVoteProgram,
    #[msg("Account not a bpf loader program")]
    AccountNotBpfLoaderProgram,
    #[msg("Account not a bpf loader deprecated program")]
    AccountNotBpfLoaderDeprecatedProgram,
    #[msg("Account not a bpf loader upgradeable program")]
    AccountNotBpfLoaderUpgradeableProgram,
    #[msg("Account not a native loader program")]
    AccountNotNativeLoaderProgram,
    #[msg("Account not a secp256k1 program")]
    AccountNotSecp256k1Program,
    #[msg("Account not a ed25519 program")]
    AccountNotEd25519Program,
    #[msg("Account not a feature gate program")]
    AccountNotFeatureGateProgram,
    #[msg("Account not a keccak program")]
    AccountNotKeccakProgram,
    #[msg("Account not a curve25519 program")]
    AccountNotCurve25519Program,
    #[msg("Account not a bls12381 program")]
    AccountNotBls12381Program,
    #[msg("Account not a poseidon program")]
    AccountNotPoseidonProgram,
    #[msg("Account not a sha256 program")]
    AccountNotSha256Program,
    #[msg("Account not a sha512 program