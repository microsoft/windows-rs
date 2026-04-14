//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  File:       advpub.h
//
//  Description:
//      Interfaces implemented by advpack.dll.
//
//----------------------------------------------------------------------------

//@[contract("advpub"), comment("MVI_tracked - https://osgwiki.com/wiki/Microsoft_Virus_Initiative")];

#ifndef _ADVPUB_H_
#define _ADVPUB_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <setupapi.h>
#include <cfgmgr32.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: RunSetupCommand
//
// SYNOPSIS:    Execute an install section in an INF file, or execute a
//              program.  Advanced INF files are supported.
//
// RETURN CODES:
//
//      S_OK                                 Everything OK, no reboot needed.
//                                           No EXE to wait for.
//      S_ASYNCHRONOUS                       Please wait on phEXE.
//      ERROR_SUCCESS_REBOOT_REQUIRED        Reboot required.
//      E_INVALIDARG                         NULL specified in szCmdName or szDir
//      HRESULT_FROM_WIN32(ERROR_OLD_WIN_VERSION) INF's not supported on this OS version
//      E_UNEXPECTED                         Catastrophic failure(should never happen).
//      HRESULT_FROM_WIN32(GetLastError())   Anything else
/////////////////////////////////////////////////////////////////////////////

#ifndef S_ASYNCHRONOUS
#define S_ASYNCHRONOUS  _HRESULT_TYPEDEF_(0x401e8L)
#endif

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI RunSetupCommandA(HWND hWnd, LPCSTR szCmdName, LPCSTR szInfSection, LPCSTR szDir, LPCSTR lpszTitle, HANDLE* phEXE, DWORD dwFlags, LPVOID pvReserved);
HRESULT WINAPI RunSetupCommandW(HWND hWnd, LPCWSTR szCmdName, LPCWSTR szInfSection, LPCWSTR szDir, LPCWSTR lpszTitle, HANDLE* phEXE, DWORD dwFlags, LPVOID pvReserved);
#ifdef UNICODE
#define RunSetupCommand  RunSetupCommandW
#else
#define RunSetupCommand  RunSetupCommandA
#endif // !UNICODE
#else
HRESULT WINAPI RunSetupCommand(HWND hWnd, LPCSTR szCmdName, LPCSTR szInfSection, LPCSTR szDir, LPCSTR lpszTitle, HANDLE* phEXE, DWORD dwFlags, LPVOID pvReserved);
#endif  // (_WIN32_IE >= 0x0605)

// FLAGS:
#define RSC_FLAG_INF                1       // exxcute INF install
#define RSC_FLAG_SKIPDISKSPACECHECK 2       // Currently does nothing
#define RSC_FLAG_QUIET              4       // quiet mode, no UI
#define RSC_FLAG_NGCONV             8       // don't run groupConv
#define RSC_FLAG_UPDHLPDLLS         16      // force to self-updating on user's system
#define RSC_FLAG_DELAYREGISTEROCX   512     // force delay of ocx registration
#define RSC_FLAG_SETUPAPI           1024    // use setupapi.dll
// don't add new RSC_FLAG_XXX flags! add LaunchINFSectionEx() flags instead.


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: NeedRebootInit
//
// SYNOPSIS:    Initializes state for reboot checking.  Call this function
//              before calling RunSetupCommand.
// RETURNS:     value required to be passed to NeedReboot()
/////////////////////////////////////////////////////////////////////////////

DWORD WINAPI NeedRebootInit();


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: NeedReboot
//
// SYNOPSIS:    Compares stored state with current state to determine if a
//              reboot is required.
//      dwRebootCheck   the return value from NeedRebootInit
//
// RETURNS:
//      TRUE            if a reboot is required;
//      FALSE           otherwise.
/////////////////////////////////////////////////////////////////////////////

BOOL WINAPI NeedReboot(DWORD dwRebootCheck);


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: RebootCheckOnInstall
//
// SYNOPSIS:    Check reboot condition if the given INF section is installed.
//      hwnd    windows handle
//      pszINF  INF filename with fully qualified path
//      pszSec  INF section.  NULL is translated as DefaultInstall or DefaultInstall.NT.
//      dwReserved Not used.
// RETURN:
//      S_OK    Reboot needed if INF section is installed.
//      S_FALSE Reboot is not needed if INF section is installed.
//      HRESULT of Win 32 errors
//
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI RebootCheckOnInstallA(HWND hwnd, LPCSTR pszINF, LPCSTR pszSec, DWORD dwReserved);
HRESULT WINAPI RebootCheckOnInstallW(HWND hwnd, LPCWSTR pszINF, LPCWSTR pszSec, DWORD dwReserved);
#ifdef UNICODE
#define RebootCheckOnInstall  RebootCheckOnInstallW
#else
#define RebootCheckOnInstall  RebootCheckOnInstallA
#endif // !UNICODE
#else
HRESULT WINAPI RebootCheckOnInstall(HWND hwnd, LPCSTR pszINF, LPCSTR pszSec, DWORD dwReserved);
#endif  // (_WIN32_IE >= 0x0605)

