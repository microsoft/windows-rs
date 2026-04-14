

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

#ifndef __msdasql_interfaces_h__
#define __msdasql_interfaces_h__

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

#ifndef __IRowsetChangeExtInfo_FWD_DEFINED__
#define __IRowsetChangeExtInfo_FWD_DEFINED__
typedef interface IRowsetChangeExtInfo IRowsetChangeExtInfo;

#endif 	/* __IRowsetChangeExtInfo_FWD_DEFINED__ */


#ifndef __ISQLRequestDiagFields_FWD_DEFINED__
#define __ISQLRequestDiagFields_FWD_DEFINED__
typedef interface ISQLRequestDiagFields ISQLRequestDiagFields;

#endif 	/* __ISQLRequestDiagFields_FWD_DEFINED__ */


#ifndef __ISQLGetDiagField_FWD_DEFINED__
#define __ISQLGetDiagField_FWD_DEFINED__
typedef interface ISQLGetDiagField ISQLGetDiagField;

#endif 	/* __ISQLGetDiagField_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"
#include "oledb.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msdasql_interfaces_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

enum KAGREQDIAGFLAGSENUM
    {
        KAGREQDIAGFLAGS_HEADER	= 0x1,
        KAGREQDIAGFLAGS_RECORD	= 0x2
    } ;


extern RPC_IF_HANDLE __MIDL_itf_msdasql_interfaces_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msdasql_interfaces_0000_0000_v0_0_s_ifspec;

#ifndef __IRowsetChangeExtInfo_INTERFACE_DEFINED__
#define __IRowsetChangeExtInfo_INTERFACE_DEFINED__

