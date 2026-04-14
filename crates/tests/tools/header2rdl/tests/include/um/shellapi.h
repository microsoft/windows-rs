#include <winapifamily.h>

/*****************************************************************************\
*                                                                             *
* shellapi.h -  SHELL.DLL functions, types, and definitions                   *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(disable:4820) /* padding added after data member */
#pragma once
#endif

#ifndef _INC_SHELLAPI
#define _INC_SHELLAPI

#include <SpecStrings.h>

//
// Define API decoration for direct importing of DLL references.
//
#ifndef WINSHELLAPI
#if !defined(_SHELL32_)
#define WINSHELLAPI       DECLSPEC_IMPORT
#else
#define WINSHELLAPI
#endif
#endif // WINSHELLAPI

#ifndef SHSTDAPI
#if !defined(_SHELL32_)
#define SHSTDAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define SHSTDAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#else
#define SHSTDAPI          STDAPI
#define SHSTDAPI_(type)   STDAPI_(type)
#endif
#endif // SHSTDAPI

#ifndef SHDOCAPI
#if !defined(_SHDOCVW_)
#define SHDOCAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define SHDOCAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#else
#define SHDOCAPI          STDAPI
#define SHDOCAPI_(type)   STDAPI_(type)
#endif
#endif // SHDOCAPI


#if !defined(_WIN64)
#include <pshpack1.h>
#endif

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */


#pragma region Desktop Family or Gaming Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

SHSTDAPI_(LPWSTR *)  CommandLineToArgvW(_In_ LPCWSTR lpCmdLine, _Out_ int* pNumArgs);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



DECLARE_HANDLE(HDROP);

_Success_(return != 0)
SHSTDAPI_(UINT) DragQueryFileA(_In_ HDROP hDrop, _In_ UINT iFile, _Out_writes_opt_(cch) LPSTR lpszFile, _In_ UINT cch);
_Success_(return != 0)
SHSTDAPI_(UINT) DragQueryFileW(_In_ HDROP hDrop, _In_ UINT iFile, _Out_writes_opt_(cch) LPWSTR lpszFile, _In_ UINT cch);
#ifdef UNICODE
#define DragQueryFile  DragQueryFileW
#else
#define DragQueryFile  DragQueryFileA
#endif // !UNICODE
SHSTDAPI_(BOOL) DragQueryPoint(_In_ HDROP hDrop, _Out_ POINT *ppt);
SHSTDAPI_(void) DragFinish(_In_ HDROP hDrop);
SHSTDAPI_(void) DragAcceptFiles(_In_ HWND hWnd, _In_ BOOL fAccept);

SHSTDAPI_(HINSTANCE) ShellExecuteA(_In_opt_ HWND hwnd, _In_opt_ LPCSTR lpOperation, _In_ LPCSTR lpFile, _In_opt_ LPCSTR lpParameters,
    _In_opt_ LPCSTR lpDirectory, _In_ INT nShowCmd);
SHSTDAPI_(HINSTANCE) ShellExecuteW(_In_opt_ HWND hwnd, _In_opt_ LPCWSTR lpOperation, _In_ LPCWSTR lpFile, _In_opt_ LPCWSTR lpParameters,
    _In_opt_ LPCWSTR lpDirectory, _In_ INT nShowCmd);
#ifdef UNICODE
#define ShellExecute  ShellExecuteW
#else
#define ShellExecute  ShellExecuteA
#endif // !UNICODE
_Success_(return > 32) // SE_ERR_DLLNOTFOUND
SHSTDAPI_(HINSTANCE) FindExecutableA(_In_ LPCSTR lpFile, _In_opt_ LPCSTR lpDirectory, _Out_writes_(MAX_PATH) LPSTR lpResult);
_Success_(return > 32) // SE_ERR_DLLNOTFOUND
SHSTDAPI_(HINSTANCE) FindExecutableW(_In_ LPCWSTR lpFile, _In_opt_ LPCWSTR lpDirectory, _Out_writes_(MAX_PATH) LPWSTR lpResult);
#ifdef UNICODE
#define FindExecutable  FindExecutableW
#else
#define FindExecutable  FindExecutableA
#endif // !UNICODE

SHSTDAPI_(INT) ShellAboutA(_In_opt_ HWND hWnd, _In_ LPCSTR szApp, _In_opt_ LPCSTR szOtherStuff, _In_opt_ HICON hIcon);
SHSTDAPI_(INT) ShellAboutW(_In_opt_ HWND hWnd, _In_ LPCWSTR szApp, _In_opt_ LPCWSTR szOtherStuff, _In_opt_ HICON hIcon);
#ifdef UNICODE
#define ShellAbout  ShellAboutW
#else
#define ShellAbout  ShellAboutA
#endif // !UNICODE
SHSTDAPI_(HICON) DuplicateIcon(_Reserved_ HINSTANCE hInst, _In_ HICON hIcon);
SHSTDAPI_(HICON) ExtractAssociatedIconA(_Reserved_ HINSTANCE hInst, _Inout_updates_(128) LPSTR pszIconPath, _Inout_ WORD *piIcon);
SHSTDAPI_(HICON) ExtractAssociatedIconW(_Reserved_ HINSTANCE hInst, _Inout_updates_(128) LPWSTR pszIconPath, _Inout_ WORD *piIcon);
#ifdef UNICODE
#define ExtractAssociatedIcon  ExtractAssociatedIconW
#else
#define ExtractAssociatedIcon  ExtractAssociatedIconA
#endif // !UNICODE
SHSTDAPI_(HICON) ExtractAssociatedIconExA(_Reserved_ HINSTANCE hInst, _Inout_updates_(128) LPSTR pszIconPath, _Inout_ WORD *piIconIndex, _Inout_ WORD *piIconId);
SHSTDAPI_(HICON) ExtractAssociatedIconExW(_Reserved_ HINSTANCE hInst, _Inout_updates_(128) LPWSTR pszIconPath, _Inout_ WORD *piIconIndex, _Inout_ WORD *piIconId);
#ifdef UNICODE
#define ExtractAssociatedIconEx  ExtractAssociatedIconExW
#else
#define ExtractAssociatedIconEx  ExtractAssociatedIconExA
#endif // !UNICODE
SHSTDAPI_(HICON) ExtractIconA(_Reserved_ HINSTANCE hInst, _In_ LPCSTR pszExeFileName, UINT nIconIndex);
SHSTDAPI_(HICON) ExtractIconW(_Reserved_ HINSTANCE hInst, _In_ LPCWSTR pszExeFileName, UINT nIconIndex);
#ifdef UNICODE
#define ExtractIcon  ExtractIconW
#else
#define ExtractIcon  ExtractIconA
#endif // !UNICODE


#if(WINVER >= 0x0400)
typedef struct _DRAGINFOA {
    UINT uSize;                 /* init with sizeof(DRAGINFO) */
    POINT pt;
    BOOL fNC;
    PZZSTR lpFileList;
    DWORD grfKeyState;
} DRAGINFOA, *LPDRAGINFOA;
typedef struct _DRAGINFOW {
    UINT uSize;                 /* init with sizeof(DRAGINFO) */
    POINT pt;
    BOOL fNC;
    PZZWSTR lpFileList;
    DWORD grfKeyState;
} DRAGINFOW, *LPDRAGINFOW;
#ifdef UNICODE
typedef DRAGINFOW DRAGINFO;
typedef LPDRAGINFOW LPDRAGINFO;
#else
typedef DRAGINFOA DRAGINFO;
typedef LPDRAGINFOA LPDRAGINFO;
#endif


////
//// AppBar stuff
////
#define ABM_NEW           0x00000000
#define ABM_REMOVE        0x00000001
#define ABM_QUERYPOS      0x00000002
#define ABM_SETPOS        0x00000003
#define ABM_GETSTATE      0x00000004
#define ABM_GETTASKBARPOS 0x00000005
#define ABM_ACTIVATE      0x00000006  // lParam == TRUE/FALSE means activate/deactivate
#define ABM_GETAUTOHIDEBAR 0x00000007
#define ABM_SETAUTOHIDEBAR 0x00000008  // this can fail at any time.  MUST check the result
                                        // lParam = TRUE/FALSE  Set/Unset
                                        // uEdge = what edge
#define ABM_WINDOWPOSCHANGED 0x0000009
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define ABM_SETSTATE         0x0000000a
#endif // (NTDDI_VERSION >= NTDDI_WINXP)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define ABM_GETAUTOHIDEBAREX    0x0000000b // multimon aware autohide bars
#define ABM_SETAUTOHIDEBAREX    0x0000000c
#endif


// these are put in the wparam of callback messages
#define ABN_STATECHANGE    0x0000000
#define ABN_POSCHANGED     0x0000001
#define ABN_FULLSCREENAPP  0x0000002
#define ABN_WINDOWARRANGE  0x0000003 // lParam == TRUE means hide


// flags for get state
#define ABS_AUTOHIDE    0x0000001
#define ABS_ALWAYSONTOP 0x0000002

#define ABE_LEFT        0
#define ABE_TOP         1
#define ABE_RIGHT       2
#define ABE_BOTTOM      3

typedef struct _AppBarData
{
    DWORD cbSize;
    HWND hWnd;
    UINT uCallbackMessage;
    UINT uEdge;
    RECT rc;
    LPARAM lParam; // message specific
} APPBARDATA, *PAPPBARDATA;


SHSTDAPI_(UINT_PTR) SHAppBarMessage(_In_ DWORD dwMessage, _Inout_ PAPPBARDATA pData);

////
////  EndAppBar
////

SHSTDAPI_(DWORD)   DoEnvironmentSubstA(_Inout_updates_(cchSrc) LPSTR pszSrc, UINT cchSrc);
SHSTDAPI_(DWORD)   DoEnvironmentSubstW(_Inout_updates_(cchSrc) LPWSTR pszSrc, UINT cchSrc);
#ifdef UNICODE
#define DoEnvironmentSubst  DoEnvironmentSubstW
#else
#define DoEnvironmentSubst  DoEnvironmentSubstA
#endif // !UNICODE

