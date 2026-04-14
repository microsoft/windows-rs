/* Copyright (c) Microsoft Corporation.  All rights reserved. */

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or CoreSetup Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CORESETUP)


/** Delta Compression Engine API. */

#include <windows.h>

/** For crypto algorithm identifiers, ALG_ID. */
#include <wincrypt.h>

/** Allows anonymous structs and unions. */
#pragma warning( disable: 4201 )

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

/** Default size limit for source and target files and buffers. */
#define DELTA_FILE_SIZE_LIMIT           ( 32 * 1024 * 1024 )

/** Default size limit for options files and buffers. */
#define DELTA_OPTIONS_SIZE_LIMIT        ( 128 * 1024 * 1024 )

/** Type for input memory blocks. */
typedef struct _DELTA_INPUT
{
    union
    {
        /** Start of memory block, if it is not Editable. */
        LPCVOID lpcStart;

        /** Start of memory block, if it is Editable. */
        LPVOID lpStart;
    };

    /** Size of memory block in bytes. */
    SIZE_T uSize;

    /** TRUE if caller allows msdelta to edit this memory block, FALSE otherwise. */
    BOOL Editable;

} DELTA_INPUT;

typedef DELTA_INPUT *LPDELTA_INPUT;

typedef const DELTA_INPUT *LPCDELTA_INPUT;

/** Type for output memory blocks. */
typedef struct _DELTA_OUTPUT
{
    /** Start of memory block. */
    LPVOID lpStart;

    /** Size of memory block in bytes. */
    SIZE_T uSize;
} DELTA_OUTPUT;

typedef DELTA_OUTPUT *LPDELTA_OUTPUT;

typedef const DELTA_OUTPUT *LPCDELTA_OUTPUT;

typedef __int64 DELTA_FILE_TYPE;

/** Raw file type. */
#define DELTA_FILE_TYPE_RAW                 ( (DELTA_FILE_TYPE) 0x00000001 )

/** File type for I386 Portable Executable files. */
#define DELTA_FILE_TYPE_I386                ( (DELTA_FILE_TYPE) 0x00000002 )

/** File type for for IA64 Portable Executable files. */
#define DELTA_FILE_TYPE_IA64                ( (DELTA_FILE_TYPE) 0x00000004 )

/** File type for AMD64 Portable Executable files. */
#define DELTA_FILE_TYPE_AMD64               ( (DELTA_FILE_TYPE) 0x00000008 )

/** File type for I386 Portable Executable files with CLI4 transform. */
#define DELTA_FILE_TYPE_CLI4_I386           ( (DELTA_FILE_TYPE) 0x00000010 )

/** File type for AMD64 Portable Executable files with CLI4 transform. */
#define DELTA_FILE_TYPE_CLI4_AMD64          ( (DELTA_FILE_TYPE) 0x00000020 )

/** File type for ARM Portable Executable files with CLI4 transform. */
#define DELTA_FILE_TYPE_CLI4_ARM            ( (DELTA_FILE_TYPE) 0x00000040 )

/** File type for ARM64 Portable Executable files with CLI4 transform. */
#define DELTA_FILE_TYPE_CLI4_ARM64          ( (DELTA_FILE_TYPE) 0x00000080 )

/** File type for distinguishing delta patches from reverse delta patches. */
#define DELTA_FILE_TYPE_REVERSE_ANY         ( (DELTA_FILE_TYPE) 0x00000100 )

/** File type set that treats any file as raw. */
#define DELTA_FILE_TYPE_SET_RAW_ONLY        ( DELTA_FILE_TYPE_RAW )

/** File type set that distinguishes I386, IA64 and AMD64 Portable Executable file and treats others as raw. */
#define DELTA_FILE_TYPE_SET_EXECUTABLES_1   ( DELTA_FILE_TYPE_RAW | \
                                              DELTA_FILE_TYPE_I386 | \
                                              DELTA_FILE_TYPE_IA64 | \
                                              DELTA_FILE_TYPE_AMD64 )

#define DELTA_FILE_TYPE_SET_EXECUTABLES     DELTA_FILE_TYPE_SET_EXECUTABLES_1

/** File type set that distinguishes I386, IA64, ARM, and AMD64 Portable Executable file and treats others as raw. */
#define DELTA_FILE_TYPE_SET_EXECUTABLES_2   ( DELTA_FILE_TYPE_RAW | \
                                              DELTA_FILE_TYPE_CLI4_I386 | \
                                              DELTA_FILE_TYPE_IA64 | \
                                              DELTA_FILE_TYPE_CLI4_AMD64 | \
                                              DELTA_FILE_TYPE_CLI4_ARM )