/* interface IRowsetChangeExtInfo */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IRowsetChangeExtInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0c733a8f-2a1c-11ce-ade5-00aa0044773d")
    IRowsetChangeExtInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOriginalRow( 
            /* [in] */ HCHAPTER hReserved,
            /* [in] */ HROW hRow,
            /* [out] */ HROW *phRowOriginal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPendingColumns( 
            /* [in] */ HCHAPTER hReserved,
            /* [in] */ HROW hRow,
            /* [in] */ ULONG cColumnOrdinals,
            /* [size_is][in] */ const ULONG rgiOrdinals[  ],
            /* [size_is][out] */ DBPENDINGSTATUS rgColumnStatus[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRowsetChangeExtInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRowsetChangeExtInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRowsetChangeExtInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRowsetChangeExtInfo * This);
        
        DECLSPEC_XFGVIRT(IRowsetChangeExtInfo, GetOriginalRow)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalRow )( 
            IRowsetChangeExtInfo * This,
            /* [in] */ HCHAPTER hReserved,
            /* [in] */ HROW hRow,
            /* [out] */ HROW *phRowOriginal);
        
        DECLSPEC_XFGVIRT(IRowsetChangeExtInfo, GetPendingColumns)
        HRESULT ( STDMETHODCALLTYPE *GetPendingColumns )( 
            IRowsetChangeExtInfo * This,
            /* [in] */ HCHAPTER hReserved,
            /* [in] */ HROW hRow,
            /* [in] */ ULONG cColumnOrdinals,
            /* [size_is][in] */ const ULONG rgiOrdinals[  ],
            /* [size_is][out] */ DBPENDINGSTATUS rgColumnStatus[  ]);
        
        END_INTERFACE
    } IRowsetChangeExtInfoVtbl;

    interface IRowsetChangeExtInfo
    {
        CONST_VTBL struct IRowsetChangeExtInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRowsetChangeExtInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRowsetChangeExtInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRowsetChangeExtInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRowsetChangeExtInfo_GetOriginalRow(This,hReserved,hRow,phRowOriginal)	\
    ( (This)->lpVtbl -> GetOriginalRow(This,hReserved,hRow,phRowOriginal) ) 

#define IRowsetChangeExtInfo_GetPendingColumns(This,hReserved,hRow,cColumnOrdinals,rgiOrdinals,rgColumnStatus)	\
    ( (This)->lpVtbl -> GetPendingColumns(This,hReserved,hRow,cColumnOrdinals,rgiOrdinals,rgColumnStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRowsetChangeExtInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msdasql_interfaces_0000_0001 */
/* [local] */ 

// the structure passed in in IRequestDiagFields::RequestDiagFields
typedef struct tagKAGREQDIAG
    {
    ULONG ulDiagFlags;
    VARTYPE vt;
    SHORT sDiagField;
    } 	KAGREQDIAG;

// the structure passed in in IGetDiagField::GetDiagField
typedef struct tagKAGGETDIAG
    {
    ULONG ulSize;
    VARIANTARG vDiagInfo;
    SHORT sDiagField;
    } 	KAGGETDIAG;



extern RPC_IF_HANDLE __MIDL_itf_msdasql_interfaces_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msdasql_interfaces_0000_0001_v0_0_s_ifspec;

#ifndef __ISQLRequestDiagFields_INTERFACE_DEFINED__
#define __ISQLRequestDiagFields_INTERFACE_DEFINED__

/* interface ISQLRequestDiagFields */
/* [object][uuid] */ 


EXTERN_C const IID IID_ISQLRequestDiagFields;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("228972F0-B5FF-11d0-8A80-00C04FD611CD")
    ISQLRequestDiagFields : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestDiagFields( 
            /* [in] */ ULONG cDiagFields,
            /* [size_is][in] */ __RPC__in_ecount_full(cDiagFields) KAGREQDIAG rgDiagFields[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISQLRequestDiagFieldsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISQLRequestDiagFields * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISQLRequestDiagFields * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISQLRequestDiagFields * This);
        
        DECLSPEC_XFGVIRT(ISQLRequestDiagFields, RequestDiagFields)
        HRESULT ( STDMETHODCALLTYPE *RequestDiagFields )( 
            __RPC__in ISQLRequestDiagFields * This,
            /* [in] */ ULONG cDiagFields,
            /* [size_is][in] */ __RPC__in_ecount_full(cDiagFields) KAGREQDIAG rgDiagFields[  ]);
        
        END_INTERFACE
    } ISQLRequestDiagFieldsVtbl;

    interface ISQLRequestDiagFields
    {
        CONST_VTBL struct ISQLRequestDiagFieldsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISQLRequestDiagFields_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISQLRequestDiagFields_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISQLRequestDiagFields_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISQLRequestDiagFields_RequestDiagFields(This,cDiagFields,rgDiagFields)	\
    ( (This)->lpVtbl -> RequestDiagFields(This,cDiagFields,rgDiagFields) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISQLRequestDiagFields_INTERFACE_DEFINED__ */


#ifndef __ISQLGetDiagField_INTERFACE_DEFINED__
#define __ISQLGetDiagField_INTERFACE_DEFINED__

/* interface ISQLGetDiagField */
/* [object][uuid] */ 


EXTERN_C const IID IID_ISQLGetDiagField;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("228972F1-B5FF-11d0-8A80-00C04FD611CD")
    ISQLGetDiagField : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDiagField( 
            /* [unique][out][in] */ __RPC__inout_opt KAGGETDIAG *pDiagInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISQLGetDiagFieldVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISQLGetDiagField * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISQLGetDiagField * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISQLGetDiagField * This);
        
        DECLSPEC_XFGVIRT(ISQLGetDiagField, GetDiagField)
        HRESULT ( STDMETHODCALLTYPE *GetDiagField )( 
            __RPC__in ISQLGetDiagField * This,
            /* [unique][out][in] */ __RPC__inout_opt KAGGETDIAG *pDiagInfo);
        
        END_INTERFACE
    } ISQLGetDiagFieldVtbl;

    interface ISQLGetDiagField
    {
        CONST_VTBL struct ISQLGetDiagFieldVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISQLGetDiagField_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISQLGetDiagField_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISQLGetDiagField_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISQLGetDiagField_GetDiagField(This,pDiagInfo)	\
    ( (This)->lpVtbl -> GetDiagField(This,pDiagInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISQLGetDiagField_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msdasql_interfaces_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msdasql_interfaces_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msdasql_interfaces_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


