// Cached result for function: sub_1000019c0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 0f56c8713baba8d04f93a2a23ecb11d8.rs

pub enum ErrorCode {
    #[msg("Invalid program ID")]
    InvalidProgramId,
    #[msg("Invalid account data")]
    InvalidAccountData,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Not rent exempt")]
    NotRentExempt,
    #[msg("Constraint raw")]
    ConstraintRaw,
    #[msg("Constraint has one")]
    ConstraintHasOne,
    #[msg("Constraint has many")]
    ConstraintHasMany,
    #[msg("Constraint signer")]
    ConstraintSigner,
    #[msg("Constraint writable")]
    ConstraintWritable,
    #[msg("Constraint executable")]
    ConstraintExecutable,
    #[msg("Constraint state")]
    ConstraintState,
    #[msg("Constraint associated")]
    ConstraintAssociated,
    #[msg("Constraint associated init")]
    ConstraintAssociatedInit,
    #[msg("Constraint close")]
    ConstraintClose,
    #[msg("Constraint address")]
    ConstraintAddress,
    #[msg("Constraint zero")]
    ConstraintZero,
    #[msg("Constraint token mint")]
    ConstraintTokenMint,
    #[msg("Constraint token owner")]
    ConstraintTokenOwner,
    #[msg("Constraint mint init")]
    ConstraintMintInit,
    #[msg("Constraint associated token")]
    ConstraintAssociatedToken,
    #[msg("Constraint mint has space")]
    ConstraintMintHasSpace,
    #[msg("Constraint account has space")]
    ConstraintAccountHasSpace,
    #[msg("Constraint seed")]
    ConstraintSeed,
    #[msg("Constraint seeds")]
    ConstraintSeeds,
    #[msg("Constraint rent exempt")]
    ConstraintRentExempt,
    #[msg("Constraint data len")]
    ConstraintDataLen,
    #[msg("Constraint program ID")]
    ConstraintProgramId,
    #[msg("Constraint account not empty")]
    ConstraintAccountNotEmpty,
    #[msg("Constraint account not closeable")]
    ConstraintAccountNotCloseable,
    #[msg("Constraint account not mutable")]
    ConstraintAccountNotMutable,
    #[msg("Constraint account not system owned")]
    ConstraintAccountNotSystemOwned,
    #[msg("Constraint account not program owned")]
    ConstraintAccountNotProgramOwned,
    #[msg("Constraint account not token owned")]
    ConstraintAccountNotTokenOwned,
    #[msg("Constraint account not associated token owned")]
    ConstraintAccountNotAssociatedTokenOwned,
    #[msg("Constraint account not mint")]
    ConstraintAccountNotMint,
    #[msg("Constraint account not token account")]
    ConstraintAccountNotTokenAccount,
    #[msg("Constraint account not native token account")]
    ConstraintAccountNotNativeTokenAccount,
    #[msg("Constraint account not program data")]
    ConstraintAccountNotProgramData,
    #[msg("Constraint account not program data address")]
    ConstraintAccountNotProgramDataAddress,
    #[msg("Constraint account not program data owner")]
    ConstraintAccountNotProgramDataOwner,
    #[msg("Constraint account not program data rent exempt")]
    ConstraintAccountNotProgramDataRentExempt,
    #[msg("Constraint account not program data has space")]
    ConstraintAccountNotProgramDataHasSpace,
    #[msg("Constraint account not program data has one")]
    ConstraintAccountNotProgramDataHasOne,
    #[msg("Constraint account not program data has many")]
    ConstraintAccountNotProgramDataHasMany,
    #[msg("Constraint account not program data signer")]
    ConstraintAccountNotProgramDataSigner,
    #[msg("Constraint account not program data writable")]
    ConstraintAccountNotProgramDataWritable,
    #[msg("Constraint account not program data executable")]
    ConstraintAccountNotProgramDataExecutable,
    #[msg("Constraint account not program data state")]
    ConstraintAccountNotProgramDataState,
    #[msg("Constraint account not program data associated")]
    ConstraintAccountNotProgramDataAssociated,
    #[msg("Constraint account not program data associated init")]
    ConstraintAccountNotProgramDataAssociatedInit,
    #[msg("Constraint account not program data close")]
    ConstraintAccount