#define EIRESID(x) (-1 * (int)(x))
SHSTDAPI_(UINT) ExtractIconExA(_In_ LPCSTR lpszFile, int nIconIndex, _Out_writes_opt_(nIcons) HICON *phiconLarge, _Out_writes_opt_(nIcons) HICON *phiconSmall, UINT nIcons);
SHSTDAPI_(UINT) ExtractIconExW(_In_ LPCWSTR lpszFile, int nIconIndex, _Out_writes_opt_(nIcons) HICON *phiconLarge, _Out_writes_opt_(nIcons) HICON *phiconSmall, UINT nIcons);
#ifdef UNICODE
#define ExtractIconEx  ExtractIconExW
#else
#define ExtractIconEx  ExtractIconExA
#endif // !UNICODE

// Shell File Operations

#define FO_MOVE                    0x0001
#define FO_COPY                    0x0002
#define FO_DELETE                  0x0003
#define FO_RENAME                  0x0004

// SHFILEOPSTRUCT.fFlags and IFileOperation::SetOperationFlags() flag values

#define FOF_MULTIDESTFILES         0x0001
#define FOF_CONFIRMMOUSE           0x0002
#define FOF_SILENT                 0x0004  // don't display progress UI (confirm prompts may be displayed still)
#define FOF_RENAMEONCOLLISION      0x0008  // automatically rename the source files to avoid the collisions
#define FOF_NOCONFIRMATION         0x0010  // don't display confirmation UI, assume "yes" for cases that can be bypassed, "no" for those that can not
#define FOF_WANTMAPPINGHANDLE      0x0020  // Fill in SHFILEOPSTRUCT.hNameMappings
                                           // Must be freed using SHFreeNameMappings
#define FOF_ALLOWUNDO              0x0040  // enable undo including Recycle behavior for IFileOperation::Delete()
#define FOF_FILESONLY              0x0080  // only operate on the files (non folders), both files and folders are assumed without this
#define FOF_SIMPLEPROGRESS         0x0100  // means don't show names of files
#define FOF_NOCONFIRMMKDIR         0x0200  // don't dispplay confirmatino UI before making any needed directories, assume "Yes" in these cases
#define FOF_NOERRORUI              0x0400  // don't put up error UI, other UI may be displayed, progress, confirmations
#define FOF_NOCOPYSECURITYATTRIBS  0x0800  // dont copy file security attributes (ACLs)
#define FOF_NORECURSION            0x1000  // don't recurse into directories for operations that would recurse
#define FOF_NO_CONNECTED_ELEMENTS  0x2000  // don't operate on connected elements ("xxx_files" folders that go with .htm files)
#define FOF_WANTNUKEWARNING        0x4000  // during delete operation, warn if object is being permanently destroyed instead of recycling (partially overrides FOF_NOCONFIRMATION)
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define FOF_NORECURSEREPARSE       0x8000  // deprecated; the operations engine always does the right thing on FolderLink objects (symlinks, reparse points, folder shortcuts)
#endif // (NTDDI_VERSION >= NTDDI_WINXP)
#define FOF_NO_UI                   (FOF_SILENT | FOF_NOCONFIRMATION | FOF_NOERRORUI | FOF_NOCONFIRMMKDIR) // don't display any UI at all

typedef WORD FILEOP_FLAGS;

#define PO_DELETE       0x0013  // printer is being deleted
#define PO_RENAME       0x0014  // printer is being renamed
#define PO_PORTCHANGE   0x0020  // port this printer connected to is being changed
                                // if this id is set, the strings received by
                                // the copyhook are a doubly-null terminated
                                // list of strings.  The first is the printer
                                // name and the second is the printer port.
#define PO_REN_PORT     0x0034  // PO_RENAME and PO_PORTCHANGE at same time.

// no POF_ flags currently defined

typedef WORD PRINTEROP_FLAGS;

// implicit parameters are:
//      if pFrom or pTo are unqualified names the current directories are
//      taken from the global current drive/directory settings managed
//      by Get/SetCurrentDrive/Directory
//
//      the global confirmation settings

typedef struct _SHFILEOPSTRUCTA
{
    HWND            hwnd;
    UINT            wFunc;
    PCZZSTR         pFrom;
    PCZZSTR         pTo;
    FILEOP_FLAGS    fFlags;
    BOOL            fAnyOperationsAborted;
    LPVOID          hNameMappings;
    PCSTR           lpszProgressTitle; // only used if FOF_SIMPLEPROGRESS
} SHFILEOPSTRUCTA, *LPSHFILEOPSTRUCTA;
typedef struct _SHFILEOPSTRUCTW
{
    HWND            hwnd;
    UINT            wFunc;
    PCZZWSTR        pFrom;
    PCZZWSTR        pTo;
    FILEOP_FLAGS    fFlags;
    BOOL            fAnyOperationsAborted;
    LPVOID          hNameMappings;
    PCWSTR          lpszProgressTitle; // only used if FOF_SIMPLEPROGRESS
} SHFILEOPSTRUCTW, *LPSHFILEOPSTRUCTW;
#ifdef UNICODE
typedef SHFILEOPSTRUCTW SHFILEOPSTRUCT;
typedef LPSHFILEOPSTRUCTW LPSHFILEOPSTRUCT;
#else
typedef SHFILEOPSTRUCTA SHFILEOPSTRUCT;
typedef LPSHFILEOPSTRUCTA LPSHFILEOPSTRUCT;
#endif

SHSTDAPI_(int) SHFileOperationA(_Inout_ LPSHFILEOPSTRUCTA lpFileOp);
SHSTDAPI_(int) SHFileOperationW(_Inout_ LPSHFILEOPSTRUCTW lpFileOp);
#ifdef UNICODE
#define SHFileOperation  SHFileOperationW
#else
#define SHFileOperation  SHFileOperationA
#endif // !UNICODE
SHSTDAPI_(void) SHFreeNameMappings(_In_opt_ HANDLE hNameMappings);

typedef struct _SHNAMEMAPPINGA
{
    LPSTR   pszOldPath;
    LPSTR   pszNewPath;
    int   cchOldPath;
    int   cchNewPath;
} SHNAMEMAPPINGA, *LPSHNAMEMAPPINGA;
typedef struct _SHNAMEMAPPINGW
{
    LPWSTR  pszOldPath;
    LPWSTR  pszNewPath;
    int   cchOldPath;
    int   cchNewPath;
} SHNAMEMAPPINGW, *LPSHNAMEMAPPINGW;
#ifdef UNICODE
typedef SHNAMEMAPPINGW SHNAMEMAPPING;
typedef LPSHNAMEMAPPINGW LPSHNAMEMAPPING;
#else
typedef SHNAMEMAPPINGA SHNAMEMAPPING;
typedef LPSHNAMEMAPPINGA LPSHNAMEMAPPING;
#endif // UNICODE


////
//// End Shell File Operations
////

////
////  Begin ShellExecuteEx and family
////

/* ShellExecute() and ShellExecuteEx() error codes */

/* regular WinExec() codes */
#define SE_ERR_FNF              2       // file not found
#define SE_ERR_PNF              3       // path not found
#define SE_ERR_ACCESSDENIED     5       // access denied
#define SE_ERR_OOM              8       // out of memory
#define SE_ERR_DLLNOTFOUND              32

#endif /* WINVER >= 0x0400 */

/* error values for ShellExecute() beyond the regular WinExec() codes */
#define SE_ERR_SHARE                    26
#define SE_ERR_ASSOCINCOMPLETE          27
#define SE_ERR_DDETIMEOUT               28
#define SE_ERR_DDEFAIL                  29
#define SE_ERR_DDEBUSY                  30
#define SE_ERR_NOASSOC                  31

#if(WINVER >= 0x0400)
// Note CLASSKEY overrides CLASSNAME
#define SEE_MASK_DEFAULT           0x00000000
#define SEE_MASK_CLASSNAME         0x00000001   // SHELLEXECUTEINFO.lpClass is valid
#define SEE_MASK_CLASSKEY          0x00000003   // SHELLEXECUTEINFO.hkeyClass is valid
// Note SEE_MASK_INVOKEIDLIST(0xC) implies SEE_MASK_IDLIST(0x04)
#define SEE_MASK_IDLIST            0x00000004   // SHELLEXECUTEINFO.lpIDList is valid
#define SEE_MASK_INVOKEIDLIST      0x0000000c   // enable IContextMenu based verbs
#if (NTDDI_VERSION < NTDDI_VISTA)
#define SEE_MASK_ICON              0x00000010   // not used
#endif // (NTDDI_VERSION < NTDDI_VISTA)
#define SEE_MASK_HOTKEY            0x00000020   // SHELLEXECUTEINFO.dwHotKey is valid
#define SEE_MASK_NOCLOSEPROCESS    0x00000040   // SHELLEXECUTEINFO.hProcess
#define SEE_MASK_CONNECTNETDRV     0x00000080   // enables re-connecting disconnected network drives
#define SEE_MASK_NOASYNC           0x00000100   // block on the call until the invoke has completed, use for callers that exit after calling ShellExecuteEx()
#define SEE_MASK_FLAG_DDEWAIT      SEE_MASK_NOASYNC // Use SEE_MASK_NOASYNC instead of SEE_MASK_FLAG_DDEWAIT as it more accuratly describes the behavior
#define SEE_MASK_DOENVSUBST        0x00000200   // indicates that SHELLEXECUTEINFO.lpFile contains env vars that should be expanded
#define SEE_MASK_FLAG_NO_UI        0x00000400   // disable UI including error messages
#define SEE_MASK_UNICODE           0x00004000
#define SEE_MASK_NO_CONSOLE        0x00008000
#define SEE_MASK_ASYNCOK           0x00100000
#if (NTDDI_VERSION >= NTDDI_WIN2K)
#define SEE_MASK_HMONITOR          0x00200000   // SHELLEXECUTEINFO.hMonitor
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)
#if (NTDDI_VERSION >= NTDDI_WINXPSP1)
#define SEE_MASK_NOZONECHECKS      0x00800000
#endif // (NTDDI_VERSION >= NTDDI_WINXPSP1)
#if (NTDDI_VERSION >= NTDDI_WIN2K)
#define SEE_MASK_NOQUERYCLASSSTORE 0x01000000
#define SEE_MASK_WAITFORINPUTIDLE  0x02000000
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define SEE_MASK_FLAG_LOG_USAGE    0x04000000
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
// When SEE_MASK_FLAG_HINST_IS_SITE is specified SHELLEXECUTEINFO.hInstApp is used as an
// _In_ parameter and specifies a IUnknown* to be used as a site pointer. The site pointer
// is used to provide services to shell execute, the handler binding process and the verb handlers
// once they are invoked.
#define SEE_MASK_FLAG_HINST_IS_SITE    0x08000000
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
// For compilers that don't support nameless unions
//
#ifndef DUMMYUNIONNAME
#ifdef NONAMELESSUNION
#define DUMMYUNIONNAME   u
#define DUMMYUNIONNAME2  u2
#define DUMMYUNIONNAME3  u3
#define DUMMYUNIONNAME4  u4
#define DUMMYUNIONNAME5  u5
#else
#define DUMMYUNIONNAME
#define DUMMYUNIONNAME2
#define DUMMYUNIONNAME3
#define DUMMYUNIONNAME4
#define DUMMYUNIONNAME5
#endif
#endif // DUMMYUNIONNAME

