/*===========================================================================

Copyright (c) Microsoft Corporation. All rights reserved.

File: shlobj.h

===========================================================================*/

#ifndef _SHLOBJ_H_
#define _SHLOBJ_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (_MSC_VER >= 800)
#if (_MSC_VER >= 1200)
#pragma warning(push)
#ifndef _MSC_EXTENSIONS
#pragma warning(disable:4309) /* truncation of constant value */
#endif
#pragma warning(disable:4820) /* padding added after data member */
#endif
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#endif

#ifndef SNDMSG
#ifdef __cplusplus
#define SNDMSG ::SendMessage
#else
#define SNDMSG SendMessage
#endif
#endif // ifndef SNDMSG

//
// Define API decoration for direct importing of DLL references.
//
#ifndef WINSHELLAPI
#if defined(_SHELL32_)
#define WINSHELLAPI
#else
#define WINSHELLAPI       DECLSPEC_IMPORT
#endif
#endif // WINSHELLAPI

#ifndef SHSTDAPI
#if defined(_SHELL32_)
#define SHSTDAPI          STDAPI
#define SHSTDAPI_(type)   STDAPI_(type)
#else
#define SHSTDAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define SHSTDAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif
#endif // SHSTDAPI

#ifndef SHDOCAPI
#if defined(_SHDOCVW_)
#define SHDOCAPI          STDAPI
#define SHDOCAPI_(type)   STDAPI_(type)
#else
#define SHDOCAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define SHDOCAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif
#endif // SHDOCAPI

// shell32 APIs that are also exported from shdocvw
#ifndef SHSTDDOCAPI
#if defined(_SHDOCVW_) || defined(_SHELL32_)
#define SHSTDDOCAPI          STDAPI
#define SHSTDDOCAPI_(type)   STDAPI_(type)
#else
#define SHSTDDOCAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define SHSTDDOCAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif
#endif // SHSTDDOCAPI

#ifndef BROWSEUIAPI
#if defined(_BROWSEUI_)
#define BROWSEUIAPI           STDAPI
#define BROWSEUIAPI_(type)    STDAPI_(type)
#else
#define BROWSEUIAPI           EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define BROWSEUIAPI_(type)    EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif // defined(_BROWSEUI_)
#endif // BROWSEUIAPI

// shell32 APIs that are also exported from shfolder
#ifndef SHFOLDERAPI
#if defined(_SHFOLDER_) || defined(_SHELL32_)
#define SHFOLDERAPI           STDAPI
#else
#define SHFOLDERAPI           EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#endif
#endif



#include <ole2.h>
#ifndef _PRSHT_H_
#include <prsht.h>
#endif
#ifndef _INC_COMMCTRL
#include <commctrl.h>   // for LPTBBUTTON
#endif


#include <shtypes.h>
#include <shobjidl.h>

#include <shlobj_core.h>

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif /* __cplusplus */

#include <pshpack1.h>   /* Assume byte packing throughout */


#if !defined(__cplusplus) && defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#endif

#if !defined(__cplusplus) && defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(pop)
#endif

#undef  INTERFACE
#define INTERFACE   INewShortcutHookA

DECLARE_INTERFACE_IID_(INewShortcutHookA, IUnknown, "000214e1-0000-0000-c000-000000000046")
{
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    STDMETHOD(SetReferent)(THIS_ _In_ PCSTR pcszReferent, _In_opt_ HWND hwnd) PURE;
    STDMETHOD(GetReferent)(THIS_ _Out_writes_(cchReferent) PSTR pszReferent, int cchReferent) PURE;
    STDMETHOD(SetFolder)(THIS_ _In_ PCSTR pcszFolder) PURE;
    STDMETHOD(GetFolder)(THIS_ _Out_writes_(cchFolder) PSTR pszFolder, int cchFolder) PURE;
    STDMETHOD(GetName)(THIS_ _Out_writes_(cchName) PSTR pszName, int cchName) PURE;
    STDMETHOD(GetExtension)(THIS_ _Out_writes_(cchExtension) PSTR pszExtension, int cchExtension) PURE;
};

#undef  INTERFACE
#define INTERFACE   INewShortcutHookW

DECLARE_INTERFACE_IID_(INewShortcutHookW, IUnknown, "000214f7-0000-0000-c000-000000000046")
{
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    STDMETHOD(SetReferent)(THIS_ _In_ PCWSTR pcszReferent, _In_opt_ HWND hwnd) PURE;
    STDMETHOD(GetReferent)(THIS_ _Out_writes_(cchReferent) PWSTR pszReferent, int cchReferent) PURE;
    STDMETHOD(SetFolder)(THIS_ _In_ PCWSTR pcszFolder) PURE;
    STDMETHOD(GetFolder)(THIS_ _Out_writes_(cchFolder) PWSTR pszFolder, int cchFolder) PURE;
    STDMETHOD(GetName)(THIS_ _Out_writes_(cchName) PWSTR pszName, int cchName) PURE;
    STDMETHOD(GetExtension)(THIS_ _Out_writes_(cchExtension) PWSTR pszExtension, int cchExtension) PURE;
};

#ifdef UNICODE
#define INewShortcutHook        INewShortcutHookW
#define INewShortcutHookVtbl    INewShortcutHookWVtbl
#else
#define INewShortcutHook        INewShortcutHookA
#define INewShortcutHookVtbl    INewShortcutHookAVtbl
#endif