//////////////////////////////////////////////////////////////////////////
// ENTRY POINT: TranslateInfString
//
// SYNOPSIS:    Translates a key value in an INF file, using advanced INF
//              syntax.
// RETURN CODES:
//      S_OK                                 Everything OK.
//      HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER)
//                                      The buffer size is too small to hold the
//                                      translated string.  Required size is in *pdwRequiredSize.
//      E_INVALIDARG                         NULL specified in pszInfFilename, pszTranslateSection,
//                                      pszTranslateKey, pdwRequiredSize.
//      HRESULT_FROM_WIN32(ERROR_OLD_WIN_VERSION)
//                                      OS not supported.
//      E_UNEXPECTED                         Catastrophic failure -- should never happen.
//      HRESULT_FROM_WIN32(ERROR_INVALID_PARAMETER)
//                                      The section or key specified does not exist.
//      HRESULT_FROM_WIN32(GetLastError())   Anything else
//
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI TranslateInfStringA(
    _In_ LPCSTR pszInfFilename,
    _In_ LPCSTR pszInstallSection,
    _In_ LPCSTR pszTranslateSection,
    _In_ LPCSTR pszTranslateKey,
    _Out_writes_opt_(cchBuffer) LPSTR pszBuffer,
    _In_ DWORD cchBuffer,
    _Out_ PDWORD pdwRequiredSize,
    _Reserved_ PVOID pvReserved
    );
HRESULT WINAPI TranslateInfStringW(
    _In_ LPCWSTR pszInfFilename,
    _In_ LPCWSTR pszInstallSection,
    _In_ LPCWSTR pszTranslateSection,
    _In_ LPCWSTR pszTranslateKey,
    _Out_writes_opt_(cchBuffer) LPWSTR pszBuffer,
    _In_ DWORD cchBuffer,
    _Out_ PDWORD pdwRequiredSize,
    _Reserved_ PVOID pvReserved
    );
#ifdef UNICODE
#define TranslateInfString  TranslateInfStringW
#else
#define TranslateInfString  TranslateInfStringA
#endif // !UNICODE
#else
HRESULT WINAPI TranslateInfString(
    _In_ LPCSTR pszInfFilename,
    _In_ LPCSTR pszInstallSection,
    _In_ LPCSTR pszTranslateSection,
    _In_ LPCSTR pszTranslateKey,
    _Out_writes_opt_(cchBuffer) LPSTR pszBuffer,
    _In_ DWORD cchBuffer,
    _Out_ PDWORD pdwRequiredSize,
    _Reserved_ PVOID pvReserved
    );
#endif  // (_WIN32_IE >= 0x0605)

/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: RegInstall
//
// SYNOPSIS:    Loads an INF from a string resource, adds some entries to the
//              INF string substitution table, and executes the INF.
// RETURNS:
//      S_OK    success.
//      E_FAIL  failure,
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
typedef struct _StrEntryA {
    LPSTR   pszName;        // String to substitute
    LPSTR   pszValue;       // Replacement string or string resource
} STRENTRYA, *LPSTRENTRYA;
typedef struct _StrEntryW {
    LPWSTR  pszName;        // String to substitute
    LPWSTR  pszValue;       // Replacement string or string resource
} STRENTRYW, *LPSTRENTRYW;
#ifdef UNICODE
typedef STRENTRYW STRENTRY;
typedef LPSTRENTRYW LPSTRENTRY;
#else
typedef STRENTRYA STRENTRY;
typedef LPSTRENTRYA LPSTRENTRY;
#endif // UNICODE

typedef struct _StrTableA {
    DWORD       cEntries;   // Number of entries in the table
    STRENTRYA*   pse;       // Array of entries
} STRTABLEA, *LPSTRTABLEA;
typedef struct _StrTableW {
    DWORD       cEntries;   // Number of entries in the table
    STRENTRYW*   pse;       // Array of entries
} STRTABLEW, *LPSTRTABLEW;
#ifdef UNICODE
typedef STRTABLEW STRTABLE;
typedef LPSTRTABLEW LPSTRTABLE;
#else
typedef STRTABLEA STRTABLE;
typedef LPSTRTABLEA LPSTRTABLE;
#endif // UNICODE

typedef const STRTABLEA CSTRTABLEA;
typedef const STRTABLEW CSTRTABLEW;
#ifdef UNICODE
typedef STRTABLEW STRTABLE;
typedef CSTRTABLEW CSTRTABLE;
#else
typedef STRTABLEA STRTABLE;
typedef CSTRTABLEA CSTRTABLE;
#endif // UNICODE
typedef const STRTABLEA *LPCSTRTABLEA;
typedef const STRTABLEW *LPCSTRTABLEW;
#ifdef UNICODE
typedef STRTABLEW STRTABLE;
typedef LPCSTRTABLEW LPCSTRTABLE;
#else
typedef STRTABLEA STRTABLE;
typedef LPCSTRTABLEA LPCSTRTABLE;
#endif // UNICODE

HRESULT WINAPI RegInstallA(HMODULE hmod, LPCSTR pszSection, const STRTABLEA* pstTable);
HRESULT WINAPI RegInstallW(HMODULE hmod, LPCWSTR pszSection, const STRTABLEW* pstTable);
#ifdef UNICODE
#define RegInstall  RegInstallW
#else
#define RegInstall  RegInstallA
#endif // !UNICODE

#else

typedef struct _StrEntryA {
    LPSTR pszName;          // String to substitute
    LPSTR pszValue;         // Replacement string or string resource
} STRENTRYA, *LPSTRENTRYA;

typedef struct _StrTableA {
    DWORD       cEntries;   // Number of entries in the table
    STRENTRYA*  pse;        // Array of entries
} STRTABLEA, *LPSTRTABLEA;

