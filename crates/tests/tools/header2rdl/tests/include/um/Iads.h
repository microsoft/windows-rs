

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


#ifndef __iads_h__
#define __iads_h__

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

#ifndef __IADs_FWD_DEFINED__
#define __IADs_FWD_DEFINED__
typedef interface IADs IADs;

#endif 	/* __IADs_FWD_DEFINED__ */


#ifndef __IADsContainer_FWD_DEFINED__
#define __IADsContainer_FWD_DEFINED__
typedef interface IADsContainer IADsContainer;

#endif 	/* __IADsContainer_FWD_DEFINED__ */


#ifndef __IADsCollection_FWD_DEFINED__
#define __IADsCollection_FWD_DEFINED__
typedef interface IADsCollection IADsCollection;

#endif 	/* __IADsCollection_FWD_DEFINED__ */


#ifndef __IADsMembers_FWD_DEFINED__
#define __IADsMembers_FWD_DEFINED__
typedef interface IADsMembers IADsMembers;

#endif 	/* __IADsMembers_FWD_DEFINED__ */


#ifndef __IADsPropertyList_FWD_DEFINED__
#define __IADsPropertyList_FWD_DEFINED__
typedef interface IADsPropertyList IADsPropertyList;

#endif 	/* __IADsPropertyList_FWD_DEFINED__ */


#ifndef __IADsPropertyEntry_FWD_DEFINED__
#define __IADsPropertyEntry_FWD_DEFINED__
typedef interface IADsPropertyEntry IADsPropertyEntry;

#endif 	/* __IADsPropertyEntry_FWD_DEFINED__ */


#ifndef __PropertyEntry_FWD_DEFINED__
#define __PropertyEntry_FWD_DEFINED__

#ifdef __cplusplus
typedef class PropertyEntry PropertyEntry;
#else
typedef struct PropertyEntry PropertyEntry;
#endif /* __cplusplus */

#endif 	/* __PropertyEntry_FWD_DEFINED__ */


#ifndef __IADsPropertyValue_FWD_DEFINED__
#define __IADsPropertyValue_FWD_DEFINED__
typedef interface IADsPropertyValue IADsPropertyValue;

#endif 	/* __IADsPropertyValue_FWD_DEFINED__ */


#ifndef __IADsPropertyValue2_FWD_DEFINED__
#define __IADsPropertyValue2_FWD_DEFINED__
typedef interface IADsPropertyValue2 IADsPropertyValue2;

#endif 	/* __IADsPropertyValue2_FWD_DEFINED__ */


#ifndef __PropertyValue_FWD_DEFINED__
#define __PropertyValue_FWD_DEFINED__

#ifdef __cplusplus
typedef class PropertyValue PropertyValue;
#else
typedef struct PropertyValue PropertyValue;
#endif /* __cplusplus */

#endif 	/* __PropertyValue_FWD_DEFINED__ */


#ifndef __IPrivateDispatch_FWD_DEFINED__
#define __IPrivateDispatch_FWD_DEFINED__
typedef interface IPrivateDispatch IPrivateDispatch;

#endif 	/* __IPrivateDispatch_FWD_DEFINED__ */


#ifndef __IPrivateUnknown_FWD_DEFINED__
#define __IPrivateUnknown_FWD_DEFINED__
typedef interface IPrivateUnknown IPrivateUnknown;

#endif 	/* __IPrivateUnknown_FWD_DEFINED__ */


#ifndef __IADsExtension_FWD_DEFINED__
#define __IADsExtension_FWD_DEFINED__
typedef interface IADsExtension IADsExtension;

#endif 	/* __IADsExtension_FWD_DEFINED__ */


#ifndef __IADsDeleteOps_FWD_DEFINED__
#define __IADsDeleteOps_FWD_DEFINED__
typedef interface IADsDeleteOps IADsDeleteOps;

#endif 	/* __IADsDeleteOps_FWD_DEFINED__ */


#ifndef __IADsNamespaces_FWD_DEFINED__
#define __IADsNamespaces_FWD_DEFINED__
typedef interface IADsNamespaces IADsNamespaces;

#endif 	/* __IADsNamespaces_FWD_DEFINED__ */


#ifndef __IADsClass_FWD_DEFINED__
#define __IADsClass_FWD_DEFINED__
typedef interface IADsClass IADsClass;

#endif 	/* __IADsClass_FWD_DEFINED__ */


#ifndef __IADsProperty_FWD_DEFINED__
#define __IADsProperty_FWD_DEFINED__
typedef interface IADsProperty IADsProperty;

#endif 	/* __IADsProperty_FWD_DEFINED__ */


#ifndef __IADsSyntax_FWD_DEFINED__
#define __IADsSyntax_FWD_DEFINED__
typedef interface IADsSyntax IADsSyntax;

#endif 	/* __IADsSyntax_FWD_DEFINED__ */


#ifndef __IADsLocality_FWD_DEFINED__
#define __IADsLocality_FWD_DEFINED__
typedef interface IADsLocality IADsLocality;

#endif 	/* __IADsLocality_FWD_DEFINED__ */


#ifndef __IADsO_FWD_DEFINED__
#define __IADsO_FWD_DEFINED__
typedef interface IADsO IADsO;

#endif 	/* __IADsO_FWD_DEFINED__ */


#ifndef __IADsOU_FWD_DEFINED__
#define __IADsOU_FWD_DEFINED__
typedef interface IADsOU IADsOU;

#endif 	/* __IADsOU_FWD_DEFINED__ */


#ifndef __IADsDomain_FWD_DEFINED__
#define __IADsDomain_FWD_DEFINED__
typedef interface IADsDomain IADsDomain;

#endif 	/* __IADsDomain_FWD_DEFINED__ */


#ifndef __IADsComputer_FWD_DEFINED__
#define __IADsComputer_FWD_DEFINED__
typedef interface IADsComputer IADsComputer;

#endif 	/* __IADsComputer_FWD_DEFINED__ */


#ifndef __IADsComputerOperations_FWD_DEFINED__
#define __IADsComputerOperations_FWD_DEFINED__
typedef interface IADsComputerOperations IADsComputerOperations;

#endif 	/* __IADsComputerOperations_FWD_DEFINED__ */


#ifndef __IADsGroup_FWD_DEFINED__
#define __IADsGroup_FWD_DEFINED__
typedef interface IADsGroup IADsGroup;

#endif 	/* __IADsGroup_FWD_DEFINED__ */


#ifndef __IADsUser_FWD_DEFINED__
#define __IADsUser_FWD_DEFINED__
typedef interface IADsUser IADsUser;

#endif 	/* __IADsUser_FWD_DEFINED__ */


#ifndef __IADsPrintQueue_FWD_DEFINED__
#define __IADsPrintQueue_FWD_DEFINED__
typedef interface IADsPrintQueue IADsPrintQueue;

#endif 	/* __IADsPrintQueue_FWD_DEFINED__ */


#ifndef __IADsPrintQueueOperations_FWD_DEFINED__
#define __IADsPrintQueueOperations_FWD_DEFINED__
typedef interface IADsPrintQueueOperations IADsPrintQueueOperations;

#endif 	/* __IADsPrintQueueOperations_FWD_DEFINED__ */


#ifndef __IADsPrintJob_FWD_DEFINED__
#define __IADsPrintJob_FWD_DEFINED__
typedef interface IADsPrintJob IADsPrintJob;

#endif 	/* __IADsPrintJob_FWD_DEFINED__ */


#ifndef __IADsPrintJobOperations_FWD_DEFINED__
#define __IADsPrintJobOperations_FWD_DEFINED__
typedef interface IADsPrintJobOperations IADsPrintJobOperations;

#endif 	/* __IADsPrintJobOperations_FWD_DEFINED__ */


#ifndef __IADsService_FWD_DEFINED__
#define __IADsService_FWD_DEFINED__
typedef interface IADsService IADsService;

#endif 	/* __IADsService_FWD_DEFINED__ */


#ifndef __IADsServiceOperations_FWD_DEFINED__
#define __IADsServiceOperations_FWD_DEFINED__
typedef interface IADsServiceOperations IADsServiceOperations;

#endif 	/* __IADsServiceOperations_FWD_DEFINED__ */


#ifndef __IADsFileService_FWD_DEFINED__
#define __IADsFileService_FWD_DEFINED__
typedef interface IADsFileService IADsFileService;

#endif 	/* __IADsFileService_FWD_DEFINED__ */


#ifndef __IADsFileServiceOperations_FWD_DEFINED__
#define __IADsFileServiceOperations_FWD_DEFINED__
typedef interface IADsFileServiceOperations IADsFileServiceOperations;

#endif 	/* __IADsFileServiceOperations_FWD_DEFINED__ */


#ifndef __IADsFileShare_FWD_DEFINED__
#define __IADsFileShare_FWD_DEFINED__
typedef interface IADsFileShare IADsFileShare;

#endif 	/* __IADsFileShare_FWD_DEFINED__ */


#ifndef __IADsSession_FWD_DEFINED__
#define __IADsSession_FWD_DEFINED__
typedef interface IADsSession IADsSession;

#endif 	/* __IADsSession_FWD_DEFINED__ */


#ifndef __IADsResource_FWD_DEFINED__
#define __IADsResource_FWD_DEFINED__
typedef interface IADsResource IADsResource;

#endif 	/* __IADsResource_FWD_DEFINED__ */


#ifndef __IADsOpenDSObject_FWD_DEFINED__
#define __IADsOpenDSObject_FWD_DEFINED__
typedef interface IADsOpenDSObject IADsOpenDSObject;

#endif 	/* __IADsOpenDSObject_FWD_DEFINED__ */


#ifndef __IDirectoryObject_FWD_DEFINED__
#define __IDirectoryObject_FWD_DEFINED__
typedef interface IDirectoryObject IDirectoryObject;

#endif 	/* __IDirectoryObject_FWD_DEFINED__ */


#ifndef __IDirectorySearch_FWD_DEFINED__
#define __IDirectorySearch_FWD_DEFINED__
typedef interface IDirectorySearch IDirectorySearch;

#endif 	/* __IDirectorySearch_FWD_DEFINED__ */


#ifndef __IDirectorySchemaMgmt_FWD_DEFINED__
#define __IDirectorySchemaMgmt_FWD_DEFINED__
typedef interface IDirectorySchemaMgmt IDirectorySchemaMgmt;

#endif 	/* __IDirectorySchemaMgmt_FWD_DEFINED__ */


#ifndef __IADsAggregatee_FWD_DEFINED__
#define __IADsAggregatee_FWD_DEFINED__
typedef interface IADsAggregatee IADsAggregatee;

#endif 	/* __IADsAggregatee_FWD_DEFINED__ */


#ifndef __IADsAggregator_FWD_DEFINED__
#define __IADsAggregator_FWD_DEFINED__
typedef interface IADsAggregator IADsAggregator;

#endif 	/* __IADsAggregator_FWD_DEFINED__ */


#ifndef __IADsAccessControlEntry_FWD_DEFINED__
#define __IADsAccessControlEntry_FWD_DEFINED__
typedef interface IADsAccessControlEntry IADsAccessControlEntry;

#endif 	/* __IADsAccessControlEntry_FWD_DEFINED__ */


#ifndef __AccessControlEntry_FWD_DEFINED__
#define __AccessControlEntry_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccessControlEntry AccessControlEntry;
#else
typedef struct AccessControlEntry AccessControlEntry;
#endif /* __cplusplus */

#endif 	/* __AccessControlEntry_FWD_DEFINED__ */


#ifndef __IADsAccessControlList_FWD_DEFINED__
#define __IADsAccessControlList_FWD_DEFINED__
typedef interface IADsAccessControlList IADsAccessControlList;

#endif 	/* __IADsAccessControlList_FWD_DEFINED__ */


#ifndef __AccessControlList_FWD_DEFINED__
#define __AccessControlList_FWD_DEFINED__

#ifdef __cplusplus
typedef class AccessControlList AccessControlList;
#else
typedef struct AccessControlList AccessControlList;
#endif /* __cplusplus */

#endif 	/* __AccessControlList_FWD_DEFINED__ */


#ifndef __IADsSecurityDescriptor_FWD_DEFINED__
#define __IADsSecurityDescriptor_FWD_DEFINED__
typedef interface IADsSecurityDescriptor IADsSecurityDescriptor;

#endif 	/* __IADsSecurityDescriptor_FWD_DEFINED__ */


#ifndef __SecurityDescriptor_FWD_DEFINED__
#define __SecurityDescriptor_FWD_DEFINED__

#ifdef __cplusplus
typedef class SecurityDescriptor SecurityDescriptor;
#else
typedef struct SecurityDescriptor SecurityDescriptor;
#endif /* __cplusplus */

#endif 	/* __SecurityDescriptor_FWD_DEFINED__ */


#ifndef __IADsLargeInteger_FWD_DEFINED__
#define __IADsLargeInteger_FWD_DEFINED__
typedef interface IADsLargeInteger IADsLargeInteger;

#endif 	/* __IADsLargeInteger_FWD_DEFINED__ */


#ifndef __LargeInteger_FWD_DEFINED__
#define __LargeInteger_FWD_DEFINED__

#ifdef __cplusplus
typedef class LargeInteger LargeInteger;
#else
typedef struct LargeInteger LargeInteger;
#endif /* __cplusplus */

#endif 	/* __LargeInteger_FWD_DEFINED__ */


#ifndef __IADsNameTranslate_FWD_DEFINED__
#define __IADsNameTranslate_FWD_DEFINED__
typedef interface IADsNameTranslate IADsNameTranslate;

#endif 	/* __IADsNameTranslate_FWD_DEFINED__ */


#ifndef __NameTranslate_FWD_DEFINED__
#define __NameTranslate_FWD_DEFINED__

#ifdef __cplusplus
typedef class NameTranslate NameTranslate;
#else
typedef struct NameTranslate NameTranslate;
#endif /* __cplusplus */

#endif 	/* __NameTranslate_FWD_DEFINED__ */


#ifndef __IADsCaseIgnoreList_FWD_DEFINED__
#define __IADsCaseIgnoreList_FWD_DEFINED__
typedef interface IADsCaseIgnoreList IADsCaseIgnoreList;

#endif 	/* __IADsCaseIgnoreList_FWD_DEFINED__ */


#ifndef __CaseIgnoreList_FWD_DEFINED__
#define __CaseIgnoreList_FWD_DEFINED__

#ifdef __cplusplus
typedef class CaseIgnoreList CaseIgnoreList;
#else
typedef struct CaseIgnoreList CaseIgnoreList;
#endif /* __cplusplus */

#endif 	/* __CaseIgnoreList_FWD_DEFINED__ */


#ifndef __IADsFaxNumber_FWD_DEFINED__
#define __IADsFaxNumber_FWD_DEFINED__
typedef interface IADsFaxNumber IADsFaxNumber;

#endif 	/* __IADsFaxNumber_FWD_DEFINED__ */


#ifndef __FaxNumber_FWD_DEFINED__
#define __FaxNumber_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxNumber FaxNumber;
#else
typedef struct FaxNumber FaxNumber;
#endif /* __cplusplus */

#endif 	/* __FaxNumber_FWD_DEFINED__ */


#ifndef __IADsNetAddress_FWD_DEFINED__
#define __IADsNetAddress_FWD_DEFINED__
typedef interface IADsNetAddress IADsNetAddress;

#endif 	/* __IADsNetAddress_FWD_DEFINED__ */


#ifndef __NetAddress_FWD_DEFINED__
#define __NetAddress_FWD_DEFINED__

#ifdef __cplusplus
typedef class NetAddress NetAddress;
#else
typedef struct NetAddress NetAddress;
#endif /* __cplusplus */

#endif 	/* __NetAddress_FWD_DEFINED__ */


#ifndef __IADsOctetList_FWD_DEFINED__
#define __IADsOctetList_FWD_DEFINED__
typedef interface IADsOctetList IADsOctetList;

#endif 	/* __IADsOctetList_FWD_DEFINED__ */


#ifndef __OctetList_FWD_DEFINED__
#define __OctetList_FWD_DEFINED__

#ifdef __cplusplus
typedef class OctetList OctetList;
#else
typedef struct OctetList OctetList;
#endif /* __cplusplus */

#endif 	/* __OctetList_FWD_DEFINED__ */


#ifndef __IADsEmail_FWD_DEFINED__
#define __IADsEmail_FWD_DEFINED__
typedef interface IADsEmail IADsEmail;

#endif 	/* __IADsEmail_FWD_DEFINED__ */


#ifndef __Email_FWD_DEFINED__
#define __Email_FWD_DEFINED__

#ifdef __cplusplus
typedef class Email Email;
#else
typedef struct Email Email;
#endif /* __cplusplus */

#endif 	/* __Email_FWD_DEFINED__ */


#ifndef __IADsPath_FWD_DEFINED__
#define __IADsPath_FWD_DEFINED__
typedef interface IADsPath IADsPath;

#endif 	/* __IADsPath_FWD_DEFINED__ */


#ifndef __Path_FWD_DEFINED__
#define __Path_FWD_DEFINED__

#ifdef __cplusplus
typedef class Path Path;
#else
typedef struct Path Path;
#endif /* __cplusplus */

#endif 	/* __Path_FWD_DEFINED__ */


#ifndef __IADsReplicaPointer_FWD_DEFINED__
#define __IADsReplicaPointer_FWD_DEFINED__
typedef interface IADsReplicaPointer IADsReplicaPointer;

#endif 	/* __IADsReplicaPointer_FWD_DEFINED__ */


#ifndef __ReplicaPointer_FWD_DEFINED__
#define __ReplicaPointer_FWD_DEFINED__

#ifdef __cplusplus
typedef class ReplicaPointer ReplicaPointer;
#else
typedef struct ReplicaPointer ReplicaPointer;
#endif /* __cplusplus */

#endif 	/* __ReplicaPointer_FWD_DEFINED__ */


#ifndef __IADsAcl_FWD_DEFINED__
#define __IADsAcl_FWD_DEFINED__
typedef interface IADsAcl IADsAcl;

#endif 	/* __IADsAcl_FWD_DEFINED__ */


#ifndef __IADsTimestamp_FWD_DEFINED__
#define __IADsTimestamp_FWD_DEFINED__
typedef interface IADsTimestamp IADsTimestamp;

#endif 	/* __IADsTimestamp_FWD_DEFINED__ */


#ifndef __Timestamp_FWD_DEFINED__
#define __Timestamp_FWD_DEFINED__

#ifdef __cplusplus
typedef class Timestamp Timestamp;
#else
typedef struct Timestamp Timestamp;
#endif /* __cplusplus */

#endif 	/* __Timestamp_FWD_DEFINED__ */


#ifndef __IADsPostalAddress_FWD_DEFINED__
#define __IADsPostalAddress_FWD_DEFINED__
typedef interface IADsPostalAddress IADsPostalAddress;

#endif 	/* __IADsPostalAddress_FWD_DEFINED__ */


#ifndef __PostalAddress_FWD_DEFINED__
#define __PostalAddress_FWD_DEFINED__

#ifdef __cplusplus
typedef class PostalAddress PostalAddress;
#else
typedef struct PostalAddress PostalAddress;
#endif /* __cplusplus */

#endif 	/* __PostalAddress_FWD_DEFINED__ */


#ifndef __IADsBackLink_FWD_DEFINED__
#define __IADsBackLink_FWD_DEFINED__
typedef interface IADsBackLink IADsBackLink;

#endif 	/* __IADsBackLink_FWD_DEFINED__ */


#ifndef __BackLink_FWD_DEFINED__
#define __BackLink_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackLink BackLink;
#else
typedef struct BackLink BackLink;
#endif /* __cplusplus */

#endif 	/* __BackLink_FWD_DEFINED__ */


#ifndef __IADsTypedName_FWD_DEFINED__
#define __IADsTypedName_FWD_DEFINED__
typedef interface IADsTypedName IADsTypedName;

#endif 	/* __IADsTypedName_FWD_DEFINED__ */


#ifndef __TypedName_FWD_DEFINED__
#define __TypedName_FWD_DEFINED__

#ifdef __cplusplus
typedef class TypedName TypedName;
#else
typedef struct TypedName TypedName;
#endif /* __cplusplus */

#endif 	/* __TypedName_FWD_DEFINED__ */


#ifndef __IADsHold_FWD_DEFINED__
#define __IADsHold_FWD_DEFINED__
typedef interface IADsHold IADsHold;

#endif 	/* __IADsHold_FWD_DEFINED__ */


#ifndef __Hold_FWD_DEFINED__
#define __Hold_FWD_DEFINED__

#ifdef __cplusplus
typedef class Hold Hold;
#else
typedef struct Hold Hold;
#endif /* __cplusplus */

#endif 	/* __Hold_FWD_DEFINED__ */


#ifndef __IADsObjectOptions_FWD_DEFINED__
#define __IADsObjectOptions_FWD_DEFINED__
typedef interface IADsObjectOptions IADsObjectOptions;

#endif 	/* __IADsObjectOptions_FWD_DEFINED__ */


#ifndef __IADsPathname_FWD_DEFINED__
#define __IADsPathname_FWD_DEFINED__
typedef interface IADsPathname IADsPathname;

#endif 	/* __IADsPathname_FWD_DEFINED__ */


#ifndef __Pathname_FWD_DEFINED__
#define __Pathname_FWD_DEFINED__

#ifdef __cplusplus
typedef class Pathname Pathname;
#else
typedef struct Pathname Pathname;
#endif /* __cplusplus */

#endif 	/* __Pathname_FWD_DEFINED__ */


#ifndef __IADsADSystemInfo_FWD_DEFINED__
#define __IADsADSystemInfo_FWD_DEFINED__
typedef interface IADsADSystemInfo IADsADSystemInfo;

#endif 	/* __IADsADSystemInfo_FWD_DEFINED__ */


#ifndef __ADSystemInfo_FWD_DEFINED__
#define __ADSystemInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class ADSystemInfo ADSystemInfo;
#else
typedef struct ADSystemInfo ADSystemInfo;
#endif /* __cplusplus */

#endif 	/* __ADSystemInfo_FWD_DEFINED__ */


#ifndef __IADsWinNTSystemInfo_FWD_DEFINED__
#define __IADsWinNTSystemInfo_FWD_DEFINED__
typedef interface IADsWinNTSystemInfo IADsWinNTSystemInfo;

#endif 	/* __IADsWinNTSystemInfo_FWD_DEFINED__ */


#ifndef __WinNTSystemInfo_FWD_DEFINED__
#define __WinNTSystemInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class WinNTSystemInfo WinNTSystemInfo;
#else
typedef struct WinNTSystemInfo WinNTSystemInfo;
#endif /* __cplusplus */

#endif 	/* __WinNTSystemInfo_FWD_DEFINED__ */


#ifndef __IADsDNWithBinary_FWD_DEFINED__
#define __IADsDNWithBinary_FWD_DEFINED__
typedef interface IADsDNWithBinary IADsDNWithBinary;

#endif 	/* __IADsDNWithBinary_FWD_DEFINED__ */


#ifndef __DNWithBinary_FWD_DEFINED__
#define __DNWithBinary_FWD_DEFINED__

#ifdef __cplusplus
typedef class DNWithBinary DNWithBinary;
#else
typedef struct DNWithBinary DNWithBinary;
#endif /* __cplusplus */

#endif 	/* __DNWithBinary_FWD_DEFINED__ */


#ifndef __IADsDNWithString_FWD_DEFINED__
#define __IADsDNWithString_FWD_DEFINED__
typedef interface IADsDNWithString IADsDNWithString;

#endif 	/* __IADsDNWithString_FWD_DEFINED__ */


#ifndef __DNWithString_FWD_DEFINED__
#define __DNWithString_FWD_DEFINED__

#ifdef __cplusplus
typedef class DNWithString DNWithString;
#else
typedef struct DNWithString DNWithString;
#endif /* __cplusplus */

#endif 	/* __DNWithString_FWD_DEFINED__ */


#ifndef __IADsSecurityUtility_FWD_DEFINED__
#define __IADsSecurityUtility_FWD_DEFINED__
typedef interface IADsSecurityUtility IADsSecurityUtility;

#endif 	/* __IADsSecurityUtility_FWD_DEFINED__ */


#ifndef __ADsSecurityUtility_FWD_DEFINED__
#define __ADsSecurityUtility_FWD_DEFINED__

#ifdef __cplusplus
typedef class ADsSecurityUtility ADsSecurityUtility;
#else
typedef struct ADsSecurityUtility ADsSecurityUtility;
#endif /* __cplusplus */

#endif 	/* __ADsSecurityUtility_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __ActiveDs_LIBRARY_DEFINED__
#define __ActiveDs_LIBRARY_DEFINED__

/* library ActiveDs */
/* [helpstring][version][uuid] */ 

#pragma once
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma once
#pragma region Input Buffer SAL 1 compatibility macros
#pragma endregion Input Buffer SAL 1 compatibility macros
#pragma once
#pragma once
#pragma warning(pop)
typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0001
    {
        ADSTYPE_INVALID	= 0,
        ADSTYPE_DN_STRING	= ( ADSTYPE_INVALID + 1 ) ,
        ADSTYPE_CASE_EXACT_STRING	= ( ADSTYPE_DN_STRING + 1 ) ,
        ADSTYPE_CASE_IGNORE_STRING	= ( ADSTYPE_CASE_EXACT_STRING + 1 ) ,
        ADSTYPE_PRINTABLE_STRING	= ( ADSTYPE_CASE_IGNORE_STRING + 1 ) ,
        ADSTYPE_NUMERIC_STRING	= ( ADSTYPE_PRINTABLE_STRING + 1 ) ,
        ADSTYPE_BOOLEAN	= ( ADSTYPE_NUMERIC_STRING + 1 ) ,
        ADSTYPE_INTEGER	= ( ADSTYPE_BOOLEAN + 1 ) ,
        ADSTYPE_OCTET_STRING	= ( ADSTYPE_INTEGER + 1 ) ,
        ADSTYPE_UTC_TIME	= ( ADSTYPE_OCTET_STRING + 1 ) ,
        ADSTYPE_LARGE_INTEGER	= ( ADSTYPE_UTC_TIME + 1 ) ,
        ADSTYPE_PROV_SPECIFIC	= ( ADSTYPE_LARGE_INTEGER + 1 ) ,
        ADSTYPE_OBJECT_CLASS	= ( ADSTYPE_PROV_SPECIFIC + 1 ) ,
        ADSTYPE_CASEIGNORE_LIST	= ( ADSTYPE_OBJECT_CLASS + 1 ) ,
        ADSTYPE_OCTET_LIST	= ( ADSTYPE_CASEIGNORE_LIST + 1 ) ,
        ADSTYPE_PATH	= ( ADSTYPE_OCTET_LIST + 1 ) ,
        ADSTYPE_POSTALADDRESS	= ( ADSTYPE_PATH + 1 ) ,
        ADSTYPE_TIMESTAMP	= ( ADSTYPE_POSTALADDRESS + 1 ) ,
        ADSTYPE_BACKLINK	= ( ADSTYPE_TIMESTAMP + 1 ) ,
        ADSTYPE_TYPEDNAME	= ( ADSTYPE_BACKLINK + 1 ) ,
        ADSTYPE_HOLD	= ( ADSTYPE_TYPEDNAME + 1 ) ,
        ADSTYPE_NETADDRESS	= ( ADSTYPE_HOLD + 1 ) ,
        ADSTYPE_REPLICAPOINTER	= ( ADSTYPE_NETADDRESS + 1 ) ,
        ADSTYPE_FAXNUMBER	= ( ADSTYPE_REPLICAPOINTER + 1 ) ,
        ADSTYPE_EMAIL	= ( ADSTYPE_FAXNUMBER + 1 ) ,
        ADSTYPE_NT_SECURITY_DESCRIPTOR	= ( ADSTYPE_EMAIL + 1 ) ,
        ADSTYPE_UNKNOWN	= ( ADSTYPE_NT_SECURITY_DESCRIPTOR + 1 ) ,
        ADSTYPE_DN_WITH_BINARY	= ( ADSTYPE_UNKNOWN + 1 ) ,
        ADSTYPE_DN_WITH_STRING	= ( ADSTYPE_DN_WITH_BINARY + 1 ) 
    } 	ADSTYPEENUM;

typedef ADSTYPEENUM ADSTYPE;

typedef unsigned char BYTE;

typedef unsigned char *LPBYTE;

typedef unsigned char *PBYTE;

typedef LPWSTR ADS_DN_STRING;

typedef LPWSTR *PADS_DN_STRING;

typedef LPWSTR ADS_CASE_EXACT_STRING;

typedef LPWSTR *PADS_CASE_EXACT_STRING;

typedef LPWSTR ADS_CASE_IGNORE_STRING;

typedef LPWSTR *PADS_CASE_IGNORE_STRING;

typedef LPWSTR ADS_PRINTABLE_STRING;

typedef LPWSTR *PADS_PRINTABLE_STRING;

typedef LPWSTR ADS_NUMERIC_STRING;

typedef LPWSTR *PADS_NUMERIC_STRING;

typedef DWORD ADS_BOOLEAN;

typedef DWORD *LPNDS_BOOLEAN;

typedef DWORD ADS_INTEGER;

typedef DWORD *PADS_INTEGER;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0002
    {
    DWORD dwLength;
    LPBYTE lpValue;
    } 	ADS_OCTET_STRING;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0002 *PADS_OCTET_STRING;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0003
    {
    DWORD dwLength;
    LPBYTE lpValue;
    } 	ADS_NT_SECURITY_DESCRIPTOR;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0003 *PADS_NT_SECURITY_DESCRIPTOR;

typedef SYSTEMTIME ADS_UTC_TIME;

typedef SYSTEMTIME *PADS_UTC_TIME;

typedef LARGE_INTEGER ADS_LARGE_INTEGER;

typedef LARGE_INTEGER *PADS_LARGE_INTEGER;

typedef LPWSTR ADS_OBJECT_CLASS;

typedef LPWSTR *PADS_OBJECT_CLASS;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0004
    {
    DWORD dwLength;
    LPBYTE lpValue;
    } 	ADS_PROV_SPECIFIC;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0004 *PADS_PROV_SPECIFIC;

typedef struct _ADS_CASEIGNORE_LIST
    {
    struct _ADS_CASEIGNORE_LIST *Next;
    LPWSTR String;
    } 	ADS_CASEIGNORE_LIST;

typedef struct _ADS_CASEIGNORE_LIST *PADS_CASEIGNORE_LIST;

typedef struct _ADS_OCTET_LIST
    {
    struct _ADS_OCTET_LIST *Next;
    DWORD Length;
    BYTE *Data;
    } 	ADS_OCTET_LIST;

typedef struct _ADS_OCTET_LIST *PADS_OCTET_LIST;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0005
    {
    DWORD Type;
    LPWSTR VolumeName;
    LPWSTR Path;
    } 	ADS_PATH;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0005 *PADS_PATH;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0006
    {
    LPWSTR PostalAddress[ 6 ];
    } 	ADS_POSTALADDRESS;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0006 *PADS_POSTALADDRESS;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0007
    {
    DWORD WholeSeconds;
    DWORD EventID;
    } 	ADS_TIMESTAMP;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0007 *PADS_TIMESTAMP;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0008
    {
    DWORD RemoteID;
    LPWSTR ObjectName;
    } 	ADS_BACKLINK;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0008 *PADS_BACKLINK;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0009
    {
    LPWSTR ObjectName;
    DWORD Level;
    DWORD Interval;
    } 	ADS_TYPEDNAME;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0009 *PADS_TYPEDNAME;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0010
    {
    LPWSTR ObjectName;
    DWORD Amount;
    } 	ADS_HOLD;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0010 *PADS_HOLD;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0011
    {
    DWORD AddressType;
    DWORD AddressLength;
    BYTE *Address;
    } 	ADS_NETADDRESS;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0011 *PADS_NETADDRESS;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0012
    {
    LPWSTR ServerName;
    DWORD ReplicaType;
    DWORD ReplicaNumber;
    DWORD Count;
    PADS_NETADDRESS ReplicaAddressHints;
    } 	ADS_REPLICAPOINTER;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0012 *PADS_REPLICAPOINTER;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0013
    {
    LPWSTR TelephoneNumber;
    DWORD NumberOfBits;
    LPBYTE Parameters;
    } 	ADS_FAXNUMBER;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0013 *PADS_FAXNUMBER;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0014
    {
    LPWSTR Address;
    DWORD Type;
    } 	ADS_EMAIL;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0014 *PADS_EMAIL;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0015
    {
    DWORD dwLength;
    LPBYTE lpBinaryValue;
    LPWSTR pszDNString;
    } 	ADS_DN_WITH_BINARY;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0015 *PADS_DN_WITH_BINARY;

typedef /* [public] */ struct __MIDL___MIDL_itf_ads_0000_0000_0016
    {
    LPWSTR pszStringValue;
    LPWSTR pszDNString;
    } 	ADS_DN_WITH_STRING;

typedef struct __MIDL___MIDL_itf_ads_0000_0000_0016 *PADS_DN_WITH_STRING;

typedef struct _adsvalue
    {
    ADSTYPE dwType;
    union 
        {
        ADS_DN_STRING DNString;
        ADS_CASE_EXACT_STRING CaseExactString;
        ADS_CASE_IGNORE_STRING CaseIgnoreString;
        ADS_PRINTABLE_STRING PrintableString;
        ADS_NUMERIC_STRING NumericString;
        ADS_BOOLEAN Boolean;
        ADS_INTEGER Integer;
        ADS_OCTET_STRING OctetString;
        ADS_UTC_TIME UTCTime;
        ADS_LARGE_INTEGER LargeInteger;
        ADS_OBJECT_CLASS ClassName;
        ADS_PROV_SPECIFIC ProviderSpecific;
        PADS_CASEIGNORE_LIST pCaseIgnoreList;
        PADS_OCTET_LIST pOctetList;
        PADS_PATH pPath;
        PADS_POSTALADDRESS pPostalAddress;
        ADS_TIMESTAMP Timestamp;
        ADS_BACKLINK BackLink;
        PADS_TYPEDNAME pTypedName;
        ADS_HOLD Hold;
        PADS_NETADDRESS pNetAddress;
        PADS_REPLICAPOINTER pReplicaPointer;
        PADS_FAXNUMBER pFaxNumber;
        ADS_EMAIL Email;
        ADS_NT_SECURITY_DESCRIPTOR SecurityDescriptor;
        PADS_DN_WITH_BINARY pDNWithBinary;
        PADS_DN_WITH_STRING pDNWithString;
        } 	;
    } 	ADSVALUE;

typedef struct _adsvalue *PADSVALUE;

typedef struct _adsvalue *LPADSVALUE;

typedef struct _ads_attr_info
    {
    LPWSTR pszAttrName;
    DWORD dwControlCode;
    ADSTYPE dwADsType;
    PADSVALUE pADsValues;
    DWORD dwNumValues;
    } 	ADS_ATTR_INFO;

typedef struct _ads_attr_info *PADS_ATTR_INFO;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0018
    {
        ADS_SECURE_AUTHENTICATION	= 0x1,
        ADS_USE_ENCRYPTION	= 0x2,
        ADS_USE_SSL	= 0x2,
        ADS_READONLY_SERVER	= 0x4,
        ADS_PROMPT_CREDENTIALS	= 0x8,
        ADS_NO_AUTHENTICATION	= 0x10,
        ADS_FAST_BIND	= 0x20,
        ADS_USE_SIGNING	= 0x40,
        ADS_USE_SEALING	= 0x80,
        ADS_USE_DELEGATION	= 0x100,
        ADS_SERVER_BIND	= 0x200,
        ADS_NO_REFERRAL_CHASING	= 0x400,
        ADS_AUTH_RESERVED	= 0x80000000
    } 	ADS_AUTHENTICATION_ENUM;

#define	ADS_ATTR_CLEAR	( 1 )

#define	ADS_ATTR_UPDATE	( 2 )

#define	ADS_ATTR_APPEND	( 3 )

#define	ADS_ATTR_DELETE	( 4 )

typedef struct _ads_object_info
    {
    LPWSTR pszRDN;
    LPWSTR pszObjectDN;
    LPWSTR pszParentDN;
    LPWSTR pszSchemaDN;
    LPWSTR pszClassName;
    } 	ADS_OBJECT_INFO;

typedef struct _ads_object_info *PADS_OBJECT_INFO;