typedef struct _SHELLEXECUTEINFOA
{
    DWORD cbSize;               // in, required, sizeof of this structure
    ULONG fMask;                // in, SEE_MASK_XXX values
    HWND hwnd;                  // in, optional
    LPCSTR   lpVerb;            // in, optional when unspecified the default verb is choosen
    LPCSTR   lpFile;            // in, either this value or lpIDList must be specified
    LPCSTR   lpParameters;      // in, optional
    LPCSTR   lpDirectory;       // in, optional
    int nShow;                  // in, required
    HINSTANCE hInstApp;         // out when SEE_MASK_NOCLOSEPROCESS is specified
    void *lpIDList;             // in, valid when SEE_MASK_IDLIST is specified, PCIDLIST_ABSOLUTE, for use with SEE_MASK_IDLIST & SEE_MASK_INVOKEIDLIST
    LPCSTR   lpClass;           // in, valid when SEE_MASK_CLASSNAME is specified
    HKEY hkeyClass;             // in, valid when SEE_MASK_CLASSKEY is specified
    DWORD dwHotKey;             // in, valid when SEE_MASK_HOTKEY is specified
    union
    {
        HANDLE hIcon;           // not used
#if (NTDDI_VERSION >= NTDDI_WIN2K)
        HANDLE hMonitor;        // in, valid when SEE_MASK_HMONITOR specified
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)
    } DUMMYUNIONNAME;
    HANDLE hProcess;            // out, valid when SEE_MASK_NOCLOSEPROCESS specified
} SHELLEXECUTEINFOA, *LPSHELLEXECUTEINFOA;
typedef struct _SHELLEXECUTEINFOW
{
    DWORD cbSize;               // in, required, sizeof of this structure
    ULONG fMask;                // in, SEE_MASK_XXX values
    HWND hwnd;                  // in, optional
    LPCWSTR  lpVerb;            // in, optional when unspecified the default verb is choosen
    LPCWSTR  lpFile;            // in, either this value or lpIDList must be specified
    LPCWSTR  lpParameters;      // in, optional
    LPCWSTR  lpDirectory;       // in, optional
    int nShow;                  // in, required
    HINSTANCE hInstApp;         // out when SEE_MASK_NOCLOSEPROCESS is specified
    void *lpIDList;             // in, valid when SEE_MASK_IDLIST is specified, PCIDLIST_ABSOLUTE, for use with SEE_MASK_IDLIST & SEE_MASK_INVOKEIDLIST
    LPCWSTR  lpClass;           // in, valid when SEE_MASK_CLASSNAME is specified
    HKEY hkeyClass;             // in, valid when SEE_MASK_CLASSKEY is specified
    DWORD dwHotKey;             // in, valid when SEE_MASK_HOTKEY is specified
    union
    {
        HANDLE hIcon;           // not used
#if (NTDDI_VERSION >= NTDDI_WIN2K)
        HANDLE hMonitor;        // in, valid when SEE_MASK_HMONITOR specified
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)
    } DUMMYUNIONNAME;
    HANDLE hProcess;            // out, valid when SEE_MASK_NOCLOSEPROCESS specified
} SHELLEXECUTEINFOW, *LPSHELLEXECUTEINFOW;
#ifdef UNICODE
typedef SHELLEXECUTEINFOW SHELLEXECUTEINFO;
typedef LPSHELLEXECUTEINFOW LPSHELLEXECUTEINFO;
#else
typedef SHELLEXECUTEINFOA SHELLEXECUTEINFO;
typedef LPSHELLEXECUTEINFOA LPSHELLEXECUTEINFO;
#endif // UNICODE

SHSTDAPI_(BOOL) ShellExecuteExA(_Inout_ SHELLEXECUTEINFOA *pExecInfo);
SHSTDAPI_(BOOL) ShellExecuteExW(_Inout_ SHELLEXECUTEINFOW *pExecInfo);
#ifdef UNICODE
#define ShellExecuteEx  ShellExecuteExW
#else
#define ShellExecuteEx  ShellExecuteExA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WIN2K)
// deprecated, no longer implemented
typedef struct _SHCREATEPROCESSINFOW
{
    DWORD cbSize;
    ULONG fMask;
    HWND hwnd;
    LPCWSTR  pszFile;
    LPCWSTR  pszParameters;
    LPCWSTR  pszCurrentDirectory;
    HANDLE hUserToken;
    LPSECURITY_ATTRIBUTES lpProcessAttributes;
    LPSECURITY_ATTRIBUTES lpThreadAttributes;
    BOOL bInheritHandles;
    DWORD dwCreationFlags;
    LPSTARTUPINFOW lpStartupInfo;
    LPPROCESS_INFORMATION lpProcessInformation;
} SHCREATEPROCESSINFOW, *PSHCREATEPROCESSINFOW;

SHSTDAPI_(BOOL) SHCreateProcessAsUserW(_Inout_ PSHCREATEPROCESSINFOW pscpi);
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)

#if (NTDDI_VERSION >= NTDDI_VISTA)

SHSTDAPI SHEvaluateSystemCommandTemplate(_In_ PCWSTR pszCmdTemplate, _Outptr_ PWSTR *ppszApplication, _Outptr_opt_ PWSTR *ppszCommandLine, _Outptr_opt_ PWSTR *ppszParameters);
//
//  SHEvaluateSystemCommandTemplate()
//      *   enforces stricter validation before calling CreateProcess().  may also be
//              used before calling ShellExecute().
//      *   should be used when caller wants the deterministic behavior from a command template
//              regardless of execution context.  it ignores the current process state,
//              such as the %PATH%, GetCurrentDirectory(), and parent process directory.
//      *   should be used when the command is hardcoded.
//      *   is used by ShellExecute() when handling file associations from HKCR.
//      *   reduces CreateProcess() commandline exploits
//      *   is not designed for processing user input, and may generate unexpected failures.
//
//  INPUT:
//      pszCmdTemplate =    command line, this may or may not include parameters.
//                          if the parameters are substitution parameters then this API
//                          should be called before parameters have been replaced.
//                          (check the examples below to see sample supported inputs.)
//
//  OUTPUT on return: S_OK
//      ppszApplication =   verified path to the Application.  this should be passed as the lpApplication
//                          parameter to CreateProcess() or the lpFile parameter to ShellExecute().
//                          (allocated using CoTaskMemAlloc(), free with CoTaskMemFree())
//
//      ppszCommandLine =   OPTIONAL - use if planning to call CreateProcess().
//                          resulting command line template.  parameters should be replaced based on this template,
//                          and then passed as the lpCommandLine parameter to CreateProcess().
//                          it is guaranteed to be of a form that PathGetArgs() will always succeed correctly.
//                          (allocated using CoTaskMemAlloc(), free with CoTaskMemFree())
//
//      ppszParameters  =   OPTIONAL - use if planning to call ShellExecute().
//                          resulting parameter list template.  parameters should be replaced based on this template,
//                          and then passed as the lpParameters parameter to ShellExecute().
//                          NOTE: identical to PathGetArgs(*ppszCommandLine).
//                          (allocated using CoTaskMemAlloc(), free with CoTaskMemFree())
//
//  OUTPUT on return: FAILED()
//      all outputs will be NULL'ed on failure
//
//  NOTES:  the parsing logic to determine a valid Application path is non-trivial, although
//              the extension is not required and if missing will be completed
//              in the following standard order:  { .PIF, .COM, .EXE, .BAT, .CMD }
//
//      Relative Paths are System Paths - if the first token has no path qualifiers
//              then the token is first checked to see if a key of the same name has
//              been installed under HKLM\Software\Microsoft\Windows\CurrentVersion\App Paths.
//              if the key or default value does not exist, it is assumed to be a child
//              of the system directories.  the following directories will be searched
//              in order for the relative token: { CSIDL_SYSTEM, CSIDL_WINDOWS }
//
//      Prefer Quoted Paths - if the first token in pszCmdTemplate is quoted and appears
//              to be an absolute path then the token is the only possible result.
//
//      Limit Forms of Unquoted Paths - if the first token is unquoted and appears
//              to be an absolute path, then it is subject to more stringent limitations.
//              if the token is a substring of CSIDL_PROGRAM_FILES or does not
//              exist on the file system, then SHEvaluateSystemCommandTemplate() will
//              attempt to complete using a token delimited by the first space of the
//              last valid path segment (usually the file name).  if this token also doesnt exist,
//              then the next space will be used, etc.
//
//  USAGE:      used before calling into CreateProcess() or ShellExecute(), callers
//              would typically look like the following:
/* #if 0 // SAMPLE CODE
HRESULT MyCreateProcessPriv(_In_ PCWSTR pszCmd)
{
    PWSTR pszApp;
    PWSTR pszCmdLine;
    HRESULT hr = SHEvaluateSystemCommandTemplate(pszCmd, &pszApp, &pszCmdLine);
    if (SUCCEEDED(hr))
    {
        //  if this was a real template, maybe some kind of wnsprintf() first?
        PROCESS_INFORMATION pi;
        STARTUPINFO si = {0};
        si.cb = sizeof(startup);
        si.wShowWindow = SW_SHOWNORMAL;

        if (CreateProcess(pszApp, pszCmdLine, NULL, NULL, FALSE,
                 CREATE_DEFAULT_ERROR_MODE, NULL, NULL, &si, &pi))
        {
            //  we are good
            ASSERT(hr == S_OK);
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        }
        else
        {
            hr = HRESULT_FROM_WIN32(GetLastError());
        }

        CoTaskMemFree(pszApp);
        CoTaskMemFree(pszCmdLine);
    }
    return hr;
}

HRESULT MyShellExec(_In_ PCWSTR pszCmd)
{
    PWSTR pszApp;
    PWSTR pszCmdLine;
    HRESULT hr = SHEvaluateSystemCommandTemplate(pszCmd, &pszApp, &pszCmdLine);
    if (SUCCEEDED(hr))
    {
        //  if this was a real template, maybe some kind of wnsprintf() first?
        SHELLEXECUTEINFOW sei = {
            sizeof(sei),           // cbSize;
            0,                     // fMask
            NULL,                  // hwnd
            NULL,                  // lpVerb
            pszApp,                // lpFile
            PathGetArgs(pszCmdLine), // lpParameters
            NULL,                  // lpDirectory
            SW_SHOWNORMAL,         // nShow
            0,                     // hInstApp
            NULL,                  // lpIDList
            NULL,                  // lpClass
            NULL,                  // hkeyClass
            0,                     // dwHotKey
            NULL,                  // hIcon
            NULL                   // hProcess
        };

        if (ShellExecuteEx(&sei))
        {
            //  we are good
            ASSERT(hr == S_OK);
        }
        else
        {
            hr = HRESULT_FROM_WIN32(GetLastError());
        }

        CoTaskMemFree(pszApp);
        CoTaskMemFree(pszCmdLine);
    }
    return hr;
}
#endif //  0 // SAMPLE CODE
*/

