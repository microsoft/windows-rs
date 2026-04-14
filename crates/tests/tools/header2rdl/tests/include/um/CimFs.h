/*
Copyright (c) Microsoft Corporation

   API to construct and mount Composite Images (CIMs)
*/

#include <windows.h>

#ifdef __cplusplus
extern "C"
{
#endif

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DECLARE_HANDLE(CIMFS_IMAGE_HANDLE);

DECLARE_HANDLE(CIMFS_STREAM_HANDLE);

typedef enum CIM_CREATE_IMAGE_FLAGS
{
    CIM_CREATE_IMAGE_NONE               = 0x00000000,
    CIM_CREATE_DO_NOT_EXPAND_PE_IMAGES  = 0x00000001,
    CIM_CREATE_FIXED_SIZE_CHUNKS        = 0x00000002,
    CIM_CREATE_IMAGE_BLOCK_CIM          = 0x00000004,
} CIM_CREATE_IMAGE_FLAGS;

#ifdef DEFINE_ENUM_FLAG_OPERATORS
DEFINE_ENUM_FLAG_OPERATORS(CIM_CREATE_IMAGE_FLAGS);
#endif

typedef enum CIM_MOUNT_IMAGE_FLAGS
{
    CIM_MOUNT_IMAGE_NONE    = 0x00000000,
    CIM_MOUNT_CHILD_ONLY    = 0x00000001,
    CIM_MOUNT_ENABLE_DAX    = 0x00000002,
    CIM_MOUNT_CACHE_FILES   = 0x00000004,
    CIM_MOUNT_CACHE_REGIONS = 0x00000008,
    CIM_MOUNT_BLOCK_CIM     = 0x00000010,
} CIM_MOUNT_IMAGE_FLAGS;

#ifdef DEFINE_ENUM_FLAG_OPERATORS
DEFINE_ENUM_FLAG_OPERATORS(CIM_MOUNT_IMAGE_FLAGS);
#endif

typedef struct _CIMFS_FILE_METADATA
{
    UINT32 Attributes;
    INT64 FileSize;

    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastWriteTime;
    LARGE_INTEGER ChangeTime;
    LARGE_INTEGER LastAccessTime;

    _Field_size_bytes_(SecurityDescriptorSize) const void* SecurityDescriptorBuffer;
    UINT32 SecurityDescriptorSize;

    _Field_size_bytes_(ReparseDataSize) const void* ReparseDataBuffer;
    UINT32 ReparseDataSize;

    _Field_size_bytes_(EaBufferSize) const void* EaBuffer;
    UINT32 EaBufferSize;

} CIMFS_FILE_METADATA;

//
//  Creates a handle representing a new image at the location
//  specified, optionally based on an existing image at that
//  location.
//

STDAPI
CimCreateImage(_In_ PCWSTR imageContainingPath,
               _In_opt_ PCWSTR existingImageName,
               _In_opt_ PCWSTR newImageName,
               _Out_ CIMFS_IMAGE_HANDLE* cimImageHandle);

//
//  Creates a handle representing a new image at the location
//  specified, optionally based on an existing image at that
//  location. Also take in a flags parameter.
//

STDAPI
CimCreateImage2(_In_ PCWSTR imageContainingPath,
                _In_ CIM_CREATE_IMAGE_FLAGS createImageFlags,
                _In_opt_ PCWSTR existingImageName,
                _In_opt_ PCWSTR newImageName,
                _Out_ CIMFS_IMAGE_HANDLE* cimImageHandle);

//
//  Adds a file with the specified metadata at a path relative
//  to the image represented by the image handle.
//  Returns a stream handle to the file primary data stream
//  which can be used to write data to the stream.
//

STDAPI
CimCreateFile(_In_ CIMFS_IMAGE_HANDLE cimImageHandle,
              _In_ PCWSTR imageRelativePath,
              _In_ const CIMFS_FILE_METADATA* fileMetadata,
              _Out_ CIMFS_STREAM_HANDLE* cimStreamHandle);

//
//  Adds an alternate stream with the specified size at a path
//  relative to the image represented by the image handle.
//  Returns a handle to the stream which can be used to write
//  data to the stream.
//

STDAPI
CimCreateAlternateStream(_In_ CIMFS_IMAGE_HANDLE cimImageHandle,
                         _In_ PCWSTR imageRelativePath,
                         _In_ UINT64 streamSize,
                         _Out_ CIMFS_STREAM_HANDLE* cimStreamHandle);

//
//  Adds a hardlink to an existing path relative to the image
//  represented by the image handle.
//

STDAPI
CimCreateHardLink(_In_ CIMFS_IMAGE_HANDLE cimImageHandle,
                  _In_ PCWSTR imageRelativePath,
                  _In_ PCWSTR existingImageRelativePath);

//
//  Writes data from the specified buffer to the stream represented
//  by the stream handle.
//

STDAPI
CimWriteStream(_In_ CIMFS_STREAM_HANDLE cimStreamHandle,
               _In_reads_bytes_(bufferSize) const void* buffer,
               _In_ UINT32 bufferSize);

//
//  Frees resources associated with the stream handle.
//

STDAPI_(void)
CimCloseStream(_In_ CIMFS_STREAM_HANDLE cimStreamHandle);

//
//  Removes the file, stream, directory or hardlink at a path
//  relative to the image represented by the image handle.
//

STDAPI
CimDeletePath(_In_ CIMFS_IMAGE_HANDLE cimImageHandle,
              _In_ PCWSTR imageRelativePath);

//
//  Commits the image represented by the image handle. The handle
//  becomes invalid for further operations on the image.
//

STDAPI
CimCommitImage(_In_ CIMFS_IMAGE_HANDLE cimImageHandle);

//
//  Frees resources associated with the image handle.
//

STDAPI_(void)
CimCloseImage(_In_ CIMFS_IMAGE_HANDLE cimImageHandle);

//
//  Mounts the named image from the location specified by cimPath
//  as a volume with the volume GUID specified by volumeId.
//

STDAPI
CimMountImage(_In_ PCWSTR imageContainingPath,
              _In_ PCWSTR imageName,
              _In_ CIM_MOUNT_IMAGE_FLAGS mountImageFlags,
              _In_ const GUID* volumeId);

//
//  Dismounts an image mounted with volumeId as the volume GUID.
//

STDAPI
CimDismountImage(_In_ const GUID* volumeId);

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

//
// Reads a file from the CIM at a given offset. Returns the read bytes in
// in the buffer provided, the number of bytes read, and the number of bytes
// remaining in the file.
//
STDAPI
CimReadFile(_In_ PCWSTR imagePath,
            _In_ PCWSTR filePath,
            _In_ UINT64 offset,
            _Out_ void* buffer,
            _In_ UINT64 bufferSize,
            _Out_ UINT64* bytesRead,
            _Out_ UINT64* bytesRemaining);
//
// Queries the file stat info for a file from the CIM.
// Returns the FILE_STAT_BASIC_INFORMATION
//
STDAPI
CimGetFileStatBasicInformation(_In_ PCWSTR imagePath,
                               _In_ PCWSTR filePath,
                               _Out_ FILE_STAT_BASIC_INFORMATION* statInfo);

//
// This API takes a path that contains a block format cim, the cim name
// and a path to a new location where it converts the block format cim
// to a file format cim.
//
STDAPI
CimConvertBlockImage(_In_ PCWSTR imageContainingPath,
                     _In_ PCWSTR existingImageName,
                     _In_ PCWSTR newImageContainingPath);

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */

#ifdef __cplusplus
}
#endif