typedef /* [public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0019
    {
        ADS_STATUS_S_OK	= 0,
        ADS_STATUS_INVALID_SEARCHPREF	= ( ADS_STATUS_S_OK + 1 ) ,
        ADS_STATUS_INVALID_SEARCHPREFVALUE	= ( ADS_STATUS_INVALID_SEARCHPREF + 1 ) 
    } 	ADS_STATUSENUM;

typedef ADS_STATUSENUM ADS_STATUS;

typedef ADS_STATUSENUM *PADS_STATUS;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0020
    {
        ADS_DEREF_NEVER	= 0,
        ADS_DEREF_SEARCHING	= 1,
        ADS_DEREF_FINDING	= 2,
        ADS_DEREF_ALWAYS	= 3
    } 	ADS_DEREFENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0021
    {
        ADS_SCOPE_BASE	= 0,
        ADS_SCOPE_ONELEVEL	= 1,
        ADS_SCOPE_SUBTREE	= 2
    } 	ADS_SCOPEENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0022
    {
        ADSIPROP_ASYNCHRONOUS	= 0,
        ADSIPROP_DEREF_ALIASES	= 0x1,
        ADSIPROP_SIZE_LIMIT	= 0x2,
        ADSIPROP_TIME_LIMIT	= 0x3,
        ADSIPROP_ATTRIBTYPES_ONLY	= 0x4,
        ADSIPROP_SEARCH_SCOPE	= 0x5,
        ADSIPROP_TIMEOUT	= 0x6,
        ADSIPROP_PAGESIZE	= 0x7,
        ADSIPROP_PAGED_TIME_LIMIT	= 0x8,
        ADSIPROP_CHASE_REFERRALS	= 0x9,
        ADSIPROP_SORT_ON	= 0xa,
        ADSIPROP_CACHE_RESULTS	= 0xb,
        ADSIPROP_ADSIFLAG	= 0xc
    } 	ADS_PREFERENCES_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0023
    {
        ADSI_DIALECT_LDAP	= 0,
        ADSI_DIALECT_SQL	= 0x1
    } 	ADSI_DIALECT_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0024
    {
        ADS_CHASE_REFERRALS_NEVER	= 0,
        ADS_CHASE_REFERRALS_SUBORDINATE	= 0x20,
        ADS_CHASE_REFERRALS_EXTERNAL	= 0x40,
        ADS_CHASE_REFERRALS_ALWAYS	= ( ADS_CHASE_REFERRALS_SUBORDINATE | ADS_CHASE_REFERRALS_EXTERNAL ) 
    } 	ADS_CHASE_REFERRALS_ENUM;

typedef /* [public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0025
    {
        ADS_SEARCHPREF_ASYNCHRONOUS	= 0,
        ADS_SEARCHPREF_DEREF_ALIASES	= ( ADS_SEARCHPREF_ASYNCHRONOUS + 1 ) ,
        ADS_SEARCHPREF_SIZE_LIMIT	= ( ADS_SEARCHPREF_DEREF_ALIASES + 1 ) ,
        ADS_SEARCHPREF_TIME_LIMIT	= ( ADS_SEARCHPREF_SIZE_LIMIT + 1 ) ,
        ADS_SEARCHPREF_ATTRIBTYPES_ONLY	= ( ADS_SEARCHPREF_TIME_LIMIT + 1 ) ,
        ADS_SEARCHPREF_SEARCH_SCOPE	= ( ADS_SEARCHPREF_ATTRIBTYPES_ONLY + 1 ) ,
        ADS_SEARCHPREF_TIMEOUT	= ( ADS_SEARCHPREF_SEARCH_SCOPE + 1 ) ,
        ADS_SEARCHPREF_PAGESIZE	= ( ADS_SEARCHPREF_TIMEOUT + 1 ) ,
        ADS_SEARCHPREF_PAGED_TIME_LIMIT	= ( ADS_SEARCHPREF_PAGESIZE + 1 ) ,
        ADS_SEARCHPREF_CHASE_REFERRALS	= ( ADS_SEARCHPREF_PAGED_TIME_LIMIT + 1 ) ,
        ADS_SEARCHPREF_SORT_ON	= ( ADS_SEARCHPREF_CHASE_REFERRALS + 1 ) ,
        ADS_SEARCHPREF_CACHE_RESULTS	= ( ADS_SEARCHPREF_SORT_ON + 1 ) ,
        ADS_SEARCHPREF_DIRSYNC	= ( ADS_SEARCHPREF_CACHE_RESULTS + 1 ) ,
        ADS_SEARCHPREF_TOMBSTONE	= ( ADS_SEARCHPREF_DIRSYNC + 1 ) ,
        ADS_SEARCHPREF_VLV	= ( ADS_SEARCHPREF_TOMBSTONE + 1 ) ,
        ADS_SEARCHPREF_ATTRIBUTE_QUERY	= ( ADS_SEARCHPREF_VLV + 1 ) ,
        ADS_SEARCHPREF_SECURITY_MASK	= ( ADS_SEARCHPREF_ATTRIBUTE_QUERY + 1 ) ,
        ADS_SEARCHPREF_DIRSYNC_FLAG	= ( ADS_SEARCHPREF_SECURITY_MASK + 1 ) ,
        ADS_SEARCHPREF_EXTENDED_DN	= ( ADS_SEARCHPREF_DIRSYNC_FLAG + 1 ) 
    } 	ADS_SEARCHPREF_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0026
    {
        ADS_PASSWORD_ENCODE_REQUIRE_SSL	= 0,
        ADS_PASSWORD_ENCODE_CLEAR	= 1
    } 	ADS_PASSWORD_ENCODING_ENUM;

typedef ADS_SEARCHPREF_ENUM ADS_SEARCHPREF;

typedef struct ads_searchpref_info
    {
    ADS_SEARCHPREF dwSearchPref;
    ADSVALUE vValue;
    ADS_STATUS dwStatus;
    } 	ADS_SEARCHPREF_INFO;

typedef struct ads_searchpref_info *PADS_SEARCHPREF_INFO;

typedef struct ads_searchpref_info *LPADS_SEARCHPREF_INFO;

#define	ADS_DIRSYNC_COOKIE	( L"fc8cb04d-311d-406c-8cb9-1ae8b843b418" )

#define	ADS_VLV_RESPONSE	( L"fc8cb04d-311d-406c-8cb9-1ae8b843b419" )

typedef HANDLE ADS_SEARCH_HANDLE;

typedef HANDLE *PADS_SEARCH_HANDLE;

typedef struct ads_search_column
    {
    LPWSTR pszAttrName;
    ADSTYPE dwADsType;
    PADSVALUE pADsValues;
    DWORD dwNumValues;
    HANDLE hReserved;
    } 	ADS_SEARCH_COLUMN;

typedef struct ads_search_column *PADS_SEARCH_COLUMN;

typedef struct _ads_attr_def
    {
    LPWSTR pszAttrName;
    ADSTYPE dwADsType;
    DWORD dwMinRange;
    DWORD dwMaxRange;
    BOOL fMultiValued;
    } 	ADS_ATTR_DEF;

typedef struct _ads_attr_def *PADS_ATTR_DEF;

typedef struct _ads_class_def
    {
    LPWSTR pszClassName;
    DWORD dwMandatoryAttrs;
    LPWSTR *ppszMandatoryAttrs;
    DWORD optionalAttrs;
    LPWSTR **ppszOptionalAttrs;
    DWORD dwNamingAttrs;
    LPWSTR **ppszNamingAttrs;
    DWORD dwSuperClasses;
    LPWSTR **ppszSuperClasses;
    BOOL fIsContainer;
    } 	ADS_CLASS_DEF;

typedef struct _ads_class_def *PADS_CLASS_DEF;

typedef struct _ads_sortkey
    {
    LPWSTR pszAttrType;
    LPWSTR pszReserved;
    BOOLEAN fReverseorder;
    } 	ADS_SORTKEY;

typedef struct _ads_sortkey *PADS_SORTKEY;

typedef struct _ads_vlv
    {
    DWORD dwBeforeCount;
    DWORD dwAfterCount;
    DWORD dwOffset;
    DWORD dwContentCount;
    LPWSTR pszTarget;
    DWORD dwContextIDLength;
    LPBYTE lpContextID;
    } 	ADS_VLV;

typedef struct _ads_vlv *PADS_VLV;

#define	ADS_EXT_MINEXTDISPID	( 1 )

#define	ADS_EXT_MAXEXTDISPID	( 16777215 )

#define	ADS_EXT_INITCREDENTIALS	( 1 )

#define	ADS_EXT_INITIALIZE_COMPLETE	( 2 )

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0000_0000_0027
    {
        ADS_PROPERTY_CLEAR	= 1,
        ADS_PROPERTY_UPDATE	= 2,
        ADS_PROPERTY_APPEND	= 3,
        ADS_PROPERTY_DELETE	= 4
    } 	ADS_PROPERTY_OPERATION_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0017_0001
    {
        ADS_SYSTEMFLAG_DISALLOW_DELETE	= 0x80000000,
        ADS_SYSTEMFLAG_CONFIG_ALLOW_RENAME	= 0x40000000,
        ADS_SYSTEMFLAG_CONFIG_ALLOW_MOVE	= 0x20000000,
        ADS_SYSTEMFLAG_CONFIG_ALLOW_LIMITED_MOVE	= 0x10000000,
        ADS_SYSTEMFLAG_DOMAIN_DISALLOW_RENAME	= 0x8000000,
        ADS_SYSTEMFLAG_DOMAIN_DISALLOW_MOVE	= 0x4000000,
        ADS_SYSTEMFLAG_CR_NTDS_NC	= 0x1,
        ADS_SYSTEMFLAG_CR_NTDS_DOMAIN	= 0x2,
        ADS_SYSTEMFLAG_ATTR_NOT_REPLICATED	= 0x1,
        ADS_SYSTEMFLAG_ATTR_IS_CONSTRUCTED	= 0x4
    } 	ADS_SYSTEMFLAG_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0023_0001
    {
        ADS_GROUP_TYPE_GLOBAL_GROUP	= 0x2,
        ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP	= 0x4,
        ADS_GROUP_TYPE_LOCAL_GROUP	= 0x4,
        ADS_GROUP_TYPE_UNIVERSAL_GROUP	= 0x8,
        ADS_GROUP_TYPE_SECURITY_ENABLED	= 0x80000000
    } 	ADS_GROUP_TYPE_ENUM;

typedef 
enum ADS_USER_FLAG
    {
        ADS_UF_SCRIPT	= 0x1,
        ADS_UF_ACCOUNTDISABLE	= 0x2,
        ADS_UF_HOMEDIR_REQUIRED	= 0x8,
        ADS_UF_LOCKOUT	= 0x10,
        ADS_UF_PASSWD_NOTREQD	= 0x20,
        ADS_UF_PASSWD_CANT_CHANGE	= 0x40,
        ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED	= 0x80,
        ADS_UF_TEMP_DUPLICATE_ACCOUNT	= 0x100,
        ADS_UF_NORMAL_ACCOUNT	= 0x200,
        ADS_UF_INTERDOMAIN_TRUST_ACCOUNT	= 0x800,
        ADS_UF_WORKSTATION_TRUST_ACCOUNT	= 0x1000,
        ADS_UF_SERVER_TRUST_ACCOUNT	= 0x2000,
        ADS_UF_DONT_EXPIRE_PASSWD	= 0x10000,
        ADS_UF_MNS_LOGON_ACCOUNT	= 0x20000,
        ADS_UF_SMARTCARD_REQUIRED	= 0x40000,
        ADS_UF_TRUSTED_FOR_DELEGATION	= 0x80000,
        ADS_UF_NOT_DELEGATED	= 0x100000,
        ADS_UF_USE_DES_KEY_ONLY	= 0x200000,
        ADS_UF_DONT_REQUIRE_PREAUTH	= 0x400000,
        ADS_UF_PASSWORD_EXPIRED	= 0x800000,
        ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION	= 0x1000000
    } 	ADS_USER_FLAG_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0048_0001
    {
        ADS_RIGHT_DELETE	= 0x10000,
        ADS_RIGHT_READ_CONTROL	= 0x20000,
        ADS_RIGHT_WRITE_DAC	= 0x40000,
        ADS_RIGHT_WRITE_OWNER	= 0x80000,
        ADS_RIGHT_SYNCHRONIZE	= 0x100000,
        ADS_RIGHT_ACCESS_SYSTEM_SECURITY	= 0x1000000,
        ADS_RIGHT_GENERIC_READ	= 0x80000000,
        ADS_RIGHT_GENERIC_WRITE	= 0x40000000,
        ADS_RIGHT_GENERIC_EXECUTE	= 0x20000000,
        ADS_RIGHT_GENERIC_ALL	= 0x10000000,
        ADS_RIGHT_DS_CREATE_CHILD	= 0x1,
        ADS_RIGHT_DS_DELETE_CHILD	= 0x2,
        ADS_RIGHT_ACTRL_DS_LIST	= 0x4,
        ADS_RIGHT_DS_SELF	= 0x8,
        ADS_RIGHT_DS_READ_PROP	= 0x10,
        ADS_RIGHT_DS_WRITE_PROP	= 0x20,
        ADS_RIGHT_DS_DELETE_TREE	= 0x40,
        ADS_RIGHT_DS_LIST_OBJECT	= 0x80,
        ADS_RIGHT_DS_CONTROL_ACCESS	= 0x100
    } 	ADS_RIGHTS_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0048_0002
    {
        ADS_ACETYPE_ACCESS_ALLOWED	= 0,
        ADS_ACETYPE_ACCESS_DENIED	= 0x1,
        ADS_ACETYPE_SYSTEM_AUDIT	= 0x2,
        ADS_ACETYPE_ACCESS_ALLOWED_OBJECT	= 0x5,
        ADS_ACETYPE_ACCESS_DENIED_OBJECT	= 0x6,
        ADS_ACETYPE_SYSTEM_AUDIT_OBJECT	= 0x7,
        ADS_ACETYPE_SYSTEM_ALARM_OBJECT	= 0x8,
        ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK	= 0x9,
        ADS_ACETYPE_ACCESS_DENIED_CALLBACK	= 0xa,
        ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK_OBJECT	= 0xb,
        ADS_ACETYPE_ACCESS_DENIED_CALLBACK_OBJECT	= 0xc,
        ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK	= 0xd,
        ADS_ACETYPE_SYSTEM_ALARM_CALLBACK	= 0xe,
        ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK_OBJECT	= 0xf,
        ADS_ACETYPE_SYSTEM_ALARM_CALLBACK_OBJECT	= 0x10
    } 	ADS_ACETYPE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0048_0003
    {
        ADS_ACEFLAG_INHERIT_ACE	= 0x2,
        ADS_ACEFLAG_NO_PROPAGATE_INHERIT_ACE	= 0x4,
        ADS_ACEFLAG_INHERIT_ONLY_ACE	= 0x8,
        ADS_ACEFLAG_INHERITED_ACE	= 0x10,
        ADS_ACEFLAG_VALID_INHERIT_FLAGS	= 0x1f,
        ADS_ACEFLAG_SUCCESSFUL_ACCESS	= 0x40,
        ADS_ACEFLAG_FAILED_ACCESS	= 0x80
    } 	ADS_ACEFLAG_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0048_0004
    {
        ADS_FLAG_OBJECT_TYPE_PRESENT	= 0x1,
        ADS_FLAG_INHERITED_OBJECT_TYPE_PRESENT	= 0x2
    } 	ADS_FLAGTYPE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0048_0005
    {
        ADS_SD_CONTROL_SE_OWNER_DEFAULTED	= 0x1,
        ADS_SD_CONTROL_SE_GROUP_DEFAULTED	= 0x2,
        ADS_SD_CONTROL_SE_DACL_PRESENT	= 0x4,
        ADS_SD_CONTROL_SE_DACL_DEFAULTED	= 0x8,
        ADS_SD_CONTROL_SE_SACL_PRESENT	= 0x10,
        ADS_SD_CONTROL_SE_SACL_DEFAULTED	= 0x20,
        ADS_SD_CONTROL_SE_DACL_AUTO_INHERIT_REQ	= 0x100,
        ADS_SD_CONTROL_SE_SACL_AUTO_INHERIT_REQ	= 0x200,
        ADS_SD_CONTROL_SE_DACL_AUTO_INHERITED	= 0x400,
        ADS_SD_CONTROL_SE_SACL_AUTO_INHERITED	= 0x800,
        ADS_SD_CONTROL_SE_DACL_PROTECTED	= 0x1000,
        ADS_SD_CONTROL_SE_SACL_PROTECTED	= 0x2000,
        ADS_SD_CONTROL_SE_SELF_RELATIVE	= 0x8000
    } 	ADS_SD_CONTROL_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0048_0006
    {
        ADS_SD_REVISION_DS	= 4
    } 	ADS_SD_REVISION_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0050_0001
    {
        ADS_NAME_TYPE_1779	= 1,
        ADS_NAME_TYPE_CANONICAL	= 2,
        ADS_NAME_TYPE_NT4	= 3,
        ADS_NAME_TYPE_DISPLAY	= 4,
        ADS_NAME_TYPE_DOMAIN_SIMPLE	= 5,
        ADS_NAME_TYPE_ENTERPRISE_SIMPLE	= 6,
        ADS_NAME_TYPE_GUID	= 7,
        ADS_NAME_TYPE_UNKNOWN	= 8,
        ADS_NAME_TYPE_USER_PRINCIPAL_NAME	= 9,
        ADS_NAME_TYPE_CANONICAL_EX	= 10,
        ADS_NAME_TYPE_SERVICE_PRINCIPAL_NAME	= 11,
        ADS_NAME_TYPE_SID_OR_SID_HISTORY_NAME	= 12
    } 	ADS_NAME_TYPE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0050_0002
    {
        ADS_NAME_INITTYPE_DOMAIN	= 1,
        ADS_NAME_INITTYPE_SERVER	= 2,
        ADS_NAME_INITTYPE_GC	= 3
    } 	ADS_NAME_INITTYPE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0077_0001
    {
        ADS_OPTION_SERVERNAME	= 0,
        ADS_OPTION_REFERRALS	= ( ADS_OPTION_SERVERNAME + 1 ) ,
        ADS_OPTION_PAGE_SIZE	= ( ADS_OPTION_REFERRALS + 1 ) ,
        ADS_OPTION_SECURITY_MASK	= ( ADS_OPTION_PAGE_SIZE + 1 ) ,
        ADS_OPTION_MUTUAL_AUTH_STATUS	= ( ADS_OPTION_SECURITY_MASK + 1 ) ,
        ADS_OPTION_QUOTA	= ( ADS_OPTION_MUTUAL_AUTH_STATUS + 1 ) ,
        ADS_OPTION_PASSWORD_PORTNUMBER	= ( ADS_OPTION_QUOTA + 1 ) ,
        ADS_OPTION_PASSWORD_METHOD	= ( ADS_OPTION_PASSWORD_PORTNUMBER + 1 ) ,
        ADS_OPTION_ACCUMULATIVE_MODIFICATION	= ( ADS_OPTION_PASSWORD_METHOD + 1 ) ,
        ADS_OPTION_SKIP_SID_LOOKUP	= ( ADS_OPTION_ACCUMULATIVE_MODIFICATION + 1 ) 
    } 	ADS_OPTION_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0077_0002
    {
        ADS_SECURITY_INFO_OWNER	= 0x1,
        ADS_SECURITY_INFO_GROUP	= 0x2,
        ADS_SECURITY_INFO_DACL	= 0x4,
        ADS_SECURITY_INFO_SACL	= 0x8
    } 	ADS_SECURITY_INFO_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0078_0001
    {
        ADS_SETTYPE_FULL	= 1,
        ADS_SETTYPE_PROVIDER	= 2,
        ADS_SETTYPE_SERVER	= 3,
        ADS_SETTYPE_DN	= 4
    } 	ADS_SETTYPE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0078_0002
    {
        ADS_FORMAT_WINDOWS	= 1,
        ADS_FORMAT_WINDOWS_NO_SERVER	= 2,
        ADS_FORMAT_WINDOWS_DN	= 3,
        ADS_FORMAT_WINDOWS_PARENT	= 4,
        ADS_FORMAT_X500	= 5,
        ADS_FORMAT_X500_NO_SERVER	= 6,
        ADS_FORMAT_X500_DN	= 7,
        ADS_FORMAT_X500_PARENT	= 8,
        ADS_FORMAT_SERVER	= 9,
        ADS_FORMAT_PROVIDER	= 10,
        ADS_FORMAT_LEAF	= 11
    } 	ADS_FORMAT_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0078_0003
    {
        ADS_DISPLAY_FULL	= 1,
        ADS_DISPLAY_VALUE_ONLY	= 2
    } 	ADS_DISPLAY_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0078_0004
    {
        ADS_ESCAPEDMODE_DEFAULT	= 1,
        ADS_ESCAPEDMODE_ON	= 2,
        ADS_ESCAPEDMODE_OFF	= 3,
        ADS_ESCAPEDMODE_OFF_EX	= 4
    } 	ADS_ESCAPE_MODE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0088_0001
    {
        ADS_PATH_FILE	= 1,
        ADS_PATH_FILESHARE	= 2,
        ADS_PATH_REGISTRY	= 3
    } 	ADS_PATHTYPE_ENUM;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_ads_0001_0088_0002
    {
        ADS_SD_FORMAT_IID	= 1,
        ADS_SD_FORMAT_RAW	= 2,
        ADS_SD_FORMAT_HEXSTRING	= 3
    } 	ADS_SD_FORMAT_ENUM;


EXTERN_C const IID LIBID_ActiveDs;

#ifndef __IADs_INTERFACE_DEFINED__
#define __IADs_INTERFACE_DEFINED__

/* interface IADs */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fd8256d0-fd15-11ce-abc4-02608c9e7553")
    IADs : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Class( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_GUID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ADsPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Schema( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetInfo( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetInfo( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Put( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEx( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PutEx( 
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetInfoEx( 
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADs * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADs * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADs * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADs * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADs * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADs * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADs * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADs * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADs * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADs * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADs * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADs * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADs * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADs * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        END_INTERFACE
    } IADsVtbl;

    interface IADs
    {
        CONST_VTBL struct IADsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADs_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADs_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADs_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADs_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADs_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADs_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADs_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADs_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADs_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADs_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADs_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADs_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADs_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADs_INTERFACE_DEFINED__ */


#ifndef __IADsContainer_INTERFACE_DEFINED__
#define __IADsContainer_INTERFACE_DEFINED__

/* interface IADsContainer */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("001677d0-fd16-11ce-abc4-02608c9e7553")
    IADsContainer : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Filter( 
            /* [retval][out] */ __RPC__out VARIANT *pVar) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Filter( 
            /* [in] */ VARIANT Var) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Hints( 
            /* [retval][out] */ __RPC__out VARIANT *pvFilter) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Hints( 
            /* [in] */ VARIANT vHints) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetObject( 
            /* [in] */ __RPC__in BSTR ClassName,
            /* [in] */ __RPC__in BSTR RelativeName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObject) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in BSTR ClassName,
            /* [in] */ __RPC__in BSTR RelativeName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObject) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in BSTR bstrClassName,
            /* [in] */ __RPC__in BSTR bstrRelativeName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CopyHere( 
            /* [in] */ __RPC__in BSTR SourceName,
            /* [in] */ __RPC__in BSTR NewName,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **ppObject) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE MoveHere( 
            /* [in] */ __RPC__in BSTR SourceName,
            /* [in] */ __RPC__in BSTR NewName,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **ppObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsContainer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsContainer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsContainer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsContainer * This,
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
        
        DECLSPEC_XFGVIRT(IADsContainer, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IADsContainer * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsContainer, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IADsContainer * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(IADsContainer, get_Filter)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in IADsContainer * This,
            /* [retval][out] */ __RPC__out VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(IADsContainer, put_Filter)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Filter )( 
            __RPC__in IADsContainer * This,
            /* [in] */ VARIANT Var);
        
        DECLSPEC_XFGVIRT(IADsContainer, get_Hints)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Hints )( 
            __RPC__in IADsContainer * This,
            /* [retval][out] */ __RPC__out VARIANT *pvFilter);
        
        DECLSPEC_XFGVIRT(IADsContainer, put_Hints)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Hints )( 
            __RPC__in IADsContainer * This,
            /* [in] */ VARIANT vHints);
        
        DECLSPEC_XFGVIRT(IADsContainer, GetObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in BSTR ClassName,
            /* [in] */ __RPC__in BSTR RelativeName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObject);
        
        DECLSPEC_XFGVIRT(IADsContainer, Create)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in BSTR ClassName,
            /* [in] */ __RPC__in BSTR RelativeName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObject);
        
        DECLSPEC_XFGVIRT(IADsContainer, Delete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in BSTR bstrClassName,
            /* [in] */ __RPC__in BSTR bstrRelativeName);
        
        DECLSPEC_XFGVIRT(IADsContainer, CopyHere)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CopyHere )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in BSTR SourceName,
            /* [in] */ __RPC__in BSTR NewName,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **ppObject);
        
        DECLSPEC_XFGVIRT(IADsContainer, MoveHere)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *MoveHere )( 
            __RPC__in IADsContainer * This,
            /* [in] */ __RPC__in BSTR SourceName,
            /* [in] */ __RPC__in BSTR NewName,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **ppObject);
        
        END_INTERFACE
    } IADsContainerVtbl;

    interface IADsContainer
    {
        CONST_VTBL struct IADsContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsContainer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsContainer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsContainer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsContainer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsContainer_get_Count(This,retval)	\
    ( (This)->lpVtbl -> get_Count(This,retval) ) 

#define IADsContainer_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define IADsContainer_get_Filter(This,pVar)	\
    ( (This)->lpVtbl -> get_Filter(This,pVar) ) 

#define IADsContainer_put_Filter(This,Var)	\
    ( (This)->lpVtbl -> put_Filter(This,Var) ) 

#define IADsContainer_get_Hints(This,pvFilter)	\
    ( (This)->lpVtbl -> get_Hints(This,pvFilter) ) 

#define IADsContainer_put_Hints(This,vHints)	\
    ( (This)->lpVtbl -> put_Hints(This,vHints) ) 

#define IADsContainer_GetObject(This,ClassName,RelativeName,ppObject)	\
    ( (This)->lpVtbl -> GetObject(This,ClassName,RelativeName,ppObject) ) 

#define IADsContainer_Create(This,ClassName,RelativeName,ppObject)	\
    ( (This)->lpVtbl -> Create(This,ClassName,RelativeName,ppObject) ) 

#define IADsContainer_Delete(This,bstrClassName,bstrRelativeName)	\
    ( (This)->lpVtbl -> Delete(This,bstrClassName,bstrRelativeName) ) 

#define IADsContainer_CopyHere(This,SourceName,NewName,ppObject)	\
    ( (This)->lpVtbl -> CopyHere(This,SourceName,NewName,ppObject) ) 

#define IADsContainer_MoveHere(This,SourceName,NewName,ppObject)	\
    ( (This)->lpVtbl -> MoveHere(This,SourceName,NewName,ppObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsContainer_INTERFACE_DEFINED__ */


#ifndef __IADsCollection_INTERFACE_DEFINED__
#define __IADsCollection_INTERFACE_DEFINED__

/* interface IADsCollection */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72b945e0-253b-11cf-a988-00aa006bc149")
    IADsCollection : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumerator) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vItem) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR bstrItemToBeRemoved) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetObject( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvItem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsCollection * This,
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
        
        DECLSPEC_XFGVIRT(IADsCollection, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IADsCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumerator);
        
        DECLSPEC_XFGVIRT(IADsCollection, Add)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IADsCollection * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vItem);
        
        DECLSPEC_XFGVIRT(IADsCollection, Remove)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IADsCollection * This,
            /* [in] */ __RPC__in BSTR bstrItemToBeRemoved);
        
        DECLSPEC_XFGVIRT(IADsCollection, GetObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IADsCollection * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvItem);
        
        END_INTERFACE
    } IADsCollectionVtbl;

    interface IADsCollection
    {
        CONST_VTBL struct IADsCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsCollection_get__NewEnum(This,ppEnumerator)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumerator) ) 

#define IADsCollection_Add(This,bstrName,vItem)	\
    ( (This)->lpVtbl -> Add(This,bstrName,vItem) ) 

#define IADsCollection_Remove(This,bstrItemToBeRemoved)	\
    ( (This)->lpVtbl -> Remove(This,bstrItemToBeRemoved) ) 

#define IADsCollection_GetObject(This,bstrName,pvItem)	\
    ( (This)->lpVtbl -> GetObject(This,bstrName,pvItem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsCollection_INTERFACE_DEFINED__ */


#ifndef __IADsMembers_INTERFACE_DEFINED__
#define __IADsMembers_INTERFACE_DEFINED__

/* interface IADsMembers */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsMembers;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("451a0030-72ec-11cf-b03b-00aa006e0975")
    IADsMembers : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumerator) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Filter( 
            /* [retval][out] */ __RPC__out VARIANT *pvFilter) = 0;
        
        virtual /* [propput][id] */ HRESULT STDMETHODCALLTYPE put_Filter( 
            /* [in] */ VARIANT pvFilter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsMembersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsMembers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsMembers * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsMembers * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsMembers * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsMembers * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsMembers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsMembers * This,
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
        
        DECLSPEC_XFGVIRT(IADsMembers, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IADsMembers * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IADsMembers, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IADsMembers * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumerator);
        
        DECLSPEC_XFGVIRT(IADsMembers, get_Filter)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Filter )( 
            __RPC__in IADsMembers * This,
            /* [retval][out] */ __RPC__out VARIANT *pvFilter);
        
        DECLSPEC_XFGVIRT(IADsMembers, put_Filter)
        /* [propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Filter )( 
            __RPC__in IADsMembers * This,
            /* [in] */ VARIANT pvFilter);
        
        END_INTERFACE
    } IADsMembersVtbl;

    interface IADsMembers
    {
        CONST_VTBL struct IADsMembersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsMembers_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsMembers_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsMembers_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsMembers_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsMembers_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsMembers_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsMembers_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsMembers_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IADsMembers_get__NewEnum(This,ppEnumerator)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumerator) ) 

#define IADsMembers_get_Filter(This,pvFilter)	\
    ( (This)->lpVtbl -> get_Filter(This,pvFilter) ) 