//===========================================================================
//
// ICopyHook Interface
//
//  The copy hook is called whenever file system directories are
//  copy/moved/deleted/renamed via the shell.  It is also called by the shell
//  on changes of status of printers.
//
//  Clients register their id under STRREG_SHEX_COPYHOOK for file system hooks
//  and STRREG_SHEx_PRNCOPYHOOK for printer hooks.
//  the CopyCallback is called prior to the action, so the hook has the chance
//  to allow, deny or cancel the operation by returning the falues:
//     IDYES  -  means allow the operation
//     IDNO   -  means disallow the operation on this file, but continue with
//              any other operations (eg. batch copy)
//     IDCANCEL - means disallow the current operation and cancel any pending
//              operations
//
//   arguments to the CopyCallback
//      hwnd - window to use for any UI
//      wFunc - what operation is being done
//      wFlags - and flags (FOF_*) set in the initial call to the file operation
//      pszSrcFile - name of the source file
//      dwSrcAttribs - file attributes of the source file
//      pszDestFile - name of the destiation file (for move and renames)
//      dwDestAttribs - file attributes of the destination file
//
//
//===========================================================================

#undef  INTERFACE
#define INTERFACE   ICopyHookA

DECLARE_INTERFACE_IID_(ICopyHookA, IUnknown, "000214EF-0000-0000-c000-000000000046")
{
    STDMETHOD_(UINT, CopyCallback)(THIS_ _In_opt_ HWND hwnd, UINT wFunc, UINT wFlags, _In_ PCSTR pszSrcFile, DWORD dwSrcAttribs,
                                   _In_opt_ PCSTR pszDestFile, DWORD dwDestAttribs) PURE;
};

typedef ICopyHookA *    LPCOPYHOOKA;

#undef  INTERFACE
#define INTERFACE   ICopyHookW

DECLARE_INTERFACE_IID_(ICopyHookW, IUnknown, "000214FC-0000-0000-c000-000000000046")
{
    STDMETHOD_(UINT,CopyCallback)(THIS_ _In_opt_ HWND hwnd, UINT wFunc, UINT wFlags, _In_ PCWSTR pszSrcFile, DWORD dwSrcAttribs,
                                  _In_opt_ PCWSTR pszDestFile, DWORD dwDestAttribs) PURE;
};

typedef ICopyHookW *    LPCOPYHOOKW;

#ifdef UNICODE
#define ICopyHook       ICopyHookW
#define ICopyHookVtbl   ICopyHookWVtbl
#define LPCOPYHOOK      LPCOPYHOOKW
#else
#define ICopyHook       ICopyHookA
#define ICopyHookVtbl   ICopyHookAVtbl
#define LPCOPYHOOK      LPCOPYHOOKA
#endif

// IFileViewer, IFileViewerSite not supported as of win2k
#if (NTDDI_VERSION < NTDDI_WIN2K)

//===========================================================================
//
// IFileViewerSite Interface
//
//===========================================================================

#undef  INTERFACE
#define INTERFACE   IFileViewerSite

DECLARE_INTERFACE_IID_(IFileViewerSite, IUnknown, "000214f3-0000-0000-c000-000000000046")
{
    STDMETHOD(SetPinnedWindow) (THIS_ _In_opt_ HWND hwnd) PURE;
    STDMETHOD(GetPinnedWindow) (THIS_ _Out_ HWND *phwnd) PURE;
};

typedef IFileViewerSite * LPFILEVIEWERSITE;

//===========================================================================
//
// IFileViewer Interface
//
// Implemented in a FileViewer component object.  Used to tell a
// FileViewer to PrintTo or to view, the latter happening though
// ShowInitialize and Show.  The filename is always given to the
// viewer through IPersistFile.
//
//===========================================================================

#include <pshpack8.h>

typedef struct
{
    // Stuff passed into viewer (in)
    DWORD cbSize;           // Size of structure for future expansion...
    HWND hwndOwner;         // who is the owner window.
    int iShow;              // The show command

    // Passed in and updated  (in/Out)
    DWORD dwFlags;          // flags
    RECT rect;              // Where to create the window may have defaults
    IUnknown *punkRel;      // Relese this interface when window is visible

    // Stuff that might be returned from viewer (out)
    OLECHAR strNewFile[MAX_PATH];   // New File to view.

} FVSHOWINFO, *LPFVSHOWINFO;

#include <poppack.h>        /* Return to byte packing */

    // Define File View Show Info Flags.
#define FVSIF_RECT      0x00000001      // The rect variable has valid data.
#define FVSIF_PINNED    0x00000002      // We should Initialize pinned

#define FVSIF_NEWFAILED 0x08000000      // The new file passed back failed
                                        // to be viewed.

#define FVSIF_NEWFILE   0x80000000      // A new file to view has been returned
#define FVSIF_CANVIEWIT 0x40000000      // The viewer can view it.

#undef  INTERFACE
#define INTERFACE   IFileViewerA

DECLARE_INTERFACE_IID(IFileViewerA, "000214f0-0000-0000-c000-000000000046")
{
    STDMETHOD(ShowInitialize) (THIS_ _In_ LPFILEVIEWERSITE lpfsi) PURE;
    STDMETHOD(Show) (THIS_ _In_ LPFVSHOWINFO pvsi) PURE;
    STDMETHOD(PrintTo) (THIS_ _In_opt_ PSTR pszDriver, BOOL fSuppressUI) PURE;
};

typedef IFileViewerA * LPFILEVIEWERA;

#undef  INTERFACE
#define INTERFACE   IFileViewerW

DECLARE_INTERFACE_IID(IFileViewerW, "000214f8-0000-0000-c000-000000000046")
{
    STDMETHOD(ShowInitialize) (THIS_ _In_ LPFILEVIEWERSITE lpfsi) PURE;
    STDMETHOD(Show) (THIS_ _In_ LPFVSHOWINFO pvsi) PURE;
    STDMETHOD(PrintTo) (THIS_ _In_opt_ PWSTR pszDriver, BOOL fSuppressUI) PURE;
};

typedef IFileViewerW * LPFILEVIEWERW;

#ifdef UNICODE
#define IFileViewer IFileViewerW
#define LPFILEVIEWER LPFILEVIEWERW
#else
#define IFileViewer IFileViewerA
#define LPFILEVIEWER LPFILEVIEWERA
#endif

