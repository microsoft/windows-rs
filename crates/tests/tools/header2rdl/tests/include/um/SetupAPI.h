/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    setupapi.h

Abstract:

    Public header file for Windows NT Setup and Device Installer services Dll.

--*/

#ifndef _INC_SETUPAPI
#define _INC_SETUPAPI

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if defined (_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#endif

//
// Define API decoration for direct importing of DLL references.
//
#if !defined(_SETUPAPI_)
#define WINSETUPAPI DECLSPEC_IMPORT
#else
#define WINSETUPAPI
#endif

//
// determine version of setupapi based on _WIN32_WINDOWS and _WIN32_WINNT
//
// NT4 version of setupapi   (_WIN32_WINNT_NT4) is earliest, and installed onto Win95 by IE.
// Win2k version of setupapi (_WIN32_WINNT_WIN2K) also shipped in WinME
// we'll use "0x0410" to indicate version of setupapi shipped with Win98
//
#ifndef _SETUPAPI_VER
#if defined(_WIN32_WINNT) && (!defined(_WIN32_WINDOWS) || (_WIN32_WINNT < _WIN32_WINDOWS))
#define _SETUPAPI_VER _WIN32_WINNT  // SetupAPI version follows Windows NT version
#elif defined(_WIN32_WINDOWS)
#if _WIN32_WINDOWS >= 0x0490
#define _SETUPAPI_VER _WIN32_WINNT_WIN2K        // WinME uses same version of SetupAPI as Win2k
#elif _WIN32_WINDOWS >= 0x0410
#define _SETUPAPI_VER 0x0410        // Indicates version of SetupAPI shipped with Win98
#else
#define _SETUPAPI_VER _WIN32_WINNT_NT4        // Earliest SetupAPI version
#endif // _WIN32_WINDOWS
#else // _WIN32_WINNT/_WIN32_WINDOWS
#define _SETUPAPI_VER _WIN32_WINNT_WINXP
#endif // _WIN32_WINNT/_WIN32_WINDOWS
#endif // !_SETUPAPI_VER

#ifndef __LPGUID_DEFINED__
#define __LPGUID_DEFINED__
typedef GUID *LPGUID;
#endif

//
// Include spapidef.h for basic definitions and flags
//
#include <spapidef.h>

//
// Include commctrl.h for our use of HIMAGELIST and wizard support.
//
#include <commctrl.h>

//
// Include devpropdef.h for our use of DEVPROPERTYKEY and DEVPROPTYPE.
//
#include <devpropdef.h>

#if defined(_WIN64)
#include <pshpack8.h>   // Assume 8-byte (64-bit) packing throughout
#else
#include <pshpack1.h>   // Assume byte packing throughout (32-bit processor)
#endif

#ifdef __cplusplus
extern "C" {
#endif

//
// Define maximum string length constants
//
#define LINE_LEN                    256 // Windows 9x-compatible maximum for
                                        // displayable strings coming from a
                                        // device INF.
#define MAX_INF_STRING_LENGTH      4096 // Actual maximum size of an INF string
                                        // (including string substitutions).
#define MAX_INF_SECTION_NAME_LENGTH 255 // For Windows 9x compatibility, INF
                                        // section names should be constrained
                                        // to 32 characters.

#define MAX_TITLE_LEN                60
#define MAX_INSTRUCTION_LEN         256
#define MAX_LABEL_LEN                30
#define MAX_SERVICE_NAME_LEN        256
#define MAX_SUBTITLE_LEN            256

//
// Define maximum length of a machine name in the format expected by ConfigMgr32
// CM_Connect_Machine (i.e., "\\\\MachineName\0").
//
#define SP_MAX_MACHINENAME_LENGTH   (MAX_PATH + 3)

//
// Define type for reference to loaded inf file
//
typedef PVOID HINF;

//
// Inf context structure. Applications must not interpret or
// overwrite values in these structures.
//
typedef struct _INFCONTEXT {
    PVOID Inf;
    PVOID CurrentInf;
    UINT Section;
    UINT Line;
} INFCONTEXT, *PINFCONTEXT;

//
// Inf file information structure.
//
typedef struct _SP_INF_INFORMATION {
    DWORD InfStyle;
    DWORD InfCount;
    BYTE VersionData[ANYSIZE_ARRAY];
} SP_INF_INFORMATION, *PSP_INF_INFORMATION;

//
// Define structure for passing alternate platform info into
// SetupSetFileQueueAlternatePlatform and SetupQueryInfOriginalFileInformation.
//
typedef struct _SP_ALTPLATFORM_INFO_V3 {
    DWORD cbSize;
    //
    // platform to use (VER_PLATFORM_WIN32_WINDOWS or VER_PLATFORM_WIN32_NT)
    //
    DWORD Platform;
    //
    // major and minor version numbers to use
    //
    DWORD MajorVersion;
    DWORD MinorVersion;
    //
    // processor architecture to use (PROCESSOR_ARCHITECTURE_INTEL,
    // PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_IA64,
    // or PROCESSOR_ARCHITECTURE_ARM).
    //
    WORD  ProcessorArchitecture;

    union {
        WORD  Reserved; // for compatibility with V1 structure
        WORD  Flags;    // indicates validity of non V1 fields
    } DUMMYUNIONNAME;

    //
    // specify SP_ALTPLATFORM_FLAGS_VERSION_RANGE in Flags
    // to use FirstValidatedMajorVersion and FirstValidatedMinorVersion
    //
    // Major and minor versions of the oldest previous OS for which this
    // package's digital signature may be considered valid.  For example, say
    // the alternate platform is VER_PLATFORM_WIN32_NT, version 5.1.  However,
    // it is wished that driver packages signed with a 5.0 osattr also be
    // considered valid.  In this case, you'd have a  MajorVersion/MinorVersion
    // of 5.1, and a FirstValidatedMajorVersion/FirstValidatedMinorVersion of
    // 5.0.  To validate packages signed for any previous OS release, specify
    // 0 for these fields.  To only validate against the target alternate
    // platform, specify the same values as those in the MajorVersion and
    // MinorVersion fields.
    //
    DWORD FirstValidatedMajorVersion;
    DWORD FirstValidatedMinorVersion;

    //
    // specify non-zero value (e.g. VER_NT_WORKSTATION) in ProductType to use
    // field, and/or specify SP_ALTPLATFORM_FLAGS_SUITE_MASK in Flags to use
    // SuiteMask field, which may be zero.
    //
    // Product type and suite mask of alternate platform.  Used to select
    // matching decorated install sections within driver packages that target
    // specific product variants of the OS.  For example, for only Server
    // products with the Enterprise or Small Business suite classification,
    // use ProductType VER_NT_SERVER with SuiteMask VER_SUITE_ENTERPRISE and
    // VER_SUITE_SMALLBUSINESS.
    //
    BYTE ProductType;
    WORD SuiteMask;

    //
    // Build number of alternate platform.  Used to select matching
    // decorated install sections within driver packages that target a
    // minimal build number with the specified OS
    // MajorVersion/MinorVersion. If no specific minimal build number
    // targeting is required, a value of zero should be specified. Note that
    // this capability is only supported on certain builds of 10.0 and
    // later.
    //
    DWORD BuildNumber;

} SP_ALTPLATFORM_INFO_V3, *PSP_ALTPLATFORM_INFO_V3;

typedef struct _SP_ALTPLATFORM_INFO_V2 {
    DWORD cbSize;
    DWORD Platform;
    DWORD MajorVersion;
    DWORD MinorVersion;
    WORD  ProcessorArchitecture;
    union {
        WORD  Reserved;
        WORD  Flags;
    } DUMMYUNIONNAME;
    DWORD FirstValidatedMajorVersion;
    DWORD FirstValidatedMinorVersion;
} SP_ALTPLATFORM_INFO_V2, *PSP_ALTPLATFORM_INFO_V2;

typedef struct _SP_ALTPLATFORM_INFO_V1 {
    DWORD cbSize;
    DWORD Platform;
    DWORD MajorVersion;
    DWORD MinorVersion;
    WORD  ProcessorArchitecture;
    WORD  Reserved; // must be zero.
} SP_ALTPLATFORM_INFO_V1, *PSP_ALTPLATFORM_INFO_V1;

#if USE_SP_ALTPLATFORM_INFO_V1 || (_SETUPAPI_VER < _WIN32_WINNT_WINXP)
// use version 1 altplatform info data structure
typedef SP_ALTPLATFORM_INFO_V1 SP_ALTPLATFORM_INFO;
typedef PSP_ALTPLATFORM_INFO_V1 PSP_ALTPLATFORM_INFO;

#elif USE_SP_ALTPLATFORM_INFO_V3 && (NTDDI_VERSION >= NTDDI_WIN10_RS1)
// use version 3 altplatform info data structure
typedef SP_ALTPLATFORM_INFO_V3 SP_ALTPLATFORM_INFO;
typedef PSP_ALTPLATFORM_INFO_V3 PSP_ALTPLATFORM_INFO;

#else
// use version 2 altplatform info data structure
typedef SP_ALTPLATFORM_INFO_V2 SP_ALTPLATFORM_INFO;
typedef PSP_ALTPLATFORM_INFO_V2 PSP_ALTPLATFORM_INFO;

#endif  // use default version of altplatform info data structure

//
// SP_ALTPLATFORM_INFO.Flags values
//
#if _WIN32_WINNT >= _WIN32_WINNT_WINXP
#define SP_ALTPLATFORM_FLAGS_VERSION_RANGE (0x0001)     // FirstValidatedMajor/MinorVersion
#endif
#if NTDDI_VERSION >= NTDDI_WIN10_RS1
#define SP_ALTPLATFORM_FLAGS_SUITE_MASK    (0x0002)     // SuiteMask
#endif


//
// Define structure that is filled in by SetupQueryInfOriginalFileInformation
// to indicate the INF's original name and the original name of the (potentially
// platform-specific) catalog file specified by that INF.
//
typedef struct _SP_ORIGINAL_FILE_INFO_A {
    DWORD  cbSize;
    CHAR   OriginalInfName[MAX_PATH];
    CHAR   OriginalCatalogName[MAX_PATH];
} SP_ORIGINAL_FILE_INFO_A, *PSP_ORIGINAL_FILE_INFO_A;

typedef struct _SP_ORIGINAL_FILE_INFO_W {
    DWORD  cbSize;
    WCHAR  OriginalInfName[MAX_PATH];
    WCHAR  OriginalCatalogName[MAX_PATH];
} SP_ORIGINAL_FILE_INFO_W, *PSP_ORIGINAL_FILE_INFO_W;

#ifdef UNICODE
typedef SP_ORIGINAL_FILE_INFO_W SP_ORIGINAL_FILE_INFO;
typedef PSP_ORIGINAL_FILE_INFO_W PSP_ORIGINAL_FILE_INFO;
#else
typedef SP_ORIGINAL_FILE_INFO_A SP_ORIGINAL_FILE_INFO;
typedef PSP_ORIGINAL_FILE_INFO_A PSP_ORIGINAL_FILE_INFO;
#endif

//
// SP_INF_INFORMATION.InfStyle values
//
#define INF_STYLE_NONE           0x00000000       // unrecognized or non-existent
#define INF_STYLE_OLDNT          0x00000001       // winnt 3.x
#define INF_STYLE_WIN4           0x00000002       // Win95

//
// Additional InfStyle flags that may be specified when calling SetupOpenInfFile.
//
//
#define INF_STYLE_CACHE_ENABLE   0x00000010 // always cache INF, even outside of %windir%\Inf
#define INF_STYLE_CACHE_DISABLE  0x00000020 // delete cached INF information
#if _SETUPAPI_VER >= _WIN32_WINNT_WS03
#define INF_STYLE_CACHE_IGNORE   0x00000040 // ignore any cached INF information
#endif


//
// Target directory specs.
//
#define DIRID_ABSOLUTE          -1              // real 32-bit -1
#define DIRID_ABSOLUTE_16BIT     0xffff         // 16-bit -1 for compat w/setupx
#define DIRID_NULL               0
#define DIRID_SRCPATH            1
#define DIRID_WINDOWS           10
#define DIRID_SYSTEM            11              // system32
#define DIRID_DRIVERS           12
#define DIRID_IOSUBSYS          DIRID_DRIVERS
#define DIRID_DRIVER_STORE      13
#define DIRID_INF               17
#define DIRID_HELP              18
#define DIRID_FONTS             20
#define DIRID_VIEWERS           21
#define DIRID_COLOR             23
#define DIRID_APPS              24
#define DIRID_SHARED            25
#define DIRID_BOOT              30

#define DIRID_SYSTEM16          50
#define DIRID_SPOOL             51
#define DIRID_SPOOLDRIVERS      52
#define DIRID_USERPROFILE       53
#define DIRID_LOADER            54
#define DIRID_PRINTPROCESSOR    55

#define DIRID_DEFAULT           DIRID_SYSTEM

//
// The following DIRIDs are for commonly-used shell "special folders".  The
// complete list of such folders is contained in shlobj.h.  In that headerfile,
// each folder is assigned a CSIDL_* value.  The DIRID values below are created
// by taking the CSIDL value in shlobj.h and OR'ing it with 0x4000.  Thus, if
// an INF needs to reference other special folders not defined below, it may
// generate one using the above mechanism, and setupapi will automatically deal
// with it and use the corresponding shell's path where appropriate.  (Remember
// that DIRIDs must be specified in decimal, not hex, in an INF when used for
// string substitution.)
//
#define DIRID_COMMON_STARTMENU        16406  // All Users\Start Menu
#define DIRID_COMMON_PROGRAMS         16407  // All Users\Start Menu\Programs
#define DIRID_COMMON_STARTUP          16408  // All Users\Start Menu\Programs\Startup
#define DIRID_COMMON_DESKTOPDIRECTORY 16409  // All Users\Desktop
#define DIRID_COMMON_FAVORITES        16415  // All Users\Favorites
#define DIRID_COMMON_APPDATA          16419  // All Users\Application Data

#define DIRID_PROGRAM_FILES           16422  // Program Files
#define DIRID_SYSTEM_X86              16425  // system32 for WOW
#define DIRID_PROGRAM_FILES_X86       16426  // Program Files for WOW
#define DIRID_PROGRAM_FILES_COMMON    16427  // Program Files\Common
#define DIRID_PROGRAM_FILES_COMMONX86 16428  // x86 Program Files\Common for WOW

#define DIRID_COMMON_TEMPLATES        16429  // All Users\Templates
#define DIRID_COMMON_DOCUMENTS        16430  // All Users\Documents


//
// First user-definable dirid. See SetupSetDirectoryId().
//
#define DIRID_USER              0x8000


//
// Setup callback notification routine type
//
typedef UINT (CALLBACK* PSP_FILE_CALLBACK_A)(
    _In_ PVOID Context,
    _In_ UINT Notification,
    _In_ UINT_PTR Param1,
    _In_ UINT_PTR Param2
    );

typedef UINT (CALLBACK* PSP_FILE_CALLBACK_W)(
    _In_ PVOID Context,
    _In_ UINT Notification,
    _In_ UINT_PTR Param1,
    _In_ UINT_PTR Param2
    );

#ifdef UNICODE
#define PSP_FILE_CALLBACK PSP_FILE_CALLBACK_W
#else
#define PSP_FILE_CALLBACK PSP_FILE_CALLBACK_A
#endif


//
// Operation/queue start/end notification. These are ordinal values.
//
#define SPFILENOTIFY_STARTQUEUE         0x00000001
#define SPFILENOTIFY_ENDQUEUE           0x00000002
#define SPFILENOTIFY_STARTSUBQUEUE      0x00000003
#define SPFILENOTIFY_ENDSUBQUEUE        0x00000004
#define SPFILENOTIFY_STARTDELETE        0x00000005
#define SPFILENOTIFY_ENDDELETE          0x00000006
#define SPFILENOTIFY_DELETEERROR        0x00000007
#define SPFILENOTIFY_STARTRENAME        0x00000008
#define SPFILENOTIFY_ENDRENAME          0x00000009
#define SPFILENOTIFY_RENAMEERROR        0x0000000a
#define SPFILENOTIFY_STARTCOPY          0x0000000b
#define SPFILENOTIFY_ENDCOPY            0x0000000c
#define SPFILENOTIFY_COPYERROR          0x0000000d
#define SPFILENOTIFY_NEEDMEDIA          0x0000000e
#define SPFILENOTIFY_QUEUESCAN          0x0000000f
//
// These are used with SetupIterateCabinet().
//
#define SPFILENOTIFY_CABINETINFO        0x00000010
#define SPFILENOTIFY_FILEINCABINET      0x00000011
#define SPFILENOTIFY_NEEDNEWCABINET     0x00000012
#define SPFILENOTIFY_FILEEXTRACTED      0x00000013
#define SPFILENOTIFY_FILEOPDELAYED      0x00000014
//
// These are used for backup operations
//
#define SPFILENOTIFY_STARTBACKUP        0x00000015
#define SPFILENOTIFY_BACKUPERROR        0x00000016
#define SPFILENOTIFY_ENDBACKUP          0x00000017
//
// Extended notification for SetupScanFileQueue(Flags=SPQ_SCAN_USE_CALLBACKEX)
//
#define SPFILENOTIFY_QUEUESCAN_EX       0x00000018

#define SPFILENOTIFY_STARTREGISTRATION  0x00000019
#define SPFILENOTIFY_ENDREGISTRATION    0x00000020

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Extended notification for SetupScanFileQueue(Flags=SPQ_SCAN_USE_CALLBACK_SIGNERINFO)
//
#define SPFILENOTIFY_QUEUESCAN_SIGNERINFO 0x00000040

#endif

//
// Copy notification. These are bit flags that may be combined.
//
#define SPFILENOTIFY_LANGMISMATCH       0x00010000
#define SPFILENOTIFY_TARGETEXISTS       0x00020000
#define SPFILENOTIFY_TARGETNEWER        0x00040000

//
// File operation codes and callback outcomes.
//
#define FILEOP_COPY                     0
#define FILEOP_RENAME                   1
#define FILEOP_DELETE                   2
#define FILEOP_BACKUP                   3

#define FILEOP_ABORT                    0
#define FILEOP_DOIT                     1
#define FILEOP_SKIP                     2
#define FILEOP_RETRY                    FILEOP_DOIT
#define FILEOP_NEWPATH                  4

//
// Flags in inf copy sections
//
#define COPYFLG_WARN_IF_SKIP            0x00000001  // warn if user tries to skip file
#define COPYFLG_NOSKIP                  0x00000002  // disallow skipping this file
#define COPYFLG_NOVERSIONCHECK          0x00000004  // ignore versions and overwrite target
#define COPYFLG_FORCE_FILE_IN_USE       0x00000008  // force file-in-use behavior
#define COPYFLG_NO_OVERWRITE            0x00000010  // do not copy if file exists on target
#define COPYFLG_NO_VERSION_DIALOG       0x00000020  // do not copy if target is newer
#define COPYFLG_OVERWRITE_OLDER_ONLY    0x00000040  // leave target alone if version same as source
#define COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE 0x00000100    // a Windows driver file to be
                            // protected as other Windows system files

#define COPYFLG_REPLACEONLY             0x00000400  // copy only if file exists on target
#define COPYFLG_NODECOMP                0x00000800  // don't attempt to decompress file; copy as-is
#define COPYFLG_REPLACE_BOOT_FILE       0x00001000  // file must be present upon reboot (i.e., it's
                                                    // needed by the loader); this flag implies a reboot
#define COPYFLG_NOPRUNE                 0x00002000  // never prune this file
#define COPYFLG_IN_USE_TRY_RENAME       0x00004000  // If file in use, try to rename the target first

//
// Flags in inf delete sections
// New flags go in high word
//
#define DELFLG_IN_USE                   0x00000001  // queue in-use file for delete
#define DELFLG_IN_USE1                  0x00010000  // high-word version of DELFLG_IN_USE

//
// Source and file paths. Used when notifying queue callback
// of SPFILENOTIFY_STARTxxx, SPFILENOTIFY_ENDxxx, and SPFILENOTIFY_xxxERROR.
//
typedef struct _FILEPATHS_A {
    PCSTR  Target;
    PCSTR  Source;  // not used for delete operations
    UINT   Win32Error;
    DWORD  Flags;   // such as SP_COPY_NOSKIP for copy errors
} FILEPATHS_A, *PFILEPATHS_A;

typedef struct _FILEPATHS_W {
    PCWSTR Target;
    PCWSTR Source;  // not used for delete operations
    UINT   Win32Error;
    DWORD  Flags;   // such as SP_COPY_NOSKIP for copy errors
} FILEPATHS_W, *PFILEPATHS_W;

#ifdef UNICODE
typedef FILEPATHS_W FILEPATHS;
typedef PFILEPATHS_W PFILEPATHS;
#else
typedef FILEPATHS_A FILEPATHS;
typedef PFILEPATHS_A PFILEPATHS;
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

typedef struct _FILEPATHS_SIGNERINFO_A {
    PCSTR  Target;
    PCSTR  Source;  // not used for delete operations
    UINT   Win32Error;
    DWORD  Flags;   // such as SP_COPY_NOSKIP for copy errors
    PCSTR  DigitalSigner;
    PCSTR  Version;
    PCSTR  CatalogFile;
} FILEPATHS_SIGNERINFO_A, *PFILEPATHS_SIGNERINFO_A;

typedef struct _FILEPATHS_SIGNERINFO_W {
    PCWSTR Target;
    PCWSTR Source;  // not used for delete operations
    UINT   Win32Error;
    DWORD  Flags;   // such as SP_COPY_NOSKIP for copy errors
    PCWSTR DigitalSigner;
    PCWSTR Version;
    PCWSTR CatalogFile;
} FILEPATHS_SIGNERINFO_W, *PFILEPATHS_SIGNERINFO_W;

#ifdef UNICODE
typedef FILEPATHS_SIGNERINFO_W FILEPATHS_SIGNERINFO;
typedef PFILEPATHS_SIGNERINFO_W PFILEPATHS_SIGNERINFO;
#else
typedef FILEPATHS_SIGNERINFO_A FILEPATHS_SIGNERINFO;
typedef PFILEPATHS_SIGNERINFO_A PFILEPATHS_SIGNERINFO;
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Structure used with SPFILENOTIFY_NEEDMEDIA
//
typedef struct _SOURCE_MEDIA_A {
    PCSTR Reserved;
    PCSTR Tagfile;          // may be NULL
    PCSTR Description;
    //
    // Pathname part and filename part of source file
    // that caused us to need the media.
    //
    PCSTR SourcePath;
    PCSTR SourceFile;
    DWORD Flags;            // subset of SP_COPY_xxx
} SOURCE_MEDIA_A, *PSOURCE_MEDIA_A;

typedef struct _SOURCE_MEDIA_W {
    PCWSTR Reserved;
    PCWSTR Tagfile;         // may be NULL
    PCWSTR Description;
    //
    // Pathname part and filename part of source file
    // that caused us to need the media.
    //
    PCWSTR SourcePath;
    PCWSTR SourceFile;
    DWORD  Flags;           // subset of SP_COPY_xxx
} SOURCE_MEDIA_W, *PSOURCE_MEDIA_W;

#ifdef UNICODE
typedef SOURCE_MEDIA_W SOURCE_MEDIA;
typedef PSOURCE_MEDIA_W PSOURCE_MEDIA;
#else
typedef SOURCE_MEDIA_A SOURCE_MEDIA;
typedef PSOURCE_MEDIA_A PSOURCE_MEDIA;
#endif

//
// Structure used with SPFILENOTIFY_CABINETINFO and
// SPFILENOTIFY_NEEDNEWCABINET
//
typedef struct _CABINET_INFO_A {
    PCSTR CabinetPath;
    PCSTR CabinetFile;
    PCSTR DiskName;
    USHORT SetId;
    USHORT CabinetNumber;
} CABINET_INFO_A, *PCABINET_INFO_A;

typedef struct _CABINET_INFO_W {
    PCWSTR CabinetPath;
    PCWSTR CabinetFile;
    PCWSTR DiskName;
    USHORT SetId;
    USHORT CabinetNumber;
} CABINET_INFO_W, *PCABINET_INFO_W;

#ifdef UNICODE
typedef CABINET_INFO_W CABINET_INFO;
typedef PCABINET_INFO_W PCABINET_INFO;
#else
typedef CABINET_INFO_A CABINET_INFO;
typedef PCABINET_INFO_A PCABINET_INFO;
#endif

//
// Structure used with SPFILENOTIFY_FILEINCABINET
//
typedef struct _FILE_IN_CABINET_INFO_A {
    PCSTR NameInCabinet;
    DWORD FileSize;
    DWORD Win32Error;
    WORD  DosDate;
    WORD  DosTime;
    WORD  DosAttribs;
    CHAR  FullTargetName[MAX_PATH];
} FILE_IN_CABINET_INFO_A, *PFILE_IN_CABINET_INFO_A;

typedef struct _FILE_IN_CABINET_INFO_W {
    PCWSTR NameInCabinet;
    DWORD  FileSize;
    DWORD  Win32Error;
    WORD   DosDate;
    WORD   DosTime;
    WORD   DosAttribs;
    WCHAR  FullTargetName[MAX_PATH];
} FILE_IN_CABINET_INFO_W, *PFILE_IN_CABINET_INFO_W;

#ifdef UNICODE
typedef FILE_IN_CABINET_INFO_W FILE_IN_CABINET_INFO;
typedef PFILE_IN_CABINET_INFO_W PFILE_IN_CABINET_INFO;
#else
typedef FILE_IN_CABINET_INFO_A FILE_IN_CABINET_INFO;
typedef PFILE_IN_CABINET_INFO_A PFILE_IN_CABINET_INFO;
#endif

//
// Structure used for SPFILENOTIFY_***REGISTRATION
// callback
//

typedef struct _SP_REGISTER_CONTROL_STATUSA {
    DWORD    cbSize;
    PCSTR    FileName;
    DWORD    Win32Error;
    DWORD    FailureCode;
} SP_REGISTER_CONTROL_STATUSA, *PSP_REGISTER_CONTROL_STATUSA;

typedef struct _SP_REGISTER_CONTROL_STATUSW {
    DWORD    cbSize;
    PCWSTR   FileName;
    DWORD    Win32Error;
    DWORD    FailureCode;
} SP_REGISTER_CONTROL_STATUSW, *PSP_REGISTER_CONTROL_STATUSW;

#ifdef UNICODE
typedef SP_REGISTER_CONTROL_STATUSW SP_REGISTER_CONTROL_STATUS;
typedef PSP_REGISTER_CONTROL_STATUSW PSP_REGISTER_CONTROL_STATUS;
#else
typedef SP_REGISTER_CONTROL_STATUSA SP_REGISTER_CONTROL_STATUS;
typedef PSP_REGISTER_CONTROL_STATUSA PSP_REGISTER_CONTROL_STATUS;
#endif


//
// valid values for SP_REGISTER_CONTROL_STATUS.FailureCode field
//

#define SPREG_SUCCESS   0x00000000
#define SPREG_LOADLIBRARY   0x00000001
#define SPREG_GETPROCADDR   0x00000002
#define SPREG_REGSVR        0x00000003
#define SPREG_DLLINSTALL    0x00000004
#define SPREG_TIMEOUT   0x00000005
#define SPREG_UNKNOWN   0xFFFFFFFF

//
// Define type for setup file queue
//
typedef PVOID HSPFILEQ;

//
// Structure used with SetupQueueCopyIndirect
//
typedef struct _SP_FILE_COPY_PARAMS_A {
    DWORD    cbSize;
    HSPFILEQ QueueHandle;
    PCSTR    SourceRootPath;     OPTIONAL
    PCSTR    SourcePath;         OPTIONAL
    PCSTR    SourceFilename;
    PCSTR    SourceDescription;  OPTIONAL
    PCSTR    SourceTagfile;      OPTIONAL
    PCSTR    TargetDirectory;
    PCSTR    TargetFilename;     OPTIONAL
    DWORD    CopyStyle;
    HINF     LayoutInf;          OPTIONAL
    PCSTR    SecurityDescriptor; OPTIONAL
} SP_FILE_COPY_PARAMS_A, *PSP_FILE_COPY_PARAMS_A;

typedef struct _SP_FILE_COPY_PARAMS_W {
    DWORD    cbSize;
    HSPFILEQ QueueHandle;
    PCWSTR   SourceRootPath;     OPTIONAL
    PCWSTR   SourcePath;         OPTIONAL
    PCWSTR   SourceFilename;
    PCWSTR   SourceDescription;  OPTIONAL
    PCWSTR   SourceTagfile;      OPTIONAL
    PCWSTR   TargetDirectory;
    PCWSTR   TargetFilename;     OPTIONAL
    DWORD    CopyStyle;
    HINF     LayoutInf;          OPTIONAL
    PCWSTR   SecurityDescriptor; OPTIONAL
} SP_FILE_COPY_PARAMS_W, *PSP_FILE_COPY_PARAMS_W;

#ifdef UNICODE
typedef SP_FILE_COPY_PARAMS_W SP_FILE_COPY_PARAMS;
typedef PSP_FILE_COPY_PARAMS_W PSP_FILE_COPY_PARAMS;
#else
typedef SP_FILE_COPY_PARAMS_A SP_FILE_COPY_PARAMS;
typedef PSP_FILE_COPY_PARAMS_A PSP_FILE_COPY_PARAMS;
#endif


//
// Define type for setup disk space list
//
typedef PVOID HDSKSPC;

//
// Define type for reference to device information set
//
typedef PVOID HDEVINFO;

//
// Device information structure (references a device instance
// that is a member of a device information set)
//
typedef struct _SP_DEVINFO_DATA {
    DWORD cbSize;
    GUID  ClassGuid;
    DWORD DevInst;    // DEVINST handle
    ULONG_PTR Reserved;
} SP_DEVINFO_DATA, *PSP_DEVINFO_DATA;

//
// Device interface information structure (references a device
// interface that is associated with the device information
// element that owns it).
//
typedef struct _SP_DEVICE_INTERFACE_DATA {
    DWORD cbSize;
    GUID  InterfaceClassGuid;
    DWORD Flags;
    ULONG_PTR Reserved;
} SP_DEVICE_INTERFACE_DATA, *PSP_DEVICE_INTERFACE_DATA;

//
// Flags for SP_DEVICE_INTERFACE_DATA.Flags field.
//
#define SPINT_ACTIVE  0x00000001
#define SPINT_DEFAULT 0x00000002
#define SPINT_REMOVED 0x00000004

//
// Backward compatibility--do not use.
//
typedef SP_DEVICE_INTERFACE_DATA  SP_INTERFACE_DEVICE_DATA;
typedef PSP_DEVICE_INTERFACE_DATA PSP_INTERFACE_DEVICE_DATA;
#define SPID_ACTIVE               SPINT_ACTIVE
#define SPID_DEFAULT              SPINT_DEFAULT
#define SPID_REMOVED              SPINT_REMOVED


typedef struct _SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    DWORD  cbSize;
    CHAR   DevicePath[ANYSIZE_ARRAY];
} SP_DEVICE_INTERFACE_DETAIL_DATA_A, *PSP_DEVICE_INTERFACE_DETAIL_DATA_A;

typedef struct _SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    DWORD  cbSize;
    WCHAR  DevicePath[ANYSIZE_ARRAY];
} SP_DEVICE_INTERFACE_DETAIL_DATA_W, *PSP_DEVICE_INTERFACE_DETAIL_DATA_W;

