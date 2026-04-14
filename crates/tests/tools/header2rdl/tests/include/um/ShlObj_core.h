/*===========================================================================

Copyright (c) Microsoft Corporation. All rights reserved.

File: shlobj_core.h

===========================================================================*/
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

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

#ifndef INITGUID
#include <shlguid.h>
#endif /* !INITGUID */


#include <shtypes.h>

#include <shobjidl_core.h>

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif /* __cplusplus */

#include <pshpack1.h>   /* Assume byte packing throughout */

//===========================================================================
//
// Legacy exports that are no longer needed, call the COM API instead
//
//===========================================================================

_Check_return_
SHSTDAPI SHGetMalloc(_Outptr_ IMalloc **ppMalloc);   // CoGetMalloc(MEMCTX_TASK,ppMalloc)
SHSTDAPI_(void *) SHAlloc(SIZE_T cb);                   // CoTaskMemAlloc(cb)
SHSTDAPI_(void)   SHFree(_In_opt_ void * pv);           // CoTaskMemFree(pv)


//===========================================================================
//
// IExtractIcon interface
//
//  This interface is used in two different places in the shell.
//
// Case-1: Icons of sub-folders for the scope-pane of the explorer.
//
//  It is used by the explorer to get the "icon location" of
// sub-folders from each shell folders. When the user expands a folder
// in the scope pane of the explorer, the explorer does following:
//  (1) binds to the folder (gets IShellFolder),
//  (2) enumerates its sub-folders by calling its EnumObjects member,
//  (3) calls its GetUIObjectOf member to get IExtractIcon interface
//     for each sub-folders.
//  In this case, the explorer uses only IExtractIcon::GetIconLocation
// member to get the location of the appropriate icon. An icon location
// always consists of a file name (typically DLL or EXE) and either an icon
// resource or an icon index.
//
//
// Case-2: Extracting an icon image from a file
//
//  It is used by the shell when it extracts an icon image
// from a file. When the shell is extracting an icon from a file,
// it does following:
//  (1) creates the icon extraction handler object (by getting its CLSID
//     under the {ProgID}\shell\ExtractIconHanler key and calling
//     CoCreateInstance requesting for IExtractIcon interface).
//  (2) Calls IExtractIcon::GetIconLocation.
//  (3) Then, calls IExtractIcon::ExtractIcon with the location/index pair.
//  (4) If (3) returns S_OK, it uses the returned icon.
//  (5) Otherwise, it recursively calls this logic with new location
//     assuming that the location string contains a fully qualified path name.
//
//  From extension programmer's point of view, there are only two cases
// where they provide implementations of IExtractIcon:
//  Case-1) providing explorer extensions (i.e., IShellFolder).
//  Case-2) providing per-instance icons for some types of files.
//
// Because Case-1 is described above, we'll explain only Case-2 here.
//
// When the shell is about display an icon for a file, it does following:
//  (1) Finds its ProgID and ClassID.
//  (2) If the file has a ClassID, it gets the icon location string from the
//    "DefaultIcon" key under it. The string indicates either per-class
//    icon (e.g., "EXAMPLE.DLL,2") or per-instance icon (e.g., "%1,1").
//  (3) If a per-instance icon is specified, the shell creates an icon
//    extraction handler object for it, and extracts the icon from it
//    (which is described above).
//
//  It is important to note that the shell calls IExtractIcon::GetIconLocation
// first, then calls IExtractIcon::Extract. Most application programs
// that support per-instance icons will probably store an icon location
// (DLL/EXE name and index/id) rather than an icon image in each file.
// In those cases, a programmer needs to implement only the GetIconLocation
// member and it Extract member simply returns S_FALSE. They need to
// implement Extract member only if they decided to store the icon images
// within files themselved or some other database (which is very rare).
//
//
//
// [Member functions]
//
//
// IExtractIcon::GetIconLocation
//
//  This function returns an icon location.
//
//  Parameters:
//   uFlags     [in]  -- Specifies if it is opened or not (GIL_OPENICON or 0)
//   szIconFile [out] -- Specifies the string buffer buffer for a location name.
//   cchMax     [in]  -- Specifies the size of szIconFile (almost always MAX_PATH)
//   piIndex    [out] -- Sepcifies the address of UINT for the index.
//   pwFlags    [out] -- Returns GIL_* flags
//  Returns:
//   S_OK, if it returns a valid location; S_FALSE, if the shell use a
//   default icon.
//
//  Notes: The location may or may not be a path to a file. The caller can
//   not assume anything unless the subsequent Extract member call returns
//   S_FALSE.
//
//   if the returned location is not a path to a file, GIL_NOTFILENAME should
//   be set in the returned flags.
//
// IExtractIcon::Extract
//
//  This function extracts an icon image from a specified file.
//
//  Parameters:
//   pszFile [in] -- Specifies the icon location (typically a path to a file).
//   nIconIndex [in] -- Specifies the icon index.
//   phiconLarge [out] -- Specifies the HICON variable for large icon.
//   phiconSmall [out] -- Specifies the HICON variable for small icon.
//   nIconSize [in] -- Specifies the size icon required (size of large icon)
//                     LOWORD is the requested large icon size
//                     HIWORD is the requested small icon size
//  Returns:
//   S_OK, if it extracted the from the file.
//   S_FALSE, if the caller should extract from the file specified in the
//           location.
//
//===========================================================================

// GetIconLocation() input flags

#define GIL_OPENICON            0x0001           // allows containers to specify an "open" look
#define GIL_FORSHELL            0x0002           // icon is to be displayed in a ShellFolder
#define GIL_ASYNC               0x0020           // this is an async extract, return E_PENDING
#define GIL_DEFAULTICON         0x0040           // get the default icon location if the final one takes too long to get
#define GIL_FORSHORTCUT         0x0080           // the icon is for a shortcut to the object
#define GIL_CHECKSHIELD         0x0200           // return GIL_SHIELD or GIL_FORCENOSHIELD, don't block if GIL_ASYNC is set

// GetIconLocation() return flags

#define GIL_SIMULATEDOC         0x0001           // simulate this document icon for this
#define GIL_PERINSTANCE         0x0002           // icons from this class are per instance (each file has its own)
#define GIL_PERCLASS            0x0004           // icons from this class per class (shared for all files of this type)
#define GIL_NOTFILENAME         0x0008           // location is not a filename, must call ::ExtractIcon
#define GIL_DONTCACHE           0x0010           // this icon should not be cached
#define GIL_SHIELD              0x0200           // icon should be "stamped" with the LUA shield
#define GIL_FORCENOSHIELD       0x0400           // icon must *not* be "stamped" with the LUA shield

#undef  INTERFACE
#define INTERFACE   IExtractIconA

DECLARE_INTERFACE_IID_(IExtractIconA, IUnknown, "000214eb-0000-0000-c000-000000000046")
{
    STDMETHOD(GetIconLocation)(THIS_
                               UINT   uFlags,
          _Out_writes_(cchMax) PSTR   pszIconFile,
                               UINT   cchMax,
                         _Out_ int   * piIndex,
                         _Out_ UINT  * pwFlags) PURE;

    STDMETHOD(Extract)(THIS_
                 _In_  PCSTR   pszFile,
                       UINT    nIconIndex,
             _Out_opt_ HICON   *phiconLarge,
             _Out_opt_ HICON   *phiconSmall,
                       UINT    nIconSize) PURE;
};

typedef IExtractIconA * LPEXTRACTICONA;

#undef  INTERFACE
#define INTERFACE   IExtractIconW

DECLARE_INTERFACE_IID_(IExtractIconW, IUnknown, "000214fa-0000-0000-c000-000000000046")
{
    STDMETHOD(GetIconLocation)(THIS_
                               UINT   uFlags,
          _Out_writes_(cchMax) PWSTR  pszIconFile,
                               UINT   cchMax,
                         _Out_ int   * piIndex,
                         _Out_ UINT  * pwFlags) PURE;

    STDMETHOD(Extract)(THIS_
                  _In_ PCWSTR  pszFile,
                       UINT    nIconIndex,
             _Out_opt_ HICON   *phiconLarge,
             _Out_opt_ HICON   *phiconSmall,
                       UINT    nIconSize) PURE;
};

typedef IExtractIconW * LPEXTRACTICONW;

#ifdef UNICODE
#define IExtractIcon        IExtractIconW
#define IExtractIconVtbl    IExtractIconWVtbl
#define LPEXTRACTICON       LPEXTRACTICONW
#else
#define IExtractIcon        IExtractIconA
#define IExtractIconVtbl    IExtractIconAVtbl
#define LPEXTRACTICON       LPEXTRACTICONA
#endif

//===========================================================================
//
// IShellIconOverlayManager
//
// Used to return the icon overlay information including OverlayIndex, Image Index or Priority for an IShellFolder object.
//
// IShellIconOverlayManager:GetFileOverlayInfo(LPCWSTR pwszPath, DWORD dwAttrib, int * pIndex, DWORD dwflags)
//      pwszPath        full path of the file
//      dwAttrib        attribute of this file
//      pIndex          pointer to the Icon Index in the system image list
//      pOverlayIndex   pointer to the OverlayIndex in the system image list
//      pPriority       pointer to the Priority of this overlay
// IShellIconOverlayManager:GetReservedOverlayInfo(LPCWSTR pwszPath, DWORD dwAttrib, int * pIndex, DWORD dwflags, int iReservedID)
//      iReservedID     reserved icon overlay id
//  returns:
//      S_OK,  if the index of an Overlay is found
//      S_FALSE, if no Overlay exists for this file
//      E_FAIL, if lpfd is bad
// IShellIconOverlayManager:RefreshOverlayImages(DWORD dwFlags)
//      This will refresh the overlay cache, depends on the dwFlags passed in
//      It will reload the icons into the imagelist, when passed SIOM_ICONINDEX
// IShellIconOverlayManager::LoadNonloadedOverlayIdentifiers()
//      This method loads any registered overlay identifiers (handlers) that
//      are not currently loaded.
// IShellIconOverlayManager::OverlayIndexFromImageIndex(int iImage, int *piIndex, BOOL fAdd)
//      iImage          existing shell image list index to look for
//      piIndex         returned overlay index
//      fAdd            Add image if not already present?
//===========================================================================

#undef  INTERFACE
#define INTERFACE   IShellIconOverlayManager

DECLARE_INTERFACE_IID_(IShellIconOverlayManager, IUnknown, "f10b5e34-dd3b-42a7-aa7d-2f4ec54bb09b")
{
    STDMETHOD(GetFileOverlayInfo)(THIS_ _In_ PCWSTR pwszPath, DWORD dwAttrib, _Out_ int * pIndex, DWORD dwflags) PURE;
    STDMETHOD(GetReservedOverlayInfo)(THIS_ _In_opt_ PCWSTR pwszPath, DWORD dwAttrib, _Out_ int * pIndex, DWORD dwflags, int iReservedID) PURE;
    STDMETHOD(RefreshOverlayImages)(THIS_ DWORD dwFlags) PURE;
    STDMETHOD(LoadNonloadedOverlayIdentifiers)(THIS) PURE;
    STDMETHOD(OverlayIndexFromImageIndex)(THIS_ int iImage, _Out_ int * piIndex, BOOL fAdd) PURE;
};

#define SIOM_OVERLAYINDEX         0x1
#define SIOM_ICONINDEX            0x2
#define SIOM_RESERVED_SHARED      0
#define SIOM_RESERVED_LINK        1
#define SIOM_RESERVED_SLOWFILE    2
#define SIOM_RESERVED_DEFAULT     3

//===========================================================================
//
// IShellIconOverlay
//
// Used to return the icon overlay index or its icon index for an IShellFolder object,
// this is always implemented with IShellFolder
//
// IShellIconOverlay:GetOverlayIndex(LPCITEMIDLIST pidl, DWORD * pdwIndex)
//      pidl            object to identify icon overlay for.
//      pdwIndex        the Overlay Index in the system image list
//
// IShellIconOverlay:GetOverlayIconIndex(LPCITEMIDLIST pidl, DWORD * pdwIndex)
//      pdwIconIndex    the Overlay Icon index in the system image list
// This method is only used for those who are interested in seeing the real bits
// of the Overlay Icon
//
//  returns:
//      S_OK,  if the index of an Overlay is found
//      S_FALSE, if no Overlay exists for this file
//      E_FAIL, if pidl is bad
//
//===========================================================================

#undef  INTERFACE
#define INTERFACE   IShellIconOverlay

DECLARE_INTERFACE_IID_(IShellIconOverlay, IUnknown, "7d688a70-c613-11d0-999b-00c04fd655e1")
{
    STDMETHOD(GetOverlayIndex)(THIS_ _In_ PCUITEMID_CHILD pidl, _Inout_ int * pIndex) PURE;
    STDMETHOD(GetOverlayIconIndex)(THIS_ _In_ PCUITEMID_CHILD pidl, _Inout_ int * pIconIndex) PURE;
};

#define OI_DEFAULT 0x00000000
#define OI_ASYNC 0xFFFFEEEE

//-------------------------------------------------------------------------
//
// SHGetIconOverlayIndex
//
// This function takes the path and icon/res id to the icon and convert it into
// an overlay index in the system image list.
// Note: there are totally only 15 slots for system image overlays, some of which
// was reserved by the system, or taken by the overlayidentifiers, so it's possible
// that this function would fail and return -1;
//
// To get the default overlays in the system, such as the share hand, link shortcut
// and slow files, pass NULL as the file name, then the IDO_SHGIOI_* flags as the icon index
//-------------------------------------------------------------------------

#define IDO_SHGIOI_SHARE  0x0FFFFFFF
#define IDO_SHGIOI_LINK   0x0FFFFFFE
#define IDO_SHGIOI_SLOWFILE 0x0FFFFFFFD
#define IDO_SHGIOI_DEFAULT  0x0FFFFFFFC
SHSTDAPI_(int) SHGetIconOverlayIndexA(_In_opt_ LPCSTR pszIconPath, int iIconIndex);
SHSTDAPI_(int) SHGetIconOverlayIndexW(_In_opt_ LPCWSTR pszIconPath, int iIconIndex);
#ifdef UNICODE
#define SHGetIconOverlayIndex  SHGetIconOverlayIndexW
#else
#define SHGetIconOverlayIndex  SHGetIconOverlayIndexA
#endif // !UNICODE


// IShellLinkDataList::GetFlags()/SetFlags()
typedef enum {
    SLDF_DEFAULT                                = 0x00000000,
    SLDF_HAS_ID_LIST                            = 0x00000001,   // Shell link saved with ID list
    SLDF_HAS_LINK_INFO                          = 0x00000002,   // Shell link saved with LinkInfo
    SLDF_HAS_NAME                               = 0x00000004,
    SLDF_HAS_RELPATH                            = 0x00000008,
    SLDF_HAS_WORKINGDIR                         = 0x00000010,
    SLDF_HAS_ARGS                               = 0x00000020,
    SLDF_HAS_ICONLOCATION                       = 0x00000040,
    SLDF_UNICODE                                = 0x00000080,   // the strings are unicode
    SLDF_FORCE_NO_LINKINFO                      = 0x00000100,   // disable LINKINFO tracking information (used to track network drives and compute UNC paths if one exists)
    SLDF_HAS_EXP_SZ                             = 0x00000200,   // the link contains expandable env strings
    SLDF_RUN_IN_SEPARATE                        = 0x00000400,   // Run the 16-bit target exe in a separate VDM/WOW
#if (NTDDI_VERSION < NTDDI_VISTA)
    SLDF_HAS_LOGO3ID                            = 0x00000800,   // not used anymore
#endif
    SLDF_HAS_DARWINID                           = 0x00001000,   // MSI (Darwin) link that can be installed on demand
    SLDF_RUNAS_USER                             = 0x00002000,   // Run target as a different user
    SLDF_HAS_EXP_ICON_SZ                        = 0x00004000,   // contains expandable env string for icon path
#if (NTDDI_VERSION >= NTDDI_WINXP)
    SLDF_NO_PIDL_ALIAS                          = 0x00008000,   // disable IDList alias mapping when parsing the IDList from the path
    SLDF_FORCE_UNCNAME                          = 0x00010000,   // make GetPath() prefer the UNC name to the local name
    SLDF_RUN_WITH_SHIMLAYER                     = 0x00020000,   // activate target of this link with shim layer active
#if (NTDDI_VERSION >= NTDDI_VISTA)
    SLDF_FORCE_NO_LINKTRACK                     = 0x00040000,   // disable ObjectID tracking information
    SLDF_ENABLE_TARGET_METADATA                 = 0x00080000,   // enable caching of target metadata into link
    SLDF_DISABLE_LINK_PATH_TRACKING             = 0x00100000,   // disable EXP_SZ_LINK_SIG tracking
    SLDF_DISABLE_KNOWNFOLDER_RELATIVE_TRACKING  = 0x00200000,   // disable KnownFolder tracking information (EXP_KNOWN_FOLDER)
#if (NTDDI_VERSION >= NTDDI_WIN7)
    SLDF_NO_KF_ALIAS                            = 0x00400000,   // disable Known Folder alias mapping when loading the IDList during deserialization
    SLDF_ALLOW_LINK_TO_LINK                     = 0x00800000,   // allows this link to point to another shell link - must only be used when it is not possible to create cycles
    SLDF_UNALIAS_ON_SAVE                        = 0x01000000,   // unalias the IDList when saving
    SLDF_PREFER_ENVIRONMENT_PATH                = 0x02000000,   // the IDList is not persisted, instead it is recalculated from the path with environmental variables at load time
                                                                // we don't hit the disk to recalculate the IDList (the result is a simple IDList).  also Resolve does nothing
                                                                // if SetPath is called and the path does not have environmental variable in it, SLDF_PREFER_ENVIRONMENT_PATH is removed
    SLDF_KEEP_LOCAL_IDLIST_FOR_UNC_TARGET       = 0x04000000,   // if target is a UNC location on a local machine, keep the local target in addition to the remote one
#if (NTDDI_VERSION >= NTDDI_WIN8)
    SLDF_PERSIST_VOLUME_ID_RELATIVE             = 0x08000000,   // persist target idlist in its volume ID-relative form to avoid dependency on drive letters
    SLDF_VALID                                  = 0x0FFFF7FF,   // bits that are valid for ::SetFlags()
#else
    SLDF_VALID                                  = 0x07FFF7FF,   // bits that are valid for ::SetFlags()
#endif
#else
    SLDF_VALID                                  = 0x003FF7FF,   // bits that are valid for ::SetFlags()
#endif
#endif
    SLDF_RESERVED                               = (int) 0x80000000,   // Reserved-- so we can use the low word as an index value in the future
#endif
} SHELL_LINK_DATA_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(SHELL_LINK_DATA_FLAGS)

#if !defined(__cplusplus) && defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#endif

typedef struct tagDATABLOCKHEADER
{
    DWORD   cbSize;             // Size of this extra data block
    DWORD   dwSignature;        // signature of this extra data block
} DATABLOCK_HEADER, *LPDATABLOCK_HEADER, *LPDBLIST;

#ifdef LF_FACESIZE
typedef struct {
#ifdef __cplusplus
    DATABLOCK_HEADER dbh;
#else
    DATABLOCK_HEADER DUMMYSTRUCTNAME;
#endif
    WORD     wFillAttribute;         // fill attribute for console
    WORD     wPopupFillAttribute;    // fill attribute for console popups
    COORD    dwScreenBufferSize;     // screen buffer size for console
    COORD    dwWindowSize;           // window size for console
    COORD    dwWindowOrigin;         // window origin for console
    DWORD    nFont;
    DWORD    nInputBufferSize;
    COORD    dwFontSize;
    UINT     uFontFamily;
    UINT     uFontWeight;
    WCHAR    FaceName[LF_FACESIZE];
    UINT     uCursorSize;
    BOOL     bFullScreen;
    BOOL     bQuickEdit;
    BOOL     bInsertMode;
    BOOL     bAutoPosition;
    UINT     uHistoryBufferSize;
    UINT     uNumberOfHistoryBuffers;
    BOOL     bHistoryNoDup;
    COLORREF ColorTable[ 16 ];
} NT_CONSOLE_PROPS, *LPNT_CONSOLE_PROPS;
#define NT_CONSOLE_PROPS_SIG 0xA0000002
#endif  // LF_FACESIZE

// This is a FE Console property
typedef struct {
#ifdef __cplusplus
    DATABLOCK_HEADER dbh;
#else
    DATABLOCK_HEADER DUMMYSTRUCTNAME;
#endif
    UINT     uCodePage;
} NT_FE_CONSOLE_PROPS, *LPNT_FE_CONSOLE_PROPS;
#define NT_FE_CONSOLE_PROPS_SIG 0xA0000004

typedef struct {
#ifdef __cplusplus
    DATABLOCK_HEADER dbh;
#else
    DATABLOCK_HEADER DUMMYSTRUCTNAME;
#endif
    CHAR        szDarwinID[MAX_PATH];  // ANSI darwin ID associated with link
    WCHAR       szwDarwinID[MAX_PATH]; // UNICODE darwin ID associated with link
} EXP_DARWIN_LINK, *LPEXP_DARWIN_LINK;
#define EXP_DARWIN_ID_SIG       0xA0000006

#if !defined(__cplusplus) && defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(pop)
#endif

#define EXP_SPECIAL_FOLDER_SIG         0xA0000005   // LPEXP_SPECIAL_FOLDER


typedef struct
{
    DWORD       cbSize;             // Size of this extra data block
    DWORD       dwSignature;        // signature of this extra data block
    DWORD       idSpecialFolder;    // special folder id this link points into
    DWORD       cbOffset;           // ofset into pidl from SLDF_HAS_ID_LIST for child
} EXP_SPECIAL_FOLDER, *LPEXP_SPECIAL_FOLDER;


typedef struct
{
    DWORD       cbSize;             // Size of this extra data block
    DWORD       dwSignature;        // signature of this extra data block
    CHAR        szTarget[ MAX_PATH ];   // ANSI target name w/EXP_SZ in it
    WCHAR       swzTarget[ MAX_PATH ];  // UNICODE target name w/EXP_SZ in it
} EXP_SZ_LINK, *LPEXP_SZ_LINK;
#define EXP_SZ_LINK_SIG                0xA0000001   // LPEXP_SZ_LINK (target)
#define EXP_SZ_ICON_SIG                0xA0000007   // LPEXP_SZ_LINK (icon)

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef struct
{
    DWORD       cbSize;             // Size of this extra data block
    DWORD       dwSignature;        // signature of this extra data block
    BYTE abPropertyStorage[1];
} EXP_PROPERTYSTORAGE;
#define EXP_PROPERTYSTORAGE_SIG     0xA0000009
#endif // NTDDI_VISTA

#ifdef _INC_SHELLAPI    /* for LPSHELLEXECUTEINFO */
//===========================================================================
//
// IShellExecuteHook Interface
//
//===========================================================================

#undef  INTERFACE
#define INTERFACE   IShellExecuteHookA

DECLARE_INTERFACE_IID_(IShellExecuteHookA, IUnknown, "000214f5-0000-0000-c000-000000000046")
{
    STDMETHOD(Execute)(THIS_ _Inout_ LPSHELLEXECUTEINFOA pei) PURE;
};

#undef  INTERFACE
#define INTERFACE   IShellExecuteHookW

DECLARE_INTERFACE_IID_(IShellExecuteHookW, IUnknown, "000214fb-0000-0000-c000-000000000046")
{
    STDMETHOD(Execute)(THIS_ _Inout_ LPSHELLEXECUTEINFOW pei) PURE;
};

#ifdef UNICODE
#define IShellExecuteHook       IShellExecuteHookW
#define IShellExecuteHookVtbl   IShellExecuteHookWVtbl
#else
#define IShellExecuteHook       IShellExecuteHookA
#define IShellExecuteHookVtbl   IShellExecuteHookAVtbl
#endif // UNICODE
#endif // _INC_SHELLAPI

//===========================================================================
//
// IURLSearchHook Interface
//
//===========================================================================

#undef  INTERFACE
#define INTERFACE   IURLSearchHook

DECLARE_INTERFACE_IID_(IURLSearchHook, IUnknown, "ac60f6a0-0fd9-11d0-99cb-00c04fd64497")
{
    STDMETHOD(Translate)(THIS_ _Out_writes_(cchBufferSize) PWSTR pwszSearchURL, DWORD cchBufferSize) PURE;
};

#undef  INTERFACE
#define INTERFACE   ISearchContext

DECLARE_INTERFACE_IID_(ISearchContext, IUnknown, "09F656A2-41AF-480C-88F7-16CC0D164615")
{
    STDMETHOD(GetSearchUrl)(THIS_ _Outptr_ BSTR * pbstrSearchUrl) PURE;
    STDMETHOD(GetSearchText)(THIS_ _Outptr_ BSTR * pbstrSearchText) PURE;
    STDMETHOD(GetSearchStyle)(THIS_ _Out_ DWORD * pdwSearchStyle) PURE;
};