// IFileViewer, IFileViewerSite not supported as of win2k
#endif // (NTDDI_VERSION < NTDDI_WIN2K)


//--------------------------------------------------------------------------
// control IDs known to the view
//--------------------------------------------------------------------------

#define FCIDM_TOOLBAR      (FCIDM_BROWSERFIRST + 0)
#define FCIDM_STATUS       (FCIDM_BROWSERFIRST + 1)

//--------------------------------------------------------------------------
//
// The resource id of the offline cursor
// This cursor is avaialble in shdocvw.dll
#define IDC_OFFLINE_HAND        103
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define IDC_PANTOOL_HAND_OPEN   104
#define IDC_PANTOOL_HAND_CLOSED 105
#endif
//
//--------------------------------------------------------------------------


// SBCMDID_GETPANE - not necessarily in order
#define PANE_NONE        ((DWORD)-1)
#define PANE_ZONE        1
#define PANE_OFFLINE     2
#define PANE_PRINTER     3
#define PANE_SSL         4
#define PANE_NAVIGATION  5
#define PANE_PROGRESS    6
#if (_WIN32_IE >= _WIN32_IE_IE60)
#define PANE_PRIVACY     7
#endif


//
// ICurrentWorkingDirectory::GetDirectory(LPWSTR pwzPath, DWORD cchSize)
//   This function gets the Current Working Directory from a COM object that
//   stores such state.
//
// ICurrentWorkingDirectory::SetDirectory(LPCWSTR pwzPath)
//   This function sets the Current Working Directory of a COM object that
//   stores such state.
//
// This function can be used generically.  One COM object that implements it
// is CLSID_ACListISF so that the AutoComplete engine can complete relative
// paths.  SetDirectory() will set the "Current Working Directory" and
// AutoComplete with then complete both absolute and relative paths.
// For Example, if ::SetDirectory(L"C:\Program Files") is called, then
// the user can AutoComplete "..\winnt".  In order to set the current
// working directory for non-file system paths, "ftp://ftp.microsoft.com/" or
// "Control Panel" for example, use IPersistFolder.
//

#undef INTERFACE
#define INTERFACE ICurrentWorkingDirectory

DECLARE_INTERFACE_IID_(ICurrentWorkingDirectory, IUnknown, "91956D21-9276-11d1-921A-006097DF5BD4")
{
    STDMETHOD(GetDirectory) (THIS_ _Out_writes_(cchSize) PWSTR pwzPath, DWORD cchSize) PURE;
    STDMETHOD(SetDirectory) (THIS_ _In_ PCWSTR pwzPath) PURE;
};


//==========================================================================
// IDockingWindowSite/IDockingWindow/IDockingWindowFrame interfaces
// IInputObjectSite/IInputObject interfaces
//
//  These interfaces allow us (or ISVs) to install/update external Internet
// Toolbar for IE and the shell. The frame will simply get the CLSID from
// registry (to be defined) and CoCreateInstance it.
//
//==========================================================================


//-------------------------------------------------------------------------
//
// IDockingWindowFrame interface
//
// [Member functions]
//
// IDockingWindowFrame::AddToolbar(punkSrc, pwszItem, dwReserved)
//
// IDockingWindowFrame::RemoveToolbar(punkSrc, dwRemoveFlags)
//
// IDockingWindowFrame::FindToolbar(pwszItem, riid, ppv)
//
//-------------------------------------------------------------------------


// flags for RemoveToolbar
#define DWFRF_NORMAL            0x0000
#define DWFRF_DELETECONFIGDATA  0x0001


// flags for AddToolbar
#define DWFAF_HIDDEN    0x0001   // add hidden
#define DWFAF_GROUP1    0x0002   // insert at end of group 1
#define DWFAF_GROUP2    0x0004   // insert at end of group 2
#define DWFAF_AUTOHIDE  0x0010   // The toolbar will be subject to AutoHide in Full Screen mode


#undef  INTERFACE
#define INTERFACE   IDockingWindowFrame

DECLARE_INTERFACE_IID_(IDockingWindowFrame, IOleWindow, "47d2657a-7b27-11d0-8ca9-00a0c92dbfe8")
{
    STDMETHOD(AddToolbar) (THIS_ _In_ IUnknown* punkSrc, _In_ PCWSTR pwszItem, DWORD dwAddFlags) PURE;
    STDMETHOD(RemoveToolbar) (THIS_ _In_ IUnknown* punkSrc, DWORD dwRemoveFlags) PURE;
    STDMETHOD(FindToolbar) (THIS_ _In_ PCWSTR pwszItem, _In_ REFIID riid, _Outptr_ void **ppv) PURE;
};



/* ***************** IThumbnailCapture
 * CaptureThumbnail : takes an IHTMLDocument2 and returns a thumbnail of specified
 *                    size as an hbitmap
 */

#undef  INTERFACE
#define INTERFACE   IThumbnailCapture

DECLARE_INTERFACE_IID_(IThumbnailCapture, IUnknown, "4ea39266-7211-409f-b622-f63dbd16c533")
{
    // *** IThumbnailCapture methods ***
    STDMETHOD (CaptureThumbnail)    ( THIS_ _In_ const SIZE * pMaxSize,
                                      _In_ IUnknown * pHTMLDoc2,
                                      _Out_ HBITMAP * phbmThumbnail ) PURE;
};
typedef IThumbnailCapture * LPTHUMBNAILCAPTURE;


#if (NTDDI_VERSION >= NTDDI_WIN2K && NTDDI_VERSION < NTDDI_VISTA)

#include <pshpack8.h>

typedef struct _EnumImageStoreDATAtag
{
    WCHAR     szPath[MAX_PATH];
    FILETIME  ftTimeStamp;
} ENUMSHELLIMAGESTOREDATA, * PENUMSHELLIMAGESTOREDATA;

#include <poppack.h>        /* Return to byte packing */

