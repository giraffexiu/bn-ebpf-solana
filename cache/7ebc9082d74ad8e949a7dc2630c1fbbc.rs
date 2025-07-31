// Cached result for function: sub_100001950
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 7ebc9082d74ad8e949a7dc2630c1fbbc.rs

pub enum ErrorCode {
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Invalid account data")]
    InvalidAccountData,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account not rent exempt")]
    AccountNotRentExempt,
    #[msg("Account not writable")]
    AccountNotWritable,
    #[msg("Account not signer")]
    AccountNotSigner,
    #[msg("Account not executable")]
    AccountNotExecutable,
    #[msg("Account not system program")]
    AccountNotSystemProgram,
    #[msg("Account not sysvar")]
    AccountNotSysvar,
    #[msg("Account not program")]
    AccountNotProgram,
    #[msg("Account not token program")]
    AccountNotTokenProgram,
    #[msg("Account not associated token program")]
    AccountNotAssociatedTokenProgram,
    #[msg("Account not spl governance program")]
    AccountNotSplGovernanceProgram,
    #[msg("Account not clock sysvar")]
    AccountNotClockSysvar,
    #[msg("Account not rent sysvar")]
    AccountNotRentSysvar,
    #[msg("Account not epoch schedule sysvar")]
    AccountNotEpochScheduleSysvar,
    #[msg("Account not recent blockhashes sysvar")]
    AccountNotRecentBlockhashesSysvar,
    #[msg("Account not fees sysvar")]
    AccountNotFeesSysvar,
    #[msg("Account not rewards sysvar")]
    AccountNotRewardsSysvar,
    #[msg("Account not slot hashes sysvar")]
    AccountNotSlotHashesSysvar,
    #[msg("Account not stake history sysvar")]
    AccountNotStakeHistorySysvar,
    #[msg("Account not instructions sysvar")]
    AccountNotInstructionsSysvar,
    #[msg("Account not last restart slot sysvar")]
    AccountNotLastRestartSlotSysvar,
    #[msg("Account not feature sysvar")]
    AccountNotFeatureSysvar,
    #[msg("Account not vote program")]
    AccountNotVoteProgram,
    #[msg("Account not stake program")]
    AccountNotStakeProgram,
    #[msg("Account not config program")]
    AccountNotConfigProgram,
    #[msg("Account not bpf loader program")]
    AccountNotBpfLoaderProgram,
    #[msg("Account not bpf loader deprecated program")]
    AccountNotBpfLoaderDeprecatedProgram,
    #[msg("Account not bpf loader upgradeable program")]
    AccountNotBpfLoaderUpgradeableProgram,
    #[msg("Account not native loader program")]
    AccountNotNativeLoaderProgram,
    #[msg("Account not secp256k1 program")]
    AccountNotSecp256k1Program,
    #[msg("Account not ed25519 program")]
    AccountNotEd25519Program,
    #[msg("Account not keccak program")]
    AccountNotKeccakProgram,
    #[msg("Account not curve25519 program")]
    AccountNotCurve25519Program,
    #[msg("Account not sha256 program")]
    AccountNotSha256Program,
    #[msg("Account not sha512 program")]
    AccountNotSha512Program,
    #[msg("Account not blake3 program")]
    AccountNotBlake3Program,
    #[msg("Account not poseidon program")]
    AccountNotPoseidonProgram,
    #[msg("Account not schnorr program")]
    AccountNotSchnorrProgram,
    #[msg("Account not secp256r1 program")]
    AccountNotSecp256r1Program,
    #[msg("Account not secp256k1 program")]
    AccountNotSecp256k1Program,
    #[msg("Account not ed25519 program")]
    AccountNotEd25519Program,
    #[msg("Account not keccak program")]