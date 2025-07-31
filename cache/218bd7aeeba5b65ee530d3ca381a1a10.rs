// Cached result for function: sub_100000238
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 218bd7aeeba5b65ee530d3ca381a1a10.rs

pub enum ErrorCode {
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account is already initialized")]
    AccountAlreadyInitialized,
    #[msg("Invalid program ID")]
    InvalidProgramId,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Invalid account data length")]
    InvalidAccountDataLength,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Invalid account key")]
    InvalidAccountKey,
    #[msg("Account is not a signer")]
    AccountIsNotSigner,
    #[msg("Account is not writable")]
    AccountIsNotWritable,
    #[msg("Account is not executable")]
    AccountIsNotExecutable,
    #[msg("Account is not rent exempt")]
    AccountIsNotRentExempt,
    #[msg("Account is not a program")]
    AccountIsNotProgram,
    #[msg("Account is not a system account")]
    AccountIsNotSystemAccount,
    #[msg("Account is not a token account")]
    AccountIsNotTokenAccount,
    #[msg("Account is not a mint account")]
    AccountIsNotMintAccount,
    #[msg("Account is not a associated token account")]
    AccountIsNotAssociatedTokenAccount,
    #[msg("Account is not a program state account")]
    AccountIsNotProgramStateAccount,
    #[msg("Account is not a program data account")]
    AccountIsNotProgramDataAccount,
    #[msg("Account is not a program buffer account")]
    AccountIsNotProgramBufferAccount,
    #[msg("Account is not a program deployer account")]
    AccountIsNotProgramDeployerAccount,
    #[msg("Account is not a program upgrade authority account")]
    AccountIsNotProgramUpgradeAuthorityAccount,
    #[msg("Account is not a program upgrade account")]
    AccountIsNotProgramUpgradeAccount,
    #[msg("Account is not a program upgrade buffer account")]
    AccountIsNotProgramUpgradeBufferAccount,
    #[msg("Account is not a program upgrade deployer account")]
    AccountIsNotProgramUpgradeDeployerAccount,
    #[msg("Account is not a program upgrade authority signer account")]
    AccountIsNotProgramUpgradeAuthoritySignerAccount,
    #[msg("Account is not a program upgrade buffer signer account")]
    AccountIsNotProgramUpgradeBufferSignerAccount,
    #[msg("Account is not a program upgrade deployer signer account")]
    AccountIsNotProgramUpgradeDeployerSignerAccount,
    #[msg("Account is not a program upgrade authority system account")]
    AccountIsNotProgramUpgradeAuthoritySystemAccount,
    #[msg("Account is not a program upgrade buffer system account")]
    AccountIsNotProgramUpgradeBufferSystemAccount,
    #[msg("Account is not a program upgrade deployer system account")]
    AccountIsNotProgramUpgradeDeployerSystemAccount,
    #[msg("Account is not a program upgrade authority token account")]
    AccountIsNotProgramUpgradeAuthorityTokenAccount,
    #[msg("Account is not a program upgrade buffer token account")]
    AccountIsNotProgramUpgradeBufferTokenAccount,
    #[msg("Account is not a program upgrade deployer token account")]
    AccountIsNotProgramUpgradeDeployerTokenAccount,
    #[msg("Account is not a program upgrade authority mint account")]
    AccountIsNotProgramUpgradeAuthorityMintAccount,
    #[msg("Account is not a program upgrade buffer mint account")]
    AccountIsNotProgramUpgradeBufferMintAccount,
    #[msg("Account is not a program upgrade deployer mint account")]
    AccountIsNotProgramUpgradeDeployerMintAccount,
    #[msg("Account is not a program upgrade authority associated token account")]
    AccountIsNotProgramUpgradeAuthorityAssociatedTokenAccount,
    #[msg("Account is not a program upgrade buffer associated token account")]
    AccountIsNotProgramUpgradeBufferAssociatedTokenAccount,
    #[msg("Account is not a program upgrade deployer associated token account")]
    AccountIsNotProgramUpgradeDeployerAssociatedTokenAccount,
    #[msg("Account is not a program upgrade authority program state account")]
    AccountIsNotProgramUpgradeAuthorityProgramStateAccount,
    #[msg("Account is not a program upgrade buffer program state account")]
    AccountIsNotProgramUpgradeBuffer