/** File type set that distinguishes I386, IA64, ARM, ARM64, and AMD64 Portable Executable file and treats others as raw. */
#define DELTA_FILE_TYPE_SET_EXECUTABLES_3   ( DELTA_FILE_TYPE_RAW | \
                                              DELTA_FILE_TYPE_CLI4_I386 | \
                                              DELTA_FILE_TYPE_IA64 | \
                                              DELTA_FILE_TYPE_CLI4_AMD64 | \
                                              DELTA_FILE_TYPE_CLI4_ARM | \
                                              DELTA_FILE_TYPE_CLI4_ARM64 )

#define DELTA_FILE_TYPE_SET_EXECUTABLES_LATEST  DELTA_FILE_TYPE_SET_EXECUTABLES_3

/** Type for msdelta flags. */
typedef __int64 DELTA_FLAG_TYPE;

/** No flags. */
#define DELTA_FLAG_NONE                     ( (DELTA_FLAG_TYPE)0x00000000 )

/** Allow application of legacy PA19 deltas by mspatcha.dll. */
#define DELTA_APPLY_FLAG_ALLOW_PA19         ( (DELTA_FLAG_TYPE)0x00000001 )

/** Transform E8 pieces (relative calls in x86) of target file . */
#define DELTA_FLAG_E8                       ( (DELTA_FLAG_TYPE)0x00000001 ) /* flags[ 0 ] */

/** Mark non-executable parts of source PE. */
#define DELTA_FLAG_MARK                     ( (DELTA_FLAG_TYPE)0x00000002 ) /* flags[ 1 ] */

/** Transform imports of source PE. */
#define DELTA_FLAG_IMPORTS                  ( (DELTA_FLAG_TYPE)0x00000004 ) /* flags[ 2 ] */

/** Transform exports of source PE. */
#define DELTA_FLAG_EXPORTS                  ( (DELTA_FLAG_TYPE)0x00000008 ) /* flags[ 3 ] */

/** Transform resources of source PE. */
#define DELTA_FLAG_RESOURCES                ( (DELTA_FLAG_TYPE)0x00000010 ) /* flags[ 4 ] */

/** Transform relocations of source PE. */
#define DELTA_FLAG_RELOCS                   ( (DELTA_FLAG_TYPE)0x00000020 ) /* flags[ 5 ] */

/** Smash lock prefixes of source PE. */
#define DELTA_FLAG_I386_SMASHLOCK           ( (DELTA_FLAG_TYPE)0x00000040 ) /* flags[ 6 ] */

/** Transform relative jumps of source I386 (x86) PE. */
#define DELTA_FLAG_I386_JMPS                ( (DELTA_FLAG_TYPE)0x00000080 ) /* flags[ 7 ] */

/** Transform relative calls of source I386 (x86) PE. */
#define DELTA_FLAG_I386_CALLS               ( (DELTA_FLAG_TYPE)0x00000100 ) /* flags[ 8 ] */

/** Transform instructions of source AMD64 (x86-64) PE. */
#define DELTA_FLAG_AMD64_DISASM             ( (DELTA_FLAG_TYPE)0x00000200 ) /* flags[ 9 ] */

/** Transform pdata of source AMD64 (x86-64) PE. */
#define DELTA_FLAG_AMD64_PDATA              ( (DELTA_FLAG_TYPE)0x00000400 ) /* flags[ 10 ] */

/** Transform intstructions of source IA64 (Itanium) PE. */
#define DELTA_FLAG_IA64_DISASM              ( (DELTA_FLAG_TYPE)0x00000800 ) /* flags[ 11 ] */

/** Transform pdata of source IA64 (Itanium) PE. */
#define DELTA_FLAG_IA64_PDATA               ( (DELTA_FLAG_TYPE)0x00001000 ) /* flags[ 12 ] */

/** Unbind source PE. */
#define DELTA_FLAG_UNBIND                   ( (DELTA_FLAG_TYPE)0x00002000 ) /* flags[ 13 ] */

/** Transform CLI instructions of source PE. */
#define DELTA_FLAG_CLI_DISASM               ( (DELTA_FLAG_TYPE)0x00004000 ) /* flags[ 14 ] */

/** Transform CLI Metadata of source PE. */
#define DELTA_FLAG_CLI_METADATA             ( (DELTA_FLAG_TYPE)0x00008000 ) /* flags[ 15 ] */

