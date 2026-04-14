//
//    Copyright (C) Microsoft.  All rights reserved.
//
#include <winapifamily.h>

#ifndef __cmnquery_h
#define __cmnquery_h

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


DEFINE_GUID(IID_IQueryForm, 0x8cfcee30, 0x39bd, 0x11d0, 0xb8, 0xd1, 0x0, 0xa0, 0x24, 0xab, 0x2d, 0xbb);
DEFINE_GUID(IID_IPersistQuery, 0x1a3114b8, 0xa62e, 0x11d0, 0xa6, 0xc5, 0x0, 0xa0, 0xc9, 0x06, 0xaf, 0x45);

DEFINE_GUID(CLSID_CommonQuery,  0x83bc5ec0, 0x6f2a, 0x11d0, 0xa1, 0xc4, 0x0, 0xaa, 0x00, 0xc1, 0x6e, 0x65);
DEFINE_GUID(IID_ICommonQuery, 0xab50dec0, 0x6f1d, 0x11d0, 0xa1, 0xc4, 0x0, 0xaa, 0x00, 0xc1, 0x6e, 0x65);


#ifndef GUID_DEFS_ONLY

//-----------------------------------------------------------------------------
// IQueryForm
//-----------------------------------------------------------------------------

//
// A query form object is registered under the query handlers CLSID,
// a list is stored in the registry:
//
//  HKCR\CLSID\{CLSID query handler}\Forms
//
// For each form object there are server values which can be defined:
//
//  Flags           = flags for the form object:
//                      QUERYFORM_CHANGESFORMLIST
//                      QUERYFORM_CHANGESOPTFORMLIST
//
//  CLSID           = string containing the CLSID of the InProc server to invoke
//                    to get the IQueryFormObject.
//
//  Forms           = a sub key containing the CLSIDs for the forms registered
//                    by IQueryForm::AddForms (or modified by ::AddPages), if
//                    the flags are 0, then we scan this list looking for a match
//                    for the default form specified.
//

#define QUERYFORM_CHANGESFORMLIST       0x000000001
#define QUERYFORM_CHANGESOPTFORMLIST    0x000000002


//
// Query Forms
// ===========
//  Query forms are registered and have query pages added to them, a form without
//  pages is not displayed.  Each form has a unique CLSID to allow it to be
//  selected by invoking the query dialog.
//

#define CQFF_NOGLOBALPAGES  0x0000001       // = 1 => doesn't have global pages added
#define CQFF_ISOPTIONAL     0x0000002       // = 1 => form is hidden, unless optional forms requested

typedef struct
{
    DWORD   cbStruct;
    DWORD   dwFlags;
    CLSID   clsid;
    HICON   hIcon;
    LPCWSTR pszTitle;
} CQFORM, * LPCQFORM;

typedef HRESULT (CALLBACK *LPCQADDFORMSPROC)(LPARAM lParam, LPCQFORM pForm);


//
// Query Form Pages
// ================
//  When a query form has been registered the caller can then add pages to it,
//  any form can have pages appended.
//


struct _cqpage;
typedef struct _cqpage CQPAGE, * LPCQPAGE;
typedef HRESULT (CALLBACK *LPCQADDPAGESPROC)(LPARAM lParam, REFCLSID clsidForm, LPCQPAGE pPage);
typedef HRESULT (CALLBACK *LPCQPAGEPROC)(LPCQPAGE pPage, HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam);

struct _cqpage
{
    DWORD        cbStruct;
    DWORD        dwFlags;
    LPCQPAGEPROC pPageProc;
    HINSTANCE    hInstance;
    INT          idPageName;
    INT          idPageTemplate;
    DLGPROC      pDlgProc;
    LPARAM       lParam;
};


//
// IQueryForm interfaces
//

#undef  INTERFACE
#define INTERFACE IQueryForm

DECLARE_INTERFACE_IID_(IQueryForm, IUnknown, "8cfcee30-39bd-11d0-b8d1-00a024ab2dbb")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;

    // IQueryForm methods
    STDMETHOD(Initialize)(THIS_ HKEY hkForm) PURE;
    STDMETHOD(AddForms)(THIS_ LPCQADDFORMSPROC pAddFormsProc, LPARAM lParam) PURE;
    STDMETHOD(AddPages)(THIS_ LPCQADDPAGESPROC pAddPagesProc, LPARAM lParam) PURE;
};


//
// Messages for pages
//

