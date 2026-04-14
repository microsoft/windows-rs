//
//    Copyright (C) Microsoft.  All rights reserved.
//
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//---------------------------------------------------------------------------//
// CLSIDs exposed for the dsclient.
//---------------------------------------------------------------------------//

// this CLSID is used to signal that the DSOBJECTNAMEs structure originated
// for the Microsoft DS.

DEFINE_GUID(CLSID_MicrosoftDS, 0xfe1290f0, 0xcfbd, 0x11cf, 0xa3, 0x30, 0x0, 0xaa, 0x0, 0xc1, 0x6e, 0x65);
#define CLSID_DsFolder CLSID_MicrosoftDS


// this is the CLSID used by clients to get the IShellExtInit, IPropSheetExt
// and IContextMenus exposed from dsuiext.dll.

DEFINE_GUID(CLSID_DsPropertyPages, 0xd45d530,  0x764b, 0x11d0, 0xa1, 0xca, 0x0, 0xaa, 0x0, 0xc1, 0x6e, 0x65);

DEFINE_GUID(CLSID_DsDomainTreeBrowser, 0x1698790a, 0xe2b4, 0x11d0, 0xb0, 0xb1, 0x00, 0xc0, 0x4f, 0xd8, 0xdc, 0xa6);
DEFINE_GUID(IID_IDsBrowseDomainTree, 0x7cabcf1e, 0x78f5, 0x11d2, 0x96, 0xc, 0x0, 0xc0, 0x4f, 0xa3, 0x1a, 0x86);

DEFINE_GUID(CLSID_DsDisplaySpecifier, 0x1ab4a8c0, 0x6a0b, 0x11d2, 0xad, 0x49, 0x0, 0xc0, 0x4f, 0xa3, 0x1a, 0x86);
#define IID_IDsDisplaySpecifier CLSID_DsDisplaySpecifier

DEFINE_GUID(CLSID_DsFolderProperties, 0x9e51e0d0, 0x6e0f, 0x11d2, 0x96, 0x1, 0x0, 0xc0, 0x4f, 0xa3, 0x1a, 0x86);

#ifndef GUID_DEFS_ONLY

#include "activeds.h"

//---------------------------------------------------------------------------//
// Clipboard formats used within DSUI
//---------------------------------------------------------------------------//

//
// CF_DSOBJECTS
// ------------
//  This clipboard format defines the seleciton for an DS IShellFolder to the
//  shell extensions.   All strings are stored as BSTR's, and an offset == 0 
//  is used to indicate that the string is not present.
// 

#define DSOBJECT_ISCONTAINER            0x00000001  // = 1 => object is a container
#define DSOBJECT_READONLYPAGES          0x80000000  // = 1 => read only pages

#define DSPROVIDER_UNUSED_0             0x00000001
#define DSPROVIDER_UNUSED_1             0x00000002
#define DSPROVIDER_UNUSED_2             0x00000004
#define DSPROVIDER_UNUSED_3             0x00000008
#define DSPROVIDER_ADVANCED             0x00000010  // = 1 => advanced mode 
#define DSPROVIDER_AD_LDS               0x00000020  // = 1 => talking to an AD LDS instance 

#define CFSTR_DSOBJECTNAMES TEXT("DsObjectNames")

typedef struct
{
    DWORD   dwFlags;                    // item flags
    DWORD   dwProviderFlags;            // flags for item provider
    DWORD   offsetName;                 // offset to ADS path of the object
    DWORD   offsetClass;                // offset to object class name / == 0 not known
} DSOBJECT, * LPDSOBJECT;

typedef struct
{
    CLSID    clsidNamespace;            // namespace identifier (indicates which namespace selection from)
    UINT     cItems;                    // number of objects
    DSOBJECT aObjects[1];               // array of objects
} DSOBJECTNAMES, * LPDSOBJECTNAMES;


//
// CF_DSDISPLAYSPECOPTIONS
// -----------------------
//  When invoking an object referenced by a display specifier (context menu, property
//  page, etc) we call the IShellExtInit interface passing a IDataObject.  This data
//  object supports the CF_DSDISPLAYSPECOPTIONS format to give out configuration
//  informaiton about admin/shell invocation.
//
//  When interacting with dsuiext.dll the interfaces uses this clipboard format
//  to determine which display specifier attributes to address (admin/shell)
//  and pick up the values accordingly.  If no format is suppoted then
//  dsuiext.dll defaults to shell.
// 

#define CFSTR_DS_DISPLAY_SPEC_OPTIONS TEXT("DsDisplaySpecOptions")
#define CFSTR_DSDISPLAYSPECOPTIONS CFSTR_DS_DISPLAY_SPEC_OPTIONS

