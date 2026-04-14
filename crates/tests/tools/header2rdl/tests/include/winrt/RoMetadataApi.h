

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

#ifndef __rometadataapi_h__
#define __rometadataapi_h__

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

#ifndef __IMetaDataDispenser_FWD_DEFINED__
#define __IMetaDataDispenser_FWD_DEFINED__
typedef interface IMetaDataDispenser IMetaDataDispenser;

#endif 	/* __IMetaDataDispenser_FWD_DEFINED__ */


#ifndef __IMetaDataDispenserEx_FWD_DEFINED__
#define __IMetaDataDispenserEx_FWD_DEFINED__
typedef interface IMetaDataDispenserEx IMetaDataDispenserEx;

#endif 	/* __IMetaDataDispenserEx_FWD_DEFINED__ */


#ifndef __IMetaDataAssemblyImport_FWD_DEFINED__
#define __IMetaDataAssemblyImport_FWD_DEFINED__
typedef interface IMetaDataAssemblyImport IMetaDataAssemblyImport;

#endif 	/* __IMetaDataAssemblyImport_FWD_DEFINED__ */


#ifndef __IMetaDataImport_FWD_DEFINED__
#define __IMetaDataImport_FWD_DEFINED__
typedef interface IMetaDataImport IMetaDataImport;

#endif 	/* __IMetaDataImport_FWD_DEFINED__ */


#ifndef __IMetaDataImport2_FWD_DEFINED__
#define __IMetaDataImport2_FWD_DEFINED__
typedef interface IMetaDataImport2 IMetaDataImport2;

#endif 	/* __IMetaDataImport2_FWD_DEFINED__ */


#ifndef __IMetaDataTables_FWD_DEFINED__
#define __IMetaDataTables_FWD_DEFINED__
typedef interface IMetaDataTables IMetaDataTables;

#endif 	/* __IMetaDataTables_FWD_DEFINED__ */


#ifndef __IMetaDataTables2_FWD_DEFINED__
#define __IMetaDataTables2_FWD_DEFINED__
typedef interface IMetaDataTables2 IMetaDataTables2;

#endif 	/* __IMetaDataTables2_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rometadataapi_0000_0000 */
/* [local] */ 

// Declare type representations for C and IDL only
#if !defined(__cplusplus)
typedef /* [custom] */ __int3264 HCORENUM;

typedef unsigned char COR_SIGNATURE;

typedef COR_SIGNATURE *PCOR_SIGNATURE;

typedef const COR_SIGNATURE *PCCOR_SIGNATURE;

typedef const char *MDUTF8CSTR;

typedef const char *UVCP_CONSTANT;

typedef ULONG32 mdToken;

typedef mdToken mdModule;

typedef mdToken mdTypeRef;

typedef mdToken mdTypeDef;

typedef mdToken mdFieldDef;

typedef mdToken mdMethodDef;

typedef mdToken mdParamDef;

typedef mdToken mdInterfaceImpl;

typedef mdToken mdMemberRef;

typedef mdToken mdCustomAttribute;

typedef mdToken mdPermission;

typedef mdToken mdSignature;

typedef mdToken mdEvent;

typedef mdToken mdProperty;

typedef mdToken mdModuleRef;

typedef mdToken mdAssembly;

typedef mdToken mdAssemblyRef;

typedef mdToken mdFile;

typedef mdToken mdExportedType;

typedef mdToken mdManifestResource;

typedef mdToken mdTypeSpec;

typedef mdToken mdGenericParam;

typedef mdToken mdMethodSpec;

typedef mdToken mdGenericParamConstraint;

typedef mdToken mdString;

typedef struct COR_FIELD_OFFSET
    {
    mdFieldDef ridOfField;
    ULONG32 ulOffset;
    } 	COR_FIELD_OFFSET;

typedef struct OSINFO
    {
    DWORD dwOSPlatformId;
    DWORD dwOSMajorVersion;
    DWORD dwOSMinorVersion;
    } 	OSINFO;

typedef struct ASSEMBLYMETADATA
    {
    USHORT usMajorVersion;
    USHORT usMinorVersion;
    USHORT usBuildNumber;
    USHORT usRevisionNumber;
    /* [size_is][string] */ LPWSTR szLocale;
    ULONG cbLocale;
    /* [size_is] */ DWORD *rProcessor;
    ULONG ulProcessor;
    /* [size_is] */ OSINFO *rOS;
    ULONG ulOS;
    } 	ASSEMBLYMETADATA;



extern RPC_IF_HANDLE __MIDL_itf_rometadataapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rometadataapi_0000_0000_v0_0_s_ifspec;

#ifndef __IMetaDataDispenser_INTERFACE_DEFINED__
#define __IMetaDataDispenser_INTERFACE_DEFINED__