//  EXAMPLE:   Each example will show an input parameter and the results returned by
//              SHEvaluateSystemCommandTemplate().  Also included is the alternate result
//              of what CreateProcess() would have created if pszCmdTemplate were
//              passed directly as lpCommandLine and lpApplication were NULL.
//              (results marked with an asterisk (*) indicate differences.)
//
//          Assume for the examples that the following paths and values exist:
//
//      SHGetFolderPath() values:
//          CSIDL_SYSTEM            =   C:\windows\system32
//          CSIDL_WINDOWS           =   C:\windows
//          CSIDL_PROGRAM_FILES     =   C:\Program Files
//
//      Environment settings
//          GetModuleFileName(NULL) =   C:\Program Files\Example\sample.exe
//          GetCurrentDirectory()   =   \\server\share\foo
//          HKLM\...\App Paths\pbrush.exe = C:\windows\system32\mspaint.exe
//          HKLM\...\App Paths\mycl.exe = C:\Program Files\Compilers\mycl.exe
//          PATH                    =   c:\windows\system32;C:\windows;c:\;C:\Program Files\Compilers\
//
//      Valid Application paths:
//          C:\Program Files\Internet Explorer\iexplore.exe
//          C:\windows\system32\rundll32.exe
//          C:\windows\system32\notepad.exe
//          C:\windows\notepad.exe
//          C:\Program Files\Example\sample.exe
//          C:\Program Files\Compilers\cl.exe
//          C:\Other Programs\prog.exe
//
//      Suspicious (possibly hostile) Application paths:
//          C:\Program.exe
//          C:\Program Files\Internet.exe
//          C:\Program Files\Example\regedit.bat
//          C:\mycl.exe
//          \\server\share\foo\rundll32.exe
//          \\server\share\foo\myapp.exe
//
//
//  Relative Path Example #1
//      pszCmdTemplate      =   notepad.exe %1
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\windows\system32\notepad.exe
//              pszCommandLine  =   "notepad.exe" %1
//          CreateProcess() would return TRUE
//              new process =   C:\windows\system32\notepad.exe
//
//  Relative Path Example #2
//      pszCmdTemplate      =   rundll32.exe shell32.dll,RunDll
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\windows\system32\rundll32.exe
//              pszCommandLine  =   "rundll32.exe" shell32.dll,RunDll
//          * CreateProcess() would return TRUE
//              new process =   \\server\share\foo\rundll32.exe
//
//  Relative Path Example #3
//      pszCmdTemplate      =   regedit %1
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\windows\system32\regedit.exe
//              pszCommandLine  =   "regedit.exe" %1
//          * CreateProcess() would return TRUE
//              new process =   C:\Program Files\Example\regedit.bat
//
//  Relative Path Example #4
//      pszCmdTemplate      =   pbrush "%1"
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\windows\system32\mspaint.exe
//              pszCommandLine  =   "mspaint.exe" "%1"
//          * CreateProcess() would return FALSE
//
//  Relative Path Example #5
//      pszCmdTemplate      =   mycl "%1" "%2"
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Compilers\mycl.exe
//              pszCommandLine  =   "mycl.exe" "%1" "%2"
//          * CreateProcess() would return TRUE
//              new process =   C:\mycl.exe
//
//  Relative Path Example #6
//      pszCmdTemplate      =   myapp.exe
//          SHEvaluateSystemCommandTemplate() returns: CO_E_APPNOTFOUND
//          * CreateProcess() would return TRUE
//              new process =   \\server\share\foo\myapp.exe
//
//  Quoted Path Example #1
//      pszCmdTemplate      =   "C:\Program Files\Internet Explorer\iexplore.exe" -nohome
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Internet Explorer\iexplore.exe
//              pszCommandLine  =   "C:\Program Files\Internet Explorer\iexplore.exe" -nohome
//          CreateProcess() would return TRUE
//              new process =   C:\Program Files\Internet Explorer\iexplore.exe
//
//  Quoted Path Example #2
//      pszCmdTemplate      =   "C:\Program Files\Internet" -url
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Internet.exe
//              pszCommandLine  =   "C:\Program Files\Internet.exe" -url
//          CreateProcess() would return TRUE
//              new process =   C:\Program Files\internet.exe
//
//  Quoted Path Example #3
//      pszCmdTemplate      =   "C:\Program" -url
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program.exe
//              pszCommandLine  =   "C:\Program.exe" -url
//          CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #1
//      pszCmdTemplate      =   C:\Program Files\Internet Explorer\iexplore.exe -nohome
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Internet Explorer\iexplore.exe
//              pszCommandLine  =   "C:\Program Files\Internet Explorer\iexplore.exe" -nohome
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #2
//      pszCmdTemplate      =   C:\Program Files\Internet Explorer\iexplore.exe -url fool.htm
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Internet Explorer\iexplore.exe
//              pszCommandLine  =   "C:\Program Files\Internet Explorer\iexplore.exe" -url fool.htm
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #3
//      pszCmdTemplate      =   C:\Program Files\Internet Explorer\iexplore.exe -url C:\fool.htm
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Internet Explorer\iexplore.exe
//              pszCommandLine  =   "C:\Program Files\Internet Explorer\iexplore.exe" -url C:\fool.htm
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #4
//      pszCmdTemplate      =   C:\Program Files\Internet -url
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Program Files\Internet.exe
//              pszCommandLine  =   "C:\Program Files\Internet.exe" -url
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #5
//      pszCmdTemplate      =   C:\Other Programs\prog.exe -go %1 \fool %2
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Other Programs\prog.exe
//              pszCommandLine  =   "C:\Other Programs\prog.exe" %1 \fool %2
//          * CreateProcess() would return TRUE
//              new process =   C:\Other Programs\prog.exe
//
//  Unquoted Example #6
//      pszCmdTemplate      =   C:\Other Programs\prog.exe -go "\fool" "%1"
//          SHEvaluateSystemCommandTemplate() returns: S_OK
//              pszApplication  =   C:\Other Programs\prog.exe
//              pszCommandLine  =   "C:\Other Programs\prog.exe" -go "\fool" "%1"
//          * CreateProcess() would return TRUE
//              new process =   C:\Other Programs\prog.exe
//
//  Unquoted Example #7
//      pszCmdTemplate      =   C:\Program Files\Internet Explorer\iexplore.exe -url \fool.htm
//          SHEvaluateSystemCommandTemplate() returns: CO_E_APPNOTFOUND
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #8
//      pszCmdTemplate      =   C:\Program -url
//          SHEvaluateSystemCommandTemplate() returns: CO_E_APPNOTFOUND
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #9
//      pszCmdTemplate      =   C:\Other Programs\prog.exe -go \fool us
//          SHEvaluateSystemCommandTemplate() returns: CO_E_APPNOTFOUND
//          * CreateProcess() would return TRUE
//              new process =   C:\Other Programs\prog.exe
//
//  Unquoted Example #10
//      pszCmdTemplate      =   C:\Other Programs\prog.exe -go \fool %1
//          SHEvaluateSystemCommandTemplate() returns: CO_E_APPNOTFOUND
//          * CreateProcess() would return TRUE
//              new process =   C:\Other Programs\prog.exe
//
//  Unquoted Example #11
//      pszCmdTemplate      =   C:\Program "%1"
//          SHEvaluateSystemCommandTemplate() returns: E_ACCESSDENIED
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//
//  Unquoted Example #12
//      pszCmdTemplate      =   C:\Program
//          SHEvaluateSystemCommandTemplate() returns: E_ACCESSDENIED
//          * CreateProcess() would return TRUE
//              new process =   C:\Program.exe
//

//  used for implementing IShellFolder::GetUIObject(IID_IQueryAssociations)
//  designed for namespace extensions with registered extensible types
//  SHCreateDefaultContextMenu() and others use IQueryAssociations to build up data sets

typedef enum ASSOCCLASS
{                               //  which other members are used
    ASSOCCLASS_SHELL_KEY = 0,   //  hkeyClass
    ASSOCCLASS_PROGID_KEY,      //  hkeyClass
    ASSOCCLASS_PROGID_STR,      //  pszClass (HKCR\pszClass)
    ASSOCCLASS_CLSID_KEY,       //  hkeyClass
    ASSOCCLASS_CLSID_STR,       //  pszClass (HKCR\CLSID\pszClass)
    ASSOCCLASS_APP_KEY,         //  hkeyClass
    ASSOCCLASS_APP_STR,         //  pszClass (HKCR\Applications\PathFindFileName(pszClass))
    ASSOCCLASS_SYSTEM_STR,      //  pszClass
    ASSOCCLASS_FOLDER,          //  none
    ASSOCCLASS_STAR,            //  none
#if (NTDDI_VERSION >= NTDDI_WIN8)
    ASSOCCLASS_FIXED_PROGID_STR,//  pszClass (HKCR\pszClass), do not apply mapping of pszClass based on user defaults
    ASSOCCLASS_PROTOCOL_STR,    //  pszClass is a protocol, apply mapping of pszClass based on user defaults
#endif
} ASSOCCLASS;