typedef struct _DSDISPLAYSPECOPTIONS
{
    DWORD   dwSize;                             // size of struct, for versioning
    DWORD   dwFlags;                            // invocation flags
    DWORD   offsetAttribPrefix;                 // offset to attribute prefix string.

    DWORD   offsetUserName;                     // offset to UNICODE user name
    DWORD   offsetPassword;                     // offset to UNICODE password
    DWORD   offsetServer;
    DWORD   offsetServerConfigPath;

} DSDISPLAYSPECOPTIONS, * PDSDISPLAYSPECOPTIONS, * LPDSDISPLAYSPECOPTIONS;

#define DS_PROP_SHELL_PREFIX L"shell"
#define DS_PROP_ADMIN_PREFIX L"admin"

#define DSDSOF_HASUSERANDSERVERINFO     0x00000001      // = 1 => user name/password are valid
#define DSDSOF_SIMPLEAUTHENTICATE       0x00000002      // = 1 => don't use secure authentication to DS
#define DSDSOF_DONTSIGNSEAL             0x00000004      // = 1 => don't sign+seal when opening DS objects
#define DSDSOF_DSAVAILABLE              0x40000000      // = 1 => ignore DS available checks

//
// CF_DSPROPERTYPAGEINFO
// ---------------------
//  When the property pages for an object are being displayed the parsed
//  display specifier string is passed to the page object via the IDataObject
//  in the following clipboard format.
//
//  Within the display specifier for a property page, the format for a
//  Win32 extension is "n,{clsid}[,bla...]" we take the "bla" section and
//  pass it down.
// 

#define CFSTR_DSPROPERTYPAGEINFO TEXT("DsPropPageInfo")

typedef struct
{
    DWORD offsetString;                 // offset to UNICODE string
} DSPROPERTYPAGEINFO, * LPDSPROPERTYPAGEINFO;


// 
// To sync property pages and the admin tools this message is broadcast
//

#define DSPROP_ATTRCHANGED_MSG  TEXT("DsPropAttrChanged")

//---------------------------------------------------------------------------//


//---------------------------------------------------------------------------//
//
// IDsBrowseDomainTree
// ===================
//  This interface returns a list of the domains from a given computer name
//  (or the current computer name if none is specified).
//
//  NOTES:
//    1) The structure returned by ::GetDomains should be free'd using
//       FreeDomains.
//
//    2) ::BrowseTo allocates a string on exit, this is allocated using
//       CoTaskMemAlloc, and therefore should be free'd using CoTaskMemFree.
//
//---------------------------------------------------------------------------//

#define DBDTF_RETURNFQDN          0x00000001  // if not set, pszNCName will be blank
#define DBDTF_RETURNMIXEDDOMAINS  0x00000002  // set it if you want downlevel trust domains too
#define DBDTF_RETURNEXTERNAL      0x00000004  // set it if you want external trust domains too
#define DBDTF_RETURNINBOUND       0x00000008  // set it if you want trusting domains
#define DBDTF_RETURNINOUTBOUND    0x00000010  // set it if you want both trusted and trusting domains

typedef struct _DOMAINDESC
{       
  LPWSTR pszName;                       // domain name (if no dns, use netbios)
  LPWSTR pszPath;                       // set to blank
  LPWSTR pszNCName;                     // FQDN, e.g.,DC=mydomain,DC=microsoft,DC=com
  LPWSTR pszTrustParent;                // parent domain name (if no dns, use netbios)
  LPWSTR pszObjectClass;                // Object class of the domain object referenced
  ULONG  ulFlags;                       // Flags, from DS_TRUSTED_DOMAINS.Flags
  BOOL   fDownLevel;                    // == 1 if downlevel domain
  struct _DOMAINDESC *pdChildList;      // Children of this node
  struct _DOMAINDESC *pdNextSibling;    // Siblings of this node            
} DOMAIN_DESC, DOMAINDESC, * PDOMAIN_DESC, * LPDOMAINDESC;

typedef struct
{
  DWORD dsSize;
  DWORD dwCount;
  DOMAINDESC aDomains[1];
} DOMAIN_TREE, DOMAINTREE, * PDOMAIN_TREE, * LPDOMAINTREE;

#undef  INTERFACE
#define INTERFACE  IDsBrowseDomainTree