#ifdef UNICODE
typedef SP_DEVICE_INTERFACE_DETAIL_DATA_W SP_DEVICE_INTERFACE_DETAIL_DATA;
typedef PSP_DEVICE_INTERFACE_DETAIL_DATA_W PSP_DEVICE_INTERFACE_DETAIL_DATA;
#else
typedef SP_DEVICE_INTERFACE_DETAIL_DATA_A SP_DEVICE_INTERFACE_DETAIL_DATA;
typedef PSP_DEVICE_INTERFACE_DETAIL_DATA_A PSP_DEVICE_INTERFACE_DETAIL_DATA;
#endif

//
// Backward compatibility--do not use.
//
typedef SP_DEVICE_INTERFACE_DETAIL_DATA_W SP_INTERFACE_DEVICE_DETAIL_DATA_W;
typedef PSP_DEVICE_INTERFACE_DETAIL_DATA_W PSP_INTERFACE_DEVICE_DETAIL_DATA_W;
typedef SP_DEVICE_INTERFACE_DETAIL_DATA_A SP_INTERFACE_DEVICE_DETAIL_DATA_A;
typedef PSP_DEVICE_INTERFACE_DETAIL_DATA_A PSP_INTERFACE_DEVICE_DETAIL_DATA_A;
#ifdef UNICODE
typedef SP_INTERFACE_DEVICE_DETAIL_DATA_W SP_INTERFACE_DEVICE_DETAIL_DATA;
typedef PSP_INTERFACE_DEVICE_DETAIL_DATA_W PSP_INTERFACE_DEVICE_DETAIL_DATA;
#else
typedef SP_INTERFACE_DEVICE_DETAIL_DATA_A SP_INTERFACE_DEVICE_DETAIL_DATA;
typedef PSP_INTERFACE_DEVICE_DETAIL_DATA_A PSP_INTERFACE_DEVICE_DETAIL_DATA;
#endif


//
// Structure for detailed information on a device information set (used for
// SetupDiGetDeviceInfoListDetail which supercedes the functionality of
// SetupDiGetDeviceInfoListClass).
//
typedef struct _SP_DEVINFO_LIST_DETAIL_DATA_A {
    DWORD  cbSize;
    GUID   ClassGuid;
    HANDLE RemoteMachineHandle;
    CHAR   RemoteMachineName[SP_MAX_MACHINENAME_LENGTH];
} SP_DEVINFO_LIST_DETAIL_DATA_A, *PSP_DEVINFO_LIST_DETAIL_DATA_A;

typedef struct _SP_DEVINFO_LIST_DETAIL_DATA_W {
    DWORD  cbSize;
    GUID   ClassGuid;
    HANDLE RemoteMachineHandle;
    WCHAR  RemoteMachineName[SP_MAX_MACHINENAME_LENGTH];
} SP_DEVINFO_LIST_DETAIL_DATA_W, *PSP_DEVINFO_LIST_DETAIL_DATA_W;

#ifdef UNICODE
typedef SP_DEVINFO_LIST_DETAIL_DATA_W SP_DEVINFO_LIST_DETAIL_DATA;
typedef PSP_DEVINFO_LIST_DETAIL_DATA_W PSP_DEVINFO_LIST_DETAIL_DATA;
#else
typedef SP_DEVINFO_LIST_DETAIL_DATA_A SP_DEVINFO_LIST_DETAIL_DATA;
typedef PSP_DEVINFO_LIST_DETAIL_DATA_A PSP_DEVINFO_LIST_DETAIL_DATA;
#endif

//
// Class installer function codes
//
#define DIF_SELECTDEVICE                    0x00000001
#define DIF_INSTALLDEVICE                   0x00000002
#define DIF_ASSIGNRESOURCES                 0x00000003
#define DIF_PROPERTIES                      0x00000004
#define DIF_REMOVE                          0x00000005
#define DIF_FIRSTTIMESETUP                  0x00000006
#define DIF_FOUNDDEVICE                     0x00000007
#define DIF_SELECTCLASSDRIVERS              0x00000008
#define DIF_VALIDATECLASSDRIVERS            0x00000009
#define DIF_INSTALLCLASSDRIVERS             0x0000000A
#define DIF_CALCDISKSPACE                   0x0000000B
#define DIF_DESTROYPRIVATEDATA              0x0000000C
#define DIF_VALIDATEDRIVER                  0x0000000D
#define DIF_DETECT                          0x0000000F
#define DIF_INSTALLWIZARD                   0x00000010
#define DIF_DESTROYWIZARDDATA               0x00000011
#define DIF_PROPERTYCHANGE                  0x00000012
#define DIF_ENABLECLASS                     0x00000013
#define DIF_DETECTVERIFY                    0x00000014
#define DIF_INSTALLDEVICEFILES              0x00000015
#define DIF_UNREMOVE                        0x00000016
#define DIF_SELECTBESTCOMPATDRV             0x00000017
#define DIF_ALLOW_INSTALL                   0x00000018
#define DIF_REGISTERDEVICE                  0x00000019
#define DIF_NEWDEVICEWIZARD_PRESELECT       0x0000001A
#define DIF_NEWDEVICEWIZARD_SELECT          0x0000001B
#define DIF_NEWDEVICEWIZARD_PREANALYZE      0x0000001C
#define DIF_NEWDEVICEWIZARD_POSTANALYZE     0x0000001D
#define DIF_NEWDEVICEWIZARD_FINISHINSTALL   0x0000001E
#define DIF_UNUSED1                         0x0000001F
#define DIF_INSTALLINTERFACES               0x00000020
#define DIF_DETECTCANCEL                    0x00000021
#define DIF_REGISTER_COINSTALLERS           0x00000022
#define DIF_ADDPROPERTYPAGE_ADVANCED        0x00000023
#define DIF_ADDPROPERTYPAGE_BASIC           0x00000024
#define DIF_RESERVED1                       0x00000025
#define DIF_TROUBLESHOOTER                  0x00000026
#define DIF_POWERMESSAGEWAKE                0x00000027
#define DIF_ADDREMOTEPROPERTYPAGE_ADVANCED  0x00000028
#define DIF_UPDATEDRIVER_UI                 0x00000029
#define DIF_FINISHINSTALL_ACTION            0x0000002A
#define DIF_RESERVED2                       0x00000030

//
// Obsoleted DIF codes (do not use)
//
#define DIF_MOVEDEVICE                      0x0000000E


typedef UINT        DI_FUNCTION;    // Function type for device installer


//
// Device installation parameters structure (associated with a
// particular device information element, or globally with a device
// information set)
//
typedef struct _SP_DEVINSTALL_PARAMS_A {
    DWORD             cbSize;
    DWORD             Flags;
    DWORD             FlagsEx;
    HWND              hwndParent;
    PSP_FILE_CALLBACK InstallMsgHandler;
    PVOID             InstallMsgHandlerContext;
    HSPFILEQ          FileQueue;
    ULONG_PTR         ClassInstallReserved;
    DWORD             Reserved;
    CHAR              DriverPath[MAX_PATH];
} SP_DEVINSTALL_PARAMS_A, *PSP_DEVINSTALL_PARAMS_A;

typedef struct _SP_DEVINSTALL_PARAMS_W {
    DWORD             cbSize;
    DWORD             Flags;
    DWORD             FlagsEx;
    HWND              hwndParent;
    PSP_FILE_CALLBACK InstallMsgHandler;
    PVOID             InstallMsgHandlerContext;
    HSPFILEQ          FileQueue;
    ULONG_PTR         ClassInstallReserved;
    DWORD             Reserved;
    WCHAR             DriverPath[MAX_PATH];
} SP_DEVINSTALL_PARAMS_W, *PSP_DEVINSTALL_PARAMS_W;

#ifdef UNICODE
typedef SP_DEVINSTALL_PARAMS_W SP_DEVINSTALL_PARAMS;
typedef PSP_DEVINSTALL_PARAMS_W PSP_DEVINSTALL_PARAMS;
#else
typedef SP_DEVINSTALL_PARAMS_A SP_DEVINSTALL_PARAMS;
typedef PSP_DEVINSTALL_PARAMS_A PSP_DEVINSTALL_PARAMS;
#endif


//
// SP_DEVINSTALL_PARAMS.Flags values
//
// Flags for choosing a device
//
#define DI_SHOWOEM                  0x00000001L     // support Other... button
#define DI_SHOWCOMPAT               0x00000002L     // show compatibility list
#define DI_SHOWCLASS                0x00000004L     // show class list
#define DI_SHOWALL                  0x00000007L     // both class & compat list shown
#define DI_NOVCP                    0x00000008L     // don't create a new copy queue--use
                                                    // caller-supplied FileQueue
#define DI_DIDCOMPAT                0x00000010L     // Searched for compatible devices
#define DI_DIDCLASS                 0x00000020L     // Searched for class devices
#define DI_AUTOASSIGNRES            0x00000040L     // No UI for resources if possible

// flags returned by DiInstallDevice to indicate need to reboot/restart
#define DI_NEEDRESTART              0x00000080L     // Reboot required to take effect
#define DI_NEEDREBOOT               0x00000100L     // ""

// flags for device installation
#define DI_NOBROWSE                 0x00000200L     // no Browse... in InsertDisk

// Flags set by DiBuildDriverInfoList
#define DI_MULTMFGS                 0x00000400L     // Set if multiple manufacturers in
                                                    // class driver list

// Flag indicates that device is disabled
#define DI_DISABLED                 0x00000800L     // Set if device disabled

// Flags for Device/Class Properties
#define DI_GENERALPAGE_ADDED        0x00001000L
#define DI_RESOURCEPAGE_ADDED       0x00002000L

// Flag to indicate the setting properties for this Device (or class) caused a change
// so the Dev Mgr UI probably needs to be updatd.
#define DI_PROPERTIES_CHANGE        0x00004000L

// Flag to indicate that the sorting from the INF file should be used.
#define DI_INF_IS_SORTED            0x00008000L

// Flag to indicate that only the the INF specified by SP_DEVINSTALL_PARAMS.DriverPath
// should be searched.
#define DI_ENUMSINGLEINF            0x00010000L

// Flag that prevents ConfigMgr from removing/re-enumerating devices during device
// registration, installation, and deletion.
#define DI_DONOTCALLCONFIGMG        0x00020000L

// The following flag can be used to install a device disabled
#define DI_INSTALLDISABLED          0x00040000L

// Flag that causes SetupDiBuildDriverInfoList to build a device's compatible driver
// list from its existing class driver list, instead of the normal INF search.
#define DI_COMPAT_FROM_CLASS        0x00080000L

// This flag is set if the Class Install params should be used.
#define DI_CLASSINSTALLPARAMS       0x00100000L

// This flag is set if the caller of DiCallClassInstaller does NOT
// want the internal default action performed if the Class installer
// returns ERROR_DI_DO_DEFAULT.
#define DI_NODI_DEFAULTACTION       0x00200000L

// The setupx flag, DI_NOSYNCPROCESSING (0x00400000L) is not support in the Setup APIs.

// flags for device installation
#define DI_QUIETINSTALL             0x00800000L     // don't confuse the user with
                                                    // questions or excess info
#define DI_NOFILECOPY               0x01000000L     // No file Copy necessary
#define DI_FORCECOPY                0x02000000L     // Force files to be copied from install path
#define DI_DRIVERPAGE_ADDED         0x04000000L     // Prop provider added Driver page.
#define DI_USECI_SELECTSTRINGS      0x08000000L     // Use Class Installer Provided strings in the Select Device Dlg
#define DI_OVERRIDE_INFFLAGS        0x10000000L     // Override INF flags
#define DI_PROPS_NOCHANGEUSAGE      0x20000000L     // No Enable/Disable in General Props

#define DI_NOSELECTICONS            0x40000000L     // No small icons in select device dialogs

#define DI_NOWRITE_IDS              0x80000000L     // Don't write HW & Compat IDs on install


//
// SP_DEVINSTALL_PARAMS.FlagsEx values
//

#define DI_FLAGSEX_RESERVED2                0x00000001L  // DI_FLAGSEX_USEOLDINFSEARCH is obsolete
#define DI_FLAGSEX_RESERVED3                0x00000002L  // DI_FLAGSEX_AUTOSELECTRANK0 is obsolete
#define DI_FLAGSEX_CI_FAILED                0x00000004L  // Failed to Load/Call class installer

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN //

#define DI_FLAGSEX_FINISHINSTALL_ACTION     0x00000008L  // Class/co-installer wants to get a DIF_FINISH_INSTALL action in client context.

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define DI_FLAGSEX_DIDINFOLIST              0x00000010L  // Did the Class Info List
#define DI_FLAGSEX_DIDCOMPATINFO            0x00000020L  // Did the Compat Info List

#define DI_FLAGSEX_FILTERCLASSES            0x00000040L
#define DI_FLAGSEX_SETFAILEDINSTALL         0x00000080L
#define DI_FLAGSEX_DEVICECHANGE             0x00000100L
#define DI_FLAGSEX_ALWAYSWRITEIDS           0x00000200L
#define DI_FLAGSEX_PROPCHANGE_PENDING       0x00000400L  // One or more device property sheets have had changes made
                                                         // to them, and need to have a DIF_PROPERTYCHANGE occur.
#define DI_FLAGSEX_ALLOWEXCLUDEDDRVS        0x00000800L
#define DI_FLAGSEX_NOUIONQUERYREMOVE        0x00001000L
#define DI_FLAGSEX_USECLASSFORCOMPAT        0x00002000L  // Use the device's class when building compat drv list.
                                                         // (Ignored if DI_COMPAT_FROM_CLASS flag is specified.)

#define DI_FLAGSEX_RESERVED4                0x00004000L  // DI_FLAGSEX_OLDINF_IN_CLASSLIST is obsolete

#define DI_FLAGSEX_NO_DRVREG_MODIFY         0x00008000L  // Don't run AddReg and DelReg for device's software (driver) key.
#define DI_FLAGSEX_IN_SYSTEM_SETUP          0x00010000L  // Installation is occurring during initial system setup.
#define DI_FLAGSEX_INET_DRIVER              0x00020000L  // Driver came from Windows Update
#define DI_FLAGSEX_APPENDDRIVERLIST         0x00040000L  // Cause SetupDiBuildDriverInfoList to append
                                                         // a new driver list to an existing list.
#define DI_FLAGSEX_PREINSTALLBACKUP         0x00080000L  // not used
#define DI_FLAGSEX_BACKUPONREPLACE          0x00100000L  // not used
#define DI_FLAGSEX_DRIVERLIST_FROM_URL      0x00200000L  // build driver list from INF(s) retrieved from URL specified
                                                         // in SP_DEVINSTALL_PARAMS.DriverPath (empty string means
                                                         // Windows Update website)
#define DI_FLAGSEX_RESERVED1                0x00400000L
#define DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS 0x00800000L  // Don't include old Internet drivers when building
                                                         // a driver list.
                                                         // Ignored on Windows Vista and later.
#define DI_FLAGSEX_POWERPAGE_ADDED          0x01000000L  // class installer added their own power page

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define DI_FLAGSEX_FILTERSIMILARDRIVERS     0x02000000L  // only include similar drivers in class list
#define DI_FLAGSEX_INSTALLEDDRIVER          0x04000000L  // only add the installed driver to the class or compat
                                                         // driver list.  Used in calls to SetupDiBuildDriverInfoList
#define DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE  0x08000000L  // Don't remove identical driver nodes from the class list
#define DI_FLAGSEX_ALTPLATFORM_DRVSEARCH    0x10000000L  // Build driver list based on alternate platform information
                                                         // specified in associated file queue
#define DI_FLAGSEX_RESTART_DEVICE_ONLY      0x20000000L  // only restart the device drivers are being installed on as
                                                         // opposed to restarting all devices using those drivers.

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define DI_FLAGSEX_RECURSIVESEARCH          0x40000000L  // Tell SetupDiBuildDriverInfoList to do a recursive search
#define DI_FLAGSEX_SEARCH_PUBLISHED_INFS    0x80000000L  // Tell SetupDiBuildDriverInfoList to do a "published INF" search

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

//
// Class installation parameters header.  This must be the first field of any
// class install parameter structure.  The InstallFunction field must be set to
// the function code corresponding to the structure, and the cbSize field must
// be set to the size of the header structure.  E.g.,
//
// SP_ENABLECLASS_PARAMS EnableClassParams;
//
// EnableClassParams.ClassInstallHeader.cbSize = sizeof(SP_CLASSINSTALL_HEADER);
// EnableClassParams.ClassInstallHeader.InstallFunction = DIF_ENABLECLASS;
//
typedef struct _SP_CLASSINSTALL_HEADER {
    DWORD       cbSize;
    DI_FUNCTION InstallFunction;
} SP_CLASSINSTALL_HEADER, *PSP_CLASSINSTALL_HEADER;


//
// Structure corresponding to a DIF_ENABLECLASS install function.
//
typedef struct _SP_ENABLECLASS_PARAMS {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    GUID                   ClassGuid;
    DWORD                  EnableMessage;
} SP_ENABLECLASS_PARAMS, *PSP_ENABLECLASS_PARAMS;

#define ENABLECLASS_QUERY   0
#define ENABLECLASS_SUCCESS 1
#define ENABLECLASS_FAILURE 2


//
// Values indicating a change in a device's state
//
#define DICS_ENABLE      0x00000001
#define DICS_DISABLE     0x00000002
#define DICS_PROPCHANGE  0x00000003
#define DICS_START       0x00000004
#define DICS_STOP        0x00000005
//
// Values specifying the scope of a device property change
//
#define DICS_FLAG_GLOBAL         0x00000001  // make change in all hardware profiles
#define DICS_FLAG_CONFIGSPECIFIC 0x00000002  // make change in specified profile only
#define DICS_FLAG_CONFIGGENERAL  0x00000004  // 1 or more hardware profile-specific
                                             // changes to follow.
//
// Structure corresponding to a DIF_PROPERTYCHANGE install function.
//
typedef struct _SP_PROPCHANGE_PARAMS {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    DWORD                  StateChange;
    DWORD                  Scope;
    DWORD                  HwProfile;
} SP_PROPCHANGE_PARAMS, *PSP_PROPCHANGE_PARAMS;


//
// Structure corresponding to a DIF_REMOVE install function.
//
typedef struct _SP_REMOVEDEVICE_PARAMS {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    DWORD Scope;
    DWORD HwProfile;
} SP_REMOVEDEVICE_PARAMS, *PSP_REMOVEDEVICE_PARAMS;

#define DI_REMOVEDEVICE_GLOBAL                  0x00000001
#define DI_REMOVEDEVICE_CONFIGSPECIFIC          0x00000002


//
// Structure corresponding to a DIF_UNREMOVE install function.
//
typedef struct _SP_UNREMOVEDEVICE_PARAMS {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    DWORD Scope;
    DWORD HwProfile;
} SP_UNREMOVEDEVICE_PARAMS, *PSP_UNREMOVEDEVICE_PARAMS;

#define DI_UNREMOVEDEVICE_CONFIGSPECIFIC        0x00000002


//
// Structure corresponding to a DIF_SELECTDEVICE install function.
//
typedef struct _SP_SELECTDEVICE_PARAMS_A {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    CHAR                   Title[MAX_TITLE_LEN];
    CHAR                   Instructions[MAX_INSTRUCTION_LEN];
    CHAR                   ListLabel[MAX_LABEL_LEN];
    CHAR                   SubTitle[MAX_SUBTITLE_LEN];
    BYTE                   Reserved[2];                  // DWORD size alignment
} SP_SELECTDEVICE_PARAMS_A, *PSP_SELECTDEVICE_PARAMS_A;

typedef struct _SP_SELECTDEVICE_PARAMS_W {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    WCHAR                  Title[MAX_TITLE_LEN];
    WCHAR                  Instructions[MAX_INSTRUCTION_LEN];
    WCHAR                  ListLabel[MAX_LABEL_LEN];
    WCHAR                  SubTitle[MAX_SUBTITLE_LEN];
} SP_SELECTDEVICE_PARAMS_W, *PSP_SELECTDEVICE_PARAMS_W;

#ifdef UNICODE
typedef SP_SELECTDEVICE_PARAMS_W SP_SELECTDEVICE_PARAMS;
typedef PSP_SELECTDEVICE_PARAMS_W PSP_SELECTDEVICE_PARAMS;
#else
typedef SP_SELECTDEVICE_PARAMS_A SP_SELECTDEVICE_PARAMS;
typedef PSP_SELECTDEVICE_PARAMS_A PSP_SELECTDEVICE_PARAMS;
#endif


//
// Callback routine for giving progress notification during detection
//
typedef BOOL (CALLBACK* PDETECT_PROGRESS_NOTIFY)(
    _In_ PVOID ProgressNotifyParam,
    _In_ DWORD DetectComplete
    );

// where:
//     ProgressNotifyParam - value supplied by caller requesting detection.
//     DetectComplete - Percent completion, to be incremented by class
//                      installer, as it steps thru its detection.
//
// Return Value - If TRUE, then detection is cancelled.  Allows caller
//                requesting detection to stop detection asap.
//

//
// Structure corresponding to a DIF_DETECT install function.
//
typedef struct _SP_DETECTDEVICE_PARAMS {
    SP_CLASSINSTALL_HEADER  ClassInstallHeader;
    PDETECT_PROGRESS_NOTIFY DetectProgressNotify;
    PVOID                   ProgressNotifyParam;
} SP_DETECTDEVICE_PARAMS, *PSP_DETECTDEVICE_PARAMS;


//
// 'Add New Device' installation wizard structure (backward-compatibility
// only--respond to DIF_NEWDEVICEWIZARD_* requests instead).
//
// Structure corresponding to a DIF_INSTALLWIZARD install function.
// (NOTE: This structure is also applicable for DIF_DESTROYWIZARDDATA,
// but DIF_INSTALLWIZARD is the associated function code in the class
// installation parameter structure in both cases.)
//
// Define maximum number of dynamic wizard pages that can be added to
// hardware install wizard.
//
#define MAX_INSTALLWIZARD_DYNAPAGES             20

typedef struct _SP_INSTALLWIZARD_DATA {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    DWORD                  Flags;
    HPROPSHEETPAGE         DynamicPages[MAX_INSTALLWIZARD_DYNAPAGES];
    DWORD                  NumDynamicPages;
    DWORD                  DynamicPageFlags;
    DWORD                  PrivateFlags;
    LPARAM                 PrivateData;
    HWND                   hwndWizardDlg;
} SP_INSTALLWIZARD_DATA, *PSP_INSTALLWIZARD_DATA;

//
// SP_INSTALLWIZARD_DATA.Flags values
//
#define NDW_INSTALLFLAG_DIDFACTDEFS         0x00000001
#define NDW_INSTALLFLAG_HARDWAREALLREADYIN  0x00000002
#define NDW_INSTALLFLAG_NEEDRESTART         DI_NEEDRESTART
#define NDW_INSTALLFLAG_NEEDREBOOT          DI_NEEDREBOOT
#define NDW_INSTALLFLAG_NEEDSHUTDOWN        0x00000200
#define NDW_INSTALLFLAG_EXPRESSINTRO        0x00000400
#define NDW_INSTALLFLAG_SKIPISDEVINSTALLED  0x00000800
#define NDW_INSTALLFLAG_NODETECTEDDEVS      0x00001000
#define NDW_INSTALLFLAG_INSTALLSPECIFIC     0x00002000
#define NDW_INSTALLFLAG_SKIPCLASSLIST       0x00004000
#define NDW_INSTALLFLAG_CI_PICKED_OEM       0x00008000
#define NDW_INSTALLFLAG_PCMCIAMODE          0x00010000
#define NDW_INSTALLFLAG_PCMCIADEVICE        0x00020000
#define NDW_INSTALLFLAG_USERCANCEL          0x00040000
#define NDW_INSTALLFLAG_KNOWNCLASS          0x00080000


//
// SP_INSTALLWIZARD_DATA.DynamicPageFlags values
//
// This flag is set if a Class installer has added pages to the install wizard.
//
#define DYNAWIZ_FLAG_PAGESADDED             0x00000001

//
// Set this flag if you jump to the analyze page, and want it to
// handle conflicts for you.  NOTE.  You will not get control back
// in the event of a conflict if you set this flag.
//
#define DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT 0x00000008

//
// The following flags are not used by the Windows NT hardware wizard.
//
#define DYNAWIZ_FLAG_INSTALLDET_NEXT        0x00000002
#define DYNAWIZ_FLAG_INSTALLDET_PREV        0x00000004


//
// Reserve a range of wizard page resource IDs for internal use.  Some of
// these IDs are for use by class installers that respond to the obsolete
// DIF_INSTALLWIZARD/DIF_DESTROYWIZARDDATA messages.  These IDs are listed
// below.
//
#define MIN_IDD_DYNAWIZ_RESOURCE_ID             10000
#define MAX_IDD_DYNAWIZ_RESOURCE_ID             11000

//
// Define wizard page resource IDs to be used when adding custom pages to the
// hardware install wizard via DIF_INSTALLWIZARD.  Pages marked with
// (CLASS INSTALLER PROVIDED) _must_ be supplied by the class installer if it
// responds to the DIF_INSTALLWIZARD request.
//

