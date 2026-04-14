

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __oleacc_h__
#define __oleacc_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IAccessible_FWD_DEFINED__
#define __IAccessible_FWD_DEFINED__
typedef interface IAccessible IAccessible;

#endif 	/* __IAccessible_FWD_DEFINED__ */


#ifndef __IAccessibleHandler_FWD_DEFINED__
#define __IAccessibleHandler_FWD_DEFINED__
typedef interface IAccessibleHandler IAccessibleHandler;

#endif 	/* __IAccessibleHandler_FWD_DEFINED__ */


#ifndef __IAccessibleWindowlessSite_FWD_DEFINED__
#define __IAccessibleWindowlessSite_FWD_DEFINED__
typedef interface IAccessibleWindowlessSite IAccessibleWindowlessSite;

#endif 	/* __IAccessibleWindowlessSite_FWD_DEFINED__ */


#ifndef __IAccIdentity_FWD_DEFINED__
#define __IAccIdentity_FWD_DEFINED__
typedef interface IAccIdentity IAccIdentity;

#endif 	/* __IAccIdentity_FWD_DEFINED__ */


#ifndef __IAccPropServer_FWD_DEFINED__
#define __IAccPropServer_FWD_DEFINED__
typedef interface IAccPropServer IAccPropServer;

#endif 	/* __IAccPropServer_FWD_DEFINED__ */


#ifndef __IAccPropServices_FWD_DEFINED__
#define __IAccPropServices_FWD_DEFINED__
typedef interface IAccPropServices IAccPropServices;

#endif 	/* __IAccPropServices_FWD_DEFINED__ */


#ifndef __IAccessible_FWD_DEFINED__
#define __IAccessible_FWD_DEFINED__
typedef interface IAccessible IAccessible;

#endif 	/* __IAccessible_FWD_DEFINED__ */


#ifndef __IAccessibleHandler_FWD_DEFINED__
#define __IAccessibleHandler_FWD_DEFINED__
typedef interface IAccessibleHandler IAccessibleHandler;

#endif 	/* __IAccessibleHandler_FWD_DEFINED__ */


#ifndef __IAccIdentity_FWD_DEFINED__
#define __IAccIdentity_FWD_DEFINED__
typedef interface IAccIdentity IAccIdentity;

#endif 	/* __IAccIdentity_FWD_DEFINED__ */


#ifndef __IAccPropServer_FWD_DEFINED__
#define __IAccPropServer_FWD_DEFINED__
typedef interface IAccPropServer IAccPropServer;

#endif 	/* __IAccPropServer_FWD_DEFINED__ */


#ifndef __IAccPropServices_FWD_DEFINED__
#define __IAccPropServices_FWD_DEFINED__
typedef interface IAccPropServices IAccPropServices;

#endif 	/* __IAccPropServices_FWD_DEFINED__ */


#ifndef __CAccPropServices_FWD_DEFINED__
#define __CAccPropServices_FWD_DEFINED__

#ifdef __cplusplus
typedef class CAccPropServices CAccPropServices;
#else
typedef struct CAccPropServices CAccPropServices;
#endif /* __cplusplus */

#endif 	/* __CAccPropServices_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_oleacc_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//=--------------------------------------------------------------------------=
// OLEACC.H
//=--------------------------------------------------------------------------=
// (C) Copyright (c) Microsoft Corporation. All rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//=--------------------------------------------------------------------------=
// Typedefs
//=--------------------------------------------------------------------------=

typedef LRESULT (STDAPICALLTYPE *LPFNLRESULTFROMOBJECT)(REFIID riid, WPARAM wParam, LPUNKNOWN punk);
typedef HRESULT (STDAPICALLTYPE *LPFNOBJECTFROMLRESULT)(LRESULT lResult, REFIID riid, WPARAM wParam, void** ppvObject);
typedef HRESULT (STDAPICALLTYPE *LPFNACCESSIBLEOBJECTFROMWINDOW)(HWND hwnd, DWORD dwId, REFIID riid, void** ppvObject);
typedef HRESULT (STDAPICALLTYPE *LPFNACCESSIBLEOBJECTFROMPOINT)(POINT ptScreen, IAccessible** ppacc, VARIANT* pvarChild);
typedef HRESULT (STDAPICALLTYPE *LPFNCREATESTDACCESSIBLEOBJECT)(HWND hwnd, LONG idObject, REFIID riid, void** ppvObject);
typedef HRESULT (STDAPICALLTYPE *LPFNACCESSIBLECHILDREN)(IAccessible* paccContainer, LONG iChildStart,LONG cChildren,VARIANT* rgvarChildren,LONG* pcObtained);

//=--------------------------------------------------------------------------=
// GUIDs
//=--------------------------------------------------------------------------=

DEFINE_GUID(LIBID_Accessibility,     0x1ea4dbf0, 0x3c3b, 0x11cf, 0x81, 0x0c, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71);
DEFINE_GUID(IID_IAccessible,         0x618736e0, 0x3c3d, 0x11cf, 0x81, 0x0c, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71);
DEFINE_GUID(IID_IAccessibleHandler,  0x03022430, 0xABC4, 0x11d0, 0xBD, 0xE2, 0x00, 0xAA, 0x00, 0x1A, 0x19, 0x53);
DEFINE_GUID(IID_IAccIdentity,        0x7852b78d, 0x1cfd, 0x41c1, 0xa6, 0x15, 0x9c, 0x0c, 0x85, 0x96, 0x0b, 0x5f);
DEFINE_GUID(IID_IAccPropServer,      0x76c0dbbb, 0x15e0, 0x4e7b, 0xb6, 0x1b, 0x20, 0xee, 0xea, 0x20, 0x01, 0xe0);
DEFINE_GUID(IID_IAccPropServices,    0x6e26e776, 0x04f0, 0x495d, 0x80, 0xe4, 0x33, 0x30, 0x35, 0x2e, 0x31, 0x69);
DEFINE_GUID(IID_IAccPropMgrInternal, 0x2bd370a9, 0x3e7f, 0x4edd, 0x8a, 0x85, 0xf8, 0xfe, 0xd1, 0xf8, 0xe5, 0x1f);
DEFINE_GUID(CLSID_AccPropServices,   0xb5f8350b, 0x0548, 0x48b1, 0xa6, 0xee, 0x88, 0xbd, 0x00, 0xb4, 0xa5, 0xe7);
DEFINE_GUID(IIS_IsOleaccProxy,       0x902697fa, 0x80e4, 0x4560, 0x80, 0x2a, 0xa1, 0x3f, 0x22, 0xa6, 0x47, 0x09);
DEFINE_GUID(IIS_ControlAccessible,   0x38c682a6, 0x9731, 0x43f2, 0x9f, 0xae, 0xe9, 0x01, 0xe6, 0x41, 0xb1, 0x01);

//=--------------------------------------------------------------------------=
// MSAA API Prototypes
//=--------------------------------------------------------------------------=