/* interface IMetaDataDispenser */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataDispenser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("809C652E-7396-11D2-9771-00A0C9B4D50C")
    IMetaDataDispenser : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DefineScope( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenScope( 
            /* [string][in] */ LPCWSTR szScope,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenScopeOnMemory( 
            /* [size_is][in] */ const BYTE *pData,
            /* [in] */ ULONG cbData,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataDispenserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataDispenser * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataDispenser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataDispenser * This);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenser, DefineScope)
        HRESULT ( STDMETHODCALLTYPE *DefineScope )( 
            IMetaDataDispenser * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenser, OpenScope)
        HRESULT ( STDMETHODCALLTYPE *OpenScope )( 
            IMetaDataDispenser * This,
            /* [string][in] */ LPCWSTR szScope,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenser, OpenScopeOnMemory)
        HRESULT ( STDMETHODCALLTYPE *OpenScopeOnMemory )( 
            IMetaDataDispenser * This,
            /* [size_is][in] */ const BYTE *pData,
            /* [in] */ ULONG cbData,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        END_INTERFACE
    } IMetaDataDispenserVtbl;

    interface IMetaDataDispenser
    {
        CONST_VTBL struct IMetaDataDispenserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataDispenser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataDispenser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataDispenser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataDispenser_DefineScope(This,rclsid,dwCreateFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> DefineScope(This,rclsid,dwCreateFlags,riid,ppIUnk) ) 

#define IMetaDataDispenser_OpenScope(This,szScope,dwOpenFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> OpenScope(This,szScope,dwOpenFlags,riid,ppIUnk) ) 

#define IMetaDataDispenser_OpenScopeOnMemory(This,pData,cbData,dwOpenFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> OpenScopeOnMemory(This,pData,cbData,dwOpenFlags,riid,ppIUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataDispenser_INTERFACE_DEFINED__ */


#ifndef __IMetaDataDispenserEx_INTERFACE_DEFINED__
#define __IMetaDataDispenserEx_INTERFACE_DEFINED__

/* interface IMetaDataDispenserEx */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataDispenserEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31BCFCE2-DAFB-11D2-9F81-00C04F79A0A3")
    IMetaDataDispenserEx : public IMetaDataDispenser
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOption( 
            /* [in] */ REFGUID optionId,
            /* [in] */ const VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOption( 
            /* [in] */ REFGUID optionId,
            /* [out] */ VARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenScopeOnITypeInfo( 
            /* [in] */ ITypeInfo *pITI,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCORSystemDirectory( 
            /* [annotation][size_is][string][out] */ 
            _Out_writes_to_opt_(cchBuffer, *pchBuffer)  LPWSTR szBuffer,
            /* [in] */ DWORD cchBuffer,
            /* [out] */ DWORD *pchBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindAssembly( 
            /* [unique][string][in] */ LPCWSTR szAppBase,
            /* [unique][string][in] */ LPCWSTR szPrivateBin,
            /* [unique][string][in] */ LPCWSTR szGlobalBin,
            /* [unique][string][in] */ LPCWSTR szAssemblyName,
            /* [length_is][size_is][string][out] */ LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindAssemblyModule( 
            /* [unique][string][in] */ LPCWSTR szAppBase,
            /* [unique][string][in] */ LPCWSTR szPrivateBin,
            /* [unique][string][in] */ LPCWSTR szGlobalBin,
            /* [string][in] */ LPCWSTR szAssemblyName,
            /* [string][in] */ LPCWSTR szModuleName,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pcName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pcName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataDispenserExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataDispenserEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataDispenserEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataDispenserEx * This);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenser, DefineScope)
        HRESULT ( STDMETHODCALLTYPE *DefineScope )( 
            IMetaDataDispenserEx * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ DWORD dwCreateFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenser, OpenScope)
        HRESULT ( STDMETHODCALLTYPE *OpenScope )( 
            IMetaDataDispenserEx * This,
            /* [string][in] */ LPCWSTR szScope,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenser, OpenScopeOnMemory)
        HRESULT ( STDMETHODCALLTYPE *OpenScopeOnMemory )( 
            IMetaDataDispenserEx * This,
            /* [size_is][in] */ const BYTE *pData,
            /* [in] */ ULONG cbData,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenserEx, SetOption)
        HRESULT ( STDMETHODCALLTYPE *SetOption )( 
            IMetaDataDispenserEx * This,
            /* [in] */ REFGUID optionId,
            /* [in] */ const VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenserEx, GetOption)
        HRESULT ( STDMETHODCALLTYPE *GetOption )( 
            IMetaDataDispenserEx * This,
            /* [in] */ REFGUID optionId,
            /* [out] */ VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenserEx, OpenScopeOnITypeInfo)
        HRESULT ( STDMETHODCALLTYPE *OpenScopeOnITypeInfo )( 
            IMetaDataDispenserEx * This,
            /* [in] */ ITypeInfo *pITI,
            /* [in] */ DWORD dwOpenFlags,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIUnk);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenserEx, GetCORSystemDirectory)
        HRESULT ( STDMETHODCALLTYPE *GetCORSystemDirectory )( 
            IMetaDataDispenserEx * This,
            /* [annotation][size_is][string][out] */ 
            _Out_writes_to_opt_(cchBuffer, *pchBuffer)  LPWSTR szBuffer,
            /* [in] */ DWORD cchBuffer,
            /* [out] */ DWORD *pchBuffer);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenserEx, FindAssembly)
        HRESULT ( STDMETHODCALLTYPE *FindAssembly )( 
            IMetaDataDispenserEx * This,
            /* [unique][string][in] */ LPCWSTR szAppBase,
            /* [unique][string][in] */ LPCWSTR szPrivateBin,
            /* [unique][string][in] */ LPCWSTR szGlobalBin,
            /* [unique][string][in] */ LPCWSTR szAssemblyName,
            /* [length_is][size_is][string][out] */ LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName);
        
        DECLSPEC_XFGVIRT(IMetaDataDispenserEx, FindAssemblyModule)
        HRESULT ( STDMETHODCALLTYPE *FindAssemblyModule )( 
            IMetaDataDispenserEx * This,
            /* [unique][string][in] */ LPCWSTR szAppBase,
            /* [unique][string][in] */ LPCWSTR szPrivateBin,
            /* [unique][string][in] */ LPCWSTR szGlobalBin,
            /* [string][in] */ LPCWSTR szAssemblyName,
            /* [string][in] */ LPCWSTR szModuleName,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pcName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pcName);
        
        END_INTERFACE
    } IMetaDataDispenserExVtbl;

    interface IMetaDataDispenserEx
    {
        CONST_VTBL struct IMetaDataDispenserExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataDispenserEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataDispenserEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataDispenserEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataDispenserEx_DefineScope(This,rclsid,dwCreateFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> DefineScope(This,rclsid,dwCreateFlags,riid,ppIUnk) ) 

#define IMetaDataDispenserEx_OpenScope(This,szScope,dwOpenFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> OpenScope(This,szScope,dwOpenFlags,riid,ppIUnk) ) 

#define IMetaDataDispenserEx_OpenScopeOnMemory(This,pData,cbData,dwOpenFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> OpenScopeOnMemory(This,pData,cbData,dwOpenFlags,riid,ppIUnk) ) 


#define IMetaDataDispenserEx_SetOption(This,optionId,pValue)	\
    ( (This)->lpVtbl -> SetOption(This,optionId,pValue) ) 

#define IMetaDataDispenserEx_GetOption(This,optionId,pValue)	\
    ( (This)->lpVtbl -> GetOption(This,optionId,pValue) ) 

#define IMetaDataDispenserEx_OpenScopeOnITypeInfo(This,pITI,dwOpenFlags,riid,ppIUnk)	\
    ( (This)->lpVtbl -> OpenScopeOnITypeInfo(This,pITI,dwOpenFlags,riid,ppIUnk) ) 

#define IMetaDataDispenserEx_GetCORSystemDirectory(This,szBuffer,cchBuffer,pchBuffer)	\
    ( (This)->lpVtbl -> GetCORSystemDirectory(This,szBuffer,cchBuffer,pchBuffer) ) 

#define IMetaDataDispenserEx_FindAssembly(This,szAppBase,szPrivateBin,szGlobalBin,szAssemblyName,szName,cchName,pchName)	\
    ( (This)->lpVtbl -> FindAssembly(This,szAppBase,szPrivateBin,szGlobalBin,szAssemblyName,szName,cchName,pchName) ) 

#define IMetaDataDispenserEx_FindAssemblyModule(This,szAppBase,szPrivateBin,szGlobalBin,szAssemblyName,szModuleName,szName,cchName,pcName)	\
    ( (This)->lpVtbl -> FindAssemblyModule(This,szAppBase,szPrivateBin,szGlobalBin,szAssemblyName,szModuleName,szName,cchName,pcName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataDispenserEx_INTERFACE_DEFINED__ */


#ifndef __IMetaDataAssemblyImport_INTERFACE_DEFINED__
#define __IMetaDataAssemblyImport_INTERFACE_DEFINED__

/* interface IMetaDataAssemblyImport */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataAssemblyImport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE62470B-E94B-424E-9B7C-2F00C9249F93")
    IMetaDataAssemblyImport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAssemblyProps( 
            /* [in] */ mdAssembly mda,
            /* [size_is][size_is][out] */ const BYTE **ppbPublicKey,
            /* [out] */ ULONG *pcbPublicKey,
            /* [out] */ ULONG *pulHashAlgId,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ ASSEMBLYMETADATA *pMetaData,
            /* [out] */ DWORD *pdwAssemblyFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAssemblyRefProps( 
            /* [in] */ mdAssemblyRef mdar,
            /* [size_is][size_is][out] */ const BYTE **ppbPublicKeyOrToken,
            /* [out] */ ULONG *pcbPublicKeyOrToken,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ ASSEMBLYMETADATA *pMetaData,
            /* [size_is][size_is][out] */ const BYTE **ppbHashValue,
            /* [out] */ ULONG *pcbHashValue,
            /* [out] */ DWORD *pdwAssemblyRefFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileProps( 
            /* [in] */ mdFile mdf,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [size_is][size_is][out] */ const BYTE **ppbHashValue,
            /* [out] */ ULONG *pcbHashValue,
            /* [out] */ DWORD *pdwFileFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExportedTypeProps( 
            /* [in] */ mdExportedType mdct,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ mdToken *ptkImplementation,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [out] */ DWORD *pdwExportedTypeFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetManifestResourceProps( 
            /* [in] */ mdManifestResource mdmr,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ mdToken *ptkImplementation,
            /* [out] */ DWORD *pdwOffset,
            /* [out] */ DWORD *pdwResourceFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumAssemblyRefs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdAssemblyRef rAssemblyRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFiles( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdFile rFiles[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumExportedTypes( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdExportedType rExportedTypes[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumManifestResources( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdManifestResource rManifestResources[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAssemblyFromScope( 
            /* [out] */ mdAssembly *ptkAssembly) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindExportedTypeByName( 
            /* [string][in] */ LPCWSTR szName,
            /* [in] */ mdToken mdtExportedType,
            /* [out] */ mdExportedType *ptkExportedType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindManifestResourceByName( 
            /* [string][in] */ LPCWSTR szName,
            /* [out] */ mdManifestResource *ptkManifestResource) = 0;
        
        virtual void STDMETHODCALLTYPE CloseEnum( 
            /* [in] */ HCORENUM hEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindAssembliesByName( 
            /* [unique][string][in] */ LPCWSTR szAppBase,
            /* [unique][string][in] */ LPCWSTR szPrivateBin,
            /* [string][in] */ LPCWSTR szAssemblyName,
            /* [length_is][size_is][out] */ IUnknown *ppIUnk[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcAssemblies) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataAssemblyImportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataAssemblyImport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataAssemblyImport * This);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, GetAssemblyProps)
        HRESULT ( STDMETHODCALLTYPE *GetAssemblyProps )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ mdAssembly mda,
            /* [size_is][size_is][out] */ const BYTE **ppbPublicKey,
            /* [out] */ ULONG *pcbPublicKey,
            /* [out] */ ULONG *pulHashAlgId,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ ASSEMBLYMETADATA *pMetaData,
            /* [out] */ DWORD *pdwAssemblyFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, GetAssemblyRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetAssemblyRefProps )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ mdAssemblyRef mdar,
            /* [size_is][size_is][out] */ const BYTE **ppbPublicKeyOrToken,
            /* [out] */ ULONG *pcbPublicKeyOrToken,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ ASSEMBLYMETADATA *pMetaData,
            /* [size_is][size_is][out] */ const BYTE **ppbHashValue,
            /* [out] */ ULONG *pcbHashValue,
            /* [out] */ DWORD *pdwAssemblyRefFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, GetFileProps)
        HRESULT ( STDMETHODCALLTYPE *GetFileProps )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ mdFile mdf,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [size_is][size_is][out] */ const BYTE **ppbHashValue,
            /* [out] */ ULONG *pcbHashValue,
            /* [out] */ DWORD *pdwFileFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, GetExportedTypeProps)
        HRESULT ( STDMETHODCALLTYPE *GetExportedTypeProps )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ mdExportedType mdct,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ mdToken *ptkImplementation,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [out] */ DWORD *pdwExportedTypeFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, GetManifestResourceProps)
        HRESULT ( STDMETHODCALLTYPE *GetManifestResourceProps )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ mdManifestResource mdmr,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ mdToken *ptkImplementation,
            /* [out] */ DWORD *pdwOffset,
            /* [out] */ DWORD *pdwResourceFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, EnumAssemblyRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumAssemblyRefs )( 
            IMetaDataAssemblyImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdAssemblyRef rAssemblyRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, EnumFiles)
        HRESULT ( STDMETHODCALLTYPE *EnumFiles )( 
            IMetaDataAssemblyImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdFile rFiles[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, EnumExportedTypes)
        HRESULT ( STDMETHODCALLTYPE *EnumExportedTypes )( 
            IMetaDataAssemblyImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdExportedType rExportedTypes[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, EnumManifestResources)
        HRESULT ( STDMETHODCALLTYPE *EnumManifestResources )( 
            IMetaDataAssemblyImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdManifestResource rManifestResources[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, GetAssemblyFromScope)
        HRESULT ( STDMETHODCALLTYPE *GetAssemblyFromScope )( 
            IMetaDataAssemblyImport * This,
            /* [out] */ mdAssembly *ptkAssembly);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, FindExportedTypeByName)
        HRESULT ( STDMETHODCALLTYPE *FindExportedTypeByName )( 
            IMetaDataAssemblyImport * This,
            /* [string][in] */ LPCWSTR szName,
            /* [in] */ mdToken mdtExportedType,
            /* [out] */ mdExportedType *ptkExportedType);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, FindManifestResourceByName)
        HRESULT ( STDMETHODCALLTYPE *FindManifestResourceByName )( 
            IMetaDataAssemblyImport * This,
            /* [string][in] */ LPCWSTR szName,
            /* [out] */ mdManifestResource *ptkManifestResource);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, CloseEnum)
        void ( STDMETHODCALLTYPE *CloseEnum )( 
            IMetaDataAssemblyImport * This,
            /* [in] */ HCORENUM hEnum);
        
        DECLSPEC_XFGVIRT(IMetaDataAssemblyImport, FindAssembliesByName)
        HRESULT ( STDMETHODCALLTYPE *FindAssembliesByName )( 
            IMetaDataAssemblyImport * This,
            /* [unique][string][in] */ LPCWSTR szAppBase,
            /* [unique][string][in] */ LPCWSTR szPrivateBin,
            /* [string][in] */ LPCWSTR szAssemblyName,
            /* [length_is][size_is][out] */ IUnknown *ppIUnk[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcAssemblies);
        
        END_INTERFACE
    } IMetaDataAssemblyImportVtbl;

    interface IMetaDataAssemblyImport
    {
        CONST_VTBL struct IMetaDataAssemblyImportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataAssemblyImport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataAssemblyImport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataAssemblyImport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataAssemblyImport_GetAssemblyProps(This,mda,ppbPublicKey,pcbPublicKey,pulHashAlgId,szName,cchName,pchName,pMetaData,pdwAssemblyFlags)	\
    ( (This)->lpVtbl -> GetAssemblyProps(This,mda,ppbPublicKey,pcbPublicKey,pulHashAlgId,szName,cchName,pchName,pMetaData,pdwAssemblyFlags) ) 

#define IMetaDataAssemblyImport_GetAssemblyRefProps(This,mdar,ppbPublicKeyOrToken,pcbPublicKeyOrToken,szName,cchName,pchName,pMetaData,ppbHashValue,pcbHashValue,pdwAssemblyRefFlags)	\
    ( (This)->lpVtbl -> GetAssemblyRefProps(This,mdar,ppbPublicKeyOrToken,pcbPublicKeyOrToken,szName,cchName,pchName,pMetaData,ppbHashValue,pcbHashValue,pdwAssemblyRefFlags) ) 

#define IMetaDataAssemblyImport_GetFileProps(This,mdf,szName,cchName,pchName,ppbHashValue,pcbHashValue,pdwFileFlags)	\
    ( (This)->lpVtbl -> GetFileProps(This,mdf,szName,cchName,pchName,ppbHashValue,pcbHashValue,pdwFileFlags) ) 

#define IMetaDataAssemblyImport_GetExportedTypeProps(This,mdct,szName,cchName,pchName,ptkImplementation,ptkTypeDef,pdwExportedTypeFlags)	\
    ( (This)->lpVtbl -> GetExportedTypeProps(This,mdct,szName,cchName,pchName,ptkImplementation,ptkTypeDef,pdwExportedTypeFlags) ) 

#define IMetaDataAssemblyImport_GetManifestResourceProps(This,mdmr,szName,cchName,pchName,ptkImplementation,pdwOffset,pdwResourceFlags)	\
    ( (This)->lpVtbl -> GetManifestResourceProps(This,mdmr,szName,cchName,pchName,ptkImplementation,pdwOffset,pdwResourceFlags) ) 

#define IMetaDataAssemblyImport_EnumAssemblyRefs(This,phEnum,rAssemblyRefs,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumAssemblyRefs(This,phEnum,rAssemblyRefs,cMax,pcTokens) ) 

#define IMetaDataAssemblyImport_EnumFiles(This,phEnum,rFiles,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumFiles(This,phEnum,rFiles,cMax,pcTokens) ) 

#define IMetaDataAssemblyImport_EnumExportedTypes(This,phEnum,rExportedTypes,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumExportedTypes(This,phEnum,rExportedTypes,cMax,pcTokens) ) 

#define IMetaDataAssemblyImport_EnumManifestResources(This,phEnum,rManifestResources,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumManifestResources(This,phEnum,rManifestResources,cMax,pcTokens) ) 

#define IMetaDataAssemblyImport_GetAssemblyFromScope(This,ptkAssembly)	\
    ( (This)->lpVtbl -> GetAssemblyFromScope(This,ptkAssembly) ) 

#define IMetaDataAssemblyImport_FindExportedTypeByName(This,szName,mdtExportedType,ptkExportedType)	\
    ( (This)->lpVtbl -> FindExportedTypeByName(This,szName,mdtExportedType,ptkExportedType) ) 

#define IMetaDataAssemblyImport_FindManifestResourceByName(This,szName,ptkManifestResource)	\
    ( (This)->lpVtbl -> FindManifestResourceByName(This,szName,ptkManifestResource) ) 

#define IMetaDataAssemblyImport_CloseEnum(This,hEnum)	\
    ( (This)->lpVtbl -> CloseEnum(This,hEnum) ) 

#define IMetaDataAssemblyImport_FindAssembliesByName(This,szAppBase,szPrivateBin,szAssemblyName,ppIUnk,cMax,pcAssemblies)	\
    ( (This)->lpVtbl -> FindAssembliesByName(This,szAppBase,szPrivateBin,szAssemblyName,ppIUnk,cMax,pcAssemblies) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataAssemblyImport_INTERFACE_DEFINED__ */


#ifndef __IMetaDataImport_INTERFACE_DEFINED__
#define __IMetaDataImport_INTERFACE_DEFINED__

/* interface IMetaDataImport */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataImport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7DAC8207-D3AE-4C75-9B67-92801A497D44")
    IMetaDataImport : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE CloseEnum( 
            /* [in] */ HCORENUM hEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CountEnum( 
            /* [in] */ HCORENUM hEnum,
            /* [retval][out] */ ULONG *pulCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetEnum( 
            /* [in] */ HCORENUM hEnum,
            /* [in] */ ULONG ulPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumTypeDefs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeDef rgTypeDefs[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcTypeDefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumInterfaceImpls( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef td,
            /* [length_is][size_is][out] */ mdInterfaceImpl rImpls[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcImpls) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumTypeRefs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeRef rgTypeRefs[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcTypeRefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindTypeDefByName( 
            /* [string][in] */ LPCWSTR szTypeDef,
            /* [in] */ mdToken tkEnclosingClass,
            /* [retval][out] */ mdTypeDef *ptkTypeDef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScopeProps( 
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [unique][out] */ GUID *pmvid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetModuleFromScope( 
            /* [out] */ mdModule *ptkModule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeDefProps( 
            /* [in] */ mdTypeDef tkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchTypeDef, pchTypeDef)  LPWSTR szTypeDef,
            /* [in] */ ULONG cchTypeDef,
            /* [out] */ ULONG *pchTypeDef,
            /* [out] */ DWORD *pdwTypeDefFlags,
            /* [out] */ mdToken *ptkExtends) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInterfaceImplProps( 
            /* [in] */ mdInterfaceImpl tkInterfaceImpl,
            /* [out] */ mdTypeDef *ptkClass,
            /* [out] */ mdToken *ptkIface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeRefProps( 
            /* [in] */ mdTypeRef tkTypeRef,
            /* [out] */ mdToken *ptkResolutionScope,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResolveTypeRef( 
            /* [in] */ mdTypeRef tkTypeRef,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIScope,
            /* [retval][out] */ mdTypeDef *ptkTypeDef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMembers( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdToken rgMembers[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMembersWithName( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdToken rgMembers[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMethods( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdMethodDef rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMethodsWithName( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdMethodDef rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFields( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdFieldDef rgFields[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFieldsWithName( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdFieldDef rFields[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumParams( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [length_is][size_is][out] */ mdParamDef rParams[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMemberRefs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tkParent,
            /* [length_is][size_is][out] */ mdMemberRef rgMemberRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMethodImpls( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdToken rMethodBody[  ],
            /* [length_is][size_is][out] */ mdToken rMethodDecl[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumPermissionSets( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [in] */ DWORD dwActions,
            /* [length_is][size_is][out] */ mdPermission rPermission[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindMember( 
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdToken *pmb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindMethod( 
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdMethodDef *ptkMethodDef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindField( 
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdFieldDef *ptkFieldDef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindMemberRef( 
            /* [in] */ mdTypeRef tkTypeRef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdMemberRef *pMemberRef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodProps( 
            /* [in] */ mdMethodDef tkMethodDef,
            /* [out] */ mdTypeDef *ptkClass,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMethod, *pchMethod)  LPWSTR szMethod,
            /* [in] */ ULONG cchMethod,
            /* [out] */ ULONG *pchMethod,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMemberRefProps( 
            /* [in] */ mdMemberRef tkMemberRef,
            /* [out] */ mdToken *ptk,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMember, *pchMember)  LPWSTR szMember,
            /* [in] */ ULONG cchMember,
            /* [out] */ ULONG *pchMember,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumProperties( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypDef,
            /* [length_is][size_is][out] */ mdProperty rgProperties[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumEvents( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypDef,
            /* [length_is][size_is][out] */ mdEvent rgEvents[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventProps( 
            /* [in] */ mdEvent tkEvent,
            /* [out] */ mdTypeDef *ptkClass,
            /* [length_is][size_is][string][out] */ LPWSTR szEvent,
            /* [in] */ ULONG cchEvent,
            /* [out] */ ULONG *pchEvent,
            /* [out] */ DWORD *pdwEventFlags,
            /* [out] */ mdToken *ptkEventType,
            /* [out] */ mdMethodDef *ptkAddOn,
            /* [out] */ mdMethodDef *ptkRemoveOn,
            /* [out] */ mdMethodDef *tkkFire,
            /* [length_is][size_is][out] */ mdMethodDef rgOtherMethod[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcOtherMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMethodSemantics( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [length_is][size_is][out] */ mdToken rgEventProp[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcEventProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodSemantics( 
            /* [in] */ mdMethodDef tkMethodDef,
            /* [in] */ mdToken tkEventProp,
            /* [out] */ DWORD *pdwSemanticsFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClassLayout( 
            /* [in] */ mdTypeDef tkTypeDef,
            /* [out] */ DWORD *pdwPackSize,
            /* [length_is][size_is][out] */ COR_FIELD_OFFSET rgFieldOffset[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcFieldOffset,
            /* [out] */ ULONG *pulClassSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldMarshal( 
            /* [in] */ mdToken tk,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvNativeType,
            /* [out] */ ULONG *pcbNativeType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRVA( 
            /* [in] */ mdToken tk,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPermissionSetProps( 
            /* [in] */ mdPermission tk,
            /* [out] */ DWORD *pdwAction,
            /* [size_is][size_is][out] */ const BYTE **ppvPermission,
            /* [out] */ ULONG *pcbPermission) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSigFromToken( 
            /* [in] */ mdSignature tkSignature,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSig,
            /* [out] */ ULONG *pcbSig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetModuleRefProps( 
            /* [in] */ mdModuleRef tkModuleRef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumModuleRefs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdModuleRef rgModuleRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcModuleRefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeSpecFromToken( 
            /* [in] */ mdTypeSpec tkTypeSpec,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSig,
            /* [out] */ ULONG *pcbSig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameFromToken( 
            /* [in] */ mdToken tk,
            /* [string][out] */ MDUTF8CSTR *pszUtf8NamePtr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumUnresolvedMethods( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdToken rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserString( 
            /* [in] */ mdString tkString,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchString, *pchString)  LPWSTR szString,
            /* [in] */ ULONG cchString,
            /* [out] */ ULONG *pchString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPinvokeMap( 
            /* [in] */ mdToken tk,
            /* [out] */ DWORD *pdwMappingFlags,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchImportName, *pchImportName)  LPWSTR szImportName,
            /* [in] */ ULONG cchImportName,
            /* [out] */ ULONG *pchImportName,
            /* [out] */ mdModuleRef *ptkImportDLL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumSignatures( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdSignature rgSignatures[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcSignatures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumTypeSpecs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeSpec rgTypeSpecs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTypeSpecs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumUserStrings( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdString rgStrings[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcStrings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParamForMethodIndex( 
            /* [in] */ mdMethodDef tkMethodDef,
            /* [in] */ ULONG ulParamSeq,
            /* [out] */ mdParamDef *ptkParamDef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCustomAttributes( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [in] */ mdToken tkType,
            /* [length_is][size_is][out] */ mdCustomAttribute rgCustomAttributes[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcCustomAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomAttributeProps( 
            /* [in] */ mdCustomAttribute cv,
            /* [unique][out] */ mdToken *ptkObj,
            /* [unique][out] */ mdToken *ptkType,
            /* [unique][size_is][size_is][out] */ const BYTE **ppBlob,
            /* [unique][out] */ ULONG *pcbBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindTypeRef( 
            /* [in] */ mdToken tkResolutionScope,
            /* [string][in] */ LPCWSTR szName,
            /* [out] */ mdTypeRef *tkTypeRef) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMemberProps( 
            /* [in] */ mdToken tkMember,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMember, *pchMember)  LPWSTR szMember,
            /* [in] */ ULONG cchMember,
            /* [out] */ ULONG *pchMember,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldProps( 
            /* [in] */ mdFieldDef tkFieldDef,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchField, *pchField)  LPWSTR szField,
            /* [in] */ ULONG cchField,
            /* [out] */ ULONG *pchField,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyProps( 
            /* [in] */ mdProperty prop,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [length_is][size_is][string][out] */ LPWSTR szProperty,
            /* [in] */ ULONG cchProperty,
            /* [out] */ ULONG *pchProperty,
            /* [out] */ DWORD *pdwPropFlags,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppDefaultValue,
            /* [out] */ ULONG *pcchDefaultValue,
            /* [out] */ mdMethodDef *ptkSetter,
            /* [out] */ mdMethodDef *ptkGetter,
            /* [length_is][size_is][out] */ mdMethodDef rgOtherMethod[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcOtherMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParamProps( 
            /* [in] */ mdParamDef tkParamDef,
            /* [out] */ mdMethodDef *ptkMethodDef,
            /* [out] */ ULONG *pulSequence,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ DWORD *pdwAttr,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustomAttributeByName( 
            /* [in] */ mdToken tkObj,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][size_is][out] */ const BYTE **ppData,
            /* [out] */ ULONG *pcbData) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsValidToken( 
            /* [in] */ mdToken tk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNestedClassProps( 
            /* [in] */ mdTypeDef tdNestedClass,
            /* [out] */ mdTypeDef *ptdEnclosingClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNativeCallConvFromSig( 
            /* [size_is][in] */ const BYTE *pvSig,
            /* [in] */ ULONG cbSig,
            /* [out] */ ULONG *pCallConv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsGlobal( 
            /* [in] */ mdToken tk,
            /* [out] */ int *pbIsGlobal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataImportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataImport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataImport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataImport * This);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, CloseEnum)
        void ( STDMETHODCALLTYPE *CloseEnum )( 
            IMetaDataImport * This,
            /* [in] */ HCORENUM hEnum);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, CountEnum)
        HRESULT ( STDMETHODCALLTYPE *CountEnum )( 
            IMetaDataImport * This,
            /* [in] */ HCORENUM hEnum,
            /* [retval][out] */ ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, ResetEnum)
        HRESULT ( STDMETHODCALLTYPE *ResetEnum )( 
            IMetaDataImport * This,
            /* [in] */ HCORENUM hEnum,
            /* [in] */ ULONG ulPos);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumTypeDefs)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeDefs )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeDef rgTypeDefs[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcTypeDefs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumInterfaceImpls)
        HRESULT ( STDMETHODCALLTYPE *EnumInterfaceImpls )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef td,
            /* [length_is][size_is][out] */ mdInterfaceImpl rImpls[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcImpls);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumTypeRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeRefs )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeRef rgTypeRefs[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcTypeRefs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindTypeDefByName)
        HRESULT ( STDMETHODCALLTYPE *FindTypeDefByName )( 
            IMetaDataImport * This,
            /* [string][in] */ LPCWSTR szTypeDef,
            /* [in] */ mdToken tkEnclosingClass,
            /* [retval][out] */ mdTypeDef *ptkTypeDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetScopeProps)
        HRESULT ( STDMETHODCALLTYPE *GetScopeProps )( 
            IMetaDataImport * This,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [unique][out] */ GUID *pmvid);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetModuleFromScope)
        HRESULT ( STDMETHODCALLTYPE *GetModuleFromScope )( 
            IMetaDataImport * This,
            /* [out] */ mdModule *ptkModule);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetTypeDefProps)
        HRESULT ( STDMETHODCALLTYPE *GetTypeDefProps )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchTypeDef, pchTypeDef)  LPWSTR szTypeDef,
            /* [in] */ ULONG cchTypeDef,
            /* [out] */ ULONG *pchTypeDef,
            /* [out] */ DWORD *pdwTypeDefFlags,
            /* [out] */ mdToken *ptkExtends);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetInterfaceImplProps)
        HRESULT ( STDMETHODCALLTYPE *GetInterfaceImplProps )( 
            IMetaDataImport * This,
            /* [in] */ mdInterfaceImpl tkInterfaceImpl,
            /* [out] */ mdTypeDef *ptkClass,
            /* [out] */ mdToken *ptkIface);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetTypeRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetTypeRefProps )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeRef tkTypeRef,
            /* [out] */ mdToken *ptkResolutionScope,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, ResolveTypeRef)
        HRESULT ( STDMETHODCALLTYPE *ResolveTypeRef )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeRef tkTypeRef,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIScope,
            /* [retval][out] */ mdTypeDef *ptkTypeDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMembers)
        HRESULT ( STDMETHODCALLTYPE *EnumMembers )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdToken rgMembers[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMembersWithName)
        HRESULT ( STDMETHODCALLTYPE *EnumMembersWithName )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdToken rgMembers[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethods)
        HRESULT ( STDMETHODCALLTYPE *EnumMethods )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdMethodDef rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethodsWithName)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodsWithName )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdMethodDef rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumFields)
        HRESULT ( STDMETHODCALLTYPE *EnumFields )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdFieldDef rgFields[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumFieldsWithName)
        HRESULT ( STDMETHODCALLTYPE *EnumFieldsWithName )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdFieldDef rFields[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumParams)
        HRESULT ( STDMETHODCALLTYPE *EnumParams )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [length_is][size_is][out] */ mdParamDef rParams[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMemberRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumMemberRefs )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tkParent,
            /* [length_is][size_is][out] */ mdMemberRef rgMemberRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethodImpls)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodImpls )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdToken rMethodBody[  ],
            /* [length_is][size_is][out] */ mdToken rMethodDecl[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumPermissionSets)
        HRESULT ( STDMETHODCALLTYPE *EnumPermissionSets )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [in] */ DWORD dwActions,
            /* [length_is][size_is][out] */ mdPermission rPermission[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindMember)
        HRESULT ( STDMETHODCALLTYPE *FindMember )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdToken *pmb);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindMethod)
        HRESULT ( STDMETHODCALLTYPE *FindMethod )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdMethodDef *ptkMethodDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindField)
        HRESULT ( STDMETHODCALLTYPE *FindField )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdFieldDef *ptkFieldDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindMemberRef)
        HRESULT ( STDMETHODCALLTYPE *FindMemberRef )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeRef tkTypeRef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdMemberRef *pMemberRef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMethodProps)
        HRESULT ( STDMETHODCALLTYPE *GetMethodProps )( 
            IMetaDataImport * This,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [out] */ mdTypeDef *ptkClass,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMethod, *pchMethod)  LPWSTR szMethod,
            /* [in] */ ULONG cchMethod,
            /* [out] */ ULONG *pchMethod,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMemberRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetMemberRefProps )( 
            IMetaDataImport * This,
            /* [in] */ mdMemberRef tkMemberRef,
            /* [out] */ mdToken *ptk,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMember, *pchMember)  LPWSTR szMember,
            /* [in] */ ULONG cchMember,
            /* [out] */ ULONG *pchMember,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumProperties)
        HRESULT ( STDMETHODCALLTYPE *EnumProperties )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypDef,
            /* [length_is][size_is][out] */ mdProperty rgProperties[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcProperties);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumEvents)
        HRESULT ( STDMETHODCALLTYPE *EnumEvents )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypDef,
            /* [length_is][size_is][out] */ mdEvent rgEvents[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcEvents);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetEventProps)
        HRESULT ( STDMETHODCALLTYPE *GetEventProps )( 
            IMetaDataImport * This,
            /* [in] */ mdEvent tkEvent,
            /* [out] */ mdTypeDef *ptkClass,
            /* [length_is][size_is][string][out] */ LPWSTR szEvent,
            /* [in] */ ULONG cchEvent,
            /* [out] */ ULONG *pchEvent,
            /* [out] */ DWORD *pdwEventFlags,
            /* [out] */ mdToken *ptkEventType,
            /* [out] */ mdMethodDef *ptkAddOn,
            /* [out] */ mdMethodDef *ptkRemoveOn,
            /* [out] */ mdMethodDef *tkkFire,
            /* [length_is][size_is][out] */ mdMethodDef rgOtherMethod[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcOtherMethod);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethodSemantics)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodSemantics )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [length_is][size_is][out] */ mdToken rgEventProp[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcEventProp);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMethodSemantics)
        HRESULT ( STDMETHODCALLTYPE *GetMethodSemantics )( 
            IMetaDataImport * This,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [in] */ mdToken tkEventProp,
            /* [out] */ DWORD *pdwSemanticsFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetClassLayout)
        HRESULT ( STDMETHODCALLTYPE *GetClassLayout )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [out] */ DWORD *pdwPackSize,
            /* [length_is][size_is][out] */ COR_FIELD_OFFSET rgFieldOffset[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcFieldOffset,
            /* [out] */ ULONG *pulClassSize);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetFieldMarshal)
        HRESULT ( STDMETHODCALLTYPE *GetFieldMarshal )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tk,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvNativeType,
            /* [out] */ ULONG *pcbNativeType);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetRVA)
        HRESULT ( STDMETHODCALLTYPE *GetRVA )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tk,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetPermissionSetProps)
        HRESULT ( STDMETHODCALLTYPE *GetPermissionSetProps )( 
            IMetaDataImport * This,
            /* [in] */ mdPermission tk,
            /* [out] */ DWORD *pdwAction,
            /* [size_is][size_is][out] */ const BYTE **ppvPermission,
            /* [out] */ ULONG *pcbPermission);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetSigFromToken)
        HRESULT ( STDMETHODCALLTYPE *GetSigFromToken )( 
            IMetaDataImport * This,
            /* [in] */ mdSignature tkSignature,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSig,
            /* [out] */ ULONG *pcbSig);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetModuleRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetModuleRefProps )( 
            IMetaDataImport * This,
            /* [in] */ mdModuleRef tkModuleRef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumModuleRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumModuleRefs )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdModuleRef rgModuleRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcModuleRefs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetTypeSpecFromToken)
        HRESULT ( STDMETHODCALLTYPE *GetTypeSpecFromToken )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeSpec tkTypeSpec,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSig,
            /* [out] */ ULONG *pcbSig);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetNameFromToken)
        HRESULT ( STDMETHODCALLTYPE *GetNameFromToken )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tk,
            /* [string][out] */ MDUTF8CSTR *pszUtf8NamePtr);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumUnresolvedMethods)
        HRESULT ( STDMETHODCALLTYPE *EnumUnresolvedMethods )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdToken rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetUserString)
        HRESULT ( STDMETHODCALLTYPE *GetUserString )( 
            IMetaDataImport * This,
            /* [in] */ mdString tkString,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchString, *pchString)  LPWSTR szString,
            /* [in] */ ULONG cchString,
            /* [out] */ ULONG *pchString);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetPinvokeMap)
        HRESULT ( STDMETHODCALLTYPE *GetPinvokeMap )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tk,
            /* [out] */ DWORD *pdwMappingFlags,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchImportName, *pchImportName)  LPWSTR szImportName,
            /* [in] */ ULONG cchImportName,
            /* [out] */ ULONG *pchImportName,
            /* [out] */ mdModuleRef *ptkImportDLL);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumSignatures)
        HRESULT ( STDMETHODCALLTYPE *EnumSignatures )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdSignature rgSignatures[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcSignatures);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumTypeSpecs)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeSpecs )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeSpec rgTypeSpecs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTypeSpecs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumUserStrings)
        HRESULT ( STDMETHODCALLTYPE *EnumUserStrings )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdString rgStrings[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcStrings);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetParamForMethodIndex)
        HRESULT ( STDMETHODCALLTYPE *GetParamForMethodIndex )( 
            IMetaDataImport * This,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [in] */ ULONG ulParamSeq,
            /* [out] */ mdParamDef *ptkParamDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumCustomAttributes)
        HRESULT ( STDMETHODCALLTYPE *EnumCustomAttributes )( 
            IMetaDataImport * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [in] */ mdToken tkType,
            /* [length_is][size_is][out] */ mdCustomAttribute rgCustomAttributes[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcCustomAttributes);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetCustomAttributeProps)
        HRESULT ( STDMETHODCALLTYPE *GetCustomAttributeProps )( 
            IMetaDataImport * This,
            /* [in] */ mdCustomAttribute cv,
            /* [unique][out] */ mdToken *ptkObj,
            /* [unique][out] */ mdToken *ptkType,
            /* [unique][size_is][size_is][out] */ const BYTE **ppBlob,
            /* [unique][out] */ ULONG *pcbBlob);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindTypeRef)
        HRESULT ( STDMETHODCALLTYPE *FindTypeRef )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tkResolutionScope,
            /* [string][in] */ LPCWSTR szName,
            /* [out] */ mdTypeRef *tkTypeRef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMemberProps)
        HRESULT ( STDMETHODCALLTYPE *GetMemberProps )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tkMember,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMember, *pchMember)  LPWSTR szMember,
            /* [in] */ ULONG cchMember,
            /* [out] */ ULONG *pchMember,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetFieldProps)
        HRESULT ( STDMETHODCALLTYPE *GetFieldProps )( 
            IMetaDataImport * This,
            /* [in] */ mdFieldDef tkFieldDef,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchField, *pchField)  LPWSTR szField,
            /* [in] */ ULONG cchField,
            /* [out] */ ULONG *pchField,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetPropertyProps)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyProps )( 
            IMetaDataImport * This,
            /* [in] */ mdProperty prop,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [length_is][size_is][string][out] */ LPWSTR szProperty,
            /* [in] */ ULONG cchProperty,
            /* [out] */ ULONG *pchProperty,
            /* [out] */ DWORD *pdwPropFlags,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppDefaultValue,
            /* [out] */ ULONG *pcchDefaultValue,
            /* [out] */ mdMethodDef *ptkSetter,
            /* [out] */ mdMethodDef *ptkGetter,
            /* [length_is][size_is][out] */ mdMethodDef rgOtherMethod[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcOtherMethod);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetParamProps)
        HRESULT ( STDMETHODCALLTYPE *GetParamProps )( 
            IMetaDataImport * This,
            /* [in] */ mdParamDef tkParamDef,
            /* [out] */ mdMethodDef *ptkMethodDef,
            /* [out] */ ULONG *pulSequence,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ DWORD *pdwAttr,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetCustomAttributeByName)
        HRESULT ( STDMETHODCALLTYPE *GetCustomAttributeByName )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tkObj,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][size_is][out] */ const BYTE **ppData,
            /* [out] */ ULONG *pcbData);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, IsValidToken)
        BOOL ( STDMETHODCALLTYPE *IsValidToken )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tk);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetNestedClassProps)
        HRESULT ( STDMETHODCALLTYPE *GetNestedClassProps )( 
            IMetaDataImport * This,
            /* [in] */ mdTypeDef tdNestedClass,
            /* [out] */ mdTypeDef *ptdEnclosingClass);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetNativeCallConvFromSig)
        HRESULT ( STDMETHODCALLTYPE *GetNativeCallConvFromSig )( 
            IMetaDataImport * This,
            /* [size_is][in] */ const BYTE *pvSig,
            /* [in] */ ULONG cbSig,
            /* [out] */ ULONG *pCallConv);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, IsGlobal)
        HRESULT ( STDMETHODCALLTYPE *IsGlobal )( 
            IMetaDataImport * This,
            /* [in] */ mdToken tk,
            /* [out] */ int *pbIsGlobal);
        
        END_INTERFACE
    } IMetaDataImportVtbl;

    interface IMetaDataImport
    {
        CONST_VTBL struct IMetaDataImportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataImport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataImport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataImport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataImport_CloseEnum(This,hEnum)	\
    ( (This)->lpVtbl -> CloseEnum(This,hEnum) ) 

#define IMetaDataImport_CountEnum(This,hEnum,pulCount)	\
    ( (This)->lpVtbl -> CountEnum(This,hEnum,pulCount) ) 

#define IMetaDataImport_ResetEnum(This,hEnum,ulPos)	\
    ( (This)->lpVtbl -> ResetEnum(This,hEnum,ulPos) ) 

#define IMetaDataImport_EnumTypeDefs(This,phEnum,rgTypeDefs,cMax,pcTypeDefs)	\
    ( (This)->lpVtbl -> EnumTypeDefs(This,phEnum,rgTypeDefs,cMax,pcTypeDefs) ) 

#define IMetaDataImport_EnumInterfaceImpls(This,phEnum,td,rImpls,cMax,pcImpls)	\
    ( (This)->lpVtbl -> EnumInterfaceImpls(This,phEnum,td,rImpls,cMax,pcImpls) ) 

#define IMetaDataImport_EnumTypeRefs(This,phEnum,rgTypeRefs,cMax,pcTypeRefs)	\
    ( (This)->lpVtbl -> EnumTypeRefs(This,phEnum,rgTypeRefs,cMax,pcTypeRefs) ) 

#define IMetaDataImport_FindTypeDefByName(This,szTypeDef,tkEnclosingClass,ptkTypeDef)	\
    ( (This)->lpVtbl -> FindTypeDefByName(This,szTypeDef,tkEnclosingClass,ptkTypeDef) ) 

#define IMetaDataImport_GetScopeProps(This,szName,cchName,pchName,pmvid)	\
    ( (This)->lpVtbl -> GetScopeProps(This,szName,cchName,pchName,pmvid) ) 

#define IMetaDataImport_GetModuleFromScope(This,ptkModule)	\
    ( (This)->lpVtbl -> GetModuleFromScope(This,ptkModule) ) 

#define IMetaDataImport_GetTypeDefProps(This,tkTypeDef,szTypeDef,cchTypeDef,pchTypeDef,pdwTypeDefFlags,ptkExtends)	\
    ( (This)->lpVtbl -> GetTypeDefProps(This,tkTypeDef,szTypeDef,cchTypeDef,pchTypeDef,pdwTypeDefFlags,ptkExtends) ) 

#define IMetaDataImport_GetInterfaceImplProps(This,tkInterfaceImpl,ptkClass,ptkIface)	\
    ( (This)->lpVtbl -> GetInterfaceImplProps(This,tkInterfaceImpl,ptkClass,ptkIface) ) 

#define IMetaDataImport_GetTypeRefProps(This,tkTypeRef,ptkResolutionScope,szName,cchName,pchName)	\
    ( (This)->lpVtbl -> GetTypeRefProps(This,tkTypeRef,ptkResolutionScope,szName,cchName,pchName) ) 

#define IMetaDataImport_ResolveTypeRef(This,tkTypeRef,riid,ppIScope,ptkTypeDef)	\
    ( (This)->lpVtbl -> ResolveTypeRef(This,tkTypeRef,riid,ppIScope,ptkTypeDef) ) 

#define IMetaDataImport_EnumMembers(This,phEnum,tkTypeDef,rgMembers,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMembers(This,phEnum,tkTypeDef,rgMembers,cMax,pcTokens) ) 

#define IMetaDataImport_EnumMembersWithName(This,phEnum,tkTypeDef,szName,rgMembers,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMembersWithName(This,phEnum,tkTypeDef,szName,rgMembers,cMax,pcTokens) ) 

#define IMetaDataImport_EnumMethods(This,phEnum,tkTypeDef,rgMethods,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMethods(This,phEnum,tkTypeDef,rgMethods,cMax,pcTokens) ) 

#define IMetaDataImport_EnumMethodsWithName(This,phEnum,tkTypeDef,szName,rgMethods,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMethodsWithName(This,phEnum,tkTypeDef,szName,rgMethods,cMax,pcTokens) ) 

#define IMetaDataImport_EnumFields(This,phEnum,tkTypeDef,rgFields,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumFields(This,phEnum,tkTypeDef,rgFields,cMax,pcTokens) ) 

#define IMetaDataImport_EnumFieldsWithName(This,phEnum,tkTypeDef,szName,rFields,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumFieldsWithName(This,phEnum,tkTypeDef,szName,rFields,cMax,pcTokens) ) 

#define IMetaDataImport_EnumParams(This,phEnum,tkMethodDef,rParams,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumParams(This,phEnum,tkMethodDef,rParams,cMax,pcTokens) ) 

#define IMetaDataImport_EnumMemberRefs(This,phEnum,tkParent,rgMemberRefs,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMemberRefs(This,phEnum,tkParent,rgMemberRefs,cMax,pcTokens) ) 

#define IMetaDataImport_EnumMethodImpls(This,phEnum,tkTypeDef,rMethodBody,rMethodDecl,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMethodImpls(This,phEnum,tkTypeDef,rMethodBody,rMethodDecl,cMax,pcTokens) ) 

#define IMetaDataImport_EnumPermissionSets(This,phEnum,tk,dwActions,rPermission,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumPermissionSets(This,phEnum,tk,dwActions,rPermission,cMax,pcTokens) ) 

#define IMetaDataImport_FindMember(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,pmb)	\
    ( (This)->lpVtbl -> FindMember(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,pmb) ) 

#define IMetaDataImport_FindMethod(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkMethodDef)	\
    ( (This)->lpVtbl -> FindMethod(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkMethodDef) ) 

#define IMetaDataImport_FindField(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkFieldDef)	\
    ( (This)->lpVtbl -> FindField(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkFieldDef) ) 

#define IMetaDataImport_FindMemberRef(This,tkTypeRef,szName,pvSigBlob,cbSigBlob,pMemberRef)	\
    ( (This)->lpVtbl -> FindMemberRef(This,tkTypeRef,szName,pvSigBlob,cbSigBlob,pMemberRef) ) 

#define IMetaDataImport_GetMethodProps(This,tkMethodDef,ptkClass,szMethod,cchMethod,pchMethod,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags)	\
    ( (This)->lpVtbl -> GetMethodProps(This,tkMethodDef,ptkClass,szMethod,cchMethod,pchMethod,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags) ) 

#define IMetaDataImport_GetMemberRefProps(This,tkMemberRef,ptk,szMember,cchMember,pchMember,ppvSigBlob,pcbSigBlob)	\
    ( (This)->lpVtbl -> GetMemberRefProps(This,tkMemberRef,ptk,szMember,cchMember,pchMember,ppvSigBlob,pcbSigBlob) ) 

#define IMetaDataImport_EnumProperties(This,phEnum,tkTypDef,rgProperties,cMax,pcProperties)	\
    ( (This)->lpVtbl -> EnumProperties(This,phEnum,tkTypDef,rgProperties,cMax,pcProperties) ) 

#define IMetaDataImport_EnumEvents(This,phEnum,tkTypDef,rgEvents,cMax,pcEvents)	\
    ( (This)->lpVtbl -> EnumEvents(This,phEnum,tkTypDef,rgEvents,cMax,pcEvents) ) 

#define IMetaDataImport_GetEventProps(This,tkEvent,ptkClass,szEvent,cchEvent,pchEvent,pdwEventFlags,ptkEventType,ptkAddOn,ptkRemoveOn,tkkFire,rgOtherMethod,cMax,pcOtherMethod)	\
    ( (This)->lpVtbl -> GetEventProps(This,tkEvent,ptkClass,szEvent,cchEvent,pchEvent,pdwEventFlags,ptkEventType,ptkAddOn,ptkRemoveOn,tkkFire,rgOtherMethod,cMax,pcOtherMethod) ) 

#define IMetaDataImport_EnumMethodSemantics(This,phEnum,tkMethodDef,rgEventProp,cMax,pcEventProp)	\
    ( (This)->lpVtbl -> EnumMethodSemantics(This,phEnum,tkMethodDef,rgEventProp,cMax,pcEventProp) ) 

#define IMetaDataImport_GetMethodSemantics(This,tkMethodDef,tkEventProp,pdwSemanticsFlags)	\
    ( (This)->lpVtbl -> GetMethodSemantics(This,tkMethodDef,tkEventProp,pdwSemanticsFlags) ) 

#define IMetaDataImport_GetClassLayout(This,tkTypeDef,pdwPackSize,rgFieldOffset,cMax,pcFieldOffset,pulClassSize)	\
    ( (This)->lpVtbl -> GetClassLayout(This,tkTypeDef,pdwPackSize,rgFieldOffset,cMax,pcFieldOffset,pulClassSize) ) 

#define IMetaDataImport_GetFieldMarshal(This,tk,ppvNativeType,pcbNativeType)	\
    ( (This)->lpVtbl -> GetFieldMarshal(This,tk,ppvNativeType,pcbNativeType) ) 

#define IMetaDataImport_GetRVA(This,tk,pulCodeRVA,pdwImplFlags)	\
    ( (This)->lpVtbl -> GetRVA(This,tk,pulCodeRVA,pdwImplFlags) ) 

#define IMetaDataImport_GetPermissionSetProps(This,tk,pdwAction,ppvPermission,pcbPermission)	\
    ( (This)->lpVtbl -> GetPermissionSetProps(This,tk,pdwAction,ppvPermission,pcbPermission) ) 

#define IMetaDataImport_GetSigFromToken(This,tkSignature,ppvSig,pcbSig)	\
    ( (This)->lpVtbl -> GetSigFromToken(This,tkSignature,ppvSig,pcbSig) ) 

#define IMetaDataImport_GetModuleRefProps(This,tkModuleRef,szName,cchName,pchName)	\
    ( (This)->lpVtbl -> GetModuleRefProps(This,tkModuleRef,szName,cchName,pchName) ) 

#define IMetaDataImport_EnumModuleRefs(This,phEnum,rgModuleRefs,cMax,pcModuleRefs)	\
    ( (This)->lpVtbl -> EnumModuleRefs(This,phEnum,rgModuleRefs,cMax,pcModuleRefs) ) 

#define IMetaDataImport_GetTypeSpecFromToken(This,tkTypeSpec,ppvSig,pcbSig)	\
    ( (This)->lpVtbl -> GetTypeSpecFromToken(This,tkTypeSpec,ppvSig,pcbSig) ) 

#define IMetaDataImport_GetNameFromToken(This,tk,pszUtf8NamePtr)	\
    ( (This)->lpVtbl -> GetNameFromToken(This,tk,pszUtf8NamePtr) ) 

#define IMetaDataImport_EnumUnresolvedMethods(This,phEnum,rgMethods,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumUnresolvedMethods(This,phEnum,rgMethods,cMax,pcTokens) ) 

#define IMetaDataImport_GetUserString(This,tkString,szString,cchString,pchString)	\
    ( (This)->lpVtbl -> GetUserString(This,tkString,szString,cchString,pchString) ) 

#define IMetaDataImport_GetPinvokeMap(This,tk,pdwMappingFlags,szImportName,cchImportName,pchImportName,ptkImportDLL)	\
    ( (This)->lpVtbl -> GetPinvokeMap(This,tk,pdwMappingFlags,szImportName,cchImportName,pchImportName,ptkImportDLL) ) 

#define IMetaDataImport_EnumSignatures(This,phEnum,rgSignatures,cMax,pcSignatures)	\
    ( (This)->lpVtbl -> EnumSignatures(This,phEnum,rgSignatures,cMax,pcSignatures) ) 

#define IMetaDataImport_EnumTypeSpecs(This,phEnum,rgTypeSpecs,cMax,pcTypeSpecs)	\
    ( (This)->lpVtbl -> EnumTypeSpecs(This,phEnum,rgTypeSpecs,cMax,pcTypeSpecs) ) 

#define IMetaDataImport_EnumUserStrings(This,phEnum,rgStrings,cMax,pcStrings)	\
    ( (This)->lpVtbl -> EnumUserStrings(This,phEnum,rgStrings,cMax,pcStrings) ) 

#define IMetaDataImport_GetParamForMethodIndex(This,tkMethodDef,ulParamSeq,ptkParamDef)	\
    ( (This)->lpVtbl -> GetParamForMethodIndex(This,tkMethodDef,ulParamSeq,ptkParamDef) ) 

#define IMetaDataImport_EnumCustomAttributes(This,phEnum,tk,tkType,rgCustomAttributes,cMax,pcCustomAttributes)	\
    ( (This)->lpVtbl -> EnumCustomAttributes(This,phEnum,tk,tkType,rgCustomAttributes,cMax,pcCustomAttributes) ) 

#define IMetaDataImport_GetCustomAttributeProps(This,cv,ptkObj,ptkType,ppBlob,pcbBlob)	\
    ( (This)->lpVtbl -> GetCustomAttributeProps(This,cv,ptkObj,ptkType,ppBlob,pcbBlob) ) 

#define IMetaDataImport_FindTypeRef(This,tkResolutionScope,szName,tkTypeRef)	\
    ( (This)->lpVtbl -> FindTypeRef(This,tkResolutionScope,szName,tkTypeRef) ) 

#define IMetaDataImport_GetMemberProps(This,tkMember,ptkTypeDef,szMember,cchMember,pchMember,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags,pdwCPlusTypeFlag,ppValue,pcchValue)	\
    ( (This)->lpVtbl -> GetMemberProps(This,tkMember,ptkTypeDef,szMember,cchMember,pchMember,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags,pdwCPlusTypeFlag,ppValue,pcchValue) ) 

#define IMetaDataImport_GetFieldProps(This,tkFieldDef,ptkTypeDef,szField,cchField,pchField,pdwAttr,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppValue,pcchValue)	\
    ( (This)->lpVtbl -> GetFieldProps(This,tkFieldDef,ptkTypeDef,szField,cchField,pchField,pdwAttr,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppValue,pcchValue) ) 

#define IMetaDataImport_GetPropertyProps(This,prop,ptkTypeDef,szProperty,cchProperty,pchProperty,pdwPropFlags,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppDefaultValue,pcchDefaultValue,ptkSetter,ptkGetter,rgOtherMethod,cMax,pcOtherMethod)	\
    ( (This)->lpVtbl -> GetPropertyProps(This,prop,ptkTypeDef,szProperty,cchProperty,pchProperty,pdwPropFlags,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppDefaultValue,pcchDefaultValue,ptkSetter,ptkGetter,rgOtherMethod,cMax,pcOtherMethod) ) 

#define IMetaDataImport_GetParamProps(This,tkParamDef,ptkMethodDef,pulSequence,szName,cchName,pchName,pdwAttr,pdwCPlusTypeFlag,ppValue,pcchValue)	\
    ( (This)->lpVtbl -> GetParamProps(This,tkParamDef,ptkMethodDef,pulSequence,szName,cchName,pchName,pdwAttr,pdwCPlusTypeFlag,ppValue,pcchValue) ) 

#define IMetaDataImport_GetCustomAttributeByName(This,tkObj,szName,ppData,pcbData)	\
    ( (This)->lpVtbl -> GetCustomAttributeByName(This,tkObj,szName,ppData,pcbData) ) 

#define IMetaDataImport_IsValidToken(This,tk)	\
    ( (This)->lpVtbl -> IsValidToken(This,tk) ) 

#define IMetaDataImport_GetNestedClassProps(This,tdNestedClass,ptdEnclosingClass)	\
    ( (This)->lpVtbl -> GetNestedClassProps(This,tdNestedClass,ptdEnclosingClass) ) 

#define IMetaDataImport_GetNativeCallConvFromSig(This,pvSig,cbSig,pCallConv)	\
    ( (This)->lpVtbl -> GetNativeCallConvFromSig(This,pvSig,cbSig,pCallConv) ) 

#define IMetaDataImport_IsGlobal(This,tk,pbIsGlobal)	\
    ( (This)->lpVtbl -> IsGlobal(This,tk,pbIsGlobal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataImport_INTERFACE_DEFINED__ */


#ifndef __IMetaDataImport2_INTERFACE_DEFINED__
#define __IMetaDataImport2_INTERFACE_DEFINED__

/* interface IMetaDataImport2 */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataImport2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FCE5EFA0-8BBA-4f8E-A036-8F2022B08466")
    IMetaDataImport2 : public IMetaDataImport
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumGenericParams( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [length_is][size_is][out] */ mdGenericParam rGenericParams[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcGenericParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGenericParamProps( 
            /* [in] */ mdGenericParam gp,
            /* [out] */ ULONG *pulParamSeq,
            /* [out] */ DWORD *pdwParamFlags,
            /* [out] */ mdToken *ptOwner,
            /* [out] */ DWORD *reserved,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR wzname,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodSpecProps( 
            /* [in] */ mdMethodSpec mi,
            /* [out] */ mdToken *tkParent,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumGenericParamConstraints( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdGenericParam tk,
            /* [length_is][size_is][out] */ mdGenericParamConstraint rGenericParamConstraints[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcGenericParamConstraints) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGenericParamConstraintProps( 
            /* [in] */ mdGenericParamConstraint gpc,
            /* [out] */ mdGenericParam *ptGenericParam,
            /* [out] */ mdToken *ptkConstraintType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPEKind( 
            /* [out] */ DWORD *pdwPEKind,
            /* [out] */ DWORD *pdwMAchine) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersionString( 
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchBufSize, pccBufSize)  LPWSTR pwzBuf,
            /* [in] */ DWORD ccBufSize,
            /* [out] */ DWORD *pccBufSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMethodSpecs( 
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [length_is][size_is][out] */ mdMethodSpec rMethodSpecs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcMethodSpecs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataImport2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataImport2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataImport2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataImport2 * This);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, CloseEnum)
        void ( STDMETHODCALLTYPE *CloseEnum )( 
            IMetaDataImport2 * This,
            /* [in] */ HCORENUM hEnum);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, CountEnum)
        HRESULT ( STDMETHODCALLTYPE *CountEnum )( 
            IMetaDataImport2 * This,
            /* [in] */ HCORENUM hEnum,
            /* [retval][out] */ ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, ResetEnum)
        HRESULT ( STDMETHODCALLTYPE *ResetEnum )( 
            IMetaDataImport2 * This,
            /* [in] */ HCORENUM hEnum,
            /* [in] */ ULONG ulPos);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumTypeDefs)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeDefs )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeDef rgTypeDefs[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcTypeDefs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumInterfaceImpls)
        HRESULT ( STDMETHODCALLTYPE *EnumInterfaceImpls )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef td,
            /* [length_is][size_is][out] */ mdInterfaceImpl rImpls[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcImpls);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumTypeRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeRefs )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeRef rgTypeRefs[  ],
            /* [in] */ ULONG cMax,
            /* [retval][out] */ ULONG *pcTypeRefs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindTypeDefByName)
        HRESULT ( STDMETHODCALLTYPE *FindTypeDefByName )( 
            IMetaDataImport2 * This,
            /* [string][in] */ LPCWSTR szTypeDef,
            /* [in] */ mdToken tkEnclosingClass,
            /* [retval][out] */ mdTypeDef *ptkTypeDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetScopeProps)
        HRESULT ( STDMETHODCALLTYPE *GetScopeProps )( 
            IMetaDataImport2 * This,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [unique][out] */ GUID *pmvid);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetModuleFromScope)
        HRESULT ( STDMETHODCALLTYPE *GetModuleFromScope )( 
            IMetaDataImport2 * This,
            /* [out] */ mdModule *ptkModule);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetTypeDefProps)
        HRESULT ( STDMETHODCALLTYPE *GetTypeDefProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchTypeDef, pchTypeDef)  LPWSTR szTypeDef,
            /* [in] */ ULONG cchTypeDef,
            /* [out] */ ULONG *pchTypeDef,
            /* [out] */ DWORD *pdwTypeDefFlags,
            /* [out] */ mdToken *ptkExtends);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetInterfaceImplProps)
        HRESULT ( STDMETHODCALLTYPE *GetInterfaceImplProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdInterfaceImpl tkInterfaceImpl,
            /* [out] */ mdTypeDef *ptkClass,
            /* [out] */ mdToken *ptkIface);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetTypeRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetTypeRefProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeRef tkTypeRef,
            /* [out] */ mdToken *ptkResolutionScope,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, ResolveTypeRef)
        HRESULT ( STDMETHODCALLTYPE *ResolveTypeRef )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeRef tkTypeRef,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppIScope,
            /* [retval][out] */ mdTypeDef *ptkTypeDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMembers)
        HRESULT ( STDMETHODCALLTYPE *EnumMembers )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdToken rgMembers[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMembersWithName)
        HRESULT ( STDMETHODCALLTYPE *EnumMembersWithName )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdToken rgMembers[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethods)
        HRESULT ( STDMETHODCALLTYPE *EnumMethods )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdMethodDef rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethodsWithName)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodsWithName )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdMethodDef rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumFields)
        HRESULT ( STDMETHODCALLTYPE *EnumFields )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdFieldDef rgFields[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumFieldsWithName)
        HRESULT ( STDMETHODCALLTYPE *EnumFieldsWithName )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [length_is][size_is][out] */ mdFieldDef rFields[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumParams)
        HRESULT ( STDMETHODCALLTYPE *EnumParams )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [length_is][size_is][out] */ mdParamDef rParams[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMemberRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumMemberRefs )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tkParent,
            /* [length_is][size_is][out] */ mdMemberRef rgMemberRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethodImpls)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodImpls )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [length_is][size_is][out] */ mdToken rMethodBody[  ],
            /* [length_is][size_is][out] */ mdToken rMethodDecl[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumPermissionSets)
        HRESULT ( STDMETHODCALLTYPE *EnumPermissionSets )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [in] */ DWORD dwActions,
            /* [length_is][size_is][out] */ mdPermission rPermission[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindMember)
        HRESULT ( STDMETHODCALLTYPE *FindMember )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdToken *pmb);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindMethod)
        HRESULT ( STDMETHODCALLTYPE *FindMethod )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdMethodDef *ptkMethodDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindField)
        HRESULT ( STDMETHODCALLTYPE *FindField )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdFieldDef *ptkFieldDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindMemberRef)
        HRESULT ( STDMETHODCALLTYPE *FindMemberRef )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeRef tkTypeRef,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][in] */ PCCOR_SIGNATURE pvSigBlob,
            /* [in] */ ULONG cbSigBlob,
            /* [out] */ mdMemberRef *pMemberRef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMethodProps)
        HRESULT ( STDMETHODCALLTYPE *GetMethodProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [out] */ mdTypeDef *ptkClass,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMethod, *pchMethod)  LPWSTR szMethod,
            /* [in] */ ULONG cchMethod,
            /* [out] */ ULONG *pchMethod,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMemberRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetMemberRefProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdMemberRef tkMemberRef,
            /* [out] */ mdToken *ptk,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMember, *pchMember)  LPWSTR szMember,
            /* [in] */ ULONG cchMember,
            /* [out] */ ULONG *pchMember,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumProperties)
        HRESULT ( STDMETHODCALLTYPE *EnumProperties )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypDef,
            /* [length_is][size_is][out] */ mdProperty rgProperties[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcProperties);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumEvents)
        HRESULT ( STDMETHODCALLTYPE *EnumEvents )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdTypeDef tkTypDef,
            /* [length_is][size_is][out] */ mdEvent rgEvents[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcEvents);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetEventProps)
        HRESULT ( STDMETHODCALLTYPE *GetEventProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdEvent tkEvent,
            /* [out] */ mdTypeDef *ptkClass,
            /* [length_is][size_is][string][out] */ LPWSTR szEvent,
            /* [in] */ ULONG cchEvent,
            /* [out] */ ULONG *pchEvent,
            /* [out] */ DWORD *pdwEventFlags,
            /* [out] */ mdToken *ptkEventType,
            /* [out] */ mdMethodDef *ptkAddOn,
            /* [out] */ mdMethodDef *ptkRemoveOn,
            /* [out] */ mdMethodDef *tkkFire,
            /* [length_is][size_is][out] */ mdMethodDef rgOtherMethod[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcOtherMethod);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumMethodSemantics)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodSemantics )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [length_is][size_is][out] */ mdToken rgEventProp[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcEventProp);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMethodSemantics)
        HRESULT ( STDMETHODCALLTYPE *GetMethodSemantics )( 
            IMetaDataImport2 * This,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [in] */ mdToken tkEventProp,
            /* [out] */ DWORD *pdwSemanticsFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetClassLayout)
        HRESULT ( STDMETHODCALLTYPE *GetClassLayout )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeDef tkTypeDef,
            /* [out] */ DWORD *pdwPackSize,
            /* [length_is][size_is][out] */ COR_FIELD_OFFSET rgFieldOffset[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcFieldOffset,
            /* [out] */ ULONG *pulClassSize);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetFieldMarshal)
        HRESULT ( STDMETHODCALLTYPE *GetFieldMarshal )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tk,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvNativeType,
            /* [out] */ ULONG *pcbNativeType);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetRVA)
        HRESULT ( STDMETHODCALLTYPE *GetRVA )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tk,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetPermissionSetProps)
        HRESULT ( STDMETHODCALLTYPE *GetPermissionSetProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdPermission tk,
            /* [out] */ DWORD *pdwAction,
            /* [size_is][size_is][out] */ const BYTE **ppvPermission,
            /* [out] */ ULONG *pcbPermission);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetSigFromToken)
        HRESULT ( STDMETHODCALLTYPE *GetSigFromToken )( 
            IMetaDataImport2 * This,
            /* [in] */ mdSignature tkSignature,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSig,
            /* [out] */ ULONG *pcbSig);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetModuleRefProps)
        HRESULT ( STDMETHODCALLTYPE *GetModuleRefProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdModuleRef tkModuleRef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumModuleRefs)
        HRESULT ( STDMETHODCALLTYPE *EnumModuleRefs )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdModuleRef rgModuleRefs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcModuleRefs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetTypeSpecFromToken)
        HRESULT ( STDMETHODCALLTYPE *GetTypeSpecFromToken )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeSpec tkTypeSpec,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSig,
            /* [out] */ ULONG *pcbSig);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetNameFromToken)
        HRESULT ( STDMETHODCALLTYPE *GetNameFromToken )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tk,
            /* [string][out] */ MDUTF8CSTR *pszUtf8NamePtr);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumUnresolvedMethods)
        HRESULT ( STDMETHODCALLTYPE *EnumUnresolvedMethods )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdToken rgMethods[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTokens);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetUserString)
        HRESULT ( STDMETHODCALLTYPE *GetUserString )( 
            IMetaDataImport2 * This,
            /* [in] */ mdString tkString,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchString, *pchString)  LPWSTR szString,
            /* [in] */ ULONG cchString,
            /* [out] */ ULONG *pchString);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetPinvokeMap)
        HRESULT ( STDMETHODCALLTYPE *GetPinvokeMap )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tk,
            /* [out] */ DWORD *pdwMappingFlags,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchImportName, *pchImportName)  LPWSTR szImportName,
            /* [in] */ ULONG cchImportName,
            /* [out] */ ULONG *pchImportName,
            /* [out] */ mdModuleRef *ptkImportDLL);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumSignatures)
        HRESULT ( STDMETHODCALLTYPE *EnumSignatures )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdSignature rgSignatures[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcSignatures);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumTypeSpecs)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeSpecs )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdTypeSpec rgTypeSpecs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcTypeSpecs);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumUserStrings)
        HRESULT ( STDMETHODCALLTYPE *EnumUserStrings )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [length_is][size_is][out] */ mdString rgStrings[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcStrings);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetParamForMethodIndex)
        HRESULT ( STDMETHODCALLTYPE *GetParamForMethodIndex )( 
            IMetaDataImport2 * This,
            /* [in] */ mdMethodDef tkMethodDef,
            /* [in] */ ULONG ulParamSeq,
            /* [out] */ mdParamDef *ptkParamDef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, EnumCustomAttributes)
        HRESULT ( STDMETHODCALLTYPE *EnumCustomAttributes )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [in] */ mdToken tkType,
            /* [length_is][size_is][out] */ mdCustomAttribute rgCustomAttributes[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcCustomAttributes);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetCustomAttributeProps)
        HRESULT ( STDMETHODCALLTYPE *GetCustomAttributeProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdCustomAttribute cv,
            /* [unique][out] */ mdToken *ptkObj,
            /* [unique][out] */ mdToken *ptkType,
            /* [unique][size_is][size_is][out] */ const BYTE **ppBlob,
            /* [unique][out] */ ULONG *pcbBlob);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, FindTypeRef)
        HRESULT ( STDMETHODCALLTYPE *FindTypeRef )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tkResolutionScope,
            /* [string][in] */ LPCWSTR szName,
            /* [out] */ mdTypeRef *tkTypeRef);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetMemberProps)
        HRESULT ( STDMETHODCALLTYPE *GetMemberProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tkMember,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchMember, *pchMember)  LPWSTR szMember,
            /* [in] */ ULONG cchMember,
            /* [out] */ ULONG *pchMember,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ ULONG *pulCodeRVA,
            /* [out] */ DWORD *pdwImplFlags,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetFieldProps)
        HRESULT ( STDMETHODCALLTYPE *GetFieldProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdFieldDef tkFieldDef,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchField, *pchField)  LPWSTR szField,
            /* [in] */ ULONG cchField,
            /* [out] */ ULONG *pchField,
            /* [out] */ DWORD *pdwAttr,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetPropertyProps)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdProperty prop,
            /* [out] */ mdTypeDef *ptkTypeDef,
            /* [length_is][size_is][string][out] */ LPWSTR szProperty,
            /* [in] */ ULONG cchProperty,
            /* [out] */ ULONG *pchProperty,
            /* [out] */ DWORD *pdwPropFlags,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppDefaultValue,
            /* [out] */ ULONG *pcchDefaultValue,
            /* [out] */ mdMethodDef *ptkSetter,
            /* [out] */ mdMethodDef *ptkGetter,
            /* [length_is][size_is][out] */ mdMethodDef rgOtherMethod[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcOtherMethod);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetParamProps)
        HRESULT ( STDMETHODCALLTYPE *GetParamProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdParamDef tkParamDef,
            /* [out] */ mdMethodDef *ptkMethodDef,
            /* [out] */ ULONG *pulSequence,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR szName,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName,
            /* [out] */ DWORD *pdwAttr,
            /* [out] */ DWORD *pdwCPlusTypeFlag,
            /* [size_is][size_is][out] */ UVCP_CONSTANT *ppValue,
            /* [out] */ ULONG *pcchValue);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetCustomAttributeByName)
        HRESULT ( STDMETHODCALLTYPE *GetCustomAttributeByName )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tkObj,
            /* [string][in] */ LPCWSTR szName,
            /* [size_is][size_is][out] */ const BYTE **ppData,
            /* [out] */ ULONG *pcbData);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, IsValidToken)
        BOOL ( STDMETHODCALLTYPE *IsValidToken )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tk);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetNestedClassProps)
        HRESULT ( STDMETHODCALLTYPE *GetNestedClassProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdTypeDef tdNestedClass,
            /* [out] */ mdTypeDef *ptdEnclosingClass);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, GetNativeCallConvFromSig)
        HRESULT ( STDMETHODCALLTYPE *GetNativeCallConvFromSig )( 
            IMetaDataImport2 * This,
            /* [size_is][in] */ const BYTE *pvSig,
            /* [in] */ ULONG cbSig,
            /* [out] */ ULONG *pCallConv);
        
        DECLSPEC_XFGVIRT(IMetaDataImport, IsGlobal)
        HRESULT ( STDMETHODCALLTYPE *IsGlobal )( 
            IMetaDataImport2 * This,
            /* [in] */ mdToken tk,
            /* [out] */ int *pbIsGlobal);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, EnumGenericParams)
        HRESULT ( STDMETHODCALLTYPE *EnumGenericParams )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [length_is][size_is][out] */ mdGenericParam rGenericParams[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcGenericParams);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, GetGenericParamProps)
        HRESULT ( STDMETHODCALLTYPE *GetGenericParamProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdGenericParam gp,
            /* [out] */ ULONG *pulParamSeq,
            /* [out] */ DWORD *pdwParamFlags,
            /* [out] */ mdToken *ptOwner,
            /* [out] */ DWORD *reserved,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchName, *pchName)  LPWSTR wzname,
            /* [in] */ ULONG cchName,
            /* [out] */ ULONG *pchName);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, GetMethodSpecProps)
        HRESULT ( STDMETHODCALLTYPE *GetMethodSpecProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdMethodSpec mi,
            /* [out] */ mdToken *tkParent,
            /* [size_is][size_is][out] */ PCCOR_SIGNATURE *ppvSigBlob,
            /* [out] */ ULONG *pcbSigBlob);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, EnumGenericParamConstraints)
        HRESULT ( STDMETHODCALLTYPE *EnumGenericParamConstraints )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdGenericParam tk,
            /* [length_is][size_is][out] */ mdGenericParamConstraint rGenericParamConstraints[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcGenericParamConstraints);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, GetGenericParamConstraintProps)
        HRESULT ( STDMETHODCALLTYPE *GetGenericParamConstraintProps )( 
            IMetaDataImport2 * This,
            /* [in] */ mdGenericParamConstraint gpc,
            /* [out] */ mdGenericParam *ptGenericParam,
            /* [out] */ mdToken *ptkConstraintType);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, GetPEKind)
        HRESULT ( STDMETHODCALLTYPE *GetPEKind )( 
            IMetaDataImport2 * This,
            /* [out] */ DWORD *pdwPEKind,
            /* [out] */ DWORD *pdwMAchine);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, GetVersionString)
        HRESULT ( STDMETHODCALLTYPE *GetVersionString )( 
            IMetaDataImport2 * This,
            /* [annotation][length_is][size_is][string][out] */ 
            _Out_writes_to_opt_(cchBufSize, pccBufSize)  LPWSTR pwzBuf,
            /* [in] */ DWORD ccBufSize,
            /* [out] */ DWORD *pccBufSize);
        
        DECLSPEC_XFGVIRT(IMetaDataImport2, EnumMethodSpecs)
        HRESULT ( STDMETHODCALLTYPE *EnumMethodSpecs )( 
            IMetaDataImport2 * This,
            /* [out][in] */ HCORENUM *phEnum,
            /* [in] */ mdToken tk,
            /* [length_is][size_is][out] */ mdMethodSpec rMethodSpecs[  ],
            /* [in] */ ULONG cMax,
            /* [out] */ ULONG *pcMethodSpecs);
        
        END_INTERFACE
    } IMetaDataImport2Vtbl;

    interface IMetaDataImport2
    {
        CONST_VTBL struct IMetaDataImport2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataImport2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataImport2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataImport2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataImport2_CloseEnum(This,hEnum)	\
    ( (This)->lpVtbl -> CloseEnum(This,hEnum) ) 

#define IMetaDataImport2_CountEnum(This,hEnum,pulCount)	\
    ( (This)->lpVtbl -> CountEnum(This,hEnum,pulCount) ) 

#define IMetaDataImport2_ResetEnum(This,hEnum,ulPos)	\
    ( (This)->lpVtbl -> ResetEnum(This,hEnum,ulPos) ) 

#define IMetaDataImport2_EnumTypeDefs(This,phEnum,rgTypeDefs,cMax,pcTypeDefs)	\
    ( (This)->lpVtbl -> EnumTypeDefs(This,phEnum,rgTypeDefs,cMax,pcTypeDefs) ) 

#define IMetaDataImport2_EnumInterfaceImpls(This,phEnum,td,rImpls,cMax,pcImpls)	\
    ( (This)->lpVtbl -> EnumInterfaceImpls(This,phEnum,td,rImpls,cMax,pcImpls) ) 

#define IMetaDataImport2_EnumTypeRefs(This,phEnum,rgTypeRefs,cMax,pcTypeRefs)	\
    ( (This)->lpVtbl -> EnumTypeRefs(This,phEnum,rgTypeRefs,cMax,pcTypeRefs) ) 

#define IMetaDataImport2_FindTypeDefByName(This,szTypeDef,tkEnclosingClass,ptkTypeDef)	\
    ( (This)->lpVtbl -> FindTypeDefByName(This,szTypeDef,tkEnclosingClass,ptkTypeDef) ) 

#define IMetaDataImport2_GetScopeProps(This,szName,cchName,pchName,pmvid)	\
    ( (This)->lpVtbl -> GetScopeProps(This,szName,cchName,pchName,pmvid) ) 

#define IMetaDataImport2_GetModuleFromScope(This,ptkModule)	\
    ( (This)->lpVtbl -> GetModuleFromScope(This,ptkModule) ) 

#define IMetaDataImport2_GetTypeDefProps(This,tkTypeDef,szTypeDef,cchTypeDef,pchTypeDef,pdwTypeDefFlags,ptkExtends)	\
    ( (This)->lpVtbl -> GetTypeDefProps(This,tkTypeDef,szTypeDef,cchTypeDef,pchTypeDef,pdwTypeDefFlags,ptkExtends) ) 

#define IMetaDataImport2_GetInterfaceImplProps(This,tkInterfaceImpl,ptkClass,ptkIface)	\
    ( (This)->lpVtbl -> GetInterfaceImplProps(This,tkInterfaceImpl,ptkClass,ptkIface) ) 

#define IMetaDataImport2_GetTypeRefProps(This,tkTypeRef,ptkResolutionScope,szName,cchName,pchName)	\
    ( (This)->lpVtbl -> GetTypeRefProps(This,tkTypeRef,ptkResolutionScope,szName,cchName,pchName) ) 

#define IMetaDataImport2_ResolveTypeRef(This,tkTypeRef,riid,ppIScope,ptkTypeDef)	\
    ( (This)->lpVtbl -> ResolveTypeRef(This,tkTypeRef,riid,ppIScope,ptkTypeDef) ) 

#define IMetaDataImport2_EnumMembers(This,phEnum,tkTypeDef,rgMembers,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMembers(This,phEnum,tkTypeDef,rgMembers,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumMembersWithName(This,phEnum,tkTypeDef,szName,rgMembers,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMembersWithName(This,phEnum,tkTypeDef,szName,rgMembers,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumMethods(This,phEnum,tkTypeDef,rgMethods,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMethods(This,phEnum,tkTypeDef,rgMethods,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumMethodsWithName(This,phEnum,tkTypeDef,szName,rgMethods,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMethodsWithName(This,phEnum,tkTypeDef,szName,rgMethods,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumFields(This,phEnum,tkTypeDef,rgFields,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumFields(This,phEnum,tkTypeDef,rgFields,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumFieldsWithName(This,phEnum,tkTypeDef,szName,rFields,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumFieldsWithName(This,phEnum,tkTypeDef,szName,rFields,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumParams(This,phEnum,tkMethodDef,rParams,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumParams(This,phEnum,tkMethodDef,rParams,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumMemberRefs(This,phEnum,tkParent,rgMemberRefs,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMemberRefs(This,phEnum,tkParent,rgMemberRefs,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumMethodImpls(This,phEnum,tkTypeDef,rMethodBody,rMethodDecl,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumMethodImpls(This,phEnum,tkTypeDef,rMethodBody,rMethodDecl,cMax,pcTokens) ) 

#define IMetaDataImport2_EnumPermissionSets(This,phEnum,tk,dwActions,rPermission,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumPermissionSets(This,phEnum,tk,dwActions,rPermission,cMax,pcTokens) ) 

#define IMetaDataImport2_FindMember(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,pmb)	\
    ( (This)->lpVtbl -> FindMember(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,pmb) ) 

#define IMetaDataImport2_FindMethod(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkMethodDef)	\
    ( (This)->lpVtbl -> FindMethod(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkMethodDef) ) 

#define IMetaDataImport2_FindField(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkFieldDef)	\
    ( (This)->lpVtbl -> FindField(This,tkTypeDef,szName,pvSigBlob,cbSigBlob,ptkFieldDef) ) 

#define IMetaDataImport2_FindMemberRef(This,tkTypeRef,szName,pvSigBlob,cbSigBlob,pMemberRef)	\
    ( (This)->lpVtbl -> FindMemberRef(This,tkTypeRef,szName,pvSigBlob,cbSigBlob,pMemberRef) ) 

#define IMetaDataImport2_GetMethodProps(This,tkMethodDef,ptkClass,szMethod,cchMethod,pchMethod,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags)	\
    ( (This)->lpVtbl -> GetMethodProps(This,tkMethodDef,ptkClass,szMethod,cchMethod,pchMethod,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags) ) 

#define IMetaDataImport2_GetMemberRefProps(This,tkMemberRef,ptk,szMember,cchMember,pchMember,ppvSigBlob,pcbSigBlob)	\
    ( (This)->lpVtbl -> GetMemberRefProps(This,tkMemberRef,ptk,szMember,cchMember,pchMember,ppvSigBlob,pcbSigBlob) ) 

#define IMetaDataImport2_EnumProperties(This,phEnum,tkTypDef,rgProperties,cMax,pcProperties)	\
    ( (This)->lpVtbl -> EnumProperties(This,phEnum,tkTypDef,rgProperties,cMax,pcProperties) ) 

#define IMetaDataImport2_EnumEvents(This,phEnum,tkTypDef,rgEvents,cMax,pcEvents)	\
    ( (This)->lpVtbl -> EnumEvents(This,phEnum,tkTypDef,rgEvents,cMax,pcEvents) ) 

#define IMetaDataImport2_GetEventProps(This,tkEvent,ptkClass,szEvent,cchEvent,pchEvent,pdwEventFlags,ptkEventType,ptkAddOn,ptkRemoveOn,tkkFire,rgOtherMethod,cMax,pcOtherMethod)	\
    ( (This)->lpVtbl -> GetEventProps(This,tkEvent,ptkClass,szEvent,cchEvent,pchEvent,pdwEventFlags,ptkEventType,ptkAddOn,ptkRemoveOn,tkkFire,rgOtherMethod,cMax,pcOtherMethod) ) 

#define IMetaDataImport2_EnumMethodSemantics(This,phEnum,tkMethodDef,rgEventProp,cMax,pcEventProp)	\
    ( (This)->lpVtbl -> EnumMethodSemantics(This,phEnum,tkMethodDef,rgEventProp,cMax,pcEventProp) ) 

#define IMetaDataImport2_GetMethodSemantics(This,tkMethodDef,tkEventProp,pdwSemanticsFlags)	\
    ( (This)->lpVtbl -> GetMethodSemantics(This,tkMethodDef,tkEventProp,pdwSemanticsFlags) ) 

#define IMetaDataImport2_GetClassLayout(This,tkTypeDef,pdwPackSize,rgFieldOffset,cMax,pcFieldOffset,pulClassSize)	\
    ( (This)->lpVtbl -> GetClassLayout(This,tkTypeDef,pdwPackSize,rgFieldOffset,cMax,pcFieldOffset,pulClassSize) ) 

#define IMetaDataImport2_GetFieldMarshal(This,tk,ppvNativeType,pcbNativeType)	\
    ( (This)->lpVtbl -> GetFieldMarshal(This,tk,ppvNativeType,pcbNativeType) ) 

#define IMetaDataImport2_GetRVA(This,tk,pulCodeRVA,pdwImplFlags)	\
    ( (This)->lpVtbl -> GetRVA(This,tk,pulCodeRVA,pdwImplFlags) ) 

#define IMetaDataImport2_GetPermissionSetProps(This,tk,pdwAction,ppvPermission,pcbPermission)	\
    ( (This)->lpVtbl -> GetPermissionSetProps(This,tk,pdwAction,ppvPermission,pcbPermission) ) 

#define IMetaDataImport2_GetSigFromToken(This,tkSignature,ppvSig,pcbSig)	\
    ( (This)->lpVtbl -> GetSigFromToken(This,tkSignature,ppvSig,pcbSig) ) 

#define IMetaDataImport2_GetModuleRefProps(This,tkModuleRef,szName,cchName,pchName)	\
    ( (This)->lpVtbl -> GetModuleRefProps(This,tkModuleRef,szName,cchName,pchName) ) 

#define IMetaDataImport2_EnumModuleRefs(This,phEnum,rgModuleRefs,cMax,pcModuleRefs)	\
    ( (This)->lpVtbl -> EnumModuleRefs(This,phEnum,rgModuleRefs,cMax,pcModuleRefs) ) 

#define IMetaDataImport2_GetTypeSpecFromToken(This,tkTypeSpec,ppvSig,pcbSig)	\
    ( (This)->lpVtbl -> GetTypeSpecFromToken(This,tkTypeSpec,ppvSig,pcbSig) ) 

#define IMetaDataImport2_GetNameFromToken(This,tk,pszUtf8NamePtr)	\
    ( (This)->lpVtbl -> GetNameFromToken(This,tk,pszUtf8NamePtr) ) 

#define IMetaDataImport2_EnumUnresolvedMethods(This,phEnum,rgMethods,cMax,pcTokens)	\
    ( (This)->lpVtbl -> EnumUnresolvedMethods(This,phEnum,rgMethods,cMax,pcTokens) ) 

#define IMetaDataImport2_GetUserString(This,tkString,szString,cchString,pchString)	\
    ( (This)->lpVtbl -> GetUserString(This,tkString,szString,cchString,pchString) ) 

#define IMetaDataImport2_GetPinvokeMap(This,tk,pdwMappingFlags,szImportName,cchImportName,pchImportName,ptkImportDLL)	\
    ( (This)->lpVtbl -> GetPinvokeMap(This,tk,pdwMappingFlags,szImportName,cchImportName,pchImportName,ptkImportDLL) ) 

#define IMetaDataImport2_EnumSignatures(This,phEnum,rgSignatures,cMax,pcSignatures)	\
    ( (This)->lpVtbl -> EnumSignatures(This,phEnum,rgSignatures,cMax,pcSignatures) ) 

#define IMetaDataImport2_EnumTypeSpecs(This,phEnum,rgTypeSpecs,cMax,pcTypeSpecs)	\
    ( (This)->lpVtbl -> EnumTypeSpecs(This,phEnum,rgTypeSpecs,cMax,pcTypeSpecs) ) 

#define IMetaDataImport2_EnumUserStrings(This,phEnum,rgStrings,cMax,pcStrings)	\
    ( (This)->lpVtbl -> EnumUserStrings(This,phEnum,rgStrings,cMax,pcStrings) ) 

#define IMetaDataImport2_GetParamForMethodIndex(This,tkMethodDef,ulParamSeq,ptkParamDef)	\
    ( (This)->lpVtbl -> GetParamForMethodIndex(This,tkMethodDef,ulParamSeq,ptkParamDef) ) 

#define IMetaDataImport2_EnumCustomAttributes(This,phEnum,tk,tkType,rgCustomAttributes,cMax,pcCustomAttributes)	\
    ( (This)->lpVtbl -> EnumCustomAttributes(This,phEnum,tk,tkType,rgCustomAttributes,cMax,pcCustomAttributes) ) 

#define IMetaDataImport2_GetCustomAttributeProps(This,cv,ptkObj,ptkType,ppBlob,pcbBlob)	\
    ( (This)->lpVtbl -> GetCustomAttributeProps(This,cv,ptkObj,ptkType,ppBlob,pcbBlob) ) 

#define IMetaDataImport2_FindTypeRef(This,tkResolutionScope,szName,tkTypeRef)	\
    ( (This)->lpVtbl -> FindTypeRef(This,tkResolutionScope,szName,tkTypeRef) ) 

#define IMetaDataImport2_GetMemberProps(This,tkMember,ptkTypeDef,szMember,cchMember,pchMember,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags,pdwCPlusTypeFlag,ppValue,pcchValue)	\
    ( (This)->lpVtbl -> GetMemberProps(This,tkMember,ptkTypeDef,szMember,cchMember,pchMember,pdwAttr,ppvSigBlob,pcbSigBlob,pulCodeRVA,pdwImplFlags,pdwCPlusTypeFlag,ppValue,pcchValue) ) 

#define IMetaDataImport2_GetFieldProps(This,tkFieldDef,ptkTypeDef,szField,cchField,pchField,pdwAttr,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppValue,pcchValue)	\
    ( (This)->lpVtbl -> GetFieldProps(This,tkFieldDef,ptkTypeDef,szField,cchField,pchField,pdwAttr,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppValue,pcchValue) ) 

#define IMetaDataImport2_GetPropertyProps(This,prop,ptkTypeDef,szProperty,cchProperty,pchProperty,pdwPropFlags,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppDefaultValue,pcchDefaultValue,ptkSetter,ptkGetter,rgOtherMethod,cMax,pcOtherMethod)	\
    ( (This)->lpVtbl -> GetPropertyProps(This,prop,ptkTypeDef,szProperty,cchProperty,pchProperty,pdwPropFlags,ppvSigBlob,pcbSigBlob,pdwCPlusTypeFlag,ppDefaultValue,pcchDefaultValue,ptkSetter,ptkGetter,rgOtherMethod,cMax,pcOtherMethod) ) 

#define IMetaDataImport2_GetParamProps(This,tkParamDef,ptkMethodDef,pulSequence,szName,cchName,pchName,pdwAttr,pdwCPlusTypeFlag,ppValue,pcchValue)	\
    ( (This)->lpVtbl -> GetParamProps(This,tkParamDef,ptkMethodDef,pulSequence,szName,cchName,pchName,pdwAttr,pdwCPlusTypeFlag,ppValue,pcchValue) ) 

#define IMetaDataImport2_GetCustomAttributeByName(This,tkObj,szName,ppData,pcbData)	\
    ( (This)->lpVtbl -> GetCustomAttributeByName(This,tkObj,szName,ppData,pcbData) ) 

#define IMetaDataImport2_IsValidToken(This,tk)	\
    ( (This)->lpVtbl -> IsValidToken(This,tk) ) 

#define IMetaDataImport2_GetNestedClassProps(This,tdNestedClass,ptdEnclosingClass)	\
    ( (This)->lpVtbl -> GetNestedClassProps(This,tdNestedClass,ptdEnclosingClass) ) 

#define IMetaDataImport2_GetNativeCallConvFromSig(This,pvSig,cbSig,pCallConv)	\
    ( (This)->lpVtbl -> GetNativeCallConvFromSig(This,pvSig,cbSig,pCallConv) ) 

#define IMetaDataImport2_IsGlobal(This,tk,pbIsGlobal)	\
    ( (This)->lpVtbl -> IsGlobal(This,tk,pbIsGlobal) ) 


#define IMetaDataImport2_EnumGenericParams(This,phEnum,tk,rGenericParams,cMax,pcGenericParams)	\
    ( (This)->lpVtbl -> EnumGenericParams(This,phEnum,tk,rGenericParams,cMax,pcGenericParams) ) 

#define IMetaDataImport2_GetGenericParamProps(This,gp,pulParamSeq,pdwParamFlags,ptOwner,reserved,wzname,cchName,pchName)	\
    ( (This)->lpVtbl -> GetGenericParamProps(This,gp,pulParamSeq,pdwParamFlags,ptOwner,reserved,wzname,cchName,pchName) ) 

#define IMetaDataImport2_GetMethodSpecProps(This,mi,tkParent,ppvSigBlob,pcbSigBlob)	\
    ( (This)->lpVtbl -> GetMethodSpecProps(This,mi,tkParent,ppvSigBlob,pcbSigBlob) ) 

#define IMetaDataImport2_EnumGenericParamConstraints(This,phEnum,tk,rGenericParamConstraints,cMax,pcGenericParamConstraints)	\
    ( (This)->lpVtbl -> EnumGenericParamConstraints(This,phEnum,tk,rGenericParamConstraints,cMax,pcGenericParamConstraints) ) 

#define IMetaDataImport2_GetGenericParamConstraintProps(This,gpc,ptGenericParam,ptkConstraintType)	\
    ( (This)->lpVtbl -> GetGenericParamConstraintProps(This,gpc,ptGenericParam,ptkConstraintType) ) 

#define IMetaDataImport2_GetPEKind(This,pdwPEKind,pdwMAchine)	\
    ( (This)->lpVtbl -> GetPEKind(This,pdwPEKind,pdwMAchine) ) 

#define IMetaDataImport2_GetVersionString(This,pwzBuf,ccBufSize,pccBufSize)	\
    ( (This)->lpVtbl -> GetVersionString(This,pwzBuf,ccBufSize,pccBufSize) ) 

#define IMetaDataImport2_EnumMethodSpecs(This,phEnum,tk,rMethodSpecs,cMax,pcMethodSpecs)	\
    ( (This)->lpVtbl -> EnumMethodSpecs(This,phEnum,tk,rMethodSpecs,cMax,pcMethodSpecs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataImport2_INTERFACE_DEFINED__ */


#ifndef __IMetaDataTables_INTERFACE_DEFINED__
#define __IMetaDataTables_INTERFACE_DEFINED__

/* interface IMetaDataTables */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataTables;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8F579AB-402D-4B8E-82D9-5D63B1065C68")
    IMetaDataTables : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStringHeapSize( 
            /* [out] */ ULONG *pcbStrings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBlobHeapSize( 
            /* [out] */ ULONG *pcbBlobs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuidHeapSize( 
            /* [out] */ ULONG *pcbGuids) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserStringHeapSize( 
            /* [out] */ ULONG *pcbUserStrings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumTables( 
            /* [out] */ ULONG *pcTables) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableIndex( 
            /* [in] */ ULONG token,
            /* [out] */ ULONG *pixTbl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTableInfo( 
            /* [in] */ ULONG ixTbl,
            /* [out] */ ULONG *pcbRow,
            /* [out] */ ULONG *pcRows,
            /* [out] */ ULONG *pcCols,
            /* [out] */ ULONG *piKey,
            /* [string][out] */ LPCSTR *ppName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnInfo( 
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG ixCol,
            /* [out] */ ULONG *poCol,
            /* [out] */ ULONG *pcbCol,
            /* [out] */ ULONG *pType,
            /* [string][out] */ LPCSTR *ppName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCodedTokenInfo( 
            /* [in] */ ULONG ixCdTkn,
            /* [out] */ ULONG *pcTokens,
            /* [size_is][size_is][out] */ const ULONG **ppTokens,
            /* [string][out] */ LPCSTR *ppName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRow( 
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG rid,
            /* [out] */ const BYTE *ppRow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumn( 
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG ixCol,
            /* [in] */ ULONG rid,
            /* [out] */ ULONG *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            /* [in] */ ULONG ixString,
            /* [string][out] */ LPCSTR *ppString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBlob( 
            /* [in] */ ULONG ixBlob,
            /* [out] */ ULONG *pcbData,
            /* [size_is][size_is][out] */ const BYTE **ppData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuid( 
            /* [in] */ ULONG ixGuid,
            /* [out] */ const GUID **ppGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserString( 
            /* [in] */ ULONG ixUserString,
            /* [out] */ ULONG *pcbData,
            /* [size_is][size_is][out] */ const BYTE **ppData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextString( 
            /* [in] */ ULONG ixString,
            /* [out] */ ULONG *pNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextBlob( 
            /* [in] */ ULONG ixBlob,
            /* [out] */ ULONG *pNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextGuid( 
            /* [in] */ ULONG ixGuid,
            /* [out] */ ULONG *pNext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextUserString( 
            /* [in] */ ULONG ixUserString,
            /* [out] */ ULONG *pNext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataTablesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataTables * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataTables * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataTables * This);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetStringHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetStringHeapSize )( 
            IMetaDataTables * This,
            /* [out] */ ULONG *pcbStrings);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetBlobHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobHeapSize )( 
            IMetaDataTables * This,
            /* [out] */ ULONG *pcbBlobs);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetGuidHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetGuidHeapSize )( 
            IMetaDataTables * This,
            /* [out] */ ULONG *pcbGuids);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetUserStringHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetUserStringHeapSize )( 
            IMetaDataTables * This,
            /* [out] */ ULONG *pcbUserStrings);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNumTables)
        HRESULT ( STDMETHODCALLTYPE *GetNumTables )( 
            IMetaDataTables * This,
            /* [out] */ ULONG *pcTables);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetTableIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableIndex )( 
            IMetaDataTables * This,
            /* [in] */ ULONG token,
            /* [out] */ ULONG *pixTbl);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetTableInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTableInfo )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixTbl,
            /* [out] */ ULONG *pcbRow,
            /* [out] */ ULONG *pcRows,
            /* [out] */ ULONG *pcCols,
            /* [out] */ ULONG *piKey,
            /* [string][out] */ LPCSTR *ppName);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetColumnInfo)
        HRESULT ( STDMETHODCALLTYPE *GetColumnInfo )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG ixCol,
            /* [out] */ ULONG *poCol,
            /* [out] */ ULONG *pcbCol,
            /* [out] */ ULONG *pType,
            /* [string][out] */ LPCSTR *ppName);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetCodedTokenInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCodedTokenInfo )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixCdTkn,
            /* [out] */ ULONG *pcTokens,
            /* [size_is][size_is][out] */ const ULONG **ppTokens,
            /* [string][out] */ LPCSTR *ppName);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetRow)
        HRESULT ( STDMETHODCALLTYPE *GetRow )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG rid,
            /* [out] */ const BYTE *ppRow);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetColumn)
        HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG ixCol,
            /* [in] */ ULONG rid,
            /* [out] */ ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixString,
            /* [string][out] */ LPCSTR *ppString);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixBlob,
            /* [out] */ ULONG *pcbData,
            /* [size_is][size_is][out] */ const BYTE **ppData);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetGuid)
        HRESULT ( STDMETHODCALLTYPE *GetGuid )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixGuid,
            /* [out] */ const GUID **ppGUID);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetUserString)
        HRESULT ( STDMETHODCALLTYPE *GetUserString )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixUserString,
            /* [out] */ ULONG *pcbData,
            /* [size_is][size_is][out] */ const BYTE **ppData);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextString)
        HRESULT ( STDMETHODCALLTYPE *GetNextString )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixString,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextBlob)
        HRESULT ( STDMETHODCALLTYPE *GetNextBlob )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixBlob,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextGuid)
        HRESULT ( STDMETHODCALLTYPE *GetNextGuid )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixGuid,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextUserString)
        HRESULT ( STDMETHODCALLTYPE *GetNextUserString )( 
            IMetaDataTables * This,
            /* [in] */ ULONG ixUserString,
            /* [out] */ ULONG *pNext);
        
        END_INTERFACE
    } IMetaDataTablesVtbl;

    interface IMetaDataTables
    {
        CONST_VTBL struct IMetaDataTablesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataTables_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataTables_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataTables_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataTables_GetStringHeapSize(This,pcbStrings)	\
    ( (This)->lpVtbl -> GetStringHeapSize(This,pcbStrings) ) 

#define IMetaDataTables_GetBlobHeapSize(This,pcbBlobs)	\
    ( (This)->lpVtbl -> GetBlobHeapSize(This,pcbBlobs) ) 

#define IMetaDataTables_GetGuidHeapSize(This,pcbGuids)	\
    ( (This)->lpVtbl -> GetGuidHeapSize(This,pcbGuids) ) 

#define IMetaDataTables_GetUserStringHeapSize(This,pcbUserStrings)	\
    ( (This)->lpVtbl -> GetUserStringHeapSize(This,pcbUserStrings) ) 

#define IMetaDataTables_GetNumTables(This,pcTables)	\
    ( (This)->lpVtbl -> GetNumTables(This,pcTables) ) 

#define IMetaDataTables_GetTableIndex(This,token,pixTbl)	\
    ( (This)->lpVtbl -> GetTableIndex(This,token,pixTbl) ) 

#define IMetaDataTables_GetTableInfo(This,ixTbl,pcbRow,pcRows,pcCols,piKey,ppName)	\
    ( (This)->lpVtbl -> GetTableInfo(This,ixTbl,pcbRow,pcRows,pcCols,piKey,ppName) ) 

#define IMetaDataTables_GetColumnInfo(This,ixTbl,ixCol,poCol,pcbCol,pType,ppName)	\
    ( (This)->lpVtbl -> GetColumnInfo(This,ixTbl,ixCol,poCol,pcbCol,pType,ppName) ) 

#define IMetaDataTables_GetCodedTokenInfo(This,ixCdTkn,pcTokens,ppTokens,ppName)	\
    ( (This)->lpVtbl -> GetCodedTokenInfo(This,ixCdTkn,pcTokens,ppTokens,ppName) ) 

#define IMetaDataTables_GetRow(This,ixTbl,rid,ppRow)	\
    ( (This)->lpVtbl -> GetRow(This,ixTbl,rid,ppRow) ) 

#define IMetaDataTables_GetColumn(This,ixTbl,ixCol,rid,pVal)	\
    ( (This)->lpVtbl -> GetColumn(This,ixTbl,ixCol,rid,pVal) ) 

#define IMetaDataTables_GetString(This,ixString,ppString)	\
    ( (This)->lpVtbl -> GetString(This,ixString,ppString) ) 

#define IMetaDataTables_GetBlob(This,ixBlob,pcbData,ppData)	\
    ( (This)->lpVtbl -> GetBlob(This,ixBlob,pcbData,ppData) ) 

#define IMetaDataTables_GetGuid(This,ixGuid,ppGUID)	\
    ( (This)->lpVtbl -> GetGuid(This,ixGuid,ppGUID) ) 

#define IMetaDataTables_GetUserString(This,ixUserString,pcbData,ppData)	\
    ( (This)->lpVtbl -> GetUserString(This,ixUserString,pcbData,ppData) ) 

#define IMetaDataTables_GetNextString(This,ixString,pNext)	\
    ( (This)->lpVtbl -> GetNextString(This,ixString,pNext) ) 

#define IMetaDataTables_GetNextBlob(This,ixBlob,pNext)	\
    ( (This)->lpVtbl -> GetNextBlob(This,ixBlob,pNext) ) 

#define IMetaDataTables_GetNextGuid(This,ixGuid,pNext)	\
    ( (This)->lpVtbl -> GetNextGuid(This,ixGuid,pNext) ) 

#define IMetaDataTables_GetNextUserString(This,ixUserString,pNext)	\
    ( (This)->lpVtbl -> GetNextUserString(This,ixUserString,pNext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataTables_INTERFACE_DEFINED__ */


#ifndef __IMetaDataTables2_INTERFACE_DEFINED__
#define __IMetaDataTables2_INTERFACE_DEFINED__

/* interface IMetaDataTables2 */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IMetaDataTables2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BADB5F70-58DA-43a9-A1C6-D74819F19B15")
    IMetaDataTables2 : public IMetaDataTables
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMetaDataStorage( 
            /* [size_is][size_is][out] */ const BYTE **ppvMd,
            /* [out] */ ULONG *pcbMd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetaDataStreamInfo( 
            /* [in] */ ULONG ix,
            /* [string][out] */ LPCSTR *ppchName,
            /* [size_is][size_is][out] */ const BYTE **ppv,
            /* [out] */ ULONG *pcb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMetaDataTables2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMetaDataTables2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMetaDataTables2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMetaDataTables2 * This);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetStringHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetStringHeapSize )( 
            IMetaDataTables2 * This,
            /* [out] */ ULONG *pcbStrings);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetBlobHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobHeapSize )( 
            IMetaDataTables2 * This,
            /* [out] */ ULONG *pcbBlobs);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetGuidHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetGuidHeapSize )( 
            IMetaDataTables2 * This,
            /* [out] */ ULONG *pcbGuids);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetUserStringHeapSize)
        HRESULT ( STDMETHODCALLTYPE *GetUserStringHeapSize )( 
            IMetaDataTables2 * This,
            /* [out] */ ULONG *pcbUserStrings);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNumTables)
        HRESULT ( STDMETHODCALLTYPE *GetNumTables )( 
            IMetaDataTables2 * This,
            /* [out] */ ULONG *pcTables);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetTableIndex)
        HRESULT ( STDMETHODCALLTYPE *GetTableIndex )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG token,
            /* [out] */ ULONG *pixTbl);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetTableInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTableInfo )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixTbl,
            /* [out] */ ULONG *pcbRow,
            /* [out] */ ULONG *pcRows,
            /* [out] */ ULONG *pcCols,
            /* [out] */ ULONG *piKey,
            /* [string][out] */ LPCSTR *ppName);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetColumnInfo)
        HRESULT ( STDMETHODCALLTYPE *GetColumnInfo )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG ixCol,
            /* [out] */ ULONG *poCol,
            /* [out] */ ULONG *pcbCol,
            /* [out] */ ULONG *pType,
            /* [string][out] */ LPCSTR *ppName);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetCodedTokenInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCodedTokenInfo )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixCdTkn,
            /* [out] */ ULONG *pcTokens,
            /* [size_is][size_is][out] */ const ULONG **ppTokens,
            /* [string][out] */ LPCSTR *ppName);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetRow)
        HRESULT ( STDMETHODCALLTYPE *GetRow )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG rid,
            /* [out] */ const BYTE *ppRow);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetColumn)
        HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixTbl,
            /* [in] */ ULONG ixCol,
            /* [in] */ ULONG rid,
            /* [out] */ ULONG *pVal);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixString,
            /* [string][out] */ LPCSTR *ppString);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixBlob,
            /* [out] */ ULONG *pcbData,
            /* [size_is][size_is][out] */ const BYTE **ppData);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetGuid)
        HRESULT ( STDMETHODCALLTYPE *GetGuid )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixGuid,
            /* [out] */ const GUID **ppGUID);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetUserString)
        HRESULT ( STDMETHODCALLTYPE *GetUserString )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixUserString,
            /* [out] */ ULONG *pcbData,
            /* [size_is][size_is][out] */ const BYTE **ppData);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextString)
        HRESULT ( STDMETHODCALLTYPE *GetNextString )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixString,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextBlob)
        HRESULT ( STDMETHODCALLTYPE *GetNextBlob )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixBlob,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextGuid)
        HRESULT ( STDMETHODCALLTYPE *GetNextGuid )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixGuid,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables, GetNextUserString)
        HRESULT ( STDMETHODCALLTYPE *GetNextUserString )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ixUserString,
            /* [out] */ ULONG *pNext);
        
        DECLSPEC_XFGVIRT(IMetaDataTables2, GetMetaDataStorage)
        HRESULT ( STDMETHODCALLTYPE *GetMetaDataStorage )( 
            IMetaDataTables2 * This,
            /* [size_is][size_is][out] */ const BYTE **ppvMd,
            /* [out] */ ULONG *pcbMd);
        
        DECLSPEC_XFGVIRT(IMetaDataTables2, GetMetaDataStreamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetMetaDataStreamInfo )( 
            IMetaDataTables2 * This,
            /* [in] */ ULONG ix,
            /* [string][out] */ LPCSTR *ppchName,
            /* [size_is][size_is][out] */ const BYTE **ppv,
            /* [out] */ ULONG *pcb);
        
        END_INTERFACE
    } IMetaDataTables2Vtbl;

    interface IMetaDataTables2
    {
        CONST_VTBL struct IMetaDataTables2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMetaDataTables2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMetaDataTables2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMetaDataTables2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMetaDataTables2_GetStringHeapSize(This,pcbStrings)	\
    ( (This)->lpVtbl -> GetStringHeapSize(This,pcbStrings) ) 

#define IMetaDataTables2_GetBlobHeapSize(This,pcbBlobs)	\
    ( (This)->lpVtbl -> GetBlobHeapSize(This,pcbBlobs) ) 

#define IMetaDataTables2_GetGuidHeapSize(This,pcbGuids)	\
    ( (This)->lpVtbl -> GetGuidHeapSize(This,pcbGuids) ) 

#define IMetaDataTables2_GetUserStringHeapSize(This,pcbUserStrings)	\
    ( (This)->lpVtbl -> GetUserStringHeapSize(This,pcbUserStrings) ) 

#define IMetaDataTables2_GetNumTables(This,pcTables)	\
    ( (This)->lpVtbl -> GetNumTables(This,pcTables) ) 

#define IMetaDataTables2_GetTableIndex(This,token,pixTbl)	\
    ( (This)->lpVtbl -> GetTableIndex(This,token,pixTbl) ) 

#define IMetaDataTables2_GetTableInfo(This,ixTbl,pcbRow,pcRows,pcCols,piKey,ppName)	\
    ( (This)->lpVtbl -> GetTableInfo(This,ixTbl,pcbRow,pcRows,pcCols,piKey,ppName) ) 

#define IMetaDataTables2_GetColumnInfo(This,ixTbl,ixCol,poCol,pcbCol,pType,ppName)	\
    ( (This)->lpVtbl -> GetColumnInfo(This,ixTbl,ixCol,poCol,pcbCol,pType,ppName) ) 

#define IMetaDataTables2_GetCodedTokenInfo(This,ixCdTkn,pcTokens,ppTokens,ppName)	\
    ( (This)->lpVtbl -> GetCodedTokenInfo(This,ixCdTkn,pcTokens,ppTokens,ppName) ) 

#define IMetaDataTables2_GetRow(This,ixTbl,rid,ppRow)	\
    ( (This)->lpVtbl -> GetRow(This,ixTbl,rid,ppRow) ) 

#define IMetaDataTables2_GetColumn(This,ixTbl,ixCol,rid,pVal)	\
    ( (This)->lpVtbl -> GetColumn(This,ixTbl,ixCol,rid,pVal) ) 

#define IMetaDataTables2_GetString(This,ixString,ppString)	\
    ( (This)->lpVtbl -> GetString(This,ixString,ppString) ) 

#define IMetaDataTables2_GetBlob(This,ixBlob,pcbData,ppData)	\
    ( (This)->lpVtbl -> GetBlob(This,ixBlob,pcbData,ppData) ) 

#define IMetaDataTables2_GetGuid(This,ixGuid,ppGUID)	\
    ( (This)->lpVtbl -> GetGuid(This,ixGuid,ppGUID) ) 

#define IMetaDataTables2_GetUserString(This,ixUserString,pcbData,ppData)	\
    ( (This)->lpVtbl -> GetUserString(This,ixUserString,pcbData,ppData) ) 

#define IMetaDataTables2_GetNextString(This,ixString,pNext)	\
    ( (This)->lpVtbl -> GetNextString(This,ixString,pNext) ) 

#define IMetaDataTables2_GetNextBlob(This,ixBlob,pNext)	\
    ( (This)->lpVtbl -> GetNextBlob(This,ixBlob,pNext) ) 

#define IMetaDataTables2_GetNextGuid(This,ixGuid,pNext)	\
    ( (This)->lpVtbl -> GetNextGuid(This,ixGuid,pNext) ) 

#define IMetaDataTables2_GetNextUserString(This,ixUserString,pNext)	\
    ( (This)->lpVtbl -> GetNextUserString(This,ixUserString,pNext) ) 


#define IMetaDataTables2_GetMetaDataStorage(This,ppvMd,pcbMd)	\
    ( (This)->lpVtbl -> GetMetaDataStorage(This,ppvMd,pcbMd) ) 

#define IMetaDataTables2_GetMetaDataStreamInfo(This,ix,ppchName,ppv,pcb)	\
    ( (This)->lpVtbl -> GetMetaDataStreamInfo(This,ix,ppchName,ppv,pcb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMetaDataTables2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_rometadataapi_0000_0007 */
/* [local] */ 

#else

} /* extern "C" */
/* Include cor.h for C++ uses */
#include <cor.h>
#include <corhdr.h>
extern "C" {
#endif


extern RPC_IF_HANDLE __MIDL_itf_rometadataapi_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rometadataapi_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


