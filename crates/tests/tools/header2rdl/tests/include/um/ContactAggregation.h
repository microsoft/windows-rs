

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

#ifndef __contactaggregation_h__
#define __contactaggregation_h__

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

#ifndef __IContactAggregationManager_FWD_DEFINED__
#define __IContactAggregationManager_FWD_DEFINED__
typedef interface IContactAggregationManager IContactAggregationManager;

#endif 	/* __IContactAggregationManager_FWD_DEFINED__ */


#ifndef __IContactAggregationContact_FWD_DEFINED__
#define __IContactAggregationContact_FWD_DEFINED__
typedef interface IContactAggregationContact IContactAggregationContact;

#endif 	/* __IContactAggregationContact_FWD_DEFINED__ */


#ifndef __IContactAggregationContactCollection_FWD_DEFINED__
#define __IContactAggregationContactCollection_FWD_DEFINED__
typedef interface IContactAggregationContactCollection IContactAggregationContactCollection;

#endif 	/* __IContactAggregationContactCollection_FWD_DEFINED__ */


#ifndef __IContactAggregationAggregate_FWD_DEFINED__
#define __IContactAggregationAggregate_FWD_DEFINED__
typedef interface IContactAggregationAggregate IContactAggregationAggregate;

#endif 	/* __IContactAggregationAggregate_FWD_DEFINED__ */


#ifndef __IContactAggregationAggregateCollection_FWD_DEFINED__
#define __IContactAggregationAggregateCollection_FWD_DEFINED__
typedef interface IContactAggregationAggregateCollection IContactAggregationAggregateCollection;

#endif 	/* __IContactAggregationAggregateCollection_FWD_DEFINED__ */


#ifndef __IContactAggregationGroup_FWD_DEFINED__
#define __IContactAggregationGroup_FWD_DEFINED__
typedef interface IContactAggregationGroup IContactAggregationGroup;

#endif 	/* __IContactAggregationGroup_FWD_DEFINED__ */


#ifndef __IContactAggregationGroupCollection_FWD_DEFINED__
#define __IContactAggregationGroupCollection_FWD_DEFINED__
typedef interface IContactAggregationGroupCollection IContactAggregationGroupCollection;

#endif 	/* __IContactAggregationGroupCollection_FWD_DEFINED__ */


#ifndef __IContactAggregationLink_FWD_DEFINED__
#define __IContactAggregationLink_FWD_DEFINED__
typedef interface IContactAggregationLink IContactAggregationLink;

#endif 	/* __IContactAggregationLink_FWD_DEFINED__ */


#ifndef __IContactAggregationLinkCollection_FWD_DEFINED__
#define __IContactAggregationLinkCollection_FWD_DEFINED__
typedef interface IContactAggregationLinkCollection IContactAggregationLinkCollection;

#endif 	/* __IContactAggregationLinkCollection_FWD_DEFINED__ */


#ifndef __IContactAggregationServerPerson_FWD_DEFINED__
#define __IContactAggregationServerPerson_FWD_DEFINED__
typedef interface IContactAggregationServerPerson IContactAggregationServerPerson;

#endif 	/* __IContactAggregationServerPerson_FWD_DEFINED__ */


#ifndef __IContactAggregationServerPersonCollection_FWD_DEFINED__
#define __IContactAggregationServerPersonCollection_FWD_DEFINED__
typedef interface IContactAggregationServerPersonCollection IContactAggregationServerPersonCollection;

#endif 	/* __IContactAggregationServerPersonCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_contactaggregation_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if NTDDI_VERSION >= NTDDI_WIN10_RS1











typedef /* [v1_enum] */ 
enum CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS
    {
        CA_CREATE_LOCAL	= 0,
        CA_CREATE_EXTERNAL	= 1
    } 	CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS;

typedef /* [v1_enum] */ 
enum CONTACT_AGGREGATION_COLLECTION_OPTIONS
    {
        CACO_DEFAULT	= 0,
        CACO_INCLUDE_EXTERNAL	= 1,
        CACO_EXTERNAL_ONLY	= 2
    } 	CONTACT_AGGREGATION_COLLECTION_OPTIONS;

typedef struct _CONTACT_AGGREGATION_BLOB
    {
    DWORD dwCount;
    /* [size_is] */ BYTE *lpb;
    } 	CONTACT_AGGREGATION_BLOB;

typedef struct _CONTACT_AGGREGATION_BLOB *PCONTACT_AGGREGATION_BLOB;

DEFINE_GUID(CLSID_ContactAggregationManager, 0x96c8ad95, 0xc199, 0x44de, 0xb3, 0x4e, 0xac, 0x33, 0xc4, 0x42, 0xdf, 0x39);
#pragma deprecated(IContactAggregationManager)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0000_v0_0_s_ifspec;

#ifndef __IContactAggregationManager_INTERFACE_DEFINED__
#define __IContactAggregationManager_INTERFACE_DEFINED__