typedef STRENTRYA STRENTRY;
typedef LPSTRENTRYA LPSTRENTRY;

typedef STRTABLEA STRTABLE;
typedef const STRTABLEA CSTRTABLE;
typedef const STRTABLEA *LPCSTRTABLEA;

HRESULT WINAPI RegInstall(HMODULE hmod, LPCSTR pszSection, const STRTABLEA* pstTable);
#endif  // (_WIN32_IE >= 0x0605)

//
// For people whos minds are too addled to use DELAYLOAD (or UNICODE for that matter)
//
#define REGINSTALL  REGINSTALLA
typedef HRESULT (WINAPI *REGINSTALLA)(
    HMODULE hm,                         // Module that contains REGINST resource
    LPCSTR pszSection,                  // Section of INF to execute
    LPCSTRTABLEA pstTable               // Additional string substitutions
);

/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: LaunchINFSectionEx
//
// SYNOPSIS:    Install INF section with BACKUP/ROLLBACK capabilities.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI LaunchINFSectionExA(
    _In_opt_ HWND hwnd,
    _In_opt_ HINSTANCE hInstance,
    _In_ LPSTR pszParms,
    _In_ INT nShow
    );
HRESULT WINAPI LaunchINFSectionExW(
    _In_opt_ HWND hwnd,
    _In_opt_ HINSTANCE hInstance,
    _In_ LPWSTR pszParms,
    _In_ INT nShow
    );
#ifdef UNICODE
#define LaunchINFSectionEx  LaunchINFSectionExW
#else
#define LaunchINFSectionEx  LaunchINFSectionExA
#endif // !UNICODE
#else
HRESULT WINAPI LaunchINFSectionEx(
    _In_opt_ HWND hwnd,
    _In_opt_ HINSTANCE hInstance,
    _In_ LPSTR pszParms,
    _In_ INT nShow
    );
#endif  // (_WIN32_IE >= 0x0605)


// FLAGS: - need to stay this way is for compatibility. Don't change them (but ok to add new ones).
//
#define ALINF_QUIET              4      // quiet mode, no UI
#define ALINF_NGCONV             8      // don't run groupConv
#define ALINF_UPDHLPDLLS         16     // force to self-updating on user's system
#define ALINF_BKINSTALL          32     // backup data before install
#define ALINF_ROLLBACK           64     // rollback to previous state
#define ALINF_CHECKBKDATA        128    // validate the backup data
#define ALINF_ROLLBKDOALL        256    // bypass building file list
#define ALINF_DELAYREGISTEROCX   512    // force delay of ocx registration


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: ExecuteCab
//
// SYNOPSIS:    Extract the an INF from the CAB file, and do INF install on it.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
typedef struct _CabInfoA {
    LPSTR   pszCab;
    LPSTR   pszInf;
    LPSTR   pszSection;
    CHAR    szSrcPath[MAX_PATH];
    DWORD   dwFlags;
} CABINFOA, *PCABINFOA;
typedef struct _CabInfoW {
    LPWSTR  pszCab;
    LPWSTR  pszInf;
    LPWSTR  pszSection;
    WCHAR   szSrcPath[MAX_PATH];
    DWORD   dwFlags;
} CABINFOW, *PCABINFOW;
#ifdef UNICODE
typedef CABINFOW CABINFO;
typedef PCABINFOW PCABINFO;
#else
typedef CABINFOA CABINFO;
typedef PCABINFOA PCABINFO;
#endif // UNICODE

HRESULT WINAPI ExecuteCabA(HWND hwnd, CABINFOA* pCab, LPVOID pReserved);
HRESULT WINAPI ExecuteCabW(HWND hwnd, CABINFOW* pCab, LPVOID pReserved);
#ifdef UNICODE
#define ExecuteCab  ExecuteCabW
#else
#define ExecuteCab  ExecuteCabA
#endif // !UNICODE

#else

typedef struct _CabInfoA {
    LPSTR   pszCab;
    LPSTR   pszInf;
    LPSTR   pszSection;
    CHAR    szSrcPath[MAX_PATH];
    DWORD   dwFlags;
} CABINFOA, *PCABINFOA;

typedef CABINFOA CABINFO;
typedef PCABINFOA PCABINFO;

HRESULT WINAPI ExecuteCab(HWND hwnd, CABINFOA* pCab, LPVOID pReserved);
#endif  // (_WIN32_IE >= 0x0605)


// flag as LaunchINFSectionEx's flag defines

/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: AdvInstallFile
//
// SYNOPSIS:    To copy a file from the source to a destination
//              Basicly a wrapper around the setupapi file copy engine
/////////////////////////////////////////////////////////////////////////////

// Flags which can be passed to AdvInstallFile
//
// Here is a copy of the flags defined in setupapi.h for reference below.
//      #define COPYFLG_WARN_IF_SKIP            0x00000001   // warn if user tries to skip file
//      #define COPYFLG_NOSKIP                  0x00000002   // disallow skipping this file
//      #define COPYFLG_NOVERSIONCHECK          0x00000004   // ignore versions and overwrite target
//      #define COPYFLG_FORCE_FILE_IN_USE       0x00000008   // force file-in-use behavior
//      #define COPYFLG_NO_OVERWRITE            0x00000010   // do not copy if file exists on target
//      #define COPYFLG_NO_VERSION_DIALOG       0x00000020   // do not copy if target is newer
//      #define COPYFLG_REPLACEONLY             0x00000400   // copy only if file exists on target

