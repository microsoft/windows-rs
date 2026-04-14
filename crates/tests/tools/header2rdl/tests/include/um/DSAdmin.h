//+--------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1994 - 1999.
//
//  File:       dsadmin.h
//
//  Contents:   DS Admin Object Creation public header
//
//---------------------------------------------------------------------------


//
// CoClass for the Object creation dialog object
//
// {E301A009-F901-11d2-82B9-00C04F68928B}
DEFINE_GUID(CLSID_DsAdminCreateObj, 
    0xe301a009, 0xf901, 0x11d2, 0x82, 0xb9, 0x0, 0xc0, 0x4f, 0x68, 0x92, 0x8b);


//
// Interface GUIDs
//


// {53554A38-F902-11d2-82B9-00C04F68928B}
DEFINE_GUID(IID_IDsAdminCreateObj, 
    0x53554a38, 0xf902, 0x11d2, 0x82, 0xb9, 0x0, 0xc0, 0x4f, 0x68, 0x92, 0x8b);

// {F2573587-E6FC-11d2-82AF-00C04F68928B}
DEFINE_GUID(IID_IDsAdminNewObj, 
    0xf2573587, 0xe6fc, 0x11d2, 0x82, 0xaf, 0x0, 0xc0, 0x4f, 0x68, 0x92, 0x8b);

// {BE2B487E-F904-11d2-82B9-00C04F68928B}
DEFINE_GUID(IID_IDsAdminNewObjPrimarySite, 
0xbe2b487e, 0xf904, 0x11d2, 0x82, 0xb9, 0x0, 0xc0, 0x4f, 0x68, 0x92, 0x8b);


// {6088EAE2-E7BF-11d2-82AF-00C04F68928B}
DEFINE_GUID(IID_IDsAdminNewObjExt, 
    0x6088eae2, 0xe7bf, 0x11d2, 0x82, 0xaf, 0x0, 0xc0, 0x4f, 0x68, 0x92, 0x8b);


// {E4A2B8B3-5A18-11d2-97C1-00A0C9A06D2D}
DEFINE_GUID(IID_IDsAdminNotifyHandler, 
    0xe4a2b8b3, 0x5a18, 0x11d2, 0x97, 0xc1, 0x0, 0xa0, 0xc9, 0xa0, 0x6d, 0x2d);


#ifndef _DSADMIN_H
#define _DSADMIN_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



// ----------------------------------------------------------------------------
// 
// Interface: IDsAdminCreateObj
//  
// Implemented by the object (implemented by the system) CLSID_DsAdminCreateObj
//
// Used by: any client needing to invoke the creation UI
//

  
#undef  INTERFACE
#define INTERFACE   IDsAdminCreateObj

DECLARE_INTERFACE_(IDsAdminCreateObj, IUnknown)
{
  // *** IUnknown methods ***
  STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
  STDMETHOD_(ULONG,AddRef)(THIS)  PURE;
  STDMETHOD_(ULONG,Release)(THIS) PURE;

  // *** IDsAdminCreateObj methods ***
  STDMETHOD(Initialize)(THIS_ /*IN*/ IADsContainer* pADsContainerObj, 
                              /*IN*/ IADs* pADsCopySource,
                              /*IN*/ LPCWSTR lpszClassName) PURE;
  STDMETHOD(CreateModal)(THIS_ /*IN*/ HWND hwndParent, 
                               /*OUT*/ IADs** ppADsObj) PURE;
};






//---------------------------------------------------------------------------
//
// Interface: IDsAdminNewObj
// 
// Implemented by: DS Admin
//
// Used by: creation extension in proc server (both primary and regular)
//

#undef  INTERFACE
#define INTERFACE   IDsAdminNewObj

DECLARE_INTERFACE_(IDsAdminNewObj, IUnknown)
{
  // *** IUnknown methods ***
  STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
  STDMETHOD_(ULONG,AddRef)(THIS)  PURE;
  STDMETHOD_(ULONG,Release)(THIS) PURE;

  // *** IDsAdminNewObj methods ***
  STDMETHOD(SetButtons)(THIS_ /*IN*/ ULONG nCurrIndex, /*IN*/ BOOL bValid) PURE; 
  STDMETHOD(GetPageCounts)(THIS_ /*OUT*/ LONG* pnTotal,
                                 /*OUT*/ LONG* pnStartIndex) PURE; 
};





//---------------------------------------------------------------------------
//
// Interface: IDsAdminNewObjPrimarySite
// 
// Implemented by: DS Admin
//
// Used by: creation extension in proc server (primary only)
//

#undef  INTERFACE
#define INTERFACE   IDsAdminNewObjPrimarySite

DECLARE_INTERFACE_(IDsAdminNewObjPrimarySite, IUnknown)
{
  // *** IUnknown methods ***
  STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
  STDMETHOD_(ULONG,AddRef)(THIS)  PURE;
  STDMETHOD_(ULONG,Release)(THIS) PURE;


  // *** IDsAdminNewObjPrimarySite methods ***
  STDMETHOD(CreateNew)(THIS_ /*IN*/ LPCWSTR pszName) PURE;
  STDMETHOD(Commit)(THIS ) PURE;
};



