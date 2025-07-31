// Cached result for function: sub_1000025a8
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 067a5aa7ded3552fb2299b966515d2da.rs

pub enum ErrorCode {
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account is already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account is not mutable")]
    AccountNotMutable,
    #[msg("Account not rent exempt")]
    AccountNotRentExempt,
    #[msg("Constraint has_one violated")]
    ConstraintHasOne,
    #[msg("Constraint raw violated")]
    ConstraintRaw,
    #[msg("Constraint seeds violated")]
    ConstraintSeeds,
    #[msg("Constraint signer violated")]
    ConstraintSigner,
    #[msg("Constraint owner violated")]
    ConstraintOwner,
    #[msg("Constraint executable violated")]
    ConstraintExecutable,
    #[msg("Constraint state violated")]
    ConstraintState,
    #[msg("Constraint associated violated")]
    ConstraintAssociated,
    #[msg("Constraint associated init violated")]
    ConstraintAssociatedInit,
    #[msg("Constraint close violated")]
    ConstraintClose,
    #[msg("Constraint address violated")]
    ConstraintAddress,
    #[msg("Constraint zero violated")]
    ConstraintZero,
    #[msg("Constraint token mint violated")]
    ConstraintTokenMint,
    #[msg("Constraint token owner violated")]
    ConstraintTokenOwner,
    #[msg("Constraint mint new account and mint to violated")]
    ConstraintMintNewAccountAndMintTo,
    #[msg("Constraint mint new account and initial supply violated")]
    ConstraintMintNewAccountAndInitialSupply,
    #[msg("Constraint mint new account and transfer violated")]
    ConstraintMintNewAccountAndTransfer,
    #[msg("Constraint mint new account and close violated")]
    ConstraintMintNewAccountAndClose,
    #[msg("Constraint mint new account and burn violated")]
    ConstraintMintNewAccountAndBurn,
    #[msg("Constraint mint new account and freeze violated")]
    ConstraintMintNewAccountAndFreeze,
    #[msg("Constraint mint new account and thaw violated")]
    ConstraintMintNewAccountAndThaw,
    #[msg("Constraint mint new account and set authority violated")]
    ConstraintMintNewAccountAndSetAuthority,
    #[msg("Constraint mint new account and approve violated")]
    ConstraintMintNewAccountAndApprove,
    #[msg("Constraint mint new account and revoke violated")]
    ConstraintMintNewAccountAndRevoke,
    #[msg("Constraint mint new account and sync native violated")]
    ConstraintMintNewAccountAndSyncNative,
    #[msg("Constraint mint new account and wrap sol violated")]
    ConstraintMintNewAccountAndWrapSol,
    #[msg("Constraint mint new account and unwrap sol violated")]
    ConstraintMintNewAccountAndUnwrapSol,
    #[msg("Constraint mint new account and create account violated")]
    ConstraintMintNewAccountAndCreateAccount,
    #[msg("Constraint mint new account and create account with seed violated")]
    ConstraintMintNewAccountAndCreateAccountWithSeed,
    #[msg("Constraint mint new account and assign violated")]
    ConstraintMintNewAccountAndAssign,
    #[msg("Constraint mint new account and transfer with seed violated")]
    ConstraintMintNewAccountAndTransferWithSeed,
    #[msg("Constraint mint new account and allocate violated")]
    ConstraintMintNewAccountAndAllocate,
    #[msg("Constraint mint new account and allocate with seed violated")]
    ConstraintMintNewAccountAndAllocateWithSeed,
    #[msg("Constraint mint new account and assign with seed violated")]
    ConstraintMintNewAccountAndAssignWithSeed,
    #[msg("Constraint mint new account and create program account violated")]
    ConstraintMintNewAccountAndCreateProgramAccount,
    #[msg("Constraint mint new account and create program account with seed violated")]
    ConstraintMintNewAccountAndCreateProgramAccountWithSeed,
    #[msg("Constraint mint new account and close account violated")]
    ConstraintMintNewAccountAndCloseAccount,
    #[msg("Constraint mint new account and close account with seed violated")]
    ConstraintMintNewAccountAndCloseAccountWithSeed,
    #[msg("Constraint mint new account and set data violated")]
    ConstraintMintNewAccountAndSetData,