#define IADsMembers_put_Filter(This,pvFilter)	\
    ( (This)->lpVtbl -> put_Filter(This,pvFilter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsMembers_INTERFACE_DEFINED__ */


#ifndef __IADsPropertyList_INTERFACE_DEFINED__
#define __IADsPropertyList_INTERFACE_DEFINED__

/* interface IADsPropertyList */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPropertyList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c6f602b6-8f69-11d0-8528-00c04fd8d503")
    IADsPropertyList : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_PropertyCount( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ long cElements) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetPropertyItem( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ LONG lnADsType,
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PutPropertyItem( 
            /* [in] */ VARIANT varData) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ResetPropertyItem( 
            /* [in] */ VARIANT varEntry) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PurgePropertyList( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPropertyListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPropertyList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPropertyList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPropertyList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPropertyList * This,
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
        
        DECLSPEC_XFGVIRT(IADsPropertyList, get_PropertyCount)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PropertyCount )( 
            __RPC__in IADsPropertyList * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, Next)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IADsPropertyList * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, Skip)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ long cElements);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, Reset)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IADsPropertyList * This);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, Item)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, GetPropertyItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyItem )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ LONG lnADsType,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, PutPropertyItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutPropertyItem )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ VARIANT varData);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, ResetPropertyItem)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ResetPropertyItem )( 
            __RPC__in IADsPropertyList * This,
            /* [in] */ VARIANT varEntry);
        
        DECLSPEC_XFGVIRT(IADsPropertyList, PurgePropertyList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PurgePropertyList )( 
            __RPC__in IADsPropertyList * This);
        
        END_INTERFACE
    } IADsPropertyListVtbl;

    interface IADsPropertyList
    {
        CONST_VTBL struct IADsPropertyListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPropertyList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPropertyList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPropertyList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPropertyList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPropertyList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPropertyList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPropertyList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPropertyList_get_PropertyCount(This,plCount)	\
    ( (This)->lpVtbl -> get_PropertyCount(This,plCount) ) 

#define IADsPropertyList_Next(This,pVariant)	\
    ( (This)->lpVtbl -> Next(This,pVariant) ) 

#define IADsPropertyList_Skip(This,cElements)	\
    ( (This)->lpVtbl -> Skip(This,cElements) ) 

#define IADsPropertyList_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IADsPropertyList_Item(This,varIndex,pVariant)	\
    ( (This)->lpVtbl -> Item(This,varIndex,pVariant) ) 

#define IADsPropertyList_GetPropertyItem(This,bstrName,lnADsType,pVariant)	\
    ( (This)->lpVtbl -> GetPropertyItem(This,bstrName,lnADsType,pVariant) ) 

#define IADsPropertyList_PutPropertyItem(This,varData)	\
    ( (This)->lpVtbl -> PutPropertyItem(This,varData) ) 

#define IADsPropertyList_ResetPropertyItem(This,varEntry)	\
    ( (This)->lpVtbl -> ResetPropertyItem(This,varEntry) ) 

#define IADsPropertyList_PurgePropertyList(This)	\
    ( (This)->lpVtbl -> PurgePropertyList(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPropertyList_INTERFACE_DEFINED__ */


#ifndef __IADsPropertyEntry_INTERFACE_DEFINED__
#define __IADsPropertyEntry_INTERFACE_DEFINED__

/* interface IADsPropertyEntry */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPropertyEntry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05792c8e-941f-11d0-8529-00c04fd8d503")
    IADsPropertyEntry : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ADsType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ADsType( 
            /* [in] */ long lnADsType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ControlCode( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ControlCode( 
            /* [in] */ long lnControlCode) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Values( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Values( 
            /* [in] */ VARIANT vValues) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPropertyEntryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPropertyEntry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPropertyEntry * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPropertyEntry * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPropertyEntry * This,
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
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, Clear)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IADsPropertyEntry * This);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsPropertyEntry * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, put_Name)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, get_ADsType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsType )( 
            __RPC__in IADsPropertyEntry * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, put_ADsType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ADsType )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ long lnADsType);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, get_ControlCode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ControlCode )( 
            __RPC__in IADsPropertyEntry * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, put_ControlCode)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ControlCode )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ long lnControlCode);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, get_Values)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Values )( 
            __RPC__in IADsPropertyEntry * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyEntry, put_Values)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Values )( 
            __RPC__in IADsPropertyEntry * This,
            /* [in] */ VARIANT vValues);
        
        END_INTERFACE
    } IADsPropertyEntryVtbl;

    interface IADsPropertyEntry
    {
        CONST_VTBL struct IADsPropertyEntryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPropertyEntry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPropertyEntry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPropertyEntry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPropertyEntry_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPropertyEntry_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPropertyEntry_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPropertyEntry_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPropertyEntry_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IADsPropertyEntry_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsPropertyEntry_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IADsPropertyEntry_get_ADsType(This,retval)	\
    ( (This)->lpVtbl -> get_ADsType(This,retval) ) 

#define IADsPropertyEntry_put_ADsType(This,lnADsType)	\
    ( (This)->lpVtbl -> put_ADsType(This,lnADsType) ) 

#define IADsPropertyEntry_get_ControlCode(This,retval)	\
    ( (This)->lpVtbl -> get_ControlCode(This,retval) ) 

#define IADsPropertyEntry_put_ControlCode(This,lnControlCode)	\
    ( (This)->lpVtbl -> put_ControlCode(This,lnControlCode) ) 

#define IADsPropertyEntry_get_Values(This,retval)	\
    ( (This)->lpVtbl -> get_Values(This,retval) ) 

#define IADsPropertyEntry_put_Values(This,vValues)	\
    ( (This)->lpVtbl -> put_Values(This,vValues) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPropertyEntry_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_PropertyEntry;

#ifdef __cplusplus

class DECLSPEC_UUID("72d3edc2-a4c4-11d0-8533-00c04fd8d503")
PropertyEntry;
#endif

#ifndef __IADsPropertyValue_INTERFACE_DEFINED__
#define __IADsPropertyValue_INTERFACE_DEFINED__

/* interface IADsPropertyValue */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPropertyValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79fa9ad0-a97c-11d0-8534-00c04fd8d503")
    IADsPropertyValue : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ADsType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ADsType( 
            /* [in] */ long lnADsType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DNString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DNString( 
            /* [in] */ __RPC__in BSTR bstrDNString) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CaseExactString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_CaseExactString( 
            /* [in] */ __RPC__in BSTR bstrCaseExactString) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CaseIgnoreString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_CaseIgnoreString( 
            /* [in] */ __RPC__in BSTR bstrCaseIgnoreString) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrintableString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PrintableString( 
            /* [in] */ __RPC__in BSTR bstrPrintableString) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NumericString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NumericString( 
            /* [in] */ __RPC__in BSTR bstrNumericString) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Boolean( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Boolean( 
            /* [in] */ long lnBoolean) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Integer( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Integer( 
            /* [in] */ long lnInteger) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OctetString( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OctetString( 
            /* [in] */ VARIANT vOctetString) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SecurityDescriptor( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SecurityDescriptor( 
            /* [in] */ __RPC__in_opt IDispatch *pSecurityDescriptor) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LargeInteger( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LargeInteger( 
            /* [in] */ __RPC__in_opt IDispatch *pLargeInteger) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UTCTime( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_UTCTime( 
            /* [in] */ DATE daUTCTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPropertyValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPropertyValue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPropertyValue * This,
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
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, Clear)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IADsPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_ADsType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsType )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_ADsType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ADsType )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ long lnADsType);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_DNString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DNString )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_DNString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DNString )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in BSTR bstrDNString);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_CaseExactString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CaseExactString )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_CaseExactString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CaseExactString )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in BSTR bstrCaseExactString);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_CaseIgnoreString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CaseIgnoreString )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_CaseIgnoreString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CaseIgnoreString )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in BSTR bstrCaseIgnoreString);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_PrintableString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrintableString )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_PrintableString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrintableString )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in BSTR bstrPrintableString);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_NumericString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumericString )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_NumericString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NumericString )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in BSTR bstrNumericString);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_Boolean)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Boolean )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_Boolean)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Boolean )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ long lnBoolean);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_Integer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Integer )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_Integer)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Integer )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ long lnInteger);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_OctetString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OctetString )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_OctetString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OctetString )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ VARIANT vOctetString);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_SecurityDescriptor)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SecurityDescriptor )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_SecurityDescriptor)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SecurityDescriptor )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in_opt IDispatch *pSecurityDescriptor);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_LargeInteger)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LargeInteger )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_LargeInteger)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LargeInteger )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ __RPC__in_opt IDispatch *pLargeInteger);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, get_UTCTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UTCTime )( 
            __RPC__in IADsPropertyValue * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue, put_UTCTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UTCTime )( 
            __RPC__in IADsPropertyValue * This,
            /* [in] */ DATE daUTCTime);
        
        END_INTERFACE
    } IADsPropertyValueVtbl;

    interface IADsPropertyValue
    {
        CONST_VTBL struct IADsPropertyValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPropertyValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPropertyValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPropertyValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPropertyValue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPropertyValue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPropertyValue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPropertyValue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPropertyValue_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IADsPropertyValue_get_ADsType(This,retval)	\
    ( (This)->lpVtbl -> get_ADsType(This,retval) ) 

#define IADsPropertyValue_put_ADsType(This,lnADsType)	\
    ( (This)->lpVtbl -> put_ADsType(This,lnADsType) ) 

#define IADsPropertyValue_get_DNString(This,retval)	\
    ( (This)->lpVtbl -> get_DNString(This,retval) ) 

#define IADsPropertyValue_put_DNString(This,bstrDNString)	\
    ( (This)->lpVtbl -> put_DNString(This,bstrDNString) ) 

#define IADsPropertyValue_get_CaseExactString(This,retval)	\
    ( (This)->lpVtbl -> get_CaseExactString(This,retval) ) 

#define IADsPropertyValue_put_CaseExactString(This,bstrCaseExactString)	\
    ( (This)->lpVtbl -> put_CaseExactString(This,bstrCaseExactString) ) 

#define IADsPropertyValue_get_CaseIgnoreString(This,retval)	\
    ( (This)->lpVtbl -> get_CaseIgnoreString(This,retval) ) 

#define IADsPropertyValue_put_CaseIgnoreString(This,bstrCaseIgnoreString)	\
    ( (This)->lpVtbl -> put_CaseIgnoreString(This,bstrCaseIgnoreString) ) 

#define IADsPropertyValue_get_PrintableString(This,retval)	\
    ( (This)->lpVtbl -> get_PrintableString(This,retval) ) 

#define IADsPropertyValue_put_PrintableString(This,bstrPrintableString)	\
    ( (This)->lpVtbl -> put_PrintableString(This,bstrPrintableString) ) 

#define IADsPropertyValue_get_NumericString(This,retval)	\
    ( (This)->lpVtbl -> get_NumericString(This,retval) ) 

#define IADsPropertyValue_put_NumericString(This,bstrNumericString)	\
    ( (This)->lpVtbl -> put_NumericString(This,bstrNumericString) ) 

#define IADsPropertyValue_get_Boolean(This,retval)	\
    ( (This)->lpVtbl -> get_Boolean(This,retval) ) 

#define IADsPropertyValue_put_Boolean(This,lnBoolean)	\
    ( (This)->lpVtbl -> put_Boolean(This,lnBoolean) ) 

#define IADsPropertyValue_get_Integer(This,retval)	\
    ( (This)->lpVtbl -> get_Integer(This,retval) ) 

#define IADsPropertyValue_put_Integer(This,lnInteger)	\
    ( (This)->lpVtbl -> put_Integer(This,lnInteger) ) 

#define IADsPropertyValue_get_OctetString(This,retval)	\
    ( (This)->lpVtbl -> get_OctetString(This,retval) ) 

#define IADsPropertyValue_put_OctetString(This,vOctetString)	\
    ( (This)->lpVtbl -> put_OctetString(This,vOctetString) ) 

#define IADsPropertyValue_get_SecurityDescriptor(This,retval)	\
    ( (This)->lpVtbl -> get_SecurityDescriptor(This,retval) ) 

#define IADsPropertyValue_put_SecurityDescriptor(This,pSecurityDescriptor)	\
    ( (This)->lpVtbl -> put_SecurityDescriptor(This,pSecurityDescriptor) ) 

#define IADsPropertyValue_get_LargeInteger(This,retval)	\
    ( (This)->lpVtbl -> get_LargeInteger(This,retval) ) 

#define IADsPropertyValue_put_LargeInteger(This,pLargeInteger)	\
    ( (This)->lpVtbl -> put_LargeInteger(This,pLargeInteger) ) 

#define IADsPropertyValue_get_UTCTime(This,retval)	\
    ( (This)->lpVtbl -> get_UTCTime(This,retval) ) 

#define IADsPropertyValue_put_UTCTime(This,daUTCTime)	\
    ( (This)->lpVtbl -> put_UTCTime(This,daUTCTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPropertyValue_INTERFACE_DEFINED__ */


#ifndef __IADsPropertyValue2_INTERFACE_DEFINED__
#define __IADsPropertyValue2_INTERFACE_DEFINED__

/* interface IADsPropertyValue2 */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPropertyValue2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("306e831c-5bc7-11d1-a3b8-00c04fb950dc")
    IADsPropertyValue2 : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetObjectProperty( 
            /* [out][in] */ __RPC__inout long *lnADsType,
            /* [retval][out] */ __RPC__out VARIANT *pvProp) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PutObjectProperty( 
            /* [in] */ long lnADsType,
            /* [in] */ VARIANT vProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPropertyValue2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPropertyValue2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPropertyValue2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPropertyValue2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPropertyValue2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPropertyValue2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPropertyValue2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPropertyValue2 * This,
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
        
        DECLSPEC_XFGVIRT(IADsPropertyValue2, GetObjectProperty)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetObjectProperty )( 
            __RPC__in IADsPropertyValue2 * This,
            /* [out][in] */ __RPC__inout long *lnADsType,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADsPropertyValue2, PutObjectProperty)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutObjectProperty )( 
            __RPC__in IADsPropertyValue2 * This,
            /* [in] */ long lnADsType,
            /* [in] */ VARIANT vProp);
        
        END_INTERFACE
    } IADsPropertyValue2Vtbl;

    interface IADsPropertyValue2
    {
        CONST_VTBL struct IADsPropertyValue2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPropertyValue2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPropertyValue2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPropertyValue2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPropertyValue2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPropertyValue2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPropertyValue2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPropertyValue2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPropertyValue2_GetObjectProperty(This,lnADsType,pvProp)	\
    ( (This)->lpVtbl -> GetObjectProperty(This,lnADsType,pvProp) ) 

#define IADsPropertyValue2_PutObjectProperty(This,lnADsType,vProp)	\
    ( (This)->lpVtbl -> PutObjectProperty(This,lnADsType,vProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPropertyValue2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_PropertyValue;

#ifdef __cplusplus

class DECLSPEC_UUID("7b9e38b0-a97c-11d0-8534-00c04fd8d503")
PropertyValue;
#endif

#ifndef __IPrivateDispatch_INTERFACE_DEFINED__
#define __IPrivateDispatch_INTERFACE_DEFINED__

/* interface IPrivateDispatch */
/* [object][uuid] */ 


EXTERN_C const IID IID_IPrivateDispatch;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86ab4bbe-65f6-11d1-8c13-00c04fd8d503")
    IPrivateDispatch : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ADSIInitializeDispatchManager( 
            /* [in] */ long dwExtensionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ADSIGetTypeInfoCount( 
            /* [out] */ __RPC__out UINT *pctinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ADSIGetTypeInfo( 
            /* [in] */ UINT itinfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **pptinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ADSIGetIDsOfNames( 
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ __RPC__deref_in_opt OLECHAR **rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__out DISPID *rgdispid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ADSIInvoke( 
            /* [in] */ DISPID dispidMember,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [in] */ __RPC__in DISPPARAMS *pdispparams,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo,
            /* [out] */ __RPC__out UINT *puArgErr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrivateDispatchVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrivateDispatch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrivateDispatch * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrivateDispatch * This);
        
        DECLSPEC_XFGVIRT(IPrivateDispatch, ADSIInitializeDispatchManager)
        HRESULT ( STDMETHODCALLTYPE *ADSIInitializeDispatchManager )( 
            __RPC__in IPrivateDispatch * This,
            /* [in] */ long dwExtensionId);
        
        DECLSPEC_XFGVIRT(IPrivateDispatch, ADSIGetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *ADSIGetTypeInfoCount )( 
            __RPC__in IPrivateDispatch * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IPrivateDispatch, ADSIGetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *ADSIGetTypeInfo )( 
            __RPC__in IPrivateDispatch * This,
            /* [in] */ UINT itinfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **pptinfo);
        
        DECLSPEC_XFGVIRT(IPrivateDispatch, ADSIGetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *ADSIGetIDsOfNames )( 
            __RPC__in IPrivateDispatch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ __RPC__deref_in_opt OLECHAR **rgszNames,
            /* [in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__out DISPID *rgdispid);
        
        DECLSPEC_XFGVIRT(IPrivateDispatch, ADSIInvoke)
        HRESULT ( STDMETHODCALLTYPE *ADSIInvoke )( 
            __RPC__in IPrivateDispatch * This,
            /* [in] */ DISPID dispidMember,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [in] */ __RPC__in DISPPARAMS *pdispparams,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo,
            /* [out] */ __RPC__out UINT *puArgErr);
        
        END_INTERFACE
    } IPrivateDispatchVtbl;

    interface IPrivateDispatch
    {
        CONST_VTBL struct IPrivateDispatchVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrivateDispatch_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrivateDispatch_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrivateDispatch_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrivateDispatch_ADSIInitializeDispatchManager(This,dwExtensionId)	\
    ( (This)->lpVtbl -> ADSIInitializeDispatchManager(This,dwExtensionId) ) 

#define IPrivateDispatch_ADSIGetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> ADSIGetTypeInfoCount(This,pctinfo) ) 

#define IPrivateDispatch_ADSIGetTypeInfo(This,itinfo,lcid,pptinfo)	\
    ( (This)->lpVtbl -> ADSIGetTypeInfo(This,itinfo,lcid,pptinfo) ) 

#define IPrivateDispatch_ADSIGetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid)	\
    ( (This)->lpVtbl -> ADSIGetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgdispid) ) 

#define IPrivateDispatch_ADSIInvoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr)	\
    ( (This)->lpVtbl -> ADSIInvoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrivateDispatch_INTERFACE_DEFINED__ */


#ifndef __IPrivateUnknown_INTERFACE_DEFINED__
#define __IPrivateUnknown_INTERFACE_DEFINED__

/* interface IPrivateUnknown */
/* [object][uuid] */ 


EXTERN_C const IID IID_IPrivateUnknown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89126bab-6ead-11d1-8c18-00c04fd8d503")
    IPrivateUnknown : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ADSIInitializeObject( 
            /* [in] */ __RPC__in BSTR lpszUserName,
            /* [in] */ __RPC__in BSTR lpszPassword,
            /* [in] */ long lnReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ADSIReleaseObject( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrivateUnknownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrivateUnknown * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrivateUnknown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrivateUnknown * This);
        
        DECLSPEC_XFGVIRT(IPrivateUnknown, ADSIInitializeObject)
        HRESULT ( STDMETHODCALLTYPE *ADSIInitializeObject )( 
            __RPC__in IPrivateUnknown * This,
            /* [in] */ __RPC__in BSTR lpszUserName,
            /* [in] */ __RPC__in BSTR lpszPassword,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IPrivateUnknown, ADSIReleaseObject)
        HRESULT ( STDMETHODCALLTYPE *ADSIReleaseObject )( 
            __RPC__in IPrivateUnknown * This);
        
        END_INTERFACE
    } IPrivateUnknownVtbl;

    interface IPrivateUnknown
    {
        CONST_VTBL struct IPrivateUnknownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrivateUnknown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrivateUnknown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrivateUnknown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrivateUnknown_ADSIInitializeObject(This,lpszUserName,lpszPassword,lnReserved)	\
    ( (This)->lpVtbl -> ADSIInitializeObject(This,lpszUserName,lpszPassword,lnReserved) ) 

#define IPrivateUnknown_ADSIReleaseObject(This)	\
    ( (This)->lpVtbl -> ADSIReleaseObject(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrivateUnknown_INTERFACE_DEFINED__ */


#ifndef __IADsExtension_INTERFACE_DEFINED__
#define __IADsExtension_INTERFACE_DEFINED__

/* interface IADsExtension */
/* [object][uuid] */ 


EXTERN_C const IID IID_IADsExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3d35553c-d2b0-11d1-b17b-0000f87593a0")
    IADsExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Operate( 
            /* [in] */ DWORD dwCode,
            /* [in] */ VARIANT varData1,
            /* [in] */ VARIANT varData2,
            /* [in] */ VARIANT varData3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrivateGetIDsOfNames( 
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ __RPC__deref_in_opt OLECHAR **rgszNames,
            /* [in] */ unsigned int cNames,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__out DISPID *rgDispid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrivateInvoke( 
            /* [in] */ DISPID dispidMember,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [in] */ __RPC__in DISPPARAMS *pdispparams,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo,
            /* [out] */ __RPC__out unsigned int *puArgErr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsExtension * This);
        
        DECLSPEC_XFGVIRT(IADsExtension, Operate)
        HRESULT ( STDMETHODCALLTYPE *Operate )( 
            __RPC__in IADsExtension * This,
            /* [in] */ DWORD dwCode,
            /* [in] */ VARIANT varData1,
            /* [in] */ VARIANT varData2,
            /* [in] */ VARIANT varData3);
        
        DECLSPEC_XFGVIRT(IADsExtension, PrivateGetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *PrivateGetIDsOfNames )( 
            __RPC__in IADsExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ __RPC__deref_in_opt OLECHAR **rgszNames,
            /* [in] */ unsigned int cNames,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__out DISPID *rgDispid);
        
        DECLSPEC_XFGVIRT(IADsExtension, PrivateInvoke)
        HRESULT ( STDMETHODCALLTYPE *PrivateInvoke )( 
            __RPC__in IADsExtension * This,
            /* [in] */ DISPID dispidMember,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ LCID lcid,
            /* [in] */ WORD wFlags,
            /* [in] */ __RPC__in DISPPARAMS *pdispparams,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo,
            /* [out] */ __RPC__out unsigned int *puArgErr);
        
        END_INTERFACE
    } IADsExtensionVtbl;

    interface IADsExtension
    {
        CONST_VTBL struct IADsExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsExtension_Operate(This,dwCode,varData1,varData2,varData3)	\
    ( (This)->lpVtbl -> Operate(This,dwCode,varData1,varData2,varData3) ) 

#define IADsExtension_PrivateGetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispid)	\
    ( (This)->lpVtbl -> PrivateGetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispid) ) 

#define IADsExtension_PrivateInvoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr)	\
    ( (This)->lpVtbl -> PrivateInvoke(This,dispidMember,riid,lcid,wFlags,pdispparams,pvarResult,pexcepinfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsExtension_INTERFACE_DEFINED__ */


#ifndef __IADsDeleteOps_INTERFACE_DEFINED__
#define __IADsDeleteOps_INTERFACE_DEFINED__

/* interface IADsDeleteOps */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsDeleteOps;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b2bd0902-8878-11d1-8c21-00c04fd8d503")
    IADsDeleteOps : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DeleteObject( 
            /* [in] */ long lnFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsDeleteOpsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsDeleteOps * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsDeleteOps * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsDeleteOps * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsDeleteOps * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsDeleteOps * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsDeleteOps * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsDeleteOps * This,
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
        
        DECLSPEC_XFGVIRT(IADsDeleteOps, DeleteObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DeleteObject )( 
            __RPC__in IADsDeleteOps * This,
            /* [in] */ long lnFlags);
        
        END_INTERFACE
    } IADsDeleteOpsVtbl;

    interface IADsDeleteOps
    {
        CONST_VTBL struct IADsDeleteOpsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsDeleteOps_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsDeleteOps_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsDeleteOps_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsDeleteOps_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsDeleteOps_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsDeleteOps_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsDeleteOps_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsDeleteOps_DeleteObject(This,lnFlags)	\
    ( (This)->lpVtbl -> DeleteObject(This,lnFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsDeleteOps_INTERFACE_DEFINED__ */


#ifndef __IADsNamespaces_INTERFACE_DEFINED__
#define __IADsNamespaces_INTERFACE_DEFINED__

/* interface IADsNamespaces */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IADsNamespaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("28b96ba0-b330-11cf-a9ad-00aa006bc149")
    IADsNamespaces : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultContainer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DefaultContainer( 
            /* [in] */ __RPC__in BSTR bstrDefaultContainer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsNamespacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsNamespaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsNamespaces * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsNamespaces * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsNamespaces * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsNamespaces * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsNamespaces * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsNamespaces, get_DefaultContainer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultContainer )( 
            __RPC__in IADsNamespaces * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsNamespaces, put_DefaultContainer)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultContainer )( 
            __RPC__in IADsNamespaces * This,
            /* [in] */ __RPC__in BSTR bstrDefaultContainer);
        
        END_INTERFACE
    } IADsNamespacesVtbl;

    interface IADsNamespaces
    {
        CONST_VTBL struct IADsNamespacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsNamespaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsNamespaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsNamespaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsNamespaces_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsNamespaces_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsNamespaces_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsNamespaces_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsNamespaces_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsNamespaces_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsNamespaces_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsNamespaces_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsNamespaces_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsNamespaces_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsNamespaces_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsNamespaces_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsNamespaces_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsNamespaces_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsNamespaces_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsNamespaces_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsNamespaces_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsNamespaces_get_DefaultContainer(This,retval)	\
    ( (This)->lpVtbl -> get_DefaultContainer(This,retval) ) 

#define IADsNamespaces_put_DefaultContainer(This,bstrDefaultContainer)	\
    ( (This)->lpVtbl -> put_DefaultContainer(This,bstrDefaultContainer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsNamespaces_INTERFACE_DEFINED__ */


#ifndef __IADsClass_INTERFACE_DEFINED__
#define __IADsClass_INTERFACE_DEFINED__

/* interface IADsClass */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsClass;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8f93dd0-4ae0-11cf-9e73-00aa004a5691")
    IADsClass : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrimaryInterface( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_CLSID( 
            /* [in] */ __RPC__in BSTR bstrCLSID) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OID( 
            /* [in] */ __RPC__in BSTR bstrOID) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Abstract( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Abstract( 
            /* [in] */ VARIANT_BOOL fAbstract) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Auxiliary( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Auxiliary( 
            /* [in] */ VARIANT_BOOL fAuxiliary) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MandatoryProperties( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MandatoryProperties( 
            /* [in] */ VARIANT vMandatoryProperties) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OptionalProperties( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OptionalProperties( 
            /* [in] */ VARIANT vOptionalProperties) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NamingProperties( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NamingProperties( 
            /* [in] */ VARIANT vNamingProperties) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DerivedFrom( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DerivedFrom( 
            /* [in] */ VARIANT vDerivedFrom) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AuxDerivedFrom( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AuxDerivedFrom( 
            /* [in] */ VARIANT vAuxDerivedFrom) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PossibleSuperiors( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PossibleSuperiors( 
            /* [in] */ VARIANT vPossibleSuperiors) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Containment( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Containment( 
            /* [in] */ VARIANT vContainment) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Container( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Container( 
            /* [in] */ VARIANT_BOOL fContainer) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HelpFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HelpFileName( 
            /* [in] */ __RPC__in BSTR bstrHelpFileName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HelpFileContext( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HelpFileContext( 
            /* [in] */ long lnHelpFileContext) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Qualifiers( 
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppQualifiers) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsClassVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsClass * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsClass * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsClass * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsClass * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsClass * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsClass * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsClass * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsClass * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsClass, get_PrimaryInterface)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrimaryInterface )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, get_CLSID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_CLSID)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CLSID )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in BSTR bstrCLSID);
        
        DECLSPEC_XFGVIRT(IADsClass, get_OID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OID )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_OID)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OID )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in BSTR bstrOID);
        
        DECLSPEC_XFGVIRT(IADsClass, get_Abstract)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Abstract )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_Abstract)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Abstract )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT_BOOL fAbstract);
        
        DECLSPEC_XFGVIRT(IADsClass, get_Auxiliary)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Auxiliary )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_Auxiliary)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Auxiliary )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT_BOOL fAuxiliary);
        
        DECLSPEC_XFGVIRT(IADsClass, get_MandatoryProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MandatoryProperties )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_MandatoryProperties)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MandatoryProperties )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vMandatoryProperties);
        
        DECLSPEC_XFGVIRT(IADsClass, get_OptionalProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OptionalProperties )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_OptionalProperties)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OptionalProperties )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vOptionalProperties);
        
        DECLSPEC_XFGVIRT(IADsClass, get_NamingProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamingProperties )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_NamingProperties)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NamingProperties )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vNamingProperties);
        
        DECLSPEC_XFGVIRT(IADsClass, get_DerivedFrom)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DerivedFrom )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_DerivedFrom)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DerivedFrom )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vDerivedFrom);
        
        DECLSPEC_XFGVIRT(IADsClass, get_AuxDerivedFrom)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuxDerivedFrom )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_AuxDerivedFrom)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AuxDerivedFrom )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vAuxDerivedFrom);
        
        DECLSPEC_XFGVIRT(IADsClass, get_PossibleSuperiors)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PossibleSuperiors )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_PossibleSuperiors)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PossibleSuperiors )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vPossibleSuperiors);
        
        DECLSPEC_XFGVIRT(IADsClass, get_Containment)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Containment )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_Containment)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Containment )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT vContainment);
        
        DECLSPEC_XFGVIRT(IADsClass, get_Container)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Container )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_Container)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Container )( 
            __RPC__in IADsClass * This,
            /* [in] */ VARIANT_BOOL fContainer);
        
        DECLSPEC_XFGVIRT(IADsClass, get_HelpFileName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HelpFileName )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_HelpFileName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HelpFileName )( 
            __RPC__in IADsClass * This,
            /* [in] */ __RPC__in BSTR bstrHelpFileName);
        
        DECLSPEC_XFGVIRT(IADsClass, get_HelpFileContext)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HelpFileContext )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsClass, put_HelpFileContext)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HelpFileContext )( 
            __RPC__in IADsClass * This,
            /* [in] */ long lnHelpFileContext);
        
        DECLSPEC_XFGVIRT(IADsClass, Qualifiers)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Qualifiers )( 
            __RPC__in IADsClass * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppQualifiers);
        
        END_INTERFACE
    } IADsClassVtbl;

    interface IADsClass
    {
        CONST_VTBL struct IADsClassVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsClass_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsClass_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsClass_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsClass_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsClass_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsClass_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsClass_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsClass_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsClass_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsClass_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsClass_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsClass_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsClass_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsClass_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsClass_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsClass_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsClass_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsClass_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsClass_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsClass_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsClass_get_PrimaryInterface(This,retval)	\
    ( (This)->lpVtbl -> get_PrimaryInterface(This,retval) ) 

#define IADsClass_get_CLSID(This,retval)	\
    ( (This)->lpVtbl -> get_CLSID(This,retval) ) 

#define IADsClass_put_CLSID(This,bstrCLSID)	\
    ( (This)->lpVtbl -> put_CLSID(This,bstrCLSID) ) 

#define IADsClass_get_OID(This,retval)	\
    ( (This)->lpVtbl -> get_OID(This,retval) ) 

#define IADsClass_put_OID(This,bstrOID)	\
    ( (This)->lpVtbl -> put_OID(This,bstrOID) ) 

#define IADsClass_get_Abstract(This,retval)	\
    ( (This)->lpVtbl -> get_Abstract(This,retval) ) 

#define IADsClass_put_Abstract(This,fAbstract)	\
    ( (This)->lpVtbl -> put_Abstract(This,fAbstract) ) 

#define IADsClass_get_Auxiliary(This,retval)	\
    ( (This)->lpVtbl -> get_Auxiliary(This,retval) ) 

#define IADsClass_put_Auxiliary(This,fAuxiliary)	\
    ( (This)->lpVtbl -> put_Auxiliary(This,fAuxiliary) ) 

#define IADsClass_get_MandatoryProperties(This,retval)	\
    ( (This)->lpVtbl -> get_MandatoryProperties(This,retval) ) 

#define IADsClass_put_MandatoryProperties(This,vMandatoryProperties)	\
    ( (This)->lpVtbl -> put_MandatoryProperties(This,vMandatoryProperties) ) 

#define IADsClass_get_OptionalProperties(This,retval)	\
    ( (This)->lpVtbl -> get_OptionalProperties(This,retval) ) 

#define IADsClass_put_OptionalProperties(This,vOptionalProperties)	\
    ( (This)->lpVtbl -> put_OptionalProperties(This,vOptionalProperties) ) 

#define IADsClass_get_NamingProperties(This,retval)	\
    ( (This)->lpVtbl -> get_NamingProperties(This,retval) ) 

#define IADsClass_put_NamingProperties(This,vNamingProperties)	\
    ( (This)->lpVtbl -> put_NamingProperties(This,vNamingProperties) ) 

#define IADsClass_get_DerivedFrom(This,retval)	\
    ( (This)->lpVtbl -> get_DerivedFrom(This,retval) ) 

#define IADsClass_put_DerivedFrom(This,vDerivedFrom)	\
    ( (This)->lpVtbl -> put_DerivedFrom(This,vDerivedFrom) ) 

#define IADsClass_get_AuxDerivedFrom(This,retval)	\
    ( (This)->lpVtbl -> get_AuxDerivedFrom(This,retval) ) 

#define IADsClass_put_AuxDerivedFrom(This,vAuxDerivedFrom)	\
    ( (This)->lpVtbl -> put_AuxDerivedFrom(This,vAuxDerivedFrom) ) 

#define IADsClass_get_PossibleSuperiors(This,retval)	\
    ( (This)->lpVtbl -> get_PossibleSuperiors(This,retval) ) 

#define IADsClass_put_PossibleSuperiors(This,vPossibleSuperiors)	\
    ( (This)->lpVtbl -> put_PossibleSuperiors(This,vPossibleSuperiors) ) 

#define IADsClass_get_Containment(This,retval)	\
    ( (This)->lpVtbl -> get_Containment(This,retval) ) 

#define IADsClass_put_Containment(This,vContainment)	\
    ( (This)->lpVtbl -> put_Containment(This,vContainment) ) 

#define IADsClass_get_Container(This,retval)	\
    ( (This)->lpVtbl -> get_Container(This,retval) ) 

#define IADsClass_put_Container(This,fContainer)	\
    ( (This)->lpVtbl -> put_Container(This,fContainer) ) 

#define IADsClass_get_HelpFileName(This,retval)	\
    ( (This)->lpVtbl -> get_HelpFileName(This,retval) ) 

#define IADsClass_put_HelpFileName(This,bstrHelpFileName)	\
    ( (This)->lpVtbl -> put_HelpFileName(This,bstrHelpFileName) ) 

#define IADsClass_get_HelpFileContext(This,retval)	\
    ( (This)->lpVtbl -> get_HelpFileContext(This,retval) ) 

#define IADsClass_put_HelpFileContext(This,lnHelpFileContext)	\
    ( (This)->lpVtbl -> put_HelpFileContext(This,lnHelpFileContext) ) 

#define IADsClass_Qualifiers(This,ppQualifiers)	\
    ( (This)->lpVtbl -> Qualifiers(This,ppQualifiers) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsClass_INTERFACE_DEFINED__ */


#ifndef __IADsProperty_INTERFACE_DEFINED__
#define __IADsProperty_INTERFACE_DEFINED__

/* interface IADsProperty */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8f93dd3-4ae0-11cf-9e73-00aa004a5691")
    IADsProperty : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OID( 
            /* [in] */ __RPC__in BSTR bstrOID) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Syntax( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Syntax( 
            /* [in] */ __RPC__in BSTR bstrSyntax) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxRange( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxRange( 
            /* [in] */ long lnMaxRange) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MinRange( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MinRange( 
            /* [in] */ long lnMinRange) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MultiValued( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MultiValued( 
            /* [in] */ VARIANT_BOOL fMultiValued) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Qualifiers( 
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppQualifiers) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsProperty * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsProperty * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsProperty * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsProperty * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsProperty * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsProperty, get_OID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OID )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsProperty, put_OID)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OID )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in BSTR bstrOID);
        
        DECLSPEC_XFGVIRT(IADsProperty, get_Syntax)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Syntax )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsProperty, put_Syntax)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Syntax )( 
            __RPC__in IADsProperty * This,
            /* [in] */ __RPC__in BSTR bstrSyntax);
        
        DECLSPEC_XFGVIRT(IADsProperty, get_MaxRange)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxRange )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsProperty, put_MaxRange)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxRange )( 
            __RPC__in IADsProperty * This,
            /* [in] */ long lnMaxRange);
        
        DECLSPEC_XFGVIRT(IADsProperty, get_MinRange)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinRange )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsProperty, put_MinRange)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinRange )( 
            __RPC__in IADsProperty * This,
            /* [in] */ long lnMinRange);
        
        DECLSPEC_XFGVIRT(IADsProperty, get_MultiValued)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MultiValued )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsProperty, put_MultiValued)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MultiValued )( 
            __RPC__in IADsProperty * This,
            /* [in] */ VARIANT_BOOL fMultiValued);
        
        DECLSPEC_XFGVIRT(IADsProperty, Qualifiers)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Qualifiers )( 
            __RPC__in IADsProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppQualifiers);
        
        END_INTERFACE
    } IADsPropertyVtbl;

    interface IADsProperty
    {
        CONST_VTBL struct IADsPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsProperty_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsProperty_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsProperty_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsProperty_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsProperty_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsProperty_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsProperty_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsProperty_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsProperty_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsProperty_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsProperty_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsProperty_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsProperty_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsProperty_get_OID(This,retval)	\
    ( (This)->lpVtbl -> get_OID(This,retval) ) 

#define IADsProperty_put_OID(This,bstrOID)	\
    ( (This)->lpVtbl -> put_OID(This,bstrOID) ) 

#define IADsProperty_get_Syntax(This,retval)	\
    ( (This)->lpVtbl -> get_Syntax(This,retval) ) 

#define IADsProperty_put_Syntax(This,bstrSyntax)	\
    ( (This)->lpVtbl -> put_Syntax(This,bstrSyntax) ) 

#define IADsProperty_get_MaxRange(This,retval)	\
    ( (This)->lpVtbl -> get_MaxRange(This,retval) ) 

#define IADsProperty_put_MaxRange(This,lnMaxRange)	\
    ( (This)->lpVtbl -> put_MaxRange(This,lnMaxRange) ) 

#define IADsProperty_get_MinRange(This,retval)	\
    ( (This)->lpVtbl -> get_MinRange(This,retval) ) 

#define IADsProperty_put_MinRange(This,lnMinRange)	\
    ( (This)->lpVtbl -> put_MinRange(This,lnMinRange) ) 

#define IADsProperty_get_MultiValued(This,retval)	\
    ( (This)->lpVtbl -> get_MultiValued(This,retval) ) 

#define IADsProperty_put_MultiValued(This,fMultiValued)	\
    ( (This)->lpVtbl -> put_MultiValued(This,fMultiValued) ) 

#define IADsProperty_Qualifiers(This,ppQualifiers)	\
    ( (This)->lpVtbl -> Qualifiers(This,ppQualifiers) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsProperty_INTERFACE_DEFINED__ */


#ifndef __IADsSyntax_INTERFACE_DEFINED__
#define __IADsSyntax_INTERFACE_DEFINED__

/* interface IADsSyntax */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsSyntax;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8f93dd2-4ae0-11cf-9e73-00aa004a5691")
    IADsSyntax : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OleAutoDataType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OleAutoDataType( 
            /* [in] */ long lnOleAutoDataType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsSyntaxVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsSyntax * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsSyntax * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsSyntax * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsSyntax * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsSyntax * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsSyntax * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsSyntax, get_OleAutoDataType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OleAutoDataType )( 
            __RPC__in IADsSyntax * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsSyntax, put_OleAutoDataType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OleAutoDataType )( 
            __RPC__in IADsSyntax * This,
            /* [in] */ long lnOleAutoDataType);
        
        END_INTERFACE
    } IADsSyntaxVtbl;

    interface IADsSyntax
    {
        CONST_VTBL struct IADsSyntaxVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsSyntax_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsSyntax_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsSyntax_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsSyntax_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsSyntax_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsSyntax_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsSyntax_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsSyntax_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsSyntax_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsSyntax_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsSyntax_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsSyntax_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsSyntax_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsSyntax_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsSyntax_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsSyntax_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsSyntax_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsSyntax_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsSyntax_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsSyntax_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsSyntax_get_OleAutoDataType(This,retval)	\
    ( (This)->lpVtbl -> get_OleAutoDataType(This,retval) ) 

#define IADsSyntax_put_OleAutoDataType(This,lnOleAutoDataType)	\
    ( (This)->lpVtbl -> put_OleAutoDataType(This,lnOleAutoDataType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsSyntax_INTERFACE_DEFINED__ */


#ifndef __IADsLocality_INTERFACE_DEFINED__
#define __IADsLocality_INTERFACE_DEFINED__

/* interface IADsLocality */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsLocality;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a05e03a2-effe-11cf-8abc-00c04fd8d503")
    IADsLocality : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalityName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalityName( 
            /* [in] */ __RPC__in BSTR bstrLocalityName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PostalAddress( 
            /* [in] */ __RPC__in BSTR bstrPostalAddress) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SeeAlso( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SeeAlso( 
            /* [in] */ VARIANT vSeeAlso) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsLocalityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsLocality * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsLocality * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsLocality * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsLocality * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsLocality * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsLocality * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsLocality * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsLocality * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsLocality * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsLocality, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsLocality, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsLocality, get_LocalityName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalityName )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsLocality, put_LocalityName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalityName )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in BSTR bstrLocalityName);
        
        DECLSPEC_XFGVIRT(IADsLocality, get_PostalAddress)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalAddress )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsLocality, put_PostalAddress)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostalAddress )( 
            __RPC__in IADsLocality * This,
            /* [in] */ __RPC__in BSTR bstrPostalAddress);
        
        DECLSPEC_XFGVIRT(IADsLocality, get_SeeAlso)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SeeAlso )( 
            __RPC__in IADsLocality * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsLocality, put_SeeAlso)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SeeAlso )( 
            __RPC__in IADsLocality * This,
            /* [in] */ VARIANT vSeeAlso);
        
        END_INTERFACE
    } IADsLocalityVtbl;

    interface IADsLocality
    {
        CONST_VTBL struct IADsLocalityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsLocality_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsLocality_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsLocality_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsLocality_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsLocality_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsLocality_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsLocality_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsLocality_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsLocality_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsLocality_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsLocality_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsLocality_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsLocality_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsLocality_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsLocality_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsLocality_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsLocality_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsLocality_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsLocality_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsLocality_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsLocality_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsLocality_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsLocality_get_LocalityName(This,retval)	\
    ( (This)->lpVtbl -> get_LocalityName(This,retval) ) 

#define IADsLocality_put_LocalityName(This,bstrLocalityName)	\
    ( (This)->lpVtbl -> put_LocalityName(This,bstrLocalityName) ) 

#define IADsLocality_get_PostalAddress(This,retval)	\
    ( (This)->lpVtbl -> get_PostalAddress(This,retval) ) 

#define IADsLocality_put_PostalAddress(This,bstrPostalAddress)	\
    ( (This)->lpVtbl -> put_PostalAddress(This,bstrPostalAddress) ) 

#define IADsLocality_get_SeeAlso(This,retval)	\
    ( (This)->lpVtbl -> get_SeeAlso(This,retval) ) 

#define IADsLocality_put_SeeAlso(This,vSeeAlso)	\
    ( (This)->lpVtbl -> put_SeeAlso(This,vSeeAlso) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsLocality_INTERFACE_DEFINED__ */


#ifndef __IADsO_INTERFACE_DEFINED__
#define __IADsO_INTERFACE_DEFINED__

/* interface IADsO */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a1cd2dc6-effe-11cf-8abc-00c04fd8d503")
    IADsO : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalityName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalityName( 
            /* [in] */ __RPC__in BSTR bstrLocalityName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PostalAddress( 
            /* [in] */ __RPC__in BSTR bstrPostalAddress) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephoneNumber( 
            /* [in] */ __RPC__in BSTR bstrTelephoneNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_FaxNumber( 
            /* [in] */ __RPC__in BSTR bstrFaxNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SeeAlso( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SeeAlso( 
            /* [in] */ VARIANT vSeeAlso) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsO * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsO * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsO * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsO * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsO * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsO * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsO * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsO * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsO, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsO, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsO, get_LocalityName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalityName )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsO, put_LocalityName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalityName )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrLocalityName);
        
        DECLSPEC_XFGVIRT(IADsO, get_PostalAddress)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalAddress )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsO, put_PostalAddress)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostalAddress )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrPostalAddress);
        
        DECLSPEC_XFGVIRT(IADsO, get_TelephoneNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneNumber )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsO, put_TelephoneNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephoneNumber )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrTelephoneNumber);
        
        DECLSPEC_XFGVIRT(IADsO, get_FaxNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsO, put_FaxNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FaxNumber )( 
            __RPC__in IADsO * This,
            /* [in] */ __RPC__in BSTR bstrFaxNumber);
        
        DECLSPEC_XFGVIRT(IADsO, get_SeeAlso)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SeeAlso )( 
            __RPC__in IADsO * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsO, put_SeeAlso)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SeeAlso )( 
            __RPC__in IADsO * This,
            /* [in] */ VARIANT vSeeAlso);
        
        END_INTERFACE
    } IADsOVtbl;

    interface IADsO
    {
        CONST_VTBL struct IADsOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsO_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsO_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsO_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsO_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsO_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsO_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsO_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsO_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsO_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsO_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsO_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsO_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsO_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsO_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsO_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsO_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsO_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsO_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsO_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsO_get_LocalityName(This,retval)	\
    ( (This)->lpVtbl -> get_LocalityName(This,retval) ) 

#define IADsO_put_LocalityName(This,bstrLocalityName)	\
    ( (This)->lpVtbl -> put_LocalityName(This,bstrLocalityName) ) 

#define IADsO_get_PostalAddress(This,retval)	\
    ( (This)->lpVtbl -> get_PostalAddress(This,retval) ) 

#define IADsO_put_PostalAddress(This,bstrPostalAddress)	\
    ( (This)->lpVtbl -> put_PostalAddress(This,bstrPostalAddress) ) 

#define IADsO_get_TelephoneNumber(This,retval)	\
    ( (This)->lpVtbl -> get_TelephoneNumber(This,retval) ) 

#define IADsO_put_TelephoneNumber(This,bstrTelephoneNumber)	\
    ( (This)->lpVtbl -> put_TelephoneNumber(This,bstrTelephoneNumber) ) 

#define IADsO_get_FaxNumber(This,retval)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,retval) ) 

#define IADsO_put_FaxNumber(This,bstrFaxNumber)	\
    ( (This)->lpVtbl -> put_FaxNumber(This,bstrFaxNumber) ) 

#define IADsO_get_SeeAlso(This,retval)	\
    ( (This)->lpVtbl -> get_SeeAlso(This,retval) ) 

#define IADsO_put_SeeAlso(This,vSeeAlso)	\
    ( (This)->lpVtbl -> put_SeeAlso(This,vSeeAlso) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsO_INTERFACE_DEFINED__ */


#ifndef __IADsOU_INTERFACE_DEFINED__
#define __IADsOU_INTERFACE_DEFINED__

/* interface IADsOU */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsOU;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a2f733b8-effe-11cf-8abc-00c04fd8d503")
    IADsOU : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalityName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalityName( 
            /* [in] */ __RPC__in BSTR bstrLocalityName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PostalAddress( 
            /* [in] */ __RPC__in BSTR bstrPostalAddress) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephoneNumber( 
            /* [in] */ __RPC__in BSTR bstrTelephoneNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_FaxNumber( 
            /* [in] */ __RPC__in BSTR bstrFaxNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SeeAlso( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SeeAlso( 
            /* [in] */ VARIANT vSeeAlso) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BusinessCategory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_BusinessCategory( 
            /* [in] */ __RPC__in BSTR bstrBusinessCategory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsOUVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsOU * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsOU * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsOU * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsOU * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsOU * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsOU * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsOU * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsOU * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsOU * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsOU, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsOU, get_LocalityName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalityName )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_LocalityName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalityName )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrLocalityName);
        
        DECLSPEC_XFGVIRT(IADsOU, get_PostalAddress)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalAddress )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_PostalAddress)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostalAddress )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrPostalAddress);
        
        DECLSPEC_XFGVIRT(IADsOU, get_TelephoneNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneNumber )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_TelephoneNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephoneNumber )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrTelephoneNumber);
        
        DECLSPEC_XFGVIRT(IADsOU, get_FaxNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_FaxNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FaxNumber )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrFaxNumber);
        
        DECLSPEC_XFGVIRT(IADsOU, get_SeeAlso)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SeeAlso )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_SeeAlso)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SeeAlso )( 
            __RPC__in IADsOU * This,
            /* [in] */ VARIANT vSeeAlso);
        
        DECLSPEC_XFGVIRT(IADsOU, get_BusinessCategory)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BusinessCategory )( 
            __RPC__in IADsOU * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsOU, put_BusinessCategory)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BusinessCategory )( 
            __RPC__in IADsOU * This,
            /* [in] */ __RPC__in BSTR bstrBusinessCategory);
        
        END_INTERFACE
    } IADsOUVtbl;

    interface IADsOU
    {
        CONST_VTBL struct IADsOUVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsOU_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsOU_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsOU_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsOU_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsOU_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsOU_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsOU_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsOU_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsOU_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsOU_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsOU_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsOU_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsOU_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsOU_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsOU_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsOU_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsOU_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsOU_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsOU_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsOU_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsOU_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsOU_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsOU_get_LocalityName(This,retval)	\
    ( (This)->lpVtbl -> get_LocalityName(This,retval) ) 

#define IADsOU_put_LocalityName(This,bstrLocalityName)	\
    ( (This)->lpVtbl -> put_LocalityName(This,bstrLocalityName) ) 

#define IADsOU_get_PostalAddress(This,retval)	\
    ( (This)->lpVtbl -> get_PostalAddress(This,retval) ) 

#define IADsOU_put_PostalAddress(This,bstrPostalAddress)	\
    ( (This)->lpVtbl -> put_PostalAddress(This,bstrPostalAddress) ) 

#define IADsOU_get_TelephoneNumber(This,retval)	\
    ( (This)->lpVtbl -> get_TelephoneNumber(This,retval) ) 

#define IADsOU_put_TelephoneNumber(This,bstrTelephoneNumber)	\
    ( (This)->lpVtbl -> put_TelephoneNumber(This,bstrTelephoneNumber) ) 

#define IADsOU_get_FaxNumber(This,retval)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,retval) ) 

#define IADsOU_put_FaxNumber(This,bstrFaxNumber)	\
    ( (This)->lpVtbl -> put_FaxNumber(This,bstrFaxNumber) ) 

#define IADsOU_get_SeeAlso(This,retval)	\
    ( (This)->lpVtbl -> get_SeeAlso(This,retval) ) 

#define IADsOU_put_SeeAlso(This,vSeeAlso)	\
    ( (This)->lpVtbl -> put_SeeAlso(This,vSeeAlso) ) 

#define IADsOU_get_BusinessCategory(This,retval)	\
    ( (This)->lpVtbl -> get_BusinessCategory(This,retval) ) 

#define IADsOU_put_BusinessCategory(This,bstrBusinessCategory)	\
    ( (This)->lpVtbl -> put_BusinessCategory(This,bstrBusinessCategory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsOU_INTERFACE_DEFINED__ */


#ifndef __IADsDomain_INTERFACE_DEFINED__
#define __IADsDomain_INTERFACE_DEFINED__

/* interface IADsDomain */
/* [object][oleautomation][dual][uuid] */ 


EXTERN_C const IID IID_IADsDomain;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00e4c220-fd16-11ce-abc4-02608c9e7553")
    IADsDomain : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_IsWorkgroup( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MinPasswordLength( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MinPasswordLength( 
            /* [in] */ long lnMinPasswordLength) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MinPasswordAge( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MinPasswordAge( 
            /* [in] */ long lnMinPasswordAge) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxPasswordAge( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxPasswordAge( 
            /* [in] */ long lnMaxPasswordAge) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxBadPasswordsAllowed( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxBadPasswordsAllowed( 
            /* [in] */ long lnMaxBadPasswordsAllowed) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PasswordHistoryLength( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PasswordHistoryLength( 
            /* [in] */ long lnPasswordHistoryLength) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PasswordAttributes( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PasswordAttributes( 
            /* [in] */ long lnPasswordAttributes) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoUnlockInterval( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoUnlockInterval( 
            /* [in] */ long lnAutoUnlockInterval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LockoutObservationInterval( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LockoutObservationInterval( 
            /* [in] */ long lnLockoutObservationInterval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsDomainVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsDomain * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsDomain * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsDomain * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsDomain * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsDomain * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsDomain * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsDomain * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsDomain * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsDomain * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsDomain * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsDomain * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsDomain * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsDomain * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_IsWorkgroup)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsWorkgroup )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_MinPasswordLength)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinPasswordLength )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_MinPasswordLength)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinPasswordLength )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnMinPasswordLength);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_MinPasswordAge)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinPasswordAge )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_MinPasswordAge)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinPasswordAge )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnMinPasswordAge);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_MaxPasswordAge)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxPasswordAge )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_MaxPasswordAge)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxPasswordAge )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnMaxPasswordAge);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_MaxBadPasswordsAllowed)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxBadPasswordsAllowed )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_MaxBadPasswordsAllowed)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxBadPasswordsAllowed )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnMaxBadPasswordsAllowed);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_PasswordHistoryLength)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PasswordHistoryLength )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_PasswordHistoryLength)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PasswordHistoryLength )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnPasswordHistoryLength);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_PasswordAttributes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PasswordAttributes )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_PasswordAttributes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PasswordAttributes )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnPasswordAttributes);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_AutoUnlockInterval)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoUnlockInterval )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_AutoUnlockInterval)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoUnlockInterval )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnAutoUnlockInterval);
        
        DECLSPEC_XFGVIRT(IADsDomain, get_LockoutObservationInterval)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LockoutObservationInterval )( 
            __RPC__in IADsDomain * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsDomain, put_LockoutObservationInterval)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LockoutObservationInterval )( 
            __RPC__in IADsDomain * This,
            /* [in] */ long lnLockoutObservationInterval);
        
        END_INTERFACE
    } IADsDomainVtbl;

    interface IADsDomain
    {
        CONST_VTBL struct IADsDomainVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsDomain_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsDomain_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsDomain_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsDomain_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsDomain_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsDomain_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsDomain_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsDomain_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsDomain_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsDomain_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsDomain_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsDomain_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsDomain_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsDomain_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsDomain_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsDomain_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsDomain_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsDomain_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsDomain_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsDomain_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsDomain_get_IsWorkgroup(This,retval)	\
    ( (This)->lpVtbl -> get_IsWorkgroup(This,retval) ) 

#define IADsDomain_get_MinPasswordLength(This,retval)	\
    ( (This)->lpVtbl -> get_MinPasswordLength(This,retval) ) 

#define IADsDomain_put_MinPasswordLength(This,lnMinPasswordLength)	\
    ( (This)->lpVtbl -> put_MinPasswordLength(This,lnMinPasswordLength) ) 

#define IADsDomain_get_MinPasswordAge(This,retval)	\
    ( (This)->lpVtbl -> get_MinPasswordAge(This,retval) ) 

#define IADsDomain_put_MinPasswordAge(This,lnMinPasswordAge)	\
    ( (This)->lpVtbl -> put_MinPasswordAge(This,lnMinPasswordAge) ) 

#define IADsDomain_get_MaxPasswordAge(This,retval)	\
    ( (This)->lpVtbl -> get_MaxPasswordAge(This,retval) ) 

#define IADsDomain_put_MaxPasswordAge(This,lnMaxPasswordAge)	\
    ( (This)->lpVtbl -> put_MaxPasswordAge(This,lnMaxPasswordAge) ) 

#define IADsDomain_get_MaxBadPasswordsAllowed(This,retval)	\
    ( (This)->lpVtbl -> get_MaxBadPasswordsAllowed(This,retval) ) 

#define IADsDomain_put_MaxBadPasswordsAllowed(This,lnMaxBadPasswordsAllowed)	\
    ( (This)->lpVtbl -> put_MaxBadPasswordsAllowed(This,lnMaxBadPasswordsAllowed) ) 

#define IADsDomain_get_PasswordHistoryLength(This,retval)	\
    ( (This)->lpVtbl -> get_PasswordHistoryLength(This,retval) ) 

#define IADsDomain_put_PasswordHistoryLength(This,lnPasswordHistoryLength)	\
    ( (This)->lpVtbl -> put_PasswordHistoryLength(This,lnPasswordHistoryLength) ) 

#define IADsDomain_get_PasswordAttributes(This,retval)	\
    ( (This)->lpVtbl -> get_PasswordAttributes(This,retval) ) 

#define IADsDomain_put_PasswordAttributes(This,lnPasswordAttributes)	\
    ( (This)->lpVtbl -> put_PasswordAttributes(This,lnPasswordAttributes) ) 

#define IADsDomain_get_AutoUnlockInterval(This,retval)	\
    ( (This)->lpVtbl -> get_AutoUnlockInterval(This,retval) ) 

#define IADsDomain_put_AutoUnlockInterval(This,lnAutoUnlockInterval)	\
    ( (This)->lpVtbl -> put_AutoUnlockInterval(This,lnAutoUnlockInterval) ) 

#define IADsDomain_get_LockoutObservationInterval(This,retval)	\
    ( (This)->lpVtbl -> get_LockoutObservationInterval(This,retval) ) 

#define IADsDomain_put_LockoutObservationInterval(This,lnLockoutObservationInterval)	\
    ( (This)->lpVtbl -> put_LockoutObservationInterval(This,lnLockoutObservationInterval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsDomain_INTERFACE_DEFINED__ */


#ifndef __IADsComputer_INTERFACE_DEFINED__
#define __IADsComputer_INTERFACE_DEFINED__

/* interface IADsComputer */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsComputer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("efe3cc70-1d9f-11cf-b1f3-02608c9e7553")
    IADsComputer : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Site( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Location( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Location( 
            /* [in] */ __RPC__in BSTR bstrLocation) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrimaryUser( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PrimaryUser( 
            /* [in] */ __RPC__in BSTR bstrPrimaryUser) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Owner( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Owner( 
            /* [in] */ __RPC__in BSTR bstrOwner) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Division( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Division( 
            /* [in] */ __RPC__in BSTR bstrDivision) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Department( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Department( 
            /* [in] */ __RPC__in BSTR bstrDepartment) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Role( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Role( 
            /* [in] */ __RPC__in BSTR bstrRole) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OperatingSystem( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OperatingSystem( 
            /* [in] */ __RPC__in BSTR bstrOperatingSystem) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OperatingSystemVersion( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OperatingSystemVersion( 
            /* [in] */ __RPC__in BSTR bstrOperatingSystemVersion) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Model( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Model( 
            /* [in] */ __RPC__in BSTR bstrModel) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Processor( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Processor( 
            /* [in] */ __RPC__in BSTR bstrProcessor) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ProcessorCount( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ProcessorCount( 
            /* [in] */ __RPC__in BSTR bstrProcessorCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MemorySize( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MemorySize( 
            /* [in] */ __RPC__in BSTR bstrMemorySize) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StorageCapacity( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_StorageCapacity( 
            /* [in] */ __RPC__in BSTR bstrStorageCapacity) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NetAddresses( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NetAddresses( 
            /* [in] */ VARIANT vNetAddresses) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsComputerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsComputer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsComputer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsComputer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsComputer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsComputer * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsComputer * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsComputer * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsComputer * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsComputer * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_ComputerID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerID )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Site)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Site )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Location)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Location )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Location)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Location )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrLocation);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_PrimaryUser)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrimaryUser )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_PrimaryUser)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrimaryUser )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrPrimaryUser);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Owner)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Owner )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Owner)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Owner )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrOwner);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Division)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Division )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Division)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Division )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrDivision);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Department)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Department )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Department)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Department )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrDepartment);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Role)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Role )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Role)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Role )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrRole);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_OperatingSystem)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OperatingSystem )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_OperatingSystem)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OperatingSystem )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrOperatingSystem);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_OperatingSystemVersion)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OperatingSystemVersion )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_OperatingSystemVersion)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OperatingSystemVersion )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrOperatingSystemVersion);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Model)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Model )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Model)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Model )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrModel);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_Processor)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Processor )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_Processor)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Processor )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrProcessor);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_ProcessorCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessorCount )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_ProcessorCount)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProcessorCount )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrProcessorCount);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_MemorySize)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MemorySize )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_MemorySize)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MemorySize )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrMemorySize);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_StorageCapacity)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StorageCapacity )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_StorageCapacity)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StorageCapacity )( 
            __RPC__in IADsComputer * This,
            /* [in] */ __RPC__in BSTR bstrStorageCapacity);
        
        DECLSPEC_XFGVIRT(IADsComputer, get_NetAddresses)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetAddresses )( 
            __RPC__in IADsComputer * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsComputer, put_NetAddresses)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetAddresses )( 
            __RPC__in IADsComputer * This,
            /* [in] */ VARIANT vNetAddresses);
        
        END_INTERFACE
    } IADsComputerVtbl;

    interface IADsComputer
    {
        CONST_VTBL struct IADsComputerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsComputer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsComputer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsComputer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsComputer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsComputer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsComputer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsComputer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsComputer_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsComputer_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsComputer_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsComputer_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsComputer_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsComputer_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsComputer_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsComputer_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsComputer_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsComputer_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsComputer_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsComputer_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsComputer_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsComputer_get_ComputerID(This,retval)	\
    ( (This)->lpVtbl -> get_ComputerID(This,retval) ) 

#define IADsComputer_get_Site(This,retval)	\
    ( (This)->lpVtbl -> get_Site(This,retval) ) 

#define IADsComputer_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsComputer_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsComputer_get_Location(This,retval)	\
    ( (This)->lpVtbl -> get_Location(This,retval) ) 

#define IADsComputer_put_Location(This,bstrLocation)	\
    ( (This)->lpVtbl -> put_Location(This,bstrLocation) ) 

#define IADsComputer_get_PrimaryUser(This,retval)	\
    ( (This)->lpVtbl -> get_PrimaryUser(This,retval) ) 

#define IADsComputer_put_PrimaryUser(This,bstrPrimaryUser)	\
    ( (This)->lpVtbl -> put_PrimaryUser(This,bstrPrimaryUser) ) 

#define IADsComputer_get_Owner(This,retval)	\
    ( (This)->lpVtbl -> get_Owner(This,retval) ) 

#define IADsComputer_put_Owner(This,bstrOwner)	\
    ( (This)->lpVtbl -> put_Owner(This,bstrOwner) ) 

#define IADsComputer_get_Division(This,retval)	\
    ( (This)->lpVtbl -> get_Division(This,retval) ) 

#define IADsComputer_put_Division(This,bstrDivision)	\
    ( (This)->lpVtbl -> put_Division(This,bstrDivision) ) 

#define IADsComputer_get_Department(This,retval)	\
    ( (This)->lpVtbl -> get_Department(This,retval) ) 

#define IADsComputer_put_Department(This,bstrDepartment)	\
    ( (This)->lpVtbl -> put_Department(This,bstrDepartment) ) 

#define IADsComputer_get_Role(This,retval)	\
    ( (This)->lpVtbl -> get_Role(This,retval) ) 

#define IADsComputer_put_Role(This,bstrRole)	\
    ( (This)->lpVtbl -> put_Role(This,bstrRole) ) 

#define IADsComputer_get_OperatingSystem(This,retval)	\
    ( (This)->lpVtbl -> get_OperatingSystem(This,retval) ) 

#define IADsComputer_put_OperatingSystem(This,bstrOperatingSystem)	\
    ( (This)->lpVtbl -> put_OperatingSystem(This,bstrOperatingSystem) ) 

#define IADsComputer_get_OperatingSystemVersion(This,retval)	\
    ( (This)->lpVtbl -> get_OperatingSystemVersion(This,retval) ) 

#define IADsComputer_put_OperatingSystemVersion(This,bstrOperatingSystemVersion)	\
    ( (This)->lpVtbl -> put_OperatingSystemVersion(This,bstrOperatingSystemVersion) ) 

#define IADsComputer_get_Model(This,retval)	\
    ( (This)->lpVtbl -> get_Model(This,retval) ) 

#define IADsComputer_put_Model(This,bstrModel)	\
    ( (This)->lpVtbl -> put_Model(This,bstrModel) ) 

#define IADsComputer_get_Processor(This,retval)	\
    ( (This)->lpVtbl -> get_Processor(This,retval) ) 

#define IADsComputer_put_Processor(This,bstrProcessor)	\
    ( (This)->lpVtbl -> put_Processor(This,bstrProcessor) ) 

#define IADsComputer_get_ProcessorCount(This,retval)	\
    ( (This)->lpVtbl -> get_ProcessorCount(This,retval) ) 

#define IADsComputer_put_ProcessorCount(This,bstrProcessorCount)	\
    ( (This)->lpVtbl -> put_ProcessorCount(This,bstrProcessorCount) ) 

#define IADsComputer_get_MemorySize(This,retval)	\
    ( (This)->lpVtbl -> get_MemorySize(This,retval) ) 

#define IADsComputer_put_MemorySize(This,bstrMemorySize)	\
    ( (This)->lpVtbl -> put_MemorySize(This,bstrMemorySize) ) 

#define IADsComputer_get_StorageCapacity(This,retval)	\
    ( (This)->lpVtbl -> get_StorageCapacity(This,retval) ) 

#define IADsComputer_put_StorageCapacity(This,bstrStorageCapacity)	\
    ( (This)->lpVtbl -> put_StorageCapacity(This,bstrStorageCapacity) ) 

#define IADsComputer_get_NetAddresses(This,retval)	\
    ( (This)->lpVtbl -> get_NetAddresses(This,retval) ) 

#define IADsComputer_put_NetAddresses(This,vNetAddresses)	\
    ( (This)->lpVtbl -> put_NetAddresses(This,vNetAddresses) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsComputer_INTERFACE_DEFINED__ */


#ifndef __IADsComputerOperations_INTERFACE_DEFINED__
#define __IADsComputerOperations_INTERFACE_DEFINED__

/* interface IADsComputerOperations */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsComputerOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ef497680-1d9f-11cf-b1f3-02608c9e7553")
    IADsComputerOperations : public IADs
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Status( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObject) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Shutdown( 
            /* [in] */ VARIANT_BOOL bReboot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsComputerOperationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsComputerOperations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsComputerOperations * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsComputerOperations * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsComputerOperations * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsComputerOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsComputerOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsComputerOperations, Status)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Status )( 
            __RPC__in IADsComputerOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppObject);
        
        DECLSPEC_XFGVIRT(IADsComputerOperations, Shutdown)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IADsComputerOperations * This,
            /* [in] */ VARIANT_BOOL bReboot);
        
        END_INTERFACE
    } IADsComputerOperationsVtbl;

    interface IADsComputerOperations
    {
        CONST_VTBL struct IADsComputerOperationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsComputerOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsComputerOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsComputerOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsComputerOperations_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsComputerOperations_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsComputerOperations_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsComputerOperations_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsComputerOperations_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsComputerOperations_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsComputerOperations_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsComputerOperations_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsComputerOperations_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsComputerOperations_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsComputerOperations_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsComputerOperations_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsComputerOperations_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsComputerOperations_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsComputerOperations_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsComputerOperations_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsComputerOperations_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsComputerOperations_Status(This,ppObject)	\
    ( (This)->lpVtbl -> Status(This,ppObject) ) 

#define IADsComputerOperations_Shutdown(This,bReboot)	\
    ( (This)->lpVtbl -> Shutdown(This,bReboot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsComputerOperations_INTERFACE_DEFINED__ */


#ifndef __IADsGroup_INTERFACE_DEFINED__
#define __IADsGroup_INTERFACE_DEFINED__

/* interface IADsGroup */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27636b00-410f-11cf-b1ff-02608c9e7553")
    IADsGroup : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Members( 
            /* [retval][out] */ __RPC__deref_out_opt IADsMembers **ppMembers) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE IsMember( 
            /* [in] */ __RPC__in BSTR bstrMember,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *bMember) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrNewItem) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR bstrItemToBeRemoved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsGroup * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsGroup * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsGroup * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsGroup * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsGroup * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsGroup, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsGroup, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsGroup, Members)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Members )( 
            __RPC__in IADsGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsMembers **ppMembers);
        
        DECLSPEC_XFGVIRT(IADsGroup, IsMember)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *IsMember )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrMember,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *bMember);
        
        DECLSPEC_XFGVIRT(IADsGroup, Add)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrNewItem);
        
        DECLSPEC_XFGVIRT(IADsGroup, Remove)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IADsGroup * This,
            /* [in] */ __RPC__in BSTR bstrItemToBeRemoved);
        
        END_INTERFACE
    } IADsGroupVtbl;

    interface IADsGroup
    {
        CONST_VTBL struct IADsGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsGroup_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsGroup_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsGroup_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsGroup_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsGroup_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsGroup_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsGroup_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsGroup_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsGroup_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsGroup_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsGroup_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsGroup_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsGroup_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsGroup_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsGroup_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsGroup_Members(This,ppMembers)	\
    ( (This)->lpVtbl -> Members(This,ppMembers) ) 

#define IADsGroup_IsMember(This,bstrMember,bMember)	\
    ( (This)->lpVtbl -> IsMember(This,bstrMember,bMember) ) 

#define IADsGroup_Add(This,bstrNewItem)	\
    ( (This)->lpVtbl -> Add(This,bstrNewItem) ) 

#define IADsGroup_Remove(This,bstrItemToBeRemoved)	\
    ( (This)->lpVtbl -> Remove(This,bstrItemToBeRemoved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsGroup_INTERFACE_DEFINED__ */


#ifndef __IADsUser_INTERFACE_DEFINED__
#define __IADsUser_INTERFACE_DEFINED__

/* interface IADsUser */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsUser;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3e37e320-17e2-11cf-abc4-02608c9e7553")
    IADsUser : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BadLoginAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BadLoginCount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LastLogin( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LastLogoff( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LastFailedLogin( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PasswordLastChanged( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Division( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Division( 
            /* [in] */ __RPC__in BSTR bstrDivision) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Department( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Department( 
            /* [in] */ __RPC__in BSTR bstrDepartment) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_EmployeeID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_EmployeeID( 
            /* [in] */ __RPC__in BSTR bstrEmployeeID) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FullName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_FullName( 
            /* [in] */ __RPC__in BSTR bstrFullName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FirstName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_FirstName( 
            /* [in] */ __RPC__in BSTR bstrFirstName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LastName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LastName( 
            /* [in] */ __RPC__in BSTR bstrLastName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OtherName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OtherName( 
            /* [in] */ __RPC__in BSTR bstrOtherName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NamePrefix( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NamePrefix( 
            /* [in] */ __RPC__in BSTR bstrNamePrefix) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NameSuffix( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NameSuffix( 
            /* [in] */ __RPC__in BSTR bstrNameSuffix) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Title( 
            /* [in] */ __RPC__in BSTR bstrTitle) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Manager( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Manager( 
            /* [in] */ __RPC__in BSTR bstrManager) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneHome( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephoneHome( 
            /* [in] */ VARIANT vTelephoneHome) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneMobile( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephoneMobile( 
            /* [in] */ VARIANT vTelephoneMobile) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneNumber( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephoneNumber( 
            /* [in] */ VARIANT vTelephoneNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephonePager( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephonePager( 
            /* [in] */ VARIANT vTelephonePager) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_FaxNumber( 
            /* [in] */ VARIANT vFaxNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OfficeLocations( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OfficeLocations( 
            /* [in] */ VARIANT vOfficeLocations) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalAddresses( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PostalAddresses( 
            /* [in] */ VARIANT vPostalAddresses) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalCodes( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PostalCodes( 
            /* [in] */ VARIANT vPostalCodes) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SeeAlso( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SeeAlso( 
            /* [in] */ VARIANT vSeeAlso) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AccountDisabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AccountDisabled( 
            /* [in] */ VARIANT_BOOL fAccountDisabled) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AccountExpirationDate( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AccountExpirationDate( 
            /* [in] */ DATE daAccountExpirationDate) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_GraceLoginsAllowed( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_GraceLoginsAllowed( 
            /* [in] */ long lnGraceLoginsAllowed) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_GraceLoginsRemaining( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_GraceLoginsRemaining( 
            /* [in] */ long lnGraceLoginsRemaining) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_IsAccountLocked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_IsAccountLocked( 
            /* [in] */ VARIANT_BOOL fIsAccountLocked) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LoginHours( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LoginHours( 
            /* [in] */ VARIANT vLoginHours) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LoginWorkstations( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LoginWorkstations( 
            /* [in] */ VARIANT vLoginWorkstations) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxLogins( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxLogins( 
            /* [in] */ long lnMaxLogins) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxStorage( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxStorage( 
            /* [in] */ long lnMaxStorage) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PasswordExpirationDate( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PasswordExpirationDate( 
            /* [in] */ DATE daPasswordExpirationDate) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PasswordMinimumLength( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PasswordMinimumLength( 
            /* [in] */ long lnPasswordMinimumLength) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PasswordRequired( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PasswordRequired( 
            /* [in] */ VARIANT_BOOL fPasswordRequired) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_RequireUniquePassword( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_RequireUniquePassword( 
            /* [in] */ VARIANT_BOOL fRequireUniquePassword) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_EmailAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_EmailAddress( 
            /* [in] */ __RPC__in BSTR bstrEmailAddress) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HomeDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HomeDirectory( 
            /* [in] */ __RPC__in BSTR bstrHomeDirectory) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Languages( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Languages( 
            /* [in] */ VARIANT vLanguages) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Profile( 
            /* [in] */ __RPC__in BSTR bstrProfile) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LoginScript( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LoginScript( 
            /* [in] */ __RPC__in BSTR bstrLoginScript) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Picture( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Picture( 
            /* [in] */ VARIANT vPicture) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HomePage( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HomePage( 
            /* [in] */ __RPC__in BSTR bstrHomePage) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Groups( 
            /* [retval][out] */ __RPC__deref_out_opt IADsMembers **ppGroups) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetPassword( 
            /* [in] */ __RPC__in BSTR NewPassword) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ChangePassword( 
            /* [in] */ __RPC__in BSTR bstrOldPassword,
            /* [in] */ __RPC__in BSTR bstrNewPassword) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsUserVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsUser * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsUser * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsUser * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsUser * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsUser * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsUser * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsUser * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsUser * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsUser, get_BadLoginAddress)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BadLoginAddress )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, get_BadLoginCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BadLoginCount )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LastLogin)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastLogin )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LastLogoff)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastLogoff )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LastFailedLogin)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastFailedLogin )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, get_PasswordLastChanged)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PasswordLastChanged )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Division)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Division )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Division)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Division )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrDivision);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Department)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Department )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Department)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Department )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrDepartment);
        
        DECLSPEC_XFGVIRT(IADsUser, get_EmployeeID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EmployeeID )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_EmployeeID)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EmployeeID )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrEmployeeID);
        
        DECLSPEC_XFGVIRT(IADsUser, get_FullName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FullName )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_FullName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FullName )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrFullName);
        
        DECLSPEC_XFGVIRT(IADsUser, get_FirstName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirstName )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_FirstName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FirstName )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrFirstName);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LastName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastName )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_LastName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LastName )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrLastName);
        
        DECLSPEC_XFGVIRT(IADsUser, get_OtherName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OtherName )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_OtherName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OtherName )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrOtherName);
        
        DECLSPEC_XFGVIRT(IADsUser, get_NamePrefix)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamePrefix )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_NamePrefix)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NamePrefix )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrNamePrefix);
        
        DECLSPEC_XFGVIRT(IADsUser, get_NameSuffix)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NameSuffix )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_NameSuffix)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NameSuffix )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrNameSuffix);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Title)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Title)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Title )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrTitle);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Manager)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Manager )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Manager)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Manager )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrManager);
        
        DECLSPEC_XFGVIRT(IADsUser, get_TelephoneHome)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneHome )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_TelephoneHome)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephoneHome )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vTelephoneHome);
        
        DECLSPEC_XFGVIRT(IADsUser, get_TelephoneMobile)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneMobile )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_TelephoneMobile)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephoneMobile )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vTelephoneMobile);
        
        DECLSPEC_XFGVIRT(IADsUser, get_TelephoneNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneNumber )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_TelephoneNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephoneNumber )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vTelephoneNumber);
        
        DECLSPEC_XFGVIRT(IADsUser, get_TelephonePager)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephonePager )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_TelephonePager)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephonePager )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vTelephonePager);
        
        DECLSPEC_XFGVIRT(IADsUser, get_FaxNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_FaxNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FaxNumber )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vFaxNumber);
        
        DECLSPEC_XFGVIRT(IADsUser, get_OfficeLocations)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfficeLocations )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_OfficeLocations)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OfficeLocations )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vOfficeLocations);
        
        DECLSPEC_XFGVIRT(IADsUser, get_PostalAddresses)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalAddresses )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_PostalAddresses)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostalAddresses )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vPostalAddresses);
        
        DECLSPEC_XFGVIRT(IADsUser, get_PostalCodes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalCodes )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_PostalCodes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostalCodes )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vPostalCodes);
        
        DECLSPEC_XFGVIRT(IADsUser, get_SeeAlso)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SeeAlso )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_SeeAlso)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SeeAlso )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vSeeAlso);
        
        DECLSPEC_XFGVIRT(IADsUser, get_AccountDisabled)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AccountDisabled )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_AccountDisabled)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AccountDisabled )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT_BOOL fAccountDisabled);
        
        DECLSPEC_XFGVIRT(IADsUser, get_AccountExpirationDate)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AccountExpirationDate )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_AccountExpirationDate)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AccountExpirationDate )( 
            __RPC__in IADsUser * This,
            /* [in] */ DATE daAccountExpirationDate);
        
        DECLSPEC_XFGVIRT(IADsUser, get_GraceLoginsAllowed)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GraceLoginsAllowed )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_GraceLoginsAllowed)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GraceLoginsAllowed )( 
            __RPC__in IADsUser * This,
            /* [in] */ long lnGraceLoginsAllowed);
        
        DECLSPEC_XFGVIRT(IADsUser, get_GraceLoginsRemaining)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GraceLoginsRemaining )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_GraceLoginsRemaining)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GraceLoginsRemaining )( 
            __RPC__in IADsUser * This,
            /* [in] */ long lnGraceLoginsRemaining);
        
        DECLSPEC_XFGVIRT(IADsUser, get_IsAccountLocked)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsAccountLocked )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_IsAccountLocked)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IsAccountLocked )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT_BOOL fIsAccountLocked);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LoginHours)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoginHours )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_LoginHours)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoginHours )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vLoginHours);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LoginWorkstations)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoginWorkstations )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_LoginWorkstations)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoginWorkstations )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vLoginWorkstations);
        
        DECLSPEC_XFGVIRT(IADsUser, get_MaxLogins)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxLogins )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_MaxLogins)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxLogins )( 
            __RPC__in IADsUser * This,
            /* [in] */ long lnMaxLogins);
        
        DECLSPEC_XFGVIRT(IADsUser, get_MaxStorage)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxStorage )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_MaxStorage)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxStorage )( 
            __RPC__in IADsUser * This,
            /* [in] */ long lnMaxStorage);
        
        DECLSPEC_XFGVIRT(IADsUser, get_PasswordExpirationDate)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PasswordExpirationDate )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_PasswordExpirationDate)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PasswordExpirationDate )( 
            __RPC__in IADsUser * This,
            /* [in] */ DATE daPasswordExpirationDate);
        
        DECLSPEC_XFGVIRT(IADsUser, get_PasswordMinimumLength)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PasswordMinimumLength )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_PasswordMinimumLength)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PasswordMinimumLength )( 
            __RPC__in IADsUser * This,
            /* [in] */ long lnPasswordMinimumLength);
        
        DECLSPEC_XFGVIRT(IADsUser, get_PasswordRequired)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PasswordRequired )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_PasswordRequired)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PasswordRequired )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT_BOOL fPasswordRequired);
        
        DECLSPEC_XFGVIRT(IADsUser, get_RequireUniquePassword)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequireUniquePassword )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_RequireUniquePassword)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RequireUniquePassword )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT_BOOL fRequireUniquePassword);
        
        DECLSPEC_XFGVIRT(IADsUser, get_EmailAddress)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EmailAddress )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_EmailAddress)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EmailAddress )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrEmailAddress);
        
        DECLSPEC_XFGVIRT(IADsUser, get_HomeDirectory)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HomeDirectory )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_HomeDirectory)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HomeDirectory )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrHomeDirectory);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Languages)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Languages )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Languages)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Languages )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vLanguages);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Profile)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Profile)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Profile )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrProfile);
        
        DECLSPEC_XFGVIRT(IADsUser, get_LoginScript)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoginScript )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_LoginScript)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoginScript )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrLoginScript);
        
        DECLSPEC_XFGVIRT(IADsUser, get_Picture)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Picture )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_Picture)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Picture )( 
            __RPC__in IADsUser * This,
            /* [in] */ VARIANT vPicture);
        
        DECLSPEC_XFGVIRT(IADsUser, get_HomePage)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HomePage )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsUser, put_HomePage)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HomePage )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrHomePage);
        
        DECLSPEC_XFGVIRT(IADsUser, Groups)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Groups )( 
            __RPC__in IADsUser * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsMembers **ppGroups);
        
        DECLSPEC_XFGVIRT(IADsUser, SetPassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPassword )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR NewPassword);
        
        DECLSPEC_XFGVIRT(IADsUser, ChangePassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ChangePassword )( 
            __RPC__in IADsUser * This,
            /* [in] */ __RPC__in BSTR bstrOldPassword,
            /* [in] */ __RPC__in BSTR bstrNewPassword);
        
        END_INTERFACE
    } IADsUserVtbl;

    interface IADsUser
    {
        CONST_VTBL struct IADsUserVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsUser_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsUser_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsUser_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsUser_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsUser_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsUser_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsUser_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsUser_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsUser_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsUser_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsUser_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsUser_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsUser_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsUser_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsUser_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsUser_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsUser_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsUser_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsUser_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsUser_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsUser_get_BadLoginAddress(This,retval)	\
    ( (This)->lpVtbl -> get_BadLoginAddress(This,retval) ) 

#define IADsUser_get_BadLoginCount(This,retval)	\
    ( (This)->lpVtbl -> get_BadLoginCount(This,retval) ) 

#define IADsUser_get_LastLogin(This,retval)	\
    ( (This)->lpVtbl -> get_LastLogin(This,retval) ) 

#define IADsUser_get_LastLogoff(This,retval)	\
    ( (This)->lpVtbl -> get_LastLogoff(This,retval) ) 

#define IADsUser_get_LastFailedLogin(This,retval)	\
    ( (This)->lpVtbl -> get_LastFailedLogin(This,retval) ) 

#define IADsUser_get_PasswordLastChanged(This,retval)	\
    ( (This)->lpVtbl -> get_PasswordLastChanged(This,retval) ) 

#define IADsUser_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsUser_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsUser_get_Division(This,retval)	\
    ( (This)->lpVtbl -> get_Division(This,retval) ) 

#define IADsUser_put_Division(This,bstrDivision)	\
    ( (This)->lpVtbl -> put_Division(This,bstrDivision) ) 

#define IADsUser_get_Department(This,retval)	\
    ( (This)->lpVtbl -> get_Department(This,retval) ) 

#define IADsUser_put_Department(This,bstrDepartment)	\
    ( (This)->lpVtbl -> put_Department(This,bstrDepartment) ) 

#define IADsUser_get_EmployeeID(This,retval)	\
    ( (This)->lpVtbl -> get_EmployeeID(This,retval) ) 

#define IADsUser_put_EmployeeID(This,bstrEmployeeID)	\
    ( (This)->lpVtbl -> put_EmployeeID(This,bstrEmployeeID) ) 

#define IADsUser_get_FullName(This,retval)	\
    ( (This)->lpVtbl -> get_FullName(This,retval) ) 

#define IADsUser_put_FullName(This,bstrFullName)	\
    ( (This)->lpVtbl -> put_FullName(This,bstrFullName) ) 

#define IADsUser_get_FirstName(This,retval)	\
    ( (This)->lpVtbl -> get_FirstName(This,retval) ) 

#define IADsUser_put_FirstName(This,bstrFirstName)	\
    ( (This)->lpVtbl -> put_FirstName(This,bstrFirstName) ) 

#define IADsUser_get_LastName(This,retval)	\
    ( (This)->lpVtbl -> get_LastName(This,retval) ) 

#define IADsUser_put_LastName(This,bstrLastName)	\
    ( (This)->lpVtbl -> put_LastName(This,bstrLastName) ) 

#define IADsUser_get_OtherName(This,retval)	\
    ( (This)->lpVtbl -> get_OtherName(This,retval) ) 

#define IADsUser_put_OtherName(This,bstrOtherName)	\
    ( (This)->lpVtbl -> put_OtherName(This,bstrOtherName) ) 

#define IADsUser_get_NamePrefix(This,retval)	\
    ( (This)->lpVtbl -> get_NamePrefix(This,retval) ) 

#define IADsUser_put_NamePrefix(This,bstrNamePrefix)	\
    ( (This)->lpVtbl -> put_NamePrefix(This,bstrNamePrefix) ) 

#define IADsUser_get_NameSuffix(This,retval)	\
    ( (This)->lpVtbl -> get_NameSuffix(This,retval) ) 

#define IADsUser_put_NameSuffix(This,bstrNameSuffix)	\
    ( (This)->lpVtbl -> put_NameSuffix(This,bstrNameSuffix) ) 

#define IADsUser_get_Title(This,retval)	\
    ( (This)->lpVtbl -> get_Title(This,retval) ) 

#define IADsUser_put_Title(This,bstrTitle)	\
    ( (This)->lpVtbl -> put_Title(This,bstrTitle) ) 

#define IADsUser_get_Manager(This,retval)	\
    ( (This)->lpVtbl -> get_Manager(This,retval) ) 

#define IADsUser_put_Manager(This,bstrManager)	\
    ( (This)->lpVtbl -> put_Manager(This,bstrManager) ) 

#define IADsUser_get_TelephoneHome(This,retval)	\
    ( (This)->lpVtbl -> get_TelephoneHome(This,retval) ) 

#define IADsUser_put_TelephoneHome(This,vTelephoneHome)	\
    ( (This)->lpVtbl -> put_TelephoneHome(This,vTelephoneHome) ) 

#define IADsUser_get_TelephoneMobile(This,retval)	\
    ( (This)->lpVtbl -> get_TelephoneMobile(This,retval) ) 

#define IADsUser_put_TelephoneMobile(This,vTelephoneMobile)	\
    ( (This)->lpVtbl -> put_TelephoneMobile(This,vTelephoneMobile) ) 

#define IADsUser_get_TelephoneNumber(This,retval)	\
    ( (This)->lpVtbl -> get_TelephoneNumber(This,retval) ) 

#define IADsUser_put_TelephoneNumber(This,vTelephoneNumber)	\
    ( (This)->lpVtbl -> put_TelephoneNumber(This,vTelephoneNumber) ) 

#define IADsUser_get_TelephonePager(This,retval)	\
    ( (This)->lpVtbl -> get_TelephonePager(This,retval) ) 

#define IADsUser_put_TelephonePager(This,vTelephonePager)	\
    ( (This)->lpVtbl -> put_TelephonePager(This,vTelephonePager) ) 

#define IADsUser_get_FaxNumber(This,retval)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,retval) ) 

#define IADsUser_put_FaxNumber(This,vFaxNumber)	\
    ( (This)->lpVtbl -> put_FaxNumber(This,vFaxNumber) ) 

#define IADsUser_get_OfficeLocations(This,retval)	\
    ( (This)->lpVtbl -> get_OfficeLocations(This,retval) ) 

#define IADsUser_put_OfficeLocations(This,vOfficeLocations)	\
    ( (This)->lpVtbl -> put_OfficeLocations(This,vOfficeLocations) ) 

#define IADsUser_get_PostalAddresses(This,retval)	\
    ( (This)->lpVtbl -> get_PostalAddresses(This,retval) ) 

#define IADsUser_put_PostalAddresses(This,vPostalAddresses)	\
    ( (This)->lpVtbl -> put_PostalAddresses(This,vPostalAddresses) ) 

#define IADsUser_get_PostalCodes(This,retval)	\
    ( (This)->lpVtbl -> get_PostalCodes(This,retval) ) 

#define IADsUser_put_PostalCodes(This,vPostalCodes)	\
    ( (This)->lpVtbl -> put_PostalCodes(This,vPostalCodes) ) 

#define IADsUser_get_SeeAlso(This,retval)	\
    ( (This)->lpVtbl -> get_SeeAlso(This,retval) ) 

#define IADsUser_put_SeeAlso(This,vSeeAlso)	\
    ( (This)->lpVtbl -> put_SeeAlso(This,vSeeAlso) ) 

#define IADsUser_get_AccountDisabled(This,retval)	\
    ( (This)->lpVtbl -> get_AccountDisabled(This,retval) ) 

#define IADsUser_put_AccountDisabled(This,fAccountDisabled)	\
    ( (This)->lpVtbl -> put_AccountDisabled(This,fAccountDisabled) ) 

#define IADsUser_get_AccountExpirationDate(This,retval)	\
    ( (This)->lpVtbl -> get_AccountExpirationDate(This,retval) ) 

#define IADsUser_put_AccountExpirationDate(This,daAccountExpirationDate)	\
    ( (This)->lpVtbl -> put_AccountExpirationDate(This,daAccountExpirationDate) ) 

#define IADsUser_get_GraceLoginsAllowed(This,retval)	\
    ( (This)->lpVtbl -> get_GraceLoginsAllowed(This,retval) ) 

#define IADsUser_put_GraceLoginsAllowed(This,lnGraceLoginsAllowed)	\
    ( (This)->lpVtbl -> put_GraceLoginsAllowed(This,lnGraceLoginsAllowed) ) 

#define IADsUser_get_GraceLoginsRemaining(This,retval)	\
    ( (This)->lpVtbl -> get_GraceLoginsRemaining(This,retval) ) 

#define IADsUser_put_GraceLoginsRemaining(This,lnGraceLoginsRemaining)	\
    ( (This)->lpVtbl -> put_GraceLoginsRemaining(This,lnGraceLoginsRemaining) ) 

#define IADsUser_get_IsAccountLocked(This,retval)	\
    ( (This)->lpVtbl -> get_IsAccountLocked(This,retval) ) 

#define IADsUser_put_IsAccountLocked(This,fIsAccountLocked)	\
    ( (This)->lpVtbl -> put_IsAccountLocked(This,fIsAccountLocked) ) 

#define IADsUser_get_LoginHours(This,retval)	\
    ( (This)->lpVtbl -> get_LoginHours(This,retval) ) 

#define IADsUser_put_LoginHours(This,vLoginHours)	\
    ( (This)->lpVtbl -> put_LoginHours(This,vLoginHours) ) 

#define IADsUser_get_LoginWorkstations(This,retval)	\
    ( (This)->lpVtbl -> get_LoginWorkstations(This,retval) ) 

#define IADsUser_put_LoginWorkstations(This,vLoginWorkstations)	\
    ( (This)->lpVtbl -> put_LoginWorkstations(This,vLoginWorkstations) ) 

#define IADsUser_get_MaxLogins(This,retval)	\
    ( (This)->lpVtbl -> get_MaxLogins(This,retval) ) 

#define IADsUser_put_MaxLogins(This,lnMaxLogins)	\
    ( (This)->lpVtbl -> put_MaxLogins(This,lnMaxLogins) ) 

#define IADsUser_get_MaxStorage(This,retval)	\
    ( (This)->lpVtbl -> get_MaxStorage(This,retval) ) 

#define IADsUser_put_MaxStorage(This,lnMaxStorage)	\
    ( (This)->lpVtbl -> put_MaxStorage(This,lnMaxStorage) ) 

#define IADsUser_get_PasswordExpirationDate(This,retval)	\
    ( (This)->lpVtbl -> get_PasswordExpirationDate(This,retval) ) 

#define IADsUser_put_PasswordExpirationDate(This,daPasswordExpirationDate)	\
    ( (This)->lpVtbl -> put_PasswordExpirationDate(This,daPasswordExpirationDate) ) 

#define IADsUser_get_PasswordMinimumLength(This,retval)	\
    ( (This)->lpVtbl -> get_PasswordMinimumLength(This,retval) ) 

#define IADsUser_put_PasswordMinimumLength(This,lnPasswordMinimumLength)	\
    ( (This)->lpVtbl -> put_PasswordMinimumLength(This,lnPasswordMinimumLength) ) 

#define IADsUser_get_PasswordRequired(This,retval)	\
    ( (This)->lpVtbl -> get_PasswordRequired(This,retval) ) 

#define IADsUser_put_PasswordRequired(This,fPasswordRequired)	\
    ( (This)->lpVtbl -> put_PasswordRequired(This,fPasswordRequired) ) 

#define IADsUser_get_RequireUniquePassword(This,retval)	\
    ( (This)->lpVtbl -> get_RequireUniquePassword(This,retval) ) 

#define IADsUser_put_RequireUniquePassword(This,fRequireUniquePassword)	\
    ( (This)->lpVtbl -> put_RequireUniquePassword(This,fRequireUniquePassword) ) 

#define IADsUser_get_EmailAddress(This,retval)	\
    ( (This)->lpVtbl -> get_EmailAddress(This,retval) ) 

#define IADsUser_put_EmailAddress(This,bstrEmailAddress)	\
    ( (This)->lpVtbl -> put_EmailAddress(This,bstrEmailAddress) ) 

#define IADsUser_get_HomeDirectory(This,retval)	\
    ( (This)->lpVtbl -> get_HomeDirectory(This,retval) ) 

#define IADsUser_put_HomeDirectory(This,bstrHomeDirectory)	\
    ( (This)->lpVtbl -> put_HomeDirectory(This,bstrHomeDirectory) ) 

#define IADsUser_get_Languages(This,retval)	\
    ( (This)->lpVtbl -> get_Languages(This,retval) ) 

#define IADsUser_put_Languages(This,vLanguages)	\
    ( (This)->lpVtbl -> put_Languages(This,vLanguages) ) 

#define IADsUser_get_Profile(This,retval)	\
    ( (This)->lpVtbl -> get_Profile(This,retval) ) 

#define IADsUser_put_Profile(This,bstrProfile)	\
    ( (This)->lpVtbl -> put_Profile(This,bstrProfile) ) 

#define IADsUser_get_LoginScript(This,retval)	\
    ( (This)->lpVtbl -> get_LoginScript(This,retval) ) 

#define IADsUser_put_LoginScript(This,bstrLoginScript)	\
    ( (This)->lpVtbl -> put_LoginScript(This,bstrLoginScript) ) 

#define IADsUser_get_Picture(This,retval)	\
    ( (This)->lpVtbl -> get_Picture(This,retval) ) 

#define IADsUser_put_Picture(This,vPicture)	\
    ( (This)->lpVtbl -> put_Picture(This,vPicture) ) 

#define IADsUser_get_HomePage(This,retval)	\
    ( (This)->lpVtbl -> get_HomePage(This,retval) ) 

#define IADsUser_put_HomePage(This,bstrHomePage)	\
    ( (This)->lpVtbl -> put_HomePage(This,bstrHomePage) ) 

#define IADsUser_Groups(This,ppGroups)	\
    ( (This)->lpVtbl -> Groups(This,ppGroups) ) 

#define IADsUser_SetPassword(This,NewPassword)	\
    ( (This)->lpVtbl -> SetPassword(This,NewPassword) ) 

#define IADsUser_ChangePassword(This,bstrOldPassword,bstrNewPassword)	\
    ( (This)->lpVtbl -> ChangePassword(This,bstrOldPassword,bstrNewPassword) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsUser_INTERFACE_DEFINED__ */


#ifndef __IADsPrintQueue_INTERFACE_DEFINED__
#define __IADsPrintQueue_INTERFACE_DEFINED__

/* interface IADsPrintQueue */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPrintQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b15160d0-1226-11cf-a985-00aa006bc149")
    IADsPrintQueue : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrinterPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PrinterPath( 
            /* [in] */ __RPC__in BSTR bstrPrinterPath) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Model( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Model( 
            /* [in] */ __RPC__in BSTR bstrModel) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Datatype( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Datatype( 
            /* [in] */ __RPC__in BSTR bstrDatatype) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrintProcessor( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PrintProcessor( 
            /* [in] */ __RPC__in BSTR bstrPrintProcessor) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Location( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Location( 
            /* [in] */ __RPC__in BSTR bstrLocation) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StartTime( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_StartTime( 
            /* [in] */ DATE daStartTime) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UntilTime( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_UntilTime( 
            /* [in] */ DATE daUntilTime) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultJobPriority( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DefaultJobPriority( 
            /* [in] */ long lnDefaultJobPriority) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lnPriority) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BannerPage( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_BannerPage( 
            /* [in] */ __RPC__in BSTR bstrBannerPage) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrintDevices( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PrintDevices( 
            /* [in] */ VARIANT vPrintDevices) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NetAddresses( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NetAddresses( 
            /* [in] */ VARIANT vNetAddresses) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPrintQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPrintQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPrintQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPrintQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPrintQueue * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsPrintQueue * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsPrintQueue * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_PrinterPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrinterPath )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_PrinterPath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrinterPath )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrPrinterPath);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_Model)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Model )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_Model)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Model )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrModel);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_Datatype)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Datatype )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_Datatype)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Datatype )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrDatatype);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_PrintProcessor)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrintProcessor )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_PrintProcessor)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrintProcessor )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrPrintProcessor);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_Location)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Location )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_Location)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Location )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrLocation);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_StartTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartTime )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_StartTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartTime )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ DATE daStartTime);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_UntilTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UntilTime )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_UntilTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UntilTime )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ DATE daUntilTime);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_DefaultJobPriority)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultJobPriority )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_DefaultJobPriority)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultJobPriority )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ long lnDefaultJobPriority);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_Priority)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_Priority)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ long lnPriority);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_BannerPage)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BannerPage )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_BannerPage)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BannerPage )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ __RPC__in BSTR bstrBannerPage);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_PrintDevices)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrintDevices )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_PrintDevices)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrintDevices )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ VARIANT vPrintDevices);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, get_NetAddresses)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetAddresses )( 
            __RPC__in IADsPrintQueue * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueue, put_NetAddresses)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetAddresses )( 
            __RPC__in IADsPrintQueue * This,
            /* [in] */ VARIANT vNetAddresses);
        
        END_INTERFACE
    } IADsPrintQueueVtbl;

    interface IADsPrintQueue
    {
        CONST_VTBL struct IADsPrintQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPrintQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPrintQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPrintQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPrintQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPrintQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPrintQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPrintQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPrintQueue_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsPrintQueue_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsPrintQueue_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsPrintQueue_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsPrintQueue_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsPrintQueue_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsPrintQueue_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsPrintQueue_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsPrintQueue_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsPrintQueue_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsPrintQueue_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsPrintQueue_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsPrintQueue_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsPrintQueue_get_PrinterPath(This,retval)	\
    ( (This)->lpVtbl -> get_PrinterPath(This,retval) ) 

#define IADsPrintQueue_put_PrinterPath(This,bstrPrinterPath)	\
    ( (This)->lpVtbl -> put_PrinterPath(This,bstrPrinterPath) ) 

#define IADsPrintQueue_get_Model(This,retval)	\
    ( (This)->lpVtbl -> get_Model(This,retval) ) 

#define IADsPrintQueue_put_Model(This,bstrModel)	\
    ( (This)->lpVtbl -> put_Model(This,bstrModel) ) 

#define IADsPrintQueue_get_Datatype(This,retval)	\
    ( (This)->lpVtbl -> get_Datatype(This,retval) ) 

#define IADsPrintQueue_put_Datatype(This,bstrDatatype)	\
    ( (This)->lpVtbl -> put_Datatype(This,bstrDatatype) ) 

#define IADsPrintQueue_get_PrintProcessor(This,retval)	\
    ( (This)->lpVtbl -> get_PrintProcessor(This,retval) ) 

#define IADsPrintQueue_put_PrintProcessor(This,bstrPrintProcessor)	\
    ( (This)->lpVtbl -> put_PrintProcessor(This,bstrPrintProcessor) ) 

#define IADsPrintQueue_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsPrintQueue_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsPrintQueue_get_Location(This,retval)	\
    ( (This)->lpVtbl -> get_Location(This,retval) ) 

#define IADsPrintQueue_put_Location(This,bstrLocation)	\
    ( (This)->lpVtbl -> put_Location(This,bstrLocation) ) 

#define IADsPrintQueue_get_StartTime(This,retval)	\
    ( (This)->lpVtbl -> get_StartTime(This,retval) ) 

#define IADsPrintQueue_put_StartTime(This,daStartTime)	\
    ( (This)->lpVtbl -> put_StartTime(This,daStartTime) ) 

#define IADsPrintQueue_get_UntilTime(This,retval)	\
    ( (This)->lpVtbl -> get_UntilTime(This,retval) ) 

#define IADsPrintQueue_put_UntilTime(This,daUntilTime)	\
    ( (This)->lpVtbl -> put_UntilTime(This,daUntilTime) ) 

#define IADsPrintQueue_get_DefaultJobPriority(This,retval)	\
    ( (This)->lpVtbl -> get_DefaultJobPriority(This,retval) ) 

#define IADsPrintQueue_put_DefaultJobPriority(This,lnDefaultJobPriority)	\
    ( (This)->lpVtbl -> put_DefaultJobPriority(This,lnDefaultJobPriority) ) 

#define IADsPrintQueue_get_Priority(This,retval)	\
    ( (This)->lpVtbl -> get_Priority(This,retval) ) 

#define IADsPrintQueue_put_Priority(This,lnPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lnPriority) ) 

#define IADsPrintQueue_get_BannerPage(This,retval)	\
    ( (This)->lpVtbl -> get_BannerPage(This,retval) ) 

#define IADsPrintQueue_put_BannerPage(This,bstrBannerPage)	\
    ( (This)->lpVtbl -> put_BannerPage(This,bstrBannerPage) ) 

#define IADsPrintQueue_get_PrintDevices(This,retval)	\
    ( (This)->lpVtbl -> get_PrintDevices(This,retval) ) 

#define IADsPrintQueue_put_PrintDevices(This,vPrintDevices)	\
    ( (This)->lpVtbl -> put_PrintDevices(This,vPrintDevices) ) 

#define IADsPrintQueue_get_NetAddresses(This,retval)	\
    ( (This)->lpVtbl -> get_NetAddresses(This,retval) ) 

#define IADsPrintQueue_put_NetAddresses(This,vNetAddresses)	\
    ( (This)->lpVtbl -> put_NetAddresses(This,vNetAddresses) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPrintQueue_INTERFACE_DEFINED__ */


#ifndef __IADsPrintQueueOperations_INTERFACE_DEFINED__
#define __IADsPrintQueueOperations_INTERFACE_DEFINED__

/* interface IADsPrintQueueOperations */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPrintQueueOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("124be5c0-156e-11cf-a986-00aa006bc149")
    IADsPrintQueueOperations : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE PrintJobs( 
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **pObject) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Purge( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPrintQueueOperationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPrintQueueOperations * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsPrintQueueOperations, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintQueueOperations, PrintJobs)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PrintJobs )( 
            __RPC__in IADsPrintQueueOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **pObject);
        
        DECLSPEC_XFGVIRT(IADsPrintQueueOperations, Pause)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        DECLSPEC_XFGVIRT(IADsPrintQueueOperations, Resume)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        DECLSPEC_XFGVIRT(IADsPrintQueueOperations, Purge)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Purge )( 
            __RPC__in IADsPrintQueueOperations * This);
        
        END_INTERFACE
    } IADsPrintQueueOperationsVtbl;

    interface IADsPrintQueueOperations
    {
        CONST_VTBL struct IADsPrintQueueOperationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPrintQueueOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPrintQueueOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPrintQueueOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPrintQueueOperations_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPrintQueueOperations_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPrintQueueOperations_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPrintQueueOperations_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPrintQueueOperations_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsPrintQueueOperations_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsPrintQueueOperations_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsPrintQueueOperations_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsPrintQueueOperations_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsPrintQueueOperations_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsPrintQueueOperations_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsPrintQueueOperations_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsPrintQueueOperations_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsPrintQueueOperations_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsPrintQueueOperations_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsPrintQueueOperations_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsPrintQueueOperations_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsPrintQueueOperations_get_Status(This,retval)	\
    ( (This)->lpVtbl -> get_Status(This,retval) ) 

#define IADsPrintQueueOperations_PrintJobs(This,pObject)	\
    ( (This)->lpVtbl -> PrintJobs(This,pObject) ) 

#define IADsPrintQueueOperations_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IADsPrintQueueOperations_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IADsPrintQueueOperations_Purge(This)	\
    ( (This)->lpVtbl -> Purge(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPrintQueueOperations_INTERFACE_DEFINED__ */


#ifndef __IADsPrintJob_INTERFACE_DEFINED__
#define __IADsPrintJob_INTERFACE_DEFINED__

/* interface IADsPrintJob */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPrintJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("32fb6780-1ed0-11cf-a988-00aa006bc149")
    IADsPrintJob : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HostPrintQueue( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_User( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TimeSubmitted( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalPages( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lnPriority) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StartTime( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_StartTime( 
            /* [in] */ DATE daStartTime) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UntilTime( 
            /* [retval][out] */ __RPC__out DATE *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_UntilTime( 
            /* [in] */ DATE daUntilTime) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Notify( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Notify( 
            /* [in] */ __RPC__in BSTR bstrNotify) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NotifyPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_NotifyPath( 
            /* [in] */ __RPC__in BSTR bstrNotifyPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPrintJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPrintJob * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPrintJob * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsPrintJob * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_HostPrintQueue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostPrintQueue )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_User)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_User )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_UserPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserPath )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_TimeSubmitted)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TimeSubmitted )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_TotalPages)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalPages )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_Size)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_Priority)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, put_Priority)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ long lnPriority);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_StartTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartTime )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, put_StartTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartTime )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ DATE daStartTime);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_UntilTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UntilTime )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__out DATE *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, put_UntilTime)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UntilTime )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ DATE daUntilTime);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_Notify)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Notify )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, put_Notify)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Notify )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in BSTR bstrNotify);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, get_NotifyPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NotifyPath )( 
            __RPC__in IADsPrintJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJob, put_NotifyPath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NotifyPath )( 
            __RPC__in IADsPrintJob * This,
            /* [in] */ __RPC__in BSTR bstrNotifyPath);
        
        END_INTERFACE
    } IADsPrintJobVtbl;

    interface IADsPrintJob
    {
        CONST_VTBL struct IADsPrintJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPrintJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPrintJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPrintJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPrintJob_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPrintJob_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPrintJob_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPrintJob_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPrintJob_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsPrintJob_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsPrintJob_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsPrintJob_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsPrintJob_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsPrintJob_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsPrintJob_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsPrintJob_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsPrintJob_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsPrintJob_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsPrintJob_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsPrintJob_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsPrintJob_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsPrintJob_get_HostPrintQueue(This,retval)	\
    ( (This)->lpVtbl -> get_HostPrintQueue(This,retval) ) 

#define IADsPrintJob_get_User(This,retval)	\
    ( (This)->lpVtbl -> get_User(This,retval) ) 

#define IADsPrintJob_get_UserPath(This,retval)	\
    ( (This)->lpVtbl -> get_UserPath(This,retval) ) 

#define IADsPrintJob_get_TimeSubmitted(This,retval)	\
    ( (This)->lpVtbl -> get_TimeSubmitted(This,retval) ) 

#define IADsPrintJob_get_TotalPages(This,retval)	\
    ( (This)->lpVtbl -> get_TotalPages(This,retval) ) 

#define IADsPrintJob_get_Size(This,retval)	\
    ( (This)->lpVtbl -> get_Size(This,retval) ) 

#define IADsPrintJob_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsPrintJob_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsPrintJob_get_Priority(This,retval)	\
    ( (This)->lpVtbl -> get_Priority(This,retval) ) 

#define IADsPrintJob_put_Priority(This,lnPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lnPriority) ) 

#define IADsPrintJob_get_StartTime(This,retval)	\
    ( (This)->lpVtbl -> get_StartTime(This,retval) ) 

#define IADsPrintJob_put_StartTime(This,daStartTime)	\
    ( (This)->lpVtbl -> put_StartTime(This,daStartTime) ) 

#define IADsPrintJob_get_UntilTime(This,retval)	\
    ( (This)->lpVtbl -> get_UntilTime(This,retval) ) 

#define IADsPrintJob_put_UntilTime(This,daUntilTime)	\
    ( (This)->lpVtbl -> put_UntilTime(This,daUntilTime) ) 

#define IADsPrintJob_get_Notify(This,retval)	\
    ( (This)->lpVtbl -> get_Notify(This,retval) ) 

#define IADsPrintJob_put_Notify(This,bstrNotify)	\
    ( (This)->lpVtbl -> put_Notify(This,bstrNotify) ) 

#define IADsPrintJob_get_NotifyPath(This,retval)	\
    ( (This)->lpVtbl -> get_NotifyPath(This,retval) ) 

#define IADsPrintJob_put_NotifyPath(This,bstrNotifyPath)	\
    ( (This)->lpVtbl -> put_NotifyPath(This,bstrNotifyPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPrintJob_INTERFACE_DEFINED__ */


#ifndef __IADsPrintJobOperations_INTERFACE_DEFINED__
#define __IADsPrintJobOperations_INTERFACE_DEFINED__

/* interface IADsPrintJobOperations */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPrintJobOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9a52db30-1ecf-11cf-a988-00aa006bc149")
    IADsPrintJobOperations : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TimeElapsed( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PagesPrinted( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Position( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Position( 
            /* [in] */ long lnPosition) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPrintJobOperationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPrintJobOperations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPrintJobOperations * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPrintJobOperations * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsPrintJobOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsPrintJobOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, get_TimeElapsed)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TimeElapsed )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, get_PagesPrinted)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PagesPrinted )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, get_Position)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Position )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, put_Position)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Position )( 
            __RPC__in IADsPrintJobOperations * This,
            /* [in] */ long lnPosition);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, Pause)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IADsPrintJobOperations * This);
        
        DECLSPEC_XFGVIRT(IADsPrintJobOperations, Resume)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IADsPrintJobOperations * This);
        
        END_INTERFACE
    } IADsPrintJobOperationsVtbl;

    interface IADsPrintJobOperations
    {
        CONST_VTBL struct IADsPrintJobOperationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPrintJobOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPrintJobOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPrintJobOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPrintJobOperations_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPrintJobOperations_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPrintJobOperations_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPrintJobOperations_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPrintJobOperations_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsPrintJobOperations_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsPrintJobOperations_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsPrintJobOperations_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsPrintJobOperations_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsPrintJobOperations_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsPrintJobOperations_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsPrintJobOperations_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsPrintJobOperations_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsPrintJobOperations_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsPrintJobOperations_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsPrintJobOperations_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsPrintJobOperations_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsPrintJobOperations_get_Status(This,retval)	\
    ( (This)->lpVtbl -> get_Status(This,retval) ) 

#define IADsPrintJobOperations_get_TimeElapsed(This,retval)	\
    ( (This)->lpVtbl -> get_TimeElapsed(This,retval) ) 

#define IADsPrintJobOperations_get_PagesPrinted(This,retval)	\
    ( (This)->lpVtbl -> get_PagesPrinted(This,retval) ) 

#define IADsPrintJobOperations_get_Position(This,retval)	\
    ( (This)->lpVtbl -> get_Position(This,retval) ) 

#define IADsPrintJobOperations_put_Position(This,lnPosition)	\
    ( (This)->lpVtbl -> put_Position(This,lnPosition) ) 

#define IADsPrintJobOperations_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IADsPrintJobOperations_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPrintJobOperations_INTERFACE_DEFINED__ */


#ifndef __IADsService_INTERFACE_DEFINED__
#define __IADsService_INTERFACE_DEFINED__

/* interface IADsService */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68af66e0-31ca-11cf-a98a-00aa006bc149")
    IADsService : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HostComputer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HostComputer( 
            /* [in] */ __RPC__in BSTR bstrHostComputer) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ __RPC__in BSTR bstrDisplayName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Version( 
            /* [in] */ __RPC__in BSTR bstrVersion) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ServiceType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ServiceType( 
            /* [in] */ long lnServiceType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StartType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_StartType( 
            /* [in] */ long lnStartType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ __RPC__in BSTR bstrPath) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StartupParameters( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_StartupParameters( 
            /* [in] */ __RPC__in BSTR bstrStartupParameters) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ErrorControl( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ErrorControl( 
            /* [in] */ long lnErrorControl) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LoadOrderGroup( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LoadOrderGroup( 
            /* [in] */ __RPC__in BSTR bstrLoadOrderGroup) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ServiceAccountName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ServiceAccountName( 
            /* [in] */ __RPC__in BSTR bstrServiceAccountName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ServiceAccountPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ServiceAccountPath( 
            /* [in] */ __RPC__in BSTR bstrServiceAccountPath) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Dependencies( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Dependencies( 
            /* [in] */ VARIANT vDependencies) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsService * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsService * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsService * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsService * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsService * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsService * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsService * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsService * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsService, get_HostComputer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostComputer )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_HostComputer)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HostComputer )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrHostComputer);
        
        DECLSPEC_XFGVIRT(IADsService, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_DisplayName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrDisplayName);
        
        DECLSPEC_XFGVIRT(IADsService, get_Version)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_Version)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrVersion);
        
        DECLSPEC_XFGVIRT(IADsService, get_ServiceType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceType )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ServiceType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceType )( 
            __RPC__in IADsService * This,
            /* [in] */ long lnServiceType);
        
        DECLSPEC_XFGVIRT(IADsService, get_StartType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartType )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_StartType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartType )( 
            __RPC__in IADsService * This,
            /* [in] */ long lnStartType);
        
        DECLSPEC_XFGVIRT(IADsService, get_Path)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_Path)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrPath);
        
        DECLSPEC_XFGVIRT(IADsService, get_StartupParameters)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartupParameters )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_StartupParameters)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartupParameters )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrStartupParameters);
        
        DECLSPEC_XFGVIRT(IADsService, get_ErrorControl)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorControl )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ErrorControl)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ErrorControl )( 
            __RPC__in IADsService * This,
            /* [in] */ long lnErrorControl);
        
        DECLSPEC_XFGVIRT(IADsService, get_LoadOrderGroup)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoadOrderGroup )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_LoadOrderGroup)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoadOrderGroup )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrLoadOrderGroup);
        
        DECLSPEC_XFGVIRT(IADsService, get_ServiceAccountName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceAccountName )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ServiceAccountName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceAccountName )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrServiceAccountName);
        
        DECLSPEC_XFGVIRT(IADsService, get_ServiceAccountPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceAccountPath )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ServiceAccountPath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceAccountPath )( 
            __RPC__in IADsService * This,
            /* [in] */ __RPC__in BSTR bstrServiceAccountPath);
        
        DECLSPEC_XFGVIRT(IADsService, get_Dependencies)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dependencies )( 
            __RPC__in IADsService * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_Dependencies)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Dependencies )( 
            __RPC__in IADsService * This,
            /* [in] */ VARIANT vDependencies);
        
        END_INTERFACE
    } IADsServiceVtbl;

    interface IADsService
    {
        CONST_VTBL struct IADsServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsService_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsService_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsService_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsService_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsService_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsService_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsService_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsService_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsService_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsService_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsService_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsService_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsService_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsService_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsService_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsService_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsService_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsService_get_HostComputer(This,retval)	\
    ( (This)->lpVtbl -> get_HostComputer(This,retval) ) 

#define IADsService_put_HostComputer(This,bstrHostComputer)	\
    ( (This)->lpVtbl -> put_HostComputer(This,bstrHostComputer) ) 

#define IADsService_get_DisplayName(This,retval)	\
    ( (This)->lpVtbl -> get_DisplayName(This,retval) ) 

#define IADsService_put_DisplayName(This,bstrDisplayName)	\
    ( (This)->lpVtbl -> put_DisplayName(This,bstrDisplayName) ) 

#define IADsService_get_Version(This,retval)	\
    ( (This)->lpVtbl -> get_Version(This,retval) ) 

#define IADsService_put_Version(This,bstrVersion)	\
    ( (This)->lpVtbl -> put_Version(This,bstrVersion) ) 

#define IADsService_get_ServiceType(This,retval)	\
    ( (This)->lpVtbl -> get_ServiceType(This,retval) ) 

#define IADsService_put_ServiceType(This,lnServiceType)	\
    ( (This)->lpVtbl -> put_ServiceType(This,lnServiceType) ) 

#define IADsService_get_StartType(This,retval)	\
    ( (This)->lpVtbl -> get_StartType(This,retval) ) 

#define IADsService_put_StartType(This,lnStartType)	\
    ( (This)->lpVtbl -> put_StartType(This,lnStartType) ) 

#define IADsService_get_Path(This,retval)	\
    ( (This)->lpVtbl -> get_Path(This,retval) ) 

#define IADsService_put_Path(This,bstrPath)	\
    ( (This)->lpVtbl -> put_Path(This,bstrPath) ) 

#define IADsService_get_StartupParameters(This,retval)	\
    ( (This)->lpVtbl -> get_StartupParameters(This,retval) ) 

#define IADsService_put_StartupParameters(This,bstrStartupParameters)	\
    ( (This)->lpVtbl -> put_StartupParameters(This,bstrStartupParameters) ) 

#define IADsService_get_ErrorControl(This,retval)	\
    ( (This)->lpVtbl -> get_ErrorControl(This,retval) ) 

#define IADsService_put_ErrorControl(This,lnErrorControl)	\
    ( (This)->lpVtbl -> put_ErrorControl(This,lnErrorControl) ) 

#define IADsService_get_LoadOrderGroup(This,retval)	\
    ( (This)->lpVtbl -> get_LoadOrderGroup(This,retval) ) 

#define IADsService_put_LoadOrderGroup(This,bstrLoadOrderGroup)	\
    ( (This)->lpVtbl -> put_LoadOrderGroup(This,bstrLoadOrderGroup) ) 

#define IADsService_get_ServiceAccountName(This,retval)	\
    ( (This)->lpVtbl -> get_ServiceAccountName(This,retval) ) 

#define IADsService_put_ServiceAccountName(This,bstrServiceAccountName)	\
    ( (This)->lpVtbl -> put_ServiceAccountName(This,bstrServiceAccountName) ) 

#define IADsService_get_ServiceAccountPath(This,retval)	\
    ( (This)->lpVtbl -> get_ServiceAccountPath(This,retval) ) 

#define IADsService_put_ServiceAccountPath(This,bstrServiceAccountPath)	\
    ( (This)->lpVtbl -> put_ServiceAccountPath(This,bstrServiceAccountPath) ) 

#define IADsService_get_Dependencies(This,retval)	\
    ( (This)->lpVtbl -> get_Dependencies(This,retval) ) 

#define IADsService_put_Dependencies(This,vDependencies)	\
    ( (This)->lpVtbl -> put_Dependencies(This,vDependencies) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsService_INTERFACE_DEFINED__ */


#ifndef __IADsServiceOperations_INTERFACE_DEFINED__
#define __IADsServiceOperations_INTERFACE_DEFINED__

/* interface IADsServiceOperations */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsServiceOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5d7b33f0-31ca-11cf-a98a-00aa006bc149")
    IADsServiceOperations : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Continue( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetPassword( 
            /* [in] */ __RPC__in BSTR bstrNewPassword) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsServiceOperationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsServiceOperations * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsServiceOperations * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IADsServiceOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Start)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Stop)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Pause)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Continue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Continue )( 
            __RPC__in IADsServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, SetPassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPassword )( 
            __RPC__in IADsServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrNewPassword);
        
        END_INTERFACE
    } IADsServiceOperationsVtbl;

    interface IADsServiceOperations
    {
        CONST_VTBL struct IADsServiceOperationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsServiceOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsServiceOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsServiceOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsServiceOperations_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsServiceOperations_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsServiceOperations_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsServiceOperations_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsServiceOperations_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsServiceOperations_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsServiceOperations_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsServiceOperations_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsServiceOperations_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsServiceOperations_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsServiceOperations_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsServiceOperations_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsServiceOperations_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsServiceOperations_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsServiceOperations_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsServiceOperations_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsServiceOperations_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsServiceOperations_get_Status(This,retval)	\
    ( (This)->lpVtbl -> get_Status(This,retval) ) 

#define IADsServiceOperations_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IADsServiceOperations_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IADsServiceOperations_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IADsServiceOperations_Continue(This)	\
    ( (This)->lpVtbl -> Continue(This) ) 

#define IADsServiceOperations_SetPassword(This,bstrNewPassword)	\
    ( (This)->lpVtbl -> SetPassword(This,bstrNewPassword) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsServiceOperations_INTERFACE_DEFINED__ */


#ifndef __IADsFileService_INTERFACE_DEFINED__
#define __IADsFileService_INTERFACE_DEFINED__

/* interface IADsFileService */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsFileService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a89d1900-31ca-11cf-a98a-00aa006bc149")
    IADsFileService : public IADsService
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxUserCount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxUserCount( 
            /* [in] */ long lnMaxUserCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsFileServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsFileService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsFileService * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsFileService * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsFileService * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsFileService * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsFileService * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsFileService * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsFileService * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsFileService * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsService, get_HostComputer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostComputer )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_HostComputer)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HostComputer )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrHostComputer);
        
        DECLSPEC_XFGVIRT(IADsService, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_DisplayName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrDisplayName);
        
        DECLSPEC_XFGVIRT(IADsService, get_Version)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_Version)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrVersion);
        
        DECLSPEC_XFGVIRT(IADsService, get_ServiceType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceType )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ServiceType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceType )( 
            __RPC__in IADsFileService * This,
            /* [in] */ long lnServiceType);
        
        DECLSPEC_XFGVIRT(IADsService, get_StartType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartType )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_StartType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartType )( 
            __RPC__in IADsFileService * This,
            /* [in] */ long lnStartType);
        
        DECLSPEC_XFGVIRT(IADsService, get_Path)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_Path)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrPath);
        
        DECLSPEC_XFGVIRT(IADsService, get_StartupParameters)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StartupParameters )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_StartupParameters)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StartupParameters )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrStartupParameters);
        
        DECLSPEC_XFGVIRT(IADsService, get_ErrorControl)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorControl )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ErrorControl)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ErrorControl )( 
            __RPC__in IADsFileService * This,
            /* [in] */ long lnErrorControl);
        
        DECLSPEC_XFGVIRT(IADsService, get_LoadOrderGroup)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoadOrderGroup )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_LoadOrderGroup)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoadOrderGroup )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrLoadOrderGroup);
        
        DECLSPEC_XFGVIRT(IADsService, get_ServiceAccountName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceAccountName )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ServiceAccountName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceAccountName )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrServiceAccountName);
        
        DECLSPEC_XFGVIRT(IADsService, get_ServiceAccountPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceAccountPath )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_ServiceAccountPath)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceAccountPath )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrServiceAccountPath);
        
        DECLSPEC_XFGVIRT(IADsService, get_Dependencies)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dependencies )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsService, put_Dependencies)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Dependencies )( 
            __RPC__in IADsFileService * This,
            /* [in] */ VARIANT vDependencies);
        
        DECLSPEC_XFGVIRT(IADsFileService, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsFileService, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsFileService * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsFileService, get_MaxUserCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxUserCount )( 
            __RPC__in IADsFileService * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsFileService, put_MaxUserCount)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxUserCount )( 
            __RPC__in IADsFileService * This,
            /* [in] */ long lnMaxUserCount);
        
        END_INTERFACE
    } IADsFileServiceVtbl;

    interface IADsFileService
    {
        CONST_VTBL struct IADsFileServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsFileService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsFileService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsFileService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsFileService_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsFileService_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsFileService_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsFileService_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsFileService_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsFileService_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsFileService_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsFileService_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsFileService_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsFileService_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsFileService_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsFileService_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsFileService_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsFileService_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsFileService_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsFileService_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsFileService_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsFileService_get_HostComputer(This,retval)	\
    ( (This)->lpVtbl -> get_HostComputer(This,retval) ) 

#define IADsFileService_put_HostComputer(This,bstrHostComputer)	\
    ( (This)->lpVtbl -> put_HostComputer(This,bstrHostComputer) ) 

#define IADsFileService_get_DisplayName(This,retval)	\
    ( (This)->lpVtbl -> get_DisplayName(This,retval) ) 

#define IADsFileService_put_DisplayName(This,bstrDisplayName)	\
    ( (This)->lpVtbl -> put_DisplayName(This,bstrDisplayName) ) 

#define IADsFileService_get_Version(This,retval)	\
    ( (This)->lpVtbl -> get_Version(This,retval) ) 

#define IADsFileService_put_Version(This,bstrVersion)	\
    ( (This)->lpVtbl -> put_Version(This,bstrVersion) ) 

#define IADsFileService_get_ServiceType(This,retval)	\
    ( (This)->lpVtbl -> get_ServiceType(This,retval) ) 

#define IADsFileService_put_ServiceType(This,lnServiceType)	\
    ( (This)->lpVtbl -> put_ServiceType(This,lnServiceType) ) 

#define IADsFileService_get_StartType(This,retval)	\
    ( (This)->lpVtbl -> get_StartType(This,retval) ) 

#define IADsFileService_put_StartType(This,lnStartType)	\
    ( (This)->lpVtbl -> put_StartType(This,lnStartType) ) 

#define IADsFileService_get_Path(This,retval)	\
    ( (This)->lpVtbl -> get_Path(This,retval) ) 

#define IADsFileService_put_Path(This,bstrPath)	\
    ( (This)->lpVtbl -> put_Path(This,bstrPath) ) 

#define IADsFileService_get_StartupParameters(This,retval)	\
    ( (This)->lpVtbl -> get_StartupParameters(This,retval) ) 

#define IADsFileService_put_StartupParameters(This,bstrStartupParameters)	\
    ( (This)->lpVtbl -> put_StartupParameters(This,bstrStartupParameters) ) 

#define IADsFileService_get_ErrorControl(This,retval)	\
    ( (This)->lpVtbl -> get_ErrorControl(This,retval) ) 

#define IADsFileService_put_ErrorControl(This,lnErrorControl)	\
    ( (This)->lpVtbl -> put_ErrorControl(This,lnErrorControl) ) 

#define IADsFileService_get_LoadOrderGroup(This,retval)	\
    ( (This)->lpVtbl -> get_LoadOrderGroup(This,retval) ) 

#define IADsFileService_put_LoadOrderGroup(This,bstrLoadOrderGroup)	\
    ( (This)->lpVtbl -> put_LoadOrderGroup(This,bstrLoadOrderGroup) ) 

#define IADsFileService_get_ServiceAccountName(This,retval)	\
    ( (This)->lpVtbl -> get_ServiceAccountName(This,retval) ) 

#define IADsFileService_put_ServiceAccountName(This,bstrServiceAccountName)	\
    ( (This)->lpVtbl -> put_ServiceAccountName(This,bstrServiceAccountName) ) 

#define IADsFileService_get_ServiceAccountPath(This,retval)	\
    ( (This)->lpVtbl -> get_ServiceAccountPath(This,retval) ) 

#define IADsFileService_put_ServiceAccountPath(This,bstrServiceAccountPath)	\
    ( (This)->lpVtbl -> put_ServiceAccountPath(This,bstrServiceAccountPath) ) 

#define IADsFileService_get_Dependencies(This,retval)	\
    ( (This)->lpVtbl -> get_Dependencies(This,retval) ) 

#define IADsFileService_put_Dependencies(This,vDependencies)	\
    ( (This)->lpVtbl -> put_Dependencies(This,vDependencies) ) 


#define IADsFileService_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsFileService_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsFileService_get_MaxUserCount(This,retval)	\
    ( (This)->lpVtbl -> get_MaxUserCount(This,retval) ) 

#define IADsFileService_put_MaxUserCount(This,lnMaxUserCount)	\
    ( (This)->lpVtbl -> put_MaxUserCount(This,lnMaxUserCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsFileService_INTERFACE_DEFINED__ */


#ifndef __IADsFileServiceOperations_INTERFACE_DEFINED__
#define __IADsFileServiceOperations_INTERFACE_DEFINED__

/* interface IADsFileServiceOperations */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsFileServiceOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a02ded10-31ca-11cf-a98a-00aa006bc149")
    IADsFileServiceOperations : public IADsServiceOperations
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Sessions( 
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppSessions) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Resources( 
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppResources) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsFileServiceOperationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsFileServiceOperations * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, get_Status)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Start)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Stop)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Pause)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, Continue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Continue )( 
            __RPC__in IADsFileServiceOperations * This);
        
        DECLSPEC_XFGVIRT(IADsServiceOperations, SetPassword)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetPassword )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [in] */ __RPC__in BSTR bstrNewPassword);
        
        DECLSPEC_XFGVIRT(IADsFileServiceOperations, Sessions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Sessions )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppSessions);
        
        DECLSPEC_XFGVIRT(IADsFileServiceOperations, Resources)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Resources )( 
            __RPC__in IADsFileServiceOperations * This,
            /* [retval][out] */ __RPC__deref_out_opt IADsCollection **ppResources);
        
        END_INTERFACE
    } IADsFileServiceOperationsVtbl;

    interface IADsFileServiceOperations
    {
        CONST_VTBL struct IADsFileServiceOperationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsFileServiceOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsFileServiceOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsFileServiceOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsFileServiceOperations_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsFileServiceOperations_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsFileServiceOperations_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsFileServiceOperations_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsFileServiceOperations_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsFileServiceOperations_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsFileServiceOperations_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsFileServiceOperations_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsFileServiceOperations_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsFileServiceOperations_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsFileServiceOperations_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsFileServiceOperations_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsFileServiceOperations_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsFileServiceOperations_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsFileServiceOperations_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsFileServiceOperations_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsFileServiceOperations_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsFileServiceOperations_get_Status(This,retval)	\
    ( (This)->lpVtbl -> get_Status(This,retval) ) 

#define IADsFileServiceOperations_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IADsFileServiceOperations_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IADsFileServiceOperations_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IADsFileServiceOperations_Continue(This)	\
    ( (This)->lpVtbl -> Continue(This) ) 

#define IADsFileServiceOperations_SetPassword(This,bstrNewPassword)	\
    ( (This)->lpVtbl -> SetPassword(This,bstrNewPassword) ) 


#define IADsFileServiceOperations_Sessions(This,ppSessions)	\
    ( (This)->lpVtbl -> Sessions(This,ppSessions) ) 

#define IADsFileServiceOperations_Resources(This,ppResources)	\
    ( (This)->lpVtbl -> Resources(This,ppResources) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsFileServiceOperations_INTERFACE_DEFINED__ */


#ifndef __IADsFileShare_INTERFACE_DEFINED__
#define __IADsFileShare_INTERFACE_DEFINED__

/* interface IADsFileShare */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsFileShare;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eb6dcaf0-4b83-11cf-a995-00aa006bc149")
    IADsFileShare : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentUserCount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HostComputer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HostComputer( 
            /* [in] */ __RPC__in BSTR bstrHostComputer) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ __RPC__in BSTR bstrPath) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxUserCount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxUserCount( 
            /* [in] */ long lnMaxUserCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsFileShareVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsFileShare * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsFileShare * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsFileShare * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsFileShare * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsFileShare * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsFileShare * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsFileShare, get_CurrentUserCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentUserCount )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsFileShare, get_Description)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsFileShare, put_Description)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IADsFileShare, get_HostComputer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostComputer )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsFileShare, put_HostComputer)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HostComputer )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in BSTR bstrHostComputer);
        
        DECLSPEC_XFGVIRT(IADsFileShare, get_Path)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsFileShare, put_Path)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ __RPC__in BSTR bstrPath);
        
        DECLSPEC_XFGVIRT(IADsFileShare, get_MaxUserCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxUserCount )( 
            __RPC__in IADsFileShare * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsFileShare, put_MaxUserCount)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxUserCount )( 
            __RPC__in IADsFileShare * This,
            /* [in] */ long lnMaxUserCount);
        
        END_INTERFACE
    } IADsFileShareVtbl;

    interface IADsFileShare
    {
        CONST_VTBL struct IADsFileShareVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsFileShare_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsFileShare_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsFileShare_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsFileShare_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsFileShare_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsFileShare_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsFileShare_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsFileShare_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsFileShare_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsFileShare_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsFileShare_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsFileShare_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsFileShare_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsFileShare_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsFileShare_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsFileShare_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsFileShare_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsFileShare_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsFileShare_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsFileShare_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsFileShare_get_CurrentUserCount(This,retval)	\
    ( (This)->lpVtbl -> get_CurrentUserCount(This,retval) ) 

#define IADsFileShare_get_Description(This,retval)	\
    ( (This)->lpVtbl -> get_Description(This,retval) ) 

#define IADsFileShare_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IADsFileShare_get_HostComputer(This,retval)	\
    ( (This)->lpVtbl -> get_HostComputer(This,retval) ) 

#define IADsFileShare_put_HostComputer(This,bstrHostComputer)	\
    ( (This)->lpVtbl -> put_HostComputer(This,bstrHostComputer) ) 

#define IADsFileShare_get_Path(This,retval)	\
    ( (This)->lpVtbl -> get_Path(This,retval) ) 

#define IADsFileShare_put_Path(This,bstrPath)	\
    ( (This)->lpVtbl -> put_Path(This,bstrPath) ) 

#define IADsFileShare_get_MaxUserCount(This,retval)	\
    ( (This)->lpVtbl -> get_MaxUserCount(This,retval) ) 

#define IADsFileShare_put_MaxUserCount(This,lnMaxUserCount)	\
    ( (This)->lpVtbl -> put_MaxUserCount(This,lnMaxUserCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsFileShare_INTERFACE_DEFINED__ */


#ifndef __IADsSession_INTERFACE_DEFINED__
#define __IADsSession_INTERFACE_DEFINED__

/* interface IADsSession */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("398b7da0-4aab-11cf-ae2c-00aa006ebfb9")
    IADsSession : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_User( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Computer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectTime( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_IdleTime( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsSession * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsSession * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsSession * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsSession * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsSession * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsSession * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsSession * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsSession * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsSession * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsSession * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsSession * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsSession, get_User)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_User )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsSession, get_UserPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserPath )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsSession, get_Computer)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Computer )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsSession, get_ComputerPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerPath )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsSession, get_ConnectTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectTime )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsSession, get_IdleTime)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IdleTime )( 
            __RPC__in IADsSession * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        END_INTERFACE
    } IADsSessionVtbl;

    interface IADsSession
    {
        CONST_VTBL struct IADsSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsSession_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsSession_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsSession_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsSession_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsSession_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsSession_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsSession_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsSession_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsSession_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsSession_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsSession_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsSession_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsSession_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsSession_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsSession_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsSession_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsSession_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsSession_get_User(This,retval)	\
    ( (This)->lpVtbl -> get_User(This,retval) ) 

#define IADsSession_get_UserPath(This,retval)	\
    ( (This)->lpVtbl -> get_UserPath(This,retval) ) 

#define IADsSession_get_Computer(This,retval)	\
    ( (This)->lpVtbl -> get_Computer(This,retval) ) 

#define IADsSession_get_ComputerPath(This,retval)	\
    ( (This)->lpVtbl -> get_ComputerPath(This,retval) ) 

#define IADsSession_get_ConnectTime(This,retval)	\
    ( (This)->lpVtbl -> get_ConnectTime(This,retval) ) 

#define IADsSession_get_IdleTime(This,retval)	\
    ( (This)->lpVtbl -> get_IdleTime(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsSession_INTERFACE_DEFINED__ */


#ifndef __IADsResource_INTERFACE_DEFINED__
#define __IADsResource_INTERFACE_DEFINED__

/* interface IADsResource */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34a05b20-4aab-11cf-ae2c-00aa006ebfb9")
    IADsResource : public IADs
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_User( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LockCount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsResource * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsResource * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsResource * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsResource * This,
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
        
        DECLSPEC_XFGVIRT(IADs, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Class)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Class )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_GUID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_ADsPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ADsPath )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Parent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, get_Schema)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Schema )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADs, GetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IADsResource * This);
        
        DECLSPEC_XFGVIRT(IADs, SetInfo)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInfo )( 
            __RPC__in IADsResource * This);
        
        DECLSPEC_XFGVIRT(IADs, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsResource * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, Put)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Put )( 
            __RPC__in IADsResource * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsResource * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out VARIANT *pvProp);
        
        DECLSPEC_XFGVIRT(IADs, PutEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *PutEx )( 
            __RPC__in IADsResource * This,
            /* [in] */ long lnControlCode,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT vProp);
        
        DECLSPEC_XFGVIRT(IADs, GetInfoEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInfoEx )( 
            __RPC__in IADsResource * This,
            /* [in] */ VARIANT vProperties,
            /* [in] */ long lnReserved);
        
        DECLSPEC_XFGVIRT(IADsResource, get_User)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_User )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsResource, get_UserPath)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserPath )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsResource, get_Path)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsResource, get_LockCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LockCount )( 
            __RPC__in IADsResource * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        END_INTERFACE
    } IADsResourceVtbl;

    interface IADsResource
    {
        CONST_VTBL struct IADsResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsResource_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsResource_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsResource_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsResource_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsResource_get_Name(This,retval)	\
    ( (This)->lpVtbl -> get_Name(This,retval) ) 

#define IADsResource_get_Class(This,retval)	\
    ( (This)->lpVtbl -> get_Class(This,retval) ) 

#define IADsResource_get_GUID(This,retval)	\
    ( (This)->lpVtbl -> get_GUID(This,retval) ) 

#define IADsResource_get_ADsPath(This,retval)	\
    ( (This)->lpVtbl -> get_ADsPath(This,retval) ) 

#define IADsResource_get_Parent(This,retval)	\
    ( (This)->lpVtbl -> get_Parent(This,retval) ) 

#define IADsResource_get_Schema(This,retval)	\
    ( (This)->lpVtbl -> get_Schema(This,retval) ) 

#define IADsResource_GetInfo(This)	\
    ( (This)->lpVtbl -> GetInfo(This) ) 

#define IADsResource_SetInfo(This)	\
    ( (This)->lpVtbl -> SetInfo(This) ) 

#define IADsResource_Get(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> Get(This,bstrName,pvProp) ) 

#define IADsResource_Put(This,bstrName,vProp)	\
    ( (This)->lpVtbl -> Put(This,bstrName,vProp) ) 

#define IADsResource_GetEx(This,bstrName,pvProp)	\
    ( (This)->lpVtbl -> GetEx(This,bstrName,pvProp) ) 

#define IADsResource_PutEx(This,lnControlCode,bstrName,vProp)	\
    ( (This)->lpVtbl -> PutEx(This,lnControlCode,bstrName,vProp) ) 

#define IADsResource_GetInfoEx(This,vProperties,lnReserved)	\
    ( (This)->lpVtbl -> GetInfoEx(This,vProperties,lnReserved) ) 


#define IADsResource_get_User(This,retval)	\
    ( (This)->lpVtbl -> get_User(This,retval) ) 

#define IADsResource_get_UserPath(This,retval)	\
    ( (This)->lpVtbl -> get_UserPath(This,retval) ) 

#define IADsResource_get_Path(This,retval)	\
    ( (This)->lpVtbl -> get_Path(This,retval) ) 

#define IADsResource_get_LockCount(This,retval)	\
    ( (This)->lpVtbl -> get_LockCount(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsResource_INTERFACE_DEFINED__ */


#ifndef __IADsOpenDSObject_INTERFACE_DEFINED__
#define __IADsOpenDSObject_INTERFACE_DEFINED__

/* interface IADsOpenDSObject */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsOpenDSObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ddf2891e-0f9c-11d0-8ad4-00c04fd8d503")
    IADsOpenDSObject : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OpenDSObject( 
            /* [in] */ __RPC__in BSTR lpszDNName,
            /* [in] */ __RPC__in BSTR lpszUserName,
            /* [in] */ __RPC__in BSTR lpszPassword,
            /* [in] */ long lnReserved,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppOleDsObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsOpenDSObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsOpenDSObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsOpenDSObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsOpenDSObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsOpenDSObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsOpenDSObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsOpenDSObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsOpenDSObject * This,
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
        
        DECLSPEC_XFGVIRT(IADsOpenDSObject, OpenDSObject)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OpenDSObject )( 
            __RPC__in IADsOpenDSObject * This,
            /* [in] */ __RPC__in BSTR lpszDNName,
            /* [in] */ __RPC__in BSTR lpszUserName,
            /* [in] */ __RPC__in BSTR lpszPassword,
            /* [in] */ long lnReserved,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppOleDsObj);
        
        END_INTERFACE
    } IADsOpenDSObjectVtbl;

    interface IADsOpenDSObject
    {
        CONST_VTBL struct IADsOpenDSObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsOpenDSObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsOpenDSObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsOpenDSObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsOpenDSObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsOpenDSObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsOpenDSObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsOpenDSObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsOpenDSObject_OpenDSObject(This,lpszDNName,lpszUserName,lpszPassword,lnReserved,ppOleDsObj)	\
    ( (This)->lpVtbl -> OpenDSObject(This,lpszDNName,lpszUserName,lpszPassword,lnReserved,ppOleDsObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsOpenDSObject_INTERFACE_DEFINED__ */


#ifndef __IDirectoryObject_INTERFACE_DEFINED__
#define __IDirectoryObject_INTERFACE_DEFINED__

/* interface IDirectoryObject */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDirectoryObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e798de2c-22e4-11d0-84fe-00c04fd8d503")
    IDirectoryObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetObjectInformation( 
            /* [out] */ __RPC__deref_out_opt PADS_OBJECT_INFO *ppObjInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectAttributes( 
            /* [in] */ __RPC__deref_in_opt LPWSTR *pAttributeNames,
            /* [in] */ DWORD dwNumberAttributes,
            /* [out] */ __RPC__deref_out_opt PADS_ATTR_INFO *ppAttributeEntries,
            /* [out] */ __RPC__out DWORD *pdwNumAttributesReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetObjectAttributes( 
            /* [in] */ __RPC__in PADS_ATTR_INFO pAttributeEntries,
            /* [in] */ DWORD dwNumAttributes,
            /* [out] */ __RPC__out DWORD *pdwNumAttributesModified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDSObject( 
            /* [in] */ __RPC__in LPWSTR pszRDNName,
            /* [in] */ __RPC__in PADS_ATTR_INFO pAttributeEntries,
            /* [in] */ DWORD dwNumAttributes,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteDSObject( 
            /* [in] */ __RPC__in LPWSTR pszRDNName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectoryObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDirectoryObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDirectoryObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDirectoryObject * This);
        
        DECLSPEC_XFGVIRT(IDirectoryObject, GetObjectInformation)
        HRESULT ( STDMETHODCALLTYPE *GetObjectInformation )( 
            __RPC__in IDirectoryObject * This,
            /* [out] */ __RPC__deref_out_opt PADS_OBJECT_INFO *ppObjInfo);
        
        DECLSPEC_XFGVIRT(IDirectoryObject, GetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetObjectAttributes )( 
            __RPC__in IDirectoryObject * This,
            /* [in] */ __RPC__deref_in_opt LPWSTR *pAttributeNames,
            /* [in] */ DWORD dwNumberAttributes,
            /* [out] */ __RPC__deref_out_opt PADS_ATTR_INFO *ppAttributeEntries,
            /* [out] */ __RPC__out DWORD *pdwNumAttributesReturned);
        
        DECLSPEC_XFGVIRT(IDirectoryObject, SetObjectAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetObjectAttributes )( 
            __RPC__in IDirectoryObject * This,
            /* [in] */ __RPC__in PADS_ATTR_INFO pAttributeEntries,
            /* [in] */ DWORD dwNumAttributes,
            /* [out] */ __RPC__out DWORD *pdwNumAttributesModified);
        
        DECLSPEC_XFGVIRT(IDirectoryObject, CreateDSObject)
        HRESULT ( STDMETHODCALLTYPE *CreateDSObject )( 
            __RPC__in IDirectoryObject * This,
            /* [in] */ __RPC__in LPWSTR pszRDNName,
            /* [in] */ __RPC__in PADS_ATTR_INFO pAttributeEntries,
            /* [in] */ DWORD dwNumAttributes,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppObject);
        
        DECLSPEC_XFGVIRT(IDirectoryObject, DeleteDSObject)
        HRESULT ( STDMETHODCALLTYPE *DeleteDSObject )( 
            __RPC__in IDirectoryObject * This,
            /* [in] */ __RPC__in LPWSTR pszRDNName);
        
        END_INTERFACE
    } IDirectoryObjectVtbl;

    interface IDirectoryObject
    {
        CONST_VTBL struct IDirectoryObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectoryObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectoryObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectoryObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectoryObject_GetObjectInformation(This,ppObjInfo)	\
    ( (This)->lpVtbl -> GetObjectInformation(This,ppObjInfo) ) 

#define IDirectoryObject_GetObjectAttributes(This,pAttributeNames,dwNumberAttributes,ppAttributeEntries,pdwNumAttributesReturned)	\
    ( (This)->lpVtbl -> GetObjectAttributes(This,pAttributeNames,dwNumberAttributes,ppAttributeEntries,pdwNumAttributesReturned) ) 

#define IDirectoryObject_SetObjectAttributes(This,pAttributeEntries,dwNumAttributes,pdwNumAttributesModified)	\
    ( (This)->lpVtbl -> SetObjectAttributes(This,pAttributeEntries,dwNumAttributes,pdwNumAttributesModified) ) 

#define IDirectoryObject_CreateDSObject(This,pszRDNName,pAttributeEntries,dwNumAttributes,ppObject)	\
    ( (This)->lpVtbl -> CreateDSObject(This,pszRDNName,pAttributeEntries,dwNumAttributes,ppObject) ) 

#define IDirectoryObject_DeleteDSObject(This,pszRDNName)	\
    ( (This)->lpVtbl -> DeleteDSObject(This,pszRDNName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectoryObject_INTERFACE_DEFINED__ */


#ifndef __IDirectorySearch_INTERFACE_DEFINED__
#define __IDirectorySearch_INTERFACE_DEFINED__

/* interface IDirectorySearch */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDirectorySearch;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("109ba8ec-92f0-11d0-a790-00c04fd8d5a8")
    IDirectorySearch : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSearchPreference( 
            /* [in] */ __RPC__in PADS_SEARCHPREF_INFO pSearchPrefs,
            /* [in] */ DWORD dwNumPrefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExecuteSearch( 
            /* [in] */ __RPC__in LPWSTR pszSearchFilter,
            /* [in] */ __RPC__deref_in_opt LPWSTR *pAttributeNames,
            /* [in] */ DWORD dwNumberAttributes,
            /* [out] */ __RPC__deref_out_opt PADS_SEARCH_HANDLE phSearchResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AbandonSearch( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE phSearchResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFirstRow( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextRow( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPreviousRow( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextColumnName( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchHandle,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszColumnName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumn( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult,
            /* [in] */ __RPC__in LPWSTR szColumnName,
            /* [out] */ __RPC__out PADS_SEARCH_COLUMN pSearchColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeColumn( 
            /* [in] */ __RPC__in PADS_SEARCH_COLUMN pSearchColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseSearchHandle( 
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectorySearchVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDirectorySearch * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDirectorySearch * This);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, SetSearchPreference)
        HRESULT ( STDMETHODCALLTYPE *SetSearchPreference )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in PADS_SEARCHPREF_INFO pSearchPrefs,
            /* [in] */ DWORD dwNumPrefs);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, ExecuteSearch)
        HRESULT ( STDMETHODCALLTYPE *ExecuteSearch )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in LPWSTR pszSearchFilter,
            /* [in] */ __RPC__deref_in_opt LPWSTR *pAttributeNames,
            /* [in] */ DWORD dwNumberAttributes,
            /* [out] */ __RPC__deref_out_opt PADS_SEARCH_HANDLE phSearchResult);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, AbandonSearch)
        HRESULT ( STDMETHODCALLTYPE *AbandonSearch )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE phSearchResult);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, GetFirstRow)
        HRESULT ( STDMETHODCALLTYPE *GetFirstRow )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, GetNextRow)
        HRESULT ( STDMETHODCALLTYPE *GetNextRow )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, GetPreviousRow)
        HRESULT ( STDMETHODCALLTYPE *GetPreviousRow )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, GetNextColumnName)
        HRESULT ( STDMETHODCALLTYPE *GetNextColumnName )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchHandle,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszColumnName);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, GetColumn)
        HRESULT ( STDMETHODCALLTYPE *GetColumn )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult,
            /* [in] */ __RPC__in LPWSTR szColumnName,
            /* [out] */ __RPC__out PADS_SEARCH_COLUMN pSearchColumn);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, FreeColumn)
        HRESULT ( STDMETHODCALLTYPE *FreeColumn )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in PADS_SEARCH_COLUMN pSearchColumn);
        
        DECLSPEC_XFGVIRT(IDirectorySearch, CloseSearchHandle)
        HRESULT ( STDMETHODCALLTYPE *CloseSearchHandle )( 
            __RPC__in IDirectorySearch * This,
            /* [in] */ __RPC__in ADS_SEARCH_HANDLE hSearchResult);
        
        END_INTERFACE
    } IDirectorySearchVtbl;

    interface IDirectorySearch
    {
        CONST_VTBL struct IDirectorySearchVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectorySearch_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectorySearch_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectorySearch_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectorySearch_SetSearchPreference(This,pSearchPrefs,dwNumPrefs)	\
    ( (This)->lpVtbl -> SetSearchPreference(This,pSearchPrefs,dwNumPrefs) ) 

#define IDirectorySearch_ExecuteSearch(This,pszSearchFilter,pAttributeNames,dwNumberAttributes,phSearchResult)	\
    ( (This)->lpVtbl -> ExecuteSearch(This,pszSearchFilter,pAttributeNames,dwNumberAttributes,phSearchResult) ) 

#define IDirectorySearch_AbandonSearch(This,phSearchResult)	\
    ( (This)->lpVtbl -> AbandonSearch(This,phSearchResult) ) 

#define IDirectorySearch_GetFirstRow(This,hSearchResult)	\
    ( (This)->lpVtbl -> GetFirstRow(This,hSearchResult) ) 

#define IDirectorySearch_GetNextRow(This,hSearchResult)	\
    ( (This)->lpVtbl -> GetNextRow(This,hSearchResult) ) 

#define IDirectorySearch_GetPreviousRow(This,hSearchResult)	\
    ( (This)->lpVtbl -> GetPreviousRow(This,hSearchResult) ) 

#define IDirectorySearch_GetNextColumnName(This,hSearchHandle,ppszColumnName)	\
    ( (This)->lpVtbl -> GetNextColumnName(This,hSearchHandle,ppszColumnName) ) 

#define IDirectorySearch_GetColumn(This,hSearchResult,szColumnName,pSearchColumn)	\
    ( (This)->lpVtbl -> GetColumn(This,hSearchResult,szColumnName,pSearchColumn) ) 

#define IDirectorySearch_FreeColumn(This,pSearchColumn)	\
    ( (This)->lpVtbl -> FreeColumn(This,pSearchColumn) ) 

#define IDirectorySearch_CloseSearchHandle(This,hSearchResult)	\
    ( (This)->lpVtbl -> CloseSearchHandle(This,hSearchResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectorySearch_INTERFACE_DEFINED__ */


#ifndef __IDirectorySchemaMgmt_INTERFACE_DEFINED__
#define __IDirectorySchemaMgmt_INTERFACE_DEFINED__

/* interface IDirectorySchemaMgmt */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDirectorySchemaMgmt;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75db3b9c-a4d8-11d0-a79c-00c04fd8d5a8")
    IDirectorySchemaMgmt : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumAttributes( 
            __RPC__deref_in_opt LPWSTR *ppszAttrNames,
            DWORD dwNumAttributes,
            __RPC__deref_in_opt PADS_ATTR_DEF *ppAttrDefinition,
            __RPC__in DWORD *pdwNumAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateAttributeDefinition( 
            __RPC__in LPWSTR pszAttributeName,
            __RPC__in PADS_ATTR_DEF pAttributeDefinition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteAttributeDefinition( 
            __RPC__in LPWSTR pszAttributeName,
            __RPC__in PADS_ATTR_DEF pAttributeDefinition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteAttributeDefinition( 
            __RPC__in LPWSTR pszAttributeName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumClasses( 
            __RPC__deref_in_opt LPWSTR *ppszClassNames,
            DWORD dwNumClasses,
            __RPC__deref_in_opt PADS_CLASS_DEF *ppClassDefinition,
            __RPC__in DWORD *pdwNumClasses) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteClassDefinition( 
            __RPC__in LPWSTR pszClassName,
            __RPC__in PADS_CLASS_DEF pClassDefinition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateClassDefinition( 
            __RPC__in LPWSTR pszClassName,
            __RPC__in PADS_CLASS_DEF pClassDefinition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteClassDefinition( 
            __RPC__in LPWSTR pszClassName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectorySchemaMgmtVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDirectorySchemaMgmt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDirectorySchemaMgmt * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDirectorySchemaMgmt * This);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, EnumAttributes)
        HRESULT ( STDMETHODCALLTYPE *EnumAttributes )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__deref_in_opt LPWSTR *ppszAttrNames,
            DWORD dwNumAttributes,
            __RPC__deref_in_opt PADS_ATTR_DEF *ppAttrDefinition,
            __RPC__in DWORD *pdwNumAttributes);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, CreateAttributeDefinition)
        HRESULT ( STDMETHODCALLTYPE *CreateAttributeDefinition )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__in LPWSTR pszAttributeName,
            __RPC__in PADS_ATTR_DEF pAttributeDefinition);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, WriteAttributeDefinition)
        HRESULT ( STDMETHODCALLTYPE *WriteAttributeDefinition )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__in LPWSTR pszAttributeName,
            __RPC__in PADS_ATTR_DEF pAttributeDefinition);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, DeleteAttributeDefinition)
        HRESULT ( STDMETHODCALLTYPE *DeleteAttributeDefinition )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__in LPWSTR pszAttributeName);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, EnumClasses)
        HRESULT ( STDMETHODCALLTYPE *EnumClasses )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__deref_in_opt LPWSTR *ppszClassNames,
            DWORD dwNumClasses,
            __RPC__deref_in_opt PADS_CLASS_DEF *ppClassDefinition,
            __RPC__in DWORD *pdwNumClasses);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, WriteClassDefinition)
        HRESULT ( STDMETHODCALLTYPE *WriteClassDefinition )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__in LPWSTR pszClassName,
            __RPC__in PADS_CLASS_DEF pClassDefinition);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, CreateClassDefinition)
        HRESULT ( STDMETHODCALLTYPE *CreateClassDefinition )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__in LPWSTR pszClassName,
            __RPC__in PADS_CLASS_DEF pClassDefinition);
        
        DECLSPEC_XFGVIRT(IDirectorySchemaMgmt, DeleteClassDefinition)
        HRESULT ( STDMETHODCALLTYPE *DeleteClassDefinition )( 
            __RPC__in IDirectorySchemaMgmt * This,
            __RPC__in LPWSTR pszClassName);
        
        END_INTERFACE
    } IDirectorySchemaMgmtVtbl;

    interface IDirectorySchemaMgmt
    {
        CONST_VTBL struct IDirectorySchemaMgmtVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectorySchemaMgmt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectorySchemaMgmt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectorySchemaMgmt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectorySchemaMgmt_EnumAttributes(This,ppszAttrNames,dwNumAttributes,ppAttrDefinition,pdwNumAttributes)	\
    ( (This)->lpVtbl -> EnumAttributes(This,ppszAttrNames,dwNumAttributes,ppAttrDefinition,pdwNumAttributes) ) 

#define IDirectorySchemaMgmt_CreateAttributeDefinition(This,pszAttributeName,pAttributeDefinition)	\
    ( (This)->lpVtbl -> CreateAttributeDefinition(This,pszAttributeName,pAttributeDefinition) ) 

#define IDirectorySchemaMgmt_WriteAttributeDefinition(This,pszAttributeName,pAttributeDefinition)	\
    ( (This)->lpVtbl -> WriteAttributeDefinition(This,pszAttributeName,pAttributeDefinition) ) 

#define IDirectorySchemaMgmt_DeleteAttributeDefinition(This,pszAttributeName)	\
    ( (This)->lpVtbl -> DeleteAttributeDefinition(This,pszAttributeName) ) 

#define IDirectorySchemaMgmt_EnumClasses(This,ppszClassNames,dwNumClasses,ppClassDefinition,pdwNumClasses)	\
    ( (This)->lpVtbl -> EnumClasses(This,ppszClassNames,dwNumClasses,ppClassDefinition,pdwNumClasses) ) 

#define IDirectorySchemaMgmt_WriteClassDefinition(This,pszClassName,pClassDefinition)	\
    ( (This)->lpVtbl -> WriteClassDefinition(This,pszClassName,pClassDefinition) ) 

#define IDirectorySchemaMgmt_CreateClassDefinition(This,pszClassName,pClassDefinition)	\
    ( (This)->lpVtbl -> CreateClassDefinition(This,pszClassName,pClassDefinition) ) 

#define IDirectorySchemaMgmt_DeleteClassDefinition(This,pszClassName)	\
    ( (This)->lpVtbl -> DeleteClassDefinition(This,pszClassName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectorySchemaMgmt_INTERFACE_DEFINED__ */


#ifndef __IADsAggregatee_INTERFACE_DEFINED__
#define __IADsAggregatee_INTERFACE_DEFINED__

/* interface IADsAggregatee */
/* [object][uuid] */ 


EXTERN_C const IID IID_IADsAggregatee;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1346ce8c-9039-11d0-8528-00c04fd8d503")
    IADsAggregatee : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConnectAsAggregatee( 
            __RPC__in_opt IUnknown *pOuterUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectAsAggregatee( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RelinquishInterface( 
            __RPC__in REFIID riid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestoreInterface( 
            __RPC__in REFIID riid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsAggregateeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsAggregatee * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsAggregatee * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsAggregatee * This);
        
        DECLSPEC_XFGVIRT(IADsAggregatee, ConnectAsAggregatee)
        HRESULT ( STDMETHODCALLTYPE *ConnectAsAggregatee )( 
            __RPC__in IADsAggregatee * This,
            __RPC__in_opt IUnknown *pOuterUnknown);
        
        DECLSPEC_XFGVIRT(IADsAggregatee, DisconnectAsAggregatee)
        HRESULT ( STDMETHODCALLTYPE *DisconnectAsAggregatee )( 
            __RPC__in IADsAggregatee * This);
        
        DECLSPEC_XFGVIRT(IADsAggregatee, RelinquishInterface)
        HRESULT ( STDMETHODCALLTYPE *RelinquishInterface )( 
            __RPC__in IADsAggregatee * This,
            __RPC__in REFIID riid);
        
        DECLSPEC_XFGVIRT(IADsAggregatee, RestoreInterface)
        HRESULT ( STDMETHODCALLTYPE *RestoreInterface )( 
            __RPC__in IADsAggregatee * This,
            __RPC__in REFIID riid);
        
        END_INTERFACE
    } IADsAggregateeVtbl;

    interface IADsAggregatee
    {
        CONST_VTBL struct IADsAggregateeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsAggregatee_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsAggregatee_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsAggregatee_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsAggregatee_ConnectAsAggregatee(This,pOuterUnknown)	\
    ( (This)->lpVtbl -> ConnectAsAggregatee(This,pOuterUnknown) ) 

#define IADsAggregatee_DisconnectAsAggregatee(This)	\
    ( (This)->lpVtbl -> DisconnectAsAggregatee(This) ) 

#define IADsAggregatee_RelinquishInterface(This,riid)	\
    ( (This)->lpVtbl -> RelinquishInterface(This,riid) ) 

#define IADsAggregatee_RestoreInterface(This,riid)	\
    ( (This)->lpVtbl -> RestoreInterface(This,riid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsAggregatee_INTERFACE_DEFINED__ */


#ifndef __IADsAggregator_INTERFACE_DEFINED__
#define __IADsAggregator_INTERFACE_DEFINED__

/* interface IADsAggregator */
/* [object][uuid] */ 


EXTERN_C const IID IID_IADsAggregator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52db5fb0-941f-11d0-8529-00c04fd8d503")
    IADsAggregator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConnectAsAggregator( 
            __RPC__in_opt IUnknown *pAggregatee) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectAsAggregator( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsAggregatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsAggregator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsAggregator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsAggregator * This);
        
        DECLSPEC_XFGVIRT(IADsAggregator, ConnectAsAggregator)
        HRESULT ( STDMETHODCALLTYPE *ConnectAsAggregator )( 
            __RPC__in IADsAggregator * This,
            __RPC__in_opt IUnknown *pAggregatee);
        
        DECLSPEC_XFGVIRT(IADsAggregator, DisconnectAsAggregator)
        HRESULT ( STDMETHODCALLTYPE *DisconnectAsAggregator )( 
            __RPC__in IADsAggregator * This);
        
        END_INTERFACE
    } IADsAggregatorVtbl;

    interface IADsAggregator
    {
        CONST_VTBL struct IADsAggregatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsAggregator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsAggregator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsAggregator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsAggregator_ConnectAsAggregator(This,pAggregatee)	\
    ( (This)->lpVtbl -> ConnectAsAggregator(This,pAggregatee) ) 

#define IADsAggregator_DisconnectAsAggregator(This)	\
    ( (This)->lpVtbl -> DisconnectAsAggregator(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsAggregator_INTERFACE_DEFINED__ */


#ifndef __IADsAccessControlEntry_INTERFACE_DEFINED__
#define __IADsAccessControlEntry_INTERFACE_DEFINED__

/* interface IADsAccessControlEntry */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsAccessControlEntry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b4f3a14c-9bdd-11d0-852c-00c04fd8d503")
    IADsAccessControlEntry : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AccessMask( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AccessMask( 
            /* [in] */ long lnAccessMask) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AceType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AceType( 
            /* [in] */ long lnAceType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AceFlags( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AceFlags( 
            /* [in] */ long lnAceFlags) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Flags( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Flags( 
            /* [in] */ long lnFlags) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ObjectType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ObjectType( 
            /* [in] */ __RPC__in BSTR bstrObjectType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_InheritedObjectType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_InheritedObjectType( 
            /* [in] */ __RPC__in BSTR bstrInheritedObjectType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Trustee( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Trustee( 
            /* [in] */ __RPC__in BSTR bstrTrustee) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsAccessControlEntryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsAccessControlEntry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsAccessControlEntry * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsAccessControlEntry * This,
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
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_AccessMask)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AccessMask )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_AccessMask)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AccessMask )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ long lnAccessMask);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_AceType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AceType )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_AceType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AceType )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ long lnAceType);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_AceFlags)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AceFlags )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_AceFlags)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AceFlags )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ long lnAceFlags);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_Flags)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_Flags)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Flags )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ long lnFlags);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_ObjectType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectType )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_ObjectType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ObjectType )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ __RPC__in BSTR bstrObjectType);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_InheritedObjectType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InheritedObjectType )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_InheritedObjectType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InheritedObjectType )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ __RPC__in BSTR bstrInheritedObjectType);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, get_Trustee)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Trustee )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlEntry, put_Trustee)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Trustee )( 
            __RPC__in IADsAccessControlEntry * This,
            /* [in] */ __RPC__in BSTR bstrTrustee);
        
        END_INTERFACE
    } IADsAccessControlEntryVtbl;

    interface IADsAccessControlEntry
    {
        CONST_VTBL struct IADsAccessControlEntryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsAccessControlEntry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsAccessControlEntry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsAccessControlEntry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsAccessControlEntry_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsAccessControlEntry_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsAccessControlEntry_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsAccessControlEntry_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsAccessControlEntry_get_AccessMask(This,retval)	\
    ( (This)->lpVtbl -> get_AccessMask(This,retval) ) 

#define IADsAccessControlEntry_put_AccessMask(This,lnAccessMask)	\
    ( (This)->lpVtbl -> put_AccessMask(This,lnAccessMask) ) 

#define IADsAccessControlEntry_get_AceType(This,retval)	\
    ( (This)->lpVtbl -> get_AceType(This,retval) ) 

#define IADsAccessControlEntry_put_AceType(This,lnAceType)	\
    ( (This)->lpVtbl -> put_AceType(This,lnAceType) ) 

#define IADsAccessControlEntry_get_AceFlags(This,retval)	\
    ( (This)->lpVtbl -> get_AceFlags(This,retval) ) 

#define IADsAccessControlEntry_put_AceFlags(This,lnAceFlags)	\
    ( (This)->lpVtbl -> put_AceFlags(This,lnAceFlags) ) 

#define IADsAccessControlEntry_get_Flags(This,retval)	\
    ( (This)->lpVtbl -> get_Flags(This,retval) ) 

#define IADsAccessControlEntry_put_Flags(This,lnFlags)	\
    ( (This)->lpVtbl -> put_Flags(This,lnFlags) ) 

#define IADsAccessControlEntry_get_ObjectType(This,retval)	\
    ( (This)->lpVtbl -> get_ObjectType(This,retval) ) 

#define IADsAccessControlEntry_put_ObjectType(This,bstrObjectType)	\
    ( (This)->lpVtbl -> put_ObjectType(This,bstrObjectType) ) 

#define IADsAccessControlEntry_get_InheritedObjectType(This,retval)	\
    ( (This)->lpVtbl -> get_InheritedObjectType(This,retval) ) 

#define IADsAccessControlEntry_put_InheritedObjectType(This,bstrInheritedObjectType)	\
    ( (This)->lpVtbl -> put_InheritedObjectType(This,bstrInheritedObjectType) ) 

#define IADsAccessControlEntry_get_Trustee(This,retval)	\
    ( (This)->lpVtbl -> get_Trustee(This,retval) ) 

#define IADsAccessControlEntry_put_Trustee(This,bstrTrustee)	\
    ( (This)->lpVtbl -> put_Trustee(This,bstrTrustee) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsAccessControlEntry_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_AccessControlEntry;

#ifdef __cplusplus

class DECLSPEC_UUID("b75ac000-9bdd-11d0-852c-00c04fd8d503")
AccessControlEntry;
#endif

#ifndef __IADsAccessControlList_INTERFACE_DEFINED__
#define __IADsAccessControlList_INTERFACE_DEFINED__

/* interface IADsAccessControlList */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsAccessControlList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b7ee91cc-9bdd-11d0-852c-00c04fd8d503")
    IADsAccessControlList : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AclRevision( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AclRevision( 
            /* [in] */ long lnAclRevision) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AceCount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AceCount( 
            /* [in] */ long lnAceCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddAce( 
            /* [in] */ __RPC__in_opt IDispatch *pAccessControlEntry) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RemoveAce( 
            /* [in] */ __RPC__in_opt IDispatch *pAccessControlEntry) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CopyAccessList( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppAccessControlList) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsAccessControlListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsAccessControlList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsAccessControlList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsAccessControlList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsAccessControlList * This,
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
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, get_AclRevision)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AclRevision )( 
            __RPC__in IADsAccessControlList * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, put_AclRevision)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AclRevision )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ long lnAclRevision);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, get_AceCount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AceCount )( 
            __RPC__in IADsAccessControlList * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, put_AceCount)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AceCount )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ long lnAceCount);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, AddAce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddAce )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ __RPC__in_opt IDispatch *pAccessControlEntry);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, RemoveAce)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RemoveAce )( 
            __RPC__in IADsAccessControlList * This,
            /* [in] */ __RPC__in_opt IDispatch *pAccessControlEntry);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, CopyAccessList)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CopyAccessList )( 
            __RPC__in IADsAccessControlList * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppAccessControlList);
        
        DECLSPEC_XFGVIRT(IADsAccessControlList, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IADsAccessControlList * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        END_INTERFACE
    } IADsAccessControlListVtbl;

    interface IADsAccessControlList
    {
        CONST_VTBL struct IADsAccessControlListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsAccessControlList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsAccessControlList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsAccessControlList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsAccessControlList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsAccessControlList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsAccessControlList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsAccessControlList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsAccessControlList_get_AclRevision(This,retval)	\
    ( (This)->lpVtbl -> get_AclRevision(This,retval) ) 

#define IADsAccessControlList_put_AclRevision(This,lnAclRevision)	\
    ( (This)->lpVtbl -> put_AclRevision(This,lnAclRevision) ) 

#define IADsAccessControlList_get_AceCount(This,retval)	\
    ( (This)->lpVtbl -> get_AceCount(This,retval) ) 

#define IADsAccessControlList_put_AceCount(This,lnAceCount)	\
    ( (This)->lpVtbl -> put_AceCount(This,lnAceCount) ) 

#define IADsAccessControlList_AddAce(This,pAccessControlEntry)	\
    ( (This)->lpVtbl -> AddAce(This,pAccessControlEntry) ) 

#define IADsAccessControlList_RemoveAce(This,pAccessControlEntry)	\
    ( (This)->lpVtbl -> RemoveAce(This,pAccessControlEntry) ) 

#define IADsAccessControlList_CopyAccessList(This,ppAccessControlList)	\
    ( (This)->lpVtbl -> CopyAccessList(This,ppAccessControlList) ) 

#define IADsAccessControlList_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsAccessControlList_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_AccessControlList;

#ifdef __cplusplus

class DECLSPEC_UUID("b85ea052-9bdd-11d0-852c-00c04fd8d503")
AccessControlList;
#endif

#ifndef __IADsSecurityDescriptor_INTERFACE_DEFINED__
#define __IADsSecurityDescriptor_INTERFACE_DEFINED__

/* interface IADsSecurityDescriptor */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsSecurityDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b8c787ca-9bdd-11d0-852c-00c04fd8d503")
    IADsSecurityDescriptor : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Revision( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Revision( 
            /* [in] */ long lnRevision) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Control( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Control( 
            /* [in] */ long lnControl) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Owner( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Owner( 
            /* [in] */ __RPC__in BSTR bstrOwner) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OwnerDefaulted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OwnerDefaulted( 
            /* [in] */ VARIANT_BOOL fOwnerDefaulted) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Group( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Group( 
            /* [in] */ __RPC__in BSTR bstrGroup) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_GroupDefaulted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_GroupDefaulted( 
            /* [in] */ VARIANT_BOOL fGroupDefaulted) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscretionaryAcl( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscretionaryAcl( 
            /* [in] */ __RPC__in_opt IDispatch *pDiscretionaryAcl) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DaclDefaulted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DaclDefaulted( 
            /* [in] */ VARIANT_BOOL fDaclDefaulted) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SystemAcl( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SystemAcl( 
            /* [in] */ __RPC__in_opt IDispatch *pSystemAcl) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SaclDefaulted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SaclDefaulted( 
            /* [in] */ VARIANT_BOOL fSaclDefaulted) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CopySecurityDescriptor( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppSecurityDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsSecurityDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsSecurityDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsSecurityDescriptor * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsSecurityDescriptor * This,
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
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_Revision)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Revision )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_Revision)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Revision )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ long lnRevision);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_Control)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Control )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_Control)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Control )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ long lnControl);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_Owner)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Owner )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_Owner)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Owner )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ __RPC__in BSTR bstrOwner);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_OwnerDefaulted)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_OwnerDefaulted)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OwnerDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ VARIANT_BOOL fOwnerDefaulted);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_Group)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Group )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_Group)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Group )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ __RPC__in BSTR bstrGroup);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_GroupDefaulted)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_GroupDefaulted)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GroupDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ VARIANT_BOOL fGroupDefaulted);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_DiscretionaryAcl)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscretionaryAcl )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_DiscretionaryAcl)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscretionaryAcl )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ __RPC__in_opt IDispatch *pDiscretionaryAcl);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_DaclDefaulted)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DaclDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_DaclDefaulted)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DaclDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ VARIANT_BOOL fDaclDefaulted);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_SystemAcl)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemAcl )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_SystemAcl)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SystemAcl )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ __RPC__in_opt IDispatch *pSystemAcl);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, get_SaclDefaulted)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SaclDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, put_SaclDefaulted)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SaclDefaulted )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [in] */ VARIANT_BOOL fSaclDefaulted);
        
        DECLSPEC_XFGVIRT(IADsSecurityDescriptor, CopySecurityDescriptor)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CopySecurityDescriptor )( 
            __RPC__in IADsSecurityDescriptor * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppSecurityDescriptor);
        
        END_INTERFACE
    } IADsSecurityDescriptorVtbl;

    interface IADsSecurityDescriptor
    {
        CONST_VTBL struct IADsSecurityDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsSecurityDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsSecurityDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsSecurityDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsSecurityDescriptor_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsSecurityDescriptor_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsSecurityDescriptor_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsSecurityDescriptor_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsSecurityDescriptor_get_Revision(This,retval)	\
    ( (This)->lpVtbl -> get_Revision(This,retval) ) 

#define IADsSecurityDescriptor_put_Revision(This,lnRevision)	\
    ( (This)->lpVtbl -> put_Revision(This,lnRevision) ) 

#define IADsSecurityDescriptor_get_Control(This,retval)	\
    ( (This)->lpVtbl -> get_Control(This,retval) ) 

#define IADsSecurityDescriptor_put_Control(This,lnControl)	\
    ( (This)->lpVtbl -> put_Control(This,lnControl) ) 

#define IADsSecurityDescriptor_get_Owner(This,retval)	\
    ( (This)->lpVtbl -> get_Owner(This,retval) ) 

#define IADsSecurityDescriptor_put_Owner(This,bstrOwner)	\
    ( (This)->lpVtbl -> put_Owner(This,bstrOwner) ) 

#define IADsSecurityDescriptor_get_OwnerDefaulted(This,retval)	\
    ( (This)->lpVtbl -> get_OwnerDefaulted(This,retval) ) 

#define IADsSecurityDescriptor_put_OwnerDefaulted(This,fOwnerDefaulted)	\
    ( (This)->lpVtbl -> put_OwnerDefaulted(This,fOwnerDefaulted) ) 

#define IADsSecurityDescriptor_get_Group(This,retval)	\
    ( (This)->lpVtbl -> get_Group(This,retval) ) 

#define IADsSecurityDescriptor_put_Group(This,bstrGroup)	\
    ( (This)->lpVtbl -> put_Group(This,bstrGroup) ) 

#define IADsSecurityDescriptor_get_GroupDefaulted(This,retval)	\
    ( (This)->lpVtbl -> get_GroupDefaulted(This,retval) ) 

#define IADsSecurityDescriptor_put_GroupDefaulted(This,fGroupDefaulted)	\
    ( (This)->lpVtbl -> put_GroupDefaulted(This,fGroupDefaulted) ) 

#define IADsSecurityDescriptor_get_DiscretionaryAcl(This,retval)	\
    ( (This)->lpVtbl -> get_DiscretionaryAcl(This,retval) ) 

#define IADsSecurityDescriptor_put_DiscretionaryAcl(This,pDiscretionaryAcl)	\
    ( (This)->lpVtbl -> put_DiscretionaryAcl(This,pDiscretionaryAcl) ) 

#define IADsSecurityDescriptor_get_DaclDefaulted(This,retval)	\
    ( (This)->lpVtbl -> get_DaclDefaulted(This,retval) ) 

#define IADsSecurityDescriptor_put_DaclDefaulted(This,fDaclDefaulted)	\
    ( (This)->lpVtbl -> put_DaclDefaulted(This,fDaclDefaulted) ) 

#define IADsSecurityDescriptor_get_SystemAcl(This,retval)	\
    ( (This)->lpVtbl -> get_SystemAcl(This,retval) ) 

#define IADsSecurityDescriptor_put_SystemAcl(This,pSystemAcl)	\
    ( (This)->lpVtbl -> put_SystemAcl(This,pSystemAcl) ) 

#define IADsSecurityDescriptor_get_SaclDefaulted(This,retval)	\
    ( (This)->lpVtbl -> get_SaclDefaulted(This,retval) ) 

#define IADsSecurityDescriptor_put_SaclDefaulted(This,fSaclDefaulted)	\
    ( (This)->lpVtbl -> put_SaclDefaulted(This,fSaclDefaulted) ) 

#define IADsSecurityDescriptor_CopySecurityDescriptor(This,ppSecurityDescriptor)	\
    ( (This)->lpVtbl -> CopySecurityDescriptor(This,ppSecurityDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsSecurityDescriptor_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_SecurityDescriptor;

#ifdef __cplusplus

class DECLSPEC_UUID("b958f73c-9bdd-11d0-852c-00c04fd8d503")
SecurityDescriptor;
#endif

#ifndef __IADsLargeInteger_INTERFACE_DEFINED__
#define __IADsLargeInteger_INTERFACE_DEFINED__

/* interface IADsLargeInteger */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsLargeInteger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9068270b-0939-11d1-8be1-00c04fd8d503")
    IADsLargeInteger : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HighPart( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_HighPart( 
            /* [in] */ long lnHighPart) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_LowPart( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_LowPart( 
            /* [in] */ long lnLowPart) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsLargeIntegerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsLargeInteger * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsLargeInteger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsLargeInteger * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsLargeInteger * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsLargeInteger * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsLargeInteger * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsLargeInteger * This,
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
        
        DECLSPEC_XFGVIRT(IADsLargeInteger, get_HighPart)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HighPart )( 
            __RPC__in IADsLargeInteger * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsLargeInteger, put_HighPart)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HighPart )( 
            __RPC__in IADsLargeInteger * This,
            /* [in] */ long lnHighPart);
        
        DECLSPEC_XFGVIRT(IADsLargeInteger, get_LowPart)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LowPart )( 
            __RPC__in IADsLargeInteger * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsLargeInteger, put_LowPart)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LowPart )( 
            __RPC__in IADsLargeInteger * This,
            /* [in] */ long lnLowPart);
        
        END_INTERFACE
    } IADsLargeIntegerVtbl;

    interface IADsLargeInteger
    {
        CONST_VTBL struct IADsLargeIntegerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsLargeInteger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsLargeInteger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsLargeInteger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsLargeInteger_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsLargeInteger_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsLargeInteger_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsLargeInteger_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsLargeInteger_get_HighPart(This,retval)	\
    ( (This)->lpVtbl -> get_HighPart(This,retval) ) 

#define IADsLargeInteger_put_HighPart(This,lnHighPart)	\
    ( (This)->lpVtbl -> put_HighPart(This,lnHighPart) ) 

#define IADsLargeInteger_get_LowPart(This,retval)	\
    ( (This)->lpVtbl -> get_LowPart(This,retval) ) 

#define IADsLargeInteger_put_LowPart(This,lnLowPart)	\
    ( (This)->lpVtbl -> put_LowPart(This,lnLowPart) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsLargeInteger_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_LargeInteger;

#ifdef __cplusplus

class DECLSPEC_UUID("927971f5-0939-11d1-8be1-00c04fd8d503")
LargeInteger;
#endif

#ifndef __IADsNameTranslate_INTERFACE_DEFINED__
#define __IADsNameTranslate_INTERFACE_DEFINED__

/* interface IADsNameTranslate */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsNameTranslate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b1b272a3-3625-11d1-a3a4-00c04fb950dc")
    IADsNameTranslate : public IDispatch
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ChaseReferral( 
            /* [in] */ long lnChaseReferral) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ long lnSetType,
            /* [in] */ __RPC__in BSTR bstrADsPath) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE InitEx( 
            /* [in] */ long lnSetType,
            /* [in] */ __RPC__in BSTR bstrADsPath,
            /* [in] */ __RPC__in BSTR bstrUserID,
            /* [in] */ __RPC__in BSTR bstrDomain,
            /* [in] */ __RPC__in BSTR bstrPassword) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Set( 
            /* [in] */ long lnSetType,
            /* [in] */ __RPC__in BSTR bstrADsPath) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ long lnFormatType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetEx( 
            /* [in] */ long lnFormatType,
            /* [in] */ VARIANT pvar) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEx( 
            /* [in] */ long lnFormatType,
            /* [retval][out] */ __RPC__out VARIANT *pvar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsNameTranslateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsNameTranslate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsNameTranslate * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsNameTranslate * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsNameTranslate * This,
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
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, put_ChaseReferral)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ChaseReferral )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnChaseReferral);
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, Init)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnSetType,
            /* [in] */ __RPC__in BSTR bstrADsPath);
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, InitEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *InitEx )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnSetType,
            /* [in] */ __RPC__in BSTR bstrADsPath,
            /* [in] */ __RPC__in BSTR bstrUserID,
            /* [in] */ __RPC__in BSTR bstrDomain,
            /* [in] */ __RPC__in BSTR bstrPassword);
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, Set)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Set )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnSetType,
            /* [in] */ __RPC__in BSTR bstrADsPath);
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, Get)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnFormatType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath);
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, SetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetEx )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnFormatType,
            /* [in] */ VARIANT pvar);
        
        DECLSPEC_XFGVIRT(IADsNameTranslate, GetEx)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEx )( 
            __RPC__in IADsNameTranslate * This,
            /* [in] */ long lnFormatType,
            /* [retval][out] */ __RPC__out VARIANT *pvar);
        
        END_INTERFACE
    } IADsNameTranslateVtbl;

    interface IADsNameTranslate
    {
        CONST_VTBL struct IADsNameTranslateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsNameTranslate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsNameTranslate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsNameTranslate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsNameTranslate_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsNameTranslate_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsNameTranslate_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsNameTranslate_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsNameTranslate_put_ChaseReferral(This,lnChaseReferral)	\
    ( (This)->lpVtbl -> put_ChaseReferral(This,lnChaseReferral) ) 

#define IADsNameTranslate_Init(This,lnSetType,bstrADsPath)	\
    ( (This)->lpVtbl -> Init(This,lnSetType,bstrADsPath) ) 

#define IADsNameTranslate_InitEx(This,lnSetType,bstrADsPath,bstrUserID,bstrDomain,bstrPassword)	\
    ( (This)->lpVtbl -> InitEx(This,lnSetType,bstrADsPath,bstrUserID,bstrDomain,bstrPassword) ) 

#define IADsNameTranslate_Set(This,lnSetType,bstrADsPath)	\
    ( (This)->lpVtbl -> Set(This,lnSetType,bstrADsPath) ) 

#define IADsNameTranslate_Get(This,lnFormatType,pbstrADsPath)	\
    ( (This)->lpVtbl -> Get(This,lnFormatType,pbstrADsPath) ) 

#define IADsNameTranslate_SetEx(This,lnFormatType,pvar)	\
    ( (This)->lpVtbl -> SetEx(This,lnFormatType,pvar) ) 

#define IADsNameTranslate_GetEx(This,lnFormatType,pvar)	\
    ( (This)->lpVtbl -> GetEx(This,lnFormatType,pvar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsNameTranslate_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_NameTranslate;

#ifdef __cplusplus

class DECLSPEC_UUID("274fae1f-3626-11d1-a3a4-00c04fb950dc")
NameTranslate;
#endif

#ifndef __IADsCaseIgnoreList_INTERFACE_DEFINED__
#define __IADsCaseIgnoreList_INTERFACE_DEFINED__

/* interface IADsCaseIgnoreList */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsCaseIgnoreList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7b66b533-4680-11d1-a3b4-00c04fb950dc")
    IADsCaseIgnoreList : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_CaseIgnoreList( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_CaseIgnoreList( 
            /* [in] */ VARIANT vCaseIgnoreList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsCaseIgnoreListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsCaseIgnoreList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsCaseIgnoreList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsCaseIgnoreList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsCaseIgnoreList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsCaseIgnoreList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsCaseIgnoreList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsCaseIgnoreList * This,
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
        
        DECLSPEC_XFGVIRT(IADsCaseIgnoreList, get_CaseIgnoreList)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CaseIgnoreList )( 
            __RPC__in IADsCaseIgnoreList * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsCaseIgnoreList, put_CaseIgnoreList)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CaseIgnoreList )( 
            __RPC__in IADsCaseIgnoreList * This,
            /* [in] */ VARIANT vCaseIgnoreList);
        
        END_INTERFACE
    } IADsCaseIgnoreListVtbl;

    interface IADsCaseIgnoreList
    {
        CONST_VTBL struct IADsCaseIgnoreListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsCaseIgnoreList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsCaseIgnoreList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsCaseIgnoreList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsCaseIgnoreList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsCaseIgnoreList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsCaseIgnoreList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsCaseIgnoreList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsCaseIgnoreList_get_CaseIgnoreList(This,retval)	\
    ( (This)->lpVtbl -> get_CaseIgnoreList(This,retval) ) 

#define IADsCaseIgnoreList_put_CaseIgnoreList(This,vCaseIgnoreList)	\
    ( (This)->lpVtbl -> put_CaseIgnoreList(This,vCaseIgnoreList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsCaseIgnoreList_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_CaseIgnoreList;

#ifdef __cplusplus

class DECLSPEC_UUID("15f88a55-4680-11d1-a3b4-00c04fb950dc")
CaseIgnoreList;
#endif

#ifndef __IADsFaxNumber_INTERFACE_DEFINED__
#define __IADsFaxNumber_INTERFACE_DEFINED__

/* interface IADsFaxNumber */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsFaxNumber;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a910dea9-4680-11d1-a3b4-00c04fb950dc")
    IADsFaxNumber : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_TelephoneNumber( 
            /* [in] */ __RPC__in BSTR bstrTelephoneNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Parameters( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Parameters( 
            /* [in] */ VARIANT vParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsFaxNumberVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsFaxNumber * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsFaxNumber * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsFaxNumber * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsFaxNumber * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsFaxNumber * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsFaxNumber * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsFaxNumber * This,
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
        
        DECLSPEC_XFGVIRT(IADsFaxNumber, get_TelephoneNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneNumber )( 
            __RPC__in IADsFaxNumber * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsFaxNumber, put_TelephoneNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TelephoneNumber )( 
            __RPC__in IADsFaxNumber * This,
            /* [in] */ __RPC__in BSTR bstrTelephoneNumber);
        
        DECLSPEC_XFGVIRT(IADsFaxNumber, get_Parameters)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parameters )( 
            __RPC__in IADsFaxNumber * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsFaxNumber, put_Parameters)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Parameters )( 
            __RPC__in IADsFaxNumber * This,
            /* [in] */ VARIANT vParameters);
        
        END_INTERFACE
    } IADsFaxNumberVtbl;

    interface IADsFaxNumber
    {
        CONST_VTBL struct IADsFaxNumberVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsFaxNumber_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsFaxNumber_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsFaxNumber_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsFaxNumber_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsFaxNumber_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsFaxNumber_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsFaxNumber_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsFaxNumber_get_TelephoneNumber(This,retval)	\
    ( (This)->lpVtbl -> get_TelephoneNumber(This,retval) ) 

#define IADsFaxNumber_put_TelephoneNumber(This,bstrTelephoneNumber)	\
    ( (This)->lpVtbl -> put_TelephoneNumber(This,bstrTelephoneNumber) ) 

#define IADsFaxNumber_get_Parameters(This,retval)	\
    ( (This)->lpVtbl -> get_Parameters(This,retval) ) 

#define IADsFaxNumber_put_Parameters(This,vParameters)	\
    ( (This)->lpVtbl -> put_Parameters(This,vParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsFaxNumber_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_FaxNumber;

#ifdef __cplusplus

class DECLSPEC_UUID("a5062215-4681-11d1-a3b4-00c04fb950dc")
FaxNumber;
#endif

#ifndef __IADsNetAddress_INTERFACE_DEFINED__
#define __IADsNetAddress_INTERFACE_DEFINED__

/* interface IADsNetAddress */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsNetAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b21a50a9-4080-11d1-a3ac-00c04fb950dc")
    IADsNetAddress : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_AddressType( 
            /* [in] */ long lnAddressType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Address( 
            /* [in] */ VARIANT vAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsNetAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsNetAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsNetAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsNetAddress * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsNetAddress * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsNetAddress * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsNetAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsNetAddress * This,
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
        
        DECLSPEC_XFGVIRT(IADsNetAddress, get_AddressType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressType )( 
            __RPC__in IADsNetAddress * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsNetAddress, put_AddressType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AddressType )( 
            __RPC__in IADsNetAddress * This,
            /* [in] */ long lnAddressType);
        
        DECLSPEC_XFGVIRT(IADsNetAddress, get_Address)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in IADsNetAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsNetAddress, put_Address)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Address )( 
            __RPC__in IADsNetAddress * This,
            /* [in] */ VARIANT vAddress);
        
        END_INTERFACE
    } IADsNetAddressVtbl;

    interface IADsNetAddress
    {
        CONST_VTBL struct IADsNetAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsNetAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsNetAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsNetAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsNetAddress_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsNetAddress_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsNetAddress_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsNetAddress_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsNetAddress_get_AddressType(This,retval)	\
    ( (This)->lpVtbl -> get_AddressType(This,retval) ) 

#define IADsNetAddress_put_AddressType(This,lnAddressType)	\
    ( (This)->lpVtbl -> put_AddressType(This,lnAddressType) ) 

#define IADsNetAddress_get_Address(This,retval)	\
    ( (This)->lpVtbl -> get_Address(This,retval) ) 

#define IADsNetAddress_put_Address(This,vAddress)	\
    ( (This)->lpVtbl -> put_Address(This,vAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsNetAddress_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_NetAddress;

#ifdef __cplusplus

class DECLSPEC_UUID("b0b71247-4080-11d1-a3ac-00c04fb950dc")
NetAddress;
#endif

#ifndef __IADsOctetList_INTERFACE_DEFINED__
#define __IADsOctetList_INTERFACE_DEFINED__

/* interface IADsOctetList */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsOctetList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7b28b80f-4680-11d1-a3b4-00c04fb950dc")
    IADsOctetList : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OctetList( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_OctetList( 
            /* [in] */ VARIANT vOctetList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsOctetListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsOctetList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsOctetList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsOctetList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsOctetList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsOctetList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsOctetList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsOctetList * This,
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
        
        DECLSPEC_XFGVIRT(IADsOctetList, get_OctetList)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OctetList )( 
            __RPC__in IADsOctetList * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsOctetList, put_OctetList)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OctetList )( 
            __RPC__in IADsOctetList * This,
            /* [in] */ VARIANT vOctetList);
        
        END_INTERFACE
    } IADsOctetListVtbl;

    interface IADsOctetList
    {
        CONST_VTBL struct IADsOctetListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsOctetList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsOctetList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsOctetList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsOctetList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsOctetList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsOctetList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsOctetList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsOctetList_get_OctetList(This,retval)	\
    ( (This)->lpVtbl -> get_OctetList(This,retval) ) 

#define IADsOctetList_put_OctetList(This,vOctetList)	\
    ( (This)->lpVtbl -> put_OctetList(This,vOctetList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsOctetList_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_OctetList;

#ifdef __cplusplus

class DECLSPEC_UUID("1241400f-4680-11d1-a3b4-00c04fb950dc")
OctetList;
#endif

#ifndef __IADsEmail_INTERFACE_DEFINED__
#define __IADsEmail_INTERFACE_DEFINED__

/* interface IADsEmail */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsEmail;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97af011a-478e-11d1-a3b4-00c04fb950dc")
    IADsEmail : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ long lnType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Address( 
            /* [in] */ __RPC__in BSTR bstrAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsEmailVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsEmail * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsEmail * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsEmail * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsEmail * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsEmail * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsEmail * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsEmail * This,
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
        
        DECLSPEC_XFGVIRT(IADsEmail, get_Type)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IADsEmail * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsEmail, put_Type)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IADsEmail * This,
            /* [in] */ long lnType);
        
        DECLSPEC_XFGVIRT(IADsEmail, get_Address)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in IADsEmail * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsEmail, put_Address)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Address )( 
            __RPC__in IADsEmail * This,
            /* [in] */ __RPC__in BSTR bstrAddress);
        
        END_INTERFACE
    } IADsEmailVtbl;

    interface IADsEmail
    {
        CONST_VTBL struct IADsEmailVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsEmail_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsEmail_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsEmail_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsEmail_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsEmail_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsEmail_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsEmail_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsEmail_get_Type(This,retval)	\
    ( (This)->lpVtbl -> get_Type(This,retval) ) 

#define IADsEmail_put_Type(This,lnType)	\
    ( (This)->lpVtbl -> put_Type(This,lnType) ) 

#define IADsEmail_get_Address(This,retval)	\
    ( (This)->lpVtbl -> get_Address(This,retval) ) 

#define IADsEmail_put_Address(This,bstrAddress)	\
    ( (This)->lpVtbl -> put_Address(This,bstrAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsEmail_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Email;

#ifdef __cplusplus

class DECLSPEC_UUID("8f92a857-478e-11d1-a3b4-00c04fb950dc")
Email;
#endif

#ifndef __IADsPath_INTERFACE_DEFINED__
#define __IADsPath_INTERFACE_DEFINED__

/* interface IADsPath */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPath;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b287fcd5-4080-11d1-a3ac-00c04fb950dc")
    IADsPath : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ long lnType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_VolumeName( 
            /* [in] */ __RPC__in BSTR bstrVolumeName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Path( 
            /* [in] */ __RPC__in BSTR bstrPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPathVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPath * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPath * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPath * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPath * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPath * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPath * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPath * This,
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
        
        DECLSPEC_XFGVIRT(IADsPath, get_Type)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IADsPath * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPath, put_Type)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IADsPath * This,
            /* [in] */ long lnType);
        
        DECLSPEC_XFGVIRT(IADsPath, get_VolumeName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeName )( 
            __RPC__in IADsPath * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPath, put_VolumeName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_VolumeName )( 
            __RPC__in IADsPath * This,
            /* [in] */ __RPC__in BSTR bstrVolumeName);
        
        DECLSPEC_XFGVIRT(IADsPath, get_Path)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IADsPath * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsPath, put_Path)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Path )( 
            __RPC__in IADsPath * This,
            /* [in] */ __RPC__in BSTR bstrPath);
        
        END_INTERFACE
    } IADsPathVtbl;

    interface IADsPath
    {
        CONST_VTBL struct IADsPathVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPath_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPath_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPath_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPath_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPath_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPath_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPath_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPath_get_Type(This,retval)	\
    ( (This)->lpVtbl -> get_Type(This,retval) ) 

#define IADsPath_put_Type(This,lnType)	\
    ( (This)->lpVtbl -> put_Type(This,lnType) ) 

#define IADsPath_get_VolumeName(This,retval)	\
    ( (This)->lpVtbl -> get_VolumeName(This,retval) ) 

#define IADsPath_put_VolumeName(This,bstrVolumeName)	\
    ( (This)->lpVtbl -> put_VolumeName(This,bstrVolumeName) ) 

#define IADsPath_get_Path(This,retval)	\
    ( (This)->lpVtbl -> get_Path(This,retval) ) 

#define IADsPath_put_Path(This,bstrPath)	\
    ( (This)->lpVtbl -> put_Path(This,bstrPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPath_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Path;

#ifdef __cplusplus

class DECLSPEC_UUID("b2538919-4080-11d1-a3ac-00c04fb950dc")
Path;
#endif

#ifndef __IADsReplicaPointer_INTERFACE_DEFINED__
#define __IADsReplicaPointer_INTERFACE_DEFINED__

/* interface IADsReplicaPointer */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsReplicaPointer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f60fb803-4080-11d1-a3ac-00c04fb950dc")
    IADsReplicaPointer : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ServerName( 
            /* [in] */ __RPC__in BSTR bstrServerName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ReplicaType( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ReplicaType( 
            /* [in] */ long lnReplicaType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ReplicaNumber( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ReplicaNumber( 
            /* [in] */ long lnReplicaNumber) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Count( 
            /* [in] */ long lnCount) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ReplicaAddressHints( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ReplicaAddressHints( 
            /* [in] */ VARIANT vReplicaAddressHints) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsReplicaPointerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsReplicaPointer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsReplicaPointer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsReplicaPointer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsReplicaPointer * This,
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
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, get_ServerName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerName )( 
            __RPC__in IADsReplicaPointer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, put_ServerName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServerName )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ __RPC__in BSTR bstrServerName);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, get_ReplicaType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReplicaType )( 
            __RPC__in IADsReplicaPointer * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, put_ReplicaType)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReplicaType )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ long lnReplicaType);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, get_ReplicaNumber)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReplicaNumber )( 
            __RPC__in IADsReplicaPointer * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, put_ReplicaNumber)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReplicaNumber )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ long lnReplicaNumber);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IADsReplicaPointer * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, put_Count)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Count )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ long lnCount);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, get_ReplicaAddressHints)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReplicaAddressHints )( 
            __RPC__in IADsReplicaPointer * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsReplicaPointer, put_ReplicaAddressHints)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReplicaAddressHints )( 
            __RPC__in IADsReplicaPointer * This,
            /* [in] */ VARIANT vReplicaAddressHints);
        
        END_INTERFACE
    } IADsReplicaPointerVtbl;

    interface IADsReplicaPointer
    {
        CONST_VTBL struct IADsReplicaPointerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsReplicaPointer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsReplicaPointer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsReplicaPointer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsReplicaPointer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsReplicaPointer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsReplicaPointer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsReplicaPointer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsReplicaPointer_get_ServerName(This,retval)	\
    ( (This)->lpVtbl -> get_ServerName(This,retval) ) 

#define IADsReplicaPointer_put_ServerName(This,bstrServerName)	\
    ( (This)->lpVtbl -> put_ServerName(This,bstrServerName) ) 

#define IADsReplicaPointer_get_ReplicaType(This,retval)	\
    ( (This)->lpVtbl -> get_ReplicaType(This,retval) ) 

#define IADsReplicaPointer_put_ReplicaType(This,lnReplicaType)	\
    ( (This)->lpVtbl -> put_ReplicaType(This,lnReplicaType) ) 

#define IADsReplicaPointer_get_ReplicaNumber(This,retval)	\
    ( (This)->lpVtbl -> get_ReplicaNumber(This,retval) ) 

#define IADsReplicaPointer_put_ReplicaNumber(This,lnReplicaNumber)	\
    ( (This)->lpVtbl -> put_ReplicaNumber(This,lnReplicaNumber) ) 

#define IADsReplicaPointer_get_Count(This,retval)	\
    ( (This)->lpVtbl -> get_Count(This,retval) ) 

#define IADsReplicaPointer_put_Count(This,lnCount)	\
    ( (This)->lpVtbl -> put_Count(This,lnCount) ) 

#define IADsReplicaPointer_get_ReplicaAddressHints(This,retval)	\
    ( (This)->lpVtbl -> get_ReplicaAddressHints(This,retval) ) 

#define IADsReplicaPointer_put_ReplicaAddressHints(This,vReplicaAddressHints)	\
    ( (This)->lpVtbl -> put_ReplicaAddressHints(This,vReplicaAddressHints) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsReplicaPointer_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ReplicaPointer;

#ifdef __cplusplus

class DECLSPEC_UUID("f5d1badf-4080-11d1-a3ac-00c04fb950dc")
ReplicaPointer;
#endif

#ifndef __IADsAcl_INTERFACE_DEFINED__
#define __IADsAcl_INTERFACE_DEFINED__

/* interface IADsAcl */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsAcl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8452d3ab-0869-11d1-a377-00c04fb950dc")
    IADsAcl : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ProtectedAttrName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ProtectedAttrName( 
            /* [in] */ __RPC__in BSTR bstrProtectedAttrName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SubjectName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SubjectName( 
            /* [in] */ __RPC__in BSTR bstrSubjectName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Privileges( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Privileges( 
            /* [in] */ long lnPrivileges) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CopyAcl( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppAcl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsAclVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsAcl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsAcl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsAcl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsAcl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsAcl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsAcl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsAcl * This,
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
        
        DECLSPEC_XFGVIRT(IADsAcl, get_ProtectedAttrName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProtectedAttrName )( 
            __RPC__in IADsAcl * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsAcl, put_ProtectedAttrName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProtectedAttrName )( 
            __RPC__in IADsAcl * This,
            /* [in] */ __RPC__in BSTR bstrProtectedAttrName);
        
        DECLSPEC_XFGVIRT(IADsAcl, get_SubjectName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubjectName )( 
            __RPC__in IADsAcl * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsAcl, put_SubjectName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubjectName )( 
            __RPC__in IADsAcl * This,
            /* [in] */ __RPC__in BSTR bstrSubjectName);
        
        DECLSPEC_XFGVIRT(IADsAcl, get_Privileges)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Privileges )( 
            __RPC__in IADsAcl * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsAcl, put_Privileges)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Privileges )( 
            __RPC__in IADsAcl * This,
            /* [in] */ long lnPrivileges);
        
        DECLSPEC_XFGVIRT(IADsAcl, CopyAcl)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CopyAcl )( 
            __RPC__in IADsAcl * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppAcl);
        
        END_INTERFACE
    } IADsAclVtbl;

    interface IADsAcl
    {
        CONST_VTBL struct IADsAclVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsAcl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsAcl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsAcl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsAcl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsAcl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsAcl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsAcl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsAcl_get_ProtectedAttrName(This,retval)	\
    ( (This)->lpVtbl -> get_ProtectedAttrName(This,retval) ) 

#define IADsAcl_put_ProtectedAttrName(This,bstrProtectedAttrName)	\
    ( (This)->lpVtbl -> put_ProtectedAttrName(This,bstrProtectedAttrName) ) 

#define IADsAcl_get_SubjectName(This,retval)	\
    ( (This)->lpVtbl -> get_SubjectName(This,retval) ) 

#define IADsAcl_put_SubjectName(This,bstrSubjectName)	\
    ( (This)->lpVtbl -> put_SubjectName(This,bstrSubjectName) ) 

#define IADsAcl_get_Privileges(This,retval)	\
    ( (This)->lpVtbl -> get_Privileges(This,retval) ) 

#define IADsAcl_put_Privileges(This,lnPrivileges)	\
    ( (This)->lpVtbl -> put_Privileges(This,lnPrivileges) ) 

#define IADsAcl_CopyAcl(This,ppAcl)	\
    ( (This)->lpVtbl -> CopyAcl(This,ppAcl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsAcl_INTERFACE_DEFINED__ */


#ifndef __IADsTimestamp_INTERFACE_DEFINED__
#define __IADsTimestamp_INTERFACE_DEFINED__

/* interface IADsTimestamp */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsTimestamp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b2f5a901-4080-11d1-a3ac-00c04fb950dc")
    IADsTimestamp : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_WholeSeconds( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_WholeSeconds( 
            /* [in] */ long lnWholeSeconds) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_EventID( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_EventID( 
            /* [in] */ long lnEventID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsTimestampVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsTimestamp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsTimestamp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsTimestamp * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsTimestamp * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsTimestamp * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsTimestamp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsTimestamp * This,
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
        
        DECLSPEC_XFGVIRT(IADsTimestamp, get_WholeSeconds)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WholeSeconds )( 
            __RPC__in IADsTimestamp * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsTimestamp, put_WholeSeconds)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WholeSeconds )( 
            __RPC__in IADsTimestamp * This,
            /* [in] */ long lnWholeSeconds);
        
        DECLSPEC_XFGVIRT(IADsTimestamp, get_EventID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventID )( 
            __RPC__in IADsTimestamp * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsTimestamp, put_EventID)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventID )( 
            __RPC__in IADsTimestamp * This,
            /* [in] */ long lnEventID);
        
        END_INTERFACE
    } IADsTimestampVtbl;

    interface IADsTimestamp
    {
        CONST_VTBL struct IADsTimestampVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsTimestamp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsTimestamp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsTimestamp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsTimestamp_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsTimestamp_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsTimestamp_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsTimestamp_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsTimestamp_get_WholeSeconds(This,retval)	\
    ( (This)->lpVtbl -> get_WholeSeconds(This,retval) ) 

#define IADsTimestamp_put_WholeSeconds(This,lnWholeSeconds)	\
    ( (This)->lpVtbl -> put_WholeSeconds(This,lnWholeSeconds) ) 

#define IADsTimestamp_get_EventID(This,retval)	\
    ( (This)->lpVtbl -> get_EventID(This,retval) ) 

#define IADsTimestamp_put_EventID(This,lnEventID)	\
    ( (This)->lpVtbl -> put_EventID(This,lnEventID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsTimestamp_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Timestamp;

#ifdef __cplusplus

class DECLSPEC_UUID("b2bed2eb-4080-11d1-a3ac-00c04fb950dc")
Timestamp;
#endif

#ifndef __IADsPostalAddress_INTERFACE_DEFINED__
#define __IADsPostalAddress_INTERFACE_DEFINED__

/* interface IADsPostalAddress */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPostalAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7adecf29-4680-11d1-a3b4-00c04fb950dc")
    IADsPostalAddress : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PostalAddress( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_PostalAddress( 
            /* [in] */ VARIANT vPostalAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPostalAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPostalAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPostalAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPostalAddress * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPostalAddress * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPostalAddress * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPostalAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPostalAddress * This,
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
        
        DECLSPEC_XFGVIRT(IADsPostalAddress, get_PostalAddress)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PostalAddress )( 
            __RPC__in IADsPostalAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsPostalAddress, put_PostalAddress)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PostalAddress )( 
            __RPC__in IADsPostalAddress * This,
            /* [in] */ VARIANT vPostalAddress);
        
        END_INTERFACE
    } IADsPostalAddressVtbl;

    interface IADsPostalAddress
    {
        CONST_VTBL struct IADsPostalAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPostalAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPostalAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPostalAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPostalAddress_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPostalAddress_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPostalAddress_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPostalAddress_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPostalAddress_get_PostalAddress(This,retval)	\
    ( (This)->lpVtbl -> get_PostalAddress(This,retval) ) 

#define IADsPostalAddress_put_PostalAddress(This,vPostalAddress)	\
    ( (This)->lpVtbl -> put_PostalAddress(This,vPostalAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPostalAddress_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_PostalAddress;

#ifdef __cplusplus

class DECLSPEC_UUID("0a75afcd-4680-11d1-a3b4-00c04fb950dc")
PostalAddress;
#endif

#ifndef __IADsBackLink_INTERFACE_DEFINED__
#define __IADsBackLink_INTERFACE_DEFINED__

/* interface IADsBackLink */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsBackLink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fd1302bd-4080-11d1-a3ac-00c04fb950dc")
    IADsBackLink : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_RemoteID( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_RemoteID( 
            /* [in] */ long lnRemoteID) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ObjectName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ObjectName( 
            /* [in] */ __RPC__in BSTR bstrObjectName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsBackLinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsBackLink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsBackLink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsBackLink * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsBackLink * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsBackLink * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsBackLink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsBackLink * This,
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
        
        DECLSPEC_XFGVIRT(IADsBackLink, get_RemoteID)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoteID )( 
            __RPC__in IADsBackLink * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsBackLink, put_RemoteID)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RemoteID )( 
            __RPC__in IADsBackLink * This,
            /* [in] */ long lnRemoteID);
        
        DECLSPEC_XFGVIRT(IADsBackLink, get_ObjectName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectName )( 
            __RPC__in IADsBackLink * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsBackLink, put_ObjectName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ObjectName )( 
            __RPC__in IADsBackLink * This,
            /* [in] */ __RPC__in BSTR bstrObjectName);
        
        END_INTERFACE
    } IADsBackLinkVtbl;

    interface IADsBackLink
    {
        CONST_VTBL struct IADsBackLinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsBackLink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsBackLink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsBackLink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsBackLink_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsBackLink_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsBackLink_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsBackLink_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsBackLink_get_RemoteID(This,retval)	\
    ( (This)->lpVtbl -> get_RemoteID(This,retval) ) 

#define IADsBackLink_put_RemoteID(This,lnRemoteID)	\
    ( (This)->lpVtbl -> put_RemoteID(This,lnRemoteID) ) 

#define IADsBackLink_get_ObjectName(This,retval)	\
    ( (This)->lpVtbl -> get_ObjectName(This,retval) ) 

#define IADsBackLink_put_ObjectName(This,bstrObjectName)	\
    ( (This)->lpVtbl -> put_ObjectName(This,bstrObjectName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsBackLink_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_BackLink;

#ifdef __cplusplus

class DECLSPEC_UUID("fcbf906f-4080-11d1-a3ac-00c04fb950dc")
BackLink;
#endif

#ifndef __IADsTypedName_INTERFACE_DEFINED__
#define __IADsTypedName_INTERFACE_DEFINED__

/* interface IADsTypedName */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsTypedName;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b371a349-4080-11d1-a3ac-00c04fb950dc")
    IADsTypedName : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ObjectName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ObjectName( 
            /* [in] */ __RPC__in BSTR bstrObjectName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Level( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Level( 
            /* [in] */ long lnLevel) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Interval( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Interval( 
            /* [in] */ long lnInterval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsTypedNameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsTypedName * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsTypedName * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsTypedName * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsTypedName * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsTypedName * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsTypedName * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsTypedName * This,
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
        
        DECLSPEC_XFGVIRT(IADsTypedName, get_ObjectName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectName )( 
            __RPC__in IADsTypedName * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsTypedName, put_ObjectName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ObjectName )( 
            __RPC__in IADsTypedName * This,
            /* [in] */ __RPC__in BSTR bstrObjectName);
        
        DECLSPEC_XFGVIRT(IADsTypedName, get_Level)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Level )( 
            __RPC__in IADsTypedName * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsTypedName, put_Level)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Level )( 
            __RPC__in IADsTypedName * This,
            /* [in] */ long lnLevel);
        
        DECLSPEC_XFGVIRT(IADsTypedName, get_Interval)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Interval )( 
            __RPC__in IADsTypedName * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsTypedName, put_Interval)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Interval )( 
            __RPC__in IADsTypedName * This,
            /* [in] */ long lnInterval);
        
        END_INTERFACE
    } IADsTypedNameVtbl;

    interface IADsTypedName
    {
        CONST_VTBL struct IADsTypedNameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsTypedName_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsTypedName_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsTypedName_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsTypedName_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsTypedName_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsTypedName_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsTypedName_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsTypedName_get_ObjectName(This,retval)	\
    ( (This)->lpVtbl -> get_ObjectName(This,retval) ) 

#define IADsTypedName_put_ObjectName(This,bstrObjectName)	\
    ( (This)->lpVtbl -> put_ObjectName(This,bstrObjectName) ) 

#define IADsTypedName_get_Level(This,retval)	\
    ( (This)->lpVtbl -> get_Level(This,retval) ) 

#define IADsTypedName_put_Level(This,lnLevel)	\
    ( (This)->lpVtbl -> put_Level(This,lnLevel) ) 

#define IADsTypedName_get_Interval(This,retval)	\
    ( (This)->lpVtbl -> get_Interval(This,retval) ) 

#define IADsTypedName_put_Interval(This,lnInterval)	\
    ( (This)->lpVtbl -> put_Interval(This,lnInterval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsTypedName_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_TypedName;

#ifdef __cplusplus

class DECLSPEC_UUID("b33143cb-4080-11d1-a3ac-00c04fb950dc")
TypedName;
#endif

#ifndef __IADsHold_INTERFACE_DEFINED__
#define __IADsHold_INTERFACE_DEFINED__

/* interface IADsHold */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsHold;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b3eb3b37-4080-11d1-a3ac-00c04fb950dc")
    IADsHold : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ObjectName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_ObjectName( 
            /* [in] */ __RPC__in BSTR bstrObjectName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Amount( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Amount( 
            /* [in] */ long lnAmount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsHoldVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsHold * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsHold * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsHold * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsHold * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsHold * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsHold * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsHold * This,
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
        
        DECLSPEC_XFGVIRT(IADsHold, get_ObjectName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectName )( 
            __RPC__in IADsHold * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsHold, put_ObjectName)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ObjectName )( 
            __RPC__in IADsHold * This,
            /* [in] */ __RPC__in BSTR bstrObjectName);
        
        DECLSPEC_XFGVIRT(IADsHold, get_Amount)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Amount )( 
            __RPC__in IADsHold * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsHold, put_Amount)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Amount )( 
            __RPC__in IADsHold * This,
            /* [in] */ long lnAmount);
        
        END_INTERFACE
    } IADsHoldVtbl;

    interface IADsHold
    {
        CONST_VTBL struct IADsHoldVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsHold_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsHold_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsHold_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsHold_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsHold_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsHold_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsHold_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsHold_get_ObjectName(This,retval)	\
    ( (This)->lpVtbl -> get_ObjectName(This,retval) ) 

#define IADsHold_put_ObjectName(This,bstrObjectName)	\
    ( (This)->lpVtbl -> put_ObjectName(This,bstrObjectName) ) 

#define IADsHold_get_Amount(This,retval)	\
    ( (This)->lpVtbl -> get_Amount(This,retval) ) 

#define IADsHold_put_Amount(This,lnAmount)	\
    ( (This)->lpVtbl -> put_Amount(This,lnAmount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsHold_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Hold;

#ifdef __cplusplus

class DECLSPEC_UUID("b3ad3e13-4080-11d1-a3ac-00c04fb950dc")
Hold;
#endif

#ifndef __IADsObjectOptions_INTERFACE_DEFINED__
#define __IADsObjectOptions_INTERFACE_DEFINED__

/* interface IADsObjectOptions */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsObjectOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("46f14fda-232b-11d1-a808-00c04fd8d5a8")
    IADsObjectOptions : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetOption( 
            /* [in] */ long lnOption,
            /* [retval][out] */ __RPC__out VARIANT *pvValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetOption( 
            /* [in] */ long lnOption,
            /* [in] */ VARIANT vValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsObjectOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsObjectOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsObjectOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsObjectOptions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsObjectOptions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsObjectOptions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsObjectOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsObjectOptions * This,
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
        
        DECLSPEC_XFGVIRT(IADsObjectOptions, GetOption)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetOption )( 
            __RPC__in IADsObjectOptions * This,
            /* [in] */ long lnOption,
            /* [retval][out] */ __RPC__out VARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IADsObjectOptions, SetOption)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetOption )( 
            __RPC__in IADsObjectOptions * This,
            /* [in] */ long lnOption,
            /* [in] */ VARIANT vValue);
        
        END_INTERFACE
    } IADsObjectOptionsVtbl;

    interface IADsObjectOptions
    {
        CONST_VTBL struct IADsObjectOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsObjectOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsObjectOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsObjectOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsObjectOptions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsObjectOptions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsObjectOptions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsObjectOptions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsObjectOptions_GetOption(This,lnOption,pvValue)	\
    ( (This)->lpVtbl -> GetOption(This,lnOption,pvValue) ) 

#define IADsObjectOptions_SetOption(This,lnOption,vValue)	\
    ( (This)->lpVtbl -> SetOption(This,lnOption,vValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsObjectOptions_INTERFACE_DEFINED__ */


#ifndef __IADsPathname_INTERFACE_DEFINED__
#define __IADsPathname_INTERFACE_DEFINED__

/* interface IADsPathname */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsPathname;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d592aed4-f420-11d0-a36e-00c04fb950dc")
    IADsPathname : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Set( 
            /* [in] */ __RPC__in BSTR bstrADsPath,
            /* [in] */ long lnSetType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetDisplayType( 
            /* [in] */ long lnDisplayType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Retrieve( 
            /* [in] */ long lnFormatType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetNumElements( 
            /* [retval][out] */ __RPC__out long *plnNumPathElements) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetElement( 
            /* [in] */ long lnElementIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrElement) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE AddLeafElement( 
            /* [in] */ __RPC__in BSTR bstrLeafElement) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RemoveLeafElement( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CopyPath( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppAdsPath) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetEscapedElement( 
            /* [in] */ long lnReserved,
            /* [in] */ __RPC__in BSTR bstrInStr,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOutStr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_EscapedMode( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_EscapedMode( 
            /* [in] */ long lnEscapedMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsPathnameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsPathname * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsPathname * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsPathname * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsPathname * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsPathname * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsPathname * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsPathname * This,
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
        
        DECLSPEC_XFGVIRT(IADsPathname, Set)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Set )( 
            __RPC__in IADsPathname * This,
            /* [in] */ __RPC__in BSTR bstrADsPath,
            /* [in] */ long lnSetType);
        
        DECLSPEC_XFGVIRT(IADsPathname, SetDisplayType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetDisplayType )( 
            __RPC__in IADsPathname * This,
            /* [in] */ long lnDisplayType);
        
        DECLSPEC_XFGVIRT(IADsPathname, Retrieve)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Retrieve )( 
            __RPC__in IADsPathname * This,
            /* [in] */ long lnFormatType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrADsPath);
        
        DECLSPEC_XFGVIRT(IADsPathname, GetNumElements)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetNumElements )( 
            __RPC__in IADsPathname * This,
            /* [retval][out] */ __RPC__out long *plnNumPathElements);
        
        DECLSPEC_XFGVIRT(IADsPathname, GetElement)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetElement )( 
            __RPC__in IADsPathname * This,
            /* [in] */ long lnElementIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrElement);
        
        DECLSPEC_XFGVIRT(IADsPathname, AddLeafElement)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *AddLeafElement )( 
            __RPC__in IADsPathname * This,
            /* [in] */ __RPC__in BSTR bstrLeafElement);
        
        DECLSPEC_XFGVIRT(IADsPathname, RemoveLeafElement)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RemoveLeafElement )( 
            __RPC__in IADsPathname * This);
        
        DECLSPEC_XFGVIRT(IADsPathname, CopyPath)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CopyPath )( 
            __RPC__in IADsPathname * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppAdsPath);
        
        DECLSPEC_XFGVIRT(IADsPathname, GetEscapedElement)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetEscapedElement )( 
            __RPC__in IADsPathname * This,
            /* [in] */ long lnReserved,
            /* [in] */ __RPC__in BSTR bstrInStr,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOutStr);
        
        DECLSPEC_XFGVIRT(IADsPathname, get_EscapedMode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EscapedMode )( 
            __RPC__in IADsPathname * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsPathname, put_EscapedMode)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EscapedMode )( 
            __RPC__in IADsPathname * This,
            /* [in] */ long lnEscapedMode);
        
        END_INTERFACE
    } IADsPathnameVtbl;

    interface IADsPathname
    {
        CONST_VTBL struct IADsPathnameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsPathname_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsPathname_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsPathname_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsPathname_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsPathname_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsPathname_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsPathname_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsPathname_Set(This,bstrADsPath,lnSetType)	\
    ( (This)->lpVtbl -> Set(This,bstrADsPath,lnSetType) ) 

#define IADsPathname_SetDisplayType(This,lnDisplayType)	\
    ( (This)->lpVtbl -> SetDisplayType(This,lnDisplayType) ) 

#define IADsPathname_Retrieve(This,lnFormatType,pbstrADsPath)	\
    ( (This)->lpVtbl -> Retrieve(This,lnFormatType,pbstrADsPath) ) 

#define IADsPathname_GetNumElements(This,plnNumPathElements)	\
    ( (This)->lpVtbl -> GetNumElements(This,plnNumPathElements) ) 

#define IADsPathname_GetElement(This,lnElementIndex,pbstrElement)	\
    ( (This)->lpVtbl -> GetElement(This,lnElementIndex,pbstrElement) ) 

#define IADsPathname_AddLeafElement(This,bstrLeafElement)	\
    ( (This)->lpVtbl -> AddLeafElement(This,bstrLeafElement) ) 

#define IADsPathname_RemoveLeafElement(This)	\
    ( (This)->lpVtbl -> RemoveLeafElement(This) ) 

#define IADsPathname_CopyPath(This,ppAdsPath)	\
    ( (This)->lpVtbl -> CopyPath(This,ppAdsPath) ) 

#define IADsPathname_GetEscapedElement(This,lnReserved,bstrInStr,pbstrOutStr)	\
    ( (This)->lpVtbl -> GetEscapedElement(This,lnReserved,bstrInStr,pbstrOutStr) ) 

#define IADsPathname_get_EscapedMode(This,retval)	\
    ( (This)->lpVtbl -> get_EscapedMode(This,retval) ) 

#define IADsPathname_put_EscapedMode(This,lnEscapedMode)	\
    ( (This)->lpVtbl -> put_EscapedMode(This,lnEscapedMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsPathname_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Pathname;

#ifdef __cplusplus

class DECLSPEC_UUID("080d0d78-f421-11d0-a36e-00c04fb950dc")
Pathname;
#endif

#ifndef __IADsADSystemInfo_INTERFACE_DEFINED__
#define __IADsADSystemInfo_INTERFACE_DEFINED__

/* interface IADsADSystemInfo */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsADSystemInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5BB11929-AFD1-11d2-9CB9-0000F87A369E")
    IADsADSystemInfo : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SiteName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainShortName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainDNSName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ForestDNSName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PDCRoleOwner( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SchemaRoleOwner( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_IsNativeMode( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetAnyDCName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDCName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetDCSiteName( 
            /* [in] */ __RPC__in BSTR szServer,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszSiteName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE RefreshSchemaCache( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetTrees( 
            /* [retval][out] */ __RPC__out VARIANT *pvTrees) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsADSystemInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsADSystemInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsADSystemInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsADSystemInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsADSystemInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsADSystemInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsADSystemInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsADSystemInfo * This,
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
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_UserName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_ComputerName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_SiteName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SiteName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_DomainShortName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainShortName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_DomainDNSName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainDNSName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_ForestDNSName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForestDNSName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_PDCRoleOwner)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PDCRoleOwner )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_SchemaRoleOwner)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SchemaRoleOwner )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, get_IsNativeMode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsNativeMode )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *retval);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, GetAnyDCName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetAnyDCName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDCName);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, GetDCSiteName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetDCSiteName )( 
            __RPC__in IADsADSystemInfo * This,
            /* [in] */ __RPC__in BSTR szServer,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszSiteName);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, RefreshSchemaCache)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *RefreshSchemaCache )( 
            __RPC__in IADsADSystemInfo * This);
        
        DECLSPEC_XFGVIRT(IADsADSystemInfo, GetTrees)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetTrees )( 
            __RPC__in IADsADSystemInfo * This,
            /* [retval][out] */ __RPC__out VARIANT *pvTrees);
        
        END_INTERFACE
    } IADsADSystemInfoVtbl;

    interface IADsADSystemInfo
    {
        CONST_VTBL struct IADsADSystemInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsADSystemInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsADSystemInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsADSystemInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsADSystemInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsADSystemInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsADSystemInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsADSystemInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsADSystemInfo_get_UserName(This,retval)	\
    ( (This)->lpVtbl -> get_UserName(This,retval) ) 

#define IADsADSystemInfo_get_ComputerName(This,retval)	\
    ( (This)->lpVtbl -> get_ComputerName(This,retval) ) 

#define IADsADSystemInfo_get_SiteName(This,retval)	\
    ( (This)->lpVtbl -> get_SiteName(This,retval) ) 

#define IADsADSystemInfo_get_DomainShortName(This,retval)	\
    ( (This)->lpVtbl -> get_DomainShortName(This,retval) ) 

#define IADsADSystemInfo_get_DomainDNSName(This,retval)	\
    ( (This)->lpVtbl -> get_DomainDNSName(This,retval) ) 

#define IADsADSystemInfo_get_ForestDNSName(This,retval)	\
    ( (This)->lpVtbl -> get_ForestDNSName(This,retval) ) 

#define IADsADSystemInfo_get_PDCRoleOwner(This,retval)	\
    ( (This)->lpVtbl -> get_PDCRoleOwner(This,retval) ) 

#define IADsADSystemInfo_get_SchemaRoleOwner(This,retval)	\
    ( (This)->lpVtbl -> get_SchemaRoleOwner(This,retval) ) 

#define IADsADSystemInfo_get_IsNativeMode(This,retval)	\
    ( (This)->lpVtbl -> get_IsNativeMode(This,retval) ) 

#define IADsADSystemInfo_GetAnyDCName(This,pszDCName)	\
    ( (This)->lpVtbl -> GetAnyDCName(This,pszDCName) ) 

#define IADsADSystemInfo_GetDCSiteName(This,szServer,pszSiteName)	\
    ( (This)->lpVtbl -> GetDCSiteName(This,szServer,pszSiteName) ) 

#define IADsADSystemInfo_RefreshSchemaCache(This)	\
    ( (This)->lpVtbl -> RefreshSchemaCache(This) ) 

#define IADsADSystemInfo_GetTrees(This,pvTrees)	\
    ( (This)->lpVtbl -> GetTrees(This,pvTrees) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsADSystemInfo_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ADSystemInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("50B6327F-AFD1-11d2-9CB9-0000F87A369E")
ADSystemInfo;
#endif

#ifndef __IADsWinNTSystemInfo_INTERFACE_DEFINED__
#define __IADsWinNTSystemInfo_INTERFACE_DEFINED__

/* interface IADsWinNTSystemInfo */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsWinNTSystemInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6C6D65DC-AFD1-11d2-9CB9-0000F87A369E")
    IADsWinNTSystemInfo : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PDC( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsWinNTSystemInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsWinNTSystemInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsWinNTSystemInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsWinNTSystemInfo * This,
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
        
        DECLSPEC_XFGVIRT(IADsWinNTSystemInfo, get_UserName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsWinNTSystemInfo, get_ComputerName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerName )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsWinNTSystemInfo, get_DomainName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainName )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsWinNTSystemInfo, get_PDC)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PDC )( 
            __RPC__in IADsWinNTSystemInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        END_INTERFACE
    } IADsWinNTSystemInfoVtbl;

    interface IADsWinNTSystemInfo
    {
        CONST_VTBL struct IADsWinNTSystemInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsWinNTSystemInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsWinNTSystemInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsWinNTSystemInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsWinNTSystemInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsWinNTSystemInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsWinNTSystemInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsWinNTSystemInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsWinNTSystemInfo_get_UserName(This,retval)	\
    ( (This)->lpVtbl -> get_UserName(This,retval) ) 

#define IADsWinNTSystemInfo_get_ComputerName(This,retval)	\
    ( (This)->lpVtbl -> get_ComputerName(This,retval) ) 

#define IADsWinNTSystemInfo_get_DomainName(This,retval)	\
    ( (This)->lpVtbl -> get_DomainName(This,retval) ) 

#define IADsWinNTSystemInfo_get_PDC(This,retval)	\
    ( (This)->lpVtbl -> get_PDC(This,retval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsWinNTSystemInfo_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WinNTSystemInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("66182EC4-AFD1-11d2-9CB9-0000F87A369E")
WinNTSystemInfo;
#endif

#ifndef __IADsDNWithBinary_INTERFACE_DEFINED__
#define __IADsDNWithBinary_INTERFACE_DEFINED__

/* interface IADsDNWithBinary */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsDNWithBinary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7e99c0a2-f935-11d2-ba96-00c04fb6d0d1")
    IADsDNWithBinary : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BinaryValue( 
            /* [retval][out] */ __RPC__out VARIANT *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_BinaryValue( 
            /* [in] */ VARIANT vBinaryValue) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DNString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DNString( 
            /* [in] */ __RPC__in BSTR bstrDNString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsDNWithBinaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsDNWithBinary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsDNWithBinary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsDNWithBinary * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsDNWithBinary * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsDNWithBinary * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsDNWithBinary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsDNWithBinary * This,
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
        
        DECLSPEC_XFGVIRT(IADsDNWithBinary, get_BinaryValue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BinaryValue )( 
            __RPC__in IADsDNWithBinary * This,
            /* [retval][out] */ __RPC__out VARIANT *retval);
        
        DECLSPEC_XFGVIRT(IADsDNWithBinary, put_BinaryValue)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BinaryValue )( 
            __RPC__in IADsDNWithBinary * This,
            /* [in] */ VARIANT vBinaryValue);
        
        DECLSPEC_XFGVIRT(IADsDNWithBinary, get_DNString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DNString )( 
            __RPC__in IADsDNWithBinary * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsDNWithBinary, put_DNString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DNString )( 
            __RPC__in IADsDNWithBinary * This,
            /* [in] */ __RPC__in BSTR bstrDNString);
        
        END_INTERFACE
    } IADsDNWithBinaryVtbl;

    interface IADsDNWithBinary
    {
        CONST_VTBL struct IADsDNWithBinaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsDNWithBinary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsDNWithBinary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsDNWithBinary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsDNWithBinary_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsDNWithBinary_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsDNWithBinary_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsDNWithBinary_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsDNWithBinary_get_BinaryValue(This,retval)	\
    ( (This)->lpVtbl -> get_BinaryValue(This,retval) ) 

#define IADsDNWithBinary_put_BinaryValue(This,vBinaryValue)	\
    ( (This)->lpVtbl -> put_BinaryValue(This,vBinaryValue) ) 

#define IADsDNWithBinary_get_DNString(This,retval)	\
    ( (This)->lpVtbl -> get_DNString(This,retval) ) 

#define IADsDNWithBinary_put_DNString(This,bstrDNString)	\
    ( (This)->lpVtbl -> put_DNString(This,bstrDNString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsDNWithBinary_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_DNWithBinary;

#ifdef __cplusplus

class DECLSPEC_UUID("7e99c0a3-f935-11d2-ba96-00c04fb6d0d1")
DNWithBinary;
#endif

#ifndef __IADsDNWithString_INTERFACE_DEFINED__
#define __IADsDNWithString_INTERFACE_DEFINED__

/* interface IADsDNWithString */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsDNWithString;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("370df02e-f934-11d2-ba96-00c04fb6d0d1")
    IADsDNWithString : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_StringValue( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_StringValue( 
            /* [in] */ __RPC__in BSTR bstrStringValue) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DNString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DNString( 
            /* [in] */ __RPC__in BSTR bstrDNString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsDNWithStringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsDNWithString * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsDNWithString * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsDNWithString * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsDNWithString * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsDNWithString * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsDNWithString * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsDNWithString * This,
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
        
        DECLSPEC_XFGVIRT(IADsDNWithString, get_StringValue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StringValue )( 
            __RPC__in IADsDNWithString * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsDNWithString, put_StringValue)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StringValue )( 
            __RPC__in IADsDNWithString * This,
            /* [in] */ __RPC__in BSTR bstrStringValue);
        
        DECLSPEC_XFGVIRT(IADsDNWithString, get_DNString)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DNString )( 
            __RPC__in IADsDNWithString * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retval);
        
        DECLSPEC_XFGVIRT(IADsDNWithString, put_DNString)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DNString )( 
            __RPC__in IADsDNWithString * This,
            /* [in] */ __RPC__in BSTR bstrDNString);
        
        END_INTERFACE
    } IADsDNWithStringVtbl;

    interface IADsDNWithString
    {
        CONST_VTBL struct IADsDNWithStringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsDNWithString_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsDNWithString_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsDNWithString_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsDNWithString_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsDNWithString_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsDNWithString_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsDNWithString_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsDNWithString_get_StringValue(This,retval)	\
    ( (This)->lpVtbl -> get_StringValue(This,retval) ) 

#define IADsDNWithString_put_StringValue(This,bstrStringValue)	\
    ( (This)->lpVtbl -> put_StringValue(This,bstrStringValue) ) 

#define IADsDNWithString_get_DNString(This,retval)	\
    ( (This)->lpVtbl -> get_DNString(This,retval) ) 

#define IADsDNWithString_put_DNString(This,bstrDNString)	\
    ( (This)->lpVtbl -> put_DNString(This,bstrDNString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsDNWithString_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_DNWithString;

#ifdef __cplusplus

class DECLSPEC_UUID("334857cc-f934-11d2-ba96-00c04fb6d0d1")
DNWithString;
#endif

#ifndef __IADsSecurityUtility_INTERFACE_DEFINED__
#define __IADsSecurityUtility_INTERFACE_DEFINED__

/* interface IADsSecurityUtility */
/* [object][dual][oleautomation][uuid] */ 


EXTERN_C const IID IID_IADsSecurityUtility;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a63251b2-5f21-474b-ab52-4a8efad10895")
    IADsSecurityUtility : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetSecurityDescriptor( 
            /* [in] */ VARIANT varPath,
            /* [in] */ long lPathFormat,
            /* [in] */ long lFormat,
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetSecurityDescriptor( 
            /* [in] */ VARIANT varPath,
            /* [in] */ long lPathFormat,
            /* [in] */ VARIANT varData,
            /* [in] */ long lDataFormat) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ConvertSecurityDescriptor( 
            /* [in] */ VARIANT varSD,
            /* [in] */ long lDataFormat,
            /* [in] */ long lOutFormat,
            /* [retval][out] */ __RPC__out VARIANT *pResult) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SecurityMask( 
            /* [retval][out] */ __RPC__out long *retval) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SecurityMask( 
            /* [in] */ long lnSecurityMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IADsSecurityUtilityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IADsSecurityUtility * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IADsSecurityUtility * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IADsSecurityUtility * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IADsSecurityUtility * This,
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
        
        DECLSPEC_XFGVIRT(IADsSecurityUtility, GetSecurityDescriptor)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityDescriptor )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ VARIANT varPath,
            /* [in] */ long lPathFormat,
            /* [in] */ long lFormat,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IADsSecurityUtility, SetSecurityDescriptor)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityDescriptor )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ VARIANT varPath,
            /* [in] */ long lPathFormat,
            /* [in] */ VARIANT varData,
            /* [in] */ long lDataFormat);
        
        DECLSPEC_XFGVIRT(IADsSecurityUtility, ConvertSecurityDescriptor)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ConvertSecurityDescriptor )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ VARIANT varSD,
            /* [in] */ long lDataFormat,
            /* [in] */ long lOutFormat,
            /* [retval][out] */ __RPC__out VARIANT *pResult);
        
        DECLSPEC_XFGVIRT(IADsSecurityUtility, get_SecurityMask)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SecurityMask )( 
            __RPC__in IADsSecurityUtility * This,
            /* [retval][out] */ __RPC__out long *retval);
        
        DECLSPEC_XFGVIRT(IADsSecurityUtility, put_SecurityMask)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SecurityMask )( 
            __RPC__in IADsSecurityUtility * This,
            /* [in] */ long lnSecurityMask);
        
        END_INTERFACE
    } IADsSecurityUtilityVtbl;

    interface IADsSecurityUtility
    {
        CONST_VTBL struct IADsSecurityUtilityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IADsSecurityUtility_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IADsSecurityUtility_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IADsSecurityUtility_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IADsSecurityUtility_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IADsSecurityUtility_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IADsSecurityUtility_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IADsSecurityUtility_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IADsSecurityUtility_GetSecurityDescriptor(This,varPath,lPathFormat,lFormat,pVariant)	\
    ( (This)->lpVtbl -> GetSecurityDescriptor(This,varPath,lPathFormat,lFormat,pVariant) ) 

#define IADsSecurityUtility_SetSecurityDescriptor(This,varPath,lPathFormat,varData,lDataFormat)	\
    ( (This)->lpVtbl -> SetSecurityDescriptor(This,varPath,lPathFormat,varData,lDataFormat) ) 

#define IADsSecurityUtility_ConvertSecurityDescriptor(This,varSD,lDataFormat,lOutFormat,pResult)	\
    ( (This)->lpVtbl -> ConvertSecurityDescriptor(This,varSD,lDataFormat,lOutFormat,pResult) ) 

#define IADsSecurityUtility_get_SecurityMask(This,retval)	\
    ( (This)->lpVtbl -> get_SecurityMask(This,retval) ) 

#define IADsSecurityUtility_put_SecurityMask(This,lnSecurityMask)	\
    ( (This)->lpVtbl -> put_SecurityMask(This,lnSecurityMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IADsSecurityUtility_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ADsSecurityUtility;

#ifdef __cplusplus

class DECLSPEC_UUID("f270c64a-ffb8-4ae4-85fe-3a75e5347966")
ADsSecurityUtility;
#endif
#endif /* __ActiveDs_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


