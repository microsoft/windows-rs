

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

#ifndef __gpmgmt_h__
#define __gpmgmt_h__

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

#ifndef __IGPM_FWD_DEFINED__
#define __IGPM_FWD_DEFINED__
typedef interface IGPM IGPM;

#endif 	/* __IGPM_FWD_DEFINED__ */


#ifndef __IGPMDomain_FWD_DEFINED__
#define __IGPMDomain_FWD_DEFINED__
typedef interface IGPMDomain IGPMDomain;

#endif 	/* __IGPMDomain_FWD_DEFINED__ */


#ifndef __IGPMBackupDir_FWD_DEFINED__
#define __IGPMBackupDir_FWD_DEFINED__
typedef interface IGPMBackupDir IGPMBackupDir;

#endif 	/* __IGPMBackupDir_FWD_DEFINED__ */


#ifndef __IGPMSitesContainer_FWD_DEFINED__
#define __IGPMSitesContainer_FWD_DEFINED__
typedef interface IGPMSitesContainer IGPMSitesContainer;

#endif 	/* __IGPMSitesContainer_FWD_DEFINED__ */


#ifndef __IGPMSearchCriteria_FWD_DEFINED__
#define __IGPMSearchCriteria_FWD_DEFINED__
typedef interface IGPMSearchCriteria IGPMSearchCriteria;

#endif 	/* __IGPMSearchCriteria_FWD_DEFINED__ */


#ifndef __IGPMTrustee_FWD_DEFINED__
#define __IGPMTrustee_FWD_DEFINED__
typedef interface IGPMTrustee IGPMTrustee;

#endif 	/* __IGPMTrustee_FWD_DEFINED__ */


#ifndef __IGPMPermission_FWD_DEFINED__
#define __IGPMPermission_FWD_DEFINED__
typedef interface IGPMPermission IGPMPermission;

#endif 	/* __IGPMPermission_FWD_DEFINED__ */


#ifndef __IGPMSecurityInfo_FWD_DEFINED__
#define __IGPMSecurityInfo_FWD_DEFINED__
typedef interface IGPMSecurityInfo IGPMSecurityInfo;

#endif 	/* __IGPMSecurityInfo_FWD_DEFINED__ */


#ifndef __IGPMBackup_FWD_DEFINED__
#define __IGPMBackup_FWD_DEFINED__
typedef interface IGPMBackup IGPMBackup;

#endif 	/* __IGPMBackup_FWD_DEFINED__ */


#ifndef __IGPMBackupCollection_FWD_DEFINED__
#define __IGPMBackupCollection_FWD_DEFINED__
typedef interface IGPMBackupCollection IGPMBackupCollection;

#endif 	/* __IGPMBackupCollection_FWD_DEFINED__ */


#ifndef __IGPMSOM_FWD_DEFINED__
#define __IGPMSOM_FWD_DEFINED__
typedef interface IGPMSOM IGPMSOM;

#endif 	/* __IGPMSOM_FWD_DEFINED__ */


#ifndef __IGPMSOMCollection_FWD_DEFINED__
#define __IGPMSOMCollection_FWD_DEFINED__
typedef interface IGPMSOMCollection IGPMSOMCollection;

#endif 	/* __IGPMSOMCollection_FWD_DEFINED__ */


#ifndef __IGPMWMIFilter_FWD_DEFINED__
#define __IGPMWMIFilter_FWD_DEFINED__
typedef interface IGPMWMIFilter IGPMWMIFilter;

#endif 	/* __IGPMWMIFilter_FWD_DEFINED__ */


#ifndef __IGPMWMIFilterCollection_FWD_DEFINED__
#define __IGPMWMIFilterCollection_FWD_DEFINED__
typedef interface IGPMWMIFilterCollection IGPMWMIFilterCollection;

#endif 	/* __IGPMWMIFilterCollection_FWD_DEFINED__ */


#ifndef __IGPMRSOP_FWD_DEFINED__
#define __IGPMRSOP_FWD_DEFINED__
typedef interface IGPMRSOP IGPMRSOP;

#endif 	/* __IGPMRSOP_FWD_DEFINED__ */


#ifndef __IGPMGPO_FWD_DEFINED__
#define __IGPMGPO_FWD_DEFINED__
typedef interface IGPMGPO IGPMGPO;

#endif 	/* __IGPMGPO_FWD_DEFINED__ */


#ifndef __IGPMGPOCollection_FWD_DEFINED__
#define __IGPMGPOCollection_FWD_DEFINED__
typedef interface IGPMGPOCollection IGPMGPOCollection;

#endif 	/* __IGPMGPOCollection_FWD_DEFINED__ */


#ifndef __IGPMGPOLink_FWD_DEFINED__
#define __IGPMGPOLink_FWD_DEFINED__
typedef interface IGPMGPOLink IGPMGPOLink;

#endif 	/* __IGPMGPOLink_FWD_DEFINED__ */


#ifndef __IGPMGPOLinksCollection_FWD_DEFINED__
#define __IGPMGPOLinksCollection_FWD_DEFINED__
typedef interface IGPMGPOLinksCollection IGPMGPOLinksCollection;

#endif 	/* __IGPMGPOLinksCollection_FWD_DEFINED__ */


#ifndef __IGPMCSECollection_FWD_DEFINED__
#define __IGPMCSECollection_FWD_DEFINED__
typedef interface IGPMCSECollection IGPMCSECollection;

#endif 	/* __IGPMCSECollection_FWD_DEFINED__ */


#ifndef __IGPMClientSideExtension_FWD_DEFINED__
#define __IGPMClientSideExtension_FWD_DEFINED__
typedef interface IGPMClientSideExtension IGPMClientSideExtension;

#endif 	/* __IGPMClientSideExtension_FWD_DEFINED__ */


#ifndef __IGPMAsyncCancel_FWD_DEFINED__
#define __IGPMAsyncCancel_FWD_DEFINED__
typedef interface IGPMAsyncCancel IGPMAsyncCancel;

#endif 	/* __IGPMAsyncCancel_FWD_DEFINED__ */


#ifndef __IGPMAsyncProgress_FWD_DEFINED__
#define __IGPMAsyncProgress_FWD_DEFINED__
typedef interface IGPMAsyncProgress IGPMAsyncProgress;

#endif 	/* __IGPMAsyncProgress_FWD_DEFINED__ */


#ifndef __IGPMStatusMsgCollection_FWD_DEFINED__
#define __IGPMStatusMsgCollection_FWD_DEFINED__
typedef interface IGPMStatusMsgCollection IGPMStatusMsgCollection;

#endif 	/* __IGPMStatusMsgCollection_FWD_DEFINED__ */


#ifndef __IGPMStatusMessage_FWD_DEFINED__
#define __IGPMStatusMessage_FWD_DEFINED__
typedef interface IGPMStatusMessage IGPMStatusMessage;

#endif 	/* __IGPMStatusMessage_FWD_DEFINED__ */


#ifndef __IGPMConstants_FWD_DEFINED__
#define __IGPMConstants_FWD_DEFINED__
typedef interface IGPMConstants IGPMConstants;

#endif 	/* __IGPMConstants_FWD_DEFINED__ */


#ifndef __IGPMResult_FWD_DEFINED__
#define __IGPMResult_FWD_DEFINED__
typedef interface IGPMResult IGPMResult;

#endif 	/* __IGPMResult_FWD_DEFINED__ */


#ifndef __IGPMMapEntryCollection_FWD_DEFINED__
#define __IGPMMapEntryCollection_FWD_DEFINED__
typedef interface IGPMMapEntryCollection IGPMMapEntryCollection;

#endif 	/* __IGPMMapEntryCollection_FWD_DEFINED__ */


#ifndef __IGPMMapEntry_FWD_DEFINED__
#define __IGPMMapEntry_FWD_DEFINED__
typedef interface IGPMMapEntry IGPMMapEntry;

#endif 	/* __IGPMMapEntry_FWD_DEFINED__ */


#ifndef __IGPMMigrationTable_FWD_DEFINED__
#define __IGPMMigrationTable_FWD_DEFINED__
typedef interface IGPMMigrationTable IGPMMigrationTable;

#endif 	/* __IGPMMigrationTable_FWD_DEFINED__ */


#ifndef __IGPMBackupDirEx_FWD_DEFINED__
#define __IGPMBackupDirEx_FWD_DEFINED__
typedef interface IGPMBackupDirEx IGPMBackupDirEx;

#endif 	/* __IGPMBackupDirEx_FWD_DEFINED__ */


#ifndef __IGPMStarterGPOBackupCollection_FWD_DEFINED__
#define __IGPMStarterGPOBackupCollection_FWD_DEFINED__
typedef interface IGPMStarterGPOBackupCollection IGPMStarterGPOBackupCollection;

#endif 	/* __IGPMStarterGPOBackupCollection_FWD_DEFINED__ */


#ifndef __IGPMStarterGPOBackup_FWD_DEFINED__
#define __IGPMStarterGPOBackup_FWD_DEFINED__
typedef interface IGPMStarterGPOBackup IGPMStarterGPOBackup;

#endif 	/* __IGPMStarterGPOBackup_FWD_DEFINED__ */


#ifndef __IGPM2_FWD_DEFINED__
#define __IGPM2_FWD_DEFINED__
typedef interface IGPM2 IGPM2;

#endif 	/* __IGPM2_FWD_DEFINED__ */


#ifndef __IGPMStarterGPO_FWD_DEFINED__
#define __IGPMStarterGPO_FWD_DEFINED__
typedef interface IGPMStarterGPO IGPMStarterGPO;

#endif 	/* __IGPMStarterGPO_FWD_DEFINED__ */


#ifndef __IGPMStarterGPOCollection_FWD_DEFINED__
#define __IGPMStarterGPOCollection_FWD_DEFINED__
typedef interface IGPMStarterGPOCollection IGPMStarterGPOCollection;

#endif 	/* __IGPMStarterGPOCollection_FWD_DEFINED__ */


#ifndef __IGPMDomain2_FWD_DEFINED__
#define __IGPMDomain2_FWD_DEFINED__
typedef interface IGPMDomain2 IGPMDomain2;

#endif 	/* __IGPMDomain2_FWD_DEFINED__ */


#ifndef __IGPMConstants2_FWD_DEFINED__
#define __IGPMConstants2_FWD_DEFINED__
typedef interface IGPMConstants2 IGPMConstants2;

#endif 	/* __IGPMConstants2_FWD_DEFINED__ */


#ifndef __IGPMGPO2_FWD_DEFINED__
#define __IGPMGPO2_FWD_DEFINED__
typedef interface IGPMGPO2 IGPMGPO2;

#endif 	/* __IGPMGPO2_FWD_DEFINED__ */


#ifndef __IGPMDomain3_FWD_DEFINED__
#define __IGPMDomain3_FWD_DEFINED__
typedef interface IGPMDomain3 IGPMDomain3;

#endif 	/* __IGPMDomain3_FWD_DEFINED__ */


#ifndef __IGPMGPO3_FWD_DEFINED__
#define __IGPMGPO3_FWD_DEFINED__
typedef interface IGPMGPO3 IGPMGPO3;

#endif 	/* __IGPMGPO3_FWD_DEFINED__ */


#ifndef __GPM_FWD_DEFINED__
#define __GPM_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPM GPM;
#else
typedef struct GPM GPM;
#endif /* __cplusplus */

#endif 	/* __GPM_FWD_DEFINED__ */


#ifndef __GPMDomain_FWD_DEFINED__
#define __GPMDomain_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMDomain GPMDomain;
#else
typedef struct GPMDomain GPMDomain;
#endif /* __cplusplus */

#endif 	/* __GPMDomain_FWD_DEFINED__ */


#ifndef __GPMSitesContainer_FWD_DEFINED__
#define __GPMSitesContainer_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMSitesContainer GPMSitesContainer;
#else
typedef struct GPMSitesContainer GPMSitesContainer;
#endif /* __cplusplus */

#endif 	/* __GPMSitesContainer_FWD_DEFINED__ */


#ifndef __GPMBackupDir_FWD_DEFINED__
#define __GPMBackupDir_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMBackupDir GPMBackupDir;
#else
typedef struct GPMBackupDir GPMBackupDir;
#endif /* __cplusplus */

#endif 	/* __GPMBackupDir_FWD_DEFINED__ */


#ifndef __GPMSOM_FWD_DEFINED__
#define __GPMSOM_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMSOM GPMSOM;
#else
typedef struct GPMSOM GPMSOM;
#endif /* __cplusplus */

#endif 	/* __GPMSOM_FWD_DEFINED__ */


#ifndef __GPMSearchCriteria_FWD_DEFINED__
#define __GPMSearchCriteria_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMSearchCriteria GPMSearchCriteria;
#else
typedef struct GPMSearchCriteria GPMSearchCriteria;
#endif /* __cplusplus */

#endif 	/* __GPMSearchCriteria_FWD_DEFINED__ */


#ifndef __GPMPermission_FWD_DEFINED__
#define __GPMPermission_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMPermission GPMPermission;
#else
typedef struct GPMPermission GPMPermission;
#endif /* __cplusplus */

#endif 	/* __GPMPermission_FWD_DEFINED__ */


#ifndef __GPMSecurityInfo_FWD_DEFINED__
#define __GPMSecurityInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMSecurityInfo GPMSecurityInfo;
#else
typedef struct GPMSecurityInfo GPMSecurityInfo;
#endif /* __cplusplus */

#endif 	/* __GPMSecurityInfo_FWD_DEFINED__ */


#ifndef __GPMBackup_FWD_DEFINED__
#define __GPMBackup_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMBackup GPMBackup;
#else
typedef struct GPMBackup GPMBackup;
#endif /* __cplusplus */

#endif 	/* __GPMBackup_FWD_DEFINED__ */


#ifndef __GPMBackupCollection_FWD_DEFINED__
#define __GPMBackupCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMBackupCollection GPMBackupCollection;
#else
typedef struct GPMBackupCollection GPMBackupCollection;
#endif /* __cplusplus */

#endif 	/* __GPMBackupCollection_FWD_DEFINED__ */


#ifndef __GPMSOMCollection_FWD_DEFINED__
#define __GPMSOMCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMSOMCollection GPMSOMCollection;
#else
typedef struct GPMSOMCollection GPMSOMCollection;
#endif /* __cplusplus */

#endif 	/* __GPMSOMCollection_FWD_DEFINED__ */


#ifndef __GPMWMIFilter_FWD_DEFINED__
#define __GPMWMIFilter_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMWMIFilter GPMWMIFilter;
#else
typedef struct GPMWMIFilter GPMWMIFilter;
#endif /* __cplusplus */

#endif 	/* __GPMWMIFilter_FWD_DEFINED__ */


#ifndef __GPMWMIFilterCollection_FWD_DEFINED__
#define __GPMWMIFilterCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMWMIFilterCollection GPMWMIFilterCollection;
#else
typedef struct GPMWMIFilterCollection GPMWMIFilterCollection;
#endif /* __cplusplus */

#endif 	/* __GPMWMIFilterCollection_FWD_DEFINED__ */


#ifndef __GPMRSOP_FWD_DEFINED__
#define __GPMRSOP_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMRSOP GPMRSOP;
#else
typedef struct GPMRSOP GPMRSOP;
#endif /* __cplusplus */

#endif 	/* __GPMRSOP_FWD_DEFINED__ */


#ifndef __GPMGPO_FWD_DEFINED__
#define __GPMGPO_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMGPO GPMGPO;
#else
typedef struct GPMGPO GPMGPO;
#endif /* __cplusplus */

#endif 	/* __GPMGPO_FWD_DEFINED__ */


#ifndef __GPMGPOCollection_FWD_DEFINED__
#define __GPMGPOCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMGPOCollection GPMGPOCollection;
#else
typedef struct GPMGPOCollection GPMGPOCollection;
#endif /* __cplusplus */

#endif 	/* __GPMGPOCollection_FWD_DEFINED__ */


#ifndef __GPMGPOLink_FWD_DEFINED__
#define __GPMGPOLink_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMGPOLink GPMGPOLink;
#else
typedef struct GPMGPOLink GPMGPOLink;
#endif /* __cplusplus */

#endif 	/* __GPMGPOLink_FWD_DEFINED__ */


#ifndef __GPMGPOLinksCollection_FWD_DEFINED__
#define __GPMGPOLinksCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMGPOLinksCollection GPMGPOLinksCollection;
#else
typedef struct GPMGPOLinksCollection GPMGPOLinksCollection;
#endif /* __cplusplus */

#endif 	/* __GPMGPOLinksCollection_FWD_DEFINED__ */


#ifndef __GPMAsyncCancel_FWD_DEFINED__
#define __GPMAsyncCancel_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMAsyncCancel GPMAsyncCancel;
#else
typedef struct GPMAsyncCancel GPMAsyncCancel;
#endif /* __cplusplus */

#endif 	/* __GPMAsyncCancel_FWD_DEFINED__ */


#ifndef __GPMStatusMsgCollection_FWD_DEFINED__
#define __GPMStatusMsgCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMStatusMsgCollection GPMStatusMsgCollection;
#else
typedef struct GPMStatusMsgCollection GPMStatusMsgCollection;
#endif /* __cplusplus */

#endif 	/* __GPMStatusMsgCollection_FWD_DEFINED__ */


#ifndef __GPMStatusMessage_FWD_DEFINED__
#define __GPMStatusMessage_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMStatusMessage GPMStatusMessage;
#else
typedef struct GPMStatusMessage GPMStatusMessage;
#endif /* __cplusplus */

#endif 	/* __GPMStatusMessage_FWD_DEFINED__ */


#ifndef __GPMTrustee_FWD_DEFINED__
#define __GPMTrustee_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMTrustee GPMTrustee;
#else
typedef struct GPMTrustee GPMTrustee;
#endif /* __cplusplus */

#endif 	/* __GPMTrustee_FWD_DEFINED__ */


#ifndef __GPMClientSideExtension_FWD_DEFINED__
#define __GPMClientSideExtension_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMClientSideExtension GPMClientSideExtension;
#else
typedef struct GPMClientSideExtension GPMClientSideExtension;
#endif /* __cplusplus */

#endif 	/* __GPMClientSideExtension_FWD_DEFINED__ */


#ifndef __GPMCSECollection_FWD_DEFINED__
#define __GPMCSECollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMCSECollection GPMCSECollection;
#else
typedef struct GPMCSECollection GPMCSECollection;
#endif /* __cplusplus */

#endif 	/* __GPMCSECollection_FWD_DEFINED__ */


#ifndef __GPMConstants_FWD_DEFINED__
#define __GPMConstants_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMConstants GPMConstants;
#else
typedef struct GPMConstants GPMConstants;
#endif /* __cplusplus */

#endif 	/* __GPMConstants_FWD_DEFINED__ */


#ifndef __GPMResult_FWD_DEFINED__
#define __GPMResult_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMResult GPMResult;
#else
typedef struct GPMResult GPMResult;
#endif /* __cplusplus */

#endif 	/* __GPMResult_FWD_DEFINED__ */


#ifndef __GPMMapEntryCollection_FWD_DEFINED__
#define __GPMMapEntryCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMMapEntryCollection GPMMapEntryCollection;
#else
typedef struct GPMMapEntryCollection GPMMapEntryCollection;
#endif /* __cplusplus */

#endif 	/* __GPMMapEntryCollection_FWD_DEFINED__ */


#ifndef __GPMMapEntry_FWD_DEFINED__
#define __GPMMapEntry_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMMapEntry GPMMapEntry;
#else
typedef struct GPMMapEntry GPMMapEntry;
#endif /* __cplusplus */

#endif 	/* __GPMMapEntry_FWD_DEFINED__ */


#ifndef __GPMMigrationTable_FWD_DEFINED__
#define __GPMMigrationTable_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMMigrationTable GPMMigrationTable;
#else
typedef struct GPMMigrationTable GPMMigrationTable;
#endif /* __cplusplus */

#endif 	/* __GPMMigrationTable_FWD_DEFINED__ */


#ifndef __GPMBackupDirEx_FWD_DEFINED__
#define __GPMBackupDirEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMBackupDirEx GPMBackupDirEx;
#else
typedef struct GPMBackupDirEx GPMBackupDirEx;
#endif /* __cplusplus */

#endif 	/* __GPMBackupDirEx_FWD_DEFINED__ */


#ifndef __GPMStarterGPOBackupCollection_FWD_DEFINED__
#define __GPMStarterGPOBackupCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMStarterGPOBackupCollection GPMStarterGPOBackupCollection;
#else
typedef struct GPMStarterGPOBackupCollection GPMStarterGPOBackupCollection;
#endif /* __cplusplus */

#endif 	/* __GPMStarterGPOBackupCollection_FWD_DEFINED__ */


#ifndef __GPMStarterGPOBackup_FWD_DEFINED__
#define __GPMStarterGPOBackup_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMStarterGPOBackup GPMStarterGPOBackup;
#else
typedef struct GPMStarterGPOBackup GPMStarterGPOBackup;
#endif /* __cplusplus */

#endif 	/* __GPMStarterGPOBackup_FWD_DEFINED__ */


#ifndef __GPMTemplate_FWD_DEFINED__
#define __GPMTemplate_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMTemplate GPMTemplate;
#else
typedef struct GPMTemplate GPMTemplate;
#endif /* __cplusplus */

#endif 	/* __GPMTemplate_FWD_DEFINED__ */


#ifndef __GPMStarterGPOCollection_FWD_DEFINED__
#define __GPMStarterGPOCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class GPMStarterGPOCollection GPMStarterGPOCollection;
#else
typedef struct GPMStarterGPOCollection GPMStarterGPOCollection;
#endif /* __cplusplus */

#endif 	/* __GPMStarterGPOCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_gpmgmt_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0001
    {
        rsopUnknown	= 0,
        rsopPlanning	= ( rsopUnknown + 1 ) ,
        rsopLogging	= ( rsopPlanning + 1 ) 
    } 	GPMRSOPMode;

typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0002
    {
        permGPOApply	= 0x10000,
        permGPORead	= 0x10100,
        permGPOEdit	= 0x10101,
        permGPOEditSecurityAndDelete	= 0x10102,
        permGPOCustom	= 0x10103,
        permWMIFilterEdit	= 0x20000,
        permWMIFilterFullControl	= 0x20001,
        permWMIFilterCustom	= 0x20002,
        permSOMLink	= 0x1c0000,
        permSOMLogging	= 0x180100,
        permSOMPlanning	= 0x180200,
        permSOMWMICreate	= 0x100300,
        permSOMWMIFullControl	= 0x100301,
        permSOMGPOCreate	= 0x100400,
        permStarterGPORead	= 0x30500,
        permStarterGPOEdit	= 0x30501,
        permStarterGPOFullControl	= 0x30502,
        permStarterGPOCustom	= 0x30503,
        permSOMStarterGPOCreate	= 0x100500
    } 	GPMPermissionType;

typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0003
    {
        gpoPermissions	= 0,
        gpoEffectivePermissions	= ( gpoPermissions + 1 ) ,
        gpoDisplayName	= ( gpoEffectivePermissions + 1 ) ,
        gpoWMIFilter	= ( gpoDisplayName + 1 ) ,
        gpoID	= ( gpoWMIFilter + 1 ) ,
        gpoComputerExtensions	= ( gpoID + 1 ) ,
        gpoUserExtensions	= ( gpoComputerExtensions + 1 ) ,
        somLinks	= ( gpoUserExtensions + 1 ) ,
        gpoDomain	= ( somLinks + 1 ) ,
        backupMostRecent	= ( gpoDomain + 1 ) ,
        starterGPOPermissions	= ( backupMostRecent + 1 ) ,
        starterGPOEffectivePermissions	= ( starterGPOPermissions + 1 ) ,
        starterGPODisplayName	= ( starterGPOEffectivePermissions + 1 ) ,
        starterGPOID	= ( starterGPODisplayName + 1 ) ,
        starterGPODomain	= ( starterGPOID + 1 ) 
    } 	GPMSearchProperty;

typedef /* [public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0004
    {
        opEquals	= 0,
        opContains	= ( opEquals + 1 ) ,
        opNotContains	= ( opContains + 1 ) ,
        opNotEquals	= ( opNotContains + 1 ) 
    } 	GPMSearchOperation;

typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0005
    {
        repXML	= 0,
        repHTML	= ( repXML + 1 ) ,
        repInfraXML	= ( repHTML + 1 ) ,
        repInfraRefreshXML	= ( repInfraXML + 1 ) ,
        repClientHealthXML	= ( repInfraRefreshXML + 1 ) ,
        repClientHealthRefreshXML	= ( repClientHealthXML + 1 ) 
    } 	GPMReportType;

typedef /* [public][public][public][public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0006
    {
        typeUser	= 0,
        typeComputer	= ( typeUser + 1 ) ,
        typeLocalGroup	= ( typeComputer + 1 ) ,
        typeGlobalGroup	= ( typeLocalGroup + 1 ) ,
        typeUniversalGroup	= ( typeGlobalGroup + 1 ) ,
        typeUNCPath	= ( typeUniversalGroup + 1 ) ,
        typeUnknown	= ( typeUNCPath + 1 ) 
    } 	GPMEntryType;

typedef /* [public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0007
    {
        opDestinationSameAsSource	= 0,
        opDestinationNone	= ( opDestinationSameAsSource + 1 ) ,
        opDestinationByRelativeName	= ( opDestinationNone + 1 ) ,
        opDestinationSet	= ( opDestinationByRelativeName + 1 ) 
    } 	GPMDestinationOption;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0000_0008
    {
        opReportLegacy	= 0,
        opReportComments	= 1
    } 	GPMReportingOptions;

#define	GPM_USE_PDC	( 0 )

#define	GPM_USE_ANYDC	( 1 )

#define	GPM_DONOTUSE_W2KDC	( 2 )

#define	GPM_DONOT_VALIDATEDC	( 1 )

#define	GPM_MIGRATIONTABLE_ONLY	( 0x1 )

#define	GPM_PROCESS_SECURITY	( 0x2 )

#define	RSOP_NO_COMPUTER	( 0x10000 )

#define	RSOP_NO_USER	( 0x20000 )

#define	RSOP_PLANNING_ASSUME_SLOW_LINK	( 0x1 )

#define	RSOP_PLANNING_ASSUME_LOOPBACK_MERGE	( 0x2 )

#define	RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE	( 0x4 )

#define	RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE	( 0x8 )

#define	RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE	( 0x10 )

































extern RPC_IF_HANDLE __MIDL_itf_gpmgmt_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_gpmgmt_0000_0000_v0_0_s_ifspec;

#ifndef __IGPM_INTERFACE_DEFINED__
#define __IGPM_INTERFACE_DEFINED__

/* interface IGPM */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPM;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5FAE809-3BD6-4DA9-A65E-17665B41D763")
    IGPM : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDomain( 
            /* [in] */ __RPC__in BSTR bstrDomain,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomainController,
            /* [in] */ long lDCFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMDomain **pIGPMDomain) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBackupDir( 
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupDir **pIGPMBackupDir) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSitesContainer( 
            /* [in] */ __RPC__in BSTR bstrForest,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomain,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomainController,
            /* [in] */ long lDCFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSitesContainer **ppIGPMSitesContainer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRSOP( 
            /* [in] */ GPMRSOPMode gpmRSoPMode,
            /* [in] */ __RPC__in BSTR bstrNamespace,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMRSOP **ppIGPMRSOP) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreatePermission( 
            /* [in] */ __RPC__in BSTR bstrTrustee,
            /* [in] */ GPMPermissionType perm,
            /* [in] */ VARIANT_BOOL bInheritable,
            /* [retval][out] */ __RPC__deref_out_opt IGPMPermission **ppPerm) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSearchCriteria( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMSearchCriteria **ppIGPMSearchCriteria) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTrustee( 
            /* [in] */ __RPC__in BSTR bstrTrustee,
            /* [retval][out] */ __RPC__deref_out_opt IGPMTrustee **ppIGPMTrustee) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetClientSideExtensions( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMCSECollection **ppIGPMCSECollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConstants( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMConstants **ppIGPMConstants) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMigrationTable( 
            /* [in] */ __RPC__in BSTR bstrMigrationTablePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMigrationTable **ppMigrationTable) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateMigrationTable( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMMigrationTable **ppMigrationTable) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitializeReporting( 
            /* [unique][in] */ __RPC__in_opt BSTR bstrAdmPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPM * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPM * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPM * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPM * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPM * This,
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
        
        DECLSPEC_XFGVIRT(IGPM, GetDomain)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDomain )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in BSTR bstrDomain,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomainController,
            /* [in] */ long lDCFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMDomain **pIGPMDomain);
        
        DECLSPEC_XFGVIRT(IGPM, GetBackupDir)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackupDir )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupDir **pIGPMBackupDir);
        
        DECLSPEC_XFGVIRT(IGPM, GetSitesContainer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSitesContainer )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in BSTR bstrForest,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomain,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomainController,
            /* [in] */ long lDCFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSitesContainer **ppIGPMSitesContainer);
        
        DECLSPEC_XFGVIRT(IGPM, GetRSOP)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRSOP )( 
            __RPC__in IGPM * This,
            /* [in] */ GPMRSOPMode gpmRSoPMode,
            /* [in] */ __RPC__in BSTR bstrNamespace,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMRSOP **ppIGPMRSOP);
        
        DECLSPEC_XFGVIRT(IGPM, CreatePermission)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreatePermission )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in BSTR bstrTrustee,
            /* [in] */ GPMPermissionType perm,
            /* [in] */ VARIANT_BOOL bInheritable,
            /* [retval][out] */ __RPC__deref_out_opt IGPMPermission **ppPerm);
        
        DECLSPEC_XFGVIRT(IGPM, CreateSearchCriteria)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSearchCriteria )( 
            __RPC__in IGPM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSearchCriteria **ppIGPMSearchCriteria);
        
        DECLSPEC_XFGVIRT(IGPM, CreateTrustee)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTrustee )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in BSTR bstrTrustee,
            /* [retval][out] */ __RPC__deref_out_opt IGPMTrustee **ppIGPMTrustee);
        
        DECLSPEC_XFGVIRT(IGPM, GetClientSideExtensions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetClientSideExtensions )( 
            __RPC__in IGPM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMCSECollection **ppIGPMCSECollection);
        
        DECLSPEC_XFGVIRT(IGPM, GetConstants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConstants )( 
            __RPC__in IGPM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMConstants **ppIGPMConstants);
        
        DECLSPEC_XFGVIRT(IGPM, GetMigrationTable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMigrationTable )( 
            __RPC__in IGPM * This,
            /* [in] */ __RPC__in BSTR bstrMigrationTablePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMigrationTable **ppMigrationTable);
        
        DECLSPEC_XFGVIRT(IGPM, CreateMigrationTable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateMigrationTable )( 
            __RPC__in IGPM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMigrationTable **ppMigrationTable);
        
        DECLSPEC_XFGVIRT(IGPM, InitializeReporting)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeReporting )( 
            __RPC__in IGPM * This,
            /* [unique][in] */ __RPC__in_opt BSTR bstrAdmPath);
        
        END_INTERFACE
    } IGPMVtbl;

    interface IGPM
    {
        CONST_VTBL struct IGPMVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPM_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPM_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPM_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPM_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPM_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPM_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPM_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPM_GetDomain(This,bstrDomain,bstrDomainController,lDCFlags,pIGPMDomain)	\
    ( (This)->lpVtbl -> GetDomain(This,bstrDomain,bstrDomainController,lDCFlags,pIGPMDomain) ) 

#define IGPM_GetBackupDir(This,bstrBackupDir,pIGPMBackupDir)	\
    ( (This)->lpVtbl -> GetBackupDir(This,bstrBackupDir,pIGPMBackupDir) ) 

#define IGPM_GetSitesContainer(This,bstrForest,bstrDomain,bstrDomainController,lDCFlags,ppIGPMSitesContainer)	\
    ( (This)->lpVtbl -> GetSitesContainer(This,bstrForest,bstrDomain,bstrDomainController,lDCFlags,ppIGPMSitesContainer) ) 

#define IGPM_GetRSOP(This,gpmRSoPMode,bstrNamespace,lFlags,ppIGPMRSOP)	\
    ( (This)->lpVtbl -> GetRSOP(This,gpmRSoPMode,bstrNamespace,lFlags,ppIGPMRSOP) ) 

#define IGPM_CreatePermission(This,bstrTrustee,perm,bInheritable,ppPerm)	\
    ( (This)->lpVtbl -> CreatePermission(This,bstrTrustee,perm,bInheritable,ppPerm) ) 

#define IGPM_CreateSearchCriteria(This,ppIGPMSearchCriteria)	\
    ( (This)->lpVtbl -> CreateSearchCriteria(This,ppIGPMSearchCriteria) ) 

#define IGPM_CreateTrustee(This,bstrTrustee,ppIGPMTrustee)	\
    ( (This)->lpVtbl -> CreateTrustee(This,bstrTrustee,ppIGPMTrustee) ) 

#define IGPM_GetClientSideExtensions(This,ppIGPMCSECollection)	\
    ( (This)->lpVtbl -> GetClientSideExtensions(This,ppIGPMCSECollection) ) 

#define IGPM_GetConstants(This,ppIGPMConstants)	\
    ( (This)->lpVtbl -> GetConstants(This,ppIGPMConstants) ) 

#define IGPM_GetMigrationTable(This,bstrMigrationTablePath,ppMigrationTable)	\
    ( (This)->lpVtbl -> GetMigrationTable(This,bstrMigrationTablePath,ppMigrationTable) ) 

#define IGPM_CreateMigrationTable(This,ppMigrationTable)	\
    ( (This)->lpVtbl -> CreateMigrationTable(This,ppMigrationTable) ) 

#define IGPM_InitializeReporting(This,bstrAdmPath)	\
    ( (This)->lpVtbl -> InitializeReporting(This,bstrAdmPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPM_INTERFACE_DEFINED__ */


#ifndef __IGPMDomain_INTERFACE_DEFINED__
#define __IGPMDomain_INTERFACE_DEFINED__

/* interface IGPMDomain */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMDomain;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6B21CC14-5A00-4F44-A738-FEEC8A94C7E3")
    IGPMDomain : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainController( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Domain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateGPO( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppNewGPO) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGPO( 
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppGPO) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchGPOs( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOCollection **ppIGPMGPOCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RestoreGPO( 
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [in] */ long lDCFlags,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSOM( 
            /* [unique][in] */ __RPC__in_opt BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppSOM) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchSOMs( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOMCollection **ppIGPMSOMCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWMIFilter( 
            /* [in] */ __RPC__in BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppWMIFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchWMIFilters( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilterCollection **ppIGPMWMIFilterCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMDomainVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMDomain * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMDomain * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMDomain * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMDomain * This,
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
        
        DECLSPEC_XFGVIRT(IGPMDomain, get_DomainController)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainController )( 
            __RPC__in IGPMDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in IGPMDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain, CreateGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateGPO )( 
            __RPC__in IGPMDomain * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppNewGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGPO )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchGPOs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchGPOs )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOCollection **ppIGPMGPOCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain, RestoreGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreGPO )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [in] */ long lDCFlags,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetSOM)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSOM )( 
            __RPC__in IGPMDomain * This,
            /* [unique][in] */ __RPC__in_opt BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppSOM);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchSOMs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchSOMs )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOMCollection **ppIGPMSOMCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIFilter )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchWMIFilters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchWMIFilters )( 
            __RPC__in IGPMDomain * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilterCollection **ppIGPMWMIFilterCollection);
        
        END_INTERFACE
    } IGPMDomainVtbl;

    interface IGPMDomain
    {
        CONST_VTBL struct IGPMDomainVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMDomain_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMDomain_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMDomain_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMDomain_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMDomain_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMDomain_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMDomain_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMDomain_get_DomainController(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainController(This,pVal) ) 

#define IGPMDomain_get_Domain(This,pVal)	\
    ( (This)->lpVtbl -> get_Domain(This,pVal) ) 

#define IGPMDomain_CreateGPO(This,ppNewGPO)	\
    ( (This)->lpVtbl -> CreateGPO(This,ppNewGPO) ) 

#define IGPMDomain_GetGPO(This,bstrGuid,ppGPO)	\
    ( (This)->lpVtbl -> GetGPO(This,bstrGuid,ppGPO) ) 

#define IGPMDomain_SearchGPOs(This,pIGPMSearchCriteria,ppIGPMGPOCollection)	\
    ( (This)->lpVtbl -> SearchGPOs(This,pIGPMSearchCriteria,ppIGPMGPOCollection) ) 

#define IGPMDomain_RestoreGPO(This,pIGPMBackup,lDCFlags,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> RestoreGPO(This,pIGPMBackup,lDCFlags,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMDomain_GetSOM(This,bstrPath,ppSOM)	\
    ( (This)->lpVtbl -> GetSOM(This,bstrPath,ppSOM) ) 

#define IGPMDomain_SearchSOMs(This,pIGPMSearchCriteria,ppIGPMSOMCollection)	\
    ( (This)->lpVtbl -> SearchSOMs(This,pIGPMSearchCriteria,ppIGPMSOMCollection) ) 

#define IGPMDomain_GetWMIFilter(This,bstrPath,ppWMIFilter)	\
    ( (This)->lpVtbl -> GetWMIFilter(This,bstrPath,ppWMIFilter) ) 

#define IGPMDomain_SearchWMIFilters(This,pIGPMSearchCriteria,ppIGPMWMIFilterCollection)	\
    ( (This)->lpVtbl -> SearchWMIFilters(This,pIGPMSearchCriteria,ppIGPMWMIFilterCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMDomain_INTERFACE_DEFINED__ */


#ifndef __IGPMBackupDir_INTERFACE_DEFINED__
#define __IGPMBackupDir_INTERFACE_DEFINED__

/* interface IGPMBackupDir */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMBackupDir;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1568BED-0A93-4ACC-810F-AFE7081019B9")
    IGPMBackupDir : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBackup( 
            /* [in] */ __RPC__in BSTR bstrID,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackup **ppBackup) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchBackups( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupCollection **ppIGPMBackupCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMBackupDirVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMBackupDir * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMBackupDir * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMBackupDir * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMBackupDir * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMBackupDir * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMBackupDir * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMBackupDir * This,
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
        
        DECLSPEC_XFGVIRT(IGPMBackupDir, get_BackupDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupDirectory )( 
            __RPC__in IGPMBackupDir * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackupDir, GetBackup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackup )( 
            __RPC__in IGPMBackupDir * This,
            /* [in] */ __RPC__in BSTR bstrID,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackup **ppBackup);
        
        DECLSPEC_XFGVIRT(IGPMBackupDir, SearchBackups)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchBackups )( 
            __RPC__in IGPMBackupDir * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupCollection **ppIGPMBackupCollection);
        
        END_INTERFACE
    } IGPMBackupDirVtbl;

    interface IGPMBackupDir
    {
        CONST_VTBL struct IGPMBackupDirVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMBackupDir_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMBackupDir_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMBackupDir_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMBackupDir_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMBackupDir_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMBackupDir_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMBackupDir_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMBackupDir_get_BackupDirectory(This,pVal)	\
    ( (This)->lpVtbl -> get_BackupDirectory(This,pVal) ) 

#define IGPMBackupDir_GetBackup(This,bstrID,ppBackup)	\
    ( (This)->lpVtbl -> GetBackup(This,bstrID,ppBackup) ) 

#define IGPMBackupDir_SearchBackups(This,pIGPMSearchCriteria,ppIGPMBackupCollection)	\
    ( (This)->lpVtbl -> SearchBackups(This,pIGPMSearchCriteria,ppIGPMBackupCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMBackupDir_INTERFACE_DEFINED__ */


#ifndef __IGPMSitesContainer_INTERFACE_DEFINED__
#define __IGPMSitesContainer_INTERFACE_DEFINED__

/* interface IGPMSitesContainer */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMSitesContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4725A899-2782-4D27-A6BB-D499246FFD72")
    IGPMSitesContainer : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainController( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Domain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Forest( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSite( 
            /* [in] */ __RPC__in BSTR bstrSiteName,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppSOM) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchSites( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOMCollection **ppIGPMSOMCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMSitesContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMSitesContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMSitesContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMSitesContainer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMSitesContainer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMSitesContainer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMSitesContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMSitesContainer * This,
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
        
        DECLSPEC_XFGVIRT(IGPMSitesContainer, get_DomainController)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainController )( 
            __RPC__in IGPMSitesContainer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSitesContainer, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in IGPMSitesContainer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSitesContainer, get_Forest)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Forest )( 
            __RPC__in IGPMSitesContainer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSitesContainer, GetSite)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSite )( 
            __RPC__in IGPMSitesContainer * This,
            /* [in] */ __RPC__in BSTR bstrSiteName,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppSOM);
        
        DECLSPEC_XFGVIRT(IGPMSitesContainer, SearchSites)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchSites )( 
            __RPC__in IGPMSitesContainer * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOMCollection **ppIGPMSOMCollection);
        
        END_INTERFACE
    } IGPMSitesContainerVtbl;

    interface IGPMSitesContainer
    {
        CONST_VTBL struct IGPMSitesContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMSitesContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMSitesContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMSitesContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMSitesContainer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMSitesContainer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMSitesContainer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMSitesContainer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMSitesContainer_get_DomainController(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainController(This,pVal) ) 

#define IGPMSitesContainer_get_Domain(This,pVal)	\
    ( (This)->lpVtbl -> get_Domain(This,pVal) ) 

#define IGPMSitesContainer_get_Forest(This,pVal)	\
    ( (This)->lpVtbl -> get_Forest(This,pVal) ) 

#define IGPMSitesContainer_GetSite(This,bstrSiteName,ppSOM)	\
    ( (This)->lpVtbl -> GetSite(This,bstrSiteName,ppSOM) ) 

#define IGPMSitesContainer_SearchSites(This,pIGPMSearchCriteria,ppIGPMSOMCollection)	\
    ( (This)->lpVtbl -> SearchSites(This,pIGPMSearchCriteria,ppIGPMSOMCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMSitesContainer_INTERFACE_DEFINED__ */


#ifndef __IGPMSearchCriteria_INTERFACE_DEFINED__
#define __IGPMSearchCriteria_INTERFACE_DEFINED__

/* interface IGPMSearchCriteria */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMSearchCriteria;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6F11C42-829B-48D4-83F5-3615B67DFC22")
    IGPMSearchCriteria : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ GPMSearchProperty searchProperty,
            /* [in] */ GPMSearchOperation searchOperation,
            /* [in] */ VARIANT varValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMSearchCriteriaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMSearchCriteria * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMSearchCriteria * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMSearchCriteria * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMSearchCriteria * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMSearchCriteria * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMSearchCriteria * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMSearchCriteria * This,
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
        
        DECLSPEC_XFGVIRT(IGPMSearchCriteria, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IGPMSearchCriteria * This,
            /* [in] */ GPMSearchProperty searchProperty,
            /* [in] */ GPMSearchOperation searchOperation,
            /* [in] */ VARIANT varValue);
        
        END_INTERFACE
    } IGPMSearchCriteriaVtbl;

    interface IGPMSearchCriteria
    {
        CONST_VTBL struct IGPMSearchCriteriaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMSearchCriteria_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMSearchCriteria_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMSearchCriteria_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMSearchCriteria_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMSearchCriteria_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMSearchCriteria_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMSearchCriteria_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMSearchCriteria_Add(This,searchProperty,searchOperation,varValue)	\
    ( (This)->lpVtbl -> Add(This,searchProperty,searchOperation,varValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMSearchCriteria_INTERFACE_DEFINED__ */


#ifndef __IGPMTrustee_INTERFACE_DEFINED__
#define __IGPMTrustee_INTERFACE_DEFINED__

/* interface IGPMTrustee */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMTrustee;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3B466DA8-C1A4-4B2A-999A-BEFCDD56CEFB")
    IGPMTrustee : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrusteeSid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrusteeName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrusteeDomain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrusteeDSPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrusteeType( 
            /* [retval][out] */ __RPC__out long *lVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMTrusteeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMTrustee * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMTrustee * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMTrustee * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMTrustee * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMTrustee * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMTrustee * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMTrustee * This,
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
        
        DECLSPEC_XFGVIRT(IGPMTrustee, get_TrusteeSid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrusteeSid )( 
            __RPC__in IGPMTrustee * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMTrustee, get_TrusteeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrusteeName )( 
            __RPC__in IGPMTrustee * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMTrustee, get_TrusteeDomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrusteeDomain )( 
            __RPC__in IGPMTrustee * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMTrustee, get_TrusteeDSPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrusteeDSPath )( 
            __RPC__in IGPMTrustee * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMTrustee, get_TrusteeType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrusteeType )( 
            __RPC__in IGPMTrustee * This,
            /* [retval][out] */ __RPC__out long *lVal);
        
        END_INTERFACE
    } IGPMTrusteeVtbl;

    interface IGPMTrustee
    {
        CONST_VTBL struct IGPMTrusteeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMTrustee_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMTrustee_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMTrustee_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMTrustee_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMTrustee_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMTrustee_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMTrustee_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMTrustee_get_TrusteeSid(This,bstrVal)	\
    ( (This)->lpVtbl -> get_TrusteeSid(This,bstrVal) ) 

#define IGPMTrustee_get_TrusteeName(This,bstrVal)	\
    ( (This)->lpVtbl -> get_TrusteeName(This,bstrVal) ) 

#define IGPMTrustee_get_TrusteeDomain(This,bstrVal)	\
    ( (This)->lpVtbl -> get_TrusteeDomain(This,bstrVal) ) 

#define IGPMTrustee_get_TrusteeDSPath(This,pVal)	\
    ( (This)->lpVtbl -> get_TrusteeDSPath(This,pVal) ) 

#define IGPMTrustee_get_TrusteeType(This,lVal)	\
    ( (This)->lpVtbl -> get_TrusteeType(This,lVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMTrustee_INTERFACE_DEFINED__ */


#ifndef __IGPMPermission_INTERFACE_DEFINED__
#define __IGPMPermission_INTERFACE_DEFINED__

/* interface IGPMPermission */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMPermission;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("35EBCA40-E1A1-4A02-8905-D79416FB464A")
    IGPMPermission : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Inherited( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Inheritable( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Denied( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Permission( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Trustee( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMTrustee **ppIGPMTrustee) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMPermissionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMPermission * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMPermission * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMPermission * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMPermission * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMPermission * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMPermission * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMPermission * This,
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
        
        DECLSPEC_XFGVIRT(IGPMPermission, get_Inherited)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Inherited )( 
            __RPC__in IGPMPermission * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IGPMPermission, get_Inheritable)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Inheritable )( 
            __RPC__in IGPMPermission * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IGPMPermission, get_Denied)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Denied )( 
            __RPC__in IGPMPermission * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IGPMPermission, get_Permission)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Permission )( 
            __RPC__in IGPMPermission * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMPermission, get_Trustee)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Trustee )( 
            __RPC__in IGPMPermission * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMTrustee **ppIGPMTrustee);
        
        END_INTERFACE
    } IGPMPermissionVtbl;

    interface IGPMPermission
    {
        CONST_VTBL struct IGPMPermissionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMPermission_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMPermission_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMPermission_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMPermission_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMPermission_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMPermission_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMPermission_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMPermission_get_Inherited(This,pVal)	\
    ( (This)->lpVtbl -> get_Inherited(This,pVal) ) 

#define IGPMPermission_get_Inheritable(This,pVal)	\
    ( (This)->lpVtbl -> get_Inheritable(This,pVal) ) 

#define IGPMPermission_get_Denied(This,pVal)	\
    ( (This)->lpVtbl -> get_Denied(This,pVal) ) 

#define IGPMPermission_get_Permission(This,pVal)	\
    ( (This)->lpVtbl -> get_Permission(This,pVal) ) 

#define IGPMPermission_get_Trustee(This,ppIGPMTrustee)	\
    ( (This)->lpVtbl -> get_Trustee(This,ppIGPMTrustee) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMPermission_INTERFACE_DEFINED__ */


#ifndef __IGPMSecurityInfo_INTERFACE_DEFINED__
#define __IGPMSecurityInfo_INTERFACE_DEFINED__

/* interface IGPMSecurityInfo */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMSecurityInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B6C31ED4-1C93-4D3E-AE84-EB6D61161B60")
    IGPMSecurityInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppEnum) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IGPMPermission *pPerm) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in_opt IGPMPermission *pPerm) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveTrustee( 
            /* [in] */ __RPC__in BSTR bstrTrustee) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMSecurityInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMSecurityInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMSecurityInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMSecurityInfo * This,
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
        
        DECLSPEC_XFGVIRT(IGPMSecurityInfo, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSecurityInfo, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMSecurityInfo * This,
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSecurityInfo, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppEnum);
        
        DECLSPEC_XFGVIRT(IGPMSecurityInfo, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [in] */ __RPC__in_opt IGPMPermission *pPerm);
        
        DECLSPEC_XFGVIRT(IGPMSecurityInfo, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [in] */ __RPC__in_opt IGPMPermission *pPerm);
        
        DECLSPEC_XFGVIRT(IGPMSecurityInfo, RemoveTrustee)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveTrustee )( 
            __RPC__in IGPMSecurityInfo * This,
            /* [in] */ __RPC__in BSTR bstrTrustee);
        
        END_INTERFACE
    } IGPMSecurityInfoVtbl;

    interface IGPMSecurityInfo
    {
        CONST_VTBL struct IGPMSecurityInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMSecurityInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMSecurityInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMSecurityInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMSecurityInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMSecurityInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMSecurityInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMSecurityInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMSecurityInfo_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMSecurityInfo_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMSecurityInfo_get__NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnum) ) 

#define IGPMSecurityInfo_Add(This,pPerm)	\
    ( (This)->lpVtbl -> Add(This,pPerm) ) 

#define IGPMSecurityInfo_Remove(This,pPerm)	\
    ( (This)->lpVtbl -> Remove(This,pPerm) ) 

#define IGPMSecurityInfo_RemoveTrustee(This,bstrTrustee)	\
    ( (This)->lpVtbl -> RemoveTrustee(This,bstrTrustee) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMSecurityInfo_INTERFACE_DEFINED__ */


#ifndef __IGPMBackup_INTERFACE_DEFINED__
#define __IGPMBackup_INTERFACE_DEFINED__

/* interface IGPMBackup */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMBackup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8A16A35-3B0D-416B-8D02-4DF6F95A7119")
    IGPMBackup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GPOID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GPODomain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GPODisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Timestamp( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Comment( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupDir( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReport( 
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReportToFile( 
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMBackupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMBackup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMBackup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMBackup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMBackup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMBackup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMBackup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMBackup * This,
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
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_GPOID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GPOID )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_GPODomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GPODomain )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_GPODisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GPODisplayName )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_Timestamp)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timestamp )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_Comment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Comment )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, get_BackupDir)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupDir )( 
            __RPC__in IGPMBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackup, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMBackup * This);
        
        DECLSPEC_XFGVIRT(IGPMBackup, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMBackup * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMBackup, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMBackup * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        END_INTERFACE
    } IGPMBackupVtbl;

    interface IGPMBackup
    {
        CONST_VTBL struct IGPMBackupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMBackup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMBackup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMBackup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMBackup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMBackup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMBackup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMBackup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMBackup_get_ID(This,pVal)	\
    ( (This)->lpVtbl -> get_ID(This,pVal) ) 

#define IGPMBackup_get_GPOID(This,pVal)	\
    ( (This)->lpVtbl -> get_GPOID(This,pVal) ) 

#define IGPMBackup_get_GPODomain(This,pVal)	\
    ( (This)->lpVtbl -> get_GPODomain(This,pVal) ) 

#define IGPMBackup_get_GPODisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_GPODisplayName(This,pVal) ) 

#define IGPMBackup_get_Timestamp(This,pVal)	\
    ( (This)->lpVtbl -> get_Timestamp(This,pVal) ) 

#define IGPMBackup_get_Comment(This,pVal)	\
    ( (This)->lpVtbl -> get_Comment(This,pVal) ) 

#define IGPMBackup_get_BackupDir(This,pVal)	\
    ( (This)->lpVtbl -> get_BackupDir(This,pVal) ) 

#define IGPMBackup_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IGPMBackup_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMBackup_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMBackup_INTERFACE_DEFINED__ */


#ifndef __IGPMBackupCollection_INTERFACE_DEFINED__
#define __IGPMBackupCollection_INTERFACE_DEFINED__

/* interface IGPMBackupCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMBackupCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C786FC0F-26D8-4BAB-A745-39CA7E800CAC")
    IGPMBackupCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMBackup) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMBackupCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMBackupCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMBackupCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMBackupCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMBackupCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMBackupCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMBackupCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMBackupCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMBackupCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMBackupCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackupCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMBackupCollection * This,
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMBackupCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMBackupCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMBackup);
        
        END_INTERFACE
    } IGPMBackupCollectionVtbl;

    interface IGPMBackupCollection
    {
        CONST_VTBL struct IGPMBackupCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMBackupCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMBackupCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMBackupCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMBackupCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMBackupCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMBackupCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMBackupCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMBackupCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMBackupCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMBackupCollection_get__NewEnum(This,ppIGPMBackup)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMBackup) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMBackupCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMSOM_INTERFACE_DEFINED__
#define __IGPMSOM_INTERFACE_DEFINED__

/* interface IGPMSOM */
/* [unique][helpstring][dual][uuid][object] */ 

typedef /* [public][public][public][public][public] */ 
enum __MIDL_IGPMSOM_0001
    {
        somSite	= 0,
        somDomain	= ( somSite + 1 ) ,
        somOU	= ( somDomain + 1 ) 
    } 	GPMSOMType;


EXTERN_C const IID IID_IGPMSOM;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0A7F09E-05A1-4F0C-8158-9E5C33684F6B")
    IGPMSOM : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GPOInheritanceBlocked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_GPOInheritanceBlocked( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateGPOLink( 
            /* [in] */ long lLinkPos,
            /* [in] */ __RPC__in_opt IGPMGPO *pGPO,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOLink **ppNewGPOLink) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out GPMSOMType *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGPOLinks( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOLinksCollection **ppGPOLinks) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInheritedGPOLinks( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOLinksCollection **ppGPOLinks) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityInfo( 
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMSOMVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMSOM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMSOM * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMSOM * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMSOM * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMSOM * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMSOM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMSOM * This,
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
        
        DECLSPEC_XFGVIRT(IGPMSOM, get_GPOInheritanceBlocked)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GPOInheritanceBlocked )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSOM, put_GPOInheritanceBlocked)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GPOInheritanceBlocked )( 
            __RPC__in IGPMSOM * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IGPMSOM, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSOM, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSOM, CreateGPOLink)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateGPOLink )( 
            __RPC__in IGPMSOM * This,
            /* [in] */ long lLinkPos,
            /* [in] */ __RPC__in_opt IGPMGPO *pGPO,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOLink **ppNewGPOLink);
        
        DECLSPEC_XFGVIRT(IGPMSOM, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSOM, GetGPOLinks)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGPOLinks )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOLinksCollection **ppGPOLinks);
        
        DECLSPEC_XFGVIRT(IGPMSOM, GetInheritedGPOLinks)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInheritedGPOLinks )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOLinksCollection **ppGPOLinks);
        
        DECLSPEC_XFGVIRT(IGPMSOM, GetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityInfo )( 
            __RPC__in IGPMSOM * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMSOM, SetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityInfo )( 
            __RPC__in IGPMSOM * This,
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo);
        
        END_INTERFACE
    } IGPMSOMVtbl;

    interface IGPMSOM
    {
        CONST_VTBL struct IGPMSOMVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMSOM_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMSOM_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMSOM_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMSOM_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMSOM_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMSOM_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMSOM_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMSOM_get_GPOInheritanceBlocked(This,pVal)	\
    ( (This)->lpVtbl -> get_GPOInheritanceBlocked(This,pVal) ) 

#define IGPMSOM_put_GPOInheritanceBlocked(This,newVal)	\
    ( (This)->lpVtbl -> put_GPOInheritanceBlocked(This,newVal) ) 

#define IGPMSOM_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IGPMSOM_get_Path(This,pVal)	\
    ( (This)->lpVtbl -> get_Path(This,pVal) ) 

#define IGPMSOM_CreateGPOLink(This,lLinkPos,pGPO,ppNewGPOLink)	\
    ( (This)->lpVtbl -> CreateGPOLink(This,lLinkPos,pGPO,ppNewGPOLink) ) 

#define IGPMSOM_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 

#define IGPMSOM_GetGPOLinks(This,ppGPOLinks)	\
    ( (This)->lpVtbl -> GetGPOLinks(This,ppGPOLinks) ) 

#define IGPMSOM_GetInheritedGPOLinks(This,ppGPOLinks)	\
    ( (This)->lpVtbl -> GetInheritedGPOLinks(This,ppGPOLinks) ) 

#define IGPMSOM_GetSecurityInfo(This,ppSecurityInfo)	\
    ( (This)->lpVtbl -> GetSecurityInfo(This,ppSecurityInfo) ) 

#define IGPMSOM_SetSecurityInfo(This,pSecurityInfo)	\
    ( (This)->lpVtbl -> SetSecurityInfo(This,pSecurityInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMSOM_INTERFACE_DEFINED__ */


#ifndef __IGPMSOMCollection_INTERFACE_DEFINED__
#define __IGPMSOMCollection_INTERFACE_DEFINED__

/* interface IGPMSOMCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMSOMCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ADC1688E-00E4-4495-ABBA-BED200DF0CAB")
    IGPMSOMCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMSOM) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMSOMCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMSOMCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMSOMCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMSOMCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMSOMCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMSOMCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMSOMCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMSOMCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMSOMCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMSOMCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSOMCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMSOMCollection * This,
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMSOMCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMSOMCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMSOM);
        
        END_INTERFACE
    } IGPMSOMCollectionVtbl;

    interface IGPMSOMCollection
    {
        CONST_VTBL struct IGPMSOMCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMSOMCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMSOMCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMSOMCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMSOMCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMSOMCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMSOMCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMSOMCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMSOMCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMSOMCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMSOMCollection_get__NewEnum(This,ppIGPMSOM)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMSOM) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMSOMCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMWMIFilter_INTERFACE_DEFINED__
#define __IGPMWMIFilter_INTERFACE_DEFINED__

/* interface IGPMWMIFilter */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMWMIFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EF2FF9B4-3C27-459A-B979-038305CEC75D")
    IGPMWMIFilter : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetQueryList( 
            /* [retval][out] */ __RPC__out VARIANT *pQryList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityInfo( 
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMWMIFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMWMIFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMWMIFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMWMIFilter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMWMIFilter * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMWMIFilter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMWMIFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMWMIFilter * This,
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
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IGPMWMIFilter * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IGPMWMIFilter * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IGPMWMIFilter * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IGPMWMIFilter * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IGPMWMIFilter * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, GetQueryList)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetQueryList )( 
            __RPC__in IGPMWMIFilter * This,
            /* [retval][out] */ __RPC__out VARIANT *pQryList);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, GetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityInfo )( 
            __RPC__in IGPMWMIFilter * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilter, SetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityInfo )( 
            __RPC__in IGPMWMIFilter * This,
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo);
        
        END_INTERFACE
    } IGPMWMIFilterVtbl;

    interface IGPMWMIFilter
    {
        CONST_VTBL struct IGPMWMIFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMWMIFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMWMIFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMWMIFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMWMIFilter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMWMIFilter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMWMIFilter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMWMIFilter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMWMIFilter_get_Path(This,pVal)	\
    ( (This)->lpVtbl -> get_Path(This,pVal) ) 

#define IGPMWMIFilter_put_Name(This,newVal)	\
    ( (This)->lpVtbl -> put_Name(This,newVal) ) 

#define IGPMWMIFilter_get_Name(This,pVal)	\
    ( (This)->lpVtbl -> get_Name(This,pVal) ) 

#define IGPMWMIFilter_put_Description(This,newVal)	\
    ( (This)->lpVtbl -> put_Description(This,newVal) ) 

#define IGPMWMIFilter_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IGPMWMIFilter_GetQueryList(This,pQryList)	\
    ( (This)->lpVtbl -> GetQueryList(This,pQryList) ) 

#define IGPMWMIFilter_GetSecurityInfo(This,ppSecurityInfo)	\
    ( (This)->lpVtbl -> GetSecurityInfo(This,ppSecurityInfo) ) 

#define IGPMWMIFilter_SetSecurityInfo(This,pSecurityInfo)	\
    ( (This)->lpVtbl -> SetSecurityInfo(This,pSecurityInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMWMIFilter_INTERFACE_DEFINED__ */


#ifndef __IGPMWMIFilterCollection_INTERFACE_DEFINED__
#define __IGPMWMIFilterCollection_INTERFACE_DEFINED__

/* interface IGPMWMIFilterCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMWMIFilterCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5782D582-1A36-4661-8A94-C3C32551945B")
    IGPMWMIFilterCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMWMIFilterCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMWMIFilterCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMWMIFilterCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMWMIFilterCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMWMIFilterCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMWMIFilterCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMWMIFilterCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMWMIFilterCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMWMIFilterCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMWMIFilterCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilterCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMWMIFilterCollection * This,
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMWMIFilterCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMWMIFilterCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **pVal);
        
        END_INTERFACE
    } IGPMWMIFilterCollectionVtbl;

    interface IGPMWMIFilterCollection
    {
        CONST_VTBL struct IGPMWMIFilterCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMWMIFilterCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMWMIFilterCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMWMIFilterCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMWMIFilterCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMWMIFilterCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMWMIFilterCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMWMIFilterCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMWMIFilterCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMWMIFilterCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMWMIFilterCollection_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMWMIFilterCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMRSOP_INTERFACE_DEFINED__
#define __IGPMRSOP_INTERFACE_DEFINED__

/* interface IGPMRSOP */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMRSOP;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49ED785A-3237-4FF2-B1F0-FDF5A8D5A1EE")
    IGPMRSOP : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Namespace( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LoggingComputer( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LoggingComputer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LoggingUser( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LoggingUser( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LoggingFlags( 
            /* [in] */ long lVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LoggingFlags( 
            /* [retval][out] */ __RPC__out long *lVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningFlags( 
            /* [in] */ long lVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningFlags( 
            /* [retval][out] */ __RPC__out long *lVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningDomainController( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningDomainController( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningSiteName( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningSiteName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningUser( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningUser( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningUserSOM( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningUserSOM( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningUserWMIFilters( 
            /* [in] */ VARIANT varVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningUserWMIFilters( 
            /* [retval][out] */ __RPC__out VARIANT *varVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningUserSecurityGroups( 
            /* [in] */ VARIANT varVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningUserSecurityGroups( 
            /* [retval][out] */ __RPC__out VARIANT *varVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningComputer( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningComputer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningComputerSOM( 
            /* [in] */ __RPC__in BSTR bstrVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningComputerSOM( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningComputerWMIFilters( 
            /* [in] */ VARIANT varVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningComputerWMIFilters( 
            /* [retval][out] */ __RPC__out VARIANT *varVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlanningComputerSecurityGroups( 
            /* [in] */ VARIANT varVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlanningComputerSecurityGroups( 
            /* [retval][out] */ __RPC__out VARIANT *varVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LoggingEnumerateUsers( 
            /* [retval][out] */ __RPC__out VARIANT *varVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateQueryResults( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseQueryResults( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReport( 
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReportToFile( 
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMRSOPVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMRSOP * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMRSOP * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMRSOP * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMRSOP * This,
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
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_Mode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_Namespace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Namespace )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_LoggingComputer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoggingComputer )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_LoggingComputer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoggingComputer )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_LoggingUser)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoggingUser )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_LoggingUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoggingUser )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_LoggingFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LoggingFlags )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ long lVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_LoggingFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoggingFlags )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out long *lVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningFlags )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ long lVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningFlags )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out long *lVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningDomainController)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningDomainController )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningDomainController)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningDomainController )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningSiteName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningSiteName )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningSiteName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningSiteName )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningUser)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningUser )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningUser )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningUserSOM)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningUserSOM )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningUserSOM)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningUserSOM )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningUserWMIFilters)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningUserWMIFilters )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ VARIANT varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningUserWMIFilters)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningUserWMIFilters )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out VARIANT *varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningUserSecurityGroups)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningUserSecurityGroups )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ VARIANT varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningUserSecurityGroups)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningUserSecurityGroups )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out VARIANT *varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningComputer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningComputer )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningComputer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningComputer )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningComputerSOM)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningComputerSOM )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ __RPC__in BSTR bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningComputerSOM)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningComputerSOM )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningComputerWMIFilters)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningComputerWMIFilters )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ VARIANT varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningComputerWMIFilters)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningComputerWMIFilters )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out VARIANT *varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, put_PlanningComputerSecurityGroups)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlanningComputerSecurityGroups )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ VARIANT varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, get_PlanningComputerSecurityGroups)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlanningComputerSecurityGroups )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out VARIANT *varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, LoggingEnumerateUsers)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoggingEnumerateUsers )( 
            __RPC__in IGPMRSOP * This,
            /* [retval][out] */ __RPC__out VARIANT *varVal);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, CreateQueryResults)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateQueryResults )( 
            __RPC__in IGPMRSOP * This);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, ReleaseQueryResults)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseQueryResults )( 
            __RPC__in IGPMRSOP * This);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMRSOP, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMRSOP * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        END_INTERFACE
    } IGPMRSOPVtbl;

    interface IGPMRSOP
    {
        CONST_VTBL struct IGPMRSOPVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMRSOP_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMRSOP_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMRSOP_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMRSOP_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMRSOP_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMRSOP_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMRSOP_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMRSOP_get_Mode(This,pVal)	\
    ( (This)->lpVtbl -> get_Mode(This,pVal) ) 

#define IGPMRSOP_get_Namespace(This,bstrVal)	\
    ( (This)->lpVtbl -> get_Namespace(This,bstrVal) ) 

#define IGPMRSOP_put_LoggingComputer(This,bstrVal)	\
    ( (This)->lpVtbl -> put_LoggingComputer(This,bstrVal) ) 

#define IGPMRSOP_get_LoggingComputer(This,bstrVal)	\
    ( (This)->lpVtbl -> get_LoggingComputer(This,bstrVal) ) 

#define IGPMRSOP_put_LoggingUser(This,bstrVal)	\
    ( (This)->lpVtbl -> put_LoggingUser(This,bstrVal) ) 

#define IGPMRSOP_get_LoggingUser(This,bstrVal)	\
    ( (This)->lpVtbl -> get_LoggingUser(This,bstrVal) ) 

#define IGPMRSOP_put_LoggingFlags(This,lVal)	\
    ( (This)->lpVtbl -> put_LoggingFlags(This,lVal) ) 

#define IGPMRSOP_get_LoggingFlags(This,lVal)	\
    ( (This)->lpVtbl -> get_LoggingFlags(This,lVal) ) 

#define IGPMRSOP_put_PlanningFlags(This,lVal)	\
    ( (This)->lpVtbl -> put_PlanningFlags(This,lVal) ) 

#define IGPMRSOP_get_PlanningFlags(This,lVal)	\
    ( (This)->lpVtbl -> get_PlanningFlags(This,lVal) ) 

#define IGPMRSOP_put_PlanningDomainController(This,bstrVal)	\
    ( (This)->lpVtbl -> put_PlanningDomainController(This,bstrVal) ) 

#define IGPMRSOP_get_PlanningDomainController(This,bstrVal)	\
    ( (This)->lpVtbl -> get_PlanningDomainController(This,bstrVal) ) 

#define IGPMRSOP_put_PlanningSiteName(This,bstrVal)	\
    ( (This)->lpVtbl -> put_PlanningSiteName(This,bstrVal) ) 

#define IGPMRSOP_get_PlanningSiteName(This,bstrVal)	\
    ( (This)->lpVtbl -> get_PlanningSiteName(This,bstrVal) ) 

#define IGPMRSOP_put_PlanningUser(This,bstrVal)	\
    ( (This)->lpVtbl -> put_PlanningUser(This,bstrVal) ) 

#define IGPMRSOP_get_PlanningUser(This,bstrVal)	\
    ( (This)->lpVtbl -> get_PlanningUser(This,bstrVal) ) 

#define IGPMRSOP_put_PlanningUserSOM(This,bstrVal)	\
    ( (This)->lpVtbl -> put_PlanningUserSOM(This,bstrVal) ) 

#define IGPMRSOP_get_PlanningUserSOM(This,bstrVal)	\
    ( (This)->lpVtbl -> get_PlanningUserSOM(This,bstrVal) ) 

#define IGPMRSOP_put_PlanningUserWMIFilters(This,varVal)	\
    ( (This)->lpVtbl -> put_PlanningUserWMIFilters(This,varVal) ) 

#define IGPMRSOP_get_PlanningUserWMIFilters(This,varVal)	\
    ( (This)->lpVtbl -> get_PlanningUserWMIFilters(This,varVal) ) 

#define IGPMRSOP_put_PlanningUserSecurityGroups(This,varVal)	\
    ( (This)->lpVtbl -> put_PlanningUserSecurityGroups(This,varVal) ) 

#define IGPMRSOP_get_PlanningUserSecurityGroups(This,varVal)	\
    ( (This)->lpVtbl -> get_PlanningUserSecurityGroups(This,varVal) ) 

#define IGPMRSOP_put_PlanningComputer(This,bstrVal)	\
    ( (This)->lpVtbl -> put_PlanningComputer(This,bstrVal) ) 

#define IGPMRSOP_get_PlanningComputer(This,bstrVal)	\
    ( (This)->lpVtbl -> get_PlanningComputer(This,bstrVal) ) 

#define IGPMRSOP_put_PlanningComputerSOM(This,bstrVal)	\
    ( (This)->lpVtbl -> put_PlanningComputerSOM(This,bstrVal) ) 

#define IGPMRSOP_get_PlanningComputerSOM(This,bstrVal)	\
    ( (This)->lpVtbl -> get_PlanningComputerSOM(This,bstrVal) ) 

#define IGPMRSOP_put_PlanningComputerWMIFilters(This,varVal)	\
    ( (This)->lpVtbl -> put_PlanningComputerWMIFilters(This,varVal) ) 

#define IGPMRSOP_get_PlanningComputerWMIFilters(This,varVal)	\
    ( (This)->lpVtbl -> get_PlanningComputerWMIFilters(This,varVal) ) 

#define IGPMRSOP_put_PlanningComputerSecurityGroups(This,varVal)	\
    ( (This)->lpVtbl -> put_PlanningComputerSecurityGroups(This,varVal) ) 

#define IGPMRSOP_get_PlanningComputerSecurityGroups(This,varVal)	\
    ( (This)->lpVtbl -> get_PlanningComputerSecurityGroups(This,varVal) ) 

#define IGPMRSOP_LoggingEnumerateUsers(This,varVal)	\
    ( (This)->lpVtbl -> LoggingEnumerateUsers(This,varVal) ) 

#define IGPMRSOP_CreateQueryResults(This)	\
    ( (This)->lpVtbl -> CreateQueryResults(This) ) 

#define IGPMRSOP_ReleaseQueryResults(This)	\
    ( (This)->lpVtbl -> ReleaseQueryResults(This) ) 

#define IGPMRSOP_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMRSOP_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMRSOP_INTERFACE_DEFINED__ */


#ifndef __IGPMGPO_INTERFACE_DEFINED__
#define __IGPMGPO_INTERFACE_DEFINED__

/* interface IGPMGPO */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMGPO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("58CC4352-1CA3-48E5-9864-1DA4D6E0D60F")
    IGPMGPO : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Path( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CreationTime( 
            /* [retval][out] */ __RPC__out DATE *pDate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModificationTime( 
            /* [retval][out] */ __RPC__out DATE *pDate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserDSVersionNumber( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerDSVersionNumber( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserSysvolVersionNumber( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerSysvolVersionNumber( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWMIFilter( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppIGPMWMIFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWMIFilter( 
            /* [in] */ __RPC__in_opt IGPMWMIFilter *pIGPMWMIFilter) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetUserEnabled( 
            /* [in] */ VARIANT_BOOL vbEnabled) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetComputerEnabled( 
            /* [in] */ VARIANT_BOOL vbEnabled) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsUserEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsComputerEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityInfo( 
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Backup( 
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ __RPC__in BSTR bstrComment,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Import( 
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReport( 
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReportToFile( 
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTo( 
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMDomain *pIGPMDomain,
            /* [optional][in] */ __RPC__in VARIANT *pvarNewDisplayName,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityDescriptor( 
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IDispatch *pSD) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityDescriptor( 
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppSD) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsACLConsistent( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbConsistent) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MakeACLConsistent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMGPOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMGPO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMGPO * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMGPO * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMGPO * This,
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
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, put_DisplayName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_DomainName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainName )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ModificationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModificationTime )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_UserDSVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDSVersionNumber )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ComputerDSVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerDSVersionNumber )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_UserSysvolVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSysvolVersionNumber )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ComputerSysvolVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerSysvolVersionNumber )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIFilter )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppIGPMWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWMIFilter )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ __RPC__in_opt IGPMWMIFilter *pIGPMWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetUserEnabled )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ VARIANT_BOOL vbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetComputerEnabled )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ VARIANT_BOOL vbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsUserEnabled )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsComputerEnabled )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityInfo )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityInfo )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMGPO * This);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Backup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ __RPC__in BSTR bstrComment,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Import)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, CopyTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMDomain *pIGPMDomain,
            /* [optional][in] */ __RPC__in VARIANT *pvarNewDisplayName,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityDescriptor )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IDispatch *pSD);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityDescriptor )( 
            __RPC__in IGPMGPO * This,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppSD);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsACLConsistent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsACLConsistent )( 
            __RPC__in IGPMGPO * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbConsistent);
        
        DECLSPEC_XFGVIRT(IGPMGPO, MakeACLConsistent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MakeACLConsistent )( 
            __RPC__in IGPMGPO * This);
        
        END_INTERFACE
    } IGPMGPOVtbl;

    interface IGPMGPO
    {
        CONST_VTBL struct IGPMGPOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMGPO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMGPO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMGPO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMGPO_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMGPO_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMGPO_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMGPO_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMGPO_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IGPMGPO_put_DisplayName(This,newVal)	\
    ( (This)->lpVtbl -> put_DisplayName(This,newVal) ) 

#define IGPMGPO_get_Path(This,pVal)	\
    ( (This)->lpVtbl -> get_Path(This,pVal) ) 

#define IGPMGPO_get_ID(This,pVal)	\
    ( (This)->lpVtbl -> get_ID(This,pVal) ) 

#define IGPMGPO_get_DomainName(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainName(This,pVal) ) 

#define IGPMGPO_get_CreationTime(This,pDate)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pDate) ) 

#define IGPMGPO_get_ModificationTime(This,pDate)	\
    ( (This)->lpVtbl -> get_ModificationTime(This,pDate) ) 

#define IGPMGPO_get_UserDSVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_UserDSVersionNumber(This,pVal) ) 

#define IGPMGPO_get_ComputerDSVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerDSVersionNumber(This,pVal) ) 

#define IGPMGPO_get_UserSysvolVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_UserSysvolVersionNumber(This,pVal) ) 

#define IGPMGPO_get_ComputerSysvolVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerSysvolVersionNumber(This,pVal) ) 

#define IGPMGPO_GetWMIFilter(This,ppIGPMWMIFilter)	\
    ( (This)->lpVtbl -> GetWMIFilter(This,ppIGPMWMIFilter) ) 

#define IGPMGPO_SetWMIFilter(This,pIGPMWMIFilter)	\
    ( (This)->lpVtbl -> SetWMIFilter(This,pIGPMWMIFilter) ) 

#define IGPMGPO_SetUserEnabled(This,vbEnabled)	\
    ( (This)->lpVtbl -> SetUserEnabled(This,vbEnabled) ) 

#define IGPMGPO_SetComputerEnabled(This,vbEnabled)	\
    ( (This)->lpVtbl -> SetComputerEnabled(This,vbEnabled) ) 

#define IGPMGPO_IsUserEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsUserEnabled(This,pvbEnabled) ) 

#define IGPMGPO_IsComputerEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsComputerEnabled(This,pvbEnabled) ) 

#define IGPMGPO_GetSecurityInfo(This,ppSecurityInfo)	\
    ( (This)->lpVtbl -> GetSecurityInfo(This,ppSecurityInfo) ) 

#define IGPMGPO_SetSecurityInfo(This,pSecurityInfo)	\
    ( (This)->lpVtbl -> SetSecurityInfo(This,pSecurityInfo) ) 

#define IGPMGPO_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IGPMGPO_Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO_Import(This,lFlags,pIGPMBackup,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Import(This,lFlags,pIGPMBackup,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#define IGPMGPO_CopyTo(This,lFlags,pIGPMDomain,pvarNewDisplayName,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> CopyTo(This,lFlags,pIGPMDomain,pvarNewDisplayName,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO_SetSecurityDescriptor(This,lFlags,pSD)	\
    ( (This)->lpVtbl -> SetSecurityDescriptor(This,lFlags,pSD) ) 

#define IGPMGPO_GetSecurityDescriptor(This,lFlags,ppSD)	\
    ( (This)->lpVtbl -> GetSecurityDescriptor(This,lFlags,ppSD) ) 

#define IGPMGPO_IsACLConsistent(This,pvbConsistent)	\
    ( (This)->lpVtbl -> IsACLConsistent(This,pvbConsistent) ) 

#define IGPMGPO_MakeACLConsistent(This)	\
    ( (This)->lpVtbl -> MakeACLConsistent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMGPO_INTERFACE_DEFINED__ */


#ifndef __IGPMGPOCollection_INTERFACE_DEFINED__
#define __IGPMGPOCollection_INTERFACE_DEFINED__

/* interface IGPMGPOCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMGPOCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0F0D5CF-70CA-4C39-9E29-B642F8726C01")
    IGPMGPOCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMGPOs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMGPOCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMGPOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMGPOCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMGPOCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMGPOCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMGPOCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMGPOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMGPOCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMGPOCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMGPOCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMGPOCollection * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMGPOCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMGPOs);
        
        END_INTERFACE
    } IGPMGPOCollectionVtbl;

    interface IGPMGPOCollection
    {
        CONST_VTBL struct IGPMGPOCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMGPOCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMGPOCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMGPOCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMGPOCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMGPOCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMGPOCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMGPOCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMGPOCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMGPOCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMGPOCollection_get__NewEnum(This,ppIGPMGPOs)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMGPOs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMGPOCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMGPOLink_INTERFACE_DEFINED__
#define __IGPMGPOLink_INTERFACE_DEFINED__

/* interface IGPMGPOLink */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMGPOLink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("434B99BD-5DE7-478A-809C-C251721DF70C")
    IGPMGPOLink : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GPOID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GPODomain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enforced( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enforced( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SOMLinkOrder( 
            /* [retval][out] */ __RPC__out long *lVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SOM( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppIGPMSOM) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMGPOLinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMGPOLink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMGPOLink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMGPOLink * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMGPOLink * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMGPOLink * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMGPOLink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMGPOLink * This,
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
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, get_GPOID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GPOID )( 
            __RPC__in IGPMGPOLink * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, get_GPODomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GPODomain )( 
            __RPC__in IGPMGPOLink * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IGPMGPOLink * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, put_Enabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IGPMGPOLink * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, get_Enforced)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enforced )( 
            __RPC__in IGPMGPOLink * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, put_Enforced)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enforced )( 
            __RPC__in IGPMGPOLink * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, get_SOMLinkOrder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMLinkOrder )( 
            __RPC__in IGPMGPOLink * This,
            /* [retval][out] */ __RPC__out long *lVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, get_SOM)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOM )( 
            __RPC__in IGPMGPOLink * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppIGPMSOM);
        
        DECLSPEC_XFGVIRT(IGPMGPOLink, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMGPOLink * This);
        
        END_INTERFACE
    } IGPMGPOLinkVtbl;

    interface IGPMGPOLink
    {
        CONST_VTBL struct IGPMGPOLinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMGPOLink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMGPOLink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMGPOLink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMGPOLink_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMGPOLink_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMGPOLink_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMGPOLink_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMGPOLink_get_GPOID(This,pVal)	\
    ( (This)->lpVtbl -> get_GPOID(This,pVal) ) 

#define IGPMGPOLink_get_GPODomain(This,pVal)	\
    ( (This)->lpVtbl -> get_GPODomain(This,pVal) ) 

#define IGPMGPOLink_get_Enabled(This,pVal)	\
    ( (This)->lpVtbl -> get_Enabled(This,pVal) ) 

#define IGPMGPOLink_put_Enabled(This,newVal)	\
    ( (This)->lpVtbl -> put_Enabled(This,newVal) ) 

#define IGPMGPOLink_get_Enforced(This,pVal)	\
    ( (This)->lpVtbl -> get_Enforced(This,pVal) ) 

#define IGPMGPOLink_put_Enforced(This,newVal)	\
    ( (This)->lpVtbl -> put_Enforced(This,newVal) ) 

#define IGPMGPOLink_get_SOMLinkOrder(This,lVal)	\
    ( (This)->lpVtbl -> get_SOMLinkOrder(This,lVal) ) 

#define IGPMGPOLink_get_SOM(This,ppIGPMSOM)	\
    ( (This)->lpVtbl -> get_SOM(This,ppIGPMSOM) ) 

#define IGPMGPOLink_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMGPOLink_INTERFACE_DEFINED__ */


#ifndef __IGPMGPOLinksCollection_INTERFACE_DEFINED__
#define __IGPMGPOLinksCollection_INTERFACE_DEFINED__

/* interface IGPMGPOLinksCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMGPOLinksCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("189D7B68-16BD-4D0D-A2EC-2E6AA2288C7F")
    IGPMGPOLinksCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMLinks) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMGPOLinksCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMGPOLinksCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMGPOLinksCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMGPOLinksCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMGPOLinksCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLinksCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPOLinksCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMGPOLinksCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMLinks);
        
        END_INTERFACE
    } IGPMGPOLinksCollectionVtbl;

    interface IGPMGPOLinksCollection
    {
        CONST_VTBL struct IGPMGPOLinksCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMGPOLinksCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMGPOLinksCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMGPOLinksCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMGPOLinksCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMGPOLinksCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMGPOLinksCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMGPOLinksCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMGPOLinksCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMGPOLinksCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMGPOLinksCollection_get__NewEnum(This,ppIGPMLinks)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMLinks) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMGPOLinksCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMCSECollection_INTERFACE_DEFINED__
#define __IGPMCSECollection_INTERFACE_DEFINED__

/* interface IGPMCSECollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMCSECollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2E52A97D-0A4A-4A6F-85DB-201622455DA0")
    IGPMCSECollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMCSEs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMCSECollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMCSECollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMCSECollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMCSECollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMCSECollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMCSECollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMCSECollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMCSECollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMCSECollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMCSECollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMCSECollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMCSECollection * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMCSECollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMCSECollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMCSEs);
        
        END_INTERFACE
    } IGPMCSECollectionVtbl;

    interface IGPMCSECollection
    {
        CONST_VTBL struct IGPMCSECollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMCSECollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMCSECollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMCSECollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMCSECollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMCSECollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMCSECollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMCSECollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMCSECollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMCSECollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMCSECollection_get__NewEnum(This,ppIGPMCSEs)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMCSEs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMCSECollection_INTERFACE_DEFINED__ */


#ifndef __IGPMClientSideExtension_INTERFACE_DEFINED__
#define __IGPMClientSideExtension_INTERFACE_DEFINED__

/* interface IGPMClientSideExtension */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMClientSideExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("69DA7488-B8DB-415E-9266-901BE4D49928")
    IGPMClientSideExtension : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsUserEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsComputerEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMClientSideExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMClientSideExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMClientSideExtension * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMClientSideExtension * This,
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
        
        DECLSPEC_XFGVIRT(IGPMClientSideExtension, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMClientSideExtension, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMClientSideExtension, IsUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsUserEnabled )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMClientSideExtension, IsComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsComputerEnabled )( 
            __RPC__in IGPMClientSideExtension * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        END_INTERFACE
    } IGPMClientSideExtensionVtbl;

    interface IGPMClientSideExtension
    {
        CONST_VTBL struct IGPMClientSideExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMClientSideExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMClientSideExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMClientSideExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMClientSideExtension_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMClientSideExtension_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMClientSideExtension_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMClientSideExtension_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMClientSideExtension_get_ID(This,pVal)	\
    ( (This)->lpVtbl -> get_ID(This,pVal) ) 

#define IGPMClientSideExtension_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IGPMClientSideExtension_IsUserEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsUserEnabled(This,pvbEnabled) ) 

#define IGPMClientSideExtension_IsComputerEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsComputerEnabled(This,pvbEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMClientSideExtension_INTERFACE_DEFINED__ */


#ifndef __IGPMAsyncCancel_INTERFACE_DEFINED__
#define __IGPMAsyncCancel_INTERFACE_DEFINED__

/* interface IGPMAsyncCancel */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMAsyncCancel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDC67754-BE67-4541-8166-F48166868C9C")
    IGPMAsyncCancel : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMAsyncCancelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMAsyncCancel * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMAsyncCancel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMAsyncCancel * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMAsyncCancel * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMAsyncCancel * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMAsyncCancel * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMAsyncCancel * This,
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
        
        DECLSPEC_XFGVIRT(IGPMAsyncCancel, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IGPMAsyncCancel * This);
        
        END_INTERFACE
    } IGPMAsyncCancelVtbl;

    interface IGPMAsyncCancel
    {
        CONST_VTBL struct IGPMAsyncCancelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMAsyncCancel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMAsyncCancel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMAsyncCancel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMAsyncCancel_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMAsyncCancel_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMAsyncCancel_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMAsyncCancel_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMAsyncCancel_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMAsyncCancel_INTERFACE_DEFINED__ */


#ifndef __IGPMAsyncProgress_INTERFACE_DEFINED__
#define __IGPMAsyncProgress_INTERFACE_DEFINED__

/* interface IGPMAsyncProgress */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMAsyncProgress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6AAC29F8-5948-4324-BF70-423818942DBC")
    IGPMAsyncProgress : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Status( 
            /* [in] */ long lProgressNumerator,
            /* [in] */ long lProgressDenominator,
            /* [in] */ HRESULT hrStatus,
            /* [in] */ __RPC__in VARIANT *pResult,
            /* [in] */ __RPC__in_opt IGPMStatusMsgCollection *ppIGPMStatusMsgCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMAsyncProgressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMAsyncProgress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMAsyncProgress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMAsyncProgress * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMAsyncProgress * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMAsyncProgress * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMAsyncProgress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMAsyncProgress * This,
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
        
        DECLSPEC_XFGVIRT(IGPMAsyncProgress, Status)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Status )( 
            __RPC__in IGPMAsyncProgress * This,
            /* [in] */ long lProgressNumerator,
            /* [in] */ long lProgressDenominator,
            /* [in] */ HRESULT hrStatus,
            /* [in] */ __RPC__in VARIANT *pResult,
            /* [in] */ __RPC__in_opt IGPMStatusMsgCollection *ppIGPMStatusMsgCollection);
        
        END_INTERFACE
    } IGPMAsyncProgressVtbl;

    interface IGPMAsyncProgress
    {
        CONST_VTBL struct IGPMAsyncProgressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMAsyncProgress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMAsyncProgress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMAsyncProgress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMAsyncProgress_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMAsyncProgress_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMAsyncProgress_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMAsyncProgress_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMAsyncProgress_Status(This,lProgressNumerator,lProgressDenominator,hrStatus,pResult,ppIGPMStatusMsgCollection)	\
    ( (This)->lpVtbl -> Status(This,lProgressNumerator,lProgressDenominator,hrStatus,pResult,ppIGPMStatusMsgCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMAsyncProgress_INTERFACE_DEFINED__ */


#ifndef __IGPMStatusMsgCollection_INTERFACE_DEFINED__
#define __IGPMStatusMsgCollection_INTERFACE_DEFINED__

/* interface IGPMStatusMsgCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMStatusMsgCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B6E1AF0-1A92-40F3-A59D-F36AC1F728B7")
    IGPMStatusMsgCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMStatusMsgCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMStatusMsgCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMStatusMsgCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMStatusMsgCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMStatusMsgCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStatusMsgCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStatusMsgCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMStatusMsgCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **pVal);
        
        END_INTERFACE
    } IGPMStatusMsgCollectionVtbl;

    interface IGPMStatusMsgCollection
    {
        CONST_VTBL struct IGPMStatusMsgCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMStatusMsgCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMStatusMsgCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMStatusMsgCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMStatusMsgCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMStatusMsgCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMStatusMsgCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMStatusMsgCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMStatusMsgCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMStatusMsgCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMStatusMsgCollection_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMStatusMsgCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMStatusMessage_INTERFACE_DEFINED__
#define __IGPMStatusMessage_INTERFACE_DEFINED__

/* interface IGPMStatusMessage */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMStatusMessage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8496C22F-F3DE-4A1F-8F58-603CAAA93D7B")
    IGPMStatusMessage : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ObjectPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ErrorCode( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtensionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SettingsName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OperationCode( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMStatusMessageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMStatusMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMStatusMessage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMStatusMessage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMStatusMessage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMStatusMessage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMStatusMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMStatusMessage * This,
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
        
        DECLSPEC_XFGVIRT(IGPMStatusMessage, get_ObjectPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ObjectPath )( 
            __RPC__in IGPMStatusMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStatusMessage, ErrorCode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ErrorCode )( 
            __RPC__in IGPMStatusMessage * This);
        
        DECLSPEC_XFGVIRT(IGPMStatusMessage, get_ExtensionName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtensionName )( 
            __RPC__in IGPMStatusMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStatusMessage, get_SettingsName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SettingsName )( 
            __RPC__in IGPMStatusMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStatusMessage, OperationCode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OperationCode )( 
            __RPC__in IGPMStatusMessage * This);
        
        DECLSPEC_XFGVIRT(IGPMStatusMessage, get_Message)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in IGPMStatusMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        END_INTERFACE
    } IGPMStatusMessageVtbl;

    interface IGPMStatusMessage
    {
        CONST_VTBL struct IGPMStatusMessageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMStatusMessage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMStatusMessage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMStatusMessage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMStatusMessage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMStatusMessage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMStatusMessage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMStatusMessage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMStatusMessage_get_ObjectPath(This,pVal)	\
    ( (This)->lpVtbl -> get_ObjectPath(This,pVal) ) 

#define IGPMStatusMessage_ErrorCode(This)	\
    ( (This)->lpVtbl -> ErrorCode(This) ) 

#define IGPMStatusMessage_get_ExtensionName(This,pVal)	\
    ( (This)->lpVtbl -> get_ExtensionName(This,pVal) ) 

#define IGPMStatusMessage_get_SettingsName(This,pVal)	\
    ( (This)->lpVtbl -> get_SettingsName(This,pVal) ) 

#define IGPMStatusMessage_OperationCode(This)	\
    ( (This)->lpVtbl -> OperationCode(This) ) 

#define IGPMStatusMessage_get_Message(This,pVal)	\
    ( (This)->lpVtbl -> get_Message(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMStatusMessage_INTERFACE_DEFINED__ */


#ifndef __IGPMConstants_INTERFACE_DEFINED__
#define __IGPMConstants_INTERFACE_DEFINED__

/* interface IGPMConstants */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMConstants;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50EF73E6-D35C-4C8D-BE63-7EA5D2AAC5C4")
    IGPMConstants : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermGPOApply( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermGPORead( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermGPOEdit( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermGPOEditSecurityAndDelete( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermGPOCustom( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermWMIFilterEdit( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermWMIFilterFullControl( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermWMIFilterCustom( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermSOMLink( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermSOMLogging( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermSOMPlanning( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermSOMGPOCreate( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermSOMWMICreate( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermSOMWMIFullControl( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPOPermissions( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPOEffectivePermissions( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPODisplayName( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPOWMIFilter( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPOID( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPOComputerExtensions( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPOUserExtensions( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertySOMLinks( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyGPODomain( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyBackupMostRecent( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchOpEquals( 
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchOpContains( 
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchOpNotContains( 
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchOpNotEquals( 
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UsePDC( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseAnyDC( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DoNotUseW2KDC( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SOMSite( 
            /* [retval][out] */ __RPC__out GPMSOMType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SOMDomain( 
            /* [retval][out] */ __RPC__out GPMSOMType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SOMOU( 
            /* [retval][out] */ __RPC__out GPMSOMType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SecurityFlags( 
            /* [in] */ VARIANT_BOOL vbOwner,
            /* [in] */ VARIANT_BOOL vbGroup,
            /* [in] */ VARIANT_BOOL vbDACL,
            /* [in] */ VARIANT_BOOL vbSACL,
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DoNotValidateDC( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportHTML( 
            /* [retval][out] */ __RPC__out GPMReportType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportXML( 
            /* [retval][out] */ __RPC__out GPMReportType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RSOPModeUnknown( 
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RSOPModePlanning( 
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RSOPModeLogging( 
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeUser( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeComputer( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeLocalGroup( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeGlobalGroup( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeUniversalGroup( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeUNCPath( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryTypeUnknown( 
            /* [retval][out] */ __RPC__out GPMEntryType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestinationOptionSameAsSource( 
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestinationOptionNone( 
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestinationOptionByRelativeName( 
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestinationOptionSet( 
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MigrationTableOnly( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProcessSecurity( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RsopLoggingNoComputer( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RsopLoggingNoUser( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RsopPlanningAssumeSlowLink( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RsopPlanningLoopbackOption( 
            /* [in] */ VARIANT_BOOL vbMerge,
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RsopPlanningAssumeUserWQLFilterTrue( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RsopPlanningAssumeCompWQLFilterTrue( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMConstantsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMConstants * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMConstants * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMConstants * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMConstants * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMConstants * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMConstants * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMConstants * This,
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
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOApply)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOApply )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPORead)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPORead )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOEdit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOEdit )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOEditSecurityAndDelete)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOEditSecurityAndDelete )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOCustom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOCustom )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermWMIFilterEdit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermWMIFilterEdit )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermWMIFilterFullControl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermWMIFilterFullControl )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermWMIFilterCustom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermWMIFilterCustom )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMLink)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMLink )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMLogging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMLogging )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMPlanning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMPlanning )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMGPOCreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMGPOCreate )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMWMICreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMWMICreate )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMWMIFullControl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMWMIFullControl )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOPermissions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOPermissions )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOEffectivePermissions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOEffectivePermissions )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPODisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPODisplayName )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOWMIFilter)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOWMIFilter )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOID )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOComputerExtensions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOComputerExtensions )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOUserExtensions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOUserExtensions )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertySOMLinks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertySOMLinks )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPODomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPODomain )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyBackupMostRecent)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyBackupMostRecent )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpEquals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpEquals )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpContains)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpContains )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpNotContains)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpNotContains )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpNotEquals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpNotEquals )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_UsePDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsePDC )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_UseAnyDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseAnyDC )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DoNotUseW2KDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotUseW2KDC )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SOMSite)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMSite )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SOMDomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMDomain )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SOMOU)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMOU )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SecurityFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SecurityFlags )( 
            __RPC__in IGPMConstants * This,
            /* [in] */ VARIANT_BOOL vbOwner,
            /* [in] */ VARIANT_BOOL vbGroup,
            /* [in] */ VARIANT_BOOL vbDACL,
            /* [in] */ VARIANT_BOOL vbSACL,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DoNotValidateDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotValidateDC )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_ReportHTML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportHTML )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMReportType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_ReportXML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportXML )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMReportType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RSOPModeUnknown)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RSOPModeUnknown )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RSOPModePlanning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RSOPModePlanning )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RSOPModeLogging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RSOPModeLogging )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUser )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeComputer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeComputer )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeLocalGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeLocalGroup )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeGlobalGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeGlobalGroup )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUniversalGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUniversalGroup )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUNCPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUNCPath )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUnknown)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUnknown )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionSameAsSource)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionSameAsSource )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionNone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionNone )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionByRelativeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionByRelativeName )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionSet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionSet )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_MigrationTableOnly)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MigrationTableOnly )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_ProcessSecurity)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessSecurity )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopLoggingNoComputer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopLoggingNoComputer )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopLoggingNoUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopLoggingNoUser )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningAssumeSlowLink)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningAssumeSlowLink )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningLoopbackOption)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningLoopbackOption )( 
            __RPC__in IGPMConstants * This,
            /* [in] */ VARIANT_BOOL vbMerge,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningAssumeUserWQLFilterTrue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningAssumeUserWQLFilterTrue )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningAssumeCompWQLFilterTrue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningAssumeCompWQLFilterTrue )( 
            __RPC__in IGPMConstants * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        END_INTERFACE
    } IGPMConstantsVtbl;

    interface IGPMConstants
    {
        CONST_VTBL struct IGPMConstantsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMConstants_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMConstants_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMConstants_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMConstants_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMConstants_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMConstants_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMConstants_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMConstants_get_PermGPOApply(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOApply(This,pVal) ) 

#define IGPMConstants_get_PermGPORead(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPORead(This,pVal) ) 

#define IGPMConstants_get_PermGPOEdit(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOEdit(This,pVal) ) 

#define IGPMConstants_get_PermGPOEditSecurityAndDelete(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOEditSecurityAndDelete(This,pVal) ) 

#define IGPMConstants_get_PermGPOCustom(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOCustom(This,pVal) ) 

#define IGPMConstants_get_PermWMIFilterEdit(This,pVal)	\
    ( (This)->lpVtbl -> get_PermWMIFilterEdit(This,pVal) ) 

#define IGPMConstants_get_PermWMIFilterFullControl(This,pVal)	\
    ( (This)->lpVtbl -> get_PermWMIFilterFullControl(This,pVal) ) 

#define IGPMConstants_get_PermWMIFilterCustom(This,pVal)	\
    ( (This)->lpVtbl -> get_PermWMIFilterCustom(This,pVal) ) 

#define IGPMConstants_get_PermSOMLink(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMLink(This,pVal) ) 

#define IGPMConstants_get_PermSOMLogging(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMLogging(This,pVal) ) 

#define IGPMConstants_get_PermSOMPlanning(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMPlanning(This,pVal) ) 

#define IGPMConstants_get_PermSOMGPOCreate(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMGPOCreate(This,pVal) ) 

#define IGPMConstants_get_PermSOMWMICreate(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMWMICreate(This,pVal) ) 

#define IGPMConstants_get_PermSOMWMIFullControl(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMWMIFullControl(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPOPermissions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOPermissions(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPOEffectivePermissions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOEffectivePermissions(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPODisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPODisplayName(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPOWMIFilter(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOWMIFilter(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPOID(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOID(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPOComputerExtensions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOComputerExtensions(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPOUserExtensions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOUserExtensions(This,pVal) ) 

#define IGPMConstants_get_SearchPropertySOMLinks(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertySOMLinks(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyGPODomain(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPODomain(This,pVal) ) 

#define IGPMConstants_get_SearchPropertyBackupMostRecent(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyBackupMostRecent(This,pVal) ) 

#define IGPMConstants_get_SearchOpEquals(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpEquals(This,pVal) ) 

#define IGPMConstants_get_SearchOpContains(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpContains(This,pVal) ) 

#define IGPMConstants_get_SearchOpNotContains(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpNotContains(This,pVal) ) 

#define IGPMConstants_get_SearchOpNotEquals(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpNotEquals(This,pVal) ) 

#define IGPMConstants_get_UsePDC(This,pVal)	\
    ( (This)->lpVtbl -> get_UsePDC(This,pVal) ) 

#define IGPMConstants_get_UseAnyDC(This,pVal)	\
    ( (This)->lpVtbl -> get_UseAnyDC(This,pVal) ) 

#define IGPMConstants_get_DoNotUseW2KDC(This,pVal)	\
    ( (This)->lpVtbl -> get_DoNotUseW2KDC(This,pVal) ) 

#define IGPMConstants_get_SOMSite(This,pVal)	\
    ( (This)->lpVtbl -> get_SOMSite(This,pVal) ) 

#define IGPMConstants_get_SOMDomain(This,pVal)	\
    ( (This)->lpVtbl -> get_SOMDomain(This,pVal) ) 

#define IGPMConstants_get_SOMOU(This,pVal)	\
    ( (This)->lpVtbl -> get_SOMOU(This,pVal) ) 

#define IGPMConstants_get_SecurityFlags(This,vbOwner,vbGroup,vbDACL,vbSACL,pVal)	\
    ( (This)->lpVtbl -> get_SecurityFlags(This,vbOwner,vbGroup,vbDACL,vbSACL,pVal) ) 

#define IGPMConstants_get_DoNotValidateDC(This,pVal)	\
    ( (This)->lpVtbl -> get_DoNotValidateDC(This,pVal) ) 

#define IGPMConstants_get_ReportHTML(This,pVal)	\
    ( (This)->lpVtbl -> get_ReportHTML(This,pVal) ) 

#define IGPMConstants_get_ReportXML(This,pVal)	\
    ( (This)->lpVtbl -> get_ReportXML(This,pVal) ) 

#define IGPMConstants_get_RSOPModeUnknown(This,pVal)	\
    ( (This)->lpVtbl -> get_RSOPModeUnknown(This,pVal) ) 

#define IGPMConstants_get_RSOPModePlanning(This,pVal)	\
    ( (This)->lpVtbl -> get_RSOPModePlanning(This,pVal) ) 

#define IGPMConstants_get_RSOPModeLogging(This,pVal)	\
    ( (This)->lpVtbl -> get_RSOPModeLogging(This,pVal) ) 

#define IGPMConstants_get_EntryTypeUser(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUser(This,pVal) ) 

#define IGPMConstants_get_EntryTypeComputer(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeComputer(This,pVal) ) 

#define IGPMConstants_get_EntryTypeLocalGroup(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeLocalGroup(This,pVal) ) 

#define IGPMConstants_get_EntryTypeGlobalGroup(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeGlobalGroup(This,pVal) ) 

#define IGPMConstants_get_EntryTypeUniversalGroup(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUniversalGroup(This,pVal) ) 

#define IGPMConstants_get_EntryTypeUNCPath(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUNCPath(This,pVal) ) 

#define IGPMConstants_get_EntryTypeUnknown(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUnknown(This,pVal) ) 

#define IGPMConstants_get_DestinationOptionSameAsSource(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionSameAsSource(This,pVal) ) 

#define IGPMConstants_get_DestinationOptionNone(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionNone(This,pVal) ) 

#define IGPMConstants_get_DestinationOptionByRelativeName(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionByRelativeName(This,pVal) ) 

#define IGPMConstants_get_DestinationOptionSet(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionSet(This,pVal) ) 

#define IGPMConstants_get_MigrationTableOnly(This,pVal)	\
    ( (This)->lpVtbl -> get_MigrationTableOnly(This,pVal) ) 

#define IGPMConstants_get_ProcessSecurity(This,pVal)	\
    ( (This)->lpVtbl -> get_ProcessSecurity(This,pVal) ) 

#define IGPMConstants_get_RsopLoggingNoComputer(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopLoggingNoComputer(This,pVal) ) 

#define IGPMConstants_get_RsopLoggingNoUser(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopLoggingNoUser(This,pVal) ) 

#define IGPMConstants_get_RsopPlanningAssumeSlowLink(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningAssumeSlowLink(This,pVal) ) 

#define IGPMConstants_get_RsopPlanningLoopbackOption(This,vbMerge,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningLoopbackOption(This,vbMerge,pVal) ) 

#define IGPMConstants_get_RsopPlanningAssumeUserWQLFilterTrue(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningAssumeUserWQLFilterTrue(This,pVal) ) 

#define IGPMConstants_get_RsopPlanningAssumeCompWQLFilterTrue(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningAssumeCompWQLFilterTrue(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMConstants_INTERFACE_DEFINED__ */


#ifndef __IGPMResult_INTERFACE_DEFINED__
#define __IGPMResult_INTERFACE_DEFINED__

/* interface IGPMResult */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86DFF7E9-F76F-42AB-9570-CEBC6BE8A52D")
    IGPMResult : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMStatusMsgCollection **ppIGPMStatusMsgCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Result( 
            /* [retval][out] */ __RPC__out VARIANT *pvarResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OverallStatus( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMResult * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMResult * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMResult * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMResult * This,
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
        
        DECLSPEC_XFGVIRT(IGPMResult, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IGPMResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStatusMsgCollection **ppIGPMStatusMsgCollection);
        
        DECLSPEC_XFGVIRT(IGPMResult, get_Result)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Result )( 
            __RPC__in IGPMResult * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarResult);
        
        DECLSPEC_XFGVIRT(IGPMResult, OverallStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OverallStatus )( 
            __RPC__in IGPMResult * This);
        
        END_INTERFACE
    } IGPMResultVtbl;

    interface IGPMResult
    {
        CONST_VTBL struct IGPMResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMResult_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMResult_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMResult_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMResult_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMResult_get_Status(This,ppIGPMStatusMsgCollection)	\
    ( (This)->lpVtbl -> get_Status(This,ppIGPMStatusMsgCollection) ) 

#define IGPMResult_get_Result(This,pvarResult)	\
    ( (This)->lpVtbl -> get_Result(This,pvarResult) ) 

#define IGPMResult_OverallStatus(This)	\
    ( (This)->lpVtbl -> OverallStatus(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMResult_INTERFACE_DEFINED__ */


#ifndef __IGPMMapEntryCollection_INTERFACE_DEFINED__
#define __IGPMMapEntryCollection_INTERFACE_DEFINED__

/* interface IGPMMapEntryCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMMapEntryCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB0BF49B-E53F-443F-B807-8BE22BFB6D42")
    IGPMMapEntryCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMMapEntryCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMMapEntryCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMMapEntryCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMMapEntryCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMMapEntryCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMMapEntryCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMMapEntryCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMMapEntryCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **pVal);
        
        END_INTERFACE
    } IGPMMapEntryCollectionVtbl;

    interface IGPMMapEntryCollection
    {
        CONST_VTBL struct IGPMMapEntryCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMMapEntryCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMMapEntryCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMMapEntryCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMMapEntryCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMMapEntryCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMMapEntryCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMMapEntryCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMMapEntryCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMMapEntryCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMMapEntryCollection_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMMapEntryCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMMapEntry_INTERFACE_DEFINED__
#define __IGPMMapEntry_INTERFACE_DEFINED__

/* interface IGPMMapEntry */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMMapEntry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8E79AD06-2381-4444-BE4C-FF693E6E6F2B")
    IGPMMapEntry : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Source( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSource) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Destination( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDestination) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestinationOption( 
            /* [retval][out] */ __RPC__out GPMDestinationOption *pgpmDestOption) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EntryType( 
            /* [retval][out] */ __RPC__out GPMEntryType *pgpmEntryType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMMapEntryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMMapEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMMapEntry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMMapEntry * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMMapEntry * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMMapEntry * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMMapEntry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMMapEntry * This,
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
        
        DECLSPEC_XFGVIRT(IGPMMapEntry, get_Source)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Source )( 
            __RPC__in IGPMMapEntry * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSource);
        
        DECLSPEC_XFGVIRT(IGPMMapEntry, get_Destination)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Destination )( 
            __RPC__in IGPMMapEntry * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDestination);
        
        DECLSPEC_XFGVIRT(IGPMMapEntry, get_DestinationOption)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOption )( 
            __RPC__in IGPMMapEntry * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pgpmDestOption);
        
        DECLSPEC_XFGVIRT(IGPMMapEntry, get_EntryType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryType )( 
            __RPC__in IGPMMapEntry * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pgpmEntryType);
        
        END_INTERFACE
    } IGPMMapEntryVtbl;

    interface IGPMMapEntry
    {
        CONST_VTBL struct IGPMMapEntryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMMapEntry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMMapEntry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMMapEntry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMMapEntry_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMMapEntry_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMMapEntry_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMMapEntry_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMMapEntry_get_Source(This,pbstrSource)	\
    ( (This)->lpVtbl -> get_Source(This,pbstrSource) ) 

#define IGPMMapEntry_get_Destination(This,pbstrDestination)	\
    ( (This)->lpVtbl -> get_Destination(This,pbstrDestination) ) 

#define IGPMMapEntry_get_DestinationOption(This,pgpmDestOption)	\
    ( (This)->lpVtbl -> get_DestinationOption(This,pgpmDestOption) ) 

#define IGPMMapEntry_get_EntryType(This,pgpmEntryType)	\
    ( (This)->lpVtbl -> get_EntryType(This,pgpmEntryType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMMapEntry_INTERFACE_DEFINED__ */


#ifndef __IGPMMigrationTable_INTERFACE_DEFINED__
#define __IGPMMigrationTable_INTERFACE_DEFINED__

/* interface IGPMMigrationTable */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMMigrationTable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("48F823B1-EFAF-470B-B6ED-40D14EE1A4EC")
    IGPMMigrationTable : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [in] */ __RPC__in BSTR bstrMigrationTablePath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ long lFlags,
            /* [in] */ VARIANT var) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddEntry( 
            /* [in] */ __RPC__in BSTR bstrSource,
            /* [in] */ GPMEntryType gpmEntryType,
            /* [optional][in] */ __RPC__in VARIANT *pvarDestination,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntry **ppEntry) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEntry( 
            /* [in] */ __RPC__in BSTR bstrSource,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntry **ppEntry) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteEntry( 
            /* [in] */ __RPC__in BSTR bstrSource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateDestination( 
            /* [in] */ __RPC__in BSTR bstrSource,
            /* [optional][in] */ __RPC__in VARIANT *pvarDestination,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntry **ppEntry) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Validate( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEntries( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntryCollection **ppEntries) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMMigrationTableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMMigrationTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMMigrationTable * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMMigrationTable * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMMigrationTable * This,
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
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in BSTR bstrMigrationTablePath);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ long lFlags,
            /* [in] */ VARIANT var);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, AddEntry)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddEntry )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in BSTR bstrSource,
            /* [in] */ GPMEntryType gpmEntryType,
            /* [optional][in] */ __RPC__in VARIANT *pvarDestination,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntry **ppEntry);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, GetEntry)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEntry )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in BSTR bstrSource,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntry **ppEntry);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, DeleteEntry)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteEntry )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in BSTR bstrSource);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, UpdateDestination)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateDestination )( 
            __RPC__in IGPMMigrationTable * This,
            /* [in] */ __RPC__in BSTR bstrSource,
            /* [optional][in] */ __RPC__in VARIANT *pvarDestination,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntry **ppEntry);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, Validate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Validate )( 
            __RPC__in IGPMMigrationTable * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppResult);
        
        DECLSPEC_XFGVIRT(IGPMMigrationTable, GetEntries)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEntries )( 
            __RPC__in IGPMMigrationTable * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMapEntryCollection **ppEntries);
        
        END_INTERFACE
    } IGPMMigrationTableVtbl;

    interface IGPMMigrationTable
    {
        CONST_VTBL struct IGPMMigrationTableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMMigrationTable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMMigrationTable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMMigrationTable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMMigrationTable_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMMigrationTable_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMMigrationTable_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMMigrationTable_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMMigrationTable_Save(This,bstrMigrationTablePath)	\
    ( (This)->lpVtbl -> Save(This,bstrMigrationTablePath) ) 

#define IGPMMigrationTable_Add(This,lFlags,var)	\
    ( (This)->lpVtbl -> Add(This,lFlags,var) ) 

#define IGPMMigrationTable_AddEntry(This,bstrSource,gpmEntryType,pvarDestination,ppEntry)	\
    ( (This)->lpVtbl -> AddEntry(This,bstrSource,gpmEntryType,pvarDestination,ppEntry) ) 

#define IGPMMigrationTable_GetEntry(This,bstrSource,ppEntry)	\
    ( (This)->lpVtbl -> GetEntry(This,bstrSource,ppEntry) ) 

#define IGPMMigrationTable_DeleteEntry(This,bstrSource)	\
    ( (This)->lpVtbl -> DeleteEntry(This,bstrSource) ) 

#define IGPMMigrationTable_UpdateDestination(This,bstrSource,pvarDestination,ppEntry)	\
    ( (This)->lpVtbl -> UpdateDestination(This,bstrSource,pvarDestination,ppEntry) ) 

#define IGPMMigrationTable_Validate(This,ppResult)	\
    ( (This)->lpVtbl -> Validate(This,ppResult) ) 

#define IGPMMigrationTable_GetEntries(This,ppEntries)	\
    ( (This)->lpVtbl -> GetEntries(This,ppEntries) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMMigrationTable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_gpmgmt_0000_0030 */
/* [local] */ 

typedef /* [public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0030_0001
    {
        typeGPO	= 0,
        typeStarterGPO	= ( typeGPO + 1 ) 
    } 	GPMBackupType;

typedef /* [public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_gpmgmt_0000_0030_0002
    {
        typeSystem	= 0,
        typeCustom	= ( typeSystem + 1 ) 
    } 	GPMStarterGPOType;



extern RPC_IF_HANDLE __MIDL_itf_gpmgmt_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_gpmgmt_0000_0030_v0_0_s_ifspec;

#ifndef __IGPMBackupDirEx_INTERFACE_DEFINED__
#define __IGPMBackupDirEx_INTERFACE_DEFINED__

/* interface IGPMBackupDirEx */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMBackupDirEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8DC55ED-3BA0-4864-AAD4-D365189EE1D5")
    IGPMBackupDirEx : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupDir( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBackupDir) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupType( 
            /* [retval][out] */ __RPC__out GPMBackupType *pgpmBackupType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBackup( 
            /* [in] */ __RPC__in BSTR bstrID,
            /* [retval][out] */ __RPC__out VARIANT *pvarBackup) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchBackups( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__out VARIANT *pvarBackupCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMBackupDirExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMBackupDirEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMBackupDirEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMBackupDirEx * This,
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
        
        DECLSPEC_XFGVIRT(IGPMBackupDirEx, get_BackupDir)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupDir )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBackupDir);
        
        DECLSPEC_XFGVIRT(IGPMBackupDirEx, get_BackupType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupType )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [retval][out] */ __RPC__out GPMBackupType *pgpmBackupType);
        
        DECLSPEC_XFGVIRT(IGPMBackupDirEx, GetBackup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackup )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [in] */ __RPC__in BSTR bstrID,
            /* [retval][out] */ __RPC__out VARIANT *pvarBackup);
        
        DECLSPEC_XFGVIRT(IGPMBackupDirEx, SearchBackups)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchBackups )( 
            __RPC__in IGPMBackupDirEx * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__out VARIANT *pvarBackupCollection);
        
        END_INTERFACE
    } IGPMBackupDirExVtbl;

    interface IGPMBackupDirEx
    {
        CONST_VTBL struct IGPMBackupDirExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMBackupDirEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMBackupDirEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMBackupDirEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMBackupDirEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMBackupDirEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMBackupDirEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMBackupDirEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMBackupDirEx_get_BackupDir(This,pbstrBackupDir)	\
    ( (This)->lpVtbl -> get_BackupDir(This,pbstrBackupDir) ) 

#define IGPMBackupDirEx_get_BackupType(This,pgpmBackupType)	\
    ( (This)->lpVtbl -> get_BackupType(This,pgpmBackupType) ) 

#define IGPMBackupDirEx_GetBackup(This,bstrID,pvarBackup)	\
    ( (This)->lpVtbl -> GetBackup(This,bstrID,pvarBackup) ) 

#define IGPMBackupDirEx_SearchBackups(This,pIGPMSearchCriteria,pvarBackupCollection)	\
    ( (This)->lpVtbl -> SearchBackups(This,pIGPMSearchCriteria,pvarBackupCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMBackupDirEx_INTERFACE_DEFINED__ */


#ifndef __IGPMStarterGPOBackupCollection_INTERFACE_DEFINED__
#define __IGPMStarterGPOBackupCollection_INTERFACE_DEFINED__

/* interface IGPMStarterGPOBackupCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMStarterGPOBackupCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C998031D-ADD0-4bb5-8DEA-298505D8423B")
    IGPMStarterGPOBackupCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMTmplBackup) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMStarterGPOBackupCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMStarterGPOBackupCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMStarterGPOBackupCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMStarterGPOBackupCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackupCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackupCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackupCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMStarterGPOBackupCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMTmplBackup);
        
        END_INTERFACE
    } IGPMStarterGPOBackupCollectionVtbl;

    interface IGPMStarterGPOBackupCollection
    {
        CONST_VTBL struct IGPMStarterGPOBackupCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMStarterGPOBackupCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMStarterGPOBackupCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMStarterGPOBackupCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMStarterGPOBackupCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMStarterGPOBackupCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMStarterGPOBackupCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMStarterGPOBackupCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMStarterGPOBackupCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMStarterGPOBackupCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMStarterGPOBackupCollection_get__NewEnum(This,ppIGPMTmplBackup)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMTmplBackup) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMStarterGPOBackupCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMStarterGPOBackup_INTERFACE_DEFINED__
#define __IGPMStarterGPOBackup_INTERFACE_DEFINED__

/* interface IGPMStarterGPOBackup */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMStarterGPOBackup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51D98EDA-A87E-43dd-B80A-0B66EF1938D6")
    IGPMStarterGPOBackup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupDir( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBackupDir) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Comment( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrComment) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Domain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTemplateDomain) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StarterGPOID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTemplateID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Timestamp( 
            /* [retval][out] */ __RPC__out DATE *pTimestamp) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReport( 
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReportToFile( 
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMStarterGPOBackupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMStarterGPOBackup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMStarterGPOBackup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMStarterGPOBackup * This,
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
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_BackupDir)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupDir )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBackupDir);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_Comment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Comment )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrComment);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTemplateDomain);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_StarterGPOID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StarterGPOID )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTemplateID);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrID);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_Timestamp)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timestamp )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__out DATE *pTimestamp);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pType);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMStarterGPOBackup * This);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOBackup, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMStarterGPOBackup * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        END_INTERFACE
    } IGPMStarterGPOBackupVtbl;

    interface IGPMStarterGPOBackup
    {
        CONST_VTBL struct IGPMStarterGPOBackupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMStarterGPOBackup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMStarterGPOBackup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMStarterGPOBackup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMStarterGPOBackup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMStarterGPOBackup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMStarterGPOBackup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMStarterGPOBackup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMStarterGPOBackup_get_BackupDir(This,pbstrBackupDir)	\
    ( (This)->lpVtbl -> get_BackupDir(This,pbstrBackupDir) ) 

#define IGPMStarterGPOBackup_get_Comment(This,pbstrComment)	\
    ( (This)->lpVtbl -> get_Comment(This,pbstrComment) ) 

#define IGPMStarterGPOBackup_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 

#define IGPMStarterGPOBackup_get_Domain(This,pbstrTemplateDomain)	\
    ( (This)->lpVtbl -> get_Domain(This,pbstrTemplateDomain) ) 

#define IGPMStarterGPOBackup_get_StarterGPOID(This,pbstrTemplateID)	\
    ( (This)->lpVtbl -> get_StarterGPOID(This,pbstrTemplateID) ) 

#define IGPMStarterGPOBackup_get_ID(This,pbstrID)	\
    ( (This)->lpVtbl -> get_ID(This,pbstrID) ) 

#define IGPMStarterGPOBackup_get_Timestamp(This,pTimestamp)	\
    ( (This)->lpVtbl -> get_Timestamp(This,pTimestamp) ) 

#define IGPMStarterGPOBackup_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define IGPMStarterGPOBackup_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IGPMStarterGPOBackup_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMStarterGPOBackup_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMStarterGPOBackup_INTERFACE_DEFINED__ */


#ifndef __IGPM2_INTERFACE_DEFINED__
#define __IGPM2_INTERFACE_DEFINED__

/* interface IGPM2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPM2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00238F8A-3D86-41ac-8F5E-06A6638A634A")
    IGPM2 : public IGPM
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBackupDirEx( 
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ GPMBackupType backupDirType,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupDirEx **ppIGPMBackupDirEx) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitializeReportingEx( 
            /* [in] */ __RPC__in BSTR bstrAdmPath,
            /* [in] */ LONG reportingOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPM2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPM2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPM2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPM2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPM2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPM2 * This,
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
        
        DECLSPEC_XFGVIRT(IGPM, GetDomain)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDomain )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrDomain,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomainController,
            /* [in] */ long lDCFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMDomain **pIGPMDomain);
        
        DECLSPEC_XFGVIRT(IGPM, GetBackupDir)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackupDir )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupDir **pIGPMBackupDir);
        
        DECLSPEC_XFGVIRT(IGPM, GetSitesContainer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSitesContainer )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrForest,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomain,
            /* [unique][in] */ __RPC__in_opt BSTR bstrDomainController,
            /* [in] */ long lDCFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSitesContainer **ppIGPMSitesContainer);
        
        DECLSPEC_XFGVIRT(IGPM, GetRSOP)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRSOP )( 
            __RPC__in IGPM2 * This,
            /* [in] */ GPMRSOPMode gpmRSoPMode,
            /* [in] */ __RPC__in BSTR bstrNamespace,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IGPMRSOP **ppIGPMRSOP);
        
        DECLSPEC_XFGVIRT(IGPM, CreatePermission)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreatePermission )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrTrustee,
            /* [in] */ GPMPermissionType perm,
            /* [in] */ VARIANT_BOOL bInheritable,
            /* [retval][out] */ __RPC__deref_out_opt IGPMPermission **ppPerm);
        
        DECLSPEC_XFGVIRT(IGPM, CreateSearchCriteria)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSearchCriteria )( 
            __RPC__in IGPM2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSearchCriteria **ppIGPMSearchCriteria);
        
        DECLSPEC_XFGVIRT(IGPM, CreateTrustee)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTrustee )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrTrustee,
            /* [retval][out] */ __RPC__deref_out_opt IGPMTrustee **ppIGPMTrustee);
        
        DECLSPEC_XFGVIRT(IGPM, GetClientSideExtensions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetClientSideExtensions )( 
            __RPC__in IGPM2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMCSECollection **ppIGPMCSECollection);
        
        DECLSPEC_XFGVIRT(IGPM, GetConstants)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConstants )( 
            __RPC__in IGPM2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMConstants **ppIGPMConstants);
        
        DECLSPEC_XFGVIRT(IGPM, GetMigrationTable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMigrationTable )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrMigrationTablePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMigrationTable **ppMigrationTable);
        
        DECLSPEC_XFGVIRT(IGPM, CreateMigrationTable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateMigrationTable )( 
            __RPC__in IGPM2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMMigrationTable **ppMigrationTable);
        
        DECLSPEC_XFGVIRT(IGPM, InitializeReporting)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeReporting )( 
            __RPC__in IGPM2 * This,
            /* [unique][in] */ __RPC__in_opt BSTR bstrAdmPath);
        
        DECLSPEC_XFGVIRT(IGPM2, GetBackupDirEx)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBackupDirEx )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ GPMBackupType backupDirType,
            /* [retval][out] */ __RPC__deref_out_opt IGPMBackupDirEx **ppIGPMBackupDirEx);
        
        DECLSPEC_XFGVIRT(IGPM2, InitializeReportingEx)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeReportingEx )( 
            __RPC__in IGPM2 * This,
            /* [in] */ __RPC__in BSTR bstrAdmPath,
            /* [in] */ LONG reportingOptions);
        
        END_INTERFACE
    } IGPM2Vtbl;

    interface IGPM2
    {
        CONST_VTBL struct IGPM2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPM2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPM2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPM2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPM2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPM2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPM2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPM2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPM2_GetDomain(This,bstrDomain,bstrDomainController,lDCFlags,pIGPMDomain)	\
    ( (This)->lpVtbl -> GetDomain(This,bstrDomain,bstrDomainController,lDCFlags,pIGPMDomain) ) 

#define IGPM2_GetBackupDir(This,bstrBackupDir,pIGPMBackupDir)	\
    ( (This)->lpVtbl -> GetBackupDir(This,bstrBackupDir,pIGPMBackupDir) ) 

#define IGPM2_GetSitesContainer(This,bstrForest,bstrDomain,bstrDomainController,lDCFlags,ppIGPMSitesContainer)	\
    ( (This)->lpVtbl -> GetSitesContainer(This,bstrForest,bstrDomain,bstrDomainController,lDCFlags,ppIGPMSitesContainer) ) 

#define IGPM2_GetRSOP(This,gpmRSoPMode,bstrNamespace,lFlags,ppIGPMRSOP)	\
    ( (This)->lpVtbl -> GetRSOP(This,gpmRSoPMode,bstrNamespace,lFlags,ppIGPMRSOP) ) 

#define IGPM2_CreatePermission(This,bstrTrustee,perm,bInheritable,ppPerm)	\
    ( (This)->lpVtbl -> CreatePermission(This,bstrTrustee,perm,bInheritable,ppPerm) ) 

#define IGPM2_CreateSearchCriteria(This,ppIGPMSearchCriteria)	\
    ( (This)->lpVtbl -> CreateSearchCriteria(This,ppIGPMSearchCriteria) ) 

#define IGPM2_CreateTrustee(This,bstrTrustee,ppIGPMTrustee)	\
    ( (This)->lpVtbl -> CreateTrustee(This,bstrTrustee,ppIGPMTrustee) ) 

#define IGPM2_GetClientSideExtensions(This,ppIGPMCSECollection)	\
    ( (This)->lpVtbl -> GetClientSideExtensions(This,ppIGPMCSECollection) ) 

#define IGPM2_GetConstants(This,ppIGPMConstants)	\
    ( (This)->lpVtbl -> GetConstants(This,ppIGPMConstants) ) 

#define IGPM2_GetMigrationTable(This,bstrMigrationTablePath,ppMigrationTable)	\
    ( (This)->lpVtbl -> GetMigrationTable(This,bstrMigrationTablePath,ppMigrationTable) ) 

#define IGPM2_CreateMigrationTable(This,ppMigrationTable)	\
    ( (This)->lpVtbl -> CreateMigrationTable(This,ppMigrationTable) ) 

#define IGPM2_InitializeReporting(This,bstrAdmPath)	\
    ( (This)->lpVtbl -> InitializeReporting(This,bstrAdmPath) ) 


#define IGPM2_GetBackupDirEx(This,bstrBackupDir,backupDirType,ppIGPMBackupDirEx)	\
    ( (This)->lpVtbl -> GetBackupDirEx(This,bstrBackupDir,backupDirType,ppIGPMBackupDirEx) ) 

#define IGPM2_InitializeReportingEx(This,bstrAdmPath,reportingOptions)	\
    ( (This)->lpVtbl -> InitializeReportingEx(This,bstrAdmPath,reportingOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPM2_INTERFACE_DEFINED__ */


#ifndef __IGPMStarterGPO_INTERFACE_DEFINED__
#define __IGPMStarterGPO_INTERFACE_DEFINED__

/* interface IGPMStarterGPO */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMStarterGPO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DFC3F61B-8880-4490-9337-D29C7BA8C2F0")
    IGPMStarterGPO : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DisplayName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Author( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Product( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CreationTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ModifiedTime( 
            /* [retval][out] */ __RPC__out DATE *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ComputerVersion( 
            /* [retval][out] */ __RPC__out USHORT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserVersion( 
            /* [retval][out] */ __RPC__out USHORT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StarterGPOVersion( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [in] */ __RPC__in BSTR bstrSaveFile,
            /* [in] */ VARIANT_BOOL bOverwrite,
            /* [in] */ VARIANT_BOOL bSaveAsSystem,
            /* [optional][in] */ __RPC__in VARIANT *bstrLanguage,
            /* [optional][in] */ __RPC__in VARIANT *bstrAuthor,
            /* [optional][in] */ __RPC__in VARIANT *bstrProduct,
            /* [optional][in] */ __RPC__in VARIANT *bstrUniqueID,
            /* [optional][in] */ __RPC__in VARIANT *bstrVersion,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Backup( 
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ __RPC__in BSTR bstrComment,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTo( 
            /* [optional][in] */ __RPC__in VARIANT *pvarNewDisplayName,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReport( 
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReportToFile( 
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecurityInfo( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSecurityInfo( 
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMStarterGPOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMStarterGPO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMStarterGPO * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMStarterGPO * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMStarterGPO * This,
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
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, put_DisplayName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_Author)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Author )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_Product)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Product )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_ModifiedTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModifiedTime )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__out DATE *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_ComputerVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerVersion )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__out USHORT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_UserVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserVersion )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__out USHORT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, get_StarterGPOVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StarterGPOVersion )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMStarterGPO * This);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in BSTR bstrSaveFile,
            /* [in] */ VARIANT_BOOL bOverwrite,
            /* [in] */ VARIANT_BOOL bSaveAsSystem,
            /* [optional][in] */ __RPC__in VARIANT *bstrLanguage,
            /* [optional][in] */ __RPC__in VARIANT *bstrAuthor,
            /* [optional][in] */ __RPC__in VARIANT *bstrProduct,
            /* [optional][in] */ __RPC__in VARIANT *bstrUniqueID,
            /* [optional][in] */ __RPC__in VARIANT *bstrVersion,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, Backup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ __RPC__in BSTR bstrComment,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, CopyTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            __RPC__in IGPMStarterGPO * This,
            /* [optional][in] */ __RPC__in VARIANT *pvarNewDisplayName,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, GetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityInfo )( 
            __RPC__in IGPMStarterGPO * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPO, SetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityInfo )( 
            __RPC__in IGPMStarterGPO * This,
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo);
        
        END_INTERFACE
    } IGPMStarterGPOVtbl;

    interface IGPMStarterGPO
    {
        CONST_VTBL struct IGPMStarterGPOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMStarterGPO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMStarterGPO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMStarterGPO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMStarterGPO_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMStarterGPO_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMStarterGPO_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMStarterGPO_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMStarterGPO_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IGPMStarterGPO_put_DisplayName(This,newVal)	\
    ( (This)->lpVtbl -> put_DisplayName(This,newVal) ) 

#define IGPMStarterGPO_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IGPMStarterGPO_put_Description(This,newVal)	\
    ( (This)->lpVtbl -> put_Description(This,newVal) ) 

#define IGPMStarterGPO_get_Author(This,pVal)	\
    ( (This)->lpVtbl -> get_Author(This,pVal) ) 

#define IGPMStarterGPO_get_Product(This,pVal)	\
    ( (This)->lpVtbl -> get_Product(This,pVal) ) 

#define IGPMStarterGPO_get_CreationTime(This,pVal)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pVal) ) 

#define IGPMStarterGPO_get_ID(This,pVal)	\
    ( (This)->lpVtbl -> get_ID(This,pVal) ) 

#define IGPMStarterGPO_get_ModifiedTime(This,pVal)	\
    ( (This)->lpVtbl -> get_ModifiedTime(This,pVal) ) 

#define IGPMStarterGPO_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 

#define IGPMStarterGPO_get_ComputerVersion(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerVersion(This,pVal) ) 

#define IGPMStarterGPO_get_UserVersion(This,pVal)	\
    ( (This)->lpVtbl -> get_UserVersion(This,pVal) ) 

#define IGPMStarterGPO_get_StarterGPOVersion(This,pVal)	\
    ( (This)->lpVtbl -> get_StarterGPOVersion(This,pVal) ) 

#define IGPMStarterGPO_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IGPMStarterGPO_Save(This,bstrSaveFile,bOverwrite,bSaveAsSystem,bstrLanguage,bstrAuthor,bstrProduct,bstrUniqueID,bstrVersion,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Save(This,bstrSaveFile,bOverwrite,bSaveAsSystem,bstrLanguage,bstrAuthor,bstrProduct,bstrUniqueID,bstrVersion,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMStarterGPO_Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMStarterGPO_CopyTo(This,pvarNewDisplayName,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> CopyTo(This,pvarNewDisplayName,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMStarterGPO_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMStarterGPO_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#define IGPMStarterGPO_GetSecurityInfo(This,ppSecurityInfo)	\
    ( (This)->lpVtbl -> GetSecurityInfo(This,ppSecurityInfo) ) 

#define IGPMStarterGPO_SetSecurityInfo(This,pSecurityInfo)	\
    ( (This)->lpVtbl -> SetSecurityInfo(This,pSecurityInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMStarterGPO_INTERFACE_DEFINED__ */


#ifndef __IGPMStarterGPOCollection_INTERFACE_DEFINED__
#define __IGPMStarterGPOCollection_INTERFACE_DEFINED__

/* interface IGPMStarterGPOCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMStarterGPOCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2E522729-2219-44ad-933A-64DFD650C423")
    IGPMStarterGPOCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMTemplates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMStarterGPOCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMStarterGPOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMStarterGPOCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMStarterGPOCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMStarterGPOCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMStarterGPOCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMStarterGPOCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMStarterGPOCollection * This,
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
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IGPMStarterGPOCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IGPMStarterGPOCollection * This,
            long lIndex,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IGPMStarterGPOCollection, get__NewEnum)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IGPMStarterGPOCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppIGPMTemplates);
        
        END_INTERFACE
    } IGPMStarterGPOCollectionVtbl;

    interface IGPMStarterGPOCollection
    {
        CONST_VTBL struct IGPMStarterGPOCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMStarterGPOCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMStarterGPOCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMStarterGPOCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMStarterGPOCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMStarterGPOCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMStarterGPOCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMStarterGPOCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMStarterGPOCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IGPMStarterGPOCollection_get_Item(This,lIndex,pVal)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pVal) ) 

#define IGPMStarterGPOCollection_get__NewEnum(This,ppIGPMTemplates)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppIGPMTemplates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMStarterGPOCollection_INTERFACE_DEFINED__ */


#ifndef __IGPMDomain2_INTERFACE_DEFINED__
#define __IGPMDomain2_INTERFACE_DEFINED__

/* interface IGPMDomain2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMDomain2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7CA6BB8B-F1EB-490a-938D-3C4E51C768E6")
    IGPMDomain2 : public IGPMDomain
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateStarterGPO( 
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPO **ppnewTemplate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateGPOFromStarterGPO( 
            /* [in] */ __RPC__in_opt IGPMStarterGPO *pGPOTemplate,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppnewGPO) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStarterGPO( 
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPO **ppTemplate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SearchStarterGPOs( 
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPOCollection **ppIGPMTemplateCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LoadStarterGPO( 
            /* [in] */ __RPC__in BSTR bstrLoadFile,
            /* [in] */ VARIANT_BOOL bOverwrite,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RestoreStarterGPO( 
            /* [in] */ __RPC__in_opt IGPMStarterGPOBackup *pIGPMTmplBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMDomain2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMDomain2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMDomain2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMDomain2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMDomain2 * This,
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
        
        DECLSPEC_XFGVIRT(IGPMDomain, get_DomainController)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainController )( 
            __RPC__in IGPMDomain2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in IGPMDomain2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain, CreateGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppNewGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchGPOs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchGPOs )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOCollection **ppIGPMGPOCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain, RestoreGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [in] */ long lDCFlags,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetSOM)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSOM )( 
            __RPC__in IGPMDomain2 * This,
            /* [unique][in] */ __RPC__in_opt BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppSOM);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchSOMs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchSOMs )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOMCollection **ppIGPMSOMCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIFilter )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchWMIFilters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchWMIFilters )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilterCollection **ppIGPMWMIFilterCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, CreateStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateStarterGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPO **ppnewTemplate);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, CreateGPOFromStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateGPOFromStarterGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMStarterGPO *pGPOTemplate,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppnewGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, GetStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStarterGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPO **ppTemplate);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, SearchStarterGPOs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchStarterGPOs )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPOCollection **ppIGPMTemplateCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, LoadStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoadStarterGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in BSTR bstrLoadFile,
            /* [in] */ VARIANT_BOOL bOverwrite,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, RestoreStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreStarterGPO )( 
            __RPC__in IGPMDomain2 * This,
            /* [in] */ __RPC__in_opt IGPMStarterGPOBackup *pIGPMTmplBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        END_INTERFACE
    } IGPMDomain2Vtbl;

    interface IGPMDomain2
    {
        CONST_VTBL struct IGPMDomain2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMDomain2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMDomain2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMDomain2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMDomain2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMDomain2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMDomain2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMDomain2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMDomain2_get_DomainController(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainController(This,pVal) ) 

#define IGPMDomain2_get_Domain(This,pVal)	\
    ( (This)->lpVtbl -> get_Domain(This,pVal) ) 

#define IGPMDomain2_CreateGPO(This,ppNewGPO)	\
    ( (This)->lpVtbl -> CreateGPO(This,ppNewGPO) ) 

#define IGPMDomain2_GetGPO(This,bstrGuid,ppGPO)	\
    ( (This)->lpVtbl -> GetGPO(This,bstrGuid,ppGPO) ) 

#define IGPMDomain2_SearchGPOs(This,pIGPMSearchCriteria,ppIGPMGPOCollection)	\
    ( (This)->lpVtbl -> SearchGPOs(This,pIGPMSearchCriteria,ppIGPMGPOCollection) ) 

#define IGPMDomain2_RestoreGPO(This,pIGPMBackup,lDCFlags,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> RestoreGPO(This,pIGPMBackup,lDCFlags,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMDomain2_GetSOM(This,bstrPath,ppSOM)	\
    ( (This)->lpVtbl -> GetSOM(This,bstrPath,ppSOM) ) 

#define IGPMDomain2_SearchSOMs(This,pIGPMSearchCriteria,ppIGPMSOMCollection)	\
    ( (This)->lpVtbl -> SearchSOMs(This,pIGPMSearchCriteria,ppIGPMSOMCollection) ) 

#define IGPMDomain2_GetWMIFilter(This,bstrPath,ppWMIFilter)	\
    ( (This)->lpVtbl -> GetWMIFilter(This,bstrPath,ppWMIFilter) ) 

#define IGPMDomain2_SearchWMIFilters(This,pIGPMSearchCriteria,ppIGPMWMIFilterCollection)	\
    ( (This)->lpVtbl -> SearchWMIFilters(This,pIGPMSearchCriteria,ppIGPMWMIFilterCollection) ) 


#define IGPMDomain2_CreateStarterGPO(This,ppnewTemplate)	\
    ( (This)->lpVtbl -> CreateStarterGPO(This,ppnewTemplate) ) 

#define IGPMDomain2_CreateGPOFromStarterGPO(This,pGPOTemplate,ppnewGPO)	\
    ( (This)->lpVtbl -> CreateGPOFromStarterGPO(This,pGPOTemplate,ppnewGPO) ) 

#define IGPMDomain2_GetStarterGPO(This,bstrGuid,ppTemplate)	\
    ( (This)->lpVtbl -> GetStarterGPO(This,bstrGuid,ppTemplate) ) 

#define IGPMDomain2_SearchStarterGPOs(This,pIGPMSearchCriteria,ppIGPMTemplateCollection)	\
    ( (This)->lpVtbl -> SearchStarterGPOs(This,pIGPMSearchCriteria,ppIGPMTemplateCollection) ) 

#define IGPMDomain2_LoadStarterGPO(This,bstrLoadFile,bOverwrite,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> LoadStarterGPO(This,bstrLoadFile,bOverwrite,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMDomain2_RestoreStarterGPO(This,pIGPMTmplBackup,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> RestoreStarterGPO(This,pIGPMTmplBackup,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMDomain2_INTERFACE_DEFINED__ */


#ifndef __IGPMConstants2_INTERFACE_DEFINED__
#define __IGPMConstants2_INTERFACE_DEFINED__

/* interface IGPMConstants2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMConstants2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05AE21B0-AC09-4032-A26F-9E7DA786DC19")
    IGPMConstants2 : public IGPMConstants
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupTypeGPO( 
            /* [retval][out] */ __RPC__out GPMBackupType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackupTypeStarterGPO( 
            /* [retval][out] */ __RPC__out GPMBackupType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StarterGPOTypeSystem( 
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StarterGPOTypeCustom( 
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyStarterGPOPermissions( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyStarterGPOEffectivePermissions( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyStarterGPODisplayName( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyStarterGPOID( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPropertyStarterGPODomain( 
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermStarterGPORead( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermStarterGPOEdit( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermStarterGPOFullControl( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermStarterGPOCustom( 
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportLegacy( 
            /* [retval][out] */ __RPC__out GPMReportingOptions *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportComments( 
            /* [retval][out] */ __RPC__out GPMReportingOptions *pVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMConstants2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMConstants2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMConstants2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMConstants2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMConstants2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMConstants2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMConstants2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMConstants2 * This,
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
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOApply)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOApply )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPORead)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPORead )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOEdit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOEdit )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOEditSecurityAndDelete)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOEditSecurityAndDelete )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermGPOCustom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermGPOCustom )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermWMIFilterEdit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermWMIFilterEdit )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermWMIFilterFullControl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermWMIFilterFullControl )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermWMIFilterCustom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermWMIFilterCustom )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMLink)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMLink )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMLogging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMLogging )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMPlanning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMPlanning )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMGPOCreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMGPOCreate )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMWMICreate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMWMICreate )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_PermSOMWMIFullControl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermSOMWMIFullControl )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOPermissions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOPermissions )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOEffectivePermissions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOEffectivePermissions )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPODisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPODisplayName )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOWMIFilter)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOWMIFilter )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOID )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOComputerExtensions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOComputerExtensions )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPOUserExtensions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPOUserExtensions )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertySOMLinks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertySOMLinks )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyGPODomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyGPODomain )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchPropertyBackupMostRecent)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyBackupMostRecent )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpEquals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpEquals )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpContains)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpContains )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpNotContains)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpNotContains )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SearchOpNotEquals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchOpNotEquals )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchOperation *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_UsePDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsePDC )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_UseAnyDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseAnyDC )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DoNotUseW2KDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotUseW2KDC )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SOMSite)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMSite )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SOMDomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMDomain )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SOMOU)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SOMOU )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSOMType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_SecurityFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SecurityFlags )( 
            __RPC__in IGPMConstants2 * This,
            /* [in] */ VARIANT_BOOL vbOwner,
            /* [in] */ VARIANT_BOOL vbGroup,
            /* [in] */ VARIANT_BOOL vbDACL,
            /* [in] */ VARIANT_BOOL vbSACL,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DoNotValidateDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotValidateDC )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_ReportHTML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportHTML )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMReportType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_ReportXML)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportXML )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMReportType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RSOPModeUnknown)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RSOPModeUnknown )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RSOPModePlanning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RSOPModePlanning )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RSOPModeLogging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RSOPModeLogging )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMRSOPMode *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUser )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeComputer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeComputer )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeLocalGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeLocalGroup )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeGlobalGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeGlobalGroup )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUniversalGroup)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUniversalGroup )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUNCPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUNCPath )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_EntryTypeUnknown)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EntryTypeUnknown )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMEntryType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionSameAsSource)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionSameAsSource )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionNone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionNone )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionByRelativeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionByRelativeName )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_DestinationOptionSet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationOptionSet )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMDestinationOption *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_MigrationTableOnly)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MigrationTableOnly )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_ProcessSecurity)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessSecurity )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopLoggingNoComputer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopLoggingNoComputer )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopLoggingNoUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopLoggingNoUser )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningAssumeSlowLink)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningAssumeSlowLink )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningLoopbackOption)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningLoopbackOption )( 
            __RPC__in IGPMConstants2 * This,
            /* [in] */ VARIANT_BOOL vbMerge,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningAssumeUserWQLFilterTrue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningAssumeUserWQLFilterTrue )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants, get_RsopPlanningAssumeCompWQLFilterTrue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RsopPlanningAssumeCompWQLFilterTrue )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_BackupTypeGPO)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupTypeGPO )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMBackupType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_BackupTypeStarterGPO)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackupTypeStarterGPO )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMBackupType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_StarterGPOTypeSystem)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StarterGPOTypeSystem )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_StarterGPOTypeCustom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StarterGPOTypeCustom )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMStarterGPOType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_SearchPropertyStarterGPOPermissions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyStarterGPOPermissions )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_SearchPropertyStarterGPOEffectivePermissions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyStarterGPOEffectivePermissions )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_SearchPropertyStarterGPODisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyStarterGPODisplayName )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_SearchPropertyStarterGPOID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyStarterGPOID )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_SearchPropertyStarterGPODomain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPropertyStarterGPODomain )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMSearchProperty *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_PermStarterGPORead)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermStarterGPORead )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_PermStarterGPOEdit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermStarterGPOEdit )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_PermStarterGPOFullControl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermStarterGPOFullControl )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_PermStarterGPOCustom)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermStarterGPOCustom )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMPermissionType *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_ReportLegacy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportLegacy )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMReportingOptions *pVal);
        
        DECLSPEC_XFGVIRT(IGPMConstants2, get_ReportComments)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportComments )( 
            __RPC__in IGPMConstants2 * This,
            /* [retval][out] */ __RPC__out GPMReportingOptions *pVal);
        
        END_INTERFACE
    } IGPMConstants2Vtbl;

    interface IGPMConstants2
    {
        CONST_VTBL struct IGPMConstants2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMConstants2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMConstants2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMConstants2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMConstants2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMConstants2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMConstants2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMConstants2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMConstants2_get_PermGPOApply(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOApply(This,pVal) ) 

#define IGPMConstants2_get_PermGPORead(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPORead(This,pVal) ) 

#define IGPMConstants2_get_PermGPOEdit(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOEdit(This,pVal) ) 

#define IGPMConstants2_get_PermGPOEditSecurityAndDelete(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOEditSecurityAndDelete(This,pVal) ) 

#define IGPMConstants2_get_PermGPOCustom(This,pVal)	\
    ( (This)->lpVtbl -> get_PermGPOCustom(This,pVal) ) 

#define IGPMConstants2_get_PermWMIFilterEdit(This,pVal)	\
    ( (This)->lpVtbl -> get_PermWMIFilterEdit(This,pVal) ) 

#define IGPMConstants2_get_PermWMIFilterFullControl(This,pVal)	\
    ( (This)->lpVtbl -> get_PermWMIFilterFullControl(This,pVal) ) 

#define IGPMConstants2_get_PermWMIFilterCustom(This,pVal)	\
    ( (This)->lpVtbl -> get_PermWMIFilterCustom(This,pVal) ) 

#define IGPMConstants2_get_PermSOMLink(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMLink(This,pVal) ) 

#define IGPMConstants2_get_PermSOMLogging(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMLogging(This,pVal) ) 

#define IGPMConstants2_get_PermSOMPlanning(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMPlanning(This,pVal) ) 

#define IGPMConstants2_get_PermSOMGPOCreate(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMGPOCreate(This,pVal) ) 

#define IGPMConstants2_get_PermSOMWMICreate(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMWMICreate(This,pVal) ) 

#define IGPMConstants2_get_PermSOMWMIFullControl(This,pVal)	\
    ( (This)->lpVtbl -> get_PermSOMWMIFullControl(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPOPermissions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOPermissions(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPOEffectivePermissions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOEffectivePermissions(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPODisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPODisplayName(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPOWMIFilter(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOWMIFilter(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPOID(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOID(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPOComputerExtensions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOComputerExtensions(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPOUserExtensions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPOUserExtensions(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertySOMLinks(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertySOMLinks(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyGPODomain(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyGPODomain(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyBackupMostRecent(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyBackupMostRecent(This,pVal) ) 

#define IGPMConstants2_get_SearchOpEquals(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpEquals(This,pVal) ) 

#define IGPMConstants2_get_SearchOpContains(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpContains(This,pVal) ) 

#define IGPMConstants2_get_SearchOpNotContains(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpNotContains(This,pVal) ) 

#define IGPMConstants2_get_SearchOpNotEquals(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchOpNotEquals(This,pVal) ) 

#define IGPMConstants2_get_UsePDC(This,pVal)	\
    ( (This)->lpVtbl -> get_UsePDC(This,pVal) ) 

#define IGPMConstants2_get_UseAnyDC(This,pVal)	\
    ( (This)->lpVtbl -> get_UseAnyDC(This,pVal) ) 

#define IGPMConstants2_get_DoNotUseW2KDC(This,pVal)	\
    ( (This)->lpVtbl -> get_DoNotUseW2KDC(This,pVal) ) 

#define IGPMConstants2_get_SOMSite(This,pVal)	\
    ( (This)->lpVtbl -> get_SOMSite(This,pVal) ) 

#define IGPMConstants2_get_SOMDomain(This,pVal)	\
    ( (This)->lpVtbl -> get_SOMDomain(This,pVal) ) 

#define IGPMConstants2_get_SOMOU(This,pVal)	\
    ( (This)->lpVtbl -> get_SOMOU(This,pVal) ) 

#define IGPMConstants2_get_SecurityFlags(This,vbOwner,vbGroup,vbDACL,vbSACL,pVal)	\
    ( (This)->lpVtbl -> get_SecurityFlags(This,vbOwner,vbGroup,vbDACL,vbSACL,pVal) ) 

#define IGPMConstants2_get_DoNotValidateDC(This,pVal)	\
    ( (This)->lpVtbl -> get_DoNotValidateDC(This,pVal) ) 

#define IGPMConstants2_get_ReportHTML(This,pVal)	\
    ( (This)->lpVtbl -> get_ReportHTML(This,pVal) ) 

#define IGPMConstants2_get_ReportXML(This,pVal)	\
    ( (This)->lpVtbl -> get_ReportXML(This,pVal) ) 

#define IGPMConstants2_get_RSOPModeUnknown(This,pVal)	\
    ( (This)->lpVtbl -> get_RSOPModeUnknown(This,pVal) ) 

#define IGPMConstants2_get_RSOPModePlanning(This,pVal)	\
    ( (This)->lpVtbl -> get_RSOPModePlanning(This,pVal) ) 

#define IGPMConstants2_get_RSOPModeLogging(This,pVal)	\
    ( (This)->lpVtbl -> get_RSOPModeLogging(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeUser(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUser(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeComputer(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeComputer(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeLocalGroup(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeLocalGroup(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeGlobalGroup(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeGlobalGroup(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeUniversalGroup(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUniversalGroup(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeUNCPath(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUNCPath(This,pVal) ) 

#define IGPMConstants2_get_EntryTypeUnknown(This,pVal)	\
    ( (This)->lpVtbl -> get_EntryTypeUnknown(This,pVal) ) 

#define IGPMConstants2_get_DestinationOptionSameAsSource(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionSameAsSource(This,pVal) ) 

#define IGPMConstants2_get_DestinationOptionNone(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionNone(This,pVal) ) 

#define IGPMConstants2_get_DestinationOptionByRelativeName(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionByRelativeName(This,pVal) ) 

#define IGPMConstants2_get_DestinationOptionSet(This,pVal)	\
    ( (This)->lpVtbl -> get_DestinationOptionSet(This,pVal) ) 

#define IGPMConstants2_get_MigrationTableOnly(This,pVal)	\
    ( (This)->lpVtbl -> get_MigrationTableOnly(This,pVal) ) 

#define IGPMConstants2_get_ProcessSecurity(This,pVal)	\
    ( (This)->lpVtbl -> get_ProcessSecurity(This,pVal) ) 

#define IGPMConstants2_get_RsopLoggingNoComputer(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopLoggingNoComputer(This,pVal) ) 

#define IGPMConstants2_get_RsopLoggingNoUser(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopLoggingNoUser(This,pVal) ) 

#define IGPMConstants2_get_RsopPlanningAssumeSlowLink(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningAssumeSlowLink(This,pVal) ) 

#define IGPMConstants2_get_RsopPlanningLoopbackOption(This,vbMerge,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningLoopbackOption(This,vbMerge,pVal) ) 

#define IGPMConstants2_get_RsopPlanningAssumeUserWQLFilterTrue(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningAssumeUserWQLFilterTrue(This,pVal) ) 

#define IGPMConstants2_get_RsopPlanningAssumeCompWQLFilterTrue(This,pVal)	\
    ( (This)->lpVtbl -> get_RsopPlanningAssumeCompWQLFilterTrue(This,pVal) ) 


#define IGPMConstants2_get_BackupTypeGPO(This,pVal)	\
    ( (This)->lpVtbl -> get_BackupTypeGPO(This,pVal) ) 

#define IGPMConstants2_get_BackupTypeStarterGPO(This,pVal)	\
    ( (This)->lpVtbl -> get_BackupTypeStarterGPO(This,pVal) ) 

#define IGPMConstants2_get_StarterGPOTypeSystem(This,pVal)	\
    ( (This)->lpVtbl -> get_StarterGPOTypeSystem(This,pVal) ) 

#define IGPMConstants2_get_StarterGPOTypeCustom(This,pVal)	\
    ( (This)->lpVtbl -> get_StarterGPOTypeCustom(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyStarterGPOPermissions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyStarterGPOPermissions(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyStarterGPOEffectivePermissions(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyStarterGPOEffectivePermissions(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyStarterGPODisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyStarterGPODisplayName(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyStarterGPOID(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyStarterGPOID(This,pVal) ) 

#define IGPMConstants2_get_SearchPropertyStarterGPODomain(This,pVal)	\
    ( (This)->lpVtbl -> get_SearchPropertyStarterGPODomain(This,pVal) ) 

#define IGPMConstants2_get_PermStarterGPORead(This,pVal)	\
    ( (This)->lpVtbl -> get_PermStarterGPORead(This,pVal) ) 

#define IGPMConstants2_get_PermStarterGPOEdit(This,pVal)	\
    ( (This)->lpVtbl -> get_PermStarterGPOEdit(This,pVal) ) 

#define IGPMConstants2_get_PermStarterGPOFullControl(This,pVal)	\
    ( (This)->lpVtbl -> get_PermStarterGPOFullControl(This,pVal) ) 

#define IGPMConstants2_get_PermStarterGPOCustom(This,pVal)	\
    ( (This)->lpVtbl -> get_PermStarterGPOCustom(This,pVal) ) 

#define IGPMConstants2_get_ReportLegacy(This,pVal)	\
    ( (This)->lpVtbl -> get_ReportLegacy(This,pVal) ) 

#define IGPMConstants2_get_ReportComments(This,pVal)	\
    ( (This)->lpVtbl -> get_ReportComments(This,pVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMConstants2_INTERFACE_DEFINED__ */


#ifndef __IGPMGPO2_INTERFACE_DEFINED__
#define __IGPMGPO2_INTERFACE_DEFINED__

/* interface IGPMGPO2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMGPO2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A66A210-B78B-4d99-88E2-C306A817C925")
    IGPMGPO2 : public IGPMGPO
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMGPO2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMGPO2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMGPO2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMGPO2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMGPO2 * This,
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
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, put_DisplayName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_DomainName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainName )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ModificationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModificationTime )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_UserDSVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDSVersionNumber )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ComputerDSVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerDSVersionNumber )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_UserSysvolVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSysvolVersionNumber )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ComputerSysvolVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerSysvolVersionNumber )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIFilter )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppIGPMWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWMIFilter )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in_opt IGPMWMIFilter *pIGPMWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetUserEnabled )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ VARIANT_BOOL vbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetComputerEnabled )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ VARIANT_BOOL vbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsUserEnabled )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsComputerEnabled )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityInfo )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityInfo )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMGPO2 * This);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Backup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ __RPC__in BSTR bstrComment,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Import)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, CopyTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMDomain *pIGPMDomain,
            /* [optional][in] */ __RPC__in VARIANT *pvarNewDisplayName,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityDescriptor )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IDispatch *pSD);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityDescriptor )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppSD);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsACLConsistent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsACLConsistent )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbConsistent);
        
        DECLSPEC_XFGVIRT(IGPMGPO, MakeACLConsistent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MakeACLConsistent )( 
            __RPC__in IGPMGPO2 * This);
        
        DECLSPEC_XFGVIRT(IGPMGPO2, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IGPMGPO2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO2, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IGPMGPO2 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        END_INTERFACE
    } IGPMGPO2Vtbl;

    interface IGPMGPO2
    {
        CONST_VTBL struct IGPMGPO2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMGPO2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMGPO2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMGPO2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMGPO2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMGPO2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMGPO2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMGPO2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMGPO2_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IGPMGPO2_put_DisplayName(This,newVal)	\
    ( (This)->lpVtbl -> put_DisplayName(This,newVal) ) 

#define IGPMGPO2_get_Path(This,pVal)	\
    ( (This)->lpVtbl -> get_Path(This,pVal) ) 

#define IGPMGPO2_get_ID(This,pVal)	\
    ( (This)->lpVtbl -> get_ID(This,pVal) ) 

#define IGPMGPO2_get_DomainName(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainName(This,pVal) ) 

#define IGPMGPO2_get_CreationTime(This,pDate)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pDate) ) 

#define IGPMGPO2_get_ModificationTime(This,pDate)	\
    ( (This)->lpVtbl -> get_ModificationTime(This,pDate) ) 

#define IGPMGPO2_get_UserDSVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_UserDSVersionNumber(This,pVal) ) 

#define IGPMGPO2_get_ComputerDSVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerDSVersionNumber(This,pVal) ) 

#define IGPMGPO2_get_UserSysvolVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_UserSysvolVersionNumber(This,pVal) ) 

#define IGPMGPO2_get_ComputerSysvolVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerSysvolVersionNumber(This,pVal) ) 

#define IGPMGPO2_GetWMIFilter(This,ppIGPMWMIFilter)	\
    ( (This)->lpVtbl -> GetWMIFilter(This,ppIGPMWMIFilter) ) 

#define IGPMGPO2_SetWMIFilter(This,pIGPMWMIFilter)	\
    ( (This)->lpVtbl -> SetWMIFilter(This,pIGPMWMIFilter) ) 

#define IGPMGPO2_SetUserEnabled(This,vbEnabled)	\
    ( (This)->lpVtbl -> SetUserEnabled(This,vbEnabled) ) 

#define IGPMGPO2_SetComputerEnabled(This,vbEnabled)	\
    ( (This)->lpVtbl -> SetComputerEnabled(This,vbEnabled) ) 

#define IGPMGPO2_IsUserEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsUserEnabled(This,pvbEnabled) ) 

#define IGPMGPO2_IsComputerEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsComputerEnabled(This,pvbEnabled) ) 

#define IGPMGPO2_GetSecurityInfo(This,ppSecurityInfo)	\
    ( (This)->lpVtbl -> GetSecurityInfo(This,ppSecurityInfo) ) 

#define IGPMGPO2_SetSecurityInfo(This,pSecurityInfo)	\
    ( (This)->lpVtbl -> SetSecurityInfo(This,pSecurityInfo) ) 

#define IGPMGPO2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IGPMGPO2_Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO2_Import(This,lFlags,pIGPMBackup,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Import(This,lFlags,pIGPMBackup,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO2_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO2_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#define IGPMGPO2_CopyTo(This,lFlags,pIGPMDomain,pvarNewDisplayName,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> CopyTo(This,lFlags,pIGPMDomain,pvarNewDisplayName,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO2_SetSecurityDescriptor(This,lFlags,pSD)	\
    ( (This)->lpVtbl -> SetSecurityDescriptor(This,lFlags,pSD) ) 

#define IGPMGPO2_GetSecurityDescriptor(This,lFlags,ppSD)	\
    ( (This)->lpVtbl -> GetSecurityDescriptor(This,lFlags,ppSD) ) 

#define IGPMGPO2_IsACLConsistent(This,pvbConsistent)	\
    ( (This)->lpVtbl -> IsACLConsistent(This,pvbConsistent) ) 

#define IGPMGPO2_MakeACLConsistent(This)	\
    ( (This)->lpVtbl -> MakeACLConsistent(This) ) 


#define IGPMGPO2_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IGPMGPO2_put_Description(This,newVal)	\
    ( (This)->lpVtbl -> put_Description(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMGPO2_INTERFACE_DEFINED__ */


#ifndef __IGPMDomain3_INTERFACE_DEFINED__
#define __IGPMDomain3_INTERFACE_DEFINED__

/* interface IGPMDomain3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMDomain3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0077FDFE-88C7-4acf-A11D-D10A7C310A03")
    IGPMDomain3 : public IGPMDomain2
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateReport( 
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InfrastructureDC( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InfrastructureDC( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InfrastructureFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMDomain3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMDomain3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMDomain3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMDomain3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMDomain3 * This,
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
        
        DECLSPEC_XFGVIRT(IGPMDomain, get_DomainController)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainController )( 
            __RPC__in IGPMDomain3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain, get_Domain)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Domain )( 
            __RPC__in IGPMDomain3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain, CreateGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppNewGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchGPOs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchGPOs )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPOCollection **ppIGPMGPOCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain, RestoreGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [in] */ long lDCFlags,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetSOM)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSOM )( 
            __RPC__in IGPMDomain3 * This,
            /* [unique][in] */ __RPC__in_opt BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOM **ppSOM);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchSOMs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchSOMs )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSOMCollection **ppIGPMSOMCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain, GetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIFilter )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in BSTR bstrPath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMDomain, SearchWMIFilters)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchWMIFilters )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilterCollection **ppIGPMWMIFilterCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, CreateStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateStarterGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPO **ppnewTemplate);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, CreateGPOFromStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateGPOFromStarterGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMStarterGPO *pGPOTemplate,
            /* [retval][out] */ __RPC__deref_out_opt IGPMGPO **ppnewGPO);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, GetStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStarterGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in BSTR bstrGuid,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPO **ppTemplate);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, SearchStarterGPOs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SearchStarterGPOs )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMSearchCriteria *pIGPMSearchCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IGPMStarterGPOCollection **ppIGPMTemplateCollection);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, LoadStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoadStarterGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in BSTR bstrLoadFile,
            /* [in] */ VARIANT_BOOL bOverwrite,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain2, RestoreStarterGPO)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreStarterGPO )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in_opt IGPMStarterGPOBackup *pIGPMTmplBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain3, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMDomain3, get_InfrastructureDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InfrastructureDC )( 
            __RPC__in IGPMDomain3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain3, put_InfrastructureDC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InfrastructureDC )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMDomain3, put_InfrastructureFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InfrastructureFlags )( 
            __RPC__in IGPMDomain3 * This,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IGPMDomain3Vtbl;

    interface IGPMDomain3
    {
        CONST_VTBL struct IGPMDomain3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMDomain3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMDomain3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMDomain3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMDomain3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMDomain3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMDomain3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMDomain3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMDomain3_get_DomainController(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainController(This,pVal) ) 

#define IGPMDomain3_get_Domain(This,pVal)	\
    ( (This)->lpVtbl -> get_Domain(This,pVal) ) 

#define IGPMDomain3_CreateGPO(This,ppNewGPO)	\
    ( (This)->lpVtbl -> CreateGPO(This,ppNewGPO) ) 

#define IGPMDomain3_GetGPO(This,bstrGuid,ppGPO)	\
    ( (This)->lpVtbl -> GetGPO(This,bstrGuid,ppGPO) ) 

#define IGPMDomain3_SearchGPOs(This,pIGPMSearchCriteria,ppIGPMGPOCollection)	\
    ( (This)->lpVtbl -> SearchGPOs(This,pIGPMSearchCriteria,ppIGPMGPOCollection) ) 

#define IGPMDomain3_RestoreGPO(This,pIGPMBackup,lDCFlags,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> RestoreGPO(This,pIGPMBackup,lDCFlags,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMDomain3_GetSOM(This,bstrPath,ppSOM)	\
    ( (This)->lpVtbl -> GetSOM(This,bstrPath,ppSOM) ) 

#define IGPMDomain3_SearchSOMs(This,pIGPMSearchCriteria,ppIGPMSOMCollection)	\
    ( (This)->lpVtbl -> SearchSOMs(This,pIGPMSearchCriteria,ppIGPMSOMCollection) ) 

#define IGPMDomain3_GetWMIFilter(This,bstrPath,ppWMIFilter)	\
    ( (This)->lpVtbl -> GetWMIFilter(This,bstrPath,ppWMIFilter) ) 

#define IGPMDomain3_SearchWMIFilters(This,pIGPMSearchCriteria,ppIGPMWMIFilterCollection)	\
    ( (This)->lpVtbl -> SearchWMIFilters(This,pIGPMSearchCriteria,ppIGPMWMIFilterCollection) ) 


#define IGPMDomain3_CreateStarterGPO(This,ppnewTemplate)	\
    ( (This)->lpVtbl -> CreateStarterGPO(This,ppnewTemplate) ) 

#define IGPMDomain3_CreateGPOFromStarterGPO(This,pGPOTemplate,ppnewGPO)	\
    ( (This)->lpVtbl -> CreateGPOFromStarterGPO(This,pGPOTemplate,ppnewGPO) ) 

#define IGPMDomain3_GetStarterGPO(This,bstrGuid,ppTemplate)	\
    ( (This)->lpVtbl -> GetStarterGPO(This,bstrGuid,ppTemplate) ) 

#define IGPMDomain3_SearchStarterGPOs(This,pIGPMSearchCriteria,ppIGPMTemplateCollection)	\
    ( (This)->lpVtbl -> SearchStarterGPOs(This,pIGPMSearchCriteria,ppIGPMTemplateCollection) ) 

#define IGPMDomain3_LoadStarterGPO(This,bstrLoadFile,bOverwrite,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> LoadStarterGPO(This,bstrLoadFile,bOverwrite,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMDomain3_RestoreStarterGPO(This,pIGPMTmplBackup,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> RestoreStarterGPO(This,pIGPMTmplBackup,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 


#define IGPMDomain3_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMDomain3_get_InfrastructureDC(This,pVal)	\
    ( (This)->lpVtbl -> get_InfrastructureDC(This,pVal) ) 

#define IGPMDomain3_put_InfrastructureDC(This,newVal)	\
    ( (This)->lpVtbl -> put_InfrastructureDC(This,newVal) ) 

#define IGPMDomain3_put_InfrastructureFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_InfrastructureFlags(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMDomain3_INTERFACE_DEFINED__ */


#ifndef __IGPMGPO3_INTERFACE_DEFINED__
#define __IGPMGPO3_INTERFACE_DEFINED__

/* interface IGPMGPO3 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IGPMGPO3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7CF123A1-F94A-4112-BFAE-6AA1DB9CB248")
    IGPMGPO3 : public IGPMGPO2
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InfrastructureDC( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InfrastructureDC( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InfrastructureFlags( 
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGPMGPO3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGPMGPO3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGPMGPO3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IGPMGPO3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IGPMGPO3 * This,
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
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_DisplayName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, put_DisplayName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DisplayName )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_Path)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Path )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_DomainName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainName )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_CreationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CreationTime )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ModificationTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ModificationTime )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out DATE *pDate);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_UserDSVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserDSVersionNumber )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ComputerDSVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerDSVersionNumber )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_UserSysvolVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserSysvolVersionNumber )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, get_ComputerSysvolVersionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ComputerSysvolVersionNumber )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIFilter )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMWMIFilter **ppIGPMWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetWMIFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWMIFilter )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in_opt IGPMWMIFilter *pIGPMWMIFilter);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetUserEnabled )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ VARIANT_BOOL vbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetComputerEnabled )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ VARIANT_BOOL vbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsUserEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsUserEnabled )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsComputerEnabled)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsComputerEnabled )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbEnabled);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityInfo )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IGPMSecurityInfo **ppSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetSecurityInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityInfo )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in_opt IGPMSecurityInfo *pSecurityInfo);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IGPMGPO3 * This);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Backup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Backup )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in BSTR bstrBackupDir,
            /* [in] */ __RPC__in BSTR bstrComment,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, Import)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMBackup *pIGPMBackup,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GenerateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReport )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GenerateReportToFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateReportToFile )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ GPMReportType gpmReportType,
            /* [in] */ __RPC__in BSTR bstrTargetFilePath,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, CopyTo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IGPMDomain *pIGPMDomain,
            /* [optional][in] */ __RPC__in VARIANT *pvarNewDisplayName,
            /* [optional][in] */ __RPC__in VARIANT *pvarMigrationTable,
            /* [optional][in] */ __RPC__in VARIANT *pvarGPMProgress,
            /* [optional][out] */ __RPC__out VARIANT *pvarGPMCancel,
            /* [retval][out] */ __RPC__deref_out_opt IGPMResult **ppIGPMResult);
        
        DECLSPEC_XFGVIRT(IGPMGPO, SetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSecurityDescriptor )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ long lFlags,
            /* [in] */ __RPC__in_opt IDispatch *pSD);
        
        DECLSPEC_XFGVIRT(IGPMGPO, GetSecurityDescriptor)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecurityDescriptor )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppSD);
        
        DECLSPEC_XFGVIRT(IGPMGPO, IsACLConsistent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsACLConsistent )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pvbConsistent);
        
        DECLSPEC_XFGVIRT(IGPMGPO, MakeACLConsistent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MakeACLConsistent )( 
            __RPC__in IGPMGPO3 * This);
        
        DECLSPEC_XFGVIRT(IGPMGPO2, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO2, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO3, get_InfrastructureDC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InfrastructureDC )( 
            __RPC__in IGPMGPO3 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO3, put_InfrastructureDC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InfrastructureDC )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IGPMGPO3, put_InfrastructureFlags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InfrastructureFlags )( 
            __RPC__in IGPMGPO3 * This,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IGPMGPO3Vtbl;

    interface IGPMGPO3
    {
        CONST_VTBL struct IGPMGPO3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGPMGPO3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGPMGPO3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGPMGPO3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGPMGPO3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IGPMGPO3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IGPMGPO3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IGPMGPO3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IGPMGPO3_get_DisplayName(This,pVal)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pVal) ) 

#define IGPMGPO3_put_DisplayName(This,newVal)	\
    ( (This)->lpVtbl -> put_DisplayName(This,newVal) ) 

#define IGPMGPO3_get_Path(This,pVal)	\
    ( (This)->lpVtbl -> get_Path(This,pVal) ) 

#define IGPMGPO3_get_ID(This,pVal)	\
    ( (This)->lpVtbl -> get_ID(This,pVal) ) 

#define IGPMGPO3_get_DomainName(This,pVal)	\
    ( (This)->lpVtbl -> get_DomainName(This,pVal) ) 

#define IGPMGPO3_get_CreationTime(This,pDate)	\
    ( (This)->lpVtbl -> get_CreationTime(This,pDate) ) 

#define IGPMGPO3_get_ModificationTime(This,pDate)	\
    ( (This)->lpVtbl -> get_ModificationTime(This,pDate) ) 

#define IGPMGPO3_get_UserDSVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_UserDSVersionNumber(This,pVal) ) 

#define IGPMGPO3_get_ComputerDSVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerDSVersionNumber(This,pVal) ) 

#define IGPMGPO3_get_UserSysvolVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_UserSysvolVersionNumber(This,pVal) ) 

#define IGPMGPO3_get_ComputerSysvolVersionNumber(This,pVal)	\
    ( (This)->lpVtbl -> get_ComputerSysvolVersionNumber(This,pVal) ) 

#define IGPMGPO3_GetWMIFilter(This,ppIGPMWMIFilter)	\
    ( (This)->lpVtbl -> GetWMIFilter(This,ppIGPMWMIFilter) ) 

#define IGPMGPO3_SetWMIFilter(This,pIGPMWMIFilter)	\
    ( (This)->lpVtbl -> SetWMIFilter(This,pIGPMWMIFilter) ) 

#define IGPMGPO3_SetUserEnabled(This,vbEnabled)	\
    ( (This)->lpVtbl -> SetUserEnabled(This,vbEnabled) ) 

#define IGPMGPO3_SetComputerEnabled(This,vbEnabled)	\
    ( (This)->lpVtbl -> SetComputerEnabled(This,vbEnabled) ) 

#define IGPMGPO3_IsUserEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsUserEnabled(This,pvbEnabled) ) 

#define IGPMGPO3_IsComputerEnabled(This,pvbEnabled)	\
    ( (This)->lpVtbl -> IsComputerEnabled(This,pvbEnabled) ) 

#define IGPMGPO3_GetSecurityInfo(This,ppSecurityInfo)	\
    ( (This)->lpVtbl -> GetSecurityInfo(This,ppSecurityInfo) ) 

#define IGPMGPO3_SetSecurityInfo(This,pSecurityInfo)	\
    ( (This)->lpVtbl -> SetSecurityInfo(This,pSecurityInfo) ) 

#define IGPMGPO3_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IGPMGPO3_Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Backup(This,bstrBackupDir,bstrComment,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO3_Import(This,lFlags,pIGPMBackup,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> Import(This,lFlags,pIGPMBackup,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO3_GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReport(This,gpmReportType,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO3_GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult)	\
    ( (This)->lpVtbl -> GenerateReportToFile(This,gpmReportType,bstrTargetFilePath,ppIGPMResult) ) 

#define IGPMGPO3_CopyTo(This,lFlags,pIGPMDomain,pvarNewDisplayName,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult)	\
    ( (This)->lpVtbl -> CopyTo(This,lFlags,pIGPMDomain,pvarNewDisplayName,pvarMigrationTable,pvarGPMProgress,pvarGPMCancel,ppIGPMResult) ) 

#define IGPMGPO3_SetSecurityDescriptor(This,lFlags,pSD)	\
    ( (This)->lpVtbl -> SetSecurityDescriptor(This,lFlags,pSD) ) 

#define IGPMGPO3_GetSecurityDescriptor(This,lFlags,ppSD)	\
    ( (This)->lpVtbl -> GetSecurityDescriptor(This,lFlags,ppSD) ) 

#define IGPMGPO3_IsACLConsistent(This,pvbConsistent)	\
    ( (This)->lpVtbl -> IsACLConsistent(This,pvbConsistent) ) 

#define IGPMGPO3_MakeACLConsistent(This)	\
    ( (This)->lpVtbl -> MakeACLConsistent(This) ) 


#define IGPMGPO3_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IGPMGPO3_put_Description(This,newVal)	\
    ( (This)->lpVtbl -> put_Description(This,newVal) ) 


#define IGPMGPO3_get_InfrastructureDC(This,pVal)	\
    ( (This)->lpVtbl -> get_InfrastructureDC(This,pVal) ) 

#define IGPMGPO3_put_InfrastructureDC(This,newVal)	\
    ( (This)->lpVtbl -> put_InfrastructureDC(This,newVal) ) 

#define IGPMGPO3_put_InfrastructureFlags(This,dwFlags)	\
    ( (This)->lpVtbl -> put_InfrastructureFlags(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGPMGPO3_INTERFACE_DEFINED__ */



#ifndef __GPMGMTLib_LIBRARY_DEFINED__
#define __GPMGMTLib_LIBRARY_DEFINED__

/* library GPMGMTLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_GPMGMTLib;

EXTERN_C const CLSID CLSID_GPM;

#ifdef __cplusplus

class DECLSPEC_UUID("F5694708-88FE-4B35-BABF-E56162D5FBC8")
GPM;
#endif

EXTERN_C const CLSID CLSID_GPMDomain;

#ifdef __cplusplus

class DECLSPEC_UUID("710901BE-1050-4CB1-838A-C5CFF259E183")
GPMDomain;
#endif

EXTERN_C const CLSID CLSID_GPMSitesContainer;

#ifdef __cplusplus

class DECLSPEC_UUID("229F5C42-852C-4B30-945F-C522BE9BD386")
GPMSitesContainer;
#endif

EXTERN_C const CLSID CLSID_GPMBackupDir;

#ifdef __cplusplus

class DECLSPEC_UUID("FCE4A59D-0F21-4AFA-B859-E6D0C62CD10C")
GPMBackupDir;
#endif

EXTERN_C const CLSID CLSID_GPMSOM;

#ifdef __cplusplus

class DECLSPEC_UUID("32D93FAC-450E-44CF-829C-8B22FF6BDAE1")
GPMSOM;
#endif

EXTERN_C const CLSID CLSID_GPMSearchCriteria;

#ifdef __cplusplus

class DECLSPEC_UUID("17AACA26-5CE0-44FA-8CC0-5259E6483566")
GPMSearchCriteria;
#endif

EXTERN_C const CLSID CLSID_GPMPermission;

#ifdef __cplusplus

class DECLSPEC_UUID("5871A40A-E9C0-46EC-913E-944EF9225A94")
GPMPermission;
#endif

EXTERN_C const CLSID CLSID_GPMSecurityInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("547A5E8F-9162-4516-A4DF-9DDB9686D846")
GPMSecurityInfo;
#endif

EXTERN_C const CLSID CLSID_GPMBackup;

#ifdef __cplusplus

class DECLSPEC_UUID("ED1A54B8-5EFA-482A-93C0-8AD86F0D68C3")
GPMBackup;
#endif

EXTERN_C const CLSID CLSID_GPMBackupCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("EB8F035B-70DB-4A9F-9676-37C25994E9DC")
GPMBackupCollection;
#endif

EXTERN_C const CLSID CLSID_GPMSOMCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("24C1F147-3720-4F5B-A9C3-06B4E4F931D2")
GPMSOMCollection;
#endif

EXTERN_C const CLSID CLSID_GPMWMIFilter;

#ifdef __cplusplus

class DECLSPEC_UUID("626745D8-0DEA-4062-BF60-CFC5B1CA1286")
GPMWMIFilter;
#endif

EXTERN_C const CLSID CLSID_GPMWMIFilterCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("74DC6D28-E820-47D6-A0B8-F08D93D7FA33")
GPMWMIFilterCollection;
#endif

EXTERN_C const CLSID CLSID_GPMRSOP;

#ifdef __cplusplus

class DECLSPEC_UUID("489B0CAF-9EC2-4EB7-91F5-B6F71D43DA8C")
GPMRSOP;
#endif

EXTERN_C const CLSID CLSID_GPMGPO;

#ifdef __cplusplus

class DECLSPEC_UUID("D2CE2994-59B5-4064-B581-4D68486A16C4")
GPMGPO;
#endif

EXTERN_C const CLSID CLSID_GPMGPOCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("7A057325-832D-4DE3-A41F-C780436A4E09")
GPMGPOCollection;
#endif

EXTERN_C const CLSID CLSID_GPMGPOLink;

#ifdef __cplusplus

class DECLSPEC_UUID("C1DF9880-5303-42C6-8A3C-0488E1BF7364")
GPMGPOLink;
#endif

EXTERN_C const CLSID CLSID_GPMGPOLinksCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("F6ED581A-49A5-47E2-B771-FD8DC02B6259")
GPMGPOLinksCollection;
#endif

EXTERN_C const CLSID CLSID_GPMAsyncCancel;

#ifdef __cplusplus

class DECLSPEC_UUID("372796A9-76EC-479D-AD6C-556318ED5F9D")
GPMAsyncCancel;
#endif

EXTERN_C const CLSID CLSID_GPMStatusMsgCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("2824E4BE-4BCC-4CAC-9E60-0E3ED7F12496")
GPMStatusMsgCollection;
#endif

EXTERN_C const CLSID CLSID_GPMStatusMessage;

#ifdef __cplusplus

class DECLSPEC_UUID("4B77CC94-D255-409B-BC62-370881715A19")
GPMStatusMessage;
#endif

EXTERN_C const CLSID CLSID_GPMTrustee;

#ifdef __cplusplus

class DECLSPEC_UUID("C54A700D-19B6-4211-BCB0-E8E2475E471E")
GPMTrustee;
#endif

EXTERN_C const CLSID CLSID_GPMClientSideExtension;

#ifdef __cplusplus

class DECLSPEC_UUID("C1A2E70E-659C-4B1A-940B-F88B0AF9C8A4")
GPMClientSideExtension;
#endif

EXTERN_C const CLSID CLSID_GPMCSECollection;

#ifdef __cplusplus

class DECLSPEC_UUID("CF92B828-2D44-4B61-B10A-B327AFD42DA8")
GPMCSECollection;
#endif

EXTERN_C const CLSID CLSID_GPMConstants;

#ifdef __cplusplus

class DECLSPEC_UUID("3855E880-CD9E-4D0C-9EAF-1579283A1888")
GPMConstants;
#endif

EXTERN_C const CLSID CLSID_GPMResult;

#ifdef __cplusplus

class DECLSPEC_UUID("92101AC0-9287-4206-A3B2-4BDB73D225F6")
GPMResult;
#endif

EXTERN_C const CLSID CLSID_GPMMapEntryCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("0CF75D5B-A3A1-4C55-B4FE-9E149C41F66D")
GPMMapEntryCollection;
#endif

EXTERN_C const CLSID CLSID_GPMMapEntry;

#ifdef __cplusplus

class DECLSPEC_UUID("8C975253-5431-4471-B35D-0626C928258A")
GPMMapEntry;
#endif

EXTERN_C const CLSID CLSID_GPMMigrationTable;

#ifdef __cplusplus

class DECLSPEC_UUID("55AF4043-2A06-4F72-ABEF-631B44079C76")
GPMMigrationTable;
#endif

EXTERN_C const CLSID CLSID_GPMBackupDirEx;

#ifdef __cplusplus

class DECLSPEC_UUID("E8C0988A-CF03-4c5b-8BE2-2AA9AD32AADA")
GPMBackupDirEx;
#endif

EXTERN_C const CLSID CLSID_GPMStarterGPOBackupCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("E75EA59D-1AEB-4cb5-A78A-281DAA582406")
GPMStarterGPOBackupCollection;
#endif

EXTERN_C const CLSID CLSID_GPMStarterGPOBackup;

#ifdef __cplusplus

class DECLSPEC_UUID("389E400A-D8EF-455b-A861-5F9CA34A6A02")
GPMStarterGPOBackup;
#endif

EXTERN_C const CLSID CLSID_GPMTemplate;

#ifdef __cplusplus

class DECLSPEC_UUID("ECF1D454-71DA-4e2f-A8C0-8185465911D9")
GPMTemplate;
#endif

EXTERN_C const CLSID CLSID_GPMStarterGPOCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("82F8AA8B-49BA-43b2-956E-3397F9B94C3A")
GPMStarterGPOCollection;
#endif
#endif /* __GPMGMTLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_gpmgmt_0000_0042 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_gpmgmt_0000_0042_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_gpmgmt_0000_0042_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


