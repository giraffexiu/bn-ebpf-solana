// Cached result for function: sub_100001498
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 64961b9a136e5be2959e912316975d48.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw = 2000,
    #[msg("An owner constraint was violated")]
    ConstraintHasOne = 2001,
    #[msg("A seed constraint was violated")]
    ConstraintSeeds = 2002,
    #[msg("A PDA constraint was violated")]
    ConstraintPda = 2003,
    #[msg("A zero copy constraint was violated")]
    ConstraintZeroCopy = 2004,
    #[msg("An associated token account constraint was violated")]
    ConstraintAssociated = 2005,
    #[msg("A token mint constraint was violated")]
    ConstraintTokenMint = 2006,
    #[msg("A token account constraint was violated")]
    ConstraintTokenAccount = 2007,
    #[msg("A signer constraint was violated")]
    ConstraintSigner = 2008,
    #[msg("An account must be empty")]
    ConstraintEmpty = 2009,
    #[msg("A solana native account must be a program account")]
    ConstraintProgram = 2010,
    #[msg("A solana native account must be a system account")]
    ConstraintSystem = 2011,
    #[msg("A solana native account must be a cpi account")]
    ConstraintCpi = 2012,
    #[msg("A solana native account must be a non-cpi account")]
    ConstraintNonCpi = 2013,
    #[msg("An account required to be mutable was not marked as mutable")]
    ConstraintMut = 2014,
    #[msg("An account required to be a close_authority was not marked as a close_authority")]
    ConstraintClose = 2015,
    #[msg("An account required to be a rent exempt was not marked as a rent exempt")]
    ConstraintRentExempt = 2016,
    #[msg("The program expected this account to be already initialized")]
    AccountNotInitialized = 2017,
    #[msg("The program expected this account to be uninitialized")]
    AccountInitialized = 2018,
    #[msg("The given account is not owned by the executing program")]
    AccountNotProgramOwned = 2019,
    #[msg("The given account is not a program account")]
    InvalidProgramId = 2020,
    #[msg("The given account is not a system account")]
    InvalidSystemAccount = 2021,
    #[msg("The given account is not a token account")]
    InvalidTokenAccount = 2022,
    #[msg("The given account is not a token mint")]
    InvalidMint = 2023,
    #[msg("The given account is not a associated token account")]
    InvalidAssociatedTokenAccount = 2024,
    #[msg("The given account has a data type that does not match the instruction")]
    InvalidAccountData = 2025,
    #[msg("The given account has a different owner than expected")]
    InvalidAccountOwner = 2026,
    #[msg("The program expected the associated token account to be empty, but it had a balance")]
    StateInvalidTokenAccountBalance = 2027,
    #[msg("The program expected the mint to have a different freeze authority")]
    StateInvalidMintFreezeAuthority = 2028,
    #[msg("The program expected the mint to have a different mint authority")]
    StateInvalidMintAuthority = 2029,
    #[msg("The program expected the mint to have a different supply")]
    StateInvalidMintSupply = 2030,
    #[msg("The program expected the mint to have a different decimals")]
    StateInvalidMintDecimals = 2031,
    #[msg("The program expected the mint to have a different is_initialized")]
    StateInvalidMintIsInitialized = 2032,
    #[msg