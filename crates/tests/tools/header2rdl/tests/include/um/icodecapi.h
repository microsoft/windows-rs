

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

#ifndef __icodecapi_h__
#define __icodecapi_h__

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

#ifndef __ICodecAPI_FWD_DEFINED__
#define __ICodecAPI_FWD_DEFINED__
typedef interface ICodecAPI ICodecAPI;

#endif 	/* __ICodecAPI_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_icodecapi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
struct CodecAPIEventData
    {
    GUID guid;
    DWORD dataLength;
    DWORD reserved[ 3 ];
    } ;



extern RPC_IF_HANDLE __MIDL_itf_icodecapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_icodecapi_0000_0000_v0_0_s_ifspec;

#ifndef __ICodecAPI_INTERFACE_DEFINED__
#define __ICodecAPI_INTERFACE_DEFINED__

/* interface ICodecAPI */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ICodecAPI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("901db4c7-31ce-41a2-85dc-8fa0bf41b8da")
    ICodecAPI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsSupported( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsModifiable( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterRange( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterValues( 
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForEvent( 
            /* [in] */ const GUID *Api,
            /* [in] */ LONG_PTR userData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterForEvent( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllDefaults( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValueWithNotify( 
            /* [in] */ const GUID *Api,
            /* [in] */ VARIANT *Value,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllDefaultsWithNotify( 
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllSettings( 
            /* [in] */ IStream *__MIDL__ICodecAPI0000) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllSettings( 
            /* [in] */ IStream *__MIDL__ICodecAPI0001) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllSettingsWithNotify( 
            IStream *__MIDL__ICodecAPI0002,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICodecAPIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICodecAPI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICodecAPI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICodecAPI * This);
        
        DECLSPEC_XFGVIRT(ICodecAPI, IsSupported)
        HRESULT ( STDMETHODCALLTYPE *IsSupported )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(ICodecAPI, IsModifiable)
        HRESULT ( STDMETHODCALLTYPE *IsModifiable )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetParameterRange)
        HRESULT ( STDMETHODCALLTYPE *GetParameterRange )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetParameterValues)
        HRESULT ( STDMETHODCALLTYPE *GetParameterValues )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetDefaultValue)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultValue )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ICodecAPI, RegisterForEvent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForEvent )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [in] */ LONG_PTR userData);
        
        DECLSPEC_XFGVIRT(ICodecAPI, UnregisterForEvent)
        HRESULT ( STDMETHODCALLTYPE *UnregisterForEvent )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllDefaults)
        HRESULT ( STDMETHODCALLTYPE *SetAllDefaults )( 
            ICodecAPI * This);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetValueWithNotify)
        HRESULT ( STDMETHODCALLTYPE *SetValueWithNotify )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [in] */ VARIANT *Value,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllDefaultsWithNotify)
        HRESULT ( STDMETHODCALLTYPE *SetAllDefaultsWithNotify )( 
            ICodecAPI * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetAllSettings)
        HRESULT ( STDMETHODCALLTYPE *GetAllSettings )( 
            ICodecAPI * This,
            /* [in] */ IStream *__MIDL__ICodecAPI0000);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllSettings)
        HRESULT ( STDMETHODCALLTYPE *SetAllSettings )( 
            ICodecAPI * This,
            /* [in] */ IStream *__MIDL__ICodecAPI0001);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllSettingsWithNotify)
        HRESULT ( STDMETHODCALLTYPE *SetAllSettingsWithNotify )( 
            ICodecAPI * This,
            IStream *__MIDL__ICodecAPI0002,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount);
        
        END_INTERFACE
    } ICodecAPIVtbl;

    interface ICodecAPI
    {
        CONST_VTBL struct ICodecAPIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICodecAPI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICodecAPI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICodecAPI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICodecAPI_IsSupported(This,Api)	\
    ( (This)->lpVtbl -> IsSupported(This,Api) ) 

#define ICodecAPI_IsModifiable(This,Api)	\
    ( (This)->lpVtbl -> IsModifiable(This,Api) ) 

#define ICodecAPI_GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta)	\
    ( (This)->lpVtbl -> GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta) ) 

#define ICodecAPI_GetParameterValues(This,Api,Values,ValuesCount)	\
    ( (This)->lpVtbl -> GetParameterValues(This,Api,Values,ValuesCount) ) 

#define ICodecAPI_GetDefaultValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetDefaultValue(This,Api,Value) ) 

#define ICodecAPI_GetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetValue(This,Api,Value) ) 

#define ICodecAPI_SetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Api,Value) ) 

#define ICodecAPI_RegisterForEvent(This,Api,userData)	\
    ( (This)->lpVtbl -> RegisterForEvent(This,Api,userData) ) 

#define ICodecAPI_UnregisterForEvent(This,Api)	\
    ( (This)->lpVtbl -> UnregisterForEvent(This,Api) ) 

#define ICodecAPI_SetAllDefaults(This)	\
    ( (This)->lpVtbl -> SetAllDefaults(This) ) 

#define ICodecAPI_SetValueWithNotify(This,Api,Value,ChangedParam,ChangedParamCount)	\
    ( (This)->lpVtbl -> SetValueWithNotify(This,Api,Value,ChangedParam,ChangedParamCount) ) 

#define ICodecAPI_SetAllDefaultsWithNotify(This,ChangedParam,ChangedParamCount)	\
    ( (This)->lpVtbl -> SetAllDefaultsWithNotify(This,ChangedParam,ChangedParamCount) ) 

#define ICodecAPI_GetAllSettings(This,__MIDL__ICodecAPI0000)	\
    ( (This)->lpVtbl -> GetAllSettings(This,__MIDL__ICodecAPI0000) ) 

#define ICodecAPI_SetAllSettings(This,__MIDL__ICodecAPI0001)	\
    ( (This)->lpVtbl -> SetAllSettings(This,__MIDL__ICodecAPI0001) ) 

#define ICodecAPI_SetAllSettingsWithNotify(This,__MIDL__ICodecAPI0002,ChangedParam,ChangedParamCount)	\
    ( (This)->lpVtbl -> SetAllSettingsWithNotify(This,__MIDL__ICodecAPI0002,ChangedParam,ChangedParamCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICodecAPI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_icodecapi_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_icodecapi_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_icodecapi_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