#undef  INTERFACE
#define INTERFACE   IURLSearchHook2

DECLARE_INTERFACE_IID_(IURLSearchHook2, IURLSearchHook, "5ee44da4-6d32-46e3-86bc-07540dedd0e0")
{
    STDMETHOD(TranslateWithSearchContext)(THIS_ _Out_writes_(cchBufferSize) PWSTR pwszSearchURL, DWORD cchBufferSize, _In_opt_ ISearchContext * pSearchContext) PURE;
};

//--------------------------------------------------------------------------
//
// Command/menuitem IDs
//
//  The explorer dispatches WM_COMMAND messages based on the range of
// command/menuitem IDs. All the IDs of menuitems that the view (right
// pane) inserts must be in FCIDM_SHVIEWFIRST/LAST (otherwise, the explorer
// won't dispatch them). The view should not deal with any menuitems
// in FCIDM_BROWSERFIRST/LAST (otherwise, it won't work with the future
// version of the shell).
//
//  FCIDM_SHVIEWFIRST/LAST      for the right pane (IShellView)
//  FCIDM_BROWSERFIRST/LAST     for the explorer frame (IShellBrowser)
//  FCIDM_GLOBAL/LAST           for the explorer's submenu IDs
//
//--------------------------------------------------------------------------

#define FCIDM_SHVIEWFIRST           0x0000
#define FCIDM_SHVIEWLAST            0x7fff
#define FCIDM_BROWSERFIRST          0xa000
#define FCIDM_BROWSERLAST           0xbf00
#define FCIDM_GLOBALFIRST           0x8000
#define FCIDM_GLOBALLAST            0x9fff

//
// Global submenu IDs and separator IDs
//
#define FCIDM_MENU_FILE             (FCIDM_GLOBALFIRST+0x0000)
#define FCIDM_MENU_EDIT             (FCIDM_GLOBALFIRST+0x0040)
#define FCIDM_MENU_VIEW             (FCIDM_GLOBALFIRST+0x0080)
#define FCIDM_MENU_VIEW_SEP_OPTIONS (FCIDM_GLOBALFIRST+0x0081)
#define FCIDM_MENU_TOOLS            (FCIDM_GLOBALFIRST+0x00c0) // for Win9x compat
#define FCIDM_MENU_TOOLS_SEP_GOTO   (FCIDM_GLOBALFIRST+0x00c1) // for Win9x compat
#define FCIDM_MENU_HELP             (FCIDM_GLOBALFIRST+0x0100)
#define FCIDM_MENU_FIND             (FCIDM_GLOBALFIRST+0x0140)
#define FCIDM_MENU_EXPLORE          (FCIDM_GLOBALFIRST+0x0150)
#define FCIDM_MENU_FAVORITES        (FCIDM_GLOBALFIRST+0x0170)

SHSTDAPI_(PIDLIST_RELATIVE)     ILClone(_In_ PCUIDLIST_RELATIVE pidl);
SHSTDAPI_(PITEMID_CHILD)        ILCloneFirst(_In_ PCUIDLIST_RELATIVE pidl);
SHSTDAPI_(PIDLIST_ABSOLUTE)     ILCombine(_In_opt_ PCIDLIST_ABSOLUTE pidl1, _In_opt_ PCUIDLIST_RELATIVE pidl2);
SHSTDAPI_(void)                 ILFree(_In_opt_ PIDLIST_RELATIVE pidl);
SHSTDAPI_(PUIDLIST_RELATIVE)    ILGetNext(_In_opt_ PCUIDLIST_RELATIVE pidl);
SHSTDAPI_(UINT)                 ILGetSize(_In_opt_ PCUIDLIST_RELATIVE pidl);
SHSTDAPI_(PUIDLIST_RELATIVE)    ILFindChild(_In_ PIDLIST_ABSOLUTE pidlParent, _In_ PCIDLIST_ABSOLUTE pidlChild);
SHSTDAPI_(PUITEMID_CHILD)       ILFindLastID(_In_ PCUIDLIST_RELATIVE pidl);
SHSTDAPI_(BOOL)                 ILRemoveLastID(_Inout_opt_ PUIDLIST_RELATIVE pidl);
SHSTDAPI_(BOOL)                 ILIsEqual(_In_ PCIDLIST_ABSOLUTE pidl1, _In_ PCIDLIST_ABSOLUTE pidl2);
SHSTDAPI_(BOOL)                 ILIsParent(_In_ PCIDLIST_ABSOLUTE pidl1, _In_ PCIDLIST_ABSOLUTE pidl2, BOOL fImmediate);
SHSTDAPI                        ILSaveToStream(_In_ IStream *pstm, _In_ PCUIDLIST_RELATIVE pidl);
EXTERN_C DECLSPEC_DEPRECATED HRESULT STDAPICALLTYPE ILLoadFromStream(_In_ IStream *pstm, _Inout_ PIDLIST_RELATIVE *pidl); // use ILLoadFromStreamEx instead
#if (NTDDI_VERSION >= NTDDI_VISTA)
SHSTDAPI                        ILLoadFromStreamEx(_In_ IStream *pstm, _Outptr_ PIDLIST_RELATIVE *pidl);
#endif // NTDDI_VISTA

SHSTDAPI_(PIDLIST_ABSOLUTE)     ILCreateFromPathA(_In_ PCSTR pszPath);
SHSTDAPI_(PIDLIST_ABSOLUTE)     ILCreateFromPathW(_In_ PCWSTR pszPath);

#ifdef NO_WRAPPERS_FOR_ILCREATEFROMPATH
SHSTDAPI_(PIDLIST_ABSOLUTE)     ILCreateFromPath(_In_ PCTSTR pszPath);
#else
#ifdef UNICODE
#define ILCreateFromPath        ILCreateFromPathW
#else
#define ILCreateFromPath        ILCreateFromPathA
#endif  // !UNICODE
#endif  // NO_WRAPPERS_FOR_ILCREATEFROMPATH

SHSTDAPI SHILCreateFromPath(_In_ PCWSTR pszPath, _Outptr_ PIDLIST_ABSOLUTE *ppidl, _Inout_opt_ DWORD *rgfInOut);


#define VOID_OFFSET(pv, cb)     ((void*)(((BYTE*)(pv))+(cb)))

#if defined(STRICT_TYPED_ITEMIDS) && defined(__cplusplus)

} // extern "C"

inline PIDLIST_ABSOLUTE ILCloneFull(_In_ PCUIDLIST_ABSOLUTE pidl) { return reinterpret_cast<PIDLIST_ABSOLUTE>(ILClone(pidl)); }
inline PITEMID_CHILD    ILCloneChild(_In_ PCUITEMID_CHILD pidl)   { return ILCloneFirst(pidl); }

#if (NTDDI_VERSION >= NTDDI_VISTA)
inline HRESULT ILLoadFromStreamEx(_In_ IStream *pstm, _Outptr_ PIDLIST_ABSOLUTE *ppidl) { return ILLoadFromStreamEx(pstm, reinterpret_cast<PIDLIST_RELATIVE*>(ppidl)); }
inline HRESULT ILLoadFromStreamEx(_In_ IStream *pstm, _Outptr_ PITEMID_CHILD *ppidl)    { return ILLoadFromStreamEx(pstm, reinterpret_cast<PIDLIST_RELATIVE*>(ppidl)); }
#endif // NTDDI_VISTA

inline PCUIDLIST_RELATIVE ILSkip(_In_ PCUIDLIST_RELATIVE pidl, UINT cb) { return reinterpret_cast<PCUIDLIST_RELATIVE>(VOID_OFFSET(pidl, cb)); }
inline PUIDLIST_RELATIVE  ILSkip(_In_ PUIDLIST_RELATIVE  pidl, UINT cb) { return const_cast<PUIDLIST_RELATIVE>(ILSkip(const_cast<PCUIDLIST_RELATIVE>(pidl), cb)); }

inline PCUIDLIST_RELATIVE ILNext(_In_ PCUIDLIST_RELATIVE pidl) { return ILSkip(pidl, pidl->mkid.cb); }
inline PUIDLIST_RELATIVE  ILNext(_In_ PUIDLIST_RELATIVE  pidl) { return const_cast<PUIDLIST_RELATIVE>(ILNext(const_cast<PCUIDLIST_RELATIVE>(pidl))); }

inline BOOL ILIsAligned(_In_ PCUIDLIST_RELATIVE pidl)       { return (((DWORD_PTR)(pidl) & (sizeof(void*) - 1)) == 0); }

inline BOOL ILIsEmpty(_In_opt_ PCUIDLIST_RELATIVE pidl)     { return ((pidl == NULL) || (pidl->mkid.cb==0)); }

// ILIsChild does not guarantee that pidl is non-null or non-empty.
inline BOOL ILIsChild(_In_opt_ PCUIDLIST_RELATIVE pidl)         { return (ILIsEmpty(pidl) || ILIsEmpty(ILNext(pidl))); }

#if defined(STRICT_TYPED_ITEMIDS) && defined(__cplusplus)
inline PCUIDLIST_RELATIVE       ILFindChild(_In_ PCIDLIST_ABSOLUTE pidlParent, _In_ PCIDLIST_ABSOLUTE pidlChild) { return const_cast<PCUIDLIST_RELATIVE>(ILFindChild(const_cast<PIDLIST_ABSOLUTE>(pidlParent), pidlChild)); }
#endif // defined(STRICT_TYPED_ITEMIDS) && defined(__cplusplus)

