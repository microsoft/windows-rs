#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2DataEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2EraseEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2RawCDEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2TrackAtOnceEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscMaster2EventsImpl: Sized + IDispatchImpl {
    fn NotifyDeviceAdded();
    fn NotifyDeviceRemoved();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DFileSystemImageEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DFileSystemImageImportEventsImpl: Sized + IDispatchImpl {
    fn UpdateImport();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DWriteEngine2EventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBlockRangeImpl: Sized + IDispatchImpl {
    fn StartLba();
    fn EndLba();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBlockRangeListImpl: Sized + IDispatchImpl {
    fn BlockRanges();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBootOptionsImpl: Sized + IDispatchImpl {
    fn BootImage();
    fn Manufacturer();
    fn SetManufacturer();
    fn PlatformId();
    fn SetPlatformId();
    fn Emulation();
    fn SetEmulation();
    fn ImageSize();
    fn AssignBootImage();
}
pub trait IBurnVerificationImpl: Sized {
    fn SetBurnVerificationLevel();
    fn BurnVerificationLevel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2Impl: Sized + IDispatchImpl {
    fn IsRecorderSupported();
    fn IsCurrentMediaSupported();
    fn MediaPhysicallyBlank();
    fn MediaHeuristicallyBlank();
    fn SupportedMediaTypes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2DataImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
    fn SetRecorder();
    fn Recorder();
    fn SetBufferUnderrunFreeDisabled();
    fn BufferUnderrunFreeDisabled();
    fn SetPostgapAlreadyInImage();
    fn PostgapAlreadyInImage();
    fn CurrentMediaStatus();
    fn WriteProtectStatus();
    fn TotalSectorsOnMedia();
    fn FreeSectorsOnMedia();
    fn NextWritableAddress();
    fn StartAddressOfPreviousSession();
    fn LastWrittenAddressOfPreviousSession();
    fn SetForceMediaToBeClosed();
    fn ForceMediaToBeClosed();
    fn SetDisableConsumerDvdCompatibilityMode();
    fn DisableConsumerDvdCompatibilityMode();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn RequestedWriteSpeed();
    fn RequestedRotationTypeIsPureCAV();
    fn CurrentWriteSpeed();
    fn CurrentRotationTypeIsPureCAV();
    fn SupportedWriteSpeeds();
    fn SupportedWriteSpeedDescriptors();
    fn SetForceOverwrite();
    fn ForceOverwrite();
    fn MultisessionInterfaces();
    fn Write();
    fn CancelWrite();
    fn SetWriteSpeed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2DataEventArgsImpl: Sized + IWriteEngine2EventArgsImpl + IDispatchImpl {
    fn ElapsedTime();
    fn RemainingTime();
    fn TotalTime();
    fn CurrentAction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2EraseImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
    fn SetRecorder();
    fn Recorder();
    fn SetFullErase();
    fn FullErase();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn EraseMedia();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2RawCDImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
    fn PrepareMedia();
    fn WriteMedia();
    fn WriteMedia2();
    fn CancelWrite();
    fn ReleaseMedia();
    fn SetWriteSpeed();
    fn SetRecorder();
    fn Recorder();
    fn SetBufferUnderrunFreeDisabled();
    fn BufferUnderrunFreeDisabled();
    fn StartOfNextSession();
    fn LastPossibleStartOfLeadout();
    fn CurrentPhysicalMediaType();
    fn SupportedSectorTypes();
    fn SetRequestedSectorType();
    fn RequestedSectorType();
    fn SetClientName();
    fn ClientName();
    fn RequestedWriteSpeed();
    fn RequestedRotationTypeIsPureCAV();
    fn CurrentWriteSpeed();
    fn CurrentRotationTypeIsPureCAV();
    fn SupportedWriteSpeeds();
    fn SupportedWriteSpeedDescriptors();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2RawCDEventArgsImpl: Sized + IWriteEngine2EventArgsImpl + IDispatchImpl {
    fn CurrentAction();
    fn ElapsedTime();
    fn RemainingTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2TrackAtOnceImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
    fn PrepareMedia();
    fn AddAudioTrack();
    fn CancelAddTrack();
    fn ReleaseMedia();
    fn SetWriteSpeed();
    fn SetRecorder();
    fn Recorder();
    fn SetBufferUnderrunFreeDisabled();
    fn BufferUnderrunFreeDisabled();
    fn NumberOfExistingTracks();
    fn TotalSectorsOnMedia();
    fn FreeSectorsOnMedia();
    fn UsedSectorsOnMedia();
    fn SetDoNotFinalizeMedia();
    fn DoNotFinalizeMedia();
    fn ExpectedTableOfContents();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn RequestedWriteSpeed();
    fn RequestedRotationTypeIsPureCAV();
    fn CurrentWriteSpeed();
    fn CurrentRotationTypeIsPureCAV();
    fn SupportedWriteSpeeds();
    fn SupportedWriteSpeedDescriptors();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2TrackAtOnceEventArgsImpl: Sized + IWriteEngine2EventArgsImpl + IDispatchImpl {
    fn CurrentTrackNumber();
    fn CurrentAction();
    fn ElapsedTime();
    fn RemainingTime();
}
pub trait IDiscMasterImpl: Sized {
    fn Open();
    fn EnumDiscMasterFormats();
    fn GetActiveDiscMasterFormat();
    fn SetActiveDiscMasterFormat();
    fn EnumDiscRecorders();
    fn GetActiveDiscRecorder();
    fn SetActiveDiscRecorder();
    fn ClearFormatContent();
    fn ProgressAdvise();
    fn ProgressUnadvise();
    fn RecordDisc();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscMaster2Impl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn IsSupportedEnvironment();
}
pub trait IDiscMasterProgressEventsImpl: Sized {
    fn QueryCancel();
    fn NotifyPnPActivity();
    fn NotifyAddProgress();
    fn NotifyBlockProgress();
    fn NotifyTrackProgress();
    fn NotifyPreparingBurn();
    fn NotifyClosingDisc();
    fn NotifyBurnComplete();
    fn NotifyEraseComplete();
}
pub trait IDiscRecorderImpl: Sized {
    fn Init();
    fn GetRecorderGUID();
    fn GetRecorderType();
    fn GetDisplayNames();
    fn GetBasePnPID();
    fn GetPath();
    fn GetRecorderProperties();
    fn SetRecorderProperties();
    fn GetRecorderState();
    fn OpenExclusive();
    fn QueryMediaType();
    fn QueryMediaInfo();
    fn Eject();
    fn Erase();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscRecorder2Impl: Sized + IDispatchImpl {
    fn EjectMedia();
    fn CloseTray();
    fn AcquireExclusiveAccess();
    fn ReleaseExclusiveAccess();
    fn DisableMcn();
    fn EnableMcn();
    fn InitializeDiscRecorder();
    fn ActiveDiscRecorder();
    fn VendorId();
    fn ProductId();
    fn ProductRevision();
    fn VolumeName();
    fn VolumePathNames();
    fn DeviceCanLoadMedia();
    fn LegacyDeviceNumber();
    fn SupportedFeaturePages();
    fn CurrentFeaturePages();
    fn SupportedProfiles();
    fn CurrentProfiles();
    fn SupportedModePages();
    fn ExclusiveAccessOwner();
}
pub trait IDiscRecorder2ExImpl: Sized {
    fn SendCommandNoData();
    fn SendCommandSendDataToDevice();
    fn SendCommandGetDataFromDevice();
    fn ReadDvdStructure();
    fn SendDvdStructure();
    fn GetAdapterDescriptor();
    fn GetDeviceDescriptor();
    fn GetDiscInformation();
    fn GetTrackInformation();
    fn GetFeaturePage();
    fn GetModePage();
    fn SetModePage();
    fn GetSupportedFeaturePages();
    fn GetSupportedProfiles();
    fn GetSupportedModePages();
    fn GetByteAlignmentMask();
    fn GetMaximumNonPageAlignedTransferSize();
    fn GetMaximumPageAlignedTransferSize();
}
pub trait IEnumDiscMasterFormatsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDiscRecordersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumFsiItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumProgressItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImageImpl: Sized + IDispatchImpl {
    fn Root();
    fn SessionStartBlock();
    fn SetSessionStartBlock();
    fn FreeMediaBlocks();
    fn SetFreeMediaBlocks();
    fn SetMaxMediaBlocksFromDevice();
    fn UsedBlocks();
    fn VolumeName();
    fn SetVolumeName();
    fn ImportedVolumeName();
    fn BootImageOptions();
    fn SetBootImageOptions();
    fn FileCount();
    fn DirectoryCount();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
    fn ChangePoint();
    fn StrictFileSystemCompliance();
    fn SetStrictFileSystemCompliance();
    fn UseRestrictedCharacterSet();
    fn SetUseRestrictedCharacterSet();
    fn FileSystemsToCreate();
    fn SetFileSystemsToCreate();
    fn FileSystemsSupported();
    fn SetUDFRevision();
    fn UDFRevision();
    fn UDFRevisionsSupported();
    fn ChooseImageDefaults();
    fn ChooseImageDefaultsForMediaType();
    fn SetISO9660InterchangeLevel();
    fn ISO9660InterchangeLevel();
    fn ISO9660InterchangeLevelsSupported();
    fn CreateResultImage();
    fn Exists();
    fn CalculateDiscIdentifier();
    fn IdentifyFileSystemsOnDisc();
    fn GetDefaultFileSystemForImport();
    fn ImportFileSystem();
    fn ImportSpecificFileSystem();
    fn RollbackToChangePoint();
    fn LockInChangePoint();
    fn CreateDirectoryItem();
    fn CreateFileItem();
    fn VolumeNameUDF();
    fn VolumeNameJoliet();
    fn VolumeNameISO9660();
    fn StageFiles();
    fn SetStageFiles();
    fn MultisessionInterfaces();
    fn SetMultisessionInterfaces();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImage2Impl: Sized + IFileSystemImageImpl + IDispatchImpl {
    fn BootImageOptionsArray();
    fn SetBootImageOptionsArray();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImage3Impl: Sized + IFileSystemImage2Impl + IFileSystemImageImpl + IDispatchImpl {
    fn CreateRedundantUdfMetadataFiles();
    fn SetCreateRedundantUdfMetadataFiles();
    fn ProbeSpecificFileSystem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImageResultImpl: Sized + IDispatchImpl {
    fn ImageStream();
    fn ProgressItems();
    fn TotalBlocks();
    fn BlockSize();
    fn DiscId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImageResult2Impl: Sized + IFileSystemImageResultImpl + IDispatchImpl {
    fn ModifiedBlocks();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiDirectoryItemImpl: Sized + IFsiItemImpl + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn EnumFsiItems();
    fn AddDirectory();
    fn AddFile();
    fn AddTree();
    fn Add();
    fn Remove();
    fn RemoveTree();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiDirectoryItem2Impl: Sized + IFsiDirectoryItemImpl + IFsiItemImpl + IDispatchImpl {
    fn AddTreeWithNamedStreams();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiFileItemImpl: Sized + IFsiItemImpl + IDispatchImpl {
    fn DataSize();
    fn DataSize32BitLow();
    fn DataSize32BitHigh();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiFileItem2Impl: Sized + IFsiFileItemImpl + IFsiItemImpl + IDispatchImpl {
    fn FsiNamedStreams();
    fn IsNamedStream();
    fn AddStream();
    fn RemoveStream();
    fn IsRealTime();
    fn SetIsRealTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiItemImpl: Sized + IDispatchImpl {
    fn Name();
    fn FullPath();
    fn CreationTime();
    fn SetCreationTime();
    fn LastAccessedTime();
    fn SetLastAccessedTime();
    fn LastModifiedTime();
    fn SetLastModifiedTime();
    fn IsHidden();
    fn SetIsHidden();
    fn FileSystemName();
    fn FileSystemPath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiNamedStreamsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn EnumNamedStreams();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIsoImageManagerImpl: Sized + IDispatchImpl {
    fn Path();
    fn Stream();
    fn SetPath();
    fn SetStream();
    fn Validate();
}
pub trait IJolietDiscMasterImpl: Sized {
    fn GetTotalDataBlocks();
    fn GetUsedDataBlocks();
    fn GetDataBlockSize();
    fn AddData();
    fn GetJolietProperties();
    fn SetJolietProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionImpl: Sized + IDispatchImpl {
    fn IsSupportedOnCurrentMediaState();
    fn SetInUse();
    fn InUse();
    fn ImportRecorder();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionRandomWriteImpl: Sized + IMultisessionImpl + IDispatchImpl {
    fn WriteUnitSize();
    fn LastWrittenAddress();
    fn TotalSectorsOnMedia();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionSequentialImpl: Sized + IMultisessionImpl + IDispatchImpl {
    fn IsFirstDataSession();
    fn StartAddressOfPreviousSession();
    fn LastWrittenAddressOfPreviousSession();
    fn NextWritableAddress();
    fn FreeSectorsOnMedia();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionSequential2Impl: Sized + IMultisessionSequentialImpl + IMultisessionImpl + IDispatchImpl {
    fn WriteUnitSize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProgressItemImpl: Sized + IDispatchImpl {
    fn Description();
    fn FirstBlock();
    fn LastBlock();
    fn BlockCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProgressItemsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ProgressItemFromBlock();
    fn ProgressItemFromDescription();
    fn EnumProgressItems();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawCDImageCreatorImpl: Sized + IDispatchImpl {
    fn CreateResultImage();
    fn AddTrack();
    fn AddSpecialPregap();
    fn AddSubcodeRWGenerator();
    fn SetResultingImageType();
    fn ResultingImageType();
    fn StartOfLeadout();
    fn SetStartOfLeadoutLimit();
    fn StartOfLeadoutLimit();
    fn SetDisableGaplessAudio();
    fn DisableGaplessAudio();
    fn SetMediaCatalogNumber();
    fn MediaCatalogNumber();
    fn SetStartingTrackNumber();
    fn StartingTrackNumber();
    fn TrackInfo();
    fn NumberOfExistingTracks();
    fn LastUsedUserSectorInImage();
    fn ExpectedTableOfContents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawCDImageTrackInfoImpl: Sized + IDispatchImpl {
    fn StartingLba();
    fn SectorCount();
    fn TrackNumber();
    fn SectorType();
    fn ISRC();
    fn SetISRC();
    fn DigitalAudioCopySetting();
    fn SetDigitalAudioCopySetting();
    fn AudioHasPreemphasis();
    fn SetAudioHasPreemphasis();
    fn TrackIndexes();
    fn AddTrackIndex();
    fn ClearTrackIndex();
}
pub trait IRedbookDiscMasterImpl: Sized {
    fn GetTotalAudioTracks();
    fn GetTotalAudioBlocks();
    fn GetUsedAudioBlocks();
    fn GetAvailableAudioTrackBlocks();
    fn GetAudioBlockSize();
    fn CreateAudioTrack();
    fn AddAudioTrackBlocks();
    fn CloseAudioTrack();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamConcatenateImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn Initialize();
    fn Initialize2();
    fn Append();
    fn Append2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamInterleaveImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamPseudoRandomBasedImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn SetSeed();
    fn Seed();
    fn SetExtendedSeed();
    fn ExtendedSeed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWriteEngine2Impl: Sized + IDispatchImpl {
    fn WriteSection();
    fn CancelWrite();
    fn SetRecorder();
    fn Recorder();
    fn SetUseStreamingWrite12();
    fn UseStreamingWrite12();
    fn SetStartingSectorsPerSecond();
    fn StartingSectorsPerSecond();
    fn SetEndingSectorsPerSecond();
    fn EndingSectorsPerSecond();
    fn SetBytesPerSector();
    fn BytesPerSector();
    fn WriteInProgress();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWriteEngine2EventArgsImpl: Sized + IDispatchImpl {
    fn StartLba();
    fn SectorCount();
    fn LastReadLba();
    fn LastWrittenLba();
    fn TotalSystemBuffer();
    fn UsedSystemBuffer();
    fn FreeSystemBuffer();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWriteSpeedDescriptorImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn RotationTypeIsPureCAV();
    fn WriteSpeed();
}
