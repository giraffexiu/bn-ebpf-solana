// Cached result for function: sub_100000548
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 5573143c61ebd92d51c1cc4d60ee846a.rs

pub enum ErrorCode {
    #[msg("Invalid account discriminator")]
    InvalidAccountDiscriminator,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account is already initialized")]
    AccountAlreadyInitialized,
    #[msg("Account is not mutable")]
    AccountNotMutable,
    #[msg("Account not rent exempt")]
    AccountNotRentExempt,
    #[msg("Constraint raw")]
    ConstraintRaw,
    #[msg("Constraint has one")]
    ConstraintHasOne,
    #[msg("Constraint signer")]
    ConstraintSigner,
    #[msg("Constraint owner")]
    ConstraintOwner,
    #[msg("Constraint close")]
    ConstraintClose,
    #[msg("Constraint address")]
    ConstraintAddress,
    #[msg("Constraint associated")]
    ConstraintAssociated,
    #[msg("Constraint zero copy")]
    ConstraintZeroCopy,
    #[msg("Constraint required space")]
    ConstraintRequiredSpace,
    #[msg("Constraint seeds")]
    ConstraintSeeds,
    #[msg("Constraint callable")]
    ConstraintCallable,
    #[msg("Constraint state")]
    ConstraintState,
    #[msg("Constraint defined at")]
    ConstraintDefinedAt,
    #[msg("Constraint derived")]
    ConstraintDerived,
    #[msg("Constraint writable")]
    ConstraintWritable,
    #[msg("Constraint executable")]
    ConstraintExecutable,
    #[msg("Constraint rent exempt")]
    ConstraintRentExempt,
    #[msg("Constraint token mint")]
    ConstraintTokenMint,
    #[msg("Constraint token owner")]
    ConstraintTokenOwner,
    #[msg("Constraint token account")]
    ConstraintTokenAccount,
    #[msg("Constraint associated token")]
    ConstraintAssociatedToken,
    #[msg("Constraint mint init")]
    ConstraintMintInit,
    #[msg("Constraint associated init")]
    ConstraintAssociatedInit,
    #[msg("Constraint token init")]
    ConstraintTokenInit,
    #[msg("Constraint mint close")]
    ConstraintMintClose,
    #[msg("Constraint token close")]
    ConstraintTokenClose,
    #[msg("Constraint mint burn")]
    ConstraintMintBurn,
    #[msg("Constraint token burn")]
    ConstraintTokenBurn,
    #[msg("Constraint mint freeze")]
    ConstraintMintFreeze,
    #[msg("Constraint token freeze")]
    ConstraintTokenFreeze,
    #[msg("Constraint mint thaw")]
    ConstraintMintThaw,
    #[msg("Constraint token thaw")]
    ConstraintTokenThaw,
    #[msg("Constraint mint transfer")]
    ConstraintMintTransfer,
    #[msg("Constraint token transfer")]
    ConstraintTokenTransfer,
    #[msg("Constraint mint approve")]
    ConstraintMintApprove,
    #[msg("Constraint token approve")]
    ConstraintTokenApprove,
    #[msg("Constraint mint revoke")]
    ConstraintMintRevoke,
    #[msg("Constraint token revoke")]
    ConstraintTokenRevoke,
    #[msg("Constraint mint set authority")]
    ConstraintMintSetAuthority,
    #[msg("Constraint token set authority")]
    ConstraintTokenSetAuthority,
    #[msg("Constraint mint close authority")]
    ConstraintMintCloseAuthority,
    #[msg("Constraint token close authority")]
    ConstraintTokenCloseAuthority,
    #[msg("Constraint mint freeze authority")]
    ConstraintMintFreezeAuthority,
    #[msg("Constraint token freeze authority")]
    ConstraintTokenFreezeAuthority,
    #[msg("Constraint mint transfer authority")]
    ConstraintMintTransferAuthority,
    #[msg("Constraint token transfer authority")]
    ConstraintTokenTransferAuthority,
    #[msg("Constraint mint burn authority")]
    ConstraintMintBurnAuthority,
    #[msg("Constraint token burn authority")]
    ConstraintTokenBurnAuthority,
    #[msg("Constraint mint approve authority")]
    ConstraintMintApproveAuthority,
    #[msg("Constraint token approve authority")]
    ConstraintTokenApproveAuthority,
    #[msg("Constraint mint revoke authority")]
    ConstraintTokenRevokeAuthority,
    #[msg("Constraint token revoke authority")]
    ConstraintTokenRevokeAuthority,
    #[msg("Constraint mint set delegate")]
    Constraint