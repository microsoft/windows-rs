

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
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

#ifndef __netcfgx_h__
#define __netcfgx_h__

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

#ifndef __IEnumNetCfgBindingInterface_FWD_DEFINED__
#define __IEnumNetCfgBindingInterface_FWD_DEFINED__
typedef interface IEnumNetCfgBindingInterface IEnumNetCfgBindingInterface;

#endif 	/* __IEnumNetCfgBindingInterface_FWD_DEFINED__ */


#ifndef __IEnumNetCfgBindingPath_FWD_DEFINED__
#define __IEnumNetCfgBindingPath_FWD_DEFINED__
typedef interface IEnumNetCfgBindingPath IEnumNetCfgBindingPath;

#endif 	/* __IEnumNetCfgBindingPath_FWD_DEFINED__ */


#ifndef __IEnumNetCfgComponent_FWD_DEFINED__
#define __IEnumNetCfgComponent_FWD_DEFINED__
typedef interface IEnumNetCfgComponent IEnumNetCfgComponent;

#endif 	/* __IEnumNetCfgComponent_FWD_DEFINED__ */


#ifndef __INetCfg_FWD_DEFINED__
#define __INetCfg_FWD_DEFINED__
typedef interface INetCfg INetCfg;

#endif 	/* __INetCfg_FWD_DEFINED__ */


#ifndef __INetCfgLock_FWD_DEFINED__
#define __INetCfgLock_FWD_DEFINED__
typedef interface INetCfgLock INetCfgLock;

#endif 	/* __INetCfgLock_FWD_DEFINED__ */


#ifndef __INetCfgBindingInterface_FWD_DEFINED__
#define __INetCfgBindingInterface_FWD_DEFINED__
typedef interface INetCfgBindingInterface INetCfgBindingInterface;

#endif 	/* __INetCfgBindingInterface_FWD_DEFINED__ */


#ifndef __INetCfgBindingPath_FWD_DEFINED__
#define __INetCfgBindingPath_FWD_DEFINED__
typedef interface INetCfgBindingPath INetCfgBindingPath;

#endif 	/* __INetCfgBindingPath_FWD_DEFINED__ */


#ifndef __INetCfgClass_FWD_DEFINED__
#define __INetCfgClass_FWD_DEFINED__
typedef interface INetCfgClass INetCfgClass;

#endif 	/* __INetCfgClass_FWD_DEFINED__ */


#ifndef __INetCfgClassSetup_FWD_DEFINED__
#define __INetCfgClassSetup_FWD_DEFINED__
typedef interface INetCfgClassSetup INetCfgClassSetup;

#endif 	/* __INetCfgClassSetup_FWD_DEFINED__ */


#ifndef __INetCfgClassSetup2_FWD_DEFINED__
#define __INetCfgClassSetup2_FWD_DEFINED__
typedef interface INetCfgClassSetup2 INetCfgClassSetup2;

#endif 	/* __INetCfgClassSetup2_FWD_DEFINED__ */


#ifndef __INetCfgComponent_FWD_DEFINED__
#define __INetCfgComponent_FWD_DEFINED__
typedef interface INetCfgComponent INetCfgComponent;

#endif 	/* __INetCfgComponent_FWD_DEFINED__ */


#ifndef __INetCfgComponentBindings_FWD_DEFINED__
#define __INetCfgComponentBindings_FWD_DEFINED__
typedef interface INetCfgComponentBindings INetCfgComponentBindings;

#endif 	/* __INetCfgComponentBindings_FWD_DEFINED__ */


#ifndef __INetCfgSysPrep_FWD_DEFINED__
#define __INetCfgSysPrep_FWD_DEFINED__
typedef interface INetCfgSysPrep INetCfgSysPrep;

#endif 	/* __INetCfgSysPrep_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "prsht.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_netcfgx_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if ( _MSC_VER >= 800 )
#pragma warning(disable:4201)
#endif

EXTERN_C const CLSID CLSID_CNetCfg;

#define NETCFG_E_ALREADY_INITIALIZED                 MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA020)
#define NETCFG_E_NOT_INITIALIZED                     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA021)
#define NETCFG_E_IN_USE                              MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA022)
#define NETCFG_E_NO_WRITE_LOCK                       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA024)
#define NETCFG_E_NEED_REBOOT                         MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA025)
#define NETCFG_E_ACTIVE_RAS_CONNECTIONS              MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA026)
#define NETCFG_E_ADAPTER_NOT_FOUND                   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA027)
#define NETCFG_E_COMPONENT_REMOVED_PENDING_REBOOT    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA028)
#define NETCFG_E_MAX_FILTER_LIMIT                    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA029)
#define NETCFG_E_VMSWITCH_ACTIVE_OVER_ADAPTER        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA02A)
#define NETCFG_E_DUPLICATE_INSTANCEID                MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA02B)
#define NETCFG_S_REBOOT                              MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0xA020)
#define NETCFG_S_DISABLE_QUERY                       MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0xA022)
#define NETCFG_S_STILL_REFERENCED                    MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0xA023)
#define NETCFG_S_CAUSED_SETUP_CHANGE                 MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0xA024)
#define NETCFG_S_COMMIT_NOW                          MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0xA025)

#define NETCFG_CLIENT_CID_MS_MSClient        TEXT("ms_msclient")
#define NETCFG_SERVICE_CID_MS_SERVER         TEXT("ms_server")
#define NETCFG_SERVICE_CID_MS_NETBIOS        TEXT("ms_netbios")
#define NETCFG_SERVICE_CID_MS_PSCHED         TEXT("ms_pschedpc")
#define NETCFG_SERVICE_CID_MS_WLBS           TEXT("ms_wlbs")
#define NETCFG_TRANS_CID_MS_APPLETALK        TEXT("ms_appletalk")
#define NETCFG_TRANS_CID_MS_NETBEUI          TEXT("ms_netbeui")
#define NETCFG_TRANS_CID_MS_NETMON           TEXT("ms_netmon")
#define NETCFG_TRANS_CID_MS_NWIPX            TEXT("ms_nwipx")
#define NETCFG_TRANS_CID_MS_NWSPX            TEXT("ms_nwspx")
#define NETCFG_TRANS_CID_MS_TCPIP            TEXT("ms_tcpip")

















