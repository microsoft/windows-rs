

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


#ifndef __waasapitypes_h__
#define __waasapitypes_h__

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

/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_waasapitypes_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= NTDDI_WINTHRESHOLD )
#pragma once
#endif

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum tagUpdateImpactLevel
    {
        UpdateImpactLevel_None	= 0,
        UpdateImpactLevel_Low	= ( UpdateImpactLevel_None + 1 ) ,
        UpdateImpactLevel_Medium	= ( UpdateImpactLevel_Low + 1 ) ,
        UpdateImpactLevel_High	= ( UpdateImpactLevel_Medium + 1 ) 
    } 	UpdateImpactLevel;

typedef 
enum tagUpdateAssessmentStatus
    {
        UpdateAssessmentStatus_Latest	= 0,
        UpdateAssessmentStatus_NotLatestSoftRestriction	= ( UpdateAssessmentStatus_Latest + 1 ) ,
        UpdateAssessmentStatus_NotLatestHardRestriction	= ( UpdateAssessmentStatus_NotLatestSoftRestriction + 1 ) ,
        UpdateAssessmentStatus_NotLatestEndOfSupport	= ( UpdateAssessmentStatus_NotLatestHardRestriction + 1 ) ,
        UpdateAssessmentStatus_NotLatestServicingTrain	= ( UpdateAssessmentStatus_NotLatestEndOfSupport + 1 ) ,
        UpdateAssessmentStatus_NotLatestDeferredFeature	= ( UpdateAssessmentStatus_NotLatestServicingTrain + 1 ) ,
        UpdateAssessmentStatus_NotLatestDeferredQuality	= ( UpdateAssessmentStatus_NotLatestDeferredFeature + 1 ) ,
        UpdateAssessmentStatus_NotLatestPausedFeature	= ( UpdateAssessmentStatus_NotLatestDeferredQuality + 1 ) ,
        UpdateAssessmentStatus_NotLatestPausedQuality	= ( UpdateAssessmentStatus_NotLatestPausedFeature + 1 ) ,
        UpdateAssessmentStatus_NotLatestManaged	= ( UpdateAssessmentStatus_NotLatestPausedQuality + 1 ) ,
        UpdateAssessmentStatus_NotLatestUnknown	= ( UpdateAssessmentStatus_NotLatestManaged + 1 ) ,
        UpdateAssessmentStatus_NotLatestTargetedVersion	= ( UpdateAssessmentStatus_NotLatestUnknown + 1 ) 
    } 	UpdateAssessmentStatus;

typedef struct tagUpdateAssessment
    {
    UpdateAssessmentStatus status;
    UpdateImpactLevel impact;
    DWORD daysOutOfDate;
    } 	UpdateAssessment;

typedef struct tagOSUpdateAssessment
    {
    BOOL isEndOfSupport;
    UpdateAssessment assessmentForCurrent;
    UpdateAssessment assessmentForUpToDate;
    UpdateAssessmentStatus securityStatus;
    FILETIME assessmentTime;
    FILETIME releaseInfoTime;
    LPWSTR currentOSBuild;
    FILETIME currentOSReleaseTime;
    LPWSTR upToDateOSBuild;
    FILETIME upToDateOSReleaseTime;
    } 	OSUpdateAssessment;

typedef struct tagCloudCampaignSpec
    {
    LPWSTR name;
    LPWSTR tool;
    } 	CloudCampaignAssessment;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_waasapitypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_waasapitypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