/** Transform headers of source PE. */
#define DELTA_FLAG_HEADERS                  ( (DELTA_FLAG_TYPE)0x00010000 ) /* flags[ 16 ] */

/** Allow source or target file or buffer to exceed its default size limit. */
#define DELTA_FLAG_IGNORE_FILE_SIZE_LIMIT   ( (DELTA_FLAG_TYPE)0x00020000 ) /* flags[ 17 ] */

/** Allow options buffer or file to exceeed its default size limit. */
#define DELTA_FLAG_IGNORE_OPTIONS_SIZE_LIMIT ((DELTA_FLAG_TYPE)0x00040000 ) /* flags[ 18 ] */

/** Transform instructions of source ARM PE. */
#define DELTA_FLAG_ARM_DISASM               ( (DELTA_FLAG_TYPE)0x00080000 ) /* flags[ 19 ] */

/** Transform pdata of source ARM PE. */
#define DELTA_FLAG_ARM_PDATA                ( (DELTA_FLAG_TYPE)0x00100000 ) /* flags[ 20 ] */

/** Transform CLI4 Metadata of source PE. */  
#define DELTA_FLAG_CLI4_METADATA            ( (DELTA_FLAG_TYPE)0x00200000 ) /* flags[ 21 ] */

/** Transform CLI4 instructions of source PE. */
#define DELTA_FLAG_CLI4_DISASM              ( (DELTA_FLAG_TYPE)0x00400000 ) /* flags[ 22 ] */

/** Transform instructions of source ARM PE. */
#define DELTA_FLAG_ARM64_DISASM             ( (DELTA_FLAG_TYPE)0x00800000 ) /* flags[ 23 ] */

/** Transform pdata of source ARM PE. */
#define DELTA_FLAG_ARM64_PDATA              ( (DELTA_FLAG_TYPE)0x01000000 ) /* flags[ 24 ] */


/** List the default transform combination for individule ISA */
#define DELTA_DEFAULT_FLAGS_RAW             ( DELTA_FLAG_NONE )

#define DELTA_DEFAULT_FLAGS_I386            ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_I386_SMASHLOCK | \
                                              DELTA_FLAG_I386_JMPS | \
                                              DELTA_FLAG_I386_CALLS | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI_DISASM | \
                                              DELTA_FLAG_CLI_METADATA ) 

#define DELTA_DEFAULT_FLAGS_IA64            ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_IA64_DISASM | \
                                              DELTA_FLAG_IA64_PDATA | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI_DISASM | \
                                              DELTA_FLAG_CLI_METADATA ) 

#define DELTA_DEFAULT_FLAGS_AMD64           ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_AMD64_DISASM | \
                                              DELTA_FLAG_AMD64_PDATA | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI_DISASM | \
                                              DELTA_FLAG_CLI_METADATA )

#define DELTA_CLI4_FLAGS_I386              ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_I386_SMASHLOCK | \
                                              DELTA_FLAG_I386_JMPS | \
                                              DELTA_FLAG_I386_CALLS | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI4_DISASM | \
                                              DELTA_FLAG_CLI4_METADATA ) 

#define DELTA_CLI4_FLAGS_AMD64             ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_AMD64_DISASM | \
                                              DELTA_FLAG_AMD64_PDATA | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI4_DISASM | \
                                              DELTA_FLAG_CLI4_METADATA )

#define DELTA_CLI4_FLAGS_ARM               ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_ARM_DISASM | \
                                              DELTA_FLAG_ARM_PDATA | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI4_DISASM | \
                                              DELTA_FLAG_CLI4_METADATA )

#define DELTA_CLI4_FLAGS_ARM64             ( DELTA_FLAG_MARK | \
                                              DELTA_FLAG_IMPORTS | \
                                              DELTA_FLAG_EXPORTS | \
                                              DELTA_FLAG_RESOURCES | \
                                              DELTA_FLAG_RELOCS | \
                                              DELTA_FLAG_ARM64_DISASM | \
                                              DELTA_FLAG_ARM64_PDATA | \
                                              DELTA_FLAG_UNBIND | \
                                              DELTA_FLAG_CLI4_DISASM | \
                                              DELTA_FLAG_CLI4_METADATA )

/** Maximal allowed size of hash in bytes. */
#define DELTA_MAX_HASH_SIZE                 32

/** Hash structure. */
typedef struct _DELTA_HASH
{
    /** Size of hash in bytes. Does not exceed DELTA_MAX_HASH_SIZE. */
    DWORD HashSize;

    /** Hash value. */
    UCHAR HashValue[ DELTA_MAX_HASH_SIZE ];
} DELTA_HASH;

