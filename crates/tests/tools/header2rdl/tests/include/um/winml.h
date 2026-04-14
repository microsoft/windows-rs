

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

#ifndef __winml_h__
#define __winml_h__

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

#ifndef __IWinMLModel_FWD_DEFINED__
#define __IWinMLModel_FWD_DEFINED__
typedef interface IWinMLModel IWinMLModel;

#endif 	/* __IWinMLModel_FWD_DEFINED__ */


#ifndef __IWinMLEvaluationContext_FWD_DEFINED__
#define __IWinMLEvaluationContext_FWD_DEFINED__
typedef interface IWinMLEvaluationContext IWinMLEvaluationContext;

#endif 	/* __IWinMLEvaluationContext_FWD_DEFINED__ */


#ifndef __IWinMLRuntime_FWD_DEFINED__
#define __IWinMLRuntime_FWD_DEFINED__
typedef interface IWinMLRuntime IWinMLRuntime;

#endif 	/* __IWinMLRuntime_FWD_DEFINED__ */


#ifndef __IWinMLRuntimeFactory_FWD_DEFINED__
#define __IWinMLRuntimeFactory_FWD_DEFINED__
typedef interface IWinMLRuntimeFactory IWinMLRuntimeFactory;

#endif 	/* __IWinMLRuntimeFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "d3d12.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_winml_0000_0000 */
/* [local] */ 

#define	WINML_TENSOR_DIMENSION_COUNT_MAX	( 4 )

typedef 
enum WINML_TENSOR_DATA_TYPE
    {
        WINML_TENSOR_UNDEFINED	= 0,
        WINML_TENSOR_FLOAT	= ( WINML_TENSOR_UNDEFINED + 1 ) ,
        WINML_TENSOR_UINT8	= ( WINML_TENSOR_FLOAT + 1 ) ,
        WINML_TENSOR_INT8	= ( WINML_TENSOR_UINT8 + 1 ) ,
        WINML_TENSOR_UINT16	= ( WINML_TENSOR_INT8 + 1 ) ,
        WINML_TENSOR_INT16	= ( WINML_TENSOR_UINT16 + 1 ) ,
        WINML_TENSOR_INT32	= ( WINML_TENSOR_INT16 + 1 ) ,
        WINML_TENSOR_INT64	= ( WINML_TENSOR_INT32 + 1 ) ,
        WINML_TENSOR_STRING	= ( WINML_TENSOR_INT64 + 1 ) ,
        WINML_TENSOR_BOOLEAN	= ( WINML_TENSOR_STRING + 1 ) ,
        WINML_TENSOR_FLOAT16	= ( WINML_TENSOR_BOOLEAN + 1 ) ,
        WINML_TENSOR_DOUBLE	= ( WINML_TENSOR_FLOAT16 + 1 ) ,
        WINML_TENSOR_UINT32	= ( WINML_TENSOR_DOUBLE + 1 ) ,
        WINML_TENSOR_UINT64	= ( WINML_TENSOR_UINT32 + 1 ) ,
        WINML_TENSOR_COMPLEX64	= ( WINML_TENSOR_UINT64 + 1 ) ,
        WINML_TENSOR_COMPLEX128	= ( WINML_TENSOR_COMPLEX64 + 1 ) 
    } 	WINML_TENSOR_DATA_TYPE;

typedef 
enum WINML_FEATURE_TYPE
    {
        WINML_FEATURE_UNDEFINED	= 0,
        WINML_FEATURE_TENSOR	= ( WINML_FEATURE_UNDEFINED + 1 ) ,
        WINML_FEATURE_SEQUENCE	= ( WINML_FEATURE_TENSOR + 1 ) ,
        WINML_FEATURE_MAP	= ( WINML_FEATURE_SEQUENCE + 1 ) ,
        WINML_FEATURE_IMAGE	= ( WINML_FEATURE_MAP + 1 ) 
    } 	WINML_FEATURE_TYPE;

