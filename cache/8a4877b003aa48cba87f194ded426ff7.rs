// Cached result for function: sub_100002cf0
// Binary: /Users/giraffe/Downloads/Work/Solana/ebpf/memo_onchain.so
// Generated: 8a4877b003aa48cba87f194ded426ff7.rs

pub enum ErrorCode {
    #[msg("Invalid account data length")]
    InvalidAccountDataLength,
    #[msg("Invalid account discriminator")]
    InvalidAccountDiscriminator,
    #[msg("Invalid account owner")]
    InvalidAccountOwner,
    #[msg("Invalid program id")]
    InvalidProgramId,
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
    #[msg("Invalid instruction discriminator")]
    InvalidInstructionDiscriminator,
    #[msg("Invalid instruction accounts")]
    InvalidInstructionAccounts,
    #[msg("Invalid instruction data length")]
    InvalidInstructionDataLength,
    #[msg("Invalid instruction data value")]
    InvalidInstructionDataValue,
    #[msg("Invalid instruction data type")]
    InvalidInstructionDataType,
    #[msg("Invalid instruction data format")]
    InvalidInstructionDataFormat,
    #[msg("Invalid instruction data encoding")]
    InvalidInstructionDataEncoding,
    #[msg("Invalid instruction data size")]
    InvalidInstructionDataSize,
    #[msg("Invalid instruction data offset")]
    InvalidInstructionDataOffset,
    #[msg("Invalid instruction data alignment")]
    InvalidInstructionDataAlignment,
    #[msg("Invalid instruction data padding")]
    InvalidInstructionDataPadding,
    #[msg("Invalid instruction data checksum")]
    InvalidInstructionDataChecksum,
    #[msg("Invalid instruction data signature")]
    InvalidInstructionDataSignature,
    #[msg("Invalid instruction data hash")]
    InvalidInstructionDataHash,
    #[msg("Invalid instruction data proof")]
    InvalidInstructionDataProof,
    #[msg("Invalid instruction data nonce")]
    InvalidInstructionDataNonce,
    #[msg("Invalid instruction data timestamp")]
    InvalidInstructionDataTimestamp,
    #[msg("Invalid instruction data sequence")]
    InvalidInstructionDataSequence,
    #[msg("Invalid instruction data version")]
    InvalidInstructionDataVersion,
    #[msg("Invalid instruction data state")]
    InvalidInstructionDataState,
    #[msg("Invalid instruction data status")]
    InvalidInstructionDataStatus,
    #[msg("Invalid instruction data result")]
    InvalidInstructionDataResult,
    #[msg("Invalid instruction data error")]
    InvalidInstructionDataError,
    #[msg("Invalid instruction data message")]
    InvalidInstructionDataMessage,
    #[msg("Invalid instruction data code")]
    InvalidInstructionDataCode,
    #[msg("Invalid instruction data reason")]
    InvalidInstructionDataReason,
    #[msg("Invalid instruction data description")]
    InvalidInstructionDataDescription,
    #[msg("Invalid instruction data source")]
    InvalidInstructionDataSource,
    #[msg("Invalid instruction data destination")]
    InvalidInstructionDataDestination,
    #[msg("Invalid instruction data amount")]
    InvalidInstructionDataAmount,
    #[msg("Invalid instruction data balance")]
    InvalidInstructionDataBalance,
    #[msg("Invalid instruction data fee")]
    InvalidInstructionDataFee,
    #[msg("Invalid instruction data rent")]
    InvalidInstructionDataRent,
    #[msg("Invalid instruction data lamports")]
    InvalidInstructionDataLamports,
    #[msg("Invalid instruction data seeds")]
    InvalidInstructionDataSeeds,
    #[msg("Invalid instruction data signer")]
    InvalidInstructionDataSigner,
    #[msg("Invalid instruction data writable")]
    InvalidInstructionDataWritable,
    #[msg("Invalid instruction data executable")]
    InvalidInstructionDataExecutable,
    #[msg("Invalid instruction data program")]
    InvalidInstructionDataProgram,
    #[msg("Invalid instruction data system")]
    InvalidInstructionDataSystem,
    #[msg("Invalid instruction data sysvar")]
    InvalidInstructionDataSysvar,
    #[msg("Invalid instruction data rent_sysvar")]
    InvalidInstructionDataRentSysvar,
    #[msg("Invalid instruction data clock_sysvar")]
    InvalidInstructionDataClockSysvar,
    #[msg("Invalid instruction data epoch_schedule_sysvar")]
    InvalidInstructionDataEpochScheduleSysvar,
    #[msg("Invalid instruction data fees_sysvar")]
    InvalidInstructionDataFeesSysvar,
    #[msg("Invalid instruction data recent_blockhashes_sysvar")]