#undef  INTERFACE
#define INTERFACE   IEnumShellImageStore

DECLARE_INTERFACE_IID_( IEnumShellImageStore, IUnknown, "6DFD582B-92E3-11D1-98A3-00C04FB687DA" )
{
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    STDMETHOD ( Reset ) ( THIS ) PURE;
    STDMETHOD ( Next ) ( THIS_ ULONG celt, _Out_writes_(*pceltFetched) PENUMSHELLIMAGESTOREDATA * prgElt, _Out_opt_ ULONG * pceltFetched ) PURE;
    STDMETHOD ( Skip ) ( THIS_ ULONG celt ) PURE;
    STDMETHOD ( Clone ) ( THIS_ _Outptr_ IEnumShellImageStore ** ppEnum ) PURE;
};

typedef IEnumShellImageStore * LPENUMSHELLIMAGESTORE;


// flags used to determine the capabilities of the storage for the images
#define SHIMSTCAPFLAG_LOCKABLE    0x0001       // does the store require/support locking
#define SHIMSTCAPFLAG_PURGEABLE   0x0002       // does the store require dead items purging externally ?

#undef  INTERFACE
#define INTERFACE   IShellImageStore

// this interface is used to manipulate the Image cache. It can potentially be used
// in a free threaded manner in conjunction with the Lock parameter to Open and close
DECLARE_INTERFACE_IID_( IShellImageStore, IUnknown, "48C8118C-B924-11D1-98D5-00C04FB687DA" )
{
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // if the lock parameter is used, then all other calls into
    // open and/or create will block until the lock is released.
    STDMETHOD ( Open ) ( THIS_ DWORD dwMode, _Out_ DWORD * pdwLock ) PURE;
    STDMETHOD ( Create ) ( THIS_ DWORD dwMode, _Out_ DWORD * pdwLock ) PURE;

    // if the lock is passed to either of these two methods, it releases the lock
    // once the operation is complete.
    STDMETHOD ( ReleaseLock ) ( THIS_ _In_ DWORD const * pdwLock ) PURE;
    STDMETHOD ( Close ) ( THIS_ _In_ DWORD const * pdwLock ) PURE;
    STDMETHOD ( Commit ) ( THIS_ _In_ DWORD const * pdwLock ) PURE;
    STDMETHOD ( IsLocked ) ( THIS ) PURE;

    STDMETHOD ( GetMode ) ( THIS_ _Out_ DWORD * pdwMode ) PURE;
    STDMETHOD ( GetCapabilities ) ( THIS_ _Out_ DWORD * pdwCapMask ) PURE;

    STDMETHOD ( AddEntry ) ( THIS_ _In_ PCWSTR pszName, _In_ const FILETIME * pftTimeStamp, DWORD dwMode, _In_ HBITMAP hImage ) PURE;
    STDMETHOD ( GetEntry ) ( THIS_ _In_ PCWSTR pszName, DWORD dwMode, _Out_ HBITMAP * phImage ) PURE;
    STDMETHOD ( DeleteEntry ) ( THIS_ _In_ PCWSTR pszName ) PURE;
    STDMETHOD ( IsEntryInStore ) ( THIS_ _In_ PCWSTR pszName, _Out_ FILETIME * pftTimeStamp ) PURE;

    STDMETHOD ( Enum ) ( THIS_ _Outptr_ LPENUMSHELLIMAGESTORE * ppEnum ) PURE;
};

typedef IShellImageStore * LPSHELLIMAGESTORE;

#endif  // (NTDDI_VERSION >= NTDDI_WIN2K && NTDDI_VERSION < NTDDI_VISTA)

////  IShellFolderBand

// Field mask
#define ISFB_MASK_STATE          0x00000001 // TRUE if dwStateMask and dwState is valid
#define ISFB_MASK_BKCOLOR        0x00000002 // TRUE if crBkgnd field is valid
#define ISFB_MASK_VIEWMODE       0x00000004 // TRUE if wViewMode field is valid
#define ISFB_MASK_SHELLFOLDER    0x00000008
#define ISFB_MASK_IDLIST         0x00000010
#define ISFB_MASK_COLORS         0x00000020 // TRUE if crXXXX fields are valid (except bkgnd)

#define ISFB_STATE_DEFAULT       0x00000000
#define ISFB_STATE_DEBOSSED      0x00000001
#define ISFB_STATE_ALLOWRENAME   0x00000002
#define ISFB_STATE_NOSHOWTEXT    0x00000004 // TRUE if _fNoShowText
#define ISFB_STATE_CHANNELBAR    0x00000010 // TRUE if we want NavigateTarget support
#define ISFB_STATE_QLINKSMODE    0x00000020 // TRUE if we want to turn off drag & drop onto content items
#define ISFB_STATE_FULLOPEN      0x00000040 // TRUE if band should maximize when opened
#define ISFB_STATE_NONAMESORT    0x00000080 // TRUE if band should _not_ sort icons by name
#define ISFB_STATE_BTNMINSIZE    0x00000100 // TRUE if band should report min thickness of button

#define ISFBVIEWMODE_SMALLICONS   0x0001
#define ISFBVIEWMODE_LARGEICONS   0x0002
#if (_WIN32_IE < _WIN32_IE_IE70)
#define ISFBVIEWMODE_LOGOS        0x0003
#endif

#include <pshpack8.h>

typedef struct {
    DWORD       dwMask;       // [in] ISFB_MASK mask of valid fields from crBkgnd on
    DWORD       dwStateMask;  // [in] ISFB_STATE mask of dwState bits being set/queried
    DWORD       dwState;      // [in/out] ISFB_STATE bits
    COLORREF    crBkgnd;      // [in/out]
    COLORREF    crBtnLt;      // [in/out]
    COLORREF    crBtnDk;      // [in/out]
    WORD        wViewMode;    // [in/out]
    WORD        wAlign;       // not used (yet)
    IShellFolder * psf;       // [out]
    PIDLIST_ABSOLUTE pidl;      // [out]
} BANDINFOSFB, *PBANDINFOSFB;