STDAPI_(LRESULT) LresultFromObject(_In_ REFIID riid, _In_ WPARAM wParam, _In_ LPUNKNOWN punk);
STDAPI          ObjectFromLresult(_In_ LRESULT lResult, _In_ REFIID riid, _In_ WPARAM wParam, _Outptr_ void** ppvObject);
STDAPI          WindowFromAccessibleObject(_In_ IAccessible*, _Out_opt_ HWND* phwnd);
STDAPI          AccessibleObjectFromWindow(_In_ HWND hwnd, _In_ DWORD dwId, _In_ REFIID riid, _Outptr_ void **ppvObject);
STDAPI          AccessibleObjectFromEvent(_In_ HWND hwnd, _In_ DWORD dwId, _In_ DWORD dwChildId, _Outptr_ IAccessible** ppacc, _Out_ VARIANT* pvarChild);
STDAPI          AccessibleObjectFromPoint(_In_ POINT ptScreen, _Outptr_ IAccessible ** ppacc, _Out_ VARIANT* pvarChild);
STDAPI          AccessibleChildren (_In_ IAccessible* paccContainer, _In_ LONG iChildStart, _In_ LONG cChildren, _Out_writes_(cChildren) VARIANT* rgvarChildren, _Out_ LONG* pcObtained);

STDAPI_(UINT)   GetRoleTextA(_In_ DWORD lRole, _Out_writes_opt_(cchRoleMax) LPSTR lpszRole, _In_ UINT cchRoleMax);
STDAPI_(UINT)   GetRoleTextW(_In_ DWORD lRole, _Out_writes_opt_(cchRoleMax) LPWSTR lpszRole, _In_ UINT cchRoleMax);

#ifdef UNICODE
#define GetRoleText     GetRoleTextW
#else
#define GetRoleText     GetRoleTextA
#endif // UNICODE

STDAPI_(UINT)   GetStateTextA(_In_ DWORD lStateBit, _Out_writes_opt_(cchState) LPSTR lpszState, _In_ UINT cchState);
STDAPI_(UINT)   GetStateTextW(_In_ DWORD lStateBit, _Out_writes_opt_(cchState) LPWSTR lpszState, _In_ UINT cchState);
#ifdef UNICODE
#define GetStateText    GetStateTextW
#else
#define GetStateText    GetStateTextA
#endif // UNICODE

STDAPI_(VOID)   GetOleaccVersionInfo(_Out_ DWORD* pVer, _Out_ DWORD* pBuild);

STDAPI          CreateStdAccessibleObject(_In_ HWND hwnd, _In_ LONG idObject, _In_ REFIID riid, _Outptr_ void** ppvObject);

STDAPI          CreateStdAccessibleProxyA(_In_ HWND hwnd, _In_ LPCSTR pClassName, _In_ LONG idObject, _In_ REFIID riid, _Outptr_ void** ppvObject);
STDAPI          CreateStdAccessibleProxyW(_In_ HWND hwnd, _In_ LPCWSTR pClassName, _In_ LONG idObject, _In_ REFIID riid, _Outptr_ void** ppvObject);

#ifdef UNICODE
#define CreateStdAccessibleProxy     CreateStdAccessibleProxyW
#else
#define CreateStdAccessibleProxy     CreateStdAccessibleProxyA
#endif // UNICODE

#define ANRUS_ON_SCREEN_KEYBOARD_ACTIVE 0x0000001
#define ANRUS_TOUCH_MODIFICATION_ACTIVE 0x0000002
#define ANRUS_PRIORITY_AUDIO_ACTIVE     0x0000004
#define ANRUS_PRIORITY_AUDIO_ACTIVE_NODUCK 0x0000008
#define ANRUS_PRIORITY_AUDIO_DYNAMIC_DUCK 0x0000010

STDAPI          AccSetRunningUtilityState(_In_ HWND hwndApp, _In_ DWORD dwUtilityStateMask, _In_ DWORD dwUtilityState);

STDAPI          AccNotifyTouchInteraction(_In_ HWND hwndApp, _In_ HWND hwndTarget, _In_ POINT ptTarget);


// Simple Owner-Drawn Menu support...

#define MSAA_MENU_SIG 0xAA0DF00DL

// Menu's dwItemData should point to one of these structs:
// (or can point to an app-defined struct containing this as the first member)
typedef struct tagMSAAMENUINFO {
    DWORD   dwMSAASignature; // Must be MSAA_MENU_SIG
    DWORD   cchWText;        // Length of text, in Unicode characters, excluding terminating NUL
    LPWSTR  pszWText;        // NUL-terminated text, in Unicode
} MSAAMENUINFO, *LPMSAAMENUINFO;


//=--------------------------------------------------------------------------=
// Property GUIDs (used by annotation interfaces)
//=--------------------------------------------------------------------------=

DEFINE_GUID( PROPID_ACC_NAME             , 0x608d3df8, 0x8128, 0x4aa7, 0xa4, 0x28, 0xf5, 0x5e, 0x49, 0x26, 0x72, 0x91);
DEFINE_GUID( PROPID_ACC_VALUE            , 0x123fe443, 0x211a, 0x4615, 0x95, 0x27, 0xc4, 0x5a, 0x7e, 0x93, 0x71, 0x7a);
DEFINE_GUID( PROPID_ACC_DESCRIPTION      , 0x4d48dfe4, 0xbd3f, 0x491f, 0xa6, 0x48, 0x49, 0x2d, 0x6f, 0x20, 0xc5, 0x88);
DEFINE_GUID( PROPID_ACC_ROLE             , 0xcb905ff2, 0x7bd1, 0x4c05, 0xb3, 0xc8, 0xe6, 0xc2, 0x41, 0x36, 0x4d, 0x70);
DEFINE_GUID( PROPID_ACC_STATE            , 0xa8d4d5b0, 0x0a21, 0x42d0, 0xa5, 0xc0, 0x51, 0x4e, 0x98, 0x4f, 0x45, 0x7b);
DEFINE_GUID( PROPID_ACC_HELP             , 0xc831e11f, 0x44db, 0x4a99, 0x97, 0x68, 0xcb, 0x8f, 0x97, 0x8b, 0x72, 0x31);
DEFINE_GUID( PROPID_ACC_KEYBOARDSHORTCUT , 0x7d9bceee, 0x7d1e, 0x4979, 0x93, 0x82, 0x51, 0x80, 0xf4, 0x17, 0x2c, 0x34);
DEFINE_GUID( PROPID_ACC_DEFAULTACTION    , 0x180c072b, 0xc27f, 0x43c7, 0x99, 0x22, 0xf6, 0x35, 0x62, 0xa4, 0x63, 0x2b);