typedef struct ASSOCIATIONELEMENT
{
    ASSOCCLASS ac;              // required
    HKEY hkClass;               // may be NULL
    PCWSTR pszClass;            // may be NULL
} ASSOCIATIONELEMENT;

// the object returned from this API implements IQueryAssociations

SHSTDAPI AssocCreateForClasses(_In_reads_(cClasses) const ASSOCIATIONELEMENT *rgClasses, ULONG cClasses, _In_ REFIID riid, _COM_Outptr_ void **ppv);

/* #if 0 // SAMPLE CODE
HRESULT CCustomFolder::_AssocCreate(_In_ PCUITEMID_CHILD pidl, _In_ REFIID riid, _Outptr_ void **ppv)
{
    *ppv = nullptr;
    ASSOCIATIONELEMENT rgAssoc[] =
    {
        { ASSOCCLASS_PROGID_STR, nullptr, CCustomFolder::_MapChildToType(pidl)},
        { ASSOCCLASS_FOLDER, nullptr, nullptr},
    };
    if (CCustomFolder::_IsFolder(pidl))
    {
        return AssocCreateForClasses(rgAssoc, ARRAYSIZE(rgAssoc), riid, ppv);
    }
    else
    {
        //  skip FOLDER at the end
        return AssocCreateForClasses(rgAssoc, ARRAYSIZE(rgAssoc)-1, riid, ppv);
    }
}

HRESULT CCustomFolder::GetUIObjectOf(...)
{
    //  validate parameters
    if (riid == IID_IQueryAssociations)
    {
        hr = _AssocCreate(apidl[0], riid, ppv);
    }
    //  else if ...
}
#endif // SAMPLE CODE
*/

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

////
////  End ShellExecuteEx and family
////

#if (NTDDI_VERSION >= NTDDI_WIN2K)
//
// RecycleBin
//

// struct for query recycle bin info
typedef struct _SHQUERYRBINFO {
    DWORD   cbSize;
#if !defined(_MAC) || defined(_MAC_INT_64)
    __int64 i64Size;
    __int64 i64NumItems;
#else
    DWORDLONG i64Size;
    DWORDLONG i64NumItems;
#endif
} SHQUERYRBINFO, *LPSHQUERYRBINFO;


// flags for SHEmptyRecycleBin
//
#define SHERB_NOCONFIRMATION    0x00000001
#define SHERB_NOPROGRESSUI      0x00000002
#define SHERB_NOSOUND           0x00000004


SHSTDAPI SHQueryRecycleBinA(_In_opt_ LPCSTR pszRootPath, _Inout_ LPSHQUERYRBINFO pSHQueryRBInfo);
SHSTDAPI SHQueryRecycleBinW(_In_opt_ LPCWSTR pszRootPath, _Inout_ LPSHQUERYRBINFO pSHQueryRBInfo);
#ifdef UNICODE
#define SHQueryRecycleBin  SHQueryRecycleBinW
#else
#define SHQueryRecycleBin  SHQueryRecycleBinA
#endif // !UNICODE
SHSTDAPI SHEmptyRecycleBinA(_In_opt_ HWND hwnd, _In_opt_ LPCSTR pszRootPath, DWORD dwFlags);
SHSTDAPI SHEmptyRecycleBinW(_In_opt_ HWND hwnd, _In_opt_ LPCWSTR pszRootPath, DWORD dwFlags);
#ifdef UNICODE
#define SHEmptyRecycleBin  SHEmptyRecycleBinW
#else
#define SHEmptyRecycleBin  SHEmptyRecycleBinA
#endif // !UNICODE

////
//// end of RecycleBin
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)


////
//// Taskbar notification definitions
////

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef enum
{
    QUNS_NOT_PRESENT               = 1,    // The user is not present.  Heuristic check for modes like: screen saver, locked machine, non-active FUS session
    QUNS_BUSY                      = 2,    // The user is busy.  Heuristic check for modes like: full-screen app
    QUNS_RUNNING_D3D_FULL_SCREEN   = 3,    // full-screen (exlusive-mode) D3D app
    QUNS_PRESENTATION_MODE         = 4,    // Windows presentation mode (laptop feature) is turned on
    QUNS_ACCEPTS_NOTIFICATIONS     = 5,    // notifications can be freely sent
#if (NTDDI_VERSION >= NTDDI_WIN7)
    QUNS_QUIET_TIME                = 6,    // We are in OOBE quiet period
#endif
#if (NTDDI_VERSION >= NTDDI_WIN8)
    QUNS_APP                       = 7,    // App-mode application
#endif
} QUERY_USER_NOTIFICATION_STATE;

SHSTDAPI SHQueryUserNotificationState(_Out_ QUERY_USER_NOTIFICATION_STATE *pquns);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN7)
// This api retrieves an IPropertyStore that stores the window's properties.
SHSTDAPI SHGetPropertyStoreForWindow(_In_ HWND hwnd, _In_ REFIID riid, _Outptr_ void** ppv);
#endif


typedef struct _NOTIFYICONDATAA {
    DWORD cbSize;
    HWND hWnd;
    UINT uID;
    UINT uFlags;
    UINT uCallbackMessage;
    HICON hIcon;
#if (NTDDI_VERSION < NTDDI_WIN2K)
    CHAR   szTip[64];
#endif
#if (NTDDI_VERSION >= NTDDI_WIN2K)
    CHAR   szTip[128];
    DWORD dwState;
    DWORD dwStateMask;
    CHAR   szInfo[256];
#ifndef _SHELL_EXPORTS_INTERNALAPI_H_
    union {
        UINT  uTimeout;
        UINT  uVersion;  // used with NIM_SETVERSION, values 0, 3 and 4
    } DUMMYUNIONNAME;
#endif
    CHAR   szInfoTitle[64];
    DWORD dwInfoFlags;
#endif
#if (NTDDI_VERSION >= NTDDI_WINXP)
    GUID guidItem;
#endif
#if (NTDDI_VERSION >= NTDDI_VISTA)
    HICON hBalloonIcon;
#endif
} NOTIFYICONDATAA, *PNOTIFYICONDATAA;
typedef struct _NOTIFYICONDATAW {
    DWORD cbSize;
    HWND hWnd;
    UINT uID;
    UINT uFlags;
    UINT uCallbackMessage;
    HICON hIcon;
#if (NTDDI_VERSION < NTDDI_WIN2K)
    WCHAR  szTip[64];
#endif
#if (NTDDI_VERSION >= NTDDI_WIN2K)
    WCHAR  szTip[128];
    DWORD dwState;
    DWORD dwStateMask;
    WCHAR  szInfo[256];
#ifndef _SHELL_EXPORTS_INTERNALAPI_H_
    union {
        UINT  uTimeout;
        UINT  uVersion;  // used with NIM_SETVERSION, values 0, 3 and 4
    } DUMMYUNIONNAME;
#endif
    WCHAR  szInfoTitle[64];
    DWORD dwInfoFlags;
#endif
#if (NTDDI_VERSION >= NTDDI_WINXP)
    GUID guidItem;
#endif
#if (NTDDI_VERSION >= NTDDI_VISTA)
    HICON hBalloonIcon;
#endif
} NOTIFYICONDATAW, *PNOTIFYICONDATAW;
#ifdef UNICODE
typedef NOTIFYICONDATAW NOTIFYICONDATA;
typedef PNOTIFYICONDATAW PNOTIFYICONDATA;
#else
typedef NOTIFYICONDATAA NOTIFYICONDATA;
typedef PNOTIFYICONDATAA PNOTIFYICONDATA;
#endif // UNICODE


#define NOTIFYICONDATAA_V1_SIZE     FIELD_OFFSET(NOTIFYICONDATAA, szTip[64])
#define NOTIFYICONDATAW_V1_SIZE     FIELD_OFFSET(NOTIFYICONDATAW, szTip[64])
#ifdef UNICODE
#define NOTIFYICONDATA_V1_SIZE      NOTIFYICONDATAW_V1_SIZE
#else
#define NOTIFYICONDATA_V1_SIZE      NOTIFYICONDATAA_V1_SIZE
#endif

#define NOTIFYICONDATAA_V2_SIZE     FIELD_OFFSET(NOTIFYICONDATAA, guidItem)
#define NOTIFYICONDATAW_V2_SIZE     FIELD_OFFSET(NOTIFYICONDATAW, guidItem)
#ifdef UNICODE
#define NOTIFYICONDATA_V2_SIZE      NOTIFYICONDATAW_V2_SIZE
#else
#define NOTIFYICONDATA_V2_SIZE      NOTIFYICONDATAA_V2_SIZE
#endif

#define NOTIFYICONDATAA_V3_SIZE     FIELD_OFFSET(NOTIFYICONDATAA, hBalloonIcon)
#define NOTIFYICONDATAW_V3_SIZE     FIELD_OFFSET(NOTIFYICONDATAW, hBalloonIcon)
#ifdef UNICODE
#define NOTIFYICONDATA_V3_SIZE      NOTIFYICONDATAW_V3_SIZE
#else
#define NOTIFYICONDATA_V3_SIZE      NOTIFYICONDATAA_V3_SIZE
#endif


#define NIN_SELECT          (WM_USER + 0)
#define NINF_KEY            0x1
#define NIN_KEYSELECT       (NIN_SELECT | NINF_KEY)

#define NIN_BALLOONSHOW         (WM_USER + 2)
#define NIN_BALLOONHIDE         (WM_USER + 3)
#define NIN_BALLOONTIMEOUT      (WM_USER + 4)
#define NIN_BALLOONUSERCLICK    (WM_USER + 5)
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define NIN_POPUPOPEN           (WM_USER + 6)
#define NIN_POPUPCLOSE          (WM_USER + 7)
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)


#define NIM_ADD         0x00000000
#define NIM_MODIFY      0x00000001
#define NIM_DELETE      0x00000002
#define NIM_SETFOCUS    0x00000003
#define NIM_SETVERSION  0x00000004


