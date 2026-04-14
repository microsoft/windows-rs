

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

#ifndef __vidcap_h__
#define __vidcap_h__

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

#ifndef __IKsTopologyInfo_FWD_DEFINED__
#define __IKsTopologyInfo_FWD_DEFINED__
typedef interface IKsTopologyInfo IKsTopologyInfo;

#endif 	/* __IKsTopologyInfo_FWD_DEFINED__ */


#ifndef __ISelector_FWD_DEFINED__
#define __ISelector_FWD_DEFINED__
typedef interface ISelector ISelector;

#endif 	/* __ISelector_FWD_DEFINED__ */


#ifndef __ICameraControl_FWD_DEFINED__
#define __ICameraControl_FWD_DEFINED__
typedef interface ICameraControl ICameraControl;

#endif 	/* __ICameraControl_FWD_DEFINED__ */


#ifndef __IVideoProcAmp_FWD_DEFINED__
#define __IVideoProcAmp_FWD_DEFINED__
typedef interface IVideoProcAmp IVideoProcAmp;

#endif 	/* __IVideoProcAmp_FWD_DEFINED__ */


#ifndef __IKsNodeControl_FWD_DEFINED__
#define __IKsNodeControl_FWD_DEFINED__
typedef interface IKsNodeControl IKsNodeControl;

#endif 	/* __IKsNodeControl_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vidcap_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include "ks.h"
#ifndef _KS_
typedef /* [public][public] */ struct __MIDL___MIDL_itf_vidcap_0000_0000_0001
    {
    ULONG FromNode;
    ULONG FromNodePin;
    ULONG ToNode;
    ULONG ToNodePin;
    } 	KSTOPOLOGY_CONNECTION;

typedef struct __MIDL___MIDL_itf_vidcap_0000_0000_0001 *PKSTOPOLOGY_CONNECTION;

#endif


extern RPC_IF_HANDLE __MIDL_itf_vidcap_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vidcap_0000_0000_v0_0_s_ifspec;

#ifndef __IKsTopologyInfo_INTERFACE_DEFINED__
#define __IKsTopologyInfo_INTERFACE_DEFINED__