//
// Resource ID for the first page that the install wizard will go to after
// adding the class installer pages.  (CLASS INSTALLER PROVIDED)
//
#define IDD_DYNAWIZ_FIRSTPAGE                   10000

//
// Resource ID for the page that the Select Device page will go back to.
// (CLASS INSTALLER PROVIDED)
//
#define IDD_DYNAWIZ_SELECT_PREVPAGE             10001

//
// Resource ID for the page that the Select Device page will go forward to.
// (CLASS INSTALLER PROVIDED)
//
#define IDD_DYNAWIZ_SELECT_NEXTPAGE             10002

//
// Resource ID for the page that the Analyze dialog should go back to
// This will only be used in the event that there is a problem, and the user
// selects Back from the analyze proc. (CLASS INSTALLER PROVIDED)
//
#define IDD_DYNAWIZ_ANALYZE_PREVPAGE            10003

//
// Resource ID for the page that the Analyze dialog should go to if it
// continues from the analyze proc. (CLASS INSTALLER PROVIDED)
//
#define IDD_DYNAWIZ_ANALYZE_NEXTPAGE            10004

//
// Resource ID of the hardware install wizard's select device page.
// This ID can be used to go directly to the hardware install wizard's select
// device page.  (This is the resource ID of the Select Device wizard page
// retrieved via SetupDiGetWizardPage when SPWPT_SELECTDEVICE is the requested
// PageType.)
//
#define IDD_DYNAWIZ_SELECTDEV_PAGE              10009

//
// Resource ID of the hardware install wizard's device analysis page.
// This ID can be use to go directly to the hardware install wizard's analysis
// page.
//
#define IDD_DYNAWIZ_ANALYZEDEV_PAGE             10010

//
// Resource ID of the hardware install wizard's install detected devices page.
// This ID can be use to go directly to the hardware install wizard's install
// detected devices page.
//
#define IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE    10011

//
// Resource ID of the hardware install wizard's select class page.
// This ID can be use to go directly to the hardware install wizard's select
// class page.
//
#define IDD_DYNAWIZ_SELECTCLASS_PAGE            10012

//
// The following class installer-provided wizard page resource IDs are not used
// by the Windows NT hardware wizard.
//
#define IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE    10006
#define IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE    10007
#define IDD_DYNAWIZ_INSTALLDETECTED_NODEVS      10008


//
// Structure corresponding to the following DIF_NEWDEVICEWIZARD_* install
// functions:
//
//     DIF_NEWDEVICEWIZARD_PRESELECT
//     DIF_NEWDEVICEWIZARD_SELECT
//     DIF_NEWDEVICEWIZARD_PREANALYZE
//     DIF_NEWDEVICEWIZARD_POSTANALYZE
//     DIF_NEWDEVICEWIZARD_FINISHINSTALL
//
typedef struct _SP_NEWDEVICEWIZARD_DATA {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    DWORD                  Flags;   // presently unused--must be zero.
    HPROPSHEETPAGE         DynamicPages[MAX_INSTALLWIZARD_DYNAPAGES];
    DWORD                  NumDynamicPages;
    HWND                   hwndWizardDlg;
} SP_NEWDEVICEWIZARD_DATA, *PSP_NEWDEVICEWIZARD_DATA;

//
// The same structure is also used for retrieval of property pages via the
// following install functions:
//
//     DIF_ADDPROPERTYPAGE_ADVANCED
//     DIF_ADDPROPERTYPAGE_BASIC
//     DIF_ADDREMOTEPROPERTYPAGE_ADVANCED
//
typedef SP_NEWDEVICEWIZARD_DATA SP_ADDPROPERTYPAGE_DATA;
typedef PSP_NEWDEVICEWIZARD_DATA PSP_ADDPROPERTYPAGE_DATA;


//
// Structure corresponding to the DIF_TROUBLESHOOTER install function
//
typedef struct _SP_TROUBLESHOOTER_PARAMS_A {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    CHAR                   ChmFile[MAX_PATH];
    CHAR                   HtmlTroubleShooter[MAX_PATH];
} SP_TROUBLESHOOTER_PARAMS_A, *PSP_TROUBLESHOOTER_PARAMS_A;

typedef struct _SP_TROUBLESHOOTER_PARAMS_W {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    WCHAR                  ChmFile[MAX_PATH];
    WCHAR                  HtmlTroubleShooter[MAX_PATH];
} SP_TROUBLESHOOTER_PARAMS_W, *PSP_TROUBLESHOOTER_PARAMS_W;

#ifdef UNICODE
typedef SP_TROUBLESHOOTER_PARAMS_W SP_TROUBLESHOOTER_PARAMS;
typedef PSP_TROUBLESHOOTER_PARAMS_W PSP_TROUBLESHOOTER_PARAMS;
#else
typedef SP_TROUBLESHOOTER_PARAMS_A SP_TROUBLESHOOTER_PARAMS;
typedef PSP_TROUBLESHOOTER_PARAMS_A PSP_TROUBLESHOOTER_PARAMS;
#endif


//
// Structure corresponding to the DIF_POWERMESSAGEWAKE install function
//
typedef struct _SP_POWERMESSAGEWAKE_PARAMS_A {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    CHAR                   PowerMessageWake[LINE_LEN*2];
} SP_POWERMESSAGEWAKE_PARAMS_A, *PSP_POWERMESSAGEWAKE_PARAMS_A;

typedef struct _SP_POWERMESSAGEWAKE_PARAMS_W {
    SP_CLASSINSTALL_HEADER ClassInstallHeader;
    WCHAR                  PowerMessageWake[LINE_LEN*2];
} SP_POWERMESSAGEWAKE_PARAMS_W, *PSP_POWERMESSAGEWAKE_PARAMS_W;

#ifdef UNICODE
typedef SP_POWERMESSAGEWAKE_PARAMS_W SP_POWERMESSAGEWAKE_PARAMS;
typedef PSP_POWERMESSAGEWAKE_PARAMS_W PSP_POWERMESSAGEWAKE_PARAMS;
#else
typedef SP_POWERMESSAGEWAKE_PARAMS_A SP_POWERMESSAGEWAKE_PARAMS;
typedef PSP_POWERMESSAGEWAKE_PARAMS_A PSP_POWERMESSAGEWAKE_PARAMS;
#endif

//
// Driver information structure (member of a driver info list that may be associated
// with a particular device instance, or (globally) with a device information set)
//
typedef struct _SP_DRVINFO_DATA_V2_A {
    DWORD     cbSize;
    DWORD     DriverType;
    ULONG_PTR Reserved;
    CHAR      Description[LINE_LEN];
    CHAR      MfgName[LINE_LEN];
    CHAR      ProviderName[LINE_LEN];
    FILETIME  DriverDate;
    DWORDLONG DriverVersion;
} SP_DRVINFO_DATA_V2_A, *PSP_DRVINFO_DATA_V2_A;

typedef struct _SP_DRVINFO_DATA_V2_W {
    DWORD     cbSize;
    DWORD     DriverType;
    ULONG_PTR Reserved;
    WCHAR     Description[LINE_LEN];
    WCHAR     MfgName[LINE_LEN];
    WCHAR     ProviderName[LINE_LEN];
    FILETIME  DriverDate;
    DWORDLONG DriverVersion;
} SP_DRVINFO_DATA_V2_W, *PSP_DRVINFO_DATA_V2_W;

//
// Version 1 of the SP_DRVINFO_DATA structures, used only for compatibility
// with Windows NT 4.0/Windows 95/98 SETUPAPI.DLL
//
typedef struct _SP_DRVINFO_DATA_V1_A {
    DWORD     cbSize;
    DWORD     DriverType;
    ULONG_PTR Reserved;
    CHAR      Description[LINE_LEN];
    CHAR      MfgName[LINE_LEN];
    CHAR      ProviderName[LINE_LEN];
} SP_DRVINFO_DATA_V1_A, *PSP_DRVINFO_DATA_V1_A;

typedef struct _SP_DRVINFO_DATA_V1_W {
    DWORD     cbSize;
    DWORD     DriverType;
    ULONG_PTR Reserved;
    WCHAR     Description[LINE_LEN];
    WCHAR     MfgName[LINE_LEN];
    WCHAR     ProviderName[LINE_LEN];
} SP_DRVINFO_DATA_V1_W, *PSP_DRVINFO_DATA_V1_W;

#ifdef UNICODE
typedef SP_DRVINFO_DATA_V1_W SP_DRVINFO_DATA_V1;
typedef PSP_DRVINFO_DATA_V1_W PSP_DRVINFO_DATA_V1;
typedef SP_DRVINFO_DATA_V2_W SP_DRVINFO_DATA_V2;
typedef PSP_DRVINFO_DATA_V2_W PSP_DRVINFO_DATA_V2;
#else
typedef SP_DRVINFO_DATA_V1_A SP_DRVINFO_DATA_V1;
typedef PSP_DRVINFO_DATA_V1_A PSP_DRVINFO_DATA_V1;
typedef SP_DRVINFO_DATA_V2_A SP_DRVINFO_DATA_V2;
typedef PSP_DRVINFO_DATA_V2_A PSP_DRVINFO_DATA_V2;
#endif

#if USE_SP_DRVINFO_DATA_V1 || (_SETUPAPI_VER < _WIN32_WINNT_WIN2K)  // use version 1 driver info data structure

typedef SP_DRVINFO_DATA_V1_A SP_DRVINFO_DATA_A;
typedef PSP_DRVINFO_DATA_V1_A PSP_DRVINFO_DATA_A;
typedef SP_DRVINFO_DATA_V1_W SP_DRVINFO_DATA_W;
typedef PSP_DRVINFO_DATA_V1_W PSP_DRVINFO_DATA_W;
typedef SP_DRVINFO_DATA_V1 SP_DRVINFO_DATA;
typedef PSP_DRVINFO_DATA_V1 PSP_DRVINFO_DATA;

#else                       // use version 2 driver info data structure

typedef SP_DRVINFO_DATA_V2_A SP_DRVINFO_DATA_A;
typedef PSP_DRVINFO_DATA_V2_A PSP_DRVINFO_DATA_A;
typedef SP_DRVINFO_DATA_V2_W SP_DRVINFO_DATA_W;
typedef PSP_DRVINFO_DATA_V2_W PSP_DRVINFO_DATA_W;
typedef SP_DRVINFO_DATA_V2 SP_DRVINFO_DATA;
typedef PSP_DRVINFO_DATA_V2 PSP_DRVINFO_DATA;

#endif  // use current version of driver info data structure

//
// Driver information details structure (provides detailed information about a
// particular driver information structure)
//
typedef struct _SP_DRVINFO_DETAIL_DATA_A {
    DWORD    cbSize;
    FILETIME InfDate;
    DWORD    CompatIDsOffset;
    DWORD    CompatIDsLength;
    ULONG_PTR Reserved;
    CHAR     SectionName[LINE_LEN];
    CHAR     InfFileName[MAX_PATH];
    CHAR     DrvDescription[LINE_LEN];
    CHAR     HardwareID[ANYSIZE_ARRAY];
} SP_DRVINFO_DETAIL_DATA_A, *PSP_DRVINFO_DETAIL_DATA_A;

typedef struct _SP_DRVINFO_DETAIL_DATA_W {
    DWORD    cbSize;
    FILETIME InfDate;
    DWORD    CompatIDsOffset;
    DWORD    CompatIDsLength;
    ULONG_PTR Reserved;
    WCHAR    SectionName[LINE_LEN];
    WCHAR    InfFileName[MAX_PATH];
    WCHAR    DrvDescription[LINE_LEN];
    WCHAR    HardwareID[ANYSIZE_ARRAY];
} SP_DRVINFO_DETAIL_DATA_W, *PSP_DRVINFO_DETAIL_DATA_W;

#ifdef UNICODE
typedef SP_DRVINFO_DETAIL_DATA_W SP_DRVINFO_DETAIL_DATA;
typedef PSP_DRVINFO_DETAIL_DATA_W PSP_DRVINFO_DETAIL_DATA;
#else
typedef SP_DRVINFO_DETAIL_DATA_A SP_DRVINFO_DETAIL_DATA;
typedef PSP_DRVINFO_DETAIL_DATA_A PSP_DRVINFO_DETAIL_DATA;
#endif


//
// Driver installation parameters (associated with a particular driver
// information element)
//
typedef struct _SP_DRVINSTALL_PARAMS {
    DWORD cbSize;
    DWORD Rank;
    DWORD Flags;
    DWORD_PTR PrivateData;
    DWORD Reserved;
} SP_DRVINSTALL_PARAMS, *PSP_DRVINSTALL_PARAMS;

//
// SP_DRVINSTALL_PARAMS.Flags values
//

#define DNF_DUPDESC             0x00000001  // Multiple providers have same desc
#define DNF_OLDDRIVER           0x00000002  // Driver node specifies old/current driver
#define DNF_EXCLUDEFROMLIST     0x00000004  // If set, this driver node will not be
                                            // displayed in any driver select dialogs.
#define DNF_NODRIVER            0x00000008  // if we want to install no driver
                                            // (e.g no mouse drv)
#define DNF_LEGACYINF           0x00000010  // Driver node came from an old-style INF (obsolete)
#define DNF_CLASS_DRIVER        0x00000020  // Driver node represents a class driver
#define DNF_COMPATIBLE_DRIVER   0x00000040  // Driver node represents a compatible driver
#define DNF_INET_DRIVER         0x00000080  // Driver comes from an internet source
#define DNF_UNUSED1             0x00000100
#define DNF_UNUSED2             0x00000200
#define DNF_OLD_INET_DRIVER     0x00000400  // Driver came from the Internet, but we don't currently
                                            // have access to it's source files.  Never attempt to
                                            // install a driver with this flag!
                                            // Note used on Windows Vista and Later.
#define DNF_BAD_DRIVER          0x00000800  // Driver node should not be used at all
#define DNF_DUPPROVIDER         0x00001000  // Multiple drivers have the same provider and desc

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP
#define DNF_INF_IS_SIGNED         0x00002000  // If file is digitally signed
#define DNF_OEM_F6_INF            0x00004000  // INF specified from F6 during textmode setup.
#define DNF_DUPDRIVERVER          0x00008000  // Multipe drivers have the same desc, provider, and DriverVer values
#define DNF_BASIC_DRIVER          0x00010000  // Driver provides basic functionality, but should
                                              // not be chosen if other signed drivers exist.
#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_WS03
#define DNF_AUTHENTICODE_SIGNED   0x00020000  // Inf file is signed by an Authenticode(tm) catalog.
#endif // _SETUPAPI_VER >= _WIN32_WINNT_WS03

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN
#define DNF_INSTALLEDDRIVER       0x00040000  // This driver node is currently installed on the device.
#define DNF_ALWAYSEXCLUDEFROMLIST 0x00080000  // If set, this driver is not even displayed in
                                              // alternative platform either.
#define DNF_INBOX_DRIVER          0x00100000  // This driver node came from an INF that shipped with Windows.
#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN7
#define DNF_REQUESTADDITIONALSOFTWARE   0x00200000  // This driver is only part of a software solution needed
                                                    // by this device
#endif // _SETUPAPI_VER >= _WIN32_WINNT_WIN7

#define DNF_UNUSED_22             0x00400000
#define DNF_UNUSED_23             0x00800000
#define DNF_UNUSED_24             0x01000000
#define DNF_UNUSED_25             0x02000000
#define DNF_UNUSED_26             0x04000000
#define DNF_UNUSED_27             0x08000000
#define DNF_UNUSED_28             0x10000000
#define DNF_UNUSED_29             0x20000000
#define DNF_UNUSED_30             0x40000000
#define DNF_UNUSED_31             0x80000000


//
// Rank values (the lower the Rank number, the better the Rank)
//
#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN
#define DRIVER_HARDWAREID_RANK  0x00000FFF  // Any rank less than or equal to
                                            // this value is a gold
                                            // HardwareID match

#define DRIVER_HARDWAREID_MASK  0x80000FFF  // If you mask these bits off (AND)
                                            // from the Rank and the result is 0
                                            // then the Rank is a trusted HardwareID
                                            // match

#define DRIVER_UNTRUSTED_RANK   0x80000000  // Any rank with this bit set is an
                                            // "untrusted" rank, meaning that
                                            // the INF was unsigned.

#define DRIVER_W9X_SUSPECT_RANK 0xC0000000  // Any rank that is greater than
                                            // or equal to this value, and lesser
                                            // than or equal to 0xFFFF is suspected
                                            // to be a Win9x-only driver, because
                                            // (a) it isn't signed, and (b) there
                                            // is no NT-specific decoration to
                                            // explicitly indicate that the INF
                                            // supports Windows NT/2000/XP

#else
#define DRIVER_HARDWAREID_RANK  0x00000FFF  // Any rank less than or equal to
                                            // this value is a trusted
                                            // HardwareID match

#define DRIVER_COMPATID_RANK    0x00003FFF  // Any rank less than or equal to
                                            // this (and greater than
                                            // DRIVER_HARDWAREID_RANK) is a
                                            // trusted CompatibleID match

#define DRIVER_UNTRUSTED_RANK   0x00008000  // Any rank with this bit set is an
                                            // "untrusted" rank, meaning that
                                            // the INF was unsigned.

#define DRIVER_UNTRUSTED_HARDWAREID_RANK  0x00008FFF  // Any rank less than or equal to
                                                      // this value (and greater than
                                                      // or equal to DRIVER_UNTRUSTED_RANK)
                                                      // is an untrusted HardwareID match

#define DRIVER_UNTRUSTED_COMPATID_RANK    0x0000BFFF  // Any rank less than or equal to
                                                      // this value (and greater than
                                                      // DRIVER_UNTRUSTED_HARDWAREID_RANK)
                                                      // is an untrusted CompatibleID match

#define DRIVER_W9X_SUSPECT_RANK            0x0000C000 // Any rank that is greater than
                                                      // or equal to this value, and lesser
                                                      // than or equal to 0xFFFF is suspected
                                                      // to be a Win9x-only driver, because
                                                      // (a) it isn't signed, and (b) there
                                                      // is no NT-specific decoration to
                                                      // explicitly indicate that the INF
                                                      // supports Windows NT/2000/XP

#define DRIVER_W9X_SUSPECT_HARDWAREID_RANK 0x0000CFFF // Any rank less than or equal to this
                                                      // (and greater than or equal to
                                                      // DRIVER_W9X_SUSPECT_RANK) is a
                                                      // hardware ID match suspected of being
                                                      // only for Windows 9x platforms.

#define DRIVER_W9X_SUSPECT_COMPATID_RANK   0x0000FFFF // Any rank less than or equal to
                                                      // this (and greater than
                                                      // DRIVER_W9X_SUSPECT_HARDWAREID_RANK)
                                                      // is a compatible ID match suspected
                                                      // of being only for Windows 9x
                                                      // platforms.
#endif // _SETUPAPI_VER < _WIN32_WINNT_LONGHORN

//
// Setup callback routine for comparing detection signatures
//
typedef DWORD (CALLBACK* PSP_DETSIG_CMPPROC)(
    _In_ HDEVINFO         DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA NewDeviceData,
    _In_ PSP_DEVINFO_DATA ExistingDeviceData,
    _In_opt_ PVOID            CompareContext
    );


//
// Define context structure handed to co-installers
//
typedef struct _COINSTALLER_CONTEXT_DATA {
    BOOL  PostProcessing;
    DWORD InstallResult;
    PVOID PrivateData;
} COINSTALLER_CONTEXT_DATA, *PCOINSTALLER_CONTEXT_DATA;


//
// Structure containing class image list information.
//
typedef struct _SP_CLASSIMAGELIST_DATA {
    DWORD      cbSize;
    HIMAGELIST ImageList;
    ULONG_PTR  Reserved;
} SP_CLASSIMAGELIST_DATA, *PSP_CLASSIMAGELIST_DATA;


//
// Structure to be passed as first parameter (LPVOID lpv) to ExtensionPropSheetPageProc
// entry point in setupapi.dll or to "EnumPropPages32" or "BasicProperties32" entry
// points provided by class/device property page providers.  Used to retrieve a handle
// (or, potentially, multiple handles) to property pages for a specified property page type.
//
typedef struct _SP_PROPSHEETPAGE_REQUEST {
    DWORD            cbSize;
    DWORD            PageRequested;
    HDEVINFO         DeviceInfoSet;
    PSP_DEVINFO_DATA DeviceInfoData;
} SP_PROPSHEETPAGE_REQUEST, *PSP_PROPSHEETPAGE_REQUEST;

//
// Property sheet codes used in SP_PROPSHEETPAGE_REQUEST.PageRequested
//
#define SPPSR_SELECT_DEVICE_RESOURCES      1    // supplied by setupapi.dll
#define SPPSR_ENUM_BASIC_DEVICE_PROPERTIES 2    // supplied by device's BasicProperties32 provider
#define SPPSR_ENUM_ADV_DEVICE_PROPERTIES   3    // supplied by class and/or device's EnumPropPages32 provider


//
// Structure used with SetupGetBackupInformation/SetupSetBackupInformation
//
typedef struct _SP_BACKUP_QUEUE_PARAMS_V2_A {
    DWORD    cbSize;                            // size of structure
    CHAR     FullInfPath[MAX_PATH];             // buffer to hold ANSI pathname of INF file
    INT      FilenameOffset;                    // offset in CHAR's of filename part (after '\')
    CHAR     ReinstallInstance[MAX_PATH];       // Instance ID (if present)
} SP_BACKUP_QUEUE_PARAMS_V2_A, *PSP_BACKUP_QUEUE_PARAMS_V2_A;

typedef struct _SP_BACKUP_QUEUE_PARAMS_V2_W {
    DWORD    cbSize;                            // size of structure
    WCHAR    FullInfPath[MAX_PATH];             // buffer to hold UNICODE pathname of INF file
    INT      FilenameOffset;                    // offset in WCHAR's of filename part (after '\')
    WCHAR    ReinstallInstance[MAX_PATH];       // Instance ID (if present)
} SP_BACKUP_QUEUE_PARAMS_V2_W, *PSP_BACKUP_QUEUE_PARAMS_V2_W;

//
// Version 1 of the SP_BACKUP_QUEUE_PARAMS structures, used only for compatibility
// with Windows 2000/Windows 95/98/ME SETUPAPI.DLL
//
typedef struct _SP_BACKUP_QUEUE_PARAMS_V1_A {
    DWORD    cbSize;                            // size of structure
    CHAR     FullInfPath[MAX_PATH];             // buffer to hold ANSI pathname of INF file
    INT      FilenameOffset;                    // offset in CHAR's of filename part (after '\')
} SP_BACKUP_QUEUE_PARAMS_V1_A, *PSP_BACKUP_QUEUE_PARAMS_V1_A;

typedef struct _SP_BACKUP_QUEUE_PARAMS_V1_W {
    DWORD    cbSize;                            // size of structure
    WCHAR    FullInfPath[MAX_PATH];             // buffer to hold UNICODE pathname of INF file
    INT      FilenameOffset;                    // offset in WCHAR's of filename part (after '\')
} SP_BACKUP_QUEUE_PARAMS_V1_W, *PSP_BACKUP_QUEUE_PARAMS_V1_W;

#ifdef UNICODE
typedef SP_BACKUP_QUEUE_PARAMS_V1_W SP_BACKUP_QUEUE_PARAMS_V1;
typedef PSP_BACKUP_QUEUE_PARAMS_V1_W PSP_BACKUP_QUEUE_PARAMS_V1;
typedef SP_BACKUP_QUEUE_PARAMS_V2_W SP_BACKUP_QUEUE_PARAMS_V2;
typedef PSP_BACKUP_QUEUE_PARAMS_V2_W PSP_BACKUP_QUEUE_PARAMS_V2;
#else
typedef SP_BACKUP_QUEUE_PARAMS_V1_A SP_BACKUP_QUEUE_PARAMS_V1;
typedef PSP_BACKUP_QUEUE_PARAMS_V1_A PSP_BACKUP_QUEUE_PARAMS_V1;
typedef SP_BACKUP_QUEUE_PARAMS_V2_A SP_BACKUP_QUEUE_PARAMS_V2;
typedef PSP_BACKUP_QUEUE_PARAMS_V2_A PSP_BACKUP_QUEUE_PARAMS_V2;
#endif


#if USE_SP_BACKUP_QUEUE_PARAMS_V1 || (_SETUPAPI_VER < _WIN32_WINNT_WINXP)  // use version 1 driver info data structure

typedef SP_BACKUP_QUEUE_PARAMS_V1_A SP_BACKUP_QUEUE_PARAMS_A;
typedef PSP_BACKUP_QUEUE_PARAMS_V1_A PSP_BACKUP_QUEUE_PARAMS_A;
typedef SP_BACKUP_QUEUE_PARAMS_V1_W SP_BACKUP_QUEUE_PARAMS_W;
typedef PSP_BACKUP_QUEUE_PARAMS_V1_W PSP_BACKUP_QUEUE_PARAMS_W;
typedef SP_BACKUP_QUEUE_PARAMS_V1 SP_BACKUP_QUEUE_PARAMS;
typedef PSP_BACKUP_QUEUE_PARAMS_V1 PSP_BACKUP_QUEUE_PARAMS;

#else                       // use version 2 driver info data structure

typedef SP_BACKUP_QUEUE_PARAMS_V2_A SP_BACKUP_QUEUE_PARAMS_A;
typedef PSP_BACKUP_QUEUE_PARAMS_V2_A PSP_BACKUP_QUEUE_PARAMS_A;
typedef SP_BACKUP_QUEUE_PARAMS_V2_W SP_BACKUP_QUEUE_PARAMS_W;
typedef PSP_BACKUP_QUEUE_PARAMS_V2_W PSP_BACKUP_QUEUE_PARAMS_W;
typedef SP_BACKUP_QUEUE_PARAMS_V2 SP_BACKUP_QUEUE_PARAMS;
typedef PSP_BACKUP_QUEUE_PARAMS_V2 PSP_BACKUP_QUEUE_PARAMS;

#endif  // use current version of driver info data structure






#ifndef _SPAPI_ERRORS
#define _SPAPI_ERRORS

