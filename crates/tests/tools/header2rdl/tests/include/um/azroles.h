

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

#ifndef __azroles_h__
#define __azroles_h__

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

#ifndef __IAzAuthorizationStore_FWD_DEFINED__
#define __IAzAuthorizationStore_FWD_DEFINED__
typedef interface IAzAuthorizationStore IAzAuthorizationStore;

#endif 	/* __IAzAuthorizationStore_FWD_DEFINED__ */


#ifndef __IAzAuthorizationStore2_FWD_DEFINED__
#define __IAzAuthorizationStore2_FWD_DEFINED__
typedef interface IAzAuthorizationStore2 IAzAuthorizationStore2;

#endif 	/* __IAzAuthorizationStore2_FWD_DEFINED__ */


#ifndef __IAzAuthorizationStore3_FWD_DEFINED__
#define __IAzAuthorizationStore3_FWD_DEFINED__
typedef interface IAzAuthorizationStore3 IAzAuthorizationStore3;

#endif 	/* __IAzAuthorizationStore3_FWD_DEFINED__ */


#ifndef __IAzApplication_FWD_DEFINED__
#define __IAzApplication_FWD_DEFINED__
typedef interface IAzApplication IAzApplication;

#endif 	/* __IAzApplication_FWD_DEFINED__ */


#ifndef __IAzApplication2_FWD_DEFINED__
#define __IAzApplication2_FWD_DEFINED__
typedef interface IAzApplication2 IAzApplication2;

#endif 	/* __IAzApplication2_FWD_DEFINED__ */


#ifndef __IAzApplications_FWD_DEFINED__
#define __IAzApplications_FWD_DEFINED__
typedef interface IAzApplications IAzApplications;

#endif 	/* __IAzApplications_FWD_DEFINED__ */


#ifndef __IAzOperation_FWD_DEFINED__
#define __IAzOperation_FWD_DEFINED__
typedef interface IAzOperation IAzOperation;

#endif 	/* __IAzOperation_FWD_DEFINED__ */


#ifndef __IAzOperations_FWD_DEFINED__
#define __IAzOperations_FWD_DEFINED__
typedef interface IAzOperations IAzOperations;

#endif 	/* __IAzOperations_FWD_DEFINED__ */


#ifndef __IAzTask_FWD_DEFINED__
#define __IAzTask_FWD_DEFINED__
typedef interface IAzTask IAzTask;

#endif 	/* __IAzTask_FWD_DEFINED__ */


#ifndef __IAzTasks_FWD_DEFINED__
#define __IAzTasks_FWD_DEFINED__
typedef interface IAzTasks IAzTasks;

#endif 	/* __IAzTasks_FWD_DEFINED__ */


#ifndef __IAzScope_FWD_DEFINED__
#define __IAzScope_FWD_DEFINED__
typedef interface IAzScope IAzScope;

#endif 	/* __IAzScope_FWD_DEFINED__ */


#ifndef __IAzScopes_FWD_DEFINED__
#define __IAzScopes_FWD_DEFINED__
typedef interface IAzScopes IAzScopes;

#endif 	/* __IAzScopes_FWD_DEFINED__ */


#ifndef __IAzApplicationGroup_FWD_DEFINED__
#define __IAzApplicationGroup_FWD_DEFINED__
typedef interface IAzApplicationGroup IAzApplicationGroup;

#endif 	/* __IAzApplicationGroup_FWD_DEFINED__ */


#ifndef __IAzApplicationGroups_FWD_DEFINED__
#define __IAzApplicationGroups_FWD_DEFINED__
typedef interface IAzApplicationGroups IAzApplicationGroups;

#endif 	/* __IAzApplicationGroups_FWD_DEFINED__ */


#ifndef __IAzRole_FWD_DEFINED__
#define __IAzRole_FWD_DEFINED__
typedef interface IAzRole IAzRole;

#endif 	/* __IAzRole_FWD_DEFINED__ */


#ifndef __IAzRoles_FWD_DEFINED__
#define __IAzRoles_FWD_DEFINED__
typedef interface IAzRoles IAzRoles;

#endif 	/* __IAzRoles_FWD_DEFINED__ */


#ifndef __IAzClientContext_FWD_DEFINED__
#define __IAzClientContext_FWD_DEFINED__
typedef interface IAzClientContext IAzClientContext;

#endif 	/* __IAzClientContext_FWD_DEFINED__ */


#ifndef __IAzClientContext2_FWD_DEFINED__
#define __IAzClientContext2_FWD_DEFINED__
typedef interface IAzClientContext2 IAzClientContext2;

#endif 	/* __IAzClientContext2_FWD_DEFINED__ */


#ifndef __IAzBizRuleContext_FWD_DEFINED__
#define __IAzBizRuleContext_FWD_DEFINED__
typedef interface IAzBizRuleContext IAzBizRuleContext;

#endif 	/* __IAzBizRuleContext_FWD_DEFINED__ */


#ifndef __IAzBizRuleParameters_FWD_DEFINED__
#define __IAzBizRuleParameters_FWD_DEFINED__
typedef interface IAzBizRuleParameters IAzBizRuleParameters;

#endif 	/* __IAzBizRuleParameters_FWD_DEFINED__ */


#ifndef __IAzBizRuleInterfaces_FWD_DEFINED__
#define __IAzBizRuleInterfaces_FWD_DEFINED__
typedef interface IAzBizRuleInterfaces IAzBizRuleInterfaces;

#endif 	/* __IAzBizRuleInterfaces_FWD_DEFINED__ */


#ifndef __IAzClientContext3_FWD_DEFINED__
#define __IAzClientContext3_FWD_DEFINED__
typedef interface IAzClientContext3 IAzClientContext3;

#endif 	/* __IAzClientContext3_FWD_DEFINED__ */


#ifndef __IAzScope2_FWD_DEFINED__
#define __IAzScope2_FWD_DEFINED__
typedef interface IAzScope2 IAzScope2;

#endif 	/* __IAzScope2_FWD_DEFINED__ */


#ifndef __IAzApplication3_FWD_DEFINED__
#define __IAzApplication3_FWD_DEFINED__
typedef interface IAzApplication3 IAzApplication3;

#endif 	/* __IAzApplication3_FWD_DEFINED__ */


#ifndef __IAzOperation2_FWD_DEFINED__
#define __IAzOperation2_FWD_DEFINED__
typedef interface IAzOperation2 IAzOperation2;

#endif 	/* __IAzOperation2_FWD_DEFINED__ */


#ifndef __IAzRoleDefinitions_FWD_DEFINED__
#define __IAzRoleDefinitions_FWD_DEFINED__
typedef interface IAzRoleDefinitions IAzRoleDefinitions;

#endif 	/* __IAzRoleDefinitions_FWD_DEFINED__ */


#ifndef __IAzRoleDefinition_FWD_DEFINED__
#define __IAzRoleDefinition_FWD_DEFINED__
typedef interface IAzRoleDefinition IAzRoleDefinition;

#endif 	/* __IAzRoleDefinition_FWD_DEFINED__ */


#ifndef __IAzRoleAssignment_FWD_DEFINED__
#define __IAzRoleAssignment_FWD_DEFINED__
typedef interface IAzRoleAssignment IAzRoleAssignment;

#endif 	/* __IAzRoleAssignment_FWD_DEFINED__ */


#ifndef __IAzRoleAssignments_FWD_DEFINED__
#define __IAzRoleAssignments_FWD_DEFINED__
typedef interface IAzRoleAssignments IAzRoleAssignments;

#endif 	/* __IAzRoleAssignments_FWD_DEFINED__ */


#ifndef __IAzPrincipalLocator_FWD_DEFINED__
#define __IAzPrincipalLocator_FWD_DEFINED__
typedef interface IAzPrincipalLocator IAzPrincipalLocator;

#endif 	/* __IAzPrincipalLocator_FWD_DEFINED__ */


#ifndef __IAzNameResolver_FWD_DEFINED__
#define __IAzNameResolver_FWD_DEFINED__
typedef interface IAzNameResolver IAzNameResolver;

#endif 	/* __IAzNameResolver_FWD_DEFINED__ */


#ifndef __IAzObjectPicker_FWD_DEFINED__
#define __IAzObjectPicker_FWD_DEFINED__
typedef interface IAzObjectPicker IAzObjectPicker;

#endif 	/* __IAzObjectPicker_FWD_DEFINED__ */


#ifndef __IAzApplicationGroup2_FWD_DEFINED__
#define __IAzApplicationGroup2_FWD_DEFINED__
typedef interface IAzApplicationGroup2 IAzApplicationGroup2;

#endif 	/* __IAzApplicationGroup2_FWD_DEFINED__ */


#ifndef __IAzTask2_FWD_DEFINED__
#define __IAzTask2_FWD_DEFINED__
typedef interface IAzTask2 IAzTask2;

#endif 	/* __IAzTask2_FWD_DEFINED__ */


#ifndef __IAzAuthorizationStore_FWD_DEFINED__
#define __IAzAuthorizationStore_FWD_DEFINED__
typedef interface IAzAuthorizationStore IAzAuthorizationStore;

#endif 	/* __IAzAuthorizationStore_FWD_DEFINED__ */


#ifndef __IAzAuthorizationStore2_FWD_DEFINED__
#define __IAzAuthorizationStore2_FWD_DEFINED__
typedef interface IAzAuthorizationStore2 IAzAuthorizationStore2;

#endif 	/* __IAzAuthorizationStore2_FWD_DEFINED__ */


#ifndef __IAzAuthorizationStore3_FWD_DEFINED__
#define __IAzAuthorizationStore3_FWD_DEFINED__
typedef interface IAzAuthorizationStore3 IAzAuthorizationStore3;

#endif 	/* __IAzAuthorizationStore3_FWD_DEFINED__ */


#ifndef __IAzApplication_FWD_DEFINED__
#define __IAzApplication_FWD_DEFINED__
typedef interface IAzApplication IAzApplication;

#endif 	/* __IAzApplication_FWD_DEFINED__ */


#ifndef __IAzApplication2_FWD_DEFINED__
#define __IAzApplication2_FWD_DEFINED__
typedef interface IAzApplication2 IAzApplication2;

#endif 	/* __IAzApplication2_FWD_DEFINED__ */


#ifndef __IAzApplication3_FWD_DEFINED__
#define __IAzApplication3_FWD_DEFINED__
typedef interface IAzApplication3 IAzApplication3;

#endif 	/* __IAzApplication3_FWD_DEFINED__ */


#ifndef __IAzApplications_FWD_DEFINED__
#define __IAzApplications_FWD_DEFINED__
typedef interface IAzApplications IAzApplications;

#endif 	/* __IAzApplications_FWD_DEFINED__ */


#ifndef __IAzOperation_FWD_DEFINED__
#define __IAzOperation_FWD_DEFINED__
typedef interface IAzOperation IAzOperation;

#endif 	/* __IAzOperation_FWD_DEFINED__ */


#ifndef __IAzOperation2_FWD_DEFINED__
#define __IAzOperation2_FWD_DEFINED__
typedef interface IAzOperation2 IAzOperation2;

#endif 	/* __IAzOperation2_FWD_DEFINED__ */


#ifndef __IAzOperations_FWD_DEFINED__
#define __IAzOperations_FWD_DEFINED__
typedef interface IAzOperations IAzOperations;

#endif 	/* __IAzOperations_FWD_DEFINED__ */


#ifndef __IAzTask_FWD_DEFINED__
#define __IAzTask_FWD_DEFINED__
typedef interface IAzTask IAzTask;

#endif 	/* __IAzTask_FWD_DEFINED__ */


#ifndef __IAzTask2_FWD_DEFINED__
#define __IAzTask2_FWD_DEFINED__
typedef interface IAzTask2 IAzTask2;

#endif 	/* __IAzTask2_FWD_DEFINED__ */


#ifndef __IAzTasks_FWD_DEFINED__
#define __IAzTasks_FWD_DEFINED__
typedef interface IAzTasks IAzTasks;

#endif 	/* __IAzTasks_FWD_DEFINED__ */


#ifndef __IAzRoleDefinition_FWD_DEFINED__
#define __IAzRoleDefinition_FWD_DEFINED__
typedef interface IAzRoleDefinition IAzRoleDefinition;

#endif 	/* __IAzRoleDefinition_FWD_DEFINED__ */


#ifndef __IAzRoleDefinitions_FWD_DEFINED__
#define __IAzRoleDefinitions_FWD_DEFINED__
typedef interface IAzRoleDefinitions IAzRoleDefinitions;

#endif 	/* __IAzRoleDefinitions_FWD_DEFINED__ */


#ifndef __IAzApplicationGroup_FWD_DEFINED__
#define __IAzApplicationGroup_FWD_DEFINED__
typedef interface IAzApplicationGroup IAzApplicationGroup;

#endif 	/* __IAzApplicationGroup_FWD_DEFINED__ */


#ifndef __IAzApplicationGroup2_FWD_DEFINED__
#define __IAzApplicationGroup2_FWD_DEFINED__
typedef interface IAzApplicationGroup2 IAzApplicationGroup2;

#endif 	/* __IAzApplicationGroup2_FWD_DEFINED__ */


#ifndef __IAzApplicationGroups_FWD_DEFINED__
#define __IAzApplicationGroups_FWD_DEFINED__
typedef interface IAzApplicationGroups IAzApplicationGroups;

#endif 	/* __IAzApplicationGroups_FWD_DEFINED__ */


#ifndef __IAzRole_FWD_DEFINED__
#define __IAzRole_FWD_DEFINED__
typedef interface IAzRole IAzRole;

#endif 	/* __IAzRole_FWD_DEFINED__ */


#ifndef __IAzRoles_FWD_DEFINED__
#define __IAzRoles_FWD_DEFINED__
typedef interface IAzRoles IAzRoles;

#endif 	/* __IAzRoles_FWD_DEFINED__ */


#ifndef __IAzRoleAssignment_FWD_DEFINED__
#define __IAzRoleAssignment_FWD_DEFINED__
typedef interface IAzRoleAssignment IAzRoleAssignment;

#endif 	/* __IAzRoleAssignment_FWD_DEFINED__ */


#ifndef __IAzRoleAssignments_FWD_DEFINED__
#define __IAzRoleAssignments_FWD_DEFINED__
typedef interface IAzRoleAssignments IAzRoleAssignments;

#endif 	/* __IAzRoleAssignments_FWD_DEFINED__ */


#ifndef __IAzScope_FWD_DEFINED__
#define __IAzScope_FWD_DEFINED__
typedef interface IAzScope IAzScope;

#endif 	/* __IAzScope_FWD_DEFINED__ */


#ifndef __IAzScope2_FWD_DEFINED__
#define __IAzScope2_FWD_DEFINED__
typedef interface IAzScope2 IAzScope2;

#endif 	/* __IAzScope2_FWD_DEFINED__ */


#ifndef __IAzScopes_FWD_DEFINED__
#define __IAzScopes_FWD_DEFINED__
typedef interface IAzScopes IAzScopes;

#endif 	/* __IAzScopes_FWD_DEFINED__ */


#ifndef __IAzClientContext_FWD_DEFINED__
#define __IAzClientContext_FWD_DEFINED__
typedef interface IAzClientContext IAzClientContext;

#endif 	/* __IAzClientContext_FWD_DEFINED__ */


#ifndef __IAzClientContext2_FWD_DEFINED__
#define __IAzClientContext2_FWD_DEFINED__
typedef interface IAzClientContext2 IAzClientContext2;

#endif 	/* __IAzClientContext2_FWD_DEFINED__ */


#ifndef __IAzClientContext3_FWD_DEFINED__
#define __IAzClientContext3_FWD_DEFINED__
typedef interface IAzClientContext3 IAzClientContext3;

#endif 	/* __IAzClientContext3_FWD_DEFINED__ */


#ifndef __IAzBizRuleContext_FWD_DEFINED__
#define __IAzBizRuleContext_FWD_DEFINED__
typedef interface IAzBizRuleContext IAzBizRuleContext;

#endif 	/* __IAzBizRuleContext_FWD_DEFINED__ */


#ifndef __IAzBizRuleParameters_FWD_DEFINED__
#define __IAzBizRuleParameters_FWD_DEFINED__
typedef interface IAzBizRuleParameters IAzBizRuleParameters;

#endif 	/* __IAzBizRuleParameters_FWD_DEFINED__ */


#ifndef __IAzBizRuleInterfaces_FWD_DEFINED__
#define __IAzBizRuleInterfaces_FWD_DEFINED__
typedef interface IAzBizRuleInterfaces IAzBizRuleInterfaces;

#endif 	/* __IAzBizRuleInterfaces_FWD_DEFINED__ */


#ifndef __IAzPrincipalLocator_FWD_DEFINED__
#define __IAzPrincipalLocator_FWD_DEFINED__
typedef interface IAzPrincipalLocator IAzPrincipalLocator;

#endif 	/* __IAzPrincipalLocator_FWD_DEFINED__ */


#ifndef __IAzNameResolver_FWD_DEFINED__
#define __IAzNameResolver_FWD_DEFINED__
typedef interface IAzNameResolver IAzNameResolver;

#endif 	/* __IAzNameResolver_FWD_DEFINED__ */


#ifndef __IAzObjectPicker_FWD_DEFINED__
#define __IAzObjectPicker_FWD_DEFINED__
typedef interface IAzObjectPicker IAzObjectPicker;

#endif 	/* __IAzObjectPicker_FWD_DEFINED__ */


#ifndef __AzAuthorizationStore_FWD_DEFINED__
#define __AzAuthorizationStore_FWD_DEFINED__

#ifdef __cplusplus
typedef class AzAuthorizationStore AzAuthorizationStore;
#else
typedef struct AzAuthorizationStore AzAuthorizationStore;
#endif /* __cplusplus */

#endif 	/* __AzAuthorizationStore_FWD_DEFINED__ */


#ifndef __AzBizRuleContext_FWD_DEFINED__
#define __AzBizRuleContext_FWD_DEFINED__

#ifdef __cplusplus
typedef class AzBizRuleContext AzBizRuleContext;
#else
typedef struct AzBizRuleContext AzBizRuleContext;
#endif /* __cplusplus */

#endif 	/* __AzBizRuleContext_FWD_DEFINED__ */


#ifndef __AzPrincipalLocator_FWD_DEFINED__
#define __AzPrincipalLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class AzPrincipalLocator AzPrincipalLocator;
#else
typedef struct AzPrincipalLocator AzPrincipalLocator;
#endif /* __cplusplus */

#endif 	/* __AzPrincipalLocator_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_azroles_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//
// IAzAuthorizationStore interface id
//
//edbd9ca9-9b82-4f6a-9e8b-98301e450f14
DEFINE_GUID(IID_IAzAuthorizationStore, 0xedbd9ca9, 0x9b82, 0x4f6a, 0x9e, 0x8b, 0x98, 0x30, 0x1e, 0x45, 0x0f, 0x14);
//
// IAzAuthorizationStore2 interface id
//
//b11e5584-d577-4273-b6c5-0973e0f8e80d
DEFINE_GUID(IID_IAzAuthorizationStore2,0xb11e5584, 0xd577, 0x4273, 0xb6, 0xc5, 0x9, 0x73, 0xe0, 0xf8, 0xe8, 0xd);
//
// IAzAuthorizationStore3 interface id
//
//abc08425-0c86-4fa0-9be3-7189956c926e
DEFINE_GUID(IID_IAzAuthorizationStore3,0xabc08425, 0x0c86, 0x4fa0, 0x9b, 0xe3, 0x71, 0x89, 0x95, 0x6c, 0x92, 0x6e);

//
// AzAuthorizationStore class id
//
//b2bcff59-a757-4b0b-a1bc-ea69981da69e
DEFINE_GUID(CLSID_AzAuthorizationStore, 0xb2bcff59, 0xa757, 0x4b0b, 0xa1, 0xbc, 0xea, 0x69, 0x98, 0x1d, 0xa6, 0x9e);

//
// IAzBizRuleContext interface id
//
//e192f17d-d59f-455e-a152-940316cd77b2
DEFINE_GUID(IID_IAzBizRuleContext, 0xe192f17d, 0xd59f, 0x455e, 0xa1, 0x52, 0x94, 0x03, 0x16, 0xcd, 0x77, 0xb2);

//
// AzBizRuleContext class id
//
//5c2dc96f-8d51-434b-b33c-379bccae77c3
DEFINE_GUID(CLSID_AzBizRuleContext, 0x5c2dc96f, 0x8d51, 0x434b, 0xb3, 0x3c, 0x37, 0x9b, 0xcc, 0xae, 0x77, 0xc3);

//
// AzPrincipalLocator class id
//
//483afb5d-70df-4e16-abdc-a1de4d015a3e
DEFINE_GUID(CLSID_AzPrincipalLocator, 0x483afb5d, 0x70df, 0x4e16, 0xab, 0xdc, 0xa1, 0xde, 0x4d, 0x01, 0x5a, 0x3e);

//
// IAzPrincipalLocator interface id
//
//e5c3507d-ad6a-4992-9c7f-74ab480b44cc
DEFINE_GUID(IID_IAzPrincipalLocator, 0xe5c3507d, 0xad6a, 0x4992, 0x9c, 0x7f, 0x74, 0xab, 0x48, 0x0b, 0x44, 0xcc);

//
// IAzNameResolver interface id
//
//504d0f15-73e2-43df-a870-a64f40714f53
DEFINE_GUID(IID_IAzNameResolver, 0x504d0f15, 0x73e2, 0x43df, 0xa8, 0x70, 0xa6, 0x4f, 0x40, 0x71, 0x4f, 0x53);

//
// IAzObjectPicker interface id
//
//63130a48-699a-42d8-bf01-c62ac3fb79f9
DEFINE_GUID(IID_IAzObjectPicker, 0x63130a48, 0x699a, 0x42d8, 0xbf, 0x01, 0xc6, 0x2a, 0xc3, 0xfb, 0x79, 0xf9);

//
// IAzApplication3 interface id
//
//181c845e-7196-4a7d-ac2e-020c0bb7a303
DEFINE_GUID(IID_IAzApplication3, 0x181c845e, 0x7196, 0x4a7d, 0xac, 0x2e, 0x02, 0x0c, 0x0b, 0xb7, 0xa3, 0x03);






