#define AIF_WARNIFSKIP          0x00000001              // system critical file: warn if user tries to skip
#define AIF_NOSKIP              0x00000002              // Skip is disallowed for this file
#define AIF_NOVERSIONCHECK      0x00000004              // don't check the version number of the file overwrite
#define AIF_FORCE_FILE_IN_USE   0x00000008              // force file-in-use behavior
#define AIF_NOOVERWRITE         0x00000010              // copy only if target doesn't exist
                                                        // if AIF_QUIET, the file is not copied and
                                                        // the user is not notified
#define AIF_NO_VERSION_DIALOG   0x00000020              // do not copy if target is newer
#define AIF_REPLACEONLY         0x00000400              // copy only if target file already present

// Flags only known to AdvInstallFile
#define AIF_NOLANGUAGECHECK     0x10000000              // don't check the language of the file
                                                        // if the flags is NOT specified and AIF_QUIET
                                                        // the file is not copied and the user is not notified
#define AIF_QUIET               0x20000000              // No UI to the user

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI AdvInstallFileA(HWND hwnd, LPCSTR lpszSourceDir, LPCSTR lpszSourceFile, LPCSTR lpszDestDir, LPCSTR lpszDestFile, DWORD dwFlags, DWORD dwReserved);
HRESULT WINAPI AdvInstallFileW(HWND hwnd, LPCWSTR lpszSourceDir, LPCWSTR lpszSourceFile, LPCWSTR lpszDestDir, LPCWSTR lpszDestFile, DWORD dwFlags, DWORD dwReserved);
#ifdef UNICODE
#define AdvInstallFile  AdvInstallFileW
#else
#define AdvInstallFile  AdvInstallFileA
#endif // !UNICODE
#else
HRESULT WINAPI AdvInstallFile(HWND hwnd, LPCSTR lpszSourceDir, LPCSTR lpszSourceFile, LPCSTR lpszDestDir, LPCSTR lpszDestFile, DWORD dwFlags, DWORD dwReserved);
#endif  // (_WIN32_IE >= 0x0605)

//////////////////////////////////////////////////////////////////
// the following flags are for backwards compatiable.  No API user
// should reference them directly now.
//
#define  IE4_RESTORE        0x00000001      // if this bit is off, save the registries.
#define  IE4_BACKNEW        0x00000002      // backup all files which are not backed up before
#define  IE4_NODELETENEW    0x00000004      // don't delete files we don't backed up before
#define  IE4_NOMESSAGES     0x00000008      // No message display in any events.
#define  IE4_NOPROGRESS     0x00000010      // this bit on: No file backup progressbar
#define  IE4_NOENUMKEY      0x00000020      // this bit on: Don't Enum sub key even there is no given valuename
#define  IE4_NO_CRC_MAPPING 0x00000040      // Normally you should not turn on this bit, advpack creates
                                            // internal mapping for all the entries backed up.
#define  IE4_REGSECTION     0x00000080      // INF AddReg/DelReg section
#define  IE4_FRDOALL        0x00000100      // FileRestore DoAll
#define  IE4_UPDREFCNT      0x00000200      // Update the ref count in .ini backup file list
#define  IE4_USEREFCNT      0x00000400      // use ref count to determin if the backup file should be put back
#define  IE4_EXTRAINCREFCNT 0x00000800      // if increase the ref cnt if it has been updated before

#define  IE4_REMOVREGBKDATA 0x00001000      // This bit should be used with restore bit


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: RegSaveRestore
//
// SYNOPSIS:    Save or Restore the given register value or given INF reg section.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI RegSaveRestoreA(HWND hWnd, LPCSTR pszTitleString, HKEY hkBckupKey, LPCSTR pcszRootKey, LPCSTR pcszSubKey, LPCSTR pcszValueName, DWORD dwFlags);
HRESULT WINAPI RegSaveRestoreW(HWND hWnd, LPCWSTR pszTitleString, HKEY hkBckupKey, LPCWSTR pcszRootKey, LPCWSTR pcszSubKey, LPCWSTR pcszValueName, DWORD dwFlags);
#ifdef UNICODE
#define RegSaveRestore  RegSaveRestoreW
#else
#define RegSaveRestore  RegSaveRestoreA
#endif // !UNICODE
#else
HRESULT WINAPI RegSaveRestore(HWND hWnd, LPCSTR pszTitleString, HKEY hkBckupKey, LPCSTR pcszRootKey, LPCSTR pcszSubKey, LPCSTR pcszValueName, DWORD dwFlags);
#endif   // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: RegSaveRestoreOnINF
//
// SYNOPSIS:    Save or Restore the given INF Reg Section. At restore, if INF and Section pointers are NULL,
//              restore all from the given backup key handle.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI RegSaveRestoreOnINFA(HWND hWnd, LPCSTR pszTitle, LPCSTR pszINF, LPCSTR pszSection, HKEY hHKLMBackKey, HKEY hHKCUBackKey, DWORD dwFlags);
HRESULT WINAPI RegSaveRestoreOnINFW(HWND hWnd, LPCWSTR pszTitle, LPCWSTR pszINF, LPCWSTR pszSection, HKEY hHKLMBackKey, HKEY hHKCUBackKey, DWORD dwFlags);
#ifdef UNICODE
#define RegSaveRestoreOnINF  RegSaveRestoreOnINFW
#else
#define RegSaveRestoreOnINF  RegSaveRestoreOnINFA
#endif // !UNICODE
#else
HRESULT WINAPI RegSaveRestoreOnINF(HWND hWnd, LPCSTR pszTitle, LPCSTR pszINF, LPCSTR pszSection, HKEY hHKLMBackKey, HKEY hHKCUBackKey, DWORD dwFlags);
#endif  // (_WIN32_IE >= 0x0605)