DEFINE_GUID( PROPID_ACC_HELPTOPIC        , 0x787d1379, 0x8ede, 0x440b, 0x8a, 0xec, 0x11, 0xf7, 0xbf, 0x90, 0x30, 0xb3);
DEFINE_GUID( PROPID_ACC_FOCUS            , 0x6eb335df, 0x1c29, 0x4127, 0xb1, 0x2c, 0xde, 0xe9, 0xfd, 0x15, 0x7f, 0x2b);
DEFINE_GUID( PROPID_ACC_SELECTION        , 0xb99d073c, 0xd731, 0x405b, 0x90, 0x61, 0xd9, 0x5e, 0x8f, 0x84, 0x29, 0x84);
DEFINE_GUID( PROPID_ACC_PARENT           , 0x474c22b6, 0xffc2, 0x467a, 0xb1, 0xb5, 0xe9, 0x58, 0xb4, 0x65, 0x73, 0x30);

DEFINE_GUID( PROPID_ACC_NAV_UP           , 0x016e1a2b, 0x1a4e, 0x4767, 0x86, 0x12, 0x33, 0x86, 0xf6, 0x69, 0x35, 0xec);
DEFINE_GUID( PROPID_ACC_NAV_DOWN         , 0x031670ed, 0x3cdf, 0x48d2, 0x96, 0x13, 0x13, 0x8f, 0x2d, 0xd8, 0xa6, 0x68);
DEFINE_GUID( PROPID_ACC_NAV_LEFT         , 0x228086cb, 0x82f1, 0x4a39, 0x87, 0x05, 0xdc, 0xdc, 0x0f, 0xff, 0x92, 0xf5);
DEFINE_GUID( PROPID_ACC_NAV_RIGHT        , 0xcd211d9f, 0xe1cb, 0x4fe5, 0xa7, 0x7c, 0x92, 0x0b, 0x88, 0x4d, 0x09, 0x5b);
DEFINE_GUID( PROPID_ACC_NAV_PREV         , 0x776d3891, 0xc73b, 0x4480, 0xb3, 0xf6, 0x07, 0x6a, 0x16, 0xa1, 0x5a, 0xf6);
DEFINE_GUID( PROPID_ACC_NAV_NEXT         , 0x1cdc5455, 0x8cd9, 0x4c92, 0xa3, 0x71, 0x39, 0x39, 0xa2, 0xfe, 0x3e, 0xee);
DEFINE_GUID( PROPID_ACC_NAV_FIRSTCHILD   , 0xcfd02558, 0x557b, 0x4c67, 0x84, 0xf9, 0x2a, 0x09, 0xfc, 0xe4, 0x07, 0x49);
DEFINE_GUID( PROPID_ACC_NAV_LASTCHILD    , 0x302ecaa5, 0x48d5, 0x4f8d, 0xb6, 0x71, 0x1a, 0x8d, 0x20, 0xa7, 0x78, 0x32);

// Value map, used by sliders and other controls...
DEFINE_GUID( PROPID_ACC_ROLEMAP          , 0xf79acda2, 0x140d, 0x4fe6, 0x89, 0x14, 0x20, 0x84, 0x76, 0x32, 0x82, 0x69);
DEFINE_GUID( PROPID_ACC_VALUEMAP         , 0xda1c3d79, 0xfc5c, 0x420e, 0xb3, 0x99, 0x9d, 0x15, 0x33, 0x54, 0x9e, 0x75);
DEFINE_GUID( PROPID_ACC_STATEMAP         , 0x43946c5e, 0x0ac0, 0x4042, 0xb5, 0x25, 0x07, 0xbb, 0xdb, 0xe1, 0x7f, 0xa7);
DEFINE_GUID( PROPID_ACC_DESCRIPTIONMAP   , 0x1ff1435f, 0x8a14, 0x477b, 0xb2, 0x26, 0xa0, 0xab, 0xe2, 0x79, 0x97, 0x5d);

DEFINE_GUID( PROPID_ACC_DODEFAULTACTION  , 0x1ba09523, 0x2e3b, 0x49a6, 0xa0, 0x59, 0x59, 0x68, 0x2a, 0x3c, 0x48, 0xfd);

//=--------------------------------------------------------------------------=
// Interface Definitions
//=--------------------------------------------------------------------------=



extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0000_v0_0_s_ifspec;

#ifndef __IAccessible_INTERFACE_DEFINED__
#define __IAccessible_INTERFACE_DEFINED__

/* interface IAccessible */
/* [unique][dual][hidden][uuid][object] */ 

#define	DISPID_ACC_PARENT	( -5000 )

#define	DISPID_ACC_CHILDCOUNT	( -5001 )

#define	DISPID_ACC_CHILD	( -5002 )

#define	DISPID_ACC_NAME	( -5003 )

#define	DISPID_ACC_VALUE	( -5004 )

#define	DISPID_ACC_DESCRIPTION	( -5005 )

#define	DISPID_ACC_ROLE	( -5006 )

#define	DISPID_ACC_STATE	( -5007 )

#define	DISPID_ACC_HELP	( -5008 )

#define	DISPID_ACC_HELPTOPIC	( -5009 )

#define	DISPID_ACC_KEYBOARDSHORTCUT	( -5010 )

#define	DISPID_ACC_FOCUS	( -5011 )

#define	DISPID_ACC_SELECTION	( -5012 )

#define	DISPID_ACC_DEFAULTACTION	( -5013 )

#define	DISPID_ACC_SELECT	( -5014 )

#define	DISPID_ACC_LOCATION	( -5015 )

#define	DISPID_ACC_NAVIGATE	( -5016 )

#define	DISPID_ACC_HITTEST	( -5017 )

#define	DISPID_ACC_DODEFAULTACTION	( -5018 )

typedef /* [unique] */  __RPC_unique_pointer IAccessible *LPACCESSIBLE;

#define	NAVDIR_MIN	( 0 )

#define	NAVDIR_UP	( 0x1 )

#define	NAVDIR_DOWN	( 0x2 )

#define	NAVDIR_LEFT	( 0x3 )

#define	NAVDIR_RIGHT	( 0x4 )

#define	NAVDIR_NEXT	( 0x5 )

#define	NAVDIR_PREVIOUS	( 0x6 )

#define	NAVDIR_FIRSTCHILD	( 0x7 )

#define	NAVDIR_LASTCHILD	( 0x8 )

#define	NAVDIR_MAX	( 0x9 )

#define	SELFLAG_NONE	( 0 )

#define	SELFLAG_TAKEFOCUS	( 0x1 )

#define	SELFLAG_TAKESELECTION	( 0x2 )

#define	SELFLAG_EXTENDSELECTION	( 0x4 )

#define	SELFLAG_ADDSELECTION	( 0x8 )

#define	SELFLAG_REMOVESELECTION	( 0x10 )

#define	SELFLAG_VALID	( 0x1f )

#ifndef STATE_SYSTEM_UNAVAILABLE
#define	STATE_SYSTEM_NORMAL	( 0 )

#define	STATE_SYSTEM_UNAVAILABLE	( 0x1 )

#define	STATE_SYSTEM_SELECTED	( 0x2 )

#define	STATE_SYSTEM_FOCUSED	( 0x4 )

#define	STATE_SYSTEM_PRESSED	( 0x8 )

#define	STATE_SYSTEM_CHECKED	( 0x10 )

#define	STATE_SYSTEM_MIXED	( 0x20 )