extern "C" {

#else // !defined(STRICT_TYPED_ITEMIDS) || !defined(__cplusplus)

#define ILCloneFull             ILClone
#define ILCloneChild            ILCloneFirst

#define ILSkip(pidl, cb)        ((PUIDLIST_RELATIVE)VOID_OFFSET((pidl), (cb)))
#define ILNext(pidl)            ILSkip(pidl, (pidl)->mkid.cb)

#define ILIsAligned(pidl)       (((DWORD_PTR)(pidl) & (sizeof(void*) - 1)) == 0)

#define ILIsEmpty(pidl)         ((pidl) == NULL || (pidl)->mkid.cb==0)

// ILIsChild does not guarantee that pidl is non-null or non-empty.
#define ILIsChild(pidl)         (ILIsEmpty(pidl) || ILIsEmpty(ILNext(pidl)))

#endif // defined(STRICT_TYPED_ITEMIDS) && defined(__cplusplus)

SHSTDAPI_(PIDLIST_RELATIVE) ILAppendID(_In_opt_ PIDLIST_RELATIVE pidl, _In_ LPCSHITEMID pmkid, BOOL fAppend);


#if (NTDDI_VERSION >= NTDDI_VISTA)
// SHGetPathFromIDListEx returns a win32 file system path for the item in the name space.
//  and has a few special cases that include returning UNC printer names too!
enum
{
    GPFIDL_DEFAULT    = 0x0000,      // normal Win32 file name, servers and drive roots included
    GPFIDL_ALTNAME    = 0x0001,      // short file name
    GPFIDL_UNCPRINTER = 0x0002,      // include UNC printer names too (non file system item)
};


typedef int GPFIDL_FLAGS;
SHSTDAPI_(BOOL) SHGetPathFromIDListEx(_In_ PCIDLIST_ABSOLUTE pidl, _Out_writes_(cchPath) PWSTR pszPath, DWORD cchPath, GPFIDL_FLAGS uOpts);
#endif // NTDDI_VISTA
//
// SHGetPathFromIDListW is the old version of SHGetPathFromIDListEx that assumes the size of the buffer (MAX_PATH).
// The pidl should point to a file system object.
_Success_(return != 0)
SHSTDAPI_(BOOL) SHGetPathFromIDListA(_In_ PCIDLIST_ABSOLUTE pidl, _Out_writes_(MAX_PATH) LPSTR pszPath);
//
// SHGetPathFromIDListW is the old version of SHGetPathFromIDListEx that assumes the size of the buffer (MAX_PATH).
// The pidl should point to a file system object.
_Success_(return != 0)
SHSTDAPI_(BOOL) SHGetPathFromIDListW(_In_ PCIDLIST_ABSOLUTE pidl, _Out_writes_(MAX_PATH) LPWSTR pszPath);
#ifdef UNICODE
#define SHGetPathFromIDList  SHGetPathFromIDListW
#else
#define SHGetPathFromIDList  SHGetPathFromIDListA
#endif // !UNICODE

SHSTDAPI_(int) SHCreateDirectory(_In_opt_ HWND hwnd, _In_ PCWSTR pszPath);
SHSTDAPI_(int) SHCreateDirectoryExA(_In_opt_ HWND hwnd, _In_ LPCSTR pszPath, _In_opt_ const SECURITY_ATTRIBUTES *psa);
SHSTDAPI_(int) SHCreateDirectoryExW(_In_opt_ HWND hwnd, _In_ LPCWSTR pszPath, _In_opt_ const SECURITY_ATTRIBUTES *psa);
#ifdef UNICODE
#define SHCreateDirectoryEx  SHCreateDirectoryExW
#else
#define SHCreateDirectoryEx  SHCreateDirectoryExA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define OFASI_EDIT          0x0001
#define OFASI_OPENDESKTOP   0x0002
#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)

SHSTDAPI SHOpenFolderAndSelectItems(_In_ PCIDLIST_ABSOLUTE pidlFolder, UINT cidl, _In_reads_opt_(cidl) PCUITEMID_CHILD_ARRAY apidl, DWORD dwFlags);

//  deprecated because of parameter ambiguity
//  call SHCreateItemWithParent() or SHCreateItemFromIDList() instead
SHSTDAPI SHCreateShellItem(_In_opt_ PCIDLIST_ABSOLUTE pidlParent, _In_opt_ IShellFolder *psfParent, _In_ PCUITEMID_CHILD pidl, _Outptr_ IShellItem **ppsi);

#endif


//
// SHGetSpecialFolderLocation
//
//  Caller should use SHGetMalloc to obtain an allocator that can free the pidl
//
// registry entries for special paths are kept in :
#define REGSTR_PATH_SPECIAL_FOLDERS     REGSTR_PATH_EXPLORER TEXT("\\Shell Folders")

#define CSIDL_DESKTOP                   0x0000        // <desktop>
#define CSIDL_INTERNET                  0x0001        // Internet Explorer (icon on desktop)
#define CSIDL_PROGRAMS                  0x0002        // Start Menu\Programs
#define CSIDL_CONTROLS                  0x0003        // My Computer\Control Panel
#define CSIDL_PRINTERS                  0x0004        // My Computer\Printers
#define CSIDL_PERSONAL                  0x0005        // My Documents
#define CSIDL_FAVORITES                 0x0006        // <user name>\Favorites
#define CSIDL_STARTUP                   0x0007        // Start Menu\Programs\Startup
#define CSIDL_RECENT                    0x0008        // <user name>\Recent
#define CSIDL_SENDTO                    0x0009        // <user name>\SendTo
#define CSIDL_BITBUCKET                 0x000a        // <desktop>\Recycle Bin
#define CSIDL_STARTMENU                 0x000b        // <user name>\Start Menu
#define CSIDL_MYDOCUMENTS               CSIDL_PERSONAL //  Personal was just a silly name for My Documents
#define CSIDL_MYMUSIC                   0x000d        // "My Music" folder
#define CSIDL_MYVIDEO                   0x000e        // "My Videos" folder
#define CSIDL_DESKTOPDIRECTORY          0x0010        // <user name>\Desktop
#define CSIDL_DRIVES                    0x0011        // My Computer
#define CSIDL_NETWORK                   0x0012        // Network Neighborhood (My Network Places)
#define CSIDL_NETHOOD                   0x0013        // <user name>\nethood
#define CSIDL_FONTS                     0x0014        // windows\fonts
#define CSIDL_TEMPLATES                 0x0015
#define CSIDL_COMMON_STARTMENU          0x0016        // All Users\Start Menu
#define CSIDL_COMMON_PROGRAMS           0X0017        // All Users\Start Menu\Programs
#define CSIDL_COMMON_STARTUP            0x0018        // All Users\Startup
#define CSIDL_COMMON_DESKTOPDIRECTORY   0x0019        // All Users\Desktop
#define CSIDL_APPDATA                   0x001a        // <user name>\Application Data
#define CSIDL_PRINTHOOD                 0x001b        // <user name>\PrintHood

#ifndef CSIDL_LOCAL_APPDATA
#define CSIDL_LOCAL_APPDATA             0x001c        // <user name>\Local Settings\Applicaiton Data (non roaming)
#endif // CSIDL_LOCAL_APPDATA

#define CSIDL_ALTSTARTUP                0x001d        // non localized startup
#define CSIDL_COMMON_ALTSTARTUP         0x001e        // non localized common startup
#define CSIDL_COMMON_FAVORITES          0x001f

#ifndef _SHFOLDER_H_
#define CSIDL_INTERNET_CACHE            0x0020
#define CSIDL_COOKIES                   0x0021
#define CSIDL_HISTORY                   0x0022
#define CSIDL_COMMON_APPDATA            0x0023        // All Users\Application Data
#define CSIDL_WINDOWS                   0x0024        // GetWindowsDirectory()
#define CSIDL_SYSTEM                    0x0025        // GetSystemDirectory()
#define CSIDL_PROGRAM_FILES             0x0026        // C:\Program Files
#define CSIDL_MYPICTURES                0x0027        // C:\Program Files\My Pictures
#endif // _SHFOLDER_H_

#define CSIDL_PROFILE                   0x0028        // USERPROFILE
#define CSIDL_SYSTEMX86                 0x0029        // x86 system directory on RISC
#define CSIDL_PROGRAM_FILESX86          0x002a        // x86 C:\Program Files on RISC

#ifndef _SHFOLDER_H_
#define CSIDL_PROGRAM_FILES_COMMON      0x002b        // C:\Program Files\Common
#endif // _SHFOLDER_H_

#define CSIDL_PROGRAM_FILES_COMMONX86   0x002c        // x86 Program Files\Common on RISC
#define CSIDL_COMMON_TEMPLATES          0x002d        // All Users\Templates

#ifndef _SHFOLDER_H_
#define CSIDL_COMMON_DOCUMENTS          0x002e        // All Users\Documents
#define CSIDL_COMMON_ADMINTOOLS         0x002f        // All Users\Start Menu\Programs\Administrative Tools
#define CSIDL_ADMINTOOLS                0x0030        // <user name>\Start Menu\Programs\Administrative Tools
#endif // _SHFOLDER_H_

#define CSIDL_CONNECTIONS               0x0031        // Network and Dial-up Connections
#define CSIDL_COMMON_MUSIC              0x0035        // All Users\My Music
#define CSIDL_COMMON_PICTURES           0x0036        // All Users\My Pictures
#define CSIDL_COMMON_VIDEO              0x0037        // All Users\My Video
#define CSIDL_RESOURCES                 0x0038        // Resource Direcotry

#ifndef _SHFOLDER_H_
#define CSIDL_RESOURCES_LOCALIZED       0x0039        // Localized Resource Direcotry
#endif // _SHFOLDER_H_

#define CSIDL_COMMON_OEM_LINKS          0x003a        // Links to All Users OEM specific apps
#define CSIDL_CDBURN_AREA               0x003b        // USERPROFILE\Local Settings\Application Data\Microsoft\CD Burning
// unused                               0x003c
#define CSIDL_COMPUTERSNEARME           0x003d        // Computers Near Me (computered from Workgroup membership)

#ifndef _SHFOLDER_H_
#define CSIDL_FLAG_CREATE               0x8000        // combine with CSIDL_ value to force folder creation in SHGetFolderPath()
#endif // _SHFOLDER_H_

#define CSIDL_FLAG_DONT_VERIFY          0x4000        // combine with CSIDL_ value to return an unverified folder path
#define CSIDL_FLAG_DONT_UNEXPAND        0x2000        // combine with CSIDL_ value to avoid unexpanding environment variables
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CSIDL_FLAG_NO_ALIAS             0x1000        // combine with CSIDL_ value to insure non-alias versions of the pidl
#define CSIDL_FLAG_PER_USER_INIT        0x0800        // combine with CSIDL_ value to indicate per-user init (eg. upgrade)
#endif  // NTDDI_WINXP
#define CSIDL_FLAG_MASK                 0xFF00        // mask for all possible flag values

_Check_return_
STDAPI SHGetSpecialFolderLocation(_Reserved_ HWND hwnd, _In_ int csidl, _Outptr_ PIDLIST_ABSOLUTE *ppidl);

SHSTDAPI_(PIDLIST_ABSOLUTE) SHCloneSpecialIDList(_Reserved_ HWND hwnd, _In_ int csidl, _In_ BOOL fCreate);
_Success_(return != 0)
SHSTDAPI_(BOOL) SHGetSpecialFolderPathA(_Reserved_ HWND hwnd, _Out_writes_(MAX_PATH) LPSTR pszPath, _In_ int csidl, _In_ BOOL fCreate);
_Success_(return != 0)
SHSTDAPI_(BOOL) SHGetSpecialFolderPathW(_Reserved_ HWND hwnd, _Out_writes_(MAX_PATH) LPWSTR pszPath, _In_ int csidl, _In_ BOOL fCreate);
#ifdef UNICODE
#define SHGetSpecialFolderPath  SHGetSpecialFolderPathW
#else
#define SHGetSpecialFolderPath  SHGetSpecialFolderPathA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WIN2K)
SHSTDAPI_(void) SHFlushSFCache(void);

typedef enum {
    SHGFP_TYPE_CURRENT  = 0,   // current value for user, verify it exists
    SHGFP_TYPE_DEFAULT  = 1,   // default value, may not exist
} SHGFP_TYPE;

SHFOLDERAPI SHGetFolderPathA(_Reserved_ HWND hwnd, _In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _Out_writes_(MAX_PATH) LPSTR pszPath);
SHFOLDERAPI SHGetFolderPathW(_Reserved_ HWND hwnd, _In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _Out_writes_(MAX_PATH) LPWSTR pszPath);
#ifdef UNICODE
#define SHGetFolderPath  SHGetFolderPathW
#else
#define SHGetFolderPath  SHGetFolderPathA
#endif // !UNICODE
SHSTDAPI SHGetFolderLocation(_Reserved_ HWND hwnd, _In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _Outptr_ PIDLIST_ABSOLUTE *ppidl);
#endif  // NTDDI_WIN2K

#if (NTDDI_VERSION >= NTDDI_WINXP)
STDAPI SHSetFolderPathA(_In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _In_ LPCSTR pszPath);
STDAPI SHSetFolderPathW(_In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _In_ LPCWSTR pszPath);
#ifdef UNICODE
#define SHSetFolderPath  SHSetFolderPathW
#else
#define SHSetFolderPath  SHSetFolderPathA
#endif // !UNICODE
STDAPI SHGetFolderPathAndSubDirA(_Reserved_ HWND hwnd, _In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _In_opt_ LPCSTR pszSubDir, _Out_writes_(MAX_PATH) LPSTR pszPath);
STDAPI SHGetFolderPathAndSubDirW(_Reserved_ HWND hwnd, _In_ int csidl, _In_opt_ HANDLE hToken, _In_ DWORD dwFlags, _In_opt_ LPCWSTR pszSubDir, _Out_writes_(MAX_PATH) LPWSTR pszPath);
#ifdef UNICODE
#define SHGetFolderPathAndSubDir  SHGetFolderPathAndSubDirW
#else
#define SHGetFolderPathAndSubDir  SHGetFolderPathAndSubDirA
#endif // !UNICODE
#endif // NTDDI_VISTA

#if (NTDDI_VERSION >= NTDDI_VISTA)

// flags for Known Folder APIs

typedef enum
{
    KF_FLAG_DEFAULT         = 0x00000000,

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
    // When called from packaged app, LocalAppData/RoamingAppData folders are redirected to
    // app private locations that match the paths returned from WinRT API:
    //   Windows.Storage.ApplicationData.Current.{LocalFolder|RoamingFolder}
    // A few other folders are redirected to subdirectories of LocalAppData.
    KF_FLAG_FORCE_APP_DATA_REDIRECTION = 0x00080000,
#endif //NTDDI_WIN10_RS3

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
    // When running in a Centennial process, some file system locations are redirected to
    // package-specific locations by the file system. Specifying this flag will cause the
    // target of the redirection to be returned for these locations. This is useful in
    // cases where the real location in the file system needs to be known.
    KF_FLAG_RETURN_FILTER_REDIRECTION_TARGET = 0x00040000,

    // New form for KF_FLAG_FORCE_APPCONTAINER_REDIRECTION and KF_FLAG_NO_APPCONTAINER_REDIRECTION
    // The new forms should be used and will exhibit the same behavior as the old flags.

    // When running inside an AppContainer process, or when providing an AppContainer token,
    // some folders are redirected to AppContainer-specific locations within the package's location.
    // Specifying this flag will force this redirection for folders that are not normally
    // redirected for centennial processes. Useful for sharing files between between UWA and
    // Centennial apps that are within the same package.
    KF_FLAG_FORCE_PACKAGE_REDIRECTION = 0x00020000,

    // When running inside a packaged process (Centennial,  AppContainer, etc.), or when providing
    // a packaged process token, some folders are redirected to package-specific locations.
    // Specifying this flag will disable redirection on locations where it applied,
    // and instead return the non-redirected location.
    KF_FLAG_NO_PACKAGE_REDIRECTION = 0x00010000,

    // see comment for KF_FLAG_FORCE_PACKAGE_REDIRECTION
    KF_FLAG_FORCE_APPCONTAINER_REDIRECTION = 0x00020000,
#endif //NTDDI_WIN10_RS2

#if (NTDDI_VERSION >= NTDDI_WIN7)
    // When running inside an AppContainer, or when poviding an AppContainer token, specifying this flag will prevent redirection to AppContainer
    // folders and instead return the path that would be returned when not running inside an AppContainer
    KF_FLAG_NO_APPCONTAINER_REDIRECTION = 0x00010000,
#endif //NTDDI_WIN7

    // Make sure that the folder already exists or create it and apply security specified in folder definition
    // If folder can not be created then function will return failure and no folder path (IDList) will be returned
    // If folder is located on the network the function may take long time to execute
    KF_FLAG_CREATE          = 0x00008000,

    // If this flag is specified then the folder path is returned and no verification is performed
    // Use this flag is you want to get folder's path (IDList) and do not need to verify folder's existence
    //
    // If this flag is NOT specified then Known Folder API will try to verify that the folder exists
    //     If folder does not exist or can not be accessed then function will return failure and no folder path (IDList) will be returned
    //     If folder is located on the network the function may take long time to execute
    KF_FLAG_DONT_VERIFY     = 0x00004000,

    // Set folder path as is and do not try to substitute parts of the path with environments variables.
    // If flag is not specified then Known Folder will try to replace parts of the path with some
    // known environment variables (%USERPROFILE%, %APPDATA% etc.)
    KF_FLAG_DONT_UNEXPAND   = 0x00002000,

    // Get file system based IDList if available. If the flag is not specified the Known Folder API
    // will try to return aliased IDList by default. Example for FOLDERID_Documents -
    // Aliased - [desktop]\[user]\[Documents] - exact location is determined by shell namespace layout and might change
    // Non aliased - [desktop]\[computer]\[disk_c]\[users]\[user]\[Documents] - location is determined by folder location in the file system
    KF_FLAG_NO_ALIAS        = 0x00001000,

    // Initialize the folder with desktop.ini settings
    // If folder can not be initialized then function will return failure and no folder path will be returned
    // If folder is located on the network the function may take long time to execute
    KF_FLAG_INIT            = 0x00000800,

    // Get the default path, will also verify folder existence unless KF_FLAG_DONT_VERIFY is also specified
    KF_FLAG_DEFAULT_PATH    = 0x00000400,

    // Get the not-parent-relative default path. Only valid with KF_FLAG_DEFAULT_PATH
    KF_FLAG_NOT_PARENT_RELATIVE = 0x00000200,

    // Build simple IDList
    KF_FLAG_SIMPLE_IDLIST   = 0x00000100,

    // only return the aliased IDLists, don't fallback to file system path
    KF_FLAG_ALIAS_ONLY      = 0x80000000,
} KNOWN_FOLDER_FLAG;


DEFINE_ENUM_FLAG_OPERATORS(KNOWN_FOLDER_FLAG)

STDAPI SHGetKnownFolderIDList(_In_ REFKNOWNFOLDERID rfid,
                              _In_ DWORD /* KNOWN_FOLDER_FLAG */ dwFlags,
                              _In_opt_ HANDLE hToken,
                              _Outptr_ PIDLIST_ABSOLUTE *ppidl);

STDAPI SHSetKnownFolderPath(_In_ REFKNOWNFOLDERID rfid,
                            _In_ DWORD /* KNOWN_FOLDER_FLAG */ dwFlags,
                            _In_opt_ HANDLE hToken,
                            _In_ PCWSTR pszPath);

STDAPI SHGetKnownFolderPath(_In_ REFKNOWNFOLDERID rfid,
                            _In_ DWORD /* KNOWN_FOLDER_FLAG */ dwFlags,
                            _In_opt_ HANDLE hToken,
                            _Outptr_ PWSTR *ppszPath); // free *ppszPath with CoTaskMemFree

#endif  // NTDDI_VISTA

#if (NTDDI_VERSION >= NTDDI_WIN7)
// returns IShellItem or related interface
STDAPI SHGetKnownFolderItem(_In_ REFKNOWNFOLDERID rfid,
                            _In_ KNOWN_FOLDER_FLAG flags,
                            _In_opt_ HANDLE hToken,
                            _In_ REFIID riid,
                            _Outptr_ void **ppv);
#endif // NTDDI_WIN7

#if (NTDDI_VERSION >= NTDDI_WIN2K)

#define FCS_READ                    0x00000001
#define FCS_FORCEWRITE              0x00000002
#define FCS_WRITE                   (FCS_READ | FCS_FORCEWRITE)

#define FCS_FLAG_DRAGDROP           2

// Mask which values have been retreived or being set.
#define FCSM_VIEWID                 0x00000001    // deprecated
#define FCSM_WEBVIEWTEMPLATE        0x00000002  // deprecated
#define FCSM_INFOTIP                0x00000004
#define FCSM_CLSID                  0x00000008
#define FCSM_ICONFILE               0x00000010
#define FCSM_LOGO                   0x00000020
#define FCSM_FLAGS                  0x00000040

#if (NTDDI_VERSION >= NTDDI_VISTA)
#include <pshpack8.h>

// Used by SHGetSetFolderCustomSettings
typedef struct
{
    DWORD           dwSize;
    DWORD           dwMask;                 // IN/OUT  Which Attributes to Get/Set
    SHELLVIEWID*    pvid;                   // OUT - if dwReadWrite is FCS_READ, IN - otherwise
    // The folder's WebView template path
    LPWSTR          pszWebViewTemplate;     // OUT - if dwReadWrite is FCS_READ, IN - otherwise
    DWORD           cchWebViewTemplate;     // IN - Specifies the size of the buffer pointed to by pszWebViewTemplate
                                            // Ignored if dwReadWrite is FCS_READ
    LPWSTR           pszWebViewTemplateVersion;  // currently IN only
    // Infotip for the folder
    LPWSTR          pszInfoTip;             // OUT - if dwReadWrite is FCS_READ, IN - otherwise
    DWORD           cchInfoTip;             // IN - Specifies the size of the buffer pointed to by pszInfoTip
                                            // Ignored if dwReadWrite is FCS_READ
    // CLSID that points to more info in the registry
    CLSID*          pclsid;                 // OUT - if dwReadWrite is FCS_READ, IN - otherwise
    // Other flags for the folder. Takes FCS_FLAG_* values
    DWORD           dwFlags;                // OUT - if dwReadWrite is FCS_READ, IN - otherwise


    LPWSTR           pszIconFile;           // OUT - if dwReadWrite is FCS_READ, IN - otherwise
    DWORD            cchIconFile;           // IN - Specifies the size of the buffer pointed to by pszIconFile
                                            // Ignored if dwReadWrite is FCS_READ

    int              iIconIndex;            // OUT - if dwReadWrite is FCS_READ, IN - otherwise

    LPWSTR           pszLogo;               // OUT - if dwReadWrite is FCS_READ, IN - otherwise
    DWORD            cchLogo;               // IN - Specifies the size of the buffer pointed to by pszIconFile
                                            // Ignored if dwReadWrite is FCS_READ
} SHFOLDERCUSTOMSETTINGS, *LPSHFOLDERCUSTOMSETTINGS;

#include <poppack.h>        /* Return to byte packing */

// Gets/Sets the Folder Custom Settings for pszPath based on dwReadWrite. dwReadWrite can be FCS_READ/FCS_WRITE/FCS_FORCEWRITE
SHSTDAPI SHGetSetFolderCustomSettings(_Inout_ LPSHFOLDERCUSTOMSETTINGS pfcs, _In_ PCWSTR pszPath, DWORD dwReadWrite);

#endif //NTDDI_VISTA
#endif  // NTDDI_WIN2K

//-------------------------------------------------------------------------
//
// SHBrowseForFolder API
//
//
//-------------------------------------------------------------------------

typedef int (CALLBACK* BFFCALLBACK)(HWND hwnd, UINT uMsg, LPARAM lParam, LPARAM lpData);

#include <pshpack8.h>

typedef struct _browseinfoA {
    HWND        hwndOwner;
    PCIDLIST_ABSOLUTE pidlRoot;
    LPSTR        pszDisplayName;        // Return display name of item selected.
    LPCSTR       lpszTitle;                     // text to go in the banner over the tree.
    UINT         ulFlags;                       // Flags that control the return stuff
    BFFCALLBACK  lpfn;
    LPARAM       lParam;                        // extra info that's passed back in callbacks
    int          iImage;                        // output var: where to return the Image index.
} BROWSEINFOA, *PBROWSEINFOA, *LPBROWSEINFOA;

typedef struct _browseinfoW {
    HWND        hwndOwner;
    PCIDLIST_ABSOLUTE pidlRoot;
    LPWSTR       pszDisplayName;        // Return display name of item selected.
    LPCWSTR      lpszTitle;                     // text to go in the banner over the tree.
    UINT         ulFlags;                       // Flags that control the return stuff
    BFFCALLBACK  lpfn;
    LPARAM       lParam;                        // extra info that's passed back in callbacks
    int          iImage;                        // output var: where to return the Image index.
} BROWSEINFOW, *PBROWSEINFOW, *LPBROWSEINFOW;

#include <poppack.h>        /* Return to byte packing */

#ifdef UNICODE
#define BROWSEINFO      BROWSEINFOW
#define PBROWSEINFO     PBROWSEINFOW
#define LPBROWSEINFO    LPBROWSEINFOW
#else
#define BROWSEINFO      BROWSEINFOA
#define PBROWSEINFO     PBROWSEINFOA
#define LPBROWSEINFO    LPBROWSEINFOA
#endif

// Browsing for directory.
#define BIF_RETURNONLYFSDIRS    0x00000001  // For finding a folder to start document searching
#define BIF_DONTGOBELOWDOMAIN   0x00000002  // For starting the Find Computer
#define BIF_STATUSTEXT          0x00000004   // Top of the dialog has 2 lines of text for BROWSEINFO.lpszTitle and one line if
                                        // this flag is set.  Passing the message BFFM_SETSTATUSTEXTA to the hwnd can set the
                                        // rest of the text.  This is not used with BIF_USENEWUI and BROWSEINFO.lpszTitle gets
                                        // all three lines of text.
#define BIF_RETURNFSANCESTORS   0x00000008
#define BIF_EDITBOX             0x00000010   // Add an editbox to the dialog
#define BIF_VALIDATE            0x00000020   // insist on valid result (or CANCEL)

#define BIF_NEWDIALOGSTYLE      0x00000040   // Use the new dialog layout with the ability to resize
                                        // Caller needs to call OleInitialize() before using this API

#define BIF_USENEWUI            (BIF_NEWDIALOGSTYLE | BIF_EDITBOX)

#define BIF_BROWSEINCLUDEURLS   0x00000080   // Allow URLs to be displayed or entered. (Requires BIF_USENEWUI)
#define BIF_UAHINT              0x00000100   // Add a UA hint to the dialog, in place of the edit box. May not be combined with BIF_EDITBOX
#define BIF_NONEWFOLDERBUTTON   0x00000200   // Do not add the "New Folder" button to the dialog.  Only applicable with BIF_NEWDIALOGSTYLE.
#define BIF_NOTRANSLATETARGETS  0x00000400   // don't traverse target as shortcut

#define BIF_BROWSEFORCOMPUTER   0x00001000  // Browsing for Computers.
#define BIF_BROWSEFORPRINTER    0x00002000  // Browsing for Printers
#define BIF_BROWSEINCLUDEFILES  0x00004000  // Browsing for Everything
#define BIF_SHAREABLE           0x00008000  // sharable resources displayed (remote shares, requires BIF_USENEWUI)
#define BIF_BROWSEFILEJUNCTIONS 0x00010000  // allow folder junctions like zip files and libraries to be browsed

// message from browser
#define BFFM_INITIALIZED        1
#define BFFM_SELCHANGED         2
#define BFFM_VALIDATEFAILEDA    3   // lParam:szPath ret:1(cont),0(EndDialog)
#define BFFM_VALIDATEFAILEDW    4   // lParam:wzPath ret:1(cont),0(EndDialog)
#define BFFM_IUNKNOWN           5   // provides IUnknown to client. lParam: IUnknown*

// messages to browser
#define BFFM_SETSTATUSTEXTA     (WM_USER + 100)
#define BFFM_ENABLEOK           (WM_USER + 101)
#define BFFM_SETSELECTIONA      (WM_USER + 102)
#define BFFM_SETSELECTIONW      (WM_USER + 103)
#define BFFM_SETSTATUSTEXTW     (WM_USER + 104)
#define BFFM_SETOKTEXT          (WM_USER + 105) // Unicode only
#define BFFM_SETEXPANDED        (WM_USER + 106) // Unicode only

SHSTDAPI_(PIDLIST_ABSOLUTE) SHBrowseForFolderA(_In_ LPBROWSEINFOA lpbi);
SHSTDAPI_(PIDLIST_ABSOLUTE) SHBrowseForFolderW(_In_ LPBROWSEINFOW lpbi);

#ifdef UNICODE
#define SHBrowseForFolder   SHBrowseForFolderW
#define BFFM_SETSTATUSTEXT  BFFM_SETSTATUSTEXTW
#define BFFM_SETSELECTION   BFFM_SETSELECTIONW

#define BFFM_VALIDATEFAILED BFFM_VALIDATEFAILEDW
#else
#define SHBrowseForFolder   SHBrowseForFolderA
#define BFFM_SETSTATUSTEXT  BFFM_SETSTATUSTEXTA
#define BFFM_SETSELECTION   BFFM_SETSELECTIONA

#define BFFM_VALIDATEFAILED BFFM_VALIDATEFAILEDA
#endif

//-------------------------------------------------------------------------
//
// SHLoadInProc
//
//   This function is no longer implemented. It will return E_NOTIMPL.
//
//-------------------------------------------------------------------------

_Check_return_
SHSTDAPI SHLoadInProc(_In_ REFCLSID rclsid);


//-------------------------------------------------------------------------
//
// Internet Shortcut Object
//
//-------------------------------------------------------------------------
// Cmds for CGID_ShortCut
enum
{
    ISHCUTCMDID_DOWNLOADICON      = 0,
    ISHCUTCMDID_INTSHORTCUTCREATE = 1,
#if (_WIN32_IE >= _WIN32_IE_IE70)
    ISHCUTCMDID_COMMITHISTORY     = 2,
    ISHCUTCMDID_SETUSERAWURL      = 3,
#endif
};
#define CMDID_INTSHORTCUTCREATE ISHCUTCMDID_INTSHORTCUTCREATE


// Bindctx key, passed to IShellFolder::ParseDiplayName.  Provides dbfolder with extra
// data, besides the name, necessary for the parse. the object in the bind context implements
// IPropertyStore and provides a fixed set of properties
#define STR_PARSE_WITH_PROPERTIES           L"ParseWithProperties"

// Bindctx key, passed to IShellFolder::ParseDisplayName(). used to pass the original item that
// is being re-parsed. that item is stored as an IShellItem that supports IParentAndItem,
// and should be the un-aliased form of the item.
#define STR_PARSE_PARTIAL_IDLIST            L"ParseOriginalItem"


//
// Helper function which returns a IShellFolder interface to the desktop
// folder. This is equivalent to call CoCreateInstance with CLSID_ShellDesktop.
//
//  CoCreateInstance(CLSID_Desktop, NULL, CLSCTX_INPROC, IID_PPV_ARGS(&pshf));
//
_Check_return_
SHSTDAPI SHGetDesktopFolder(_Outptr_ IShellFolder **ppshf);

// this interface is deprecated, data sources should
// implement IShellFolder2::GetDetailsOf()/GetDetailsEx() instead

#undef  INTERFACE
#define INTERFACE   IShellDetails

DECLARE_INTERFACE_IID_(IShellDetails, IUnknown, "000214EC-0000-0000-c000-000000000046")
{
    STDMETHOD(GetDetailsOf)(THIS_ _In_opt_ PCUITEMID_CHILD pidl, UINT iColumn, _Out_ SHELLDETAILS *pDetails) PURE;
    STDMETHOD(ColumnClick)(THIS_ UINT iColumn) PURE;
};

//
// IObjMgr::Append(punk)
//   This function adds an object to the end of a list of objects.
//
// IObjMgr::Remove(punk)
//   This function removes an object from a list of objects.
//
// This is implemented by CLSID_ACLMulti so each AutoComplete List
// (CLSID_ACLHistory, CLSID_ACListISF, CLSID_ACLMRU) can be added.
// CLSID_ACLMulti's IEnumString will then be the union of the results
// from the COM Objects added.
//

#undef INTERFACE
#define INTERFACE IObjMgr

DECLARE_INTERFACE_IID_(IObjMgr, IUnknown, "00BB2761-6A77-11D0-A535-00C04FD7D062")
{
    STDMETHOD(Append) (THIS_ _In_ IUnknown *punk) PURE;
    STDMETHOD(Remove) (THIS_ _In_ IUnknown *punk) PURE;
};

//
// IACList::Expand(LPCWSTR)
//   This function tells an autocomplete list to expand a specific string.
//
// If the user enters a multi-level path, AutoComplete (CLSID_AutoComplete)
// will use this interface to tell the "AutoComplete Lists" where to expand
// the results.
//
// For Example, if the user enters "C:\Program Files\Micros", AutoComplete
// first completely enumerate the "AutoComplete Lists" via IEnumString.  Then it
// will call the "AutoComplete Lists" with IACList::Expand(L"C:\Program Files").
// It will then enumerate the IEnumString interface again to get results in
// that directory.
//

#undef INTERFACE
#define INTERFACE IACList

DECLARE_INTERFACE_IID_(IACList, IUnknown, "77A130B0-94FD-11D0-A544-00C04FD7d062")
{
    STDMETHOD(Expand) (THIS_ _In_ PCWSTR pszExpand) PURE;
};

// This interface exists to allow the caller to set filter criteria
// for an AutoComplete List.  AutoComplete Lists generates the list of
// possible AutoComplete completions.  CLSID_ACListISF is one AutoComplete
// List COM object that implements this interface.

#undef INTERFACE
#define INTERFACE IACList2

typedef enum _tagAUTOCOMPLETELISTOPTIONS
{
    ACLO_NONE            = 0,    // don't enumerate anything
    ACLO_CURRENTDIR      = 1,    // enumerate current directory
    ACLO_MYCOMPUTER      = 2,    // enumerate MyComputer
    ACLO_DESKTOP         = 4,    // enumerate Desktop Folder
    ACLO_FAVORITES       = 8,    // enumerate Favorites Folder
    ACLO_FILESYSONLY     = 16,   // enumerate only the file system
#if (_WIN32_IE >= _WIN32_IE_IE60)
    ACLO_FILESYSDIRS     = 32,   // enumerate only the file system dirs, UNC shares, and UNC servers.
#endif
#if (_WIN32_IE >= _WIN32_IE_IE70)
    ACLO_VIRTUALNAMESPACE = 64,  // enumereate on the virual namespace
#endif
} AUTOCOMPLETELISTOPTIONS;

DECLARE_INTERFACE_IID_(IACList2, IACList, "470141a0-5186-11d2-bbb6-0060977b464c")
{
    // *** IACList2 specific methods ***
    STDMETHOD(SetOptions)(THIS_ DWORD dwFlag) PURE;
    STDMETHOD(GetOptions)(THIS_ _Out_ DWORD* pdwFlag) PURE;
};


/*-------------------------------------------------------------------------*\
    INTERFACE: IProgressDialog

    DESCRIPTION:
        CLSID_ProgressDialog/IProgressDialog exist to allow a caller to create
    a progress dialog, set it's title, animation, text lines, progress, and
    it will do all the work of updating on a background thread, being modeless,
    handling the user canceling the operation, and estimating the time remaining
    until the operation completes.

    USAGE:
        This is how the dialog is used during operations that require progress
    and the ability to cancel:
    {
        DWORD dwComplete, dwTotal;
        IProgressDialog * ppd;
        CoCreateInstance(CLSID_ProgressDialog, NULL, CLSCTX_INPROC_SERVER, IID_IProgressDialog, (void **)&ppd);
        ppd->SetTitle(L"My Slow Operation");                                // Set the title of the dialog.
        ppd->SetAnimation(hInstApp, IDA_OPERATION_ANIMATION);               // Set the animation to play.
        ppd->StartProgressDialog(hwndParent, punk, PROGDLG_AUTOTIME, NULL); // Display and enable automatic estimated time remaining.
        ppd->SetCancelMsg(L"Please wait while the current operation is cleaned up", NULL);   // Will only be displayed if Cancel button is pressed.

        dwComplete = 0;
        dwTotal = CalcTotalUnitsToDo();

        // Reset because CalcTotalUnitsToDo() took a long time and the estimated time
        // is based on the time between ::StartProgressDialog() and the first
        // ::SetProgress() call.
        ppd->Timer(PDTIMER_RESET, NULL);

        for (nIndex = 0; nIndex < nTotal; nIndex++)
        {
            if (TRUE == ppd->HasUserCancelled())
                break;

            ppd->SetLine(2, L"I'm processing item n", FALSE, NULL);
            dwComplete += DoSlowOperation();

            ppd->SetProgress(dwCompleted, dwTotal);
        }

        ppd->StopProgressDialog();
        ppd->Release();
    }
\*-------------------------------------------------------------------------*/

// Flags for IProgressDialog::StartProgressDialog() (dwFlags)
// The flag space includes OPPROGDLG_ and PROGDLG_ values, please guarantee they don't conflict. See shobjidl.idl for OPPROGDLG_*
#define PROGDLG_NORMAL          0x00000000      // default normal progress dlg behavior
#define PROGDLG_MODAL           0x00000001      // the dialog is modal to its hwndParent (default is modeless)
#define PROGDLG_AUTOTIME        0x00000002      // automatically updates the "Line3" text with the "time remaining" (you cant call SetLine3 if you passs this!)
#define PROGDLG_NOTIME          0x00000004      // we dont show the "time remaining" if this is set. We need this if dwTotal < dwCompleted for sparse files
#define PROGDLG_NOMINIMIZE      0x00000008      // Do not have a minimize button in the caption bar.
#define PROGDLG_NOPROGRESSBAR   0x00000010      // Don't display the progress bar
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define PROGDLG_MARQUEEPROGRESS 0x00000020      // Use marquee progress (comctl32 v6 required)
#define PROGDLG_NOCANCEL        0x00000040      // No cancel button (operation cannot be canceled) (use sparingly)
#endif

// Time Actions (dwTimerAction)
#define PDTIMER_RESET       0x00000001       // Reset the timer so the progress will be calculated from now until the first ::SetProgress() is called so
                                             // those this time will correspond to the values passed to ::SetProgress().  Only do this before ::SetProgress() is called.
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define PDTIMER_PAUSE       0x00000002       // Progress has been suspended
#define PDTIMER_RESUME      0x00000003       // Progress has resumed
#endif


#undef  INTERFACE
#define INTERFACE   IProgressDialog

DECLARE_INTERFACE_IID_(IProgressDialog, IUnknown, "EBBC7C04-315E-11d2-B62F-006097DF5BD4")
{
    STDMETHOD(StartProgressDialog)(THIS_ _In_opt_ HWND hwndParent, _In_opt_ IUnknown * punkEnableModless, DWORD dwFlags, _Reserved_ LPCVOID pvResevered) PURE;
    STDMETHOD(StopProgressDialog)(THIS) PURE;
    STDMETHOD(SetTitle)(THIS_ _In_ PCWSTR pwzTitle) PURE;
    STDMETHOD(SetAnimation)(THIS_ _In_opt_ HINSTANCE hInstAnimation, UINT idAnimation) PURE;
    STDMETHOD_(BOOL,HasUserCancelled) (THIS) PURE;
    STDMETHOD(SetProgress)(THIS_ DWORD dwCompleted, DWORD dwTotal) PURE;
    STDMETHOD(SetProgress64)(THIS_ ULONGLONG ullCompleted, ULONGLONG ullTotal) PURE;
    STDMETHOD(SetLine)(THIS_ DWORD dwLineNum, _In_ PCWSTR pwzString, BOOL fCompactPath, _Reserved_ LPCVOID pvResevered) PURE;
    STDMETHOD(SetCancelMsg)(THIS_ _In_ PCWSTR pwzCancelMsg, _Reserved_ LPCVOID pvResevered) PURE;
    STDMETHOD(Timer)(THIS_ DWORD dwTimerAction, _Reserved_ LPCVOID pvResevered) PURE;
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
// IDockingWindowSite interface
//
//   A site implements this interface so the object can negotiate for
// and inquire about real estate on the site.
//
// [Member functions]
//
// IDockingWindowSite::GetBorderDW(punkObj, prcBorder)
//   Site returns the bounding rectangle of the given source object
//   (punkObj).
//
// IDockingWindowSite::RequestBorderSpaceDW(punkObj, pbw)
//   Object requests that the site makes room for it, as specified in
//   *pbw.
//
// IDockingWindowSite::SetBorderSpaceDW(punkObj, pbw)
//   Object requests that the site set the border spacing to the size
//   specified in *pbw.
//
//-------------------------------------------------------------------------


#undef  INTERFACE
#define INTERFACE   IDockingWindowSite

DECLARE_INTERFACE_IID_(IDockingWindowSite, IOleWindow, "2a342fc2-7b26-11d0-8ca9-00a0c92dbfe8")
{
    STDMETHOD(GetBorderDW) (THIS_ _In_ IUnknown* punkObj, _Out_ RECT *prcBorder) PURE;
    STDMETHOD(RequestBorderSpaceDW) (THIS_ _In_ IUnknown* punkObj, _In_ LPCBORDERWIDTHS pbw) PURE;
    STDMETHOD(SetBorderSpaceDW) (THIS_ _In_ IUnknown* punkObj, _In_ LPCBORDERWIDTHS pbw) PURE;
};


//
// We need to make sure that WININET.H is included before this interface is
// used because the COMPONENT structure uses INTERNET_MAX_URL_LENGTH
//
#ifdef _WININET_
//
//  Flags and structures used by IActiveDesktop
//

typedef struct _tagWALLPAPEROPT
{
    DWORD   dwSize;     // size of this Structure.
    DWORD   dwStyle;    // WPSTYLE_* mentioned above
}
WALLPAPEROPT;

typedef WALLPAPEROPT  *LPWALLPAPEROPT;
typedef const WALLPAPEROPT *LPCWALLPAPEROPT;

typedef struct _tagCOMPONENTSOPT
{
    DWORD   dwSize;             //Size of this structure
    BOOL    fEnableComponents;  //Enable components?
    BOOL    fActiveDesktop;     // Active desktop enabled ?
}
COMPONENTSOPT;

typedef COMPONENTSOPT   *LPCOMPONENTSOPT;
typedef const COMPONENTSOPT   *LPCCOMPONENTSOPT;

typedef struct _tagCOMPPOS
{
    DWORD   dwSize;             //Size of this structure
    int     iLeft;              //Left of top-left corner in screen co-ordinates.
    int     iTop;               //Top of top-left corner in screen co-ordinates.
    DWORD   dwWidth;            // Width in pixels.
    DWORD   dwHeight;           // Height in pixels.
    int     izIndex;            // Indicates the Z-order of the component.
    BOOL    fCanResize;         // Is the component resizeable?
    BOOL    fCanResizeX;        // Resizeable in X-direction?
    BOOL    fCanResizeY;        // Resizeable in Y-direction?
    int     iPreferredLeftPercent;    //Left of top-left corner as percent of screen width
    int     iPreferredTopPercent;     //Top of top-left corner as percent of screen height
}
COMPPOS;

typedef COMPPOS *LPCOMPPOS;
typedef const COMPPOS *LPCCOMPPOS;

typedef struct  _tagCOMPSTATEINFO
{
    DWORD   dwSize;             // Size of this structure.
    int     iLeft;              // Left of the top-left corner in screen co-ordinates.
    int     iTop;               // Top of top-left corner in screen co-ordinates.
    DWORD   dwWidth;            // Width in pixels.
    DWORD   dwHeight;           // Height in pixels.
    DWORD   dwItemState;        // State of the component (full-screen mode or split-screen or normal state.
}
COMPSTATEINFO;

typedef COMPSTATEINFO   *LPCOMPSTATEINFO;
typedef const COMPSTATEINFO *LPCCOMPSTATEINFO;



#define COMPONENT_TOP (0x3fffffff)  // izOrder value meaning component is at the top


// iCompType values
#define COMP_TYPE_HTMLDOC       0
#define COMP_TYPE_PICTURE       1
#define COMP_TYPE_WEBSITE       2
#define COMP_TYPE_CONTROL       3
#define COMP_TYPE_CFHTML        4
#define COMP_TYPE_MAX           4

// The following is the COMPONENT structure used in IE4.01, IE4.0 and Memphis. It is kept here for compatibility
// reasons.
typedef struct _tagIE4COMPONENT
{
    DWORD   dwSize;             //Size of this structure
    DWORD   dwID;               //Reserved: Set it always to zero.
    int     iComponentType;     //One of COMP_TYPE_*
    BOOL    fChecked;           // Is this component enabled?
    BOOL    fDirty;             // Had the component been modified and not yet saved to disk?
    BOOL    fNoScroll;          // Is the component scrollable?
    COMPPOS cpPos;              // Width, height etc.,
    WCHAR   wszFriendlyName[MAX_PATH];          // Friendly name of component.
    WCHAR   wszSource[INTERNET_MAX_URL_LENGTH]; //URL of the component.
    WCHAR   wszSubscribedURL[INTERNET_MAX_URL_LENGTH]; //Subscrined URL
}
IE4COMPONENT;

typedef IE4COMPONENT *LPIE4COMPONENT;
typedef const IE4COMPONENT *LPCIE4COMPONENT;

//
// The following is the new NT5 component structure. Note that the initial portion of this component exactly
// matches the IE4COMPONENT structure. All new fields are added at the bottom and the dwSize field is used to
// distinguish between IE4COMPONENT and the new COMPONENT structures.
//
typedef struct _tagCOMPONENT
{
    DWORD   dwSize;             //Size of this structure
    DWORD   dwID;               //Reserved: Set it always to zero.
    int     iComponentType;     //One of COMP_TYPE_*
    BOOL    fChecked;           // Is this component enabled?
    BOOL    fDirty;             // Had the component been modified and not yet saved to disk?
    BOOL    fNoScroll;          // Is the component scrollable?
    COMPPOS cpPos;              // Width, height etc.,
    WCHAR   wszFriendlyName[MAX_PATH];          // Friendly name of component.
    WCHAR   wszSource[INTERNET_MAX_URL_LENGTH]; //URL of the component.
    WCHAR   wszSubscribedURL[INTERNET_MAX_URL_LENGTH]; //Subscrined URL

    //New fields are added below. Everything above here must exactly match the IE4COMPONENT Structure.
    DWORD           dwCurItemState; // Current state of the Component.
    COMPSTATEINFO   csiOriginal;    // Original state of the component when it was first added.
    COMPSTATEINFO   csiRestored;    // Restored state of the component.
}
COMPONENT;

typedef COMPONENT *LPCOMPONENT;
typedef const COMPONENT *LPCCOMPONENT;


// Defines for dwCurItemState
#define IS_NORMAL               0x00000001
#define IS_FULLSCREEN           0x00000002
#define IS_SPLIT                0x00000004
#define IS_VALIDSIZESTATEBITS   (IS_NORMAL | IS_SPLIT | IS_FULLSCREEN)  // The set of IS_* state bits which define the "size" of the component - these bits are mutually exclusive.
#define IS_VALIDSTATEBITS       (IS_NORMAL | IS_SPLIT | IS_FULLSCREEN | 0x80000000 | 0x40000000)  // All of the currently defined IS_* bits.

////////////////////////////////////////////
// Flags for IActiveDesktop::ApplyChanges()
#define AD_APPLY_SAVE             0x00000001
#define AD_APPLY_HTMLGEN          0x00000002
#define AD_APPLY_REFRESH          0x00000004
#define AD_APPLY_ALL              (AD_APPLY_SAVE | AD_APPLY_HTMLGEN | AD_APPLY_REFRESH)
#define AD_APPLY_FORCE            0x00000008
#define AD_APPLY_BUFFERED_REFRESH 0x00000010
#define AD_APPLY_DYNAMICREFRESH   0x00000020

////////////////////////////////////////////
// Flags for IActiveDesktop::GetWallpaper()
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define AD_GETWP_BMP            0x00000000
#define AD_GETWP_IMAGE          0x00000001
#define AD_GETWP_LAST_APPLIED   0x00000002
#endif


////////////////////////////////////////////
// Flags for IActiveDesktop::GetWallpaperOptions()
//           IActiveDesktop::SetWallpaperOptions()
#define WPSTYLE_CENTER      0
#define WPSTYLE_TILE        1
#define WPSTYLE_STRETCH     2
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define WPSTYLE_KEEPASPECT  3
#define WPSTYLE_CROPTOFIT   4
#endif // NTDDI_WIN7
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define WPSTYLE_SPAN        5
#endif // NTDDI_WIN8

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define WPSTYLE_MAX         6
#elif (NTDDI_VERSION >= NTDDI_WIN7)
#define WPSTYLE_MAX         5
#else
#define WPSTYLE_MAX         3
#endif


////////////////////////////////////////////
// Flags for IActiveDesktop::ModifyComponent()

#define COMP_ELEM_TYPE          0x00000001
#define COMP_ELEM_CHECKED       0x00000002
#define COMP_ELEM_DIRTY         0x00000004
#define COMP_ELEM_NOSCROLL      0x00000008
#define COMP_ELEM_POS_LEFT      0x00000010
#define COMP_ELEM_POS_TOP       0x00000020
#define COMP_ELEM_SIZE_WIDTH    0x00000040
#define COMP_ELEM_SIZE_HEIGHT   0x00000080
#define COMP_ELEM_POS_ZINDEX    0x00000100
#define COMP_ELEM_SOURCE        0x00000200
#define COMP_ELEM_FRIENDLYNAME  0x00000400
#define COMP_ELEM_SUBSCRIBEDURL 0x00000800
#define COMP_ELEM_ORIGINAL_CSI  0x00001000
#define COMP_ELEM_RESTORED_CSI  0x00002000
#define COMP_ELEM_CURITEMSTATE  0x00004000

#define COMP_ELEM_ALL   (COMP_ELEM_TYPE | COMP_ELEM_CHECKED | COMP_ELEM_DIRTY |                     \
                         COMP_ELEM_NOSCROLL | COMP_ELEM_POS_LEFT | COMP_ELEM_SIZE_WIDTH  |          \
                         COMP_ELEM_SIZE_HEIGHT | COMP_ELEM_POS_ZINDEX | COMP_ELEM_SOURCE |          \
                         COMP_ELEM_FRIENDLYNAME | COMP_ELEM_POS_TOP | COMP_ELEM_SUBSCRIBEDURL |     \
                         COMP_ELEM_ORIGINAL_CSI | COMP_ELEM_RESTORED_CSI | COMP_ELEM_CURITEMSTATE)


////////////////////////////////////////////
// Flags for IActiveDesktop::AddDesktopItemWithUI()
enum tagDTI_ADTIWUI
{
    DTI_ADDUI_DEFAULT               = 0x00000000,
    DTI_ADDUI_DISPSUBWIZARD         = 0x00000001,
    DTI_ADDUI_POSITIONITEM          = 0x00000002,
};


////////////////////////////////////////////
// Flags for IActiveDesktop::AddUrl()
#define ADDURL_SILENT           0X0001


////////////////////////////////////////////
// Default positions for ADI
#define COMPONENT_DEFAULT_LEFT    (0xFFFF)
#define COMPONENT_DEFAULT_TOP     (0xFFFF)




//
//  Interface for manipulating the Active Desktop.
//

#undef INTERFACE
#define INTERFACE IActiveDesktop

DECLARE_INTERFACE_IID_(IActiveDesktop, IUnknown, "f490eb00-1240-11d1-9888-006097deacf9")
{
    // IUnknown methods
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)  (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // IActiveDesktop methods
    STDMETHOD (ApplyChanges)(THIS_ DWORD dwFlags) PURE;
    STDMETHOD (GetWallpaper)(THIS_ _Out_writes_(cchWallpaper) PWSTR pwszWallpaper, UINT cchWallpaper, DWORD dwFlags) PURE;
    STDMETHOD (SetWallpaper)(THIS_ _In_ PCWSTR pwszWallpaper, DWORD dwReserved) PURE;
    STDMETHOD (GetWallpaperOptions)(THIS_ _Inout_ LPWALLPAPEROPT pwpo, DWORD dwReserved) PURE;
    STDMETHOD (SetWallpaperOptions)(THIS_ _In_ LPCWALLPAPEROPT pwpo, DWORD dwReserved) PURE;
    STDMETHOD (GetPattern)(THIS_ _Out_writes_(cchPattern) PWSTR pwszPattern, UINT cchPattern, DWORD dwReserved) PURE;
    STDMETHOD (SetPattern)(THIS_ _In_ PCWSTR pwszPattern, DWORD dwReserved) PURE;
    STDMETHOD (GetDesktopItemOptions)(THIS_ _Inout_ LPCOMPONENTSOPT pco, DWORD dwReserved) PURE;
    STDMETHOD (SetDesktopItemOptions)(THIS_ _In_ LPCCOMPONENTSOPT pco, DWORD dwReserved) PURE;
    STDMETHOD (AddDesktopItem)(THIS_ _In_ LPCCOMPONENT pcomp, DWORD dwReserved) PURE;
    STDMETHOD (AddDesktopItemWithUI)(THIS_ _In_opt_ HWND hwnd, _In_ LPCOMPONENT pcomp, DWORD dwReserved) PURE;
    STDMETHOD (ModifyDesktopItem)(THIS_ _Inout_ LPCCOMPONENT pcomp, DWORD dwFlags) PURE;
    STDMETHOD (RemoveDesktopItem)(THIS_ _In_ LPCCOMPONENT pcomp, DWORD dwReserved) PURE;
    STDMETHOD (GetDesktopItemCount)(THIS_ _Out_ int *pcItems, DWORD dwReserved) PURE;
    STDMETHOD (GetDesktopItem)(THIS_ int nComponent, _Inout_ LPCOMPONENT pcomp, DWORD dwReserved) PURE;
    STDMETHOD (GetDesktopItemByID)(THIS_ ULONG_PTR dwID, _Inout_ LPCOMPONENT pcomp, DWORD dwReserved) PURE;
    STDMETHOD (GenerateDesktopItemHtml)(THIS_ _In_ PCWSTR pwszFileName, _In_ LPCOMPONENT pcomp, DWORD dwReserved) PURE;
    STDMETHOD (AddUrl)(THIS_ _In_opt_ HWND hwnd, _In_ PCWSTR pszSource, _In_ LPCOMPONENT pcomp, DWORD dwFlags) PURE;
    STDMETHOD (GetDesktopItemBySource)(THIS_ _In_ PCWSTR pwszSource, _Inout_ LPCOMPONENT pcomp, DWORD dwReserved) PURE;
};

typedef IActiveDesktop * LPACTIVEDESKTOP;

#endif // _WININET_

#define MAX_COLUMN_NAME_LEN 80
#define MAX_COLUMN_DESC_LEN 128

//==========================================================================
// Clipboard format which may be supported by IDataObject from system
// defined shell folders (such as directories, network, ...).
//==========================================================================

#define CFSTR_SHELLIDLIST                   TEXT("Shell IDList Array")                  // CF_IDLIST
#define CFSTR_SHELLIDLISTOFFSET             TEXT("Shell Object Offsets")                // CF_OBJECTPOSITIONS
#define CFSTR_NETRESOURCES                  TEXT("Net Resource")                        // CF_NETRESOURCE
#define CFSTR_FILEDESCRIPTORA               TEXT("FileGroupDescriptor")                 // CF_FILEGROUPDESCRIPTORA
#define CFSTR_FILEDESCRIPTORW               TEXT("FileGroupDescriptorW")                // CF_FILEGROUPDESCRIPTORW
#define CFSTR_FILECONTENTS                  TEXT("FileContents")                        // CF_FILECONTENTS
#define CFSTR_FILENAMEA                     TEXT("FileName")                            // CF_FILENAMEA
#define CFSTR_FILENAMEW                     TEXT("FileNameW")                           // CF_FILENAMEW
#define CFSTR_PRINTERGROUP                  TEXT("PrinterFriendlyName")                 // CF_PRINTERS
#define CFSTR_FILENAMEMAPA                  TEXT("FileNameMap")                         // CF_FILENAMEMAPA
#define CFSTR_FILENAMEMAPW                  TEXT("FileNameMapW")                        // CF_FILENAMEMAPW
#define CFSTR_SHELLURL                      TEXT("UniformResourceLocator")
#define CFSTR_INETURLA                      CFSTR_SHELLURL
#define CFSTR_INETURLW                      TEXT("UniformResourceLocatorW")
#define CFSTR_PREFERREDDROPEFFECT           TEXT("Preferred DropEffect")
#define CFSTR_PERFORMEDDROPEFFECT           TEXT("Performed DropEffect")
#define CFSTR_PASTESUCCEEDED                TEXT("Paste Succeeded")
#define CFSTR_INDRAGLOOP                    TEXT("InShellDragLoop")
#define CFSTR_MOUNTEDVOLUME                 TEXT("MountedVolume")
#define CFSTR_PERSISTEDDATAOBJECT           TEXT("PersistedDataObject")
#define CFSTR_TARGETCLSID                   TEXT("TargetCLSID")                         // HGLOBAL with a CLSID of the drop target
#define CFSTR_LOGICALPERFORMEDDROPEFFECT    TEXT("Logical Performed DropEffect")
#define CFSTR_AUTOPLAY_SHELLIDLISTS         TEXT("Autoplay Enumerated IDList Array")    // (HGLOBAL with LPIDA)
#define CFSTR_UNTRUSTEDDRAGDROP             TEXT("UntrustedDragDrop")                   //  DWORD
#define CFSTR_FILE_ATTRIBUTES_ARRAY         TEXT("File Attributes Array")               // (FILE_ATTRIBUTES_ARRAY format on HGLOBAL)
#define CFSTR_INVOKECOMMAND_DROPPARAM       TEXT("InvokeCommand DropParam")             // (HGLOBAL with LPWSTR)
#define CFSTR_SHELLDROPHANDLER              TEXT("DropHandlerCLSID")                    // (HGLOBAL with CLSID of drop handler)
#define CFSTR_DROPDESCRIPTION               TEXT("DropDescription")                     // (HGLOBAL with DROPDESCRIPTION)
#define CFSTR_ZONEIDENTIFIER                TEXT("ZoneIdentifier")                      //  DWORD, to be used with CFSTR_FILECONTENTS data transfers

#ifdef UNICODE
#define CFSTR_FILEDESCRIPTOR    CFSTR_FILEDESCRIPTORW
#define CFSTR_FILENAME          CFSTR_FILENAMEW
#define CFSTR_FILENAMEMAP       CFSTR_FILENAMEMAPW
#define CFSTR_INETURL           CFSTR_INETURLW
#else
#define CFSTR_FILEDESCRIPTOR    CFSTR_FILEDESCRIPTORA
#define CFSTR_FILENAME          CFSTR_FILENAMEA
#define CFSTR_FILENAMEMAP       CFSTR_FILENAMEMAPA
#define CFSTR_INETURL           CFSTR_INETURLA
#endif

#define DVASPECT_SHORTNAME      2 // use for CF_HDROP to get short name version of file paths
#define DVASPECT_COPY           3 // use to indicate format is a "Copy" of the data (FILECONTENTS, FILEDESCRIPTOR, etc)
#define DVASPECT_LINK           4 // use to indicate format is a "Shortcut" to the data (FILECONTENTS, FILEDESCRIPTOR, etc)

#include <pshpack8.h>
//
// format of CF_NETRESOURCE
//
typedef struct _NRESARRAY {     // anr
    UINT cItems;
    NETRESOURCE nr[1];
} NRESARRAY, * LPNRESARRAY;
#include <poppack.h>        /* Return to byte packing */

//
// format of CF_IDLIST
//
typedef struct _IDA {
    UINT cidl;          // number of relative IDList
    UINT aoffset[1];    // [0]: folder IDList, [1]-[cidl]: item IDList
} CIDA, * LPIDA;

//
// FILEDESCRIPTOR.dwFlags field indicate which fields are valid in the FILEDESCRIPTOR struct
//
typedef enum {
    FD_CLSID            = 0x00000001,
    FD_SIZEPOINT        = 0x00000002,
    FD_ATTRIBUTES       = 0x00000004,
    FD_CREATETIME       = 0x00000008,
    FD_ACCESSTIME       = 0x00000010,
    FD_WRITESTIME       = 0x00000020,
    FD_FILESIZE         = 0x00000040,
    FD_PROGRESSUI       = 0x00004000,       // Show Progress UI w/Drag and Drop
    FD_LINKUI           = 0x00008000,       // 'link' UI is prefered
#if (NTDDI_VERSION >= NTDDI_VISTA)
    FD_UNICODE          = (int) 0x80000000,       // this descriptor is UNICODE
#endif
} FD_FLAGS;

typedef struct _FILEDESCRIPTORA {
    DWORD dwFlags;
    CLSID clsid;
    SIZEL sizel;
    POINTL pointl;
    DWORD dwFileAttributes;
    FILETIME ftCreationTime;
    FILETIME ftLastAccessTime;
    FILETIME ftLastWriteTime;
    DWORD nFileSizeHigh;
    DWORD nFileSizeLow;
    CHAR   cFileName[ MAX_PATH ];
} FILEDESCRIPTORA, *LPFILEDESCRIPTORA;

typedef struct _FILEDESCRIPTORW {
    DWORD dwFlags;

    CLSID clsid;
    SIZEL sizel;
    POINTL pointl;

    DWORD dwFileAttributes;
    FILETIME ftCreationTime;
    FILETIME ftLastAccessTime;
    FILETIME ftLastWriteTime;
    DWORD nFileSizeHigh;
    DWORD nFileSizeLow;
    WCHAR  cFileName[ MAX_PATH ];
} FILEDESCRIPTORW, *LPFILEDESCRIPTORW;

#ifdef UNICODE
#define FILEDESCRIPTOR      FILEDESCRIPTORW
#define LPFILEDESCRIPTOR    LPFILEDESCRIPTORW
#else
#define FILEDESCRIPTOR      FILEDESCRIPTORA
#define LPFILEDESCRIPTOR    LPFILEDESCRIPTORA
#endif

//
// format of CF_FILEGROUPDESCRIPTOR
//
typedef struct _FILEGROUPDESCRIPTORA { // fgd
     UINT cItems;
     FILEDESCRIPTORA fgd[1];
} FILEGROUPDESCRIPTORA, * LPFILEGROUPDESCRIPTORA;

typedef struct _FILEGROUPDESCRIPTORW { // fgd
     UINT cItems;
     FILEDESCRIPTORW fgd[1];
} FILEGROUPDESCRIPTORW, * LPFILEGROUPDESCRIPTORW;

#ifdef UNICODE
#define FILEGROUPDESCRIPTOR     FILEGROUPDESCRIPTORW
#define LPFILEGROUPDESCRIPTOR   LPFILEGROUPDESCRIPTORW
#else
#define FILEGROUPDESCRIPTOR     FILEGROUPDESCRIPTORA
#define LPFILEGROUPDESCRIPTOR   LPFILEGROUPDESCRIPTORA
#endif

//
// format of CF_HDROP and CF_PRINTERS, in the HDROP case the data that follows
// is a double null terinated list of file names, for printers they are printer
// friendly names
//
typedef struct _DROPFILES {
   DWORD pFiles;                       // offset of file list
   POINT pt;                           // drop point (client coords)
   BOOL fNC;                           // is it on NonClient area
                                       // and pt is in screen coords
   BOOL fWide;                         // WIDE character switch
} DROPFILES, *LPDROPFILES;


#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef struct
{
    UINT cItems;                    // number of items in rgdwFileAttributes array
    DWORD dwSumFileAttributes;      // all of the attributes ORed together
    DWORD dwProductFileAttributes;  // all of the attributes ANDed together
    DWORD rgdwFileAttributes[1];    // array
} FILE_ATTRIBUTES_ARRAY;            // clipboard format definition for CFSTR_FILE_ATTRIBUTES_ARRAY
#endif // NTDDI_VISTA

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef enum
{
    DROPIMAGE_INVALID             = -1,                // no image preference (use default)
    DROPIMAGE_NONE                = 0,                 // red "no" circle
    DROPIMAGE_COPY                = DROPEFFECT_COPY,   // plus for copy
    DROPIMAGE_MOVE                = DROPEFFECT_MOVE,   // movement arrow for move
    DROPIMAGE_LINK                = DROPEFFECT_LINK,   // link arrow for link
    DROPIMAGE_LABEL               = 6,                 // tag icon to indicate metadata will be changed
    DROPIMAGE_WARNING             = 7,                 // yellow exclamation, something is amiss with the operation
    DROPIMAGE_NOIMAGE             = 8,                 // no image at all
} DROPIMAGETYPE;

typedef struct
{
    DROPIMAGETYPE type;                 // indicates the stock image to use

// text such as "Move to %1"
    WCHAR szMessage[MAX_PATH];

// text such as "Documents", inserted as specified by szMessage
    WCHAR szInsert[MAX_PATH];

// some UI coloring is applied to the text in szInsert, if used by specifying %1 in szMessage.
// %% and %1 are the subset of FormatMessage markers that are processed here.
} DROPDESCRIPTION;

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//====== File System Notification APIs ===============================
//
typedef struct _SHChangeNotifyEntry
{
    PCIDLIST_ABSOLUTE pidl;
    BOOL   fRecursive;
} SHChangeNotifyEntry;


//
//  File System Notification flags
//

#define SHCNRF_InterruptLevel      0x0001
#define SHCNRF_ShellLevel          0x0002
#define SHCNRF_RecursiveInterrupt  0x1000
#define SHCNRF_NewDelivery         0x8000

#define SHCNE_RENAMEITEM          0x00000001L
#define SHCNE_CREATE              0x00000002L
#define SHCNE_DELETE              0x00000004L
#define SHCNE_MKDIR               0x00000008L
#define SHCNE_RMDIR               0x00000010L
#define SHCNE_MEDIAINSERTED       0x00000020L
#define SHCNE_MEDIAREMOVED        0x00000040L
#define SHCNE_DRIVEREMOVED        0x00000080L
#define SHCNE_DRIVEADD            0x00000100L
#define SHCNE_NETSHARE            0x00000200L
#define SHCNE_NETUNSHARE          0x00000400L
#define SHCNE_ATTRIBUTES          0x00000800L
#define SHCNE_UPDATEDIR           0x00001000L
#define SHCNE_UPDATEITEM          0x00002000L
#define SHCNE_SERVERDISCONNECT    0x00004000L
#define SHCNE_UPDATEIMAGE         0x00008000L
#define SHCNE_DRIVEADDGUI         0x00010000L
#define SHCNE_RENAMEFOLDER        0x00020000L
#define SHCNE_FREESPACE           0x00040000L

// SHCNE_EXTENDED_EVENT: the extended event is identified in dwItem1,
// packed in LPITEMIDLIST format (same as SHCNF_DWORD packing).
// Additional information can be passed in the dwItem2 parameter
// of SHChangeNotify (called "pidl2" below), which if present, must also
// be in LPITEMIDLIST format.
//
// Unlike the standard events, the extended events are ORDINALs, so we
// don't run out of bits.  Extended events follow the SHCNEE_* naming
// convention.
//
// The dwItem2 parameter varies according to the extended event.

#define SHCNE_EXTENDED_EVENT      0x04000000L

#define SHCNE_ASSOCCHANGED        0x08000000L

#define SHCNE_DISKEVENTS          0x0002381FL
#define SHCNE_GLOBALEVENTS        0x0C0581E0L // Events that dont match pidls first
#define SHCNE_ALLEVENTS           0x7FFFFFFFL
#define SHCNE_INTERRUPT           0x80000000L // The presence of this flag indicates
                                            // that the event was generated by an
                                            // interrupt.  It is stripped out before
                                            // the clients of SHCNNotify_ see it.

// SHCNE_EXTENDED_EVENT extended events.  These events are ordinals.
// This is not a bitfield.

#define SHCNEE_ORDERCHANGED             2L  // pidl2 is the changed folder
#define SHCNEE_MSI_CHANGE               4L  // pidl2 is a SHChangeProductKeyAsIDList
#define SHCNEE_MSI_UNINSTALL            5L  // pidl2 is a SHChangeProductKeyAsIDList

// Flags
// uFlags & SHCNF_TYPE is an ID which indicates what dwItem1 and dwItem2 mean
#define SHCNF_IDLIST      0x0000        // LPITEMIDLIST
#define SHCNF_PATHA       0x0001        // path name
#define SHCNF_PRINTERA    0x0002        // printer friendly name
#define SHCNF_DWORD       0x0003        // DWORD
#define SHCNF_PATHW       0x0005        // path name
#define SHCNF_PRINTERW    0x0006        // printer friendly name
#define SHCNF_TYPE        0x00FF
#define SHCNF_FLUSH       0x1000
#define SHCNF_FLUSHNOWAIT 0x3000        // includes SHCNF_FLUSH

#define SHCNF_NOTIFYRECURSIVE      0x10000 // Notify clients registered for any child


#ifdef UNICODE
#define SHCNF_PATH      SHCNF_PATHW
#define SHCNF_PRINTER   SHCNF_PRINTERW
#else
#define SHCNF_PATH      SHCNF_PATHA
#define SHCNF_PRINTER   SHCNF_PRINTERA
#endif


//
//  APIs
//
SHSTDAPI_(void) SHChangeNotify(LONG wEventId, UINT uFlags, _In_opt_ LPCVOID dwItem1, _In_opt_ LPCVOID dwItem2);

//
// IShellChangeNotify
//
#undef  INTERFACE
#define INTERFACE  IShellChangeNotify

DECLARE_INTERFACE_IID_(IShellChangeNotify, IUnknown, "D82BE2B1-5764-11D0-A96E-00C04FD705A2")
{
    STDMETHOD(OnChange) (THIS_ LONG lEvent, _In_opt_ PCIDLIST_ABSOLUTE pidl1, _In_opt_ PCIDLIST_ABSOLUTE pidl2) PURE;
};

//
// IQueryInfo
//
//-------------------------------------------------------------------------
//
// IQueryInfo interface
//
// [Methods]
//              ::GetInfoTip()
//-------------------------------------------------------------------------

#undef  INTERFACE
#define INTERFACE  IQueryInfo

DECLARE_INTERFACE_IID_(IQueryInfo, IUnknown, "00021500-0000-0000-c000-000000000046")
{
    STDMETHOD(GetInfoTip)(THIS_ DWORD dwFlags, _Outptr_ PWSTR *ppwszTip) PURE;
    STDMETHOD(GetInfoFlags)(THIS_ _Out_ DWORD *pdwFlags) PURE;
};

#define QITIPF_DEFAULT          0x00000000
#define QITIPF_USENAME          0x00000001
#define QITIPF_LINKNOTARGET     0x00000002
#define QITIPF_LINKUSETARGET    0x00000004
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define QITIPF_USESLOWTIP       0x00000008  // Flag says it's OK to take a long time generating tip
#endif
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define QITIPF_SINGLELINE       0x00000010
#endif

#define QIF_CACHED              0x00000001
#define QIF_DONTEXPANDFOLDER    0x00000002

//
// SHAddToRecentDocs
//
typedef enum
{
    SHARD_PIDL            = 0x00000001L,
    SHARD_PATHA           = 0x00000002L,
    SHARD_PATHW           = 0x00000003L,
#if (NTDDI_VERSION >= NTDDI_WIN7)
    SHARD_APPIDINFO       = 0x00000004L, // indicates the data type is a pointer to a SHARDAPPIDINFO structure
    SHARD_APPIDINFOIDLIST = 0x00000005L, // indicates the data type is a pointer to a SHARDAPPIDINFOIDLIST structure
    SHARD_LINK            = 0x00000006L, // indicates the data type is a pointer to an IShellLink instance
    SHARD_APPIDINFOLINK   = 0x00000007L, // indicates the data type is a pointer to a SHARDAPPIDINFOLINK structure
    SHARD_SHELLITEM       = 0x00000008L, // indicates the data type is a pointer to an IShellItem instance
#endif
} SHARD;

#if (NTDDI_VERSION >= NTDDI_WIN7)

typedef struct SHARDAPPIDINFO
{
    IShellItem *psi;        // The namespace location of the the item that should be added to the recent docs folder.
    PCWSTR pszAppID;        // The id of the application that should be associated with this recent doc.
} SHARDAPPIDINFO;

typedef struct SHARDAPPIDINFOIDLIST
{
    PCIDLIST_ABSOLUTE pidl; // The idlist for the shell item that should be added to the recent docs folder.
    PCWSTR pszAppID;        // The id of the application that should be associated with this recent doc.
} SHARDAPPIDINFOIDLIST;

typedef struct SHARDAPPIDINFOLINK
{
    IShellLink *psl;        // An IShellLink instance that when launched opens a recently used item in the specified
                            // application. This link is not added to the recent docs folder, but will be added to the
                            // specified application's destination list.
    PCWSTR pszAppID;        // The id of the application that should be associated with this recent doc.
} SHARDAPPIDINFOLINK;

#endif

#ifdef UNICODE
#define SHARD_PATH  SHARD_PATHW
#else
#define SHARD_PATH  SHARD_PATHA
#endif

// The type of the data pointed to by pv is a function of uFlags values that are SHARD_XXX values.  PV can be a PCIDLIST_ABSOLUTE, PCWSTR, PCSTR, SHARDAPPIDINFO, or SHARDAPPIDINFOIDLIST.
SHSTDAPI_(void) SHAddToRecentDocs(UINT uFlags, _In_opt_ LPCVOID pv);


#include <pshpack1.h>
typedef struct _SHChangeDWORDAsIDList {
    USHORT   cb;
    DWORD    dwItem1;
    DWORD    dwItem2;
    USHORT   cbZero;
} SHChangeDWORDAsIDList, *LPSHChangeDWORDAsIDList;
#include <poppack.h>


#if (NTDDI_VERSION >= NTDDI_WIN2K)


typedef struct _SHChangeUpdateImageIDList {
    USHORT cb;
    int iIconIndex;
    int iCurIndex;
    UINT uFlags;
    DWORD dwProcessID;
    WCHAR szName[MAX_PATH];
    USHORT cbZero;
} SHChangeUpdateImageIDList, * LPSHChangeUpdateImageIDList;
SHSTDAPI_(int) SHHandleUpdateImage(_In_ PCIDLIST_ABSOLUTE pidlExtra);
SHSTDAPI_(void) SHUpdateImageA(_In_ LPCSTR pszHashItem, int iIndex, UINT uFlags, int iImageIndex);
SHSTDAPI_(void) SHUpdateImageW(_In_ LPCWSTR pszHashItem, int iIndex, UINT uFlags, int iImageIndex);
#ifdef UNICODE
#define SHUpdateImage  SHUpdateImageW
#else
#define SHUpdateImage  SHUpdateImageA
#endif // !UNICODE
#endif /* NTDDI_WIN2K */

SHSTDAPI_(ULONG) SHChangeNotifyRegister(_In_ HWND hwnd, int fSources, LONG fEvents, UINT wMsg, int cEntries, _In_ const SHChangeNotifyEntry *pshcne);
SHSTDAPI_(BOOL) SHChangeNotifyDeregister(ULONG ulID);

typedef enum
{
    SCNRT_ENABLE  = 0,
    SCNRT_DISABLE = 1,
} SCNRT_STATUS;

SHSTDAPI_(HANDLE) SHChangeNotification_Lock(_In_ HANDLE hChange, DWORD dwProcId, _Outptr_opt_result_buffer_(2) PIDLIST_ABSOLUTE **pppidl, _Out_opt_ LONG *plEvent);
SHSTDAPI_(BOOL) SHChangeNotification_Unlock(_In_ HANDLE hLock);
// The pidls that are given to the view via the ChangeNotifyEvents are simple Pidls,
// SHGetRealIDL() will convert them to true PIDLs.
SHSTDAPI SHGetRealIDL(_In_ IShellFolder *psf, _In_ PCUITEMID_CHILD pidlSimple, _Outptr_ PITEMID_CHILD *ppidlReal);


_Check_return_
SHSTDAPI SHGetInstanceExplorer(_Outptr_ IUnknown **ppunk);

//
// SHGetDataFromIDListA/W
//
// SHGetDataFromIDList nFormat values TCHAR
#define SHGDFIL_FINDDATA        1
#define SHGDFIL_NETRESOURCE     2
#define SHGDFIL_DESCRIPTIONID   3

#define SHDID_ROOT_REGITEM          1
#define SHDID_FS_FILE               2
#define SHDID_FS_DIRECTORY          3
#define SHDID_FS_OTHER              4
#define SHDID_COMPUTER_DRIVE35      5
#define SHDID_COMPUTER_DRIVE525     6
#define SHDID_COMPUTER_REMOVABLE    7
#define SHDID_COMPUTER_FIXED        8
#define SHDID_COMPUTER_NETDRIVE     9
#define SHDID_COMPUTER_CDROM        10
#define SHDID_COMPUTER_RAMDISK      11
#define SHDID_COMPUTER_OTHER        12
#define SHDID_NET_DOMAIN            13
#define SHDID_NET_SERVER            14
#define SHDID_NET_SHARE             15
#define SHDID_NET_RESTOFNET         16
#define SHDID_NET_OTHER             17
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define SHDID_COMPUTER_IMAGING      18
#define SHDID_COMPUTER_AUDIO        19
#define SHDID_COMPUTER_SHAREDDOCS   20
#endif
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define SHDID_MOBILE_DEVICE         21  // PDA/PalmPC
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define SHDID_REMOTE_DESKTOP_DRIVE  22
#endif

#include <pshpack8.h>

typedef struct _SHDESCRIPTIONID {
    DWORD   dwDescriptionId;
    CLSID   clsid;
} SHDESCRIPTIONID, *LPSHDESCRIPTIONID;

#include <poppack.h>        /* Return to byte packing */

// these delegate to IShellFolder2::GetDetailsEx()

SHSTDAPI SHGetDataFromIDListA(_In_ IShellFolder *psf, _In_ PCUITEMID_CHILD pidl, int nFormat, _Out_writes_bytes_(cb) void *pv, int cb);
SHSTDAPI SHGetDataFromIDListW(_In_ IShellFolder *psf, _In_ PCUITEMID_CHILD pidl, int nFormat, _Out_writes_bytes_(cb) void *pv, int cb);
#ifdef UNICODE
#define SHGetDataFromIDList  SHGetDataFromIDListW
#else
#define SHGetDataFromIDList  SHGetDataFromIDListA
#endif // !UNICODE


//===========================================================================

// PathResolve flags
#define PRF_VERIFYEXISTS            0x0001
#define PRF_TRYPROGRAMEXTENSIONS    (0x0002 | PRF_VERIFYEXISTS)
#define PRF_FIRSTDIRDEF             0x0004
#define PRF_DONTFINDLNK             0x0008      // if PRF_TRYPROGRAMEXTENSIONS is specified
#if (NTDDI_VERSION >= NTDDI_WINXPSP2)
#define PRF_REQUIREABSOLUTE         0x0010
#endif

SHSTDAPI_(int) RestartDialog(_In_opt_ HWND hwnd, _In_opt_ PCWSTR pszPrompt, DWORD dwReturn);
#if (NTDDI_VERSION >= NTDDI_WINXP)
SHSTDAPI_(int) RestartDialogEx(_In_opt_ HWND hwnd, _In_opt_ PCWSTR pszPrompt, DWORD dwReturn, DWORD dwReasonCode);
#endif

SHSTDAPI SHCoCreateInstance(_In_opt_ PCWSTR pszCLSID, _In_opt_ const CLSID *pclsid, _In_opt_ IUnknown *pUnkOuter, _In_ REFIID riid, _Outptr_ void **ppv);

#if (NTDDI_VERSION >= NTDDI_VISTA)
SHSTDAPI SHCreateDataObject(_In_opt_ PCIDLIST_ABSOLUTE pidlFolder, _In_ UINT cidl, _In_reads_opt_(cidl) PCUITEMID_CHILD_ARRAY apidl, _In_opt_ IDataObject *pdtInner, _In_ REFIID riid, _Outptr_ void **ppv);
#endif
SHSTDAPI CIDLData_CreateFromIDArray(_In_ PCIDLIST_ABSOLUTE pidlFolder, _In_ UINT cidl, _In_reads_opt_(cidl) PCUIDLIST_RELATIVE_ARRAY apidl, _Outptr_ IDataObject **ppdtobj);
SHSTDAPI SHCreateStdEnumFmtEtc(_In_ UINT cfmt, _In_reads_(cfmt) const FORMATETC afmt[], _Outptr_ IEnumFORMATETC **ppenumFormatEtc);
SHSTDAPI SHDoDragDrop(_In_opt_ HWND hwnd, _In_ IDataObject *pdata, _In_opt_ IDropSource *pdsrc, _In_ DWORD dwEffect, _Out_ DWORD *pdwEffect);
// stuff for doing auto scrolling
#define NUM_POINTS      3
typedef struct {        // asd
    int iNextSample;
    DWORD dwLastScroll;
    BOOL bFull;
    POINT pts[NUM_POINTS];
    DWORD dwTimes[NUM_POINTS];
} AUTO_SCROLL_DATA;

SHSTDAPI_(BOOL) DAD_SetDragImage(HIMAGELIST him, POINT * pptOffset);
SHSTDAPI_(BOOL) DAD_DragEnterEx(HWND hwndTarget, const POINT ptStart);
SHSTDAPI_(BOOL) DAD_DragEnterEx2(_In_ HWND hwndTarget, const POINT ptStart, _In_opt_ IDataObject *pdtObject);
SHSTDAPI_(BOOL) DAD_ShowDragImage(BOOL fShow);
SHSTDAPI_(BOOL) DAD_DragMove(POINT pt);
SHSTDAPI_(BOOL) DAD_DragLeave(void);
SHSTDAPI_(BOOL) DAD_AutoScroll(HWND hwnd, AUTO_SCROLL_DATA *pad, const POINT *pptNow);
typedef struct {
    WORD cLength;
    WORD nVersion;

    BOOL fFullPathTitle            : 1;
    BOOL fSaveLocalView            : 1;
    BOOL fNotShell                 : 1;
    BOOL fSimpleDefault            : 1;
    BOOL fDontShowDescBar          : 1;
    BOOL fNewWindowMode            : 1;
    BOOL fShowCompColor            : 1;  // NT: Show compressed volumes in a different colour
    BOOL fDontPrettyNames          : 1;  // NT: Do 8.3 name conversion, or not!
    BOOL fAdminsCreateCommonGroups : 1;  // NT: Administrators create comon groups
    UINT fUnusedFlags : 7;

    UINT fMenuEnumFilter;

} CABINETSTATE, * LPCABINETSTATE;

#define CABINETSTATE_VERSION 2

// APIs for reading and writing the cabinet state.
SHSTDAPI_(BOOL) ReadCabinetState(_Out_writes_bytes_(cLength) CABINETSTATE *pcs, int cLength);
SHSTDAPI_(BOOL) WriteCabinetState(_In_ CABINETSTATE *pcs);
SHSTDAPI_(BOOL) PathMakeUniqueName(_Out_writes_(cchMax) PWSTR pszUniqueName, UINT cchMax, _In_ PCWSTR pszTemplate, _In_opt_ PCWSTR pszLongPlate, _In_opt_ PCWSTR pszDir);
SHSTDAPI_(BOOL) PathIsExe(_In_ PCWSTR pszPath);
//
//  Return codes from PathCleanupSpec.  Negative return values are
//  unrecoverable errors
//
#define PCS_FATAL           0x80000000
#define PCS_REPLACEDCHAR    0x00000001
#define PCS_REMOVEDCHAR     0x00000002
#define PCS_TRUNCATED       0x00000004
#define PCS_PATHTOOLONG     0x00000008  // Always combined with FATAL

SHSTDAPI_(int) PathCleanupSpec(_In_opt_ PCWSTR pszDir, _Inout_ PWSTR pszSpec);

SHSTDAPI_(int) PathResolve(_Inout_updates_(MAX_PATH) PWSTR pszPath, _In_opt_ PZPCWSTR dirs, UINT fFlags);
SHSTDAPI_(BOOL) GetFileNameFromBrowse(_In_opt_ HWND hwnd, _Inout_updates_(cchFilePath) PWSTR pszFilePath, UINT cchFilePath,
                                      _In_opt_ PCWSTR pszWorkingDir, _In_ PCWSTR pszDefExt, _In_opt_ PCWSTR pszFilters, _In_opt_ PCWSTR pszTitle);
SHSTDAPI_(int) DriveType(int iDrive);

SHSTDAPI_(int) RealDriveType(int iDrive, BOOL fOKToHitNet);
SHSTDAPI_(int) IsNetDrive(int iDrive);
// Flags for Shell_MergeMenus
#define MM_ADDSEPARATOR         0x00000001L
#define MM_SUBMENUSHAVEIDS      0x00000002L
#define MM_DONTREMOVESEPS       0x00000004L

SHSTDAPI_(UINT) Shell_MergeMenus(_In_ HMENU hmDst, _In_ HMENU hmSrc, UINT uInsert, UINT uIDAdjust, UINT uIDAdjustMax, ULONG uFlags);


/*
 * The SHObjectProperties API provides an easy way to invoke the Properties
 *   context menu command on a subset of the shell item namespace.
 *
 *   PARAMETERS
 *
 *     hwnd              The window handle of the window which will own the dialog
 *     shopObjectType    A SHOP_ value as defined below
 *     pszObjectName     Name of the object, see SHOP_ values below
 *     pszPropertyPage   The name of the property sheet page to open to or NULL.
 *
 *   RETURN
 *
 *     TRUE if the Properties command was invoked
 */
SHSTDAPI_(BOOL) SHObjectProperties(_In_opt_ HWND hwnd, _In_ DWORD shopObjectType, _In_ PCWSTR pszObjectName, _In_opt_ PCWSTR pszPropertyPage);

#define SHOP_PRINTERNAME 0x00000001  // pszObjectName points to a printer friendly name
#define SHOP_FILEPATH    0x00000002  // pszObjectName points to a fully qualified path+file name
#define SHOP_VOLUMEGUID  0x00000004  // pszObjectName points to a Volume GUID


/*
 * The SHFormatDrive API provides access to the Shell
 *   format dialog. This allows apps which want to format disks
 *   to bring up the same dialog that the Shell does to do it.
 *
 *   This dialog is not sub-classable. You cannot put custom
 *   controls in it. If you want this ability, you will have
 *   to write your own front end for the DMaint_FormatDrive
 *   engine.
 *
 *   NOTE that the user can format as many diskettes in the specified
 *   drive, or as many times, as he/she wishes to. There is no way to
 *   force any specififc number of disks to format. If you want this
 *   ability, you will have to write your own front end for the
 *   DMaint_FormatDrive engine.
 *
 *   NOTE also that the format will not start till the user pushes the
 *   start button in the dialog. There is no way to do auto start. If
 *   you want this ability, you will have to write your own front end
 *   for the DMaint_FormatDrive engine.
 *
 *   PARAMETERS
 *
 *     hwnd    = The window handle of the window which will own the dialog
 *               NOTE that unlike SHCheckDrive, hwnd == NULL does not cause
 *               this dialog to come up as a "top level application" window.
 *               This parameter should always be non-null, this dialog is
 *               only designed to be the child of another window, not a
 *               stand-alone application.
 *     drive   = The 0 based (A: == 0) drive number of the drive to format
 *     fmtID   = The ID of the physical format to format the disk with
 *               NOTE: The special value SHFMT_ID_DEFAULT means "use the
 *                     default format specified by the DMaint_FormatDrive
 *                     engine". If you want to FORCE a particular format
 *                     ID "up front" you will have to call
 *                     DMaint_GetFormatOptions yourself before calling
 *                     this to obtain the valid list of phys format IDs
 *                     (contents of the PhysFmtIDList array in the
 *                     FMTINFOSTRUCT).
 *     options = There is currently only two option bits defined
 *
 *                SHFMT_OPT_FULL
 *                SHFMT_OPT_SYSONLY
 *
 *               The normal defualt in the Shell format dialog is
 *               "Quick Format", setting this option bit indicates that
 *               the caller wants to start with FULL format selected
 *               (this is useful for folks detecting "unformatted" disks
 *               and wanting to bring up the format dialog).
 *
 *               The SHFMT_OPT_SYSONLY initializes the dialog to
 *               default to just sys the disk.
 *
 *               All other bits are reserved for future expansion and
 *               must be 0.
 *
 *               Please note that this is a bit field and not a value
 *               and treat it accordingly.
 *
 *   RETURN
 *      The return is either one of the SHFMT_* values, or if the
 *      returned DWORD value is not == to one of these values, then
 *      the return is the physical format ID of the last succesful
 *      format. The LOWORD of this value can be passed on subsequent
 *      calls as the fmtID parameter to "format the same type you did
 *      last time".
 *
 */
SHSTDAPI_(DWORD) SHFormatDrive(_In_ HWND hwnd, UINT drive, UINT fmtID, UINT options);

//
// Special value of fmtID which means "use the default format"
//
#define SHFMT_ID_DEFAULT    0xFFFF

//
// Option bits for options parameter
//
#define SHFMT_OPT_FULL     0x0001
#define SHFMT_OPT_SYSONLY  0x0002

//
// Special return values. PLEASE NOTE that these are DWORD values.
//
#define SHFMT_ERROR     0xFFFFFFFFL     // Error on last format, drive may be formatable
#define SHFMT_CANCEL    0xFFFFFFFEL     // Last format was canceled
#define SHFMT_NOFORMAT  0xFFFFFFFDL     // Drive is not formatable

#ifndef HPSXA_DEFINED
#define HPSXA_DEFINED
DECLARE_HANDLE( HPSXA );
#endif
WINSHELLAPI void WINAPI SHDestroyPropSheetExtArray(_In_ HPSXA hpsxa);
WINSHELLAPI UINT WINAPI SHAddFromPropSheetExtArray(_In_ HPSXA hpsxa, _In_ LPFNADDPROPSHEETPAGE lpfnAddPage, LPARAM lParam);
WINSHELLAPI UINT WINAPI SHReplaceFromPropSheetExtArray(_In_ HPSXA hpsxa, UINT uPageID, _In_ LPFNADDPROPSHEETPAGE lpfnReplaceWith, LPARAM lParam);

// shell restriction values, parameter for SHRestricted()
typedef enum RESTRICTIONS
{
    REST_NONE                       = 0x00000000,
    REST_NORUN                      = 0x00000001,
    REST_NOCLOSE                    = 0x00000002,
    REST_NOSAVESET                  = 0x00000004,
    REST_NOFILEMENU                 = 0x00000008,
    REST_NOSETFOLDERS               = 0x00000010,
    REST_NOSETTASKBAR               = 0x00000020,
    REST_NODESKTOP                  = 0x00000040,
    REST_NOFIND                     = 0x00000080,
    REST_NODRIVES                   = 0x00000100,
    REST_NODRIVEAUTORUN             = 0x00000200,
    REST_NODRIVETYPEAUTORUN         = 0x00000400,
    REST_NONETHOOD                  = 0x00000800,
    REST_STARTBANNER                = 0x00001000,
    REST_RESTRICTRUN                = 0x00002000,
    REST_NOPRINTERTABS              = 0x00004000,
    REST_NOPRINTERDELETE            = 0x00008000,
    REST_NOPRINTERADD               = 0x00010000,
    REST_NOSTARTMENUSUBFOLDERS      = 0x00020000,
    REST_MYDOCSONNET                = 0x00040000,
    REST_NOEXITTODOS                = 0x00080000,
    REST_ENFORCESHELLEXTSECURITY    = 0x00100000,
    REST_LINKRESOLVEIGNORELINKINFO  = 0x00200000,
    REST_NOCOMMONGROUPS             = 0x00400000,
    REST_SEPARATEDESKTOPPROCESS     = 0x00800000,
    REST_NOWEB                      = 0x01000000,
    REST_NOTRAYCONTEXTMENU          = 0x02000000,
    REST_NOVIEWCONTEXTMENU          = 0x04000000,
    REST_NONETCONNECTDISCONNECT     = 0x08000000,
    REST_STARTMENULOGOFF            = 0x10000000,
    REST_NOSETTINGSASSIST           = 0x20000000,
    REST_NOINTERNETICON             = 0x40000001,
    REST_NORECENTDOCSHISTORY        = 0x40000002,
    REST_NORECENTDOCSMENU           = 0x40000003,
    REST_NOACTIVEDESKTOP            = 0x40000004,
    REST_NOACTIVEDESKTOPCHANGES     = 0x40000005,
    REST_NOFAVORITESMENU            = 0x40000006,
    REST_CLEARRECENTDOCSONEXIT      = 0x40000007,
    REST_CLASSICSHELL               = 0x40000008,
    REST_NOCUSTOMIZEWEBVIEW         = 0x40000009,
    REST_NOHTMLWALLPAPER            = 0x40000010,
    REST_NOCHANGINGWALLPAPER        = 0x40000011,
    REST_NODESKCOMP                 = 0x40000012,
    REST_NOADDDESKCOMP              = 0x40000013,
    REST_NODELDESKCOMP              = 0x40000014,
    REST_NOCLOSEDESKCOMP            = 0x40000015,
    REST_NOCLOSE_DRAGDROPBAND       = 0x40000016,   // Disable Close and Drag & Drop on ALL Bands
    REST_NOMOVINGBAND               = 0x40000017,   // Disable Moving ALL Bands
    REST_NOEDITDESKCOMP             = 0x40000018,
    REST_NORESOLVESEARCH            = 0x40000019,
    REST_NORESOLVETRACK             = 0x4000001A,
    REST_FORCECOPYACLWITHFILE       = 0x4000001B,
#if (NTDDI_VERSION < NTDDI_VISTA)
    REST_NOLOGO3CHANNELNOTIFY       = 0x4000001C,
#endif
    REST_NOFORGETSOFTWAREUPDATE     = 0x4000001D,
    REST_NOSETACTIVEDESKTOP         = 0x4000001E,   // No Active desktop on Settings Menu
    REST_NOUPDATEWINDOWS            = 0x4000001F,   // No Windows Update on Settings Menu
    REST_NOCHANGESTARMENU           = 0x40000020,   // No Context menu or Drag and Drop on Start menu
    REST_NOFOLDEROPTIONS            = 0x40000021,   // No Folder Options on Settings Menu
    REST_HASFINDCOMPUTERS           = 0x40000022,   // Show Start/Search/Computers
    REST_INTELLIMENUS               = 0x40000023,
    REST_RUNDLGMEMCHECKBOX          = 0x40000024,
    REST_ARP_ShowPostSetup          = 0x40000025,   // ARP: Show Post-Setup page
    REST_NOCSC                      = 0x40000026,   // Disable the ClientSide caching on SM
    REST_NOCONTROLPANEL             = 0x40000027,   // Remove the Control Panel only from SM|Settings
    REST_ENUMWORKGROUP              = 0x40000028,   // Enumerate workgroup in root of nethood
    REST_ARP_NOARP                  = 0x40000029,   // ARP: Don't Allow ARP to come up at all
    REST_ARP_NOREMOVEPAGE           = 0x4000002A,   // ARP: Don't allow Remove page
    REST_ARP_NOADDPAGE              = 0x4000002B,   // ARP: Don't allow Add page
    REST_ARP_NOWINSETUPPAGE         = 0x4000002C,   // ARP: Don't allow opt components page
    REST_GREYMSIADS                 = 0x4000002D,    // SM: Allow the greying of Darwin Ads in SM
    REST_NOCHANGEMAPPEDDRIVELABEL   = 0x4000002E,   // Don't enable the UI which allows users to rename mapped drive labels
    REST_NOCHANGEMAPPEDDRIVECOMMENT = 0x4000002F,   // Don't enable the UI which allows users to change mapped drive comments
    REST_MaxRecentDocs              = 0x40000030,
    REST_NONETWORKCONNECTIONS       = 0x40000031,   // No Start Menu | Settings |Network Connections
    REST_FORCESTARTMENULOGOFF       = 0x40000032,   // Force logoff on the Start Menu
    REST_NOWEBVIEW                  = 0x40000033,   // Disable Web View
    REST_NOCUSTOMIZETHISFOLDER      = 0x40000034,   // Disable Customize This Folder
    REST_NOENCRYPTION               = 0x40000035,   // Don't allow file encryption
//  Do NOT use me                     0x40000036,
    REST_DONTSHOWSUPERHIDDEN        = 0x40000037,   // don't show super hidden files
    REST_NOSHELLSEARCHBUTTON        = 0x40000038,
    REST_NOHARDWARETAB              = 0x40000039,   // No Hardware tab on Drives or in control panel
    REST_NORUNASINSTALLPROMPT       = 0x4000003A,   // Don't bring up "Run As" prompt for install programs
    REST_PROMPTRUNASINSTALLNETPATH  = 0x4000003B,   // Force the  "Run As" prompt for install programs on unc/network shares
    REST_NOMANAGEMYCOMPUTERVERB     = 0x4000003C,   // No Manage verb on My Computer
//  Do NOT use me                     0x4000003D,
    REST_DISALLOWRUN                = 0x4000003E,   // don't allow certain apps to be run
    REST_NOWELCOMESCREEN            = 0x4000003F,   // don't allow the welcome screen to be displayed.
    REST_RESTRICTCPL                = 0x40000040,   // only allow certain cpls to be run
    REST_DISALLOWCPL                = 0x40000041,   // don't allow certain cpls to be run
    REST_NOSMBALLOONTIP             = 0x40000042,   // No Start Menu Balloon Tip
    REST_NOSMHELP                   = 0x40000043,   // No Help on the Start Menu
    REST_NOWINKEYS                  = 0x40000044,   // No Windows-X Hot keys
    REST_NOENCRYPTONMOVE            = 0x40000045,   // Don't automatically try to encrypt files that are moved to encryped directories
    REST_NOLOCALMACHINERUN          = 0x40000046,   // ignore HKLM\sw\ms\win\cv\Run and all of it's sub keys
    REST_NOCURRENTUSERRUN           = 0x40000047,   // ignore HKCU\sw\ms\win\cv\Run and all of it's sub keys
    REST_NOLOCALMACHINERUNONCE      = 0x40000048,   // ignore HKLM\sw\ms\win\cv\RunOnce and all of it's sub keys
    REST_NOCURRENTUSERRUNONCE       = 0x40000049,   // ignore HKCU\sw\ms\win\cv\RunOnce and all of it's sub keys
    REST_FORCEACTIVEDESKTOPON       = 0x4000004A,   // Force ActiveDesktop to be turned ON all the time.
//  Do NOT use me                     0x4000004B,
    REST_NOVIEWONDRIVE              = 0x4000004C,   // disallows CreateViewObject() on specified drives (CFSFolder only)
#if (NTDDI_VERSION >= NTDDI_WINXP) || defined(IE_BACKCOMPAT_VERSION)
    REST_NONETCRAWL                 = 0x4000004D,   // disables the crawling of the WNet namespace.
    REST_NOSHAREDDOCUMENTS          = 0x4000004E,   // don't auto share the Shared Documents/create link
#endif  // NTDDI_WINXP
    REST_NOSMMYDOCS                 = 0x4000004F,   // Don't show the My Documents item on the Start Menu.
#if (NTDDI_VERSION >= NTDDI_WINXP)
    REST_NOSMMYPICS                 = 0x40000050,   // Don't show the My Pictures item on the Start Menu
    REST_ALLOWBITBUCKDRIVES         = 0x40000051,   // Bit mask indicating which which drives have bit bucket support
    REST_NONLEGACYSHELLMODE         = 0x40000052,   // new consumer shell modes
    REST_NOCONTROLPANELBARRICADE    = 0x40000053,   // The webview barricade in Control Panel
    REST_NOSTARTPAGE                = 0x40000054,   // Whistler Start Page on desktop.
    REST_NOAUTOTRAYNOTIFY           = 0x40000055,   // Whistler auto-tray notify feature
    REST_NOTASKGROUPING             = 0x40000056,   // Whistler taskbar button grouping feature
    REST_NOCDBURNING                = 0x40000057,   // whistler cd burning feature
#endif  // NTDDI_WINXP
#if (NTDDI_VERSION >= NTDDI_WIN2KSP3)
    REST_MYCOMPNOPROP               = 0x40000058,   // disables Properties on My Computer's context menu
    REST_MYDOCSNOPROP               = 0x40000059,   // disables Properties on My Documents' context menu
#endif  // NTDDI_WIN2KSP3
#if (NTDDI_VERSION >= NTDDI_WINXP)
    REST_NOSTARTPANEL               = 0x4000005A,   // Windows start panel (New start menu) for Whistler.
    REST_NODISPLAYAPPEARANCEPAGE    = 0x4000005B,   // disable Themes and Appearance tabs in the Display Control Panel.
    REST_NOTHEMESTAB                = 0x4000005C,   // disable the Themes tab in the Display Control Panel.
    REST_NOVISUALSTYLECHOICE        = 0x4000005D,   // disable the visual style drop down in the Appearance tab of the Display Control Panel.
    REST_NOSIZECHOICE               = 0x4000005E,   // disable the size drop down in the Appearance tab of the Display Control Panel.
    REST_NOCOLORCHOICE              = 0x4000005F,   // disable the color drop down in the Appearance tab of the Display Control Panel.
    REST_SETVISUALSTYLE             = 0x40000060,   // Load the specified file as the visual style.
#endif  // NTDDI_WINXP
#if (NTDDI_VERSION >= NTDDI_WIN2KSP3)
    REST_STARTRUNNOHOMEPATH         = 0x40000061,   // dont use the %HOMEPATH% env var for the Start-Run dialog
#endif  // NTDDI_WIN2KSP3
#if (NTDDI_VERSION >= NTDDI_WINXP)
    REST_NOUSERNAMEINSTARTPANEL     = 0x40000062,   // don't show the username is the startpanel.
    REST_NOMYCOMPUTERICON           = 0x40000063,   // don't show my computer anywhere, hide its contents
    REST_NOSMNETWORKPLACES          = 0x40000064,   // don't show network places in startpanel.
    REST_NOSMPINNEDLIST             = 0x40000065,   // don't show the pinned list in startpanel.
    REST_NOSMMYMUSIC                = 0x40000066,   // don't show MyMusic folder in startpanel
    REST_NOSMEJECTPC                = 0x40000067,   // don't show "Undoc PC" command in startmenu
    REST_NOSMMOREPROGRAMS           = 0x40000068,   // don't show "More Programs" button in StartPanel.
    REST_NOSMMFUPROGRAMS            = 0x40000069,   // don't show the MFU programs list in StartPanel.
    REST_NOTRAYITEMSDISPLAY         = 0x4000006A,   // disables the display of the system tray
    REST_NOTOOLBARSONTASKBAR        = 0x4000006B,   // disables toolbar display on the taskbar
#endif  // NTDDI_WINXP
#if (NTDDI_VERSION >= NTDDI_WIN2KSP3)
    REST_NOSMCONFIGUREPROGRAMS      = 0x4000006F,   // No Configure Programs on Settings Menu
#endif  // NTDDI_WIN2KSP3
#if (NTDDI_VERSION >= NTDDI_WINXP)
    REST_HIDECLOCK                  = 0x40000070,   // don't show the clock
    REST_NOLOWDISKSPACECHECKS       = 0x40000071,   // disable the low disk space checking
#endif  // NTDDI_WINXP
#if (NTDDI_VERSION >= NTDDI_WIN2KSP4)
    REST_NOENTIRENETWORK            = 0x40000072,   // removes the "Entire Network" link (i.e. from "My Network Places")
#endif  // NTDDI_WIN2KSP4
#if (NTDDI_VERSION >= NTDDI_WINXP)
    REST_NODESKTOPCLEANUP           = 0x40000073,   // disable the desktop cleanup wizard
    REST_BITBUCKNUKEONDELETE        = 0x40000074,   // disables recycling of files
    REST_BITBUCKCONFIRMDELETE       = 0x40000075,   // always show the delete confirmation dialog when deleting files
    REST_BITBUCKNOPROP              = 0x40000076,   // disables Properties on Recycle Bin's context menu
    REST_NODISPBACKGROUND           = 0x40000077,   // disables the Desktop tab in the Display CPL
    REST_NODISPSCREENSAVEPG         = 0x40000078,   // disables the Screen Saver tab in the Display CPL
    REST_NODISPSETTINGSPG           = 0x40000079,   // disables the Settings tab in the Display CPL
    REST_NODISPSCREENSAVEPREVIEW    = 0x4000007A,   // disables the screen saver on the Screen Saver tab in the Display CPL
    REST_NODISPLAYCPL               = 0x4000007B,   // disables the Display CPL
    REST_HIDERUNASVERB              = 0x4000007C,   // hides the "Run As..." context menu item
    REST_NOTHUMBNAILCACHE           = 0x4000007D,   // disables use of the thumbnail cache
#endif  // NTDDI_WINXP
#if (NTDDI_VERSION >= NTDDI_WINXPSP1) || defined(IE_BACKCOMPAT_VERSION)
    REST_NOSTRCMPLOGICAL            = 0x4000007E,   // dont use StrCmpLogical() instead use default CompareString()
    REST_NOPUBLISHWIZARD            = 0x4000007F,   // disables publishing wizard (WPW)
    REST_NOONLINEPRINTSWIZARD       = 0x40000080,   // disables online prints wizard (OPW)
    REST_NOWEBSERVICES              = 0x40000081,   // disables the web specified services for both OPW and WPW
#endif  // NTDDI_WINXPSP1
#if (NTDDI_VERSION >= NTDDI_WIN2KSP3)
    REST_ALLOWUNHASHEDWEBVIEW       = 0x40000082,   // allow the user to be promted to accept web view templates that don't already have an md5 hash in the registry
#endif  // NTDDI_WIN2KSP3
    REST_ALLOWLEGACYWEBVIEW         = 0x40000083,   // allow legacy webview template to be shown.
#if (NTDDI_VERSION >= NTDDI_WIN2KSP3)
    REST_REVERTWEBVIEWSECURITY      = 0x40000084,   // disable added webview security measures (revert to w2k functionality).
#endif  // NTDDI_WIN2KSP3
#if (NTDDI_VERSION >= NTDDI_WIN2KSP4)
    REST_INHERITCONSOLEHANDLES      = 0x40000086,   // ShellExec() will check for the current process and target process being console processes to inherit handles
#endif  // NTDDI_WIN2KSP4
#if (NTDDI_VERSION >= NTDDI_WINXPSP2 && NTDDI_VERSION < NTDDI_VISTA)
    REST_SORTMAXITEMCOUNT           = 0x40000087,   // Do not sort views with more items than this key. Useful for viewing big amount of files in one folder.
#endif
#if (NTDDI_VERSION >= NTDDI_WINXPSP2)
    REST_NOREMOTERECURSIVEEVENTS    = 0x40000089,   // Dont register network change events recursively to avoid network traffic
#endif  // NTDDI_WINXPSP2
#if (NTDDI_VERSION >= NTDDI_WINXPSP2)
    REST_NOREMOTECHANGENOTIFY       = 0x40000091,   // Do not notify for remote changy notifies
#if (NTDDI_VERSION < NTDDI_VISTA)
    REST_NOSIMPLENETIDLIST          = 0x40000092,   // No simple network IDLists
#endif
    REST_NOENUMENTIRENETWORK        = 0x40000093,   // Don't enumerate entire network if we happen to get to it (in conjunction with REST_NOENTIRENETWORK)
#if (NTDDI_VERSION < NTDDI_VISTA)
    REST_NODETAILSTHUMBNAILONNETWORK= 0x40000094,   // Disable Thumbnail for Network files in DUI Details pane
#endif
    REST_NOINTERNETOPENWITH         = 0x40000095,   // dont allow looking on the internet for file associations
#endif  // NTDDI_WINXPSP2
#if (NTDDI_VERSION >= NTDDI_WINXPSP2)
    REST_DONTRETRYBADNETNAME        = 0x4000009B,   // In Network Places: if provider returns ERROR_BAD_NET_NAME, give up
    REST_ALLOWFILECLSIDJUNCTIONS    = 0x4000009C,   // re-enable legacy support for file.{guid} junctions in FileSystem Folder
    REST_NOUPNPINSTALL              = 0x4000009D,   // disable "install UPnP" task in My Net Places
#endif  // NTDDI_WINXPSP2
    REST_ARP_DONTGROUPPATCHES       = 0x400000AC,   //List individual patches in Add/Remove Programs
    REST_ARP_NOCHOOSEPROGRAMSPAGE   = 0x400000AD,   //Choose programs page

    REST_NODISCONNECT               = 0x41000001,   // No Disconnect option in Start menu
    REST_NOSECURITY                 = 0x41000002,   // No Security option in start menu
    REST_NOFILEASSOCIATE            = 0x41000003,   // Do not allow user to change file association
#if (NTDDI_VERSION >= NTDDI_WINXPSP2)
    REST_ALLOWCOMMENTTOGGLE         = 0x41000004,   // Allow the user to toggle the positions of the Comment and the Computer Name
#if (NTDDI_VERSION < NTDDI_VISTA)
    REST_USEDESKTOPINICACHE         = 0x41000005,   // Cache desktop.ini entries from network folders
#endif  // NTDDI_VISTA
#endif  // NTDDI_WINXPSP2
} RESTRICTIONS;
SHSTDAPI_(IStream *) OpenRegStream(_In_ HKEY hkey, _In_opt_ PCWSTR pszSubkey, _In_opt_ PCWSTR pszValue, DWORD grfMode);
SHSTDAPI_(BOOL) SHFindFiles(_In_opt_ PCIDLIST_ABSOLUTE pidlFolder, _In_opt_ PCIDLIST_ABSOLUTE pidlSaveFile);
SHSTDAPI_(void) PathGetShortPath(_Inout_updates_(MAX_PATH) PWSTR pszLongPath);
_Success_(return != 0)
SHSTDAPI_(BOOL) PathYetAnotherMakeUniqueName(_Out_writes_(MAX_PATH) PWSTR pszUniqueName, _In_ PCWSTR pszPath, _In_opt_ PCWSTR pszShort, _In_opt_ PCWSTR pszFileSpec);
SHSTDAPI_(BOOL) Win32DeleteFile(_In_ PCWSTR pszPath);
SHSTDAPI_(DWORD) SHRestricted(RESTRICTIONS rest);
SHSTDAPI_(BOOL) SignalFileOpen(_In_ PCIDLIST_ABSOLUTE pidl);

#if (NTDDI_VERSION >= NTDDI_VISTA)
SHSTDAPI AssocGetDetailsOfPropKey(_In_ IShellFolder *psf, _In_ PCUITEMID_CHILD pidl, _In_ const PROPERTYKEY *pkey, _Out_ VARIANT *pv, _Out_opt_ BOOL *pfFoundPropKey);
#endif


// both ANSI and UNICODE
SHSTDAPI SHStartNetConnectionDialogA(_In_opt_ HWND hwnd, _In_opt_ LPCSTR pszRemoteName, DWORD dwType);
// both ANSI and UNICODE
SHSTDAPI SHStartNetConnectionDialogW(_In_opt_ HWND hwnd, _In_opt_ LPCWSTR pszRemoteName, DWORD dwType);
#ifdef UNICODE
#define SHStartNetConnectionDialog  SHStartNetConnectionDialogW
#else
#define SHStartNetConnectionDialog  SHStartNetConnectionDialogA
#endif // !UNICODE
SHSTDAPI SHDefExtractIconA(_In_ LPCSTR pszIconFile, int iIndex, UINT uFlags,
                           _Out_opt_ HICON *phiconLarge, _Out_opt_ HICON *phiconSmall, UINT nIconSize);
SHSTDAPI SHDefExtractIconW(_In_ LPCWSTR pszIconFile, int iIndex, UINT uFlags,
                           _Out_opt_ HICON *phiconLarge, _Out_opt_ HICON *phiconSmall, UINT nIconSize);
#ifdef UNICODE
#define SHDefExtractIcon  SHDefExtractIconW
#else
#define SHDefExtractIcon  SHDefExtractIconA
#endif // !UNICODE

// Elevation

// OpenAsInfo flags
enum tagOPEN_AS_INFO_FLAGS {
    OAIF_ALLOW_REGISTRATION = 0x00000001,     // enable the "always use this file" checkbox (NOTE if you don't pass this, it will be disabled)
    OAIF_REGISTER_EXT       = 0x00000002,     // do the registration after the user hits "ok"
    OAIF_EXEC               = 0x00000004,     // execute file after registering
    OAIF_FORCE_REGISTRATION = 0x00000008,     // force the "always use this file" checkbox to be checked (normally, you won't use the OAIF_ALLOW_REGISTRATION when you pass this)
#if (NTDDI_VERSION >= NTDDI_VISTA)
    OAIF_HIDE_REGISTRATION  = 0x00000020,     // hide the "always use this file" checkbox
    OAIF_URL_PROTOCOL       = 0x00000040,     // the "extension" passed is actually a protocol (uri scheme), and open with should show apps registered as capable of handling that protocol
#endif
#if (NTDDI_VERSION >= NTDDI_WIN8)
    OAIF_FILE_IS_URI        = 0x00000080,     // pcszFile is actually a URI
#endif
};
typedef int OPEN_AS_INFO_FLAGS;


#include <pshpack8.h>

typedef struct _openasinfo
{
    LPCWSTR pcszFile;               // [in] file name, or protocol name if
                                    //      OAIF_URL_PROTOCOL is set.
    LPCWSTR pcszClass;              // [in] file class description. NULL means
                                    //      use pcszFile's extension. ignored
                                    //      if OAIF_URL_PROTOCOL is set.
    OPEN_AS_INFO_FLAGS oaifInFlags; // [in] input flags from OPEN_AS_INFO_FLAGS enumeration
} OPENASINFO, * POPENASINFO;

#include <poppack.h>        /* Return to byte packing */

#if (NTDDI_VERSION >= NTDDI_VISTA)
SHSTDAPI SHOpenWithDialog(_In_opt_ HWND hwndParent, _In_ const OPENASINFO* poainfo);
#endif // NTDDI_VISTA

SHSTDAPI_(BOOL) Shell_GetImageLists(_Out_opt_ HIMAGELIST *phiml, _Out_opt_ HIMAGELIST *phimlSmall);
SHSTDAPI_(int)  Shell_GetCachedImageIndex(_In_ PCWSTR pwszIconPath, int iIconIndex, UINT uIconFlags);
#if (NTDDI_VERSION >= NTDDI_VISTA)
SHSTDAPI_(int)  Shell_GetCachedImageIndexA(_In_ LPCSTR pszIconPath, int iIconIndex, UINT uIconFlags);
SHSTDAPI_(int)  Shell_GetCachedImageIndexW(_In_ LPCWSTR pszIconPath, int iIconIndex, UINT uIconFlags);
#ifdef UNICODE
#define Shell_GetCachedImageIndex  Shell_GetCachedImageIndexW
#else
#define Shell_GetCachedImageIndex  Shell_GetCachedImageIndexA
#endif // !UNICODE
#endif // NTDDI_VISTA
#define VALIDATEUNC_CONNECT     0x0001          // connect a drive letter
#define VALIDATEUNC_NOUI        0x0002          // don't bring up UI
#define VALIDATEUNC_PRINT       0x0004          // validate as print share instead of disk share
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define VALIDATEUNC_PERSIST     0x0008          // the connection should be made persistent
#define VALIDATEUNC_VALID       0x000F          // valid flags
#else
#define VALIDATEUNC_VALID       0x0007          // valid flags
#endif

SHSTDAPI_(BOOL) SHValidateUNC(_In_opt_ HWND hwndOwner, _Inout_ PWSTR pszFile, UINT fConnect);
#define PIFNAMESIZE     30
#define PIFSTARTLOCSIZE 63
#define PIFDEFPATHSIZE  64
#define PIFPARAMSSIZE   64
#define PIFSHPROGSIZE   64
#define PIFSHDATASIZE   64
#define PIFDEFFILESIZE  80
#define PIFMAXFILEPATH  260

typedef struct PROPPRG {                    /* prg */
    WORD    flPrg;                          // see PRG_ flags
    WORD    flPrgInit;                      // see PRGINIT_ flags
    CHAR    achTitle[PIFNAMESIZE];          // name[30]
    CHAR    achCmdLine[PIFSTARTLOCSIZE+PIFPARAMSSIZE+1];// startfile[63] + params[64]
    CHAR    achWorkDir[PIFDEFPATHSIZE];     // defpath[64]
    WORD    wHotKey;                        // PfHotKeyScan thru PfHotKeyVal
    CHAR    achIconFile[PIFDEFFILESIZE];    // name of file containing icon
    WORD    wIconIndex;                     // index of icon within file
    DWORD   dwEnhModeFlags;                 // reserved enh-mode flags
    DWORD   dwRealModeFlags;                // real-mode flags (see RMOPT_*)
    CHAR    achOtherFile[PIFDEFFILESIZE];   // name of "other" file in directory
    CHAR    achPIFFile[PIFMAXFILEPATH];     // name of PIF file
} PROPPRG;
typedef UNALIGNED PROPPRG *PPROPPRG;
typedef UNALIGNED PROPPRG FAR *LPPROPPRG;
typedef const UNALIGNED PROPPRG FAR *LPCPROPPRG;

SHSTDAPI_(HANDLE) PifMgr_OpenProperties(_In_ PCWSTR pszApp, _In_opt_ PCWSTR pszPIF, UINT hInf, UINT flOpt);
SHSTDAPI_(int)    PifMgr_GetProperties(_In_opt_ HANDLE hProps, _In_opt_ PCSTR pszGroup, _Out_writes_bytes_opt_(cbProps) void *lpProps, int cbProps, UINT flOpt);
SHSTDAPI_(int)    PifMgr_SetProperties(_In_opt_ HANDLE hProps, _In_opt_ PCSTR pszGroup, _In_reads_bytes_(cbProps) const void *lpProps, int cbProps, UINT flOpt);
SHSTDAPI_(HANDLE) PifMgr_CloseProperties(_In_opt_ HANDLE hProps, UINT flOpt);

SHSTDAPI_(void) SHSetInstanceExplorer(_In_opt_ IUnknown *punk);
SHSTDAPI_(BOOL) IsUserAnAdmin(void);

SHSTDAPI_(LRESULT) SHShellFolderView_Message(_In_ HWND hwndMain, UINT uMsg, LPARAM lParam);

//
// Callback interface for the IShellView object returned from SHCreateShellFolderView.

//
#undef  INTERFACE
#define INTERFACE   IShellFolderViewCB

DECLARE_INTERFACE_IID_(IShellFolderViewCB, IUnknown, "2047E320-F2A9-11CE-AE65-08002B2E1262")
{
    STDMETHOD(MessageSFVCB)(THIS_ UINT uMsg, WPARAM wParam, LPARAM lParam) PURE;
};

#include <pshpack8.h>

#define QCMINFO_PLACE_BEFORE    0
#define QCMINFO_PLACE_AFTER     1
typedef struct _QCMINFO_IDMAP_PLACEMENT
{
    UINT id;
    UINT fFlags;
} QCMINFO_IDMAP_PLACEMENT;

typedef struct _QCMINFO_IDMAP
{
    UINT                    nMaxIds;
    QCMINFO_IDMAP_PLACEMENT pIdList[1];
} QCMINFO_IDMAP;

typedef struct _QCMINFO
{
    HMENU       hmenu;          // in
    UINT        indexMenu;      // in
    UINT        idCmdFirst;     // in/out
    UINT        idCmdLast;      // in
    QCMINFO_IDMAP const*  pIdMap; // in / unused
} QCMINFO;
typedef QCMINFO * LPQCMINFO;

typedef struct _DETAILSINFO
{
    PCUITEMID_CHILD pidl;
    int fmt;
    int cxChar;
    STRRET str;
    int iImage;
} DETAILSINFO;
typedef DETAILSINFO *PDETAILSINFO;

typedef struct _SFVM_PROPPAGE_DATA
{
    DWORD                dwReserved;
    LPFNADDPROPSHEETPAGE pfn;
    LPARAM               lParam;
} SFVM_PROPPAGE_DATA;

typedef struct _SFVM_HELPTOPIC_DATA
{
    WCHAR wszHelpFile[MAX_PATH];
    WCHAR wszHelpTopic[MAX_PATH];
} SFVM_HELPTOPIC_DATA;

//                                 uMsg    wParam             lParam
#define SFVM_MERGEMENU             1    // -                  LPQCMINFO
#define SFVM_INVOKECOMMAND         2    // idCmd              -
#define SFVM_GETHELPTEXT           3    // idCmd,cchMax       pszText
#define SFVM_GETTOOLTIPTEXT        4    // idCmd,cchMax       pszText
#define SFVM_GETBUTTONINFO         5    // -                  LPTBINFO
#define SFVM_GETBUTTONS            6    // idCmdFirst,cbtnMax LPTBBUTTON
#define SFVM_INITMENUPOPUP         7    // idCmdFirst,nIndex  hmenu
#define SFVM_FSNOTIFY             14    // LPCITEMIDLIST*     lEvent
#define SFVM_WINDOWCREATED        15    // hwnd               -
#define SFVM_GETDETAILSOF         23    // iColumn            DETAILSINFO*
#define SFVM_COLUMNCLICK          24    // iColumn            -
#define SFVM_QUERYFSNOTIFY        25    // -                  SHChangeNotifyEntry *
#define SFVM_DEFITEMCOUNT         26    // -                  UINT*
#define SFVM_DEFVIEWMODE          27    // -                  FOLDERVIEWMODE*
#define SFVM_UNMERGEMENU          28    // -                  hmenu
#define SFVM_UPDATESTATUSBAR      31    // fInitialize        -
#define SFVM_BACKGROUNDENUM       32    // -                  -
#define SFVM_DIDDRAGDROP          36    // dwEffect           IDataObject *
#define SFVM_SETISFV              39    // -                  IShellFolderView*
#define SFVM_THISIDLIST           41    // -                  LPITMIDLIST*
#define SFVM_ADDPROPERTYPAGES     47    // -                  SFVM_PROPPAGE_DATA *
#define SFVM_BACKGROUNDENUMDONE   48    // -                  -
#define SFVM_GETNOTIFY            49    // LPITEMIDLIST*      LONG*
#define SFVM_GETSORTDEFAULTS      53    // iDirection         iParamSort
#define SFVM_SIZE                 57    // -                  -
#define SFVM_GETZONE              58    // -                  DWORD*
#define SFVM_GETPANE              59    // Pane ID            DWORD*
#define SFVM_GETHELPTOPIC         63    // -                  SFVM_HELPTOPIC_DATA *
#define SFVM_GETANIMATION         68    // HINSTANCE *        WCHAR *

// IShellFolderView
//
// Deprecated: use IFolderView and IFolderView2 instead.
//
// IShellFolderView is supported by the IShellView object returned from SHCreateShellFolderView.
//
// Warnings:
//  - Some methods on this interface do not follow standard COM rules.
//  - Some methods can be used to configure the IShellView or cause it to behave incorrectly.
//  - Few of these methods have parameter or range validation, so callers can cause the IShellView to fault.

typedef struct _ITEMSPACING
{
    int cxSmall;
    int cySmall;
    int cxLarge;
    int cyLarge;
} ITEMSPACING;

// defines for IShellFolderView::SetObjectCount
#define SFVSOC_INVALIDATE_ALL   0x00000001  // Assumed to reset only what is neccessary...
#define SFVSOC_NOSCROLL         LVSICF_NOSCROLL

// defines for IShellFolderView::SelectItems()
#define SFVS_SELECT_NONE        0x0 // unselect all
#define SFVS_SELECT_ALLITEMS    0x1 // select all
#define SFVS_SELECT_INVERT      0x2 // Invert the selection

#undef  INTERFACE
#define INTERFACE   IShellFolderView

DECLARE_INTERFACE_IID_(IShellFolderView, IUnknown, "37A378C0-F82D-11CE-AE65-08002B2E1262")
{
    STDMETHOD(Rearrange) (THIS_ LPARAM lParamSort) PURE; // use IFolderView2::SetSortColumns
    STDMETHOD(GetArrangeParam) (THIS_ _Out_ LPARAM *plParamSort) PURE; // use IFolderView2::GetSortColumns
    STDMETHOD(ArrangeGrid) (THIS) PURE; // select Arrange by Grid
    STDMETHOD(AutoArrange) (THIS) PURE; // select Auto Arrange
    STDMETHOD(GetAutoArrange) (THIS) PURE; // use IFolderView::GetAutoArrange
    STDMETHOD(AddObject) (THIS_ _In_ PUITEMID_CHILD pidl, _Out_ UINT *puItem) PURE; // items added here may disappear (the data source is the final arbiter of which items are available to the view)
    STDMETHOD(GetObject) (THIS_ _Outptr_ PITEMID_CHILD *ppidl, UINT uItem) PURE; // use IFolderView::Item
    STDMETHOD(RemoveObject) (THIS_ _In_opt_ PUITEMID_CHILD pidl, _Out_ UINT *puItem) PURE; // items removed here may reappear (the data source is the final arbiter of which items are available to the view)
    STDMETHOD(GetObjectCount) (THIS_ _Out_ UINT *puCount) PURE; // use IFolderView::ItemCount
    STDMETHOD(SetObjectCount) (THIS_ UINT uCount, UINT dwFlags) PURE; // not implemented on Vista.  Sends LVM_SETITEMCOUNT with WPARAM=uCount and LPARAM=dwFlags to listview on XP.
    STDMETHOD(UpdateObject) (THIS_ _In_ PUITEMID_CHILD pidlOld, _In_ PUITEMID_CHILD pidlNew, _Out_ UINT *puItem) PURE; // swaps ITEMID_CHILDs, returning new index.  Changes may be discarded (the data source is the final arbiter of which items are available to the view)
    STDMETHOD(RefreshObject) (THIS_ _In_ PUITEMID_CHILD pidl, _Out_ UINT *puItem) PURE; // tickles the listview to re-draw the item
    STDMETHOD(SetRedraw) (THIS_ BOOL bRedraw) PURE; // sends WM_SETREDRAW to the listview
    STDMETHOD(GetSelectedCount) (THIS_ _Out_ UINT *puSelected) PURE; // use IFolderView2::GetSelection
    // NOTE: GetSelectedObjects hands out const pointers to internal ITEMID_CHILD structures. The caller is expected to act on them immediately (and not cache them).  LocalFree the array, but not the items it contains.
    STDMETHOD(GetSelectedObjects) (THIS_ _Outptr_result_buffer_(*puItems) PCUITEMID_CHILD **pppidl, _Out_ UINT *puItems) PURE; // use IFolderView2::GetSelection.
    STDMETHOD(IsDropOnSource) (THIS_ _In_opt_ IDropTarget *pDropTarget) PURE; // use IFolderView2::IsMoveInSameFolder
    STDMETHOD(GetDragPoint) (THIS_ _Out_ POINT *ppt) PURE; // returns point corresponding to drag-and-drop operation
    STDMETHOD(GetDropPoint) (THIS_ _Out_ POINT *ppt) PURE; // returns point corresponding to drag-and-drop operation
    STDMETHOD(MoveIcons) (THIS_ _In_ IDataObject *pDataObject) PURE; // not implemented
    STDMETHOD(SetItemPos) (THIS_ _In_ PCUITEMID_CHILD pidl, _In_ POINT *ppt) PURE; // use IFolderView::SelectAndPositionItems
    STDMETHOD(IsBkDropTarget) (THIS_ _In_opt_ IDropTarget *pDropTarget) PURE; // returns S_OK if drag-and-drop is on the background, S_FALSE otherwise
    STDMETHOD(SetClipboard) (THIS_ BOOL bMove) PURE; // if bMove is TRUE, this attempts to cut (edit.cut, ctrl-x) the current selection.  bMove of FALSE is not supported.
    STDMETHOD(SetPoints) (THIS_ _In_ IDataObject *pDataObject) PURE; // copies points of current selection in to data object.  Call is not needed if drag operation was originated by the IShellView.
    STDMETHOD(GetItemSpacing) (THIS_ _Out_ ITEMSPACING *pSpacing) PURE; // use IFolderView::GetSpacing instead.  GetItemSpacing returns the spacing for small and large view modes only, returning S_OK if the current view mode is is positionable, S_FALSE otherwise.
    STDMETHOD(SetCallback) (THIS_ _In_opt_ IShellFolderViewCB* pNewCB, _Outptr_result_maybenull_ IShellFolderViewCB** ppOldCB) PURE; // replace the IShellFolderViewCB that the IShellView uses
    STDMETHOD(Select) ( THIS_  UINT dwFlags ) PURE; // SFVS_ select flags: select all, select none, invert selection
    STDMETHOD(QuerySupport) (THIS_ _Inout_ UINT * pdwSupport ) PURE; // does nothing, returns S_OK.
    STDMETHOD(SetAutomationObject)(THIS_ _In_opt_ IDispatch* pdisp) PURE; // replaces the IShellView's internal automation object.
};

// SHCreateShellFolderView struct
typedef struct _SFV_CREATE
{
    UINT            cbSize;     // must be sizeof(SFV_CREATE)
    IShellFolder*   pshf;       // IShellFolder the IShellView will use
    IShellView*     psvOuter;   // optional: IShellView to pass to psfvcb
    IShellFolderViewCB* psfvcb; // No callback if NULL
} SFV_CREATE;

SHSTDAPI SHCreateShellFolderView(_In_ const SFV_CREATE* pcsfv, _Outptr_ IShellView ** ppsv);
typedef HRESULT (CALLBACK * LPFNDFMCALLBACK)(_In_opt_ IShellFolder *psf, _In_opt_ HWND hwnd,
                                             _In_opt_ IDataObject *pdtobj, UINT uMsg, WPARAM wParam, LPARAM lParam);

SHSTDAPI CDefFolderMenu_Create2(_In_opt_ PCIDLIST_ABSOLUTE pidlFolder, _In_opt_ HWND hwnd,
                                UINT cidl, _In_reads_opt_(cidl) PCUITEMID_CHILD_ARRAY apidl,
                                _In_opt_ IShellFolder *psf, _In_opt_ LPFNDFMCALLBACK pfn,
                                UINT nKeys, _In_reads_opt_(nKeys) const HKEY *ahkeys,
                                _Outptr_ IContextMenu **ppcm);
typedef struct {
    HWND hwnd;
    IContextMenuCB *pcmcb;          // optional: callback object
    PCIDLIST_ABSOLUTE pidlFolder;   // optional: IDList to folder of the items, computed from psf if NULL
    IShellFolder *psf;              // folder of the items
    UINT cidl;                      // # of items in apidl
    PCUITEMID_CHILD_ARRAY apidl;    // items operating on, used to get IDataObject and IAssociationArray
    IUnknown *punkAssociationInfo;  // optional: IQueryAssociations, specifies where to load extensions from, computed from apidl if NULL
    UINT cKeys;                     // # of items in aKeys, may be zero
    const HKEY *aKeys;              // optional: specifies where to load extensions from
} DEFCONTEXTMENU;

// creates object that implements IContextMenu/IContextMenu2/IContextMenu3, typically
// used in the implemetnation of ::GetUIObjectOf()

#if (NTDDI_VERSION >= NTDDI_VISTA)
SHSTDAPI SHCreateDefaultContextMenu(_In_ const DEFCONTEXTMENU *pdcm, _In_ REFIID riid, _Outptr_ void **ppv);
#endif // NTDDI_VISTA

// structure for lParam of DFM_INFOKECOMMANDEX
typedef struct
{
    DWORD  cbSize;
    DWORD  fMask;   // CMIC_MASK_ values for the invoke
    LPARAM lParam;  // same as lParam of DFM_INFOKECOMMAND
    UINT idCmdFirst;
    UINT idDefMax;
    LPCMINVOKECOMMANDINFO pici; // the whole thing so you can re-invoke on a child
#if (NTDDI_VERSION >= NTDDI_VISTA)
    IUnknown *punkSite;         // site pointer for context menu handler
#endif
} DFMICS, *PDFMICS;

// Note on context menus ranges:
//  Standard Items // DFM_MERGECONTEXTMENU, context menu extensions, DFM_MERGECONTEXTMENU_TOP
//  Separator
//  View Items   // context menu extensions can get here
//  Separator
//  (defcm S_FALSE "default" items, if applicable)
//  Separator
//  Folder Items // context menu extensions can get here
//  Separator
//  Bottom Items // DFM_MERGECONTEXTMENU_BOTTOM

//                                  uMsg       wParam       lParam
#define DFM_MERGECONTEXTMENU         1      // uFlags       LPQCMINFO
#define DFM_INVOKECOMMAND            2      // idCmd        pszArgs
#define DFM_GETHELPTEXT              5      // idCmd,cchMax pszText -Ansi
#define DFM_WM_MEASUREITEM           6      // ---from the message---
#define DFM_WM_DRAWITEM              7      // ---from the message---
#define DFM_WM_INITMENUPOPUP         8      // ---from the message---
#define DFM_VALIDATECMD              9      // idCmd        0
#define DFM_MERGECONTEXTMENU_TOP     10     // uFlags       LPQCMINFO
#define DFM_GETHELPTEXTW             11     // idCmd,cchMax pszText -Unicode
#define DFM_INVOKECOMMANDEX          12     // idCmd        PDFMICS
#define DFM_MAPCOMMANDNAME           13     // idCmd *      pszCommandName
#define DFM_GETDEFSTATICID           14     // idCmd *      0
#define DFM_GETVERBW                 15     // idCmd,cchMax pszText -Unicode
#define DFM_GETVERBA                 16     // idCmd,cchMax pszText -Ansi
#define DFM_MERGECONTEXTMENU_BOTTOM  17     // uFlags       LPQCMINFO
#define DFM_MODIFYQCMFLAGS           18     // uFlags       UINT *puNewFlags;   modify the CFM_XXX values passed to IContextMenu::QueryContextMenu

// Commands from DFM_INVOKECOMMAND when strings are passed in
#define DFM_CMD_DELETE          ((UINT)-1)
#define DFM_CMD_MOVE            ((UINT)-2)
#define DFM_CMD_COPY            ((UINT)-3)
#define DFM_CMD_LINK            ((UINT)-4)
#define DFM_CMD_PROPERTIES      ((UINT)-5)
#define DFM_CMD_NEWFOLDER       ((UINT)-6)
#define DFM_CMD_PASTE           ((UINT)-7)
#define DFM_CMD_VIEWLIST        ((UINT)-8)
#define DFM_CMD_VIEWDETAILS     ((UINT)-9)
#define DFM_CMD_PASTELINK       ((UINT)-10)
#define DFM_CMD_PASTESPECIAL    ((UINT)-11)
#define DFM_CMD_MODALPROP       ((UINT)-12)
#define DFM_CMD_RENAME          ((UINT)-13)


typedef HRESULT (CALLBACK * LPFNVIEWCALLBACK)(_In_ IShellView *psvOuter,
                                              _In_ IShellFolder *psf,
                                              _In_ HWND hwndMain,
                                              UINT uMsg,
                                              WPARAM wParam,
                                              LPARAM lParam);

// SHCreateShellFolderViewEx struct
typedef struct _CSFV
{
    UINT              cbSize;
    IShellFolder *    pshf;
    IShellView *      psvOuter;
    PCIDLIST_ABSOLUTE pidl;
    LONG              lEvents;
    LPFNVIEWCALLBACK  pfnCallback;       // No callback if NULL
    FOLDERVIEWMODE    fvm;
} CSFV, * LPCSFV;

#include <poppack.h>        /* Return to byte packing */

SHSTDAPI_(IContextMenu *) SHFind_InitMenuPopup(_In_ HMENU hmenu, _In_opt_ HWND hwndOwner, UINT idCmdFirst, UINT idCmdLast);
SHSTDAPI SHCreateShellFolderViewEx(_In_ CSFV *pcsfv, _Outptr_ IShellView **ppsv);


// Legacy PROPIDs for Internet Shortcuts (FMTID_Intshcut) to be used with
// IPropertySetStorage/IPropertyStorage.
//
// The known property ids and their variant types are:
//      PID_IS_URL          [VT_LPWSTR]   URL
//      PID_IS_NAME         [VT_LPWSTR]   Name of the internet shortcut
//      PID_IS_WORKINGDIR   [VT_LPWSTR]   Working directory for the shortcut
//      PID_IS_HOTKEY       [VT_UI2]      Hotkey for the shortcut
//      PID_IS_SHOWCMD      [VT_I4]       Show command for shortcut
//      PID_IS_ICONINDEX    [VT_I4]       Index into file that has icon
//      PID_IS_ICONFILE     [VT_LPWSTR]   File that has the icon
//      PID_IS_WHATSNEW     [VT_LPWSTR]   What's New text
//      PID_IS_AUTHOR       [VT_LPWSTR]   Author
//      PID_IS_DESCRIPTION  [VT_LPWSTR]   Description text of site
//      PID_IS_COMMENT      [VT_LPWSTR]   User annotated comment

#define PID_IS_URL           2
#define PID_IS_NAME          4
#define PID_IS_WORKINGDIR    5
#define PID_IS_HOTKEY        6
#define PID_IS_SHOWCMD       7
#define PID_IS_ICONINDEX     8
#define PID_IS_ICONFILE      9
#define PID_IS_WHATSNEW      10
#define PID_IS_AUTHOR        11
#define PID_IS_DESCRIPTION   12
#define PID_IS_COMMENT       13
#define PID_IS_ROAMED        15

// PROPIDs for Internet Sites (FMTID_InternetSite) to be used with
// IPropertySetStorage/IPropertyStorage
//
// The known property ids and their variant types are:
//      PID_INTSITE_WHATSNEW     [VT_LPWSTR]   What's New text
//      PID_INTSITE_AUTHOR       [VT_LPWSTR]   Author
//      PID_INTSITE_LASTVISIT    [VT_FILETIME] Time site was last visited
//      PID_INTSITE_LASTMOD      [VT_FILETIME] Time site was last modified
//      PID_INTSITE_VISITCOUNT   [VT_UI4]      Number of times user has visited
//      PID_INTSITE_DESCRIPTION  [VT_LPWSTR]   Description text of site
//      PID_INTSITE_COMMENT      [VT_LPWSTR]   User annotated comment
//      PID_INTSITE_RECURSE      [VT_UI4]      Levels to recurse (0-3)
//      PID_INTSITE_WATCH        [VT_UI4]      PIDISM_ flags
//      PID_INTSITE_SUBSCRIPTION [VT_UI8]      Subscription cookie
//      PID_INTSITE_URL          [VT_LPWSTR]   URL
//      PID_INTSITE_TITLE        [VT_LPWSTR]   Title
//      PID_INTSITE_CODEPAGE     [VT_UI4]      Codepage of the document
//      PID_INTSITE_TRACKING     [VT_UI4]      Tracking
//      PID_INTSITE_ICONINDEX    [VT_I4]       Retrieve the index to the icon
//      PID_INTSITE_ICONFILE     [VT_LPWSTR]   Retrieve the file containing the icon index.
//      PID_INTSITE_RAWURL       [VT_LPWSTR]   The raw, un-encoded, unicode url.
//      PID_INTSITE_ROAMED       [VT_UI4]      Indicates that this entry was roamed from a different machine


#define PID_INTSITE_WHATSNEW      2
#define PID_INTSITE_AUTHOR        3
#define PID_INTSITE_LASTVISIT     4
#define PID_INTSITE_LASTMOD       5
#define PID_INTSITE_VISITCOUNT    6
#define PID_INTSITE_DESCRIPTION   7
#define PID_INTSITE_COMMENT       8
#define PID_INTSITE_FLAGS         9
#define PID_INTSITE_CONTENTLEN    10
#define PID_INTSITE_CONTENTCODE   11
#define PID_INTSITE_RECURSE       12
#define PID_INTSITE_WATCH         13
#define PID_INTSITE_SUBSCRIPTION  14
#define PID_INTSITE_URL           15
#define PID_INTSITE_TITLE         16
#define PID_INTSITE_CODEPAGE      18
#define PID_INTSITE_TRACKING      19
#define PID_INTSITE_ICONINDEX     20
#define PID_INTSITE_ICONFILE      21
#define PID_INTSITE_ROAMED        34

// Flags for PID_IS_FLAGS
#define PIDISF_RECENTLYCHANGED  0x00000001
#define PIDISF_CACHEDSTICKY     0x00000002
#define PIDISF_CACHEIMAGES      0x00000010
#define PIDISF_FOLLOWALLLINKS   0x00000020


// Values for PID_INTSITE_WATCH
#define PIDISM_GLOBAL           0       // Monitor based on global setting
#define PIDISM_WATCH            1       // User says watch
#define PIDISM_DONTWATCH        2       // User says don't watch

// Values for PID_INTSITE_ROAMED
#define PIDISR_UP_TO_DATE       0 // No action needed
#define PIDISR_NEEDS_ADD        1 // The entry was added due to roaming
#define PIDISR_NEEDS_UPDATE     2 // The entry was roamed and contains updated information
#define PIDISR_NEEDS_DELETE     3 // The entry was roamed and should be deleted

////////////////////////////////////////////////////////////////////
//
// The shell keeps track of some per-user state to handle display
// options that is of major interest to ISVs.
// The key one requested right now is "DoubleClickInWebView".
typedef struct {
    BOOL fShowAllObjects : 1;
    BOOL fShowExtensions : 1;
    BOOL fNoConfirmRecycle : 1;

    BOOL fShowSysFiles : 1;
    BOOL fShowCompColor : 1;
    BOOL fDoubleClickInWebView : 1;
    BOOL fDesktopHTML : 1;
    BOOL fWin95Classic : 1;
    BOOL fDontPrettyPath : 1;
    BOOL fShowAttribCol : 1; // No longer used, dead bit
    BOOL fMapNetDrvBtn : 1;
    BOOL fShowInfoTip : 1;
    BOOL fHideIcons : 1;
    BOOL fWebView : 1;
    BOOL fFilter : 1;
    BOOL fShowSuperHidden : 1;
    BOOL fNoNetCrawling : 1;

    DWORD dwWin95Unused; // Win95 only - no longer supported pszHiddenFileExts
    UINT  uWin95Unused; // Win95 only - no longer supported cbHiddenFileExts

    // Note: Not a typo!  This is a persisted structure so we cannot use LPARAM
    LONG   lParamSort;
    int    iSortDirection;

    UINT   version;

    // new for win2k. need notUsed var to calc the right size of ie4 struct
    // FIELD_OFFSET does not work on bit fields
    UINT uNotUsed; // feel free to rename and use
    BOOL fSepProcess: 1;

    // new for Whistler.
    BOOL fStartPanelOn: 1;       //Indicates if the Whistler StartPanel mode is ON or OFF.
    BOOL fShowStartPage: 1;      //Indicates if the Whistler StartPage on desktop is ON or OFF.

    // new for Windows Vista
    BOOL fAutoCheckSelect: 1;
    BOOL fIconsOnly: 1;
    BOOL fShowTypeOverlay: 1;

    // new for Windows 8
    BOOL fShowStatusBar : 1;

    UINT fSpareFlags : 9;
} SHELLSTATEA, *LPSHELLSTATEA;

typedef struct {
    BOOL fShowAllObjects : 1;
    BOOL fShowExtensions : 1;
    BOOL fNoConfirmRecycle : 1;
    BOOL fShowSysFiles : 1;
    BOOL fShowCompColor : 1;
    BOOL fDoubleClickInWebView : 1;
    BOOL fDesktopHTML : 1;
    BOOL fWin95Classic : 1;
    BOOL fDontPrettyPath : 1;
    BOOL fShowAttribCol : 1; // No longer used, dead bit
    BOOL fMapNetDrvBtn : 1;
    BOOL fShowInfoTip : 1;
    BOOL fHideIcons : 1;
    BOOL fWebView : 1;
    BOOL fFilter : 1;
    BOOL fShowSuperHidden : 1;
    BOOL fNoNetCrawling : 1;

    DWORD dwWin95Unused; // Win95 only - no longer supported pszHiddenFileExts
    UINT  uWin95Unused; // Win95 only - no longer supported cbHiddenFileExts

    // Note: Not a typo!  This is a persisted structure so we cannot use LPARAM
    LONG   lParamSort;
    int    iSortDirection;
    UINT   version;

    // new for win2k. need notUsed var to calc the right size of ie4 struct
    // FIELD_OFFSET does not work on bit fields
    UINT uNotUsed; // feel free to rename and use
    BOOL fSepProcess: 1;

    // new for Whistler.
    BOOL fStartPanelOn: 1;       //Indicates if the Whistler StartPanel mode is ON or OFF.
    BOOL fShowStartPage: 1;      //Indicates if the Whistler StartPage on desktop is ON or OFF.

    // new for Windows Vista
    BOOL fAutoCheckSelect: 1;
    BOOL fIconsOnly: 1;
    BOOL fShowTypeOverlay: 1;

    // new for Windows 8
    BOOL fShowStatusBar : 1;

    UINT fSpareFlags : 9;
} SHELLSTATEW, *LPSHELLSTATEW;

#define SHELLSTATEVERSION_IE4   9
#define SHELLSTATEVERSION_WIN2K 10

#ifdef UNICODE
#define SHELLSTATE   SHELLSTATEW
#define LPSHELLSTATE LPSHELLSTATEW
#else
#define SHELLSTATE   SHELLSTATEA
#define LPSHELLSTATE LPSHELLSTATEA
#endif

#define SHELLSTATE_SIZE_WIN95 FIELD_OFFSET(SHELLSTATE,lParamSort)
#define SHELLSTATE_SIZE_NT4   FIELD_OFFSET(SHELLSTATE,version)
#define SHELLSTATE_SIZE_IE4   FIELD_OFFSET(SHELLSTATE,uNotUsed)
#define SHELLSTATE_SIZE_WIN2K sizeof(SHELLSTATE)

SHSTDAPI_(void) SHGetSetSettings(_Inout_opt_ LPSHELLSTATE lpss, DWORD dwMask, BOOL bSet);

//
//  SysFiles are these windows special files:
//      "dll sys vxd 386 drv"
//
//  hidden files are files with the FILE_ATTRIBUTE_HIDDEN attribute
//
//  system files are files with the FILE_ATTRIBUTE_SYSTEM attribute
//
//      fShowAllObjects fShowSysFiles   Result
//      --------------- -------------   ------
//      0               0               hide hidden + SysFiles + system files
//      0               1               hide hidden files.
//      1               0               show all files.
//      1               1               show all files.
//
typedef struct {
    BOOL fShowAllObjects : 1;
    BOOL fShowExtensions : 1;
    BOOL fNoConfirmRecycle : 1;
    BOOL fShowSysFiles : 1;
    BOOL fShowCompColor : 1;
    BOOL fDoubleClickInWebView : 1;
    BOOL fDesktopHTML : 1;
    BOOL fWin95Classic : 1;
    BOOL fDontPrettyPath : 1;
    BOOL fShowAttribCol : 1;
    BOOL fMapNetDrvBtn : 1;
    BOOL fShowInfoTip : 1;
    BOOL fHideIcons : 1;
#if (NTDDI_VERSION >= NTDDI_VISTA)
    BOOL fAutoCheckSelect: 1;
    BOOL fIconsOnly: 1;
    UINT fRestFlags : 1; // when adding additional flags keep SHELLSTATE and SHGetSettings in sync.
#else
    UINT fRestFlags : 3; // when adding additional flags keep SHELLSTATE and SHGetSettings in sync.
#endif
} SHELLFLAGSTATE, *LPSHELLFLAGSTATE;

#define SSF_SHOWALLOBJECTS          0x00000001
#define SSF_SHOWEXTENSIONS          0x00000002
#define SSF_HIDDENFILEEXTS          0x00000004
#define SSF_SERVERADMINUI           0x00000004
#define SSF_SHOWCOMPCOLOR           0x00000008
#define SSF_SORTCOLUMNS             0x00000010
#define SSF_SHOWSYSFILES            0x00000020
#define SSF_DOUBLECLICKINWEBVIEW    0x00000080
#define SSF_SHOWATTRIBCOL           0x00000100
#define SSF_DESKTOPHTML             0x00000200
#define SSF_WIN95CLASSIC            0x00000400
#define SSF_DONTPRETTYPATH          0x00000800
#define SSF_SHOWINFOTIP             0x00002000
#define SSF_MAPNETDRVBUTTON         0x00001000
#define SSF_NOCONFIRMRECYCLE        0x00008000
#define SSF_HIDEICONS               0x00004000
#define SSF_FILTER                  0x00010000
#define SSF_WEBVIEW                 0x00020000
#define SSF_SHOWSUPERHIDDEN         0x00040000
#define SSF_SEPPROCESS              0x00080000
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define SSF_NONETCRAWLING           0x00100000
#define SSF_STARTPANELON            0x00200000
#define SSF_SHOWSTARTPAGE           0x00400000
#endif  // NTDDI_WINXP
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define SSF_AUTOCHECKSELECT         0x00800000
#define SSF_ICONSONLY               0x01000000
#define SSF_SHOWTYPEOVERLAY         0x02000000
#endif  // NTDDI_VISTA
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define SSF_SHOWSTATUSBAR           0x04000000
#endif  // NTDDI_VISTA

//
// Specify the bits you are interested in in dwMask and they will be
// filled out in the lpss structure.
//
// When these settings change, a WM_SETTINGCHANGE message is sent
// with the string lParam value of "ShellState".
//
SHSTDAPI_(void) SHGetSettings(_Out_ SHELLFLAGSTATE *psfs, DWORD dwMask);

//
// Given a pidl, you can get an interface pointer (as specified by riid) of the pidl's parent folder (in ppv)
// If ppidlLast is non-NULL, you can also get the pidl of the last item.
//
SHSTDAPI SHBindToParent(_In_ PCIDLIST_ABSOLUTE pidl, _In_ REFIID riid, _Outptr_ void **ppv, _Outptr_opt_ PCUITEMID_CHILD *ppidlLast);

#if (NTDDI_VERSION >= NTDDI_VISTA)

// Same as SHBindToParent, except you also specify which root to use.
SHSTDAPI SHBindToFolderIDListParent(_In_opt_ IShellFolder *psfRoot, _In_ PCUIDLIST_RELATIVE pidl, _In_ REFIID riid, _Outptr_ void **ppv, _Outptr_opt_ PCUITEMID_CHILD *ppidlLast);

// same as SHBindToFolderIDListParent but with a IBindCtx *pbc;
SHSTDAPI SHBindToFolderIDListParentEx(_In_opt_ IShellFolder *psfRoot, _In_ PCUIDLIST_RELATIVE pidl, _In_opt_ IBindCtx *ppbc, _In_ REFIID riid, _Outptr_ void **ppv, _Outptr_opt_ PCUITEMID_CHILD *ppidlLast);

// helper function that gets the desktop object, then calls BindToObject on that
SHSTDAPI SHBindToObject(_In_opt_ IShellFolder *psf, _In_ PCUIDLIST_RELATIVE pidl, _In_opt_ IBindCtx *pbc, _In_ REFIID riid, _Outptr_ void **ppv);

#endif // NTDDI_VISTA

// This function is used to validate that the container structure of an IDList is valid.
// This should be used by all code that reads an IDList from a persistence format to ensure
// that invalid forms do not lead to a security exploit in the code that interprets the IDList.
// Shell data sources are responsible for validating the private parts of the ITEMIDs.
// Hidden data is validated by the functions that interpret that data.
__inline BOOL IDListContainerIsConsistent(_In_reads_bytes_(cbAlloc) PCUIDLIST_RELATIVE pidl, _In_ UINT cbAlloc)
{
    //  test to make sure that the pidl does not overrun itself
    //  this is for callers that un-persist pidl data, and
    //  assumes that the caller knows the allocated size of the pidl
    //  similar to ILGetSize(pidl) <= cbAlloc except that
    //  it doesnt assert or throw exceptions
    UINT cbPidl = sizeof(pidl->mkid.cb);
    while (cbPidl <= cbAlloc &&                         // can read pidl->mkid.cb
           pidl->mkid.cb >= sizeof(pidl->mkid.cb) &&    // not end of pidl and >= 2
           pidl->mkid.cb <= cbAlloc - cbPidl)           // doesn't go past end of buffer
    {
        cbPidl += pidl->mkid.cb;
        pidl = ILNext(pidl);
    }
    return cbPidl <= cbAlloc && 0 == pidl->mkid.cb;
}

//
//  given a string it will call psfDesktop->ParseDisplayName() to try and create a pidl
//  if no pbc specified, it uses the preferred options for parsing.
//  this includes mapping file system paths to their appropriate aliased location (RegisterObjectParam(STR_PARSE_TRANSLATE_ALIASES))
//  psfgaoOut is optional for SFGAO attributes
//
#if (NTDDI_VERSION >= NTDDI_WINXP)
SHSTDAPI SHParseDisplayName(_In_ PCWSTR pszName, _In_opt_ IBindCtx *pbc, _Outptr_ PIDLIST_ABSOLUTE *ppidl, _In_ SFGAOF sfgaoIn, _Out_opt_ SFGAOF *psfgaoOut);
#endif // NTDDI_WINXP

//
// This API will make its best effort to prepare the path for the caller.  This includes:
// 1. Prompting for the ejectable media to be re-inserted. (Floppy, CD-ROM, ZIP drive, etc.)
// 2. Prompting for the media to be formatted. (Floppy, hard drive, etc.)
// 3. Remount mapped drives if the connection was lost. (\\unc\share mapped to N: becomes disconnected)
// 4. If the path doesn't exist, create it.  (SHPPFW_DIRCREATE and SHPPFW_ASKDIRCREATE)
// 5. Display an error if the media is read only. (SHPPFW_NOWRITECHECK not set)
//
// PARAMETERS:
//      hwnd: Parernt window for UI.  NULL means don't display UI. OPTIONAL
//      punkEnableModless: Parent that will be set to modal during UI using IOleInPlaceActiveObject::EnableModeless(). OPTIONAL
//      pszPath: Path to verify is valid for writting.  This can be a UNC or file drive path.  The path
//               should only contain directories.  Pass SHPPFW_IGNOREFILENAME if the last path segment
//               is always filename to ignore.
//      dwFlags: SHPPFW_* Flags to modify behavior

#define SHPPFW_NONE             0x00000000
#define SHPPFW_DEFAULT          SHPPFW_DIRCREATE        // May change
#define SHPPFW_DIRCREATE        0x00000001              // Create the directory if it doesn't exist without asking the user.
#define SHPPFW_ASKDIRCREATE     0x00000002              // Create the directory if it doesn't exist after asking the user.
#define SHPPFW_IGNOREFILENAME   0x00000004              // Ignore the last item in pszPath because it's a file.  Example: pszPath="C:\DirA\DirB", only use "C:\DirA".
#define SHPPFW_NOWRITECHECK     0x00000008              // Caller only needs to read from the drive, so don't check if it's READ ONLY.
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define SHPPFW_MEDIACHECKONLY   0x00000010              // do the retrys on the media (or net path), return errors if the file can't be found
#endif // NTDDI_WINXP


SHSTDAPI SHPathPrepareForWriteA(_In_opt_ HWND hwnd, _In_opt_ IUnknown *punkEnableModless, _In_ LPCSTR pszPath, DWORD dwFlags);
SHSTDAPI SHPathPrepareForWriteW(_In_opt_ HWND hwnd, _In_opt_ IUnknown *punkEnableModless, _In_ LPCWSTR pszPath, DWORD dwFlags);
#ifdef UNICODE
#define SHPathPrepareForWrite  SHPathPrepareForWriteW
#else
#define SHPathPrepareForWrite  SHPathPrepareForWriteA
#endif // !UNICODE

//--------------------------------------------------------------------------
//
// Interface used for exposing the INI file methods on a shortcut file
//
//
//--------------------------------------------------------------------------
#undef  INTERFACE
#define INTERFACE  INamedPropertyBag
DECLARE_INTERFACE_IID_(INamedPropertyBag, IUnknown, "FB700430-952C-11d1-946F-000000000000")
{
    STDMETHOD(ReadPropertyNPB) (THIS_ _In_ PCWSTR pszBagname,
                                _In_ PCWSTR pszPropName,
                                _Inout_ PROPVARIANT *pVar) PURE;

    STDMETHOD(WritePropertyNPB)(THIS_ _In_ PCWSTR pszBagname,
                                _In_ PCWSTR pszPropName,
                                _In_ PROPVARIANT  *pVar) PURE;

    STDMETHOD(RemovePropertyNPB)(THIS_ _In_ PCWSTR pszBagname,
                                 _In_ PCWSTR pszPropName) PURE;
};

//  SHPropStgCreate()
//  Wrap of IPropertySetStorage::Open/Create
//
//  This function ensures proper handling of code page retrieval/assignment
//  for the requested property set operation.
//
//  psstg,          //  Address of IPropertySetStorage vtable
//  fmtid,          //  property set ID
//  pclsid,         //  class ID associated with the set. This can be NULL
//  grfFlags,       //  PROPSETFLAG_xxx.  All sets containing ansi bytes should be created with
                    //  PROPSETFLAG_ANSI, otherwise PROPSETFLAG_DEFAULT.
//  grfMode,        //  STGM_ flags.  Must contain STGM_DIRECT|STGM_EXCLUSIVE.
//  dwDisposition,  //  OPEN_EXISTING. OPEN_ALWAYS, CREATE_NEW, or CREATE_ALWAYS
//  IPropertyStorage** ppstg,  // Address to receive requested vtable
//  puCodePage      //  Optional address to receive the code page ID for the set.
//
SHSTDAPI SHPropStgCreate(_In_ IPropertySetStorage* psstg, _In_ REFFMTID fmtid, _In_opt_ const CLSID *pclsid, DWORD grfFlags, DWORD grfMode, DWORD dwDisposition, _Outptr_ IPropertyStorage** ppstg, _Out_opt_ UINT* puCodePage);


//  SHPropStgReadMultiple()
//  IPropertyStorage::ReadMultiple wrap
//
//  The wrap ensures ANSI/UNICODE translations are handled properly for
//  legacy property sets.
//
//  pps,       // address of IPropertyStorage vtable.
//  uCodePage, //Code page value retrieved from SHCreatePropertySet
//  cpspec,    //Count of properties being read
//  rgpspec,   //Array of the properties to be read
//  rgvar      //Array of PROPVARIANTs containing the property values on return
//
SHSTDAPI SHPropStgReadMultiple(_In_ IPropertyStorage* pps, UINT uCodePage, ULONG cpspec, _In_reads_(cpspec) PROPSPEC const rgpspec[], _Out_writes_all_(cpspec) PROPVARIANT rgvar[] );


//  SHPropStgWriteMultiple()
//  IPropertyStorage::WriteMultiple wrap
//
//  The wrap ensures ANSI/UNICODE translations are handled properly for
//  legacy property sets.
//
//  pps,       // address of IPropertyStorage vtable.
//  uCodePage, // code page retrieved from SHCreatePropertySet.
//  cpspec,    // The number of properties being set
//  rgpspec,   // Property specifiers
//  rgvar,     // Array of PROPVARIANT values
//  propidNameFirst // Minimum value for property identifiers. This value should be >= PID_FIRST_USABLE
//
SHSTDAPI SHPropStgWriteMultiple(_In_ IPropertyStorage* pps, _Inout_opt_ UINT* puCodePage, ULONG cpspec, _In_reads_(cpspec) PROPSPEC const rgpspec[], _Inout_updates_(cpspec) PROPVARIANT rgvar[], PROPID propidNameFirst );

#if (NTDDI_VERSION >= NTDDI_WINXP)
SHSTDAPI SHCreateFileExtractIconA(_In_ LPCSTR pszFile, _In_ DWORD dwFileAttributes, _In_ REFIID riid, _Outptr_ void **ppv);
SHSTDAPI SHCreateFileExtractIconW(_In_ LPCWSTR pszFile, _In_ DWORD dwFileAttributes, _In_ REFIID riid, _Outptr_ void **ppv);
#ifdef UNICODE
#define SHCreateFileExtractIcon  SHCreateFileExtractIconW
#else
#define SHCreateFileExtractIcon  SHCreateFileExtractIconA
#endif // !UNICODE

SHSTDAPI SHLimitInputEdit(_In_ HWND hwndEdit, _In_ IShellFolder *psf);

STDAPI SHGetAttributesFromDataObject(_In_opt_ IDataObject *pdo, DWORD dwAttributeMask, _Out_opt_ DWORD *pdwAttributes, _Out_opt_ UINT *pcItems);

#endif  // NTDDI_WINXP

// A usefull function in Defview for mapping idlist into index into system
// image list.  Optionally it can also look up the index of the selected
// icon.
SHSTDAPI_(int) SHMapPIDLToSystemImageListIndex(_In_ IShellFolder *pshf, _In_ PCUITEMID_CHILD pidl, _Out_opt_ int *piIndexSel);

SHSTDAPI SHCLSIDFromString(_In_ PCWSTR psz, _Out_ CLSID *pclsid);
SHSTDAPI_(int) PickIconDlg(_In_opt_ HWND hwnd, _Inout_updates_(cchIconPath) PWSTR pszIconPath, UINT cchIconPath, _Inout_opt_ int *piIconIndex);


#if (NTDDI_VERSION >= NTDDI_WIN7)
// returns an IStream or IStorage via riid/ppv
STDAPI StgMakeUniqueName(_In_ IStorage *pstgParent, _In_ PCWSTR pszFileSpec, _In_ DWORD grfMode, _In_ REFIID riid, _Outptr_ void **ppv);
#endif // NTDDI_WIN7

#if (_WIN32_IE >= _WIN32_IE_IE70)
typedef enum tagIESHORTCUTFLAGS
{
    IESHORTCUT_NEWBROWSER    = 0x01,
    IESHORTCUT_OPENNEWTAB    = 0x02,
    IESHORTCUT_FORCENAVIGATE = 0x04,
    IESHORTCUT_BACKGROUNDTAB = 0x08,
} IESHORTCUTFLAGS;
#endif // _WIN32_IE_IE70

#include <poppack.h>

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
