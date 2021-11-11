#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddLogContainer();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddLogContainerSet();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn AddUsersToEncryptedFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn AdvanceLogBase();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlignReservedLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocReservedLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreFileApisANSI();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreShortNamesEnabled();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupRead();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupSeek();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupWrite();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingCancelRequest();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingReadFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn BuildIoRingRegisterBuffers();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingRegisterFileHandles();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckNameLegalDOS8Dot3A();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckNameLegalDOS8Dot3W();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseAndResetLogFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn CloseEncryptedFileRaw();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn CloseIoRing();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitComplete();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitTransaction();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitTransactionAsync();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareFileTime();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFile2();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn CopyLZFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFile2();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFile2FromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn CreateIoRing();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CreateLogContainerScanContext();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateLogFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateLogMarshallingArea();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateResourceManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTapePartition();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateTransaction();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateTransactionManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecryptFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecryptFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefineDosDeviceA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefineDosDeviceW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogByHandle();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogMarshallingArea();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVolumeMountPointA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVolumeMountPointW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterManageableLogClient();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DuplicateEncryptionInfoFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptionDisable();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EraseTape();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileEncryptionStatusA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileEncryptionStatusW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToLocalFileTime();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindClose();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCloseChangeNotification();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstChangeNotificationA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstChangeNotificationW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileNameTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstStreamTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstStreamW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeMountPointA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeMountPointW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextChangeNotification();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextStreamW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeMountPointA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeMountPointW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindVolumeClose();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindVolumeMountPointClose();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushFileBuffers();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FlushLogBuffers();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FlushLogToLsn();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn FreeEncryptedFileMetadata();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FreeEncryptionCertificateHashList();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeReservedLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBinaryTypeA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBinaryTypeW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentClockTransactionManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskSpaceInformationA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskSpaceInformationW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDriveTypeA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDriveTypeW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEncryptedFileMetadata();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnlistmentId();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnlistmentRecoveryInformation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExpandedNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExpandedNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileBandwidthReservation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileInformationByHandle();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileInformationByHandleEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSize();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSizeEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTime();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileType();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFinalPathNameByHandleA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFinalPathNameByHandleW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn GetIoRingInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogContainerName();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogFileInformation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogIoStatistics();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogReservationInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalDriveStringsA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalDriveStringsW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn GetLogicalDrives();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextLogArchiveExtent();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNotificationResourceManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetNotificationResourceManagerAsync();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetShortPathNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetShortPathNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapeParameters();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapePosition();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapeStatus();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempFileNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempFileNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPath2A();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPath2W();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPathA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPathW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionId();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionInformation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionManagerId();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationByHandleW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeNameForVolumeMountPointA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeNameForVolumeMountPointW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNamesForVolumeNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNamesForVolumeNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HandleLogFull();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallLogPolicy();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsIoRingOpSupported();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LZClose();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LZCopy();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LZDone();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LZInit();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZOpenFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZOpenFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZRead();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LZSeek();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LZStart();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalFileTimeToFileTime();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn LockFileEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogTailAdvanceFailure();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LsnBlockOffset();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LsnContainer();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LsnCreate();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnEqual();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnGreater();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LsnIncrement();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnInvalid();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnLess();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnNull();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn LsnRecordSequence();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileExA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileWithProgressA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileWithProgressW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConnectionEnum();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetFileClose();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetFileEnum();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetFileGetInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerAliasAdd();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerAliasDel();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerAliasEnum();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSessionDel();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSessionEnum();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSessionGetInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareAdd();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareCheck();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareDel();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareDelEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareDelSticky();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareEnum();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareEnumSticky();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareGetInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareSetInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn NetStatisticsGet();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn NtCreateFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEncryptedFileRawA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEncryptedFileRawW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenFileById();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenResourceManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransaction();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransactionManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransactionManagerById();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn PopIoRingCompletion();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrePrepareComplete();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrePrepareEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareComplete();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareLogArchive();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareTape();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDosDeviceA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDosDeviceW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn QueryIoRingCapabilities();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryLogPolicy();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryRecoveryAgentsOnEncryptedFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryUsersOnEncryptedFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReOpenFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadDirectoryChangesExW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadDirectoryChangesW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn ReadEncryptedFileRaw();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFileEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFileScatter();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadLogArchiveMetadata();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogNotification();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogRecord();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogRestartArea();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadNextLogRecord();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadOnlyEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadPreviousLogRestartArea();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverResourceManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverTransactionManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterForLogWriteNotification();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterManageableLogClient();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogContainer();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogContainerSet();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogPolicy();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RemoveUsersFromEncryptedFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RenameTransactionManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReserveAndAppendLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReserveAndAppendLogAligned();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackComplete();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackEnlistment();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackTransaction();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackTransactionAsync();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollforwardTransactionManager();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanLogContainers();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchPathA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchPathW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SetEncryptedFileMetadata();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEndOfFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn SetEndOfLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnlistmentRecoveryInformation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn SetFileApisToANSI();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn SetFileApisToOEM();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesFromAppW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesTransactedA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesTransactedW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileBandwidthReservation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileCompletionNotificationModes();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileInformationByHandle();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileIoOverlappedRange();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFilePointer();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFilePointerEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileShortNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileShortNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileTime();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileValidData();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIoRingCompletionEvent();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogArchiveMode();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogArchiveTail();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogFileSizeWithPolicy();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetResourceManagerCompletionPort();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSearchPathMode();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTapeParameters();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTapePosition();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTransactionInformation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn SetUserFileEncryptionKey();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn SetUserFileEncryptionKeyEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeLabelA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeLabelW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeMountPointA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeMountPointW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SinglePhaseReject();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn SubmitIoRing();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateLogArchive();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateReadLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TruncateLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn TxfGetThreadMiniVersionForCreate();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogCreateFileReadContext();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogCreateRangeReadContext();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogDestroyReadContext();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogReadRecords();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogRecordGetFileName();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogRecordGetGenericType();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfReadMetadataInfo();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn TxfSetThreadMiniVersionForCreate();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn UnlockFileEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ValidateLog();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerFindFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerFindFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerInstallFileA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerInstallFileW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerLanguageNameA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerLanguageNameW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerQueryValueA();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerQueryValueW();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofEnumEntries();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofFileEnumFiles();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofGetDriverVersion();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofIsExternalFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofSetFileDataLocation();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofShouldCompressBinaries();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimAddEntry();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimEnumFiles();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimRemoveEntry();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimSuspendEntry();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimUpdateEntry();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64DisableWow64FsRedirection();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64EnableWow64FsRedirection();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64RevertWow64FsRedirection();
    #[doc = "*Required features: `Win32_Storage_FileSystem`*"]
    pub fn WriteEncryptedFileRaw();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFile();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFileEx();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFileGather();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteLogRestartArea();
    #[doc = "*Required features: `Win32_Storage_FileSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteTapemark();
}