#include <poppack.h>        /* Return to byte packing */

#undef INTERFACE
#define INTERFACE IShellFolderBand

DECLARE_INTERFACE_IID_(IShellFolderBand, IUnknown, "7FE80CC8-C247-11d0-B93A-00A0C90312E1")
{
    STDMETHOD(InitializeSFB)(THIS_ _In_opt_ IShellFolder *psf, _In_opt_ PCIDLIST_ABSOLUTE pidl) PURE;
    STDMETHOD(SetBandInfoSFB)(THIS_ _In_ PBANDINFOSFB pbi) PURE;
    STDMETHOD(GetBandInfoSFB)(THIS_ _Inout_ PBANDINFOSFB pbi) PURE;
};

// Command Target IDs
enum {
    SFBID_PIDLCHANGED,
};

////  IDeskBarClient

#undef  INTERFACE
#define INTERFACE   IDeskBarClient

DECLARE_INTERFACE_IID_(IDeskBarClient, IOleWindow, "EB0FE175-1A3A-11D0-89B3-00A0C90A90AC")
{
    STDMETHOD(SetDeskBarSite) (THIS_ _In_opt_ IUnknown* punkSite) PURE;
    STDMETHOD(SetModeDBC) (THIS_ DWORD dwMode) PURE;
    STDMETHOD(UIActivateDBC) (THIS_ DWORD dwState) PURE;
    STDMETHOD(GetSize) (THIS_ DWORD dwWhich, _Out_ LPRECT prc) PURE;
};

#define DBC_GS_IDEAL          0  // get the ideal size
#define DBC_GS_SIZEDOWN       1  // clip the height of a rect to a multiple of the rebar's integral size


#define DBC_HIDE        0 // Band is hidden (being destroyed)
#define DBC_SHOW        1 // Band is visible
#define DBC_SHOWOBSCURE 2 // Band is completely obscured


enum {
    DBCID_EMPTY = 0,        // bandsite is empty
    DBCID_ONDRAG = 1,       // (down)DragMoveEnter/Leave vaIn:I4:eDrag
    DBCID_CLSIDOFBAR = 2,   // clsid of bar inside
    DBCID_RESIZE = 3,       // resize from keyboard accelerator
    DBCID_GETBAR = 4,       // returns vaOut:VT_UNKNOWN of hosting dockbar (IDeskBar)
    DBCID_UPDATESIZE = 5,   // resize from non-user source (search activation failure)
};



// Flags for SetSafeMode
#define SSM_CLEAR   0x0000
#define SSM_SET     0x0001
#define SSM_REFRESH 0x0002
#define SSM_UPDATE  0x0004

// Flags for Set/GetScheme
#define SCHEME_DISPLAY          0x0001
#define SCHEME_EDIT             0x0002
#define SCHEME_LOCAL            0x0004
#define SCHEME_GLOBAL           0x0008
#define SCHEME_REFRESH          0x0010
#define SCHEME_UPDATE           0x0020
#define SCHEME_DONOTUSE 0x0040 // used to be SCHEME_ENUMERATE; no longer supported
#define SCHEME_CREATE           0x0080

#undef INTERFACE
#define INTERFACE IActiveDesktopP

DECLARE_INTERFACE_IID_(IActiveDesktopP, IUnknown, "52502EE0-EC80-11D0-89AB-00C04FC2972D")
{
    // IUnknown methods
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // IActiveDesktopP methods
    STDMETHOD (SetSafeMode)(THIS_ DWORD dwFlags) PURE;
    STDMETHOD (EnsureUpdateHTML)(THIS) PURE;
    STDMETHOD (SetScheme)(THIS_ _In_ PCWSTR pwszSchemeName, DWORD dwFlags) PURE;
    STDMETHOD (GetScheme)(THIS_ _Out_writes_(*pdwcchBuffer) PWSTR pwszSchemeName, _Inout_ DWORD *pdwcchBuffer, DWORD dwFlags) PURE;
};

typedef IActiveDesktopP * LPACTIVEDESKTOPP;

//Flags for GetObjectFlags
#define GADOF_DIRTY    0x00000001

#undef INTERFACE
#define INTERFACE IADesktopP2

DECLARE_INTERFACE_IID_(IADesktopP2, IUnknown, "B22754E2-4574-11d1-9888-006097DEACF9")
{
    // IUnknown methods
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // IADesktopP2 methods
    STDMETHOD (ReReadWallpaper)(THIS) PURE;
    STDMETHOD (GetADObjectFlags)(THIS_ _Out_ DWORD *pdwFlags, DWORD dwMask) PURE;
    STDMETHOD (UpdateAllDesktopSubscriptions)(THIS) PURE;
    STDMETHOD (MakeDynamicChanges)(THIS_ _In_ IOleObject *pOleObj) PURE;
};

typedef IADesktopP2 * LPADESKTOPP2;


#include <pshpack1.h>

typedef struct {
    SHCOLUMNID  scid;                           // OUT the unique identifier of this column
    VARTYPE     vt;                             // OUT the native type of the data returned
    DWORD       fmt;                            // OUT this listview format (LVCFMT_LEFT, usually)
    UINT        cChars;                         // OUT the default width of the column, in characters
    DWORD       csFlags;                        // OUT SHCOLSTATE flags
    WCHAR wszTitle[MAX_COLUMN_NAME_LEN];        // OUT the title of the column
    WCHAR wszDescription[MAX_COLUMN_DESC_LEN];  // OUT full description of this column
} SHCOLUMNINFO, *LPSHCOLUMNINFO;
typedef const SHCOLUMNINFO* LPCSHCOLUMNINFO;

#include <poppack.h>        /* Return to default */

#include <pshpack8.h>

