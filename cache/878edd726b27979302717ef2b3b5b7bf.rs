// Cached result for function: sub_100001938
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 878edd726b27979302717ef2b3b5b7bf.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw = 2000,
    #[msg("Account does not have the required owner")]
    ConstraintHasOne = 2001,
    #[msg("Account is not initialized")]
    ConstraintInitialized = 2002,
    #[msg("The given account is not a program data account")]
    ConstraintProgramIsProgram = 2003,
    #[msg("The given account is not a program account")]
    ConstraintProgramIsProgramData = 2004,
    #[msg("The given account is not a program account")]
    ConstraintProgramIsProgramOrProgramData = 2005,
    #[msg("The given account is not a program account")]
    ConstraintAssociated = 2006,
    #[msg("The given account is not a program account")]
    ConstraintTokenMint = 2007,
    #[msg("The given account is not a program account")]
    ConstraintTokenOwner = 2008,
    #[msg("The given account is not a program account")]
    ConstraintMintHasOne = 2009,
    #[msg("The given account is not a program account")]
    ConstraintSigner = 2010,
    #[msg("The given account is not a program account")]
    ConstraintRentExempt = 2011,
    #[msg("The given account is not a program account")]
    ConstraintSeeds = 2012,
    #[msg("The given account is not a program account")]
    ConstraintExecutable = 2013,
    #[msg("The given account is not a program account")]
    ConstraintState = 2014,
    #[msg("The given account is not a program account")]
    ConstraintClosed = 2015,
    #[msg("The given account is not a program account")]
    ConstraintZeroCopy = 2016,
    #[msg("The given account is not a program account")]
    ConstraintOwner = 2017,
    #[msg("The given account is not a program account")]
    ConstraintMut = 2018,
    #[msg("The given account is not a program account")]
    ConstraintDiscriminator = 2019,
    #[msg("The given account is not a program account")]
    ConstraintTokenClose = 2020,
    #[msg("The given account is not a program account")]
    ConstraintTokenFreeze = 2021,
    #[msg("The given account is not a program account")]
    ConstraintTokenBurn = 2022,
    #[msg("The given account is not a program account")]
    ConstraintTokenThaw = 2023,
    #[msg("The given account is not a program account")]
    ConstraintTokenTransfer = 2024,
    #[msg("The given account is not a program account")]
    ConstraintTokenApprove = 2025,
    #[msg("The given account is not a program account")]
    ConstraintTokenRevoke = 2026,
    #[msg("The given account is not a program account")]
    ConstraintTokenSetAuthority = 2027,
    #[msg("The given account is not a program account")]
    ConstraintTokenMintTo = 2028,
    #[msg("The given account is not a program account")]
    ConstraintTokenBurnFrom = 2029,
    #[msg("The given account is not a program account")]
    ConstraintTokenTransferFrom = 2030,
    #[msg("The given account is not a program account")]
    ConstraintTokenApproveDelegate = 2031,
    #[msg("The given account is not a program account")]
    ConstraintTokenRevokeDelegate = 2032,
    #[msg("The given account is not a program account")]
    ConstraintTokenSetAuthorityDelegate = 2033,
    #[msg("The given account is not a program account")]
    ConstraintTokenMintToDelegate = 2034,