//
// struct passed to IDsAdminNewObjExt::Initialize()
//
// it contains information regarding UI look
//

typedef struct
{
    DWORD   dwSize;                     // size of struct, for versioning
    HICON   hObjClassIcon;              // class icon for the object to be created
    LPWSTR  lpszWizTitle;               // title of the wizard
    LPWSTR  lpszContDisplayName;        // container display name (canonical name)
} DSA_NEWOBJ_DISPINFO, * LPDSA_NEWOBJ_DISPINFO;




//
// context flags passed to IDsAdminNewObjExt::OnError() and to IDsAdminNewObjExt::WriteData()
//

#define DSA_NEWOBJ_CTX_PRECOMMIT      0x00000001  // before SetInfo()
#define DSA_NEWOBJ_CTX_COMMIT         0x00000002  // SetInfo(), commit phase
#define DSA_NEWOBJ_CTX_POSTCOMMIT     0x00000003  // after SetInfo()
#define DSA_NEWOBJ_CTX_CLEANUP        0x00000004  // on post commit fail



//---------------------------------------------------------------------------
//
// Interface: IDsAdminNewObjExt
// 
// Implemented by: creation extension in proc server (both primary and regular)
//
// Used by: DS Admin
//

#undef  INTERFACE
#define INTERFACE   IDsAdminNewObjExt

DECLARE_INTERFACE_(IDsAdminNewObjExt, IUnknown)
{
  // *** IUnknown methods ***
  STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
  STDMETHOD_(ULONG,AddRef)(THIS)  PURE;
  STDMETHOD_(ULONG,Release)(THIS) PURE;


  // *** IDsAdminNewObjExt methods ***
  STDMETHOD(Initialize)(THIS_ /*IN*/ IADsContainer* pADsContainerObj, 
                              /*IN*/ IADs* pADsCopySource,
                              /*IN*/ LPCWSTR lpszClassName,
                              /*IN*/ IDsAdminNewObj* pDsAdminNewObj,
                              /*IN*/ LPDSA_NEWOBJ_DISPINFO pDispInfo) PURE;

  STDMETHOD(AddPages)(THIS_ /*IN*/ LPFNADDPROPSHEETPAGE lpfnAddPage, 
                            /*IN*/ LPARAM lParam) PURE;

  STDMETHOD(SetObject)(THIS_ /*IN*/ IADs* pADsObj) PURE;

  STDMETHOD(WriteData)(THIS_ /*IN*/ HWND hWnd, 
                             /*IN*/ ULONG uContext) PURE;
  
  STDMETHOD(OnError)(THIS_ /*IN*/ HWND hWnd, 
                           /*IN*/ HRESULT hr,
                              /*IN*/ ULONG uContext) PURE;
  
  STDMETHOD(GetSummaryInfo)(THIS_ /*OUT*/BSTR* pBstrText) PURE;
};


//
// Notification opcodes for IDsAdminNotifyHandler
//

#define DSA_NOTIFY_DEL      0x00000001  // delete
#define DSA_NOTIFY_REN      0x00000002  // rename
#define DSA_NOTIFY_MOV      0x00000004  // move
#define DSA_NOTIFY_PROP     0x00000008  // property change

#define DSA_NOTIFY_ALL      (DSA_NOTIFY_DEL|DSA_NOTIFY_REN|DSA_NOTIFY_MOV|DSA_NOTIFY_PROP)

//
// TODO: add explaination
//
// flags to handle additional data
//

#define DSA_NOTIFY_FLAG_ADDITIONAL_DATA        0x00000002   //process additional extension data?
#define DSA_NOTIFY_FLAG_FORCE_ADDITIONAL_DATA  0x00000001   //operation forced



//---------------------------------------------------------------------------
//
// Interface: IDsAdminNotifyHandler
// 
// Implemented by: notification handler in proc server
//
// Used by: DS Admin
//

#undef  INTERFACE
#define INTERFACE   IDsAdminNotifyHandler


DECLARE_INTERFACE_(IDsAdminNotifyHandler, IUnknown)
{
  // *** IUnknown methods ***
  STDMETHOD(QueryInterface)(THIS_ REFIID riid, LPVOID * ppvObj) PURE;
  STDMETHOD_(ULONG,AddRef)(THIS)  PURE;
  STDMETHOD_(ULONG,Release)(THIS) PURE;

  // IDsAdminNotifyHandler methods
  STDMETHOD(Initialize)(THIS_ /*IN*/ IDataObject* pExtraInfo, 
                              /*OUT*/ ULONG* puEventFlags) PURE;
  STDMETHOD(Begin)(THIS_ /*IN*/ ULONG uEvent,
                         /*IN*/ IDataObject* pArg1,
                         /*IN*/ IDataObject* pArg2,
                         /*OUT*/ ULONG* puFlags,
                         /*OUT*/ BSTR* pBstr) PURE;

  STDMETHOD(Notify)(THIS_ /*IN*/ ULONG nItem, /*IN*/ ULONG uFlags) PURE; 

  STDMETHOD(End)(THIS) PURE; 
};




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _DSADMIN_H