// FLAGS:
#define ARSR_RESTORE    IE4_RESTORE       // if this bit is off, means Save. Otherwise, restore.
#define ARSR_NOMESSAGES IE4_NOMESSAGES    // Quiet no messages in any event.
#define ARSR_REGSECTION IE4_REGSECTION    // if this bit is off, the given section is GenInstall Section
#define ARSR_REMOVREGBKDATA IE4_REMOVREGBKDATA // if both this bit and restore bit on, remove the backup reg data without restore it

// Turn on the logging by add these RegVale in HKLM\software\microsoft\IE4
#define  REG_SAVE_LOG_KEY       TEXT("RegSaveLogFile")
#define  REG_RESTORE_LOG_KEY    TEXT("RegRestoreLogFile")


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: RegRestoreAll
//
// SYNOPSIS:    
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI RegRestoreAllA(
    _In_opt_ HWND hWnd,
    _In_opt_ LPCSTR pszTitleString,
    _In_ HKEY hkBckupKey
    );
HRESULT WINAPI RegRestoreAllW(
    _In_opt_ HWND hWnd,
    _In_opt_ LPCWSTR pszTitleString,
    _In_ HKEY hkBckupKey
    );
#ifdef UNICODE
#define RegRestoreAll  RegRestoreAllW
#else
#define RegRestoreAll  RegRestoreAllA
#endif // !UNICODE
#else
HRESULT WINAPI RegRestoreAll(
    _In_opt_ HWND hWnd,
    _In_opt_ LPCSTR pszTitleString,
    _In_ HKEY hkBckupKey
    );
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: FileSaveRestore
//
// SYNOPSIS:    Save or Restore the files on the list lpFileList.
//              If lpFileList is NULL at restore time, the function will restore
//              all based on INI index file.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI FileSaveRestoreA(
    HWND hDlg,
    _In_opt_ LPSTR lpFileList,
    LPCSTR lpDir,
    LPCSTR lpBaseName,
    DWORD dwFlags);
HRESULT WINAPI FileSaveRestoreW(
    HWND hDlg,
    _In_opt_ LPWSTR lpFileList,
    LPCWSTR lpDir,
    LPCWSTR lpBaseName,
    DWORD dwFlags);
#ifdef UNICODE
#define FileSaveRestore  FileSaveRestoreW
#else
#define FileSaveRestore  FileSaveRestoreA
#endif // !UNICODE
#else
HRESULT WINAPI FileSaveRestore(
    HWND hDlg, 
    _In_opt_ LPSTR lpFileList,
    LPCSTR lpDir,
    LPCSTR lpBaseName,
    DWORD dwFlags);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: FileSaveRestoreOnINF
//
// SYNOPSIS:    
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI FileSaveRestoreOnINFA(HWND hWnd, LPCSTR pszTitle, LPCSTR pszINF, LPCSTR pszSection, LPCSTR pszBackupDir, LPCSTR pszBaseBackupFile, DWORD dwFlags);
HRESULT WINAPI FileSaveRestoreOnINFW(HWND hWnd, LPCWSTR pszTitle, LPCWSTR pszINF, LPCWSTR pszSection, LPCWSTR pszBackupDir, LPCWSTR pszBaseBackupFile, DWORD dwFlags);
#ifdef UNICODE
#define FileSaveRestoreOnINF  FileSaveRestoreOnINFW
#else
#define FileSaveRestoreOnINF  FileSaveRestoreOnINFA
#endif // !UNICODE
#else
HRESULT WINAPI FileSaveRestoreOnINF(HWND hWnd, LPCSTR pszTitle, LPCSTR pszINF, LPCSTR pszSection, LPCSTR pszBackupDir, LPCSTR pszBaseBackupFile, DWORD dwFlags);
#endif  // (_WIN32_IE >= 0x0605)

// FLAGS:
#define  AFSR_RESTORE        IE4_RESTORE      // if this bit is off, save the file.
#define  AFSR_BACKNEW        IE4_BACKNEW      // backup all files which are not backed up before
#define  AFSR_NODELETENEW    IE4_NODELETENEW  // don't delete files we don't backed up before
#define  AFSR_NOMESSAGES     IE4_NOMESSAGES   // No message display in any events.
#define  AFSR_NOPROGRESS     IE4_NOPROGRESS   // this bit on: No file backup progressbar
#define  AFSR_UPDREFCNT      IE4_UPDREFCNT    // update the reference count for the files
#define  AFSR_USEREFCNT      IE4_USEREFCNT    // use the ref count to guide the restore file
#define  AFSR_EXTRAINCREFCNT IE4_EXTRAINCREFCNT


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: AddDelBackupEntry
//
// SYNOPSIS:    If AADBE_ADD_ENTRY is specified, mark the file in the File list as not existing
//              during file save in the INI file.  This can be used to mark additional files that
//              they did not exist during backup to avoid having them backup the next time the
//              FileSaveRestore is called to save files.
//              If AADBE_DEL_ENTRY is specified, delete the entry from the INI.  This mechanism can
//              be used to leave files permanently on the system.
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI AddDelBackupEntryA(_In_opt_ LPCSTR lpcszFileList, _In_opt_ LPCSTR lpcszBackupDir, _In_opt_ LPCSTR lpcszBaseName, DWORD dwFlags);
HRESULT WINAPI AddDelBackupEntryW(_In_opt_ LPCWSTR lpcszFileList, _In_opt_ LPCWSTR lpcszBackupDir, _In_opt_ LPCWSTR lpcszBaseName, DWORD dwFlags);
#ifdef UNICODE
#define AddDelBackupEntry  AddDelBackupEntryW
#else
#define AddDelBackupEntry  AddDelBackupEntryA
#endif // !UNICODE
#else
HRESULT WINAPI AddDelBackupEntry(_In_opt_ LPCSTR lpcszFileList, _In_opt_ LPCSTR lpcszBackupDir, _In_opt_ LPCSTR lpcszBaseName, DWORD dwFlags);
#endif  // (_WIN32_IE >= 0x0605)