/* interface IKsTopologyInfo */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IKsTopologyInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("720D4AC0-7533-11D0-A5D6-28DB04C10000")
    IKsTopologyInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_NumCategories( 
            /* [out] */ DWORD *pdwNumCategories) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Category( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ GUID *pCategory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NumConnections( 
            /* [out] */ DWORD *pdwNumConnections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ConnectionInfo( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ KSTOPOLOGY_CONNECTION *pConnectionInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NodeName( 
            /* [in] */ DWORD dwNodeId,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(dwBufSize)  WCHAR *pwchNodeName,
            /* [in] */ DWORD dwBufSize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwNameLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NumNodes( 
            /* [out] */ DWORD *pdwNumNodes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NodeType( 
            /* [in] */ DWORD dwNodeId,
            /* [out] */ GUID *pNodeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateNodeInstance( 
            /* [in] */ DWORD dwNodeId,
            /* [in] */ REFIID iid,
            /* [out] */ void **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IKsTopologyInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IKsTopologyInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IKsTopologyInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IKsTopologyInfo * This);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_NumCategories)
        HRESULT ( STDMETHODCALLTYPE *get_NumCategories )( 
            IKsTopologyInfo * This,
            /* [out] */ DWORD *pdwNumCategories);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_Category)
        HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            IKsTopologyInfo * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ GUID *pCategory);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_NumConnections)
        HRESULT ( STDMETHODCALLTYPE *get_NumConnections )( 
            IKsTopologyInfo * This,
            /* [out] */ DWORD *pdwNumConnections);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_ConnectionInfo)
        HRESULT ( STDMETHODCALLTYPE *get_ConnectionInfo )( 
            IKsTopologyInfo * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ KSTOPOLOGY_CONNECTION *pConnectionInfo);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_NodeName)
        HRESULT ( STDMETHODCALLTYPE *get_NodeName )( 
            IKsTopologyInfo * This,
            /* [in] */ DWORD dwNodeId,
            /* [annotation][out] */ 
            _Out_writes_bytes_opt_(dwBufSize)  WCHAR *pwchNodeName,
            /* [in] */ DWORD dwBufSize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwNameLen);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_NumNodes)
        HRESULT ( STDMETHODCALLTYPE *get_NumNodes )( 
            IKsTopologyInfo * This,
            /* [out] */ DWORD *pdwNumNodes);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, get_NodeType)
        HRESULT ( STDMETHODCALLTYPE *get_NodeType )( 
            IKsTopologyInfo * This,
            /* [in] */ DWORD dwNodeId,
            /* [out] */ GUID *pNodeType);
        
        DECLSPEC_XFGVIRT(IKsTopologyInfo, CreateNodeInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateNodeInstance )( 
            IKsTopologyInfo * This,
            /* [in] */ DWORD dwNodeId,
            /* [in] */ REFIID iid,
            /* [out] */ void **ppvObject);
        
        END_INTERFACE
    } IKsTopologyInfoVtbl;

    interface IKsTopologyInfo
    {
        CONST_VTBL struct IKsTopologyInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IKsTopologyInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IKsTopologyInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IKsTopologyInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IKsTopologyInfo_get_NumCategories(This,pdwNumCategories)	\
    ( (This)->lpVtbl -> get_NumCategories(This,pdwNumCategories) ) 

#define IKsTopologyInfo_get_Category(This,dwIndex,pCategory)	\
    ( (This)->lpVtbl -> get_Category(This,dwIndex,pCategory) ) 

#define IKsTopologyInfo_get_NumConnections(This,pdwNumConnections)	\
    ( (This)->lpVtbl -> get_NumConnections(This,pdwNumConnections) ) 

#define IKsTopologyInfo_get_ConnectionInfo(This,dwIndex,pConnectionInfo)	\
    ( (This)->lpVtbl -> get_ConnectionInfo(This,dwIndex,pConnectionInfo) ) 

#define IKsTopologyInfo_get_NodeName(This,dwNodeId,pwchNodeName,dwBufSize,pdwNameLen)	\
    ( (This)->lpVtbl -> get_NodeName(This,dwNodeId,pwchNodeName,dwBufSize,pdwNameLen) ) 

#define IKsTopologyInfo_get_NumNodes(This,pdwNumNodes)	\
    ( (This)->lpVtbl -> get_NumNodes(This,pdwNumNodes) ) 

#define IKsTopologyInfo_get_NodeType(This,dwNodeId,pNodeType)	\
    ( (This)->lpVtbl -> get_NodeType(This,dwNodeId,pNodeType) ) 

#define IKsTopologyInfo_CreateNodeInstance(This,dwNodeId,iid,ppvObject)	\
    ( (This)->lpVtbl -> CreateNodeInstance(This,dwNodeId,iid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IKsTopologyInfo_INTERFACE_DEFINED__ */


#ifndef __ISelector_INTERFACE_DEFINED__
#define __ISelector_INTERFACE_DEFINED__

/* interface ISelector */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ISelector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1ABDAECA-68B6-4F83-9371-B413907C7B9F")
    ISelector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_NumSources( 
            /* [out] */ DWORD *pdwNumSources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SourceNodeId( 
            /* [out] */ DWORD *pdwPinId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SourceNodeId( 
            /* [in] */ DWORD dwPinId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISelectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISelector * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISelector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISelector * This);
        
        DECLSPEC_XFGVIRT(ISelector, get_NumSources)
        HRESULT ( STDMETHODCALLTYPE *get_NumSources )( 
            ISelector * This,
            /* [out] */ DWORD *pdwNumSources);
        
        DECLSPEC_XFGVIRT(ISelector, get_SourceNodeId)
        HRESULT ( STDMETHODCALLTYPE *get_SourceNodeId )( 
            ISelector * This,
            /* [out] */ DWORD *pdwPinId);
        
        DECLSPEC_XFGVIRT(ISelector, put_SourceNodeId)
        HRESULT ( STDMETHODCALLTYPE *put_SourceNodeId )( 
            ISelector * This,
            /* [in] */ DWORD dwPinId);
        
        END_INTERFACE
    } ISelectorVtbl;

    interface ISelector
    {
        CONST_VTBL struct ISelectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISelector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISelector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISelector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISelector_get_NumSources(This,pdwNumSources)	\
    ( (This)->lpVtbl -> get_NumSources(This,pdwNumSources) ) 

#define ISelector_get_SourceNodeId(This,pdwPinId)	\
    ( (This)->lpVtbl -> get_SourceNodeId(This,pdwPinId) ) 

#define ISelector_put_SourceNodeId(This,dwPinId)	\
    ( (This)->lpVtbl -> put_SourceNodeId(This,dwPinId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISelector_INTERFACE_DEFINED__ */


#ifndef __ICameraControl_INTERFACE_DEFINED__
#define __ICameraControl_INTERFACE_DEFINED__

/* interface ICameraControl */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ICameraControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2BA1785D-4D1B-44EF-85E8-C7F1D3F20184")
    ICameraControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Exposure( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Exposure( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Exposure( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Focus( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Focus( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Focus( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Iris( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Iris( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Iris( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Zoom( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Zoom( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Zoom( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_FocalLengths( 
            /* [out] */ long *plOcularFocalLength,
            /* [out] */ long *plObjectiveFocalLengthMin,
            /* [out] */ long *plObjectiveFocalLengthMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Pan( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Pan( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Pan( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Tilt( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Tilt( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Tilt( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PanTilt( 
            /* [out] */ long *pPanValue,
            /* [out] */ long *pTiltValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PanTilt( 
            /* [in] */ long PanValue,
            /* [in] */ long TiltValue,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Roll( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Roll( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Roll( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ExposureRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_ExposureRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_ExposureRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_FocusRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_FocusRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_FocusRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IrisRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_IrisRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_IrisRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ZoomRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_ZoomRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_ZoomRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PanRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PanRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TiltRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_TiltRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_TiltRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PanTiltRelative( 
            /* [out] */ long *pPanValue,
            /* [out] */ long *pTiltValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PanTiltRelative( 
            /* [in] */ long PanValue,
            /* [in] */ long TiltValue,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_PanRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_RollRelative( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_RollRelative( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_RollRelative( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ScanMode( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_ScanMode( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PrivacyMode( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PrivacyMode( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICameraControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICameraControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICameraControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICameraControl * This);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Exposure)
        HRESULT ( STDMETHODCALLTYPE *get_Exposure )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Exposure)
        HRESULT ( STDMETHODCALLTYPE *put_Exposure )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Exposure)
        HRESULT ( STDMETHODCALLTYPE *getRange_Exposure )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Focus)
        HRESULT ( STDMETHODCALLTYPE *get_Focus )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Focus)
        HRESULT ( STDMETHODCALLTYPE *put_Focus )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Focus)
        HRESULT ( STDMETHODCALLTYPE *getRange_Focus )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Iris)
        HRESULT ( STDMETHODCALLTYPE *get_Iris )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Iris)
        HRESULT ( STDMETHODCALLTYPE *put_Iris )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Iris)
        HRESULT ( STDMETHODCALLTYPE *getRange_Iris )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Zoom)
        HRESULT ( STDMETHODCALLTYPE *get_Zoom )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Zoom)
        HRESULT ( STDMETHODCALLTYPE *put_Zoom )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Zoom)
        HRESULT ( STDMETHODCALLTYPE *getRange_Zoom )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_FocalLengths)
        HRESULT ( STDMETHODCALLTYPE *get_FocalLengths )( 
            ICameraControl * This,
            /* [out] */ long *plOcularFocalLength,
            /* [out] */ long *plObjectiveFocalLengthMin,
            /* [out] */ long *plObjectiveFocalLengthMax);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Pan)
        HRESULT ( STDMETHODCALLTYPE *get_Pan )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Pan)
        HRESULT ( STDMETHODCALLTYPE *put_Pan )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Pan)
        HRESULT ( STDMETHODCALLTYPE *getRange_Pan )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Tilt)
        HRESULT ( STDMETHODCALLTYPE *get_Tilt )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Tilt)
        HRESULT ( STDMETHODCALLTYPE *put_Tilt )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Tilt)
        HRESULT ( STDMETHODCALLTYPE *getRange_Tilt )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_PanTilt)
        HRESULT ( STDMETHODCALLTYPE *get_PanTilt )( 
            ICameraControl * This,
            /* [out] */ long *pPanValue,
            /* [out] */ long *pTiltValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_PanTilt)
        HRESULT ( STDMETHODCALLTYPE *put_PanTilt )( 
            ICameraControl * This,
            /* [in] */ long PanValue,
            /* [in] */ long TiltValue,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_Roll)
        HRESULT ( STDMETHODCALLTYPE *get_Roll )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_Roll)
        HRESULT ( STDMETHODCALLTYPE *put_Roll )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_Roll)
        HRESULT ( STDMETHODCALLTYPE *getRange_Roll )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_ExposureRelative)
        HRESULT ( STDMETHODCALLTYPE *get_ExposureRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_ExposureRelative)
        HRESULT ( STDMETHODCALLTYPE *put_ExposureRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_ExposureRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_ExposureRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_FocusRelative)
        HRESULT ( STDMETHODCALLTYPE *get_FocusRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_FocusRelative)
        HRESULT ( STDMETHODCALLTYPE *put_FocusRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_FocusRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_FocusRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_IrisRelative)
        HRESULT ( STDMETHODCALLTYPE *get_IrisRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_IrisRelative)
        HRESULT ( STDMETHODCALLTYPE *put_IrisRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_IrisRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_IrisRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_ZoomRelative)
        HRESULT ( STDMETHODCALLTYPE *get_ZoomRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_ZoomRelative)
        HRESULT ( STDMETHODCALLTYPE *put_ZoomRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_ZoomRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_ZoomRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_PanRelative)
        HRESULT ( STDMETHODCALLTYPE *get_PanRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_PanRelative)
        HRESULT ( STDMETHODCALLTYPE *put_PanRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_TiltRelative)
        HRESULT ( STDMETHODCALLTYPE *get_TiltRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_TiltRelative)
        HRESULT ( STDMETHODCALLTYPE *put_TiltRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_TiltRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_TiltRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_PanTiltRelative)
        HRESULT ( STDMETHODCALLTYPE *get_PanTiltRelative )( 
            ICameraControl * This,
            /* [out] */ long *pPanValue,
            /* [out] */ long *pTiltValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_PanTiltRelative)
        HRESULT ( STDMETHODCALLTYPE *put_PanTiltRelative )( 
            ICameraControl * This,
            /* [in] */ long PanValue,
            /* [in] */ long TiltValue,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_PanRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_PanRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_RollRelative)
        HRESULT ( STDMETHODCALLTYPE *get_RollRelative )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_RollRelative)
        HRESULT ( STDMETHODCALLTYPE *put_RollRelative )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, getRange_RollRelative)
        HRESULT ( STDMETHODCALLTYPE *getRange_RollRelative )( 
            ICameraControl * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_ScanMode)
        HRESULT ( STDMETHODCALLTYPE *get_ScanMode )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_ScanMode)
        HRESULT ( STDMETHODCALLTYPE *put_ScanMode )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(ICameraControl, get_PrivacyMode)
        HRESULT ( STDMETHODCALLTYPE *get_PrivacyMode )( 
            ICameraControl * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(ICameraControl, put_PrivacyMode)
        HRESULT ( STDMETHODCALLTYPE *put_PrivacyMode )( 
            ICameraControl * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        END_INTERFACE
    } ICameraControlVtbl;

    interface ICameraControl
    {
        CONST_VTBL struct ICameraControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICameraControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICameraControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICameraControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICameraControl_get_Exposure(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Exposure(This,pValue,pFlags) ) 

#define ICameraControl_put_Exposure(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Exposure(This,Value,Flags) ) 

#define ICameraControl_getRange_Exposure(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Exposure(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_Focus(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Focus(This,pValue,pFlags) ) 

#define ICameraControl_put_Focus(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Focus(This,Value,Flags) ) 

#define ICameraControl_getRange_Focus(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Focus(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_Iris(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Iris(This,pValue,pFlags) ) 

#define ICameraControl_put_Iris(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Iris(This,Value,Flags) ) 

#define ICameraControl_getRange_Iris(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Iris(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_Zoom(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Zoom(This,pValue,pFlags) ) 

#define ICameraControl_put_Zoom(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Zoom(This,Value,Flags) ) 

#define ICameraControl_getRange_Zoom(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Zoom(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_FocalLengths(This,plOcularFocalLength,plObjectiveFocalLengthMin,plObjectiveFocalLengthMax)	\
    ( (This)->lpVtbl -> get_FocalLengths(This,plOcularFocalLength,plObjectiveFocalLengthMin,plObjectiveFocalLengthMax) ) 

#define ICameraControl_get_Pan(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Pan(This,pValue,pFlags) ) 

#define ICameraControl_put_Pan(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Pan(This,Value,Flags) ) 

#define ICameraControl_getRange_Pan(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Pan(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_Tilt(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Tilt(This,pValue,pFlags) ) 

#define ICameraControl_put_Tilt(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Tilt(This,Value,Flags) ) 

#define ICameraControl_getRange_Tilt(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Tilt(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_PanTilt(This,pPanValue,pTiltValue,pFlags)	\
    ( (This)->lpVtbl -> get_PanTilt(This,pPanValue,pTiltValue,pFlags) ) 

#define ICameraControl_put_PanTilt(This,PanValue,TiltValue,Flags)	\
    ( (This)->lpVtbl -> put_PanTilt(This,PanValue,TiltValue,Flags) ) 

#define ICameraControl_get_Roll(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Roll(This,pValue,pFlags) ) 

#define ICameraControl_put_Roll(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Roll(This,Value,Flags) ) 

#define ICameraControl_getRange_Roll(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Roll(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_ExposureRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_ExposureRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_ExposureRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_ExposureRelative(This,Value,Flags) ) 

#define ICameraControl_getRange_ExposureRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_ExposureRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_FocusRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_FocusRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_FocusRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_FocusRelative(This,Value,Flags) ) 

#define ICameraControl_getRange_FocusRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_FocusRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_IrisRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_IrisRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_IrisRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_IrisRelative(This,Value,Flags) ) 

#define ICameraControl_getRange_IrisRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_IrisRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_ZoomRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_ZoomRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_ZoomRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_ZoomRelative(This,Value,Flags) ) 

#define ICameraControl_getRange_ZoomRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_ZoomRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_PanRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_PanRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_PanRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_PanRelative(This,Value,Flags) ) 

#define ICameraControl_get_TiltRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_TiltRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_TiltRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_TiltRelative(This,Value,Flags) ) 

#define ICameraControl_getRange_TiltRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_TiltRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_PanTiltRelative(This,pPanValue,pTiltValue,pFlags)	\
    ( (This)->lpVtbl -> get_PanTiltRelative(This,pPanValue,pTiltValue,pFlags) ) 

#define ICameraControl_put_PanTiltRelative(This,PanValue,TiltValue,Flags)	\
    ( (This)->lpVtbl -> put_PanTiltRelative(This,PanValue,TiltValue,Flags) ) 

#define ICameraControl_getRange_PanRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_PanRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_RollRelative(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_RollRelative(This,pValue,pFlags) ) 

#define ICameraControl_put_RollRelative(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_RollRelative(This,Value,Flags) ) 

#define ICameraControl_getRange_RollRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_RollRelative(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define ICameraControl_get_ScanMode(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_ScanMode(This,pValue,pFlags) ) 

#define ICameraControl_put_ScanMode(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_ScanMode(This,Value,Flags) ) 

#define ICameraControl_get_PrivacyMode(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_PrivacyMode(This,pValue,pFlags) ) 

#define ICameraControl_put_PrivacyMode(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_PrivacyMode(This,Value,Flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICameraControl_INTERFACE_DEFINED__ */


#ifndef __IVideoProcAmp_INTERFACE_DEFINED__
#define __IVideoProcAmp_INTERFACE_DEFINED__

/* interface IVideoProcAmp */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IVideoProcAmp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4050560E-42A7-413a-85C2-09269A2D0F44")
    IVideoProcAmp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_BacklightCompensation( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_BacklightCompensation( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_BacklightCompensation( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Brightness( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Brightness( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Brightness( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ColorEnable( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_ColorEnable( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_ColorEnable( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Contrast( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Contrast( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Contrast( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Gamma( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Gamma( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Gamma( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Saturation( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Saturation( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Saturation( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Sharpness( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Sharpness( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Sharpness( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_WhiteBalance( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_WhiteBalance( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_WhiteBalance( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Gain( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Gain( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Gain( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Hue( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Hue( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_Hue( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DigitalMultiplier( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DigitalMultiplier( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_DigitalMultiplier( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PowerlineFrequency( 
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PowerlineFrequency( 
            /* [in] */ long Value,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_PowerlineFrequency( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_WhiteBalanceComponent( 
            /* [out] */ long *pValue1,
            /* [out] */ long *pValue2,
            /* [out][in] */ long *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_WhiteBalanceComponent( 
            /* [in] */ long Value1,
            /* [in] */ long Value2,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE getRange_WhiteBalanceComponent( 
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out][in] */ long *pCapsFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVideoProcAmpVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVideoProcAmp * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVideoProcAmp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVideoProcAmp * This);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_BacklightCompensation)
        HRESULT ( STDMETHODCALLTYPE *get_BacklightCompensation )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_BacklightCompensation)
        HRESULT ( STDMETHODCALLTYPE *put_BacklightCompensation )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_BacklightCompensation)
        HRESULT ( STDMETHODCALLTYPE *getRange_BacklightCompensation )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Brightness)
        HRESULT ( STDMETHODCALLTYPE *get_Brightness )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Brightness)
        HRESULT ( STDMETHODCALLTYPE *put_Brightness )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Brightness)
        HRESULT ( STDMETHODCALLTYPE *getRange_Brightness )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_ColorEnable)
        HRESULT ( STDMETHODCALLTYPE *get_ColorEnable )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_ColorEnable)
        HRESULT ( STDMETHODCALLTYPE *put_ColorEnable )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_ColorEnable)
        HRESULT ( STDMETHODCALLTYPE *getRange_ColorEnable )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Contrast)
        HRESULT ( STDMETHODCALLTYPE *get_Contrast )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Contrast)
        HRESULT ( STDMETHODCALLTYPE *put_Contrast )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Contrast)
        HRESULT ( STDMETHODCALLTYPE *getRange_Contrast )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Gamma)
        HRESULT ( STDMETHODCALLTYPE *get_Gamma )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Gamma)
        HRESULT ( STDMETHODCALLTYPE *put_Gamma )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Gamma)
        HRESULT ( STDMETHODCALLTYPE *getRange_Gamma )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Saturation)
        HRESULT ( STDMETHODCALLTYPE *get_Saturation )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Saturation)
        HRESULT ( STDMETHODCALLTYPE *put_Saturation )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Saturation)
        HRESULT ( STDMETHODCALLTYPE *getRange_Saturation )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Sharpness)
        HRESULT ( STDMETHODCALLTYPE *get_Sharpness )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Sharpness)
        HRESULT ( STDMETHODCALLTYPE *put_Sharpness )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Sharpness)
        HRESULT ( STDMETHODCALLTYPE *getRange_Sharpness )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_WhiteBalance)
        HRESULT ( STDMETHODCALLTYPE *get_WhiteBalance )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_WhiteBalance)
        HRESULT ( STDMETHODCALLTYPE *put_WhiteBalance )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_WhiteBalance)
        HRESULT ( STDMETHODCALLTYPE *getRange_WhiteBalance )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Gain)
        HRESULT ( STDMETHODCALLTYPE *get_Gain )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Gain)
        HRESULT ( STDMETHODCALLTYPE *put_Gain )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Gain)
        HRESULT ( STDMETHODCALLTYPE *getRange_Gain )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_Hue)
        HRESULT ( STDMETHODCALLTYPE *get_Hue )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_Hue)
        HRESULT ( STDMETHODCALLTYPE *put_Hue )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_Hue)
        HRESULT ( STDMETHODCALLTYPE *getRange_Hue )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_DigitalMultiplier)
        HRESULT ( STDMETHODCALLTYPE *get_DigitalMultiplier )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_DigitalMultiplier)
        HRESULT ( STDMETHODCALLTYPE *put_DigitalMultiplier )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_DigitalMultiplier)
        HRESULT ( STDMETHODCALLTYPE *getRange_DigitalMultiplier )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_PowerlineFrequency)
        HRESULT ( STDMETHODCALLTYPE *get_PowerlineFrequency )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue,
            /* [out] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_PowerlineFrequency)
        HRESULT ( STDMETHODCALLTYPE *put_PowerlineFrequency )( 
            IVideoProcAmp * This,
            /* [in] */ long Value,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_PowerlineFrequency)
        HRESULT ( STDMETHODCALLTYPE *getRange_PowerlineFrequency )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out] */ long *pCapsFlag);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, get_WhiteBalanceComponent)
        HRESULT ( STDMETHODCALLTYPE *get_WhiteBalanceComponent )( 
            IVideoProcAmp * This,
            /* [out] */ long *pValue1,
            /* [out] */ long *pValue2,
            /* [out][in] */ long *pFlags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, put_WhiteBalanceComponent)
        HRESULT ( STDMETHODCALLTYPE *put_WhiteBalanceComponent )( 
            IVideoProcAmp * This,
            /* [in] */ long Value1,
            /* [in] */ long Value2,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IVideoProcAmp, getRange_WhiteBalanceComponent)
        HRESULT ( STDMETHODCALLTYPE *getRange_WhiteBalanceComponent )( 
            IVideoProcAmp * This,
            /* [out] */ long *pMin,
            /* [out] */ long *pMax,
            /* [out] */ long *pSteppingDelta,
            /* [out] */ long *pDefault,
            /* [out][in] */ long *pCapsFlag);
        
        END_INTERFACE
    } IVideoProcAmpVtbl;

    interface IVideoProcAmp
    {
        CONST_VTBL struct IVideoProcAmpVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVideoProcAmp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVideoProcAmp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVideoProcAmp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVideoProcAmp_get_BacklightCompensation(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_BacklightCompensation(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_BacklightCompensation(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_BacklightCompensation(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_BacklightCompensation(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_BacklightCompensation(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Brightness(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Brightness(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Brightness(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Brightness(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Brightness(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Brightness(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_ColorEnable(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_ColorEnable(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_ColorEnable(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_ColorEnable(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_ColorEnable(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_ColorEnable(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Contrast(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Contrast(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Contrast(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Contrast(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Contrast(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Contrast(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Gamma(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Gamma(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Gamma(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Gamma(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Gamma(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Gamma(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Saturation(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Saturation(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Saturation(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Saturation(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Saturation(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Saturation(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Sharpness(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Sharpness(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Sharpness(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Sharpness(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Sharpness(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Sharpness(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_WhiteBalance(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_WhiteBalance(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_WhiteBalance(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_WhiteBalance(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_WhiteBalance(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_WhiteBalance(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Gain(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Gain(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Gain(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Gain(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Gain(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Gain(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_Hue(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_Hue(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_Hue(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_Hue(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_Hue(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_Hue(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_DigitalMultiplier(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_DigitalMultiplier(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_DigitalMultiplier(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_DigitalMultiplier(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_DigitalMultiplier(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_DigitalMultiplier(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_PowerlineFrequency(This,pValue,pFlags)	\
    ( (This)->lpVtbl -> get_PowerlineFrequency(This,pValue,pFlags) ) 

#define IVideoProcAmp_put_PowerlineFrequency(This,Value,Flags)	\
    ( (This)->lpVtbl -> put_PowerlineFrequency(This,Value,Flags) ) 

#define IVideoProcAmp_getRange_PowerlineFrequency(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_PowerlineFrequency(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#define IVideoProcAmp_get_WhiteBalanceComponent(This,pValue1,pValue2,pFlags)	\
    ( (This)->lpVtbl -> get_WhiteBalanceComponent(This,pValue1,pValue2,pFlags) ) 

#define IVideoProcAmp_put_WhiteBalanceComponent(This,Value1,Value2,Flags)	\
    ( (This)->lpVtbl -> put_WhiteBalanceComponent(This,Value1,Value2,Flags) ) 

#define IVideoProcAmp_getRange_WhiteBalanceComponent(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag)	\
    ( (This)->lpVtbl -> getRange_WhiteBalanceComponent(This,pMin,pMax,pSteppingDelta,pDefault,pCapsFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVideoProcAmp_INTERFACE_DEFINED__ */


#ifndef __IKsNodeControl_INTERFACE_DEFINED__
#define __IKsNodeControl_INTERFACE_DEFINED__

/* interface IKsNodeControl */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IKsNodeControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11737C14-24A7-4bb5-81A0-0D003813B0C4")
    IKsNodeControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_NodeId( 
            /* [in] */ DWORD dwNodeId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_KsControl( 
            /* [in] */ PVOID pKsControl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IKsNodeControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IKsNodeControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IKsNodeControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IKsNodeControl * This);
        
        DECLSPEC_XFGVIRT(IKsNodeControl, put_NodeId)
        HRESULT ( STDMETHODCALLTYPE *put_NodeId )( 
            IKsNodeControl * This,
            /* [in] */ DWORD dwNodeId);
        
        DECLSPEC_XFGVIRT(IKsNodeControl, put_KsControl)
        HRESULT ( STDMETHODCALLTYPE *put_KsControl )( 
            IKsNodeControl * This,
            /* [in] */ PVOID pKsControl);
        
        END_INTERFACE
    } IKsNodeControlVtbl;

    interface IKsNodeControl
    {
        CONST_VTBL struct IKsNodeControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IKsNodeControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IKsNodeControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IKsNodeControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IKsNodeControl_put_NodeId(This,dwNodeId)	\
    ( (This)->lpVtbl -> put_NodeId(This,dwNodeId) ) 

#define IKsNodeControl_put_KsControl(This,pKsControl)	\
    ( (This)->lpVtbl -> put_KsControl(This,pKsControl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IKsNodeControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_vidcap_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vidcap_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vidcap_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


