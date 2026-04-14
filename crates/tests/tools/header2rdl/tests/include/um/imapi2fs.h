

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

#ifndef __imapi2fs_h__
#define __imapi2fs_h__

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

#ifndef __IBootOptions_FWD_DEFINED__
#define __IBootOptions_FWD_DEFINED__
typedef interface IBootOptions IBootOptions;

#endif 	/* __IBootOptions_FWD_DEFINED__ */


#ifndef __IProgressItem_FWD_DEFINED__
#define __IProgressItem_FWD_DEFINED__
typedef interface IProgressItem IProgressItem;

#endif 	/* __IProgressItem_FWD_DEFINED__ */


#ifndef __IEnumProgressItems_FWD_DEFINED__
#define __IEnumProgressItems_FWD_DEFINED__
typedef interface IEnumProgressItems IEnumProgressItems;

#endif 	/* __IEnumProgressItems_FWD_DEFINED__ */


#ifndef __IProgressItems_FWD_DEFINED__
#define __IProgressItems_FWD_DEFINED__
typedef interface IProgressItems IProgressItems;

#endif 	/* __IProgressItems_FWD_DEFINED__ */


#ifndef __IFileSystemImageResult_FWD_DEFINED__
#define __IFileSystemImageResult_FWD_DEFINED__
typedef interface IFileSystemImageResult IFileSystemImageResult;

#endif 	/* __IFileSystemImageResult_FWD_DEFINED__ */


#ifndef __IFileSystemImageResult2_FWD_DEFINED__
#define __IFileSystemImageResult2_FWD_DEFINED__
typedef interface IFileSystemImageResult2 IFileSystemImageResult2;

#endif 	/* __IFileSystemImageResult2_FWD_DEFINED__ */


#ifndef __IFsiItem_FWD_DEFINED__
#define __IFsiItem_FWD_DEFINED__
typedef interface IFsiItem IFsiItem;

#endif 	/* __IFsiItem_FWD_DEFINED__ */


#ifndef __IEnumFsiItems_FWD_DEFINED__
#define __IEnumFsiItems_FWD_DEFINED__
typedef interface IEnumFsiItems IEnumFsiItems;

#endif 	/* __IEnumFsiItems_FWD_DEFINED__ */


#ifndef __IFsiFileItem_FWD_DEFINED__
#define __IFsiFileItem_FWD_DEFINED__
typedef interface IFsiFileItem IFsiFileItem;

#endif 	/* __IFsiFileItem_FWD_DEFINED__ */


#ifndef __IFsiFileItem2_FWD_DEFINED__
#define __IFsiFileItem2_FWD_DEFINED__
typedef interface IFsiFileItem2 IFsiFileItem2;

#endif 	/* __IFsiFileItem2_FWD_DEFINED__ */


#ifndef __IFsiNamedStreams_FWD_DEFINED__
#define __IFsiNamedStreams_FWD_DEFINED__
typedef interface IFsiNamedStreams IFsiNamedStreams;

#endif 	/* __IFsiNamedStreams_FWD_DEFINED__ */


#ifndef __IFsiDirectoryItem_FWD_DEFINED__
#define __IFsiDirectoryItem_FWD_DEFINED__
typedef interface IFsiDirectoryItem IFsiDirectoryItem;

#endif 	/* __IFsiDirectoryItem_FWD_DEFINED__ */


#ifndef __IFsiDirectoryItem2_FWD_DEFINED__
#define __IFsiDirectoryItem2_FWD_DEFINED__
typedef interface IFsiDirectoryItem2 IFsiDirectoryItem2;

#endif 	/* __IFsiDirectoryItem2_FWD_DEFINED__ */


#ifndef __IFileSystemImage_FWD_DEFINED__
#define __IFileSystemImage_FWD_DEFINED__
typedef interface IFileSystemImage IFileSystemImage;

#endif 	/* __IFileSystemImage_FWD_DEFINED__ */


#ifndef __IFileSystemImage2_FWD_DEFINED__
#define __IFileSystemImage2_FWD_DEFINED__
typedef interface IFileSystemImage2 IFileSystemImage2;

#endif 	/* __IFileSystemImage2_FWD_DEFINED__ */


#ifndef __IFileSystemImage3_FWD_DEFINED__
#define __IFileSystemImage3_FWD_DEFINED__
typedef interface IFileSystemImage3 IFileSystemImage3;

#endif 	/* __IFileSystemImage3_FWD_DEFINED__ */


#ifndef __DFileSystemImageEvents_FWD_DEFINED__
#define __DFileSystemImageEvents_FWD_DEFINED__
typedef interface DFileSystemImageEvents DFileSystemImageEvents;

#endif 	/* __DFileSystemImageEvents_FWD_DEFINED__ */


#ifndef __DFileSystemImageImportEvents_FWD_DEFINED__
#define __DFileSystemImageImportEvents_FWD_DEFINED__
typedef interface DFileSystemImageImportEvents DFileSystemImageImportEvents;

#endif 	/* __DFileSystemImageImportEvents_FWD_DEFINED__ */


#ifndef __IIsoImageManager_FWD_DEFINED__
#define __IIsoImageManager_FWD_DEFINED__
typedef interface IIsoImageManager IIsoImageManager;

#endif 	/* __IIsoImageManager_FWD_DEFINED__ */


#ifndef __DFileSystemImageEvents_FWD_DEFINED__
#define __DFileSystemImageEvents_FWD_DEFINED__
typedef interface DFileSystemImageEvents DFileSystemImageEvents;

#endif 	/* __DFileSystemImageEvents_FWD_DEFINED__ */


#ifndef __DFileSystemImageImportEvents_FWD_DEFINED__
#define __DFileSystemImageImportEvents_FWD_DEFINED__
typedef interface DFileSystemImageImportEvents DFileSystemImageImportEvents;

#endif 	/* __DFileSystemImageImportEvents_FWD_DEFINED__ */


#ifndef __BootOptions_FWD_DEFINED__
#define __BootOptions_FWD_DEFINED__

#ifdef __cplusplus
typedef class BootOptions BootOptions;
#else
typedef struct BootOptions BootOptions;
#endif /* __cplusplus */

#endif 	/* __BootOptions_FWD_DEFINED__ */


#ifndef __FsiStream_FWD_DEFINED__
#define __FsiStream_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsiStream FsiStream;
#else
typedef struct FsiStream FsiStream;
#endif /* __cplusplus */

#endif 	/* __FsiStream_FWD_DEFINED__ */


#ifndef __FileSystemImageResult_FWD_DEFINED__
#define __FileSystemImageResult_FWD_DEFINED__

#ifdef __cplusplus
typedef class FileSystemImageResult FileSystemImageResult;
#else
typedef struct FileSystemImageResult FileSystemImageResult;
#endif /* __cplusplus */

#endif 	/* __FileSystemImageResult_FWD_DEFINED__ */


#ifndef __ProgressItem_FWD_DEFINED__
#define __ProgressItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class ProgressItem ProgressItem;
#else
typedef struct ProgressItem ProgressItem;
#endif /* __cplusplus */

#endif 	/* __ProgressItem_FWD_DEFINED__ */


#ifndef __EnumProgressItems_FWD_DEFINED__
#define __EnumProgressItems_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnumProgressItems EnumProgressItems;
#else
typedef struct EnumProgressItems EnumProgressItems;
#endif /* __cplusplus */

#endif 	/* __EnumProgressItems_FWD_DEFINED__ */


#ifndef __ProgressItems_FWD_DEFINED__
#define __ProgressItems_FWD_DEFINED__

#ifdef __cplusplus
typedef class ProgressItems ProgressItems;
#else
typedef struct ProgressItems ProgressItems;
#endif /* __cplusplus */

#endif 	/* __ProgressItems_FWD_DEFINED__ */


#ifndef __FsiDirectoryItem_FWD_DEFINED__
#define __FsiDirectoryItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsiDirectoryItem FsiDirectoryItem;
#else
typedef struct FsiDirectoryItem FsiDirectoryItem;
#endif /* __cplusplus */

#endif 	/* __FsiDirectoryItem_FWD_DEFINED__ */


#ifndef __FsiFileItem_FWD_DEFINED__
#define __FsiFileItem_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsiFileItem FsiFileItem;
#else
typedef struct FsiFileItem FsiFileItem;
#endif /* __cplusplus */

#endif 	/* __FsiFileItem_FWD_DEFINED__ */


#ifndef __EnumFsiItems_FWD_DEFINED__
#define __EnumFsiItems_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnumFsiItems EnumFsiItems;
#else
typedef struct EnumFsiItems EnumFsiItems;
#endif /* __cplusplus */

#endif 	/* __EnumFsiItems_FWD_DEFINED__ */


#ifndef __FsiNamedStreams_FWD_DEFINED__
#define __FsiNamedStreams_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsiNamedStreams FsiNamedStreams;
#else
typedef struct FsiNamedStreams FsiNamedStreams;
#endif /* __cplusplus */

#endif 	/* __FsiNamedStreams_FWD_DEFINED__ */


#ifndef __MsftFileSystemImage_FWD_DEFINED__
#define __MsftFileSystemImage_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftFileSystemImage MsftFileSystemImage;
#else
typedef struct MsftFileSystemImage MsftFileSystemImage;
#endif /* __cplusplus */

#endif 	/* __MsftFileSystemImage_FWD_DEFINED__ */


#ifndef __MsftIsoImageManager_FWD_DEFINED__
#define __MsftIsoImageManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class MsftIsoImageManager MsftIsoImageManager;
#else
typedef struct MsftIsoImageManager MsftIsoImageManager;
#endif /* __cplusplus */

#endif 	/* __MsftIsoImageManager_FWD_DEFINED__ */


#ifndef __BlockRange_FWD_DEFINED__
#define __BlockRange_FWD_DEFINED__

#ifdef __cplusplus
typedef class BlockRange BlockRange;
#else
typedef struct BlockRange BlockRange;
#endif /* __cplusplus */

#endif 	/* __BlockRange_FWD_DEFINED__ */


#ifndef __BlockRangeList_FWD_DEFINED__
#define __BlockRangeList_FWD_DEFINED__

#ifdef __cplusplus
typedef class BlockRangeList BlockRangeList;
#else
typedef struct BlockRangeList BlockRangeList;
#endif /* __cplusplus */

#endif 	/* __BlockRangeList_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "imapi2.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_imapi2fs_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define IMAPI2FS_BOOT_ENTRY_COUNT_MAX    32
typedef /* [helpstring][public][v1_enum] */ 
enum FsiItemType
    {
        FsiItemNotFound	= 0,
        FsiItemDirectory	= 1,
        FsiItemFile	= 2
    } 	FsiItemType;

typedef /* [helpstring][public][v1_enum] */ 
enum FsiFileSystems
    {
        FsiFileSystemNone	= 0,
        FsiFileSystemISO9660	= 1,
        FsiFileSystemJoliet	= 2,
        FsiFileSystemUDF	= 4,
        FsiFileSystemUnknown	= 0x40000000
    } 	FsiFileSystems;

typedef /* [helpstring][public][v1_enum] */ 
enum EmulationType
    {
        EmulationNone	= 0,
        Emulation12MFloppy	= 1,
        Emulation144MFloppy	= 2,
        Emulation288MFloppy	= 3,
        EmulationHardDisk	= 4
    } 	EmulationType;

typedef /* [helpstring][public][v1_enum] */ 
enum PlatformId
    {
        PlatformX86	= 0,
        PlatformPowerPC	= 1,
        PlatformMac	= 2,
        PlatformEFI	= 0xef
    } 	PlatformId;



extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0000_v0_0_s_ifspec;

#ifndef __IBootOptions_INTERFACE_DEFINED__
#define __IBootOptions_INTERFACE_DEFINED__

/* interface IBootOptions */
/* [helpstring][uuid][oleautomation][nonextensible][dual][unique][object] */ 


EXTERN_C const IID IID_IBootOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FD4-975B-59BE-A960-9A2A262853A5")
    IBootOptions : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BootImage( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Manufacturer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Manufacturer( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlatformId( 
            /* [retval][out] */ __RPC__out PlatformId *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlatformId( 
            /* [in] */ PlatformId newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Emulation( 
            /* [retval][out] */ __RPC__out EmulationType *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Emulation( 
            /* [in] */ EmulationType newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageSize( 
            /* [retval][out] */ __RPC__out ULONG *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AssignBootImage( 
            /* [in] */ __RPC__in_opt IStream *newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBootOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBootOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBootOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBootOptions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IBootOptions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IBootOptions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IBootOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IBootOptions * This,
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
        
        DECLSPEC_XFGVIRT(IBootOptions, get_BootImage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BootImage )( 
            __RPC__in IBootOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, get_Manufacturer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Manufacturer )( 
            __RPC__in IBootOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, put_Manufacturer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Manufacturer )( 
            __RPC__in IBootOptions * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, get_PlatformId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlatformId )( 
            __RPC__in IBootOptions * This,
            /* [retval][out] */ __RPC__out PlatformId *pVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, put_PlatformId)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlatformId )( 
            __RPC__in IBootOptions * This,
            /* [in] */ PlatformId newVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, get_Emulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Emulation )( 
            __RPC__in IBootOptions * This,
            /* [retval][out] */ __RPC__out EmulationType *pVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, put_Emulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Emulation )( 
            __RPC__in IBootOptions * This,
            /* [in] */ EmulationType newVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, get_ImageSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageSize )( 
            __RPC__in IBootOptions * This,
            /* [retval][out] */ __RPC__out ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IBootOptions, AssignBootImage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AssignBootImage )( 
            __RPC__in IBootOptions * This,
            /* [in] */ __RPC__in_opt IStream *newVal);
        
        END_INTERFACE
    } IBootOptionsVtbl;

    interface IBootOptions
    {
        CONST_VTBL struct IBootOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBootOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBootOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBootOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBootOptions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IBootOptions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IBootOptions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IBootOptions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IBootOptions_get_BootImage(This,pVal)	\
    ( (This)->lpVtbl -> get_BootImage(This,pVal) ) 

#define IBootOptions_get_Manufacturer(This,pVal)	\
    ( (This)->lpVtbl -> get_Manufacturer(This,pVal) ) 

#define IBootOptions_put_Manufacturer(This,newVal)	\
    ( (This)->lpVtbl -> put_Manufacturer(This,newVal) ) 

#define IBootOptions_get_PlatformId(This,pVal)	\
    ( (This)->lpVtbl -> get_PlatformId(This,pVal) ) 

#define IBootOptions_put_PlatformId(This,newVal)	\
    ( (This)->lpVtbl -> put_PlatformId(This,newVal) ) 

#define IBootOptions_get_Emulation(This,pVal)	\
    ( (This)->lpVtbl -> get_Emulation(This,pVal) ) 

#define IBootOptions_put_Emulation(This,newVal)	\
    ( (This)->lpVtbl -> put_Emulation(This,newVal) ) 

#define IBootOptions_get_ImageSize(This,pVal)	\
    ( (This)->lpVtbl -> get_ImageSize(This,pVal) ) 

#define IBootOptions_AssignBootImage(This,newVal)	\
    ( (This)->lpVtbl -> AssignBootImage(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBootOptions_INTERFACE_DEFINED__ */


#ifndef __IProgressItem_INTERFACE_DEFINED__
#define __IProgressItem_INTERFACE_DEFINED__

/* interface IProgressItem */
/* [helpstring][uuid][oleautomation][nonextensible][dual][unique][object] */ 


EXTERN_C const IID IID_IProgressItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FD5-975B-59BE-A960-9A2A262853A5")
    IProgressItem : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *desc) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FirstBlock( 
            /* [retval][out] */ __RPC__out ULONG *block) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastBlock( 
            /* [retval][out] */ __RPC__out ULONG *block) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockCount( 
            /* [retval][out] */ __RPC__out ULONG *blocks) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProgressItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProgressItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProgressItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProgressItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IProgressItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IProgressItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IProgressItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IProgressItem * This,
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
        
        DECLSPEC_XFGVIRT(IProgressItem, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IProgressItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *desc);
        
        DECLSPEC_XFGVIRT(IProgressItem, get_FirstBlock)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirstBlock )( 
            __RPC__in IProgressItem * This,
            /* [retval][out] */ __RPC__out ULONG *block);
        
        DECLSPEC_XFGVIRT(IProgressItem, get_LastBlock)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastBlock )( 
            __RPC__in IProgressItem * This,
            /* [retval][out] */ __RPC__out ULONG *block);
        
        DECLSPEC_XFGVIRT(IProgressItem, get_BlockCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockCount )( 
            __RPC__in IProgressItem * This,
            /* [retval][out] */ __RPC__out ULONG *blocks);
        
        END_INTERFACE
    } IProgressItemVtbl;

    interface IProgressItem
    {
        CONST_VTBL struct IProgressItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProgressItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProgressItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProgressItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProgressItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IProgressItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IProgressItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IProgressItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IProgressItem_get_Description(This,desc)	\
    ( (This)->lpVtbl -> get_Description(This,desc) ) 

#define IProgressItem_get_FirstBlock(This,block)	\
    ( (This)->lpVtbl -> get_FirstBlock(This,block) ) 

#define IProgressItem_get_LastBlock(This,block)	\
    ( (This)->lpVtbl -> get_LastBlock(This,block) ) 

#define IProgressItem_get_BlockCount(This,blocks)	\
    ( (This)->lpVtbl -> get_BlockCount(This,blocks) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProgressItem_INTERFACE_DEFINED__ */


#ifndef __IEnumProgressItems_INTERFACE_DEFINED__
#define __IEnumProgressItems_INTERFACE_DEFINED__

/* interface IEnumProgressItems */
/* [helpstring][uuid][unique][object] */ 


EXTERN_C const IID IID_IEnumProgressItems;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FD6-975B-59BE-A960-9A2A262853A5")
    IEnumProgressItems : public IUnknown
    {
    public:
        virtual /* [helpstring][local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ IProgressItem **rgelt,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumProgressItems **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumProgressItemsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumProgressItems * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumProgressItems * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumProgressItems * This);
        
        DECLSPEC_XFGVIRT(IEnumProgressItems, Next)
        /* [helpstring][local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumProgressItems * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ IProgressItem **rgelt,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumProgressItems, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumProgressItems * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumProgressItems, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumProgressItems * This);
        
        DECLSPEC_XFGVIRT(IEnumProgressItems, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumProgressItems * This,
            /* [out] */ __RPC__deref_out_opt IEnumProgressItems **ppEnum);
        
        END_INTERFACE
    } IEnumProgressItemsVtbl;

    interface IEnumProgressItems
    {
        CONST_VTBL struct IEnumProgressItemsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumProgressItems_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumProgressItems_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumProgressItems_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumProgressItems_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumProgressItems_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumProgressItems_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumProgressItems_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumProgressItems_RemoteNext_Proxy( 
    __RPC__in IEnumProgressItems * This,
    /* [range][in] */ __RPC__in_range(1,0x7fffffff) ULONG celt,
    /* [size_is][out] */ __RPC__out_ecount_full(celt) IProgressItem **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumProgressItems_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumProgressItems_INTERFACE_DEFINED__ */


#ifndef __IProgressItems_INTERFACE_DEFINED__
#define __IProgressItems_INTERFACE_DEFINED__

/* interface IProgressItems */
/* [helpstring][uuid][oleautomation][nonextensible][dual][unique][object] */ 


EXTERN_C const IID IID_IProgressItems;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FD7-975B-59BE-A960-9A2A262853A5")
    IProgressItems : public IDispatch
    {
    public:
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItem **item) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProgressItemFromBlock( 
            /* [in] */ ULONG block,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItem **item) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProgressItemFromDescription( 
            /* [in] */ __RPC__in BSTR description,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItem **item) = 0;
        
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnumProgressItems( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumProgressItems **NewEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProgressItemsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProgressItems * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProgressItems * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProgressItems * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IProgressItems * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IProgressItems * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IProgressItems * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IProgressItems * This,
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
        
        DECLSPEC_XFGVIRT(IProgressItems, get__NewEnum)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IProgressItems * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum);
        
        DECLSPEC_XFGVIRT(IProgressItems, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IProgressItems * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItem **item);
        
        DECLSPEC_XFGVIRT(IProgressItems, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IProgressItems * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IProgressItems, ProgressItemFromBlock)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ProgressItemFromBlock )( 
            __RPC__in IProgressItems * This,
            /* [in] */ ULONG block,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItem **item);
        
        DECLSPEC_XFGVIRT(IProgressItems, ProgressItemFromDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ProgressItemFromDescription )( 
            __RPC__in IProgressItems * This,
            /* [in] */ __RPC__in BSTR description,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItem **item);
        
        DECLSPEC_XFGVIRT(IProgressItems, get_EnumProgressItems)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumProgressItems )( 
            __RPC__in IProgressItems * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumProgressItems **NewEnum);
        
        END_INTERFACE
    } IProgressItemsVtbl;

    interface IProgressItems
    {
        CONST_VTBL struct IProgressItemsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProgressItems_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProgressItems_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProgressItems_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProgressItems_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IProgressItems_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IProgressItems_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IProgressItems_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IProgressItems_get__NewEnum(This,NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,NewEnum) ) 

#define IProgressItems_get_Item(This,Index,item)	\
    ( (This)->lpVtbl -> get_Item(This,Index,item) ) 

#define IProgressItems_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IProgressItems_ProgressItemFromBlock(This,block,item)	\
    ( (This)->lpVtbl -> ProgressItemFromBlock(This,block,item) ) 

#define IProgressItems_ProgressItemFromDescription(This,description,item)	\
    ( (This)->lpVtbl -> ProgressItemFromDescription(This,description,item) ) 

#define IProgressItems_get_EnumProgressItems(This,NewEnum)	\
    ( (This)->lpVtbl -> get_EnumProgressItems(This,NewEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProgressItems_INTERFACE_DEFINED__ */


#ifndef __IFileSystemImageResult_INTERFACE_DEFINED__
#define __IFileSystemImageResult_INTERFACE_DEFINED__

/* interface IFileSystemImageResult */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFileSystemImageResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FD8-975B-59BE-A960-9A2A262853A5")
    IFileSystemImageResult : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProgressItems( 
            /* [retval][out] */ __RPC__deref_out_opt IProgressItems **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalBlocks( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockSize( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSystemImageResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileSystemImageResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileSystemImageResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileSystemImageResult * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFileSystemImageResult * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFileSystemImageResult * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFileSystemImageResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFileSystemImageResult * This,
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
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_ImageStream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageStream )( 
            __RPC__in IFileSystemImageResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_ProgressItems)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgressItems )( 
            __RPC__in IFileSystemImageResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItems **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_TotalBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalBlocks )( 
            __RPC__in IFileSystemImageResult * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_BlockSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockSize )( 
            __RPC__in IFileSystemImageResult * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_DiscId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscId )( 
            __RPC__in IFileSystemImageResult * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        END_INTERFACE
    } IFileSystemImageResultVtbl;

    interface IFileSystemImageResult
    {
        CONST_VTBL struct IFileSystemImageResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSystemImageResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSystemImageResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSystemImageResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSystemImageResult_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFileSystemImageResult_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFileSystemImageResult_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFileSystemImageResult_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFileSystemImageResult_get_ImageStream(This,pVal)	\
    ( (This)->lpVtbl -> get_ImageStream(This,pVal) ) 

#define IFileSystemImageResult_get_ProgressItems(This,pVal)	\
    ( (This)->lpVtbl -> get_ProgressItems(This,pVal) ) 

#define IFileSystemImageResult_get_TotalBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_TotalBlocks(This,pVal) ) 

#define IFileSystemImageResult_get_BlockSize(This,pVal)	\
    ( (This)->lpVtbl -> get_BlockSize(This,pVal) ) 

#define IFileSystemImageResult_get_DiscId(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscId(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSystemImageResult_INTERFACE_DEFINED__ */


#ifndef __IFileSystemImageResult2_INTERFACE_DEFINED__
#define __IFileSystemImageResult2_INTERFACE_DEFINED__

/* interface IFileSystemImageResult2 */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFileSystemImageResult2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B507CA29-2204-11DD-966A-001AA01BBC58")
    IFileSystemImageResult2 : public IFileSystemImageResult
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModifiedBlocks( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IBlockRangeList **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSystemImageResult2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileSystemImageResult2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileSystemImageResult2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFileSystemImageResult2 * This,
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
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_ImageStream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageStream )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_ProgressItems)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgressItems )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IProgressItems **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_TotalBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalBlocks )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_BlockSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockSize )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult, get_DiscId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscId )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImageResult2, get_ModifiedBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModifiedBlocks )( 
            __RPC__in IFileSystemImageResult2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IBlockRangeList **pVal);
        
        END_INTERFACE
    } IFileSystemImageResult2Vtbl;

    interface IFileSystemImageResult2
    {
        CONST_VTBL struct IFileSystemImageResult2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSystemImageResult2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSystemImageResult2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSystemImageResult2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSystemImageResult2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFileSystemImageResult2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFileSystemImageResult2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFileSystemImageResult2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFileSystemImageResult2_get_ImageStream(This,pVal)	\
    ( (This)->lpVtbl -> get_ImageStream(This,pVal) ) 

#define IFileSystemImageResult2_get_ProgressItems(This,pVal)	\
    ( (This)->lpVtbl -> get_ProgressItems(This,pVal) ) 

#define IFileSystemImageResult2_get_TotalBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_TotalBlocks(This,pVal) ) 

#define IFileSystemImageResult2_get_BlockSize(This,pVal)	\
    ( (This)->lpVtbl -> get_BlockSize(This,pVal) ) 

#define IFileSystemImageResult2_get_DiscId(This,pVal)	\
    ( (This)->lpVtbl -> get_DiscId(This,pVal) ) 


#define IFileSystemImageResult2_get_ModifiedBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_ModifiedBlocks(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSystemImageResult2_INTERFACE_DEFINED__ */


#ifndef __IFsiItem_INTERFACE_DEFINED__
#define __IFsiItem_INTERFACE_DEFINED__

/* interface IFsiItem */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFsiItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FD9-975B-59BE-A960-9A2A262853A5")
    IFsiItem : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FullPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CreationTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CreationTime( 
            /* [in] */ DATE newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastAccessedTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LastAccessedTime( 
            /* [in] */ DATE newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastModifiedTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LastModifiedTime( 
            /* [in] */ DATE newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsHidden( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IsHidden( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FileSystemName( 
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FileSystemPath( 
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsiItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsiItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsiItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsiItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsiItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsiItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsiItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsiItem * This,
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
        
        DECLSPEC_XFGVIRT(IFsiItem, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsiItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_FullPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullPath )( 
            __RPC__in IFsiItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IFsiItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_CreationTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreationTime )( 
            __RPC__in IFsiItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastAccessedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastAccessedTime )( 
            __RPC__in IFsiItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastAccessedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastAccessedTime )( 
            __RPC__in IFsiItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastModifiedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastModifiedTime )( 
            __RPC__in IFsiItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastModifiedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastModifiedTime )( 
            __RPC__in IFsiItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_IsHidden)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsHidden )( 
            __RPC__in IFsiItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_IsHidden)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsHidden )( 
            __RPC__in IFsiItem * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemName )( 
            __RPC__in IFsiItem * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemPath )( 
            __RPC__in IFsiItem * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        END_INTERFACE
    } IFsiItemVtbl;

    interface IFsiItem
    {
        CONST_VTBL struct IFsiItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsiItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsiItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsiItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsiItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsiItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsiItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsiItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsiItem_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IFsiItem_get_FullPath(This,pVal)	\
    ( (This)->lpVtbl -> get_FullPath(This,pVal) ) 

#define IFsiItem_get_CreationTime(This,pVal)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pVal) ) 

#define IFsiItem_put_CreationTime(This,newVal)	\
    ( (This)->lpVtbl -> put_CreationTime(This,newVal) ) 

#define IFsiItem_get_LastAccessedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastAccessedTime(This,pVal) ) 

#define IFsiItem_put_LastAccessedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastAccessedTime(This,newVal) ) 

#define IFsiItem_get_LastModifiedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastModifiedTime(This,pVal) ) 

#define IFsiItem_put_LastModifiedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastModifiedTime(This,newVal) ) 

#define IFsiItem_get_IsHidden(This,pVal)	\
    ( (This)->lpVtbl -> get_IsHidden(This,pVal) ) 

#define IFsiItem_put_IsHidden(This,newVal)	\
    ( (This)->lpVtbl -> put_IsHidden(This,newVal) ) 

#define IFsiItem_FileSystemName(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemName(This,fileSystem,pVal) ) 

#define IFsiItem_FileSystemPath(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemPath(This,fileSystem,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsiItem_INTERFACE_DEFINED__ */


#ifndef __IEnumFsiItems_INTERFACE_DEFINED__
#define __IEnumFsiItems_INTERFACE_DEFINED__

/* interface IEnumFsiItems */
/* [helpstring][uuid][unique][object] */ 


EXTERN_C const IID IID_IEnumFsiItems;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FDA-975B-59BE-A960-9A2A262853A5")
    IEnumFsiItems : public IUnknown
    {
    public:
        virtual /* [helpstring][local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ IFsiItem **rgelt,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumFsiItems **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumFsiItemsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumFsiItems * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumFsiItems * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumFsiItems * This);
        
        DECLSPEC_XFGVIRT(IEnumFsiItems, Next)
        /* [helpstring][local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumFsiItems * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ IFsiItem **rgelt,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumFsiItems, Skip)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumFsiItems * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumFsiItems, Reset)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumFsiItems * This);
        
        DECLSPEC_XFGVIRT(IEnumFsiItems, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumFsiItems * This,
            /* [out] */ __RPC__deref_out_opt IEnumFsiItems **ppEnum);
        
        END_INTERFACE
    } IEnumFsiItemsVtbl;

    interface IEnumFsiItems
    {
        CONST_VTBL struct IEnumFsiItemsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumFsiItems_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumFsiItems_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumFsiItems_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumFsiItems_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumFsiItems_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumFsiItems_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumFsiItems_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumFsiItems_RemoteNext_Proxy( 
    __RPC__in IEnumFsiItems * This,
    /* [range][in] */ __RPC__in_range(0,0x7fffffff) ULONG celt,
    /* [size_is][out] */ __RPC__out_ecount_full(celt) IFsiItem **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumFsiItems_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumFsiItems_INTERFACE_DEFINED__ */


#ifndef __IFsiFileItem_INTERFACE_DEFINED__
#define __IFsiFileItem_INTERFACE_DEFINED__

/* interface IFsiFileItem */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFsiFileItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FDB-975B-59BE-A960-9A2A262853A5")
    IFsiFileItem : public IFsiItem
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataSize( 
            /* [retval][out] */ __RPC__out LONGLONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataSize32BitLow( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataSize32BitHigh( 
            /* [retval][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ __RPC__in_opt IStream *newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsiFileItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsiFileItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsiFileItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsiFileItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsiFileItem * This,
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
        
        DECLSPEC_XFGVIRT(IFsiItem, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_FullPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullPath )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_CreationTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreationTime )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastAccessedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastAccessedTime )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastAccessedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastAccessedTime )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastModifiedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastModifiedTime )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastModifiedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastModifiedTime )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_IsHidden)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsHidden )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_IsHidden)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsHidden )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemName )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemPath )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_DataSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSize )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out LONGLONG *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_DataSize32BitLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSize32BitLow )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_DataSize32BitHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSize32BitHigh )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_Data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IFsiFileItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, put_Data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IFsiFileItem * This,
            /* [in] */ __RPC__in_opt IStream *newVal);
        
        END_INTERFACE
    } IFsiFileItemVtbl;

    interface IFsiFileItem
    {
        CONST_VTBL struct IFsiFileItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsiFileItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsiFileItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsiFileItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsiFileItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsiFileItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsiFileItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsiFileItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsiFileItem_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IFsiFileItem_get_FullPath(This,pVal)	\
    ( (This)->lpVtbl -> get_FullPath(This,pVal) ) 

#define IFsiFileItem_get_CreationTime(This,pVal)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pVal) ) 

#define IFsiFileItem_put_CreationTime(This,newVal)	\
    ( (This)->lpVtbl -> put_CreationTime(This,newVal) ) 

#define IFsiFileItem_get_LastAccessedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastAccessedTime(This,pVal) ) 

#define IFsiFileItem_put_LastAccessedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastAccessedTime(This,newVal) ) 

#define IFsiFileItem_get_LastModifiedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastModifiedTime(This,pVal) ) 

#define IFsiFileItem_put_LastModifiedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastModifiedTime(This,newVal) ) 

#define IFsiFileItem_get_IsHidden(This,pVal)	\
    ( (This)->lpVtbl -> get_IsHidden(This,pVal) ) 

#define IFsiFileItem_put_IsHidden(This,newVal)	\
    ( (This)->lpVtbl -> put_IsHidden(This,newVal) ) 

#define IFsiFileItem_FileSystemName(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemName(This,fileSystem,pVal) ) 

#define IFsiFileItem_FileSystemPath(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemPath(This,fileSystem,pVal) ) 


#define IFsiFileItem_get_DataSize(This,pVal)	\
    ( (This)->lpVtbl -> get_DataSize(This,pVal) ) 

#define IFsiFileItem_get_DataSize32BitLow(This,pVal)	\
    ( (This)->lpVtbl -> get_DataSize32BitLow(This,pVal) ) 

#define IFsiFileItem_get_DataSize32BitHigh(This,pVal)	\
    ( (This)->lpVtbl -> get_DataSize32BitHigh(This,pVal) ) 

#define IFsiFileItem_get_Data(This,pVal)	\
    ( (This)->lpVtbl -> get_Data(This,pVal) ) 

#define IFsiFileItem_put_Data(This,newVal)	\
    ( (This)->lpVtbl -> put_Data(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsiFileItem_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2fs_0000_0009 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0009_v0_0_s_ifspec;

#ifndef __IFsiFileItem2_INTERFACE_DEFINED__
#define __IFsiFileItem2_INTERFACE_DEFINED__

/* interface IFsiFileItem2 */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFsiFileItem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("199D0C19-11E1-40eb-8EC2-C8C822A07792")
    IFsiFileItem2 : public IFsiFileItem
    {
    public:
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_FsiNamedStreams( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiNamedStreams **streams) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsNamedStream( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddStream( 
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ __RPC__in_opt IStream *streamData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveStream( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsRealTime( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IsRealTime( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsiFileItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsiFileItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsiFileItem2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsiFileItem2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsiFileItem2 * This,
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
        
        DECLSPEC_XFGVIRT(IFsiItem, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_FullPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullPath )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_CreationTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreationTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastAccessedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastAccessedTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastAccessedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastAccessedTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastModifiedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastModifiedTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastModifiedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastModifiedTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_IsHidden)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsHidden )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_IsHidden)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsHidden )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemName )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemPath )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_DataSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSize )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out LONGLONG *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_DataSize32BitLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSize32BitLow )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_DataSize32BitHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataSize32BitHigh )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, get_Data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem, put_Data)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ __RPC__in_opt IStream *newVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem2, get_FsiNamedStreams)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FsiNamedStreams )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiNamedStreams **streams);
        
        DECLSPEC_XFGVIRT(IFsiFileItem2, get_IsNamedStream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsNamedStream )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem2, AddStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ __RPC__in_opt IStream *streamData);
        
        DECLSPEC_XFGVIRT(IFsiFileItem2, RemoveStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveStream )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFsiFileItem2, get_IsRealTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsRealTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiFileItem2, put_IsRealTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsRealTime )( 
            __RPC__in IFsiFileItem2 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        END_INTERFACE
    } IFsiFileItem2Vtbl;

    interface IFsiFileItem2
    {
        CONST_VTBL struct IFsiFileItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsiFileItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsiFileItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsiFileItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsiFileItem2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsiFileItem2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsiFileItem2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsiFileItem2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsiFileItem2_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IFsiFileItem2_get_FullPath(This,pVal)	\
    ( (This)->lpVtbl -> get_FullPath(This,pVal) ) 

#define IFsiFileItem2_get_CreationTime(This,pVal)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pVal) ) 

#define IFsiFileItem2_put_CreationTime(This,newVal)	\
    ( (This)->lpVtbl -> put_CreationTime(This,newVal) ) 

#define IFsiFileItem2_get_LastAccessedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastAccessedTime(This,pVal) ) 

#define IFsiFileItem2_put_LastAccessedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastAccessedTime(This,newVal) ) 

#define IFsiFileItem2_get_LastModifiedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastModifiedTime(This,pVal) ) 

#define IFsiFileItem2_put_LastModifiedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastModifiedTime(This,newVal) ) 

#define IFsiFileItem2_get_IsHidden(This,pVal)	\
    ( (This)->lpVtbl -> get_IsHidden(This,pVal) ) 

#define IFsiFileItem2_put_IsHidden(This,newVal)	\
    ( (This)->lpVtbl -> put_IsHidden(This,newVal) ) 

#define IFsiFileItem2_FileSystemName(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemName(This,fileSystem,pVal) ) 

#define IFsiFileItem2_FileSystemPath(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemPath(This,fileSystem,pVal) ) 


#define IFsiFileItem2_get_DataSize(This,pVal)	\
    ( (This)->lpVtbl -> get_DataSize(This,pVal) ) 

#define IFsiFileItem2_get_DataSize32BitLow(This,pVal)	\
    ( (This)->lpVtbl -> get_DataSize32BitLow(This,pVal) ) 

#define IFsiFileItem2_get_DataSize32BitHigh(This,pVal)	\
    ( (This)->lpVtbl -> get_DataSize32BitHigh(This,pVal) ) 

#define IFsiFileItem2_get_Data(This,pVal)	\
    ( (This)->lpVtbl -> get_Data(This,pVal) ) 

#define IFsiFileItem2_put_Data(This,newVal)	\
    ( (This)->lpVtbl -> put_Data(This,newVal) ) 


#define IFsiFileItem2_get_FsiNamedStreams(This,streams)	\
    ( (This)->lpVtbl -> get_FsiNamedStreams(This,streams) ) 

#define IFsiFileItem2_get_IsNamedStream(This,pVal)	\
    ( (This)->lpVtbl -> get_IsNamedStream(This,pVal) ) 

#define IFsiFileItem2_AddStream(This,name,streamData)	\
    ( (This)->lpVtbl -> AddStream(This,name,streamData) ) 

#define IFsiFileItem2_RemoveStream(This,name)	\
    ( (This)->lpVtbl -> RemoveStream(This,name) ) 

#define IFsiFileItem2_get_IsRealTime(This,pVal)	\
    ( (This)->lpVtbl -> get_IsRealTime(This,pVal) ) 

#define IFsiFileItem2_put_IsRealTime(This,newVal)	\
    ( (This)->lpVtbl -> put_IsRealTime(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsiFileItem2_INTERFACE_DEFINED__ */


#ifndef __IFsiNamedStreams_INTERFACE_DEFINED__
#define __IFsiNamedStreams_INTERFACE_DEFINED__

/* interface IFsiNamedStreams */
/* [helpstring][uuid][nonextensible][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFsiNamedStreams;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ED79BA56-5294-4250-8D46-F9AECEE23459")
    IFsiNamedStreams : public IDispatch
    {
    public:
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG index,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiFileItem2 **item) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][ref][out] */ __RPC__out LONG *count) = 0;
        
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnumNamedStreams( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IEnumFsiItems **NewEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsiNamedStreamsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsiNamedStreams * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsiNamedStreams * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsiNamedStreams * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsiNamedStreams * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsiNamedStreams * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsiNamedStreams * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsiNamedStreams * This,
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
        
        DECLSPEC_XFGVIRT(IFsiNamedStreams, get__NewEnum)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFsiNamedStreams * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum);
        
        DECLSPEC_XFGVIRT(IFsiNamedStreams, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFsiNamedStreams * This,
            /* [in] */ LONG index,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiFileItem2 **item);
        
        DECLSPEC_XFGVIRT(IFsiNamedStreams, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFsiNamedStreams * This,
            /* [retval][ref][out] */ __RPC__out LONG *count);
        
        DECLSPEC_XFGVIRT(IFsiNamedStreams, get_EnumNamedStreams)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumNamedStreams )( 
            __RPC__in IFsiNamedStreams * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IEnumFsiItems **NewEnum);
        
        END_INTERFACE
    } IFsiNamedStreamsVtbl;

    interface IFsiNamedStreams
    {
        CONST_VTBL struct IFsiNamedStreamsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsiNamedStreams_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsiNamedStreams_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsiNamedStreams_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsiNamedStreams_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsiNamedStreams_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsiNamedStreams_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsiNamedStreams_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsiNamedStreams_get__NewEnum(This,NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,NewEnum) ) 

#define IFsiNamedStreams_get_Item(This,index,item)	\
    ( (This)->lpVtbl -> get_Item(This,index,item) ) 

#define IFsiNamedStreams_get_Count(This,count)	\
    ( (This)->lpVtbl -> get_Count(This,count) ) 

#define IFsiNamedStreams_get_EnumNamedStreams(This,NewEnum)	\
    ( (This)->lpVtbl -> get_EnumNamedStreams(This,NewEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsiNamedStreams_INTERFACE_DEFINED__ */


#ifndef __IFsiDirectoryItem_INTERFACE_DEFINED__
#define __IFsiDirectoryItem_INTERFACE_DEFINED__

/* interface IFsiDirectoryItem */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFsiDirectoryItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FDC-975B-59BE-A960-9A2A262853A5")
    IFsiDirectoryItem : public IFsiItem
    {
    public:
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsiItem **item) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *Count) = 0;
        
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnumFsiItems( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumFsiItems **NewEnum) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddDirectory( 
            /* [in] */ __RPC__in BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddFile( 
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in_opt IStream *fileData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddTree( 
            /* [in] */ __RPC__in BSTR sourceDirectory,
            /* [in] */ VARIANT_BOOL includeBaseDirectory) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IFsiItem *item) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveTree( 
            /* [in] */ __RPC__in BSTR path) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsiDirectoryItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsiDirectoryItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsiDirectoryItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsiDirectoryItem * This,
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
        
        DECLSPEC_XFGVIRT(IFsiItem, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_FullPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullPath )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_CreationTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreationTime )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastAccessedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastAccessedTime )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastAccessedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastAccessedTime )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastModifiedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastModifiedTime )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastModifiedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastModifiedTime )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_IsHidden)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsHidden )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_IsHidden)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsHidden )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemName )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemPath )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get__NewEnum)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsiItem **item);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__out LONG *Count);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get_EnumFsiItems)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumFsiItems )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumFsiItems **NewEnum);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, AddDirectory)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddDirectory )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, AddFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddFile )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in_opt IStream *fileData);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, AddTree)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddTree )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in BSTR sourceDirectory,
            /* [in] */ VARIANT_BOOL includeBaseDirectory);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in_opt IFsiItem *item);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, RemoveTree)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveTree )( 
            __RPC__in IFsiDirectoryItem * This,
            /* [in] */ __RPC__in BSTR path);
        
        END_INTERFACE
    } IFsiDirectoryItemVtbl;

    interface IFsiDirectoryItem
    {
        CONST_VTBL struct IFsiDirectoryItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsiDirectoryItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsiDirectoryItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsiDirectoryItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsiDirectoryItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsiDirectoryItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsiDirectoryItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsiDirectoryItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsiDirectoryItem_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IFsiDirectoryItem_get_FullPath(This,pVal)	\
    ( (This)->lpVtbl -> get_FullPath(This,pVal) ) 

#define IFsiDirectoryItem_get_CreationTime(This,pVal)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pVal) ) 

#define IFsiDirectoryItem_put_CreationTime(This,newVal)	\
    ( (This)->lpVtbl -> put_CreationTime(This,newVal) ) 

#define IFsiDirectoryItem_get_LastAccessedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastAccessedTime(This,pVal) ) 

#define IFsiDirectoryItem_put_LastAccessedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastAccessedTime(This,newVal) ) 

#define IFsiDirectoryItem_get_LastModifiedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastModifiedTime(This,pVal) ) 

#define IFsiDirectoryItem_put_LastModifiedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastModifiedTime(This,newVal) ) 

#define IFsiDirectoryItem_get_IsHidden(This,pVal)	\
    ( (This)->lpVtbl -> get_IsHidden(This,pVal) ) 

#define IFsiDirectoryItem_put_IsHidden(This,newVal)	\
    ( (This)->lpVtbl -> put_IsHidden(This,newVal) ) 

#define IFsiDirectoryItem_FileSystemName(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemName(This,fileSystem,pVal) ) 

#define IFsiDirectoryItem_FileSystemPath(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemPath(This,fileSystem,pVal) ) 


#define IFsiDirectoryItem_get__NewEnum(This,NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,NewEnum) ) 

#define IFsiDirectoryItem_get_Item(This,path,item)	\
    ( (This)->lpVtbl -> get_Item(This,path,item) ) 

#define IFsiDirectoryItem_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IFsiDirectoryItem_get_EnumFsiItems(This,NewEnum)	\
    ( (This)->lpVtbl -> get_EnumFsiItems(This,NewEnum) ) 

#define IFsiDirectoryItem_AddDirectory(This,path)	\
    ( (This)->lpVtbl -> AddDirectory(This,path) ) 

#define IFsiDirectoryItem_AddFile(This,path,fileData)	\
    ( (This)->lpVtbl -> AddFile(This,path,fileData) ) 

#define IFsiDirectoryItem_AddTree(This,sourceDirectory,includeBaseDirectory)	\
    ( (This)->lpVtbl -> AddTree(This,sourceDirectory,includeBaseDirectory) ) 

#define IFsiDirectoryItem_Add(This,item)	\
    ( (This)->lpVtbl -> Add(This,item) ) 

#define IFsiDirectoryItem_Remove(This,path)	\
    ( (This)->lpVtbl -> Remove(This,path) ) 

#define IFsiDirectoryItem_RemoveTree(This,path)	\
    ( (This)->lpVtbl -> RemoveTree(This,path) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsiDirectoryItem_INTERFACE_DEFINED__ */


#ifndef __IFsiDirectoryItem2_INTERFACE_DEFINED__
#define __IFsiDirectoryItem2_INTERFACE_DEFINED__

/* interface IFsiDirectoryItem2 */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFsiDirectoryItem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F7FB4B9B-6D96-4d7b-9115-201B144811EF")
    IFsiDirectoryItem2 : public IFsiDirectoryItem
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddTreeWithNamedStreams( 
            /* [in] */ __RPC__in BSTR sourceDirectory,
            /* [in] */ VARIANT_BOOL includeBaseDirectory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsiDirectoryItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsiDirectoryItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsiDirectoryItem2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsiDirectoryItem2 * This,
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
        
        DECLSPEC_XFGVIRT(IFsiItem, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_FullPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullPath )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_CreationTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreationTime )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastAccessedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastAccessedTime )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastAccessedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastAccessedTime )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_LastModifiedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastModifiedTime )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_LastModifiedTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastModifiedTime )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ DATE newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, get_IsHidden)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsHidden )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, put_IsHidden)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsHidden )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemName )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiItem, FileSystemPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FileSystemPath )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ FsiFileSystems fileSystem,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get__NewEnum)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR path,
            /* [retval][out] */ __RPC__deref_out_opt IFsiItem **item);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__out LONG *Count);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, get_EnumFsiItems)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumFsiItems )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumFsiItems **NewEnum);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, AddDirectory)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddDirectory )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, AddFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddFile )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR path,
            /* [in] */ __RPC__in_opt IStream *fileData);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, AddTree)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddTree )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR sourceDirectory,
            /* [in] */ VARIANT_BOOL includeBaseDirectory);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in_opt IFsiItem *item);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem, RemoveTree)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveTree )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IFsiDirectoryItem2, AddTreeWithNamedStreams)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddTreeWithNamedStreams )( 
            __RPC__in IFsiDirectoryItem2 * This,
            /* [in] */ __RPC__in BSTR sourceDirectory,
            /* [in] */ VARIANT_BOOL includeBaseDirectory);
        
        END_INTERFACE
    } IFsiDirectoryItem2Vtbl;

    interface IFsiDirectoryItem2
    {
        CONST_VTBL struct IFsiDirectoryItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsiDirectoryItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsiDirectoryItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsiDirectoryItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsiDirectoryItem2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsiDirectoryItem2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsiDirectoryItem2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsiDirectoryItem2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsiDirectoryItem2_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IFsiDirectoryItem2_get_FullPath(This,pVal)	\
    ( (This)->lpVtbl -> get_FullPath(This,pVal) ) 

#define IFsiDirectoryItem2_get_CreationTime(This,pVal)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pVal) ) 

#define IFsiDirectoryItem2_put_CreationTime(This,newVal)	\
    ( (This)->lpVtbl -> put_CreationTime(This,newVal) ) 

#define IFsiDirectoryItem2_get_LastAccessedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastAccessedTime(This,pVal) ) 

#define IFsiDirectoryItem2_put_LastAccessedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastAccessedTime(This,newVal) ) 

#define IFsiDirectoryItem2_get_LastModifiedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_LastModifiedTime(This,pVal) ) 

#define IFsiDirectoryItem2_put_LastModifiedTime(This,newVal)	\
    ( (This)->lpVtbl -> put_LastModifiedTime(This,newVal) ) 

#define IFsiDirectoryItem2_get_IsHidden(This,pVal)	\
    ( (This)->lpVtbl -> get_IsHidden(This,pVal) ) 

#define IFsiDirectoryItem2_put_IsHidden(This,newVal)	\
    ( (This)->lpVtbl -> put_IsHidden(This,newVal) ) 

#define IFsiDirectoryItem2_FileSystemName(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemName(This,fileSystem,pVal) ) 

#define IFsiDirectoryItem2_FileSystemPath(This,fileSystem,pVal)	\
    ( (This)->lpVtbl -> FileSystemPath(This,fileSystem,pVal) ) 


#define IFsiDirectoryItem2_get__NewEnum(This,NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,NewEnum) ) 

#define IFsiDirectoryItem2_get_Item(This,path,item)	\
    ( (This)->lpVtbl -> get_Item(This,path,item) ) 

#define IFsiDirectoryItem2_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IFsiDirectoryItem2_get_EnumFsiItems(This,NewEnum)	\
    ( (This)->lpVtbl -> get_EnumFsiItems(This,NewEnum) ) 

#define IFsiDirectoryItem2_AddDirectory(This,path)	\
    ( (This)->lpVtbl -> AddDirectory(This,path) ) 

#define IFsiDirectoryItem2_AddFile(This,path,fileData)	\
    ( (This)->lpVtbl -> AddFile(This,path,fileData) ) 

#define IFsiDirectoryItem2_AddTree(This,sourceDirectory,includeBaseDirectory)	\
    ( (This)->lpVtbl -> AddTree(This,sourceDirectory,includeBaseDirectory) ) 

#define IFsiDirectoryItem2_Add(This,item)	\
    ( (This)->lpVtbl -> Add(This,item) ) 

#define IFsiDirectoryItem2_Remove(This,path)	\
    ( (This)->lpVtbl -> Remove(This,path) ) 

#define IFsiDirectoryItem2_RemoveTree(This,path)	\
    ( (This)->lpVtbl -> RemoveTree(This,path) ) 


#define IFsiDirectoryItem2_AddTreeWithNamedStreams(This,sourceDirectory,includeBaseDirectory)	\
    ( (This)->lpVtbl -> AddTreeWithNamedStreams(This,sourceDirectory,includeBaseDirectory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsiDirectoryItem2_INTERFACE_DEFINED__ */


#ifndef __IFileSystemImage_INTERFACE_DEFINED__
#define __IFileSystemImage_INTERFACE_DEFINED__

/* interface IFileSystemImage */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFileSystemImage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FE1-975B-59BE-A960-9A2A262853A5")
    IFileSystemImage : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Root( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SessionStartBlock( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SessionStartBlock( 
            /* [in] */ LONG newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FreeMediaBlocks( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FreeMediaBlocks( 
            /* [in] */ LONG newVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetMaxMediaBlocksFromDevice( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UsedBlocks( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_VolumeName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImportedVolumeName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BootImageOptions( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IBootOptions **pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BootImageOptions( 
            /* [in] */ __RPC__in_opt IBootOptions *newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileCount( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DirectoryCount( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WorkingDirectory( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_WorkingDirectory( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ChangePoint( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StrictFileSystemCompliance( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StrictFileSystemCompliance( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseRestrictedCharacterSet( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseRestrictedCharacterSet( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileSystemsToCreate( 
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FileSystemsToCreate( 
            /* [in] */ FsiFileSystems newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileSystemsSupported( 
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UDFRevision( 
            /* [in] */ LONG newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UDFRevision( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UDFRevisionsSupported( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ChooseImageDefaults( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ChooseImageDefaultsForMediaType( 
            /* [in] */ IMAPI_MEDIA_PHYSICAL_TYPE value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ISO9660InterchangeLevel( 
            /* [in] */ LONG newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ISO9660InterchangeLevel( 
            /* [retval][ref][out] */ __RPC__out LONG *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ISO9660InterchangeLevelsSupported( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateResultImage( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IFileSystemImageResult **resultStream) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Exists( 
            /* [in] */ __RPC__in BSTR fullPath,
            /* [retval][ref][out] */ __RPC__out FsiItemType *itemType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CalculateDiscIdentifier( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *discIdentifier) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IdentifyFileSystemsOnDisc( 
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *fileSystems) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultFileSystemForImport( 
            /* [in] */ FsiFileSystems fileSystems,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importDefault) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportFileSystem( 
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importedFileSystem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportSpecificFileSystem( 
            /* [in] */ FsiFileSystems fileSystemToUse) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RollbackToChangePoint( 
            /* [in] */ LONG changePoint) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LockInChangePoint( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateDirectoryItem( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **newItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateFileItem( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiFileItem **newItem) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeNameUDF( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeNameJoliet( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeNameISO9660( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StageFiles( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StageFiles( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MultisessionInterfaces( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MultisessionInterfaces( 
            /* [in] */ __RPC__in SAFEARRAY * newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSystemImageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileSystemImage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileSystemImage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFileSystemImage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFileSystemImage * This,
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
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_Root)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Root )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_SessionStartBlock)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionStartBlock )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_SessionStartBlock)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SessionStartBlock )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FreeMediaBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeMediaBlocks )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_FreeMediaBlocks)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FreeMediaBlocks )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, SetMaxMediaBlocksFromDevice)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetMaxMediaBlocksFromDevice )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UsedBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedBlocks )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeName )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_VolumeName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_VolumeName )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ImportedVolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportedVolumeName )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_BootImageOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BootImageOptions )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IBootOptions **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_BootImageOptions)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BootImageOptions )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in_opt IBootOptions *newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileCount )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_DirectoryCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DirectoryCount )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_WorkingDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_WorkingDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ChangePoint)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChangePoint )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_StrictFileSystemCompliance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StrictFileSystemCompliance )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_StrictFileSystemCompliance)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StrictFileSystemCompliance )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UseRestrictedCharacterSet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseRestrictedCharacterSet )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_UseRestrictedCharacterSet)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseRestrictedCharacterSet )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileSystemsToCreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemsToCreate )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_FileSystemsToCreate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileSystemsToCreate )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ FsiFileSystems newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileSystemsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemsSupported )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_UDFRevision)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UDFRevision )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UDFRevision)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UDFRevision )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UDFRevisionsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UDFRevisionsSupported )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ChooseImageDefaults)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChooseImageDefaults )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ChooseImageDefaultsForMediaType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChooseImageDefaultsForMediaType )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ IMAPI_MEDIA_PHYSICAL_TYPE value);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_ISO9660InterchangeLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ISO9660InterchangeLevel )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ISO9660InterchangeLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISO9660InterchangeLevel )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ISO9660InterchangeLevelsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISO9660InterchangeLevelsSupported )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateResultImage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateResultImage )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFileSystemImageResult **resultStream);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, Exists)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Exists )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in BSTR fullPath,
            /* [retval][ref][out] */ __RPC__out FsiItemType *itemType);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CalculateDiscIdentifier)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CalculateDiscIdentifier )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *discIdentifier);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, IdentifyFileSystemsOnDisc)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IdentifyFileSystemsOnDisc )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *fileSystems);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, GetDefaultFileSystemForImport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultFileSystemForImport )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ FsiFileSystems fileSystems,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importDefault);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ImportFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportFileSystem )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importedFileSystem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ImportSpecificFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportSpecificFileSystem )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ FsiFileSystems fileSystemToUse);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, RollbackToChangePoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RollbackToChangePoint )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ LONG changePoint);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, LockInChangePoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LockInChangePoint )( 
            __RPC__in IFileSystemImage * This);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateDirectoryItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDirectoryItem )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **newItem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateFileItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateFileItem )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiFileItem **newItem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameUDF)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameUDF )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameJoliet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameJoliet )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameISO9660)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameISO9660 )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_StageFiles)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StageFiles )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_StageFiles)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StageFiles )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_MultisessionInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultisessionInterfaces )( 
            __RPC__in IFileSystemImage * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_MultisessionInterfaces)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MultisessionInterfaces )( 
            __RPC__in IFileSystemImage * This,
            /* [in] */ __RPC__in SAFEARRAY * newVal);
        
        END_INTERFACE
    } IFileSystemImageVtbl;

    interface IFileSystemImage
    {
        CONST_VTBL struct IFileSystemImageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSystemImage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSystemImage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSystemImage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSystemImage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFileSystemImage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFileSystemImage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFileSystemImage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFileSystemImage_get_Root(This,pVal)	\
    ( (This)->lpVtbl -> get_Root(This,pVal) ) 

#define IFileSystemImage_get_SessionStartBlock(This,pVal)	\
    ( (This)->lpVtbl -> get_SessionStartBlock(This,pVal) ) 

#define IFileSystemImage_put_SessionStartBlock(This,newVal)	\
    ( (This)->lpVtbl -> put_SessionStartBlock(This,newVal) ) 

#define IFileSystemImage_get_FreeMediaBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_FreeMediaBlocks(This,pVal) ) 

#define IFileSystemImage_put_FreeMediaBlocks(This,newVal)	\
    ( (This)->lpVtbl -> put_FreeMediaBlocks(This,newVal) ) 

#define IFileSystemImage_SetMaxMediaBlocksFromDevice(This,discRecorder)	\
    ( (This)->lpVtbl -> SetMaxMediaBlocksFromDevice(This,discRecorder) ) 

#define IFileSystemImage_get_UsedBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_UsedBlocks(This,pVal) ) 

#define IFileSystemImage_get_VolumeName(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeName(This,pVal) ) 

#define IFileSystemImage_put_VolumeName(This,newVal)	\
    ( (This)->lpVtbl -> put_VolumeName(This,newVal) ) 

#define IFileSystemImage_get_ImportedVolumeName(This,pVal)	\
    ( (This)->lpVtbl -> get_ImportedVolumeName(This,pVal) ) 

#define IFileSystemImage_get_BootImageOptions(This,pVal)	\
    ( (This)->lpVtbl -> get_BootImageOptions(This,pVal) ) 

#define IFileSystemImage_put_BootImageOptions(This,newVal)	\
    ( (This)->lpVtbl -> put_BootImageOptions(This,newVal) ) 

#define IFileSystemImage_get_FileCount(This,pVal)	\
    ( (This)->lpVtbl -> get_FileCount(This,pVal) ) 

#define IFileSystemImage_get_DirectoryCount(This,pVal)	\
    ( (This)->lpVtbl -> get_DirectoryCount(This,pVal) ) 

#define IFileSystemImage_get_WorkingDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pVal) ) 

#define IFileSystemImage_put_WorkingDirectory(This,newVal)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,newVal) ) 

#define IFileSystemImage_get_ChangePoint(This,pVal)	\
    ( (This)->lpVtbl -> get_ChangePoint(This,pVal) ) 

#define IFileSystemImage_get_StrictFileSystemCompliance(This,pVal)	\
    ( (This)->lpVtbl -> get_StrictFileSystemCompliance(This,pVal) ) 

#define IFileSystemImage_put_StrictFileSystemCompliance(This,newVal)	\
    ( (This)->lpVtbl -> put_StrictFileSystemCompliance(This,newVal) ) 

#define IFileSystemImage_get_UseRestrictedCharacterSet(This,pVal)	\
    ( (This)->lpVtbl -> get_UseRestrictedCharacterSet(This,pVal) ) 

#define IFileSystemImage_put_UseRestrictedCharacterSet(This,newVal)	\
    ( (This)->lpVtbl -> put_UseRestrictedCharacterSet(This,newVal) ) 

#define IFileSystemImage_get_FileSystemsToCreate(This,pVal)	\
    ( (This)->lpVtbl -> get_FileSystemsToCreate(This,pVal) ) 

#define IFileSystemImage_put_FileSystemsToCreate(This,newVal)	\
    ( (This)->lpVtbl -> put_FileSystemsToCreate(This,newVal) ) 

#define IFileSystemImage_get_FileSystemsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_FileSystemsSupported(This,pVal) ) 

#define IFileSystemImage_put_UDFRevision(This,newVal)	\
    ( (This)->lpVtbl -> put_UDFRevision(This,newVal) ) 

#define IFileSystemImage_get_UDFRevision(This,pVal)	\
    ( (This)->lpVtbl -> get_UDFRevision(This,pVal) ) 

#define IFileSystemImage_get_UDFRevisionsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_UDFRevisionsSupported(This,pVal) ) 

#define IFileSystemImage_ChooseImageDefaults(This,discRecorder)	\
    ( (This)->lpVtbl -> ChooseImageDefaults(This,discRecorder) ) 

#define IFileSystemImage_ChooseImageDefaultsForMediaType(This,value)	\
    ( (This)->lpVtbl -> ChooseImageDefaultsForMediaType(This,value) ) 

#define IFileSystemImage_put_ISO9660InterchangeLevel(This,newVal)	\
    ( (This)->lpVtbl -> put_ISO9660InterchangeLevel(This,newVal) ) 

#define IFileSystemImage_get_ISO9660InterchangeLevel(This,pVal)	\
    ( (This)->lpVtbl -> get_ISO9660InterchangeLevel(This,pVal) ) 

#define IFileSystemImage_get_ISO9660InterchangeLevelsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_ISO9660InterchangeLevelsSupported(This,pVal) ) 

#define IFileSystemImage_CreateResultImage(This,resultStream)	\
    ( (This)->lpVtbl -> CreateResultImage(This,resultStream) ) 

#define IFileSystemImage_Exists(This,fullPath,itemType)	\
    ( (This)->lpVtbl -> Exists(This,fullPath,itemType) ) 

#define IFileSystemImage_CalculateDiscIdentifier(This,discIdentifier)	\
    ( (This)->lpVtbl -> CalculateDiscIdentifier(This,discIdentifier) ) 

#define IFileSystemImage_IdentifyFileSystemsOnDisc(This,discRecorder,fileSystems)	\
    ( (This)->lpVtbl -> IdentifyFileSystemsOnDisc(This,discRecorder,fileSystems) ) 

#define IFileSystemImage_GetDefaultFileSystemForImport(This,fileSystems,importDefault)	\
    ( (This)->lpVtbl -> GetDefaultFileSystemForImport(This,fileSystems,importDefault) ) 

#define IFileSystemImage_ImportFileSystem(This,importedFileSystem)	\
    ( (This)->lpVtbl -> ImportFileSystem(This,importedFileSystem) ) 

#define IFileSystemImage_ImportSpecificFileSystem(This,fileSystemToUse)	\
    ( (This)->lpVtbl -> ImportSpecificFileSystem(This,fileSystemToUse) ) 

#define IFileSystemImage_RollbackToChangePoint(This,changePoint)	\
    ( (This)->lpVtbl -> RollbackToChangePoint(This,changePoint) ) 

#define IFileSystemImage_LockInChangePoint(This)	\
    ( (This)->lpVtbl -> LockInChangePoint(This) ) 

#define IFileSystemImage_CreateDirectoryItem(This,name,newItem)	\
    ( (This)->lpVtbl -> CreateDirectoryItem(This,name,newItem) ) 

#define IFileSystemImage_CreateFileItem(This,name,newItem)	\
    ( (This)->lpVtbl -> CreateFileItem(This,name,newItem) ) 

#define IFileSystemImage_get_VolumeNameUDF(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameUDF(This,pVal) ) 

#define IFileSystemImage_get_VolumeNameJoliet(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameJoliet(This,pVal) ) 

#define IFileSystemImage_get_VolumeNameISO9660(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameISO9660(This,pVal) ) 

#define IFileSystemImage_get_StageFiles(This,pVal)	\
    ( (This)->lpVtbl -> get_StageFiles(This,pVal) ) 

#define IFileSystemImage_put_StageFiles(This,newVal)	\
    ( (This)->lpVtbl -> put_StageFiles(This,newVal) ) 

#define IFileSystemImage_get_MultisessionInterfaces(This,pVal)	\
    ( (This)->lpVtbl -> get_MultisessionInterfaces(This,pVal) ) 

#define IFileSystemImage_put_MultisessionInterfaces(This,newVal)	\
    ( (This)->lpVtbl -> put_MultisessionInterfaces(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSystemImage_INTERFACE_DEFINED__ */


#ifndef __IFileSystemImage2_INTERFACE_DEFINED__
#define __IFileSystemImage2_INTERFACE_DEFINED__

/* interface IFileSystemImage2 */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFileSystemImage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7644B2C-1537-4767-B62F-F1387B02DDFD")
    IFileSystemImage2 : public IFileSystemImage
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BootImageOptionsArray( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BootImageOptionsArray( 
            /* [in] */ __RPC__in SAFEARRAY * newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSystemImage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileSystemImage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileSystemImage2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFileSystemImage2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFileSystemImage2 * This,
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
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_Root)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Root )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_SessionStartBlock)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionStartBlock )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_SessionStartBlock)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SessionStartBlock )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FreeMediaBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeMediaBlocks )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_FreeMediaBlocks)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FreeMediaBlocks )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, SetMaxMediaBlocksFromDevice)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetMaxMediaBlocksFromDevice )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UsedBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedBlocks )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeName )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_VolumeName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_VolumeName )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ImportedVolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportedVolumeName )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_BootImageOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BootImageOptions )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IBootOptions **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_BootImageOptions)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BootImageOptions )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in_opt IBootOptions *newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileCount )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_DirectoryCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DirectoryCount )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_WorkingDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_WorkingDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ChangePoint)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChangePoint )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_StrictFileSystemCompliance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StrictFileSystemCompliance )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_StrictFileSystemCompliance)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StrictFileSystemCompliance )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UseRestrictedCharacterSet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseRestrictedCharacterSet )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_UseRestrictedCharacterSet)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseRestrictedCharacterSet )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileSystemsToCreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemsToCreate )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_FileSystemsToCreate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileSystemsToCreate )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ FsiFileSystems newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileSystemsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemsSupported )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_UDFRevision)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UDFRevision )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UDFRevision)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UDFRevision )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UDFRevisionsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UDFRevisionsSupported )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ChooseImageDefaults)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChooseImageDefaults )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ChooseImageDefaultsForMediaType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChooseImageDefaultsForMediaType )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ IMAPI_MEDIA_PHYSICAL_TYPE value);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_ISO9660InterchangeLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ISO9660InterchangeLevel )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ISO9660InterchangeLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISO9660InterchangeLevel )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ISO9660InterchangeLevelsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISO9660InterchangeLevelsSupported )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateResultImage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateResultImage )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFileSystemImageResult **resultStream);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, Exists)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Exists )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in BSTR fullPath,
            /* [retval][ref][out] */ __RPC__out FsiItemType *itemType);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CalculateDiscIdentifier)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CalculateDiscIdentifier )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *discIdentifier);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, IdentifyFileSystemsOnDisc)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IdentifyFileSystemsOnDisc )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *fileSystems);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, GetDefaultFileSystemForImport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultFileSystemForImport )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ FsiFileSystems fileSystems,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importDefault);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ImportFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportFileSystem )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importedFileSystem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ImportSpecificFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportSpecificFileSystem )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ FsiFileSystems fileSystemToUse);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, RollbackToChangePoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RollbackToChangePoint )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ LONG changePoint);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, LockInChangePoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LockInChangePoint )( 
            __RPC__in IFileSystemImage2 * This);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateDirectoryItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDirectoryItem )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **newItem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateFileItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateFileItem )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiFileItem **newItem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameUDF)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameUDF )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameJoliet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameJoliet )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameISO9660)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameISO9660 )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_StageFiles)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StageFiles )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_StageFiles)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StageFiles )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_MultisessionInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultisessionInterfaces )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_MultisessionInterfaces)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MultisessionInterfaces )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in SAFEARRAY * newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage2, get_BootImageOptionsArray)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BootImageOptionsArray )( 
            __RPC__in IFileSystemImage2 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage2, put_BootImageOptionsArray)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BootImageOptionsArray )( 
            __RPC__in IFileSystemImage2 * This,
            /* [in] */ __RPC__in SAFEARRAY * newVal);
        
        END_INTERFACE
    } IFileSystemImage2Vtbl;

    interface IFileSystemImage2
    {
        CONST_VTBL struct IFileSystemImage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSystemImage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSystemImage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSystemImage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSystemImage2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFileSystemImage2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFileSystemImage2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFileSystemImage2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFileSystemImage2_get_Root(This,pVal)	\
    ( (This)->lpVtbl -> get_Root(This,pVal) ) 

#define IFileSystemImage2_get_SessionStartBlock(This,pVal)	\
    ( (This)->lpVtbl -> get_SessionStartBlock(This,pVal) ) 

#define IFileSystemImage2_put_SessionStartBlock(This,newVal)	\
    ( (This)->lpVtbl -> put_SessionStartBlock(This,newVal) ) 

#define IFileSystemImage2_get_FreeMediaBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_FreeMediaBlocks(This,pVal) ) 

#define IFileSystemImage2_put_FreeMediaBlocks(This,newVal)	\
    ( (This)->lpVtbl -> put_FreeMediaBlocks(This,newVal) ) 

#define IFileSystemImage2_SetMaxMediaBlocksFromDevice(This,discRecorder)	\
    ( (This)->lpVtbl -> SetMaxMediaBlocksFromDevice(This,discRecorder) ) 

#define IFileSystemImage2_get_UsedBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_UsedBlocks(This,pVal) ) 

#define IFileSystemImage2_get_VolumeName(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeName(This,pVal) ) 

#define IFileSystemImage2_put_VolumeName(This,newVal)	\
    ( (This)->lpVtbl -> put_VolumeName(This,newVal) ) 

#define IFileSystemImage2_get_ImportedVolumeName(This,pVal)	\
    ( (This)->lpVtbl -> get_ImportedVolumeName(This,pVal) ) 

#define IFileSystemImage2_get_BootImageOptions(This,pVal)	\
    ( (This)->lpVtbl -> get_BootImageOptions(This,pVal) ) 

#define IFileSystemImage2_put_BootImageOptions(This,newVal)	\
    ( (This)->lpVtbl -> put_BootImageOptions(This,newVal) ) 

#define IFileSystemImage2_get_FileCount(This,pVal)	\
    ( (This)->lpVtbl -> get_FileCount(This,pVal) ) 

#define IFileSystemImage2_get_DirectoryCount(This,pVal)	\
    ( (This)->lpVtbl -> get_DirectoryCount(This,pVal) ) 

#define IFileSystemImage2_get_WorkingDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pVal) ) 

#define IFileSystemImage2_put_WorkingDirectory(This,newVal)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,newVal) ) 

#define IFileSystemImage2_get_ChangePoint(This,pVal)	\
    ( (This)->lpVtbl -> get_ChangePoint(This,pVal) ) 

#define IFileSystemImage2_get_StrictFileSystemCompliance(This,pVal)	\
    ( (This)->lpVtbl -> get_StrictFileSystemCompliance(This,pVal) ) 

#define IFileSystemImage2_put_StrictFileSystemCompliance(This,newVal)	\
    ( (This)->lpVtbl -> put_StrictFileSystemCompliance(This,newVal) ) 

#define IFileSystemImage2_get_UseRestrictedCharacterSet(This,pVal)	\
    ( (This)->lpVtbl -> get_UseRestrictedCharacterSet(This,pVal) ) 

#define IFileSystemImage2_put_UseRestrictedCharacterSet(This,newVal)	\
    ( (This)->lpVtbl -> put_UseRestrictedCharacterSet(This,newVal) ) 

#define IFileSystemImage2_get_FileSystemsToCreate(This,pVal)	\
    ( (This)->lpVtbl -> get_FileSystemsToCreate(This,pVal) ) 

#define IFileSystemImage2_put_FileSystemsToCreate(This,newVal)	\
    ( (This)->lpVtbl -> put_FileSystemsToCreate(This,newVal) ) 

#define IFileSystemImage2_get_FileSystemsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_FileSystemsSupported(This,pVal) ) 

#define IFileSystemImage2_put_UDFRevision(This,newVal)	\
    ( (This)->lpVtbl -> put_UDFRevision(This,newVal) ) 

#define IFileSystemImage2_get_UDFRevision(This,pVal)	\
    ( (This)->lpVtbl -> get_UDFRevision(This,pVal) ) 

#define IFileSystemImage2_get_UDFRevisionsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_UDFRevisionsSupported(This,pVal) ) 

#define IFileSystemImage2_ChooseImageDefaults(This,discRecorder)	\
    ( (This)->lpVtbl -> ChooseImageDefaults(This,discRecorder) ) 

#define IFileSystemImage2_ChooseImageDefaultsForMediaType(This,value)	\
    ( (This)->lpVtbl -> ChooseImageDefaultsForMediaType(This,value) ) 

#define IFileSystemImage2_put_ISO9660InterchangeLevel(This,newVal)	\
    ( (This)->lpVtbl -> put_ISO9660InterchangeLevel(This,newVal) ) 

#define IFileSystemImage2_get_ISO9660InterchangeLevel(This,pVal)	\
    ( (This)->lpVtbl -> get_ISO9660InterchangeLevel(This,pVal) ) 

#define IFileSystemImage2_get_ISO9660InterchangeLevelsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_ISO9660InterchangeLevelsSupported(This,pVal) ) 

#define IFileSystemImage2_CreateResultImage(This,resultStream)	\
    ( (This)->lpVtbl -> CreateResultImage(This,resultStream) ) 

#define IFileSystemImage2_Exists(This,fullPath,itemType)	\
    ( (This)->lpVtbl -> Exists(This,fullPath,itemType) ) 

#define IFileSystemImage2_CalculateDiscIdentifier(This,discIdentifier)	\
    ( (This)->lpVtbl -> CalculateDiscIdentifier(This,discIdentifier) ) 

#define IFileSystemImage2_IdentifyFileSystemsOnDisc(This,discRecorder,fileSystems)	\
    ( (This)->lpVtbl -> IdentifyFileSystemsOnDisc(This,discRecorder,fileSystems) ) 

#define IFileSystemImage2_GetDefaultFileSystemForImport(This,fileSystems,importDefault)	\
    ( (This)->lpVtbl -> GetDefaultFileSystemForImport(This,fileSystems,importDefault) ) 

#define IFileSystemImage2_ImportFileSystem(This,importedFileSystem)	\
    ( (This)->lpVtbl -> ImportFileSystem(This,importedFileSystem) ) 

#define IFileSystemImage2_ImportSpecificFileSystem(This,fileSystemToUse)	\
    ( (This)->lpVtbl -> ImportSpecificFileSystem(This,fileSystemToUse) ) 

#define IFileSystemImage2_RollbackToChangePoint(This,changePoint)	\
    ( (This)->lpVtbl -> RollbackToChangePoint(This,changePoint) ) 

#define IFileSystemImage2_LockInChangePoint(This)	\
    ( (This)->lpVtbl -> LockInChangePoint(This) ) 

#define IFileSystemImage2_CreateDirectoryItem(This,name,newItem)	\
    ( (This)->lpVtbl -> CreateDirectoryItem(This,name,newItem) ) 

#define IFileSystemImage2_CreateFileItem(This,name,newItem)	\
    ( (This)->lpVtbl -> CreateFileItem(This,name,newItem) ) 

#define IFileSystemImage2_get_VolumeNameUDF(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameUDF(This,pVal) ) 

#define IFileSystemImage2_get_VolumeNameJoliet(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameJoliet(This,pVal) ) 

#define IFileSystemImage2_get_VolumeNameISO9660(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameISO9660(This,pVal) ) 

#define IFileSystemImage2_get_StageFiles(This,pVal)	\
    ( (This)->lpVtbl -> get_StageFiles(This,pVal) ) 

#define IFileSystemImage2_put_StageFiles(This,newVal)	\
    ( (This)->lpVtbl -> put_StageFiles(This,newVal) ) 

#define IFileSystemImage2_get_MultisessionInterfaces(This,pVal)	\
    ( (This)->lpVtbl -> get_MultisessionInterfaces(This,pVal) ) 

#define IFileSystemImage2_put_MultisessionInterfaces(This,newVal)	\
    ( (This)->lpVtbl -> put_MultisessionInterfaces(This,newVal) ) 


#define IFileSystemImage2_get_BootImageOptionsArray(This,pVal)	\
    ( (This)->lpVtbl -> get_BootImageOptionsArray(This,pVal) ) 

#define IFileSystemImage2_put_BootImageOptionsArray(This,newVal)	\
    ( (This)->lpVtbl -> put_BootImageOptionsArray(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSystemImage2_INTERFACE_DEFINED__ */


#ifndef __IFileSystemImage3_INTERFACE_DEFINED__
#define __IFileSystemImage3_INTERFACE_DEFINED__

/* interface IFileSystemImage3 */
/* [helpstring][uuid][oleautomation][dual][unique][object] */ 


EXTERN_C const IID IID_IFileSystemImage3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7CFF842C-7E97-4807-8304-910DD8F7C051")
    IFileSystemImage3 : public IFileSystemImage2
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CreateRedundantUdfMetadataFiles( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CreateRedundantUdfMetadataFiles( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProbeSpecificFileSystem( 
            /* [in] */ FsiFileSystems fileSystemToProbe,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *isAppendable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSystemImage3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFileSystemImage3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFileSystemImage3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFileSystemImage3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFileSystemImage3 * This,
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
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_Root)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Root )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_SessionStartBlock)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionStartBlock )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_SessionStartBlock)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SessionStartBlock )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FreeMediaBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeMediaBlocks )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_FreeMediaBlocks)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FreeMediaBlocks )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, SetMaxMediaBlocksFromDevice)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetMaxMediaBlocksFromDevice )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UsedBlocks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedBlocks )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeName )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_VolumeName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_VolumeName )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ImportedVolumeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImportedVolumeName )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_BootImageOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BootImageOptions )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IBootOptions **pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_BootImageOptions)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BootImageOptions )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in_opt IBootOptions *newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileCount )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_DirectoryCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DirectoryCount )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_WorkingDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WorkingDirectory )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_WorkingDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WorkingDirectory )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ChangePoint)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChangePoint )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_StrictFileSystemCompliance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StrictFileSystemCompliance )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_StrictFileSystemCompliance)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StrictFileSystemCompliance )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UseRestrictedCharacterSet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseRestrictedCharacterSet )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_UseRestrictedCharacterSet)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseRestrictedCharacterSet )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileSystemsToCreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemsToCreate )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_FileSystemsToCreate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileSystemsToCreate )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ FsiFileSystems newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_FileSystemsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemsSupported )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_UDFRevision)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UDFRevision )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UDFRevision)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UDFRevision )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_UDFRevisionsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UDFRevisionsSupported )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ChooseImageDefaults)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChooseImageDefaults )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ChooseImageDefaultsForMediaType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChooseImageDefaultsForMediaType )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ IMAPI_MEDIA_PHYSICAL_TYPE value);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_ISO9660InterchangeLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ISO9660InterchangeLevel )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ LONG newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ISO9660InterchangeLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISO9660InterchangeLevel )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out LONG *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_ISO9660InterchangeLevelsSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ISO9660InterchangeLevelsSupported )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateResultImage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateResultImage )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFileSystemImageResult **resultStream);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, Exists)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Exists )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in BSTR fullPath,
            /* [retval][ref][out] */ __RPC__out FsiItemType *itemType);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CalculateDiscIdentifier)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CalculateDiscIdentifier )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *discIdentifier);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, IdentifyFileSystemsOnDisc)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IdentifyFileSystemsOnDisc )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in_opt IDiscRecorder2 *discRecorder,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *fileSystems);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, GetDefaultFileSystemForImport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultFileSystemForImport )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ FsiFileSystems fileSystems,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importDefault);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ImportFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportFileSystem )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out FsiFileSystems *importedFileSystem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, ImportSpecificFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportSpecificFileSystem )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ FsiFileSystems fileSystemToUse);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, RollbackToChangePoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RollbackToChangePoint )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ LONG changePoint);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, LockInChangePoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LockInChangePoint )( 
            __RPC__in IFileSystemImage3 * This);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateDirectoryItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDirectoryItem )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiDirectoryItem **newItem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, CreateFileItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateFileItem )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][ref][out] */ __RPC__deref_out_opt IFsiFileItem **newItem);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameUDF)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameUDF )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameJoliet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameJoliet )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_VolumeNameISO9660)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeNameISO9660 )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_StageFiles)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StageFiles )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_StageFiles)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StageFiles )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, get_MultisessionInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultisessionInterfaces )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage, put_MultisessionInterfaces)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MultisessionInterfaces )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in SAFEARRAY * newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage2, get_BootImageOptionsArray)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BootImageOptionsArray )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage2, put_BootImageOptionsArray)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BootImageOptionsArray )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ __RPC__in SAFEARRAY * newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage3, get_CreateRedundantUdfMetadataFiles)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreateRedundantUdfMetadataFiles )( 
            __RPC__in IFileSystemImage3 * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage3, put_CreateRedundantUdfMetadataFiles)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CreateRedundantUdfMetadataFiles )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IFileSystemImage3, ProbeSpecificFileSystem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ProbeSpecificFileSystem )( 
            __RPC__in IFileSystemImage3 * This,
            /* [in] */ FsiFileSystems fileSystemToProbe,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *isAppendable);
        
        END_INTERFACE
    } IFileSystemImage3Vtbl;

    interface IFileSystemImage3
    {
        CONST_VTBL struct IFileSystemImage3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSystemImage3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSystemImage3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSystemImage3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSystemImage3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFileSystemImage3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFileSystemImage3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFileSystemImage3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFileSystemImage3_get_Root(This,pVal)	\
    ( (This)->lpVtbl -> get_Root(This,pVal) ) 

#define IFileSystemImage3_get_SessionStartBlock(This,pVal)	\
    ( (This)->lpVtbl -> get_SessionStartBlock(This,pVal) ) 

#define IFileSystemImage3_put_SessionStartBlock(This,newVal)	\
    ( (This)->lpVtbl -> put_SessionStartBlock(This,newVal) ) 

#define IFileSystemImage3_get_FreeMediaBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_FreeMediaBlocks(This,pVal) ) 

#define IFileSystemImage3_put_FreeMediaBlocks(This,newVal)	\
    ( (This)->lpVtbl -> put_FreeMediaBlocks(This,newVal) ) 

#define IFileSystemImage3_SetMaxMediaBlocksFromDevice(This,discRecorder)	\
    ( (This)->lpVtbl -> SetMaxMediaBlocksFromDevice(This,discRecorder) ) 

#define IFileSystemImage3_get_UsedBlocks(This,pVal)	\
    ( (This)->lpVtbl -> get_UsedBlocks(This,pVal) ) 

#define IFileSystemImage3_get_VolumeName(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeName(This,pVal) ) 

#define IFileSystemImage3_put_VolumeName(This,newVal)	\
    ( (This)->lpVtbl -> put_VolumeName(This,newVal) ) 

#define IFileSystemImage3_get_ImportedVolumeName(This,pVal)	\
    ( (This)->lpVtbl -> get_ImportedVolumeName(This,pVal) ) 

#define IFileSystemImage3_get_BootImageOptions(This,pVal)	\
    ( (This)->lpVtbl -> get_BootImageOptions(This,pVal) ) 

#define IFileSystemImage3_put_BootImageOptions(This,newVal)	\
    ( (This)->lpVtbl -> put_BootImageOptions(This,newVal) ) 

#define IFileSystemImage3_get_FileCount(This,pVal)	\
    ( (This)->lpVtbl -> get_FileCount(This,pVal) ) 

#define IFileSystemImage3_get_DirectoryCount(This,pVal)	\
    ( (This)->lpVtbl -> get_DirectoryCount(This,pVal) ) 

#define IFileSystemImage3_get_WorkingDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_WorkingDirectory(This,pVal) ) 

#define IFileSystemImage3_put_WorkingDirectory(This,newVal)	\
    ( (This)->lpVtbl -> put_WorkingDirectory(This,newVal) ) 

#define IFileSystemImage3_get_ChangePoint(This,pVal)	\
    ( (This)->lpVtbl -> get_ChangePoint(This,pVal) ) 

#define IFileSystemImage3_get_StrictFileSystemCompliance(This,pVal)	\
    ( (This)->lpVtbl -> get_StrictFileSystemCompliance(This,pVal) ) 

#define IFileSystemImage3_put_StrictFileSystemCompliance(This,newVal)	\
    ( (This)->lpVtbl -> put_StrictFileSystemCompliance(This,newVal) ) 

#define IFileSystemImage3_get_UseRestrictedCharacterSet(This,pVal)	\
    ( (This)->lpVtbl -> get_UseRestrictedCharacterSet(This,pVal) ) 

#define IFileSystemImage3_put_UseRestrictedCharacterSet(This,newVal)	\
    ( (This)->lpVtbl -> put_UseRestrictedCharacterSet(This,newVal) ) 

#define IFileSystemImage3_get_FileSystemsToCreate(This,pVal)	\
    ( (This)->lpVtbl -> get_FileSystemsToCreate(This,pVal) ) 

#define IFileSystemImage3_put_FileSystemsToCreate(This,newVal)	\
    ( (This)->lpVtbl -> put_FileSystemsToCreate(This,newVal) ) 

#define IFileSystemImage3_get_FileSystemsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_FileSystemsSupported(This,pVal) ) 

#define IFileSystemImage3_put_UDFRevision(This,newVal)	\
    ( (This)->lpVtbl -> put_UDFRevision(This,newVal) ) 

#define IFileSystemImage3_get_UDFRevision(This,pVal)	\
    ( (This)->lpVtbl -> get_UDFRevision(This,pVal) ) 

#define IFileSystemImage3_get_UDFRevisionsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_UDFRevisionsSupported(This,pVal) ) 

#define IFileSystemImage3_ChooseImageDefaults(This,discRecorder)	\
    ( (This)->lpVtbl -> ChooseImageDefaults(This,discRecorder) ) 

#define IFileSystemImage3_ChooseImageDefaultsForMediaType(This,value)	\
    ( (This)->lpVtbl -> ChooseImageDefaultsForMediaType(This,value) ) 

#define IFileSystemImage3_put_ISO9660InterchangeLevel(This,newVal)	\
    ( (This)->lpVtbl -> put_ISO9660InterchangeLevel(This,newVal) ) 

#define IFileSystemImage3_get_ISO9660InterchangeLevel(This,pVal)	\
    ( (This)->lpVtbl -> get_ISO9660InterchangeLevel(This,pVal) ) 

#define IFileSystemImage3_get_ISO9660InterchangeLevelsSupported(This,pVal)	\
    ( (This)->lpVtbl -> get_ISO9660InterchangeLevelsSupported(This,pVal) ) 

#define IFileSystemImage3_CreateResultImage(This,resultStream)	\
    ( (This)->lpVtbl -> CreateResultImage(This,resultStream) ) 

#define IFileSystemImage3_Exists(This,fullPath,itemType)	\
    ( (This)->lpVtbl -> Exists(This,fullPath,itemType) ) 

#define IFileSystemImage3_CalculateDiscIdentifier(This,discIdentifier)	\
    ( (This)->lpVtbl -> CalculateDiscIdentifier(This,discIdentifier) ) 

#define IFileSystemImage3_IdentifyFileSystemsOnDisc(This,discRecorder,fileSystems)	\
    ( (This)->lpVtbl -> IdentifyFileSystemsOnDisc(This,discRecorder,fileSystems) ) 

#define IFileSystemImage3_GetDefaultFileSystemForImport(This,fileSystems,importDefault)	\
    ( (This)->lpVtbl -> GetDefaultFileSystemForImport(This,fileSystems,importDefault) ) 

#define IFileSystemImage3_ImportFileSystem(This,importedFileSystem)	\
    ( (This)->lpVtbl -> ImportFileSystem(This,importedFileSystem) ) 

#define IFileSystemImage3_ImportSpecificFileSystem(This,fileSystemToUse)	\
    ( (This)->lpVtbl -> ImportSpecificFileSystem(This,fileSystemToUse) ) 

#define IFileSystemImage3_RollbackToChangePoint(This,changePoint)	\
    ( (This)->lpVtbl -> RollbackToChangePoint(This,changePoint) ) 

#define IFileSystemImage3_LockInChangePoint(This)	\
    ( (This)->lpVtbl -> LockInChangePoint(This) ) 

#define IFileSystemImage3_CreateDirectoryItem(This,name,newItem)	\
    ( (This)->lpVtbl -> CreateDirectoryItem(This,name,newItem) ) 

#define IFileSystemImage3_CreateFileItem(This,name,newItem)	\
    ( (This)->lpVtbl -> CreateFileItem(This,name,newItem) ) 

#define IFileSystemImage3_get_VolumeNameUDF(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameUDF(This,pVal) ) 

#define IFileSystemImage3_get_VolumeNameJoliet(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameJoliet(This,pVal) ) 

#define IFileSystemImage3_get_VolumeNameISO9660(This,pVal)	\
    ( (This)->lpVtbl -> get_VolumeNameISO9660(This,pVal) ) 

#define IFileSystemImage3_get_StageFiles(This,pVal)	\
    ( (This)->lpVtbl -> get_StageFiles(This,pVal) ) 

#define IFileSystemImage3_put_StageFiles(This,newVal)	\
    ( (This)->lpVtbl -> put_StageFiles(This,newVal) ) 

#define IFileSystemImage3_get_MultisessionInterfaces(This,pVal)	\
    ( (This)->lpVtbl -> get_MultisessionInterfaces(This,pVal) ) 

#define IFileSystemImage3_put_MultisessionInterfaces(This,newVal)	\
    ( (This)->lpVtbl -> put_MultisessionInterfaces(This,newVal) ) 


#define IFileSystemImage3_get_BootImageOptionsArray(This,pVal)	\
    ( (This)->lpVtbl -> get_BootImageOptionsArray(This,pVal) ) 

#define IFileSystemImage3_put_BootImageOptionsArray(This,newVal)	\
    ( (This)->lpVtbl -> put_BootImageOptionsArray(This,newVal) ) 


#define IFileSystemImage3_get_CreateRedundantUdfMetadataFiles(This,pVal)	\
    ( (This)->lpVtbl -> get_CreateRedundantUdfMetadataFiles(This,pVal) ) 

#define IFileSystemImage3_put_CreateRedundantUdfMetadataFiles(This,newVal)	\
    ( (This)->lpVtbl -> put_CreateRedundantUdfMetadataFiles(This,newVal) ) 

#define IFileSystemImage3_ProbeSpecificFileSystem(This,fileSystemToProbe,isAppendable)	\
    ( (This)->lpVtbl -> ProbeSpecificFileSystem(This,fileSystemToProbe,isAppendable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSystemImage3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2fs_0000_0016 */
/* [local] */ 

#define DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE 0x100


extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0016_v0_0_s_ifspec;

#ifndef __DFileSystemImageEvents_INTERFACE_DEFINED__
#define __DFileSystemImageEvents_INTERFACE_DEFINED__

/* interface DFileSystemImageEvents */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DFileSystemImageEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C941FDF-975B-59BE-A960-9A2A262853A5")
    DFileSystemImageEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Update( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in BSTR currentFile,
            /* [in] */ LONG copiedSectors,
            /* [in] */ LONG totalSectors) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DFileSystemImageEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DFileSystemImageEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DFileSystemImageEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DFileSystemImageEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DFileSystemImageEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DFileSystemImageEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DFileSystemImageEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DFileSystemImageEvents * This,
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
        
        DECLSPEC_XFGVIRT(DFileSystemImageEvents, Update)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in DFileSystemImageEvents * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ __RPC__in BSTR currentFile,
            /* [in] */ LONG copiedSectors,
            /* [in] */ LONG totalSectors);
        
        END_INTERFACE
    } DFileSystemImageEventsVtbl;

    interface DFileSystemImageEvents
    {
        CONST_VTBL struct DFileSystemImageEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DFileSystemImageEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DFileSystemImageEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DFileSystemImageEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DFileSystemImageEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DFileSystemImageEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DFileSystemImageEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DFileSystemImageEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DFileSystemImageEvents_Update(This,object,currentFile,copiedSectors,totalSectors)	\
    ( (This)->lpVtbl -> Update(This,object,currentFile,copiedSectors,totalSectors) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DFileSystemImageEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2fs_0000_0017 */
/* [local] */ 

#define DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT 0x101


extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0017_v0_0_s_ifspec;

#ifndef __DFileSystemImageImportEvents_INTERFACE_DEFINED__
#define __DFileSystemImageImportEvents_INTERFACE_DEFINED__

/* interface DFileSystemImageImportEvents */
/* [helpstring][unique][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_DFileSystemImageImportEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D25C30F9-4087-4366-9E24-E55BE286424B")
    DFileSystemImageImportEvents : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateImport( 
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ FsiFileSystems fileSystem,
            /* [in] */ __RPC__in BSTR currentItem,
            /* [in] */ LONG importedDirectoryItems,
            /* [in] */ LONG totalDirectoryItems,
            /* [in] */ LONG importedFileItems,
            /* [in] */ LONG totalFileItems) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct DFileSystemImageImportEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DFileSystemImageImportEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DFileSystemImageImportEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DFileSystemImageImportEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DFileSystemImageImportEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DFileSystemImageImportEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DFileSystemImageImportEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DFileSystemImageImportEvents * This,
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
        
        DECLSPEC_XFGVIRT(DFileSystemImageImportEvents, UpdateImport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateImport )( 
            __RPC__in DFileSystemImageImportEvents * This,
            /* [in] */ __RPC__in_opt IDispatch *object,
            /* [in] */ FsiFileSystems fileSystem,
            /* [in] */ __RPC__in BSTR currentItem,
            /* [in] */ LONG importedDirectoryItems,
            /* [in] */ LONG totalDirectoryItems,
            /* [in] */ LONG importedFileItems,
            /* [in] */ LONG totalFileItems);
        
        END_INTERFACE
    } DFileSystemImageImportEventsVtbl;

    interface DFileSystemImageImportEvents
    {
        CONST_VTBL struct DFileSystemImageImportEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DFileSystemImageImportEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DFileSystemImageImportEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DFileSystemImageImportEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DFileSystemImageImportEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DFileSystemImageImportEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DFileSystemImageImportEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DFileSystemImageImportEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define DFileSystemImageImportEvents_UpdateImport(This,object,fileSystem,currentItem,importedDirectoryItems,totalDirectoryItems,importedFileItems,totalFileItems)	\
    ( (This)->lpVtbl -> UpdateImport(This,object,fileSystem,currentItem,importedDirectoryItems,totalDirectoryItems,importedFileItems,totalFileItems) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __DFileSystemImageImportEvents_INTERFACE_DEFINED__ */


#ifndef __IIsoImageManager_INTERFACE_DEFINED__
#define __IIsoImageManager_INTERFACE_DEFINED__

/* interface IIsoImageManager */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IIsoImageManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6CA38BE5-FBBB-4800-95A1-A438865EB0D4")
    IIsoImageManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Stream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPath( 
            /* [in] */ __RPC__in BSTR Val) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetStream( 
            /* [in] */ __RPC__in_opt IStream *data) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Validate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIsoImageManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIsoImageManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIsoImageManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIsoImageManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IIsoImageManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IIsoImageManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IIsoImageManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIsoImageManager * This,
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
        
        DECLSPEC_XFGVIRT(IIsoImageManager, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IIsoImageManager * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IIsoImageManager, get_Stream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Stream )( 
            __RPC__in IIsoImageManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **data);
        
        DECLSPEC_XFGVIRT(IIsoImageManager, SetPath)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPath )( 
            __RPC__in IIsoImageManager * This,
            /* [in] */ __RPC__in BSTR Val);
        
        DECLSPEC_XFGVIRT(IIsoImageManager, SetStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetStream )( 
            __RPC__in IIsoImageManager * This,
            /* [in] */ __RPC__in_opt IStream *data);
        
        DECLSPEC_XFGVIRT(IIsoImageManager, Validate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Validate )( 
            __RPC__in IIsoImageManager * This);
        
        END_INTERFACE
    } IIsoImageManagerVtbl;

    interface IIsoImageManager
    {
        CONST_VTBL struct IIsoImageManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIsoImageManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIsoImageManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIsoImageManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIsoImageManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIsoImageManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIsoImageManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIsoImageManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIsoImageManager_get_Path(This,pVal)	\
    ( (This)->lpVtbl -> get_Path(This,pVal) ) 

#define IIsoImageManager_get_Stream(This,data)	\
    ( (This)->lpVtbl -> get_Stream(This,data) ) 

#define IIsoImageManager_SetPath(This,Val)	\
    ( (This)->lpVtbl -> SetPath(This,Val) ) 

#define IIsoImageManager_SetStream(This,data)	\
    ( (This)->lpVtbl -> SetStream(This,data) ) 

#define IIsoImageManager_Validate(This)	\
    ( (This)->lpVtbl -> Validate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIsoImageManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imapi2fs_0000_0019 */
/* [local] */ 


//
// IMAPIv2 FileSystemImaging version information for TYPELib loading
//
#define IMAPI2FS_MajorVersion          1  
#define IMAPI2FS_MinorVersion          0  
#define IMAPI2FS_FullVersion_STR    "1.0"
#define IMAPI2FS_FullVersion_WSTR  L"1.0"


extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0019_v0_0_s_ifspec;


#ifndef __IMAPI2FS_LIBRARY_DEFINED__
#define __IMAPI2FS_LIBRARY_DEFINED__

/* library IMAPI2FS */
/* [helpstring][uuid][version] */ 




EXTERN_C const IID LIBID_IMAPI2FS;

EXTERN_C const CLSID CLSID_BootOptions;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FCE-975B-59BE-A960-9A2A262853A5")
BootOptions;
#endif

EXTERN_C const CLSID CLSID_FsiStream;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FCD-975B-59BE-A960-9A2A262853A5")
FsiStream;
#endif

EXTERN_C const CLSID CLSID_FileSystemImageResult;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FCC-975B-59BE-A960-9A2A262853A5")
FileSystemImageResult;
#endif

EXTERN_C const CLSID CLSID_ProgressItem;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FCB-975B-59BE-A960-9A2A262853A5")
ProgressItem;
#endif

EXTERN_C const CLSID CLSID_EnumProgressItems;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FCA-975B-59BE-A960-9A2A262853A5")
EnumProgressItems;
#endif

EXTERN_C const CLSID CLSID_ProgressItems;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FC9-975B-59BE-A960-9A2A262853A5")
ProgressItems;
#endif

EXTERN_C const CLSID CLSID_FsiDirectoryItem;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FC8-975B-59BE-A960-9A2A262853A5")
FsiDirectoryItem;
#endif

EXTERN_C const CLSID CLSID_FsiFileItem;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FC7-975B-59BE-A960-9A2A262853A5")
FsiFileItem;
#endif

EXTERN_C const CLSID CLSID_EnumFsiItems;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FC6-975B-59BE-A960-9A2A262853A5")
EnumFsiItems;
#endif

EXTERN_C const CLSID CLSID_FsiNamedStreams;

#ifdef __cplusplus

class DECLSPEC_UUID("C6B6F8ED-6D19-44b4-B539-B159B793A32D")
FsiNamedStreams;
#endif

EXTERN_C const CLSID CLSID_MsftFileSystemImage;

#ifdef __cplusplus

class DECLSPEC_UUID("2C941FC5-975B-59BE-A960-9A2A262853A5")
MsftFileSystemImage;
#endif

EXTERN_C const CLSID CLSID_MsftIsoImageManager;

#ifdef __cplusplus

class DECLSPEC_UUID("CEEE3B62-8F56-4056-869B-EF16917E3EFC")
MsftIsoImageManager;
#endif

EXTERN_C const CLSID CLSID_BlockRange;

#ifdef __cplusplus

class DECLSPEC_UUID("B507CA27-2204-11DD-966A-001AA01BBC58")
BlockRange;
#endif

EXTERN_C const CLSID CLSID_BlockRangeList;

#ifdef __cplusplus

class DECLSPEC_UUID("B507CA28-2204-11DD-966A-001AA01BBC58")
BlockRangeList;
#endif
#endif /* __IMAPI2FS_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_imapi2fs_0000_0020 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imapi2fs_0000_0020_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* [helpstring][local] */ HRESULT STDMETHODCALLTYPE IEnumProgressItems_Next_Proxy( 
    IEnumProgressItems * This,
    /* [in] */ ULONG celt,
    /* [size_is][out] */ IProgressItem **rgelt,
    /* [out] */ ULONG *pceltFetched);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumProgressItems_Next_Stub( 
    __RPC__in IEnumProgressItems * This,
    /* [range][in] */ __RPC__in_range(1,0x7fffffff) ULONG celt,
    /* [size_is][out] */ __RPC__out_ecount_full(celt) IProgressItem **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [helpstring][local] */ HRESULT STDMETHODCALLTYPE IEnumFsiItems_Next_Proxy( 
    IEnumFsiItems * This,
    /* [in] */ ULONG celt,
    /* [size_is][out] */ IFsiItem **rgelt,
    /* [out] */ ULONG *pceltFetched);


/* [helpstring][call_as] */ HRESULT STDMETHODCALLTYPE IEnumFsiItems_Next_Stub( 
    __RPC__in IEnumFsiItems * This,
    /* [range][in] */ __RPC__in_range(0,0x7fffffff) ULONG celt,
    /* [size_is][out] */ __RPC__out_ecount_full(celt) IFsiItem **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