extern RPC_IF_HANDLE __MIDL_itf_netcfgx_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netcfgx_0000_0000_v0_0_s_ifspec;

#ifndef __IEnumNetCfgBindingInterface_INTERFACE_DEFINED__
#define __IEnumNetCfgBindingInterface_INTERFACE_DEFINED__

/* interface IEnumNetCfgBindingInterface */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumNetCfgBindingInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE90-306E-11D1-AACF-00805FC1270E")
    IEnumNetCfgBindingInterface : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _At_(*rgelt, _Out_writes_to_(celt, *pceltFetched))  INetCfgBindingInterface **rgelt,
            /* [annotation][out] */ 
            _When_(celt==1, _Out_opt_) _When_(celt!=1, _Out_)  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Reserved_  IEnumNetCfgBindingInterface **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumNetCfgBindingInterfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumNetCfgBindingInterface * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumNetCfgBindingInterface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumNetCfgBindingInterface * This);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingInterface, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumNetCfgBindingInterface * This,
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _At_(*rgelt, _Out_writes_to_(celt, *pceltFetched))  INetCfgBindingInterface **rgelt,
            /* [annotation][out] */ 
            _When_(celt==1, _Out_opt_) _When_(celt!=1, _Out_)  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingInterface, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumNetCfgBindingInterface * This,
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingInterface, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumNetCfgBindingInterface * This);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingInterface, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumNetCfgBindingInterface * This,
            /* [annotation][out] */ 
            _Reserved_  IEnumNetCfgBindingInterface **ppenum);
        
        END_INTERFACE
    } IEnumNetCfgBindingInterfaceVtbl;

    interface IEnumNetCfgBindingInterface
    {
        CONST_VTBL struct IEnumNetCfgBindingInterfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumNetCfgBindingInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumNetCfgBindingInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumNetCfgBindingInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumNetCfgBindingInterface_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumNetCfgBindingInterface_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumNetCfgBindingInterface_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumNetCfgBindingInterface_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumNetCfgBindingInterface_INTERFACE_DEFINED__ */


#ifndef __IEnumNetCfgBindingPath_INTERFACE_DEFINED__
#define __IEnumNetCfgBindingPath_INTERFACE_DEFINED__

/* interface IEnumNetCfgBindingPath */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumNetCfgBindingPath;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE91-306E-11D1-AACF-00805FC1270E")
    IEnumNetCfgBindingPath : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _At_(*rgelt, _Out_writes_to_(celt, *pceltFetched))  INetCfgBindingPath **rgelt,
            /* [annotation][out] */ 
            _When_(celt==1, _Out_opt_) _When_(celt!=1, _Out_)  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [annotation][in] */ 
            _In_range_(>=, 1)  ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Reserved_  IEnumNetCfgBindingPath **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumNetCfgBindingPathVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumNetCfgBindingPath * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumNetCfgBindingPath * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumNetCfgBindingPath * This);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingPath, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumNetCfgBindingPath * This,
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _At_(*rgelt, _Out_writes_to_(celt, *pceltFetched))  INetCfgBindingPath **rgelt,
            /* [annotation][out] */ 
            _When_(celt==1, _Out_opt_) _When_(celt!=1, _Out_)  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingPath, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumNetCfgBindingPath * This,
            /* [annotation][in] */ 
            _In_range_(>=, 1)  ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingPath, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumNetCfgBindingPath * This);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgBindingPath, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumNetCfgBindingPath * This,
            /* [annotation][out] */ 
            _Reserved_  IEnumNetCfgBindingPath **ppenum);
        
        END_INTERFACE
    } IEnumNetCfgBindingPathVtbl;

    interface IEnumNetCfgBindingPath
    {
        CONST_VTBL struct IEnumNetCfgBindingPathVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumNetCfgBindingPath_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumNetCfgBindingPath_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumNetCfgBindingPath_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumNetCfgBindingPath_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumNetCfgBindingPath_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumNetCfgBindingPath_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumNetCfgBindingPath_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumNetCfgBindingPath_INTERFACE_DEFINED__ */


#ifndef __IEnumNetCfgComponent_INTERFACE_DEFINED__
#define __IEnumNetCfgComponent_INTERFACE_DEFINED__

/* interface IEnumNetCfgComponent */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumNetCfgComponent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE92-306E-11D1-AACF-00805FC1270E")
    IEnumNetCfgComponent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _At_(*rgelt, _Out_writes_to_(celt, *pceltFetched))  INetCfgComponent **rgelt,
            /* [annotation][out] */ 
            _When_(celt==1, _Out_opt_) _When_(celt!=1, _Out_)  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [annotation][in] */ 
            _In_range_(>=, 1)  ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Reserved_  IEnumNetCfgComponent **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumNetCfgComponentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumNetCfgComponent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumNetCfgComponent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumNetCfgComponent * This);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgComponent, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumNetCfgComponent * This,
            /* [annotation][in] */ 
            _In_range_(>=,1)  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _At_(*rgelt, _Out_writes_to_(celt, *pceltFetched))  INetCfgComponent **rgelt,
            /* [annotation][out] */ 
            _When_(celt==1, _Out_opt_) _When_(celt!=1, _Out_)  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgComponent, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumNetCfgComponent * This,
            /* [annotation][in] */ 
            _In_range_(>=, 1)  ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgComponent, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumNetCfgComponent * This);
        
        DECLSPEC_XFGVIRT(IEnumNetCfgComponent, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumNetCfgComponent * This,
            /* [annotation][out] */ 
            _Reserved_  IEnumNetCfgComponent **ppenum);
        
        END_INTERFACE
    } IEnumNetCfgComponentVtbl;

    interface IEnumNetCfgComponent
    {
        CONST_VTBL struct IEnumNetCfgComponentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumNetCfgComponent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumNetCfgComponent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumNetCfgComponent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumNetCfgComponent_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumNetCfgComponent_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumNetCfgComponent_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumNetCfgComponent_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumNetCfgComponent_INTERFACE_DEFINED__ */


#ifndef __INetCfg_INTERFACE_DEFINED__
#define __INetCfg_INTERFACE_DEFINED__

/* interface INetCfg */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfg;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE93-306E-11D1-AACF-00805FC1270E")
    INetCfg : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][in] */ 
            _Reserved_  PVOID pvReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Uninitialize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Apply( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumComponents( 
            /* [annotation][in] */ 
            _In_  const GUID *pguidClass,
            /* [annotation][out] */ 
            _At_(*ppenumComponent, _Out_opt_)  IEnumNetCfgComponent **ppenumComponent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindComponent( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][out] */ 
            _At_(*pComponent, _Out_opt_)  INetCfgComponent **pComponent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryNetCfgClass( 
            /* [annotation][in] */ 
            _In_  const GUID *pguidClass,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _At_(*ppvObject, _Out_opt_)  void **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfg * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfg * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfg * This);
        
        DECLSPEC_XFGVIRT(INetCfg, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            INetCfg * This,
            /* [annotation][in] */ 
            _Reserved_  PVOID pvReserved);
        
        DECLSPEC_XFGVIRT(INetCfg, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            INetCfg * This);
        
        DECLSPEC_XFGVIRT(INetCfg, Apply)
        HRESULT ( STDMETHODCALLTYPE *Apply )( 
            INetCfg * This);
        
        DECLSPEC_XFGVIRT(INetCfg, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            INetCfg * This);
        
        DECLSPEC_XFGVIRT(INetCfg, EnumComponents)
        HRESULT ( STDMETHODCALLTYPE *EnumComponents )( 
            INetCfg * This,
            /* [annotation][in] */ 
            _In_  const GUID *pguidClass,
            /* [annotation][out] */ 
            _At_(*ppenumComponent, _Out_opt_)  IEnumNetCfgComponent **ppenumComponent);
        
        DECLSPEC_XFGVIRT(INetCfg, FindComponent)
        HRESULT ( STDMETHODCALLTYPE *FindComponent )( 
            INetCfg * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][out] */ 
            _At_(*pComponent, _Out_opt_)  INetCfgComponent **pComponent);
        
        DECLSPEC_XFGVIRT(INetCfg, QueryNetCfgClass)
        HRESULT ( STDMETHODCALLTYPE *QueryNetCfgClass )( 
            INetCfg * This,
            /* [annotation][in] */ 
            _In_  const GUID *pguidClass,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _At_(*ppvObject, _Out_opt_)  void **ppvObject);
        
        END_INTERFACE
    } INetCfgVtbl;

    interface INetCfg
    {
        CONST_VTBL struct INetCfgVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfg_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfg_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfg_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfg_Initialize(This,pvReserved)	\
    ( (This)->lpVtbl -> Initialize(This,pvReserved) ) 

#define INetCfg_Uninitialize(This)	\
    ( (This)->lpVtbl -> Uninitialize(This) ) 

#define INetCfg_Apply(This)	\
    ( (This)->lpVtbl -> Apply(This) ) 

#define INetCfg_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define INetCfg_EnumComponents(This,pguidClass,ppenumComponent)	\
    ( (This)->lpVtbl -> EnumComponents(This,pguidClass,ppenumComponent) ) 

#define INetCfg_FindComponent(This,pszwInfId,pComponent)	\
    ( (This)->lpVtbl -> FindComponent(This,pszwInfId,pComponent) ) 

#define INetCfg_QueryNetCfgClass(This,pguidClass,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryNetCfgClass(This,pguidClass,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfg_INTERFACE_DEFINED__ */


#ifndef __INetCfgLock_INTERFACE_DEFINED__
#define __INetCfgLock_INTERFACE_DEFINED__

/* interface INetCfgLock */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgLock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE9F-306E-11D1-AACF-00805FC1270E")
    INetCfgLock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AcquireWriteLock( 
            /* [annotation][in] */ 
            _In_  DWORD cmsTimeout,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwClientDescription,
            /* [annotation][string][out] */ 
            _At_(*ppszwClientDescription, _Out_opt_)  LPWSTR *ppszwClientDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseWriteLock( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsWriteLocked( 
            /* [annotation][string][out] */ 
            _At_(*ppszwClientDescription, _Out_opt_)  LPWSTR *ppszwClientDescription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgLockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgLock * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgLock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgLock * This);
        
        DECLSPEC_XFGVIRT(INetCfgLock, AcquireWriteLock)
        HRESULT ( STDMETHODCALLTYPE *AcquireWriteLock )( 
            INetCfgLock * This,
            /* [annotation][in] */ 
            _In_  DWORD cmsTimeout,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwClientDescription,
            /* [annotation][string][out] */ 
            _At_(*ppszwClientDescription, _Out_opt_)  LPWSTR *ppszwClientDescription);
        
        DECLSPEC_XFGVIRT(INetCfgLock, ReleaseWriteLock)
        HRESULT ( STDMETHODCALLTYPE *ReleaseWriteLock )( 
            INetCfgLock * This);
        
        DECLSPEC_XFGVIRT(INetCfgLock, IsWriteLocked)
        HRESULT ( STDMETHODCALLTYPE *IsWriteLocked )( 
            INetCfgLock * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwClientDescription, _Out_opt_)  LPWSTR *ppszwClientDescription);
        
        END_INTERFACE
    } INetCfgLockVtbl;

    interface INetCfgLock
    {
        CONST_VTBL struct INetCfgLockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgLock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgLock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgLock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgLock_AcquireWriteLock(This,cmsTimeout,pszwClientDescription,ppszwClientDescription)	\
    ( (This)->lpVtbl -> AcquireWriteLock(This,cmsTimeout,pszwClientDescription,ppszwClientDescription) ) 

#define INetCfgLock_ReleaseWriteLock(This)	\
    ( (This)->lpVtbl -> ReleaseWriteLock(This) ) 

#define INetCfgLock_IsWriteLocked(This,ppszwClientDescription)	\
    ( (This)->lpVtbl -> IsWriteLocked(This,ppszwClientDescription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgLock_INTERFACE_DEFINED__ */


#ifndef __INetCfgBindingInterface_INTERFACE_DEFINED__
#define __INetCfgBindingInterface_INTERFACE_DEFINED__

/* interface INetCfgBindingInterface */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgBindingInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE94-306E-11D1-AACF-00805FC1270E")
    INetCfgBindingInterface : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [annotation][string][out] */ 
            _At_(*ppszwInterfaceName, _Out_opt_)  LPWSTR *ppszwInterfaceName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUpperComponent( 
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLowerComponent( 
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgBindingInterfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgBindingInterface * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgBindingInterface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgBindingInterface * This);
        
        DECLSPEC_XFGVIRT(INetCfgBindingInterface, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            INetCfgBindingInterface * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwInterfaceName, _Out_opt_)  LPWSTR *ppszwInterfaceName);
        
        DECLSPEC_XFGVIRT(INetCfgBindingInterface, GetUpperComponent)
        HRESULT ( STDMETHODCALLTYPE *GetUpperComponent )( 
            INetCfgBindingInterface * This,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgBindingInterface, GetLowerComponent)
        HRESULT ( STDMETHODCALLTYPE *GetLowerComponent )( 
            INetCfgBindingInterface * This,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        END_INTERFACE
    } INetCfgBindingInterfaceVtbl;

    interface INetCfgBindingInterface
    {
        CONST_VTBL struct INetCfgBindingInterfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgBindingInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgBindingInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgBindingInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgBindingInterface_GetName(This,ppszwInterfaceName)	\
    ( (This)->lpVtbl -> GetName(This,ppszwInterfaceName) ) 

#define INetCfgBindingInterface_GetUpperComponent(This,ppnccItem)	\
    ( (This)->lpVtbl -> GetUpperComponent(This,ppnccItem) ) 

#define INetCfgBindingInterface_GetLowerComponent(This,ppnccItem)	\
    ( (This)->lpVtbl -> GetLowerComponent(This,ppnccItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgBindingInterface_INTERFACE_DEFINED__ */


#ifndef __INetCfgBindingPath_INTERFACE_DEFINED__
#define __INetCfgBindingPath_INTERFACE_DEFINED__

/* interface INetCfgBindingPath */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgBindingPath;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE96-306E-11D1-AACF-00805FC1270E")
    INetCfgBindingPath : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsSamePathAs( 
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSubPathOf( 
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEnabled( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enable( 
            /* [annotation][in] */ 
            _In_  BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPathToken( 
            /* [annotation][string][out] */ 
            _At_(*ppszwPathToken, _Out_opt_)  LPWSTR *ppszwPathToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [annotation][out] */ 
            _At_(*ppComponent, _Out_opt_)  INetCfgComponent **ppComponent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDepth( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcInterfaces) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumBindingInterfaces( 
            /* [annotation][out] */ 
            _At_(*ppenumInterface, _Out_opt_)  IEnumNetCfgBindingInterface **ppenumInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgBindingPathVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgBindingPath * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgBindingPath * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgBindingPath * This);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, IsSamePathAs)
        HRESULT ( STDMETHODCALLTYPE *IsSamePathAs )( 
            INetCfgBindingPath * This,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pPath);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, IsSubPathOf)
        HRESULT ( STDMETHODCALLTYPE *IsSubPathOf )( 
            INetCfgBindingPath * This,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pPath);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, IsEnabled)
        HRESULT ( STDMETHODCALLTYPE *IsEnabled )( 
            INetCfgBindingPath * This);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            INetCfgBindingPath * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnable);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, GetPathToken)
        HRESULT ( STDMETHODCALLTYPE *GetPathToken )( 
            INetCfgBindingPath * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwPathToken, _Out_opt_)  LPWSTR *ppszwPathToken);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            INetCfgBindingPath * This,
            /* [annotation][out] */ 
            _At_(*ppComponent, _Out_opt_)  INetCfgComponent **ppComponent);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, GetDepth)
        HRESULT ( STDMETHODCALLTYPE *GetDepth )( 
            INetCfgBindingPath * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcInterfaces);
        
        DECLSPEC_XFGVIRT(INetCfgBindingPath, EnumBindingInterfaces)
        HRESULT ( STDMETHODCALLTYPE *EnumBindingInterfaces )( 
            INetCfgBindingPath * This,
            /* [annotation][out] */ 
            _At_(*ppenumInterface, _Out_opt_)  IEnumNetCfgBindingInterface **ppenumInterface);
        
        END_INTERFACE
    } INetCfgBindingPathVtbl;

    interface INetCfgBindingPath
    {
        CONST_VTBL struct INetCfgBindingPathVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgBindingPath_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgBindingPath_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgBindingPath_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgBindingPath_IsSamePathAs(This,pPath)	\
    ( (This)->lpVtbl -> IsSamePathAs(This,pPath) ) 

#define INetCfgBindingPath_IsSubPathOf(This,pPath)	\
    ( (This)->lpVtbl -> IsSubPathOf(This,pPath) ) 

#define INetCfgBindingPath_IsEnabled(This)	\
    ( (This)->lpVtbl -> IsEnabled(This) ) 

#define INetCfgBindingPath_Enable(This,fEnable)	\
    ( (This)->lpVtbl -> Enable(This,fEnable) ) 

#define INetCfgBindingPath_GetPathToken(This,ppszwPathToken)	\
    ( (This)->lpVtbl -> GetPathToken(This,ppszwPathToken) ) 

#define INetCfgBindingPath_GetOwner(This,ppComponent)	\
    ( (This)->lpVtbl -> GetOwner(This,ppComponent) ) 

#define INetCfgBindingPath_GetDepth(This,pcInterfaces)	\
    ( (This)->lpVtbl -> GetDepth(This,pcInterfaces) ) 

#define INetCfgBindingPath_EnumBindingInterfaces(This,ppenumInterface)	\
    ( (This)->lpVtbl -> EnumBindingInterfaces(This,ppenumInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgBindingPath_INTERFACE_DEFINED__ */


#ifndef __INetCfgClass_INTERFACE_DEFINED__
#define __INetCfgClass_INTERFACE_DEFINED__

/* interface INetCfgClass */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgClass;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE97-306E-11D1-AACF-00805FC1270E")
    INetCfgClass : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindComponent( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumComponents( 
            /* [annotation][out] */ 
            _At_(*ppenumComponent, _Out_opt_)  IEnumNetCfgComponent **ppenumComponent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgClassVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgClass * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgClass * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgClass * This);
        
        DECLSPEC_XFGVIRT(INetCfgClass, FindComponent)
        HRESULT ( STDMETHODCALLTYPE *FindComponent )( 
            INetCfgClass * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgClass, EnumComponents)
        HRESULT ( STDMETHODCALLTYPE *EnumComponents )( 
            INetCfgClass * This,
            /* [annotation][out] */ 
            _At_(*ppenumComponent, _Out_opt_)  IEnumNetCfgComponent **ppenumComponent);
        
        END_INTERFACE
    } INetCfgClassVtbl;

    interface INetCfgClass
    {
        CONST_VTBL struct INetCfgClassVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgClass_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgClass_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgClass_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgClass_FindComponent(This,pszwInfId,ppnccItem)	\
    ( (This)->lpVtbl -> FindComponent(This,pszwInfId,ppnccItem) ) 

#define INetCfgClass_EnumComponents(This,ppenumComponent)	\
    ( (This)->lpVtbl -> EnumComponents(This,ppenumComponent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgClass_INTERFACE_DEFINED__ */


#ifndef __INetCfgClassSetup_INTERFACE_DEFINED__
#define __INetCfgClassSetup_INTERFACE_DEFINED__

/* interface INetCfgClassSetup */
/* [unique][uuid][object][local] */ 

typedef 
enum tagOBO_TOKEN_TYPE
    {
        OBO_USER	= 1,
        OBO_COMPONENT	= 2,
        OBO_SOFTWARE	= 3
    } 	OBO_TOKEN_TYPE;

typedef struct tagOBO_TOKEN
    {
    OBO_TOKEN_TYPE Type;
    INetCfgComponent *pncc;
    LPCWSTR pszwManufacturer;
    LPCWSTR pszwProduct;
    LPCWSTR pszwDisplayName;
    BOOL fRegistered;
    } 	OBO_TOKEN;


EXTERN_C const IID IID_INetCfgClassSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE9D-306E-11D1-AACF-00805FC1270E")
    INetCfgClassSetup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SelectAndInstall( 
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Install( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][in] */ 
            _In_opt_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _In_opt_  DWORD dwUpgradeFromBuildNo,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pszwAnswerFile,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pszwAnswerSections,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeInstall( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pComponent,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][out] */ 
            _Inout_opt_ _At_(*pmszwRefs, _Out_opt_)  LPWSTR *pmszwRefs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgClassSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgClassSetup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgClassSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgClassSetup * This);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup, SelectAndInstall)
        HRESULT ( STDMETHODCALLTYPE *SelectAndInstall )( 
            INetCfgClassSetup * This,
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            INetCfgClassSetup * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][in] */ 
            _In_opt_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _In_opt_  DWORD dwUpgradeFromBuildNo,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pszwAnswerFile,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pszwAnswerSections,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup, DeInstall)
        HRESULT ( STDMETHODCALLTYPE *DeInstall )( 
            INetCfgClassSetup * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pComponent,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][out] */ 
            _Inout_opt_ _At_(*pmszwRefs, _Out_opt_)  LPWSTR *pmszwRefs);
        
        END_INTERFACE
    } INetCfgClassSetupVtbl;

    interface INetCfgClassSetup
    {
        CONST_VTBL struct INetCfgClassSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgClassSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgClassSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgClassSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgClassSetup_SelectAndInstall(This,hwndParent,pOboToken,ppnccItem)	\
    ( (This)->lpVtbl -> SelectAndInstall(This,hwndParent,pOboToken,ppnccItem) ) 

#define INetCfgClassSetup_Install(This,pszwInfId,pOboToken,dwSetupFlags,dwUpgradeFromBuildNo,pszwAnswerFile,pszwAnswerSections,ppnccItem)	\
    ( (This)->lpVtbl -> Install(This,pszwInfId,pOboToken,dwSetupFlags,dwUpgradeFromBuildNo,pszwAnswerFile,pszwAnswerSections,ppnccItem) ) 

#define INetCfgClassSetup_DeInstall(This,pComponent,pOboToken,pmszwRefs)	\
    ( (This)->lpVtbl -> DeInstall(This,pComponent,pOboToken,pmszwRefs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgClassSetup_INTERFACE_DEFINED__ */


#ifndef __INetCfgClassSetup2_INTERFACE_DEFINED__
#define __INetCfgClassSetup2_INTERFACE_DEFINED__

/* interface INetCfgClassSetup2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgClassSetup2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AEA0-306E-11D1-AACF-00805FC1270E")
    INetCfgClassSetup2 : public INetCfgClassSetup
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UpdateNonEnumeratedComponent( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pIComp,
            /* [annotation][in] */ 
            _Reserved_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _Reserved_  DWORD dwUpgradeFromBuildNo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgClassSetup2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgClassSetup2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgClassSetup2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgClassSetup2 * This);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup, SelectAndInstall)
        HRESULT ( STDMETHODCALLTYPE *SelectAndInstall )( 
            INetCfgClassSetup2 * This,
            /* [annotation][in] */ 
            _In_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup, Install)
        HRESULT ( STDMETHODCALLTYPE *Install )( 
            INetCfgClassSetup2 * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwInfId,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][in] */ 
            _In_opt_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _In_opt_  DWORD dwUpgradeFromBuildNo,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pszwAnswerFile,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pszwAnswerSections,
            /* [annotation][out] */ 
            _At_(*ppnccItem, _Out_opt_)  INetCfgComponent **ppnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup, DeInstall)
        HRESULT ( STDMETHODCALLTYPE *DeInstall )( 
            INetCfgClassSetup2 * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pComponent,
            /* [annotation][in] */ 
            _In_opt_  OBO_TOKEN *pOboToken,
            /* [annotation][out] */ 
            _Inout_opt_ _At_(*pmszwRefs, _Out_opt_)  LPWSTR *pmszwRefs);
        
        DECLSPEC_XFGVIRT(INetCfgClassSetup2, UpdateNonEnumeratedComponent)
        HRESULT ( STDMETHODCALLTYPE *UpdateNonEnumeratedComponent )( 
            INetCfgClassSetup2 * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pIComp,
            /* [annotation][in] */ 
            _Reserved_  DWORD dwSetupFlags,
            /* [annotation][in] */ 
            _Reserved_  DWORD dwUpgradeFromBuildNo);
        
        END_INTERFACE
    } INetCfgClassSetup2Vtbl;

    interface INetCfgClassSetup2
    {
        CONST_VTBL struct INetCfgClassSetup2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgClassSetup2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgClassSetup2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgClassSetup2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgClassSetup2_SelectAndInstall(This,hwndParent,pOboToken,ppnccItem)	\
    ( (This)->lpVtbl -> SelectAndInstall(This,hwndParent,pOboToken,ppnccItem) ) 

#define INetCfgClassSetup2_Install(This,pszwInfId,pOboToken,dwSetupFlags,dwUpgradeFromBuildNo,pszwAnswerFile,pszwAnswerSections,ppnccItem)	\
    ( (This)->lpVtbl -> Install(This,pszwInfId,pOboToken,dwSetupFlags,dwUpgradeFromBuildNo,pszwAnswerFile,pszwAnswerSections,ppnccItem) ) 

#define INetCfgClassSetup2_DeInstall(This,pComponent,pOboToken,pmszwRefs)	\
    ( (This)->lpVtbl -> DeInstall(This,pComponent,pOboToken,pmszwRefs) ) 


#define INetCfgClassSetup2_UpdateNonEnumeratedComponent(This,pIComp,dwSetupFlags,dwUpgradeFromBuildNo)	\
    ( (This)->lpVtbl -> UpdateNonEnumeratedComponent(This,pIComp,dwSetupFlags,dwUpgradeFromBuildNo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgClassSetup2_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponent_INTERFACE_DEFINED__
#define __INetCfgComponent_INTERFACE_DEFINED__

/* interface INetCfgComponent */
/* [unique][uuid][object][local] */ 

typedef 
enum tagCOMPONENT_CHARACTERISTICS
    {
        NCF_VIRTUAL	= 0x1,
        NCF_SOFTWARE_ENUMERATED	= 0x2,
        NCF_PHYSICAL	= 0x4,
        NCF_HIDDEN	= 0x8,
        NCF_NO_SERVICE	= 0x10,
        NCF_NOT_USER_REMOVABLE	= 0x20,
        NCF_MULTIPORT_INSTANCED_ADAPTER	= 0x40,
        NCF_HAS_UI	= 0x80,
        NCF_SINGLE_INSTANCE	= 0x100,
        NCF_FILTER	= 0x400,
        NCF_DONTEXPOSELOWER	= 0x1000,
        NCF_HIDE_BINDING	= 0x2000,
        NCF_NDIS_PROTOCOL	= 0x4000,
        NCF_FIXED_BINDING	= 0x20000,
        NCF_LW_FILTER	= 0x40000
    } 	COMPONENT_CHARACTERISTICS;

typedef 
enum tagNCRP_FLAGS
    {
        NCRP_QUERY_PROPERTY_UI	= 0x1,
        NCRP_SHOW_PROPERTY_UI	= 0x2
    } 	NCRP_FLAGS;


EXTERN_C const IID IID_INetCfgComponent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE99-306E-11D1-AACF-00805FC1270E")
    INetCfgComponent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [annotation][string][out] */ 
            _At_(*ppszwDisplayName, _Out_opt_)  LPWSTR *ppszwDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDisplayName( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelpText( 
            /* [annotation][string][out] */ 
            _At_(*pszwHelpText, _Out_opt_)  LPWSTR *pszwHelpText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetId( 
            /* [annotation][string][out] */ 
            _At_(*ppszwId, _Out_opt_)  LPWSTR *ppszwId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCharacteristics( 
            /* [annotation][out] */ 
            _Out_  LPDWORD pdwCharacteristics) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstanceGuid( 
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPnpDevNodeId( 
            /* [annotation][string][out] */ 
            _At_(*ppszwDevNodeId, _Out_opt_)  LPWSTR *ppszwDevNodeId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClassGuid( 
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBindName( 
            /* [annotation][string][out] */ 
            _At_(*ppszwBindName, _Out_opt_)  LPWSTR *ppszwBindName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceStatus( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenParamKey( 
            /* [annotation][out] */ 
            _At_(*phkey, _Out_opt_)  HKEY *phkey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RaisePropertyUi( 
            /* [annotation][in] */ 
            _In_opt_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponent * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            INetCfgComponent * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwDisplayName, _Out_opt_)  LPWSTR *ppszwDisplayName);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            INetCfgComponent * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pszwDisplayName);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetHelpText)
        HRESULT ( STDMETHODCALLTYPE *GetHelpText )( 
            INetCfgComponent * This,
            /* [annotation][string][out] */ 
            _At_(*pszwHelpText, _Out_opt_)  LPWSTR *pszwHelpText);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetId)
        HRESULT ( STDMETHODCALLTYPE *GetId )( 
            INetCfgComponent * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwId, _Out_opt_)  LPWSTR *ppszwId);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetCharacteristics )( 
            INetCfgComponent * This,
            /* [annotation][out] */ 
            _Out_  LPDWORD pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetInstanceGuid)
        HRESULT ( STDMETHODCALLTYPE *GetInstanceGuid )( 
            INetCfgComponent * This,
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuid);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetPnpDevNodeId)
        HRESULT ( STDMETHODCALLTYPE *GetPnpDevNodeId )( 
            INetCfgComponent * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwDevNodeId, _Out_opt_)  LPWSTR *ppszwDevNodeId);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetClassGuid)
        HRESULT ( STDMETHODCALLTYPE *GetClassGuid )( 
            INetCfgComponent * This,
            /* [annotation][out] */ 
            _Out_opt_  GUID *pGuid);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetBindName)
        HRESULT ( STDMETHODCALLTYPE *GetBindName )( 
            INetCfgComponent * This,
            /* [annotation][string][out] */ 
            _At_(*ppszwBindName, _Out_opt_)  LPWSTR *ppszwBindName);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, GetDeviceStatus)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceStatus )( 
            INetCfgComponent * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, OpenParamKey)
        HRESULT ( STDMETHODCALLTYPE *OpenParamKey )( 
            INetCfgComponent * This,
            /* [annotation][out] */ 
            _At_(*phkey, _Out_opt_)  HKEY *phkey);
        
        DECLSPEC_XFGVIRT(INetCfgComponent, RaisePropertyUi)
        HRESULT ( STDMETHODCALLTYPE *RaisePropertyUi )( 
            INetCfgComponent * This,
            /* [annotation][in] */ 
            _In_opt_  HWND hwndParent,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkContext);
        
        END_INTERFACE
    } INetCfgComponentVtbl;

    interface INetCfgComponent
    {
        CONST_VTBL struct INetCfgComponentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponent_GetDisplayName(This,ppszwDisplayName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,ppszwDisplayName) ) 

#define INetCfgComponent_SetDisplayName(This,pszwDisplayName)	\
    ( (This)->lpVtbl -> SetDisplayName(This,pszwDisplayName) ) 

#define INetCfgComponent_GetHelpText(This,pszwHelpText)	\
    ( (This)->lpVtbl -> GetHelpText(This,pszwHelpText) ) 

#define INetCfgComponent_GetId(This,ppszwId)	\
    ( (This)->lpVtbl -> GetId(This,ppszwId) ) 

#define INetCfgComponent_GetCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetCharacteristics(This,pdwCharacteristics) ) 

#define INetCfgComponent_GetInstanceGuid(This,pGuid)	\
    ( (This)->lpVtbl -> GetInstanceGuid(This,pGuid) ) 

#define INetCfgComponent_GetPnpDevNodeId(This,ppszwDevNodeId)	\
    ( (This)->lpVtbl -> GetPnpDevNodeId(This,ppszwDevNodeId) ) 

#define INetCfgComponent_GetClassGuid(This,pGuid)	\
    ( (This)->lpVtbl -> GetClassGuid(This,pGuid) ) 

#define INetCfgComponent_GetBindName(This,ppszwBindName)	\
    ( (This)->lpVtbl -> GetBindName(This,ppszwBindName) ) 

#define INetCfgComponent_GetDeviceStatus(This,pulStatus)	\
    ( (This)->lpVtbl -> GetDeviceStatus(This,pulStatus) ) 

#define INetCfgComponent_OpenParamKey(This,phkey)	\
    ( (This)->lpVtbl -> OpenParamKey(This,phkey) ) 

#define INetCfgComponent_RaisePropertyUi(This,hwndParent,dwFlags,punkContext)	\
    ( (This)->lpVtbl -> RaisePropertyUi(This,hwndParent,dwFlags,punkContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponent_INTERFACE_DEFINED__ */


#ifndef __INetCfgComponentBindings_INTERFACE_DEFINED__
#define __INetCfgComponentBindings_INTERFACE_DEFINED__

/* interface INetCfgComponentBindings */
/* [unique][uuid][object][local] */ 

typedef 
enum tagSUPPORTS_BINDING_INTERFACE_FLAGS
    {
        NCF_LOWER	= 0x1,
        NCF_UPPER	= 0x2
    } 	SUPPORTS_BINDING_INTERFACE_FLAGS;

typedef 
enum tagENUM_BINDING_PATHS_FLAGS
    {
        EBP_ABOVE	= 0x1,
        EBP_BELOW	= 0x2
    } 	ENUM_BINDING_PATHS_FLAGS;


EXTERN_C const IID IID_INetCfgComponentBindings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE9E-306E-11D1-AACF-00805FC1270E")
    INetCfgComponentBindings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BindTo( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnbindFrom( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SupportsBindingInterface( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwInterfaceName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsBoundTo( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsBindableTo( 
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumBindingPaths( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][out] */ 
            _At_(*ppIEnum, _Out_opt_)  IEnumNetCfgBindingPath **ppIEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveBefore( 
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pncbItemSrc,
            /* [annotation][in] */ 
            _In_opt_  INetCfgBindingPath *pncbItemDest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveAfter( 
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pncbItemSrc,
            /* [annotation][in] */ 
            _In_opt_  INetCfgBindingPath *pncbItemDest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgComponentBindingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgComponentBindings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgComponentBindings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgComponentBindings * This);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, BindTo)
        HRESULT ( STDMETHODCALLTYPE *BindTo )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, UnbindFrom)
        HRESULT ( STDMETHODCALLTYPE *UnbindFrom )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, SupportsBindingInterface)
        HRESULT ( STDMETHODCALLTYPE *SupportsBindingInterface )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszwInterfaceName);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, IsBoundTo)
        HRESULT ( STDMETHODCALLTYPE *IsBoundTo )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, IsBindableTo)
        HRESULT ( STDMETHODCALLTYPE *IsBindableTo )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  INetCfgComponent *pnccItem);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, EnumBindingPaths)
        HRESULT ( STDMETHODCALLTYPE *EnumBindingPaths )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][out] */ 
            _At_(*ppIEnum, _Out_opt_)  IEnumNetCfgBindingPath **ppIEnum);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, MoveBefore)
        HRESULT ( STDMETHODCALLTYPE *MoveBefore )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pncbItemSrc,
            /* [annotation][in] */ 
            _In_opt_  INetCfgBindingPath *pncbItemDest);
        
        DECLSPEC_XFGVIRT(INetCfgComponentBindings, MoveAfter)
        HRESULT ( STDMETHODCALLTYPE *MoveAfter )( 
            INetCfgComponentBindings * This,
            /* [annotation][in] */ 
            _In_  INetCfgBindingPath *pncbItemSrc,
            /* [annotation][in] */ 
            _In_opt_  INetCfgBindingPath *pncbItemDest);
        
        END_INTERFACE
    } INetCfgComponentBindingsVtbl;

    interface INetCfgComponentBindings
    {
        CONST_VTBL struct INetCfgComponentBindingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgComponentBindings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgComponentBindings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgComponentBindings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgComponentBindings_BindTo(This,pnccItem)	\
    ( (This)->lpVtbl -> BindTo(This,pnccItem) ) 

#define INetCfgComponentBindings_UnbindFrom(This,pnccItem)	\
    ( (This)->lpVtbl -> UnbindFrom(This,pnccItem) ) 

#define INetCfgComponentBindings_SupportsBindingInterface(This,dwFlags,pszwInterfaceName)	\
    ( (This)->lpVtbl -> SupportsBindingInterface(This,dwFlags,pszwInterfaceName) ) 

#define INetCfgComponentBindings_IsBoundTo(This,pnccItem)	\
    ( (This)->lpVtbl -> IsBoundTo(This,pnccItem) ) 

#define INetCfgComponentBindings_IsBindableTo(This,pnccItem)	\
    ( (This)->lpVtbl -> IsBindableTo(This,pnccItem) ) 

#define INetCfgComponentBindings_EnumBindingPaths(This,dwFlags,ppIEnum)	\
    ( (This)->lpVtbl -> EnumBindingPaths(This,dwFlags,ppIEnum) ) 

#define INetCfgComponentBindings_MoveBefore(This,pncbItemSrc,pncbItemDest)	\
    ( (This)->lpVtbl -> MoveBefore(This,pncbItemSrc,pncbItemDest) ) 

#define INetCfgComponentBindings_MoveAfter(This,pncbItemSrc,pncbItemDest)	\
    ( (This)->lpVtbl -> MoveAfter(This,pncbItemSrc,pncbItemDest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgComponentBindings_INTERFACE_DEFINED__ */


#ifndef __INetCfgSysPrep_INTERFACE_DEFINED__
#define __INetCfgSysPrep_INTERFACE_DEFINED__

/* interface INetCfgSysPrep */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_INetCfgSysPrep;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0E8AE98-306E-11D1-AACF-00805FC1270E")
    INetCfgSysPrep : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HrSetupSetFirstDword( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [in] */ DWORD dwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HrSetupSetFirstString( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HrSetupSetFirstStringAsBool( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [in] */ BOOL fValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HrSetupSetFirstMultiSzField( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pmszValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INetCfgSysPrepVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INetCfgSysPrep * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INetCfgSysPrep * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INetCfgSysPrep * This);
        
        DECLSPEC_XFGVIRT(INetCfgSysPrep, HrSetupSetFirstDword)
        HRESULT ( STDMETHODCALLTYPE *HrSetupSetFirstDword )( 
            INetCfgSysPrep * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [in] */ DWORD dwValue);
        
        DECLSPEC_XFGVIRT(INetCfgSysPrep, HrSetupSetFirstString)
        HRESULT ( STDMETHODCALLTYPE *HrSetupSetFirstString )( 
            INetCfgSysPrep * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszValue);
        
        DECLSPEC_XFGVIRT(INetCfgSysPrep, HrSetupSetFirstStringAsBool)
        HRESULT ( STDMETHODCALLTYPE *HrSetupSetFirstStringAsBool )( 
            INetCfgSysPrep * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [in] */ BOOL fValue);
        
        DECLSPEC_XFGVIRT(INetCfgSysPrep, HrSetupSetFirstMultiSzField)
        HRESULT ( STDMETHODCALLTYPE *HrSetupSetFirstMultiSzField )( 
            INetCfgSysPrep * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszSection,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pwszKey,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR pmszValue);
        
        END_INTERFACE
    } INetCfgSysPrepVtbl;

    interface INetCfgSysPrep
    {
        CONST_VTBL struct INetCfgSysPrepVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INetCfgSysPrep_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INetCfgSysPrep_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INetCfgSysPrep_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INetCfgSysPrep_HrSetupSetFirstDword(This,pwszSection,pwszKey,dwValue)	\
    ( (This)->lpVtbl -> HrSetupSetFirstDword(This,pwszSection,pwszKey,dwValue) ) 

#define INetCfgSysPrep_HrSetupSetFirstString(This,pwszSection,pwszKey,pwszValue)	\
    ( (This)->lpVtbl -> HrSetupSetFirstString(This,pwszSection,pwszKey,pwszValue) ) 

#define INetCfgSysPrep_HrSetupSetFirstStringAsBool(This,pwszSection,pwszKey,fValue)	\
    ( (This)->lpVtbl -> HrSetupSetFirstStringAsBool(This,pwszSection,pwszKey,fValue) ) 

#define INetCfgSysPrep_HrSetupSetFirstMultiSzField(This,pwszSection,pwszKey,pmszValue)	\
    ( (This)->lpVtbl -> HrSetupSetFirstMultiSzField(This,pwszSection,pwszKey,pmszValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INetCfgSysPrep_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_netcfgx_0000_0013 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_netcfgx_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netcfgx_0000_0013_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


