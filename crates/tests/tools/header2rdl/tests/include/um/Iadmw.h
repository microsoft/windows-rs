

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

#ifndef __iadmw_h__
#define __iadmw_h__

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

#ifndef __IMSAdminBaseW_FWD_DEFINED__
#define __IMSAdminBaseW_FWD_DEFINED__
typedef interface IMSAdminBaseW IMSAdminBaseW;

#endif 	/* __IMSAdminBaseW_FWD_DEFINED__ */


#ifndef __IMSAdminBase2W_FWD_DEFINED__
#define __IMSAdminBase2W_FWD_DEFINED__
typedef interface IMSAdminBase2W IMSAdminBase2W;

#endif 	/* __IMSAdminBase2W_FWD_DEFINED__ */


#ifndef __IMSAdminBase3W_FWD_DEFINED__
#define __IMSAdminBase3W_FWD_DEFINED__
typedef interface IMSAdminBase3W IMSAdminBase3W;

#endif 	/* __IMSAdminBase3W_FWD_DEFINED__ */


#ifndef __IMSImpExpHelpW_FWD_DEFINED__
#define __IMSImpExpHelpW_FWD_DEFINED__
typedef interface IMSImpExpHelpW IMSImpExpHelpW;

#endif 	/* __IMSImpExpHelpW_FWD_DEFINED__ */


#ifndef __IMSAdminBaseSinkW_FWD_DEFINED__
#define __IMSAdminBaseSinkW_FWD_DEFINED__
typedef interface IMSAdminBaseSinkW IMSAdminBaseSinkW;

#endif 	/* __IMSAdminBaseSinkW_FWD_DEFINED__ */


#ifndef __AsyncIMSAdminBaseSinkW_FWD_DEFINED__
#define __AsyncIMSAdminBaseSinkW_FWD_DEFINED__
typedef interface AsyncIMSAdminBaseSinkW AsyncIMSAdminBaseSinkW;

#endif 	/* __AsyncIMSAdminBaseSinkW_FWD_DEFINED__ */


/* header files for imported files */
#include "mddefw.h"
#include "objidl.h"
#include "ocidl.h"


#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iadmw_0000_0000 */
/* [local] */ 

/*++
                                                                                
Copyright (c) 1997-1999 Microsoft Corporation
                                                                                
Module Name: iadmw.h
                                                                                
    Admin Objects Interfaces
                                                                                
--*/
#ifndef _ADM_IADMW_
#define _ADM_IADMW_
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <mdcommsg.h>
#include <mdmsg.h>
/*                                                                              
    Error Codes                                                                 
                                                                                
        Admin api's all return HRESULTS. Since internal results are either   
        winerrors or Metadata specific return codes (see mdmsg.h), they are     
        converted to HRESULTS using the RETURNCODETOHRESULT macro (see          
        commsg.h).                                                              
*/                                                                              
                                                                                
/*                                                                              
    Max Name Length                                                             
        The maximum number of characters in the length of a metaobject name,    
        including the terminating NULL. This refers to each node in the tree,   
        not the entire path.                                                    
        eg. strlen("Root") < ADMINDATA_MAX_NAME_LEN                           
*/                                                                              
#define ADMINDATA_MAX_NAME_LEN           256
                                                                 
#define CLSID_MSAdminBase            CLSID_MSAdminBase_W         
#define IID_IMSAdminBase             IID_IMSAdminBase_W          
#define IMSAdminBase                 IMSAdminBaseW               
#define IID_IMSAdminBase2            IID_IMSAdminBase2_W         
#define IMSAdminBase2                IMSAdminBase2W              
#define IID_IMSAdminBase3            IID_IMSAdminBase3_W         
#define IMSAdminBase3                IMSAdminBase3W              
#define IMSAdminBaseSink             IMSAdminBaseSinkW           
#define IID_IMSAdminBaseSink         IID_IMSAdminBaseSink_W      
#define IMSImpExpHelp                IMSImpExpHelpW              
#define IID_IMSImpExpHelp            IID_IMSImpExpHelp_W         
#define GETAdminBaseCLSID            GETAdminBaseCLSIDW          
                                                                 
#define AsyncIMSAdminBaseSink        AsyncIMSAdminBaseSinkW      
#define IID_AsyncIMSAdminBaseSink    IID_AsyncIMSAdminBaseSink_W 
DEFINE_GUID(CLSID_MSAdminBase_W,         0xa9e69610, 0xb80d, 0x11d0, 0xb9, 0xb9, 0x00, 0xa0, 0xc9, 0x22, 0xe7, 0x50);
DEFINE_GUID(IID_IMSAdminBase_W,          0x70b51430, 0xb6ca, 0x11d0, 0xb9, 0xb9, 0x00, 0xa0, 0xc9, 0x22, 0xe7, 0x50);
DEFINE_GUID(IID_IMSAdminBase2_W,         0x8298d101, 0xf992, 0x43b7, 0x8e, 0xca, 0x50, 0x52, 0xd8, 0x85, 0xb9, 0x95);
DEFINE_GUID(IID_IMSAdminBase3_W,         0xf612954d, 0x3b0b, 0x4c56, 0x95, 0x63, 0x22, 0x7b, 0x7b, 0xe6, 0x24, 0xb4);
DEFINE_GUID(IID_IMSImpExpHelp_W,         0x29ff67ff, 0x8050, 0x480f, 0x9f, 0x30, 0xcc, 0x41, 0x63, 0x5f, 0x2f, 0x9d);
DEFINE_GUID(IID_IMSAdminBaseSink_W,      0xa9e69612, 0xb80d, 0x11d0, 0xb9, 0xb9, 0x00, 0xa0, 0xc9, 0x22, 0xe7, 0x50);
DEFINE_GUID(IID_AsyncIMSAdminBaseSink_W, 0xa9e69613, 0xb80d, 0x11d0, 0xb9, 0xb9, 0x00, 0xa0, 0xc9, 0x22, 0xe7, 0x50);
#define GETAdminBaseCLSIDW(IsService)    CLSID_MSAdminBase_W
/*                                                                              
The Main Interface, UNICODE                                                     
*/                                                                              


extern RPC_IF_HANDLE __MIDL_itf_iadmw_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iadmw_0000_0000_v0_0_s_ifspec;

#ifndef __IMSAdminBaseW_INTERFACE_DEFINED__
#define __IMSAdminBaseW_INTERFACE_DEFINED__

