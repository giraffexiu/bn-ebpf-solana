// Cached result for function: sub_1000004f0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 894c193a8f168cbb5674bbb21904a5fb.rs

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
    ConstraintMintOrFreezeAuthority,
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
    ConstraintZeroCopy,
    #[msg("A constraint was violated")]
    ConstraintMut,
    #[msg("A constraint was violated")]
    ConstraintClosed,
    #[msg("A constraint was violated")]
    ConstraintAddress,
    #[msg("A constraint was violated")]
    ConstraintLiteral,
    #[msg("The program expected this account to be already initialized")]
    AccountNotInitialized,
    #[msg("The program expected this account to be uninitialized")]
    AccountAlreadyInitialized,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    AccountDiscriminatorMismatch,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    AccountDiscriminatorNotFound,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    AccountDiscriminatorCorrect,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    AccountNotMutable,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    AccountNotProgramOwned,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidProgramId,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidTokenMint,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidTokenOwner,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidMintAuthority,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidFreezeAuthority,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidVaultAuthority,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidAssociatedTokenAddress,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidAutoDelegate,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidSeeds,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    InvalidMetadata,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintTokenAccount,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintEscrow,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintOverflow,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintAccountIsPda,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintAccountNotPda,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintRawLen,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintRawType,
    #[msg("The program expected the associated token account to be owned by the given wallet")]
    ConstraintTokenClose,
    #[msg("The program expected the associated token account