// set NOTIFYICONDATA.uVersion with 0, 3 or 4
// please read the documentation on the behavior difference that the different versions imply
#define     NOTIFYICON_VERSION      3
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define     NOTIFYICON_VERSION_4    4
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define NIF_MESSAGE     0x00000001
#define NIF_ICON        0x00000002
#define NIF_TIP         0x00000004
#define NIF_STATE       0x00000008
#define NIF_INFO        0x00000010
#if (_WIN32_IE >= 0x600)
#define NIF_GUID        0x00000020
#endif
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define NIF_REALTIME    0x00000040
#define NIF_SHOWTIP     0x00000080
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define NIS_HIDDEN              0x00000001
#define NIS_SHAREDICON          0x00000002

// says this is the source of a shared icon

// Notify Icon Infotip flags
#define NIIF_NONE       0x00000000
// icon flags are mutually exclusive
// and take only the lowest 2 bits
#define NIIF_INFO       0x00000001
#define NIIF_WARNING    0x00000002
#define NIIF_ERROR      0x00000003
#if (NTDDI_VERSION >= NTDDI_WINXPSP2) // also available in NTDDI_WS03SP1
#define NIIF_USER       0x00000004
#endif // (NTDDI_VERSION >= NTDDI_WINXPSP2)
#define NIIF_ICON_MASK  0x0000000F
#define NIIF_NOSOUND    0x00000010

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define NIIF_LARGE_ICON 0x00000020
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define NIIF_RESPECT_QUIET_TIME 0x00000080
#endif // (NTDDI_VERSION >= NTDDI_WIN7)


typedef struct _NOTIFYICONIDENTIFIER {
    DWORD cbSize;
    HWND hWnd;
    UINT uID;
    GUID guidItem;
} NOTIFYICONIDENTIFIER, *PNOTIFYICONIDENTIFIER;


SHSTDAPI_(BOOL) Shell_NotifyIconA(DWORD dwMessage, _In_ PNOTIFYICONDATAA lpData);
SHSTDAPI_(BOOL) Shell_NotifyIconW(DWORD dwMessage, _In_ PNOTIFYICONDATAW lpData);
#ifdef UNICODE
#define Shell_NotifyIcon  Shell_NotifyIconW
#else
#define Shell_NotifyIcon  Shell_NotifyIconA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WIN7)
SHSTDAPI Shell_NotifyIconGetRect(_In_ const NOTIFYICONIDENTIFIER* identifier, _Out_ RECT* iconLocation);
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

////
//// End Taskbar Notification Icons
////

#ifndef SHFILEINFO_DEFINED
#define SHFILEINFO_DEFINED
////
//// Begin SHGetFileInfo
////

/*
 * The SHGetFileInfo API provides an easy way to get attributes
 * for a file given a pathname.
 *
 *   PARAMETERS
 *
 *     pszPath              file name to get info about
 *     dwFileAttributes     file attribs, only used with SHGFI_USEFILEATTRIBUTES
 *     psfi                 place to return file info
 *     cbFileInfo           size of structure
 *     uFlags               flags
 *
 *   RETURN
 *     TRUE if things worked
 */

typedef struct _SHFILEINFOA
{
        HICON       hIcon;                      // out: icon
        int         iIcon;                      // out: icon index
        DWORD       dwAttributes;               // out: SFGAO_ flags
        CHAR        szDisplayName[MAX_PATH];    // out: display name (or path)
        CHAR        szTypeName[80];             // out: type name
} SHFILEINFOA;
typedef struct _SHFILEINFOW
{
        HICON       hIcon;                      // out: icon
        int         iIcon;                      // out: icon index
        DWORD       dwAttributes;               // out: SFGAO_ flags
        WCHAR       szDisplayName[MAX_PATH];    // out: display name (or path)
        WCHAR       szTypeName[80];             // out: type name
} SHFILEINFOW;
#ifdef UNICODE
typedef SHFILEINFOW SHFILEINFO;
#else
typedef SHFILEINFOA SHFILEINFO;
#endif // UNICODE


// NOTE: This is also in shlwapi.h.  Please keep in synch.
#endif // !SHFILEINFO_DEFINED

#define SHGFI_ICON              0x000000100     // get icon
#define SHGFI_DISPLAYNAME       0x000000200     // get display name
#define SHGFI_TYPENAME          0x000000400     // get type name
#define SHGFI_ATTRIBUTES        0x000000800     // get attributes
#define SHGFI_ICONLOCATION      0x000001000     // get icon location
#define SHGFI_EXETYPE           0x000002000     // return exe type
#define SHGFI_SYSICONINDEX      0x000004000     // get system icon index
#define SHGFI_LINKOVERLAY       0x000008000     // put a link overlay on icon
#define SHGFI_SELECTED          0x000010000     // show icon in selected state
#if (NTDDI_VERSION >= NTDDI_WIN2K)
#define SHGFI_ATTR_SPECIFIED    0x000020000     // get only specified attributes
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)
#define SHGFI_LARGEICON         0x000000000     // get large icon
#define SHGFI_SMALLICON         0x000000001     // get small icon
#define SHGFI_OPENICON          0x000000002     // get open icon
#define SHGFI_SHELLICONSIZE     0x000000004     // get shell size icon
#define SHGFI_PIDL              0x000000008     // pszPath is a pidl
#define SHGFI_USEFILEATTRIBUTES 0x000000010     // use passed dwFileAttribute

#define SHGFI_ADDOVERLAYS       0x000000020     // apply the appropriate overlays
#define SHGFI_OVERLAYINDEX      0x000000040     // Get the index of the overlay
                                                // in the upper 8 bits of the iIcon

SHSTDAPI_(DWORD_PTR) SHGetFileInfoA(_In_ LPCSTR pszPath, DWORD dwFileAttributes, _Inout_updates_bytes_opt_(cbFileInfo) SHFILEINFOA *psfi,
    UINT cbFileInfo, UINT uFlags);
SHSTDAPI_(DWORD_PTR) SHGetFileInfoW(_In_ LPCWSTR pszPath, DWORD dwFileAttributes, _Inout_updates_bytes_opt_(cbFileInfo) SHFILEINFOW *psfi,
    UINT cbFileInfo, UINT uFlags);
#ifdef UNICODE
#define SHGetFileInfo  SHGetFileInfoW
#else
#define SHGetFileInfo  SHGetFileInfoA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef struct _SHSTOCKICONINFO
{
    DWORD cbSize;
    HICON hIcon;
    int   iSysImageIndex;
    int   iIcon;
    WCHAR szPath[MAX_PATH];
} SHSTOCKICONINFO;

#define SHGSI_ICONLOCATION      0 // you always get the icon location
#define SHGSI_ICON              SHGFI_ICON
#define SHGSI_SYSICONINDEX      SHGFI_SYSICONINDEX
#define SHGSI_LINKOVERLAY       SHGFI_LINKOVERLAY
#define SHGSI_SELECTED          SHGFI_SELECTED
#define SHGSI_LARGEICON         SHGFI_LARGEICON
#define SHGSI_SMALLICON         SHGFI_SMALLICON
#define SHGSI_SHELLICONSIZE     SHGFI_SHELLICONSIZE

//  Shell icons


