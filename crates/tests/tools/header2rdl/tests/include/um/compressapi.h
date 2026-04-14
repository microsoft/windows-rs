/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    compressapi.h

Abstract:

    This module exposes the APIs to use the Windows Compression algorithms.

Environment:

    User mode.

--*/

#include <windef.h>

#if (NTDDI_VERSION >= NTDDI_WIN8)

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DECLARE_HANDLE(COMPRESSOR_HANDLE);
typedef COMPRESSOR_HANDLE *PCOMPRESSOR_HANDLE;

typedef COMPRESSOR_HANDLE DECOMPRESSOR_HANDLE;
typedef COMPRESSOR_HANDLE *PDECOMPRESSOR_HANDLE;

#define COMPRESS_ALGORITHM_INVALID      0
#define COMPRESS_ALGORITHM_NULL         1
#define COMPRESS_ALGORITHM_MSZIP        2
#define COMPRESS_ALGORITHM_XPRESS       3
#define COMPRESS_ALGORITHM_XPRESS_HUFF  4
#define COMPRESS_ALGORITHM_LZMS         5
#define COMPRESS_ALGORITHM_MAX          6

#define COMPRESS_RAW             (1 << 29)

typedef PVOID (__cdecl *PFN_COMPRESS_ALLOCATE) (
    _In_ PVOID UserContext,
    _In_ SIZE_T Size
    );

typedef VOID (__cdecl *PFN_COMPRESS_FREE) (
    _In_ PVOID UserContext,
    _In_ PVOID Memory
    );

typedef struct _COMPRESS_ALLOCATION_ROUTINES {
    PFN_COMPRESS_ALLOCATE Allocate;
    PFN_COMPRESS_FREE Free;
    PVOID UserContext;
} COMPRESS_ALLOCATION_ROUTINES, *PCOMPRESS_ALLOCATION_ROUTINES;

typedef enum {
    COMPRESS_INFORMATION_CLASS_INVALID = 0,
    COMPRESS_INFORMATION_CLASS_BLOCK_SIZE,
    COMPRESS_INFORMATION_CLASS_LEVEL
} COMPRESS_INFORMATION_CLASS;

// Compression routines

BOOL
WINAPI
CreateCompressor (
    _In_ DWORD Algorithm,
    _In_opt_ PCOMPRESS_ALLOCATION_ROUTINES AllocationRoutines,
    _Out_ PCOMPRESSOR_HANDLE CompressorHandle
    );

BOOL
WINAPI
SetCompressorInformation (
    _In_ COMPRESSOR_HANDLE CompressorHandle,
    _In_ COMPRESS_INFORMATION_CLASS CompressInformationClass,
    _In_reads_bytes_(CompressInformationSize) LPCVOID CompressInformation,
    _In_ SIZE_T CompressInformationSize
    );

BOOL
WINAPI
QueryCompressorInformation (
    _In_ COMPRESSOR_HANDLE CompressorHandle,
    _In_ COMPRESS_INFORMATION_CLASS CompressInformationClass,
    _Out_writes_bytes_(CompressInformationSize) PVOID CompressInformation,
    _In_ SIZE_T CompressInformationSize
    );

BOOL
WINAPI
Compress (
    _In_ COMPRESSOR_HANDLE CompressorHandle,
    _In_reads_bytes_opt_(UncompressedDataSize) LPCVOID UncompressedData,
    _In_ SIZE_T UncompressedDataSize,
    _Out_writes_bytes_opt_(CompressedBufferSize) PVOID CompressedBuffer,
    _In_ SIZE_T CompressedBufferSize,
    _Out_ PSIZE_T CompressedDataSize
    );

BOOL
WINAPI
ResetCompressor (
    _In_ COMPRESSOR_HANDLE CompressorHandle
    );

BOOL
WINAPI
CloseCompressor (
    _In_ COMPRESSOR_HANDLE CompressorHandle
    );

// Decompression routines

BOOL
WINAPI
CreateDecompressor (
    _In_ DWORD Algorithm,
    _In_opt_ PCOMPRESS_ALLOCATION_ROUTINES AllocationRoutines,
    _Out_ PDECOMPRESSOR_HANDLE DecompressorHandle
    );

BOOL
WINAPI
SetDecompressorInformation (
    _In_ DECOMPRESSOR_HANDLE DecompressorHandle,
    _In_ COMPRESS_INFORMATION_CLASS CompressInformationClass,
    _In_reads_bytes_(CompressInformationSize) LPCVOID CompressInformation,
    _In_ SIZE_T CompressInformationSize
    );

BOOL
WINAPI
QueryDecompressorInformation (
    _In_ DECOMPRESSOR_HANDLE DecompressorHandle,
    _In_ COMPRESS_INFORMATION_CLASS CompressInformationClass,
    _Out_writes_bytes_(CompressInformationSize) PVOID CompressInformation,
    _In_ SIZE_T CompressInformationSize
    );

BOOL
WINAPI
Decompress (
    _In_ DECOMPRESSOR_HANDLE DecompressorHandle,
    _In_reads_bytes_opt_(CompressedDataSize) LPCVOID CompressedData,
    _In_ SIZE_T CompressedDataSize,
    _Out_writes_bytes_opt_(UncompressedBufferSize) PVOID UncompressedBuffer,
    _In_ SIZE_T UncompressedBufferSize,
    _Out_opt_ PSIZE_T UncompressedDataSize
    );

BOOL
WINAPI
ResetDecompressor (
    _In_ DECOMPRESSOR_HANDLE DecompressorHandle
    );

BOOL
WINAPI
CloseDecompressor (
    _In_ DECOMPRESSOR_HANDLE DecompressorHandle
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif
