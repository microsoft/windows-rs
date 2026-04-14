

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __ddpbackup_h__
#define __ddpbackup_h__

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

#ifndef __IDedupReadFileCallback_FWD_DEFINED__
#define __IDedupReadFileCallback_FWD_DEFINED__
typedef interface IDedupReadFileCallback IDedupReadFileCallback;

#endif 	/* __IDedupReadFileCallback_FWD_DEFINED__ */


#ifndef __IDedupBackupSupport_FWD_DEFINED__
#define __IDedupBackupSupport_FWD_DEFINED__
typedef interface IDedupBackupSupport IDedupBackupSupport;

#endif 	/* __IDedupBackupSupport_FWD_DEFINED__ */


#ifndef __DedupBackupSupport_FWD_DEFINED__
#define __DedupBackupSupport_FWD_DEFINED__

#ifdef __cplusplus
typedef class DedupBackupSupport DedupBackupSupport;
#else
typedef struct DedupBackupSupport DedupBackupSupport;
#endif /* __cplusplus */

#endif 	/* __DedupBackupSupport_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ddpbackup_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct _DEDUP_CONTAINER_EXTENT
    {
    ULONG ContainerIndex;
    hyper StartOffset;
    hyper Length;
    } 	DEDUP_CONTAINER_EXTENT;

typedef struct _DDP_FILE_EXTENT
    {
    hyper Length;
    hyper Offset;
    } 	DDP_FILE_EXTENT;

typedef 
enum _DEDUP_BACKUP_SUPPORT_PARAM_TYPE
    {
        DEDUP_RECONSTRUCT_UNOPTIMIZED	= 1,
        DEDUP_RECONSTRUCT_OPTIMIZED	= 2
    } 	DEDUP_BACKUP_SUPPORT_PARAM_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_ddpbackup_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpbackup_0000_0000_v0_0_s_ifspec;

#ifndef __IDedupReadFileCallback_INTERFACE_DEFINED__
#define __IDedupReadFileCallback_INTERFACE_DEFINED__

/* interface IDedupReadFileCallback */
/* [version][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IDedupReadFileCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7BACC67A-2F1D-42D0-897E-6FF62DD533BB")
    IDedupReadFileCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReadBackupFile( 
            /* [in] */ __RPC__in BSTR FileFullPath,
            /* [in] */ hyper FileOffset,
            /* [in] */ ULONG SizeToRead,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(SizeToRead, *ReturnedSize) BYTE *FileBuffer,
            /* [out] */ __RPC__out ULONG *ReturnedSize,
            /* [in] */ DWORD Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OrderContainersRestore( 
            /* [in] */ ULONG NumberOfContainers,
            /* [size_is][in] */ __RPC__in_ecount_full(NumberOfContainers) BSTR *ContainerPaths,
            /* [out] */ __RPC__out ULONG *ReadPlanEntries,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*ReadPlanEntries) DEDUP_CONTAINER_EXTENT **ReadPlan) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PreviewContainerRead( 
            /* [in] */ __RPC__in BSTR FileFullPath,
            /* [in] */ ULONG NumberOfReads,
            /* [size_is][in] */ __RPC__in_ecount_full(NumberOfReads) DDP_FILE_EXTENT *ReadOffsets) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDedupReadFileCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDedupReadFileCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDedupReadFileCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDedupReadFileCallback * This);
        
        DECLSPEC_XFGVIRT(IDedupReadFileCallback, ReadBackupFile)
        HRESULT ( STDMETHODCALLTYPE *ReadBackupFile )( 
            __RPC__in IDedupReadFileCallback * This,
            /* [in] */ __RPC__in BSTR FileFullPath,
            /* [in] */ hyper FileOffset,
            /* [in] */ ULONG SizeToRead,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(SizeToRead, *ReturnedSize) BYTE *FileBuffer,
            /* [out] */ __RPC__out ULONG *ReturnedSize,
            /* [in] */ DWORD Flags);
        
        DECLSPEC_XFGVIRT(IDedupReadFileCallback, OrderContainersRestore)
        HRESULT ( STDMETHODCALLTYPE *OrderContainersRestore )( 
            __RPC__in IDedupReadFileCallback * This,
            /* [in] */ ULONG NumberOfContainers,
            /* [size_is][in] */ __RPC__in_ecount_full(NumberOfContainers) BSTR *ContainerPaths,
            /* [out] */ __RPC__out ULONG *ReadPlanEntries,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*ReadPlanEntries) DEDUP_CONTAINER_EXTENT **ReadPlan);
        
        DECLSPEC_XFGVIRT(IDedupReadFileCallback, PreviewContainerRead)
        HRESULT ( STDMETHODCALLTYPE *PreviewContainerRead )( 
            __RPC__in IDedupReadFileCallback * This,
            /* [in] */ __RPC__in BSTR FileFullPath,
            /* [in] */ ULONG NumberOfReads,
            /* [size_is][in] */ __RPC__in_ecount_full(NumberOfReads) DDP_FILE_EXTENT *ReadOffsets);
        
        END_INTERFACE
    } IDedupReadFileCallbackVtbl;

    interface IDedupReadFileCallback
    {
        CONST_VTBL struct IDedupReadFileCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDedupReadFileCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDedupReadFileCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDedupReadFileCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDedupReadFileCallback_ReadBackupFile(This,FileFullPath,FileOffset,SizeToRead,FileBuffer,ReturnedSize,Flags)	\
    ( (This)->lpVtbl -> ReadBackupFile(This,FileFullPath,FileOffset,SizeToRead,FileBuffer,ReturnedSize,Flags) ) 

#define IDedupReadFileCallback_OrderContainersRestore(This,NumberOfContainers,ContainerPaths,ReadPlanEntries,ReadPlan)	\
    ( (This)->lpVtbl -> OrderContainersRestore(This,NumberOfContainers,ContainerPaths,ReadPlanEntries,ReadPlan) ) 

#define IDedupReadFileCallback_PreviewContainerRead(This,FileFullPath,NumberOfReads,ReadOffsets)	\
    ( (This)->lpVtbl -> PreviewContainerRead(This,FileFullPath,NumberOfReads,ReadOffsets) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDedupReadFileCallback_INTERFACE_DEFINED__ */


#ifndef __IDedupBackupSupport_INTERFACE_DEFINED__
#define __IDedupBackupSupport_INTERFACE_DEFINED__

/* interface IDedupBackupSupport */
/* [version][uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IDedupBackupSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C719D963-2B2D-415E-ACF7-7EB7CA596FF4")
    IDedupBackupSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RestoreFiles( 
            /* [in] */ ULONG NumberOfFiles,
            /* [size_is][in] */ __RPC__in_ecount_full(NumberOfFiles) BSTR *FileFullPaths,
            /* [in] */ __RPC__in_opt IDedupReadFileCallback *Store,
            /* [in] */ DWORD Flags,
            /* [size_is][out] */ __RPC__out_ecount_full(NumberOfFiles) HRESULT *FileResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDedupBackupSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDedupBackupSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDedupBackupSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDedupBackupSupport * This);
        
        DECLSPEC_XFGVIRT(IDedupBackupSupport, RestoreFiles)
        HRESULT ( STDMETHODCALLTYPE *RestoreFiles )( 
            __RPC__in IDedupBackupSupport * This,
            /* [in] */ ULONG NumberOfFiles,
            /* [size_is][in] */ __RPC__in_ecount_full(NumberOfFiles) BSTR *FileFullPaths,
            /* [in] */ __RPC__in_opt IDedupReadFileCallback *Store,
            /* [in] */ DWORD Flags,
            /* [size_is][out] */ __RPC__out_ecount_full(NumberOfFiles) HRESULT *FileResults);
        
        END_INTERFACE
    } IDedupBackupSupportVtbl;

    interface IDedupBackupSupport
    {
        CONST_VTBL struct IDedupBackupSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDedupBackupSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDedupBackupSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDedupBackupSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDedupBackupSupport_RestoreFiles(This,NumberOfFiles,FileFullPaths,Store,Flags,FileResults)	\
    ( (This)->lpVtbl -> RestoreFiles(This,NumberOfFiles,FileFullPaths,Store,Flags,FileResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDedupBackupSupport_INTERFACE_DEFINED__ */



#ifndef __DedupBackup_LIBRARY_DEFINED__
#define __DedupBackup_LIBRARY_DEFINED__

/* library DedupBackup */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_DedupBackup;

EXTERN_C const CLSID CLSID_DedupBackupSupport;

#ifdef __cplusplus

class DECLSPEC_UUID("73D6B2AD-2984-4715-B2E3-924C149744DD")
DedupBackupSupport;
#endif
#endif /* __DedupBackup_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_ddpbackup_0000_0003 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WIN8)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ddpbackup_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddpbackup_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