typedef DELTA_HASH *LPDELTA_HASH;

typedef const DELTA_HASH *LPCDELTA_HASH;

/** Delta header information. */
typedef struct _DELTA_HEADER_INFO
{
    /** Used file type set. */
    DELTA_FILE_TYPE FileTypeSet;

    /** Source file type. */
    DELTA_FILE_TYPE FileType;

    /** Delta flags. */
    DELTA_FLAG_TYPE Flags;

    /** Size of target file in bytes. */
    SIZE_T  TargetSize;

    /** Time of target file. */
    FILETIME TargetFileTime;

    /** Algorithm used for hashing. */
    ALG_ID TargetHashAlgId;

    /** Target hash. */
    DELTA_HASH TargetHash;

} DELTA_HEADER_INFO;

typedef DELTA_HEADER_INFO *LPDELTA_HEADER_INFO;

typedef const DELTA_HEADER_INFO *LPCDELTA_HEADER_INFO;

/**
 * Gets header information for a delta in memory.
 * @param Delta         Delta memory block.
 * @param lpHeaderInfo  Header information for given Delta.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
GetDeltaInfoB(
    _In_ DELTA_INPUT Delta,
    _Out_ LPDELTA_HEADER_INFO lpHeaderInfo
    );

/**
 * Gets header information for a delta accessed by ASCII file name.
 * @param lpDeltaName   Delta file name, ASCII.
 * @param lpHeaderInfo  Header information for given Delta.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
GetDeltaInfoA(
    _In_ LPCSTR lpDeltaName,
    _Out_ LPDELTA_HEADER_INFO lpHeaderInfo
    );

/**
 * Gets header information for a delta accessed by Unicode file name.
 * @param lpDeltaName   Delta file name, Unicode.
 * @param lpHeaderInfo  Header information for given Delta.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
GetDeltaInfoW(
    _In_ LPCWSTR lpDeltaName,
    _Out_ LPDELTA_HEADER_INFO lpHeaderInfo
    );

#ifdef _UNICODE
#define GetDeltaInfo                        GetDeltaInfoW
#else
#define GetDeltaInfo                        GetDeltaInfoA
#endif /* _UNICODE */

/**
 * Applies a given delta to a given source file and creates the reverse delta to the original.
 * The resultant target file and reverse is put into allocated memory.
 * @param ApplyFlags        Apply-specific flags.
 * @param Source            Source memory block.
 * @param Delta             Delta memory block.
 * @param lpReverseFileTime Timestamp
 * @param lpTarget          Target memory block. Caller DeltaFree.
 * @param lpTargetReverse   Target reverse memory block. Caller DeltaFree.
 * @return                  TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
ApplyDeltaGetReverseB(
    _In_ DELTA_FLAG_TYPE ApplyFlags,
    _In_ DELTA_INPUT Source,
    _In_ DELTA_INPUT Delta,
    _In_opt_ const FILETIME* lpReverseFileTime,
    _Out_ LPDELTA_OUTPUT lpTarget,
    _Out_ LPDELTA_OUTPUT lpTargetReverse
    );

/**
 * Applies a given delta to a given source file.
 * The resultant target file is put into allocated memory.
 * @param ApplyFlags    Apply-specific flags, such as DELTA_APPLY_FLAG_ALLOW_PA19.
 * @param Source        Source memory block.
 * @param Delta         Delta memory block.
 * @param lpTarget      Target memory block. Caller DeltaFree.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
ApplyDeltaB(
    _In_ DELTA_FLAG_TYPE ApplyFlags,
    _In_ DELTA_INPUT Source,
    _In_ DELTA_INPUT Delta,
    _Out_ LPDELTA_OUTPUT lpTarget
    );


/**
 * Applies a given delta to a given source file.
 * The resultant target file is put into caller-provided memory.
 * @param ApplyFlags    Apply-specific flags, such as DELTA_APPLY_FLAG_ALLOW_PA19.
 * @param Source        Source memory block.
 * @param Delta         Delta memory block.
 * @param lpTarget      Pointer to caller-allocated target memory block.
 * @param uTargetSize   Size of target memory block in bytes, caller needs to call GetDeltaInfo to obtain it.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
ApplyDeltaProvidedB(
    _In_ DELTA_FLAG_TYPE ApplyFlags,
    _In_ DELTA_INPUT Source,
    _In_ DELTA_INPUT Delta,
    _Inout_updates_bytes_( uTargetSize ) LPVOID lpTarget,
    _In_ SIZE_T uTargetSize
    );

/**
 * Applies a given delta to a given source file. The resultant target file is written to disk.
 * All files accessed by ASCII file names.
 * @param ApplyFlags    Apply-specific flags, such as DELTA_APPLY_FLAG_ALLOW_PA19.
 * @param lpSourceName  Source file name, ASCII.
 * @param lpDeltaName   Delta file name, ASCII.
 * @param lpTargetName  Target file name, ASCII.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
ApplyDeltaA(
    _In_ DELTA_FLAG_TYPE ApplyFlags,
    _In_ LPCSTR lpSourceName,
    _In_ LPCSTR lpDeltaName,
    _In_ LPCSTR lpTargetName
    );

/**
 * Applies a given delta to a given source file. The resultant target file is written to disk.
 * All files accessed by Unicode file names.
 * @param ApplyFlags    Apply-specific flags, such as DELTA_APPLY_FLAG_ALLOW_PA19.
 * @param lpSourceName  Source file name, Unicode.
 * @param lpDeltaName   Delta file name, Unicode.
 * @param lpTargetName  Target file name, Unicode.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
ApplyDeltaW(
    _In_ DELTA_FLAG_TYPE ApplyFlags,
    _In_ LPCWSTR lpSourceName,
    _In_ LPCWSTR lpDeltaName,
    _In_ LPCWSTR lpTargetName
    );

#ifdef _UNICODE
#define ApplyDelta                          ApplyDeltaW
#else
#define ApplyDelta                          ApplyDeltaA
#endif /* _UNICODE */

