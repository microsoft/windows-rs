

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

#ifndef __icontact_h__
#define __icontact_h__

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

#ifndef __IContactManager_FWD_DEFINED__
#define __IContactManager_FWD_DEFINED__
typedef interface IContactManager IContactManager;

#endif 	/* __IContactManager_FWD_DEFINED__ */


#ifndef __IContactCollection_FWD_DEFINED__
#define __IContactCollection_FWD_DEFINED__
typedef interface IContactCollection IContactCollection;

#endif 	/* __IContactCollection_FWD_DEFINED__ */


#ifndef __IContactProperties_FWD_DEFINED__
#define __IContactProperties_FWD_DEFINED__
typedef interface IContactProperties IContactProperties;

#endif 	/* __IContactProperties_FWD_DEFINED__ */


#ifndef __IContact_FWD_DEFINED__
#define __IContact_FWD_DEFINED__
typedef interface IContact IContact;

#endif 	/* __IContact_FWD_DEFINED__ */


#ifndef __IContactPropertyCollection_FWD_DEFINED__
#define __IContactPropertyCollection_FWD_DEFINED__
typedef interface IContactPropertyCollection IContactPropertyCollection;

#endif 	/* __IContactPropertyCollection_FWD_DEFINED__ */


#ifndef __Contact_FWD_DEFINED__
#define __Contact_FWD_DEFINED__

#ifdef __cplusplus
typedef class Contact Contact;
#else
typedef struct Contact Contact;
#endif /* __cplusplus */

#endif 	/* __Contact_FWD_DEFINED__ */


#ifndef __ContactManager_FWD_DEFINED__
#define __ContactManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class ContactManager ContactManager;
#else
typedef struct ContactManager ContactManager;
#endif /* __cplusplus */

#endif 	/* __ContactManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_icontact_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






extern RPC_IF_HANDLE __MIDL_itf_icontact_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_icontact_0000_0000_v0_0_s_ifspec;

#ifndef __IContactManager_INTERFACE_DEFINED__
#define __IContactManager_INTERFACE_DEFINED__