#define  AADBE_ADD_ENTRY    0x01            // add entries to the INI file
#define  AADBE_DEL_ENTRY    0x02            // delete entries from the INI file

/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: FileSaveMarkNotExist
//
// SYNOPSIS:    Mark the file in the File list as not existing during file save in the INI file
//              This can be used to mark additional files that they did not exist during backup
//              to avoid having them backup the next time the FileSaveRestore is called to save
//              files
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI FileSaveMarkNotExistA(
    _In_opt_ LPCSTR lpFileList,
    _In_opt_ LPCSTR lpDir,
    _In_opt_ LPCSTR lpBaseName);
HRESULT WINAPI FileSaveMarkNotExistW(
    _In_opt_ LPCWSTR lpFileList,
    _In_opt_ LPCWSTR lpDir,
    _In_opt_ LPCWSTR lpBaseName);
#ifdef UNICODE
#define FileSaveMarkNotExist  FileSaveMarkNotExistW
#else
#define FileSaveMarkNotExist  FileSaveMarkNotExistA
#endif // !UNICODE
#else
HRESULT WINAPI FileSaveMarkNotExist(
    _In_opt_ LPCSTR lpFileList,
    _In_opt_ LPCSTR lpDir,
    _In_opt_ LPCSTR lpBaseName);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: GetVersionFromFile
//
// SYNOPSIS:    Get the given file's version and lang information.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI GetVersionFromFileA(LPCSTR lpszFilename, _Out_ LPDWORD pdwMSVer, _Out_ LPDWORD pdwLSVer, BOOL bVersion);
HRESULT WINAPI GetVersionFromFileW(LPCWSTR lpszFilename, _Out_ LPDWORD pdwMSVer, _Out_ LPDWORD pdwLSVer, BOOL bVersion);
#ifdef UNICODE
#define GetVersionFromFile  GetVersionFromFileW
#else
#define GetVersionFromFile  GetVersionFromFileA
#endif // !UNICODE
#else
HRESULT WINAPI GetVersionFromFile(LPCSTR lpszFilename, _Out_ LPDWORD pdwMSVer, _Out_ LPDWORD pdwLSVer, BOOL bVersion);
#endif // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: GetVersionFromFileEx
//
// SYNOPSIS:    Get the given disk file's version and lang information.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI GetVersionFromFileExA(LPCSTR lpszFilename, _Out_ LPDWORD pdwMSVer, _Out_ LPDWORD pdwLSVer, BOOL bVersion);
HRESULT WINAPI GetVersionFromFileExW(LPCWSTR lpszFilename, _Out_ LPDWORD pdwMSVer, _Out_ LPDWORD pdwLSVer, BOOL bVersion);
#ifdef UNICODE
#define GetVersionFromFileEx  GetVersionFromFileExW
#else
#define GetVersionFromFileEx  GetVersionFromFileExA
#endif // !UNICODE
#else
HRESULT WINAPI GetVersionFromFileEx(LPCSTR lpszFilename, _Out_ LPDWORD pdwMSVer, _Out_ LPDWORD pdwLSVer, BOOL bVersion);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: IsNTAdmin
//
// SYNOPSIS:    On NT, check if user has admin right.
//
// RETURNS:     TURE  has admin right; FLSE  no admin right.
/////////////////////////////////////////////////////////////////////////////

BOOL WINAPI IsNTAdmin(DWORD dwReserved, DWORD *lpdwReserved);


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: DelNode
//
// SYNOPSIS:    Deletes a file or directory
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI DelNodeA(LPCSTR pszFileOrDirName, DWORD dwFlags);
HRESULT WINAPI DelNodeW(LPCWSTR pszFileOrDirName, DWORD dwFlags);
#ifdef UNICODE
#define DelNode  DelNodeW
#else
#define DelNode  DelNodeA
#endif // !UNICODE
#else
HRESULT WINAPI DelNode(LPCSTR pszFileOrDirName, DWORD dwFlags);
#endif  // (_WIN32_IE >= 0x0605)