/**
 * Creates a delta from a given source file to a given target file in memory.
 * The resultant delta is put into allocated memory.
 * @param FileTypeSet   File type set.
 * @param SetFlags      Set these flags.
 * @param ResetFlags    Reset (supress) these flags.
 * @param Source        Source memory block.
 * @param Target        Target memory block.
 * @param SourceOptions Memory block with source-specific options.
 * @param TargetOptions Memory block with target-specific options.
 * @param GlobalOptions Memory block with global options.
 * @param lpTargetFileTime  Target file time to use, null to use current time.
 * @param HashAlgId     Algorithm for hashing.
 * @param lpDelta       Result delta memory block.
 * @return              TRUE if sucess, FALSE otherwise.
 */
BOOL
WINAPI
CreateDeltaB(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ DELTA_FLAG_TYPE SetFlags,
    _In_ DELTA_FLAG_TYPE ResetFlags,
    _In_ DELTA_INPUT Source,
    _In_ DELTA_INPUT Target,
    _In_ DELTA_INPUT SourceOptions,
    _In_ DELTA_INPUT TargetOptions,
    _In_ DELTA_INPUT GlobalOptions,
    _In_opt_ const FILETIME *lpTargetFileTime,
    _In_ ALG_ID HashAlgId,
    _Out_ LPDELTA_OUTPUT lpDelta
    );

/**
 * Creates a delta from a given source file to a given target file. The resultant delta is written to disk
 * All files accessed by ASCII file names.
 * @param FileTypeSet   File type set.
 * @param SetFlags      Set these flags.
 * @param ResetFlags    Reset (supress) these flags.
 * @param lpSourceName  Source file name, ASCII.
 * @param lpTargetName  Target file name, ASCII.
 * @param lpSourceOptionsName   Name of file with source-specific options, ASCII.
 * @param lpTargetOptionsName   Name of file with target-specific options, ASCII.
 * @param GlobalOptions Memory block with global options.
 * @param lpTargetFileTime  Target file time to use, null to use actual target file time.
 * @param HashAlgId     Algorithm for hashing.
 * @param lpDeltaName   Result delta file name, ASCII.
 * @return              TRUE if sucess, FALSE otherwise.
 */
BOOL
WINAPI
CreateDeltaA(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ DELTA_FLAG_TYPE SetFlags,
    _In_ DELTA_FLAG_TYPE ResetFlags,
    _In_ LPCSTR lpSourceName,
    _In_ LPCSTR lpTargetName,
    _In_opt_ LPCSTR lpSourceOptionsName,
    _In_opt_ LPCSTR lpTargetOptionsName,
    _In_ DELTA_INPUT GlobalOptions,
    _In_opt_ const FILETIME *lpTargetFileTime,
    _In_ ALG_ID HashAlgId,
    _In_ LPCSTR lpDeltaName
    );