#define CQPM_INITIALIZE             0x00000001
#define CQPM_RELEASE                0x00000002
#define CQPM_ENABLE                 0x00000003 // wParam = TRUE/FALSE (enable, disable), lParam = 0
#define CQPM_GETPARAMETERS          0x00000005 // wParam = 0, lParam = -> receives the LocalAlloc
#define CQPM_CLEARFORM              0x00000006 // wParam, lParam = 0
#define CQPM_PERSIST                0x00000007 // wParam = fRead, lParam -> IPersistQuery
#define CQPM_HELP                   0x00000008 // wParam = 0, lParam -> LPHELPINFO
#define CQPM_SETDEFAULTPARAMETERS   0x00000009 // wParam = 0, lParam -> OPENQUERYWINDOW

#define CQPM_HANDLERSPECIFIC        0x10000000

//-----------------------------------------------------------------------------
// IPersistQuery
//-----------------------------------------------------------------------------

// IPersistQuery interface

#undef  INTERFACE
#define INTERFACE IPersistQuery

DECLARE_INTERFACE_IID_(IPersistQuery, IPersist, "1a3114b8-a62e-11d0-a6c5-00a0c906af45")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;

    // IPersist
    STDMETHOD(GetClassID)(THIS_ CLSID* pClassID) PURE;

    // IPersistQuery
    STDMETHOD(WriteString)(THIS_ LPCWSTR pSection, LPCWSTR pValueName, LPCWSTR pValue) PURE;
    STDMETHOD(ReadString)(THIS_ LPCWSTR pSection, LPCWSTR pValueName, _Out_ LPWSTR pBuffer, INT cchBuffer) PURE;
    STDMETHOD(WriteInt)(THIS_ LPCWSTR pSection, LPCWSTR pValueName, INT value) PURE;
    STDMETHOD(ReadInt)(THIS_ LPCWSTR pSection, LPCWSTR pValueName, LPINT pValue) PURE;
    STDMETHOD(WriteStruct)(THIS_ LPCWSTR pSection, LPCWSTR pValueName, LPVOID pStruct, DWORD cbStruct) PURE;
    STDMETHOD(ReadStruct)(THIS_ LPCWSTR pSection, LPCWSTR pValueName, LPVOID pStruct, DWORD cbStruct) PURE;
    STDMETHOD(Clear)(THIS) PURE;
};


//-----------------------------------------------------------------------------
// ICommonQuery
//-----------------------------------------------------------------------------

#define OQWF_OKCANCEL               0x00000001 // = 1 => Provide OK/Cancel buttons
#define OQWF_DEFAULTFORM            0x00000002 // = 1 => clsidDefaultQueryForm is valid
#define OQWF_SINGLESELECT           0x00000004 // = 1 => view to have single selection (depends on viewer)
#define OQWF_LOADQUERY              0x00000008 // = 1 => use the IPersistQuery to load the given query
#define OQWF_REMOVESCOPES           0x00000010 // = 1 => remove scope picker from dialog
#define OQWF_REMOVEFORMS            0x00000020 // = 1 => remove form picker from dialog
#define OQWF_ISSUEONOPEN            0x00000040 // = 1 => issue query on opening the dialog
#define OQWF_SHOWOPTIONAL           0x00000080 // = 1 => list optional forms by default
#define OQWF_SAVEQUERYONOK          0x00000200 // = 1 => use the IPersistQuery to write the query on close
#define OQWF_HIDEMENUS              0x00000400 // = 1 => no menu bar displayed
#define OQWF_HIDESEARCHUI           0x00000800 // = 1 => dialog is filter, therefore start, stop, new search etc

#define OQWF_PARAMISPROPERTYBAG     0x80000000 // = 1 => the form parameters ptr is an IPropertyBag (ppbFormParameters)

typedef struct
{
    DWORD           cbStruct;                   // structure size
    DWORD           dwFlags;                    // flags (OQFW_*)
    CLSID           clsidHandler;               // clsid of handler we are using
    LPVOID          pHandlerParameters;         // handler specific structure for initialization
    CLSID           clsidDefaultForm;           // default form to be selected (if OQF_DEFAULTFORM == 1 )
    IPersistQuery*  pPersistQuery;              // IPersistQuery used for loading queries
    union
    {
        void*         pFormParameters;
        IPropertyBag* ppbFormParameters;
    };
} OPENQUERYWINDOW, * LPOPENQUERYWINDOW;


// ICommonQuery

#undef  INTERFACE
#define INTERFACE ICommonQuery

DECLARE_INTERFACE_IID_(ICommonQuery, IUnknown, "ab50dec0-6f1d-11d0-a1c4-00aa00c16e65")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ _In_ REFIID riid, _Outptr_ void **ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;

    // ICommonQuery methods
    STDMETHOD(OpenQueryWindow)(THIS_ HWND hwndParent, LPOPENQUERYWINDOW pQueryWnd, IDataObject** ppDataObject) PURE;
};



#endif  // GUID_DEFS_ONLY

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