DECLARE_INTERFACE_IID_(IDsBrowseDomainTree, IUnknown, "7cabcf1e-78f5-11d2-960c-00c04fa31a86")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;

    // *** IDsBrowseDomainTree methods ***
    STDMETHOD(BrowseTo)(THIS_ HWND hwndParent, _Out_ LPWSTR *ppszTargetPath, _In_ DWORD dwFlags) PURE;
    STDMETHOD(GetDomains)(THIS_ PDOMAIN_TREE *ppDomainTree, DWORD dwFlags) PURE;
    STDMETHOD(FreeDomains)(THIS_ PDOMAIN_TREE *ppDomainTree) PURE;
    STDMETHOD(FlushCachedDomains)(THIS) PURE;
    STDMETHOD(SetComputer)(THIS_ LPCWSTR pszComputerName, LPCWSTR pszUserName, LPCWSTR pszPassword) PURE;
};

//---------------------------------------------------------------------------//


//---------------------------------------------------------------------------//
//
// IDsDisplaySpecifier
// ===================
//  This interface gives client UI access to the display specifiers for 
//  specific attributes.
//
//---------------------------------------------------------------------------//

//
// IDsDisplaySpecifier::SetServer flags
//
#define DSSSF_SIMPLEAUTHENTICATE        0x00000001  // = 1 => don't use secure authentication to DS
#define DSSSF_DONTSIGNSEAL              0x00000002  // = 1 => don't use sign+seal when opening objects in the DS
#define DSSSF_DSAVAILABLE               0x80000000  // = 1 => ignore DS available checks

//
// Flags for IDsDisplaySpecifier::GetIcon / GetIconLocation
//
#define DSGIF_ISNORMAL                  0x0000000   // = icon is in normal state (default)
#define DSGIF_ISOPEN                    0x0000001   // = icon is in open state
#define DSGIF_ISDISABLED                0x0000002   // = icon is in a disabled state
#define DSGIF_ISMASK                    0x000000f
#define DSGIF_GETDEFAULTICON            0x0000010   // = 1 => if no icon then get default (from shell32.dll)
#define DSGIF_DEFAULTISCONTAINER        0x0000020   // = 1 => if returning default icon, return it as a container

//
// Flags for IDsDisplaySpecifier::IsClassContainer
//
#define DSICCF_IGNORETREATASLEAF        0x00000001  // = 1 => igore the "treatAsLeaf" and use only schema information

//
// Callback function used for IDsDisplaySpecifier::EnumClassAttributes
//

#define DSECAF_NOTLISTED               0x00000001  // = 1 => hide from the field drop down in the query UI

typedef HRESULT (CALLBACK *LPDSENUMATTRIBUTES)(LPARAM lParam, LPCWSTR pszAttributeName, LPCWSTR pszDisplayName, DWORD dwFlags);

//
// IDsDisplaySpecifier::GetClassCreationInfo information
//

#define DSCCIF_HASWIZARDDIALOG          0x00000001  // = 1 => return the wizard dialog CLSID
#define DSCCIF_HASWIZARDPRIMARYPAGE     0x00000002  // = 1 => returning a primary wizard dlg CLSID

typedef struct
{
    DWORD dwFlags;
    CLSID clsidWizardDialog;
    CLSID clsidWizardPrimaryPage;
    DWORD cWizardExtensions;            // how many extension CLSIDs?
    CLSID aWizardExtensions[1];
} DSCLASSCREATIONINFO, * LPDSCLASSCREATIONINFO;

//
// IDsDisplaySpecifier - a COM object for interacting with display specifiers
//

#undef  INTERFACE
#define INTERFACE IDsDisplaySpecifier

DECLARE_INTERFACE_IID_(IDsDisplaySpecifier, IUnknown, "1ab4a8c0-6a0b-11d2-ad49-00c04fa31a86")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;

    // *** IDsDisplaySpecifier methods ***
    STDMETHOD(SetServer)(THIS_ LPCWSTR pszServer, LPCWSTR pszUserName, LPCWSTR pszPassword, DWORD dwFlags) PURE;
    STDMETHOD(SetLanguageID)(THIS_ LANGID langid) PURE;
    STDMETHOD(GetDisplaySpecifier)(THIS_ LPCWSTR pszObjectClass, REFIID riid, void **ppv) PURE;
    STDMETHOD(GetIconLocation)(THIS_ LPCWSTR pszObjectClass, _In_ DWORD dwFlags, _Out_writes_(cchBuffer) LPWSTR pszBuffer, _In_ INT cchBuffer, _Out_ INT *presid) PURE;
    STDMETHOD_(HICON, GetIcon)(THIS_ LPCWSTR pszObjectClass, DWORD dwFlags, INT cxIcon, INT cyIcon) PURE;
    STDMETHOD(GetFriendlyClassName)(THIS_ LPCWSTR pszObjectClass, _Out_writes_(cchBuffer) LPWSTR pszBuffer, _In_ INT cchBuffer) PURE;
    STDMETHOD(GetFriendlyAttributeName)(THIS_ LPCWSTR pszObjectClass, _In_ LPCWSTR pszAttributeName, _Out_writes_(cchBuffer) LPWSTR pszBuffer, _In_ UINT cchBuffer) PURE;
    STDMETHOD_(BOOL, IsClassContainer)(THIS_ LPCWSTR pszObjectClass, LPCWSTR pszADsPath, DWORD dwFlags) PURE;
    STDMETHOD(GetClassCreationInfo)(THIS_ LPCWSTR pszObjectClass, LPDSCLASSCREATIONINFO* ppdscci) PURE;
    STDMETHOD(EnumClassAttributes)(THIS_ LPCWSTR pszObjectClass, LPDSENUMATTRIBUTES pcbEnum, LPARAM lParam) PURE;
    STDMETHOD_(ADSTYPE, GetAttributeADsType)(THIS_ LPCWSTR pszAttributeName) PURE;
};