/**
 * Creates a delta from a given source file to a given target file. The resultant delta is written to disk
 * All files accessed by Unicode file names.
 * @param FileTypeSet  File type set.
 * @param SetFlags      Set these flags.
 * @param ResetFlags    Reset (supress) these flags.
 * @param lpSourceName  Source file name, Unicode.
 * @param lpTargetName  Target file name, Unicode.
 * @param lpSourceOptionsName   Name of file with source-specific options, Unicode.
 * @param lpTargetOptionsName   Name of file with target-specific options, Unicode.
 * @param GlobalOptions Memory block with global options.
 * @param lpTargetFileTime  Target file time to use, null to use actual target file time.
 * @param HashAlgId     Algorithm for hashing.
 * @param lpDeltaName   Result delta file name, Unicode.
 * @return              TRUE if sucess, FALSE otherwise.
 */
BOOL
WINAPI
CreateDeltaW(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ DELTA_FLAG_TYPE SetFlags,
    _In_ DELTA_FLAG_TYPE ResetFlags,
    _In_ LPCWSTR lpSourceName,
    _In_ LPCWSTR lpTargetName,
    _In_opt_ LPCWSTR lpSourceOptionsName,
    _In_opt_ LPCWSTR lpTargetOptionsName,
    _In_ DELTA_INPUT GlobalOptions,
    _In_opt_ const FILETIME *lpTargetFileTime,
    _In_ ALG_ID HashAlgId,
    _In_ LPCWSTR lpDeltaName
    );

#ifdef _UNICODE
#define CreateDelta                         CreateDeltaW
#else
#define CreateDelta                         CreateDeltaA
#endif /* _UNICODE */


/**
 * Calculates a hash for normalized source file in memory.
 * Normalization is based on source file type,
 * which is determined automatically according to the given file type set.
 * @param FileTypeSet   File type set.
 * @param HashAlgId     Algorithm for hashing.
 * @param Source        Source memory block.
 * @param lpHash        Result hash.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
GetDeltaSignatureB(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ ALG_ID HashAlgId,
    _In_ DELTA_INPUT Source,
    _Out_ LPDELTA_HASH lpHash
    );

/**
 * Calculates a hash for normalized source file accessed by ASCII file name.
 * Normalization is based on source file type,
 * which is determined automatically according to the given file type set.
 * @param FileTypeSet   File type set.
 * @param HashAlgId     Algorithm for hashing.
 * @param lpSourceName  Source file name, ASCII.
 * @param lpHash        Result hash.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
GetDeltaSignatureA(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ ALG_ID HashAlgId,
    _In_ LPCSTR lpSourceName,
    _Out_ LPDELTA_HASH lpHash
    );

/**
 * Calculates a hash for normalized source file accessed by Unicode file name.
 * Normalization is based on source file type,
 * which is determined automatically according to the given file type set.
 * @param FileTypeSet   File type set.
 * @param HashAlgId     Algorithm for hashing.
 * @param lpSourceName  Source file name, Unicode.
 * @param lpHash        Result hash.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
GetDeltaSignatureW(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ ALG_ID HashAlgId,
    _In_ LPCWSTR lpSourceName,
    _Out_ LPDELTA_HASH lpHash
    );


#ifdef _UNICODE
#define GetDeltaSignature                   GetDeltaSignatureW
#else
#define GetDeltaSignature                   GetDeltaSignatureA
#endif /* _UNICODE */

/**
 * Normalizes source buffer, normalization is  based on source file type, which is
 * determined automatically according to the given file type set.
 * @param FileTypeSet   File type set.
 * @param NormalizeFlags    Normalization flags.
 * @param NormalizeOptions  Normalization options.
 * @param lpSource      Pointer to source buffer.
 * @param uSourceSize   Size of source buffer in bytes.
 * @return              TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
DeltaNormalizeProvidedB(
    _In_ DELTA_FILE_TYPE FileTypeSet,
    _In_ DELTA_FLAG_TYPE NormalizeFlags,
    _In_ DELTA_INPUT NormalizeOptions,
    _Inout_updates_bytes_( uSourceSize ) LPVOID lpSource,
    _In_ SIZE_T uSourceSize
    );

/**
 * Frees memory block allocated by msdelta.
 * @param lpMemory Pointer to memory block, previously allocated by msdelta.
 * @return TRUE if success, FALSE otherwise.
 */
BOOL
WINAPI
DeltaFree(
    _In_ LPVOID lpMemory
    );

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CORESETUP) */
#pragma endregion

