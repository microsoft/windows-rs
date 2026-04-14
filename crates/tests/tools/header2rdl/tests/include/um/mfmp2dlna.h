

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

#ifndef __mfmp2dlna_h__
#define __mfmp2dlna_h__

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

#ifndef __IMFDLNASinkInit_FWD_DEFINED__
#define __IMFDLNASinkInit_FWD_DEFINED__
typedef interface IMFDLNASinkInit IMFDLNASinkInit;

#endif 	/* __IMFDLNASinkInit_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfmp2dlna_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_mfmp2dlna_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmp2dlna_0000_0000_v0_0_s_ifspec;

#ifndef __IMFDLNASinkInit_INTERFACE_DEFINED__
#define __IMFDLNASinkInit_INTERFACE_DEFINED__

/* interface IMFDLNASinkInit */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFDLNASinkInit;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0c012799-1b61-4c10-bda9-04445be5f561")
    IMFDLNASinkInit : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            IMFByteStream *pByteStream,
            BOOL fPal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDLNASinkInitVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDLNASinkInit * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDLNASinkInit * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDLNASinkInit * This);
        
        DECLSPEC_XFGVIRT(IMFDLNASinkInit, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IMFDLNASinkInit * This,
            IMFByteStream *pByteStream,
            BOOL fPal);
        
        END_INTERFACE
    } IMFDLNASinkInitVtbl;

    interface IMFDLNASinkInit
    {
        CONST_VTBL struct IMFDLNASinkInitVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDLNASinkInit_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDLNASinkInit_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDLNASinkInit_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDLNASinkInit_Initialize(This,pByteStream,fPal)	\
    ( (This)->lpVtbl -> Initialize(This,pByteStream,fPal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDLNASinkInit_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmp2dlna_0000_0001 */
/* [local] */ 

EXTERN_GUID( CLSID_MPEG2DLNASink, 0xfa5fe7c5, 0x6a1d, 0x4b11, 0xb4, 0x1f, 0xf9, 0x59, 0xd6, 0xc7, 0x65, 0x00 );
EXTERN_GUID( MF_MP2DLNA_USE_MMCSS, 0x54f3e2ee, 0xa2a2, 0x497d, 0x98, 0x34, 0x97, 0x3a, 0xfd, 0xe5, 0x21, 0xeb );
EXTERN_GUID( MF_MP2DLNA_VIDEO_BIT_RATE, 0xe88548de, 0x73b4, 0x42d7, 0x9c, 0x75, 0xad, 0xfa, 0xa, 0x2a, 0x6e, 0x4c );
EXTERN_GUID( MF_MP2DLNA_AUDIO_BIT_RATE, 0x2d1c070e, 0x2b5f, 0x4ab3, 0xa7, 0xe6, 0x8d, 0x94, 0x3b, 0xa8, 0xd0, 0x0a);
EXTERN_GUID( MF_MP2DLNA_ENCODE_QUALITY, 0xb52379d7, 0x1d46, 0x4fb6, 0xa3, 0x17, 0xa4, 0xa5, 0xf6, 0x09, 0x59, 0xf8);
EXTERN_GUID( MF_MP2DLNA_STATISTICS, 0x75e488a3, 0xd5ad, 0x4898, 0x85, 0xe0, 0xbc, 0xce, 0x24, 0xa7, 0x22, 0xd7 );
typedef struct _MFMPEG2DLNASINKSTATS         
{                                            
   DWORDLONG  cBytesWritten;                 
   BOOL       fPAL;                          
   DWORD      fccVideo;                      
   DWORD      dwVideoWidth;                  
   DWORD      dwVideoHeight;                 
   DWORDLONG  cVideoFramesReceived;          
   DWORDLONG  cVideoFramesEncoded;           
   DWORDLONG  cVideoFramesSkipped;           
   DWORDLONG  cBlackVideoFramesEncoded;      
   DWORDLONG  cVideoFramesDuplicated;        
   DWORD      cAudioSamplesPerSec;           
   DWORD      cAudioChannels;                
   DWORDLONG  cAudioBytesReceived;           
   DWORDLONG  cAudioFramesEncoded;           
} MFMPEG2DLNASINKSTATS;                      
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfmp2dlna_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmp2dlna_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