// FLAGS:
#define ADN_DEL_IF_EMPTY        0x00000001  // delete the directory only if it's empty
#define ADN_DONT_DEL_SUBDIRS    0x00000002  // don't delete any sub-dirs; delete only the files
#define ADN_DONT_DEL_DIR        0x00000004  // don't delete the dir itself
#define ADN_DEL_UNC_PATHS       0x00000008  // delete UNC paths


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: DelNodeRunDLL32
//
// SYNOPSIS:    Deletes a file or directory; the parameters to this API are of
//              WinMain type
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI DelNodeRunDLL32A(HWND hwnd, HINSTANCE hInstance, _Inout_ LPSTR pszParms, INT nShow);
HRESULT WINAPI DelNodeRunDLL32W(HWND hwnd, HINSTANCE hInstance, _Inout_ LPWSTR pszParms, INT nShow);
#ifdef UNICODE
#define DelNodeRunDLL32  DelNodeRunDLL32W
#else
#define DelNodeRunDLL32  DelNodeRunDLL32A
#endif // !UNICODE
#else
HRESULT WINAPI DelNodeRunDLL32(HWND hwnd, HINSTANCE hInstance, _Inout_ LPSTR pszParms, INT nShow);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: OpenINFEngine, TranslateINFStringEx, CloseINFEngine
//
// SYNOPSIS:    Three APIs give the caller the option to be more efficient when need
//              Advpack to translate INF file in a continue fashion.
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI OpenINFEngineA(LPCSTR pszInfFilename, LPCSTR pszInstallSection, DWORD dwFlags, HINF* phInf, PVOID pvReserved);
HRESULT WINAPI OpenINFEngineW(LPCWSTR pszInfFilename, LPCWSTR pszInstallSection, DWORD dwFlags, HINF* phInf, PVOID pvReserved);
#ifdef UNICODE
#define OpenINFEngine  OpenINFEngineW
#else
#define OpenINFEngine  OpenINFEngineA
#endif // !UNICODE
#else
HRESULT WINAPI OpenINFEngine(LPCSTR pszInfFilename, LPCSTR pszInstallSection, DWORD dwFlags, HINF* phInf, PVOID pvReserved);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: OpenINFEngine, TranslateINFStringEx, CloseINFEngine
//
// SYNOPSIS:    
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI TranslateInfStringExA(HINF hInf, LPCSTR pszInfFilename, LPCSTR pszTranslateSection, LPCSTR pszTranslateKey, _Inout_updates_to_(dwBufferSize, *pdwRequiredSize) LPSTR pszBuffer, DWORD dwBufferSize, _Out_ PDWORD pdwRequiredSize, _Reserved_ PVOID pvReserved);
HRESULT WINAPI TranslateInfStringExW(HINF hInf, LPCWSTR pszInfFilename, LPCWSTR pszTranslateSection, LPCWSTR pszTranslateKey, _Inout_updates_to_(dwBufferSize, *pdwRequiredSize) LPWSTR pszBuffer, DWORD dwBufferSize, _Out_ PDWORD pdwRequiredSize, _Reserved_ PVOID pvReserved);
#ifdef UNICODE
#define TranslateInfStringEx  TranslateInfStringExW
#else
#define TranslateInfStringEx  TranslateInfStringExA
#endif // !UNICODE
#else
HRESULT WINAPI TranslateInfStringEx(HINF hInf, LPCSTR pszInfFilename, LPCSTR pszTranslateSection, LPCSTR pszTranslateKey, _Inout_updates_to_(dwBufferSize, *pdwRequiredSize) LPSTR pszBuffer, DWORD dwBufferSize, PDWORD pdwRequiredSize, PVOID pvReserved);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: OpenINFEngine, TranslateINFStringEx, CloseINFEngine
//
// SYNOPSIS:    
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

HRESULT WINAPI CloseINFEngine(HINF hInf);


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: OpenINFEngine, TranslateINFStringEx, CloseINFEngine
//
// SYNOPSIS:    
//
// RETURNS:
//      S_OK    success
//      E_FAIL  failure
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI ExtractFilesA(LPCSTR pszCabName, LPCSTR pszExpandDir, DWORD dwFlags, LPCSTR pszFileList, LPVOID lpReserved, DWORD dwReserved);
HRESULT WINAPI ExtractFilesW(LPCWSTR pszCabName, LPCWSTR pszExpandDir, DWORD dwFlags, LPCWSTR pszFileList, LPVOID lpReserved, DWORD dwReserved);
#ifdef UNICODE
#define ExtractFiles  ExtractFilesW
#else
#define ExtractFiles  ExtractFilesA
#endif // !UNICODE
#else
//@[comment("MVI_tracked")]
HRESULT WINAPI ExtractFiles(LPCSTR pszCabName, LPCSTR pszExpandDir, DWORD dwFlags, LPCSTR pszFileList, LPVOID lpReserved, DWORD dwReserved);
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: LaunchINFSection
//
// SYNOPSIS:    Install INF section WITHOUT BACKUP/ROLLBACK capabilities.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
INT     WINAPI LaunchINFSectionA(HWND hwndOwner, HINSTANCE hInstance, _Inout_ LPSTR pszParams, INT nShow);
INT     WINAPI LaunchINFSectionW(HWND hwndOwner, HINSTANCE hInstance, _Inout_ LPWSTR pszParams, INT nShow);
#ifdef UNICODE
#define LaunchINFSection  LaunchINFSectionW
#else
#define LaunchINFSection  LaunchINFSectionA
#endif // !UNICODE
#else
INT     WINAPI LaunchINFSection(HWND hwndOwner, HINSTANCE hInstance, _Inout_ LPSTR pszParams, INT nShow);
#endif  // (_WIN32_IE >= 0x0605)