//
// Setupapi-specific error codes
//
// Inf parse outcomes
//
#define ERROR_EXPECTED_SECTION_NAME  (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0)
#define ERROR_BAD_SECTION_NAME_LINE  (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|1)
#define ERROR_SECTION_NAME_TOO_LONG  (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|2)
#define ERROR_GENERAL_SYNTAX         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|3)
//
// Inf runtime errors
//
#define ERROR_WRONG_INF_STYLE        (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x100)
#define ERROR_SECTION_NOT_FOUND      (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x101)
#define ERROR_LINE_NOT_FOUND         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x102)
#define ERROR_NO_BACKUP              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x103)
//
// Device Installer/other errors
//
#define ERROR_NO_ASSOCIATED_CLASS                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x200)
#define ERROR_CLASS_MISMATCH                     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x201)
#define ERROR_DUPLICATE_FOUND                    (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x202)
#define ERROR_NO_DRIVER_SELECTED                 (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x203)
#define ERROR_KEY_DOES_NOT_EXIST                 (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x204)
#define ERROR_INVALID_DEVINST_NAME               (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x205)
#define ERROR_INVALID_CLASS                      (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x206)
#define ERROR_DEVINST_ALREADY_EXISTS             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x207)
#define ERROR_DEVINFO_NOT_REGISTERED             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x208)
#define ERROR_INVALID_REG_PROPERTY               (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x209)
#define ERROR_NO_INF                             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x20A)
#define ERROR_NO_SUCH_DEVINST                    (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x20B)
#define ERROR_CANT_LOAD_CLASS_ICON               (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x20C)
#define ERROR_INVALID_CLASS_INSTALLER            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x20D)
#define ERROR_DI_DO_DEFAULT                      (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x20E)
#define ERROR_DI_NOFILECOPY                      (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x20F)
#define ERROR_INVALID_HWPROFILE                  (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x210)
#define ERROR_NO_DEVICE_SELECTED                 (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x211)
#define ERROR_DEVINFO_LIST_LOCKED                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x212)
#define ERROR_DEVINFO_DATA_LOCKED                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x213)
#define ERROR_DI_BAD_PATH                        (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x214)
#define ERROR_NO_CLASSINSTALL_PARAMS             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x215)
#define ERROR_FILEQUEUE_LOCKED                   (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x216)
#define ERROR_BAD_SERVICE_INSTALLSECT            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x217)
#define ERROR_NO_CLASS_DRIVER_LIST               (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x218)
#define ERROR_NO_ASSOCIATED_SERVICE              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x219)
#define ERROR_NO_DEFAULT_DEVICE_INTERFACE        (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x21A)
#define ERROR_DEVICE_INTERFACE_ACTIVE            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x21B)
#define ERROR_DEVICE_INTERFACE_REMOVED           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x21C)
#define ERROR_BAD_INTERFACE_INSTALLSECT          (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x21D)
#define ERROR_NO_SUCH_INTERFACE_CLASS            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x21E)
#define ERROR_INVALID_REFERENCE_STRING           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x21F)
#define ERROR_INVALID_MACHINENAME                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x220)
#define ERROR_REMOTE_COMM_FAILURE                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x221)
#define ERROR_MACHINE_UNAVAILABLE                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x222)
#define ERROR_NO_CONFIGMGR_SERVICES              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x223)
#define ERROR_INVALID_PROPPAGE_PROVIDER          (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x224)
#define ERROR_NO_SUCH_DEVICE_INTERFACE           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x225)
#define ERROR_DI_POSTPROCESSING_REQUIRED         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x226)
#define ERROR_INVALID_COINSTALLER                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x227)
#define ERROR_NO_COMPAT_DRIVERS                  (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x228)
#define ERROR_NO_DEVICE_ICON                     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x229)
#define ERROR_INVALID_INF_LOGCONFIG              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x22A)
#define ERROR_DI_DONT_INSTALL                    (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x22B)
#define ERROR_INVALID_FILTER_DRIVER              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x22C)
#define ERROR_NON_WINDOWS_NT_DRIVER              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x22D)
#define ERROR_NON_WINDOWS_DRIVER                 (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x22E)
#define ERROR_NO_CATALOG_FOR_OEM_INF             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x22F)
#define ERROR_DEVINSTALL_QUEUE_NONNATIVE         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x230)
#define ERROR_NOT_DISABLEABLE                    (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x231)
#define ERROR_CANT_REMOVE_DEVINST                (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x232)
#define ERROR_INVALID_TARGET                     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x233)
#define ERROR_DRIVER_NONNATIVE                   (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x234)
#define ERROR_IN_WOW64                           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x235)
#define ERROR_SET_SYSTEM_RESTORE_POINT           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x236)

#define ERROR_SCE_DISABLED                       (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x238)
#define ERROR_UNKNOWN_EXCEPTION                  (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x239)
#define ERROR_PNP_REGISTRY_ERROR                 (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x23A)
#define ERROR_REMOTE_REQUEST_UNSUPPORTED         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x23B)
#define ERROR_NOT_AN_INSTALLED_OEM_INF           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x23C)
#define ERROR_INF_IN_USE_BY_DEVICES              (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x23D)
#define ERROR_DI_FUNCTION_OBSOLETE               (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x23E)
#define ERROR_NO_AUTHENTICODE_CATALOG            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x23F)
#define ERROR_AUTHENTICODE_DISALLOWED            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x240)
#define ERROR_AUTHENTICODE_TRUSTED_PUBLISHER     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x241)
#define ERROR_AUTHENTICODE_TRUST_NOT_ESTABLISHED (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x242)
#define ERROR_AUTHENTICODE_PUBLISHER_NOT_TRUSTED (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x243)
#define ERROR_SIGNATURE_OSATTRIBUTE_MISMATCH     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x244)
#define ERROR_ONLY_VALIDATE_VIA_AUTHENTICODE     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x245)
#define ERROR_DEVICE_INSTALLER_NOT_READY         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x246)
#define ERROR_DRIVER_STORE_ADD_FAILED            (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x247)
#define ERROR_DEVICE_INSTALL_BLOCKED             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x248)
#define ERROR_DRIVER_INSTALL_BLOCKED             (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x249)
#define ERROR_WRONG_INF_TYPE                     (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x24A)
#define ERROR_FILE_HASH_NOT_IN_CATALOG           (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x24B)
#define ERROR_DRIVER_STORE_DELETE_FAILED         (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x24C)

//
// Setupapi exception codes
//
#define ERROR_UNRECOVERABLE_STACK_OVERFLOW (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x300)
#define EXCEPTION_SPAPI_UNRECOVERABLE_STACK_OVERFLOW ERROR_UNRECOVERABLE_STACK_OVERFLOW

//
// Backward compatibility--do not use.
//
#define ERROR_NO_DEFAULT_INTERFACE_DEVICE ERROR_NO_DEFAULT_DEVICE_INTERFACE
#define ERROR_INTERFACE_DEVICE_ACTIVE     ERROR_DEVICE_INTERFACE_ACTIVE
#define ERROR_INTERFACE_DEVICE_REMOVED    ERROR_DEVICE_INTERFACE_REMOVED
#define ERROR_NO_SUCH_INTERFACE_DEVICE    ERROR_NO_SUCH_DEVICE_INTERFACE


//
// Win9x migration DLL error code
//
#define ERROR_NOT_INSTALLED (APPLICATION_ERROR_MASK|ERROR_SEVERITY_ERROR|0x1000)

