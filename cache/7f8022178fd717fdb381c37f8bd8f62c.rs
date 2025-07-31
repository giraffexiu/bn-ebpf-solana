// Cached result for function: sub_1000004e0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 7f8022178fd717fdb381c37f8bd8f62c.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw = 2000,
    #[msg("An owner constraint was violated")]
    ConstraintOwner = 2001,
    #[msg("A close constraint was violated")]
    ConstraintClose = 2002,
    #[msg("A zero copy constraint was violated")]
    ConstraintZeroCopy = 2003,
    #[msg("A token mint constraint was violated")]
    ConstraintTokenMint = 2004,
    #[msg("A token owner constraint was violated")]
    ConstraintTokenOwner = 2005,
    #[msg("A token account was not empty")]
    ConstraintTokenNotEmpty = 2006,
    #[msg("An associated token account constraint was violated")]
    ConstraintAssociatedToken = 2007,
    #[msg("A signer constraint was violated")]
    ConstraintSigner = 2008,
    #[msg("An account has not been initialized")]
    ConstraintInitialized = 2009,
    #[msg("The provided account is not a program account")]
    ConstraintProgram = 2010,
    #[msg("A seed constraint was violated")]
    ConstraintSeeds = 2011,
    #[msg("A PDA constraint was violated")]
    ConstraintPda = 2012,
    #[msg("A rent exemption constraint was violated")]
    ConstraintRentExemption = 2013,
    #[msg("A data constraint was violated")]
    ConstraintData = 2014,
    #[msg("A space constraint was violated")]
    ConstraintSpace = 2015,
    #[msg("The account discriminator was invalid")]
    AccountDiscriminatorMismatch = 2016,
    #[msg("The account discriminator was already set")]
    AccountDiscriminatorAlreadySet = 2017,
    #[msg("The account data is not of the expected size")]
    AccountDidNotDeserialize = 2018,
    #[msg("The account data start was invalid")]
    AccountNotInitialized = 2019,
    #[msg("The account is not mutable")]
    AccountNotMutable = 2020,
    #[msg("The account is not a signer")]
    AccountNotSigner = 2021,
    #[msg("The account is not writable")]
    AccountNotWritable = 2022,
    #[msg("The account's owner is not the current program")]
    AccountNotProgramOwned = 2023,
    #[msg("Program ID was not as expected")]
    InvalidProgramId = 2024,
    #[msg("The instruction data was not understood")]
    InvalidInstructionData = 2025,
    #[msg("The provided account is not a system account")]
    InvalidAccountLen = 2026,
    #[msg("The provided account is not a system account")]
    InvalidAccountData = 2027,
    #[msg("The provided account is not a system account")]
    InvalidAccountCloseData = 2028,
    #[msg("The provided account is not a system account")]
    InvalidAccountRealloc = 2029,
    #[msg("The provided account is not a system account")]
    InvalidAccountHeader = 2030,
    #[msg("The provided account is not a system account")]
    InvalidAccountDiscriminator = 2031,
    #[msg("The provided account is not a system account")]
    InvalidAccountKey = 2032,
    #[msg("The provided account is not a system account")]
    InvalidAccountOwner = 2033,
    #[msg("The provided account is not a system account")]
    InvalidAccountRentExemption = 2034,
    #[msg("The provided account is not a system account")]
    InvalidAccountSpace = 2035,
    #[msg("The provided account is not a system account")]
    InvalidAccountDataLen = 2036,