extern RPC_IF_HANDLE __MIDL_itf_azroles_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_azroles_0000_0000_v0_0_s_ifspec;

#ifndef __IAzAuthorizationStore_INTERFACE_DEFINED__
#define __IAzAuthorizationStore_INTERFACE_DEFINED__

/* interface IAzAuthorizationStore */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzAuthorizationStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("edbd9ca9-9b82-4f6a-9e8b-98301e450f14")
    IAzAuthorizationStore : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplicationData( 
            /* [in] */ __RPC__in BSTR bstrApplicationData) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DomainTimeout( 
            /* [retval][out] */ __RPC__out LONG *plProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DomainTimeout( 
            /* [in] */ LONG lProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ScriptEngineTimeout( 
            /* [retval][out] */ __RPC__out LONG *plProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ScriptEngineTimeout( 
            /* [in] */ LONG lProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MaxScriptEngines( 
            /* [retval][out] */ __RPC__out LONG *plProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MaxScriptEngines( 
            /* [in] */ LONG lProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_GenerateAudits( 
            /* [retval][out] */ __RPC__out BOOL *pbProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_GenerateAudits( 
            /* [in] */ BOOL bProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyAdministrators( 
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyReaders( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyAdministrator( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyAdministrator( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyReader( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyReader( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrPolicyURL,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateCache( 
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Applications( 
            /* [retval][out] */ __RPC__deref_out_opt IAzApplications **ppAppCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DelegatedPolicyUsers( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDelegatedPolicyUser( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteDelegatedPolicyUser( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TargetMachine( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTargetMachine) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplyStoreSacl( 
            /* [retval][out] */ __RPC__out BOOL *pbApplyStoreSacl) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplyStoreSacl( 
            /* [in] */ BOOL bApplyStoreSacl) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyAdministratorsName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyReadersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyAdministratorName( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyAdministratorName( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyReaderName( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyReaderName( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DelegatedPolicyUsersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDelegatedPolicyUserName( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteDelegatedPolicyUserName( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [in] */ LONG lFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzAuthorizationStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzAuthorizationStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzAuthorizationStore * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzAuthorizationStore * This,
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
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DomainTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainTimeout )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_DomainTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DomainTimeout )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ScriptEngineTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScriptEngineTimeout )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ScriptEngineTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScriptEngineTimeout )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_MaxScriptEngines)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxScriptEngines )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_MaxScriptEngines)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxScriptEngines )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_GenerateAudits)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerateAudits )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_GenerateAudits)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenerateAudits )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrPolicyURL,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, UpdateCache)
        HRESULT ( STDMETHODCALLTYPE *UpdateCache )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Applications)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Applications )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplications **ppAppCollection);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, OpenApplication)
        HRESULT ( STDMETHODCALLTYPE *OpenApplication )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CreateApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteApplication)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplication )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DelegatedPolicyUsers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsers )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUser )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUser )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_TargetMachine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetMachine )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTargetMachine);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplyStoreSacl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplyStoreSacl )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out BOOL *pbApplyStoreSacl);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ApplyStoreSacl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplyStoreSacl )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ BOOL bApplyStoreSacl);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DelegatedPolicyUsersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsersName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUserName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUserName )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CloseApplication)
        HRESULT ( STDMETHODCALLTYPE *CloseApplication )( 
            __RPC__in IAzAuthorizationStore * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [in] */ LONG lFlag);
        
        END_INTERFACE
    } IAzAuthorizationStoreVtbl;

    interface IAzAuthorizationStore
    {
        CONST_VTBL struct IAzAuthorizationStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzAuthorizationStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzAuthorizationStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzAuthorizationStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzAuthorizationStore_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzAuthorizationStore_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzAuthorizationStore_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzAuthorizationStore_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzAuthorizationStore_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzAuthorizationStore_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzAuthorizationStore_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzAuthorizationStore_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzAuthorizationStore_get_DomainTimeout(This,plProp)	\
    ( (This)->lpVtbl -> get_DomainTimeout(This,plProp) ) 

#define IAzAuthorizationStore_put_DomainTimeout(This,lProp)	\
    ( (This)->lpVtbl -> put_DomainTimeout(This,lProp) ) 

#define IAzAuthorizationStore_get_ScriptEngineTimeout(This,plProp)	\
    ( (This)->lpVtbl -> get_ScriptEngineTimeout(This,plProp) ) 

#define IAzAuthorizationStore_put_ScriptEngineTimeout(This,lProp)	\
    ( (This)->lpVtbl -> put_ScriptEngineTimeout(This,lProp) ) 

#define IAzAuthorizationStore_get_MaxScriptEngines(This,plProp)	\
    ( (This)->lpVtbl -> get_MaxScriptEngines(This,plProp) ) 

#define IAzAuthorizationStore_put_MaxScriptEngines(This,lProp)	\
    ( (This)->lpVtbl -> put_MaxScriptEngines(This,lProp) ) 

#define IAzAuthorizationStore_get_GenerateAudits(This,pbProp)	\
    ( (This)->lpVtbl -> get_GenerateAudits(This,pbProp) ) 

#define IAzAuthorizationStore_put_GenerateAudits(This,bProp)	\
    ( (This)->lpVtbl -> put_GenerateAudits(This,bProp) ) 

#define IAzAuthorizationStore_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzAuthorizationStore_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzAuthorizationStore_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzAuthorizationStore_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzAuthorizationStore_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore_Initialize(This,lFlags,bstrPolicyURL,varReserved)	\
    ( (This)->lpVtbl -> Initialize(This,lFlags,bstrPolicyURL,varReserved) ) 

#define IAzAuthorizationStore_UpdateCache(This,varReserved)	\
    ( (This)->lpVtbl -> UpdateCache(This,varReserved) ) 

#define IAzAuthorizationStore_Delete(This,varReserved)	\
    ( (This)->lpVtbl -> Delete(This,varReserved) ) 

#define IAzAuthorizationStore_get_Applications(This,ppAppCollection)	\
    ( (This)->lpVtbl -> get_Applications(This,ppAppCollection) ) 

#define IAzAuthorizationStore_OpenApplication(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> OpenApplication(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore_CreateApplication(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> CreateApplication(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore_DeleteApplication(This,bstrApplicationName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplication(This,bstrApplicationName,varReserved) ) 

#define IAzAuthorizationStore_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzAuthorizationStore_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzAuthorizationStore_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzAuthorizationStore_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzAuthorizationStore_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzAuthorizationStore_get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers) ) 

#define IAzAuthorizationStore_AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore_DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore_get_TargetMachine(This,pbstrTargetMachine)	\
    ( (This)->lpVtbl -> get_TargetMachine(This,pbstrTargetMachine) ) 

#define IAzAuthorizationStore_get_ApplyStoreSacl(This,pbApplyStoreSacl)	\
    ( (This)->lpVtbl -> get_ApplyStoreSacl(This,pbApplyStoreSacl) ) 

#define IAzAuthorizationStore_put_ApplyStoreSacl(This,bApplyStoreSacl)	\
    ( (This)->lpVtbl -> put_ApplyStoreSacl(This,bApplyStoreSacl) ) 

#define IAzAuthorizationStore_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzAuthorizationStore_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzAuthorizationStore_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore_get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers) ) 

#define IAzAuthorizationStore_AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore_DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore_CloseApplication(This,bstrApplicationName,lFlag)	\
    ( (This)->lpVtbl -> CloseApplication(This,bstrApplicationName,lFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzAuthorizationStore_INTERFACE_DEFINED__ */


#ifndef __IAzAuthorizationStore2_INTERFACE_DEFINED__
#define __IAzAuthorizationStore2_INTERFACE_DEFINED__

/* interface IAzAuthorizationStore2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzAuthorizationStore2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b11e5584-d577-4273-b6c5-0973e0f8e80d")
    IAzAuthorizationStore2 : public IAzAuthorizationStore
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenApplication2( 
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication2 **ppApplication) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplication2( 
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication2 **ppApplication) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzAuthorizationStore2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzAuthorizationStore2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzAuthorizationStore2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzAuthorizationStore2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DomainTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainTimeout )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_DomainTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DomainTimeout )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ScriptEngineTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScriptEngineTimeout )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ScriptEngineTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScriptEngineTimeout )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_MaxScriptEngines)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxScriptEngines )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_MaxScriptEngines)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxScriptEngines )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_GenerateAudits)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerateAudits )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_GenerateAudits)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenerateAudits )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrPolicyURL,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, UpdateCache)
        HRESULT ( STDMETHODCALLTYPE *UpdateCache )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Applications)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Applications )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplications **ppAppCollection);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, OpenApplication)
        HRESULT ( STDMETHODCALLTYPE *OpenApplication )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CreateApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteApplication)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplication )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DelegatedPolicyUsers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsers )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUser )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUser )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_TargetMachine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetMachine )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTargetMachine);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplyStoreSacl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplyStoreSacl )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out BOOL *pbApplyStoreSacl);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ApplyStoreSacl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplyStoreSacl )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ BOOL bApplyStoreSacl);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DelegatedPolicyUsersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsersName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUserName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUserName )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CloseApplication)
        HRESULT ( STDMETHODCALLTYPE *CloseApplication )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [in] */ LONG lFlag);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore2, OpenApplication2)
        HRESULT ( STDMETHODCALLTYPE *OpenApplication2 )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication2 **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore2, CreateApplication2)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication2 )( 
            __RPC__in IAzAuthorizationStore2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication2 **ppApplication);
        
        END_INTERFACE
    } IAzAuthorizationStore2Vtbl;

    interface IAzAuthorizationStore2
    {
        CONST_VTBL struct IAzAuthorizationStore2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzAuthorizationStore2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzAuthorizationStore2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzAuthorizationStore2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzAuthorizationStore2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzAuthorizationStore2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzAuthorizationStore2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzAuthorizationStore2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzAuthorizationStore2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzAuthorizationStore2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzAuthorizationStore2_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzAuthorizationStore2_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzAuthorizationStore2_get_DomainTimeout(This,plProp)	\
    ( (This)->lpVtbl -> get_DomainTimeout(This,plProp) ) 

#define IAzAuthorizationStore2_put_DomainTimeout(This,lProp)	\
    ( (This)->lpVtbl -> put_DomainTimeout(This,lProp) ) 

#define IAzAuthorizationStore2_get_ScriptEngineTimeout(This,plProp)	\
    ( (This)->lpVtbl -> get_ScriptEngineTimeout(This,plProp) ) 

#define IAzAuthorizationStore2_put_ScriptEngineTimeout(This,lProp)	\
    ( (This)->lpVtbl -> put_ScriptEngineTimeout(This,lProp) ) 

#define IAzAuthorizationStore2_get_MaxScriptEngines(This,plProp)	\
    ( (This)->lpVtbl -> get_MaxScriptEngines(This,plProp) ) 

#define IAzAuthorizationStore2_put_MaxScriptEngines(This,lProp)	\
    ( (This)->lpVtbl -> put_MaxScriptEngines(This,lProp) ) 

#define IAzAuthorizationStore2_get_GenerateAudits(This,pbProp)	\
    ( (This)->lpVtbl -> get_GenerateAudits(This,pbProp) ) 

#define IAzAuthorizationStore2_put_GenerateAudits(This,bProp)	\
    ( (This)->lpVtbl -> put_GenerateAudits(This,bProp) ) 

#define IAzAuthorizationStore2_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzAuthorizationStore2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzAuthorizationStore2_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore2_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore2_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore2_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzAuthorizationStore2_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzAuthorizationStore2_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore2_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore2_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore2_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore2_Initialize(This,lFlags,bstrPolicyURL,varReserved)	\
    ( (This)->lpVtbl -> Initialize(This,lFlags,bstrPolicyURL,varReserved) ) 

#define IAzAuthorizationStore2_UpdateCache(This,varReserved)	\
    ( (This)->lpVtbl -> UpdateCache(This,varReserved) ) 

#define IAzAuthorizationStore2_Delete(This,varReserved)	\
    ( (This)->lpVtbl -> Delete(This,varReserved) ) 

#define IAzAuthorizationStore2_get_Applications(This,ppAppCollection)	\
    ( (This)->lpVtbl -> get_Applications(This,ppAppCollection) ) 

#define IAzAuthorizationStore2_OpenApplication(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> OpenApplication(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore2_CreateApplication(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> CreateApplication(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore2_DeleteApplication(This,bstrApplicationName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplication(This,bstrApplicationName,varReserved) ) 

#define IAzAuthorizationStore2_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzAuthorizationStore2_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzAuthorizationStore2_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzAuthorizationStore2_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzAuthorizationStore2_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzAuthorizationStore2_get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers) ) 

#define IAzAuthorizationStore2_AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore2_DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore2_get_TargetMachine(This,pbstrTargetMachine)	\
    ( (This)->lpVtbl -> get_TargetMachine(This,pbstrTargetMachine) ) 

#define IAzAuthorizationStore2_get_ApplyStoreSacl(This,pbApplyStoreSacl)	\
    ( (This)->lpVtbl -> get_ApplyStoreSacl(This,pbApplyStoreSacl) ) 

#define IAzAuthorizationStore2_put_ApplyStoreSacl(This,bApplyStoreSacl)	\
    ( (This)->lpVtbl -> put_ApplyStoreSacl(This,bApplyStoreSacl) ) 

#define IAzAuthorizationStore2_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzAuthorizationStore2_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzAuthorizationStore2_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore2_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore2_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore2_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore2_get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers) ) 

#define IAzAuthorizationStore2_AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore2_DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore2_CloseApplication(This,bstrApplicationName,lFlag)	\
    ( (This)->lpVtbl -> CloseApplication(This,bstrApplicationName,lFlag) ) 


#define IAzAuthorizationStore2_OpenApplication2(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> OpenApplication2(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore2_CreateApplication2(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> CreateApplication2(This,bstrApplicationName,varReserved,ppApplication) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzAuthorizationStore2_INTERFACE_DEFINED__ */


#ifndef __IAzAuthorizationStore3_INTERFACE_DEFINED__
#define __IAzAuthorizationStore3_INTERFACE_DEFINED__

/* interface IAzAuthorizationStore3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzAuthorizationStore3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("abc08425-0c86-4fa0-9be3-7189956c926e")
    IAzAuthorizationStore3 : public IAzAuthorizationStore2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsUpdateNeeded( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsUpdateNeeded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BizruleGroupSupported( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSupported) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpgradeStoresFunctionalLevel( 
            /* [in] */ LONG lFunctionalLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsFunctionalLevelUpgradeSupported( 
            /* [in] */ LONG lFunctionalLevel,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSupported) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSchemaVersion( 
            /* [out] */ __RPC__out LONG *plMajorVersion,
            /* [out] */ __RPC__out LONG *plMinorVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzAuthorizationStore3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzAuthorizationStore3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzAuthorizationStore3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzAuthorizationStore3 * This,
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
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DomainTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainTimeout )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_DomainTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DomainTimeout )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ScriptEngineTimeout)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScriptEngineTimeout )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ScriptEngineTimeout)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScriptEngineTimeout )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_MaxScriptEngines)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxScriptEngines )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_MaxScriptEngines)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxScriptEngines )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_GenerateAudits)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerateAudits )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_GenerateAudits)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenerateAudits )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lFlags,
            /* [in] */ __RPC__in BSTR bstrPolicyURL,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, UpdateCache)
        HRESULT ( STDMETHODCALLTYPE *UpdateCache )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_Applications)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Applications )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplications **ppAppCollection);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, OpenApplication)
        HRESULT ( STDMETHODCALLTYPE *OpenApplication )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CreateApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteApplication)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplication )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DelegatedPolicyUsers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsers )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUser )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUser )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_TargetMachine)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetMachine )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTargetMachine);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_ApplyStoreSacl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplyStoreSacl )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out BOOL *pbApplyStoreSacl);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, put_ApplyStoreSacl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplyStoreSacl )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ BOOL bApplyStoreSacl);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, get_DelegatedPolicyUsersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsersName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, AddDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUserName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, DeleteDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUserName )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore, CloseApplication)
        HRESULT ( STDMETHODCALLTYPE *CloseApplication )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [in] */ LONG lFlag);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore2, OpenApplication2)
        HRESULT ( STDMETHODCALLTYPE *OpenApplication2 )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication2 **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore2, CreateApplication2)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication2 )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplication2 **ppApplication);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore3, IsUpdateNeeded)
        HRESULT ( STDMETHODCALLTYPE *IsUpdateNeeded )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsUpdateNeeded);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore3, BizruleGroupSupported)
        HRESULT ( STDMETHODCALLTYPE *BizruleGroupSupported )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSupported);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore3, UpgradeStoresFunctionalLevel)
        HRESULT ( STDMETHODCALLTYPE *UpgradeStoresFunctionalLevel )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lFunctionalLevel);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore3, IsFunctionalLevelUpgradeSupported)
        HRESULT ( STDMETHODCALLTYPE *IsFunctionalLevelUpgradeSupported )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [in] */ LONG lFunctionalLevel,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSupported);
        
        DECLSPEC_XFGVIRT(IAzAuthorizationStore3, GetSchemaVersion)
        HRESULT ( STDMETHODCALLTYPE *GetSchemaVersion )( 
            __RPC__in IAzAuthorizationStore3 * This,
            /* [out] */ __RPC__out LONG *plMajorVersion,
            /* [out] */ __RPC__out LONG *plMinorVersion);
        
        END_INTERFACE
    } IAzAuthorizationStore3Vtbl;

    interface IAzAuthorizationStore3
    {
        CONST_VTBL struct IAzAuthorizationStore3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzAuthorizationStore3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzAuthorizationStore3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzAuthorizationStore3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzAuthorizationStore3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzAuthorizationStore3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzAuthorizationStore3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzAuthorizationStore3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzAuthorizationStore3_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzAuthorizationStore3_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzAuthorizationStore3_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzAuthorizationStore3_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzAuthorizationStore3_get_DomainTimeout(This,plProp)	\
    ( (This)->lpVtbl -> get_DomainTimeout(This,plProp) ) 

#define IAzAuthorizationStore3_put_DomainTimeout(This,lProp)	\
    ( (This)->lpVtbl -> put_DomainTimeout(This,lProp) ) 

#define IAzAuthorizationStore3_get_ScriptEngineTimeout(This,plProp)	\
    ( (This)->lpVtbl -> get_ScriptEngineTimeout(This,plProp) ) 

#define IAzAuthorizationStore3_put_ScriptEngineTimeout(This,lProp)	\
    ( (This)->lpVtbl -> put_ScriptEngineTimeout(This,lProp) ) 

#define IAzAuthorizationStore3_get_MaxScriptEngines(This,plProp)	\
    ( (This)->lpVtbl -> get_MaxScriptEngines(This,plProp) ) 

#define IAzAuthorizationStore3_put_MaxScriptEngines(This,lProp)	\
    ( (This)->lpVtbl -> put_MaxScriptEngines(This,lProp) ) 

#define IAzAuthorizationStore3_get_GenerateAudits(This,pbProp)	\
    ( (This)->lpVtbl -> get_GenerateAudits(This,pbProp) ) 

#define IAzAuthorizationStore3_put_GenerateAudits(This,bProp)	\
    ( (This)->lpVtbl -> put_GenerateAudits(This,bProp) ) 

#define IAzAuthorizationStore3_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzAuthorizationStore3_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzAuthorizationStore3_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore3_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore3_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzAuthorizationStore3_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzAuthorizationStore3_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzAuthorizationStore3_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore3_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore3_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore3_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore3_Initialize(This,lFlags,bstrPolicyURL,varReserved)	\
    ( (This)->lpVtbl -> Initialize(This,lFlags,bstrPolicyURL,varReserved) ) 

#define IAzAuthorizationStore3_UpdateCache(This,varReserved)	\
    ( (This)->lpVtbl -> UpdateCache(This,varReserved) ) 

#define IAzAuthorizationStore3_Delete(This,varReserved)	\
    ( (This)->lpVtbl -> Delete(This,varReserved) ) 

#define IAzAuthorizationStore3_get_Applications(This,ppAppCollection)	\
    ( (This)->lpVtbl -> get_Applications(This,ppAppCollection) ) 

#define IAzAuthorizationStore3_OpenApplication(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> OpenApplication(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore3_CreateApplication(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> CreateApplication(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore3_DeleteApplication(This,bstrApplicationName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplication(This,bstrApplicationName,varReserved) ) 

#define IAzAuthorizationStore3_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzAuthorizationStore3_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzAuthorizationStore3_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzAuthorizationStore3_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzAuthorizationStore3_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzAuthorizationStore3_get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers) ) 

#define IAzAuthorizationStore3_AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore3_DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore3_get_TargetMachine(This,pbstrTargetMachine)	\
    ( (This)->lpVtbl -> get_TargetMachine(This,pbstrTargetMachine) ) 

#define IAzAuthorizationStore3_get_ApplyStoreSacl(This,pbApplyStoreSacl)	\
    ( (This)->lpVtbl -> get_ApplyStoreSacl(This,pbApplyStoreSacl) ) 

#define IAzAuthorizationStore3_put_ApplyStoreSacl(This,bApplyStoreSacl)	\
    ( (This)->lpVtbl -> put_ApplyStoreSacl(This,bApplyStoreSacl) ) 

#define IAzAuthorizationStore3_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzAuthorizationStore3_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzAuthorizationStore3_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore3_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzAuthorizationStore3_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore3_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzAuthorizationStore3_get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers) ) 

#define IAzAuthorizationStore3_AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore3_DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzAuthorizationStore3_CloseApplication(This,bstrApplicationName,lFlag)	\
    ( (This)->lpVtbl -> CloseApplication(This,bstrApplicationName,lFlag) ) 


#define IAzAuthorizationStore3_OpenApplication2(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> OpenApplication2(This,bstrApplicationName,varReserved,ppApplication) ) 

#define IAzAuthorizationStore3_CreateApplication2(This,bstrApplicationName,varReserved,ppApplication)	\
    ( (This)->lpVtbl -> CreateApplication2(This,bstrApplicationName,varReserved,ppApplication) ) 


#define IAzAuthorizationStore3_IsUpdateNeeded(This,pbIsUpdateNeeded)	\
    ( (This)->lpVtbl -> IsUpdateNeeded(This,pbIsUpdateNeeded) ) 

#define IAzAuthorizationStore3_BizruleGroupSupported(This,pbSupported)	\
    ( (This)->lpVtbl -> BizruleGroupSupported(This,pbSupported) ) 

#define IAzAuthorizationStore3_UpgradeStoresFunctionalLevel(This,lFunctionalLevel)	\
    ( (This)->lpVtbl -> UpgradeStoresFunctionalLevel(This,lFunctionalLevel) ) 

#define IAzAuthorizationStore3_IsFunctionalLevelUpgradeSupported(This,lFunctionalLevel,pbSupported)	\
    ( (This)->lpVtbl -> IsFunctionalLevelUpgradeSupported(This,lFunctionalLevel,pbSupported) ) 

#define IAzAuthorizationStore3_GetSchemaVersion(This,plMajorVersion,plMinorVersion)	\
    ( (This)->lpVtbl -> GetSchemaVersion(This,plMajorVersion,plMinorVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzAuthorizationStore3_INTERFACE_DEFINED__ */


#ifndef __IAzApplication_INTERFACE_DEFINED__
#define __IAzApplication_INTERFACE_DEFINED__

/* interface IAzApplication */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("987bc7c7-b813-4d27-bede-6ba5ae867e95")
    IAzApplication : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplicationData( 
            /* [in] */ __RPC__in BSTR bstrApplicationData) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AuthzInterfaceClsid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AuthzInterfaceClsid( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Version( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_GenerateAudits( 
            /* [retval][out] */ __RPC__out BOOL *pbProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_GenerateAudits( 
            /* [in] */ BOOL bProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplyStoreSacl( 
            /* [retval][out] */ __RPC__out BOOL *pbProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplyStoreSacl( 
            /* [in] */ BOOL bProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyAdministrators( 
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyReaders( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyAdministrator( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyAdministrator( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyReader( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyReader( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Scopes( 
            /* [retval][out] */ __RPC__deref_out_opt IAzScopes **ppScopeCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenScope( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateScope( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteScope( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Operations( 
            /* [retval][out] */ __RPC__deref_out_opt IAzOperations **ppOperationCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenOperation( 
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateOperation( 
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteOperation( 
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Tasks( 
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenTask( 
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTask( 
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteTask( 
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Roles( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenRole( 
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRole( 
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRole( 
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeClientContextFromToken( 
            /* [in] */ ULONGLONG ullTokenHandle,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeClientContextFromName( 
            /* [in] */ __RPC__in BSTR ClientName,
            /* [defaultvalue][in] */ __RPC__in BSTR DomainName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DelegatedPolicyUsers( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDelegatedPolicyUser( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteDelegatedPolicyUser( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeClientContextFromStringSid( 
            /* [in] */ __RPC__in BSTR SidString,
            /* [in] */ LONG lOptions,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyAdministratorsName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyReadersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyAdministratorName( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyAdministratorName( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyReaderName( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyReaderName( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DelegatedPolicyUsersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDelegatedPolicyUserName( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteDelegatedPolicyUserName( 
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplication * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplication * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplication * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplication * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_AuthzInterfaceClsid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuthzInterfaceClsid )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_AuthzInterfaceClsid)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AuthzInterfaceClsid )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Version)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_GenerateAudits)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerateAudits )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_GenerateAudits)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenerateAudits )( 
            __RPC__in IAzApplication * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplyStoreSacl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplyStoreSacl )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_ApplyStoreSacl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplyStoreSacl )( 
            __RPC__in IAzApplication * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzApplication * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzApplication * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Scopes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Scopes )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzScopes **ppScopeCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenScope)
        HRESULT ( STDMETHODCALLTYPE *OpenScope )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateScope)
        HRESULT ( STDMETHODCALLTYPE *CreateScope )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteScope)
        HRESULT ( STDMETHODCALLTYPE *DeleteScope )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperations **ppOperationCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenOperation)
        HRESULT ( STDMETHODCALLTYPE *OpenOperation )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateOperation)
        HRESULT ( STDMETHODCALLTYPE *CreateOperation )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenTask)
        HRESULT ( STDMETHODCALLTYPE *OpenTask )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateTask)
        HRESULT ( STDMETHODCALLTYPE *CreateTask )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Roles)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Roles )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenRole)
        HRESULT ( STDMETHODCALLTYPE *OpenRole )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateRole)
        HRESULT ( STDMETHODCALLTYPE *CreateRole )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteRole)
        HRESULT ( STDMETHODCALLTYPE *DeleteRole )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromToken)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromToken )( 
            __RPC__in IAzApplication * This,
            /* [in] */ ULONGLONG ullTokenHandle,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzApplication * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzApplication * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzApplication * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromName)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR ClientName,
            /* [defaultvalue][in] */ __RPC__in BSTR DomainName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_DelegatedPolicyUsers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsers )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUser )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUser )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromStringSid)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromStringSid )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR SidString,
            /* [in] */ LONG lOptions,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_DelegatedPolicyUsersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsersName )( 
            __RPC__in IAzApplication * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUserName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUserName )( 
            __RPC__in IAzApplication * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        END_INTERFACE
    } IAzApplicationVtbl;

    interface IAzApplication
    {
        CONST_VTBL struct IAzApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplication_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplication_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplication_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplication_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplication_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzApplication_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzApplication_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzApplication_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzApplication_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzApplication_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzApplication_get_AuthzInterfaceClsid(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_AuthzInterfaceClsid(This,pbstrProp) ) 

#define IAzApplication_put_AuthzInterfaceClsid(This,bstrProp)	\
    ( (This)->lpVtbl -> put_AuthzInterfaceClsid(This,bstrProp) ) 

#define IAzApplication_get_Version(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_Version(This,pbstrProp) ) 

#define IAzApplication_put_Version(This,bstrProp)	\
    ( (This)->lpVtbl -> put_Version(This,bstrProp) ) 

#define IAzApplication_get_GenerateAudits(This,pbProp)	\
    ( (This)->lpVtbl -> get_GenerateAudits(This,pbProp) ) 

#define IAzApplication_put_GenerateAudits(This,bProp)	\
    ( (This)->lpVtbl -> put_GenerateAudits(This,bProp) ) 

#define IAzApplication_get_ApplyStoreSacl(This,pbProp)	\
    ( (This)->lpVtbl -> get_ApplyStoreSacl(This,pbProp) ) 

#define IAzApplication_put_ApplyStoreSacl(This,bProp)	\
    ( (This)->lpVtbl -> put_ApplyStoreSacl(This,bProp) ) 

#define IAzApplication_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzApplication_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzApplication_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzApplication_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzApplication_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzApplication_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzApplication_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzApplication_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzApplication_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzApplication_get_Scopes(This,ppScopeCollection)	\
    ( (This)->lpVtbl -> get_Scopes(This,ppScopeCollection) ) 

#define IAzApplication_OpenScope(This,bstrScopeName,varReserved,ppScope)	\
    ( (This)->lpVtbl -> OpenScope(This,bstrScopeName,varReserved,ppScope) ) 

#define IAzApplication_CreateScope(This,bstrScopeName,varReserved,ppScope)	\
    ( (This)->lpVtbl -> CreateScope(This,bstrScopeName,varReserved,ppScope) ) 

#define IAzApplication_DeleteScope(This,bstrScopeName,varReserved)	\
    ( (This)->lpVtbl -> DeleteScope(This,bstrScopeName,varReserved) ) 

#define IAzApplication_get_Operations(This,ppOperationCollection)	\
    ( (This)->lpVtbl -> get_Operations(This,ppOperationCollection) ) 

#define IAzApplication_OpenOperation(This,bstrOperationName,varReserved,ppOperation)	\
    ( (This)->lpVtbl -> OpenOperation(This,bstrOperationName,varReserved,ppOperation) ) 

#define IAzApplication_CreateOperation(This,bstrOperationName,varReserved,ppOperation)	\
    ( (This)->lpVtbl -> CreateOperation(This,bstrOperationName,varReserved,ppOperation) ) 

#define IAzApplication_DeleteOperation(This,bstrOperationName,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrOperationName,varReserved) ) 

#define IAzApplication_get_Tasks(This,ppTaskCollection)	\
    ( (This)->lpVtbl -> get_Tasks(This,ppTaskCollection) ) 

#define IAzApplication_OpenTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> OpenTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzApplication_CreateTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> CreateTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzApplication_DeleteTask(This,bstrTaskName,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTaskName,varReserved) ) 

#define IAzApplication_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzApplication_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzApplication_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzApplication_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzApplication_get_Roles(This,ppRoleCollection)	\
    ( (This)->lpVtbl -> get_Roles(This,ppRoleCollection) ) 

#define IAzApplication_OpenRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> OpenRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzApplication_CreateRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> CreateRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzApplication_DeleteRole(This,bstrRoleName,varReserved)	\
    ( (This)->lpVtbl -> DeleteRole(This,bstrRoleName,varReserved) ) 

#define IAzApplication_InitializeClientContextFromToken(This,ullTokenHandle,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromToken(This,ullTokenHandle,varReserved,ppClientContext) ) 

#define IAzApplication_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplication_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplication_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzApplication_InitializeClientContextFromName(This,ClientName,DomainName,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromName(This,ClientName,DomainName,varReserved,ppClientContext) ) 

#define IAzApplication_get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers) ) 

#define IAzApplication_AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication_DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication_InitializeClientContextFromStringSid(This,SidString,lOptions,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromStringSid(This,SidString,lOptions,varReserved,ppClientContext) ) 

#define IAzApplication_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzApplication_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzApplication_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzApplication_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzApplication_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzApplication_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzApplication_get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers) ) 

#define IAzApplication_AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication_DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplication_INTERFACE_DEFINED__ */


#ifndef __IAzApplication2_INTERFACE_DEFINED__
#define __IAzApplication2_INTERFACE_DEFINED__

/* interface IAzApplication2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplication2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("086a68af-a249-437c-b18d-d4d86d6a9660")
    IAzApplication2 : public IAzApplication
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeClientContextFromToken2( 
            /* [in] */ ULONG ulTokenHandleLowPart,
            /* [in] */ ULONG ulTokenHandleHighPart,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext2 **ppClientContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeClientContext2( 
            /* [in] */ __RPC__in BSTR IdentifyingString,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext2 **ppClientContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplication2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplication2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplication2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplication2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplication2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_AuthzInterfaceClsid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuthzInterfaceClsid )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_AuthzInterfaceClsid)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AuthzInterfaceClsid )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Version)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_GenerateAudits)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerateAudits )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_GenerateAudits)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenerateAudits )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplyStoreSacl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplyStoreSacl )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_ApplyStoreSacl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplyStoreSacl )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Scopes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Scopes )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzScopes **ppScopeCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenScope)
        HRESULT ( STDMETHODCALLTYPE *OpenScope )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateScope)
        HRESULT ( STDMETHODCALLTYPE *CreateScope )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteScope)
        HRESULT ( STDMETHODCALLTYPE *DeleteScope )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperations **ppOperationCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenOperation)
        HRESULT ( STDMETHODCALLTYPE *OpenOperation )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateOperation)
        HRESULT ( STDMETHODCALLTYPE *CreateOperation )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenTask)
        HRESULT ( STDMETHODCALLTYPE *OpenTask )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateTask)
        HRESULT ( STDMETHODCALLTYPE *CreateTask )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Roles)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Roles )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenRole)
        HRESULT ( STDMETHODCALLTYPE *OpenRole )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateRole)
        HRESULT ( STDMETHODCALLTYPE *CreateRole )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteRole)
        HRESULT ( STDMETHODCALLTYPE *DeleteRole )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromToken)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromToken )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ ULONGLONG ullTokenHandle,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzApplication2 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromName)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR ClientName,
            /* [defaultvalue][in] */ __RPC__in BSTR DomainName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_DelegatedPolicyUsers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsers )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUser )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUser )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromStringSid)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromStringSid )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR SidString,
            /* [in] */ LONG lOptions,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_DelegatedPolicyUsersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsersName )( 
            __RPC__in IAzApplication2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUserName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUserName )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication2, InitializeClientContextFromToken2)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromToken2 )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ ULONG ulTokenHandleLowPart,
            /* [in] */ ULONG ulTokenHandleHighPart,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext2 **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication2, InitializeClientContext2)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContext2 )( 
            __RPC__in IAzApplication2 * This,
            /* [in] */ __RPC__in BSTR IdentifyingString,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext2 **ppClientContext);
        
        END_INTERFACE
    } IAzApplication2Vtbl;

    interface IAzApplication2
    {
        CONST_VTBL struct IAzApplication2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplication2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplication2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplication2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplication2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplication2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplication2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplication2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplication2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzApplication2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzApplication2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzApplication2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzApplication2_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzApplication2_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzApplication2_get_AuthzInterfaceClsid(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_AuthzInterfaceClsid(This,pbstrProp) ) 

#define IAzApplication2_put_AuthzInterfaceClsid(This,bstrProp)	\
    ( (This)->lpVtbl -> put_AuthzInterfaceClsid(This,bstrProp) ) 

#define IAzApplication2_get_Version(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_Version(This,pbstrProp) ) 

#define IAzApplication2_put_Version(This,bstrProp)	\
    ( (This)->lpVtbl -> put_Version(This,bstrProp) ) 

#define IAzApplication2_get_GenerateAudits(This,pbProp)	\
    ( (This)->lpVtbl -> get_GenerateAudits(This,pbProp) ) 

#define IAzApplication2_put_GenerateAudits(This,bProp)	\
    ( (This)->lpVtbl -> put_GenerateAudits(This,bProp) ) 

#define IAzApplication2_get_ApplyStoreSacl(This,pbProp)	\
    ( (This)->lpVtbl -> get_ApplyStoreSacl(This,pbProp) ) 

#define IAzApplication2_put_ApplyStoreSacl(This,bProp)	\
    ( (This)->lpVtbl -> put_ApplyStoreSacl(This,bProp) ) 

#define IAzApplication2_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzApplication2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzApplication2_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzApplication2_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzApplication2_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzApplication2_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzApplication2_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzApplication2_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzApplication2_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzApplication2_get_Scopes(This,ppScopeCollection)	\
    ( (This)->lpVtbl -> get_Scopes(This,ppScopeCollection) ) 

#define IAzApplication2_OpenScope(This,bstrScopeName,varReserved,ppScope)	\
    ( (This)->lpVtbl -> OpenScope(This,bstrScopeName,varReserved,ppScope) ) 

#define IAzApplication2_CreateScope(This,bstrScopeName,varReserved,ppScope)	\
    ( (This)->lpVtbl -> CreateScope(This,bstrScopeName,varReserved,ppScope) ) 

#define IAzApplication2_DeleteScope(This,bstrScopeName,varReserved)	\
    ( (This)->lpVtbl -> DeleteScope(This,bstrScopeName,varReserved) ) 

#define IAzApplication2_get_Operations(This,ppOperationCollection)	\
    ( (This)->lpVtbl -> get_Operations(This,ppOperationCollection) ) 

#define IAzApplication2_OpenOperation(This,bstrOperationName,varReserved,ppOperation)	\
    ( (This)->lpVtbl -> OpenOperation(This,bstrOperationName,varReserved,ppOperation) ) 

#define IAzApplication2_CreateOperation(This,bstrOperationName,varReserved,ppOperation)	\
    ( (This)->lpVtbl -> CreateOperation(This,bstrOperationName,varReserved,ppOperation) ) 

#define IAzApplication2_DeleteOperation(This,bstrOperationName,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrOperationName,varReserved) ) 

#define IAzApplication2_get_Tasks(This,ppTaskCollection)	\
    ( (This)->lpVtbl -> get_Tasks(This,ppTaskCollection) ) 

#define IAzApplication2_OpenTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> OpenTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzApplication2_CreateTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> CreateTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzApplication2_DeleteTask(This,bstrTaskName,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTaskName,varReserved) ) 

#define IAzApplication2_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzApplication2_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzApplication2_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzApplication2_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzApplication2_get_Roles(This,ppRoleCollection)	\
    ( (This)->lpVtbl -> get_Roles(This,ppRoleCollection) ) 

#define IAzApplication2_OpenRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> OpenRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzApplication2_CreateRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> CreateRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzApplication2_DeleteRole(This,bstrRoleName,varReserved)	\
    ( (This)->lpVtbl -> DeleteRole(This,bstrRoleName,varReserved) ) 

#define IAzApplication2_InitializeClientContextFromToken(This,ullTokenHandle,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromToken(This,ullTokenHandle,varReserved,ppClientContext) ) 

#define IAzApplication2_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplication2_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplication2_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzApplication2_InitializeClientContextFromName(This,ClientName,DomainName,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromName(This,ClientName,DomainName,varReserved,ppClientContext) ) 

#define IAzApplication2_get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers) ) 

#define IAzApplication2_AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication2_DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication2_InitializeClientContextFromStringSid(This,SidString,lOptions,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromStringSid(This,SidString,lOptions,varReserved,ppClientContext) ) 

#define IAzApplication2_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzApplication2_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzApplication2_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzApplication2_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzApplication2_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzApplication2_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzApplication2_get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers) ) 

#define IAzApplication2_AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication2_DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 


#define IAzApplication2_InitializeClientContextFromToken2(This,ulTokenHandleLowPart,ulTokenHandleHighPart,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromToken2(This,ulTokenHandleLowPart,ulTokenHandleHighPart,varReserved,ppClientContext) ) 

#define IAzApplication2_InitializeClientContext2(This,IdentifyingString,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContext2(This,IdentifyingString,varReserved,ppClientContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplication2_INTERFACE_DEFINED__ */


#ifndef __IAzApplications_INTERFACE_DEFINED__
#define __IAzApplications_INTERFACE_DEFINED__

/* interface IAzApplications */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplications;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("929b11a9-95c5-4a84-a29a-20ad42c2f16c")
    IAzApplications : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplicationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplications * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplications * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplications * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplications * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplications * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplications * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplications * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplications, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzApplications * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzApplications, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzApplications * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IAzApplications, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzApplications * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzApplicationsVtbl;

    interface IAzApplications
    {
        CONST_VTBL struct IAzApplicationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplications_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplications_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplications_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplications_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplications_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplications_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplications_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplications_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzApplications_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzApplications_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplications_INTERFACE_DEFINED__ */


#ifndef __IAzOperation_INTERFACE_DEFINED__
#define __IAzOperation_INTERFACE_DEFINED__

/* interface IAzOperation */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e56b24f-ea01-4d61-be44-c49b5e4eaf74")
    IAzOperation : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplicationData( 
            /* [in] */ __RPC__in BSTR bstrApplicationData) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_OperationID( 
            /* [retval][out] */ __RPC__out LONG *plProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_OperationID( 
            /* [in] */ LONG lProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzOperation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzOperation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzOperation * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzOperation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzOperation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzOperation * This,
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
        
        DECLSPEC_XFGVIRT(IAzOperation, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzOperation * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzOperation * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzOperation * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzOperation * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzOperation * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzOperation * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_OperationID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OperationID )( 
            __RPC__in IAzOperation * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_OperationID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_OperationID )( 
            __RPC__in IAzOperation * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzOperation * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzOperation * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzOperation * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzOperation, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzOperation * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        END_INTERFACE
    } IAzOperationVtbl;

    interface IAzOperation
    {
        CONST_VTBL struct IAzOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzOperation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzOperation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzOperation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzOperation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzOperation_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzOperation_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzOperation_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzOperation_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzOperation_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzOperation_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzOperation_get_OperationID(This,plProp)	\
    ( (This)->lpVtbl -> get_OperationID(This,plProp) ) 

#define IAzOperation_put_OperationID(This,lProp)	\
    ( (This)->lpVtbl -> put_OperationID(This,lProp) ) 

#define IAzOperation_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzOperation_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzOperation_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzOperation_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzOperation_INTERFACE_DEFINED__ */


#ifndef __IAzOperations_INTERFACE_DEFINED__
#define __IAzOperations_INTERFACE_DEFINED__

/* interface IAzOperations */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90ef9c07-9706-49d9-af80-0438a5f3ec35")
    IAzOperations : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzOperationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzOperations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzOperations * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzOperations * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzOperations * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzOperations * This,
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
        
        DECLSPEC_XFGVIRT(IAzOperations, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzOperations * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzOperations, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzOperations * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzOperations, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzOperationsVtbl;

    interface IAzOperations
    {
        CONST_VTBL struct IAzOperationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzOperations_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzOperations_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzOperations_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzOperations_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzOperations_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzOperations_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzOperations_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzOperations_INTERFACE_DEFINED__ */


#ifndef __IAzTask_INTERFACE_DEFINED__
#define __IAzTask_INTERFACE_DEFINED__

/* interface IAzTask */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzTask;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cb94e592-2e0e-4a6c-a336-b89a6dc1e388")
    IAzTask : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplicationData( 
            /* [in] */ __RPC__in BSTR bstrApplicationData) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRule( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRule( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRuleLanguage( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRuleLanguage( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRuleImportedPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRuleImportedPath( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsRoleDefinition( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_IsRoleDefinition( 
            /* [in] */ BOOL fProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Operations( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Tasks( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddOperation( 
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteOperation( 
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTask( 
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteTask( 
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzTaskVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzTask * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzTask * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzTask * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzTask * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzTask * This,
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
        
        DECLSPEC_XFGVIRT(IAzTask, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzTask, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzTask, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzTask, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzTask, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRule)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRule )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRule)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRule )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRuleLanguage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleLanguage )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRuleLanguage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleLanguage )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRuleImportedPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleImportedPath )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRuleImportedPath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleImportedPath )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_IsRoleDefinition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsRoleDefinition )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_IsRoleDefinition)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsRoleDefinition )( 
            __RPC__in IAzTask * This,
            /* [in] */ BOOL fProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, AddOperation)
        HRESULT ( STDMETHODCALLTYPE *AddOperation )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, AddTask)
        HRESULT ( STDMETHODCALLTYPE *AddTask )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzTask * This,
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzTask * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzTask, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzTask * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzTask * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzTask * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzTask * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzTask * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        END_INTERFACE
    } IAzTaskVtbl;

    interface IAzTask
    {
        CONST_VTBL struct IAzTaskVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzTask_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzTask_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzTask_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzTask_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzTask_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzTask_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzTask_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzTask_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzTask_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzTask_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzTask_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzTask_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzTask_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzTask_get_BizRule(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRule(This,pbstrProp) ) 

#define IAzTask_put_BizRule(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRule(This,bstrProp) ) 

#define IAzTask_get_BizRuleLanguage(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleLanguage(This,pbstrProp) ) 

#define IAzTask_put_BizRuleLanguage(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleLanguage(This,bstrProp) ) 

#define IAzTask_get_BizRuleImportedPath(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleImportedPath(This,pbstrProp) ) 

#define IAzTask_put_BizRuleImportedPath(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleImportedPath(This,bstrProp) ) 

#define IAzTask_get_IsRoleDefinition(This,pfProp)	\
    ( (This)->lpVtbl -> get_IsRoleDefinition(This,pfProp) ) 

#define IAzTask_put_IsRoleDefinition(This,fProp)	\
    ( (This)->lpVtbl -> put_IsRoleDefinition(This,fProp) ) 

#define IAzTask_get_Operations(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Operations(This,pvarProp) ) 

#define IAzTask_get_Tasks(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Tasks(This,pvarProp) ) 

#define IAzTask_AddOperation(This,bstrOp,varReserved)	\
    ( (This)->lpVtbl -> AddOperation(This,bstrOp,varReserved) ) 

#define IAzTask_DeleteOperation(This,bstrOp,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrOp,varReserved) ) 

#define IAzTask_AddTask(This,bstrTask,varReserved)	\
    ( (This)->lpVtbl -> AddTask(This,bstrTask,varReserved) ) 

#define IAzTask_DeleteTask(This,bstrTask,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTask,varReserved) ) 

#define IAzTask_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzTask_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzTask_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzTask_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzTask_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzTask_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzTask_INTERFACE_DEFINED__ */


#ifndef __IAzTasks_INTERFACE_DEFINED__
#define __IAzTasks_INTERFACE_DEFINED__

/* interface IAzTasks */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzTasks;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b338ccab-4c85-4388-8c0a-c58592bad398")
    IAzTasks : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzTasksVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzTasks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzTasks * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzTasks * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzTasks * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzTasks * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzTasks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzTasks * This,
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
        
        DECLSPEC_XFGVIRT(IAzTasks, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzTasks * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzTasks, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzTasks * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzTasks, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzTasks * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzTasksVtbl;

    interface IAzTasks
    {
        CONST_VTBL struct IAzTasksVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzTasks_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzTasks_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzTasks_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzTasks_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzTasks_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzTasks_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzTasks_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzTasks_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzTasks_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzTasks_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzTasks_INTERFACE_DEFINED__ */


#ifndef __IAzScope_INTERFACE_DEFINED__
#define __IAzScope_INTERFACE_DEFINED__

/* interface IAzScope */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzScope;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00e52487-e08d-4514-b62e-877d5645f5ab")
    IAzScope : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplicationData( 
            /* [in] */ __RPC__in BSTR bstrApplicationData) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyAdministrators( 
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyReaders( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyAdministrator( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyAdministrator( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyReader( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyReader( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteApplicationGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Roles( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenRole( 
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRole( 
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRole( 
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Tasks( 
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenTask( 
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTask( 
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteTask( 
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanBeDelegated( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizrulesWritable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyAdministratorsName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PolicyReadersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyAdministratorName( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyAdministratorName( 
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPolicyReaderName( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePolicyReaderName( 
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzScopeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzScope * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzScope * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzScope * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzScope * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzScope * This,
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
        
        DECLSPEC_XFGVIRT(IAzScope, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzScope, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzScope, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzScope, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzScope, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzScope, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzScope * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzScope, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzScope * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzScope * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzScope * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzScope, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzScope, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzScope, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Roles)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Roles )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection);
        
        DECLSPEC_XFGVIRT(IAzScope, OpenRole)
        HRESULT ( STDMETHODCALLTYPE *OpenRole )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzScope, CreateRole)
        HRESULT ( STDMETHODCALLTYPE *CreateRole )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzScope, DeleteRole)
        HRESULT ( STDMETHODCALLTYPE *DeleteRole )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection);
        
        DECLSPEC_XFGVIRT(IAzScope, OpenTask)
        HRESULT ( STDMETHODCALLTYPE *OpenTask )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzScope, CreateTask)
        HRESULT ( STDMETHODCALLTYPE *CreateTask )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzScope, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzScope * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_CanBeDelegated)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanBeDelegated )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzScope, get_BizrulesWritable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizrulesWritable )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzScope * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzScope * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        END_INTERFACE
    } IAzScopeVtbl;

    interface IAzScope
    {
        CONST_VTBL struct IAzScopeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzScope_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzScope_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzScope_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzScope_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzScope_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzScope_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzScope_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzScope_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzScope_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzScope_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzScope_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzScope_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzScope_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzScope_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzScope_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzScope_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzScope_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzScope_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzScope_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzScope_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzScope_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzScope_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzScope_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzScope_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzScope_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzScope_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzScope_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzScope_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzScope_get_Roles(This,ppRoleCollection)	\
    ( (This)->lpVtbl -> get_Roles(This,ppRoleCollection) ) 

#define IAzScope_OpenRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> OpenRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzScope_CreateRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> CreateRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzScope_DeleteRole(This,bstrRoleName,varReserved)	\
    ( (This)->lpVtbl -> DeleteRole(This,bstrRoleName,varReserved) ) 

#define IAzScope_get_Tasks(This,ppTaskCollection)	\
    ( (This)->lpVtbl -> get_Tasks(This,ppTaskCollection) ) 

#define IAzScope_OpenTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> OpenTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzScope_CreateTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> CreateTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzScope_DeleteTask(This,bstrTaskName,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTaskName,varReserved) ) 

#define IAzScope_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzScope_get_CanBeDelegated(This,pfProp)	\
    ( (This)->lpVtbl -> get_CanBeDelegated(This,pfProp) ) 

#define IAzScope_get_BizrulesWritable(This,pfProp)	\
    ( (This)->lpVtbl -> get_BizrulesWritable(This,pfProp) ) 

#define IAzScope_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzScope_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzScope_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzScope_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzScope_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzScope_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzScope_INTERFACE_DEFINED__ */


#ifndef __IAzScopes_INTERFACE_DEFINED__
#define __IAzScopes_INTERFACE_DEFINED__

/* interface IAzScopes */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzScopes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("78e14853-9f5e-406d-9b91-6bdba6973510")
    IAzScopes : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzScopesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzScopes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzScopes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzScopes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzScopes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzScopes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzScopes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzScopes * This,
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
        
        DECLSPEC_XFGVIRT(IAzScopes, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzScopes * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzScopes, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzScopes * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzScopes, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzScopes * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzScopesVtbl;

    interface IAzScopes
    {
        CONST_VTBL struct IAzScopesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzScopes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzScopes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzScopes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzScopes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzScopes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzScopes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzScopes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzScopes_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzScopes_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzScopes_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzScopes_INTERFACE_DEFINED__ */


#ifndef __IAzApplicationGroup_INTERFACE_DEFINED__
#define __IAzApplicationGroup_INTERFACE_DEFINED__

/* interface IAzApplicationGroup */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplicationGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f1b744cd-58a6-4e06-9fbf-36f6d779e21e")
    IAzApplicationGroup : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out LONG *plProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ LONG lProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LdapQuery( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LdapQuery( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AppMembers( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AppNonMembers( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Members( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NonMembers( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddAppMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAppMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddAppNonMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAppNonMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddNonMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteNonMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddMemberName( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMemberName( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddNonMemberName( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteNonMemberName( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MembersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NonMembersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplicationGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplicationGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplicationGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplicationGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplicationGroup * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Type)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_Type)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_LdapQuery)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LdapQuery )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_LdapQuery)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LdapQuery )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_AppMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppMembers )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_AppNonMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppNonMembers )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Members)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_NonMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NonMembers )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddAppMember)
        HRESULT ( STDMETHODCALLTYPE *AddAppMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteAppMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteAppMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddAppNonMember)
        HRESULT ( STDMETHODCALLTYPE *AddAppNonMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteAppNonMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteAppNonMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddMember)
        HRESULT ( STDMETHODCALLTYPE *AddMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddNonMember)
        HRESULT ( STDMETHODCALLTYPE *AddNonMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteNonMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteNonMember )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzApplicationGroup * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddMemberName)
        HRESULT ( STDMETHODCALLTYPE *AddMemberName )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteMemberName)
        HRESULT ( STDMETHODCALLTYPE *DeleteMemberName )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddNonMemberName)
        HRESULT ( STDMETHODCALLTYPE *AddNonMemberName )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteNonMemberName)
        HRESULT ( STDMETHODCALLTYPE *DeleteNonMemberName )( 
            __RPC__in IAzApplicationGroup * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_MembersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MembersName )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_NonMembersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NonMembersName )( 
            __RPC__in IAzApplicationGroup * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        END_INTERFACE
    } IAzApplicationGroupVtbl;

    interface IAzApplicationGroup
    {
        CONST_VTBL struct IAzApplicationGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplicationGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplicationGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplicationGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplicationGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplicationGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplicationGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplicationGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplicationGroup_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzApplicationGroup_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzApplicationGroup_get_Type(This,plProp)	\
    ( (This)->lpVtbl -> get_Type(This,plProp) ) 

#define IAzApplicationGroup_put_Type(This,lProp)	\
    ( (This)->lpVtbl -> put_Type(This,lProp) ) 

#define IAzApplicationGroup_get_LdapQuery(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_LdapQuery(This,pbstrProp) ) 

#define IAzApplicationGroup_put_LdapQuery(This,bstrProp)	\
    ( (This)->lpVtbl -> put_LdapQuery(This,bstrProp) ) 

#define IAzApplicationGroup_get_AppMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_AppMembers(This,pvarProp) ) 

#define IAzApplicationGroup_get_AppNonMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_AppNonMembers(This,pvarProp) ) 

#define IAzApplicationGroup_get_Members(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Members(This,pvarProp) ) 

#define IAzApplicationGroup_get_NonMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_NonMembers(This,pvarProp) ) 

#define IAzApplicationGroup_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzApplicationGroup_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzApplicationGroup_AddAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddAppMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_DeleteAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteAppMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_AddAppNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddAppNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_DeleteAppNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteAppNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_AddMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_DeleteMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_AddNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_DeleteNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzApplicationGroup_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzApplicationGroup_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzApplicationGroup_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplicationGroup_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplicationGroup_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzApplicationGroup_AddMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_DeleteMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_AddNonMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddNonMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_DeleteNonMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteNonMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup_get_MembersName(This,pvarProp)	\
    ( (This)->lpVtbl -> get_MembersName(This,pvarProp) ) 

#define IAzApplicationGroup_get_NonMembersName(This,pvarProp)	\
    ( (This)->lpVtbl -> get_NonMembersName(This,pvarProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplicationGroup_INTERFACE_DEFINED__ */


#ifndef __IAzApplicationGroups_INTERFACE_DEFINED__
#define __IAzApplicationGroups_INTERFACE_DEFINED__

/* interface IAzApplicationGroups */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplicationGroups;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ce66ad5-9f3c-469d-a911-b99887a7e685")
    IAzApplicationGroups : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplicationGroupsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplicationGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplicationGroups * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplicationGroups * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplicationGroups * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplicationGroups * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplicationGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplicationGroups * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplicationGroups, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzApplicationGroups * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroups, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzApplicationGroups * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroups, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzApplicationGroups * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzApplicationGroupsVtbl;

    interface IAzApplicationGroups
    {
        CONST_VTBL struct IAzApplicationGroupsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplicationGroups_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplicationGroups_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplicationGroups_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplicationGroups_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplicationGroups_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplicationGroups_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplicationGroups_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplicationGroups_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzApplicationGroups_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzApplicationGroups_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplicationGroups_INTERFACE_DEFINED__ */


#ifndef __IAzRole_INTERFACE_DEFINED__
#define __IAzRole_INTERFACE_DEFINED__

/* interface IAzRole */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzRole;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("859e0d8d-62d7-41d8-a034-c0cd5d43fdfa")
    IAzRole : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ApplicationData( 
            /* [in] */ __RPC__in BSTR bstrApplicationData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddAppMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAppMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTask( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteTask( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddOperation( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteOperation( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMember( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Writable( 
            /* [retval][out] */ __RPC__out BOOL *pfProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AppMembers( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Members( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Operations( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Tasks( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePropertyItem( 
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Submit( 
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddMemberName( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMemberName( 
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_MembersName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzRoleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzRole * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzRole * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzRole * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzRole * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzRole * This,
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
        
        DECLSPEC_XFGVIRT(IAzRole, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzRole, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzRole, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzRole, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzRole, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzRole, AddAppMember)
        HRESULT ( STDMETHODCALLTYPE *AddAppMember )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteAppMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteAppMember )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddTask)
        HRESULT ( STDMETHODCALLTYPE *AddTask )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddOperation)
        HRESULT ( STDMETHODCALLTYPE *AddOperation )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddMember)
        HRESULT ( STDMETHODCALLTYPE *AddMember )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteMember )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzRole, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzRole * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzRole * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, get_AppMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppMembers )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Members)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzRole * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzRole * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzRole * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddMemberName)
        HRESULT ( STDMETHODCALLTYPE *AddMemberName )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteMemberName)
        HRESULT ( STDMETHODCALLTYPE *DeleteMemberName )( 
            __RPC__in IAzRole * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, get_MembersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MembersName )( 
            __RPC__in IAzRole * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        END_INTERFACE
    } IAzRoleVtbl;

    interface IAzRole
    {
        CONST_VTBL struct IAzRoleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzRole_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzRole_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzRole_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzRole_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzRole_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzRole_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzRole_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzRole_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzRole_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzRole_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzRole_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzRole_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzRole_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzRole_AddAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddAppMember(This,bstrProp,varReserved) ) 

#define IAzRole_DeleteAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteAppMember(This,bstrProp,varReserved) ) 

#define IAzRole_AddTask(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddTask(This,bstrProp,varReserved) ) 

#define IAzRole_DeleteTask(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrProp,varReserved) ) 

#define IAzRole_AddOperation(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddOperation(This,bstrProp,varReserved) ) 

#define IAzRole_DeleteOperation(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrProp,varReserved) ) 

#define IAzRole_AddMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMember(This,bstrProp,varReserved) ) 

#define IAzRole_DeleteMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMember(This,bstrProp,varReserved) ) 

#define IAzRole_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzRole_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzRole_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzRole_get_AppMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_AppMembers(This,pvarProp) ) 

#define IAzRole_get_Members(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Members(This,pvarProp) ) 

#define IAzRole_get_Operations(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Operations(This,pvarProp) ) 

#define IAzRole_get_Tasks(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Tasks(This,pvarProp) ) 

#define IAzRole_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzRole_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzRole_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzRole_AddMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMemberName(This,bstrProp,varReserved) ) 

#define IAzRole_DeleteMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMemberName(This,bstrProp,varReserved) ) 

#define IAzRole_get_MembersName(This,pvarProp)	\
    ( (This)->lpVtbl -> get_MembersName(This,pvarProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzRole_INTERFACE_DEFINED__ */


#ifndef __IAzRoles_INTERFACE_DEFINED__
#define __IAzRoles_INTERFACE_DEFINED__

/* interface IAzRoles */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzRoles;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95e0f119-13b4-4dae-b65f-2f7d60d822e4")
    IAzRoles : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzRolesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzRoles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzRoles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzRoles * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzRoles * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzRoles * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzRoles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzRoles * This,
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
        
        DECLSPEC_XFGVIRT(IAzRoles, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzRoles * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzRoles, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzRoles * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzRoles, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzRoles * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzRolesVtbl;

    interface IAzRoles
    {
        CONST_VTBL struct IAzRolesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzRoles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzRoles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzRoles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzRoles_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzRoles_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzRoles_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzRoles_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzRoles_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzRoles_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzRoles_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzRoles_INTERFACE_DEFINED__ */


#ifndef __IAzClientContext_INTERFACE_DEFINED__
#define __IAzClientContext_INTERFACE_DEFINED__

/* interface IAzClientContext */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzClientContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eff1f00b-488a-466d-afd9-a401c5f9eef5")
    IAzClientContext : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AccessCheck( 
            /* [in] */ __RPC__in BSTR bstrObjectName,
            /* [in] */ VARIANT varScopeNames,
            /* [in] */ VARIANT varOperations,
            /* [optional][in] */ VARIANT varParameterNames,
            /* [optional][in] */ VARIANT varParameterValues,
            /* [optional][in] */ VARIANT varInterfaceNames,
            /* [optional][in] */ VARIANT varInterfaceFlags,
            /* [optional][in] */ VARIANT varInterfaces,
            /* [retval][out] */ __RPC__out VARIANT *pvarResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBusinessRuleString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBusinessRuleString) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserDn( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserSamCompat( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserDisplay( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserCanonical( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserUpn( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_UserDnsSamCompat( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRoles( 
            /* [defaultvalue][in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__out VARIANT *pvarRoleNames) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleForAccessCheck( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RoleForAccessCheck( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzClientContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzClientContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzClientContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzClientContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzClientContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzClientContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzClientContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzClientContext * This,
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
        
        DECLSPEC_XFGVIRT(IAzClientContext, AccessCheck)
        HRESULT ( STDMETHODCALLTYPE *AccessCheck )( 
            __RPC__in IAzClientContext * This,
            /* [in] */ __RPC__in BSTR bstrObjectName,
            /* [in] */ VARIANT varScopeNames,
            /* [in] */ VARIANT varOperations,
            /* [optional][in] */ VARIANT varParameterNames,
            /* [optional][in] */ VARIANT varParameterValues,
            /* [optional][in] */ VARIANT varInterfaceNames,
            /* [optional][in] */ VARIANT varInterfaceFlags,
            /* [optional][in] */ VARIANT varInterfaces,
            /* [retval][out] */ __RPC__out VARIANT *pvarResults);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetBusinessRuleString)
        HRESULT ( STDMETHODCALLTYPE *GetBusinessRuleString )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBusinessRuleString);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDn)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDn )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserSamCompat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSamCompat )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDisplay)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDisplay )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserGuid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserGuid )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserCanonical)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserCanonical )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserUpn)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserUpn )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDnsSamCompat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDnsSamCompat )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzClientContext * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetRoles)
        HRESULT ( STDMETHODCALLTYPE *GetRoles )( 
            __RPC__in IAzClientContext * This,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__out VARIANT *pvarRoleNames);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_RoleForAccessCheck)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleForAccessCheck )( 
            __RPC__in IAzClientContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, put_RoleForAccessCheck)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RoleForAccessCheck )( 
            __RPC__in IAzClientContext * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        END_INTERFACE
    } IAzClientContextVtbl;

    interface IAzClientContext
    {
        CONST_VTBL struct IAzClientContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzClientContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzClientContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzClientContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzClientContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzClientContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzClientContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzClientContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzClientContext_AccessCheck(This,bstrObjectName,varScopeNames,varOperations,varParameterNames,varParameterValues,varInterfaceNames,varInterfaceFlags,varInterfaces,pvarResults)	\
    ( (This)->lpVtbl -> AccessCheck(This,bstrObjectName,varScopeNames,varOperations,varParameterNames,varParameterValues,varInterfaceNames,varInterfaceFlags,varInterfaces,pvarResults) ) 

#define IAzClientContext_GetBusinessRuleString(This,pbstrBusinessRuleString)	\
    ( (This)->lpVtbl -> GetBusinessRuleString(This,pbstrBusinessRuleString) ) 

#define IAzClientContext_get_UserDn(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDn(This,pbstrProp) ) 

#define IAzClientContext_get_UserSamCompat(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserSamCompat(This,pbstrProp) ) 

#define IAzClientContext_get_UserDisplay(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDisplay(This,pbstrProp) ) 

#define IAzClientContext_get_UserGuid(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserGuid(This,pbstrProp) ) 

#define IAzClientContext_get_UserCanonical(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserCanonical(This,pbstrProp) ) 

#define IAzClientContext_get_UserUpn(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserUpn(This,pbstrProp) ) 

#define IAzClientContext_get_UserDnsSamCompat(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDnsSamCompat(This,pbstrProp) ) 

#define IAzClientContext_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzClientContext_GetRoles(This,bstrScopeName,pvarRoleNames)	\
    ( (This)->lpVtbl -> GetRoles(This,bstrScopeName,pvarRoleNames) ) 

#define IAzClientContext_get_RoleForAccessCheck(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_RoleForAccessCheck(This,pbstrProp) ) 

#define IAzClientContext_put_RoleForAccessCheck(This,bstrProp)	\
    ( (This)->lpVtbl -> put_RoleForAccessCheck(This,bstrProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzClientContext_INTERFACE_DEFINED__ */


#ifndef __IAzClientContext2_INTERFACE_DEFINED__
#define __IAzClientContext2_INTERFACE_DEFINED__

/* interface IAzClientContext2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzClientContext2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2b0c92b8-208a-488a-8f81-e4edb22111cd")
    IAzClientContext2 : public IAzClientContext
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAssignedScopesPage( 
            /* [in] */ LONG lOptions,
            /* [in] */ LONG PageSize,
            /* [out][in] */ __RPC__inout VARIANT *pvarCursor,
            /* [retval][out] */ __RPC__out VARIANT *pvarScopeNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRoles( 
            /* [in] */ VARIANT varRoles,
            /* [in] */ __RPC__in BSTR bstrScopeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddApplicationGroups( 
            /* [in] */ VARIANT varApplicationGroups) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStringSids( 
            /* [in] */ VARIANT varStringSids) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_LDAPQueryDN( 
            /* [in] */ __RPC__in BSTR bstrLDAPQueryDN) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LDAPQueryDN( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLDAPQueryDN) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzClientContext2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzClientContext2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzClientContext2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzClientContext2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzClientContext2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzClientContext, AccessCheck)
        HRESULT ( STDMETHODCALLTYPE *AccessCheck )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ __RPC__in BSTR bstrObjectName,
            /* [in] */ VARIANT varScopeNames,
            /* [in] */ VARIANT varOperations,
            /* [optional][in] */ VARIANT varParameterNames,
            /* [optional][in] */ VARIANT varParameterValues,
            /* [optional][in] */ VARIANT varInterfaceNames,
            /* [optional][in] */ VARIANT varInterfaceFlags,
            /* [optional][in] */ VARIANT varInterfaces,
            /* [retval][out] */ __RPC__out VARIANT *pvarResults);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetBusinessRuleString)
        HRESULT ( STDMETHODCALLTYPE *GetBusinessRuleString )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBusinessRuleString);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDn)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDn )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserSamCompat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSamCompat )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDisplay)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDisplay )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserGuid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserGuid )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserCanonical)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserCanonical )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserUpn)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserUpn )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDnsSamCompat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDnsSamCompat )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetRoles)
        HRESULT ( STDMETHODCALLTYPE *GetRoles )( 
            __RPC__in IAzClientContext2 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__out VARIANT *pvarRoleNames);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_RoleForAccessCheck)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleForAccessCheck )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, put_RoleForAccessCheck)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RoleForAccessCheck )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, GetAssignedScopesPage)
        HRESULT ( STDMETHODCALLTYPE *GetAssignedScopesPage )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ LONG lOptions,
            /* [in] */ LONG PageSize,
            /* [out][in] */ __RPC__inout VARIANT *pvarCursor,
            /* [retval][out] */ __RPC__out VARIANT *pvarScopeNames);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, AddRoles)
        HRESULT ( STDMETHODCALLTYPE *AddRoles )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ VARIANT varRoles,
            /* [in] */ __RPC__in BSTR bstrScopeName);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, AddApplicationGroups)
        HRESULT ( STDMETHODCALLTYPE *AddApplicationGroups )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ VARIANT varApplicationGroups);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, AddStringSids)
        HRESULT ( STDMETHODCALLTYPE *AddStringSids )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ VARIANT varStringSids);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, put_LDAPQueryDN)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LDAPQueryDN )( 
            __RPC__in IAzClientContext2 * This,
            /* [in] */ __RPC__in BSTR bstrLDAPQueryDN);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, get_LDAPQueryDN)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LDAPQueryDN )( 
            __RPC__in IAzClientContext2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLDAPQueryDN);
        
        END_INTERFACE
    } IAzClientContext2Vtbl;

    interface IAzClientContext2
    {
        CONST_VTBL struct IAzClientContext2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzClientContext2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzClientContext2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzClientContext2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzClientContext2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzClientContext2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzClientContext2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzClientContext2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzClientContext2_AccessCheck(This,bstrObjectName,varScopeNames,varOperations,varParameterNames,varParameterValues,varInterfaceNames,varInterfaceFlags,varInterfaces,pvarResults)	\
    ( (This)->lpVtbl -> AccessCheck(This,bstrObjectName,varScopeNames,varOperations,varParameterNames,varParameterValues,varInterfaceNames,varInterfaceFlags,varInterfaces,pvarResults) ) 

#define IAzClientContext2_GetBusinessRuleString(This,pbstrBusinessRuleString)	\
    ( (This)->lpVtbl -> GetBusinessRuleString(This,pbstrBusinessRuleString) ) 

#define IAzClientContext2_get_UserDn(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDn(This,pbstrProp) ) 

#define IAzClientContext2_get_UserSamCompat(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserSamCompat(This,pbstrProp) ) 

#define IAzClientContext2_get_UserDisplay(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDisplay(This,pbstrProp) ) 

#define IAzClientContext2_get_UserGuid(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserGuid(This,pbstrProp) ) 

#define IAzClientContext2_get_UserCanonical(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserCanonical(This,pbstrProp) ) 

#define IAzClientContext2_get_UserUpn(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserUpn(This,pbstrProp) ) 

#define IAzClientContext2_get_UserDnsSamCompat(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDnsSamCompat(This,pbstrProp) ) 

#define IAzClientContext2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzClientContext2_GetRoles(This,bstrScopeName,pvarRoleNames)	\
    ( (This)->lpVtbl -> GetRoles(This,bstrScopeName,pvarRoleNames) ) 

#define IAzClientContext2_get_RoleForAccessCheck(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_RoleForAccessCheck(This,pbstrProp) ) 

#define IAzClientContext2_put_RoleForAccessCheck(This,bstrProp)	\
    ( (This)->lpVtbl -> put_RoleForAccessCheck(This,bstrProp) ) 


#define IAzClientContext2_GetAssignedScopesPage(This,lOptions,PageSize,pvarCursor,pvarScopeNames)	\
    ( (This)->lpVtbl -> GetAssignedScopesPage(This,lOptions,PageSize,pvarCursor,pvarScopeNames) ) 

#define IAzClientContext2_AddRoles(This,varRoles,bstrScopeName)	\
    ( (This)->lpVtbl -> AddRoles(This,varRoles,bstrScopeName) ) 

#define IAzClientContext2_AddApplicationGroups(This,varApplicationGroups)	\
    ( (This)->lpVtbl -> AddApplicationGroups(This,varApplicationGroups) ) 

#define IAzClientContext2_AddStringSids(This,varStringSids)	\
    ( (This)->lpVtbl -> AddStringSids(This,varStringSids) ) 

#define IAzClientContext2_put_LDAPQueryDN(This,bstrLDAPQueryDN)	\
    ( (This)->lpVtbl -> put_LDAPQueryDN(This,bstrLDAPQueryDN) ) 

#define IAzClientContext2_get_LDAPQueryDN(This,pbstrLDAPQueryDN)	\
    ( (This)->lpVtbl -> get_LDAPQueryDN(This,pbstrLDAPQueryDN) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzClientContext2_INTERFACE_DEFINED__ */


#ifndef __IAzBizRuleContext_INTERFACE_DEFINED__
#define __IAzBizRuleContext_INTERFACE_DEFINED__

/* interface IAzBizRuleContext */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzBizRuleContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e192f17d-d59f-455e-a152-940316cd77b2")
    IAzBizRuleContext : public IDispatch
    {
    public:
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BusinessRuleResult( 
            /* [in] */ BOOL bResult) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BusinessRuleString( 
            /* [in] */ __RPC__in BSTR bstrBusinessRuleString) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BusinessRuleString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBusinessRuleString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameter( 
            /* [in] */ __RPC__in BSTR bstrParameterName,
            /* [retval][out] */ __RPC__out VARIANT *pvarParameterValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzBizRuleContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzBizRuleContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzBizRuleContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzBizRuleContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzBizRuleContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzBizRuleContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzBizRuleContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzBizRuleContext * This,
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
        
        DECLSPEC_XFGVIRT(IAzBizRuleContext, put_BusinessRuleResult)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BusinessRuleResult )( 
            __RPC__in IAzBizRuleContext * This,
            /* [in] */ BOOL bResult);
        
        DECLSPEC_XFGVIRT(IAzBizRuleContext, put_BusinessRuleString)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BusinessRuleString )( 
            __RPC__in IAzBizRuleContext * This,
            /* [in] */ __RPC__in BSTR bstrBusinessRuleString);
        
        DECLSPEC_XFGVIRT(IAzBizRuleContext, get_BusinessRuleString)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BusinessRuleString )( 
            __RPC__in IAzBizRuleContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBusinessRuleString);
        
        DECLSPEC_XFGVIRT(IAzBizRuleContext, GetParameter)
        HRESULT ( STDMETHODCALLTYPE *GetParameter )( 
            __RPC__in IAzBizRuleContext * This,
            /* [in] */ __RPC__in BSTR bstrParameterName,
            /* [retval][out] */ __RPC__out VARIANT *pvarParameterValue);
        
        END_INTERFACE
    } IAzBizRuleContextVtbl;

    interface IAzBizRuleContext
    {
        CONST_VTBL struct IAzBizRuleContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzBizRuleContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzBizRuleContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzBizRuleContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzBizRuleContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzBizRuleContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzBizRuleContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzBizRuleContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzBizRuleContext_put_BusinessRuleResult(This,bResult)	\
    ( (This)->lpVtbl -> put_BusinessRuleResult(This,bResult) ) 

#define IAzBizRuleContext_put_BusinessRuleString(This,bstrBusinessRuleString)	\
    ( (This)->lpVtbl -> put_BusinessRuleString(This,bstrBusinessRuleString) ) 

#define IAzBizRuleContext_get_BusinessRuleString(This,pbstrBusinessRuleString)	\
    ( (This)->lpVtbl -> get_BusinessRuleString(This,pbstrBusinessRuleString) ) 

#define IAzBizRuleContext_GetParameter(This,bstrParameterName,pvarParameterValue)	\
    ( (This)->lpVtbl -> GetParameter(This,bstrParameterName,pvarParameterValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzBizRuleContext_INTERFACE_DEFINED__ */


#ifndef __IAzBizRuleParameters_INTERFACE_DEFINED__
#define __IAzBizRuleParameters_INTERFACE_DEFINED__

/* interface IAzBizRuleParameters */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzBizRuleParameters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fc17685f-e25d-4dcd-bae1-276ec9533cb5")
    IAzBizRuleParameters : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddParameter( 
            /* [in] */ __RPC__in BSTR bstrParameterName,
            /* [in] */ VARIANT varParameterValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddParameters( 
            /* [in] */ VARIANT varParameterNames,
            /* [in] */ VARIANT varParameterValues) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterValue( 
            /* [in] */ __RPC__in BSTR bstrParameterName,
            /* [retval][out] */ __RPC__out VARIANT *pvarParameterValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR varParameterName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAll( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out unsigned long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzBizRuleParametersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzBizRuleParameters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzBizRuleParameters * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzBizRuleParameters * This,
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
        
        DECLSPEC_XFGVIRT(IAzBizRuleParameters, AddParameter)
        HRESULT ( STDMETHODCALLTYPE *AddParameter )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ __RPC__in BSTR bstrParameterName,
            /* [in] */ VARIANT varParameterValue);
        
        DECLSPEC_XFGVIRT(IAzBizRuleParameters, AddParameters)
        HRESULT ( STDMETHODCALLTYPE *AddParameters )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ VARIANT varParameterNames,
            /* [in] */ VARIANT varParameterValues);
        
        DECLSPEC_XFGVIRT(IAzBizRuleParameters, GetParameterValue)
        HRESULT ( STDMETHODCALLTYPE *GetParameterValue )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ __RPC__in BSTR bstrParameterName,
            /* [retval][out] */ __RPC__out VARIANT *pvarParameterValue);
        
        DECLSPEC_XFGVIRT(IAzBizRuleParameters, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [in] */ __RPC__in BSTR varParameterName);
        
        DECLSPEC_XFGVIRT(IAzBizRuleParameters, RemoveAll)
        HRESULT ( STDMETHODCALLTYPE *RemoveAll )( 
            __RPC__in IAzBizRuleParameters * This);
        
        DECLSPEC_XFGVIRT(IAzBizRuleParameters, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzBizRuleParameters * This,
            /* [retval][out] */ __RPC__out unsigned long *plCount);
        
        END_INTERFACE
    } IAzBizRuleParametersVtbl;

    interface IAzBizRuleParameters
    {
        CONST_VTBL struct IAzBizRuleParametersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzBizRuleParameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzBizRuleParameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzBizRuleParameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzBizRuleParameters_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzBizRuleParameters_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzBizRuleParameters_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzBizRuleParameters_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzBizRuleParameters_AddParameter(This,bstrParameterName,varParameterValue)	\
    ( (This)->lpVtbl -> AddParameter(This,bstrParameterName,varParameterValue) ) 

#define IAzBizRuleParameters_AddParameters(This,varParameterNames,varParameterValues)	\
    ( (This)->lpVtbl -> AddParameters(This,varParameterNames,varParameterValues) ) 

#define IAzBizRuleParameters_GetParameterValue(This,bstrParameterName,pvarParameterValue)	\
    ( (This)->lpVtbl -> GetParameterValue(This,bstrParameterName,pvarParameterValue) ) 

#define IAzBizRuleParameters_Remove(This,varParameterName)	\
    ( (This)->lpVtbl -> Remove(This,varParameterName) ) 

#define IAzBizRuleParameters_RemoveAll(This)	\
    ( (This)->lpVtbl -> RemoveAll(This) ) 

#define IAzBizRuleParameters_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzBizRuleParameters_INTERFACE_DEFINED__ */


#ifndef __IAzBizRuleInterfaces_INTERFACE_DEFINED__
#define __IAzBizRuleInterfaces_INTERFACE_DEFINED__

/* interface IAzBizRuleInterfaces */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzBizRuleInterfaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e94128c7-e9da-44cc-b0bd-53036f3aab3d")
    IAzBizRuleInterfaces : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddInterface( 
            /* [in] */ __RPC__in BSTR bstrInterfaceName,
            /* [in] */ LONG lInterfaceFlag,
            /* [in] */ VARIANT varInterface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddInterfaces( 
            /* [in] */ VARIANT varInterfaceNames,
            /* [in] */ VARIANT varInterfaceFlags,
            /* [in] */ VARIANT varInterfaces) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInterfaceValue( 
            /* [in] */ __RPC__in BSTR bstrInterfaceName,
            /* [out] */ __RPC__out LONG *lInterfaceFlag,
            /* [out] */ __RPC__out VARIANT *varInterface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR bstrInterfaceName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAll( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out unsigned long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzBizRuleInterfacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzBizRuleInterfaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzBizRuleInterfaces * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzBizRuleInterfaces * This,
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
        
        DECLSPEC_XFGVIRT(IAzBizRuleInterfaces, AddInterface)
        HRESULT ( STDMETHODCALLTYPE *AddInterface )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ __RPC__in BSTR bstrInterfaceName,
            /* [in] */ LONG lInterfaceFlag,
            /* [in] */ VARIANT varInterface);
        
        DECLSPEC_XFGVIRT(IAzBizRuleInterfaces, AddInterfaces)
        HRESULT ( STDMETHODCALLTYPE *AddInterfaces )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ VARIANT varInterfaceNames,
            /* [in] */ VARIANT varInterfaceFlags,
            /* [in] */ VARIANT varInterfaces);
        
        DECLSPEC_XFGVIRT(IAzBizRuleInterfaces, GetInterfaceValue)
        HRESULT ( STDMETHODCALLTYPE *GetInterfaceValue )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ __RPC__in BSTR bstrInterfaceName,
            /* [out] */ __RPC__out LONG *lInterfaceFlag,
            /* [out] */ __RPC__out VARIANT *varInterface);
        
        DECLSPEC_XFGVIRT(IAzBizRuleInterfaces, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [in] */ __RPC__in BSTR bstrInterfaceName);
        
        DECLSPEC_XFGVIRT(IAzBizRuleInterfaces, RemoveAll)
        HRESULT ( STDMETHODCALLTYPE *RemoveAll )( 
            __RPC__in IAzBizRuleInterfaces * This);
        
        DECLSPEC_XFGVIRT(IAzBizRuleInterfaces, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzBizRuleInterfaces * This,
            /* [retval][out] */ __RPC__out unsigned long *plCount);
        
        END_INTERFACE
    } IAzBizRuleInterfacesVtbl;

    interface IAzBizRuleInterfaces
    {
        CONST_VTBL struct IAzBizRuleInterfacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzBizRuleInterfaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzBizRuleInterfaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzBizRuleInterfaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzBizRuleInterfaces_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzBizRuleInterfaces_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzBizRuleInterfaces_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzBizRuleInterfaces_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzBizRuleInterfaces_AddInterface(This,bstrInterfaceName,lInterfaceFlag,varInterface)	\
    ( (This)->lpVtbl -> AddInterface(This,bstrInterfaceName,lInterfaceFlag,varInterface) ) 

#define IAzBizRuleInterfaces_AddInterfaces(This,varInterfaceNames,varInterfaceFlags,varInterfaces)	\
    ( (This)->lpVtbl -> AddInterfaces(This,varInterfaceNames,varInterfaceFlags,varInterfaces) ) 

#define IAzBizRuleInterfaces_GetInterfaceValue(This,bstrInterfaceName,lInterfaceFlag,varInterface)	\
    ( (This)->lpVtbl -> GetInterfaceValue(This,bstrInterfaceName,lInterfaceFlag,varInterface) ) 

#define IAzBizRuleInterfaces_Remove(This,bstrInterfaceName)	\
    ( (This)->lpVtbl -> Remove(This,bstrInterfaceName) ) 

#define IAzBizRuleInterfaces_RemoveAll(This)	\
    ( (This)->lpVtbl -> RemoveAll(This) ) 

#define IAzBizRuleInterfaces_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzBizRuleInterfaces_INTERFACE_DEFINED__ */


#ifndef __IAzClientContext3_INTERFACE_DEFINED__
#define __IAzClientContext3_INTERFACE_DEFINED__

/* interface IAzClientContext3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzClientContext3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11894fde-1deb-4b4b-8907-6d1cda1f5d4f")
    IAzClientContext3 : public IAzClientContext2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AccessCheck2( 
            /* [in] */ __RPC__in BSTR bstrObjectName,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ long lOperation,
            /* [retval][out] */ __RPC__out unsigned long *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsInRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOperations( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperations **ppOperationCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTasks( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRuleParameters( 
            /* [retval][out] */ __RPC__deref_out_opt IAzBizRuleParameters **ppBizRuleParam) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRuleInterfaces( 
            /* [retval][out] */ __RPC__deref_out_opt IAzBizRuleInterfaces **ppBizRuleInterfaces) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGroups( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ ULONG ulOptions,
            /* [retval][out] */ __RPC__out VARIANT *pGroupArray) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Sids( 
            /* [retval][out] */ __RPC__out VARIANT *pStringSidArray) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzClientContext3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzClientContext3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzClientContext3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzClientContext3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzClientContext3 * This,
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
        
        DECLSPEC_XFGVIRT(IAzClientContext, AccessCheck)
        HRESULT ( STDMETHODCALLTYPE *AccessCheck )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrObjectName,
            /* [in] */ VARIANT varScopeNames,
            /* [in] */ VARIANT varOperations,
            /* [optional][in] */ VARIANT varParameterNames,
            /* [optional][in] */ VARIANT varParameterValues,
            /* [optional][in] */ VARIANT varInterfaceNames,
            /* [optional][in] */ VARIANT varInterfaceFlags,
            /* [optional][in] */ VARIANT varInterfaces,
            /* [retval][out] */ __RPC__out VARIANT *pvarResults);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetBusinessRuleString)
        HRESULT ( STDMETHODCALLTYPE *GetBusinessRuleString )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBusinessRuleString);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDn)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDn )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserSamCompat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSamCompat )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDisplay)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDisplay )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserGuid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserGuid )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserCanonical)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserCanonical )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserUpn)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserUpn )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_UserDnsSamCompat)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDnsSamCompat )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, GetRoles)
        HRESULT ( STDMETHODCALLTYPE *GetRoles )( 
            __RPC__in IAzClientContext3 * This,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__out VARIANT *pvarRoleNames);
        
        DECLSPEC_XFGVIRT(IAzClientContext, get_RoleForAccessCheck)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleForAccessCheck )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext, put_RoleForAccessCheck)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RoleForAccessCheck )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, GetAssignedScopesPage)
        HRESULT ( STDMETHODCALLTYPE *GetAssignedScopesPage )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ LONG lOptions,
            /* [in] */ LONG PageSize,
            /* [out][in] */ __RPC__inout VARIANT *pvarCursor,
            /* [retval][out] */ __RPC__out VARIANT *pvarScopeNames);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, AddRoles)
        HRESULT ( STDMETHODCALLTYPE *AddRoles )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ VARIANT varRoles,
            /* [in] */ __RPC__in BSTR bstrScopeName);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, AddApplicationGroups)
        HRESULT ( STDMETHODCALLTYPE *AddApplicationGroups )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ VARIANT varApplicationGroups);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, AddStringSids)
        HRESULT ( STDMETHODCALLTYPE *AddStringSids )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ VARIANT varStringSids);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, put_LDAPQueryDN)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LDAPQueryDN )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrLDAPQueryDN);
        
        DECLSPEC_XFGVIRT(IAzClientContext2, get_LDAPQueryDN)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LDAPQueryDN )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrLDAPQueryDN);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, AccessCheck2)
        HRESULT ( STDMETHODCALLTYPE *AccessCheck2 )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrObjectName,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ long lOperation,
            /* [retval][out] */ __RPC__out unsigned long *plResult);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, IsInRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *IsInRoleAssignment )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIsInRole);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, GetOperations)
        HRESULT ( STDMETHODCALLTYPE *GetOperations )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperations **ppOperationCollection);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, GetTasks)
        HRESULT ( STDMETHODCALLTYPE *GetTasks )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, get_BizRuleParameters)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleParameters )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzBizRuleParameters **ppBizRuleParam);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, get_BizRuleInterfaces)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleInterfaces )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzBizRuleInterfaces **ppBizRuleInterfaces);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, GetGroups)
        HRESULT ( STDMETHODCALLTYPE *GetGroups )( 
            __RPC__in IAzClientContext3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ ULONG ulOptions,
            /* [retval][out] */ __RPC__out VARIANT *pGroupArray);
        
        DECLSPEC_XFGVIRT(IAzClientContext3, get_Sids)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sids )( 
            __RPC__in IAzClientContext3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pStringSidArray);
        
        END_INTERFACE
    } IAzClientContext3Vtbl;

    interface IAzClientContext3
    {
        CONST_VTBL struct IAzClientContext3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzClientContext3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzClientContext3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzClientContext3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzClientContext3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzClientContext3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzClientContext3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzClientContext3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzClientContext3_AccessCheck(This,bstrObjectName,varScopeNames,varOperations,varParameterNames,varParameterValues,varInterfaceNames,varInterfaceFlags,varInterfaces,pvarResults)	\
    ( (This)->lpVtbl -> AccessCheck(This,bstrObjectName,varScopeNames,varOperations,varParameterNames,varParameterValues,varInterfaceNames,varInterfaceFlags,varInterfaces,pvarResults) ) 

#define IAzClientContext3_GetBusinessRuleString(This,pbstrBusinessRuleString)	\
    ( (This)->lpVtbl -> GetBusinessRuleString(This,pbstrBusinessRuleString) ) 

#define IAzClientContext3_get_UserDn(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDn(This,pbstrProp) ) 

#define IAzClientContext3_get_UserSamCompat(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserSamCompat(This,pbstrProp) ) 

#define IAzClientContext3_get_UserDisplay(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDisplay(This,pbstrProp) ) 

#define IAzClientContext3_get_UserGuid(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserGuid(This,pbstrProp) ) 

#define IAzClientContext3_get_UserCanonical(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserCanonical(This,pbstrProp) ) 

#define IAzClientContext3_get_UserUpn(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserUpn(This,pbstrProp) ) 

#define IAzClientContext3_get_UserDnsSamCompat(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_UserDnsSamCompat(This,pbstrProp) ) 

#define IAzClientContext3_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzClientContext3_GetRoles(This,bstrScopeName,pvarRoleNames)	\
    ( (This)->lpVtbl -> GetRoles(This,bstrScopeName,pvarRoleNames) ) 

#define IAzClientContext3_get_RoleForAccessCheck(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_RoleForAccessCheck(This,pbstrProp) ) 

#define IAzClientContext3_put_RoleForAccessCheck(This,bstrProp)	\
    ( (This)->lpVtbl -> put_RoleForAccessCheck(This,bstrProp) ) 


#define IAzClientContext3_GetAssignedScopesPage(This,lOptions,PageSize,pvarCursor,pvarScopeNames)	\
    ( (This)->lpVtbl -> GetAssignedScopesPage(This,lOptions,PageSize,pvarCursor,pvarScopeNames) ) 

#define IAzClientContext3_AddRoles(This,varRoles,bstrScopeName)	\
    ( (This)->lpVtbl -> AddRoles(This,varRoles,bstrScopeName) ) 

#define IAzClientContext3_AddApplicationGroups(This,varApplicationGroups)	\
    ( (This)->lpVtbl -> AddApplicationGroups(This,varApplicationGroups) ) 

#define IAzClientContext3_AddStringSids(This,varStringSids)	\
    ( (This)->lpVtbl -> AddStringSids(This,varStringSids) ) 

#define IAzClientContext3_put_LDAPQueryDN(This,bstrLDAPQueryDN)	\
    ( (This)->lpVtbl -> put_LDAPQueryDN(This,bstrLDAPQueryDN) ) 

#define IAzClientContext3_get_LDAPQueryDN(This,pbstrLDAPQueryDN)	\
    ( (This)->lpVtbl -> get_LDAPQueryDN(This,pbstrLDAPQueryDN) ) 


#define IAzClientContext3_AccessCheck2(This,bstrObjectName,bstrScopeName,lOperation,plResult)	\
    ( (This)->lpVtbl -> AccessCheck2(This,bstrObjectName,bstrScopeName,lOperation,plResult) ) 

#define IAzClientContext3_IsInRoleAssignment(This,bstrScopeName,bstrRoleName,pbIsInRole)	\
    ( (This)->lpVtbl -> IsInRoleAssignment(This,bstrScopeName,bstrRoleName,pbIsInRole) ) 

#define IAzClientContext3_GetOperations(This,bstrScopeName,ppOperationCollection)	\
    ( (This)->lpVtbl -> GetOperations(This,bstrScopeName,ppOperationCollection) ) 

#define IAzClientContext3_GetTasks(This,bstrScopeName,ppTaskCollection)	\
    ( (This)->lpVtbl -> GetTasks(This,bstrScopeName,ppTaskCollection) ) 

#define IAzClientContext3_get_BizRuleParameters(This,ppBizRuleParam)	\
    ( (This)->lpVtbl -> get_BizRuleParameters(This,ppBizRuleParam) ) 

#define IAzClientContext3_get_BizRuleInterfaces(This,ppBizRuleInterfaces)	\
    ( (This)->lpVtbl -> get_BizRuleInterfaces(This,ppBizRuleInterfaces) ) 

#define IAzClientContext3_GetGroups(This,bstrScopeName,ulOptions,pGroupArray)	\
    ( (This)->lpVtbl -> GetGroups(This,bstrScopeName,ulOptions,pGroupArray) ) 

#define IAzClientContext3_get_Sids(This,pStringSidArray)	\
    ( (This)->lpVtbl -> get_Sids(This,pStringSidArray) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzClientContext3_INTERFACE_DEFINED__ */


#ifndef __IAzScope2_INTERFACE_DEFINED__
#define __IAzScope2_INTERFACE_DEFINED__

/* interface IAzScope2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzScope2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ee9fe8c9-c9f3-40e2-aa12-d1d8599727fd")
    IAzScope2 : public IAzScope
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleDefinitions( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleAssignments( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzScope2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzScope2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzScope2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzScope2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzScope2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzScope, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzScope, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzScope, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzScope, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzScope, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzScope, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzScope, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzScope, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzScope, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzScope, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Roles)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Roles )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection);
        
        DECLSPEC_XFGVIRT(IAzScope, OpenRole)
        HRESULT ( STDMETHODCALLTYPE *OpenRole )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzScope, CreateRole)
        HRESULT ( STDMETHODCALLTYPE *CreateRole )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzScope, DeleteRole)
        HRESULT ( STDMETHODCALLTYPE *DeleteRole )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection);
        
        DECLSPEC_XFGVIRT(IAzScope, OpenTask)
        HRESULT ( STDMETHODCALLTYPE *OpenTask )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzScope, CreateTask)
        HRESULT ( STDMETHODCALLTYPE *CreateTask )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzScope, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzScope2 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, get_CanBeDelegated)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanBeDelegated )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzScope, get_BizrulesWritable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizrulesWritable )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzScope, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzScope2, get_RoleDefinitions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleDefinitions )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzScope2, CreateRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *CreateRoleDefinition )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzScope2, OpenRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *OpenRoleDefinition )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzScope2, DeleteRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *DeleteRoleDefinition )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName);
        
        DECLSPEC_XFGVIRT(IAzScope2, get_RoleAssignments)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleAssignments )( 
            __RPC__in IAzScope2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments);
        
        DECLSPEC_XFGVIRT(IAzScope2, CreateRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *CreateRoleAssignment )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment);
        
        DECLSPEC_XFGVIRT(IAzScope2, OpenRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *OpenRoleAssignment )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment);
        
        DECLSPEC_XFGVIRT(IAzScope2, DeleteRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *DeleteRoleAssignment )( 
            __RPC__in IAzScope2 * This,
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName);
        
        END_INTERFACE
    } IAzScope2Vtbl;

    interface IAzScope2
    {
        CONST_VTBL struct IAzScope2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzScope2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzScope2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzScope2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzScope2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzScope2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzScope2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzScope2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzScope2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzScope2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzScope2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzScope2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzScope2_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzScope2_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzScope2_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzScope2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzScope2_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzScope2_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzScope2_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzScope2_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzScope2_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzScope2_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzScope2_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzScope2_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzScope2_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzScope2_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzScope2_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzScope2_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzScope2_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzScope2_get_Roles(This,ppRoleCollection)	\
    ( (This)->lpVtbl -> get_Roles(This,ppRoleCollection) ) 

#define IAzScope2_OpenRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> OpenRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzScope2_CreateRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> CreateRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzScope2_DeleteRole(This,bstrRoleName,varReserved)	\
    ( (This)->lpVtbl -> DeleteRole(This,bstrRoleName,varReserved) ) 

#define IAzScope2_get_Tasks(This,ppTaskCollection)	\
    ( (This)->lpVtbl -> get_Tasks(This,ppTaskCollection) ) 

#define IAzScope2_OpenTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> OpenTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzScope2_CreateTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> CreateTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzScope2_DeleteTask(This,bstrTaskName,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTaskName,varReserved) ) 

#define IAzScope2_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzScope2_get_CanBeDelegated(This,pfProp)	\
    ( (This)->lpVtbl -> get_CanBeDelegated(This,pfProp) ) 

#define IAzScope2_get_BizrulesWritable(This,pfProp)	\
    ( (This)->lpVtbl -> get_BizrulesWritable(This,pfProp) ) 

#define IAzScope2_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzScope2_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzScope2_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzScope2_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzScope2_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzScope2_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 


#define IAzScope2_get_RoleDefinitions(This,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> get_RoleDefinitions(This,ppRoleDefinitions) ) 

#define IAzScope2_CreateRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> CreateRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions) ) 

#define IAzScope2_OpenRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> OpenRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions) ) 

#define IAzScope2_DeleteRoleDefinition(This,bstrRoleDefinitionName)	\
    ( (This)->lpVtbl -> DeleteRoleDefinition(This,bstrRoleDefinitionName) ) 

#define IAzScope2_get_RoleAssignments(This,ppRoleAssignments)	\
    ( (This)->lpVtbl -> get_RoleAssignments(This,ppRoleAssignments) ) 

#define IAzScope2_CreateRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment)	\
    ( (This)->lpVtbl -> CreateRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment) ) 

#define IAzScope2_OpenRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment)	\
    ( (This)->lpVtbl -> OpenRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment) ) 

#define IAzScope2_DeleteRoleAssignment(This,bstrRoleAssignmentName)	\
    ( (This)->lpVtbl -> DeleteRoleAssignment(This,bstrRoleAssignmentName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzScope2_INTERFACE_DEFINED__ */


#ifndef __IAzApplication3_INTERFACE_DEFINED__
#define __IAzApplication3_INTERFACE_DEFINED__

/* interface IAzApplication3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplication3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("181c845e-7196-4a7d-ac2e-020c0bb7a303")
    IAzApplication3 : public IAzApplication2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ScopeExists( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbExist) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenScope2( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope2 **ppScope2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateScope2( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope2 **ppScope2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteScope2( 
            /* [in] */ __RPC__in BSTR bstrScopeName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleDefinitions( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleAssignments( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRoleAssignment( 
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRulesEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRulesEnabled( 
            /* [in] */ VARIANT_BOOL bEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplication3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplication3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplication3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplication3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplication3 * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_AuthzInterfaceClsid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuthzInterfaceClsid )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_AuthzInterfaceClsid)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AuthzInterfaceClsid )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Version)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_Version)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_GenerateAudits)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerateAudits )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_GenerateAudits)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_GenerateAudits )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplyStoreSacl)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplyStoreSacl )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out BOOL *pbProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, put_ApplyStoreSacl)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplyStoreSacl )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ BOOL bProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplication, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyAdministrators)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministrators )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyReaders)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReaders )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministrator )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyAdministrator)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministrator )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyReader)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReader )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyReader)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReader )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Scopes)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Scopes )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzScopes **ppScopeCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenScope)
        HRESULT ( STDMETHODCALLTYPE *OpenScope )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateScope)
        HRESULT ( STDMETHODCALLTYPE *CreateScope )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteScope)
        HRESULT ( STDMETHODCALLTYPE *DeleteScope )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperations **ppOperationCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenOperation)
        HRESULT ( STDMETHODCALLTYPE *OpenOperation )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateOperation)
        HRESULT ( STDMETHODCALLTYPE *CreateOperation )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzOperation **ppOperation);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrOperationName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzTasks **ppTaskCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenTask)
        HRESULT ( STDMETHODCALLTYPE *OpenTask )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateTask)
        HRESULT ( STDMETHODCALLTYPE *CreateTask )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzTask **ppTask);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrTaskName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_ApplicationGroups)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationGroups )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroups **ppGroupCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *OpenApplicationGroup )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationGroup )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzApplicationGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteApplicationGroup)
        HRESULT ( STDMETHODCALLTYPE *DeleteApplicationGroup )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_Roles)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Roles )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoles **ppRoleCollection);
        
        DECLSPEC_XFGVIRT(IAzApplication, OpenRole)
        HRESULT ( STDMETHODCALLTYPE *OpenRole )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzApplication, CreateRole)
        HRESULT ( STDMETHODCALLTYPE *CreateRole )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzRole **ppRole);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteRole)
        HRESULT ( STDMETHODCALLTYPE *DeleteRole )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleName,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromToken)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromToken )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ ULONGLONG ullTokenHandle,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzApplication3 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromName)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR ClientName,
            /* [defaultvalue][in] */ __RPC__in BSTR DomainName,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_DelegatedPolicyUsers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsers )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUser )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteDelegatedPolicyUser)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUser )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, InitializeClientContextFromStringSid)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromStringSid )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR SidString,
            /* [in] */ LONG lOptions,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyAdministratorsName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyAdministratorsName )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarAdmins);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_PolicyReadersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolicyReadersName )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReaders);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyAdministratorName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyAdministratorName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyAdministratorName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrAdmin,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddPolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *AddPolicyReaderName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeletePolicyReaderName)
        HRESULT ( STDMETHODCALLTYPE *DeletePolicyReaderName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrReader,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, get_DelegatedPolicyUsersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DelegatedPolicyUsersName )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarDelegatedPolicyUsers);
        
        DECLSPEC_XFGVIRT(IAzApplication, AddDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *AddDelegatedPolicyUserName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication, DeleteDelegatedPolicyUserName)
        HRESULT ( STDMETHODCALLTYPE *DeleteDelegatedPolicyUserName )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrDelegatedPolicyUser,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplication2, InitializeClientContextFromToken2)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContextFromToken2 )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ ULONG ulTokenHandleLowPart,
            /* [in] */ ULONG ulTokenHandleHighPart,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext2 **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication2, InitializeClientContext2)
        HRESULT ( STDMETHODCALLTYPE *InitializeClientContext2 )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR IdentifyingString,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__deref_out_opt IAzClientContext2 **ppClientContext);
        
        DECLSPEC_XFGVIRT(IAzApplication3, ScopeExists)
        HRESULT ( STDMETHODCALLTYPE *ScopeExists )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbExist);
        
        DECLSPEC_XFGVIRT(IAzApplication3, OpenScope2)
        HRESULT ( STDMETHODCALLTYPE *OpenScope2 )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope2 **ppScope2);
        
        DECLSPEC_XFGVIRT(IAzApplication3, CreateScope2)
        HRESULT ( STDMETHODCALLTYPE *CreateScope2 )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope2 **ppScope2);
        
        DECLSPEC_XFGVIRT(IAzApplication3, DeleteScope2)
        HRESULT ( STDMETHODCALLTYPE *DeleteScope2 )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName);
        
        DECLSPEC_XFGVIRT(IAzApplication3, get_RoleDefinitions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleDefinitions )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzApplication3, CreateRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *CreateRoleDefinition )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzApplication3, OpenRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *OpenRoleDefinition )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinition **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzApplication3, DeleteRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *DeleteRoleDefinition )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinitionName);
        
        DECLSPEC_XFGVIRT(IAzApplication3, get_RoleAssignments)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleAssignments )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments);
        
        DECLSPEC_XFGVIRT(IAzApplication3, CreateRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *CreateRoleAssignment )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment);
        
        DECLSPEC_XFGVIRT(IAzApplication3, OpenRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *OpenRoleAssignment )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignment **ppRoleAssignment);
        
        DECLSPEC_XFGVIRT(IAzApplication3, DeleteRoleAssignment)
        HRESULT ( STDMETHODCALLTYPE *DeleteRoleAssignment )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ __RPC__in BSTR bstrRoleAssignmentName);
        
        DECLSPEC_XFGVIRT(IAzApplication3, get_BizRulesEnabled)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRulesEnabled )( 
            __RPC__in IAzApplication3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled);
        
        DECLSPEC_XFGVIRT(IAzApplication3, put_BizRulesEnabled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRulesEnabled )( 
            __RPC__in IAzApplication3 * This,
            /* [in] */ VARIANT_BOOL bEnabled);
        
        END_INTERFACE
    } IAzApplication3Vtbl;

    interface IAzApplication3
    {
        CONST_VTBL struct IAzApplication3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplication3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplication3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplication3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplication3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplication3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplication3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplication3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplication3_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzApplication3_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzApplication3_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzApplication3_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzApplication3_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzApplication3_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzApplication3_get_AuthzInterfaceClsid(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_AuthzInterfaceClsid(This,pbstrProp) ) 

#define IAzApplication3_put_AuthzInterfaceClsid(This,bstrProp)	\
    ( (This)->lpVtbl -> put_AuthzInterfaceClsid(This,bstrProp) ) 

#define IAzApplication3_get_Version(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_Version(This,pbstrProp) ) 

#define IAzApplication3_put_Version(This,bstrProp)	\
    ( (This)->lpVtbl -> put_Version(This,bstrProp) ) 

#define IAzApplication3_get_GenerateAudits(This,pbProp)	\
    ( (This)->lpVtbl -> get_GenerateAudits(This,pbProp) ) 

#define IAzApplication3_put_GenerateAudits(This,bProp)	\
    ( (This)->lpVtbl -> put_GenerateAudits(This,bProp) ) 

#define IAzApplication3_get_ApplyStoreSacl(This,pbProp)	\
    ( (This)->lpVtbl -> get_ApplyStoreSacl(This,pbProp) ) 

#define IAzApplication3_put_ApplyStoreSacl(This,bProp)	\
    ( (This)->lpVtbl -> put_ApplyStoreSacl(This,bProp) ) 

#define IAzApplication3_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzApplication3_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzApplication3_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzApplication3_get_PolicyAdministrators(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministrators(This,pvarAdmins) ) 

#define IAzApplication3_get_PolicyReaders(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReaders(This,pvarReaders) ) 

#define IAzApplication3_AddPolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzApplication3_DeletePolicyAdministrator(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministrator(This,bstrAdmin,varReserved) ) 

#define IAzApplication3_AddPolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReader(This,bstrReader,varReserved) ) 

#define IAzApplication3_DeletePolicyReader(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReader(This,bstrReader,varReserved) ) 

#define IAzApplication3_get_Scopes(This,ppScopeCollection)	\
    ( (This)->lpVtbl -> get_Scopes(This,ppScopeCollection) ) 

#define IAzApplication3_OpenScope(This,bstrScopeName,varReserved,ppScope)	\
    ( (This)->lpVtbl -> OpenScope(This,bstrScopeName,varReserved,ppScope) ) 

#define IAzApplication3_CreateScope(This,bstrScopeName,varReserved,ppScope)	\
    ( (This)->lpVtbl -> CreateScope(This,bstrScopeName,varReserved,ppScope) ) 

#define IAzApplication3_DeleteScope(This,bstrScopeName,varReserved)	\
    ( (This)->lpVtbl -> DeleteScope(This,bstrScopeName,varReserved) ) 

#define IAzApplication3_get_Operations(This,ppOperationCollection)	\
    ( (This)->lpVtbl -> get_Operations(This,ppOperationCollection) ) 

#define IAzApplication3_OpenOperation(This,bstrOperationName,varReserved,ppOperation)	\
    ( (This)->lpVtbl -> OpenOperation(This,bstrOperationName,varReserved,ppOperation) ) 

#define IAzApplication3_CreateOperation(This,bstrOperationName,varReserved,ppOperation)	\
    ( (This)->lpVtbl -> CreateOperation(This,bstrOperationName,varReserved,ppOperation) ) 

#define IAzApplication3_DeleteOperation(This,bstrOperationName,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrOperationName,varReserved) ) 

#define IAzApplication3_get_Tasks(This,ppTaskCollection)	\
    ( (This)->lpVtbl -> get_Tasks(This,ppTaskCollection) ) 

#define IAzApplication3_OpenTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> OpenTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzApplication3_CreateTask(This,bstrTaskName,varReserved,ppTask)	\
    ( (This)->lpVtbl -> CreateTask(This,bstrTaskName,varReserved,ppTask) ) 

#define IAzApplication3_DeleteTask(This,bstrTaskName,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTaskName,varReserved) ) 

#define IAzApplication3_get_ApplicationGroups(This,ppGroupCollection)	\
    ( (This)->lpVtbl -> get_ApplicationGroups(This,ppGroupCollection) ) 

#define IAzApplication3_OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> OpenApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzApplication3_CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup)	\
    ( (This)->lpVtbl -> CreateApplicationGroup(This,bstrGroupName,varReserved,ppGroup) ) 

#define IAzApplication3_DeleteApplicationGroup(This,bstrGroupName,varReserved)	\
    ( (This)->lpVtbl -> DeleteApplicationGroup(This,bstrGroupName,varReserved) ) 

#define IAzApplication3_get_Roles(This,ppRoleCollection)	\
    ( (This)->lpVtbl -> get_Roles(This,ppRoleCollection) ) 

#define IAzApplication3_OpenRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> OpenRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzApplication3_CreateRole(This,bstrRoleName,varReserved,ppRole)	\
    ( (This)->lpVtbl -> CreateRole(This,bstrRoleName,varReserved,ppRole) ) 

#define IAzApplication3_DeleteRole(This,bstrRoleName,varReserved)	\
    ( (This)->lpVtbl -> DeleteRole(This,bstrRoleName,varReserved) ) 

#define IAzApplication3_InitializeClientContextFromToken(This,ullTokenHandle,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromToken(This,ullTokenHandle,varReserved,ppClientContext) ) 

#define IAzApplication3_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplication3_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplication3_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzApplication3_InitializeClientContextFromName(This,ClientName,DomainName,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromName(This,ClientName,DomainName,varReserved,ppClientContext) ) 

#define IAzApplication3_get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsers(This,pvarDelegatedPolicyUsers) ) 

#define IAzApplication3_AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication3_DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUser(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication3_InitializeClientContextFromStringSid(This,SidString,lOptions,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromStringSid(This,SidString,lOptions,varReserved,ppClientContext) ) 

#define IAzApplication3_get_PolicyAdministratorsName(This,pvarAdmins)	\
    ( (This)->lpVtbl -> get_PolicyAdministratorsName(This,pvarAdmins) ) 

#define IAzApplication3_get_PolicyReadersName(This,pvarReaders)	\
    ( (This)->lpVtbl -> get_PolicyReadersName(This,pvarReaders) ) 

#define IAzApplication3_AddPolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzApplication3_DeletePolicyAdministratorName(This,bstrAdmin,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyAdministratorName(This,bstrAdmin,varReserved) ) 

#define IAzApplication3_AddPolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> AddPolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzApplication3_DeletePolicyReaderName(This,bstrReader,varReserved)	\
    ( (This)->lpVtbl -> DeletePolicyReaderName(This,bstrReader,varReserved) ) 

#define IAzApplication3_get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers)	\
    ( (This)->lpVtbl -> get_DelegatedPolicyUsersName(This,pvarDelegatedPolicyUsers) ) 

#define IAzApplication3_AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> AddDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 

#define IAzApplication3_DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved)	\
    ( (This)->lpVtbl -> DeleteDelegatedPolicyUserName(This,bstrDelegatedPolicyUser,varReserved) ) 


#define IAzApplication3_InitializeClientContextFromToken2(This,ulTokenHandleLowPart,ulTokenHandleHighPart,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContextFromToken2(This,ulTokenHandleLowPart,ulTokenHandleHighPart,varReserved,ppClientContext) ) 

#define IAzApplication3_InitializeClientContext2(This,IdentifyingString,varReserved,ppClientContext)	\
    ( (This)->lpVtbl -> InitializeClientContext2(This,IdentifyingString,varReserved,ppClientContext) ) 


#define IAzApplication3_ScopeExists(This,bstrScopeName,pbExist)	\
    ( (This)->lpVtbl -> ScopeExists(This,bstrScopeName,pbExist) ) 

#define IAzApplication3_OpenScope2(This,bstrScopeName,ppScope2)	\
    ( (This)->lpVtbl -> OpenScope2(This,bstrScopeName,ppScope2) ) 

#define IAzApplication3_CreateScope2(This,bstrScopeName,ppScope2)	\
    ( (This)->lpVtbl -> CreateScope2(This,bstrScopeName,ppScope2) ) 

#define IAzApplication3_DeleteScope2(This,bstrScopeName)	\
    ( (This)->lpVtbl -> DeleteScope2(This,bstrScopeName) ) 

#define IAzApplication3_get_RoleDefinitions(This,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> get_RoleDefinitions(This,ppRoleDefinitions) ) 

#define IAzApplication3_CreateRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> CreateRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions) ) 

#define IAzApplication3_OpenRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> OpenRoleDefinition(This,bstrRoleDefinitionName,ppRoleDefinitions) ) 

#define IAzApplication3_DeleteRoleDefinition(This,bstrRoleDefinitionName)	\
    ( (This)->lpVtbl -> DeleteRoleDefinition(This,bstrRoleDefinitionName) ) 

#define IAzApplication3_get_RoleAssignments(This,ppRoleAssignments)	\
    ( (This)->lpVtbl -> get_RoleAssignments(This,ppRoleAssignments) ) 

#define IAzApplication3_CreateRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment)	\
    ( (This)->lpVtbl -> CreateRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment) ) 