#define	STATE_SYSTEM_INDETERMINATE	( STATE_SYSTEM_MIXED )

#define	STATE_SYSTEM_READONLY	( 0x40 )

#define	STATE_SYSTEM_HOTTRACKED	( 0x80 )

#define	STATE_SYSTEM_DEFAULT	( 0x100 )

#define	STATE_SYSTEM_EXPANDED	( 0x200 )

#define	STATE_SYSTEM_COLLAPSED	( 0x400 )

#define	STATE_SYSTEM_BUSY	( 0x800 )

#define	STATE_SYSTEM_FLOATING	( 0x1000 )

#define	STATE_SYSTEM_MARQUEED	( 0x2000 )

#define	STATE_SYSTEM_ANIMATED	( 0x4000 )

#define	STATE_SYSTEM_INVISIBLE	( 0x8000 )

#define	STATE_SYSTEM_OFFSCREEN	( 0x10000 )

#define	STATE_SYSTEM_SIZEABLE	( 0x20000 )

#define	STATE_SYSTEM_MOVEABLE	( 0x40000 )

#define	STATE_SYSTEM_SELFVOICING	( 0x80000 )

#define	STATE_SYSTEM_FOCUSABLE	( 0x100000 )

#define	STATE_SYSTEM_SELECTABLE	( 0x200000 )

#define	STATE_SYSTEM_LINKED	( 0x400000 )

#define	STATE_SYSTEM_TRAVERSED	( 0x800000 )

#define	STATE_SYSTEM_MULTISELECTABLE	( 0x1000000 )

#define	STATE_SYSTEM_EXTSELECTABLE	( 0x2000000 )

#define	STATE_SYSTEM_ALERT_LOW	( 0x4000000 )

#define	STATE_SYSTEM_ALERT_MEDIUM	( 0x8000000 )

#define	STATE_SYSTEM_ALERT_HIGH	( 0x10000000 )

#define	STATE_SYSTEM_PROTECTED	( 0x20000000 )

#define	STATE_SYSTEM_VALID	( 0x7fffffff )

#endif //STATE_SYSTEM_UNAVAILABLE
#ifndef STATE_SYSTEM_HASPOPUP
#define	STATE_SYSTEM_HASPOPUP	( 0x40000000 )

#endif //STATE_SYSTEM_HASPOPUP
#define	ROLE_SYSTEM_TITLEBAR	( 0x1 )

#define	ROLE_SYSTEM_MENUBAR	( 0x2 )

#define	ROLE_SYSTEM_SCROLLBAR	( 0x3 )

#define	ROLE_SYSTEM_GRIP	( 0x4 )

#define	ROLE_SYSTEM_SOUND	( 0x5 )

#define	ROLE_SYSTEM_CURSOR	( 0x6 )

#define	ROLE_SYSTEM_CARET	( 0x7 )

#define	ROLE_SYSTEM_ALERT	( 0x8 )

#define	ROLE_SYSTEM_WINDOW	( 0x9 )

#define	ROLE_SYSTEM_CLIENT	( 0xa )

#define	ROLE_SYSTEM_MENUPOPUP	( 0xb )

#define	ROLE_SYSTEM_MENUITEM	( 0xc )

#define	ROLE_SYSTEM_TOOLTIP	( 0xd )

#define	ROLE_SYSTEM_APPLICATION	( 0xe )

#define	ROLE_SYSTEM_DOCUMENT	( 0xf )

#define	ROLE_SYSTEM_PANE	( 0x10 )

#define	ROLE_SYSTEM_CHART	( 0x11 )

#define	ROLE_SYSTEM_DIALOG	( 0x12 )

#define	ROLE_SYSTEM_BORDER	( 0x13 )

#define	ROLE_SYSTEM_GROUPING	( 0x14 )

#define	ROLE_SYSTEM_SEPARATOR	( 0x15 )

#define	ROLE_SYSTEM_TOOLBAR	( 0x16 )

#define	ROLE_SYSTEM_STATUSBAR	( 0x17 )

#define	ROLE_SYSTEM_TABLE	( 0x18 )

#define	ROLE_SYSTEM_COLUMNHEADER	( 0x19 )

#define	ROLE_SYSTEM_ROWHEADER	( 0x1a )

#define	ROLE_SYSTEM_COLUMN	( 0x1b )

#define	ROLE_SYSTEM_ROW	( 0x1c )

#define	ROLE_SYSTEM_CELL	( 0x1d )

#define	ROLE_SYSTEM_LINK	( 0x1e )

#define	ROLE_SYSTEM_HELPBALLOON	( 0x1f )

#define	ROLE_SYSTEM_CHARACTER	( 0x20 )

#define	ROLE_SYSTEM_LIST	( 0x21 )

#define	ROLE_SYSTEM_LISTITEM	( 0x22 )

#define	ROLE_SYSTEM_OUTLINE	( 0x23 )

#define	ROLE_SYSTEM_OUTLINEITEM	( 0x24 )

#define	ROLE_SYSTEM_PAGETAB	( 0x25 )

#define	ROLE_SYSTEM_PROPERTYPAGE	( 0x26 )

#define	ROLE_SYSTEM_INDICATOR	( 0x27 )

#define	ROLE_SYSTEM_GRAPHIC	( 0x28 )

#define	ROLE_SYSTEM_STATICTEXT	( 0x29 )

#define	ROLE_SYSTEM_TEXT	( 0x2a )

#define	ROLE_SYSTEM_PUSHBUTTON	( 0x2b )

#define	ROLE_SYSTEM_CHECKBUTTON	( 0x2c )

#define	ROLE_SYSTEM_RADIOBUTTON	( 0x2d )

#define	ROLE_SYSTEM_COMBOBOX	( 0x2e )

#define	ROLE_SYSTEM_DROPLIST	( 0x2f )

#define	ROLE_SYSTEM_PROGRESSBAR	( 0x30 )

#define	ROLE_SYSTEM_DIAL	( 0x31 )

#define	ROLE_SYSTEM_HOTKEYFIELD	( 0x32 )

#define	ROLE_SYSTEM_SLIDER	( 0x33 )

#define	ROLE_SYSTEM_SPINBUTTON	( 0x34 )

#define	ROLE_SYSTEM_DIAGRAM	( 0x35 )

#define	ROLE_SYSTEM_ANIMATION	( 0x36 )

#define	ROLE_SYSTEM_EQUATION	( 0x37 )

#define	ROLE_SYSTEM_BUTTONDROPDOWN	( 0x38 )

#define	ROLE_SYSTEM_BUTTONMENU	( 0x39 )

#define	ROLE_SYSTEM_BUTTONDROPDOWNGRID	( 0x3a )

#define	ROLE_SYSTEM_WHITESPACE	( 0x3b )

#define	ROLE_SYSTEM_PAGETABLIST	( 0x3c )

#define	ROLE_SYSTEM_CLOCK	( 0x3d )

#define	ROLE_SYSTEM_SPLITBUTTON	( 0x3e )

#define	ROLE_SYSTEM_IPADDRESS	( 0x3f )