typedef struct {
    ULONG   dwFlags;              // initialization flags
    ULONG   dwReserved;           // reserved for future use.
    WCHAR   wszFolder[MAX_PATH];  // fully qualified folder path (or empty if multiple folders)
} SHCOLUMNINIT, *LPSHCOLUMNINIT;
typedef const SHCOLUMNINIT* LPCSHCOLUMNINIT;

#define SHCDF_UPDATEITEM        0x00000001      // this flag is a hint that the file has changed since the last call to GetItemData

typedef struct {
    ULONG   dwFlags;             // combination of SHCDF_ flags.
    DWORD   dwFileAttributes;    // file attributes.
    ULONG   dwReserved;          // reserved for future use.
    WCHAR*  pwszExt;             // address of file name extension
    WCHAR   wszFile[MAX_PATH];   // Absolute path of file.
} SHCOLUMNDATA, *LPSHCOLUMNDATA;
typedef const SHCOLUMNDATA* LPCSHCOLUMNDATA;

#include <poppack.h>        /* Return to byte packing */

#undef INTERFACE
#define INTERFACE IColumnProvider

// Note: these objects must be threadsafe!  GetItemData _will_ be called
// simultaneously from multiple threads.
DECLARE_INTERFACE_IID_(IColumnProvider, IUnknown, "E8025004-1C42-11d2-BE2C-00A0C9A83DA1")
{
    // IUnknown methods
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // IColumnProvider methods
    STDMETHOD (Initialize)(THIS_ _In_ LPCSHCOLUMNINIT psci) PURE;
    STDMETHOD (GetColumnInfo)(THIS_ DWORD dwIndex, _Out_ SHCOLUMNINFO *psci) PURE;
    STDMETHOD (GetItemData)(THIS_ _In_ LPCSHCOLUMNID pscid, _In_ LPCSHCOLUMNDATA pscd, _Out_ VARIANT *pvarData) PURE;
};


#if (NTDDI_VERSION >= NTDDI_WIN2K)

typedef struct _SHChangeProductKeyAsIDList {
    USHORT cb;
    WCHAR wszProductKey[39];
    USHORT cbZero;
} SHChangeProductKeyAsIDList, *LPSHChangeProductKeyAsIDList;


#endif /* NTDDI_WIN2K */

#if (NTDDI_VERSION >= NTDDI_VISTA)
//  use SHChangeNotifyRegisterThread() to enable Async Register and Deregister.
//  call with SCNRT_ENABLE at the thread proc begining and SCNRT_DISABLE at the end
//  the call with SCNRT_DISABLE can block while it synchronizes with the main ChangeNotify thread
STDAPI_(void) SHChangeNotifyRegisterThread(SCNRT_STATUS status);
#endif


//===========================================================================

#if (NTDDI_VERSION >= NTDDI_WINXP)
#endif
SHSTDAPI_(void) PathQualify(_Inout_ PWSTR psz);
SHSTDAPI_(BOOL) PathIsSlowA(_In_ LPCSTR pszFile, DWORD dwAttr);
SHSTDAPI_(BOOL) PathIsSlowW(_In_ LPCWSTR pszFile, DWORD dwAttr);
#ifdef UNICODE
#define PathIsSlow  PathIsSlowW
#else
#define PathIsSlow  PathIsSlowA
#endif // !UNICODE

SHSTDAPI_(BOOL) GetFileNameFromBrowse(_In_opt_ HWND hwnd, _Inout_updates_(cchFilePath) PWSTR pszFilePath, UINT cchFilePath,
                                      _In_opt_ PCWSTR pszWorkingDir, _In_ PCWSTR pszDefExt, _In_opt_ PCWSTR pszFilters, _In_opt_ PCWSTR pszTitle);
SHSTDAPI_(int) DriveType(int iDrive);

WINSHELLAPI HPSXA WINAPI SHCreatePropSheetExtArray(_In_ HKEY hKey, _In_opt_ PCWSTR pszSubKey, UINT max_iface);

#if (NTDDI_VERSION >= NTDDI_WIN2K && NTDDI_VERSION < NTDDI_VISTA)
#undef  INTERFACE
#define INTERFACE   IDefViewFrame
DECLARE_INTERFACE_IID_(IDefViewFrame, IUnknown, "710EB7A0-45ED-11D0-924A-0020AFC7AC4D")
{
    STDMETHOD(GetWindowLV) (THIS_ _Out_ HWND *phwnd) PURE;
    STDMETHOD(ReleaseWindowLV) (THIS) PURE;
    STDMETHOD(GetShellFolder)(THIS_ _Outptr_ IShellFolder **ppsf) PURE;
};
#endif

#if (NTDDI_VERSION < NTDDI_VISTA)
//
// Path processing function
//
#define PPCF_ADDQUOTES               0x00000001        // return a quoted name if required
#define PPCF_ADDARGUMENTS            0x00000003        // appends arguments (and wraps in quotes if required)
#define PPCF_NODIRECTORIES           0x00000010        // don't match to directories
#define PPCF_FORCEQUALIFY            0x00000040        // qualify even non-relative names
#define PPCF_LONGESTPOSSIBLE         0x00000080        // always find the longest possible name

SHSTDAPI_(LONG) PathProcessCommand(_In_ PCWSTR pszSrc, _Out_writes_(cchDest) PWSTR pszDest, int cchDest, DWORD dwFlags);
#endif

#if (NTDDI_VERSION < NTDDI_VISTA)
SHSTDAPI SHLoadOLE(LPARAM lParam);
#endif


//
// IDocViewSite
//
#undef  INTERFACE
#define INTERFACE  IDocViewSite
DECLARE_INTERFACE_IID_(IDocViewSite, IUnknown, "87D605E0-C511-11CF-89A9-00A0C9054129")
{
    STDMETHOD(OnSetTitle) (THIS_ _In_ VARIANTARG *pvTitle) PURE;
};
#define OPENPROPS_NONE          0x0000
#define OPENPROPS_INHIBITPIF    0x8000
#define GETPROPS_NONE           0x0000
#define SETPROPS_NONE           0x0000
#define CLOSEPROPS_NONE         0x0000
#define CLOSEPROPS_DISCARD      0x0001