/* interface IContactManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IContactManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ad553d98-deb1-474a-8e17-fc0c2075b738")
    IContactManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszAppName,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszAppVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszContactID,
            /* [out] */ __RPC__deref_out_opt IContact **ppContact) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MergeContactIDs( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszNewContactID,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszOldContactID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMeContact( 
            /* [out] */ __RPC__deref_out_opt IContact **ppMeContact) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMeContact( 
            /* [in] */ __RPC__in_opt IContact *pMeContact) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContactCollection( 
            /* [out] */ __RPC__deref_out_opt IContactCollection **ppContactCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactManager * This);
        
        DECLSPEC_XFGVIRT(IContactManager, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IContactManager * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszAppName,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszAppVersion);
        
        DECLSPEC_XFGVIRT(IContactManager, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IContactManager * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszContactID,
            /* [out] */ __RPC__deref_out_opt IContact **ppContact);
        
        DECLSPEC_XFGVIRT(IContactManager, MergeContactIDs)
        HRESULT ( STDMETHODCALLTYPE *MergeContactIDs )( 
            __RPC__in IContactManager * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszNewContactID,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszOldContactID);
        
        DECLSPEC_XFGVIRT(IContactManager, GetMeContact)
        HRESULT ( STDMETHODCALLTYPE *GetMeContact )( 
            __RPC__in IContactManager * This,
            /* [out] */ __RPC__deref_out_opt IContact **ppMeContact);
        
        DECLSPEC_XFGVIRT(IContactManager, SetMeContact)
        HRESULT ( STDMETHODCALLTYPE *SetMeContact )( 
            __RPC__in IContactManager * This,
            /* [in] */ __RPC__in_opt IContact *pMeContact);
        
        DECLSPEC_XFGVIRT(IContactManager, GetContactCollection)
        HRESULT ( STDMETHODCALLTYPE *GetContactCollection )( 
            __RPC__in IContactManager * This,
            /* [out] */ __RPC__deref_out_opt IContactCollection **ppContactCollection);
        
        END_INTERFACE
    } IContactManagerVtbl;

    interface IContactManager
    {
        CONST_VTBL struct IContactManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactManager_Initialize(This,pszAppName,pszAppVersion)	\
    ( (This)->lpVtbl -> Initialize(This,pszAppName,pszAppVersion) ) 

#define IContactManager_Load(This,pszContactID,ppContact)	\
    ( (This)->lpVtbl -> Load(This,pszContactID,ppContact) ) 

#define IContactManager_MergeContactIDs(This,pszNewContactID,pszOldContactID)	\
    ( (This)->lpVtbl -> MergeContactIDs(This,pszNewContactID,pszOldContactID) ) 

#define IContactManager_GetMeContact(This,ppMeContact)	\
    ( (This)->lpVtbl -> GetMeContact(This,ppMeContact) ) 

#define IContactManager_SetMeContact(This,pMeContact)	\
    ( (This)->lpVtbl -> SetMeContact(This,pMeContact) ) 

#define IContactManager_GetContactCollection(This,ppContactCollection)	\
    ( (This)->lpVtbl -> GetContactCollection(This,ppContactCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactManager_INTERFACE_DEFINED__ */


#ifndef __IContactCollection_INTERFACE_DEFINED__
#define __IContactCollection_INTERFACE_DEFINED__

/* interface IContactCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IContactCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b6afa338-d779-11d9-8bde-f66bad1e3f3a")
    IContactCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrent( 
            /* [out] */ __RPC__deref_out_opt IContact **ppContact) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactCollection * This);
        
        DECLSPEC_XFGVIRT(IContactCollection, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IContactCollection * This);
        
        DECLSPEC_XFGVIRT(IContactCollection, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IContactCollection * This);
        
        DECLSPEC_XFGVIRT(IContactCollection, GetCurrent)
        HRESULT ( STDMETHODCALLTYPE *GetCurrent )( 
            __RPC__in IContactCollection * This,
            /* [out] */ __RPC__deref_out_opt IContact **ppContact);
        
        END_INTERFACE
    } IContactCollectionVtbl;

    interface IContactCollection
    {
        CONST_VTBL struct IContactCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactCollection_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IContactCollection_Next(This)	\
    ( (This)->lpVtbl -> Next(This) ) 

#define IContactCollection_GetCurrent(This,ppContact)	\
    ( (This)->lpVtbl -> GetCurrent(This,ppContact) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactCollection_INTERFACE_DEFINED__ */


#ifndef __IContactProperties_INTERFACE_DEFINED__
#define __IContactProperties_INTERFACE_DEFINED__

/* interface IContactProperties */
/* [unique][helpstring][uuid][object] */ 

#define CGD_DEFAULT                      0x00000000

EXTERN_C const IID IID_IContactProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70dd27dd-5cbd-46e8-bef0-23b6b346288f")
    IContactProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [size_is][unique][string][out][in] */ __RPC__inout_ecount_full_opt_string(cchValue) LPWSTR pszValue,
            /* [in] */ DWORD cchValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchPropertyValueRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDate( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [unique][out][in] */ __RPC__inout_opt FILETIME *pftDateTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBinary( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cchContentType) LPWSTR pszContentType,
            /* [in] */ DWORD cchContentType,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchContentTypeRequired,
            /* [out] */ __RPC__deref_out_opt IStream **ppStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLabels( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cchLabels) LPWSTR pszLabels,
            /* [in] */ DWORD cchLabels,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchLabelsRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetString( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDate( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [in] */ FILETIME ftDateTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBinary( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszContentType,
            /* [unique][in] */ __RPC__in_opt IStream *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLabels( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags,
            /* [in] */ DWORD dwLabelCount,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwLabelCount) LPCWSTR ppszLabels[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateArrayNode( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayName,
            DWORD dwFlags,
            BOOL fAppend,
            /* [size_is][string][unique][out][in] */ __RPC__inout_ecount_full_opt_string(cchNewArrayElementName) LPWSTR pszNewArrayElementName,
            DWORD cchNewArrayElementName,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchNewArrayElementNameRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteProperty( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteArrayNode( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteLabels( 
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyCollection( 
            /* [out] */ __RPC__deref_out_opt IContactPropertyCollection **ppPropertyCollection,
            DWORD dwFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszMultiValueName,
            /* [in] */ DWORD dwLabelCount,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwLabelCount) LPCWSTR ppszLabels[  ],
            BOOL fAnyLabelMatches) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactProperties * This);
        
        DECLSPEC_XFGVIRT(IContactProperties, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [size_is][unique][string][out][in] */ __RPC__inout_ecount_full_opt_string(cchValue) LPWSTR pszValue,
            /* [in] */ DWORD cchValue,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchPropertyValueRequired);
        
        DECLSPEC_XFGVIRT(IContactProperties, GetDate)
        HRESULT ( STDMETHODCALLTYPE *GetDate )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [unique][out][in] */ __RPC__inout_opt FILETIME *pftDateTime);
        
        DECLSPEC_XFGVIRT(IContactProperties, GetBinary)
        HRESULT ( STDMETHODCALLTYPE *GetBinary )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cchContentType) LPWSTR pszContentType,
            /* [in] */ DWORD cchContentType,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchContentTypeRequired,
            /* [out] */ __RPC__deref_out_opt IStream **ppStream);
        
        DECLSPEC_XFGVIRT(IContactProperties, GetLabels)
        HRESULT ( STDMETHODCALLTYPE *GetLabels )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cchLabels) LPWSTR pszLabels,
            /* [in] */ DWORD cchLabels,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchLabelsRequired);
        
        DECLSPEC_XFGVIRT(IContactProperties, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszValue);
        
        DECLSPEC_XFGVIRT(IContactProperties, SetDate)
        HRESULT ( STDMETHODCALLTYPE *SetDate )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [in] */ FILETIME ftDateTime);
        
        DECLSPEC_XFGVIRT(IContactProperties, SetBinary)
        HRESULT ( STDMETHODCALLTYPE *SetBinary )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszContentType,
            /* [unique][in] */ __RPC__in_opt IStream *pStream);
        
        DECLSPEC_XFGVIRT(IContactProperties, SetLabels)
        HRESULT ( STDMETHODCALLTYPE *SetLabels )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags,
            /* [in] */ DWORD dwLabelCount,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwLabelCount) LPCWSTR ppszLabels[  ]);
        
        DECLSPEC_XFGVIRT(IContactProperties, CreateArrayNode)
        HRESULT ( STDMETHODCALLTYPE *CreateArrayNode )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayName,
            DWORD dwFlags,
            BOOL fAppend,
            /* [size_is][string][unique][out][in] */ __RPC__inout_ecount_full_opt_string(cchNewArrayElementName) LPWSTR pszNewArrayElementName,
            DWORD cchNewArrayElementName,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchNewArrayElementNameRequired);
        
        DECLSPEC_XFGVIRT(IContactProperties, DeleteProperty)
        HRESULT ( STDMETHODCALLTYPE *DeleteProperty )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszPropertyName,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IContactProperties, DeleteArrayNode)
        HRESULT ( STDMETHODCALLTYPE *DeleteArrayNode )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IContactProperties, DeleteLabels)
        HRESULT ( STDMETHODCALLTYPE *DeleteLabels )( 
            __RPC__in IContactProperties * This,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszArrayElementName,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IContactProperties, GetPropertyCollection)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyCollection )( 
            __RPC__in IContactProperties * This,
            /* [out] */ __RPC__deref_out_opt IContactPropertyCollection **ppPropertyCollection,
            DWORD dwFlags,
            /* [unique][string][in] */ __RPC__in_opt_string LPCWSTR pszMultiValueName,
            /* [in] */ DWORD dwLabelCount,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(dwLabelCount) LPCWSTR ppszLabels[  ],
            BOOL fAnyLabelMatches);
        
        END_INTERFACE
    } IContactPropertiesVtbl;

    interface IContactProperties
    {
        CONST_VTBL struct IContactPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactProperties_GetString(This,pszPropertyName,dwFlags,pszValue,cchValue,pdwcchPropertyValueRequired)	\
    ( (This)->lpVtbl -> GetString(This,pszPropertyName,dwFlags,pszValue,cchValue,pdwcchPropertyValueRequired) ) 

#define IContactProperties_GetDate(This,pszPropertyName,dwFlags,pftDateTime)	\
    ( (This)->lpVtbl -> GetDate(This,pszPropertyName,dwFlags,pftDateTime) ) 

#define IContactProperties_GetBinary(This,pszPropertyName,dwFlags,pszContentType,cchContentType,pdwcchContentTypeRequired,ppStream)	\
    ( (This)->lpVtbl -> GetBinary(This,pszPropertyName,dwFlags,pszContentType,cchContentType,pdwcchContentTypeRequired,ppStream) ) 

#define IContactProperties_GetLabels(This,pszArrayElementName,dwFlags,pszLabels,cchLabels,pdwcchLabelsRequired)	\
    ( (This)->lpVtbl -> GetLabels(This,pszArrayElementName,dwFlags,pszLabels,cchLabels,pdwcchLabelsRequired) ) 

#define IContactProperties_SetString(This,pszPropertyName,dwFlags,pszValue)	\
    ( (This)->lpVtbl -> SetString(This,pszPropertyName,dwFlags,pszValue) ) 

#define IContactProperties_SetDate(This,pszPropertyName,dwFlags,ftDateTime)	\
    ( (This)->lpVtbl -> SetDate(This,pszPropertyName,dwFlags,ftDateTime) ) 

#define IContactProperties_SetBinary(This,pszPropertyName,dwFlags,pszContentType,pStream)	\
    ( (This)->lpVtbl -> SetBinary(This,pszPropertyName,dwFlags,pszContentType,pStream) ) 

#define IContactProperties_SetLabels(This,pszArrayElementName,dwFlags,dwLabelCount,ppszLabels)	\
    ( (This)->lpVtbl -> SetLabels(This,pszArrayElementName,dwFlags,dwLabelCount,ppszLabels) ) 

#define IContactProperties_CreateArrayNode(This,pszArrayName,dwFlags,fAppend,pszNewArrayElementName,cchNewArrayElementName,pdwcchNewArrayElementNameRequired)	\
    ( (This)->lpVtbl -> CreateArrayNode(This,pszArrayName,dwFlags,fAppend,pszNewArrayElementName,cchNewArrayElementName,pdwcchNewArrayElementNameRequired) ) 

#define IContactProperties_DeleteProperty(This,pszPropertyName,dwFlags)	\
    ( (This)->lpVtbl -> DeleteProperty(This,pszPropertyName,dwFlags) ) 

#define IContactProperties_DeleteArrayNode(This,pszArrayElementName,dwFlags)	\
    ( (This)->lpVtbl -> DeleteArrayNode(This,pszArrayElementName,dwFlags) ) 

#define IContactProperties_DeleteLabels(This,pszArrayElementName,dwFlags)	\
    ( (This)->lpVtbl -> DeleteLabels(This,pszArrayElementName,dwFlags) ) 

#define IContactProperties_GetPropertyCollection(This,ppPropertyCollection,dwFlags,pszMultiValueName,dwLabelCount,ppszLabels,fAnyLabelMatches)	\
    ( (This)->lpVtbl -> GetPropertyCollection(This,ppPropertyCollection,dwFlags,pszMultiValueName,dwLabelCount,ppszLabels,fAnyLabelMatches) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactProperties_INTERFACE_DEFINED__ */


#ifndef __IContact_INTERFACE_DEFINED__
#define __IContact_INTERFACE_DEFINED__

/* interface IContact */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IContact;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F941B671-BDA7-4f77-884A-F46462F226A7")
    IContact : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetContactID( 
            /* [size_is][string][out][in] */ __RPC__inout_ecount_full_string(cchContactID) LPWSTR pszContactID,
            /* [in] */ DWORD cchContactID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchContactIDRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPath( 
            /* [size_is][string][out][in] */ __RPC__inout_ecount_full_string(cchPath) LPWSTR pszPath,
            /* [in] */ DWORD cchPath,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchPathRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitChanges( 
            /* [in] */ DWORD dwCommitFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContact * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContact * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContact * This);
        
        DECLSPEC_XFGVIRT(IContact, GetContactID)
        HRESULT ( STDMETHODCALLTYPE *GetContactID )( 
            __RPC__in IContact * This,
            /* [size_is][string][out][in] */ __RPC__inout_ecount_full_string(cchContactID) LPWSTR pszContactID,
            /* [in] */ DWORD cchContactID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchContactIDRequired);
        
        DECLSPEC_XFGVIRT(IContact, GetPath)
        HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            __RPC__in IContact * This,
            /* [size_is][string][out][in] */ __RPC__inout_ecount_full_string(cchPath) LPWSTR pszPath,
            /* [in] */ DWORD cchPath,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchPathRequired);
        
        DECLSPEC_XFGVIRT(IContact, CommitChanges)
        HRESULT ( STDMETHODCALLTYPE *CommitChanges )( 
            __RPC__in IContact * This,
            /* [in] */ DWORD dwCommitFlags);
        
        END_INTERFACE
    } IContactVtbl;

    interface IContact
    {
        CONST_VTBL struct IContactVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContact_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContact_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContact_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContact_GetContactID(This,pszContactID,cchContactID,pdwcchContactIDRequired)	\
    ( (This)->lpVtbl -> GetContactID(This,pszContactID,cchContactID,pdwcchContactIDRequired) ) 

#define IContact_GetPath(This,pszPath,cchPath,pdwcchPathRequired)	\
    ( (This)->lpVtbl -> GetPath(This,pszPath,cchPath,pdwcchPathRequired) ) 

#define IContact_CommitChanges(This,dwCommitFlags)	\
    ( (This)->lpVtbl -> CommitChanges(This,dwCommitFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContact_INTERFACE_DEFINED__ */


#ifndef __IContactPropertyCollection_INTERFACE_DEFINED__
#define __IContactPropertyCollection_INTERFACE_DEFINED__

/* interface IContactPropertyCollection */
/* [unique][helpstring][uuid][object] */ 

#define CGD_UNKNOWN_PROPERTY     0x00000000
#define CGD_STRING_PROPERTY      0x00000001
#define CGD_DATE_PROPERTY        0x00000002
#define CGD_BINARY_PROPERTY      0x00000004
#define CGD_ARRAY_NODE           0x00000008

EXTERN_C const IID IID_IContactPropertyCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ffd3adf8-fa64-4328-b1b6-2e0db509cb3c")
    IContactPropertyCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyName( 
            /* [unique][size_is][string][unique][out][in] */ __RPC__inout_ecount_full_opt_string(cchPropertyName) LPWSTR pszPropertyName,
            /* [in] */ DWORD cchPropertyName,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchPropertyNameRequired) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyType( 
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyVersion( 
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyModificationDate( 
            /* [unique][out][in] */ __RPC__inout_opt FILETIME *pftModificationDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyArrayElementID( 
            /* [unique][size_is][string][unique][out][in] */ __RPC__inout_ecount_full_opt_string(cchArrayElementID) LPWSTR pszArrayElementID,
            /* [in] */ DWORD cchArrayElementID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchArrayElementIDRequired) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactPropertyCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactPropertyCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IContactPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IContactPropertyCollection * This);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, GetPropertyName)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyName )( 
            __RPC__in IContactPropertyCollection * This,
            /* [unique][size_is][string][unique][out][in] */ __RPC__inout_ecount_full_opt_string(cchPropertyName) LPWSTR pszPropertyName,
            /* [in] */ DWORD cchPropertyName,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchPropertyNameRequired);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, GetPropertyType)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyType )( 
            __RPC__in IContactPropertyCollection * This,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, GetPropertyVersion)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyVersion )( 
            __RPC__in IContactPropertyCollection * This,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, GetPropertyModificationDate)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyModificationDate )( 
            __RPC__in IContactPropertyCollection * This,
            /* [unique][out][in] */ __RPC__inout_opt FILETIME *pftModificationDate);
        
        DECLSPEC_XFGVIRT(IContactPropertyCollection, GetPropertyArrayElementID)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyArrayElementID )( 
            __RPC__in IContactPropertyCollection * This,
            /* [unique][size_is][string][unique][out][in] */ __RPC__inout_ecount_full_opt_string(cchArrayElementID) LPWSTR pszArrayElementID,
            /* [in] */ DWORD cchArrayElementID,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwcchArrayElementIDRequired);
        
        END_INTERFACE
    } IContactPropertyCollectionVtbl;

    interface IContactPropertyCollection
    {
        CONST_VTBL struct IContactPropertyCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactPropertyCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactPropertyCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactPropertyCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactPropertyCollection_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IContactPropertyCollection_Next(This)	\
    ( (This)->lpVtbl -> Next(This) ) 

#define IContactPropertyCollection_GetPropertyName(This,pszPropertyName,cchPropertyName,pdwcchPropertyNameRequired)	\
    ( (This)->lpVtbl -> GetPropertyName(This,pszPropertyName,cchPropertyName,pdwcchPropertyNameRequired) ) 

#define IContactPropertyCollection_GetPropertyType(This,pdwType)	\
    ( (This)->lpVtbl -> GetPropertyType(This,pdwType) ) 

#define IContactPropertyCollection_GetPropertyVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetPropertyVersion(This,pdwVersion) ) 

#define IContactPropertyCollection_GetPropertyModificationDate(This,pftModificationDate)	\
    ( (This)->lpVtbl -> GetPropertyModificationDate(This,pftModificationDate) ) 

#define IContactPropertyCollection_GetPropertyArrayElementID(This,pszArrayElementID,cchArrayElementID,pdwcchArrayElementIDRequired)	\
    ( (This)->lpVtbl -> GetPropertyArrayElementID(This,pszArrayElementID,cchArrayElementID,pdwcchArrayElementIDRequired) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactPropertyCollection_INTERFACE_DEFINED__ */



#ifndef __CONTACT_LIBRARY_DEFINED__
#define __CONTACT_LIBRARY_DEFINED__

/* library CONTACT */
/* [version][lcid][helpstring][uuid] */ 


EXTERN_C const IID LIBID_CONTACT;

EXTERN_C const CLSID CLSID_Contact;

#ifdef __cplusplus

class DECLSPEC_UUID("61b68808-8eee-4fd1-acb8-3d804c8db056")
Contact;
#endif

EXTERN_C const CLSID CLSID_ContactManager;

#ifdef __cplusplus

class DECLSPEC_UUID("7165c8ab-af88-42bd-86fd-5310b4285a02")
ContactManager;
#endif
#endif /* __CONTACT_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_icontact_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_icontact_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_icontact_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