#define	ROLE_SYSTEM_OUTLINEBUTTON	( 0x40 )


EXTERN_C const IID IID_IAccessible;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("618736e0-3c3d-11cf-810c-00aa00389b71")
    IAccessible : public IDispatch
    {
    public:
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accParent( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispParent) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accChildCount( 
            /* [retval][out] */ __RPC__out long *pcountChildren) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accChild( 
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispChild) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accName( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszName) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accValue( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszValue) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accDescription( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDescription) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accRole( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out VARIANT *pvarRole) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accState( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out VARIANT *pvarState) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accHelp( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszHelp) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accHelpTopic( 
            /* [out] */ __RPC__deref_out_opt BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out long *pidTopic) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accKeyboardShortcut( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszKeyboardShortcut) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accFocus( 
            /* [retval][out] */ __RPC__out VARIANT *pvarChild) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accSelection( 
            /* [retval][out] */ __RPC__out VARIANT *pvarChildren) = 0;
        
        virtual /* [id][propget][hidden] */ HRESULT STDMETHODCALLTYPE get_accDefaultAction( 
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDefaultAction) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE accSelect( 
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE accLocation( 
            /* [out] */ __RPC__out long *pxLeft,
            /* [out] */ __RPC__out long *pyTop,
            /* [out] */ __RPC__out long *pcxWidth,
            /* [out] */ __RPC__out long *pcyHeight,
            /* [optional][in] */ VARIANT varChild) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE accNavigate( 
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ __RPC__out VARIANT *pvarEndUpAt) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE accHitTest( 
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ __RPC__out VARIANT *pvarChild) = 0;
        
        virtual /* [id][hidden] */ HRESULT STDMETHODCALLTYPE accDoDefaultAction( 
            /* [optional][in] */ VARIANT varChild) = 0;
        
        virtual /* [id][propput][hidden] */ HRESULT STDMETHODCALLTYPE put_accName( 
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ __RPC__in BSTR szName) = 0;
        
        virtual /* [id][propput][hidden] */ HRESULT STDMETHODCALLTYPE put_accValue( 
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ __RPC__in BSTR szValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessible * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessible * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessible * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAccessible * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAccessible * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAccessible * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAccessible * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accParent)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accParent )( 
            __RPC__in IAccessible * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispParent);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChildCount)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChildCount )( 
            __RPC__in IAccessible * This,
            /* [retval][out] */ __RPC__out long *pcountChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accChild)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accChild )( 
            __RPC__in IAccessible * This,
            /* [in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accName)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accName )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszName);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accValue)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accValue )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDescription)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDescription )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accRole)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accRole )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out VARIANT *pvarRole);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accState)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accState )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out VARIANT *pvarState);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelp)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelp )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accHelpTopic)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accHelpTopic )( 
            __RPC__in IAccessible * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pszHelpFile,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__out long *pidTopic);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accKeyboardShortcut)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accKeyboardShortcut )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accFocus)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accFocus )( 
            __RPC__in IAccessible * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accSelection)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accSelection )( 
            __RPC__in IAccessible * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarChildren);
        
        DECLSPEC_XFGVIRT(IAccessible, get_accDefaultAction)
        /* [id][propget][hidden] */ HRESULT ( STDMETHODCALLTYPE *get_accDefaultAction )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDefaultAction);
        
        DECLSPEC_XFGVIRT(IAccessible, accSelect)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accSelect )( 
            __RPC__in IAccessible * This,
            /* [in] */ long flagsSelect,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accLocation)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accLocation )( 
            __RPC__in IAccessible * This,
            /* [out] */ __RPC__out long *pxLeft,
            /* [out] */ __RPC__out long *pyTop,
            /* [out] */ __RPC__out long *pcxWidth,
            /* [out] */ __RPC__out long *pcyHeight,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accNavigate)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accNavigate )( 
            __RPC__in IAccessible * This,
            /* [in] */ long navDir,
            /* [optional][in] */ VARIANT varStart,
            /* [retval][out] */ __RPC__out VARIANT *pvarEndUpAt);
        
        DECLSPEC_XFGVIRT(IAccessible, accHitTest)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accHitTest )( 
            __RPC__in IAccessible * This,
            /* [in] */ long xLeft,
            /* [in] */ long yTop,
            /* [retval][out] */ __RPC__out VARIANT *pvarChild);
        
        DECLSPEC_XFGVIRT(IAccessible, accDoDefaultAction)
        /* [id][hidden] */ HRESULT ( STDMETHODCALLTYPE *accDoDefaultAction )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accName)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accName )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ __RPC__in BSTR szName);
        
        DECLSPEC_XFGVIRT(IAccessible, put_accValue)
        /* [id][propput][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_accValue )( 
            __RPC__in IAccessible * This,
            /* [optional][in] */ VARIANT varChild,
            /* [in] */ __RPC__in BSTR szValue);
        
        END_INTERFACE
    } IAccessibleVtbl;

    interface IAccessible
    {
        CONST_VTBL struct IAccessibleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessible_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessible_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessible_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessible_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAccessible_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAccessible_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAccessible_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAccessible_get_accParent(This,ppdispParent)	\
    ( (This)->lpVtbl -> get_accParent(This,ppdispParent) ) 

#define IAccessible_get_accChildCount(This,pcountChildren)	\
    ( (This)->lpVtbl -> get_accChildCount(This,pcountChildren) ) 

#define IAccessible_get_accChild(This,varChild,ppdispChild)	\
    ( (This)->lpVtbl -> get_accChild(This,varChild,ppdispChild) ) 

#define IAccessible_get_accName(This,varChild,pszName)	\
    ( (This)->lpVtbl -> get_accName(This,varChild,pszName) ) 

#define IAccessible_get_accValue(This,varChild,pszValue)	\
    ( (This)->lpVtbl -> get_accValue(This,varChild,pszValue) ) 

#define IAccessible_get_accDescription(This,varChild,pszDescription)	\
    ( (This)->lpVtbl -> get_accDescription(This,varChild,pszDescription) ) 

#define IAccessible_get_accRole(This,varChild,pvarRole)	\
    ( (This)->lpVtbl -> get_accRole(This,varChild,pvarRole) ) 

#define IAccessible_get_accState(This,varChild,pvarState)	\
    ( (This)->lpVtbl -> get_accState(This,varChild,pvarState) ) 

#define IAccessible_get_accHelp(This,varChild,pszHelp)	\
    ( (This)->lpVtbl -> get_accHelp(This,varChild,pszHelp) ) 

#define IAccessible_get_accHelpTopic(This,pszHelpFile,varChild,pidTopic)	\
    ( (This)->lpVtbl -> get_accHelpTopic(This,pszHelpFile,varChild,pidTopic) ) 

#define IAccessible_get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_accKeyboardShortcut(This,varChild,pszKeyboardShortcut) ) 

#define IAccessible_get_accFocus(This,pvarChild)	\
    ( (This)->lpVtbl -> get_accFocus(This,pvarChild) ) 

#define IAccessible_get_accSelection(This,pvarChildren)	\
    ( (This)->lpVtbl -> get_accSelection(This,pvarChildren) ) 

#define IAccessible_get_accDefaultAction(This,varChild,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_accDefaultAction(This,varChild,pszDefaultAction) ) 

#define IAccessible_accSelect(This,flagsSelect,varChild)	\
    ( (This)->lpVtbl -> accSelect(This,flagsSelect,varChild) ) 

#define IAccessible_accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild)	\
    ( (This)->lpVtbl -> accLocation(This,pxLeft,pyTop,pcxWidth,pcyHeight,varChild) ) 

#define IAccessible_accNavigate(This,navDir,varStart,pvarEndUpAt)	\
    ( (This)->lpVtbl -> accNavigate(This,navDir,varStart,pvarEndUpAt) ) 

#define IAccessible_accHitTest(This,xLeft,yTop,pvarChild)	\
    ( (This)->lpVtbl -> accHitTest(This,xLeft,yTop,pvarChild) ) 

#define IAccessible_accDoDefaultAction(This,varChild)	\
    ( (This)->lpVtbl -> accDoDefaultAction(This,varChild) ) 

#define IAccessible_put_accName(This,varChild,szName)	\
    ( (This)->lpVtbl -> put_accName(This,varChild,szName) ) 

#define IAccessible_put_accValue(This,varChild,szValue)	\
    ( (This)->lpVtbl -> put_accValue(This,varChild,szValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessible_INTERFACE_DEFINED__ */


#ifndef __IAccessibleHandler_INTERFACE_DEFINED__
#define __IAccessibleHandler_INTERFACE_DEFINED__

/* interface IAccessibleHandler */
/* [unique][oleautomation][hidden][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IAccessibleHandler *LPACCESSIBLEHANDLER;


EXTERN_C const IID IID_IAccessibleHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03022430-ABC4-11d0-BDE2-00AA001A1953")
    IAccessibleHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AccessibleObjectFromID( 
            /* [in] */ long hwnd,
            /* [in] */ long lObjectID,
            /* [out] */ __RPC__deref_out_opt LPACCESSIBLE *pIAccessible) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibleHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibleHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibleHandler * This);
        
        DECLSPEC_XFGVIRT(IAccessibleHandler, AccessibleObjectFromID)
        HRESULT ( STDMETHODCALLTYPE *AccessibleObjectFromID )( 
            __RPC__in IAccessibleHandler * This,
            /* [in] */ long hwnd,
            /* [in] */ long lObjectID,
            /* [out] */ __RPC__deref_out_opt LPACCESSIBLE *pIAccessible);
        
        END_INTERFACE
    } IAccessibleHandlerVtbl;

    interface IAccessibleHandler
    {
        CONST_VTBL struct IAccessibleHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleHandler_AccessibleObjectFromID(This,hwnd,lObjectID,pIAccessible)	\
    ( (This)->lpVtbl -> AccessibleObjectFromID(This,hwnd,lObjectID,pIAccessible) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleHandler_INTERFACE_DEFINED__ */


#ifndef __IAccessibleWindowlessSite_INTERFACE_DEFINED__
#define __IAccessibleWindowlessSite_INTERFACE_DEFINED__

/* interface IAccessibleWindowlessSite */
/* [unique][oleautomation][hidden][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IAccessibleWindowlessSite *LPACCESSIBLEWINDOWLESSSITE;


EXTERN_C const IID IID_IAccessibleWindowlessSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF3ABD9C-76DA-4389-9EB6-1427D25ABAB7")
    IAccessibleWindowlessSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AcquireObjectIdRange( 
            /* [in] */ long rangeSize,
            /* [in] */ __RPC__in_opt IAccessibleHandler *pRangeOwner,
            /* [out] */ __RPC__out long *pRangeBase) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseObjectIdRange( 
            /* [in] */ long rangeBase,
            /* [in] */ __RPC__in_opt IAccessibleHandler *pRangeOwner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryObjectIdRanges( 
            /* [in] */ __RPC__in_opt IAccessibleHandler *pRangesOwner,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *psaRanges) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParentAccessible( 
            /* [out] */ __RPC__deref_out_opt IAccessible **ppParent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleWindowlessSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibleWindowlessSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibleWindowlessSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibleWindowlessSite * This);
        
        DECLSPEC_XFGVIRT(IAccessibleWindowlessSite, AcquireObjectIdRange)
        HRESULT ( STDMETHODCALLTYPE *AcquireObjectIdRange )( 
            __RPC__in IAccessibleWindowlessSite * This,
            /* [in] */ long rangeSize,
            /* [in] */ __RPC__in_opt IAccessibleHandler *pRangeOwner,
            /* [out] */ __RPC__out long *pRangeBase);
        
        DECLSPEC_XFGVIRT(IAccessibleWindowlessSite, ReleaseObjectIdRange)
        HRESULT ( STDMETHODCALLTYPE *ReleaseObjectIdRange )( 
            __RPC__in IAccessibleWindowlessSite * This,
            /* [in] */ long rangeBase,
            /* [in] */ __RPC__in_opt IAccessibleHandler *pRangeOwner);
        
        DECLSPEC_XFGVIRT(IAccessibleWindowlessSite, QueryObjectIdRanges)
        HRESULT ( STDMETHODCALLTYPE *QueryObjectIdRanges )( 
            __RPC__in IAccessibleWindowlessSite * This,
            /* [in] */ __RPC__in_opt IAccessibleHandler *pRangesOwner,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *psaRanges);
        
        DECLSPEC_XFGVIRT(IAccessibleWindowlessSite, GetParentAccessible)
        HRESULT ( STDMETHODCALLTYPE *GetParentAccessible )( 
            __RPC__in IAccessibleWindowlessSite * This,
            /* [out] */ __RPC__deref_out_opt IAccessible **ppParent);
        
        END_INTERFACE
    } IAccessibleWindowlessSiteVtbl;

    interface IAccessibleWindowlessSite
    {
        CONST_VTBL struct IAccessibleWindowlessSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleWindowlessSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleWindowlessSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleWindowlessSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleWindowlessSite_AcquireObjectIdRange(This,rangeSize,pRangeOwner,pRangeBase)	\
    ( (This)->lpVtbl -> AcquireObjectIdRange(This,rangeSize,pRangeOwner,pRangeBase) ) 

#define IAccessibleWindowlessSite_ReleaseObjectIdRange(This,rangeBase,pRangeOwner)	\
    ( (This)->lpVtbl -> ReleaseObjectIdRange(This,rangeBase,pRangeOwner) ) 

#define IAccessibleWindowlessSite_QueryObjectIdRanges(This,pRangesOwner,psaRanges)	\
    ( (This)->lpVtbl -> QueryObjectIdRanges(This,pRangesOwner,psaRanges) ) 

#define IAccessibleWindowlessSite_GetParentAccessible(This,ppParent)	\
    ( (This)->lpVtbl -> GetParentAccessible(This,ppParent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleWindowlessSite_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oleacc_0000_0003 */
/* [local] */ 

typedef 
enum AnnoScope
    {
        ANNO_THIS	= 0,
        ANNO_CONTAINER	= 1
    } 	AnnoScope;

typedef GUID MSAAPROPID;



extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0003_v0_0_s_ifspec;

#ifndef __IAccIdentity_INTERFACE_DEFINED__
#define __IAccIdentity_INTERFACE_DEFINED__

/* interface IAccIdentity */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAccIdentity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7852b78d-1cfd-41c1-a615-9c0c85960b5f")
    IAccIdentity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIdentityString( 
            /* [in] */ DWORD dwIDChild,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwIDStringLen) BYTE **ppIDString,
            /* [out] */ __RPC__out DWORD *pdwIDStringLen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccIdentityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccIdentity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccIdentity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccIdentity * This);
        
        DECLSPEC_XFGVIRT(IAccIdentity, GetIdentityString)
        HRESULT ( STDMETHODCALLTYPE *GetIdentityString )( 
            __RPC__in IAccIdentity * This,
            /* [in] */ DWORD dwIDChild,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwIDStringLen) BYTE **ppIDString,
            /* [out] */ __RPC__out DWORD *pdwIDStringLen);
        
        END_INTERFACE
    } IAccIdentityVtbl;

    interface IAccIdentity
    {
        CONST_VTBL struct IAccIdentityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccIdentity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccIdentity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccIdentity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccIdentity_GetIdentityString(This,dwIDChild,ppIDString,pdwIDStringLen)	\
    ( (This)->lpVtbl -> GetIdentityString(This,dwIDChild,ppIDString,pdwIDStringLen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccIdentity_INTERFACE_DEFINED__ */


#ifndef __IAccPropServer_INTERFACE_DEFINED__
#define __IAccPropServer_INTERFACE_DEFINED__

/* interface IAccPropServer */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAccPropServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76c0dbbb-15e0-4e7b-b61b-20eeea2001e0")
    IAccPropServer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPropValue( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [in] */ MSAAPROPID idProp,
            /* [out] */ __RPC__out VARIANT *pvarValue,
            /* [out] */ __RPC__out BOOL *pfHasProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccPropServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccPropServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccPropServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccPropServer * This);
        
        DECLSPEC_XFGVIRT(IAccPropServer, GetPropValue)
        HRESULT ( STDMETHODCALLTYPE *GetPropValue )( 
            __RPC__in IAccPropServer * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [in] */ MSAAPROPID idProp,
            /* [out] */ __RPC__out VARIANT *pvarValue,
            /* [out] */ __RPC__out BOOL *pfHasProp);
        
        END_INTERFACE
    } IAccPropServerVtbl;

    interface IAccPropServer
    {
        CONST_VTBL struct IAccPropServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccPropServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccPropServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccPropServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccPropServer_GetPropValue(This,pIDString,dwIDStringLen,idProp,pvarValue,pfHasProp)	\
    ( (This)->lpVtbl -> GetPropValue(This,pIDString,dwIDStringLen,idProp,pvarValue,pfHasProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccPropServer_INTERFACE_DEFINED__ */


#ifndef __IAccPropServices_INTERFACE_DEFINED__
#define __IAccPropServices_INTERFACE_DEFINED__

/* interface IAccPropServices */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAccPropServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6e26e776-04f0-495d-80e4-3330352e3169")
    IAccPropServices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPropValue( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [in] */ MSAAPROPID idProp,
            /* [in] */ VARIANT var) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPropServer( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps,
            /* [in] */ __RPC__in_opt IAccPropServer *pServer,
            /* [in] */ AnnoScope annoScope) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearProps( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHwndProp( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [in] */ VARIANT var) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHwndPropStr( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [string][in] */ __RPC__in_string LPCWSTR str) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHwndPropServer( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps,
            /* [in] */ __RPC__in_opt IAccPropServer *pServer,
            /* [in] */ AnnoScope annoScope) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearHwndProps( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ComposeHwndIdentityString( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwIDStringLen) BYTE **ppIDString,
            /* [out] */ __RPC__out DWORD *pdwIDStringLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DecomposeHwndIdentityString( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__out DWORD *pidObject,
            /* [out] */ __RPC__out DWORD *pidChild) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHmenuProp( 
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [in] */ VARIANT var) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHmenuPropStr( 
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [string][in] */ __RPC__in_string LPCWSTR str) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHmenuPropServer( 
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps,
            /* [in] */ __RPC__in_opt IAccPropServer *pServer,
            /* [in] */ AnnoScope annoScope) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearHmenuProps( 
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ComposeHmenuIdentityString( 
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwIDStringLen) BYTE **ppIDString,
            /* [out] */ __RPC__out DWORD *pdwIDStringLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DecomposeHmenuIdentityString( 
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [out] */ __RPC__deref_out_opt HMENU *phmenu,
            /* [out] */ __RPC__out DWORD *pidChild) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccPropServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccPropServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccPropServices * This);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetPropValue)
        HRESULT ( STDMETHODCALLTYPE *SetPropValue )( 
            __RPC__in IAccPropServices * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [in] */ MSAAPROPID idProp,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetPropServer)
        HRESULT ( STDMETHODCALLTYPE *SetPropServer )( 
            __RPC__in IAccPropServices * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps,
            /* [in] */ __RPC__in_opt IAccPropServer *pServer,
            /* [in] */ AnnoScope annoScope);
        
        DECLSPEC_XFGVIRT(IAccPropServices, ClearProps)
        HRESULT ( STDMETHODCALLTYPE *ClearProps )( 
            __RPC__in IAccPropServices * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetHwndProp)
        HRESULT ( STDMETHODCALLTYPE *SetHwndProp )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetHwndPropStr)
        HRESULT ( STDMETHODCALLTYPE *SetHwndPropStr )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [string][in] */ __RPC__in_string LPCWSTR str);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetHwndPropServer)
        HRESULT ( STDMETHODCALLTYPE *SetHwndPropServer )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps,
            /* [in] */ __RPC__in_opt IAccPropServer *pServer,
            /* [in] */ AnnoScope annoScope);
        
        DECLSPEC_XFGVIRT(IAccPropServices, ClearHwndProps)
        HRESULT ( STDMETHODCALLTYPE *ClearHwndProps )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps);
        
        DECLSPEC_XFGVIRT(IAccPropServices, ComposeHwndIdentityString)
        HRESULT ( STDMETHODCALLTYPE *ComposeHwndIdentityString )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ DWORD idObject,
            /* [in] */ DWORD idChild,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwIDStringLen) BYTE **ppIDString,
            /* [out] */ __RPC__out DWORD *pdwIDStringLen);
        
        DECLSPEC_XFGVIRT(IAccPropServices, DecomposeHwndIdentityString)
        HRESULT ( STDMETHODCALLTYPE *DecomposeHwndIdentityString )( 
            __RPC__in IAccPropServices * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd,
            /* [out] */ __RPC__out DWORD *pidObject,
            /* [out] */ __RPC__out DWORD *pidChild);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetHmenuProp)
        HRESULT ( STDMETHODCALLTYPE *SetHmenuProp )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetHmenuPropStr)
        HRESULT ( STDMETHODCALLTYPE *SetHmenuPropStr )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [in] */ MSAAPROPID idProp,
            /* [string][in] */ __RPC__in_string LPCWSTR str);
        
        DECLSPEC_XFGVIRT(IAccPropServices, SetHmenuPropServer)
        HRESULT ( STDMETHODCALLTYPE *SetHmenuPropServer )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps,
            /* [in] */ __RPC__in_opt IAccPropServer *pServer,
            /* [in] */ AnnoScope annoScope);
        
        DECLSPEC_XFGVIRT(IAccPropServices, ClearHmenuProps)
        HRESULT ( STDMETHODCALLTYPE *ClearHmenuProps )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [size_is][in] */ __RPC__in_ecount_full(cProps) const MSAAPROPID *paProps,
            /* [in] */ int cProps);
        
        DECLSPEC_XFGVIRT(IAccPropServices, ComposeHmenuIdentityString)
        HRESULT ( STDMETHODCALLTYPE *ComposeHmenuIdentityString )( 
            __RPC__in IAccPropServices * This,
            /* [in] */ __RPC__in HMENU hmenu,
            /* [in] */ DWORD idChild,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwIDStringLen) BYTE **ppIDString,
            /* [out] */ __RPC__out DWORD *pdwIDStringLen);
        
        DECLSPEC_XFGVIRT(IAccPropServices, DecomposeHmenuIdentityString)
        HRESULT ( STDMETHODCALLTYPE *DecomposeHmenuIdentityString )( 
            __RPC__in IAccPropServices * This,
            /* [size_is][in] */ __RPC__in_ecount_full(dwIDStringLen) const BYTE *pIDString,
            /* [in] */ DWORD dwIDStringLen,
            /* [out] */ __RPC__deref_out_opt HMENU *phmenu,
            /* [out] */ __RPC__out DWORD *pidChild);
        
        END_INTERFACE
    } IAccPropServicesVtbl;

    interface IAccPropServices
    {
        CONST_VTBL struct IAccPropServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccPropServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccPropServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccPropServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccPropServices_SetPropValue(This,pIDString,dwIDStringLen,idProp,var)	\
    ( (This)->lpVtbl -> SetPropValue(This,pIDString,dwIDStringLen,idProp,var) ) 

#define IAccPropServices_SetPropServer(This,pIDString,dwIDStringLen,paProps,cProps,pServer,annoScope)	\
    ( (This)->lpVtbl -> SetPropServer(This,pIDString,dwIDStringLen,paProps,cProps,pServer,annoScope) ) 

#define IAccPropServices_ClearProps(This,pIDString,dwIDStringLen,paProps,cProps)	\
    ( (This)->lpVtbl -> ClearProps(This,pIDString,dwIDStringLen,paProps,cProps) ) 

#define IAccPropServices_SetHwndProp(This,hwnd,idObject,idChild,idProp,var)	\
    ( (This)->lpVtbl -> SetHwndProp(This,hwnd,idObject,idChild,idProp,var) ) 

#define IAccPropServices_SetHwndPropStr(This,hwnd,idObject,idChild,idProp,str)	\
    ( (This)->lpVtbl -> SetHwndPropStr(This,hwnd,idObject,idChild,idProp,str) ) 

#define IAccPropServices_SetHwndPropServer(This,hwnd,idObject,idChild,paProps,cProps,pServer,annoScope)	\
    ( (This)->lpVtbl -> SetHwndPropServer(This,hwnd,idObject,idChild,paProps,cProps,pServer,annoScope) ) 

#define IAccPropServices_ClearHwndProps(This,hwnd,idObject,idChild,paProps,cProps)	\
    ( (This)->lpVtbl -> ClearHwndProps(This,hwnd,idObject,idChild,paProps,cProps) ) 

#define IAccPropServices_ComposeHwndIdentityString(This,hwnd,idObject,idChild,ppIDString,pdwIDStringLen)	\
    ( (This)->lpVtbl -> ComposeHwndIdentityString(This,hwnd,idObject,idChild,ppIDString,pdwIDStringLen) ) 

#define IAccPropServices_DecomposeHwndIdentityString(This,pIDString,dwIDStringLen,phwnd,pidObject,pidChild)	\
    ( (This)->lpVtbl -> DecomposeHwndIdentityString(This,pIDString,dwIDStringLen,phwnd,pidObject,pidChild) ) 

#define IAccPropServices_SetHmenuProp(This,hmenu,idChild,idProp,var)	\
    ( (This)->lpVtbl -> SetHmenuProp(This,hmenu,idChild,idProp,var) ) 

#define IAccPropServices_SetHmenuPropStr(This,hmenu,idChild,idProp,str)	\
    ( (This)->lpVtbl -> SetHmenuPropStr(This,hmenu,idChild,idProp,str) ) 

#define IAccPropServices_SetHmenuPropServer(This,hmenu,idChild,paProps,cProps,pServer,annoScope)	\
    ( (This)->lpVtbl -> SetHmenuPropServer(This,hmenu,idChild,paProps,cProps,pServer,annoScope) ) 

#define IAccPropServices_ClearHmenuProps(This,hmenu,idChild,paProps,cProps)	\
    ( (This)->lpVtbl -> ClearHmenuProps(This,hmenu,idChild,paProps,cProps) ) 

#define IAccPropServices_ComposeHmenuIdentityString(This,hmenu,idChild,ppIDString,pdwIDStringLen)	\
    ( (This)->lpVtbl -> ComposeHmenuIdentityString(This,hmenu,idChild,ppIDString,pdwIDStringLen) ) 

#define IAccPropServices_DecomposeHmenuIdentityString(This,pIDString,dwIDStringLen,phmenu,pidChild)	\
    ( (This)->lpVtbl -> DecomposeHmenuIdentityString(This,pIDString,dwIDStringLen,phmenu,pidChild) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccPropServices_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oleacc_0000_0006 */
/* [local] */ 


//=--------------------------------------------------------------------------=
// Type Library Definitions
//=--------------------------------------------------------------------------=



extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0006_v0_0_s_ifspec;


#ifndef __Accessibility_LIBRARY_DEFINED__
#define __Accessibility_LIBRARY_DEFINED__

/* library Accessibility */
/* [hidden][version][lcid][uuid] */ 







EXTERN_C const IID LIBID_Accessibility;

EXTERN_C const CLSID CLSID_CAccPropServices;

#ifdef __cplusplus

class DECLSPEC_UUID("b5f8350b-0548-48b1-a6ee-88bd00b4a5e7")
CAccPropServices;
#endif
#endif /* __Accessibility_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_oleacc_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oleacc_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HMENU_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * ); 
void                      __RPC_USER  HMENU_UserFree(     __RPC__in unsigned long *, __RPC__in HMENU * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