// LaunchINFSection flags
#define LIS_QUIET               0x0001      // Bit 0
#define LIS_NOGRPCONV           0x0002      // Bit 1

// Flags in Advanced INF RunPreSetupCommands and RunPostSetupCommands of the Install section
// Those flags can tell advpack how to run those commands, quiet or not quiet, wait or not wait.
// The Default for runing those commands are:  Not Quiet and Wait for finish before return the caller.
// I.E>  RunPostSetupCommands = MyCmdsSecA:1, MyCmdsSecB:2, MyCmdsSecC
#define RUNCMDS_QUIET       0x00000001
#define RUNCMDS_NOWAIT      0x00000002
#define RUNCMDS_DELAYPOSTCMD    0x00000004

/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: UserStubWrapper
//
// SYNOPSIS:    The function wrapper around the real per-user restore stub to 
//              do some generic/intelligent function on behalf of every component.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI UserInstStubWrapperA(HWND hwnd, HINSTANCE hInstance, LPCSTR pszParms, INT nShow); 
HRESULT WINAPI UserInstStubWrapperW(HWND hwnd, HINSTANCE hInstance, LPCWSTR pszParms, INT nShow); 
#ifdef UNICODE
#define UserInstStubWrapper  UserInstStubWrapperW
#else
#define UserInstStubWrapper  UserInstStubWrapperA
#endif // !UNICODE
#else
HRESULT WINAPI UserInstStubWrapper(HWND hwnd, HINSTANCE hInstance, LPCSTR pszParms, INT nShow); 
#endif  // (_WIN32_IE >= 0x0605)


// ENTRY POINT: UserUnInstStubWrapper
//
// SYNOPSIS:    The function wrapper around the real per-user restore stub to 
//              do some generic/intelligent function on behalf of every component.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
HRESULT WINAPI UserUnInstStubWrapperA(HWND hwnd, HINSTANCE hInstance, LPCSTR pszParms, INT nShow); 
HRESULT WINAPI UserUnInstStubWrapperW(HWND hwnd, HINSTANCE hInstance, LPCWSTR pszParms, INT nShow); 
#ifdef UNICODE
#define UserUnInstStubWrapper  UserUnInstStubWrapperW
#else
#define UserUnInstStubWrapper  UserUnInstStubWrapperA
#endif // !UNICODE
#else
HRESULT WINAPI UserUnInstStubWrapper(HWND hwnd, HINSTANCE hInstance, LPCSTR pszParms, INT nShow); 
#endif  // (_WIN32_IE >= 0x0605)


/////////////////////////////////////////////////////////////////////////////
// ENTRY POINT: SetPerUserInstValues
//
// SYNOPSIS:    The function set the per-user stub reg values under IsInstalled\{GUID} 
//              related key to ensure the later per-user process correctly.
//
// RETURNS:     E_FAIL on failure, S_OK on success.
/////////////////////////////////////////////////////////////////////////////

#if (_WIN32_IE >= 0x0605)
typedef struct _PERUSERSECTIONA {
    CHAR   szGUID[MAX_GUID_STRING_LEN + 20];
    CHAR   szDispName[128];
    CHAR   szLocale[10];
    CHAR   szStub[MAX_PATH*4];
    CHAR   szVersion[32];
    CHAR   szCompID[128]; 
    DWORD dwIsInstalled;
    BOOL  bRollback;
} PERUSERSECTIONA, *PPERUSERSECTIONA;
typedef struct _PERUSERSECTIONW {
    WCHAR  szGUID[MAX_GUID_STRING_LEN + 20];
    WCHAR  szDispName[128];
    WCHAR  szLocale[10];
    WCHAR  szStub[MAX_PATH*4];
    WCHAR  szVersion[32];
    WCHAR  szCompID[128]; 
    DWORD dwIsInstalled;
    BOOL  bRollback;
} PERUSERSECTIONW, *PPERUSERSECTIONW;
#ifdef UNICODE
typedef PERUSERSECTIONW PERUSERSECTION;
typedef PPERUSERSECTIONW PPERUSERSECTION;
#else
typedef PERUSERSECTIONA PERUSERSECTION;
typedef PPERUSERSECTIONA PPERUSERSECTION;
#endif // UNICODE

HRESULT WINAPI SetPerUserSecValuesA(PERUSERSECTIONA* pPerUser);
HRESULT WINAPI SetPerUserSecValuesW(PERUSERSECTIONW* pPerUser);
#ifdef UNICODE
#define SetPerUserSecValues  SetPerUserSecValuesW
#else
#define SetPerUserSecValues  SetPerUserSecValuesA
#endif // !UNICODE

#else

typedef struct _PERUSERSECTIONA {
    CHAR   szGUID[MAX_GUID_STRING_LEN + 20];
    CHAR   szDispName[128];
    CHAR   szLocale[10];
    CHAR   szStub[MAX_PATH*4];
    CHAR   szVersion[32];
    CHAR   szCompID[128];
    DWORD dwIsInstalled;
    BOOL  bRollback;
} PERUSERSECTIONA, *PPERUSERSECTIONA;

typedef PERUSERSECTIONA PERUSERSECTION;
typedef PPERUSERSECTIONA PPERUSERSECTION;

HRESULT WINAPI SetPerUserSecValues(PERUSERSECTIONA* pPerUser);
#endif  // (_WIN32_IE >= 0x0605)


#ifdef __cplusplus
}
#endif /* __cplusplus */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _ADVPUB_H_

