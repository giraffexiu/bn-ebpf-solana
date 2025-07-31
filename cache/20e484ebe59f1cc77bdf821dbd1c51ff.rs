// Cached result for function: sub_100002200
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 20e484ebe59f1cc77bdf821dbd1c51ff.rs

pub enum ErrorCode {
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Invalid account data")]
    InvalidAccountData,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Account not writable")]
    AccountNotWritable,
    #[msg("Account not signer")]
    AccountNotSigner,
    #[msg("Account not executable")]
    AccountNotExecutable,
    #[msg("Account not rent exempt")]
    AccountNotRentExempt,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account data too small")]
    AccountDataTooSmall,
    #[msg("Account data too large")]
    AccountDataTooLarge,
    #[msg("Account data mismatch")]
    AccountDataMismatch,
    #[msg("Account data not empty")]
    AccountDataNotEmpty,
    #[msg("Account data not zeroed")]
    AccountDataNotZeroed,
    #[msg("Account data not aligned")]
    AccountDataNotAligned,
    #[msg("Account data not deserializable")]
    AccountDataNotDeserializable,
    #[msg("Account data not serializable")]
    AccountDataNotSerializable,
    #[msg("Account data not valid")]
    AccountDataNotValid,
    #[msg("Account data not found")]
    AccountDataNotFound,
    #[msg("Account data not unique")]
    AccountDataNotUnique,
    #[msg("Account data not mutable")]
    AccountDataNotMutable,
    #[msg("Account data not immutable")]
    AccountDataNotImmutable,
    #[msg("Account data not a program")]
    AccountDataNotAProgram,
    #[msg("Account data not a system account")]
    AccountDataNotASystemAccount,
    #[msg("Account data not a token account")]
    AccountDataNotATokenAccount,
    #[msg("Account data not an associated token account")]
    AccountDataNotAnAssociatedTokenAccount,
    #[msg("Account data not a mint account")]
    AccountDataNotAMintAccount,
    #[msg("Account data not a rent sysvar")]
    AccountDataNotARentSysvar,
    #[msg("Account data not a clock sysvar")]
    AccountDataNotAClockSysvar,
    #[msg("Account data not a stake sysvar")]
    AccountDataNotAStakeSysvar,
    #[msg("Account data not a vote sysvar")]
    AccountDataNotAVoteSysvar,
    #[msg("Account data not a recent blockhashes sysvar")]
    AccountDataNotARecentBlockhashesSysvar,
    #[msg("Account data not a program data account")]
    AccountDataNotAProgramDataAccount,
    #[msg("Account data not a buffer account")]
    AccountDataNotABufferAccount,
    #[msg("Account data not a BPF loader account")]
    AccountDataNotABpfLoaderAccount,
    #[msg("Account data not a BPF loader deprecated account")]
    AccountDataNotABpfLoaderDeprecatedAccount,
    #[msg("Account data not a BPF loader upgradeable account")]
    AccountDataNotABpfLoaderUpgradeableAccount,
    #[msg("Account data not a native program")]
    AccountDataNotANativeProgram,
    #[msg("Account data not a system program")]
    AccountData