//---------------------------------------------------------------------------//
//
// DsBrowseForContainer
// --------------------
//  Provides a container browser similar to the SHBrowseForFolder, except
//  targetting the DS.
//
// In:
//  pInfo -> DSBROWSEINFO structure
//
// Out:
//  == IDOK/IDCANCEL depending on buttons, -1 if error
//
//---------------------------------------------------------------------------//

typedef struct
{
    DWORD           cbStruct;       // size of structure in bytes
    HWND            hwndOwner;      // dialog owner
    LPCWSTR         pszCaption;     // dialog caption text (can be NULL)
    LPCWSTR         pszTitle;       // displayed above the tree view control (can be NULL)
    LPCWSTR         pszRoot;        // ADS path to root (NULL == root of DS namespace)
    LPWSTR          pszPath;        // [in/out] initial selection & returned path (required)
    ULONG           cchPath;        // size of pszPath buffer in characters
    DWORD           dwFlags;
    BFFCALLBACK     pfnCallback;    // callback function (see SHBrowseForFolder)
    LPARAM          lParam;         // passed to pfnCallback as lpUserData
    DWORD           dwReturnFormat; // ADS_FORMAT_* (default is ADS_FORMAT_X500_NO_SERVER)
    LPCWSTR         pUserName;      // Username and Password to authenticate against DS with  
    LPCWSTR         pPassword;
    LPWSTR          pszObjectClass; // UNICODE string for the object class
    ULONG           cchObjectClass;
} DSBROWSEINFOW, *PDSBROWSEINFOW;

typedef struct
{
    DWORD           cbStruct;
    HWND            hwndOwner;
    LPCSTR          pszCaption;
    LPCSTR          pszTitle;
    LPCWSTR         pszRoot;        // ADS paths are always UNICODE
    LPWSTR          pszPath;        // ditto
    ULONG           cchPath;
    DWORD           dwFlags;
    BFFCALLBACK     pfnCallback;
    LPARAM          lParam;
    DWORD           dwReturnFormat;
    LPCWSTR         pUserName;      // Username and Password to authenticate against DS with  
    LPCWSTR         pPassword;
    LPWSTR          pszObjectClass; // object class of the selected object
    ULONG           cchObjectClass;
} DSBROWSEINFOA, *PDSBROWSEINFOA;

#ifdef UNICODE
#define DSBROWSEINFO   DSBROWSEINFOW
#define PDSBROWSEINFO  PDSBROWSEINFOW
#else
#define DSBROWSEINFO   DSBROWSEINFOA
#define PDSBROWSEINFO  PDSBROWSEINFOA
#endif

// DSBROWSEINFO flags
#define DSBI_NOBUTTONS          0x00000001  // NOT TVS_HASBUTTONS
#define DSBI_NOLINES            0x00000002  // NOT TVS_HASLINES
#define DSBI_NOLINESATROOT      0x00000004  // NOT TVS_LINESATROOT
#define DSBI_CHECKBOXES         0x00000100  // TVS_CHECKBOXES
#define DSBI_NOROOT             0x00010000  // don't include pszRoot in tree (its children become top level nodes)
#define DSBI_INCLUDEHIDDEN      0x00020000  // display hidden objects
#define DSBI_EXPANDONOPEN       0x00040000  // expand to the path specified in pszPath when opening the dialog
#define DSBI_ENTIREDIRECTORY    0x00090000  // browse the entire directory (defaults to having DSBI_NOROOT set)
#define DSBI_RETURN_FORMAT      0x00100000  // dwReturnFormat field is valid
#define DSBI_HASCREDENTIALS     0x00200000  // pUserName & pPassword are valid
#define DSBI_IGNORETREATASLEAF  0x00400000  // ignore the treat as leaf flag when calling IsClassContainer
#define DSBI_SIMPLEAUTHENTICATE 0x00800000  // don't use secure authentication to DS
#define DSBI_RETURNOBJECTCLASS  0x01000000  // return object class of selected object
#define DSBI_DONTSIGNSEAL       0x02000000  // don't sign+seal communication with DS