#undef  INTERFACE
#define INTERFACE   IInitializeObject

DECLARE_INTERFACE_IID_(IInitializeObject, IUnknown, "4622AD16-FF23-11d0-8D34-00A0C90F2719")
{
    STDMETHOD(Initialize)(THIS) PURE;
};

enum
{
    BMICON_LARGE = 0,
    BMICON_SMALL
};

#undef  INTERFACE
#define INTERFACE   IBanneredBar

DECLARE_INTERFACE_IID_(IBanneredBar, IUnknown, "596A9A94-013E-11d1-8D34-00A0C90F2719")
{
    STDMETHOD(SetIconSize)(THIS_ DWORD iIcon) PURE;
    STDMETHOD(GetIconSize)(THIS_ _Out_ DWORD* piIcon) PURE;
    STDMETHOD(SetBitmap)(THIS_ _In_ HBITMAP hBitmap) PURE;
    STDMETHOD(GetBitmap)(THIS_ _Out_ HBITMAP* phBitmap) PURE;
};


#include <pshpack8.h>

// TBINFO flags
#define TBIF_APPEND     0
#define TBIF_PREPEND    1
#define TBIF_REPLACE    2
#define TBIF_DEFAULT      0x00000000
#define TBIF_INTERNETBAR  0x00010000
#define TBIF_STANDARDTOOLBAR   0x00020000
#define TBIF_NOTOOLBAR  0x00030000

typedef struct _TBINFO
{
    UINT        cbuttons;       // out
    UINT        uFlags;         // out (one of TBIF_ flags)
} TBINFO;
typedef TBINFO * LPTBINFO;

SHSTDAPI_(BOOL) SHOpenPropSheetA(_In_opt_ LPCSTR pszCaption, _In_reads_opt_(ckeys) HKEY ahkeys[], UINT ckeys,
                                 _In_opt_ const CLSID * pclsidDefault, _In_ IDataObject *pdtobj,
                                 _In_opt_ IShellBrowser *psb, _In_opt_ LPCSTR pStartPage);
SHSTDAPI_(BOOL) SHOpenPropSheetW(_In_opt_ LPCWSTR pszCaption, _In_reads_opt_(ckeys) HKEY ahkeys[], UINT ckeys,
                                 _In_opt_ const CLSID * pclsidDefault, _In_ IDataObject *pdtobj,
                                 _In_opt_ IShellBrowser *psb, _In_opt_ LPCWSTR pStartPage);
#ifdef UNICODE
#define SHOpenPropSheet  SHOpenPropSheetW
#else
#define SHOpenPropSheet  SHOpenPropSheetA
#endif // !UNICODE

// Tell the FolderView to rearrange.  The lParam will be passed to
// IShellFolder::CompareIDs
#define SFVM_REARRANGE          0x00000001
#define ShellFolderView_ReArrange(_hwnd, _lparam) \
        (BOOL)SHShellFolderView_Message(_hwnd, SFVM_REARRANGE, _lparam)
// Add an OBJECT into the view
#define SFVM_ADDOBJECT         0x00000003
#define ShellFolderView_AddObject(_hwnd, _pidl) \
        (LPARAM)SHShellFolderView_Message(_hwnd, SFVM_ADDOBJECT, (LPARAM)(_pidl))
// Remove an OBJECT into the view
#define SFVM_REMOVEOBJECT         0x00000006
#define ShellFolderView_RemoveObject(_hwnd, _pidl) \
        (LPARAM)SHShellFolderView_Message(_hwnd, SFVM_REMOVEOBJECT, (LPARAM)(_pidl))

// updates an object by passing in pointer to two PIDLS, the first
// is the old pidl, the second one is the one with update information.
//
// _ppidl[1] must be a *copy* of a pidl, as control over the lifetime
// of the pidl belongs to the view after successful completion of
// this call.  (Unsuccessful completion (a -1 return) implies failure
// and the caller must free the memory.)  Win95 waits a while before
// freeing the pidl, IE4 frees the pidl immediately.
// IShellFolderView::UpdateObject does not suffer from this problem.
//
#define SFVM_UPDATEOBJECT         0x00000007
#define ShellFolderView_UpdateObject(_hwnd, _ppidl) \
        (LPARAM)SHShellFolderView_Message(_hwnd, SFVM_UPDATEOBJECT, (LPARAM)(_ppidl))


// Returns an array of the selected IDS to the caller.
//     lparam is a pointer to receive the idlists into
//     return value is the count of items in the array.
#define SFVM_GETSELECTEDOBJECTS 0x00000009
#define ShellFolderView_GetSelectedObjects(_hwnd, ppidl) \
        (LPARAM)SHShellFolderView_Message(_hwnd, SFVM_GETSELECTEDOBJECTS, (LPARAM)(ppidl))
typedef struct _SFV_SETITEMPOS
{
        PCUITEMID_CHILD pidl;
        POINT pt;
} SFV_SETITEMPOS;
typedef SFV_SETITEMPOS *LPSFV_SETITEMPOS;
typedef const SFV_SETITEMPOS *PCSFV_SETITEMPOS;

// Sets the position of an item in the viewer
//     lparam is a pointer to a SVF_SETITEMPOS
//     return value is unused
#define SFVM_SETITEMPOS         0x0000000e
#define ShellFolderView_SetItemPos(_hwnd, _pidl, _x, _y) \
{       SFV_SETITEMPOS _sip = {_pidl, {_x, _y}}; \
        SHShellFolderView_Message(_hwnd, SFVM_SETITEMPOS, (LPARAM)(LPSFV_SETITEMPOS)&_sip);}