typedef 
enum WINML_BINDING_TYPE
    {
        WINML_BINDING_UNDEFINED	= 0,
        WINML_BINDING_TENSOR	= ( WINML_BINDING_UNDEFINED + 1 ) ,
        WINML_BINDING_SEQUENCE	= ( WINML_BINDING_TENSOR + 1 ) ,
        WINML_BINDING_MAP	= ( WINML_BINDING_SEQUENCE + 1 ) ,
        WINML_BINDING_IMAGE	= ( WINML_BINDING_MAP + 1 ) ,
        WINML_BINDING_RESOURCE	= ( WINML_BINDING_IMAGE + 1 ) 
    } 	WINML_BINDING_TYPE;

typedef struct WINML_TENSOR_BINDING_DESC
    {
    WINML_TENSOR_DATA_TYPE DataType;
    /* [annotation] */ 
    _In_range_(1, WINML_TENSOR_DIMENSION_COUNT_MAX)  UINT NumDimensions;
    /* [annotation] */ 
    _Field_size_full_(NumDimensions)  INT64 *pShape;
    UINT DataSize;
    /* [annotation] */ 
    _Field_size_bytes_full_(DataSize)  void *pData;
    } 	WINML_TENSOR_BINDING_DESC;

typedef struct WINML_SEQUENCE_BINDING_DESC
    {
    UINT ElementCount;
    WINML_TENSOR_DATA_TYPE ElementType;
    union 
        {
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  LPWSTR *pStrings;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  INT64 *pInts;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  FLOAT *pFloats;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  DOUBLE *pDoubles;
        } 	;
    } 	WINML_SEQUENCE_BINDING_DESC;

typedef struct WINML_MAP_BINDING_DESC
    {
    UINT ElementCount;
    WINML_TENSOR_DATA_TYPE KeyType;
    union 
        {
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  LPWSTR *pStringKeys;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  INT64 *pIntKeys;
        } 	;
    WINML_TENSOR_DATA_TYPE Fields;
    union 
        {
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  LPWSTR *pStringFields;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  INT64 *pIntFields;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  FLOAT *pFloatFields;
        /* [annotation] */ 
        _Field_size_full_(ElementCount)  DOUBLE *pDoubleFields;
        } 	;
    } 	WINML_MAP_BINDING_DESC;

typedef struct WINML_IMAGE_BINDING_DESC
    {
    WINML_TENSOR_DATA_TYPE ElementType;
    /* [annotation] */ 
    _In_range_(1, WINML_TENSOR_DIMENSION_COUNT_MAX)  UINT NumDimensions;
    /* [annotation] */ 
    _Field_size_full_(NumDimensions)  INT64 *pShape;
    UINT DataSize;
    /* [annotation] */ 
    _Field_size_bytes_full_(DataSize)  void *pData;
    } 	WINML_IMAGE_BINDING_DESC;

typedef struct WINML_RESOURCE_BINDING_DESC
    {
    WINML_TENSOR_DATA_TYPE ElementType;
    /* [annotation] */ 
    _In_range_(1, WINML_TENSOR_DIMENSION_COUNT_MAX)  UINT NumDimensions;
    /* [annotation] */ 
    _Field_size_full_(NumDimensions)  INT64 *pShape;
    /* [annotation] */ 
    _In_  ID3D12Resource *pResource;
    } 	WINML_RESOURCE_BINDING_DESC;

typedef struct WINML_BINDING_DESC
    {
    LPCWSTR Name;
    WINML_BINDING_TYPE BindType;
    union 
        {
        WINML_TENSOR_BINDING_DESC Tensor;
        WINML_SEQUENCE_BINDING_DESC Sequence;
        WINML_MAP_BINDING_DESC Map;
        WINML_IMAGE_BINDING_DESC Image;
        WINML_RESOURCE_BINDING_DESC Resource;
        } 	;
    } 	WINML_BINDING_DESC;

typedef struct WINML_TENSOR_VARIABLE_DESC
    {
    WINML_TENSOR_DATA_TYPE ElementType;
    /* [annotation] */ 
    _In_range_(1, WINML_TENSOR_DIMENSION_COUNT_MAX)  UINT NumDimensions;
    /* [annotation] */ 
    _Field_size_full_(NumDimensions)  INT64 *pShape;
    } 	WINML_TENSOR_VARIABLE_DESC;