#define DSB_MAX_DISPLAYNAME_CHARS   64

typedef struct
{
    DWORD           cbStruct;
    LPCWSTR         pszADsPath;     // ADS paths are always Unicode
    LPCWSTR         pszClass;       // ADS properties are always Unicode
    DWORD           dwMask;
    DWORD           dwState;
    DWORD           dwStateMask;
    WCHAR           szDisplayName[DSB_MAX_DISPLAYNAME_CHARS];
    WCHAR           szIconLocation[MAX_PATH];
    INT             iIconResID;
} DSBITEMW, *PDSBITEMW;

typedef struct
{
    DWORD           cbStruct;
    LPCWSTR         pszADsPath;     // ADS paths are always Unicode
    LPCWSTR         pszClass;       // ADS properties are always Unicode
    DWORD           dwMask;
    DWORD           dwState;
    DWORD           dwStateMask;
    CHAR            szDisplayName[DSB_MAX_DISPLAYNAME_CHARS];
    CHAR            szIconLocation[MAX_PATH];
    INT             iIconResID;
} DSBITEMA, *PDSBITEMA;

#ifdef UNICODE
#define DSBITEM     DSBITEMW
#define PDSBITEM    PDSBITEMW
#else
#define DSBITEM     DSBITEMA
#define PDSBITEM    PDSBITEMA
#endif

// DSBITEM mask flags
#define DSBF_STATE              0x00000001
#define DSBF_ICONLOCATION       0x00000002
#define DSBF_DISPLAYNAME        0x00000004

// DSBITEM state flags
#define DSBS_CHECKED            0x00000001
#define DSBS_HIDDEN             0x00000002
#define DSBS_ROOT               0x00000004

//
// this message is sent to the callback to see if it wants to insert or modify 
// the item that is about to be inserted into the view.
//

#define DSBM_QUERYINSERTW       100 // lParam = PDSBITEMW (state, icon & name may be modified). Return TRUE if handled.
#define DSBM_QUERYINSERTA       101 // lParam = PDSBITEMA (state, icon & name may be modified). Return TRUE if handled.

#ifdef UNICODE
#define DSBM_QUERYINSERT DSBM_QUERYINSERTW
#else
#define DSBM_QUERYINSERT DSBM_QUERYINSERTA
#endif

//
// Called before we change the state of the icon (on tree collapse/expand)
//

#define DSBM_CHANGEIMAGESTATE   102 // lParam = adspath.  Return TRUE/FALSE top allow/disallow

//
// The dialog receives a WM_HELP
//

#define DSBM_HELP               103 // lParam == LPHELPINFO structure

//
// The dialog receives a WM_CONTEXTMENU, DSBID_xxx are the control ID's for this
// dialog so that you can display suitable help.
//

#define DSBM_CONTEXTMENU        104 // lParam == window handle to retrieve help for


//
// These are the control IDs for the controls in the dialog.   The callback can use
// these to modify the contents of the dialog as required.
//

#define DSBID_BANNER            256
#define DSBID_CONTAINERLIST     257

//
// API exported for browsing for containers.
//

STDAPI_(int) DsBrowseForContainerW(_Inout_ PDSBROWSEINFOW pInfo);
STDAPI_(int) DsBrowseForContainerA(_Inout_ PDSBROWSEINFOA pInfo);

#ifdef UNICODE
#define DsBrowseForContainer    DsBrowseForContainerW
#else
#define DsBrowseForContainer    DsBrowseForContainerA
#endif


// NOTE: these are here to keep old clients building - remove soon

STDAPI_(HICON) DsGetIcon(DWORD dwFlags, LPCWSTR pszObjectClass, INT cxImage, INT cyImage);
STDAPI DsGetFriendlyClassName(LPCWSTR pszObjectClass, _Out_writes_(cchBuffer) LPWSTR pszBuffer, UINT cchBuffer);


#ifdef  __cplusplus
struct __declspec(uuid("1ab4a8c0-6a0b-11d2-ad49-00c04fa31a86")) IDsDisplaySpecifier;
#endif

#endif  // GUID_DEFS_ONLY

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


