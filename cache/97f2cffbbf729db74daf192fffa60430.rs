// Cached result for function: sub_1000019b0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 97f2cffbbf729db74daf192fffa60430.rs

pub enum ErrorCode {
    #[msg("A raw constraint was violated")]
    ConstraintRaw = 2000,
    #[msg("An owner constraint was violated")]
    ConstraintOwner = 2001,
    #[msg("A close constraint was violated")]
    ConstraintClose = 2002,
    #[msg("A signer constraint was violated")]
    ConstraintSigner = 2003,
    #[msg("An account has not been initialized")]
    ConstraintInitialized = 2004,
    #[msg("The associated token account has not been initialized")]
    ConstraintATAInitialized = 2005,
    #[msg("A seed constraint was violated")]
    ConstraintSeeds = 2006,
    #[msg("A PDA constraint was violated")]
    ConstraintPda = 2007,
    #[msg("A rent exemption constraint was violated")]
    ConstraintRentExemption = 2008,
    #[msg("A zero copy constraint was violated")]
    ConstraintZeroCopy = 2009,
    #[msg("A token mint constraint was violated")]
    ConstraintTokenMint = 2010,
    #[msg("A token owner constraint was violated")]
    ConstraintTokenOwner = 2011,
    #[msg("A token account amount constraint was violated")]
    ConstraintTokenAmount = 2012,
    #[msg("A token account state constraint was violated")]
    ConstraintTokenState = 2013,
    #[msg("A associated token account constraint was violated")]
    ConstraintAssociated = 2014,
    #[msg("A program ID constraint was violated")]
    ConstraintProgramId = 2015,
    #[msg("A writable constraint was violated")]
    ConstraintWritable = 2016,
    #[msg("An executable constraint was violated")]
    ConstraintExecutable = 2017,
    #[msg("A state constraint was violated")]
    ConstraintState = 2018,
    #[msg("An address constraint was violated")]
    ConstraintAddress = 2019,
    #[msg("An account not closeable constraint was violated")]
    ConstraintNotCloseable = 2020,
    #[msg("The account discriminator was invalid")]
    InvalidAccountDiscriminator = 2021,
    #[msg("The account BPF instruction data was invalid")]
    InvalidInstructionData = 2022,
    #[msg("The account BPF instruction data was invalid")]
    InvalidInstruction = 2023,
    #[msg("The account BPF instruction data was invalid")]
    InvalidProgramId = 2024,
    #[msg("The account BPF instruction data was invalid")]
    InvalidAccountLen = 2025,
    #[msg("The account BPF instruction data was invalid")]
    InvalidAccountData = 2026,
    #[msg("The account BPF instruction data was invalid")]
    InvalidAccountKey = 2027,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotMutable = 2028,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotProgram = 2029,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotSystem = 2030,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotToken = 2031,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotAssociatedToken = 2032,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotUninitialized = 2033,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotRentExempt = 2034,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotOwnedByProgram = 2035,
    #[msg("The account BPF instruction data was invalid")]
    AccountNotSigner = 2036,