#define IAzApplication3_OpenRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment)	\
    ( (This)->lpVtbl -> OpenRoleAssignment(This,bstrRoleAssignmentName,ppRoleAssignment) ) 

#define IAzApplication3_DeleteRoleAssignment(This,bstrRoleAssignmentName)	\
    ( (This)->lpVtbl -> DeleteRoleAssignment(This,bstrRoleAssignmentName) ) 

#define IAzApplication3_get_BizRulesEnabled(This,pbEnabled)	\
    ( (This)->lpVtbl -> get_BizRulesEnabled(This,pbEnabled) ) 

#define IAzApplication3_put_BizRulesEnabled(This,bEnabled)	\
    ( (This)->lpVtbl -> put_BizRulesEnabled(This,bEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplication3_INTERFACE_DEFINED__ */


#ifndef __IAzOperation2_INTERFACE_DEFINED__
#define __IAzOperation2_INTERFACE_DEFINED__

/* interface IAzOperation2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzOperation2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1f5ea01f-44a2-4184-9c48-a75b4dcc8ccc")
    IAzOperation2 : public IAzOperation
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RoleAssignments( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzOperation2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzOperation2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzOperation2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzOperation2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzOperation2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzOperation, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzOperation2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzOperation2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzOperation2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_OperationID)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_OperationID )( 
            __RPC__in IAzOperation2 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, put_OperationID)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_OperationID )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzOperation2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzOperation, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzOperation, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzOperation2 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzOperation2, RoleAssignments)
        HRESULT ( STDMETHODCALLTYPE *RoleAssignments )( 
            __RPC__in IAzOperation2 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments);
        
        END_INTERFACE
    } IAzOperation2Vtbl;

    interface IAzOperation2
    {
        CONST_VTBL struct IAzOperation2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzOperation2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzOperation2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzOperation2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzOperation2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzOperation2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzOperation2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzOperation2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzOperation2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzOperation2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzOperation2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzOperation2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzOperation2_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzOperation2_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzOperation2_get_OperationID(This,plProp)	\
    ( (This)->lpVtbl -> get_OperationID(This,plProp) ) 

#define IAzOperation2_put_OperationID(This,lProp)	\
    ( (This)->lpVtbl -> put_OperationID(This,lProp) ) 

#define IAzOperation2_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzOperation2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzOperation2_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzOperation2_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 


#define IAzOperation2_RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments)	\
    ( (This)->lpVtbl -> RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzOperation2_INTERFACE_DEFINED__ */


#ifndef __IAzRoleDefinitions_INTERFACE_DEFINED__
#define __IAzRoleDefinitions_INTERFACE_DEFINED__

/* interface IAzRoleDefinitions */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzRoleDefinitions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("881f25a5-d755-4550-957a-d503a3b34001")
    IAzRoleDefinitions : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzRoleDefinitionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzRoleDefinitions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzRoleDefinitions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzRoleDefinitions * This,
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
        
        DECLSPEC_XFGVIRT(IAzRoleDefinitions, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzRoleDefinitions, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzRoleDefinitions, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzRoleDefinitions * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzRoleDefinitionsVtbl;

    interface IAzRoleDefinitions
    {
        CONST_VTBL struct IAzRoleDefinitionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzRoleDefinitions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzRoleDefinitions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzRoleDefinitions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzRoleDefinitions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzRoleDefinitions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzRoleDefinitions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzRoleDefinitions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzRoleDefinitions_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzRoleDefinitions_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzRoleDefinitions_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzRoleDefinitions_INTERFACE_DEFINED__ */


#ifndef __IAzRoleDefinition_INTERFACE_DEFINED__
#define __IAzRoleDefinition_INTERFACE_DEFINED__

/* interface IAzRoleDefinition */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzRoleDefinition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d97fcea1-2599-44f1-9fc3-58e9fbe09466")
    IAzRoleDefinition : public IAzTask
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RoleAssignments( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinition) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleDefinitions( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzRoleDefinitionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzRoleDefinition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzRoleDefinition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzRoleDefinition * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzRoleDefinition * This,
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
        
        DECLSPEC_XFGVIRT(IAzTask, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzTask, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzTask, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzTask, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzTask, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRule)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRule )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRule)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRule )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRuleLanguage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleLanguage )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRuleLanguage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleLanguage )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRuleImportedPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleImportedPath )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRuleImportedPath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleImportedPath )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_IsRoleDefinition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsRoleDefinition )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_IsRoleDefinition)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsRoleDefinition )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ BOOL fProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, AddOperation)
        HRESULT ( STDMETHODCALLTYPE *AddOperation )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, AddTask)
        HRESULT ( STDMETHODCALLTYPE *AddTask )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzTask, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzRoleDefinition * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRoleDefinition, RoleAssignments)
        HRESULT ( STDMETHODCALLTYPE *RoleAssignments )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments);
        
        DECLSPEC_XFGVIRT(IAzRoleDefinition, AddRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *AddRoleDefinition )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinition);
        
        DECLSPEC_XFGVIRT(IAzRoleDefinition, DeleteRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *DeleteRoleDefinition )( 
            __RPC__in IAzRoleDefinition * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinition);
        
        DECLSPEC_XFGVIRT(IAzRoleDefinition, get_RoleDefinitions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleDefinitions )( 
            __RPC__in IAzRoleDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions);
        
        END_INTERFACE
    } IAzRoleDefinitionVtbl;

    interface IAzRoleDefinition
    {
        CONST_VTBL struct IAzRoleDefinitionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzRoleDefinition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzRoleDefinition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzRoleDefinition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzRoleDefinition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzRoleDefinition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzRoleDefinition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzRoleDefinition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzRoleDefinition_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzRoleDefinition_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzRoleDefinition_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzRoleDefinition_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzRoleDefinition_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzRoleDefinition_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzRoleDefinition_get_BizRule(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRule(This,pbstrProp) ) 

#define IAzRoleDefinition_put_BizRule(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRule(This,bstrProp) ) 

#define IAzRoleDefinition_get_BizRuleLanguage(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleLanguage(This,pbstrProp) ) 

#define IAzRoleDefinition_put_BizRuleLanguage(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleLanguage(This,bstrProp) ) 

#define IAzRoleDefinition_get_BizRuleImportedPath(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleImportedPath(This,pbstrProp) ) 

#define IAzRoleDefinition_put_BizRuleImportedPath(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleImportedPath(This,bstrProp) ) 

#define IAzRoleDefinition_get_IsRoleDefinition(This,pfProp)	\
    ( (This)->lpVtbl -> get_IsRoleDefinition(This,pfProp) ) 

#define IAzRoleDefinition_put_IsRoleDefinition(This,fProp)	\
    ( (This)->lpVtbl -> put_IsRoleDefinition(This,fProp) ) 

#define IAzRoleDefinition_get_Operations(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Operations(This,pvarProp) ) 

#define IAzRoleDefinition_get_Tasks(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Tasks(This,pvarProp) ) 

#define IAzRoleDefinition_AddOperation(This,bstrOp,varReserved)	\
    ( (This)->lpVtbl -> AddOperation(This,bstrOp,varReserved) ) 

#define IAzRoleDefinition_DeleteOperation(This,bstrOp,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrOp,varReserved) ) 

#define IAzRoleDefinition_AddTask(This,bstrTask,varReserved)	\
    ( (This)->lpVtbl -> AddTask(This,bstrTask,varReserved) ) 

#define IAzRoleDefinition_DeleteTask(This,bstrTask,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTask,varReserved) ) 

#define IAzRoleDefinition_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzRoleDefinition_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzRoleDefinition_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzRoleDefinition_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzRoleDefinition_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzRoleDefinition_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 


#define IAzRoleDefinition_RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments)	\
    ( (This)->lpVtbl -> RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments) ) 