/* interface IMSAdminBaseW */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMSAdminBaseW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70B51430-B6CA-11d0-B9B9-00A0C922E750")
    IMSAdminBaseW : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddKey( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteKey( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteChildKeys( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumKeys( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [size_is][out] */ __RPC__out_ecount_full(256) LPWSTR pszMDName,
            /* [in] */ DWORD dwMDEnumObjectIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyKey( 
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ BOOL bMDOverwriteFlag,
            /* [in] */ BOOL bMDCopyFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenameKey( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDNewName) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetData( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ PMETADATA_RECORD pmdrMDData) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetData( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [out] */ DWORD *pdwMDRequiredDataLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteData( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EnumData( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [in] */ DWORD dwMDEnumDataIndex,
            /* [out] */ DWORD *pdwMDRequiredDataLen) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetAllData( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [out] */ DWORD *pdwMDNumDataEntries,
            /* [out] */ DWORD *pdwMDDataSetNumber,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ unsigned char *pbMDBuffer,
            /* [out] */ DWORD *pdwMDRequiredBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAllData( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyData( 
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ BOOL bMDCopyFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataPaths( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwMDBufferSize) WCHAR *pszBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenKey( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAccessRequested,
            /* [in] */ DWORD dwMDTimeOut,
            /* [out] */ __RPC__out PMETADATA_HANDLE phMDNewHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseKey( 
            /* [in] */ METADATA_HANDLE hMDHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangePermissions( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [in] */ DWORD dwMDTimeOut,
            /* [in] */ DWORD dwMDAccessRequested) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveData( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHandleInfo( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [out] */ __RPC__out PMETADATA_HANDLE_INFO pmdhiInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSystemChangeNumber( 
            /* [out] */ __RPC__out DWORD *pdwSystemChangeNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataSetNumber( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out DWORD *pdwMDDataSetNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLastChangeTime( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ __RPC__in PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastChangeTime( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime) = 0;
        
        virtual /* [restricted][local] */ HRESULT STDMETHODCALLTYPE KeyExchangePhase1( void) = 0;
        
        virtual /* [restricted][local] */ HRESULT STDMETHODCALLTYPE KeyExchangePhase2( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Backup( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Restore( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumBackups( 
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDBackupLocation,
            /* [out] */ __RPC__out DWORD *pdwMDVersion,
            /* [out] */ __RPC__out PFILETIME pftMDBackupTime,
            /* [in] */ DWORD dwMDEnumIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteBackup( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnmarshalInterface( 
            /* [out] */ __RPC__deref_out_opt IMSAdminBaseW **piadmbwInterface) = 0;
        
        virtual /* [restricted][local] */ HRESULT STDMETHODCALLTYPE GetServerGuid( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSAdminBaseWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSAdminBaseW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSAdminBaseW * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, AddKey)
        HRESULT ( STDMETHODCALLTYPE *AddKey )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteKey)
        HRESULT ( STDMETHODCALLTYPE *DeleteKey )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteChildKeys)
        HRESULT ( STDMETHODCALLTYPE *DeleteChildKeys )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumKeys)
        HRESULT ( STDMETHODCALLTYPE *EnumKeys )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [size_is][out] */ __RPC__out_ecount_full(256) LPWSTR pszMDName,
            /* [in] */ DWORD dwMDEnumObjectIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CopyKey)
        HRESULT ( STDMETHODCALLTYPE *CopyKey )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ BOOL bMDOverwriteFlag,
            /* [in] */ BOOL bMDCopyFlag);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, RenameKey)
        HRESULT ( STDMETHODCALLTYPE *RenameKey )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDNewName);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetData )( 
            IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ PMETADATA_RECORD pmdrMDData);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [out] */ DWORD *pdwMDRequiredDataLen);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteData)
        HRESULT ( STDMETHODCALLTYPE *DeleteData )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EnumData )( 
            IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [in] */ DWORD dwMDEnumDataIndex,
            /* [out] */ DWORD *pdwMDRequiredDataLen);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetAllData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetAllData )( 
            IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [out] */ DWORD *pdwMDNumDataEntries,
            /* [out] */ DWORD *pdwMDDataSetNumber,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ unsigned char *pbMDBuffer,
            /* [out] */ DWORD *pdwMDRequiredBufferSize);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteAllData)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllData )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CopyData)
        HRESULT ( STDMETHODCALLTYPE *CopyData )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ BOOL bMDCopyFlag);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetDataPaths)
        HRESULT ( STDMETHODCALLTYPE *GetDataPaths )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwMDBufferSize) WCHAR *pszBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, OpenKey)
        HRESULT ( STDMETHODCALLTYPE *OpenKey )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAccessRequested,
            /* [in] */ DWORD dwMDTimeOut,
            /* [out] */ __RPC__out PMETADATA_HANDLE phMDNewHandle);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CloseKey)
        HRESULT ( STDMETHODCALLTYPE *CloseKey )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, ChangePermissions)
        HRESULT ( STDMETHODCALLTYPE *ChangePermissions )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [in] */ DWORD dwMDTimeOut,
            /* [in] */ DWORD dwMDAccessRequested);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SaveData)
        HRESULT ( STDMETHODCALLTYPE *SaveData )( 
            __RPC__in IMSAdminBaseW * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetHandleInfo)
        HRESULT ( STDMETHODCALLTYPE *GetHandleInfo )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [out] */ __RPC__out PMETADATA_HANDLE_INFO pmdhiInfo);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetSystemChangeNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSystemChangeNumber )( 
            __RPC__in IMSAdminBaseW * This,
            /* [out] */ __RPC__out DWORD *pdwSystemChangeNumber);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetDataSetNumber)
        HRESULT ( STDMETHODCALLTYPE *GetDataSetNumber )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out DWORD *pdwMDDataSetNumber);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SetLastChangeTime)
        HRESULT ( STDMETHODCALLTYPE *SetLastChangeTime )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ __RPC__in PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetLastChangeTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastChangeTime )( 
            __RPC__in IMSAdminBaseW * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, KeyExchangePhase1)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *KeyExchangePhase1 )( 
            IMSAdminBaseW * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, KeyExchangePhase2)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *KeyExchangePhase2 )( 
            IMSAdminBaseW * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, Backup)
        HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IMSAdminBaseW * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, Restore)
        HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in IMSAdminBaseW * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumBackups)
        HRESULT ( STDMETHODCALLTYPE *EnumBackups )( 
            __RPC__in IMSAdminBaseW * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDBackupLocation,
            /* [out] */ __RPC__out DWORD *pdwMDVersion,
            /* [out] */ __RPC__out PFILETIME pftMDBackupTime,
            /* [in] */ DWORD dwMDEnumIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteBackup)
        HRESULT ( STDMETHODCALLTYPE *DeleteBackup )( 
            __RPC__in IMSAdminBaseW * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, UnmarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalInterface )( 
            __RPC__in IMSAdminBaseW * This,
            /* [out] */ __RPC__deref_out_opt IMSAdminBaseW **piadmbwInterface);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetServerGuid)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *GetServerGuid )( 
            IMSAdminBaseW * This);
        
        END_INTERFACE
    } IMSAdminBaseWVtbl;

    interface IMSAdminBaseW
    {
        CONST_VTBL struct IMSAdminBaseWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSAdminBaseW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSAdminBaseW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSAdminBaseW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSAdminBaseW_AddKey(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> AddKey(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBaseW_DeleteKey(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> DeleteKey(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBaseW_DeleteChildKeys(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> DeleteChildKeys(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBaseW_EnumKeys(This,hMDHandle,pszMDPath,pszMDName,dwMDEnumObjectIndex)	\
    ( (This)->lpVtbl -> EnumKeys(This,hMDHandle,pszMDPath,pszMDName,dwMDEnumObjectIndex) ) 

#define IMSAdminBaseW_CopyKey(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,bMDOverwriteFlag,bMDCopyFlag)	\
    ( (This)->lpVtbl -> CopyKey(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,bMDOverwriteFlag,bMDCopyFlag) ) 

#define IMSAdminBaseW_RenameKey(This,hMDHandle,pszMDPath,pszMDNewName)	\
    ( (This)->lpVtbl -> RenameKey(This,hMDHandle,pszMDPath,pszMDNewName) ) 

#define IMSAdminBaseW_SetData(This,hMDHandle,pszMDPath,pmdrMDData)	\
    ( (This)->lpVtbl -> SetData(This,hMDHandle,pszMDPath,pmdrMDData) ) 

#define IMSAdminBaseW_GetData(This,hMDHandle,pszMDPath,pmdrMDData,pdwMDRequiredDataLen)	\
    ( (This)->lpVtbl -> GetData(This,hMDHandle,pszMDPath,pmdrMDData,pdwMDRequiredDataLen) ) 

#define IMSAdminBaseW_DeleteData(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType)	\
    ( (This)->lpVtbl -> DeleteData(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType) ) 

#define IMSAdminBaseW_EnumData(This,hMDHandle,pszMDPath,pmdrMDData,dwMDEnumDataIndex,pdwMDRequiredDataLen)	\
    ( (This)->lpVtbl -> EnumData(This,hMDHandle,pszMDPath,pmdrMDData,dwMDEnumDataIndex,pdwMDRequiredDataLen) ) 

#define IMSAdminBaseW_GetAllData(This,hMDHandle,pszMDPath,dwMDAttributes,dwMDUserType,dwMDDataType,pdwMDNumDataEntries,pdwMDDataSetNumber,dwMDBufferSize,pbMDBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetAllData(This,hMDHandle,pszMDPath,dwMDAttributes,dwMDUserType,dwMDDataType,pdwMDNumDataEntries,pdwMDDataSetNumber,dwMDBufferSize,pbMDBuffer,pdwMDRequiredBufferSize) ) 

#define IMSAdminBaseW_DeleteAllData(This,hMDHandle,pszMDPath,dwMDUserType,dwMDDataType)	\
    ( (This)->lpVtbl -> DeleteAllData(This,hMDHandle,pszMDPath,dwMDUserType,dwMDDataType) ) 

#define IMSAdminBaseW_CopyData(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,dwMDAttributes,dwMDUserType,dwMDDataType,bMDCopyFlag)	\
    ( (This)->lpVtbl -> CopyData(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,dwMDAttributes,dwMDUserType,dwMDDataType,bMDCopyFlag) ) 

#define IMSAdminBaseW_GetDataPaths(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetDataPaths(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize) ) 

#define IMSAdminBaseW_OpenKey(This,hMDHandle,pszMDPath,dwMDAccessRequested,dwMDTimeOut,phMDNewHandle)	\
    ( (This)->lpVtbl -> OpenKey(This,hMDHandle,pszMDPath,dwMDAccessRequested,dwMDTimeOut,phMDNewHandle) ) 

#define IMSAdminBaseW_CloseKey(This,hMDHandle)	\
    ( (This)->lpVtbl -> CloseKey(This,hMDHandle) ) 

#define IMSAdminBaseW_ChangePermissions(This,hMDHandle,dwMDTimeOut,dwMDAccessRequested)	\
    ( (This)->lpVtbl -> ChangePermissions(This,hMDHandle,dwMDTimeOut,dwMDAccessRequested) ) 

#define IMSAdminBaseW_SaveData(This)	\
    ( (This)->lpVtbl -> SaveData(This) ) 

#define IMSAdminBaseW_GetHandleInfo(This,hMDHandle,pmdhiInfo)	\
    ( (This)->lpVtbl -> GetHandleInfo(This,hMDHandle,pmdhiInfo) ) 

#define IMSAdminBaseW_GetSystemChangeNumber(This,pdwSystemChangeNumber)	\
    ( (This)->lpVtbl -> GetSystemChangeNumber(This,pdwSystemChangeNumber) ) 

#define IMSAdminBaseW_GetDataSetNumber(This,hMDHandle,pszMDPath,pdwMDDataSetNumber)	\
    ( (This)->lpVtbl -> GetDataSetNumber(This,hMDHandle,pszMDPath,pdwMDDataSetNumber) ) 

#define IMSAdminBaseW_SetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime)	\
    ( (This)->lpVtbl -> SetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime) ) 

#define IMSAdminBaseW_GetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime)	\
    ( (This)->lpVtbl -> GetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime) ) 

#define IMSAdminBaseW_KeyExchangePhase1(This)	\
    ( (This)->lpVtbl -> KeyExchangePhase1(This) ) 

#define IMSAdminBaseW_KeyExchangePhase2(This)	\
    ( (This)->lpVtbl -> KeyExchangePhase2(This) ) 

#define IMSAdminBaseW_Backup(This,pszMDBackupLocation,dwMDVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> Backup(This,pszMDBackupLocation,dwMDVersion,dwMDFlags) ) 

#define IMSAdminBaseW_Restore(This,pszMDBackupLocation,dwMDVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> Restore(This,pszMDBackupLocation,dwMDVersion,dwMDFlags) ) 

#define IMSAdminBaseW_EnumBackups(This,pszMDBackupLocation,pdwMDVersion,pftMDBackupTime,dwMDEnumIndex)	\
    ( (This)->lpVtbl -> EnumBackups(This,pszMDBackupLocation,pdwMDVersion,pftMDBackupTime,dwMDEnumIndex) ) 

#define IMSAdminBaseW_DeleteBackup(This,pszMDBackupLocation,dwMDVersion)	\
    ( (This)->lpVtbl -> DeleteBackup(This,pszMDBackupLocation,dwMDVersion) ) 

#define IMSAdminBaseW_UnmarshalInterface(This,piadmbwInterface)	\
    ( (This)->lpVtbl -> UnmarshalInterface(This,piadmbwInterface) ) 

#define IMSAdminBaseW_GetServerGuid(This)	\
    ( (This)->lpVtbl -> GetServerGuid(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_SetData_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [in] */ __RPC__in PMETADATA_RECORD pmdrMDData);


void __RPC_STUB IMSAdminBaseW_R_SetData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_GetData_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [out][in] */ __RPC__inout PMETADATA_RECORD pmdrMDData,
    /* [out] */ __RPC__out DWORD *pdwMDRequiredDataLen,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppDataBlob);


void __RPC_STUB IMSAdminBaseW_R_GetData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_EnumData_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [out][in] */ __RPC__inout PMETADATA_RECORD pmdrMDData,
    /* [in] */ DWORD dwMDEnumDataIndex,
    /* [out] */ __RPC__out DWORD *pdwMDRequiredDataLen,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppDataBlob);


void __RPC_STUB IMSAdminBaseW_R_EnumData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_GetAllData_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [in] */ DWORD dwMDAttributes,
    /* [in] */ DWORD dwMDUserType,
    /* [in] */ DWORD dwMDDataType,
    /* [out] */ __RPC__out DWORD *pdwMDNumDataEntries,
    /* [out] */ __RPC__out DWORD *pdwMDDataSetNumber,
    /* [in] */ DWORD dwMDBufferSize,
    /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppDataBlob);


void __RPC_STUB IMSAdminBaseW_R_GetAllData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_KeyExchangePhase1_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientKeyExchangeKeyBlob,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientSignatureKeyBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerKeyExchangeKeyBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerSignatureKeyBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerSessionKeyBlob);


void __RPC_STUB IMSAdminBaseW_R_KeyExchangePhase1_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_KeyExchangePhase2_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientSessionKeyBlob,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientHashBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerHashBlob);


void __RPC_STUB IMSAdminBaseW_R_KeyExchangePhase2_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_R_GetServerGuid_Proxy( 
    __RPC__in IMSAdminBaseW * This,
    /* [out] */ __RPC__out GUID *pServerGuid);


void __RPC_STUB IMSAdminBaseW_R_GetServerGuid_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMSAdminBaseW_INTERFACE_DEFINED__ */


#ifndef __IMSAdminBase2W_INTERFACE_DEFINED__
#define __IMSAdminBase2W_INTERFACE_DEFINED__

/* interface IMSAdminBase2W */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMSAdminBase2W;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8298d101-f992-43b7-8eca-5052d885b995")
    IMSAdminBase2W : public IMSAdminBaseW
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BackupWithPasswd( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestoreWithPasswd( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Export( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszSourcePath,
            /* [in] */ DWORD dwMDFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Import( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszSourcePath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszDestPath,
            /* [in] */ DWORD dwMDFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestoreHistory( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDHistoryLocation,
            /* [in] */ DWORD dwMDMajorVersion,
            /* [in] */ DWORD dwMDMinorVersion,
            /* [in] */ DWORD dwMDFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumHistory( 
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDHistoryLocation,
            /* [out] */ __RPC__out DWORD *pdwMDMajorVersion,
            /* [out] */ __RPC__out DWORD *pdwMDMinorVersion,
            /* [out] */ __RPC__out PFILETIME pftMDHistoryTime,
            /* [in] */ DWORD dwMDEnumIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSAdminBase2WVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSAdminBase2W * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSAdminBase2W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, AddKey)
        HRESULT ( STDMETHODCALLTYPE *AddKey )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteKey)
        HRESULT ( STDMETHODCALLTYPE *DeleteKey )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteChildKeys)
        HRESULT ( STDMETHODCALLTYPE *DeleteChildKeys )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumKeys)
        HRESULT ( STDMETHODCALLTYPE *EnumKeys )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [size_is][out] */ __RPC__out_ecount_full(256) LPWSTR pszMDName,
            /* [in] */ DWORD dwMDEnumObjectIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CopyKey)
        HRESULT ( STDMETHODCALLTYPE *CopyKey )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ BOOL bMDOverwriteFlag,
            /* [in] */ BOOL bMDCopyFlag);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, RenameKey)
        HRESULT ( STDMETHODCALLTYPE *RenameKey )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDNewName);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetData )( 
            IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ PMETADATA_RECORD pmdrMDData);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [out] */ DWORD *pdwMDRequiredDataLen);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteData)
        HRESULT ( STDMETHODCALLTYPE *DeleteData )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EnumData )( 
            IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [in] */ DWORD dwMDEnumDataIndex,
            /* [out] */ DWORD *pdwMDRequiredDataLen);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetAllData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetAllData )( 
            IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [out] */ DWORD *pdwMDNumDataEntries,
            /* [out] */ DWORD *pdwMDDataSetNumber,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ unsigned char *pbMDBuffer,
            /* [out] */ DWORD *pdwMDRequiredBufferSize);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteAllData)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllData )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CopyData)
        HRESULT ( STDMETHODCALLTYPE *CopyData )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ BOOL bMDCopyFlag);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetDataPaths)
        HRESULT ( STDMETHODCALLTYPE *GetDataPaths )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwMDBufferSize) WCHAR *pszBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, OpenKey)
        HRESULT ( STDMETHODCALLTYPE *OpenKey )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAccessRequested,
            /* [in] */ DWORD dwMDTimeOut,
            /* [out] */ __RPC__out PMETADATA_HANDLE phMDNewHandle);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CloseKey)
        HRESULT ( STDMETHODCALLTYPE *CloseKey )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, ChangePermissions)
        HRESULT ( STDMETHODCALLTYPE *ChangePermissions )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [in] */ DWORD dwMDTimeOut,
            /* [in] */ DWORD dwMDAccessRequested);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SaveData)
        HRESULT ( STDMETHODCALLTYPE *SaveData )( 
            __RPC__in IMSAdminBase2W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetHandleInfo)
        HRESULT ( STDMETHODCALLTYPE *GetHandleInfo )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [out] */ __RPC__out PMETADATA_HANDLE_INFO pmdhiInfo);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetSystemChangeNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSystemChangeNumber )( 
            __RPC__in IMSAdminBase2W * This,
            /* [out] */ __RPC__out DWORD *pdwSystemChangeNumber);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetDataSetNumber)
        HRESULT ( STDMETHODCALLTYPE *GetDataSetNumber )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out DWORD *pdwMDDataSetNumber);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SetLastChangeTime)
        HRESULT ( STDMETHODCALLTYPE *SetLastChangeTime )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ __RPC__in PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetLastChangeTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastChangeTime )( 
            __RPC__in IMSAdminBase2W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, KeyExchangePhase1)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *KeyExchangePhase1 )( 
            IMSAdminBase2W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, KeyExchangePhase2)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *KeyExchangePhase2 )( 
            IMSAdminBase2W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, Backup)
        HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, Restore)
        HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumBackups)
        HRESULT ( STDMETHODCALLTYPE *EnumBackups )( 
            __RPC__in IMSAdminBase2W * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDBackupLocation,
            /* [out] */ __RPC__out DWORD *pdwMDVersion,
            /* [out] */ __RPC__out PFILETIME pftMDBackupTime,
            /* [in] */ DWORD dwMDEnumIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteBackup)
        HRESULT ( STDMETHODCALLTYPE *DeleteBackup )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, UnmarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalInterface )( 
            __RPC__in IMSAdminBase2W * This,
            /* [out] */ __RPC__deref_out_opt IMSAdminBaseW **piadmbwInterface);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetServerGuid)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *GetServerGuid )( 
            IMSAdminBase2W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, BackupWithPasswd)
        HRESULT ( STDMETHODCALLTYPE *BackupWithPasswd )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, RestoreWithPasswd)
        HRESULT ( STDMETHODCALLTYPE *RestoreWithPasswd )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, Export)
        HRESULT ( STDMETHODCALLTYPE *Export )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszSourcePath,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, Import)
        HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszSourcePath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszDestPath,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, RestoreHistory)
        HRESULT ( STDMETHODCALLTYPE *RestoreHistory )( 
            __RPC__in IMSAdminBase2W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDHistoryLocation,
            /* [in] */ DWORD dwMDMajorVersion,
            /* [in] */ DWORD dwMDMinorVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, EnumHistory)
        HRESULT ( STDMETHODCALLTYPE *EnumHistory )( 
            __RPC__in IMSAdminBase2W * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDHistoryLocation,
            /* [out] */ __RPC__out DWORD *pdwMDMajorVersion,
            /* [out] */ __RPC__out DWORD *pdwMDMinorVersion,
            /* [out] */ __RPC__out PFILETIME pftMDHistoryTime,
            /* [in] */ DWORD dwMDEnumIndex);
        
        END_INTERFACE
    } IMSAdminBase2WVtbl;

    interface IMSAdminBase2W
    {
        CONST_VTBL struct IMSAdminBase2WVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSAdminBase2W_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSAdminBase2W_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSAdminBase2W_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSAdminBase2W_AddKey(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> AddKey(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBase2W_DeleteKey(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> DeleteKey(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBase2W_DeleteChildKeys(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> DeleteChildKeys(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBase2W_EnumKeys(This,hMDHandle,pszMDPath,pszMDName,dwMDEnumObjectIndex)	\
    ( (This)->lpVtbl -> EnumKeys(This,hMDHandle,pszMDPath,pszMDName,dwMDEnumObjectIndex) ) 

#define IMSAdminBase2W_CopyKey(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,bMDOverwriteFlag,bMDCopyFlag)	\
    ( (This)->lpVtbl -> CopyKey(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,bMDOverwriteFlag,bMDCopyFlag) ) 

#define IMSAdminBase2W_RenameKey(This,hMDHandle,pszMDPath,pszMDNewName)	\
    ( (This)->lpVtbl -> RenameKey(This,hMDHandle,pszMDPath,pszMDNewName) ) 

#define IMSAdminBase2W_SetData(This,hMDHandle,pszMDPath,pmdrMDData)	\
    ( (This)->lpVtbl -> SetData(This,hMDHandle,pszMDPath,pmdrMDData) ) 

#define IMSAdminBase2W_GetData(This,hMDHandle,pszMDPath,pmdrMDData,pdwMDRequiredDataLen)	\
    ( (This)->lpVtbl -> GetData(This,hMDHandle,pszMDPath,pmdrMDData,pdwMDRequiredDataLen) ) 

#define IMSAdminBase2W_DeleteData(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType)	\
    ( (This)->lpVtbl -> DeleteData(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType) ) 

#define IMSAdminBase2W_EnumData(This,hMDHandle,pszMDPath,pmdrMDData,dwMDEnumDataIndex,pdwMDRequiredDataLen)	\
    ( (This)->lpVtbl -> EnumData(This,hMDHandle,pszMDPath,pmdrMDData,dwMDEnumDataIndex,pdwMDRequiredDataLen) ) 

#define IMSAdminBase2W_GetAllData(This,hMDHandle,pszMDPath,dwMDAttributes,dwMDUserType,dwMDDataType,pdwMDNumDataEntries,pdwMDDataSetNumber,dwMDBufferSize,pbMDBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetAllData(This,hMDHandle,pszMDPath,dwMDAttributes,dwMDUserType,dwMDDataType,pdwMDNumDataEntries,pdwMDDataSetNumber,dwMDBufferSize,pbMDBuffer,pdwMDRequiredBufferSize) ) 

#define IMSAdminBase2W_DeleteAllData(This,hMDHandle,pszMDPath,dwMDUserType,dwMDDataType)	\
    ( (This)->lpVtbl -> DeleteAllData(This,hMDHandle,pszMDPath,dwMDUserType,dwMDDataType) ) 

#define IMSAdminBase2W_CopyData(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,dwMDAttributes,dwMDUserType,dwMDDataType,bMDCopyFlag)	\
    ( (This)->lpVtbl -> CopyData(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,dwMDAttributes,dwMDUserType,dwMDDataType,bMDCopyFlag) ) 

#define IMSAdminBase2W_GetDataPaths(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetDataPaths(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize) ) 

#define IMSAdminBase2W_OpenKey(This,hMDHandle,pszMDPath,dwMDAccessRequested,dwMDTimeOut,phMDNewHandle)	\
    ( (This)->lpVtbl -> OpenKey(This,hMDHandle,pszMDPath,dwMDAccessRequested,dwMDTimeOut,phMDNewHandle) ) 

#define IMSAdminBase2W_CloseKey(This,hMDHandle)	\
    ( (This)->lpVtbl -> CloseKey(This,hMDHandle) ) 

#define IMSAdminBase2W_ChangePermissions(This,hMDHandle,dwMDTimeOut,dwMDAccessRequested)	\
    ( (This)->lpVtbl -> ChangePermissions(This,hMDHandle,dwMDTimeOut,dwMDAccessRequested) ) 

#define IMSAdminBase2W_SaveData(This)	\
    ( (This)->lpVtbl -> SaveData(This) ) 

#define IMSAdminBase2W_GetHandleInfo(This,hMDHandle,pmdhiInfo)	\
    ( (This)->lpVtbl -> GetHandleInfo(This,hMDHandle,pmdhiInfo) ) 

#define IMSAdminBase2W_GetSystemChangeNumber(This,pdwSystemChangeNumber)	\
    ( (This)->lpVtbl -> GetSystemChangeNumber(This,pdwSystemChangeNumber) ) 

#define IMSAdminBase2W_GetDataSetNumber(This,hMDHandle,pszMDPath,pdwMDDataSetNumber)	\
    ( (This)->lpVtbl -> GetDataSetNumber(This,hMDHandle,pszMDPath,pdwMDDataSetNumber) ) 

#define IMSAdminBase2W_SetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime)	\
    ( (This)->lpVtbl -> SetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime) ) 

#define IMSAdminBase2W_GetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime)	\
    ( (This)->lpVtbl -> GetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime) ) 

#define IMSAdminBase2W_KeyExchangePhase1(This)	\
    ( (This)->lpVtbl -> KeyExchangePhase1(This) ) 

#define IMSAdminBase2W_KeyExchangePhase2(This)	\
    ( (This)->lpVtbl -> KeyExchangePhase2(This) ) 

#define IMSAdminBase2W_Backup(This,pszMDBackupLocation,dwMDVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> Backup(This,pszMDBackupLocation,dwMDVersion,dwMDFlags) ) 

#define IMSAdminBase2W_Restore(This,pszMDBackupLocation,dwMDVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> Restore(This,pszMDBackupLocation,dwMDVersion,dwMDFlags) ) 

#define IMSAdminBase2W_EnumBackups(This,pszMDBackupLocation,pdwMDVersion,pftMDBackupTime,dwMDEnumIndex)	\
    ( (This)->lpVtbl -> EnumBackups(This,pszMDBackupLocation,pdwMDVersion,pftMDBackupTime,dwMDEnumIndex) ) 

#define IMSAdminBase2W_DeleteBackup(This,pszMDBackupLocation,dwMDVersion)	\
    ( (This)->lpVtbl -> DeleteBackup(This,pszMDBackupLocation,dwMDVersion) ) 

#define IMSAdminBase2W_UnmarshalInterface(This,piadmbwInterface)	\
    ( (This)->lpVtbl -> UnmarshalInterface(This,piadmbwInterface) ) 

#define IMSAdminBase2W_GetServerGuid(This)	\
    ( (This)->lpVtbl -> GetServerGuid(This) ) 


#define IMSAdminBase2W_BackupWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd)	\
    ( (This)->lpVtbl -> BackupWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd) ) 

#define IMSAdminBase2W_RestoreWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd)	\
    ( (This)->lpVtbl -> RestoreWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd) ) 

#define IMSAdminBase2W_Export(This,pszPasswd,pszFileName,pszSourcePath,dwMDFlags)	\
    ( (This)->lpVtbl -> Export(This,pszPasswd,pszFileName,pszSourcePath,dwMDFlags) ) 

#define IMSAdminBase2W_Import(This,pszPasswd,pszFileName,pszSourcePath,pszDestPath,dwMDFlags)	\
    ( (This)->lpVtbl -> Import(This,pszPasswd,pszFileName,pszSourcePath,pszDestPath,dwMDFlags) ) 

#define IMSAdminBase2W_RestoreHistory(This,pszMDHistoryLocation,dwMDMajorVersion,dwMDMinorVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> RestoreHistory(This,pszMDHistoryLocation,dwMDMajorVersion,dwMDMinorVersion,dwMDFlags) ) 

#define IMSAdminBase2W_EnumHistory(This,pszMDHistoryLocation,pdwMDMajorVersion,pdwMDMinorVersion,pftMDHistoryTime,dwMDEnumIndex)	\
    ( (This)->lpVtbl -> EnumHistory(This,pszMDHistoryLocation,pdwMDMajorVersion,pdwMDMinorVersion,pftMDHistoryTime,dwMDEnumIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSAdminBase2W_INTERFACE_DEFINED__ */


#ifndef __IMSAdminBase3W_INTERFACE_DEFINED__
#define __IMSAdminBase3W_INTERFACE_DEFINED__

/* interface IMSAdminBase3W */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMSAdminBase3W;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f612954d-3b0b-4c56-9563-227b7be624b4")
    IMSAdminBase3W : public IMSAdminBase2W
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetChildPaths( 
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD cchMDBufferSize,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cchMDBufferSize) WCHAR *pszBuffer,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcchMDRequiredBufferSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSAdminBase3WVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSAdminBase3W * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSAdminBase3W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, AddKey)
        HRESULT ( STDMETHODCALLTYPE *AddKey )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteKey)
        HRESULT ( STDMETHODCALLTYPE *DeleteKey )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteChildKeys)
        HRESULT ( STDMETHODCALLTYPE *DeleteChildKeys )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumKeys)
        HRESULT ( STDMETHODCALLTYPE *EnumKeys )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [size_is][out] */ __RPC__out_ecount_full(256) LPWSTR pszMDName,
            /* [in] */ DWORD dwMDEnumObjectIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CopyKey)
        HRESULT ( STDMETHODCALLTYPE *CopyKey )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ BOOL bMDOverwriteFlag,
            /* [in] */ BOOL bMDCopyFlag);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, RenameKey)
        HRESULT ( STDMETHODCALLTYPE *RenameKey )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDNewName);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetData )( 
            IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ PMETADATA_RECORD pmdrMDData);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [out] */ DWORD *pdwMDRequiredDataLen);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteData)
        HRESULT ( STDMETHODCALLTYPE *DeleteData )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EnumData )( 
            IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [out][in] */ PMETADATA_RECORD pmdrMDData,
            /* [in] */ DWORD dwMDEnumDataIndex,
            /* [out] */ DWORD *pdwMDRequiredDataLen);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetAllData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetAllData )( 
            IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [out] */ DWORD *pdwMDNumDataEntries,
            /* [out] */ DWORD *pdwMDDataSetNumber,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ unsigned char *pbMDBuffer,
            /* [out] */ DWORD *pdwMDRequiredBufferSize);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteAllData)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllData )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CopyData)
        HRESULT ( STDMETHODCALLTYPE *CopyData )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDSourceHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDSourcePath,
            /* [in] */ METADATA_HANDLE hMDDestHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDDestPath,
            /* [in] */ DWORD dwMDAttributes,
            /* [in] */ DWORD dwMDUserType,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ BOOL bMDCopyFlag);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetDataPaths)
        HRESULT ( STDMETHODCALLTYPE *GetDataPaths )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDIdentifier,
            /* [in] */ DWORD dwMDDataType,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwMDBufferSize) WCHAR *pszBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, OpenKey)
        HRESULT ( STDMETHODCALLTYPE *OpenKey )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD dwMDAccessRequested,
            /* [in] */ DWORD dwMDTimeOut,
            /* [out] */ __RPC__out PMETADATA_HANDLE phMDNewHandle);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, CloseKey)
        HRESULT ( STDMETHODCALLTYPE *CloseKey )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, ChangePermissions)
        HRESULT ( STDMETHODCALLTYPE *ChangePermissions )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [in] */ DWORD dwMDTimeOut,
            /* [in] */ DWORD dwMDAccessRequested);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SaveData)
        HRESULT ( STDMETHODCALLTYPE *SaveData )( 
            __RPC__in IMSAdminBase3W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetHandleInfo)
        HRESULT ( STDMETHODCALLTYPE *GetHandleInfo )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [out] */ __RPC__out PMETADATA_HANDLE_INFO pmdhiInfo);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetSystemChangeNumber)
        HRESULT ( STDMETHODCALLTYPE *GetSystemChangeNumber )( 
            __RPC__in IMSAdminBase3W * This,
            /* [out] */ __RPC__out DWORD *pdwSystemChangeNumber);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetDataSetNumber)
        HRESULT ( STDMETHODCALLTYPE *GetDataSetNumber )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out DWORD *pdwMDDataSetNumber);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, SetLastChangeTime)
        HRESULT ( STDMETHODCALLTYPE *SetLastChangeTime )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ __RPC__in PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetLastChangeTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastChangeTime )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [out] */ __RPC__out PFILETIME pftMDLastChangeTime,
            /* [in] */ BOOL bLocalTime);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, KeyExchangePhase1)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *KeyExchangePhase1 )( 
            IMSAdminBase3W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, KeyExchangePhase2)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *KeyExchangePhase2 )( 
            IMSAdminBase3W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, Backup)
        HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, Restore)
        HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, EnumBackups)
        HRESULT ( STDMETHODCALLTYPE *EnumBackups )( 
            __RPC__in IMSAdminBase3W * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDBackupLocation,
            /* [out] */ __RPC__out DWORD *pdwMDVersion,
            /* [out] */ __RPC__out PFILETIME pftMDBackupTime,
            /* [in] */ DWORD dwMDEnumIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, DeleteBackup)
        HRESULT ( STDMETHODCALLTYPE *DeleteBackup )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, UnmarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalInterface )( 
            __RPC__in IMSAdminBase3W * This,
            /* [out] */ __RPC__deref_out_opt IMSAdminBaseW **piadmbwInterface);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseW, GetServerGuid)
        /* [restricted][local] */ HRESULT ( STDMETHODCALLTYPE *GetServerGuid )( 
            IMSAdminBase3W * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, BackupWithPasswd)
        HRESULT ( STDMETHODCALLTYPE *BackupWithPasswd )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, RestoreWithPasswd)
        HRESULT ( STDMETHODCALLTYPE *RestoreWithPasswd )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDBackupLocation,
            /* [in] */ DWORD dwMDVersion,
            /* [in] */ DWORD dwMDFlags,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, Export)
        HRESULT ( STDMETHODCALLTYPE *Export )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszSourcePath,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, Import)
        HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszPasswd,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszSourcePath,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszDestPath,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, RestoreHistory)
        HRESULT ( STDMETHODCALLTYPE *RestoreHistory )( 
            __RPC__in IMSAdminBase3W * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDHistoryLocation,
            /* [in] */ DWORD dwMDMajorVersion,
            /* [in] */ DWORD dwMDMinorVersion,
            /* [in] */ DWORD dwMDFlags);
        
        DECLSPEC_XFGVIRT(IMSAdminBase2W, EnumHistory)
        HRESULT ( STDMETHODCALLTYPE *EnumHistory )( 
            __RPC__in IMSAdminBase3W * This,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(256) LPWSTR pszMDHistoryLocation,
            /* [out] */ __RPC__out DWORD *pdwMDMajorVersion,
            /* [out] */ __RPC__out DWORD *pdwMDMinorVersion,
            /* [out] */ __RPC__out PFILETIME pftMDHistoryTime,
            /* [in] */ DWORD dwMDEnumIndex);
        
        DECLSPEC_XFGVIRT(IMSAdminBase3W, GetChildPaths)
        HRESULT ( STDMETHODCALLTYPE *GetChildPaths )( 
            __RPC__in IMSAdminBase3W * This,
            /* [in] */ METADATA_HANDLE hMDHandle,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
            /* [in] */ DWORD cchMDBufferSize,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cchMDBufferSize) WCHAR *pszBuffer,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcchMDRequiredBufferSize);
        
        END_INTERFACE
    } IMSAdminBase3WVtbl;

    interface IMSAdminBase3W
    {
        CONST_VTBL struct IMSAdminBase3WVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSAdminBase3W_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSAdminBase3W_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSAdminBase3W_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSAdminBase3W_AddKey(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> AddKey(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBase3W_DeleteKey(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> DeleteKey(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBase3W_DeleteChildKeys(This,hMDHandle,pszMDPath)	\
    ( (This)->lpVtbl -> DeleteChildKeys(This,hMDHandle,pszMDPath) ) 

#define IMSAdminBase3W_EnumKeys(This,hMDHandle,pszMDPath,pszMDName,dwMDEnumObjectIndex)	\
    ( (This)->lpVtbl -> EnumKeys(This,hMDHandle,pszMDPath,pszMDName,dwMDEnumObjectIndex) ) 

#define IMSAdminBase3W_CopyKey(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,bMDOverwriteFlag,bMDCopyFlag)	\
    ( (This)->lpVtbl -> CopyKey(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,bMDOverwriteFlag,bMDCopyFlag) ) 

#define IMSAdminBase3W_RenameKey(This,hMDHandle,pszMDPath,pszMDNewName)	\
    ( (This)->lpVtbl -> RenameKey(This,hMDHandle,pszMDPath,pszMDNewName) ) 

#define IMSAdminBase3W_SetData(This,hMDHandle,pszMDPath,pmdrMDData)	\
    ( (This)->lpVtbl -> SetData(This,hMDHandle,pszMDPath,pmdrMDData) ) 

#define IMSAdminBase3W_GetData(This,hMDHandle,pszMDPath,pmdrMDData,pdwMDRequiredDataLen)	\
    ( (This)->lpVtbl -> GetData(This,hMDHandle,pszMDPath,pmdrMDData,pdwMDRequiredDataLen) ) 

#define IMSAdminBase3W_DeleteData(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType)	\
    ( (This)->lpVtbl -> DeleteData(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType) ) 

#define IMSAdminBase3W_EnumData(This,hMDHandle,pszMDPath,pmdrMDData,dwMDEnumDataIndex,pdwMDRequiredDataLen)	\
    ( (This)->lpVtbl -> EnumData(This,hMDHandle,pszMDPath,pmdrMDData,dwMDEnumDataIndex,pdwMDRequiredDataLen) ) 

#define IMSAdminBase3W_GetAllData(This,hMDHandle,pszMDPath,dwMDAttributes,dwMDUserType,dwMDDataType,pdwMDNumDataEntries,pdwMDDataSetNumber,dwMDBufferSize,pbMDBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetAllData(This,hMDHandle,pszMDPath,dwMDAttributes,dwMDUserType,dwMDDataType,pdwMDNumDataEntries,pdwMDDataSetNumber,dwMDBufferSize,pbMDBuffer,pdwMDRequiredBufferSize) ) 

#define IMSAdminBase3W_DeleteAllData(This,hMDHandle,pszMDPath,dwMDUserType,dwMDDataType)	\
    ( (This)->lpVtbl -> DeleteAllData(This,hMDHandle,pszMDPath,dwMDUserType,dwMDDataType) ) 

#define IMSAdminBase3W_CopyData(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,dwMDAttributes,dwMDUserType,dwMDDataType,bMDCopyFlag)	\
    ( (This)->lpVtbl -> CopyData(This,hMDSourceHandle,pszMDSourcePath,hMDDestHandle,pszMDDestPath,dwMDAttributes,dwMDUserType,dwMDDataType,bMDCopyFlag) ) 

#define IMSAdminBase3W_GetDataPaths(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetDataPaths(This,hMDHandle,pszMDPath,dwMDIdentifier,dwMDDataType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize) ) 

#define IMSAdminBase3W_OpenKey(This,hMDHandle,pszMDPath,dwMDAccessRequested,dwMDTimeOut,phMDNewHandle)	\
    ( (This)->lpVtbl -> OpenKey(This,hMDHandle,pszMDPath,dwMDAccessRequested,dwMDTimeOut,phMDNewHandle) ) 

#define IMSAdminBase3W_CloseKey(This,hMDHandle)	\
    ( (This)->lpVtbl -> CloseKey(This,hMDHandle) ) 

#define IMSAdminBase3W_ChangePermissions(This,hMDHandle,dwMDTimeOut,dwMDAccessRequested)	\
    ( (This)->lpVtbl -> ChangePermissions(This,hMDHandle,dwMDTimeOut,dwMDAccessRequested) ) 

#define IMSAdminBase3W_SaveData(This)	\
    ( (This)->lpVtbl -> SaveData(This) ) 

#define IMSAdminBase3W_GetHandleInfo(This,hMDHandle,pmdhiInfo)	\
    ( (This)->lpVtbl -> GetHandleInfo(This,hMDHandle,pmdhiInfo) ) 

#define IMSAdminBase3W_GetSystemChangeNumber(This,pdwSystemChangeNumber)	\
    ( (This)->lpVtbl -> GetSystemChangeNumber(This,pdwSystemChangeNumber) ) 

#define IMSAdminBase3W_GetDataSetNumber(This,hMDHandle,pszMDPath,pdwMDDataSetNumber)	\
    ( (This)->lpVtbl -> GetDataSetNumber(This,hMDHandle,pszMDPath,pdwMDDataSetNumber) ) 

#define IMSAdminBase3W_SetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime)	\
    ( (This)->lpVtbl -> SetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime) ) 

#define IMSAdminBase3W_GetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime)	\
    ( (This)->lpVtbl -> GetLastChangeTime(This,hMDHandle,pszMDPath,pftMDLastChangeTime,bLocalTime) ) 

#define IMSAdminBase3W_KeyExchangePhase1(This)	\
    ( (This)->lpVtbl -> KeyExchangePhase1(This) ) 

#define IMSAdminBase3W_KeyExchangePhase2(This)	\
    ( (This)->lpVtbl -> KeyExchangePhase2(This) ) 

#define IMSAdminBase3W_Backup(This,pszMDBackupLocation,dwMDVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> Backup(This,pszMDBackupLocation,dwMDVersion,dwMDFlags) ) 

#define IMSAdminBase3W_Restore(This,pszMDBackupLocation,dwMDVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> Restore(This,pszMDBackupLocation,dwMDVersion,dwMDFlags) ) 

#define IMSAdminBase3W_EnumBackups(This,pszMDBackupLocation,pdwMDVersion,pftMDBackupTime,dwMDEnumIndex)	\
    ( (This)->lpVtbl -> EnumBackups(This,pszMDBackupLocation,pdwMDVersion,pftMDBackupTime,dwMDEnumIndex) ) 

#define IMSAdminBase3W_DeleteBackup(This,pszMDBackupLocation,dwMDVersion)	\
    ( (This)->lpVtbl -> DeleteBackup(This,pszMDBackupLocation,dwMDVersion) ) 

#define IMSAdminBase3W_UnmarshalInterface(This,piadmbwInterface)	\
    ( (This)->lpVtbl -> UnmarshalInterface(This,piadmbwInterface) ) 

#define IMSAdminBase3W_GetServerGuid(This)	\
    ( (This)->lpVtbl -> GetServerGuid(This) ) 


#define IMSAdminBase3W_BackupWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd)	\
    ( (This)->lpVtbl -> BackupWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd) ) 

#define IMSAdminBase3W_RestoreWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd)	\
    ( (This)->lpVtbl -> RestoreWithPasswd(This,pszMDBackupLocation,dwMDVersion,dwMDFlags,pszPasswd) ) 

#define IMSAdminBase3W_Export(This,pszPasswd,pszFileName,pszSourcePath,dwMDFlags)	\
    ( (This)->lpVtbl -> Export(This,pszPasswd,pszFileName,pszSourcePath,dwMDFlags) ) 

#define IMSAdminBase3W_Import(This,pszPasswd,pszFileName,pszSourcePath,pszDestPath,dwMDFlags)	\
    ( (This)->lpVtbl -> Import(This,pszPasswd,pszFileName,pszSourcePath,pszDestPath,dwMDFlags) ) 

#define IMSAdminBase3W_RestoreHistory(This,pszMDHistoryLocation,dwMDMajorVersion,dwMDMinorVersion,dwMDFlags)	\
    ( (This)->lpVtbl -> RestoreHistory(This,pszMDHistoryLocation,dwMDMajorVersion,dwMDMinorVersion,dwMDFlags) ) 

#define IMSAdminBase3W_EnumHistory(This,pszMDHistoryLocation,pdwMDMajorVersion,pdwMDMinorVersion,pftMDHistoryTime,dwMDEnumIndex)	\
    ( (This)->lpVtbl -> EnumHistory(This,pszMDHistoryLocation,pdwMDMajorVersion,pdwMDMinorVersion,pftMDHistoryTime,dwMDEnumIndex) ) 


#define IMSAdminBase3W_GetChildPaths(This,hMDHandle,pszMDPath,cchMDBufferSize,pszBuffer,pcchMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> GetChildPaths(This,hMDHandle,pszMDPath,cchMDBufferSize,pszBuffer,pcchMDRequiredBufferSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSAdminBase3W_INTERFACE_DEFINED__ */


#ifndef __IMSImpExpHelpW_INTERFACE_DEFINED__
#define __IMSImpExpHelpW_INTERFACE_DEFINED__

/* interface IMSImpExpHelpW */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMSImpExpHelpW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29FF67FF-8050-480f-9F30-CC41635F2F9D")
    IMSImpExpHelpW : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumeratePathsInFile( 
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszKeyType,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(dwMDBufferSize) WCHAR *pszBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSImpExpHelpWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSImpExpHelpW * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSImpExpHelpW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSImpExpHelpW * This);
        
        DECLSPEC_XFGVIRT(IMSImpExpHelpW, EnumeratePathsInFile)
        HRESULT ( STDMETHODCALLTYPE *EnumeratePathsInFile )( 
            __RPC__in IMSImpExpHelpW * This,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszFileName,
            /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszKeyType,
            /* [in] */ DWORD dwMDBufferSize,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(dwMDBufferSize) WCHAR *pszBuffer,
            /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize);
        
        END_INTERFACE
    } IMSImpExpHelpWVtbl;

    interface IMSImpExpHelpW
    {
        CONST_VTBL struct IMSImpExpHelpWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSImpExpHelpW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSImpExpHelpW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSImpExpHelpW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSImpExpHelpW_EnumeratePathsInFile(This,pszFileName,pszKeyType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize)	\
    ( (This)->lpVtbl -> EnumeratePathsInFile(This,pszFileName,pszKeyType,dwMDBufferSize,pszBuffer,pdwMDRequiredBufferSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSImpExpHelpW_INTERFACE_DEFINED__ */


#ifndef __IMSAdminBaseSinkW_INTERFACE_DEFINED__
#define __IMSAdminBaseSinkW_INTERFACE_DEFINED__

/* interface IMSAdminBaseSinkW */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IMSAdminBaseSinkW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9E69612-B80D-11d0-B9B9-00A0C922E750")
    IMSAdminBaseSinkW : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SinkNotify( 
            /* [in] */ DWORD dwMDNumElements,
            /* [size_is][in] */ __RPC__in_ecount_full(dwMDNumElements) MD_CHANGE_OBJECT_W pcoChangeList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShutdownNotify( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMSAdminBaseSinkWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMSAdminBaseSinkW * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMSAdminBaseSinkW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMSAdminBaseSinkW * This);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseSinkW, SinkNotify)
        HRESULT ( STDMETHODCALLTYPE *SinkNotify )( 
            __RPC__in IMSAdminBaseSinkW * This,
            /* [in] */ DWORD dwMDNumElements,
            /* [size_is][in] */ __RPC__in_ecount_full(dwMDNumElements) MD_CHANGE_OBJECT_W pcoChangeList[  ]);
        
        DECLSPEC_XFGVIRT(IMSAdminBaseSinkW, ShutdownNotify)
        HRESULT ( STDMETHODCALLTYPE *ShutdownNotify )( 
            __RPC__in IMSAdminBaseSinkW * This);
        
        END_INTERFACE
    } IMSAdminBaseSinkWVtbl;

    interface IMSAdminBaseSinkW
    {
        CONST_VTBL struct IMSAdminBaseSinkWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMSAdminBaseSinkW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMSAdminBaseSinkW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMSAdminBaseSinkW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMSAdminBaseSinkW_SinkNotify(This,dwMDNumElements,pcoChangeList)	\
    ( (This)->lpVtbl -> SinkNotify(This,dwMDNumElements,pcoChangeList) ) 

#define IMSAdminBaseSinkW_ShutdownNotify(This)	\
    ( (This)->lpVtbl -> ShutdownNotify(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMSAdminBaseSinkW_INTERFACE_DEFINED__ */


#ifndef __AsyncIMSAdminBaseSinkW_INTERFACE_DEFINED__
#define __AsyncIMSAdminBaseSinkW_INTERFACE_DEFINED__

/* interface AsyncIMSAdminBaseSinkW */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIMSAdminBaseSinkW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9E69613-B80D-11d0-B9B9-00A0C922E750")
    AsyncIMSAdminBaseSinkW : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_SinkNotify( 
            /* [in] */ DWORD dwMDNumElements,
            /* [size_is][in] */ __RPC__in_xcount_full(dwMDNumElements) MD_CHANGE_OBJECT_W pcoChangeList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_SinkNotify( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_ShutdownNotify( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_ShutdownNotify( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIMSAdminBaseSinkWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This);
        
        DECLSPEC_XFGVIRT(AsyncIMSAdminBaseSinkW, Begin_SinkNotify)
        HRESULT ( STDMETHODCALLTYPE *Begin_SinkNotify )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This,
            /* [in] */ DWORD dwMDNumElements,
            /* [size_is][in] */ __RPC__in_xcount_full(dwMDNumElements) MD_CHANGE_OBJECT_W pcoChangeList[  ]);
        
        DECLSPEC_XFGVIRT(AsyncIMSAdminBaseSinkW, Finish_SinkNotify)
        HRESULT ( STDMETHODCALLTYPE *Finish_SinkNotify )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This);
        
        DECLSPEC_XFGVIRT(AsyncIMSAdminBaseSinkW, Begin_ShutdownNotify)
        HRESULT ( STDMETHODCALLTYPE *Begin_ShutdownNotify )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This);
        
        DECLSPEC_XFGVIRT(AsyncIMSAdminBaseSinkW, Finish_ShutdownNotify)
        HRESULT ( STDMETHODCALLTYPE *Finish_ShutdownNotify )( 
            __RPC__in AsyncIMSAdminBaseSinkW * This);
        
        END_INTERFACE
    } AsyncIMSAdminBaseSinkWVtbl;

    interface AsyncIMSAdminBaseSinkW
    {
        CONST_VTBL struct AsyncIMSAdminBaseSinkWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIMSAdminBaseSinkW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIMSAdminBaseSinkW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIMSAdminBaseSinkW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIMSAdminBaseSinkW_Begin_SinkNotify(This,dwMDNumElements,pcoChangeList)	\
    ( (This)->lpVtbl -> Begin_SinkNotify(This,dwMDNumElements,pcoChangeList) ) 

#define AsyncIMSAdminBaseSinkW_Finish_SinkNotify(This)	\
    ( (This)->lpVtbl -> Finish_SinkNotify(This) ) 

#define AsyncIMSAdminBaseSinkW_Begin_ShutdownNotify(This)	\
    ( (This)->lpVtbl -> Begin_ShutdownNotify(This) ) 

#define AsyncIMSAdminBaseSinkW_Finish_ShutdownNotify(This)	\
    ( (This)->lpVtbl -> Finish_ShutdownNotify(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIMSAdminBaseSinkW_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_iadmw_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif


extern RPC_IF_HANDLE __MIDL_itf_iadmw_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iadmw_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* [local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_SetData_Proxy( 
    IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ LPCWSTR pszMDPath,
    /* [in] */ PMETADATA_RECORD pmdrMDData);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_SetData_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [in] */ __RPC__in PMETADATA_RECORD pmdrMDData);

/* [local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_GetData_Proxy( 
    IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ LPCWSTR pszMDPath,
    /* [out][in] */ PMETADATA_RECORD pmdrMDData,
    /* [out] */ DWORD *pdwMDRequiredDataLen);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_GetData_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [out][in] */ __RPC__inout PMETADATA_RECORD pmdrMDData,
    /* [out] */ __RPC__out DWORD *pdwMDRequiredDataLen,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppDataBlob);

/* [local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_EnumData_Proxy( 
    IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ LPCWSTR pszMDPath,
    /* [out][in] */ PMETADATA_RECORD pmdrMDData,
    /* [in] */ DWORD dwMDEnumDataIndex,
    /* [out] */ DWORD *pdwMDRequiredDataLen);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_EnumData_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [out][in] */ __RPC__inout PMETADATA_RECORD pmdrMDData,
    /* [in] */ DWORD dwMDEnumDataIndex,
    /* [out] */ __RPC__out DWORD *pdwMDRequiredDataLen,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppDataBlob);

/* [local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_GetAllData_Proxy( 
    IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ LPCWSTR pszMDPath,
    /* [in] */ DWORD dwMDAttributes,
    /* [in] */ DWORD dwMDUserType,
    /* [in] */ DWORD dwMDDataType,
    /* [out] */ DWORD *pdwMDNumDataEntries,
    /* [out] */ DWORD *pdwMDDataSetNumber,
    /* [in] */ DWORD dwMDBufferSize,
    /* [size_is][out] */ unsigned char *pbMDBuffer,
    /* [out] */ DWORD *pdwMDRequiredBufferSize);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_GetAllData_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [in] */ METADATA_HANDLE hMDHandle,
    /* [string][in][unique] */ __RPC__in_opt_string LPCWSTR pszMDPath,
    /* [in] */ DWORD dwMDAttributes,
    /* [in] */ DWORD dwMDUserType,
    /* [in] */ DWORD dwMDDataType,
    /* [out] */ __RPC__out DWORD *pdwMDNumDataEntries,
    /* [out] */ __RPC__out DWORD *pdwMDDataSetNumber,
    /* [in] */ DWORD dwMDBufferSize,
    /* [out] */ __RPC__out DWORD *pdwMDRequiredBufferSize,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppDataBlob);

/* [restricted][local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_KeyExchangePhase1_Proxy( 
    IMSAdminBaseW * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_KeyExchangePhase1_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientKeyExchangeKeyBlob,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientSignatureKeyBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerKeyExchangeKeyBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerSignatureKeyBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerSessionKeyBlob);

/* [restricted][local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_KeyExchangePhase2_Proxy( 
    IMSAdminBaseW * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_KeyExchangePhase2_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientSessionKeyBlob,
    /* [in][unique] */ __RPC__in_opt struct _IIS_CRYPTO_BLOB *pClientHashBlob,
    /* [out] */ __RPC__deref_out_opt struct _IIS_CRYPTO_BLOB **ppServerHashBlob);

/* [restricted][local] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_GetServerGuid_Proxy( 
    IMSAdminBaseW * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMSAdminBaseW_GetServerGuid_Stub( 
    __RPC__in IMSAdminBaseW * This,
    /* [out] */ __RPC__out GUID *pServerGuid);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