typedef struct WINML_SEQUENCE_VARIABLE_DESC
    {
    WINML_TENSOR_DATA_TYPE ElementType;
    } 	WINML_SEQUENCE_VARIABLE_DESC;

typedef struct WINML_MAP_VARIABLE_DESC
    {
    WINML_TENSOR_DATA_TYPE KeyType;
    WINML_TENSOR_DATA_TYPE Fields;
    } 	WINML_MAP_VARIABLE_DESC;

typedef struct WINML_IMAGE_VARIABLE_DESC
    {
    WINML_TENSOR_DATA_TYPE ElementType;
    UINT NumDimensions;
    /* [annotation] */ 
    _Field_size_full_(NumDimensions)  INT64 *pShape;
    } 	WINML_IMAGE_VARIABLE_DESC;

typedef struct WINML_VARIABLE_DESC
    {
    LPWSTR Name;
    LPWSTR Description;
    WINML_FEATURE_TYPE FeatureType;
    BOOL Required;
    union 
        {
        WINML_TENSOR_VARIABLE_DESC Tensor;
        WINML_SEQUENCE_VARIABLE_DESC Sequence;
        WINML_MAP_VARIABLE_DESC Map;
        WINML_IMAGE_VARIABLE_DESC Image;
        } 	;
    } 	WINML_VARIABLE_DESC;

typedef struct WINML_MODEL_DESC
    {
    LPWSTR Author;
    LPWSTR Name;
    LPWSTR Domain;
    LPWSTR Description;
    SIZE_T Version;
    } 	WINML_MODEL_DESC;



extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0000_v0_0_s_ifspec;

#ifndef __IWinMLModel_INTERFACE_DEFINED__
#define __IWinMLModel_INTERFACE_DEFINED__

