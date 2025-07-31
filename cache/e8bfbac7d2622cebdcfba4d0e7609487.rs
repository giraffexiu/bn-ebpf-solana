// Cached result for function: sub_100003370
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: e8bfbac7d2622cebdcfba4d0e7609487.rs

pub enum ErrorCode {
    #[msg("Invalid account data")]
    InvalidAccountData,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Invalid program address")]
    InvalidProgramAddress,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Not rent exempt")]
    NotRentExempt,
    #[msg("Account not initialized")]
    AccountNotInitialized,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,
    #[msg("Invalid freeze authority")]
    InvalidFreezeAuthority,
    #[msg("Invalid close authority")]
    InvalidCloseAuthority,
    #[msg("Invalid token program")]
    InvalidTokenProgram,
    #[msg("Invalid associated token program")]
    InvalidAssociatedTokenProgram,
    #[msg("Invalid system program")]
    InvalidSystemProgram,
    #[msg("Invalid rent sysvar")]
    InvalidRentSysvar,
    #[msg("Invalid clock sysvar")]
    InvalidClockSysvar,
    #[msg("Invalid epoch schedule sysvar")]
    InvalidEpochScheduleSysvar,
    #[msg("Invalid recent blockhashes sysvar")]
    InvalidRecentBlockhashesSysvar,
    #[msg("Invalid slot hashes sysvar")]
    InvalidSlotHashesSysvar,
    #[msg("Invalid stake history sysvar")]
    InvalidStakeHistorySysvar,
    #[msg("Invalid rewards sysvar")]
    InvalidRewardsSysvar,
    #[msg("Invalid fees sysvar")]
    InvalidFeesSysvar,
    #[msg("Invalid last restart slot sysvar")]
    InvalidLastRestartSlotSysvar,
    #[msg("Invalid vote program")]
    InvalidVoteProgram,
    #[msg("Invalid stake program")]
    InvalidStakeProgram,
    #[msg("Invalid config program")]
    InvalidConfigProgram,
    #[msg("Invalid bpf loader program")]
    InvalidBpfLoaderProgram,
    #[msg("Invalid bpf loader upgraded program")]
    InvalidBpfLoaderUpgradedProgram,
    #[msg("Invalid bpf loader deprecated program")]
    InvalidBpfLoaderDeprecatedProgram,
    #[msg("Invalid bpf loader v2 program")]
    InvalidBpfLoaderV2Program,
    #[msg("Invalid bpf loader v3 program")]
    InvalidBpfLoaderV3Program,
    #[msg("Invalid compute budget program")]
    InvalidComputeBudgetProgram,
    #[msg("Invalid address lookup table program")]
    InvalidAddressLookupTableProgram,
    #[msg("Invalid feature gate program")]
    InvalidFeatureGateProgram,
    #[msg("Invalid native mint")]
    InvalidNativeMint,
    #[msg("Invalid wrapped sol mint")]
    InvalidWrappedSolMint,
    #[msg("Invalid spl token program")]
    InvalidSplTokenProgram,
    #[msg("Invalid spl associated token account program")]
    InvalidSplAssociatedTokenAccountProgram,
    #[msg("Invalid spl memo program")]
    InvalidSplMemoProgram,
    #[msg("Invalid spl governance program")]
    InvalidSplGovernanceProgram,
    #[msg("Invalid spl stake pool program")]
    InvalidSplStakePoolProgram,
    #[msg("Invalid spl token swap program")]
    InvalidSplTokenSwapProgram,
    #[msg("Invalid spl token lending program")]
    InvalidSplTokenLendingProgram,
    #[msg("Invalid spl token vault program")]
    InvalidSplTokenVaultProgram,
    #[msg("Invalid spl token registry program")]
    InvalidSplTokenRegistryProgram,
    #[msg("Invalid spl token faucet program")]
    InvalidSplTokenFaucetProgram,
    #[msg("Invalid spl token bridge program")]
    InvalidSplTokenBridgeProgram,
    #[msg("Invalid spl token metadata program")]
    InvalidSplTokenMetadataProgram,
    #[msg("Invalid spl token program 2022")]
    InvalidSplTokenProgram2022,
    #[msg("Invalid spl token program 2022 associated token account program")]
    InvalidSplTokenProgram2022AssociatedTokenAccountProgram,
    #[msg("Invalid spl token