//  Notifies a ShellView when one of its objects get put on the clipboard
//  as a result of a menu command.
//
//
//     lparam is the dwEffect (DROPEFFECT_MOVE, DROPEFFECT_COPY)
//     return value is void.
#define SFVM_SETCLIPBOARD       0x00000010
#define ShellFolderView_SetClipboard(_hwnd, _dwEffect) \
        (void)SHShellFolderView_Message(_hwnd, SFVM_SETCLIPBOARD, (LPARAM)(DWORD)(_dwEffect))
#define SFVM_SETPOINTS           0x00000017
#define ShellFolderView_SetPoints(_hwnd, _pdtobj) \
        (void)SHShellFolderView_Message(_hwnd, SFVM_SETPOINTS, (LPARAM)(_pdtobj))
#include <poppack.h>        /* Return to byte packing */

#ifdef __urlmon_h__
//    NOTE: urlmon.h must be included before shlobj.h to access this function.
//
//    SoftwareUpdateMessageBox
//
//    Provides a standard message box for the alerting the user that a software
//    update is available or installed. No UI will be displayed if there is no
//    update available or if the available update version is less than or equal
//    to the Advertised update version.
//
//    hWnd                - [in] Handle of owner window
//    szDistUnit          - [in] Unique identifier string for a code distribution unit. For
//                               ActiveX controls and Active Setup installed components, this
//                               is typically a GUID string.
//    dwFlags             - [in] Must be 0.
//    psdi                - [in,out] Pointer to SOFTDISTINFO ( see URLMon.h ). May be NULL.
//                                cbSize should be initialized
//                                by the caller to sizeof(SOFTDISTINFO), dwReserved should be set to 0.
//
//    RETURNS:
//
//    IDNO     - The user chose cancel. If *pbRemind is FALSE, the caller should save the
//               update version from the SOFTDISTINFO and pass it in as the Advertised
//               version in future calls.
//
//    IDYES    - The user has selected Update Now/About Update. The caller should navigate to
//               the SOFTDISTINFO's pszHREF to initiate the install or learn about it.
//               The caller should save the update version from the SOFTDISTINFO and pass
//               it in as the Advertised version in future calls.
//
//    IDIGNORE - There is no pending software update. Note: There is
//               no Ignore button in the standard UI. This occurs if the available
//               version is less than the installed version or is not present or if the
//               Advertised version is greater than or equal to the update version.
//
//    IDABORT  - An error occured. Call GetSoftwareUpdateInfo() for a more specific HRESULT.
//               Note: There is no Abort button in the standard UI.


SHDOCAPI_(DWORD) SoftwareUpdateMessageBox( _In_opt_ HWND hWnd,
                                           _In_ PCWSTR pszDistUnit,
                                           DWORD dwFlags,
                                           _Out_opt_ LPSOFTDISTINFO psdi );
#endif // if __urlmon_h__


#if (NTDDI_VERSION >= NTDDI_WINXP)
#include <pshpack8.h>
#include <poppack.h>        /* Return to byte packing */
#endif  // NTDDI_WINXP

#if (NTDDI_VERSION >= NTDDI_WIN2K)

//
// The SHMultiFileProperties API displays a property sheet for a
// set of files specified in an IDList Array.
//
// Parameters:
//      pdtobj  - Data object containing list of files.  The data
//                object must provide the "Shell IDList Array"
//                clipboard format.  The parent folder's implementation of
//                IShellFolder::GetDisplayNameOf must return a fully-qualified
//                filesystem path for each item in response to the
//                SHGDN_FORPARSING flag.
//
//      dwFlags - Reserved for future use.  Should be set to 0.
//
// Returns:
//      S_OK
//
SHSTDAPI SHMultiFileProperties(_In_ IDataObject *pdtobj, DWORD dwFlags);

#endif  // NTDDI_WIN2K

SHSTDAPI SHCreateQueryCancelAutoPlayMoniker(_Outptr_ IMoniker** ppmoniker);    // deprecated: use CreateClassMoniker(CLSID_YourOwnClsid, ...)
STDAPI_(void) PerUserInit(void);
SHSTDAPI_(BOOL)SHRunControlPanel(_In_ PCWSTR lpcszCmdLine, _In_opt_ HWND hwndMsgParent);

#include <pshpack8.h>

typedef struct tagAAMENUFILENAME
{
  SHORT  cbTotal;
  BYTE   rgbReserved[12];
  WCHAR  szFileName[1];     // variable length string
} AASHELLMENUFILENAME, *LPAASHELLMENUFILENAME;

typedef struct tagAASHELLMENUITEM
{
  void*  lpReserved1;
  int    iReserved;
  UINT   uiReserved;
  LPAASHELLMENUFILENAME lpName; // name of file
  LPWSTR psz;           // text to use if no file
} AASHELLMENUITEM, *LPAASHELLMENUITEM;

#include <poppack.h>        /* Return to byte packing */


#if (_WIN32_IE >= _WIN32_IE_XP)

SHDOCAPI_(BOOL) ImportPrivacySettings(_In_ PCWSTR pszFilename, _Inout_ BOOL* pfParsePrivacyPreferences, _Inout_ BOOL* pfParsePerSiteRules);
#ifndef IEnumPrivacyRecords
typedef interface IEnumPrivacyRecords IEnumPrivacyRecords;
#endif
SHDOCAPI DoPrivacyDlg(_In_opt_ HWND hwndOwner, _In_ PCWSTR pszUrl, _In_ IEnumPrivacyRecords *pPrivacyEnum, BOOL fReportAllSites);

#endif  // _WIN32_IE_XP

#include <poppack.h>

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#if (_MSC_VER >= 800)
#if (_MSC_VER >= 1200)
#pragma warning(pop)
#else
#pragma warning(default:4001)
#endif
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _SHLOBJ_H_ */