typedef enum SHSTOCKICONID
{
    SIID_DOCNOASSOC = 0,          // document (blank page), no associated program
    SIID_DOCASSOC = 1,            // document with an associated program
    SIID_APPLICATION = 2,         // generic application with no custom icon
    SIID_FOLDER = 3,              // folder (closed)
    SIID_FOLDEROPEN = 4,          // folder (open)
    SIID_DRIVE525 = 5,            // 5.25" floppy disk drive
    SIID_DRIVE35 = 6,             // 3.5" floppy disk drive
    SIID_DRIVEREMOVE = 7,         // removable drive
    SIID_DRIVEFIXED = 8,          // fixed (hard disk) drive
    SIID_DRIVENET = 9,            // network drive
    SIID_DRIVENETDISABLED = 10,   // disconnected network drive
    SIID_DRIVECD = 11,            // CD drive
    SIID_DRIVERAM = 12,           // RAM disk drive
    SIID_WORLD = 13,              // entire network
    SIID_SERVER = 15,             // a computer on the network
    SIID_PRINTER = 16,            // printer
    SIID_MYNETWORK = 17,          // My network places
    SIID_FIND = 22,               // Find
    SIID_HELP = 23,               // Help
    SIID_SHARE = 28,              // overlay for shared items
    SIID_LINK = 29,               // overlay for shortcuts to items
    SIID_SLOWFILE = 30,           // overlay for slow items
    SIID_RECYCLER = 31,           // empty recycle bin
    SIID_RECYCLERFULL = 32,       // full recycle bin
    SIID_MEDIACDAUDIO = 40,       // Audio CD Media
    SIID_LOCK = 47,               // Security lock
    SIID_AUTOLIST = 49,           // AutoList
    SIID_PRINTERNET = 50,         // Network printer
    SIID_SERVERSHARE = 51,        // Server share
    SIID_PRINTERFAX = 52,         // Fax printer
    SIID_PRINTERFAXNET = 53,      // Networked Fax Printer
    SIID_PRINTERFILE = 54,        // Print to File
    SIID_STACK = 55,              // Stack
    SIID_MEDIASVCD = 56,          // SVCD Media
    SIID_STUFFEDFOLDER = 57,      // Folder containing other items
    SIID_DRIVEUNKNOWN = 58,       // Unknown drive
    SIID_DRIVEDVD = 59,           // DVD Drive
    SIID_MEDIADVD = 60,           // DVD Media
    SIID_MEDIADVDRAM = 61,        // DVD-RAM Media
    SIID_MEDIADVDRW = 62,         // DVD-RW Media
    SIID_MEDIADVDR = 63,          // DVD-R Media
    SIID_MEDIADVDROM = 64,        // DVD-ROM Media
    SIID_MEDIACDAUDIOPLUS = 65,   // CD+ (Enhanced CD) Media
    SIID_MEDIACDRW = 66,          // CD-RW Media
    SIID_MEDIACDR = 67,           // CD-R Media
    SIID_MEDIACDBURN = 68,        // Burning CD
    SIID_MEDIABLANKCD = 69,       // Blank CD Media
    SIID_MEDIACDROM = 70,         // CD-ROM Media
    SIID_AUDIOFILES = 71,         // Audio files
    SIID_IMAGEFILES = 72,         // Image files
    SIID_VIDEOFILES = 73,         // Video files
    SIID_MIXEDFILES = 74,         // Mixed files
    SIID_FOLDERBACK = 75,         // Folder back
    SIID_FOLDERFRONT = 76,        // Folder front
    SIID_SHIELD = 77,             // Security shield. Use for UAC prompts only.
    SIID_WARNING = 78,            // Warning
    SIID_INFO = 79,               // Informational
    SIID_ERROR = 80,              // Error
    SIID_KEY = 81,                // Key / Secure
    SIID_SOFTWARE = 82,           // Software
    SIID_RENAME = 83,             // Rename
    SIID_DELETE = 84,             // Delete
    SIID_MEDIAAUDIODVD = 85,      // Audio DVD Media
    SIID_MEDIAMOVIEDVD = 86,      // Movie DVD Media
    SIID_MEDIAENHANCEDCD = 87,    // Enhanced CD Media
    SIID_MEDIAENHANCEDDVD = 88,   // Enhanced DVD Media
    SIID_MEDIAHDDVD = 89,         // HD-DVD Media
    SIID_MEDIABLURAY = 90,        // BluRay Media
    SIID_MEDIAVCD = 91,           // VCD Media
    SIID_MEDIADVDPLUSR = 92,      // DVD+R Media
    SIID_MEDIADVDPLUSRW = 93,     // DVD+RW Media
    SIID_DESKTOPPC = 94,          // desktop computer
    SIID_MOBILEPC = 95,           // mobile computer (laptop/notebook)
    SIID_USERS = 96,              // users
    SIID_MEDIASMARTMEDIA = 97,    // Smart Media
    SIID_MEDIACOMPACTFLASH = 98,  // Compact Flash
    SIID_DEVICECELLPHONE = 99,    // Cell phone
    SIID_DEVICECAMERA = 100,      // Camera
    SIID_DEVICEVIDEOCAMERA = 101, // Video camera
    SIID_DEVICEAUDIOPLAYER = 102, // Audio player
    SIID_NETWORKCONNECT = 103,    // Connect to network
    SIID_INTERNET = 104,          // Internet
    SIID_ZIPFILE = 105,           // ZIP file
    SIID_SETTINGS = 106,          // Settings
    // 107-131 are internal Vista RTM icons
    // 132-159 for SP1 icons
    SIID_DRIVEHDDVD = 132,        // HDDVD Drive (all types)
    SIID_DRIVEBD = 133,           // BluRay Drive (all types)
    SIID_MEDIAHDDVDROM = 134,     // HDDVD-ROM Media
    SIID_MEDIAHDDVDR = 135,       // HDDVD-R Media
    SIID_MEDIAHDDVDRAM = 136,     // HDDVD-RAM Media
    SIID_MEDIABDROM = 137,        // BluRay ROM Media
    SIID_MEDIABDR = 138,          // BluRay R Media
    SIID_MEDIABDRE = 139,         // BluRay RE Media (Rewriable and RAM)
    SIID_CLUSTEREDDRIVE = 140,    // Clustered disk
    // 160+ are for Windows 7 icons
    SIID_MAX_ICONS = 181,
} SHSTOCKICONID;

#define SIID_INVALID ((SHSTOCKICONID)-1)

SHSTDAPI SHGetStockIconInfo(SHSTOCKICONID siid, UINT uFlags, _Inout_ SHSTOCKICONINFO *psii);

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


#if (NTDDI_VERSION >= NTDDI_WIN2K)
#define SHGetDiskFreeSpace SHGetDiskFreeSpaceEx

SHSTDAPI_(BOOL) SHGetDiskFreeSpaceExA(_In_ LPCSTR pszDirectoryName, _Out_opt_ ULARGE_INTEGER* pulFreeBytesAvailableToCaller,
    _Out_opt_ ULARGE_INTEGER* pulTotalNumberOfBytes, _Out_opt_ ULARGE_INTEGER* pulTotalNumberOfFreeBytes);
SHSTDAPI_(BOOL) SHGetDiskFreeSpaceExW(_In_ LPCWSTR pszDirectoryName, _Out_opt_ ULARGE_INTEGER* pulFreeBytesAvailableToCaller,
    _Out_opt_ ULARGE_INTEGER* pulTotalNumberOfBytes, _Out_opt_ ULARGE_INTEGER* pulTotalNumberOfFreeBytes);
#ifdef UNICODE
#define SHGetDiskFreeSpaceEx  SHGetDiskFreeSpaceExW
#else
#define SHGetDiskFreeSpaceEx  SHGetDiskFreeSpaceExA
#endif // !UNICODE
_Success_(return != 0)
SHSTDAPI_(BOOL) SHGetNewLinkInfoA(_In_ LPCSTR pszLinkTo, _In_ LPCSTR pszDir, _Out_writes_(MAX_PATH) LPSTR pszName, _Out_ BOOL *pfMustCopy, _In_ UINT uFlags);
_Success_(return != 0)
SHSTDAPI_(BOOL) SHGetNewLinkInfoW(_In_ LPCWSTR pszLinkTo, _In_ LPCWSTR pszDir, _Out_writes_(MAX_PATH) LPWSTR pszName, _Out_ BOOL *pfMustCopy, _In_ UINT uFlags);
#ifdef UNICODE
#define SHGetNewLinkInfo  SHGetNewLinkInfoW
#else
#define SHGetNewLinkInfo  SHGetNewLinkInfoA
#endif // !UNICODE

#define SHGNLI_PIDL             0x000000001     // pszLinkTo is a pidl
#define SHGNLI_PREFIXNAME       0x000000002     // Make name "Shortcut to xxx"
#define SHGNLI_NOUNIQUE         0x000000004     // don't do the unique name generation
#define SHGNLI_NOLNK            0x000000008     // don't add ".lnk" extension
#if (_WIN32_IE >= 0x0600)
#define SHGNLI_NOLOCNAME        0x000000010     // use non localized (parsing) name from the target
#endif
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define SHGNLI_USEURLEXT        0x000000020     // use ".url" extension instead of ".lnk"
#endif
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)


#if (NTDDI_VERSION >= NTDDI_WIN2K)
#define PRINTACTION_OPEN           0        // pszBuf1:<PrinterName>
#define PRINTACTION_PROPERTIES     1        // pszBuf1:<PrinterName>, pszBuf2:optional <PageName>
#define PRINTACTION_NETINSTALL     2        // pszBuf1:<NetPrinterName>
#define PRINTACTION_NETINSTALLLINK 3        // pszBuf1:<NetPrinterName>, pszBuf2:<path to store link>
#define PRINTACTION_TESTPAGE       4        // pszBuf1:<PrinterName>
#define PRINTACTION_OPENNETPRN     5        // pszBuf1:<NetPrinterName>
#define PRINTACTION_DOCUMENTDEFAULTS 6      // pszBuf1:<PrinterName>
#define PRINTACTION_SERVERPROPERTIES 7      // pszBuf1:<Server> or <NetPrinterName>

// deprecated, instead invoke verbs on printers/netprinters using IContextMenu or ShellExecute()

SHSTDAPI_(BOOL) SHInvokePrinterCommandA(_In_opt_ HWND hwnd, UINT uAction, _In_ LPCSTR lpBuf1, _In_opt_ LPCSTR lpBuf2, BOOL fModal);
SHSTDAPI_(BOOL) SHInvokePrinterCommandW(_In_opt_ HWND hwnd, UINT uAction, _In_ LPCWSTR lpBuf1, _In_opt_ LPCWSTR lpBuf2, BOOL fModal);
#ifdef UNICODE
#define SHInvokePrinterCommand  SHInvokePrinterCommandW
#else
#define SHInvokePrinterCommand  SHInvokePrinterCommandA
#endif // !UNICODE
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef struct _OPEN_PRINTER_PROPS_INFOA
{
    DWORD       dwSize;
    LPSTR       pszSheetName;
    UINT        uSheetIndex;
    DWORD       dwFlags;
    BOOL        bModal;
} OPEN_PRINTER_PROPS_INFOA, *POPEN_PRINTER_PROPS_INFOA;
typedef struct _OPEN_PRINTER_PROPS_INFOW
{
    DWORD       dwSize;
    LPWSTR      pszSheetName;
    UINT        uSheetIndex;
    DWORD       dwFlags;
    BOOL        bModal;
} OPEN_PRINTER_PROPS_INFOW, *POPEN_PRINTER_PROPS_INFOW;
#ifdef UNICODE
typedef OPEN_PRINTER_PROPS_INFOW OPEN_PRINTER_PROPS_INFO;
typedef POPEN_PRINTER_PROPS_INFOW POPEN_PRINTER_PROPS_INFO;
#else
typedef OPEN_PRINTER_PROPS_INFOA OPEN_PRINTER_PROPS_INFO;
typedef POPEN_PRINTER_PROPS_INFOA POPEN_PRINTER_PROPS_INFO;
#endif // UNICODE
#define PRINT_PROP_FORCE_NAME   0x01
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)


#endif /* WINVER >= 0x0400 */

#if (NTDDI_VERSION >= NTDDI_WIN2K)

//
// The SHLoadNonloadedIconOverlayIdentifiers API causes the shell's
// icon overlay manager to load any registered icon overlay
// identifers that are not currently loaded.  This is useful if an
// overlay identifier did not load at shell startup but is needed
// and can be loaded at a later time.  Identifiers already loaded
// are not affected.  Overlay identifiers implement the
// IShellIconOverlayIdentifier interface.
//
// Returns:
//      S_OK
//
SHSTDAPI SHLoadNonloadedIconOverlayIdentifiers(void);

//
// The SHIsFileAvailableOffline API determines whether a file
// or folder is available for offline use.
//
// Parameters:
//     pwszPath             file name to get info about
//     pdwStatus            (optional) OFFLINE_STATUS_* flags returned here
//
// Returns:
//     S_OK                 File/directory is available offline, unless
//                            OFFLINE_STATUS_INCOMPLETE is returned.
//     E_INVALIDARG         Path is invalid, or not a net path
//     E_FAIL               File/directory is not available offline
//
// Notes:
//     OFFLINE_STATUS_INCOMPLETE is never returned for directories.
//     Both OFFLINE_STATUS_LOCAL and OFFLINE_STATUS_REMOTE may be returned,
//     indicating "open in both places." This is common when the server is online.
//
SHSTDAPI SHIsFileAvailableOffline(_In_ PCWSTR pwszPath, _Out_opt_ DWORD *pdwStatus);