/* interface IContactAggregationManager */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1D865989-4B1F-4B60-8F34-C2AD468B2B50")
    IContactAggregationManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVersionInfo( 
            /* [out] */ __RPC__out long *plMajorVersion,
            /* [out] */ __RPC__out long *plMinorVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateOrOpenGroup( 
            /* [in] */ __RPC__in LPCWSTR pGroupName,
            /* [in] */ CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS options,
            /* [out] */ __RPC__out BOOL *pCreatedGroup,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateExternalContact( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateServerPerson( 
            /* [out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateServerContactLink( 
            /* [out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenAggregateContact( 
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenContact( 
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenServerContactLink( 
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationLink **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenServerPerson( 
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppItem) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Contacts( 
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContactCollection **ppItems) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AggregateContacts( 
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [out] */ __RPC__deref_out_opt IContactAggregationAggregateCollection **ppAggregates) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [out] */ __RPC__deref_out_opt IContactAggregationGroupCollection **ppGroups) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ServerPersons( 
            /* [out] */ __RPC__deref_out_opt IContactAggregationServerPersonCollection **ppServerPersonCollection) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ServerContactLinks( 
            /* [in] */ __RPC__in LPCWSTR pPersonItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationLinkCollection **ppServerContactLinkCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationManager * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, GetVersionInfo)
        HRESULT ( STDMETHODCALLTYPE *GetVersionInfo )( 
            __RPC__in IContactAggregationManager * This,
            /* [out] */ __RPC__out long *plMajorVersion,
            /* [out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, CreateOrOpenGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateOrOpenGroup )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in LPCWSTR pGroupName,
            /* [in] */ CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS options,
            /* [out] */ __RPC__out BOOL *pCreatedGroup,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, CreateExternalContact)
        HRESULT ( STDMETHODCALLTYPE *CreateExternalContact )( 
            __RPC__in IContactAggregationManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, CreateServerPerson)
        HRESULT ( STDMETHODCALLTYPE *CreateServerPerson )( 
            __RPC__in IContactAggregationManager * This,
            /* [out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, CreateServerContactLink)
        HRESULT ( STDMETHODCALLTYPE *CreateServerContactLink )( 
            __RPC__in IContactAggregationManager * This,
            /* [out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in IContactAggregationManager * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, OpenAggregateContact)
        HRESULT ( STDMETHODCALLTYPE *OpenAggregateContact )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, OpenContact)
        HRESULT ( STDMETHODCALLTYPE *OpenContact )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, OpenServerContactLink)
        HRESULT ( STDMETHODCALLTYPE *OpenServerContactLink )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationLink **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, OpenServerPerson)
        HRESULT ( STDMETHODCALLTYPE *OpenServerPerson )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in LPCWSTR pItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, get_Contacts)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Contacts )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContactCollection **ppItems);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, get_AggregateContacts)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AggregateContacts )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [out] */ __RPC__deref_out_opt IContactAggregationAggregateCollection **ppAggregates);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, get_Groups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [out] */ __RPC__deref_out_opt IContactAggregationGroupCollection **ppGroups);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, get_ServerPersons)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerPersons )( 
            __RPC__in IContactAggregationManager * This,
            /* [out] */ __RPC__deref_out_opt IContactAggregationServerPersonCollection **ppServerPersonCollection);
        
        DECLSPEC_XFGVIRT(IContactAggregationManager, get_ServerContactLinks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerContactLinks )( 
            __RPC__in IContactAggregationManager * This,
            /* [in] */ __RPC__in LPCWSTR pPersonItemId,
            /* [out] */ __RPC__deref_out_opt IContactAggregationLinkCollection **ppServerContactLinkCollection);
        
        END_INTERFACE
    } IContactAggregationManagerVtbl;

    interface IContactAggregationManager
    {
        CONST_VTBL struct IContactAggregationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationManager_GetVersionInfo(This,plMajorVersion,plMinorVersion)	\
    ( (This)->lpVtbl -> GetVersionInfo(This,plMajorVersion,plMinorVersion) ) 

#define IContactAggregationManager_CreateOrOpenGroup(This,pGroupName,options,pCreatedGroup,ppGroup)	\
    ( (This)->lpVtbl -> CreateOrOpenGroup(This,pGroupName,options,pCreatedGroup,ppGroup) ) 

#define IContactAggregationManager_CreateExternalContact(This,ppItem)	\
    ( (This)->lpVtbl -> CreateExternalContact(This,ppItem) ) 

#define IContactAggregationManager_CreateServerPerson(This,ppServerPerson)	\
    ( (This)->lpVtbl -> CreateServerPerson(This,ppServerPerson) ) 

#define IContactAggregationManager_CreateServerContactLink(This,ppServerContactLink)	\
    ( (This)->lpVtbl -> CreateServerContactLink(This,ppServerContactLink) ) 

#define IContactAggregationManager_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define IContactAggregationManager_OpenAggregateContact(This,pItemId,ppItem)	\
    ( (This)->lpVtbl -> OpenAggregateContact(This,pItemId,ppItem) ) 

#define IContactAggregationManager_OpenContact(This,pItemId,ppItem)	\
    ( (This)->lpVtbl -> OpenContact(This,pItemId,ppItem) ) 

#define IContactAggregationManager_OpenServerContactLink(This,pItemId,ppItem)	\
    ( (This)->lpVtbl -> OpenServerContactLink(This,pItemId,ppItem) ) 

#define IContactAggregationManager_OpenServerPerson(This,pItemId,ppItem)	\
    ( (This)->lpVtbl -> OpenServerPerson(This,pItemId,ppItem) ) 

#define IContactAggregationManager_get_Contacts(This,options,ppItems)	\
    ( (This)->lpVtbl -> get_Contacts(This,options,ppItems) ) 

#define IContactAggregationManager_get_AggregateContacts(This,options,ppAggregates)	\
    ( (This)->lpVtbl -> get_AggregateContacts(This,options,ppAggregates) ) 

#define IContactAggregationManager_get_Groups(This,options,ppGroups)	\
    ( (This)->lpVtbl -> get_Groups(This,options,ppGroups) ) 

#define IContactAggregationManager_get_ServerPersons(This,ppServerPersonCollection)	\
    ( (This)->lpVtbl -> get_ServerPersons(This,ppServerPersonCollection) ) 

#define IContactAggregationManager_get_ServerContactLinks(This,pPersonItemId,ppServerContactLinkCollection)	\
    ( (This)->lpVtbl -> get_ServerContactLinks(This,pPersonItemId,ppServerContactLinkCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0001 */
/* [local] */ 

#pragma deprecated(IContactAggregationContact)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0001_v0_0_s_ifspec;

#ifndef __IContactAggregationContact_INTERFACE_DEFINED__
#define __IContactAggregationContact_INTERFACE_DEFINED__

/* interface IContactAggregationContact */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationContact;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1EB22E86-4C86-41F0-9F9F-C251E9FDA6C3")
    IContactAggregationContact : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveToAggregate( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unlink( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AccountId( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAccountId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AccountId( 
            /* [in] */ __RPC__in LPCWSTR pAccountId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AggregateId( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppAggregateId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsMe( 
            /* [retval][out] */ __RPC__out BOOL *pIsMe) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsExternal( 
            /* [retval][out] */ __RPC__out BOOL *pIsExternal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NetworkSourceId( 
            /* [retval][out] */ __RPC__out ULONG *pNetworkSourceId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_NetworkSourceId( 
            /* [in] */ ULONG networkSourceId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NetworkSourceIdString( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppNetworkSourceId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_NetworkSourceIdString( 
            /* [in] */ __RPC__in LPCWSTR pNetworkSourceId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RemoteObjectId( 
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppRemoteObjectId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RemoteObjectId( 
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteObjectId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SyncIdentityHash( 
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppSyncIdentityHash) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SyncIdentityHash( 
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pSyncIdentityHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationContactVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationContact * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationContact * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IContactAggregationContact * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IContactAggregationContact * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, MoveToAggregate)
        HRESULT ( STDMETHODCALLTYPE *MoveToAggregate )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, Unlink)
        HRESULT ( STDMETHODCALLTYPE *Unlink )( 
            __RPC__in IContactAggregationContact * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_AccountId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AccountId )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAccountId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, put_AccountId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AccountId )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ __RPC__in LPCWSTR pAccountId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_AggregateId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AggregateId )( 
            __RPC__in IContactAggregationContact * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_IsMe)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsMe )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__out BOOL *pIsMe);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_IsExternal)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsExternal )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__out BOOL *pIsExternal);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_NetworkSourceId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkSourceId )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__out ULONG *pNetworkSourceId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, put_NetworkSourceId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkSourceId )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ ULONG networkSourceId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_NetworkSourceIdString)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkSourceIdString )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppNetworkSourceId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, put_NetworkSourceIdString)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkSourceIdString )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ __RPC__in LPCWSTR pNetworkSourceId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_RemoteObjectId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoteObjectId )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppRemoteObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, put_RemoteObjectId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RemoteObjectId )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, get_SyncIdentityHash)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SyncIdentityHash )( 
            __RPC__in IContactAggregationContact * This,
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppSyncIdentityHash);
        
        DECLSPEC_XFGVIRT(IContactAggregationContact, put_SyncIdentityHash)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SyncIdentityHash )( 
            __RPC__in IContactAggregationContact * This,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pSyncIdentityHash);
        
        END_INTERFACE
    } IContactAggregationContactVtbl;

    interface IContactAggregationContact
    {
        CONST_VTBL struct IContactAggregationContactVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationContact_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationContact_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationContact_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationContact_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IContactAggregationContact_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IContactAggregationContact_MoveToAggregate(This,pAggregateId)	\
    ( (This)->lpVtbl -> MoveToAggregate(This,pAggregateId) ) 

#define IContactAggregationContact_Unlink(This)	\
    ( (This)->lpVtbl -> Unlink(This) ) 

#define IContactAggregationContact_get_AccountId(This,ppAccountId)	\
    ( (This)->lpVtbl -> get_AccountId(This,ppAccountId) ) 

#define IContactAggregationContact_put_AccountId(This,pAccountId)	\
    ( (This)->lpVtbl -> put_AccountId(This,pAccountId) ) 

#define IContactAggregationContact_get_AggregateId(This,ppAggregateId)	\
    ( (This)->lpVtbl -> get_AggregateId(This,ppAggregateId) ) 

#define IContactAggregationContact_get_Id(This,ppItemId)	\
    ( (This)->lpVtbl -> get_Id(This,ppItemId) ) 

#define IContactAggregationContact_get_IsMe(This,pIsMe)	\
    ( (This)->lpVtbl -> get_IsMe(This,pIsMe) ) 

#define IContactAggregationContact_get_IsExternal(This,pIsExternal)	\
    ( (This)->lpVtbl -> get_IsExternal(This,pIsExternal) ) 

#define IContactAggregationContact_get_NetworkSourceId(This,pNetworkSourceId)	\
    ( (This)->lpVtbl -> get_NetworkSourceId(This,pNetworkSourceId) ) 

#define IContactAggregationContact_put_NetworkSourceId(This,networkSourceId)	\
    ( (This)->lpVtbl -> put_NetworkSourceId(This,networkSourceId) ) 

#define IContactAggregationContact_get_NetworkSourceIdString(This,ppNetworkSourceId)	\
    ( (This)->lpVtbl -> get_NetworkSourceIdString(This,ppNetworkSourceId) ) 

#define IContactAggregationContact_put_NetworkSourceIdString(This,pNetworkSourceId)	\
    ( (This)->lpVtbl -> put_NetworkSourceIdString(This,pNetworkSourceId) ) 

#define IContactAggregationContact_get_RemoteObjectId(This,ppRemoteObjectId)	\
    ( (This)->lpVtbl -> get_RemoteObjectId(This,ppRemoteObjectId) ) 

#define IContactAggregationContact_put_RemoteObjectId(This,pRemoteObjectId)	\
    ( (This)->lpVtbl -> put_RemoteObjectId(This,pRemoteObjectId) ) 

#define IContactAggregationContact_get_SyncIdentityHash(This,ppSyncIdentityHash)	\
    ( (This)->lpVtbl -> get_SyncIdentityHash(This,ppSyncIdentityHash) ) 

#define IContactAggregationContact_put_SyncIdentityHash(This,pSyncIdentityHash)	\
    ( (This)->lpVtbl -> put_SyncIdentityHash(This,pSyncIdentityHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationContact_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0002 */
/* [local] */ 

#pragma deprecated(IContactAggregationContactCollection)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0002_v0_0_s_ifspec;

#ifndef __IContactAggregationContactCollection_INTERFACE_DEFINED__
#define __IContactAggregationContactCollection_INTERFACE_DEFINED__

/* interface IContactAggregationContactCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationContactCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("826E66FA-81DE-43CA-A6FB-8C785CD996C6")
    IContactAggregationContactCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFirst( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNext( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByIdentityHash( 
            /* [in] */ __RPC__in LPCWSTR pSourceType,
            /* [in] */ __RPC__in LPCWSTR pAccountId,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pIdentityHash,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out int *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByRemoteId( 
            /* [in] */ __RPC__in LPCWSTR pSourceType,
            /* [in] */ __RPC__in LPCWSTR pAccountId,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteObjectId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationContactCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationContactCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationContactCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationContactCollection * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationContactCollection, FindFirst)
        HRESULT ( STDMETHODCALLTYPE *FindFirst )( 
            __RPC__in IContactAggregationContactCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationContactCollection, FindNext)
        HRESULT ( STDMETHODCALLTYPE *FindNext )( 
            __RPC__in IContactAggregationContactCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationContactCollection, FindFirstByIdentityHash)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByIdentityHash )( 
            __RPC__in IContactAggregationContactCollection * This,
            /* [in] */ __RPC__in LPCWSTR pSourceType,
            /* [in] */ __RPC__in LPCWSTR pAccountId,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pIdentityHash,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem);
        
        DECLSPEC_XFGVIRT(IContactAggregationContactCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IContactAggregationContactCollection * This,
            /* [retval][out] */ __RPC__out int *pCount);
        
        DECLSPEC_XFGVIRT(IContactAggregationContactCollection, FindFirstByRemoteId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByRemoteId )( 
            __RPC__in IContactAggregationContactCollection * This,
            /* [in] */ __RPC__in LPCWSTR pSourceType,
            /* [in] */ __RPC__in LPCWSTR pAccountId,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteObjectId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationContact **ppItem);
        
        END_INTERFACE
    } IContactAggregationContactCollectionVtbl;

    interface IContactAggregationContactCollection
    {
        CONST_VTBL struct IContactAggregationContactCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationContactCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationContactCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationContactCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationContactCollection_FindFirst(This,ppItem)	\
    ( (This)->lpVtbl -> FindFirst(This,ppItem) ) 

#define IContactAggregationContactCollection_FindNext(This,ppItem)	\
    ( (This)->lpVtbl -> FindNext(This,ppItem) ) 

#define IContactAggregationContactCollection_FindFirstByIdentityHash(This,pSourceType,pAccountId,pIdentityHash,ppItem)	\
    ( (This)->lpVtbl -> FindFirstByIdentityHash(This,pSourceType,pAccountId,pIdentityHash,ppItem) ) 

#define IContactAggregationContactCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define IContactAggregationContactCollection_FindFirstByRemoteId(This,pSourceType,pAccountId,pRemoteObjectId,ppItem)	\
    ( (This)->lpVtbl -> FindFirstByRemoteId(This,pSourceType,pAccountId,pRemoteObjectId,ppItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationContactCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0003 */
/* [local] */ 

#pragma deprecated(IContactAggregationAggregate)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0003_v0_0_s_ifspec;

#ifndef __IContactAggregationAggregate_INTERFACE_DEFINED__
#define __IContactAggregationAggregate_INTERFACE_DEFINED__

/* interface IContactAggregationAggregate */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationAggregate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7ED1C814-CD30-43C8-9B8D-2E489E53D54B")
    IContactAggregationAggregate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetComponentItems( 
            /* [out] */ __RPC__deref_out_opt IContactAggregationContactCollection **pComponentItems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Link( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [out] */ __RPC__deref_out_opt IContactAggregationGroupCollection **ppGroups) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AntiLink( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAntiLink) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AntiLink( 
            /* [in] */ __RPC__in LPCWSTR pAntiLink) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FavoriteOrder( 
            /* [retval][out] */ __RPC__out ULONG *pFavoriteOrder) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FavoriteOrder( 
            /* [in] */ ULONG favoriteOrder) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationAggregateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationAggregate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationAggregate * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IContactAggregationAggregate * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, GetComponentItems)
        HRESULT ( STDMETHODCALLTYPE *GetComponentItems )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [out] */ __RPC__deref_out_opt IContactAggregationContactCollection **pComponentItems);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, Link)
        HRESULT ( STDMETHODCALLTYPE *Link )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, get_Groups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [in] */ CONTACT_AGGREGATION_COLLECTION_OPTIONS options,
            /* [out] */ __RPC__deref_out_opt IContactAggregationGroupCollection **ppGroups);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, get_AntiLink)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntiLink )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAntiLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, put_AntiLink)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AntiLink )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [in] */ __RPC__in LPCWSTR pAntiLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, get_FavoriteOrder)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FavoriteOrder )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [retval][out] */ __RPC__out ULONG *pFavoriteOrder);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, put_FavoriteOrder)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FavoriteOrder )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [in] */ ULONG favoriteOrder);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregate, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IContactAggregationAggregate * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId);
        
        END_INTERFACE
    } IContactAggregationAggregateVtbl;

    interface IContactAggregationAggregate
    {
        CONST_VTBL struct IContactAggregationAggregateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationAggregate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationAggregate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationAggregate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationAggregate_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IContactAggregationAggregate_GetComponentItems(This,pComponentItems)	\
    ( (This)->lpVtbl -> GetComponentItems(This,pComponentItems) ) 

#define IContactAggregationAggregate_Link(This,pAggregateId)	\
    ( (This)->lpVtbl -> Link(This,pAggregateId) ) 

#define IContactAggregationAggregate_get_Groups(This,options,ppGroups)	\
    ( (This)->lpVtbl -> get_Groups(This,options,ppGroups) ) 

#define IContactAggregationAggregate_get_AntiLink(This,ppAntiLink)	\
    ( (This)->lpVtbl -> get_AntiLink(This,ppAntiLink) ) 

#define IContactAggregationAggregate_put_AntiLink(This,pAntiLink)	\
    ( (This)->lpVtbl -> put_AntiLink(This,pAntiLink) ) 

#define IContactAggregationAggregate_get_FavoriteOrder(This,pFavoriteOrder)	\
    ( (This)->lpVtbl -> get_FavoriteOrder(This,pFavoriteOrder) ) 

#define IContactAggregationAggregate_put_FavoriteOrder(This,favoriteOrder)	\
    ( (This)->lpVtbl -> put_FavoriteOrder(This,favoriteOrder) ) 

#define IContactAggregationAggregate_get_Id(This,ppItemId)	\
    ( (This)->lpVtbl -> get_Id(This,ppItemId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationAggregate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0004 */
/* [local] */ 

#pragma deprecated(IContactAggregationAggregateCollection)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0004_v0_0_s_ifspec;

#ifndef __IContactAggregationAggregateCollection_INTERFACE_DEFINED__
#define __IContactAggregationAggregateCollection_INTERFACE_DEFINED__

/* interface IContactAggregationAggregateCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationAggregateCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2359F3A6-3A68-40AF-98DB-0F9EB143C3BB")
    IContactAggregationAggregateCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFirst( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppAggregate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByAntiLinkId( 
            /* [in] */ __RPC__in LPCWSTR pAntiLinkId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppAggregate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNext( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppAggregate) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out int *pCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationAggregateCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationAggregateCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationAggregateCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationAggregateCollection * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregateCollection, FindFirst)
        HRESULT ( STDMETHODCALLTYPE *FindFirst )( 
            __RPC__in IContactAggregationAggregateCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppAggregate);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregateCollection, FindFirstByAntiLinkId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByAntiLinkId )( 
            __RPC__in IContactAggregationAggregateCollection * This,
            /* [in] */ __RPC__in LPCWSTR pAntiLinkId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppAggregate);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregateCollection, FindNext)
        HRESULT ( STDMETHODCALLTYPE *FindNext )( 
            __RPC__in IContactAggregationAggregateCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregate **ppAggregate);
        
        DECLSPEC_XFGVIRT(IContactAggregationAggregateCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IContactAggregationAggregateCollection * This,
            /* [retval][out] */ __RPC__out int *pCount);
        
        END_INTERFACE
    } IContactAggregationAggregateCollectionVtbl;

    interface IContactAggregationAggregateCollection
    {
        CONST_VTBL struct IContactAggregationAggregateCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationAggregateCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationAggregateCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationAggregateCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationAggregateCollection_FindFirst(This,ppAggregate)	\
    ( (This)->lpVtbl -> FindFirst(This,ppAggregate) ) 

#define IContactAggregationAggregateCollection_FindFirstByAntiLinkId(This,pAntiLinkId,ppAggregate)	\
    ( (This)->lpVtbl -> FindFirstByAntiLinkId(This,pAntiLinkId,ppAggregate) ) 

#define IContactAggregationAggregateCollection_FindNext(This,ppAggregate)	\
    ( (This)->lpVtbl -> FindNext(This,ppAggregate) ) 

#define IContactAggregationAggregateCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationAggregateCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0005 */
/* [local] */ 

#pragma deprecated(IContactAggregationGroup)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0005_v0_0_s_ifspec;

#ifndef __IContactAggregationGroup_INTERFACE_DEFINED__
#define __IContactAggregationGroup_INTERFACE_DEFINED__

/* interface IContactAggregationGroup */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C93C545F-1284-499B-96AF-07372AF473E0")
    IContactAggregationGroup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Members( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregateCollection **ppAggregateContactCollection) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_GlobalObjectId( 
            /* [retval][out] */ __RPC__out GUID *pGlobalObjectId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_GlobalObjectId( 
            /* [in] */ __RPC__in const GUID *pGlobalObjectId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in LPCWSTR pName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationGroup * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IContactAggregationGroup * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IContactAggregationGroup * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IContactAggregationGroup * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IContactAggregationGroup * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, get_Members)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in IContactAggregationGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationAggregateCollection **ppAggregateContactCollection);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, get_GlobalObjectId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GlobalObjectId )( 
            __RPC__in IContactAggregationGroup * This,
            /* [retval][out] */ __RPC__out GUID *pGlobalObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, put_GlobalObjectId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GlobalObjectId )( 
            __RPC__in IContactAggregationGroup * This,
            /* [in] */ __RPC__in const GUID *pGlobalObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IContactAggregationGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IContactAggregationGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppName);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroup, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IContactAggregationGroup * This,
            /* [in] */ __RPC__in LPCWSTR pName);
        
        END_INTERFACE
    } IContactAggregationGroupVtbl;

    interface IContactAggregationGroup
    {
        CONST_VTBL struct IContactAggregationGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationGroup_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IContactAggregationGroup_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IContactAggregationGroup_Add(This,pAggregateId)	\
    ( (This)->lpVtbl -> Add(This,pAggregateId) ) 

#define IContactAggregationGroup_Remove(This,pAggregateId)	\
    ( (This)->lpVtbl -> Remove(This,pAggregateId) ) 

#define IContactAggregationGroup_get_Members(This,ppAggregateContactCollection)	\
    ( (This)->lpVtbl -> get_Members(This,ppAggregateContactCollection) ) 

#define IContactAggregationGroup_get_GlobalObjectId(This,pGlobalObjectId)	\
    ( (This)->lpVtbl -> get_GlobalObjectId(This,pGlobalObjectId) ) 

#define IContactAggregationGroup_put_GlobalObjectId(This,pGlobalObjectId)	\
    ( (This)->lpVtbl -> put_GlobalObjectId(This,pGlobalObjectId) ) 

#define IContactAggregationGroup_get_Id(This,ppItemId)	\
    ( (This)->lpVtbl -> get_Id(This,ppItemId) ) 

#define IContactAggregationGroup_get_Name(This,ppName)	\
    ( (This)->lpVtbl -> get_Name(This,ppName) ) 

#define IContactAggregationGroup_put_Name(This,pName)	\
    ( (This)->lpVtbl -> put_Name(This,pName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationGroup_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0006 */
/* [local] */ 

#pragma deprecated(IContactAggregationGroupCollection)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0006_v0_0_s_ifspec;

#ifndef __IContactAggregationGroupCollection_INTERFACE_DEFINED__
#define __IContactAggregationGroupCollection_INTERFACE_DEFINED__

/* interface IContactAggregationGroupCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationGroupCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20A19A9C-D2F3-4B83-9143-BEFFD2CC226D")
    IContactAggregationGroupCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFirst( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByGlobalObjectId( 
            /* [in] */ __RPC__in const GUID *pGlobalObjectId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNext( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out UINT *pCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationGroupCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationGroupCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationGroupCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationGroupCollection * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroupCollection, FindFirst)
        HRESULT ( STDMETHODCALLTYPE *FindFirst )( 
            __RPC__in IContactAggregationGroupCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroupCollection, FindFirstByGlobalObjectId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByGlobalObjectId )( 
            __RPC__in IContactAggregationGroupCollection * This,
            /* [in] */ __RPC__in const GUID *pGlobalObjectId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroupCollection, FindNext)
        HRESULT ( STDMETHODCALLTYPE *FindNext )( 
            __RPC__in IContactAggregationGroupCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IContactAggregationGroupCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IContactAggregationGroupCollection * This,
            /* [retval][out] */ __RPC__out UINT *pCount);
        
        END_INTERFACE
    } IContactAggregationGroupCollectionVtbl;

    interface IContactAggregationGroupCollection
    {
        CONST_VTBL struct IContactAggregationGroupCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationGroupCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationGroupCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationGroupCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationGroupCollection_FindFirst(This,ppGroup)	\
    ( (This)->lpVtbl -> FindFirst(This,ppGroup) ) 

#define IContactAggregationGroupCollection_FindFirstByGlobalObjectId(This,pGlobalObjectId,ppGroup)	\
    ( (This)->lpVtbl -> FindFirstByGlobalObjectId(This,pGlobalObjectId,ppGroup) ) 

#define IContactAggregationGroupCollection_FindNext(This,ppGroup)	\
    ( (This)->lpVtbl -> FindNext(This,ppGroup) ) 

#define IContactAggregationGroupCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationGroupCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0007 */
/* [local] */ 

#pragma deprecated(IContactAggregationLink)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0007_v0_0_s_ifspec;

#ifndef __IContactAggregationLink_INTERFACE_DEFINED__
#define __IContactAggregationLink_INTERFACE_DEFINED__

/* interface IContactAggregationLink */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationLink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B6813323-A183-4654-8627-79B30DE3A0EC")
    IContactAggregationLink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AccountId( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAccountId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AccountId( 
            /* [in] */ __RPC__in LPCWSTR pAccountId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsLinkResolved( 
            /* [retval][out] */ __RPC__out BOOL *pIsLinkResolved) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IsLinkResolved( 
            /* [in] */ BOOL isLinkResolved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NetworkSourceIdString( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppNetworkSourceId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_NetworkSourceIdString( 
            /* [in] */ __RPC__in LPCWSTR pNetworkSourceId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RemoteObjectId( 
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppRemoteObjectId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RemoteObjectId( 
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteObjectId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ServerPerson( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppServerPersonId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ServerPerson( 
            /* [in] */ __RPC__in LPCWSTR pServerPersonId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ServerPersonBaseline( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppServerPersonId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ServerPersonBaseline( 
            /* [in] */ __RPC__in LPCWSTR pServerPersonId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SyncIdentityHash( 
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppSyncIdentityHash) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_SyncIdentityHash( 
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pSyncIdentityHash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationLinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationLink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationLink * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IContactAggregationLink * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IContactAggregationLink * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_AccountId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AccountId )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAccountId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_AccountId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AccountId )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in LPCWSTR pAccountId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppItemId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_IsLinkResolved)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsLinkResolved )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__out BOOL *pIsLinkResolved);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_IsLinkResolved)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsLinkResolved )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ BOOL isLinkResolved);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_NetworkSourceIdString)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkSourceIdString )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppNetworkSourceId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_NetworkSourceIdString)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkSourceIdString )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in LPCWSTR pNetworkSourceId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_RemoteObjectId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoteObjectId )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppRemoteObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_RemoteObjectId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RemoteObjectId )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_ServerPerson)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerPerson )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppServerPersonId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_ServerPerson)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServerPerson )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in LPCWSTR pServerPersonId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_ServerPersonBaseline)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerPersonBaseline )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppServerPersonId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_ServerPersonBaseline)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServerPersonBaseline )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in LPCWSTR pServerPersonId);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, get_SyncIdentityHash)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SyncIdentityHash )( 
            __RPC__in IContactAggregationLink * This,
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppSyncIdentityHash);
        
        DECLSPEC_XFGVIRT(IContactAggregationLink, put_SyncIdentityHash)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_SyncIdentityHash )( 
            __RPC__in IContactAggregationLink * This,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pSyncIdentityHash);
        
        END_INTERFACE
    } IContactAggregationLinkVtbl;

    interface IContactAggregationLink
    {
        CONST_VTBL struct IContactAggregationLinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationLink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationLink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationLink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationLink_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IContactAggregationLink_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IContactAggregationLink_get_AccountId(This,ppAccountId)	\
    ( (This)->lpVtbl -> get_AccountId(This,ppAccountId) ) 

#define IContactAggregationLink_put_AccountId(This,pAccountId)	\
    ( (This)->lpVtbl -> put_AccountId(This,pAccountId) ) 

#define IContactAggregationLink_get_Id(This,ppItemId)	\
    ( (This)->lpVtbl -> get_Id(This,ppItemId) ) 

#define IContactAggregationLink_get_IsLinkResolved(This,pIsLinkResolved)	\
    ( (This)->lpVtbl -> get_IsLinkResolved(This,pIsLinkResolved) ) 

#define IContactAggregationLink_put_IsLinkResolved(This,isLinkResolved)	\
    ( (This)->lpVtbl -> put_IsLinkResolved(This,isLinkResolved) ) 

#define IContactAggregationLink_get_NetworkSourceIdString(This,ppNetworkSourceId)	\
    ( (This)->lpVtbl -> get_NetworkSourceIdString(This,ppNetworkSourceId) ) 

#define IContactAggregationLink_put_NetworkSourceIdString(This,pNetworkSourceId)	\
    ( (This)->lpVtbl -> put_NetworkSourceIdString(This,pNetworkSourceId) ) 

#define IContactAggregationLink_get_RemoteObjectId(This,ppRemoteObjectId)	\
    ( (This)->lpVtbl -> get_RemoteObjectId(This,ppRemoteObjectId) ) 

#define IContactAggregationLink_put_RemoteObjectId(This,pRemoteObjectId)	\
    ( (This)->lpVtbl -> put_RemoteObjectId(This,pRemoteObjectId) ) 

#define IContactAggregationLink_get_ServerPerson(This,ppServerPersonId)	\
    ( (This)->lpVtbl -> get_ServerPerson(This,ppServerPersonId) ) 

#define IContactAggregationLink_put_ServerPerson(This,pServerPersonId)	\
    ( (This)->lpVtbl -> put_ServerPerson(This,pServerPersonId) ) 

#define IContactAggregationLink_get_ServerPersonBaseline(This,ppServerPersonId)	\
    ( (This)->lpVtbl -> get_ServerPersonBaseline(This,ppServerPersonId) ) 

#define IContactAggregationLink_put_ServerPersonBaseline(This,pServerPersonId)	\
    ( (This)->lpVtbl -> put_ServerPersonBaseline(This,pServerPersonId) ) 

#define IContactAggregationLink_get_SyncIdentityHash(This,ppSyncIdentityHash)	\
    ( (This)->lpVtbl -> get_SyncIdentityHash(This,ppSyncIdentityHash) ) 

#define IContactAggregationLink_put_SyncIdentityHash(This,pSyncIdentityHash)	\
    ( (This)->lpVtbl -> put_SyncIdentityHash(This,pSyncIdentityHash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationLink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0008 */
/* [local] */ 

#pragma deprecated(IContactAggregationLinkCollection)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0008_v0_0_s_ifspec;

#ifndef __IContactAggregationLinkCollection_INTERFACE_DEFINED__
#define __IContactAggregationLinkCollection_INTERFACE_DEFINED__

/* interface IContactAggregationLinkCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationLinkCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8BC0E93-FB55-4F28-B9FA-B1C274153292")
    IContactAggregationLinkCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFirst( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByRemoteId( 
            /* [in] */ __RPC__in LPCWSTR pSourceType,
            /* [in] */ __RPC__in LPCWSTR pAccountId,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNext( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out UINT *pCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationLinkCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationLinkCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationLinkCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationLinkCollection * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationLinkCollection, FindFirst)
        HRESULT ( STDMETHODCALLTYPE *FindFirst )( 
            __RPC__in IContactAggregationLinkCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationLinkCollection, FindFirstByRemoteId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByRemoteId )( 
            __RPC__in IContactAggregationLinkCollection * This,
            /* [in] */ __RPC__in LPCWSTR pSourceType,
            /* [in] */ __RPC__in LPCWSTR pAccountId,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pRemoteId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationLinkCollection, FindNext)
        HRESULT ( STDMETHODCALLTYPE *FindNext )( 
            __RPC__in IContactAggregationLinkCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationLink **ppServerContactLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationLinkCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IContactAggregationLinkCollection * This,
            /* [retval][out] */ __RPC__out UINT *pCount);
        
        END_INTERFACE
    } IContactAggregationLinkCollectionVtbl;

    interface IContactAggregationLinkCollection
    {
        CONST_VTBL struct IContactAggregationLinkCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationLinkCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationLinkCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationLinkCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationLinkCollection_FindFirst(This,ppServerContactLink)	\
    ( (This)->lpVtbl -> FindFirst(This,ppServerContactLink) ) 

#define IContactAggregationLinkCollection_FindFirstByRemoteId(This,pSourceType,pAccountId,pRemoteId,ppServerContactLink)	\
    ( (This)->lpVtbl -> FindFirstByRemoteId(This,pSourceType,pAccountId,pRemoteId,ppServerContactLink) ) 

#define IContactAggregationLinkCollection_FindNext(This,ppServerContactLink)	\
    ( (This)->lpVtbl -> FindNext(This,ppServerContactLink) ) 

#define IContactAggregationLinkCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationLinkCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0009 */
/* [local] */ 

#pragma deprecated(IContactAggregationServerPerson)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0009_v0_0_s_ifspec;

#ifndef __IContactAggregationServerPerson_INTERFACE_DEFINED__
#define __IContactAggregationServerPerson_INTERFACE_DEFINED__

/* interface IContactAggregationServerPerson */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationServerPerson;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7FDC3D4B-1B82-4334-85C5-25184EE5A5F2")
    IContactAggregationServerPerson : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AggregateId( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAggregateId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AggregateId( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AntiLink( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAntiLink) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AntiLink( 
            /* [in] */ __RPC__in LPCWSTR pAntiLink) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AntiLinkBaseline( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAntiLink) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AntiLinkBaseline( 
            /* [in] */ __RPC__in LPCWSTR pAntiLink) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FavoriteOrder( 
            /* [retval][out] */ __RPC__out ULONG *pFavoriteOrder) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FavoriteOrder( 
            /* [in] */ ULONG favoriteOrder) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FavoriteOrderBaseline( 
            /* [retval][out] */ __RPC__out ULONG *pFavoriteOrder) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_FavoriteOrderBaseline( 
            /* [in] */ ULONG favoriteOrder) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **pGroups) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Groups( 
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pGroups) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_GroupsBaseline( 
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppGroups) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_GroupsBaseline( 
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pGroups) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsTombstone( 
            /* [retval][out] */ __RPC__out BOOL *pIsTombstone) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IsTombstone( 
            /* [in] */ BOOL isTombstone) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LinkedAggregateId( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppLinkedAggregateId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LinkedAggregateId( 
            /* [in] */ __RPC__in LPCWSTR pLinkedAggregateId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ObjectId( 
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppObjectId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ObjectId( 
            /* [in] */ __RPC__in LPCWSTR pObjectId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationServerPersonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationServerPerson * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationServerPerson * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IContactAggregationServerPerson * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IContactAggregationServerPerson * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_AggregateId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AggregateId )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_AggregateId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AggregateId )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_AntiLink)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntiLink )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAntiLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_AntiLink)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AntiLink )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in LPCWSTR pAntiLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_AntiLinkBaseline)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntiLinkBaseline )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppAntiLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_AntiLinkBaseline)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AntiLinkBaseline )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in LPCWSTR pAntiLink);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_FavoriteOrder)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FavoriteOrder )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__out ULONG *pFavoriteOrder);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_FavoriteOrder)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FavoriteOrder )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ ULONG favoriteOrder);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_FavoriteOrderBaseline)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FavoriteOrderBaseline )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__out ULONG *pFavoriteOrder);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_FavoriteOrderBaseline)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_FavoriteOrderBaseline )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ ULONG favoriteOrder);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_Groups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **pGroups);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_Groups)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Groups )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pGroups);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_GroupsBaseline)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupsBaseline )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt CONTACT_AGGREGATION_BLOB **ppGroups);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_GroupsBaseline)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GroupsBaseline )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in const CONTACT_AGGREGATION_BLOB *pGroups);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppId);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_IsTombstone)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsTombstone )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__out BOOL *pIsTombstone);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_IsTombstone)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsTombstone )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ BOOL isTombstone);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_LinkedAggregateId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LinkedAggregateId )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppLinkedAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_LinkedAggregateId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LinkedAggregateId )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in LPCWSTR pLinkedAggregateId);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, get_ObjectId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectId )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [retval][out] */ __RPC__deref_out_opt LPWSTR *ppObjectId);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPerson, put_ObjectId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ObjectId )( 
            __RPC__in IContactAggregationServerPerson * This,
            /* [in] */ __RPC__in LPCWSTR pObjectId);
        
        END_INTERFACE
    } IContactAggregationServerPersonVtbl;

    interface IContactAggregationServerPerson
    {
        CONST_VTBL struct IContactAggregationServerPersonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationServerPerson_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationServerPerson_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationServerPerson_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationServerPerson_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IContactAggregationServerPerson_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IContactAggregationServerPerson_get_AggregateId(This,ppAggregateId)	\
    ( (This)->lpVtbl -> get_AggregateId(This,ppAggregateId) ) 

#define IContactAggregationServerPerson_put_AggregateId(This,pAggregateId)	\
    ( (This)->lpVtbl -> put_AggregateId(This,pAggregateId) ) 

#define IContactAggregationServerPerson_get_AntiLink(This,ppAntiLink)	\
    ( (This)->lpVtbl -> get_AntiLink(This,ppAntiLink) ) 

#define IContactAggregationServerPerson_put_AntiLink(This,pAntiLink)	\
    ( (This)->lpVtbl -> put_AntiLink(This,pAntiLink) ) 

#define IContactAggregationServerPerson_get_AntiLinkBaseline(This,ppAntiLink)	\
    ( (This)->lpVtbl -> get_AntiLinkBaseline(This,ppAntiLink) ) 

#define IContactAggregationServerPerson_put_AntiLinkBaseline(This,pAntiLink)	\
    ( (This)->lpVtbl -> put_AntiLinkBaseline(This,pAntiLink) ) 

#define IContactAggregationServerPerson_get_FavoriteOrder(This,pFavoriteOrder)	\
    ( (This)->lpVtbl -> get_FavoriteOrder(This,pFavoriteOrder) ) 

#define IContactAggregationServerPerson_put_FavoriteOrder(This,favoriteOrder)	\
    ( (This)->lpVtbl -> put_FavoriteOrder(This,favoriteOrder) ) 

#define IContactAggregationServerPerson_get_FavoriteOrderBaseline(This,pFavoriteOrder)	\
    ( (This)->lpVtbl -> get_FavoriteOrderBaseline(This,pFavoriteOrder) ) 

#define IContactAggregationServerPerson_put_FavoriteOrderBaseline(This,favoriteOrder)	\
    ( (This)->lpVtbl -> put_FavoriteOrderBaseline(This,favoriteOrder) ) 

#define IContactAggregationServerPerson_get_Groups(This,pGroups)	\
    ( (This)->lpVtbl -> get_Groups(This,pGroups) ) 

#define IContactAggregationServerPerson_put_Groups(This,pGroups)	\
    ( (This)->lpVtbl -> put_Groups(This,pGroups) ) 

#define IContactAggregationServerPerson_get_GroupsBaseline(This,ppGroups)	\
    ( (This)->lpVtbl -> get_GroupsBaseline(This,ppGroups) ) 

#define IContactAggregationServerPerson_put_GroupsBaseline(This,pGroups)	\
    ( (This)->lpVtbl -> put_GroupsBaseline(This,pGroups) ) 

#define IContactAggregationServerPerson_get_Id(This,ppId)	\
    ( (This)->lpVtbl -> get_Id(This,ppId) ) 

#define IContactAggregationServerPerson_get_IsTombstone(This,pIsTombstone)	\
    ( (This)->lpVtbl -> get_IsTombstone(This,pIsTombstone) ) 

#define IContactAggregationServerPerson_put_IsTombstone(This,isTombstone)	\
    ( (This)->lpVtbl -> put_IsTombstone(This,isTombstone) ) 

#define IContactAggregationServerPerson_get_LinkedAggregateId(This,ppLinkedAggregateId)	\
    ( (This)->lpVtbl -> get_LinkedAggregateId(This,ppLinkedAggregateId) ) 

#define IContactAggregationServerPerson_put_LinkedAggregateId(This,pLinkedAggregateId)	\
    ( (This)->lpVtbl -> put_LinkedAggregateId(This,pLinkedAggregateId) ) 

#define IContactAggregationServerPerson_get_ObjectId(This,ppObjectId)	\
    ( (This)->lpVtbl -> get_ObjectId(This,ppObjectId) ) 

#define IContactAggregationServerPerson_put_ObjectId(This,pObjectId)	\
    ( (This)->lpVtbl -> put_ObjectId(This,pObjectId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationServerPerson_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0010 */
/* [local] */ 

#pragma deprecated(IContactAggregationServerPersonCollection)


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0010_v0_0_s_ifspec;

#ifndef __IContactAggregationServerPersonCollection_INTERFACE_DEFINED__
#define __IContactAggregationServerPersonCollection_INTERFACE_DEFINED__

/* interface IContactAggregationServerPersonCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IContactAggregationServerPersonCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F730A4A-6604-47B6-A987-669ECF1E5751")
    IContactAggregationServerPersonCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFirst( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByServerId( 
            /* [in] */ __RPC__in LPCWSTR pServerId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByAggregateId( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFirstByLinkedAggregateId( 
            /* [in] */ __RPC__in LPCWSTR pAggregateId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNext( 
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out UINT *pCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContactAggregationServerPersonCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContactAggregationServerPersonCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContactAggregationServerPersonCollection * This);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPersonCollection, FindFirst)
        HRESULT ( STDMETHODCALLTYPE *FindFirst )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPersonCollection, FindFirstByServerId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByServerId )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [in] */ __RPC__in LPCWSTR pServerId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPersonCollection, FindFirstByAggregateId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByAggregateId )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPersonCollection, FindFirstByLinkedAggregateId)
        HRESULT ( STDMETHODCALLTYPE *FindFirstByLinkedAggregateId )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [in] */ __RPC__in LPCWSTR pAggregateId,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPersonCollection, FindNext)
        HRESULT ( STDMETHODCALLTYPE *FindNext )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IContactAggregationServerPerson **ppServerPerson);
        
        DECLSPEC_XFGVIRT(IContactAggregationServerPersonCollection, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IContactAggregationServerPersonCollection * This,
            /* [retval][out] */ __RPC__out UINT *pCount);
        
        END_INTERFACE
    } IContactAggregationServerPersonCollectionVtbl;

    interface IContactAggregationServerPersonCollection
    {
        CONST_VTBL struct IContactAggregationServerPersonCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContactAggregationServerPersonCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContactAggregationServerPersonCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContactAggregationServerPersonCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContactAggregationServerPersonCollection_FindFirst(This,ppServerPerson)	\
    ( (This)->lpVtbl -> FindFirst(This,ppServerPerson) ) 

#define IContactAggregationServerPersonCollection_FindFirstByServerId(This,pServerId,ppServerPerson)	\
    ( (This)->lpVtbl -> FindFirstByServerId(This,pServerId,ppServerPerson) ) 

#define IContactAggregationServerPersonCollection_FindFirstByAggregateId(This,pAggregateId,ppServerPerson)	\
    ( (This)->lpVtbl -> FindFirstByAggregateId(This,pAggregateId,ppServerPerson) ) 

#define IContactAggregationServerPersonCollection_FindFirstByLinkedAggregateId(This,pAggregateId,ppServerPerson)	\
    ( (This)->lpVtbl -> FindFirstByLinkedAggregateId(This,pAggregateId,ppServerPerson) ) 

#define IContactAggregationServerPersonCollection_FindNext(This,ppServerPerson)	\
    ( (This)->lpVtbl -> FindNext(This,ppServerPerson) ) 

#define IContactAggregationServerPersonCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContactAggregationServerPersonCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_contactaggregation_0000_0011 */
/* [local] */ 

#endif /* NTDDI_VERSION >=NTDDI_WIN10_RS1 */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_contactaggregation_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


