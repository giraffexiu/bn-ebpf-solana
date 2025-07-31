// Cached result for function: sub_100002c00
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: eb23b1a735f4385089fb415238265d01.rs

pub enum ErrorCode {
    #[msg("Invalid account data length")]
    InvalidAccountDataLength,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Invalid account key")]
    InvalidAccountKey,
    #[msg("Account not writable")]
    AccountNotWritable,
    #[msg("Account not signer")]
    AccountNotSigner,
    #[msg("Account not executable")]
    AccountNotExecutable,
    #[msg("Account not rent exempt")]
    AccountNotRentExempt,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account data too small")]
    AccountDataTooSmall,
    #[msg("Account data too large")]
    AccountDataTooLarge,
    #[msg("Account data mismatch")]
    AccountDataMismatch,
    #[msg("Account data corrupted")]
    AccountDataCorrupted,
    #[msg("Account data invalid")]
    AccountDataInvalid,
    #[msg("Account data not deserializable")]
    AccountDataNotDeserializable,
    #[msg("Account data not serializable")]
    AccountDataNotSerializable,
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
    #[msg("Account data not owned by SPL rent program")]
    AccountDataNotOwnedBySplRentProgram,
    #[msg("Account data not owned by SPL clock program")]
    AccountDataNotOwnedBySplClockProgram,
    #[msg("Account data not owned by SPL epoch rewards program")]
    AccountDataNotOwnedBySplEpochRewardsProgram,
    #[msg("Account data not owned by SPL feature gate program")]
    AccountDataNotOwnedBySplFeatureGateProgram,
    #[msg("Account data not owned by SPL stake program")]
    AccountDataNotOwnedBySplStakeProgram,
    #[msg("Account data not owned by SPL vote program")]
    AccountDataNotOwnedBySplVoteProgram,
    #[msg("Account data not owned by SPL config program")]
    AccountDataNotOwnedBySplConfigProgram,
    #[msg("Account data not owned by SPL secp256k1 program")]
    AccountDataNotOwnedBySplSecp256k1Program,
    #[msg("Account data not owned by SPL ed25519 program")]
    AccountDataNotOwnedBySplEd25519Program,
    #[msg("Account data not owned by SPL keccak256 program")]
    AccountDataNotOwnedBySplKeccak256Program,
    #[msg("Account data not owned by SPL sha256 program")]
    AccountDataNotOwnedBySplSha256Program,
    #[msg("Account data not owned by SPL bls12381 program")]
    AccountDataNotOwnedBySplBls12381Program,
    #[msg("Account data not owned by SPL curve25519 program")]
    AccountDataNotOwnedBySplCurve25519Program,
    #[