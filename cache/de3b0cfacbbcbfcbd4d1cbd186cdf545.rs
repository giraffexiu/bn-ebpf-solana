// Cached result for function: sub_1000014a8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: de3b0cfacbbcbfcbd4d1cbd186cdf545.rs

pub enum ErrorCode {
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Invalid account data")]
    InvalidAccountData,
    #[msg("Invalid program ID")]
    InvalidProgramId,
    #[msg("Invalid account key")]
    InvalidAccountKey,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Not rent exempt")]
    NotRentExempt,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account not writable")]
    AccountNotWritable,
    #[msg("Account not signer")]
    AccountNotSigner,
    #[msg("Account not executable")]
    AccountNotExecutable,
    #[msg("Account data too small")]
    AccountDataTooSmall,
    #[msg("Account data too large")]
    AccountDataTooLarge,
    #[msg("Account data not aligned")]
    AccountDataNotAligned,
    #[msg("Account data not empty")]
    AccountDataNotEmpty,
    #[msg("Account data not zeroed")]
    AccountDataNotZeroed,
    #[msg("Account data not mutable")]
    AccountDataNotMutable,
    #[msg("Account data not owned by program")]
    AccountDataNotOwnedByProgram,
    #[msg("Account data not owned by system program")]
    AccountDataNotOwnedBySystemProgram,
    #[msg("Account data not owned by token program")]
    AccountDataNotOwnedByTokenProgram,
    #[msg("Account data not owned by associated token program")]
    AccountDataNotOwnedByAssociatedTokenProgram,
    #[msg("Account data not owned by SPL governance program")]
    AccountDataNotOwnedBySplGovernanceProgram,
    #[msg("Account data not owned by SPL token program")]
    AccountDataNotOwnedBySplTokenProgram,
    #[msg("Account data not owned by SPL memo program")]
    AccountDataNotOwnedBySplMemoProgram,
    #[msg("Account data not owned by SPL system program")]
    AccountDataNotOwnedBySplSystemProgram,
    #[msg("Account data not owned by SPL stake program")]
    AccountDataNotOwnedBySplStakeProgram,
    #[msg("Account data not owned by SPL vote program")]
    AccountDataNotOwnedBySplVoteProgram,
    #[msg("Account data not owned by SPL config program")]
    AccountDataNotOwnedBySplConfigProgram,
    #[msg("Account data not owned by SPL feature program")]
    AccountDataNotOwnedBySplFeatureProgram,
    #[msg("Account data not owned by SPL bpf loader program")]
    AccountDataNotOwnedBySplBpfLoaderProgram,
    #[msg("Account data not owned by SPL bpf loader deprecated program")]
    AccountDataNotOwnedBySplBpfLoaderDeprecatedProgram,
    #[msg("Account data not owned by SPL bpf loader upgradeable program")]
    AccountDataNotOwnedBySplBpfLoaderUpgradeableProgram,
    #[msg("Account data not owned by SPL bpf loader extension program")]
    AccountDataNotOwnedBySplBpfLoaderExtensionProgram,
    #[msg("Account data not owned by SPL bpf loader v2 program")]
    AccountDataNotOwnedBySplBpfLoaderV2Program,
    #[msg("Account data not owned by SPL bpf loader v3 program")]
    AccountDataNotOwnedBySplBpfLoaderV3Program,
    #[msg("Account data not owned by SPL bpf loader v4 program")]
    AccountDataNotOwnedBySplBpfLoaderV4Program,
    #[msg("Account data not owned by SPL bpf loader v5 program")]
    AccountDataNotOwnedBySplBpfLoaderV5Program,
    #[msg("Account data not owned by SPL bpf loader v6 program")]
    AccountDataNotOwnedBySplBpfLoaderV6Program,
    #[msg("Account data not owned by SPL bpf loader v7 program")]
    AccountDataNotOwnedBySplBpfLoaderV7Program,
    #[msg("Account data not owned by SPL bpf loader v8 program")]
    AccountDataNotOwnedBySplBpfLoaderV