/* interface IWinMLModel */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IWinMLModel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e2eeb6a9-f31f-4055-a521-e30b5b33664a")
    IWinMLModel : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [annotation] */ 
            _Out_  WINML_MODEL_DESC **ppDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateMetadata( 
            /* [annotation] */ 
            _In_  UINT Index,
            /* [annotation] */ 
            _Outptr_  LPCWSTR *pKey,
            /* [annotation] */ 
            _Outptr_  LPCWSTR *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateModelInputs( 
            /* [annotation] */ 
            _In_  UINT Index,
            /* [annotation] */ 
            _Out_  WINML_VARIABLE_DESC **ppInputDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateModelOutputs( 
            /* [annotation] */ 
            _In_  UINT Index,
            /* [annotation] */ 
            _Out_  WINML_VARIABLE_DESC **ppOutputDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinMLModelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinMLModel * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinMLModel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinMLModel * This);
        
        DECLSPEC_XFGVIRT(IWinMLModel, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            IWinMLModel * This,
            /* [annotation] */ 
            _Out_  WINML_MODEL_DESC **ppDescription);
        
        DECLSPEC_XFGVIRT(IWinMLModel, EnumerateMetadata)
        HRESULT ( STDMETHODCALLTYPE *EnumerateMetadata )( 
            IWinMLModel * This,
            /* [annotation] */ 
            _In_  UINT Index,
            /* [annotation] */ 
            _Outptr_  LPCWSTR *pKey,
            /* [annotation] */ 
            _Outptr_  LPCWSTR *pValue);
        
        DECLSPEC_XFGVIRT(IWinMLModel, EnumerateModelInputs)
        HRESULT ( STDMETHODCALLTYPE *EnumerateModelInputs )( 
            IWinMLModel * This,
            /* [annotation] */ 
            _In_  UINT Index,
            /* [annotation] */ 
            _Out_  WINML_VARIABLE_DESC **ppInputDescriptor);
        
        DECLSPEC_XFGVIRT(IWinMLModel, EnumerateModelOutputs)
        HRESULT ( STDMETHODCALLTYPE *EnumerateModelOutputs )( 
            IWinMLModel * This,
            /* [annotation] */ 
            _In_  UINT Index,
            /* [annotation] */ 
            _Out_  WINML_VARIABLE_DESC **ppOutputDescriptor);
        
        END_INTERFACE
    } IWinMLModelVtbl;

    interface IWinMLModel
    {
        CONST_VTBL struct IWinMLModelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinMLModel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinMLModel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinMLModel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinMLModel_GetDescription(This,ppDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,ppDescription) ) 

#define IWinMLModel_EnumerateMetadata(This,Index,pKey,pValue)	\
    ( (This)->lpVtbl -> EnumerateMetadata(This,Index,pKey,pValue) ) 

#define IWinMLModel_EnumerateModelInputs(This,Index,ppInputDescriptor)	\
    ( (This)->lpVtbl -> EnumerateModelInputs(This,Index,ppInputDescriptor) ) 

#define IWinMLModel_EnumerateModelOutputs(This,Index,ppOutputDescriptor)	\
    ( (This)->lpVtbl -> EnumerateModelOutputs(This,Index,ppOutputDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinMLModel_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_winml_0000_0001 */
/* [local] */ 

#pragma deprecated(IWinMLModel)


extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0001_v0_0_s_ifspec;

#ifndef __IWinMLEvaluationContext_INTERFACE_DEFINED__
#define __IWinMLEvaluationContext_INTERFACE_DEFINED__

/* interface IWinMLEvaluationContext */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IWinMLEvaluationContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95848f9e-583d-4054-af12-916387cd8426")
    IWinMLEvaluationContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BindValue( 
            /* [annotation] */ 
            _In_  WINML_BINDING_DESC *pDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValueByName( 
            /* [annotation] */ 
            _In_z_  LPCWSTR Name,
            /* [annotation] */ 
            _Out_  WINML_BINDING_DESC **pDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinMLEvaluationContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinMLEvaluationContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinMLEvaluationContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinMLEvaluationContext * This);
        
        DECLSPEC_XFGVIRT(IWinMLEvaluationContext, BindValue)
        HRESULT ( STDMETHODCALLTYPE *BindValue )( 
            IWinMLEvaluationContext * This,
            /* [annotation] */ 
            _In_  WINML_BINDING_DESC *pDescriptor);
        
        DECLSPEC_XFGVIRT(IWinMLEvaluationContext, GetValueByName)
        HRESULT ( STDMETHODCALLTYPE *GetValueByName )( 
            IWinMLEvaluationContext * This,
            /* [annotation] */ 
            _In_z_  LPCWSTR Name,
            /* [annotation] */ 
            _Out_  WINML_BINDING_DESC **pDescriptor);
        
        DECLSPEC_XFGVIRT(IWinMLEvaluationContext, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IWinMLEvaluationContext * This);
        
        END_INTERFACE
    } IWinMLEvaluationContextVtbl;

    interface IWinMLEvaluationContext
    {
        CONST_VTBL struct IWinMLEvaluationContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinMLEvaluationContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinMLEvaluationContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinMLEvaluationContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinMLEvaluationContext_BindValue(This,pDescriptor)	\
    ( (This)->lpVtbl -> BindValue(This,pDescriptor) ) 

#define IWinMLEvaluationContext_GetValueByName(This,Name,pDescriptor)	\
    ( (This)->lpVtbl -> GetValueByName(This,Name,pDescriptor) ) 

#define IWinMLEvaluationContext_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinMLEvaluationContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_winml_0000_0002 */
/* [local] */ 

#pragma deprecated(IWinMLEvaluationContext)


extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0002_v0_0_s_ifspec;

#ifndef __IWinMLRuntime_INTERFACE_DEFINED__
#define __IWinMLRuntime_INTERFACE_DEFINED__

/* interface IWinMLRuntime */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IWinMLRuntime;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a0425329-40ae-48d9-bce3-829ef7b8a41a")
    IWinMLRuntime : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadModel( 
            /* [annotation] */ 
            _In_z_  LPCWSTR Path,
            /* [annotation][out] */ 
            _COM_Outptr_  IWinMLModel **ppModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateEvaluationContext( 
            /* [annotation] */ 
            _In_opt_  ID3D12Device *device,
            /* [annotation][out] */ 
            _COM_Outptr_  IWinMLEvaluationContext **ppContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EvaluateModel( 
            /* [annotation] */ 
            _In_  IWinMLEvaluationContext *pContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinMLRuntimeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinMLRuntime * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinMLRuntime * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinMLRuntime * This);
        
        DECLSPEC_XFGVIRT(IWinMLRuntime, LoadModel)
        HRESULT ( STDMETHODCALLTYPE *LoadModel )( 
            IWinMLRuntime * This,
            /* [annotation] */ 
            _In_z_  LPCWSTR Path,
            /* [annotation][out] */ 
            _COM_Outptr_  IWinMLModel **ppModel);
        
        DECLSPEC_XFGVIRT(IWinMLRuntime, CreateEvaluationContext)
        HRESULT ( STDMETHODCALLTYPE *CreateEvaluationContext )( 
            IWinMLRuntime * This,
            /* [annotation] */ 
            _In_opt_  ID3D12Device *device,
            /* [annotation][out] */ 
            _COM_Outptr_  IWinMLEvaluationContext **ppContext);
        
        DECLSPEC_XFGVIRT(IWinMLRuntime, EvaluateModel)
        HRESULT ( STDMETHODCALLTYPE *EvaluateModel )( 
            IWinMLRuntime * This,
            /* [annotation] */ 
            _In_  IWinMLEvaluationContext *pContext);
        
        END_INTERFACE
    } IWinMLRuntimeVtbl;

    interface IWinMLRuntime
    {
        CONST_VTBL struct IWinMLRuntimeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinMLRuntime_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinMLRuntime_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinMLRuntime_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinMLRuntime_LoadModel(This,Path,ppModel)	\
    ( (This)->lpVtbl -> LoadModel(This,Path,ppModel) ) 

#define IWinMLRuntime_CreateEvaluationContext(This,device,ppContext)	\
    ( (This)->lpVtbl -> CreateEvaluationContext(This,device,ppContext) ) 

#define IWinMLRuntime_EvaluateModel(This,pContext)	\
    ( (This)->lpVtbl -> EvaluateModel(This,pContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinMLRuntime_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_winml_0000_0003 */
/* [local] */ 

#pragma deprecated(IWinMLRuntime)
typedef /* [public] */ 
enum WINML_RUNTIME_TYPE
    {
        WINML_RUNTIME_CNTK	= 0
    } 	WINML_RUNTIME_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0003_v0_0_s_ifspec;

#ifndef __IWinMLRuntimeFactory_INTERFACE_DEFINED__
#define __IWinMLRuntimeFactory_INTERFACE_DEFINED__

/* interface IWinMLRuntimeFactory */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IWinMLRuntimeFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a807b84d-4ae5-4bc0-a76a-941aa246bd41")
    IWinMLRuntimeFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateRuntime( 
            /* [annotation] */ 
            _In_  WINML_RUNTIME_TYPE RuntimeType,
            /* [annotation][out] */ 
            _COM_Outptr_  IWinMLRuntime **ppRuntime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWinMLRuntimeFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWinMLRuntimeFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWinMLRuntimeFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWinMLRuntimeFactory * This);
        
        DECLSPEC_XFGVIRT(IWinMLRuntimeFactory, CreateRuntime)
        HRESULT ( STDMETHODCALLTYPE *CreateRuntime )( 
            IWinMLRuntimeFactory * This,
            /* [annotation] */ 
            _In_  WINML_RUNTIME_TYPE RuntimeType,
            /* [annotation][out] */ 
            _COM_Outptr_  IWinMLRuntime **ppRuntime);
        
        END_INTERFACE
    } IWinMLRuntimeFactoryVtbl;

    interface IWinMLRuntimeFactory
    {
        CONST_VTBL struct IWinMLRuntimeFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWinMLRuntimeFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWinMLRuntimeFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWinMLRuntimeFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWinMLRuntimeFactory_CreateRuntime(This,RuntimeType,ppRuntime)	\
    ( (This)->lpVtbl -> CreateRuntime(This,RuntimeType,ppRuntime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWinMLRuntimeFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_winml_0000_0004 */
/* [local] */ 

#pragma deprecated(IWinMLRuntimeFactory)
HRESULT WINAPI WinMLCreateRuntime(
    _COM_Outptr_ IWinMLRuntime **runtime
    );
#pragma deprecated(WinMLCreateRuntime)


extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_winml_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