#endif // _SPAPI_ERRORS

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupGetInfInformationA(
    _In_ LPCVOID InfSpec,
    _In_ DWORD SearchControl,
    _Out_writes_bytes_to_opt_(ReturnBufferSize, *RequiredSize) PSP_INF_INFORMATION ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupGetInfInformationW(
    _In_ LPCVOID InfSpec,
    _In_ DWORD SearchControl,
    _Out_writes_bytes_to_opt_(ReturnBufferSize, *RequiredSize) PSP_INF_INFORMATION ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

//
// SearchControl flags for SetupGetInfInformation
//
#define INFINFO_INF_SPEC_IS_HINF        1
#define INFINFO_INF_NAME_IS_ABSOLUTE    2
#define INFINFO_DEFAULT_SEARCH          3
#define INFINFO_REVERSE_DEFAULT_SEARCH  4
#define INFINFO_INF_PATH_LIST_SEARCH    5

#ifdef UNICODE
#define SetupGetInfInformation SetupGetInfInformationW
#else
#define SetupGetInfInformation SetupGetInfInformationA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueryInfFileInformationA(
    _In_ PSP_INF_INFORMATION InfInformation,
    _In_ UINT InfIndex,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueryInfFileInformationW(
    _In_ PSP_INF_INFORMATION InfInformation,
    _In_ UINT InfIndex,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupQueryInfFileInformation SetupQueryInfFileInformationW
#else
#define SetupQueryInfFileInformation SetupQueryInfFileInformationA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueryInfOriginalFileInformationA(
    _In_ PSP_INF_INFORMATION InfInformation,
    _In_ UINT InfIndex,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _Out_ PSP_ORIGINAL_FILE_INFO_A OriginalFileInfo
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueryInfOriginalFileInformationW(
    _In_ PSP_INF_INFORMATION InfInformation,
    _In_ UINT InfIndex,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _Out_ PSP_ORIGINAL_FILE_INFO_W OriginalFileInfo
    );

#ifdef UNICODE
#define SetupQueryInfOriginalFileInformation SetupQueryInfOriginalFileInformationW
#else
#define SetupQueryInfOriginalFileInformation SetupQueryInfOriginalFileInformationA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueryInfVersionInformationA(
    _In_ PSP_INF_INFORMATION InfInformation,
    _In_ UINT InfIndex,
    _In_opt_ PCSTR Key,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueryInfVersionInformationW(
    _In_ PSP_INF_INFORMATION InfInformation,
    _In_ UINT InfIndex,
    _In_opt_ PCWSTR Key,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupQueryInfVersionInformation SetupQueryInfVersionInformationW
#else
#define SetupQueryInfVersionInformation SetupQueryInfVersionInformationA
#endif


#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

WINSETUPAPI
BOOL
WINAPI
SetupGetInfDriverStoreLocationA(
    _In_ PCSTR FileName,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _In_opt_ PCSTR LocaleName,
    _Out_writes_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetInfDriverStoreLocationW(
    _In_ PCWSTR FileName,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _In_opt_ PCWSTR LocaleName,
    _Out_writes_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetInfDriverStoreLocation SetupGetInfDriverStoreLocationW
#else
#define SetupGetInfDriverStoreLocation SetupGetInfDriverStoreLocationA
#endif

WINSETUPAPI
BOOL
WINAPI
SetupGetInfPublishedNameA(
    _In_ PCSTR DriverStoreLocation,
    _Out_writes_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetInfPublishedNameW(
    _In_ PCWSTR DriverStoreLocation,
    _Out_writes_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetInfPublishedName SetupGetInfPublishedNameW
#else
#define SetupGetInfPublishedName SetupGetInfPublishedNameA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN


WINSETUPAPI
BOOL
WINAPI
SetupGetInfFileListA(
    _In_opt_ PCSTR DirectoryPath,
    _In_ DWORD InfStyle,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetInfFileListW(
    _In_opt_ PCWSTR DirectoryPath,
    _In_ DWORD InfStyle,
    _Out_writes_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetInfFileList SetupGetInfFileListW
#else
#define SetupGetInfFileList SetupGetInfFileListA
#endif


WINSETUPAPI
HINF
WINAPI
SetupOpenInfFileW(
    _In_ PCWSTR FileName,
    _In_opt_ PCWSTR InfClass,
    _In_ DWORD InfStyle,
    _Out_opt_ PUINT ErrorLine
    );

WINSETUPAPI
HINF
WINAPI
SetupOpenInfFileA(
    _In_ PCSTR FileName,
    _In_opt_ PCSTR InfClass,
    _In_ DWORD InfStyle,
    _Out_opt_ PUINT ErrorLine
    );

#ifdef UNICODE
#define SetupOpenInfFile SetupOpenInfFileW
#else
#define SetupOpenInfFile SetupOpenInfFileA
#endif


WINSETUPAPI
HINF
WINAPI
SetupOpenMasterInf(
    VOID
    );


WINSETUPAPI
BOOL
WINAPI
SetupOpenAppendInfFileW(
    _In_opt_ PCWSTR FileName,
    _In_ HINF InfHandle,
    _Out_opt_ PUINT ErrorLine
    );

WINSETUPAPI
BOOL
WINAPI
SetupOpenAppendInfFileA(
    _In_opt_ PCSTR FileName,
    _In_ HINF InfHandle,
    _Out_opt_ PUINT ErrorLine
    );

#ifdef UNICODE
#define SetupOpenAppendInfFile SetupOpenAppendInfFileW
#else
#define SetupOpenAppendInfFile SetupOpenAppendInfFileA
#endif


WINSETUPAPI
VOID
WINAPI
SetupCloseInfFile(
    _In_ HINF InfHandle
    );


WINSETUPAPI
BOOL
WINAPI
SetupFindFirstLineA(
    _In_ HINF InfHandle,
    _In_ PCSTR Section,
    _In_opt_ PCSTR Key,
    _Out_ PINFCONTEXT Context
    );

WINSETUPAPI
BOOL
WINAPI
SetupFindFirstLineW(
    _In_ HINF InfHandle,
    _In_ PCWSTR Section,
    _In_opt_ PCWSTR Key,
    _Out_ PINFCONTEXT Context
    );

#ifdef UNICODE
#define SetupFindFirstLine SetupFindFirstLineW
#else
#define SetupFindFirstLine SetupFindFirstLineA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupFindNextLine(
    _In_ PINFCONTEXT ContextIn,
    _Out_ PINFCONTEXT ContextOut
    );


WINSETUPAPI
BOOL
WINAPI
SetupFindNextMatchLineA(
    _In_ PINFCONTEXT ContextIn,
    _In_opt_ PCSTR Key,
    _Out_ PINFCONTEXT ContextOut
    );

WINSETUPAPI
BOOL
WINAPI
SetupFindNextMatchLineW(
    _In_ PINFCONTEXT ContextIn,
    _In_opt_ PCWSTR Key,
    _Out_ PINFCONTEXT ContextOut
    );

#ifdef UNICODE
#define SetupFindNextMatchLine SetupFindNextMatchLineW
#else
#define SetupFindNextMatchLine SetupFindNextMatchLineA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupGetLineByIndexA(
    _In_ HINF InfHandle,
    _In_ PCSTR Section,
    _In_ DWORD Index,
    _Out_ PINFCONTEXT Context
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetLineByIndexW(
    _In_ HINF InfHandle,
    _In_ PCWSTR Section,
    _In_ DWORD Index,
    _Out_ PINFCONTEXT Context
    );

#ifdef UNICODE
#define SetupGetLineByIndex SetupGetLineByIndexW
#else
#define SetupGetLineByIndex SetupGetLineByIndexA
#endif


WINSETUPAPI
LONG
WINAPI
SetupGetLineCountA(
    _In_ HINF InfHandle,
    _In_ PCSTR Section
    );

WINSETUPAPI
LONG
WINAPI
SetupGetLineCountW(
    _In_ HINF InfHandle,
    _In_ PCWSTR Section
    );

#ifdef UNICODE
#define SetupGetLineCount SetupGetLineCountW
#else
#define SetupGetLineCount SetupGetLineCountA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupGetLineTextA(
    _In_opt_ PINFCONTEXT Context,
    _In_opt_ HINF InfHandle,
    _In_opt_ PCSTR Section,
    _In_opt_ PCSTR Key,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetLineTextW(
    _In_opt_ PINFCONTEXT Context,
    _In_opt_ HINF InfHandle,
    _In_opt_ PCWSTR Section,
    _In_opt_ PCWSTR Key,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetLineText SetupGetLineTextW
#else
#define SetupGetLineText SetupGetLineTextA
#endif


WINSETUPAPI
DWORD
WINAPI
SetupGetFieldCount(
    _In_ PINFCONTEXT Context
    );


WINSETUPAPI
BOOL
WINAPI
SetupGetStringFieldA(
    _In_ PINFCONTEXT Context,
    _In_ DWORD FieldIndex,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetStringFieldW(
    _In_ PINFCONTEXT Context,
    _In_ DWORD FieldIndex,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetStringField SetupGetStringFieldW
#else
#define SetupGetStringField SetupGetStringFieldA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupGetIntField(
    _In_ PINFCONTEXT Context,
    _In_ DWORD FieldIndex,
    _Out_ PINT IntegerValue
    );


WINSETUPAPI
BOOL
WINAPI
SetupGetMultiSzFieldA(
    _In_ PINFCONTEXT Context,
    _In_ DWORD FieldIndex,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ LPDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetMultiSzFieldW(
    _In_ PINFCONTEXT Context,
    _In_ DWORD FieldIndex,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ LPDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetMultiSzField SetupGetMultiSzFieldW
#else
#define SetupGetMultiSzField SetupGetMultiSzFieldA
#endif

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupGetBinaryField(
    _In_ PINFCONTEXT Context,
    _In_ DWORD FieldIndex,
    _Out_writes_bytes_to_opt_(ReturnBufferSize, *RequiredSize) PBYTE ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ LPDWORD RequiredSize
    );

//
// SetupGetFileCompressionInfo is depreciated
// use SetupGetFileCompressionInfoEx instead
//
// ActualSourceFileName returned by SetupGetFileCompressionInfo
// must be freed by the export setupapi!MyFree (NT4+ Win95+)
// or LocalFree (Win2k+)
//
WINSETUPAPI
DWORD
WINAPI
SetupGetFileCompressionInfoA(
    _In_ PCSTR SourceFileName,
    _Out_ PSTR *ActualSourceFileName,
    _Out_ PDWORD SourceFileSize,
    _Out_ PDWORD TargetFileSize,
    _Out_ PUINT CompressionType
    );

WINSETUPAPI
DWORD
WINAPI
SetupGetFileCompressionInfoW(
    _In_ PCWSTR SourceFileName,
    _Out_ PWSTR *ActualSourceFileName,
    _Out_ PDWORD SourceFileSize,
    _Out_ PDWORD TargetFileSize,
    _Out_ PUINT CompressionType
    );

#ifdef UNICODE
#define SetupGetFileCompressionInfo SetupGetFileCompressionInfoW
#else
#define SetupGetFileCompressionInfo SetupGetFileCompressionInfoA
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// SetupGetFileCompressionInfoEx is the preferred API over
// SetupGetFileCompressionInfo. It follows the normal
// conventions of returning BOOL and writing to user-supplied
// buffer.
//

WINSETUPAPI
BOOL
WINAPI
SetupGetFileCompressionInfoExA(
    _In_ PCSTR SourceFileName,
    _In_reads_opt_(ActualSourceFileNameBufferLen) PSTR ActualSourceFileNameBuffer,
    _In_ DWORD ActualSourceFileNameBufferLen,
    _Out_opt_ PDWORD RequiredBufferLen,
    _Out_ PDWORD SourceFileSize,
    _Out_ PDWORD TargetFileSize,
    _Out_ PUINT CompressionType
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetFileCompressionInfoExW(
    _In_ PCWSTR SourceFileName,
    _In_reads_opt_(ActualSourceFileNameBufferLen) PWSTR ActualSourceFileNameBuffer,
    _In_ DWORD ActualSourceFileNameBufferLen,
    _Out_opt_ PDWORD RequiredBufferLen,
    _Out_ PDWORD SourceFileSize,
    _Out_ PDWORD TargetFileSize,
    _Out_ PUINT CompressionType
    );

#ifdef UNICODE
#define SetupGetFileCompressionInfoEx SetupGetFileCompressionInfoExW
#else
#define SetupGetFileCompressionInfoEx SetupGetFileCompressionInfoExA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Compression types
//
#define FILE_COMPRESSION_NONE       0
#define FILE_COMPRESSION_WINLZA     1
#define FILE_COMPRESSION_MSZIP      2
#define FILE_COMPRESSION_NTCAB      3


WINSETUPAPI
DWORD
WINAPI
SetupDecompressOrCopyFileA(
    _In_ PCSTR SourceFileName,
    _In_ PCSTR TargetFileName,
    _In_opt_ PUINT CompressionType
    );

WINSETUPAPI
DWORD
WINAPI
SetupDecompressOrCopyFileW(
    _In_ PCWSTR SourceFileName,
    _In_ PCWSTR TargetFileName,
    _In_opt_ PUINT CompressionType
    );

#ifdef UNICODE
#define SetupDecompressOrCopyFile SetupDecompressOrCopyFileW
#else
#define SetupDecompressOrCopyFile SetupDecompressOrCopyFileA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupGetSourceFileLocationA(
    _In_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCSTR FileName,
    _Out_ PUINT SourceId,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetSourceFileLocationW(
    _In_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCWSTR FileName,
    _Out_ PUINT SourceId,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetSourceFileLocation SetupGetSourceFileLocationW
#else
#define SetupGetSourceFileLocation SetupGetSourceFileLocationA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupGetSourceFileSizeA(
    _In_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCSTR FileName,
    _In_opt_ PCSTR Section,
    _Out_ PDWORD FileSize,
    _In_ UINT RoundingFactor
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetSourceFileSizeW(
    _In_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCWSTR FileName,
    _In_opt_ PCWSTR Section,
    _Out_ PDWORD FileSize,
    _In_ UINT RoundingFactor
    );

#ifdef UNICODE
#define SetupGetSourceFileSize SetupGetSourceFileSizeW
#else
#define SetupGetSourceFileSize SetupGetSourceFileSizeA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupGetTargetPathA(
    _In_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCSTR Section,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetTargetPathW(
    _In_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCWSTR Section,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetTargetPath SetupGetTargetPathW
#else
#define SetupGetTargetPath SetupGetTargetPathA
#endif


//
// Define flags for SourceList APIs.
//
#define SRCLIST_TEMPORARY       0x00000001
#define SRCLIST_NOBROWSE        0x00000002
#define SRCLIST_SYSTEM          0x00000010
#define SRCLIST_USER            0x00000020
#define SRCLIST_SYSIFADMIN      0x00000040
#define SRCLIST_SUBDIRS         0x00000100
#define SRCLIST_APPEND          0x00000200
#define SRCLIST_NOSTRIPPLATFORM 0x00000400


WINSETUPAPI
BOOL
WINAPI
SetupSetSourceListA(
    _In_ DWORD Flags,
    _In_reads_(SourceCount) PCSTR *SourceList,
    _In_ UINT SourceCount
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetSourceListW(
    _In_ DWORD Flags,
    _In_reads_(SourceCount) PCWSTR *SourceList,
    _In_ UINT SourceCount
    );

#ifdef UNICODE
#define SetupSetSourceList SetupSetSourceListW
#else
#define SetupSetSourceList SetupSetSourceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupCancelTemporarySourceList(
    VOID
    );


WINSETUPAPI
BOOL
WINAPI
SetupAddToSourceListA(
    _In_ DWORD Flags,
    _In_ PCSTR Source
    );

WINSETUPAPI
BOOL
WINAPI
SetupAddToSourceListW(
    _In_ DWORD Flags,
    _In_ PCWSTR Source
    );

#ifdef UNICODE
#define SetupAddToSourceList SetupAddToSourceListW
#else
#define SetupAddToSourceList SetupAddToSourceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupRemoveFromSourceListA(
    _In_ DWORD Flags,
    _In_ PCSTR Source
    );

WINSETUPAPI
BOOL
WINAPI
SetupRemoveFromSourceListW(
    _In_ DWORD Flags,
    _In_ PCWSTR Source
    );

#ifdef UNICODE
#define SetupRemoveFromSourceList SetupRemoveFromSourceListW
#else
#define SetupRemoveFromSourceList SetupRemoveFromSourceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQuerySourceListA(
    _In_ DWORD Flags,
    _Outptr_result_buffer_(*Count) PCSTR **List,
    _Out_ PUINT Count
    );

WINSETUPAPI
BOOL
WINAPI
SetupQuerySourceListW(
    _In_ DWORD Flags,
    _Outptr_result_buffer_(*Count) PCWSTR **List,
    _Out_ PUINT Count
    );

#ifdef UNICODE
#define SetupQuerySourceList SetupQuerySourceListW
#else
#define SetupQuerySourceList SetupQuerySourceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupFreeSourceListA(
    _Inout_ _At_(*List, _Pre_readable_size_(Count) _Post_null_) PCSTR **List,
    _In_ UINT Count
    );

WINSETUPAPI
BOOL
WINAPI
SetupFreeSourceListW(
    _Inout_ _At_(*List, _Pre_readable_size_(Count) _Post_null_) PCWSTR **List,
    _In_ UINT Count
    );

#ifdef UNICODE
#define SetupFreeSourceList SetupFreeSourceListW
#else
#define SetupFreeSourceList SetupFreeSourceListA
#endif


WINSETUPAPI
UINT
WINAPI
SetupPromptForDiskA(
    _In_ HWND hwndParent,
    _In_opt_ PCSTR DialogTitle,
    _In_opt_ PCSTR DiskName,
    _In_opt_ PCSTR PathToSource,
    _In_ PCSTR FileSought,
    _In_opt_ PCSTR TagFile,
    _In_ DWORD DiskPromptStyle,
    _Out_writes_opt_(PathBufferSize) PSTR PathBuffer,
    _In_ DWORD PathBufferSize,
    _Out_opt_ PDWORD PathRequiredSize
    );

WINSETUPAPI
UINT
WINAPI
SetupPromptForDiskW(
    _In_ HWND hwndParent,
    _In_opt_ PCWSTR DialogTitle,
    _In_opt_ PCWSTR DiskName,
    _In_opt_ PCWSTR PathToSource,
    _In_ PCWSTR FileSought,
    _In_opt_ PCWSTR TagFile,
    _In_ DWORD DiskPromptStyle,
    _Out_writes_opt_(PathBufferSize) PWSTR PathBuffer,
    _In_ DWORD PathBufferSize,
    _Out_opt_ PDWORD PathRequiredSize
    );

#ifdef UNICODE
#define SetupPromptForDisk SetupPromptForDiskW
#else
#define SetupPromptForDisk SetupPromptForDiskA
#endif


WINSETUPAPI
UINT
WINAPI
SetupCopyErrorA(
    _In_ HWND hwndParent,
    _In_opt_ PCSTR DialogTitle,
    _In_opt_ PCSTR DiskName,
    _In_ PCSTR PathToSource,
    _In_ PCSTR SourceFile,
    _In_opt_ PCSTR TargetPathFile,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style,
    _Out_writes_opt_(PathBufferSize) PSTR PathBuffer,
    _In_ DWORD PathBufferSize,
    _Out_opt_ PDWORD PathRequiredSize
    );

WINSETUPAPI
UINT
WINAPI
SetupCopyErrorW(
    _In_ HWND hwndParent,
    _In_opt_ PCWSTR DialogTitle,
    _In_opt_ PCWSTR DiskName,
    _In_ PCWSTR PathToSource,
    _In_ PCWSTR SourceFile,
    _In_opt_ PCWSTR TargetPathFile,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style,
    _Out_writes_opt_(PathBufferSize) PWSTR PathBuffer,
    _In_ DWORD PathBufferSize,
    _Out_opt_ PDWORD PathRequiredSize
    );

#ifdef UNICODE
#define SetupCopyError SetupCopyErrorW
#else
#define SetupCopyError SetupCopyErrorA
#endif


WINSETUPAPI
UINT
WINAPI
SetupRenameErrorA(
    _In_ HWND hwndParent,
    _In_opt_ PCSTR DialogTitle,
    _In_ PCSTR SourceFile,
    _In_ PCSTR TargetFile,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style
    );

WINSETUPAPI
UINT
WINAPI
SetupRenameErrorW(
    _In_ HWND hwndParent,
    _In_opt_ PCWSTR DialogTitle,
    _In_ PCWSTR SourceFile,
    _In_ PCWSTR TargetFile,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style
    );

#ifdef UNICODE
#define SetupRenameError SetupRenameErrorW
#else
#define SetupRenameError SetupRenameErrorA
#endif


WINSETUPAPI
UINT
WINAPI
SetupDeleteErrorA(
    _In_ HWND hwndParent,
    _In_opt_ PCSTR DialogTitle,
    _In_ PCSTR File,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style
    );

WINSETUPAPI
UINT
WINAPI
SetupDeleteErrorW(
    _In_ HWND hwndParent,
    _In_opt_ PCWSTR DialogTitle,
    _In_ PCWSTR File,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style
    );

#ifdef UNICODE
#define SetupDeleteError SetupDeleteErrorW
#else
#define SetupDeleteError SetupDeleteErrorA
#endif

WINSETUPAPI
UINT
WINAPI
SetupBackupErrorA(
    _In_ HWND hwndParent,
    _In_opt_ PCSTR DialogTitle,
    _In_ PCSTR SourceFile,
    _In_opt_ PCSTR TargetFile,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style
    );

WINSETUPAPI
UINT
WINAPI
SetupBackupErrorW(
    _In_ HWND hwndParent,
    _In_opt_ PCWSTR DialogTitle,
    _In_ PCWSTR SourceFile,
    _In_opt_ PCWSTR TargetFile,
    _In_ UINT Win32ErrorCode,
    _In_ DWORD Style
    );

#ifdef UNICODE
#define SetupBackupError SetupBackupErrorW
#else
#define SetupBackupError SetupBackupErrorA
#endif


//
// Styles for SetupPromptForDisk, SetupCopyError,
// SetupRenameError, SetupDeleteError
//
#define IDF_NOBROWSE                    0x00000001
#define IDF_NOSKIP                      0x00000002
#define IDF_NODETAILS                   0x00000004
#define IDF_NOCOMPRESSED                0x00000008
#define IDF_CHECKFIRST                  0x00000100
#define IDF_NOBEEP                      0x00000200
#define IDF_NOFOREGROUND                0x00000400
#define IDF_WARNIFSKIP                  0x00000800

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define IDF_NOREMOVABLEMEDIAPROMPT      0x00001000
#define IDF_USEDISKNAMEASPROMPT         0x00002000
#define IDF_OEMDISK                     0x80000000

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Return values for SetupPromptForDisk, SetupCopyError,
// SetupRenameError, SetupDeleteError, SetupBackupError
//
#define DPROMPT_SUCCESS         0
#define DPROMPT_CANCEL          1
#define DPROMPT_SKIPFILE        2
#define DPROMPT_BUFFERTOOSMALL  3
#define DPROMPT_OUTOFMEMORY     4


WINSETUPAPI
BOOL
WINAPI
SetupSetDirectoryIdA(
    _In_ HINF InfHandle,
    _In_ DWORD Id,
    _In_opt_ PCSTR Directory
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetDirectoryIdW(
    _In_ HINF InfHandle,
    _In_ DWORD Id,
    _In_opt_ PCWSTR Directory
    );

#ifdef UNICODE
#define SetupSetDirectoryId SetupSetDirectoryIdW
#else
#define SetupSetDirectoryId SetupSetDirectoryIdA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupSetDirectoryIdExA(
    _In_ HINF InfHandle,
    _In_ DWORD Id,
    _In_opt_ PCSTR Directory,
    _In_ DWORD Flags,
    _Reserved_ DWORD Reserved1,
    _Reserved_ PVOID Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetDirectoryIdExW(
    _In_ HINF InfHandle,
    _In_ DWORD Id,
    _In_opt_ PCWSTR Directory,
    _In_ DWORD Flags,
    _Reserved_ DWORD Reserved1,
    _Reserved_ PVOID Reserved2
    );

#ifdef UNICODE
#define SetupSetDirectoryIdEx SetupSetDirectoryIdExW
#else
#define SetupSetDirectoryIdEx SetupSetDirectoryIdExA
#endif

//
// Flags for SetupSetDirectoryIdEx
//
#define SETDIRID_NOT_FULL_PATH      0x00000001


WINSETUPAPI
BOOL
WINAPI
SetupGetSourceInfoA(
    _In_ HINF InfHandle,
    _In_ UINT SourceId,
    _In_ UINT InfoDesired,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetSourceInfoW(
    _In_ HINF InfHandle,
    _In_ UINT SourceId,
    _In_ UINT InfoDesired,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupGetSourceInfo SetupGetSourceInfoW
#else
#define SetupGetSourceInfo SetupGetSourceInfoA
#endif

//
// InfoDesired values for SetupGetSourceInfo
//

#define SRCINFO_PATH            1
#define SRCINFO_TAGFILE         2
#define SRCINFO_DESCRIPTION     3
#define SRCINFO_FLAGS           4

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP
//
// SRC_FLAGS allow special treatment of source
// lower 4 bits are reserved for OS use
// the flags may determine what other parameters exist
//
#define SRCINFO_TAGFILE2        5  // alternate tagfile, when SRCINFO_TAGFILE is a cabfile

#define SRC_FLAGS_CABFILE       (0x0010) // if set, treat SRCINFO_TAGFILE as a cabfile and specify alternate tagfile

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupInstallFileA(
    _In_opt_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCSTR SourceFile,
    _In_opt_ PCSTR SourcePathRoot,
    _In_opt_ PCSTR DestinationName,
    _In_ DWORD CopyStyle,
    _In_opt_ PSP_FILE_CALLBACK_A CopyMsgHandler,
    _In_opt_ PVOID Context
    );

WINSETUPAPI
BOOL
WINAPI
SetupInstallFileW(
    _In_opt_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCWSTR SourceFile,
    _In_opt_ PCWSTR SourcePathRoot,
    _In_opt_ PCWSTR DestinationName,
    _In_ DWORD CopyStyle,
    _In_opt_ PSP_FILE_CALLBACK_W CopyMsgHandler,
    _In_opt_ PVOID Context
    );

#ifdef UNICODE
#define SetupInstallFile SetupInstallFileW
#else
#define SetupInstallFile SetupInstallFileA
#endif

WINSETUPAPI
BOOL
WINAPI
SetupInstallFileExA(
    _In_opt_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCSTR SourceFile,
    _In_opt_ PCSTR SourcePathRoot,
    _In_opt_ PCSTR DestinationName,
    _In_ DWORD CopyStyle,
    _In_opt_ PSP_FILE_CALLBACK_A CopyMsgHandler,
    _In_opt_ PVOID Context,
    _Out_ PBOOL FileWasInUse
    );

WINSETUPAPI
BOOL
WINAPI
SetupInstallFileExW(
    _In_opt_ HINF InfHandle,
    _In_opt_ PINFCONTEXT InfContext,
    _In_opt_ PCWSTR SourceFile,
    _In_opt_ PCWSTR SourcePathRoot,
    _In_opt_ PCWSTR DestinationName,
    _In_ DWORD CopyStyle,
    _In_opt_ PSP_FILE_CALLBACK_W CopyMsgHandler,
    _In_opt_ PVOID Context,
    _Out_ PBOOL FileWasInUse
    );

#ifdef UNICODE
#define SetupInstallFileEx SetupInstallFileExW
#else
#define SetupInstallFileEx SetupInstallFileExA
#endif

//
// CopyStyle values for copy and queue-related APIs
//
#define SP_COPY_DELETESOURCE        0x0000001   // delete source file on successful copy
#define SP_COPY_REPLACEONLY         0x0000002   // copy only if target file already present
#define SP_COPY_NEWER               0x0000004   // copy only if source newer than or same as target
#define SP_COPY_NEWER_OR_SAME       SP_COPY_NEWER
#define SP_COPY_NOOVERWRITE         0x0000008   // copy only if target doesn't exist
#define SP_COPY_NODECOMP            0x0000010   // don't decompress source file while copying
#define SP_COPY_LANGUAGEAWARE       0x0000020   // don't overwrite file of different language
#define SP_COPY_SOURCE_ABSOLUTE     0x0000040   // SourceFile is a full source path
#define SP_COPY_SOURCEPATH_ABSOLUTE 0x0000080   // SourcePathRoot is the full path
#define SP_COPY_IN_USE_NEEDS_REBOOT 0x0000100   // System needs reboot if file in use
#define SP_COPY_FORCE_IN_USE        0x0000200   // Force target-in-use behavior
#define SP_COPY_NOSKIP              0x0000400   // Skip is disallowed for this file or section
#define SP_FLAG_CABINETCONTINUATION 0x0000800   // Used with need media notification
#define SP_COPY_FORCE_NOOVERWRITE   0x0001000   // like NOOVERWRITE but no callback nofitication
#define SP_COPY_FORCE_NEWER         0x0002000   // like NEWER but no callback nofitication
#define SP_COPY_WARNIFSKIP          0x0004000   // system critical file: warn if user tries to skip
#define SP_COPY_NOBROWSE            0x0008000   // Browsing is disallowed for this file or section
#define SP_COPY_NEWER_ONLY          0x0010000   // copy only if source file newer than target
#define SP_COPY_RESERVED            0x0020000   // was: SP_COPY_SOURCE_SIS_MASTER (deprecated)
#define SP_COPY_OEMINF_CATALOG_ONLY 0x0040000   // (SetupCopyOEMInf only) don't copy INF--just catalog
#define SP_COPY_REPLACE_BOOT_FILE   0x0080000   // file must be present upon reboot (i.e., it's
                                                // needed by the loader); this flag implies a reboot
#define SP_COPY_NOPRUNE             0x0100000   // never prune this file

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define SP_COPY_OEM_F6_INF          0x0200000   // Used when calling SetupCopyOemInf

#endif //_SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define SP_COPY_ALREADYDECOMP       0x0400000   // similar to SP_COPY_NODECOMP

#endif //_SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define SP_COPY_WINDOWS_SIGNED      0x1000000   // BuildLab or WinSE signed
#define SP_COPY_PNPLOCKED           0x2000000   // Used with the signature flag
#define SP_COPY_IN_USE_TRY_RENAME   0x4000000   // If file in use, try to rename the target first
#define SP_COPY_INBOX_INF           0x8000000   // Referred by CopyFiles of inbox inf

#endif //_SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN7

#define SP_COPY_HARDLINK            0x10000000  // Copy using hardlink, if possible

#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Flags passed to Backup notification
//
#define SP_BACKUP_BACKUPPASS        0x00000001  // file backed up during backup pass
#define SP_BACKUP_DEMANDPASS        0x00000002  // file backed up on demand
#define SP_BACKUP_SPECIAL           0x00000004  // if set, special type of backup
#define SP_BACKUP_BOOTFILE          0x00000008  // file marked with COPYFLG_REPLACE_BOOT_FILE


#endif //_SETUPAPI_VER >= _WIN32_WINNT_WINXP


WINSETUPAPI
HSPFILEQ
WINAPI
SetupOpenFileQueue(
    VOID
    );

WINSETUPAPI
BOOL
WINAPI
SetupCloseFileQueue(
    _In_ HSPFILEQ QueueHandle
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetFileQueueAlternatePlatformA(
    _In_ HSPFILEQ QueueHandle,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _In_opt_ PCSTR AlternateDefaultCatalogFile
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetFileQueueAlternatePlatformW(
    _In_ HSPFILEQ QueueHandle,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _In_opt_ PCWSTR AlternateDefaultCatalogFile
    );

#ifdef UNICODE
#define SetupSetFileQueueAlternatePlatform SetupSetFileQueueAlternatePlatformW
#else
#define SetupSetFileQueueAlternatePlatform SetupSetFileQueueAlternatePlatformA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupSetPlatformPathOverrideA(
    _In_opt_ PCSTR Override
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetPlatformPathOverrideW(
    _In_opt_ PCWSTR Override
    );

#ifdef UNICODE
#define SetupSetPlatformPathOverride SetupSetPlatformPathOverrideW
#else
#define SetupSetPlatformPathOverride SetupSetPlatformPathOverrideA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueCopyA(
    _In_ HSPFILEQ QueueHandle,
    _In_opt_ PCSTR SourceRootPath,
    _In_opt_ PCSTR SourcePath,
    _In_ PCSTR SourceFilename,
    _In_opt_ PCSTR SourceDescription,
    _In_opt_ PCSTR SourceTagfile,
    _In_ PCSTR TargetDirectory,
    _In_opt_ PCSTR TargetFilename,
    _In_ DWORD CopyStyle
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueCopyW(
    _In_ HSPFILEQ QueueHandle,
    _In_opt_ PCWSTR SourceRootPath,
    _In_opt_ PCWSTR SourcePath,
    _In_ PCWSTR SourceFilename,
    _In_opt_ PCWSTR SourceDescription,
    _In_opt_ PCWSTR SourceTagfile,
    _In_ PCWSTR TargetDirectory,
    _In_opt_ PCWSTR TargetFilename,
    _In_ DWORD CopyStyle
    );

#ifdef UNICODE
#define SetupQueueCopy SetupQueueCopyW
#else
#define SetupQueueCopy SetupQueueCopyA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueCopyIndirectA(
    _In_ PSP_FILE_COPY_PARAMS_A CopyParams
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueCopyIndirectW(
    _In_ PSP_FILE_COPY_PARAMS_W CopyParams
    );

#ifdef UNICODE
#define SetupQueueCopyIndirect SetupQueueCopyIndirectW
#else
#define SetupQueueCopyIndirect SetupQueueCopyIndirectA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueDefaultCopyA(
    _In_ HSPFILEQ QueueHandle,
    _In_ HINF InfHandle,
    _In_opt_ PCSTR SourceRootPath,
    _In_ PCSTR SourceFilename,
    _In_opt_ PCSTR TargetFilename,
    _In_ DWORD CopyStyle
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueDefaultCopyW(
    _In_ HSPFILEQ QueueHandle,
    _In_ HINF InfHandle,
    _In_opt_ PCWSTR SourceRootPath,
    _In_ PCWSTR SourceFilename,
    _In_opt_ PCWSTR TargetFilename,
    _In_ DWORD CopyStyle
    );

#ifdef UNICODE
#define SetupQueueDefaultCopy SetupQueueDefaultCopyW
#else
#define SetupQueueDefaultCopy SetupQueueDefaultCopyA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueCopySectionA(
    _In_ HSPFILEQ QueueHandle,
    _In_opt_ PCSTR SourceRootPath,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCSTR Section,
    _In_ DWORD CopyStyle
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueCopySectionW(
    _In_ HSPFILEQ QueueHandle,
    _In_opt_ PCWSTR SourceRootPath,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCWSTR Section,
    _In_ DWORD CopyStyle
    );

#ifdef UNICODE
#define SetupQueueCopySection SetupQueueCopySectionW
#else
#define SetupQueueCopySection SetupQueueCopySectionA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueDeleteA(
    _In_ HSPFILEQ QueueHandle,
    _In_ PCSTR PathPart1,
    _In_opt_ PCSTR PathPart2
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueDeleteW(
    _In_ HSPFILEQ QueueHandle,
    _In_ PCWSTR PathPart1,
    _In_opt_ PCWSTR PathPart2
    );

#ifdef UNICODE
#define SetupQueueDelete SetupQueueDeleteW
#else
#define SetupQueueDelete SetupQueueDeleteA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueDeleteSectionA(
    _In_ HSPFILEQ QueueHandle,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCSTR Section
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueDeleteSectionW(
    _In_ HSPFILEQ QueueHandle,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCWSTR Section
    );

#ifdef UNICODE
#define SetupQueueDeleteSection SetupQueueDeleteSectionW
#else
#define SetupQueueDeleteSection SetupQueueDeleteSectionA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueRenameA(
    _In_ HSPFILEQ QueueHandle,
    _In_ PCSTR SourcePath,
    _In_opt_ PCSTR SourceFilename,
    _In_opt_ PCSTR TargetPath,
    _In_ PCSTR TargetFilename
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueRenameW(
    _In_ HSPFILEQ QueueHandle,
    _In_ PCWSTR SourcePath,
    _In_opt_ PCWSTR SourceFilename,
    _In_opt_ PCWSTR TargetPath,
    _In_ PCWSTR TargetFilename
    );

#ifdef UNICODE
#define SetupQueueRename SetupQueueRenameW
#else
#define SetupQueueRename SetupQueueRenameA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQueueRenameSectionA(
    _In_ HSPFILEQ QueueHandle,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCSTR Section
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueueRenameSectionW(
    _In_ HSPFILEQ QueueHandle,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCWSTR Section
    );

#ifdef UNICODE
#define SetupQueueRenameSection SetupQueueRenameSectionW
#else
#define SetupQueueRenameSection SetupQueueRenameSectionA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupCommitFileQueueA(
    _In_opt_ HWND Owner,
    _In_ HSPFILEQ QueueHandle,
    _In_ PSP_FILE_CALLBACK_A MsgHandler,
    _In_ PVOID Context
    );

WINSETUPAPI
BOOL
WINAPI
SetupCommitFileQueueW(
    _In_opt_ HWND Owner,
    _In_ HSPFILEQ QueueHandle,
    _In_ PSP_FILE_CALLBACK_W MsgHandler,
    _In_ PVOID Context
    );

#ifdef UNICODE
#define SetupCommitFileQueue SetupCommitFileQueueW
#else
#define SetupCommitFileQueue SetupCommitFileQueueA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupScanFileQueueA(
    _In_ HSPFILEQ FileQueue,
    _In_ DWORD Flags,
    _In_opt_ HWND Window,
    _In_opt_ PSP_FILE_CALLBACK_A CallbackRoutine,
    _In_opt_ PVOID CallbackContext,
    _Out_ PDWORD Result
    );

WINSETUPAPI
BOOL
WINAPI
SetupScanFileQueueW(
    _In_ HSPFILEQ FileQueue,
    _In_ DWORD Flags,
    _In_opt_ HWND Window,
    _In_opt_ PSP_FILE_CALLBACK_W CallbackRoutine,
    _In_opt_ PVOID CallbackContext,
    _Out_ PDWORD Result
    );

#ifdef UNICODE
#define SetupScanFileQueue SetupScanFileQueueW
#else
#define SetupScanFileQueue SetupScanFileQueueA
#endif

//
// Define flags for SetupScanFileQueue.
//
#define SPQ_SCAN_FILE_PRESENCE                  0x00000001
#define SPQ_SCAN_FILE_VALIDITY                  0x00000002
#define SPQ_SCAN_USE_CALLBACK                   0x00000004
#define SPQ_SCAN_USE_CALLBACKEX                 0x00000008
#define SPQ_SCAN_INFORM_USER                    0x00000010
#define SPQ_SCAN_PRUNE_COPY_QUEUE               0x00000020

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define SPQ_SCAN_USE_CALLBACK_SIGNERINFO        0x00000040
#define SPQ_SCAN_PRUNE_DELREN                   0x00000080 // remote Delete/Rename queue

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP


#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE   0x00000100
#define SPQ_SCAN_FILE_COMPARISON                0x00000200
#define SPQ_SCAN_ACTIVATE_DRP                   0x00000400

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN10

#define SPQ_SCAN_USE_OEM_CATALOGS               0x00000800

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WIN10

//
// Define flags used with Param2 for SPFILENOTIFY_QUEUESCAN
//
#define SPQ_DELAYED_COPY                        0x00000001  // file was in use; registered for delayed copy

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupGetFileQueueCount(
    _In_ HSPFILEQ FileQueue,
    _In_ UINT SubQueueFileOp,
    _Out_ PUINT NumOperations
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetFileQueueFlags(
    _In_ HSPFILEQ FileQueue,
    _Out_ PDWORD Flags
    );

WINSETUPAPI
BOOL
WINAPI
SetupSetFileQueueFlags(
    _In_ HSPFILEQ FileQueue,
    _In_ DWORD FlagMask,
    _In_ DWORD Flags
    );

//
// Flags/FlagMask for use with SetupSetFileQueueFlags and returned by SetupGetFileQueueFlags
//
#define SPQ_FLAG_BACKUP_AWARE      0x00000001  // If set, SetupCommitFileQueue will
                                               // issue backup notifications.

#define SPQ_FLAG_ABORT_IF_UNSIGNED 0x00000002  // If set, SetupCommitFileQueue will
                                               // fail with ERROR_SET_SYSTEM_RESTORE_POINT
                                               // if the user elects to proceed with an
                                               // unsigned queue committal.  This allows
                                               // the caller to set a system restore point,
                                               // then re-commit the file queue.

#define SPQ_FLAG_FILES_MODIFIED    0x00000004  // If set, at least one file was
                                               // replaced by a different version

#define SPQ_FLAG_DO_SHUFFLEMOVE    0x00000008  // If set then always do a shuffle move. A shuffle
                                               // move will first try to copy the source over the
                                               // destination file, but if the destination file is
                                               // in use it will rename the destination file to a
                                               // temp name and queue the temp name for deletion.
                                               // It will then be free to copy the source to the
                                               // destination name.  It is considered an error if
                                               // the destination file can't be renamed for some
                                               // reason.

#define SPQ_FLAG_VALID             0x0000000F  // mask of valid flags (can be passed as FlagMask)

#endif  // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Define OEM Source Type values for use in SetupCopyOEMInf.
//
#define SPOST_NONE  0
#define SPOST_PATH  1
#define SPOST_URL   2
#define SPOST_MAX   3

WINSETUPAPI
BOOL
WINAPI
SetupCopyOEMInfA(
    _In_ PCSTR SourceInfFileName,
    _In_opt_ PCSTR OEMSourceMediaLocation,
    _In_ DWORD OEMSourceMediaType,
    _In_ DWORD CopyStyle,
    _Out_writes_opt_(DestinationInfFileNameSize) PSTR DestinationInfFileName,
    _In_ DWORD DestinationInfFileNameSize,
    _Out_opt_ PDWORD RequiredSize,
    _Out_opt_ PSTR *DestinationInfFileNameComponent
    );

WINSETUPAPI
BOOL
WINAPI
SetupCopyOEMInfW(
    _In_ PCWSTR SourceInfFileName,
    _In_opt_ PCWSTR OEMSourceMediaLocation,
    _In_ DWORD OEMSourceMediaType,
    _In_ DWORD CopyStyle,
    _Out_writes_opt_(DestinationInfFileNameSize) PWSTR DestinationInfFileName,
    _In_ DWORD DestinationInfFileNameSize,
    _Out_opt_ PDWORD RequiredSize,
    _Out_opt_ PWSTR  *DestinationInfFileNameComponent
    );

#ifdef UNICODE
#define SetupCopyOEMInf SetupCopyOEMInfW
#else
#define SetupCopyOEMInf SetupCopyOEMInfA
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Flags used by SetupUninstallOEMInf
//
#define SUOI_FORCEDELETE   0x00000001

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define SUOI_INTERNAL1     0x00000002

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN


#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP
WINSETUPAPI
BOOL
WINAPI
SetupUninstallOEMInfA(
    _In_ PCSTR InfFileName,
    _In_ DWORD Flags,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupUninstallOEMInfW(
    _In_ PCWSTR InfFileName,
    _In_ DWORD Flags,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupUninstallOEMInf SetupUninstallOEMInfW
#else
#define SetupUninstallOEMInf SetupUninstallOEMInfA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupUninstallNewlyCopiedInfs(
    _In_ HSPFILEQ FileQueue,
    _In_ DWORD Flags,
    _Reserved_ PVOID Reserved
    );

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP


//
// Disk space list APIs
//
WINSETUPAPI
HDSKSPC
WINAPI
SetupCreateDiskSpaceListA(
    _Reserved_ PVOID Reserved1,
    _Reserved_ DWORD Reserved2,
    _In_ UINT Flags
    );

WINSETUPAPI
HDSKSPC
WINAPI
SetupCreateDiskSpaceListW(
    _Reserved_ PVOID Reserved1,
    _Reserved_ DWORD Reserved2,
    _In_ UINT Flags
    );

#ifdef UNICODE
#define SetupCreateDiskSpaceList SetupCreateDiskSpaceListW
#else
#define SetupCreateDiskSpaceList SetupCreateDiskSpaceListA
#endif

//
// Flags for SetupCreateDiskSpaceList
//
#define SPDSL_IGNORE_DISK              0x00000001  // ignore deletes and on-disk files in copies
#define SPDSL_DISALLOW_NEGATIVE_ADJUST 0x00000002


WINSETUPAPI
HDSKSPC
WINAPI
SetupDuplicateDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _Reserved_ PVOID Reserved1,
    _Reserved_ DWORD Reserved2,
    _In_ UINT Flags
    );

WINSETUPAPI
HDSKSPC
WINAPI
SetupDuplicateDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _Reserved_ PVOID Reserved1,
    _Reserved_ DWORD Reserved2,
    _In_ UINT Flags
    );

#ifdef UNICODE
#define SetupDuplicateDiskSpaceList SetupDuplicateDiskSpaceListW
#else
#define SetupDuplicateDiskSpaceList SetupDuplicateDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDestroyDiskSpaceList(
    _Inout_ HDSKSPC DiskSpace
    );


WINSETUPAPI
BOOL
WINAPI
SetupQueryDrivesInDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _Out_writes_opt_(ReturnBufferSize) PSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueryDrivesInDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _Out_writes_opt_(ReturnBufferSize) PWSTR ReturnBuffer,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupQueryDrivesInDiskSpaceList SetupQueryDrivesInDiskSpaceListW
#else
#define SetupQueryDrivesInDiskSpaceList SetupQueryDrivesInDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupQuerySpaceRequiredOnDriveA(
    _In_ HDSKSPC DiskSpace,
    _In_ PCSTR DriveSpec,
    _Out_ LONGLONG *SpaceRequired,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupQuerySpaceRequiredOnDriveW(
    _In_ HDSKSPC DiskSpace,
    _In_ PCWSTR DriveSpec,
    _Out_ LONGLONG *SpaceRequired,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupQuerySpaceRequiredOnDrive SetupQuerySpaceRequiredOnDriveW
#else
#define SetupQuerySpaceRequiredOnDrive SetupQuerySpaceRequiredOnDriveA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupAdjustDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ LPCSTR DriveRoot,
    _In_ LONGLONG Amount,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupAdjustDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ LPCWSTR DriveRoot,
    _In_ LONGLONG Amount,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupAdjustDiskSpaceList SetupAdjustDiskSpaceListW
#else
#define SetupAdjustDiskSpaceList SetupAdjustDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupAddToDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ PCSTR TargetFilespec,
    _In_ LONGLONG FileSize,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupAddToDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ PCWSTR TargetFilespec,
    _In_ LONGLONG FileSize,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupAddToDiskSpaceList SetupAddToDiskSpaceListW
#else
#define SetupAddToDiskSpaceList SetupAddToDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupAddSectionToDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCSTR SectionName,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupAddSectionToDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCWSTR SectionName,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupAddSectionToDiskSpaceList SetupAddSectionToDiskSpaceListW
#else
#define SetupAddSectionToDiskSpaceList SetupAddSectionToDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupAddInstallSectionToDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF LayoutInfHandle,
    _In_ PCSTR SectionName,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupAddInstallSectionToDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF LayoutInfHandle,
    _In_ PCWSTR SectionName,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupAddInstallSectionToDiskSpaceList SetupAddInstallSectionToDiskSpaceListW
#else
#define SetupAddInstallSectionToDiskSpaceList SetupAddInstallSectionToDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupRemoveFromDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ PCSTR TargetFilespec,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupRemoveFromDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ PCWSTR TargetFilespec,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupRemoveFromDiskSpaceList SetupRemoveFromDiskSpaceListW
#else
#define SetupRemoveFromDiskSpaceList SetupRemoveFromDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupRemoveSectionFromDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCSTR SectionName,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupRemoveSectionFromDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF ListInfHandle,
    _In_ PCWSTR SectionName,
    _In_ UINT Operation,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupRemoveSectionFromDiskSpaceList SetupRemoveSectionFromDiskSpaceListW
#else
#define SetupRemoveSectionFromDiskSpaceList SetupRemoveSectionFromDiskSpaceListA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupRemoveInstallSectionFromDiskSpaceListA(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF LayoutInfHandle,
    _In_ PCSTR SectionName,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupRemoveInstallSectionFromDiskSpaceListW(
    _In_ HDSKSPC DiskSpace,
    _In_ HINF InfHandle,
    _In_opt_ HINF LayoutInfHandle,
    _In_ PCWSTR SectionName,
    _Reserved_ PVOID Reserved1,
    _Reserved_ UINT Reserved2
    );

#ifdef UNICODE
#define SetupRemoveInstallSectionFromDiskSpaceList SetupRemoveInstallSectionFromDiskSpaceListW
#else
#define SetupRemoveInstallSectionFromDiskSpaceList SetupRemoveInstallSectionFromDiskSpaceListA
#endif


//
// Cabinet APIs
//

WINSETUPAPI
BOOL
WINAPI
SetupIterateCabinetA(
    _In_ PCSTR CabinetFile,
    _Reserved_ DWORD Reserved,
    _In_ PSP_FILE_CALLBACK_A MsgHandler,
    _In_ PVOID Context
    );

WINSETUPAPI
BOOL
WINAPI
SetupIterateCabinetW(
    _In_ PCWSTR CabinetFile,
    _Reserved_ DWORD Reserved,
    _In_ PSP_FILE_CALLBACK_W MsgHandler,
    _In_ PVOID Context
    );

#ifdef UNICODE
#define SetupIterateCabinet SetupIterateCabinetW
#else
#define SetupIterateCabinet SetupIterateCabinetA
#endif


WINSETUPAPI
INT
WINAPI
SetupPromptReboot(
    _In_opt_ HSPFILEQ FileQueue,
    _In_opt_ HWND Owner,
    _In_ BOOL ScanOnly
    );

//
// Define flags that are returned by SetupPromptReboot
//
#define SPFILEQ_FILE_IN_USE         0x00000001
#define SPFILEQ_REBOOT_RECOMMENDED  0x00000002
#define SPFILEQ_REBOOT_IN_PROGRESS  0x00000004


WINSETUPAPI
PVOID
WINAPI
SetupInitDefaultQueueCallback(
    _In_opt_ HWND OwnerWindow
    );

WINSETUPAPI
PVOID
WINAPI
SetupInitDefaultQueueCallbackEx(
    _In_opt_ HWND OwnerWindow,
    _In_opt_ HWND AlternateProgressWindow,
    _In_ UINT ProgressMessage,
    _Reserved_ DWORD Reserved1,
    _Reserved_ PVOID Reserved2
    );

WINSETUPAPI
VOID
WINAPI
SetupTermDefaultQueueCallback(
    _In_ PVOID Context
    );

WINSETUPAPI
UINT
WINAPI
SetupDefaultQueueCallbackA(
    _In_ PVOID Context,
    _In_ UINT Notification,
    _In_ UINT_PTR Param1,
    _In_ UINT_PTR Param2
    );

WINSETUPAPI
UINT
WINAPI
SetupDefaultQueueCallbackW(
    _In_ PVOID Context,
    _In_ UINT Notification,
    _In_ UINT_PTR Param1,
    _In_ UINT_PTR Param2
    );

#ifdef UNICODE
#define SetupDefaultQueueCallback SetupDefaultQueueCallbackW
#else
#define SetupDefaultQueueCallback SetupDefaultQueueCallbackA
#endif


//
// Flags for AddReg section lines in INF.  The corresponding value
// is <ValueType> in the AddReg line format given below:
//
// <RegRootString>,<SubKey>,<ValueName>,<ValueType>,<Value>...
//
// The low word contains basic flags concerning the general data type
// and AddReg action. The high word contains values that more specifically
// identify the data type of the registry value.  The high word is ignored
// by the 16-bit Windows 95 SETUPX APIs.
//
// If <ValueType> has FLG_ADDREG_DELREG_BIT set, it will be ignored by AddReg
// (not supported by SetupX).
//

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define FLG_ADDREG_DELREG_BIT       ( 0x00008000 ) // if set, interpret as DELREG, see below

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define FLG_ADDREG_BINVALUETYPE     ( 0x00000001 )
#define FLG_ADDREG_NOCLOBBER        ( 0x00000002 )
#define FLG_ADDREG_DELVAL           ( 0x00000004 )
#define FLG_ADDREG_APPEND           ( 0x00000008 ) // Currently supported only
                                                   // for REG_MULTI_SZ values.
#define FLG_ADDREG_KEYONLY          ( 0x00000010 ) // Just create the key, ignore value
#define FLG_ADDREG_OVERWRITEONLY    ( 0x00000020 ) // Set only if value already exists

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define FLG_ADDREG_64BITKEY         ( 0x00001000 ) // make this change in the 64 bit registry.
#define FLG_ADDREG_KEYONLY_COMMON   ( 0x00002000 ) // same as FLG_ADDREG_KEYONLY but also works for DELREG
#define FLG_ADDREG_32BITKEY         ( 0x00004000 ) // make this change in the 32 bit registry.

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// The INF may supply any arbitrary data type ordinal in the highword except
// for the following: REG_NONE, REG_SZ, REG_EXPAND_SZ, REG_MULTI_SZ.  If this
// technique is used, then the data is given in binary format, one byte per
// field.
//
#define FLG_ADDREG_TYPE_MASK        ( 0xFFFF0000 | FLG_ADDREG_BINVALUETYPE )
#define FLG_ADDREG_TYPE_SZ          ( 0x00000000                           )
#define FLG_ADDREG_TYPE_MULTI_SZ    ( 0x00010000                           )
#define FLG_ADDREG_TYPE_EXPAND_SZ   ( 0x00020000                           )
#define FLG_ADDREG_TYPE_BINARY      ( 0x00000000 | FLG_ADDREG_BINVALUETYPE )
#define FLG_ADDREG_TYPE_DWORD       ( 0x00010000 | FLG_ADDREG_BINVALUETYPE )
#define FLG_ADDREG_TYPE_NONE        ( 0x00020000 | FLG_ADDREG_BINVALUETYPE )

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN10

#define FLG_ADDREG_TYPE_QWORD       ( 0x000B0000 | FLG_ADDREG_BINVALUETYPE )

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WIN10

//
// Flags for DelReg section lines in INF.  The corresponding value
// is <Operation> in the extended DelReg line format given below:
//
// <RegRootString>,<SubKey>,<ValueName>,<Operation>[,...]
//
// In SetupX and some versions of SetupAPI, <Operation> will be ignored and <ValueName> will
// be deleted. Use with care.
//
// The bits determined by mask FLG_DELREG_TYPE_MASK indicates type of data expected.
// <Operation> must have FLG_ADDREG_DELREG_BIT set, otherwise it is ignored and specified
// value will be deleted (allowing an AddReg section to also be used as a DelReg section)
// if <Operation> is not specified, <ValueName> will be deleted (if specified) otherwise
// <SubKey> will be deleted.
//
// the compatability flag
//
#define FLG_DELREG_VALUE            (0x00000000)

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define FLG_DELREG_TYPE_MASK        FLG_ADDREG_TYPE_MASK        // 0xFFFF0001
#define FLG_DELREG_TYPE_SZ          FLG_ADDREG_TYPE_SZ          // 0x00000000
#define FLG_DELREG_TYPE_MULTI_SZ    FLG_ADDREG_TYPE_MULTI_SZ    // 0x00010000
#define FLG_DELREG_TYPE_EXPAND_SZ   FLG_ADDREG_TYPE_EXPAND_SZ   // 0x00020000
#define FLG_DELREG_TYPE_BINARY      FLG_ADDREG_TYPE_BINARY      // 0x00000001
#define FLG_DELREG_TYPE_DWORD       FLG_ADDREG_TYPE_DWORD       // 0x00010001
#define FLG_DELREG_TYPE_NONE        FLG_ADDREG_TYPE_NONE        // 0x00020001
#define FLG_DELREG_64BITKEY         FLG_ADDREG_64BITKEY         // 0x00001000
#define FLG_DELREG_KEYONLY_COMMON   FLG_ADDREG_KEYONLY_COMMON   // 0x00002000
#define FLG_DELREG_32BITKEY         FLG_ADDREG_32BITKEY         // 0x00004000

//
// <Operation> = FLG_DELREG_MULTI_SZ_DELSTRING
//               <RegRootString>,<SubKey>,<ValueName>,0x00018002,<String>
//               removes all entries matching <String> (case ignored) from multi-sz registry value
//

#define FLG_DELREG_OPERATION_MASK   (0x000000FE)
#define FLG_DELREG_MULTI_SZ_DELSTRING ( FLG_DELREG_TYPE_MULTI_SZ | FLG_ADDREG_DELREG_BIT | 0x00000002 ) // 0x00018002

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN10

#define FLG_DELREG_TYPE_QWORD       FLG_ADDREG_TYPE_QWORD       // 0x000B0001

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WIN10

//
// Flags for BitReg section lines in INF.
//
#define FLG_BITREG_CLEARBITS        ( 0x00000000 )
#define FLG_BITREG_SETBITS          ( 0x00000001 )

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define FLG_BITREG_64BITKEY         ( 0x00001000 )
#define FLG_BITREG_32BITKEY         ( 0x00004000 )

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Flags for Ini2Reg section lines in INF.
//
#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define FLG_INI2REG_64BITKEY        ( 0x00001000 )
#define FLG_INI2REG_32BITKEY        ( 0x00004000 )

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Flags for RegSvr section lines in INF
//
#define FLG_REGSVR_DLLREGISTER      ( 0x00000001 )
#define FLG_REGSVR_DLLINSTALL       ( 0x00000002 )

// Flags for RegSvr section lines in INF
//

#define FLG_PROFITEM_CURRENTUSER    ( 0x00000001 )
#define FLG_PROFITEM_DELETE         ( 0x00000002 )
#define FLG_PROFITEM_GROUP          ( 0x00000004 )
#define FLG_PROFITEM_CSIDL          ( 0x00000008 )

//
// Flags for AddProperty section lines in the INF
//

#define FLG_ADDPROPERTY_NOCLOBBER       ( 0x00000001 )
#define FLG_ADDPROPERTY_OVERWRITEONLY   ( 0x00000002 )
#define FLG_ADDPROPERTY_APPEND          ( 0x00000004 )
#define FLG_ADDPROPERTY_OR              ( 0x00000008 )
#define FLG_ADDPROPERTY_AND             ( 0x00000010 )

//
// Flags for DelProperty section lines in the INF
//

#define FLG_DELPROPERTY_MULTI_SZ_DELSTRING  ( 0x00000001 )


WINSETUPAPI
BOOL
WINAPI
SetupInstallFromInfSectionA(
    _In_opt_ HWND Owner,
    _In_ HINF InfHandle,
    _In_ PCSTR SectionName,
    _In_ UINT Flags,
    _In_opt_ HKEY RelativeKeyRoot,
    _In_opt_ PCSTR SourceRootPath,
    _In_ UINT CopyFlags,
    _In_opt_ PSP_FILE_CALLBACK_A MsgHandler,
    _In_opt_ PVOID Context,
    _In_opt_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

WINSETUPAPI
BOOL
WINAPI
SetupInstallFromInfSectionW(
    _In_opt_ HWND Owner,
    _In_ HINF InfHandle,
    _In_ PCWSTR SectionName,
    _In_ UINT Flags,
    _In_opt_ HKEY RelativeKeyRoot,
    _In_opt_ PCWSTR SourceRootPath,
    _In_ UINT CopyFlags,
    _In_opt_ PSP_FILE_CALLBACK_W MsgHandler,
    _In_opt_ PVOID Context,
    _In_opt_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

#ifdef UNICODE
#define SetupInstallFromInfSection SetupInstallFromInfSectionW
#else
#define SetupInstallFromInfSection SetupInstallFromInfSectionA
#endif

//
// Flags for SetupInstallFromInfSection
//
#define SPINST_LOGCONFIG                0x00000001
#define SPINST_INIFILES                 0x00000002
#define SPINST_REGISTRY                 0x00000004
#define SPINST_INI2REG                  0x00000008
#define SPINST_FILES                    0x00000010
#define SPINST_BITREG                   0x00000020
#define SPINST_REGSVR                   0x00000040
#define SPINST_UNREGSVR                 0x00000080
#define SPINST_PROFILEITEMS             0x00000100

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define SPINST_COPYINF                  0x00000200

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define SPINST_PROPERTIES               0x00000400
#define SPINST_ALL                      0x000007ff

#else

#define SPINST_ALL                      0x000003ff

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#else

#define SPINST_ALL                      0x000001ff

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define SPINST_SINGLESECTION            0x00010000
#define SPINST_LOGCONFIG_IS_FORCED      0x00020000
#define SPINST_LOGCONFIGS_ARE_OVERRIDES 0x00040000

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define SPINST_REGISTERCALLBACKAWARE    0x00080000

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#define SPINST_DEVICEINSTALL            0x00100000

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

WINSETUPAPI
BOOL
WINAPI
SetupInstallFilesFromInfSectionA(
    _In_ HINF InfHandle,
    _In_opt_ HINF LayoutInfHandle,
    _In_ HSPFILEQ FileQueue,
    _In_ PCSTR SectionName,
    _In_opt_ PCSTR SourceRootPath,
    _In_ UINT CopyFlags
    );

WINSETUPAPI
BOOL
WINAPI
SetupInstallFilesFromInfSectionW(
    _In_ HINF InfHandle,
    _In_opt_ HINF LayoutInfHandle,
    _In_ HSPFILEQ FileQueue,
    _In_ PCWSTR SectionName,
    _In_opt_ PCWSTR SourceRootPath,
    _In_ UINT CopyFlags
    );

#ifdef UNICODE
#define SetupInstallFilesFromInfSection SetupInstallFilesFromInfSectionW
#else
#define SetupInstallFilesFromInfSection SetupInstallFilesFromInfSectionA
#endif


//
// Flags for SetupInstallServicesFromInfSection(Ex).  These flags are also used
// in the flags field of AddService or DelService lines in a device INF.  Some
// of these flags are not permitted in the non-Ex API.  These flags are marked
// as such below.
//

//
// (AddService) move service's tag to front of its group order list
//
#define SPSVCINST_TAGTOFRONT                   (0x00000001)

//
// (AddService) **Ex API only** mark this service as the function driver for the
// device being installed
//
#define SPSVCINST_ASSOCSERVICE                 (0x00000002)

//
// (DelService) delete the associated event log entry for a service specified in
// a DelService entry
//
#define SPSVCINST_DELETEEVENTLOGENTRY          (0x00000004)

//
// (AddService) don't overwrite display name if it already exists
//
#define SPSVCINST_NOCLOBBER_DISPLAYNAME        (0x00000008)

//
// (AddService) don't overwrite start type value if service already exists
//
#define SPSVCINST_NOCLOBBER_STARTTYPE          (0x00000010)

//
// (AddService) don't overwrite error control value if service already exists
//
#define SPSVCINST_NOCLOBBER_ERRORCONTROL       (0x00000020)

//
// (AddService) don't overwrite load order group if it already exists
//
#define SPSVCINST_NOCLOBBER_LOADORDERGROUP     (0x00000040)

//
// (AddService) don't overwrite dependencies list if it already exists
//
#define SPSVCINST_NOCLOBBER_DEPENDENCIES       (0x00000080)

//
// (AddService) don't overwrite description if it already exists
//
#define SPSVCINST_NOCLOBBER_DESCRIPTION        (0x00000100)
//
// (DelService) stop the associated service specified in
// a DelService entry before deleting the service
//
#define SPSVCINST_STOPSERVICE                  (0x00000200)

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP
//
// (AddService) force overwrite of security settings
//
#define SPSVCINST_CLOBBER_SECURITY             (0x00000400)

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN
//
// (Start Service) start a service manually after install
//
#define SPSVCINST_STARTSERVICE                 (0x00000800)

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN7
//
// (AddService) don't overwrite required privileges list if it already exists
//
#define SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES (0x00001000)

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WIN7

#if _SETUPAPI_VER >= _WIN32_WINNT_WIN10
//
// (AddService) don't overwrite triggers if they already exist
//
#define SPSVCINST_NOCLOBBER_TRIGGERS (0x00002000)

//
// (AddService) don't overwrite service SID type if it already exists
//
#define SPSVCINST_NOCLOBBER_SERVICESIDTYPE (0x00004000)

//
// (AddService) don't overwrite delayed auto start if it already exists
//
#define SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART (0x00008000)

//
// (AddService) create service with a unique name
//
#define SPSVCINST_UNIQUE_NAME (0x00010000)

//
// (AddService) don't overwrite failure actions if they already exist
//
#define SPSVCINST_NOCLOBBER_FAILUREACTIONS (0x00020000)

//
// (AddService) don't overwrite boot flags if they already exist
//
#define SPSVCINST_NOCLOBBER_BOOTFLAGS (0x00040000)

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WIN10

WINSETUPAPI
BOOL
WINAPI
SetupInstallServicesFromInfSectionA(
    _In_ HINF InfHandle,
    _In_ PCSTR SectionName,
    _In_ DWORD Flags
    );

WINSETUPAPI
BOOL
WINAPI
SetupInstallServicesFromInfSectionW(
    _In_ HINF InfHandle,
    _In_ PCWSTR SectionName,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define SetupInstallServicesFromInfSection SetupInstallServicesFromInfSectionW
#else
#define SetupInstallServicesFromInfSection SetupInstallServicesFromInfSectionA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupInstallServicesFromInfSectionExA(
    _In_ HINF InfHandle,
    _In_ PCSTR SectionName,
    _In_ DWORD Flags,
    _In_opt_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Reserved_ PVOID Reserved1,
    _Reserved_ PVOID Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupInstallServicesFromInfSectionExW(
    _In_ HINF InfHandle,
    _In_ PCWSTR SectionName,
    _In_ DWORD Flags,
    _In_opt_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Reserved_ PVOID Reserved1,
    _Reserved_ PVOID Reserved2
    );

#ifdef UNICODE
#define SetupInstallServicesFromInfSectionEx SetupInstallServicesFromInfSectionExW
#else
#define SetupInstallServicesFromInfSectionEx SetupInstallServicesFromInfSectionExA
#endif



//
// High level routine, usually used via rundll32.dll
// to perform right-click install action on INFs
// May be called directly:
//
// wsprintf(CmdLineBuffer,TEXT("DefaultInstall 132 %s"),InfPath);
// InstallHinfSection(NULL,NULL,CmdLineBuffer,0);
//
VOID
WINAPI
InstallHinfSectionA(
    _In_ HWND Window,
    _In_ HINSTANCE ModuleHandle,
    _In_ PCSTR CommandLine,
    _In_ INT ShowCommand
    );

VOID
WINAPI
InstallHinfSectionW(
    _In_ HWND Window,
    _In_ HINSTANCE ModuleHandle,
    _In_ PCWSTR CommandLine,
    _In_ INT ShowCommand
    );

#ifdef UNICODE
#define InstallHinfSection InstallHinfSectionW
#else
#define InstallHinfSection InstallHinfSectionA
#endif





//
// Define handle type for Setup file log.
//
typedef PVOID HSPFILELOG;

WINSETUPAPI
HSPFILELOG
WINAPI
SetupInitializeFileLogA(
    _In_opt_ PCSTR LogFileName,
    _In_ DWORD Flags
    );

WINSETUPAPI
HSPFILELOG
WINAPI
SetupInitializeFileLogW(
    _In_opt_ PCWSTR LogFileName,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define SetupInitializeFileLog SetupInitializeFileLogW
#else
#define SetupInitializeFileLog SetupInitializeFileLogA
#endif

//
// Flags for SetupInitializeFileLog
//
#define SPFILELOG_SYSTEMLOG     0x00000001  // use system log -- must be Administrator
#define SPFILELOG_FORCENEW      0x00000002  // not valid with SPFILELOG_SYSTEMLOG
#define SPFILELOG_QUERYONLY     0x00000004  // allows non-administrators to read system log


WINSETUPAPI
BOOL
WINAPI
SetupTerminateFileLog(
    _In_ HSPFILELOG FileLogHandle
    );


WINSETUPAPI
BOOL
WINAPI
SetupLogFileA(
    _In_ HSPFILELOG FileLogHandle,
    _In_opt_ PCSTR LogSectionName,
    _In_ PCSTR SourceFilename,
    _In_ PCSTR TargetFilename,
    _In_ DWORD Checksum,
    _In_opt_ PCSTR DiskTagfile,
    _In_opt_ PCSTR DiskDescription,
    _In_opt_ PCSTR OtherInfo,
    _In_ DWORD Flags
    );

WINSETUPAPI
BOOL
WINAPI
SetupLogFileW(
    _In_ HSPFILELOG FileLogHandle,
    _In_opt_ PCWSTR LogSectionName,
    _In_ PCWSTR SourceFilename,
    _In_ PCWSTR TargetFilename,
    _In_ DWORD Checksum,
    _In_opt_ PCWSTR DiskTagfile,
    _In_opt_ PCWSTR DiskDescription,
    _In_opt_ PCWSTR OtherInfo,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define SetupLogFile SetupLogFileW
#else
#define SetupLogFile SetupLogFileA
#endif

//
// Flags for SetupLogFile
//
#define SPFILELOG_OEMFILE   0x00000001


WINSETUPAPI
BOOL
WINAPI
SetupRemoveFileLogEntryA(
    _In_ HSPFILELOG FileLogHandle,
    _In_opt_ PCSTR LogSectionName,
    _In_opt_ PCSTR TargetFilename
    );

WINSETUPAPI
BOOL
WINAPI
SetupRemoveFileLogEntryW(
    _In_ HSPFILELOG FileLogHandle,
    _In_opt_ PCWSTR LogSectionName,
    _In_opt_ PCWSTR TargetFilename
    );

#ifdef UNICODE
#define SetupRemoveFileLogEntry SetupRemoveFileLogEntryW
#else
#define SetupRemoveFileLogEntry SetupRemoveFileLogEntryA
#endif


//
// Items retrievable from SetupQueryFileLog()
//
typedef enum {
    SetupFileLogSourceFilename,
    SetupFileLogChecksum,
    SetupFileLogDiskTagfile,
    SetupFileLogDiskDescription,
    SetupFileLogOtherInfo,
    SetupFileLogMax
} SetupFileLogInfo;

WINSETUPAPI
BOOL
WINAPI
SetupQueryFileLogA(
    _In_ HSPFILELOG FileLogHandle,
    _In_opt_ PCSTR LogSectionName,
    _In_ PCSTR TargetFilename,
    _In_ SetupFileLogInfo DesiredInfo,
    _Out_writes_opt_(ReturnBufferSize) PSTR DataOut,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupQueryFileLogW(
    _In_ HSPFILELOG FileLogHandle,
    _In_opt_ PCWSTR LogSectionName,
    _In_ PCWSTR TargetFilename,
    _In_ SetupFileLogInfo DesiredInfo,
    _Out_writes_opt_(ReturnBufferSize) PWSTR DataOut,
    _In_ DWORD ReturnBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupQueryFileLog SetupQueryFileLogW
#else
#define SetupQueryFileLog SetupQueryFileLogA
#endif

//
// Text logging APIs
//
#define LogSeverity                 DWORD
#define LogSevInformation           0x00000000
#define LogSevWarning               0x00000001
#define LogSevError                 0x00000002
#define LogSevFatalError            0x00000003
#define LogSevMaximum               0x00000004

WINSETUPAPI
BOOL
WINAPI
SetupOpenLog (
    _In_ BOOL Erase
    );

WINSETUPAPI
BOOL
WINAPI
SetupLogErrorA (
    _In_ LPCSTR MessageString,
    _In_ LogSeverity Severity
    );

WINSETUPAPI
BOOL
WINAPI
SetupLogErrorW (
    _In_ LPCWSTR MessageString,
    _In_ LogSeverity Severity
    );

#ifdef UNICODE
#define SetupLogError SetupLogErrorW
#else
#define SetupLogError SetupLogErrorA
#endif

WINSETUPAPI
VOID
WINAPI
SetupCloseLog (
    VOID
    );

//
// Text log for INF debugging
//

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN


WINSETUPAPI
SP_LOG_TOKEN
WINAPI
SetupGetThreadLogToken(
    VOID
    );

WINSETUPAPI
VOID
WINAPI
SetupSetThreadLogToken(
    _In_ SP_LOG_TOKEN LogToken
    );

WINSETUPAPI
VOID
WINAPI
SetupWriteTextLog(
    _In_ SP_LOG_TOKEN LogToken,
    _In_ DWORD Category,
    _In_ DWORD Flags,
    _In_ PCSTR MessageStr,
    ...
    );

WINSETUPAPI
VOID
WINAPI
SetupWriteTextLogError(
    _In_ SP_LOG_TOKEN LogToken,
    _In_ DWORD Category,
    _In_ DWORD LogFlags,
    _In_ DWORD Error,
    _In_ PCSTR MessageStr,
    ...
    );

WINSETUPAPI
VOID
WINAPI
SetupWriteTextLogInfLine(
    _In_ SP_LOG_TOKEN LogToken,
    _In_ DWORD Flags,
    _In_ HINF InfHandle,
    _In_ PINFCONTEXT Context
    );

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN


//
// Backup Information API's
//

WINSETUPAPI
BOOL
WINAPI
SetupGetBackupInformationA(
    _In_ HSPFILEQ QueueHandle,
    _Inout_ PSP_BACKUP_QUEUE_PARAMS_A BackupParams
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetBackupInformationW(
    _In_ HSPFILEQ QueueHandle,
    _Inout_ PSP_BACKUP_QUEUE_PARAMS_W BackupParams
    );

#ifdef UNICODE
#define SetupGetBackupInformation SetupGetBackupInformationW
#else
#define SetupGetBackupInformation SetupGetBackupInformationA
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupPrepareQueueForRestoreA(
    _In_ HSPFILEQ QueueHandle,
    _In_ PCSTR BackupPath,
    _In_ DWORD RestoreFlags
    );

WINSETUPAPI
BOOL
WINAPI
SetupPrepareQueueForRestoreW(
    _In_ HSPFILEQ QueueHandle,
    _In_ PCWSTR BackupPath,
    _In_ DWORD RestoreFlags
    );

#ifdef UNICODE
#define SetupPrepareQueueForRestore SetupPrepareQueueForRestoreW
#else
#define SetupPrepareQueueForRestore SetupPrepareQueueForRestoreA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Control forcing of Non-Interactive Mode
// Overriden if SetupAPI is run in non-interactive window session
//

WINSETUPAPI
BOOL
WINAPI
SetupSetNonInteractiveMode(
    _In_ BOOL NonInteractiveFlag
    );

WINSETUPAPI
BOOL
WINAPI
SetupGetNonInteractiveMode(
    VOID
    );

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Device Installer APIs
//

_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiCreateDeviceInfoList(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ HWND hwndParent
    );


_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiCreateDeviceInfoListExA(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ HWND hwndParent,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiCreateDeviceInfoListExW(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ HWND hwndParent,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiCreateDeviceInfoListEx SetupDiCreateDeviceInfoListExW
#else
#define SetupDiCreateDeviceInfoListEx SetupDiCreateDeviceInfoListExA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInfoListClass(
    _In_ HDEVINFO DeviceInfoSet,
    _Out_ LPGUID ClassGuid
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInfoListDetailA(
    _In_ HDEVINFO DeviceInfoSet,
    _Out_ PSP_DEVINFO_LIST_DETAIL_DATA_A DeviceInfoSetDetailData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInfoListDetailW(
    _In_ HDEVINFO DeviceInfoSet,
    _Out_ PSP_DEVINFO_LIST_DETAIL_DATA_W DeviceInfoSetDetailData
    );

#ifdef UNICODE
#define SetupDiGetDeviceInfoListDetail SetupDiGetDeviceInfoListDetailW
#else
#define SetupDiGetDeviceInfoListDetail SetupDiGetDeviceInfoListDetailA
#endif


//
// Flags for SetupDiCreateDeviceInfo
//
#define DICD_GENERATE_ID        0x00000001
#define DICD_INHERIT_CLASSDRVS  0x00000002

WINSETUPAPI
BOOL
WINAPI
SetupDiCreateDeviceInfoA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PCSTR DeviceName,
    _In_ CONST GUID *ClassGuid,
    _In_opt_ PCSTR DeviceDescription,
    _In_opt_ HWND hwndParent,
    _In_ DWORD CreationFlags,
    _Out_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiCreateDeviceInfoW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PCWSTR DeviceName,
    _In_ CONST GUID *ClassGuid,
    _In_opt_ PCWSTR DeviceDescription,
    _In_opt_ HWND hwndParent,
    _In_ DWORD CreationFlags,
    _Out_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

#ifdef UNICODE
#define SetupDiCreateDeviceInfo SetupDiCreateDeviceInfoW
#else
#define SetupDiCreateDeviceInfo SetupDiCreateDeviceInfoA
#endif


//
// Flags for SetupDiOpenDeviceInfo
//
#define DIOD_INHERIT_CLASSDRVS  0x00000002
#define DIOD_CANCEL_REMOVE      0x00000004

WINSETUPAPI
BOOL
WINAPI
SetupDiOpenDeviceInfoA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PCSTR DeviceInstanceId,
    _In_opt_ HWND hwndParent,
    _In_ DWORD OpenFlags,
    _Out_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiOpenDeviceInfoW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PCWSTR DeviceInstanceId,
    _In_opt_ HWND hwndParent,
    _In_ DWORD OpenFlags,
    _Out_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

#ifdef UNICODE
#define SetupDiOpenDeviceInfo SetupDiOpenDeviceInfoW
#else
#define SetupDiOpenDeviceInfo SetupDiOpenDeviceInfoA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInstanceIdA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_writes_opt_(DeviceInstanceIdSize) PSTR DeviceInstanceId,
    _In_ DWORD DeviceInstanceIdSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInstanceIdW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_writes_opt_(DeviceInstanceIdSize) PWSTR DeviceInstanceId,
    _In_ DWORD DeviceInstanceIdSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetDeviceInstanceId SetupDiGetDeviceInstanceIdW
#else
#define SetupDiGetDeviceInstanceId SetupDiGetDeviceInstanceIdA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiDeleteDeviceInfo(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiEnumDeviceInfo(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ DWORD MemberIndex,
    _Out_ PSP_DEVINFO_DATA DeviceInfoData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiDestroyDeviceInfoList(
    _In_ HDEVINFO DeviceInfoSet
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiEnumDeviceInterfaces(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ CONST GUID *InterfaceClassGuid,
    _In_ DWORD MemberIndex,
    _Out_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

//
// Backward compatibility--do not use
//
#define SetupDiEnumInterfaceDevice SetupDiEnumDeviceInterfaces


WINSETUPAPI
BOOL
WINAPI
SetupDiCreateDeviceInterfaceA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ CONST GUID *InterfaceClassGuid,
    _In_opt_ PCSTR ReferenceString,
    _In_ DWORD CreationFlags,
    _Out_opt_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiCreateDeviceInterfaceW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ CONST GUID *InterfaceClassGuid,
    _In_opt_ PCWSTR ReferenceString,
    _In_ DWORD CreationFlags,
    _Out_opt_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

#ifdef UNICODE
#define SetupDiCreateDeviceInterface SetupDiCreateDeviceInterfaceW
#else
#define SetupDiCreateDeviceInterface SetupDiCreateDeviceInterfaceA
#endif

//
// Backward compatibility--do not use.
//
#define SetupDiCreateInterfaceDeviceW SetupDiCreateDeviceInterfaceW
#define SetupDiCreateInterfaceDeviceA SetupDiCreateDeviceInterfaceA
#ifdef UNICODE
#define SetupDiCreateInterfaceDevice SetupDiCreateDeviceInterfaceW
#else
#define SetupDiCreateInterfaceDevice SetupDiCreateDeviceInterfaceA
#endif


//
// Flags for SetupDiOpenDeviceInterface
//
#define DIODI_NO_ADD    0x00000001

WINSETUPAPI
BOOL
WINAPI
SetupDiOpenDeviceInterfaceA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PCSTR DevicePath,
    _In_ DWORD OpenFlags,
    _Out_opt_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiOpenDeviceInterfaceW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PCWSTR DevicePath,
    _In_ DWORD OpenFlags,
    _Out_opt_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

#ifdef UNICODE
#define SetupDiOpenDeviceInterface SetupDiOpenDeviceInterfaceW
#else
#define SetupDiOpenDeviceInterface SetupDiOpenDeviceInterfaceA
#endif

//
// Backward compatibility--do not use
//
#define SetupDiOpenInterfaceDeviceW SetupDiOpenDeviceInterfaceW
#define SetupDiOpenInterfaceDeviceA SetupDiOpenDeviceInterfaceA
#ifdef UNICODE
#define SetupDiOpenInterfaceDevice SetupDiOpenDeviceInterfaceW
#else
#define SetupDiOpenInterfaceDevice SetupDiOpenDeviceInterfaceA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInterfaceAlias(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _In_ CONST GUID *AliasInterfaceClassGuid,
    _Out_ PSP_DEVICE_INTERFACE_DATA AliasDeviceInterfaceData
    );

//
// Backward compatibility--do not use.
//
#define SetupDiGetInterfaceDeviceAlias SetupDiGetDeviceInterfaceAlias


WINSETUPAPI
BOOL
WINAPI
SetupDiDeleteDeviceInterfaceData(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

//
// Backward compatibility--do not use.
//
#define SetupDiDeleteInterfaceDeviceData SetupDiDeleteDeviceInterfaceData


WINSETUPAPI
BOOL
WINAPI
SetupDiRemoveDeviceInterface(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData
    );

//
// Backward compatibility--do not use.
//
#define SetupDiRemoveInterfaceDevice SetupDiRemoveDeviceInterface

_Success_(return != FALSE)
_At_((LPSTR)DeviceInterfaceDetailData->DevicePath, _Post_z_)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInterfaceDetailA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Inout_updates_bytes_to_opt_(DeviceInterfaceDetailDataSize, *RequiredSize) PSP_DEVICE_INTERFACE_DETAIL_DATA_A DeviceInterfaceDetailData,
    _In_ DWORD DeviceInterfaceDetailDataSize,
    _Out_opt_ _Out_range_(>=, sizeof(SP_DEVICE_INTERFACE_DETAIL_DATA_A)) PDWORD RequiredSize,
    _Out_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

_Success_(return != FALSE)
_At_((LPWSTR)DeviceInterfaceDetailData->DevicePath, _Post_z_)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInterfaceDetailW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Out_writes_bytes_to_opt_(DeviceInterfaceDetailDataSize, *RequiredSize) PSP_DEVICE_INTERFACE_DETAIL_DATA_W DeviceInterfaceDetailData,
    _In_ DWORD DeviceInterfaceDetailDataSize,
    _Out_opt_ _Out_range_(>=, sizeof(SP_DEVICE_INTERFACE_DETAIL_DATA_W)) PDWORD RequiredSize,
    _Out_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );

#ifdef UNICODE
#define SetupDiGetDeviceInterfaceDetail SetupDiGetDeviceInterfaceDetailW
#else
#define SetupDiGetDeviceInterfaceDetail SetupDiGetDeviceInterfaceDetailA
#endif

//
// Backward compatibility--do not use.
//
#define SetupDiGetInterfaceDeviceDetailW SetupDiGetDeviceInterfaceDetailW
#define SetupDiGetInterfaceDeviceDetailA SetupDiGetDeviceInterfaceDetailA
#ifdef UNICODE
#define SetupDiGetInterfaceDeviceDetail SetupDiGetDeviceInterfaceDetailW
#else
#define SetupDiGetInterfaceDeviceDetail SetupDiGetDeviceInterfaceDetailA
#endif


//
// Default install handler for DIF_INSTALLINTERFACES.
//
WINSETUPAPI
BOOL
WINAPI
SetupDiInstallDeviceInterfaces(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData
    );

//
// Backward compatibility--do not use.
//
#define SetupDiInstallInterfaceDevices SetupDiInstallDeviceInterfaces


#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDeviceInterfaceDefault(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _In_ DWORD Flags,
    _Reserved_ PVOID Reserved
    );

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP


//
// Default install handler for DIF_REGISTERDEVICE
//

//
// Flags for SetupDiRegisterDeviceInfo
//
#define SPRDI_FIND_DUPS        0x00000001

WINSETUPAPI
BOOL
WINAPI
SetupDiRegisterDeviceInfo(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Flags,
    _In_opt_ PSP_DETSIG_CMPPROC CompareProc,
    _In_opt_ PVOID CompareContext,
    _Out_opt_ PSP_DEVINFO_DATA DupDeviceInfoData
    );


//
// Ordinal values distinguishing between class drivers and
// device drivers.
// (Passed in 'DriverType' parameter of driver information list APIs)
//
#define SPDIT_NODRIVER           0x00000000
#define SPDIT_CLASSDRIVER        0x00000001
#define SPDIT_COMPATDRIVER       0x00000002

WINSETUPAPI
BOOL
WINAPI
SetupDiBuildDriverInfoList(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD DriverType
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiCancelDriverInfoSearch(
    _In_ HDEVINFO DeviceInfoSet
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiEnumDriverInfoA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD DriverType,
    _In_ DWORD MemberIndex,
    _Out_ PSP_DRVINFO_DATA_A DriverInfoData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiEnumDriverInfoW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD DriverType,
    _In_ DWORD MemberIndex,
    _Out_ PSP_DRVINFO_DATA_W DriverInfoData
    );

#ifdef UNICODE
#define SetupDiEnumDriverInfo SetupDiEnumDriverInfoW
#else
#define SetupDiEnumDriverInfo SetupDiEnumDriverInfoA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetSelectedDriverA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_ PSP_DRVINFO_DATA_A DriverInfoData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetSelectedDriverW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_ PSP_DRVINFO_DATA_W DriverInfoData
    );

#ifdef UNICODE
#define SetupDiGetSelectedDriver SetupDiGetSelectedDriverW
#else
#define SetupDiGetSelectedDriver SetupDiGetSelectedDriverA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiSetSelectedDriverA(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Inout_opt_ PSP_DRVINFO_DATA_A DriverInfoData
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiSetSelectedDriverW(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Inout_opt_ PSP_DRVINFO_DATA_W DriverInfoData
    );

#ifdef UNICODE
#define SetupDiSetSelectedDriver SetupDiSetSelectedDriverW
#else
#define SetupDiSetSelectedDriver SetupDiSetSelectedDriverA
#endif

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDriverInfoDetailA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DRVINFO_DATA_A DriverInfoData,
    _Inout_updates_bytes_opt_(DriverInfoDetailDataSize) PSP_DRVINFO_DETAIL_DATA_A DriverInfoDetailData,
    _In_ DWORD DriverInfoDetailDataSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDriverInfoDetailW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DRVINFO_DATA_W DriverInfoData,
    _Inout_updates_bytes_opt_(DriverInfoDetailDataSize) PSP_DRVINFO_DETAIL_DATA_W DriverInfoDetailData,
    _In_ DWORD DriverInfoDetailDataSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetDriverInfoDetail SetupDiGetDriverInfoDetailW
#else
#define SetupDiGetDriverInfoDetail SetupDiGetDriverInfoDetailA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiDestroyDriverInfoList(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD DriverType
    );


//
// Flags controlling what is included in the device information set built
// by SetupDiGetClassDevs
//
#define DIGCF_DEFAULT           0x00000001  // only valid with DIGCF_DEVICEINTERFACE
#define DIGCF_PRESENT           0x00000002
#define DIGCF_ALLCLASSES        0x00000004
#define DIGCF_PROFILE           0x00000008
#define DIGCF_DEVICEINTERFACE   0x00000010

//
// Backward compatibility--do not use.
//
#define DIGCF_INTERFACEDEVICE DIGCF_DEVICEINTERFACE


_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiGetClassDevsA(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ PCSTR Enumerator,
    _In_opt_ HWND hwndParent,
    _In_ DWORD Flags
    );

_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiGetClassDevsW(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ PCWSTR Enumerator,
    _In_opt_ HWND hwndParent,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define SetupDiGetClassDevs SetupDiGetClassDevsW
#else
#define SetupDiGetClassDevs SetupDiGetClassDevsA
#endif


_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiGetClassDevsExA(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ PCSTR Enumerator,
    _In_opt_ HWND hwndParent,
    _In_ DWORD Flags,
    _In_opt_ HDEVINFO DeviceInfoSet,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Check_return_
WINSETUPAPI
HDEVINFO
WINAPI
SetupDiGetClassDevsExW(
    _In_opt_ CONST GUID *ClassGuid,
    _In_opt_ PCWSTR Enumerator,
    _In_opt_ HWND hwndParent,
    _In_ DWORD Flags,
    _In_opt_ HDEVINFO DeviceInfoSet,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetClassDevsEx SetupDiGetClassDevsExW
#else
#define SetupDiGetClassDevsEx SetupDiGetClassDevsExA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetINFClassA(
    _In_ PCSTR InfName,
    _Out_ LPGUID ClassGuid,
    _Out_writes_(ClassNameSize) PSTR ClassName,
    _In_ DWORD ClassNameSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetINFClassW(
    _In_ PCWSTR InfName,
    _Out_ LPGUID ClassGuid,
    _Out_writes_(ClassNameSize) PWSTR ClassName,
    _In_ DWORD ClassNameSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetINFClass SetupDiGetINFClassW
#else
#define SetupDiGetINFClass SetupDiGetINFClassA
#endif


//
// Flags controlling exclusion from the class information list built
// by SetupDiBuildClassInfoList(Ex)
//
#define DIBCI_NOINSTALLCLASS   0x00000001
#define DIBCI_NODISPLAYCLASS   0x00000002

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiBuildClassInfoList(
    _In_ DWORD Flags,
    _Out_writes_to_opt_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiBuildClassInfoListExA(
    _In_ DWORD Flags,
    _Out_writes_to_opt_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiBuildClassInfoListExW(
    _In_ DWORD Flags,
    _Out_writes_to_opt_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiBuildClassInfoListEx SetupDiBuildClassInfoListExW
#else
#define SetupDiBuildClassInfoListEx SetupDiBuildClassInfoListExA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassDescriptionA(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassDescriptionSize) PSTR ClassDescription,
    _In_ DWORD ClassDescriptionSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassDescriptionW(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassDescriptionSize) PWSTR ClassDescription,
    _In_ DWORD ClassDescriptionSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetClassDescription SetupDiGetClassDescriptionW
#else
#define SetupDiGetClassDescription SetupDiGetClassDescriptionA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassDescriptionExA(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassDescriptionSize) PSTR ClassDescription,
    _In_ DWORD ClassDescriptionSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassDescriptionExW(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassDescriptionSize) PWSTR ClassDescription,
    _In_ DWORD ClassDescriptionSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetClassDescriptionEx SetupDiGetClassDescriptionExW
#else
#define SetupDiGetClassDescriptionEx SetupDiGetClassDescriptionExA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiCallClassInstaller(
    _In_ DI_FUNCTION InstallFunction,
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_SELECTDEVICE
//
WINSETUPAPI
BOOL
WINAPI
SetupDiSelectDevice(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_SELECTBESTCOMPATDRV
//
WINSETUPAPI
BOOL
WINAPI
SetupDiSelectBestCompatDrv(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_INSTALLDEVICE
//
WINSETUPAPI
BOOL
WINAPI
SetupDiInstallDevice(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_INSTALLDEVICEFILES
//
WINSETUPAPI
BOOL
WINAPI
SetupDiInstallDriverFiles(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_REGISTER_COINSTALLERS
//
WINSETUPAPI
BOOL
WINAPI
SetupDiRegisterCoDeviceInstallers(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_REMOVE
//
WINSETUPAPI
BOOL
WINAPI
SetupDiRemoveDevice(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData
    );


//
// Default install handler for DIF_UNREMOVE
//
WINSETUPAPI
BOOL
WINAPI
SetupDiUnremoveDevice(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData
    );

#if _SETUPAPI_VER >= _WIN32_WINNT_WS03

WINSETUPAPI
BOOL
WINAPI
SetupDiRestartDevices(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData
    );

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WS03

//
// Default install handler for DIF_PROPERTYCHANGE
//
WINSETUPAPI
BOOL
WINAPI
SetupDiChangeState(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiInstallClassA(
    _In_opt_ HWND hwndParent,
    _In_ PCSTR InfFileName,
    _In_ DWORD Flags,
    _In_opt_ HSPFILEQ FileQueue
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiInstallClassW(
    _In_opt_ HWND hwndParent,
    _In_ PCWSTR InfFileName,
    _In_ DWORD Flags,
    _In_opt_ HSPFILEQ FileQueue
    );

#ifdef UNICODE
#define SetupDiInstallClass SetupDiInstallClassW
#else
#define SetupDiInstallClass SetupDiInstallClassA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiInstallClassExA(
    _In_opt_ HWND hwndParent,
    _In_opt_ PCSTR InfFileName,
    _In_ DWORD Flags,
    _In_opt_ HSPFILEQ FileQueue,
    _In_opt_ CONST GUID *InterfaceClassGuid,
    _Reserved_ PVOID Reserved1,
    _Reserved_ PVOID Reserved2
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiInstallClassExW(
    _In_opt_ HWND hwndParent,
    _In_opt_ PCWSTR InfFileName,
    _In_ DWORD Flags,
    _In_opt_ HSPFILEQ FileQueue,
    _In_opt_ CONST GUID *InterfaceClassGuid,
    _Reserved_ PVOID Reserved1,
    _Reserved_ PVOID Reserved2
    );

#ifdef UNICODE
#define SetupDiInstallClassEx SetupDiInstallClassExW
#else
#define SetupDiInstallClassEx SetupDiInstallClassExA
#endif


_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiOpenClassRegKey(
    _In_opt_ CONST GUID *ClassGuid,
    _In_ REGSAM samDesired
    );


//
// Flags for SetupDiOpenClassRegKeyEx
//
#define DIOCR_INSTALLER   0x00000001    // class installer registry branch
#define DIOCR_INTERFACE   0x00000002    // interface class registry branch

_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiOpenClassRegKeyExA(
    _In_opt_ CONST GUID *ClassGuid,
    _In_ REGSAM samDesired,
    _In_ DWORD Flags,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiOpenClassRegKeyExW(
    _In_opt_ CONST GUID *ClassGuid,
    _In_ REGSAM samDesired,
    _In_ DWORD Flags,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiOpenClassRegKeyEx SetupDiOpenClassRegKeyExW
#else
#define SetupDiOpenClassRegKeyEx SetupDiOpenClassRegKeyExA
#endif


_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiCreateDeviceInterfaceRegKeyA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Reserved_ DWORD Reserved,
    _In_ REGSAM samDesired,
    _In_opt_ HINF InfHandle,
    _In_opt_ PCSTR InfSectionName
    );

_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiCreateDeviceInterfaceRegKeyW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Reserved_ DWORD Reserved,
    _In_ REGSAM samDesired,
    _In_opt_ HINF InfHandle,
    _In_opt_ PCWSTR InfSectionName
    );

#ifdef UNICODE
#define SetupDiCreateDeviceInterfaceRegKey SetupDiCreateDeviceInterfaceRegKeyW
#else
#define SetupDiCreateDeviceInterfaceRegKey SetupDiCreateDeviceInterfaceRegKeyA
#endif

//
// Backward compatibility--do not use.
//
#define SetupDiCreateInterfaceDeviceRegKeyW SetupDiCreateDeviceInterfaceRegKeyW
#define SetupDiCreateInterfaceDeviceRegKeyA SetupDiCreateDeviceInterfaceRegKeyA
#ifdef UNICODE
#define SetupDiCreateInterfaceDeviceRegKey SetupDiCreateDeviceInterfaceRegKeyW
#else
#define SetupDiCreateInterfaceDeviceRegKey SetupDiCreateDeviceInterfaceRegKeyA
#endif


_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiOpenDeviceInterfaceRegKey(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Reserved_ DWORD Reserved,
    _In_ REGSAM samDesired
    );

//
// Backward compatibility--do not use.
//
#define SetupDiOpenInterfaceDeviceRegKey SetupDiOpenDeviceInterfaceRegKey


WINSETUPAPI
BOOL
WINAPI
SetupDiDeleteDeviceInterfaceRegKey(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Reserved_ DWORD Reserved
    );

//
// Backward compatibility--do not use.
//
#define SetupDiDeleteInterfaceDeviceRegKey SetupDiDeleteDeviceInterfaceRegKey


//
// KeyType values for SetupDiCreateDevRegKey, SetupDiOpenDevRegKey, and
// SetupDiDeleteDevRegKey.
//
#define DIREG_DEV       0x00000001          // Open/Create/Delete device key
#define DIREG_DRV       0x00000002          // Open/Create/Delete driver key
#define DIREG_BOTH      0x00000004          // Delete both driver and Device key

_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiCreateDevRegKeyA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Scope,
    _In_ DWORD HwProfile,
    _In_ DWORD KeyType,
    _In_opt_ HINF InfHandle,
    _In_opt_ PCSTR InfSectionName
    );

_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiCreateDevRegKeyW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Scope,
    _In_ DWORD HwProfile,
    _In_ DWORD KeyType,
    _In_opt_ HINF InfHandle,
    _In_opt_ PCWSTR InfSectionName
    );

#ifdef UNICODE
#define SetupDiCreateDevRegKey SetupDiCreateDevRegKeyW
#else
#define SetupDiCreateDevRegKey SetupDiCreateDevRegKeyA
#endif


_Check_return_
WINSETUPAPI
HKEY
WINAPI
SetupDiOpenDevRegKey(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Scope,
    _In_ DWORD HwProfile,
    _In_ DWORD KeyType,
    _In_ REGSAM samDesired
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiDeleteDevRegKey(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Scope,
    _In_ DWORD HwProfile,
    _In_ DWORD KeyType
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileList(
    _Out_writes_to_(HwProfileListSize, *RequiredSize) PDWORD HwProfileList,
    _In_ DWORD HwProfileListSize,
    _Out_ PDWORD RequiredSize,
    _Out_opt_ PDWORD CurrentlyActiveIndex
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileListExA(
    _Out_writes_to_(HwProfileListSize, *RequiredSize) PDWORD HwProfileList,
    _In_ DWORD HwProfileListSize,
    _Out_ PDWORD RequiredSize,
    _Out_opt_ PDWORD CurrentlyActiveIndex,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileListExW(
    _Out_writes_to_(HwProfileListSize, *RequiredSize) PDWORD HwProfileList,
    _In_ DWORD HwProfileListSize,
    _Out_ PDWORD RequiredSize,
    _Out_opt_ PDWORD CurrentlyActiveIndex,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetHwProfileListEx SetupDiGetHwProfileListExW
#else
#define SetupDiGetHwProfileListEx SetupDiGetHwProfileListExA
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDevicePropertyKeys(
    _In_         HDEVINFO         DeviceInfoSet,
    _In_         PSP_DEVINFO_DATA DeviceInfoData,
    _Out_writes_opt_(PropertyKeyCount) DEVPROPKEY *PropertyKeyArray,
    _In_         DWORD            PropertyKeyCount,
    _Out_opt_    PDWORD           RequiredPropertyKeyCount,
    _In_         DWORD            Flags
    );

_Success_(return != FALSE)
_When_(*PropertyType == DEVPROP_TYPE_STRING, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_INDIRECT, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_LIST, _At_((PZZWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
WINSETUPAPI
BOOL
WINAPI
SetupDiGetDevicePropertyW(
    _In_         HDEVINFO         DeviceInfoSet,
    _In_         PSP_DEVINFO_DATA DeviceInfoData,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _Out_        DEVPROPTYPE     *PropertyType,
    _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _Out_opt_    PDWORD           RequiredSize,
    _In_         DWORD            Flags
    );

#ifdef UNICODE
#define SetupDiGetDeviceProperty SetupDiGetDevicePropertyW
#endif

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDevicePropertyW(
    _In_         HDEVINFO         DeviceInfoSet,
    _In_         PSP_DEVINFO_DATA DeviceInfoData,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _In_         DEVPROPTYPE      PropertyType,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _In_         DWORD            Flags
    );

#ifdef UNICODE
#define SetupDiSetDeviceProperty SetupDiSetDevicePropertyW
#endif

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInterfacePropertyKeys(
    _In_         HDEVINFO         DeviceInfoSet,
    _In_         PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _Out_writes_opt_(PropertyKeyCount) DEVPROPKEY *PropertyKeyArray,
    _In_         DWORD            PropertyKeyCount,
    _Out_opt_    PDWORD           RequiredPropertyKeyCount,
    _In_         DWORD            Flags
    );

_Success_(return != FALSE)
_When_(*PropertyType == DEVPROP_TYPE_STRING, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_INDIRECT, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_LIST, _At_((PZZWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInterfacePropertyW(
    _In_         HDEVINFO         DeviceInfoSet,
    _In_         PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _Out_        DEVPROPTYPE     *PropertyType,
    _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _Out_opt_    PDWORD           RequiredSize,
    _In_         DWORD            Flags
    );

#ifdef UNICODE
#define SetupDiGetDeviceInterfaceProperty SetupDiGetDeviceInterfacePropertyW
#endif

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDeviceInterfacePropertyW(
    _In_         HDEVINFO         DeviceInfoSet,
    _In_         PSP_DEVICE_INTERFACE_DATA DeviceInterfaceData,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _In_         DEVPROPTYPE      PropertyType,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _In_         DWORD            Flags
    );

#ifdef UNICODE
#define SetupDiSetDeviceInterfaceProperty SetupDiSetDeviceInterfacePropertyW
#endif

//
// Flags for SetupDiGetClassPropertyKeys, SetupDiGetClassProperty, and
// SetupDiSetClassProperty.
//
#define DICLASSPROP_INSTALLER   0x00000001    // device setup class property
#define DICLASSPROP_INTERFACE   0x00000002    // device interface class property

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassPropertyKeys(
    _In_   CONST GUID            *ClassGuid,
    _Out_writes_opt_(PropertyKeyCount) DEVPROPKEY *PropertyKeyArray,
    _In_         DWORD            PropertyKeyCount,
    _Out_opt_    PDWORD           RequiredPropertyKeyCount,
    _In_         DWORD            Flags
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassPropertyKeysExW(
    _In_   CONST GUID            *ClassGuid,
    _Out_writes_opt_(PropertyKeyCount) DEVPROPKEY *PropertyKeyArray,
    _In_         DWORD            PropertyKeyCount,
    _Out_opt_    PDWORD           RequiredPropertyKeyCount,
    _In_         DWORD            Flags,
    _In_opt_     PCWSTR           MachineName,
    _Reserved_   PVOID            Reserved
    );

#ifdef UNICODE
#define SetupDiGetClassPropertyKeysEx SetupDiGetClassPropertyKeysExW
#endif

_Success_(return != FALSE)
_When_(*PropertyType == DEVPROP_TYPE_STRING, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_INDIRECT, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_LIST, _At_((PZZWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassPropertyW(
    _In_   CONST GUID            *ClassGuid,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _Out_        DEVPROPTYPE     *PropertyType,
    _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _Out_opt_    PDWORD           RequiredSize,
    _In_         DWORD            Flags
    );

#ifdef UNICODE
#define SetupDiGetClassProperty SetupDiGetClassPropertyW
#endif

_Success_(return != FALSE)
_When_(*PropertyType == DEVPROP_TYPE_STRING, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_INDIRECT, _At_((PWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
_When_(*PropertyType == DEVPROP_TYPE_STRING_LIST, _At_((PZZWSTR) PropertyBuffer, _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize)))
WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassPropertyExW(
    _In_   CONST GUID            *ClassGuid,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _Out_        DEVPROPTYPE     *PropertyType,
    _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _Out_opt_    PDWORD           RequiredSize,
    _In_         DWORD            Flags,
    _In_opt_     PCWSTR           MachineName,
    _Reserved_   PVOID            Reserved
    );

#ifdef UNICODE
#define SetupDiGetClassPropertyEx SetupDiGetClassPropertyExW
#endif

WINSETUPAPI
BOOL
WINAPI
SetupDiSetClassPropertyW(
    _In_   CONST GUID            *ClassGuid,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _In_         DEVPROPTYPE      PropertyType,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _In_         DWORD            Flags
    );

#ifdef UNICODE
#define SetupDiSetClassProperty SetupDiSetClassPropertyW
#endif

WINSETUPAPI
BOOL
WINAPI
SetupDiSetClassPropertyExW(
    _In_   CONST GUID            *ClassGuid,
    _In_   CONST DEVPROPKEY      *PropertyKey,
    _In_         DEVPROPTYPE      PropertyType,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST PBYTE PropertyBuffer,
    _In_         DWORD            PropertyBufferSize,
    _In_         DWORD            Flags,
    _In_opt_     PCWSTR           MachineName,
    _Reserved_   PVOID            Reserved
    );

#ifdef UNICODE
#define SetupDiSetClassPropertyEx SetupDiSetClassPropertyExW
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN


//
// Device registry property codes
// (Codes marked as read-only (R) may only be used for
// SetupDiGetDeviceRegistryProperty)
//
// These values should cover the same set of registry properties
// as defined by the CM_DRP codes in cfgmgr32.h.
//
// Note that SPDRP codes are zero based while CM_DRP codes are one based!
//
#define SPDRP_DEVICEDESC                  (0x00000000)  // DeviceDesc (R/W)
#define SPDRP_HARDWAREID                  (0x00000001)  // HardwareID (R/W)
#define SPDRP_COMPATIBLEIDS               (0x00000002)  // CompatibleIDs (R/W)
#define SPDRP_UNUSED0                     (0x00000003)  // unused
#define SPDRP_SERVICE                     (0x00000004)  // Service (R/W)
#define SPDRP_UNUSED1                     (0x00000005)  // unused
#define SPDRP_UNUSED2                     (0x00000006)  // unused
#define SPDRP_CLASS                       (0x00000007)  // Class (R--tied to ClassGUID)
#define SPDRP_CLASSGUID                   (0x00000008)  // ClassGUID (R/W)
#define SPDRP_DRIVER                      (0x00000009)  // Driver (R/W)
#define SPDRP_CONFIGFLAGS                 (0x0000000A)  // ConfigFlags (R/W)
#define SPDRP_MFG                         (0x0000000B)  // Mfg (R/W)
#define SPDRP_FRIENDLYNAME                (0x0000000C)  // FriendlyName (R/W)
#define SPDRP_LOCATION_INFORMATION        (0x0000000D)  // LocationInformation (R/W)
#define SPDRP_PHYSICAL_DEVICE_OBJECT_NAME (0x0000000E)  // PhysicalDeviceObjectName (R)
#define SPDRP_CAPABILITIES                (0x0000000F)  // Capabilities (R)
#define SPDRP_UI_NUMBER                   (0x00000010)  // UiNumber (R)
#define SPDRP_UPPERFILTERS                (0x00000011)  // UpperFilters (R/W)
#define SPDRP_LOWERFILTERS                (0x00000012)  // LowerFilters (R/W)
#define SPDRP_BUSTYPEGUID                 (0x00000013)  // BusTypeGUID (R)
#define SPDRP_LEGACYBUSTYPE               (0x00000014)  // LegacyBusType (R)
#define SPDRP_BUSNUMBER                   (0x00000015)  // BusNumber (R)
#define SPDRP_ENUMERATOR_NAME             (0x00000016)  // Enumerator Name (R)
#define SPDRP_SECURITY                    (0x00000017)  // Security (R/W, binary form)
#define SPDRP_SECURITY_SDS                (0x00000018)  // Security (W, SDS form)
#define SPDRP_DEVTYPE                     (0x00000019)  // Device Type (R/W)
#define SPDRP_EXCLUSIVE                   (0x0000001A)  // Device is exclusive-access (R/W)
#define SPDRP_CHARACTERISTICS             (0x0000001B)  // Device Characteristics (R/W)
#define SPDRP_ADDRESS                     (0x0000001C)  // Device Address (R)
#define SPDRP_UI_NUMBER_DESC_FORMAT       (0X0000001D)  // UiNumberDescFormat (R/W)
#define SPDRP_DEVICE_POWER_DATA           (0x0000001E)  // Device Power Data (R)
#define SPDRP_REMOVAL_POLICY              (0x0000001F)  // Removal Policy (R)
#define SPDRP_REMOVAL_POLICY_HW_DEFAULT   (0x00000020)  // Hardware Removal Policy (R)
#define SPDRP_REMOVAL_POLICY_OVERRIDE     (0x00000021)  // Removal Policy Override (RW)
#define SPDRP_INSTALL_STATE               (0x00000022)  // Device Install State (R)
#define SPDRP_LOCATION_PATHS              (0x00000023)  // Device Location Paths (R)
#define SPDRP_BASE_CONTAINERID            (0x00000024)  // Base ContainerID (R)

#define SPDRP_MAXIMUM_PROPERTY            (0x00000025)  // Upper bound on ordinals

//
// Class registry property codes
// (Codes marked as read-only (R) may only be used for
// SetupDiGetClassRegistryProperty)
//
// These values should cover the same set of registry properties
// as defined by the CM_CRP codes in cfgmgr32.h.
// they should also have a 1:1 correspondence with Device registers, where applicable
// but no overlap otherwise
//
#define SPCRP_UPPERFILTERS                (0x00000011)  // UpperFilters (R/W)
#define SPCRP_LOWERFILTERS                (0x00000012)  // LowerFilters (R/W)
#define SPCRP_SECURITY                    (0x00000017)  // Security (R/W, binary form)
#define SPCRP_SECURITY_SDS                (0x00000018)  // Security (W, SDS form)
#define SPCRP_DEVTYPE                     (0x00000019)  // Device Type (R/W)
#define SPCRP_EXCLUSIVE                   (0x0000001A)  // Device is exclusive-access (R/W)
#define SPCRP_CHARACTERISTICS             (0x0000001B)  // Device Characteristics (R/W)
#define SPCRP_MAXIMUM_PROPERTY            (0x0000001C)  // Upper bound on ordinals

_Success_(return != FALSE)
_When_((*PropertyRegDataType == REG_SZ), _At_((PSTR) PropertyBuffer, _Post_valid_))
_When_((*PropertyRegDataType == REG_MULTI_SZ), _At_((PZZSTR) PropertyBuffer, _Post_valid_))
WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceRegistryPropertyA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Property,
    _Out_opt_ PDWORD PropertyRegDataType,
    _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

_Success_(return != FALSE)
_When_((*PropertyRegDataType == REG_SZ), _At_((PWSTR) PropertyBuffer, _Post_valid_))
_When_((*PropertyRegDataType == REG_MULTI_SZ), _At_((PZZWSTR) PropertyBuffer, _Post_valid_))
WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceRegistryPropertyW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Property,
    _Out_opt_ PDWORD PropertyRegDataType,
    _Out_writes_bytes_to_opt_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetDeviceRegistryProperty SetupDiGetDeviceRegistryPropertyW
#else
#define SetupDiGetDeviceRegistryProperty SetupDiGetDeviceRegistryPropertyA
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassRegistryPropertyA(
    _In_ CONST GUID *ClassGuid,
    _In_ DWORD Property,
    _Out_opt_ PDWORD PropertyRegDataType,
    _Out_writes_bytes_to_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassRegistryPropertyW(
    _In_ CONST GUID *ClassGuid,
    _In_ DWORD Property,
    _Out_opt_ PDWORD PropertyRegDataType,
    _Out_writes_bytes_to_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetClassRegistryProperty SetupDiGetClassRegistryPropertyW
#else
#define SetupDiGetClassRegistryProperty SetupDiGetClassRegistryPropertyA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDeviceRegistryPropertyA(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Property,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST BYTE *PropertyBuffer,
    _In_ DWORD PropertyBufferSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDeviceRegistryPropertyW(
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ DWORD Property,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST BYTE *PropertyBuffer,
    _In_ DWORD PropertyBufferSize
    );

#ifdef UNICODE
#define SetupDiSetDeviceRegistryProperty SetupDiSetDeviceRegistryPropertyW
#else
#define SetupDiSetDeviceRegistryProperty SetupDiSetDeviceRegistryPropertyA
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupDiSetClassRegistryPropertyA(
    _In_ CONST GUID *ClassGuid,
    _In_ DWORD Property,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST BYTE *PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiSetClassRegistryPropertyW(
    _In_ CONST GUID *ClassGuid,
    _In_ DWORD Property,
    _In_reads_bytes_opt_(PropertyBufferSize) CONST BYTE *PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiSetClassRegistryProperty SetupDiSetClassRegistryPropertyW
#else
#define SetupDiSetClassRegistryProperty SetupDiSetClassRegistryPropertyA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInstallParamsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_ PSP_DEVINSTALL_PARAMS_A DeviceInstallParams
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDeviceInstallParamsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_ PSP_DEVINSTALL_PARAMS_W DeviceInstallParams
    );

#ifdef UNICODE
#define SetupDiGetDeviceInstallParams SetupDiGetDeviceInstallParamsW
#else
#define SetupDiGetDeviceInstallParams SetupDiGetDeviceInstallParamsA
#endif

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassInstallParamsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_writes_bytes_to_opt_(ClassInstallParamsSize, *RequiredSize) PSP_CLASSINSTALL_HEADER ClassInstallParams,
    _In_ DWORD ClassInstallParamsSize,
    _Out_opt_ PDWORD RequiredSize
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassInstallParamsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _Out_writes_bytes_to_opt_(ClassInstallParamsSize, *RequiredSize) PSP_CLASSINSTALL_HEADER ClassInstallParams,
    _In_ DWORD ClassInstallParamsSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetClassInstallParams SetupDiGetClassInstallParamsW
#else
#define SetupDiGetClassInstallParams SetupDiGetClassInstallParamsA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiSetDeviceInstallParamsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DEVINSTALL_PARAMS_A DeviceInstallParams
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDeviceInstallParamsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DEVINSTALL_PARAMS_W DeviceInstallParams
    );

#ifdef UNICODE
#define SetupDiSetDeviceInstallParams SetupDiSetDeviceInstallParamsW
#else
#define SetupDiSetDeviceInstallParams SetupDiSetDeviceInstallParamsA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiSetClassInstallParamsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_reads_bytes_opt_(ClassInstallParamsSize) PSP_CLASSINSTALL_HEADER ClassInstallParams,
    _In_ DWORD ClassInstallParamsSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiSetClassInstallParamsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_reads_bytes_opt_(ClassInstallParamsSize) PSP_CLASSINSTALL_HEADER ClassInstallParams,
    _In_ DWORD ClassInstallParamsSize
    );

#ifdef UNICODE
#define SetupDiSetClassInstallParams SetupDiSetClassInstallParamsW
#else
#define SetupDiSetClassInstallParams SetupDiSetClassInstallParamsA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetDriverInstallParamsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DRVINFO_DATA_A DriverInfoData,
    _Out_ PSP_DRVINSTALL_PARAMS DriverInstallParams
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetDriverInstallParamsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DRVINFO_DATA_W DriverInfoData,
    _Out_ PSP_DRVINSTALL_PARAMS DriverInstallParams
    );

#ifdef UNICODE
#define SetupDiGetDriverInstallParams SetupDiGetDriverInstallParamsW
#else
#define SetupDiGetDriverInstallParams SetupDiGetDriverInstallParamsA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiSetDriverInstallParamsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DRVINFO_DATA_A DriverInfoData,
    _In_ PSP_DRVINSTALL_PARAMS DriverInstallParams
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiSetDriverInstallParamsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_DRVINFO_DATA_W DriverInfoData,
    _In_ PSP_DRVINSTALL_PARAMS DriverInstallParams
    );

#ifdef UNICODE
#define SetupDiSetDriverInstallParams SetupDiSetDriverInstallParamsW
#else
#define SetupDiSetDriverInstallParams SetupDiSetDriverInstallParamsA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiLoadClassIcon(
    _In_ CONST GUID *ClassGuid,
    _Out_opt_ HICON *LargeIcon,
    _Out_opt_ PINT MiniIconIndex
    );

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

WINSETUPAPI
BOOL
WINAPI
SetupDiLoadDeviceIcon(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ UINT cxIcon,
    _In_ UINT cyIcon,
    _In_ DWORD Flags,
    _Out_ HICON *hIcon
    );

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN


//
// Flags controlling the drawing of mini-icons
//
#define DMI_MASK      0x00000001
#define DMI_BKCOLOR   0x00000002
#define DMI_USERECT   0x00000004

WINSETUPAPI
INT
WINAPI
SetupDiDrawMiniIcon(
    _In_ HDC hdc,
    _In_ RECT rc,
    _In_ INT MiniIconIndex,
    _In_ DWORD Flags
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassBitmapIndex(
    _In_opt_ CONST GUID *ClassGuid,
    _Out_ PINT MiniIconIndex
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassImageList(
    _Out_ PSP_CLASSIMAGELIST_DATA ClassImageListData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassImageListExA(
    _Out_ PSP_CLASSIMAGELIST_DATA ClassImageListData,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassImageListExW(
    _Out_ PSP_CLASSIMAGELIST_DATA ClassImageListData,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetClassImageListEx SetupDiGetClassImageListExW
#else
#define SetupDiGetClassImageListEx SetupDiGetClassImageListExA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassImageIndex(
    _In_ PSP_CLASSIMAGELIST_DATA ClassImageListData,
    _In_ CONST GUID *ClassGuid,
    _Out_ PINT ImageIndex
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiDestroyClassImageList(
    _In_ PSP_CLASSIMAGELIST_DATA ClassImageListData
    );


//
// PropertySheetType values for the SetupDiGetClassDevPropertySheets API
//
#define DIGCDP_FLAG_BASIC           0x00000001
#define DIGCDP_FLAG_ADVANCED        0x00000002

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#define DIGCDP_FLAG_REMOTE_BASIC    0x00000003  // not presently implemented
#define DIGCDP_FLAG_REMOTE_ADVANCED 0x00000004

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassDevPropertySheetsA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ LPPROPSHEETHEADERA PropertySheetHeader,
    _In_ DWORD PropertySheetHeaderPageListSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_ DWORD PropertySheetType
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetClassDevPropertySheetsW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ LPPROPSHEETHEADERW PropertySheetHeader,
    _In_ DWORD PropertySheetHeaderPageListSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_ DWORD PropertySheetType
    );

#ifdef UNICODE
#define SetupDiGetClassDevPropertySheets SetupDiGetClassDevPropertySheetsW
#else
#define SetupDiGetClassDevPropertySheets SetupDiGetClassDevPropertySheetsA
#endif


//
// Define ICON IDs publicly exposed from setupapi.
//
#define IDI_RESOURCEFIRST           159
#define IDI_RESOURCE                159
#define IDI_RESOURCELAST            161
#define IDI_RESOURCEOVERLAYFIRST    161
#define IDI_RESOURCEOVERLAYLAST     161
#define IDI_CONFLICT                161

#define IDI_CLASSICON_OVERLAYFIRST  500
#define IDI_CLASSICON_OVERLAYLAST   502
#define IDI_PROBLEM_OVL             500
#define IDI_DISABLED_OVL            501
#define IDI_FORCED_OVL              502


WINSETUPAPI
BOOL
WINAPI
SetupDiAskForOEMDisk(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiSelectOEMDrv(
    _In_opt_ HWND hwndParent,
    _In_ HDEVINFO DeviceInfoSet,
    _Inout_opt_ PSP_DEVINFO_DATA DeviceInfoData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiClassNameFromGuidA(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassNameSize) PSTR ClassName,
    _In_ DWORD ClassNameSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiClassNameFromGuidW(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassNameSize) PWSTR ClassName,
    _In_ DWORD ClassNameSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiClassNameFromGuid SetupDiClassNameFromGuidW
#else
#define SetupDiClassNameFromGuid SetupDiClassNameFromGuidA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiClassNameFromGuidExA(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassNameSize) PSTR ClassName,
    _In_ DWORD ClassNameSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiClassNameFromGuidExW(
    _In_ CONST GUID *ClassGuid,
    _Out_writes_(ClassNameSize) PWSTR ClassName,
    _In_ DWORD ClassNameSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiClassNameFromGuidEx SetupDiClassNameFromGuidExW
#else
#define SetupDiClassNameFromGuidEx SetupDiClassNameFromGuidExA
#endif

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiClassGuidsFromNameA(
    _In_ PCSTR ClassName,
    _Out_writes_to_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiClassGuidsFromNameW(
    _In_ PCWSTR ClassName,
    _Out_writes_to_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiClassGuidsFromName SetupDiClassGuidsFromNameW
#else
#define SetupDiClassGuidsFromName SetupDiClassGuidsFromNameA
#endif


_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiClassGuidsFromNameExA(
    _In_ PCSTR ClassName,
    _Out_writes_to_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiClassGuidsFromNameExW(
    _In_ PCWSTR ClassName,
    _Out_writes_to_(ClassGuidListSize, *RequiredSize) LPGUID ClassGuidList,
    _In_ DWORD ClassGuidListSize,
    _Out_ PDWORD RequiredSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiClassGuidsFromNameEx SetupDiClassGuidsFromNameExW
#else
#define SetupDiClassGuidsFromNameEx SetupDiClassGuidsFromNameExA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileFriendlyNameA(
    _In_ DWORD HwProfile,
    _Out_writes_(FriendlyNameSize) PSTR FriendlyName,
    _In_ DWORD FriendlyNameSize,
    _Out_opt_ PDWORD RequiredSize
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileFriendlyNameW(
    _In_ DWORD HwProfile,
    _Out_writes_(FriendlyNameSize) PWSTR FriendlyName,
    _In_ DWORD FriendlyNameSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetHwProfileFriendlyName SetupDiGetHwProfileFriendlyNameW
#else
#define SetupDiGetHwProfileFriendlyName SetupDiGetHwProfileFriendlyNameA
#endif


WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileFriendlyNameExA(
    _In_ DWORD HwProfile,
    _Out_writes_(FriendlyNameSize) PSTR FriendlyName,
    _In_ DWORD FriendlyNameSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCSTR MachineName,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetHwProfileFriendlyNameExW(
    _In_ DWORD HwProfile,
    _Out_writes_(FriendlyNameSize) PWSTR FriendlyName,
    _In_ DWORD FriendlyNameSize,
    _Out_opt_ PDWORD RequiredSize,
    _In_opt_ PCWSTR MachineName,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetHwProfileFriendlyNameEx SetupDiGetHwProfileFriendlyNameExW
#else
#define SetupDiGetHwProfileFriendlyNameEx SetupDiGetHwProfileFriendlyNameExA
#endif


//
// PageType values for SetupDiGetWizardPage API
//
#define SPWPT_SELECTDEVICE      0x00000001

//
// Flags for SetupDiGetWizardPage API
//
#define SPWP_USE_DEVINFO_DATA   0x00000001

WINSETUPAPI
HPROPSHEETPAGE
WINAPI
SetupDiGetWizardPage(
    _In_ HDEVINFO DeviceInfoSet,
    _In_opt_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PSP_INSTALLWIZARD_DATA InstallWizardData,
    _In_ DWORD PageType,
    _In_ DWORD Flags
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiGetSelectedDevice(
    _In_ HDEVINFO DeviceInfoSet,
    _Out_ PSP_DEVINFO_DATA DeviceInfoData
    );


WINSETUPAPI
BOOL
WINAPI
SetupDiSetSelectedDevice(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData
    );


#if _SETUPAPI_VER >= _WIN32_WINNT_WS03

WINSETUPAPI
BOOL
WINAPI
SetupDiGetActualModelsSectionA(
    _In_ PINFCONTEXT Context,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _Out_writes_opt_(InfSectionWithExtSize) PSTR InfSectionWithExt,
    _In_ DWORD InfSectionWithExtSize,
    _Out_opt_ PDWORD RequiredSize,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetActualModelsSectionW(
    _In_ PINFCONTEXT Context,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _Out_writes_opt_(InfSectionWithExtSize) PWSTR InfSectionWithExt,
    _In_ DWORD InfSectionWithExtSize,
    _Out_opt_ PDWORD RequiredSize,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetActualModelsSection SetupDiGetActualModelsSectionW
#else
#define SetupDiGetActualModelsSection SetupDiGetActualModelsSectionA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WS03


WINSETUPAPI
BOOL
WINAPI
SetupDiGetActualSectionToInstallA(
    _In_ HINF InfHandle,
    _In_ PCSTR InfSectionName,
    _Out_writes_opt_(InfSectionWithExtSize) PSTR InfSectionWithExt,
    _In_ DWORD InfSectionWithExtSize,
    _Out_opt_ PDWORD RequiredSize,
    _Out_opt_ PSTR *Extension
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetActualSectionToInstallW(
    _In_ HINF InfHandle,
    _In_ PCWSTR InfSectionName,
    _Out_writes_opt_(InfSectionWithExtSize) PWSTR InfSectionWithExt,
    _In_ DWORD InfSectionWithExtSize,
    _Out_opt_ PDWORD RequiredSize,
    _Out_opt_ PWSTR *Extension
    );

#ifdef UNICODE
#define SetupDiGetActualSectionToInstall SetupDiGetActualSectionToInstallW
#else
#define SetupDiGetActualSectionToInstall SetupDiGetActualSectionToInstallA
#endif


#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

WINSETUPAPI
BOOL
WINAPI
SetupDiGetActualSectionToInstallExA(
    _In_ HINF InfHandle,
    _In_ PCSTR InfSectionName,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _Out_writes_opt_(InfSectionWithExtSize) PSTR InfSectionWithExt,
    _In_ DWORD InfSectionWithExtSize,
    _Out_opt_ PDWORD RequiredSize,
    _Out_opt_ PSTR *Extension,
    _Reserved_ PVOID Reserved
    );

WINSETUPAPI
BOOL
WINAPI
SetupDiGetActualSectionToInstallExW(
    _In_ HINF InfHandle,
    _In_ PCWSTR InfSectionName,
    _In_opt_ PSP_ALTPLATFORM_INFO AlternatePlatformInfo,
    _Out_writes_opt_(InfSectionWithExtSize) PWSTR InfSectionWithExt,
    _In_ DWORD InfSectionWithExtSize,
    _Out_opt_ PDWORD RequiredSize,
    _Out_opt_ PWSTR *Extension,
    _Reserved_ PVOID Reserved
    );

#ifdef UNICODE
#define SetupDiGetActualSectionToInstallEx SetupDiGetActualSectionToInstallExW
#else
#define SetupDiGetActualSectionToInstallEx SetupDiGetActualSectionToInstallExA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP


#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// SetupEnumInfSections is for low-level parsing of an INF
//
WINSETUPAPI
BOOL
WINAPI
SetupEnumInfSectionsA (
    _In_ HINF InfHandle,
    _In_ UINT Index,
    _Out_writes_opt_(Size) PSTR Buffer,
    _In_ UINT Size,
    _Out_opt_ UINT *SizeNeeded
    );

WINSETUPAPI
BOOL
WINAPI
SetupEnumInfSectionsW (
    _In_ HINF InfHandle,
    _In_ UINT Index,
    _Out_writes_opt_(Size) PWSTR Buffer,
    _In_ UINT Size,
    _Out_opt_ UINT *SizeNeeded
    );

#ifdef UNICODE
#define SetupEnumInfSections SetupEnumInfSectionsW
#else
#define SetupEnumInfSections SetupEnumInfSectionsA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

typedef struct _SP_INF_SIGNER_INFO_V1_A {
    DWORD  cbSize;
    CHAR   CatalogFile[MAX_PATH];
    CHAR   DigitalSigner[MAX_PATH];
    CHAR   DigitalSignerVersion[MAX_PATH];
} SP_INF_SIGNER_INFO_V1_A, *PSP_INF_SIGNER_INFO_V1_A;

typedef struct _SP_INF_SIGNER_INFO_V1_W {
    DWORD  cbSize;
    WCHAR  CatalogFile[MAX_PATH];
    WCHAR  DigitalSigner[MAX_PATH];
    WCHAR  DigitalSignerVersion[MAX_PATH];
} SP_INF_SIGNER_INFO_V1_W, *PSP_INF_SIGNER_INFO_V1_W;

#ifdef UNICODE
typedef SP_INF_SIGNER_INFO_V1_W SP_INF_SIGNER_INFO_V1;
typedef PSP_INF_SIGNER_INFO_V1_W PSP_INF_SIGNER_INFO_V1;
#else
typedef SP_INF_SIGNER_INFO_V1_A SP_INF_SIGNER_INFO_V1;
typedef PSP_INF_SIGNER_INFO_V1_A PSP_INF_SIGNER_INFO_V1;
#endif

#if _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

typedef struct _SP_INF_SIGNER_INFO_V2_A {
    DWORD  cbSize;
    CHAR   CatalogFile[MAX_PATH];
    CHAR   DigitalSigner[MAX_PATH];
    CHAR   DigitalSignerVersion[MAX_PATH];
    DWORD  SignerScore;
} SP_INF_SIGNER_INFO_V2_A, *PSP_INF_SIGNER_INFO_V2_A;

typedef struct _SP_INF_SIGNER_INFO_V2_W {
    DWORD  cbSize;
    WCHAR  CatalogFile[MAX_PATH];
    WCHAR  DigitalSigner[MAX_PATH];
    WCHAR  DigitalSignerVersion[MAX_PATH];
    DWORD  SignerScore;
} SP_INF_SIGNER_INFO_V2_W, *PSP_INF_SIGNER_INFO_V2_W;

#ifdef UNICODE
typedef SP_INF_SIGNER_INFO_V2_W SP_INF_SIGNER_INFO_V2;
typedef PSP_INF_SIGNER_INFO_V2_W PSP_INF_SIGNER_INFO_V2;
#else
typedef SP_INF_SIGNER_INFO_V2_A SP_INF_SIGNER_INFO_V2;
typedef PSP_INF_SIGNER_INFO_V2_A PSP_INF_SIGNER_INFO_V2;
#endif

//
// Driver signer scores (high order bit of the signing byte means unsigned)
//
#define SIGNERSCORE_UNKNOWN         0xFF000000
#define SIGNERSCORE_W9X_SUSPECT     0xC0000000
#define SIGNERSCORE_UNSIGNED        0x80000000
#define SIGNERSCORE_AUTHENTICODE    0x0F000000
#define SIGNERSCORE_WHQL            0x0D000005  // base WHQL.
#define SIGNERSCORE_UNCLASSIFIED    0x0D000004  // UNCLASSIFIED == INBOX == STANDARD == PREMIUM when the SIGNERSCORE_MASK
#define SIGNERSCORE_INBOX           0x0D000003  // filter is applied.
#define SIGNERSCORE_LOGO_STANDARD   0x0D000002
#define SIGNERSCORE_LOGO_PREMIUM    0x0D000001

#define SIGNERSCORE_MASK            0xFF000000  // Mask out all but the upper BYTE which contains the ranking signer information
#define SIGNERSCORE_SIGNED_MASK     0xF0000000  // Mask out only the upper nibble, which tells us if the package is signed or not.

#endif // _SETUPAPI_VER >= _WIN32_WINNT_LONGHORN

#if USE_SP_INF_SIGNER_INFO_V1 || (_SETUPAPI_VER < _WIN32_WINNT_LONGHORN)  // use version 1 signer info structure

typedef SP_INF_SIGNER_INFO_V1_A SP_INF_SIGNER_INFO_A;
typedef PSP_INF_SIGNER_INFO_V1_A PSP_INF_SIGNER_INFO_A;
typedef SP_INF_SIGNER_INFO_V1_W SP_INF_SIGNER_INFO_W;
typedef PSP_INF_SIGNER_INFO_V1_W PSP_INF_SIGNER_INFO_W;
typedef SP_INF_SIGNER_INFO_V1 SP_INF_SIGNER_INFO;
typedef PSP_INF_SIGNER_INFO_V1 PSP_INF_SIGNER_INFO;

#else                       // use version 2 signer info structure

typedef SP_INF_SIGNER_INFO_V2_A SP_INF_SIGNER_INFO_A;
typedef PSP_INF_SIGNER_INFO_V2_A PSP_INF_SIGNER_INFO_A;
typedef SP_INF_SIGNER_INFO_V2_W SP_INF_SIGNER_INFO_W;
typedef PSP_INF_SIGNER_INFO_V2_W PSP_INF_SIGNER_INFO_W;
typedef SP_INF_SIGNER_INFO_V2 SP_INF_SIGNER_INFO;
typedef PSP_INF_SIGNER_INFO_V2 PSP_INF_SIGNER_INFO;

#endif  // use current version of signer info structure


WINSETUPAPI
BOOL
WINAPI
SetupVerifyInfFileA(
    _In_ PCSTR InfName,
    _In_opt_ PSP_ALTPLATFORM_INFO AltPlatformInfo,
    _Inout_ PSP_INF_SIGNER_INFO_A InfSignerInfo
    );

WINSETUPAPI
BOOL
WINAPI
SetupVerifyInfFileW(
    _In_ PCWSTR InfName,
    _In_opt_ PSP_ALTPLATFORM_INFO AltPlatformInfo,
    _Inout_ PSP_INF_SIGNER_INFO_W InfSignerInfo
    );

#ifdef UNICODE
#define SetupVerifyInfFile SetupVerifyInfFileW
#else
#define SetupVerifyInfFile SetupVerifyInfFileA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP

#if _SETUPAPI_VER >= _WIN32_WINNT_WINXP

//
// Flags for use by SetupDiGetCustomDeviceProperty
//
#define DICUSTOMDEVPROP_MERGE_MULTISZ    0x00000001

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetCustomDevicePropertyA(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PCSTR CustomPropertyName,
    _In_ DWORD Flags,
    _Out_opt_ PDWORD PropertyRegDataType,
    _Out_writes_bytes_to_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

_Success_(return != FALSE)
WINSETUPAPI
BOOL
WINAPI
SetupDiGetCustomDevicePropertyW(
    _In_ HDEVINFO DeviceInfoSet,
    _In_ PSP_DEVINFO_DATA DeviceInfoData,
    _In_ PCWSTR CustomPropertyName,
    _In_ DWORD Flags,
    _Out_opt_ PDWORD PropertyRegDataType,
    _Out_writes_bytes_to_(PropertyBufferSize, *RequiredSize) PBYTE PropertyBuffer,
    _In_ DWORD PropertyBufferSize,
    _Out_opt_ PDWORD RequiredSize
    );

#ifdef UNICODE
#define SetupDiGetCustomDeviceProperty SetupDiGetCustomDevicePropertyW
#else
#define SetupDiGetCustomDeviceProperty SetupDiGetCustomDevicePropertyA
#endif

#endif // _SETUPAPI_VER >= _WIN32_WINNT_WINXP


#if _SETUPAPI_VER >= _WIN32_WINNT_WS03

//
// To configure WMI security for downlevel platforms where the [DDInstall.WMI]
// section isn't natively supported by setupapi, a redistributable co-installer
// is supplied in the DDK for use on those platforms.
//

//
// Flags for use by SetupConfigureWmiFromInfSection
//
#define SCWMI_CLOBBER_SECURITY  0x00000001

WINSETUPAPI
BOOL
WINAPI
SetupConfigureWmiFromInfSectionA(
    _In_ HINF InfHandle,
    _In_ PCSTR SectionName,
    _In_ DWORD Flags
    );

WINSETUPAPI
BOOL
WINAPI
SetupConfigureWmiFromInfSectionW(
    _In_ HINF InfHandle,
    _In_ PCWSTR SectionName,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define SetupConfigureWmiFromInfSection SetupConfigureWmiFromInfSectionW
#else
#define SetupConfigureWmiFromInfSection SetupConfigureWmiFromInfSectionA
#endif


#endif // _SETUPAPI_VER >= _WIN32_WINNT_WS03



#ifdef __cplusplus
}
#endif

#include <poppack.h>

#if defined (_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _INC_SETUPAPI