#define OFFLINE_STATUS_LOCAL        0x0001  // If open, it's open locally
#define OFFLINE_STATUS_REMOTE       0x0002  // If open, it's open remotely
#define OFFLINE_STATUS_INCOMPLETE   0x0004  // The local copy is currently imcomplete.
                                            // The file will not be available offline
                                            // until it has been synchronized.

#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)
//  sets the specified path to use the string resource
//  as the UI instead of the file system name
SHSTDAPI SHSetLocalizedName(_In_ PCWSTR pszPath, _In_ PCWSTR pszResModule, int idsRes);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)
#if (NTDDI_VERSION >= NTDDI_VISTA)
//  sets the specified path to use the string resource
//  as the UI instead of the file system name
SHSTDAPI SHRemoveLocalizedName(_In_ PCWSTR pszPath);
//  gets the string resource for the specified path
SHSTDAPI SHGetLocalizedName(_In_ PCWSTR pszPath, _Out_writes_(cch) PWSTR pszResModule, UINT cch, _Out_ int *pidsRes);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//====== ShellMessageBox ================================================

// If lpcTitle is NULL, the title is taken from hWnd
// If lpcText is NULL, this is assumed to be an Out Of Memory message
// If the selector of lpcTitle or lpcText is NULL, the offset should be a
//     string resource ID
// The variable arguments must all be 32-bit values (even if fewer bits
//     are actually used)
// lpcText (or whatever string resource it causes to be loaded) should
//     be a formatting string similar to wsprintf except that only the
//     following formats are available:
//         %%              formats to a single '%'
//         %nn%s           the nn-th arg is a string which is inserted
//         %nn%ld          the nn-th arg is a DWORD, and formatted decimal
//         %nn%lx          the nn-th arg is a DWORD, and formatted hex
//     note that lengths are allowed on the %s, %ld, and %lx, just
//                         like wsprintf
//

#if !defined(_SHLWAPI_)
#define LWSTDAPIV_(type)  EXTERN_C DECLSPEC_IMPORT type STDAPIVCALLTYPE
#else
#define LWSTDAPIV_(type)  STDAPIV_(type)
#endif

LWSTDAPIV_(int) ShellMessageBoxA(
    _In_opt_ HINSTANCE hAppInst,
    _In_opt_ HWND hWnd,
    _In_ LPCSTR lpcText,
    _In_opt_ LPCSTR lpcTitle,
    _In_ UINT fuStyle, ...);
LWSTDAPIV_(int) ShellMessageBoxW(
    _In_opt_ HINSTANCE hAppInst,
    _In_opt_ HWND hWnd,
    _In_ LPCWSTR lpcText,
    _In_opt_ LPCWSTR lpcTitle,
    _In_ UINT fuStyle, ...);
#ifdef UNICODE
#define ShellMessageBox  ShellMessageBoxW
#else
#define ShellMessageBox  ShellMessageBoxA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WIN2K)
SHSTDAPI_(BOOL) IsLFNDriveA(_In_opt_ LPCSTR pszPath);
SHSTDAPI_(BOOL) IsLFNDriveW(_In_opt_ LPCWSTR pszPath);
#ifdef UNICODE
#define IsLFNDrive  IsLFNDriveW
#else
#define IsLFNDrive  IsLFNDriveA
#endif // !UNICODE
#endif // (NTDDI_VERSION >= NTDDI_WIN2K)


#if         _WIN32_IE >= 0x0600

STDAPI SHEnumerateUnreadMailAccountsA(_In_opt_ HKEY hKeyUser, DWORD dwIndex, _Out_writes_(cchMailAddress) LPSTR pszMailAddress, int cchMailAddress);
STDAPI SHEnumerateUnreadMailAccountsW(_In_opt_ HKEY hKeyUser, DWORD dwIndex, _Out_writes_(cchMailAddress) LPWSTR pszMailAddress, int cchMailAddress);
#ifdef UNICODE
#define SHEnumerateUnreadMailAccounts  SHEnumerateUnreadMailAccountsW
#else
#define SHEnumerateUnreadMailAccounts  SHEnumerateUnreadMailAccountsA
#endif // !UNICODE
STDAPI SHGetUnreadMailCountA(_In_opt_ HKEY hKeyUser, _In_opt_ LPCSTR pszMailAddress, _Out_opt_ DWORD *pdwCount, _Out_opt_ FILETIME *pFileTime, _Out_writes_opt_(cchShellExecuteCommand) LPSTR pszShellExecuteCommand, int cchShellExecuteCommand);
STDAPI SHGetUnreadMailCountW(_In_opt_ HKEY hKeyUser, _In_opt_ LPCWSTR pszMailAddress, _Out_opt_ DWORD *pdwCount, _Out_opt_ FILETIME *pFileTime, _Out_writes_opt_(cchShellExecuteCommand) LPWSTR pszShellExecuteCommand, int cchShellExecuteCommand);
#ifdef UNICODE
#define SHGetUnreadMailCount  SHGetUnreadMailCountW
#else
#define SHGetUnreadMailCount  SHGetUnreadMailCountA
#endif // !UNICODE
STDAPI SHSetUnreadMailCountA(_In_ LPCSTR pszMailAddress, DWORD dwCount, _In_ LPCSTR pszShellExecuteCommand);
STDAPI SHSetUnreadMailCountW(_In_ LPCWSTR pszMailAddress, DWORD dwCount, _In_ LPCWSTR pszShellExecuteCommand);
#ifdef UNICODE
#define SHSetUnreadMailCount  SHSetUnreadMailCountW
#else
#define SHSetUnreadMailCount  SHSetUnreadMailCountA
#endif // !UNICODE

#endif  /*  _WIN32_IE >= 0x0600     */

#if (_WIN32_IE >= 0x0601)
STDAPI_(BOOL)   SHTestTokenMembership(_In_opt_ HANDLE hToken, ULONG ulRID);
#endif // (_WIN32_IE >= 0x0601)

#if         _WIN32_IE >= 0x0600

#if (NTDDI_VERSION >= NTDDI_WINXP)
SHSTDAPI SHGetImageList(_In_ int iImageList, _In_ REFIID riid, _Outptr_ void **ppvObj);
#endif // (NTDDI_VERSION >= NTDDI_WINXP)


#if (NTDDI_VERSION >= NTDDI_WINXP)
#define SHIL_LARGE          0   // normally 32x32
#define SHIL_SMALL          1   // normally 16x16
#define SHIL_EXTRALARGE     2
#define SHIL_SYSSMALL       3   // like SHIL_SMALL, but tracks system small icon metric correctly

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define SHIL_JUMBO          4   // normally 256x256
#define SHIL_LAST           SHIL_JUMBO
#else
#define SHIL_LAST           SHIL_SYSSMALL
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


#endif // (NTDDI_VERSION >= NTDDI_WINXP)


// Function call types for ntshrui folder sharing helpers
typedef HRESULT (STDMETHODCALLTYPE *PFNCANSHAREFOLDERW)(_In_ PCWSTR pszPath);
typedef HRESULT (STDMETHODCALLTYPE *PFNSHOWSHAREFOLDERUIW)(_In_opt_ HWND hwndParent, _In_ PCWSTR pszPath);

#endif  /*  _WIN32_IE >= 0x0600     */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#if !defined(_WIN64)
#include <poppack.h>
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (NTDDI_VERSION >= NTDDI_VISTA)
// API for new Network Address Control

// Instantiation
#define WC_NETADDRESS L"msctls_netaddress"
SHSTDAPI_(BOOL) InitNetworkAddressControl(void);

// Address Control Messages

// NCM_GETADDRESS returns the type of address that is present in the
// control (based on TBD Net Address flags).  If the input string has
// not been validated using this message will force the validation of
// the input string.  The WPARAM is a BOOL to determine to show the
// balloon tip.  The LPARAM is a pointer to the structure to fill in
// with the address type and address string.
#define NCM_GETADDRESS (WM_USER+1)
#define NetAddr_GetAddress(hwnd,pv) \
    (HRESULT)SNDMSG(hwnd,NCM_GETADDRESS,0,(LPARAM)pv)
typedef struct tagNC_ADDRESS
{
   struct NET_ADDRESS_INFO_ *pAddrInfo; // defined in iphlpapi.h
   USHORT  PortNumber;
   BYTE    PrefixLength;
} NC_ADDRESS, *PNC_ADDRESS;

// NCM_SETALLOWTYPE sets the type of addresses that the control will allow.
// The address flags are defined in iphlpapi.h
#define NCM_SETALLOWTYPE (WM_USER+2)
#define NetAddr_SetAllowType(hwnd,addrMask) \
    (HRESULT)SNDMSG(hwnd,NCM_SETALLOWTYPE,(WPARAM)addrMask,0)
// NCM_GETALLOWTYPE returns the currently allowed type mask.
#define NCM_GETALLOWTYPE (WM_USER+3)
#define NetAddr_GetAllowType(hwnd) \
    (DWORD)SNDMSG(hwnd,NCM_GETALLOWTYPE,0,0)

// NCM_DISPLAYERRORTIP displays the error balloon tip with the correct
// error string (based on the last failure from the NCM_GETADDRESS call
#define NCM_DISPLAYERRORTIP (WM_USER+4)
#define NetAddr_DisplayErrorTip(hwnd) \
    (HRESULT)SNDMSG(hwnd,NCM_DISPLAYERRORTIP,0,0)

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)
// Returns the type of media (CD, DVD, Blank, etc) that is in the drive.
// dwMediaContent is set to a combination of ARCONTENT flags.
STDAPI SHGetDriveMedia(_In_ PCWSTR pszDrive, _Out_ DWORD *pdwMediaContent);
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

// Windows Parental Controls (WPC) query apis
#if (NTDDI_VERSION >= NTDDI_VISTA)
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _INC_SHELLAPI */

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(pop)
#endif