#define IAzRoleDefinition_AddRoleDefinition(This,bstrRoleDefinition)	\
    ( (This)->lpVtbl -> AddRoleDefinition(This,bstrRoleDefinition) ) 

#define IAzRoleDefinition_DeleteRoleDefinition(This,bstrRoleDefinition)	\
    ( (This)->lpVtbl -> DeleteRoleDefinition(This,bstrRoleDefinition) ) 

#define IAzRoleDefinition_get_RoleDefinitions(This,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> get_RoleDefinitions(This,ppRoleDefinitions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzRoleDefinition_INTERFACE_DEFINED__ */


#ifndef __IAzRoleAssignment_INTERFACE_DEFINED__
#define __IAzRoleAssignment_INTERFACE_DEFINED__

/* interface IAzRoleAssignment */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzRoleAssignment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55647d31-0d5a-4fa3-b4ac-2b5f9ad5ab76")
    IAzRoleAssignment : public IAzRole
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteRoleDefinition( 
            /* [in] */ __RPC__in BSTR bstrRoleDefinition) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RoleDefinitions( 
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Scope( 
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzRoleAssignmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzRoleAssignment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzRoleAssignment * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzRoleAssignment * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzRoleAssignment * This,
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
        
        DECLSPEC_XFGVIRT(IAzRole, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzRole, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzRole, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzRole, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzRole, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzRole, AddAppMember)
        HRESULT ( STDMETHODCALLTYPE *AddAppMember )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteAppMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteAppMember )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddTask)
        HRESULT ( STDMETHODCALLTYPE *AddTask )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddOperation)
        HRESULT ( STDMETHODCALLTYPE *AddOperation )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddMember)
        HRESULT ( STDMETHODCALLTYPE *AddMember )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteMember )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzRole, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, get_AppMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppMembers )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Members)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRole, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzRoleAssignment * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, AddMemberName)
        HRESULT ( STDMETHODCALLTYPE *AddMemberName )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, DeleteMemberName)
        HRESULT ( STDMETHODCALLTYPE *DeleteMemberName )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzRole, get_MembersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MembersName )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzRoleAssignment, AddRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *AddRoleDefinition )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinition);
        
        DECLSPEC_XFGVIRT(IAzRoleAssignment, DeleteRoleDefinition)
        HRESULT ( STDMETHODCALLTYPE *DeleteRoleDefinition )( 
            __RPC__in IAzRoleAssignment * This,
            /* [in] */ __RPC__in BSTR bstrRoleDefinition);
        
        DECLSPEC_XFGVIRT(IAzRoleAssignment, get_RoleDefinitions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoleDefinitions )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleDefinitions **ppRoleDefinitions);
        
        DECLSPEC_XFGVIRT(IAzRoleAssignment, get_Scope)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Scope )( 
            __RPC__in IAzRoleAssignment * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzScope **ppScope);
        
        END_INTERFACE
    } IAzRoleAssignmentVtbl;

    interface IAzRoleAssignment
    {
        CONST_VTBL struct IAzRoleAssignmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzRoleAssignment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzRoleAssignment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzRoleAssignment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzRoleAssignment_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzRoleAssignment_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzRoleAssignment_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzRoleAssignment_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzRoleAssignment_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzRoleAssignment_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzRoleAssignment_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzRoleAssignment_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzRoleAssignment_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzRoleAssignment_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzRoleAssignment_AddAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddAppMember(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_DeleteAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteAppMember(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_AddTask(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddTask(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_DeleteTask(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_AddOperation(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddOperation(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_DeleteOperation(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_AddMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMember(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_DeleteMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMember(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzRoleAssignment_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzRoleAssignment_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzRoleAssignment_get_AppMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_AppMembers(This,pvarProp) ) 

#define IAzRoleAssignment_get_Members(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Members(This,pvarProp) ) 

#define IAzRoleAssignment_get_Operations(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Operations(This,pvarProp) ) 

#define IAzRoleAssignment_get_Tasks(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Tasks(This,pvarProp) ) 

#define IAzRoleAssignment_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzRoleAssignment_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzRoleAssignment_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzRoleAssignment_AddMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMemberName(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_DeleteMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMemberName(This,bstrProp,varReserved) ) 

#define IAzRoleAssignment_get_MembersName(This,pvarProp)	\
    ( (This)->lpVtbl -> get_MembersName(This,pvarProp) ) 


#define IAzRoleAssignment_AddRoleDefinition(This,bstrRoleDefinition)	\
    ( (This)->lpVtbl -> AddRoleDefinition(This,bstrRoleDefinition) ) 

#define IAzRoleAssignment_DeleteRoleDefinition(This,bstrRoleDefinition)	\
    ( (This)->lpVtbl -> DeleteRoleDefinition(This,bstrRoleDefinition) ) 

#define IAzRoleAssignment_get_RoleDefinitions(This,ppRoleDefinitions)	\
    ( (This)->lpVtbl -> get_RoleDefinitions(This,ppRoleDefinitions) ) 

#define IAzRoleAssignment_get_Scope(This,ppScope)	\
    ( (This)->lpVtbl -> get_Scope(This,ppScope) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzRoleAssignment_INTERFACE_DEFINED__ */


#ifndef __IAzRoleAssignments_INTERFACE_DEFINED__
#define __IAzRoleAssignments_INTERFACE_DEFINED__

/* interface IAzRoleAssignments */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzRoleAssignments;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9c80b900-fceb-4d73-a0f4-c83b0bbf2481")
    IAzRoleAssignments : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out LONG *plCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzRoleAssignmentsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzRoleAssignments * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzRoleAssignments * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzRoleAssignments * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzRoleAssignments * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzRoleAssignments * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzRoleAssignments * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzRoleAssignments * This,
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
        
        DECLSPEC_XFGVIRT(IAzRoleAssignments, get_Item)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IAzRoleAssignments * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out VARIANT *pvarObtPtr);
        
        DECLSPEC_XFGVIRT(IAzRoleAssignments, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IAzRoleAssignments * This,
            /* [retval][out] */ __RPC__out LONG *plCount);
        
        DECLSPEC_XFGVIRT(IAzRoleAssignments, get__NewEnum)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IAzRoleAssignments * This,
            /* [retval][out] */ __RPC__deref_out_opt LPUNKNOWN *ppEnumPtr);
        
        END_INTERFACE
    } IAzRoleAssignmentsVtbl;

    interface IAzRoleAssignments
    {
        CONST_VTBL struct IAzRoleAssignmentsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzRoleAssignments_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzRoleAssignments_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzRoleAssignments_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzRoleAssignments_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzRoleAssignments_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzRoleAssignments_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzRoleAssignments_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzRoleAssignments_get_Item(This,Index,pvarObtPtr)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pvarObtPtr) ) 

#define IAzRoleAssignments_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IAzRoleAssignments_get__NewEnum(This,ppEnumPtr)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumPtr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzRoleAssignments_INTERFACE_DEFINED__ */


#ifndef __IAzPrincipalLocator_INTERFACE_DEFINED__
#define __IAzPrincipalLocator_INTERFACE_DEFINED__

/* interface IAzPrincipalLocator */
/* [unique][dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAzPrincipalLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e5c3507d-ad6a-4992-9c7f-74ab480b44cc")
    IAzPrincipalLocator : public IDispatch
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NameResolver( 
            /* [retval][out] */ __RPC__deref_out_opt IAzNameResolver **ppNameResolver) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ObjectPicker( 
            /* [retval][out] */ __RPC__deref_out_opt IAzObjectPicker **ppObjectPicker) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzPrincipalLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzPrincipalLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzPrincipalLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzPrincipalLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzPrincipalLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzPrincipalLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzPrincipalLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzPrincipalLocator * This,
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
        
        DECLSPEC_XFGVIRT(IAzPrincipalLocator, get_NameResolver)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NameResolver )( 
            __RPC__in IAzPrincipalLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzNameResolver **ppNameResolver);
        
        DECLSPEC_XFGVIRT(IAzPrincipalLocator, get_ObjectPicker)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectPicker )( 
            __RPC__in IAzPrincipalLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt IAzObjectPicker **ppObjectPicker);
        
        END_INTERFACE
    } IAzPrincipalLocatorVtbl;

    interface IAzPrincipalLocator
    {
        CONST_VTBL struct IAzPrincipalLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzPrincipalLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzPrincipalLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzPrincipalLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzPrincipalLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzPrincipalLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzPrincipalLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzPrincipalLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzPrincipalLocator_get_NameResolver(This,ppNameResolver)	\
    ( (This)->lpVtbl -> get_NameResolver(This,ppNameResolver) ) 

#define IAzPrincipalLocator_get_ObjectPicker(This,ppObjectPicker)	\
    ( (This)->lpVtbl -> get_ObjectPicker(This,ppObjectPicker) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzPrincipalLocator_INTERFACE_DEFINED__ */


#ifndef __IAzNameResolver_INTERFACE_DEFINED__
#define __IAzNameResolver_INTERFACE_DEFINED__

/* interface IAzNameResolver */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzNameResolver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("504d0f15-73e2-43df-a870-a64f40714f53")
    IAzNameResolver : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NameFromSid( 
            /* [in] */ __RPC__in BSTR bstrSid,
            /* [out] */ __RPC__out long *pSidType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NamesFromSids( 
            /* [in] */ VARIANT vSids,
            /* [out] */ __RPC__out VARIANT *pvSidTypes,
            /* [retval][out] */ __RPC__out VARIANT *pvNames) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzNameResolverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzNameResolver * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzNameResolver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzNameResolver * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzNameResolver * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzNameResolver * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzNameResolver * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzNameResolver * This,
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
        
        DECLSPEC_XFGVIRT(IAzNameResolver, NameFromSid)
        HRESULT ( STDMETHODCALLTYPE *NameFromSid )( 
            __RPC__in IAzNameResolver * This,
            /* [in] */ __RPC__in BSTR bstrSid,
            /* [out] */ __RPC__out long *pSidType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzNameResolver, NamesFromSids)
        HRESULT ( STDMETHODCALLTYPE *NamesFromSids )( 
            __RPC__in IAzNameResolver * This,
            /* [in] */ VARIANT vSids,
            /* [out] */ __RPC__out VARIANT *pvSidTypes,
            /* [retval][out] */ __RPC__out VARIANT *pvNames);
        
        END_INTERFACE
    } IAzNameResolverVtbl;

    interface IAzNameResolver
    {
        CONST_VTBL struct IAzNameResolverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzNameResolver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzNameResolver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzNameResolver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzNameResolver_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzNameResolver_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzNameResolver_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzNameResolver_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzNameResolver_NameFromSid(This,bstrSid,pSidType,pbstrName)	\
    ( (This)->lpVtbl -> NameFromSid(This,bstrSid,pSidType,pbstrName) ) 

#define IAzNameResolver_NamesFromSids(This,vSids,pvSidTypes,pvNames)	\
    ( (This)->lpVtbl -> NamesFromSids(This,vSids,pvSidTypes,pvNames) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzNameResolver_INTERFACE_DEFINED__ */


#ifndef __IAzObjectPicker_INTERFACE_DEFINED__
#define __IAzObjectPicker_INTERFACE_DEFINED__

/* interface IAzObjectPicker */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzObjectPicker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63130a48-699a-42d8-bf01-c62ac3fb79f9")
    IAzObjectPicker : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPrincipals( 
            /* [in] */ __RPC__in HWND hParentWnd,
            /* [in] */ __RPC__in BSTR bstrTitle,
            /* [out] */ __RPC__out VARIANT *pvSidTypes,
            /* [out] */ __RPC__out VARIANT *pvNames,
            /* [retval][out] */ __RPC__out VARIANT *pvSids) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzObjectPickerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzObjectPicker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzObjectPicker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzObjectPicker * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzObjectPicker * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzObjectPicker * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzObjectPicker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzObjectPicker * This,
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
        
        DECLSPEC_XFGVIRT(IAzObjectPicker, GetPrincipals)
        HRESULT ( STDMETHODCALLTYPE *GetPrincipals )( 
            __RPC__in IAzObjectPicker * This,
            /* [in] */ __RPC__in HWND hParentWnd,
            /* [in] */ __RPC__in BSTR bstrTitle,
            /* [out] */ __RPC__out VARIANT *pvSidTypes,
            /* [out] */ __RPC__out VARIANT *pvNames,
            /* [retval][out] */ __RPC__out VARIANT *pvSids);
        
        DECLSPEC_XFGVIRT(IAzObjectPicker, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzObjectPicker * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        END_INTERFACE
    } IAzObjectPickerVtbl;

    interface IAzObjectPicker
    {
        CONST_VTBL struct IAzObjectPickerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzObjectPicker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzObjectPicker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzObjectPicker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzObjectPicker_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzObjectPicker_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzObjectPicker_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzObjectPicker_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzObjectPicker_GetPrincipals(This,hParentWnd,bstrTitle,pvSidTypes,pvNames,pvSids)	\
    ( (This)->lpVtbl -> GetPrincipals(This,hParentWnd,bstrTitle,pvSidTypes,pvNames,pvSids) ) 

#define IAzObjectPicker_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzObjectPicker_INTERFACE_DEFINED__ */


#ifndef __IAzApplicationGroup2_INTERFACE_DEFINED__
#define __IAzApplicationGroup2_INTERFACE_DEFINED__

/* interface IAzApplicationGroup2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzApplicationGroup2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3f0613fc-b71a-464e-a11d-5b881a56cefa")
    IAzApplicationGroup2 : public IAzApplicationGroup
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRule( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRule( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRuleLanguage( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRuleLanguage( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_BizRuleImportedPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_BizRuleImportedPath( 
            /* [in] */ __RPC__in BSTR bstrProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RoleAssignments( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzApplicationGroup2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzApplicationGroup2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzApplicationGroup2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzApplicationGroup2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Type)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out LONG *plProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_Type)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ LONG lProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_LdapQuery)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LdapQuery )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_LdapQuery)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_LdapQuery )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_AppMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppMembers )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_AppNonMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppNonMembers )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Members)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Members )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_NonMembers)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NonMembers )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddAppMember)
        HRESULT ( STDMETHODCALLTYPE *AddAppMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteAppMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteAppMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddAppNonMember)
        HRESULT ( STDMETHODCALLTYPE *AddAppNonMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteAppNonMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteAppNonMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddMember)
        HRESULT ( STDMETHODCALLTYPE *AddMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddNonMember)
        HRESULT ( STDMETHODCALLTYPE *AddNonMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteNonMember)
        HRESULT ( STDMETHODCALLTYPE *DeleteNonMember )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddMemberName)
        HRESULT ( STDMETHODCALLTYPE *AddMemberName )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteMemberName)
        HRESULT ( STDMETHODCALLTYPE *DeleteMemberName )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, AddNonMemberName)
        HRESULT ( STDMETHODCALLTYPE *AddNonMemberName )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, DeleteNonMemberName)
        HRESULT ( STDMETHODCALLTYPE *DeleteNonMemberName )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_MembersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_MembersName )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup, get_NonMembersName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NonMembersName )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, get_BizRule)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRule )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, put_BizRule)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRule )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, get_BizRuleLanguage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleLanguage )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, put_BizRuleLanguage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleLanguage )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, get_BizRuleImportedPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleImportedPath )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, put_BizRuleImportedPath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleImportedPath )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzApplicationGroup2, RoleAssignments)
        HRESULT ( STDMETHODCALLTYPE *RoleAssignments )( 
            __RPC__in IAzApplicationGroup2 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments);
        
        END_INTERFACE
    } IAzApplicationGroup2Vtbl;

    interface IAzApplicationGroup2
    {
        CONST_VTBL struct IAzApplicationGroup2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzApplicationGroup2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzApplicationGroup2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzApplicationGroup2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzApplicationGroup2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzApplicationGroup2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzApplicationGroup2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzApplicationGroup2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzApplicationGroup2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzApplicationGroup2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzApplicationGroup2_get_Type(This,plProp)	\
    ( (This)->lpVtbl -> get_Type(This,plProp) ) 

#define IAzApplicationGroup2_put_Type(This,lProp)	\
    ( (This)->lpVtbl -> put_Type(This,lProp) ) 

#define IAzApplicationGroup2_get_LdapQuery(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_LdapQuery(This,pbstrProp) ) 

#define IAzApplicationGroup2_put_LdapQuery(This,bstrProp)	\
    ( (This)->lpVtbl -> put_LdapQuery(This,bstrProp) ) 

#define IAzApplicationGroup2_get_AppMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_AppMembers(This,pvarProp) ) 

#define IAzApplicationGroup2_get_AppNonMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_AppNonMembers(This,pvarProp) ) 

#define IAzApplicationGroup2_get_Members(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Members(This,pvarProp) ) 

#define IAzApplicationGroup2_get_NonMembers(This,pvarProp)	\
    ( (This)->lpVtbl -> get_NonMembers(This,pvarProp) ) 

#define IAzApplicationGroup2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzApplicationGroup2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzApplicationGroup2_AddAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddAppMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_DeleteAppMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteAppMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_AddAppNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddAppNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_DeleteAppNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteAppNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_AddMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_DeleteMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_AddNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_DeleteNonMember(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteNonMember(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzApplicationGroup2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzApplicationGroup2_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzApplicationGroup2_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplicationGroup2_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzApplicationGroup2_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 

#define IAzApplicationGroup2_AddMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_DeleteMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_AddNonMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> AddNonMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_DeleteNonMemberName(This,bstrProp,varReserved)	\
    ( (This)->lpVtbl -> DeleteNonMemberName(This,bstrProp,varReserved) ) 

#define IAzApplicationGroup2_get_MembersName(This,pvarProp)	\
    ( (This)->lpVtbl -> get_MembersName(This,pvarProp) ) 

#define IAzApplicationGroup2_get_NonMembersName(This,pvarProp)	\
    ( (This)->lpVtbl -> get_NonMembersName(This,pvarProp) ) 


#define IAzApplicationGroup2_get_BizRule(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRule(This,pbstrProp) ) 

#define IAzApplicationGroup2_put_BizRule(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRule(This,bstrProp) ) 

#define IAzApplicationGroup2_get_BizRuleLanguage(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleLanguage(This,pbstrProp) ) 

#define IAzApplicationGroup2_put_BizRuleLanguage(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleLanguage(This,bstrProp) ) 

#define IAzApplicationGroup2_get_BizRuleImportedPath(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleImportedPath(This,pbstrProp) ) 

#define IAzApplicationGroup2_put_BizRuleImportedPath(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleImportedPath(This,bstrProp) ) 

#define IAzApplicationGroup2_RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments)	\
    ( (This)->lpVtbl -> RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzApplicationGroup2_INTERFACE_DEFINED__ */


#ifndef __IAzTask2_INTERFACE_DEFINED__
#define __IAzTask2_INTERFACE_DEFINED__

/* interface IAzTask2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAzTask2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03a9a5ee-48c8-4832-9025-aad503c46526")
    IAzTask2 : public IAzTask
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RoleAssignments( 
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAzTask2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAzTask2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAzTask2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAzTask2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAzTask2 * This,
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
        
        DECLSPEC_XFGVIRT(IAzTask, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IAzTask, put_Name)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IAzTask, put_Description)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IAzTask, get_ApplicationData)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationData )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzTask, put_ApplicationData)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ApplicationData )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationData);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRule)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRule )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRule)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRule )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRuleLanguage)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleLanguage )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRuleLanguage)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleLanguage )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_BizRuleImportedPath)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_BizRuleImportedPath )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_BizRuleImportedPath)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_BizRuleImportedPath )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_IsRoleDefinition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsRoleDefinition )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzTask, put_IsRoleDefinition)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsRoleDefinition )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ BOOL fProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Operations)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operations )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Tasks)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tasks )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, AddOperation)
        HRESULT ( STDMETHODCALLTYPE *AddOperation )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeleteOperation)
        HRESULT ( STDMETHODCALLTYPE *DeleteOperation )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrOp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, AddTask)
        HRESULT ( STDMETHODCALLTYPE *AddTask )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeleteTask)
        HRESULT ( STDMETHODCALLTYPE *DeleteTask )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrTask,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, get_Writable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Writable )( 
            __RPC__in IAzTask2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfProp);
        
        DECLSPEC_XFGVIRT(IAzTask, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ LONG lPropId,
            /* [optional][in] */ VARIANT varReserved,
            /* [retval][out] */ __RPC__out VARIANT *pvarProp);
        
        DECLSPEC_XFGVIRT(IAzTask, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, AddPropertyItem)
        HRESULT ( STDMETHODCALLTYPE *AddPropertyItem )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, DeletePropertyItem)
        HRESULT ( STDMETHODCALLTYPE *DeletePropertyItem )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ LONG lPropId,
            /* [in] */ VARIANT varProp,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask, Submit)
        HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IAzTask2 * This,
            /* [defaultvalue][in] */ LONG lFlags,
            /* [optional][in] */ VARIANT varReserved);
        
        DECLSPEC_XFGVIRT(IAzTask2, RoleAssignments)
        HRESULT ( STDMETHODCALLTYPE *RoleAssignments )( 
            __RPC__in IAzTask2 * This,
            /* [in] */ __RPC__in BSTR bstrScopeName,
            /* [in] */ VARIANT_BOOL bRecursive,
            /* [retval][out] */ __RPC__deref_out_opt IAzRoleAssignments **ppRoleAssignments);
        
        END_INTERFACE
    } IAzTask2Vtbl;

    interface IAzTask2
    {
        CONST_VTBL struct IAzTask2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAzTask2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAzTask2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAzTask2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAzTask2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAzTask2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAzTask2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAzTask2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAzTask2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IAzTask2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IAzTask2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IAzTask2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IAzTask2_get_ApplicationData(This,pbstrApplicationData)	\
    ( (This)->lpVtbl -> get_ApplicationData(This,pbstrApplicationData) ) 

#define IAzTask2_put_ApplicationData(This,bstrApplicationData)	\
    ( (This)->lpVtbl -> put_ApplicationData(This,bstrApplicationData) ) 

#define IAzTask2_get_BizRule(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRule(This,pbstrProp) ) 

#define IAzTask2_put_BizRule(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRule(This,bstrProp) ) 

#define IAzTask2_get_BizRuleLanguage(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleLanguage(This,pbstrProp) ) 

#define IAzTask2_put_BizRuleLanguage(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleLanguage(This,bstrProp) ) 

#define IAzTask2_get_BizRuleImportedPath(This,pbstrProp)	\
    ( (This)->lpVtbl -> get_BizRuleImportedPath(This,pbstrProp) ) 

#define IAzTask2_put_BizRuleImportedPath(This,bstrProp)	\
    ( (This)->lpVtbl -> put_BizRuleImportedPath(This,bstrProp) ) 

#define IAzTask2_get_IsRoleDefinition(This,pfProp)	\
    ( (This)->lpVtbl -> get_IsRoleDefinition(This,pfProp) ) 

#define IAzTask2_put_IsRoleDefinition(This,fProp)	\
    ( (This)->lpVtbl -> put_IsRoleDefinition(This,fProp) ) 

#define IAzTask2_get_Operations(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Operations(This,pvarProp) ) 

#define IAzTask2_get_Tasks(This,pvarProp)	\
    ( (This)->lpVtbl -> get_Tasks(This,pvarProp) ) 

#define IAzTask2_AddOperation(This,bstrOp,varReserved)	\
    ( (This)->lpVtbl -> AddOperation(This,bstrOp,varReserved) ) 

#define IAzTask2_DeleteOperation(This,bstrOp,varReserved)	\
    ( (This)->lpVtbl -> DeleteOperation(This,bstrOp,varReserved) ) 

#define IAzTask2_AddTask(This,bstrTask,varReserved)	\
    ( (This)->lpVtbl -> AddTask(This,bstrTask,varReserved) ) 

#define IAzTask2_DeleteTask(This,bstrTask,varReserved)	\
    ( (This)->lpVtbl -> DeleteTask(This,bstrTask,varReserved) ) 

#define IAzTask2_get_Writable(This,pfProp)	\
    ( (This)->lpVtbl -> get_Writable(This,pfProp) ) 

#define IAzTask2_GetProperty(This,lPropId,varReserved,pvarProp)	\
    ( (This)->lpVtbl -> GetProperty(This,lPropId,varReserved,pvarProp) ) 

#define IAzTask2_SetProperty(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> SetProperty(This,lPropId,varProp,varReserved) ) 

#define IAzTask2_AddPropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> AddPropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzTask2_DeletePropertyItem(This,lPropId,varProp,varReserved)	\
    ( (This)->lpVtbl -> DeletePropertyItem(This,lPropId,varProp,varReserved) ) 

#define IAzTask2_Submit(This,lFlags,varReserved)	\
    ( (This)->lpVtbl -> Submit(This,lFlags,varReserved) ) 


#define IAzTask2_RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments)	\
    ( (This)->lpVtbl -> RoleAssignments(This,bstrScopeName,bRecursive,ppRoleAssignments) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAzTask2_INTERFACE_DEFINED__ */



#ifndef __AZROLESLib_LIBRARY_DEFINED__
#define __AZROLESLib_LIBRARY_DEFINED__

/* library AZROLESLib */
/* [helpstring][version][uuid] */ 



































// List of poperty IDs for Az objects.

typedef 
enum tagAZ_PROP_CONSTANTS
    {
        AZ_PROP_NAME	= 1,
        AZ_PROP_DESCRIPTION	= 2,
        AZ_PROP_WRITABLE	= 3,
        AZ_PROP_APPLICATION_DATA	= 4,
        AZ_PROP_CHILD_CREATE	= 5,
        AZ_MAX_APPLICATION_NAME_LENGTH	= 512,
        AZ_MAX_OPERATION_NAME_LENGTH	= 64,
        AZ_MAX_TASK_NAME_LENGTH	= 64,
        AZ_MAX_SCOPE_NAME_LENGTH	= 65536,
        AZ_MAX_GROUP_NAME_LENGTH	= 64,
        AZ_MAX_ROLE_NAME_LENGTH	= 64,
        AZ_MAX_NAME_LENGTH	= 65536,
        AZ_MAX_DESCRIPTION_LENGTH	= 1024,
        AZ_MAX_APPLICATION_DATA_LENGTH	= 4096,
        AZ_SUBMIT_FLAG_ABORT	= 0x1,
        AZ_SUBMIT_FLAG_FLUSH	= 0x2,
        AZ_MAX_POLICY_URL_LENGTH	= 65536,
        AZ_AZSTORE_FLAG_CREATE	= 0x1,
        AZ_AZSTORE_FLAG_MANAGE_STORE_ONLY	= 0x2,
        AZ_AZSTORE_FLAG_BATCH_UPDATE	= 0x4,
        AZ_AZSTORE_FLAG_AUDIT_IS_CRITICAL	= 0x8,
        AZ_AZSTORE_FORCE_APPLICATION_CLOSE	= 0x10,
        AZ_AZSTORE_NT6_FUNCTION_LEVEL	= 0x20,
        AZ_AZSTORE_FLAG_MANAGE_ONLY_PASSIVE_SUBMIT	= 0x8000,
        AZ_PROP_AZSTORE_DOMAIN_TIMEOUT	= 100,
        AZ_AZSTORE_DEFAULT_DOMAIN_TIMEOUT	= ( 15 * 1000 ) ,
        AZ_PROP_AZSTORE_SCRIPT_ENGINE_TIMEOUT	= 101,
        AZ_AZSTORE_MIN_DOMAIN_TIMEOUT	= 500,
        AZ_AZSTORE_MIN_SCRIPT_ENGINE_TIMEOUT	= ( 5 * 1000 ) ,
        AZ_AZSTORE_DEFAULT_SCRIPT_ENGINE_TIMEOUT	= ( 45 * 1000 ) ,
        AZ_PROP_AZSTORE_MAX_SCRIPT_ENGINES	= 102,
        AZ_AZSTORE_DEFAULT_MAX_SCRIPT_ENGINES	= 120,
        AZ_PROP_AZSTORE_MAJOR_VERSION	= 103,
        AZ_PROP_AZSTORE_MINOR_VERSION	= 104,
        AZ_PROP_AZSTORE_TARGET_MACHINE	= 105,
        AZ_PROP_AZTORE_IS_ADAM_INSTANCE	= 106,
        AZ_PROP_OPERATION_ID	= 200,
        AZ_PROP_TASK_OPERATIONS	= 300,
        AZ_PROP_TASK_BIZRULE	= 301,
        AZ_PROP_TASK_BIZRULE_LANGUAGE	= 302,
        AZ_PROP_TASK_TASKS	= 303,
        AZ_PROP_TASK_BIZRULE_IMPORTED_PATH	= 304,
        AZ_PROP_TASK_IS_ROLE_DEFINITION	= 305,
        AZ_MAX_TASK_BIZRULE_LENGTH	= 65536,
        AZ_MAX_TASK_BIZRULE_LANGUAGE_LENGTH	= 64,
        AZ_MAX_TASK_BIZRULE_IMPORTED_PATH_LENGTH	= 512,
        AZ_MAX_BIZRULE_STRING	= 65536,
        AZ_PROP_GROUP_TYPE	= 400,
        AZ_GROUPTYPE_LDAP_QUERY	= 1,
        AZ_GROUPTYPE_BASIC	= 2,
        AZ_GROUPTYPE_BIZRULE	= 3,
        AZ_PROP_GROUP_APP_MEMBERS	= 401,
        AZ_PROP_GROUP_APP_NON_MEMBERS	= 402,
        AZ_PROP_GROUP_LDAP_QUERY	= 403,
        AZ_MAX_GROUP_LDAP_QUERY_LENGTH	= 4096,
        AZ_PROP_GROUP_MEMBERS	= 404,
        AZ_PROP_GROUP_NON_MEMBERS	= 405,
        AZ_PROP_GROUP_MEMBERS_NAME	= 406,
        AZ_PROP_GROUP_NON_MEMBERS_NAME	= 407,
        AZ_PROP_GROUP_BIZRULE	= 408,
        AZ_PROP_GROUP_BIZRULE_LANGUAGE	= 409,
        AZ_PROP_GROUP_BIZRULE_IMPORTED_PATH	= 410,
        AZ_MAX_GROUP_BIZRULE_LENGTH	= 65536,
        AZ_MAX_GROUP_BIZRULE_LANGUAGE_LENGTH	= 64,
        AZ_MAX_GROUP_BIZRULE_IMPORTED_PATH_LENGTH	= 512,
        AZ_PROP_ROLE_APP_MEMBERS	= 500,
        AZ_PROP_ROLE_MEMBERS	= 501,
        AZ_PROP_ROLE_OPERATIONS	= 502,
        AZ_PROP_ROLE_TASKS	= 504,
        AZ_PROP_ROLE_MEMBERS_NAME	= 505,
        AZ_PROP_SCOPE_BIZRULES_WRITABLE	= 600,
        AZ_PROP_SCOPE_CAN_BE_DELEGATED	= 601,
        AZ_PROP_CLIENT_CONTEXT_USER_DN	= 700,
        AZ_PROP_CLIENT_CONTEXT_USER_SAM_COMPAT	= 701,
        AZ_PROP_CLIENT_CONTEXT_USER_DISPLAY	= 702,
        AZ_PROP_CLIENT_CONTEXT_USER_GUID	= 703,
        AZ_PROP_CLIENT_CONTEXT_USER_CANONICAL	= 704,
        AZ_PROP_CLIENT_CONTEXT_USER_UPN	= 705,
        AZ_PROP_CLIENT_CONTEXT_USER_DNS_SAM_COMPAT	= 707,
        AZ_PROP_CLIENT_CONTEXT_ROLE_FOR_ACCESS_CHECK	= 708,
        AZ_PROP_CLIENT_CONTEXT_LDAP_QUERY_DN	= 709,
        AZ_PROP_APPLICATION_AUTHZ_INTERFACE_CLSID	= 800,
        AZ_PROP_APPLICATION_VERSION	= 801,
        AZ_MAX_APPLICATION_VERSION_LENGTH	= 512,
        AZ_PROP_APPLICATION_NAME	= 802,
        AZ_PROP_APPLICATION_BIZRULE_ENABLED	= 803,
        AZ_PROP_APPLY_STORE_SACL	= 900,
        AZ_PROP_GENERATE_AUDITS	= 901,
        AZ_PROP_POLICY_ADMINS	= 902,
        AZ_PROP_POLICY_READERS	= 903,
        AZ_PROP_DELEGATED_POLICY_USERS	= 904,
        AZ_PROP_POLICY_ADMINS_NAME	= 905,
        AZ_PROP_POLICY_READERS_NAME	= 906,
        AZ_PROP_DELEGATED_POLICY_USERS_NAME	= 907,
        AZ_CLIENT_CONTEXT_SKIP_GROUP	= 1,
        AZ_CLIENT_CONTEXT_SKIP_LDAP_QUERY	= 1,
        AZ_CLIENT_CONTEXT_GET_GROUP_RECURSIVE	= 2,
        AZ_CLIENT_CONTEXT_GET_GROUPS_STORE_LEVEL_ONLY	= 2
    } 	AZ_PROP_CONSTANTS;


EXTERN_C const IID LIBID_AZROLESLib;

EXTERN_C const CLSID CLSID_AzAuthorizationStore;

#ifdef __cplusplus

class DECLSPEC_UUID("b2bcff59-a757-4b0b-a1bc-ea69981da69e")
AzAuthorizationStore;
#endif

EXTERN_C const CLSID CLSID_AzBizRuleContext;

#ifdef __cplusplus

class DECLSPEC_UUID("5c2dc96f-8d51-434b-b33c-379bccae77c3")
AzBizRuleContext;
#endif

EXTERN_C const CLSID CLSID_AzPrincipalLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("483afb5d-70df-4e16-abdc-a1de4d015a3e")
AzPrincipalLocator;
#endif
#endif /* __AZROLESLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_azroles_0000_0035 */
/* [local] */ 

#ifndef OLESCRIPT_E_SYNTAX
#define OLESCRIPT_E_SYNTAX _HRESULT_TYPEDEF_(0x80020101L)
#endif // OLESCRIPT_E_SYNTAX
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_azroles_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_azroles_0000_0035_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


