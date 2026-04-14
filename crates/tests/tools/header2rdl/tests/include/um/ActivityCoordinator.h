/****************************************************************************************************
*                                                                                                   *
* activitycoordinator.h - ApiSet Contract for ext-ms-win-resourcemanager-activitycoordinator-l1.lib *
*                                                                                                   *
* Copyright (c) Microsoft Corporation. All rights reserved.                                         *
*                                                                                                   *
****************************************************************************************************/

#pragma once

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <ActivityCoordinatorTypes.h>

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#ifdef __cplusplus
extern "C" {
#endif

STDAPI
CreateActivityCoordinatorPolicy(
    _In_ ACTIVITY_COORDINATOR_POLICY_TEMPLATE policyTemplate,
    _Outptr_result_nullonfailure_ ACTIVITY_COORDINATOR_POLICY* policy
    );

STDAPI
SetActivityCoordinatorPolicyResourceCondition(
    _Inout_ ACTIVITY_COORDINATOR_POLICY policy,
    _In_ ACTIVITY_COORDINATOR_RESOURCE resource,
    _In_ ACTIVITY_COORDINATOR_CONDITION condition
    );

STDAPI
GetActivityCoordinatorPolicyResourceCondition(
    _In_ ACTIVITY_COORDINATOR_POLICY policy,
    _In_ ACTIVITY_COORDINATOR_RESOURCE resource,
    _Out_ ACTIVITY_COORDINATOR_CONDITION* condition
    );

void
WINAPI
DestroyActivityCoordinatorPolicy(
    _In_ _Post_invalid_ ACTIVITY_COORDINATOR_POLICY policy
    );

STDAPI
SubscribeActivityCoordinatorPolicy(
    _In_ ACTIVITY_COORDINATOR_POLICY policy,
    _In_ ACTIVITY_COORDINATOR_CALLBACK callback,
    _In_opt_ void* callbackContext,
    _Outptr_result_nullonfailure_ ACTIVITY_COORDINATOR_SUBSCRIPTION* subscription
    );

STDAPI
UnsubscribeActivityCoordinatorPolicy(
    _In_ _Post_invalid_ ACTIVITY_COORDINATOR_SUBSCRIPTION subscription
    );

BOOL
WINAPI
IsActivityCoordinatorResourceSupported(
    _In_ ACTIVITY_COORDINATOR_RESOURCE Resource
    );

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */

#ifndef ext_ms_win_resourcemanager_activitycoordinator_l1_1_1_query_routines
#define ext_ms_win_resourcemanager_activitycoordinator_l1_1_1_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

BOOLEAN
__stdcall
IsCreateActivityCoordinatorPolicyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSetActivityCoordinatorPolicyResourceConditionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsGetActivityCoordinatorPolicyResourceConditionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsDestroyActivityCoordinatorPolicyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSubscribeActivityCoordinatorPolicyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsUnsubscribeActivityCoordinatorPolicyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsIsActivityCoordinatorResourceSupportedPresent(
    VOID
    );

#ifdef __cplusplus
}
#endif

#endif // endof guard

