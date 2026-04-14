/*++ BUILD Version: 0000     Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntsecapi.h

Abstract:

    This module defines the Local Security Authority APIs.

Revision History:

--*/

#include <winapifamily.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)





//
// All the subcategories are named as <Audit_CategoryName_SubCategoryName>
//

#ifdef DEFINE_GUID

/* 0cce9210-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_System_SecurityStateChange_defined)
    DEFINE_GUID(
        Audit_System_SecurityStateChange,
        0x0cce9210,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_System_SecurityStateChange_defined
    #endif
#endif

/* 0cce9211-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_System_SecuritySubsystemExtension_defined)
    DEFINE_GUID(
        Audit_System_SecuritySubsystemExtension,
        0x0cce9211,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_System_SecuritySubsystemExtension_defined
    #endif
#endif

/* 0cce9212-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_System_Integrity_defined)
    DEFINE_GUID(
        Audit_System_Integrity,
        0x0cce9212,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_System_Integrity_defined
    #endif
#endif

/* 0cce9213-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_System_IPSecDriverEvents_defined)
    DEFINE_GUID(
        Audit_System_IPSecDriverEvents,
        0x0cce9213,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_System_IPSecDriverEvents_defined
    #endif
#endif

/* 0cce9214-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_System_Others_defined)
    DEFINE_GUID(
        Audit_System_Others,
        0x0cce9214,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_System_Others_defined
    #endif
#endif

/* 0cce9215-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_Logon_defined)
    DEFINE_GUID(
        Audit_Logon_Logon,
        0x0cce9215,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_Logon_defined
    #endif
#endif

/* 0cce9216-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_Logoff_defined)
    DEFINE_GUID(
        Audit_Logon_Logoff,
        0x0cce9216,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_Logoff_defined
    #endif
#endif

/* 0cce9217-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_AccountLockout_defined)
    DEFINE_GUID(
        Audit_Logon_AccountLockout,
        0x0cce9217,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_AccountLockout_defined
    #endif
#endif

/* 0cce9218-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_IPSecMainMode_defined)
    DEFINE_GUID(
        Audit_Logon_IPSecMainMode,
        0x0cce9218,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_IPSecMainMode_defined
    #endif
#endif

/* 0cce9219-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_IPSecQuickMode_defined)
    DEFINE_GUID(
        Audit_Logon_IPSecQuickMode,
        0x0cce9219,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_IPSecQuickMode_defined
    #endif
#endif

/* 0cce921a-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_IPSecUserMode_defined)
    DEFINE_GUID(
        Audit_Logon_IPSecUserMode,
        0x0cce921a,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_IPSecUserMode_defined
    #endif
#endif

/* 0cce921b-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_SpecialLogon_defined)
    DEFINE_GUID(
        Audit_Logon_SpecialLogon,
        0x0cce921b,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_SpecialLogon_defined
    #endif
#endif

/* 0cce921c-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_Others_defined)
    DEFINE_GUID(
        Audit_Logon_Others,
        0x0cce921c,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_Others_defined
    #endif
#endif

/* 0cce921d-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_FileSystem_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_FileSystem,
        0x0cce921d,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_FileSystem_defined
    #endif
#endif

/* 0cce921e-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_Registry_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_Registry,
        0x0cce921e,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_Registry_defined
    #endif
#endif

/* 0cce921f-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_Kernel_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_Kernel,
        0x0cce921f,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_Kernel_defined
    #endif
#endif

/* 0cce9220-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_Sam_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_Sam,
        0x0cce9220,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_Sam_defined
    #endif
#endif

/* 0cce9221-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_CertificationServices_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_CertificationServices,
        0x0cce9221,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_CertificationServices_defined
    #endif
#endif

/* 0cce9222-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_ApplicationGenerated_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_ApplicationGenerated,
        0x0cce9222,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_ApplicationGenerated_defined
    #endif
#endif

/*
The Audit_ObjectAccess_Handle sub-category behaves different from the other sub-categories.
For handle based audits to be generated (Open handle AuditId: 0x1230, Close handle AuditId:
0x1232), the corresponding object sub-category AND Audit_ObjectAccess_Handle must be
enabled. For eg, to generate handle based audits for Reg keys, both
Audit_ObjectAccess_Registry and Audit_ObjectAccess_Handle must be enabled
*/

/* 0cce9223-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_Handle_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_Handle,
        0x0cce9223,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_Handle_defined
    #endif
#endif

/* 0cce9224-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_Share_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_Share,
        0x0cce9224,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_Share_defined
    #endif
#endif

/* 0cce9225-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_FirewallPacketDrops_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_FirewallPacketDrops,
        0x0cce9225,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_FirewallPacketDrops_defined
    #endif
#endif

/* 0cce9226-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_FirewallConnection_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_FirewallConnection,
        0x0cce9226,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_FirewallConnection_defined
    #endif
#endif

/* 0cce9227-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_Other_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_Other,
        0x0cce9227,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_Other_defined
    #endif
#endif

/* 0cce9228-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PrivilegeUse_Sensitive_defined)
    DEFINE_GUID(
        Audit_PrivilegeUse_Sensitive,
        0x0cce9228,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PrivilegeUse_Sensitive_defined
    #endif
#endif

/* 0cce9229-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PrivilegeUse_NonSensitive_defined)
    DEFINE_GUID(
        Audit_PrivilegeUse_NonSensitive,
        0x0cce9229,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PrivilegeUse_NonSensitive_defined
    #endif
#endif

/* 0cce922a-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PrivilegeUse_Others_defined)
    DEFINE_GUID(
        Audit_PrivilegeUse_Others,
        0x0cce922a,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PrivilegeUse_Others_defined
    #endif
#endif

/* 0cce922b-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_ProcessCreation_defined)
    DEFINE_GUID(
        Audit_DetailedTracking_ProcessCreation,
        0x0cce922b,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_ProcessCreation_defined
    #endif
#endif

/* 0cce922c-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_ProcessTermination_defined)
    DEFINE_GUID(
        Audit_DetailedTracking_ProcessTermination,
        0x0cce922c,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_ProcessTermination_defined
    #endif
#endif

/* 0cce922d-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_DpapiActivity_defined)
    DEFINE_GUID(
        Audit_DetailedTracking_DpapiActivity,
        0x0cce922d,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_DpapiActivity_defined
    #endif
#endif

/* 0cce922e-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_RpcCall_defined)
    DEFINE_GUID(
        Audit_DetailedTracking_RpcCall,
        0x0cce922e,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_RpcCall_defined
    #endif
#endif

/* 0cce922f-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_AuditPolicy_defined)
    DEFINE_GUID(
        Audit_PolicyChange_AuditPolicy,
        0x0cce922f,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_AuditPolicy_defined
    #endif
#endif

/* 0cce9230-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_AuthenticationPolicy_defined)
    DEFINE_GUID(
        Audit_PolicyChange_AuthenticationPolicy,
        0x0cce9230,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_AuthenticationPolicy_defined
    #endif
#endif

/* 0cce9231-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_AuthorizationPolicy_defined)
    DEFINE_GUID(
        Audit_PolicyChange_AuthorizationPolicy,
        0x0cce9231,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_AuthorizationPolicy_defined
    #endif
#endif

/* 0cce9232-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_MpsscvRulePolicy_defined)
    DEFINE_GUID(
        Audit_PolicyChange_MpsscvRulePolicy,
        0x0cce9232,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_MpsscvRulePolicy_defined
    #endif
#endif

/* 0cce9233-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_WfpIPSecPolicy_defined)
    DEFINE_GUID(
        Audit_PolicyChange_WfpIPSecPolicy,
        0x0cce9233,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_WfpIPSecPolicy_defined
    #endif
#endif

/* 0cce9234-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_Others_defined)
    DEFINE_GUID(
        Audit_PolicyChange_Others,
        0x0cce9234,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_Others_defined
    #endif
#endif

/* 0cce9235-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_UserAccount_defined)
    DEFINE_GUID(
        Audit_AccountManagement_UserAccount,
        0x0cce9235,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_UserAccount_defined
    #endif
#endif

/* 0cce9236-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_ComputerAccount_defined)
    DEFINE_GUID(
        Audit_AccountManagement_ComputerAccount,
        0x0cce9236,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_ComputerAccount_defined
    #endif
#endif

/* 0cce9237-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_SecurityGroup_defined)
    DEFINE_GUID(
        Audit_AccountManagement_SecurityGroup,
        0x0cce9237,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_SecurityGroup_defined
    #endif
#endif

/* 0cce9238-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_DistributionGroup_defined)
    DEFINE_GUID(
        Audit_AccountManagement_DistributionGroup,
        0x0cce9238,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_DistributionGroup_defined
    #endif
#endif

/* 0cce9239-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_ApplicationGroup_defined)
    DEFINE_GUID(
        Audit_AccountManagement_ApplicationGroup,
        0x0cce9239,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_ApplicationGroup_defined
    #endif
#endif

/* 0cce923a-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_Others_defined)
    DEFINE_GUID(
        Audit_AccountManagement_Others,
        0x0cce923a,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_Others_defined
    #endif
#endif

/* 0cce923b-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DSAccess_DSAccess_defined)
    DEFINE_GUID(
        Audit_DSAccess_DSAccess,
        0x0cce923b,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DSAccess_DSAccess_defined
    #endif
#endif

/* 0cce923c-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DsAccess_AdAuditChanges_defined)
    DEFINE_GUID(
        Audit_DsAccess_AdAuditChanges,
        0x0cce923c,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DsAccess_AdAuditChanges_defined
    #endif
#endif

/* 0cce923d-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Ds_Replication_defined)
    DEFINE_GUID(
        Audit_Ds_Replication,
        0x0cce923d,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Ds_Replication_defined
    #endif
#endif

/* 0cce923e-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Ds_DetailedReplication_defined)
    DEFINE_GUID(
        Audit_Ds_DetailedReplication,
        0x0cce923e,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Ds_DetailedReplication_defined
    #endif
#endif

/* 0cce923f-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountLogon_CredentialValidation_defined)
    DEFINE_GUID(
        Audit_AccountLogon_CredentialValidation,
        0x0cce923f,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountLogon_CredentialValidation_defined
    #endif
#endif

/* 0cce9240-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountLogon_Kerberos_defined)
    DEFINE_GUID(
        Audit_AccountLogon_Kerberos,
        0x0cce9240,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountLogon_Kerberos_defined
    #endif
#endif

/* 0cce9241-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountLogon_Others_defined)
    DEFINE_GUID(
        Audit_AccountLogon_Others,
        0x0cce9241,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountLogon_Others_defined
    #endif
#endif

/* 0cce9242-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountLogon_KerbCredentialValidation_defined)
    DEFINE_GUID(
        Audit_AccountLogon_KerbCredentialValidation,
        0x0cce9242,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountLogon_KerbCredentialValidation_defined
    #endif
#endif

/* 0cce9243-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_NPS_defined)
    DEFINE_GUID(
        Audit_Logon_NPS,
        0x0cce9243,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_NPS_defined
    #endif
#endif

/* 0cce9244-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_DetailedFileShare_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_DetailedFileShare,
        0x0cce9244,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_DetailedFileShare_defined
    #endif
#endif

/* 0cce9245-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_RemovableStorage_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_RemovableStorage,
        0x0cce9245,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_RemovableStorage_defined
    #endif
#endif

/* 0cce9246-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_CbacStaging_defined)
    DEFINE_GUID(
        Audit_ObjectAccess_CbacStaging,
        0x0cce9246,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_CbacStaging_defined
    #endif
#endif

/* 0cce9247-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_Claims_defined)
    DEFINE_GUID(
        Audit_Logon_Claims,
        0x0cce9247,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_Claims_defined
    #endif
#endif

/* 0cce9248-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_PnpActivity_defined)
    DEFINE_GUID(
        Audit_DetailedTracking_PnpActivity,
        0x0cce9248,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_PnpActivity_defined
    #endif
#endif

/* 0cce9249-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_Groups_defined)
    DEFINE_GUID(
        Audit_Logon_Groups,
        0x0cce9249,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_Groups_defined
    #endif
#endif

/* 0cce924a-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_TokenRightAdjusted_defined)
    DEFINE_GUID(
        Audit_DetailedTracking_TokenRightAdjusted,
        0x0cce924a,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_TokenRightAdjusted_defined
    #endif
#endif

/* 0cce924b-69ae-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_AccessRights_defined)
    DEFINE_GUID(
        Audit_Logon_AccessRights,
        0x0cce924b,
        0x69ae, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_AccessRights_defined
    #endif
#endif

#endif // DEFINE_GUID


//
// All categories are named as <Audit_CategoryName>
//

#ifdef DEFINE_GUID

/* 69979848-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_System_defined)
    DEFINE_GUID(
        Audit_System,
        0x69979848,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_System_defined
    #endif
#endif

/* 69979849-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_Logon_defined)
    DEFINE_GUID(
        Audit_Logon,
        0x69979849,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_Logon_defined
    #endif
#endif

/* 6997984a-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_ObjectAccess_defined)
    DEFINE_GUID(
        Audit_ObjectAccess,
        0x6997984a,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_ObjectAccess_defined
    #endif
#endif

/* 6997984b-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PrivilegeUse_defined)
    DEFINE_GUID(
        Audit_PrivilegeUse,
        0x6997984b,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PrivilegeUse_defined
    #endif
#endif

/* 6997984c-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DetailedTracking_defined)
    DEFINE_GUID(
        Audit_DetailedTracking,
        0x6997984c,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DetailedTracking_defined
    #endif
#endif

/* 6997984d-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_PolicyChange_defined)
    DEFINE_GUID(
        Audit_PolicyChange,
        0x6997984d,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_PolicyChange_defined
    #endif
#endif

/* 6997984e-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountManagement_defined)
    DEFINE_GUID(
        Audit_AccountManagement,
        0x6997984e,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountManagement_defined
    #endif
#endif

/* 6997984f-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_DirectoryServiceAccess_defined)
    DEFINE_GUID(
        Audit_DirectoryServiceAccess,
        0x6997984f,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_DirectoryServiceAccess_defined
    #endif
#endif

/* 69979850-797a-11d9-bed3-505054503030 */
#if !defined(INITGUID) || !defined(Audit_AccountLogon_defined)
    DEFINE_GUID(
        Audit_AccountLogon,
        0x69979850,
        0x797a, 0x11d9, 0xbe, 0xd3, 0x50, 0x50, 0x54, 0x50, 0x30, 0x30
        );
    #ifdef INITGUID
    #define Audit_AccountLogon_defined
    #endif
#endif

#endif // DEFINE_GUID




#ifndef _NTSECAPI_
#define _NTSECAPI_

#ifdef __cplusplus
extern "C" {
#endif

#ifndef _NTDEF_
typedef _Return_type_success_(return >= 0) LONG NTSTATUS, *PNTSTATUS;
#endif

#include <lsalookup.h>
#ifndef _NTLSA_IFS_
// begin_ntifs


//
// Security operation mode of the system is held in a control
// longword.
//

typedef ULONG  LSA_OPERATIONAL_MODE, *PLSA_OPERATIONAL_MODE;

// end_ntifs
#endif // _NTLSA_IFS_

//
// The flags in the security operational mode are defined
// as:
//
//    PasswordProtected - Some level of authentication (such as
//        a password) must be provided by users before they are
//        allowed to use the system.  Once set, this value will
//        not be cleared without re-booting the system.
//
//    IndividualAccounts - Each user must identify an account to
//        logon to.  This flag is only meaningful if the
//        PasswordProtected flag is also set.  If this flag is
//        not set and the PasswordProtected flag is set, then all
//        users may logon to the same account.  Once set, this value
//        will not be cleared without re-booting the system.
//
//    MandatoryAccess - Indicates the system is running in a mandatory
//        access control mode (e.g., B-level as defined by the U.S.A's
//        Department of Defense's "Orange Book").  This is not utilized
//        in the current release of NT.  This flag is only meaningful
//        if both the PasswordProtected and IndividualAccounts flags are
//        set.  Once set, this value will not be cleared without
//        re-booting the system.
//
//    LogFull - Indicates the system has been brought up in a mode in
//        which if must perform security auditing, but its audit log
//        is full.  This may (should) restrict the operations that
//        can occur until the audit log is made not-full again.  THIS
//        VALUE MAY BE CLEARED WHILE THE SYSTEM IS RUNNING (I.E., WITHOUT
//        REBOOTING).
//
// If the PasswordProtected flag is not set, then the system is running
// without security, and user interface should be adjusted appropriately.
//

#define LSA_MODE_PASSWORD_PROTECTED     (0x00000001L)
#define LSA_MODE_INDIVIDUAL_ACCOUNTS    (0x00000002L)
#define LSA_MODE_MANDATORY_ACCESS       (0x00000004L)
#define LSA_MODE_LOG_FULL               (0x00000008L)

#ifndef _NTLSA_IFS_
// begin_ntifs
//
// Used by a logon process to indicate what type of logon is being
// requested.
//

typedef enum _SECURITY_LOGON_TYPE {
    UndefinedLogonType = 0, // This is used to specify an undefied logon type
    Interactive = 2,      // Interactively logged on (locally or remotely)
    Network,              // Accessing system via network
    Batch,                // Started via a batch queue
    Service,              // Service started by service controller
    Proxy,                // Proxy logon
    Unlock,               // Unlock workstation
    NetworkCleartext,     // Network logon with cleartext credentials
    NewCredentials,       // Clone caller, new default credentials
    //The types below only exist in Windows XP and greater
#if (_WIN32_WINNT >= 0x0501)
    RemoteInteractive,  // Remote, yet interactive. Terminal server
    CachedInteractive,  // Try cached credentials without hitting the net.
    // The types below only exist in Windows Server 2003 and greater
#endif
#if (_WIN32_WINNT >= 0x0502)
    CachedRemoteInteractive, // Same as RemoteInteractive, this is used internally for auditing purpose
    CachedUnlock        // Cached Unlock workstation
#endif
} SECURITY_LOGON_TYPE, *PSECURITY_LOGON_TYPE;

// end_ntifs
#endif // _NTLSA_IFS_

#ifndef _NTLSA_IFS_
// begin_ntifs

//
// All of this stuff (between the Ifndef _NTLSA_AUDIT_ and its endif) were not
// present in NTIFS prior to Windows Server 2003 SP1. All of the definitions however
// exist down to windows 2000 (except for the few exceptions noted in the code).
//

#ifndef _NTLSA_AUDIT_
#define _NTLSA_AUDIT_

/////////////////////////////////////////////////////////////////////////
//                                                                     //
// Data types related to Auditing                                      //
//                                                                     //
/////////////////////////////////////////////////////////////////////////


//
// The following enumerated type is used between the reference monitor and
// LSA in the generation of audit messages.  It is used to indicate the
// type of data being passed as a parameter from the reference monitor
// to LSA.  LSA is responsible for transforming the specified data type
// into a set of unicode strings that are added to the event record in
// the audit log.
//

typedef enum _SE_ADT_PARAMETER_TYPE {

    SeAdtParmTypeNone = 0,          //Produces 1 parameter
                                    //Received value:
                                    //
                                    //  None.
                                    //
                                    //Results in:
                                    //
                                    //  a unicode string containing "-".
                                    //
                                    //Note:  This is typically used to
                                    //       indicate that a parameter value
                                    //       was not available.
                                    //

    SeAdtParmTypeString,            //Produces 1 parameter.
                                    //Received Value:
                                    //
                                    //  Unicode String (variable length)
                                    //
                                    //Results in:
                                    //
                                    //  No transformation.  The string
                                    //  entered into the event record as
                                    //  received.
                                    //
                                    // The Address value of the audit info
                                    // should be a pointer to a UNICODE_STRING
                                    // structure.



    SeAdtParmTypeFileSpec,          //Produces 1 parameter.
                                    //Received value:
                                    //
                                    //  Unicode string containing a file or
                                    //  directory name.
                                    //
                                    //Results in:
                                    //
                                    //  Unicode string with the prefix of the
                                    //  file's path replaced by a drive letter
                                    //  if possible.
                                    //




    SeAdtParmTypeUlong,             //Produces 1 parameter
                                    //Received value:
                                    //
                                    //  Ulong
                                    //
                                    //Results in:
                                    //
                                    //  Unicode string representation of
                                    //  unsigned integer value.


    SeAdtParmTypeSid,               //Produces 1 parameter.
                                    //Received value:
                                    //
                                    //  SID (variable length)
                                    //
                                    //Results in:
                                    //
                                    //  String representation of SID
                                    //




    SeAdtParmTypeLogonId,           //Produces 4 parameters.
                                    //Received Value:
                                    //
                                    //  LUID (fixed length)
                                    //
                                    //Results in:
                                    //
                                    //  param 1: Sid string
                                    //  param 2: Username string
                                    //  param 3: domain name string
                                    //  param 4: Logon ID (Luid) string


    SeAdtParmTypeNoLogonId,         //Produces 3 parameters.
                                    //Received value:
                                    //
                                    //  None.
                                    //
                                    //Results in:
                                    //
                                    //  param 1: "-"
                                    //  param 2: "-"
                                    //  param 3: "-"
                                    //  param 4: "-"
                                    //
                                    //Note:
                                    //
                                    //  This type is used when a logon ID
                                    //  is needed, but one is not available
                                    //  to pass.  For example, if an
                                    //  impersonation logon ID is expected
                                    //  but the subject is not impersonating
                                    //  anyone.
                                    //

    SeAdtParmTypeAccessMask,        //Produces 1 parameter with formatting.
                                    //Received value:
                                    //
                                    //  ACCESS_MASK followed by
                                    //  a Unicode string.  The unicode
                                    //  string contains the name of the
                                    //  type of object the access mask
                                    //  applies to.  The event's source
                                    //  further qualifies the object type.
                                    //
                                    //Results in:
                                    //
                                    //  formatted unicode string built to
                                    //  take advantage of the specified
                                    //  source's parameter message file.
                                    //
                                    //Note:
                                    //
                                    //  An access mask containing three
                                    //  access types for a Widget object
                                    //  type (defined by the Foozle source)
                                    //  might end up looking like:
                                    //
                                    //      %%1062\n\t\t%1066\n\t\t%%601
                                    //
                                    //  The %%numbers are signals to the
                                    //  event viewer to perform parameter
                                    //  substitution before display.
                                    //



    SeAdtParmTypePrivs,             //Produces 1 parameter with formatting.
                                    //Received value:
                                    //
                                    //Results in:
                                    //
                                    //  formatted unicode string similar to
                                    //  that for access types.  Each priv
                                    //  will be formatted to be displayed
                                    //  on its own line.  E.g.,
                                    //
                                    //      %%642\n\t\t%%651\n\t\t%%655
                                    //

    SeAdtParmTypeObjectTypes,       //Produces 10 parameters with formatting.
                                    //Received value:
                                    //
                                    // Produces a list a stringized GUIDS along
                                    // with information similar to that for
                                    // an access mask.

    SeAdtParmTypeHexUlong,          //Produces 1 parameter
                                    //Received value:
                                    //
                                    //  Ulong
                                    //
                                    //Results in:
                                    //
                                    //  Unicode string representation of
                                    //  unsigned integer value in hexadecimal.

// In W2k this value did not exist, it was ParmTypeLUID

    SeAdtParmTypePtr,               //Produces 1 parameter
                                    //Received value:
                                    //
                                    //  pointer
                                    //
                                    //Results in:
                                    //
                                    //  Unicode string representation of
                                    //  unsigned integer value in hexadecimal.

//
// Everything below exists only in Windows XP and greater
//

    SeAdtParmTypeTime,              //Produces 2 parameters
                                    //Received value:
                                    //
                                    //  LARGE_INTEGER
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of
                                    // date and time.

                                    //
    SeAdtParmTypeGuid,              //Produces 1 parameter
                                    //Received value:
                                    //
                                    //  GUID pointer
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of GUID
                                    // {xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}
                                    //

//
// Everything below exists only in Windows Server 2003 and Greater
//

    SeAdtParmTypeLuid,              //
                                    //Produces 1 parameter
                                    //Received value:
                                    //
                                    // LUID
                                    //
                                    //Results in:
                                    //
                                    // Hex LUID
                                    //

    SeAdtParmTypeHexInt64,          //Produces 1 parameter
                                    //Received value:
                                    //
                                    //  64 bit integer
                                    //
                                    //Results in:
                                    //
                                    //  Unicode string representation of
                                    //  unsigned integer value in hexadecimal.

    SeAdtParmTypeStringList,        //Produces 1 parameter
                                    //Received value:
                                    //
                                    // ptr to LSAP_ADT_STRING_LIST
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of
                                    // concatenation of the strings in the list

    SeAdtParmTypeSidList,           //Produces 1 parameter
                                    //Received value:
                                    //
                                    // ptr to LSAP_ADT_SID_LIST
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of
                                    // concatenation of the SIDs in the list

    SeAdtParmTypeDuration,          //Produces 1 parameters
                                    //Received value:
                                    //
                                    //  LARGE_INTEGER
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of
                                    // a duration.

    SeAdtParmTypeUserAccountControl,//Produces 3 parameters
                                    //Received value:
                                    //
                                    // old and new UserAccountControl values
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representations of
                                    // the flags in UserAccountControl.
                                    // 1 - old value in hex
                                    // 2 - new value in hex
                                    // 3 - difference as strings

    SeAdtParmTypeNoUac,             //Produces 3 parameters
                                    //Received value:
                                    //
                                    // none
                                    //
                                    //Results in:
                                    //
                                    // Three dashes ('-') as unicode strings.

    SeAdtParmTypeMessage,           //Produces 1 Parameter
                                    //Received value:
                                    //
                                    //  ULONG (MessageNo from msobjs.mc)
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of
                                    // %%MessageNo which the event viewer
                                    // will replace with the message string
                                    // from msobjs.mc

    SeAdtParmTypeDateTime,          //Produces 1 Parameter
                                    //Received value:
                                    //
                                    //  LARGE_INTEGER
                                    //
                                    //Results in:
                                    //
                                    // Unicode string representation of
                                    // date and time (in _one_ string).

    SeAdtParmTypeSockAddr,          // Produces 2 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to SOCKADDR_IN/SOCKADDR_IN6
                                    // structure
                                    //
                                    // Results in:
                                    //
                                    // param 1: IP address string
                                    // param 2: Port number string
                                    //

//
// Everything below this exists only in Windows Server 2008 and greater
//

    SeAdtParmTypeSD,                // Produces 1 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to SECURITY_DESCRIPTOR
                                    // structure. This HAS to appear in pairs.
                                    // The first parameter will represent the
                                    // old SD and the second parameter will
                                    // represent the New SD
                                    //
                                    // Results in:
                                    //
                                    // SDDL string representation of SD
                                    //

    SeAdtParmTypeLogonHours,        // Produces 1 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to LOGON_HOURS
                                    // structure
                                    //
                                    // Results in:
                                    //
                                    // String representation of allowed logon hours
                                    //

    SeAdtParmTypeLogonIdNoSid,      //Produces 3 parameters.
                                    //Received Value:
                                    //
                                    //  LUID (fixed length)
                                    //
                                    //Results in:
                                    //
                                    //  param 1: Username string
                                    //  param 2: domain name string
                                    //  param 3: Logon ID (Luid) string

    SeAdtParmTypeUlongNoConv,       // Produces 1 parameter.
                                    // Received Value:
                                    // Ulong
                                    //
                                    //Results in:
                                    // Not converted to string
                                    //

    SeAdtParmTypeSockAddrNoPort,    // Produces 1 parameter
                                    //
                                    // Received value:
                                    //
                                    // pointer to SOCKADDR_IN/SOCKADDR_IN6
                                    // structure
                                    //
                                    // Results in:
                                    //
                                    // param 1: IPv4/IPv6 address string
                                    //
//
// Everything below this exists only in Windows Server 2008 and greater
//

    SeAdtParmTypeAccessReason,      // Produces 1 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to SE_ADT_ACCESS_REASON structure
                                    //
                                    // Results in:
                                    //
                                    // String representation of the access reason.
                                    //
//
// Everything below this exists only in Windows Server 2012 and greater
//

    SeAdtParmTypeStagingReason,     // Produces 1 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to SE_ADT_ACCESS_REASON structure
                                    //
                                    // Results in:
                                    //
                                    // String representation of Staging policy's
                                    // access reason.
                                    //

    SeAdtParmTypeResourceAttribute, // Produces 1 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to SECURITY_DESCRIPTOR
                                    // structure
                                    //
                                    // Results in:
                                    //
                                    // SDDL string representation of the
                                    // Resource Attribute ACEs in the SD
                                    //

    SeAdtParmTypeClaims,            // Produces 1 parameters
                                    //
                                    // Received value:
                                    //
                                    // pointer to the structure -
                                    // CLAIM_SECURITY_ATTRIBUTES_INFORMATION
                                    // structure
                                    //
                                    // Results in:
                                    //
                                    // Claims information as attributes, value
                                    // pairs
                                    //

    SeAdtParmTypeLogonIdAsSid,      // Produces 4 parameters.
                                    // Received Value:
                                    //
                                    //  SID  (variable length)
                                    //
                                    //Results in:
                                    //
                                    //  param 1: Sid string (based on SID and not derived from the LUID)
                                    //  param 2: -
                                    //  param 3: -
                                    //  param 4: -

    SeAdtParmTypeMultiSzString,     //Produces 1 parameter
                                    //Received value:
                                    //
                                    // PZZWSTR string
                                    //
                                    //Results in:
                                    //
                                    // Unicode string with each null replaced with /r/n

    SeAdtParmTypeLogonIdEx,         //Produces 4 parameters.
                                    //Received Value:
                                    //
                                    //  LUID (fixed length)
                                    //
                                    //Results in:
                                    //
                                    //  param 1: Sid string
                                    //  param 2: Username string
                                    //  param 3: domain name string
                                    //  param 4: Logon ID (Luid) string

} SE_ADT_PARAMETER_TYPE, *PSE_ADT_PARAMETER_TYPE;

#ifndef GUID_DEFINED
#include <guiddef.h>
#endif /* GUID_DEFINED */

typedef struct _SE_ADT_OBJECT_TYPE {
    GUID ObjectType;
    USHORT Flags;
#define SE_ADT_OBJECT_ONLY 0x1
    USHORT Level;
    ACCESS_MASK AccessMask;
} SE_ADT_OBJECT_TYPE, *PSE_ADT_OBJECT_TYPE;

typedef struct _SE_ADT_PARAMETER_ARRAY_ENTRY {

    SE_ADT_PARAMETER_TYPE Type;
    ULONG Length;
    ULONG_PTR Data[2];
    PVOID Address;

} SE_ADT_PARAMETER_ARRAY_ENTRY, *PSE_ADT_PARAMETER_ARRAY_ENTRY;


typedef struct _SE_ADT_ACCESS_REASON{
    ACCESS_MASK AccessMask;
    ULONG  AccessReasons[32];
    ULONG  ObjectTypeIndex;
    ULONG AccessGranted;
    PSECURITY_DESCRIPTOR SecurityDescriptor;    // multple SDs may be stored here in self-relative way.
} SE_ADT_ACCESS_REASON, *PSE_ADT_ACCESS_REASON;

typedef struct _SE_ADT_CLAIMS {

    ULONG Length;
    PCLAIMS_BLOB Claims; // one claim blob will be stored here in self-relative way

} SE_ADT_CLAIMS, *PSE_ADT_CLAIMS;

//
// Structure that will be passed between the Reference Monitor and LSA
// to transmit auditing information.
//

#define SE_MAX_AUDIT_PARAMETERS 32
#define SE_MAX_GENERIC_AUDIT_PARAMETERS 28

typedef struct _SE_ADT_PARAMETER_ARRAY {

    ULONG CategoryId;
    ULONG AuditId;
    ULONG ParameterCount;
    ULONG Length;
    USHORT FlatSubCategoryId;
    USHORT Type;
    ULONG Flags;
    SE_ADT_PARAMETER_ARRAY_ENTRY Parameters[ SE_MAX_AUDIT_PARAMETERS ];

} SE_ADT_PARAMETER_ARRAY, *PSE_ADT_PARAMETER_ARRAY;

typedef struct _SE_ADT_PARAMETER_ARRAY_EX {

    ULONG CategoryId;
    ULONG AuditId;
    ULONG Version;
    ULONG ParameterCount;
    ULONG Length;
    USHORT FlatSubCategoryId;
    USHORT Type;
    ULONG Flags;
    SE_ADT_PARAMETER_ARRAY_ENTRY Parameters[ SE_MAX_AUDIT_PARAMETERS ];

} SE_ADT_PARAMETER_ARRAY_EX, *PSE_ADT_PARAMETER_ARRAY_EX;


#define SE_ADT_PARAMETERS_SELF_RELATIVE     0x00000001
#define SE_ADT_PARAMETERS_SEND_TO_LSA       0x00000002
#define SE_ADT_PARAMETER_EXTENSIBLE_AUDIT   0x00000004
#define SE_ADT_PARAMETER_GENERIC_AUDIT      0x00000008
#define SE_ADT_PARAMETER_WRITE_SYNCHRONOUS  0x00000010


//
// This macro only existed in Windows Server 2008 and after
//

#define LSAP_SE_ADT_PARAMETER_ARRAY_TRUE_SIZE(AuditParameters)    \
     ( sizeof(SE_ADT_PARAMETER_ARRAY) -                           \
       sizeof(SE_ADT_PARAMETER_ARRAY_ENTRY) *                     \
       (SE_MAX_AUDIT_PARAMETERS - AuditParameters->ParameterCount) )

#endif // _NTLSA_AUDIT_

// end_ntifs
#endif // _NTLSA_IFS_

//
// Audit Event Categories
//
// The following are the built-in types or Categories of audit event.
// WARNING!  This structure is subject to expansion.  The user should not
// compute the number of elements of this type directly, but instead
// should obtain the count of elements by calling LsaQueryInformationPolicy()
// for the PolicyAuditEventsInformation class and extracting the count from
// the MaximumAuditEventCount field of the returned structure.
//

typedef enum _POLICY_AUDIT_EVENT_TYPE {

    AuditCategorySystem = 0,
    AuditCategoryLogon,
    AuditCategoryObjectAccess,
    AuditCategoryPrivilegeUse,
    AuditCategoryDetailedTracking,
    AuditCategoryPolicyChange,
    AuditCategoryAccountManagement,
    AuditCategoryDirectoryServiceAccess,
    AuditCategoryAccountLogon

} POLICY_AUDIT_EVENT_TYPE, *PPOLICY_AUDIT_EVENT_TYPE;


//
// The following defines describe the auditing options for each
// event type
//

// Leave options specified for this event unchanged

#define POLICY_AUDIT_EVENT_UNCHANGED       (0x00000000L)

// Audit successful occurrences of events of this type

#define POLICY_AUDIT_EVENT_SUCCESS         (0x00000001L)

// Audit failed attempts to cause an event of this type to occur

#define POLICY_AUDIT_EVENT_FAILURE         (0x00000002L)

#define POLICY_AUDIT_EVENT_NONE            (0x00000004L)

// Mask of valid event auditing options

#define POLICY_AUDIT_EVENT_MASK \
    (POLICY_AUDIT_EVENT_SUCCESS | \
     POLICY_AUDIT_EVENT_FAILURE | \
     POLICY_AUDIT_EVENT_UNCHANGED | \
     POLICY_AUDIT_EVENT_NONE)

#ifndef _NTDEF_
#ifndef IN
#define IN
#endif

#ifndef OUT
#define OUT
#endif

#ifndef OPTIONAL
#define OPTIONAL
#endif
#endif  // _NTDEF_

//
// Macro for determining whether an API succeeded.
//

#define LSA_SUCCESS(Error) ((LONG)(Error) >= 0)

#ifndef _NTLSA_IFS_
// begin_ntifs

_IRQL_requires_same_
_IRQL_requires_max_(PASSIVE_LEVEL)
NTSTATUS
NTAPI
LsaRegisterLogonProcess (
    _In_ PLSA_STRING LogonProcessName,
    _Out_ PHANDLE LsaHandle,
    _Out_ PLSA_OPERATIONAL_MODE SecurityMode
    );

//
// The function below did not exist in NTIFS before windows XP
// However, the function has always been there, so it is okay to use
// even on w2k
//
_IRQL_requires_same_
_IRQL_requires_max_(PASSIVE_LEVEL)
NTSTATUS
NTAPI
LsaLogonUser (
    _In_        HANDLE          LsaHandle,
    _In_        PLSA_STRING     OriginName,
    _In_        SECURITY_LOGON_TYPE LogonType,
    _In_        ULONG           AuthenticationPackage,
    _In_reads_bytes_(AuthenticationInformationLength) PVOID AuthenticationInformation,
    _In_        ULONG           AuthenticationInformationLength,
    _In_opt_    PTOKEN_GROUPS   LocalGroups,
    _In_        PTOKEN_SOURCE   SourceContext,
    _Out_       PVOID           *ProfileBuffer,
    _Out_       PULONG          ProfileBufferLength,
    _Inout_     PLUID           LogonId,
    _Out_       PHANDLE         Token,
    _Out_       PQUOTA_LIMITS   Quotas,
    _Out_       PNTSTATUS       SubStatus
    );

// end_ntifs

_IRQL_requires_same_
_IRQL_requires_max_(PASSIVE_LEVEL)
NTSTATUS
NTAPI
LsaLookupAuthenticationPackage (
    _In_ HANDLE LsaHandle,
    _In_ PLSA_STRING PackageName,
    _Out_ PULONG AuthenticationPackage
    );

// begin_ntifs

_IRQL_requires_same_
NTSTATUS
NTAPI
LsaFreeReturnBuffer (
    _In_ PVOID Buffer
    );

// end_ntifs

_IRQL_requires_same_
_IRQL_requires_max_(PASSIVE_LEVEL)
NTSTATUS
NTAPI
LsaCallAuthenticationPackage (
    _In_ HANDLE LsaHandle,
    _In_ ULONG AuthenticationPackage,
    _In_reads_bytes_(SubmitBufferLength) PVOID ProtocolSubmitBuffer,
    _In_ ULONG SubmitBufferLength,
    _Outptr_opt_result_buffer_maybenull_(*ReturnBufferLength) PVOID *ProtocolReturnBuffer,
    _Out_opt_ PULONG ReturnBufferLength,
    _Out_opt_ PNTSTATUS ProtocolStatus
    );

_IRQL_requires_same_
_IRQL_requires_max_(PASSIVE_LEVEL)
NTSTATUS
NTAPI
LsaDeregisterLogonProcess (
    _In_ HANDLE LsaHandle
    );

_IRQL_requires_same_
_IRQL_requires_max_(PASSIVE_LEVEL)
NTSTATUS
NTAPI
LsaConnectUntrusted (
    _Out_ PHANDLE LsaHandle
    );

__drv_sameIRQL
NTSTATUS
LsaInsertProtectedProcessAddress (
    __in PVOID BufferAddress,
    __in ULONG BufferSize
    );

__drv_sameIRQL
NTSTATUS
LsaRemoveProtectedProcessAddress (
    __in PVOID BufferAddress,
    __in ULONG BufferSize
    );

#endif // _NTLSA_IFS_


////////////////////////////////////////////////////////////////////////////
//                                                                        //
// Local Security Policy Administration API datatypes and defines         //
//                                                                        //
////////////////////////////////////////////////////////////////////////////

//
// Access types for the Policy object
//

#define POLICY_VIEW_LOCAL_INFORMATION              0x00000001L
#define POLICY_VIEW_AUDIT_INFORMATION              0x00000002L
#define POLICY_GET_PRIVATE_INFORMATION             0x00000004L
#define POLICY_TRUST_ADMIN                         0x00000008L
#define POLICY_CREATE_ACCOUNT                      0x00000010L
#define POLICY_CREATE_SECRET                       0x00000020L
#define POLICY_CREATE_PRIVILEGE                    0x00000040L
#define POLICY_SET_DEFAULT_QUOTA_LIMITS            0x00000080L
#define POLICY_SET_AUDIT_REQUIREMENTS              0x00000100L
#define POLICY_AUDIT_LOG_ADMIN                     0x00000200L
#define POLICY_SERVER_ADMIN                        0x00000400L
#define POLICY_LOOKUP_NAMES                        0x00000800L
#define POLICY_NOTIFICATION                        0x00001000L

#define POLICY_ALL_ACCESS     (STANDARD_RIGHTS_REQUIRED         |\
                               POLICY_VIEW_LOCAL_INFORMATION    |\
                               POLICY_VIEW_AUDIT_INFORMATION    |\
                               POLICY_GET_PRIVATE_INFORMATION   |\
                               POLICY_TRUST_ADMIN               |\
                               POLICY_CREATE_ACCOUNT            |\
                               POLICY_CREATE_SECRET             |\
                               POLICY_CREATE_PRIVILEGE          |\
                               POLICY_SET_DEFAULT_QUOTA_LIMITS  |\
                               POLICY_SET_AUDIT_REQUIREMENTS    |\
                               POLICY_AUDIT_LOG_ADMIN           |\
                               POLICY_SERVER_ADMIN              |\
                               POLICY_LOOKUP_NAMES)


#define POLICY_READ           (STANDARD_RIGHTS_READ             |\
                               POLICY_VIEW_AUDIT_INFORMATION    |\
                               POLICY_GET_PRIVATE_INFORMATION)

#define POLICY_WRITE          (STANDARD_RIGHTS_WRITE            |\
                               POLICY_TRUST_ADMIN               |\
                               POLICY_CREATE_ACCOUNT            |\
                               POLICY_CREATE_SECRET             |\
                               POLICY_CREATE_PRIVILEGE          |\
                               POLICY_SET_DEFAULT_QUOTA_LIMITS  |\
                               POLICY_SET_AUDIT_REQUIREMENTS    |\
                               POLICY_AUDIT_LOG_ADMIN           |\
                               POLICY_SERVER_ADMIN)

#define POLICY_EXECUTE        (STANDARD_RIGHTS_EXECUTE          |\
                               POLICY_VIEW_LOCAL_INFORMATION    |\
                               POLICY_LOOKUP_NAMES)


//
// Legacy policy object specific data types.
//
// The following data type is used in name to SID lookup services to describe
// the domains referenced in the lookup operation.
//

typedef struct _LSA_TRANSLATED_SID {

    SID_NAME_USE Use;
    ULONG RelativeId;
    LONG DomainIndex;

} LSA_TRANSLATED_SID, *PLSA_TRANSLATED_SID;

// where members have the following usage:
//
//     Use - identifies the use of the SID.  If this value is SidUnknown or
//         SidInvalid, then the remainder of the record is not set and
//         should be ignored.
//
//     RelativeId - Contains the relative ID of the translated SID.  The
//         remainder of the SID (the prefix) is obtained using the
//         DomainIndex field.
//
//     DomainIndex - Is the index of an entry in a related
//         LSA_REFERENCED_DOMAIN_LIST data structure describing the
//         domain in which the account was found.
//
//         If there is no corresponding reference domain for an entry, then
//         this field will contain a negative value.
//


//
// The following data type is used to represent the role of the LSA
// server (primary or backup).
//

typedef enum _POLICY_LSA_SERVER_ROLE {

    PolicyServerRoleBackup = 2,
    PolicyServerRolePrimary

} POLICY_LSA_SERVER_ROLE, *PPOLICY_LSA_SERVER_ROLE;

#if (_WIN32_WINNT < 0x0502)
//
// The following data type is used to represent the state of the LSA
// server (enabled or disabled).  Some operations may only be performed on
// an enabled LSA server.
//

typedef enum _POLICY_SERVER_ENABLE_STATE {

    PolicyServerEnabled = 2,
    PolicyServerDisabled

} POLICY_SERVER_ENABLE_STATE, *PPOLICY_SERVER_ENABLE_STATE;
#endif

//
// The following data type is used to specify the auditing options for
// an Audit Event Type.
//

typedef ULONG POLICY_AUDIT_EVENT_OPTIONS, *PPOLICY_AUDIT_EVENT_OPTIONS;

// where the following flags can be set:
//
//     POLICY_AUDIT_EVENT_UNCHANGED - Leave existing auditing options
//         unchanged for events of this type.  This flag is only used for
//         set operations.  If this flag is set, then all other flags
//         are ignored.
//
//     POLICY_AUDIT_EVENT_NONE - Cancel all auditing options for events
//         of this type.  If this flag is set, the success/failure flags
//         are ignored.
//
//     POLICY_AUDIT_EVENT_SUCCESS - When auditing is enabled, audit all
//         successful occurrences of events of the given type.
//
//     POLICY_AUDIT_EVENT_FAILURE - When auditing is enabled, audit all
//         unsuccessful occurrences of events of the given type.
//




//
// The following data type defines the classes of Policy Information
// that may be queried/set.
//

typedef enum _POLICY_INFORMATION_CLASS {

    PolicyAuditLogInformation = 1,
    PolicyAuditEventsInformation,
    PolicyPrimaryDomainInformation,
    PolicyPdAccountInformation,
    PolicyAccountDomainInformation,
    PolicyLsaServerRoleInformation,
    PolicyReplicaSourceInformation,
    PolicyDefaultQuotaInformation,
    PolicyModificationInformation,
    PolicyAuditFullSetInformation,
    PolicyAuditFullQueryInformation,
    PolicyDnsDomainInformation,
    PolicyDnsDomainInformationInt,
    PolicyLocalAccountDomainInformation,
    PolicyMachineAccountInformation,
    PolicyMachineAccountInformation2,
    PolicyLastEntry

} POLICY_INFORMATION_CLASS, *PPOLICY_INFORMATION_CLASS;


//
// The following data type corresponds to the PolicyAuditLogInformation
// information class.  It is used to represent information relating to
// the Audit Log.
//
// This structure may be used in both query and set operations.  However,
// when used in set operations, some fields are ignored.
//

typedef struct _POLICY_AUDIT_LOG_INFO {

    ULONG AuditLogPercentFull;
    ULONG MaximumLogSize;
    LARGE_INTEGER AuditRetentionPeriod;
    BOOLEAN AuditLogFullShutdownInProgress;
    LARGE_INTEGER TimeToShutdown;
    ULONG NextAuditRecordId;

} POLICY_AUDIT_LOG_INFO, *PPOLICY_AUDIT_LOG_INFO;

// where the members have the following usage:
//
//     AuditLogPercentFull - Indicates the percentage of the Audit Log
//         currently being used.
//
//     MaximumLogSize - Specifies the maximum size of the Audit Log in
//         kilobytes.
//
//     AuditRetentionPeriod - Indicates the length of time that Audit
//         Records are to be retained.  Audit Records are discardable
//         if their timestamp predates the current time minus the
//         retention period.
//
//     AuditLogFullShutdownInProgress - Indicates whether or not a system
//         shutdown is being initiated due to the security Audit Log becoming
//         full.  This condition will only occur if the system is configured
//         to shutdown when the log becomes full.
//
//         TRUE indicates that a shutdown is in progress
//         FALSE indicates that a shutdown is not in progress.
//
//         Once a shutdown has been initiated, this flag will be set to
//         TRUE.  If an administrator is able to currect the situation
//         before the shutdown becomes irreversible, then this flag will
//         be reset to false.
//
//         This field is ignored for set operations.
//
//     TimeToShutdown - If the AuditLogFullShutdownInProgress flag is set,
//         then this field contains the time left before the shutdown
//         becomes irreversible.
//
//         This field is ignored for set operations.
//


//
// The following data type corresponds to the PolicyAuditEventsInformation
// information class.  It is used to represent information relating to
// the audit requirements.
//

typedef struct _POLICY_AUDIT_EVENTS_INFO {

    BOOLEAN AuditingMode;
    PPOLICY_AUDIT_EVENT_OPTIONS EventAuditingOptions;
    ULONG MaximumAuditEventCount;

} POLICY_AUDIT_EVENTS_INFO, *PPOLICY_AUDIT_EVENTS_INFO;

// where the members have the following usage:
//
//     AuditingMode - A Boolean variable specifying the Auditing Mode value.
//         This value is interpreted as follows:
//
//         TRUE - Auditing is to be enabled (set operations) or is enabled
//             (query operations).  Audit Records will be generated according
//             to the Event Auditing Options in effect (see the
//             EventAuditingOptions field.
//
//         FALSE - Auditing is to be disabled (set operations) or is
//             disabled (query operations).  No Audit Records will be
//             generated.  Note that for set operations the Event Auditing
//             Options in effect will still be updated as specified by the
//             EventAuditingOptions field whether Auditing is enabled or
//             disabled.
//
//    EventAuditingOptions - Pointer to an array of Auditing Options
//        indexed by Audit Event Type.
//
//    MaximumAuditEventCount - Specifiesa count of the number of Audit
//        Event Types specified by the EventAuditingOptions parameter.  If
//        this count is less than the number of Audit Event Types supported
//        by the system, the Auditing Options for Event Types with IDs
//        higher than (MaximumAuditEventCount + 1) are left unchanged.
//


//
// The following data type is used to represent information relating to
// the audit requirements.
//

typedef struct _POLICY_AUDIT_SUBCATEGORIES_INFO {

    ULONG MaximumSubCategoryCount;
    PPOLICY_AUDIT_EVENT_OPTIONS EventAuditingOptions;

} POLICY_AUDIT_SUBCATEGORIES_INFO, *PPOLICY_AUDIT_SUBCATEGORIES_INFO;

typedef struct _POLICY_AUDIT_CATEGORIES_INFO {

    ULONG MaximumCategoryCount;
    PPOLICY_AUDIT_SUBCATEGORIES_INFO SubCategoriesInfo;

} POLICY_AUDIT_CATEGORIES_INFO, *PPOLICY_AUDIT_CATEGORIES_INFO;


//
// Valid bits for Per user policy mask.
//

#define PER_USER_POLICY_UNCHANGED               (0x00)
#define PER_USER_AUDIT_SUCCESS_INCLUDE          (0x01)
#define PER_USER_AUDIT_SUCCESS_EXCLUDE          (0x02)
#define PER_USER_AUDIT_FAILURE_INCLUDE          (0x04)
#define PER_USER_AUDIT_FAILURE_EXCLUDE          (0x08)
#define PER_USER_AUDIT_NONE                     (0x10)


#define VALID_PER_USER_AUDIT_POLICY_FLAG (PER_USER_AUDIT_SUCCESS_INCLUDE | \
                                          PER_USER_AUDIT_SUCCESS_EXCLUDE | \
                                          PER_USER_AUDIT_FAILURE_INCLUDE | \
                                          PER_USER_AUDIT_FAILURE_EXCLUDE | \
                                          PER_USER_AUDIT_NONE)


//
// The following structure corresponds to the PolicyPrimaryDomainInformation
// information class.
//

typedef struct _POLICY_PRIMARY_DOMAIN_INFO {

    LSA_UNICODE_STRING Name;
    PSID Sid;

} POLICY_PRIMARY_DOMAIN_INFO, *PPOLICY_PRIMARY_DOMAIN_INFO;

// where the members have the following usage:
//
//     Name - Is the name of the domain
//
//     Sid - Is the Sid of the domain
//


//
// The following structure corresponds to the PolicyPdAccountInformation
// information class.  This structure may be used in Query operations
// only.
//

typedef struct _POLICY_PD_ACCOUNT_INFO {

    LSA_UNICODE_STRING Name;

} POLICY_PD_ACCOUNT_INFO, *PPOLICY_PD_ACCOUNT_INFO;

// where the members have the following usage:
//
//     Name - Is the name of an account in the domain that should be used
//         for authentication and name/ID lookup requests.
//


//
// The following structure corresponds to the PolicyLsaServerRoleInformation
// information class.
//

typedef struct _POLICY_LSA_SERVER_ROLE_INFO {

    POLICY_LSA_SERVER_ROLE LsaServerRole;

} POLICY_LSA_SERVER_ROLE_INFO, *PPOLICY_LSA_SERVER_ROLE_INFO;

// where the fields have the following usage:
//
// TBS
//


//
// The following structure corresponds to the PolicyReplicaSourceInformation
// information class.
//

typedef struct _POLICY_REPLICA_SOURCE_INFO {

    LSA_UNICODE_STRING ReplicaSource;
    LSA_UNICODE_STRING ReplicaAccountName;

} POLICY_REPLICA_SOURCE_INFO, *PPOLICY_REPLICA_SOURCE_INFO;


//
// The following structure corresponds to the PolicyDefaultQuotaInformation
// information class.
//

typedef struct _POLICY_DEFAULT_QUOTA_INFO {

    QUOTA_LIMITS QuotaLimits;

} POLICY_DEFAULT_QUOTA_INFO, *PPOLICY_DEFAULT_QUOTA_INFO;


//
// The following structure corresponds to the PolicyModificationInformation
// information class.
//

typedef struct _POLICY_MODIFICATION_INFO {

    LARGE_INTEGER ModifiedId;
    LARGE_INTEGER DatabaseCreationTime;

} POLICY_MODIFICATION_INFO, *PPOLICY_MODIFICATION_INFO;

// where the members have the following usage:
//
//     ModifiedId - Is a 64-bit unsigned integer that is incremented each
//         time anything in the LSA database is modified.  This value is
//         only modified on Primary Domain Controllers.
//
//     DatabaseCreationTime - Is the date/time that the LSA Database was
//         created.  On Backup Domain Controllers, this value is replicated
//         from the Primary Domain Controller.
//

//
// The following structure type corresponds to the PolicyAuditFullSetInformation
// Information Class.
//

typedef struct _POLICY_AUDIT_FULL_SET_INFO {

    BOOLEAN ShutDownOnFull;

} POLICY_AUDIT_FULL_SET_INFO, *PPOLICY_AUDIT_FULL_SET_INFO;

//
// The following structure type corresponds to the PolicyAuditFullQueryInformation
// Information Class.
//

typedef struct _POLICY_AUDIT_FULL_QUERY_INFO {

    BOOLEAN ShutDownOnFull;
    BOOLEAN LogIsFull;

} POLICY_AUDIT_FULL_QUERY_INFO, *PPOLICY_AUDIT_FULL_QUERY_INFO;

//
// The following data type defines the classes of Policy Information
// that may be queried/set that has domain wide effect.
//

typedef enum _POLICY_DOMAIN_INFORMATION_CLASS {

#if (_WIN32_WINNT <= 0x0500)
    PolicyDomainQualityOfServiceInformation = 1,
#endif
    PolicyDomainEfsInformation = 2,
    PolicyDomainKerberosTicketInformation

} POLICY_DOMAIN_INFORMATION_CLASS, *PPOLICY_DOMAIN_INFORMATION_CLASS;

#if (_WIN32_WINNT < 0x0502)
//
// QualityOfService information.  Corresponds to PolicyDomainQualityOfServiceInformation
//

#define POLICY_QOS_SCHANNEL_REQUIRED            0x00000001
#define POLICY_QOS_OUTBOUND_INTEGRITY           0x00000002
#define POLICY_QOS_OUTBOUND_CONFIDENTIALITY     0x00000004
#define POLICY_QOS_INBOUND_INTEGRITY            0x00000008
#define POLICY_QOS_INBOUND_CONFIDENTIALITY      0x00000010
#define POLICY_QOS_ALLOW_LOCAL_ROOT_CERT_STORE  0x00000020
#define POLICY_QOS_RAS_SERVER_ALLOWED           0x00000040
#define POLICY_QOS_DHCP_SERVER_ALLOWED          0x00000080

//
// Bits 0x00000100 through 0xFFFFFFFF are reserved for future use.
//
#endif

#if (_WIN32_WINNT == 0x0500)
typedef struct _POLICY_DOMAIN_QUALITY_OF_SERVICE_INFO {

    ULONG QualityOfService;

} POLICY_DOMAIN_QUALITY_OF_SERVICE_INFO, *PPOLICY_DOMAIN_QUALITY_OF_SERVICE_INFO;
//
// where the members have the following usage:
//
//  QualityOfService - Determines what specific QOS actions a machine should take
//
#endif

//
// The following structure corresponds to the PolicyEfsInformation
// information class
//

typedef struct _POLICY_DOMAIN_EFS_INFO {

    ULONG   InfoLength;
    PUCHAR  EfsBlob;

} POLICY_DOMAIN_EFS_INFO, *PPOLICY_DOMAIN_EFS_INFO;

//
// where the members have the following usage:
//
//      InfoLength - Length of the EFS Information blob
//
//      EfsBlob - Efs blob data
//


//
// The following structure corresponds to the PolicyDomainKerberosTicketInformation
// information class
//

#define POLICY_KERBEROS_VALIDATE_CLIENT 0x00000080


typedef struct _POLICY_DOMAIN_KERBEROS_TICKET_INFO {

    ULONG AuthenticationOptions;
    LARGE_INTEGER MaxServiceTicketAge;
    LARGE_INTEGER MaxTicketAge;
    LARGE_INTEGER MaxRenewAge;
    LARGE_INTEGER MaxClockSkew;
    LARGE_INTEGER Reserved;
} POLICY_DOMAIN_KERBEROS_TICKET_INFO, *PPOLICY_DOMAIN_KERBEROS_TICKET_INFO;

//
// where the members have the following usage
//
//      AuthenticationOptions -- allowed ticket options (POLICY_KERBEROS_* flags )
//
//      MaxServiceTicketAge   -- Maximum lifetime for a service ticket
//
//      MaxTicketAge -- Maximum lifetime for the initial ticket
//
//      MaxRenewAge -- Maximum cumulative age a renewable ticket can be with
//                     requring authentication
//
//      MaxClockSkew -- Maximum tolerance for synchronization of computer clocks
//
//      Reserved   --  Reserved

//
// The following structure corresponds to the PolicyMachineAccountInformation
// information class.  Only valid when the machine is joined to an AD domain.
// When not joined, will return 0+NULL.
//
// Note, DN is not cached because it may change (if\when the account is
// moved within the domain tree).
//
typedef struct _POLICY_MACHINE_ACCT_INFO {

    ULONG Rid;
    PSID Sid;

} POLICY_MACHINE_ACCT_INFO, *PPOLICY_MACHINE_ACCT_INFO;

//
// The following structure corresponds to the PolicyMachineAccountInformation2
// information class.  Only valid when the machine is joined to an AD domain.
// When not joined, will return 0+NULL+GUID_NULL.
//
typedef struct _POLICY_MACHINE_ACCT_INFO2 {

    ULONG Rid;
    PSID Sid;
    GUID ObjectGuid;

} POLICY_MACHINE_ACCT_INFO2, *PPOLICY_MACHINE_ACCT_INFO2;

//
// The following data type defines the classes of Policy Information / Policy Domain Information
// that may be used to request notification
//

typedef enum _POLICY_NOTIFICATION_INFORMATION_CLASS {

    PolicyNotifyAuditEventsInformation = 1,
    PolicyNotifyAccountDomainInformation,
    PolicyNotifyServerRoleInformation,
    PolicyNotifyDnsDomainInformation,
    PolicyNotifyDomainEfsInformation,
    PolicyNotifyDomainKerberosTicketInformation,
    PolicyNotifyMachineAccountPasswordInformation,
    PolicyNotifyGlobalSaclInformation,
    PolicyNotifyMax // must always be the last entry

} POLICY_NOTIFICATION_INFORMATION_CLASS, *PPOLICY_NOTIFICATION_INFORMATION_CLASS;


//
// LSA RPC Context Handle (Opaque form).  Note that a Context Handle is
// always a pointer type unlike regular handles.
//

typedef PVOID LSA_HANDLE, *PLSA_HANDLE;


//
// Trusted Domain Object specific data types
//

//
// Various buffer sizes for LSAD wire encryption of Auth Infos
//
#define LSAD_AES_CRYPT_SHA512_HASH_SIZE     64
#define LSAD_AES_KEY_SIZE                   16
#define LSAD_AES_SALT_SIZE                  16
#define LSAD_AES_BLOCK_SIZE                 16

//
// This data type defines the following information classes that may be
// queried or set.
//

typedef enum _TRUSTED_INFORMATION_CLASS {

    TrustedDomainNameInformation = 1,
    TrustedControllersInformation,
    TrustedPosixOffsetInformation,
    TrustedPasswordInformation,
    TrustedDomainInformationBasic,
    TrustedDomainInformationEx,
    TrustedDomainAuthInformation,
    TrustedDomainFullInformation,
    TrustedDomainAuthInformationInternal,
    TrustedDomainFullInformationInternal,
    TrustedDomainInformationEx2Internal,
    TrustedDomainFullInformation2Internal,
    TrustedDomainSupportedEncryptionTypes,
    TrustedDomainAuthInformationInternalAes,
    TrustedDomainFullInformationInternalAes,
} TRUSTED_INFORMATION_CLASS, *PTRUSTED_INFORMATION_CLASS;

//
// The following data type corresponds to the TrustedDomainNameInformation
// information class.
//

typedef struct _TRUSTED_DOMAIN_NAME_INFO {

    LSA_UNICODE_STRING Name;

} TRUSTED_DOMAIN_NAME_INFO, *PTRUSTED_DOMAIN_NAME_INFO;

// where members have the following meaning:
//
// Name - The name of the Trusted Domain.
//

//
// The following data type corresponds to the TrustedControllersInformation
// information class.
//

typedef struct _TRUSTED_CONTROLLERS_INFO {

    ULONG Entries;
    PLSA_UNICODE_STRING Names;

} TRUSTED_CONTROLLERS_INFO, *PTRUSTED_CONTROLLERS_INFO;

// where members have the following meaning:
//
// Entries - Indicate how mamy entries there are in the Names array.
//
// Names - Pointer to an array of LSA_UNICODE_STRING structures containing the
//     names of domain controllers of the domain.  This information may not
//     be accurate and should be used only as a hint.  The order of this
//     list is considered significant and will be maintained.
//
//     By convention, the first name in this list is assumed to be the
//     Primary Domain Controller of the domain.  If the Primary Domain
//     Controller is not known, the first name should be set to the NULL
//     string.
//


//
// The following data type corresponds to the TrustedPosixOffsetInformation
// information class.
//

typedef struct _TRUSTED_POSIX_OFFSET_INFO {

    ULONG Offset;

} TRUSTED_POSIX_OFFSET_INFO, *PTRUSTED_POSIX_OFFSET_INFO;

// where members have the following meaning:
//
// Offset - Is an offset to use for the generation of Posix user and group
//     IDs from SIDs.  The Posix ID corresponding to any particular SID is
//     generated by adding the RID of that SID to the Offset of the SID's
//     corresponding TrustedDomain object.
//

//
// The following data type corresponds to the TrustedPasswordInformation
// information class.
//

typedef struct _TRUSTED_PASSWORD_INFO {
    LSA_UNICODE_STRING Password;
    LSA_UNICODE_STRING OldPassword;
} TRUSTED_PASSWORD_INFO, *PTRUSTED_PASSWORD_INFO;


typedef  LSA_TRUST_INFORMATION TRUSTED_DOMAIN_INFORMATION_BASIC;

typedef PLSA_TRUST_INFORMATION PTRUSTED_DOMAIN_INFORMATION_BASIC;

//
// Direction of the trust
//
#define TRUST_DIRECTION_DISABLED        0x00000000
#define TRUST_DIRECTION_INBOUND         0x00000001
#define TRUST_DIRECTION_OUTBOUND        0x00000002
#define TRUST_DIRECTION_BIDIRECTIONAL   (TRUST_DIRECTION_INBOUND | TRUST_DIRECTION_OUTBOUND)

#define TRUST_TYPE_DOWNLEVEL            0x00000001  // NT4 and before
#define TRUST_TYPE_UPLEVEL              0x00000002  // NT5
#define TRUST_TYPE_MIT                  0x00000003  // Trust with a MIT Kerberos realm

#if (_WIN32_WINNT < 0x0502)
#define TRUST_TYPE_DCE                  0x00000004  // Trust with a DCE realm
#endif

#define TRUST_TYPE_AAD                  0x00000005 // Trust with Azure AD

// Levels 0x6 - 0x000FFFFF reserved for future use
// Provider specific trust levels are from 0x00100000 to 0xFFF00000

#define TRUST_ATTRIBUTE_NON_TRANSITIVE                0x00000001  // Disallow transitivity
#define TRUST_ATTRIBUTE_UPLEVEL_ONLY                  0x00000002  // Trust link only valid for uplevel client
#if (_WIN32_WINNT == 0x0500)
#define TRUST_ATTRIBUTE_TREE_PARENT     0x00400000  // Denotes that we are setting the trust
                                                    // to our parent in the org tree...
#define TRUST_ATTRIBUTE_TREE_ROOT       0x00800000  // Denotes that we are setting the trust
                                                    // to another tree root in a forest...
// Trust attributes 0x00000004 through 0x004FFFFF reserved for future use
// Trust attributes 0x00F00000 through 0x00400000 are reserved for internal use
// Trust attributes 0x01000000 through 0xFF000000 are reserved for user
// defined values
#define TRUST_ATTRIBUTES_VALID  0xFF02FFFF
#endif

#if (_WIN32_WINNT < 0x0502)
#define TRUST_ATTRIBUTE_FILTER_SIDS        0x00000004  // Used to quarantine domains
#else
#define TRUST_ATTRIBUTE_QUARANTINED_DOMAIN            0x00000004  // Used to quarantine domains
#endif

#if (_WIN32_WINNT >= 0x0501)
#define TRUST_ATTRIBUTE_FOREST_TRANSITIVE             0x00000008  // This link may contain forest trust information
#if (_WIN32_WINNT >= 0x0502)
#define TRUST_ATTRIBUTE_CROSS_ORGANIZATION            0x00000010  // This trust is to a domain/forest which is not part of this enterprise
#define TRUST_ATTRIBUTE_WITHIN_FOREST                 0x00000020  // Trust is internal to this forest
#define TRUST_ATTRIBUTE_TREAT_AS_EXTERNAL             0x00000040  // Trust is to be treated as external for trust boundary purposes
#if (_WIN32_WINNT >= 0x0600)
#define TRUST_ATTRIBUTE_TRUST_USES_RC4_ENCRYPTION     0x00000080  // MIT trust with RC4
#define TRUST_ATTRIBUTE_TRUST_USES_AES_KEYS           0x00000100  // Use AES keys to encrypte KRB TGTs
#endif
#if (_WIN32_WINNT >= 0x0602)
#define TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION 0x00000200  // do not forward TGT to the other side of the trust which is not part of this enterprise
#define TRUST_ATTRIBUTE_PIM_TRUST                     0x00000400  // Outgoing trust to a PIM forest.
#endif

#if (_WIN32_WINNT >= 0x0603)
// Forward the TGT to the other side of the trust which is not part of this enterprise
// This flag has the opposite meaning of TRUST_ATTRIBUTE_CROSS_ORGANIZATION_NO_TGT_DELEGATION which is now deprecated.
// Note: setting TRUST_ATTRIBUTE_CROSS_ORGANIZATION_ENABLE_TGT_DELEGATION is not recommended from a security standpoint.
#define TRUST_ATTRIBUTE_CROSS_ORGANIZATION_ENABLE_TGT_DELEGATION 0x00000800
#endif

// Disables authentication target validation for all NTLM pass-through authentication
// requests using this trust.
#define TRUST_ATTRIBUTE_DISABLE_AUTH_TARGET_VALIDATION 0x00001000

// Trust attributes 0x00000040 through 0x00200000 are reserved for future use
#else
// Trust attributes 0x00000010 through 0x00200000 are reserved for future use
#endif
// Trust attributes 0x00400000 through 0x00800000 were used previously (up to W2K) and should not be re-used
// Trust attributes 0x01000000 through 0x80000000 are reserved for user
#define TRUST_ATTRIBUTES_VALID          0xFF03FFFF
#endif
#define TRUST_ATTRIBUTES_USER           0xFF000000

typedef struct _TRUSTED_DOMAIN_INFORMATION_EX {

    LSA_UNICODE_STRING Name;
    LSA_UNICODE_STRING FlatName;
    PSID  Sid;
    ULONG TrustDirection;
    ULONG TrustType;
    ULONG TrustAttributes;

} TRUSTED_DOMAIN_INFORMATION_EX, *PTRUSTED_DOMAIN_INFORMATION_EX;

typedef struct _TRUSTED_DOMAIN_INFORMATION_EX2 {

    LSA_UNICODE_STRING Name;
    LSA_UNICODE_STRING FlatName;
    PSID  Sid;
    ULONG TrustDirection;
    ULONG TrustType;
    ULONG TrustAttributes;
    ULONG ForestTrustLength;
#ifdef MIDL_PASS
    [size_is( ForestTrustLength )]
#endif
    PUCHAR ForestTrustInfo;

} TRUSTED_DOMAIN_INFORMATION_EX2, *PTRUSTED_DOMAIN_INFORMATION_EX2;

//
// Type of authentication information
//
#define TRUST_AUTH_TYPE_NONE    0   // Ignore this entry
#define TRUST_AUTH_TYPE_NT4OWF  1   // NT4 OWF password
#define TRUST_AUTH_TYPE_CLEAR   2   // Cleartext password
#define TRUST_AUTH_TYPE_VERSION 3   // Cleartext password version number

typedef struct _LSA_AUTH_INFORMATION {

    LARGE_INTEGER LastUpdateTime;
    ULONG AuthType;
    ULONG AuthInfoLength;
    PUCHAR AuthInfo;
} LSA_AUTH_INFORMATION, *PLSA_AUTH_INFORMATION;

typedef struct _TRUSTED_DOMAIN_AUTH_INFORMATION {

    ULONG IncomingAuthInfos;
    PLSA_AUTH_INFORMATION   IncomingAuthenticationInformation;
    PLSA_AUTH_INFORMATION   IncomingPreviousAuthenticationInformation;
    ULONG OutgoingAuthInfos;
    PLSA_AUTH_INFORMATION   OutgoingAuthenticationInformation;
    PLSA_AUTH_INFORMATION   OutgoingPreviousAuthenticationInformation;

} TRUSTED_DOMAIN_AUTH_INFORMATION, *PTRUSTED_DOMAIN_AUTH_INFORMATION;

typedef struct _TRUSTED_DOMAIN_FULL_INFORMATION {

    TRUSTED_DOMAIN_INFORMATION_EX   Information;
    TRUSTED_POSIX_OFFSET_INFO       PosixOffset;
    TRUSTED_DOMAIN_AUTH_INFORMATION AuthInformation;

} TRUSTED_DOMAIN_FULL_INFORMATION, *PTRUSTED_DOMAIN_FULL_INFORMATION;

typedef struct _TRUSTED_DOMAIN_FULL_INFORMATION2 {

    TRUSTED_DOMAIN_INFORMATION_EX2  Information;
    TRUSTED_POSIX_OFFSET_INFO       PosixOffset;
    TRUSTED_DOMAIN_AUTH_INFORMATION AuthInformation;

} TRUSTED_DOMAIN_FULL_INFORMATION2, *PTRUSTED_DOMAIN_FULL_INFORMATION2;

typedef struct _TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES {

    ULONG      SupportedEncryptionTypes;

} TRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES, *PTRUSTED_DOMAIN_SUPPORTED_ENCRYPTION_TYPES;

typedef enum {

    ForestTrustTopLevelName,
    ForestTrustTopLevelNameEx,
    ForestTrustDomainInfo,
    ForestTrustBinaryInfo,
    ForestTrustScannerInfo,
    ForestTrustRecordTypeLast = ForestTrustScannerInfo

} LSA_FOREST_TRUST_RECORD_TYPE;

#if (_WIN32_WINNT < 0x0502)
#define LSA_FOREST_TRUST_RECORD_TYPE_UNRECOGNIZED 0x80000000
#endif

//
// Bottom 16 bits of the flags are reserved for disablement reasons
//

#define LSA_FTRECORD_DISABLED_REASONS            ( 0x0000FFFFL )

//
// Reasons for a top-level name forest trust record to be disabled
//

#define LSA_TLN_DISABLED_NEW                     ( 0x00000001L )
#define LSA_TLN_DISABLED_ADMIN                   ( 0x00000002L )
#define LSA_TLN_DISABLED_CONFLICT                ( 0x00000004L )

//
// Reasons for a domain information forest trust record to be disabled
//

#define LSA_SID_DISABLED_ADMIN                   ( 0x00000001L )
#define LSA_SID_DISABLED_CONFLICT                ( 0x00000002L )
#define LSA_NB_DISABLED_ADMIN                    ( 0x00000004L )
#define LSA_NB_DISABLED_CONFLICT                 ( 0x00000008L )

//
// FLag definitions for the LSA_FOREST_TRUST_SCANNER_INFO record
//

#define LSA_SCANNER_INFO_DISABLE_AUTH_TARGET_VALIDATION  ( 0x00000001L )
#define LSA_SCANNER_INFO_ADMIN_ALL_FLAGS         (LSA_SCANNER_INFO_DISABLE_AUTH_TARGET_VALIDATION)

typedef struct _LSA_FOREST_TRUST_DOMAIN_INFO {

#ifdef MIDL_PASS
    PISID Sid;
#else
    PSID Sid;
#endif
    LSA_UNICODE_STRING DnsName;
    LSA_UNICODE_STRING NetbiosName;

} LSA_FOREST_TRUST_DOMAIN_INFO, *PLSA_FOREST_TRUST_DOMAIN_INFO;

// LSA_FOREST_TRUST_SCANNER_INFO is usually written from
// the trust scanner logic that runs internally on the PDC FSMO
// in the root domain of the forest.
typedef struct _LSA_FOREST_TRUST_SCANNER_INFO {

#ifdef MIDL_PASS
    [unique] PISID DomainSid;
#else
    PSID DomainSid;
#endif
    LSA_UNICODE_STRING DnsName;
    LSA_UNICODE_STRING NetbiosName;

} LSA_FOREST_TRUST_SCANNER_INFO, *PLSA_FOREST_TRUST_SCANNER_INFO;

#if (_WIN32_WINNT >= 0x0502)
//
//  To prevent huge data to be passed in, we should put a limit on LSA_FOREST_TRUST_BINARY_DATA.
//      128K is large enough that can't be reached in the near future, and small enough not to
//      cause memory problems.

#define MAX_FOREST_TRUST_BINARY_DATA_SIZE ( 128 * 1024 )
#endif

typedef struct _LSA_FOREST_TRUST_BINARY_DATA {

#ifdef MIDL_PASS
    [range(0, MAX_FOREST_TRUST_BINARY_DATA_SIZE)] ULONG Length;
    [size_is( Length )] PUCHAR Buffer;
#else
    ULONG Length;
    PUCHAR Buffer;
#endif

} LSA_FOREST_TRUST_BINARY_DATA, *PLSA_FOREST_TRUST_BINARY_DATA;

typedef struct _LSA_FOREST_TRUST_RECORD {

    ULONG Flags;
    LSA_FOREST_TRUST_RECORD_TYPE ForestTrustType; // type of record
    LARGE_INTEGER Time;

#ifdef MIDL_PASS
    [switch_type( LSA_FOREST_TRUST_RECORD_TYPE ), switch_is( ForestTrustType )]
#endif

    union {                                       // actual data

#ifdef MIDL_PASS
        [case( ForestTrustTopLevelName,
               ForestTrustTopLevelNameEx )] LSA_UNICODE_STRING TopLevelName;
        [case( ForestTrustDomainInfo )] LSA_FOREST_TRUST_DOMAIN_INFO DomainInfo;
        [default] LSA_FOREST_TRUST_BINARY_DATA Data;

#else
        LSA_UNICODE_STRING TopLevelName;
        LSA_FOREST_TRUST_DOMAIN_INFO DomainInfo;
        LSA_FOREST_TRUST_BINARY_DATA Data;        // used for unrecognized types
#endif
    } ForestTrustData;

} LSA_FOREST_TRUST_RECORD, *PLSA_FOREST_TRUST_RECORD;

typedef struct _LSA_FOREST_TRUST_RECORD2 {

    ULONG Flags;
    LSA_FOREST_TRUST_RECORD_TYPE ForestTrustType; // type of record
    LARGE_INTEGER Time;

#ifdef MIDL_PASS
    [switch_type( LSA_FOREST_TRUST_RECORD_TYPE ), switch_is( ForestTrustType )]
#endif

    union {                                       // actual data

#ifdef MIDL_PASS
        [case( ForestTrustTopLevelName,
               ForestTrustTopLevelNameEx )] LSA_UNICODE_STRING TopLevelName;
        [case( ForestTrustDomainInfo )] LSA_FOREST_TRUST_DOMAIN_INFO DomainInfo;
        [case( ForestTrustBinaryInfo )] LSA_FOREST_TRUST_BINARY_DATA BinaryData;
        [case( ForestTrustScannerInfo )] LSA_FOREST_TRUST_SCANNER_INFO ScannerInfo;

#else
        LSA_UNICODE_STRING TopLevelName;
        LSA_FOREST_TRUST_DOMAIN_INFO DomainInfo;
        LSA_FOREST_TRUST_BINARY_DATA BinaryData;
        LSA_FOREST_TRUST_SCANNER_INFO ScannerInfo;
#endif
    } ForestTrustData;

} LSA_FOREST_TRUST_RECORD2, *PLSA_FOREST_TRUST_RECORD2;

#if (_WIN32_WINNT >= 0x0502)
//
// To prevent forest trust blobs of large size, number of records must be
// smaller than MAX_RECORDS_IN_FOREST_TRUST_INFO
//

#define MAX_RECORDS_IN_FOREST_TRUST_INFO 4000
#endif

typedef struct _LSA_FOREST_TRUST_INFORMATION {

#ifdef MIDL_PASS
    [range(0, MAX_RECORDS_IN_FOREST_TRUST_INFO)] ULONG RecordCount;
    [size_is( RecordCount )] PLSA_FOREST_TRUST_RECORD * Entries;
#else
    ULONG RecordCount;
    PLSA_FOREST_TRUST_RECORD * Entries;
#endif

} LSA_FOREST_TRUST_INFORMATION, *PLSA_FOREST_TRUST_INFORMATION;

typedef struct _LSA_FOREST_TRUST_INFORMATION2 {

#ifdef MIDL_PASS
    [range(0, MAX_RECORDS_IN_FOREST_TRUST_INFO)] ULONG RecordCount;
    [size_is( RecordCount )] PLSA_FOREST_TRUST_RECORD2 * Entries;
#else
    ULONG RecordCount;
    PLSA_FOREST_TRUST_RECORD2 * Entries;
#endif

} LSA_FOREST_TRUST_INFORMATION2, *PLSA_FOREST_TRUST_INFORMATION2;


typedef enum {

    CollisionTdo,
    CollisionXref,
    CollisionOther

} LSA_FOREST_TRUST_COLLISION_RECORD_TYPE;

typedef struct _LSA_FOREST_TRUST_COLLISION_RECORD {

    ULONG Index;
    LSA_FOREST_TRUST_COLLISION_RECORD_TYPE Type;
    ULONG Flags;
    LSA_UNICODE_STRING Name;

} LSA_FOREST_TRUST_COLLISION_RECORD, *PLSA_FOREST_TRUST_COLLISION_RECORD;

typedef struct _LSA_FOREST_TRUST_COLLISION_INFORMATION {

    ULONG RecordCount;
#ifdef MIDL_PASS
    [size_is( RecordCount )]
#endif
    PLSA_FOREST_TRUST_COLLISION_RECORD * Entries;

} LSA_FOREST_TRUST_COLLISION_INFORMATION, *PLSA_FOREST_TRUST_COLLISION_INFORMATION;


//
// LSA Enumeration Context
//

typedef ULONG LSA_ENUMERATION_HANDLE, *PLSA_ENUMERATION_HANDLE;

//
// LSA Enumeration Information
//

typedef struct _LSA_ENUMERATION_INFORMATION {

    PSID Sid;

} LSA_ENUMERATION_INFORMATION, *PLSA_ENUMERATION_INFORMATION;


////////////////////////////////////////////////////////////////////////////
//                                                                        //
// Local Security Policy - Miscellaneous API function prototypes          //
//                                                                        //
////////////////////////////////////////////////////////////////////////////


NTSTATUS
NTAPI
LsaFreeMemory(
    _In_opt_ PVOID Buffer
    );

NTSTATUS
NTAPI
LsaClose(
    _In_ LSA_HANDLE ObjectHandle
    );


#if (_WIN32_WINNT >= 0x0600)

typedef struct _LSA_LAST_INTER_LOGON_INFO {
    LARGE_INTEGER LastSuccessfulLogon;
    LARGE_INTEGER LastFailedLogon;
    ULONG FailedAttemptCountSinceLastSuccessfulLogon;
} LSA_LAST_INTER_LOGON_INFO, *PLSA_LAST_INTER_LOGON_INFO;

#endif

#if (_WIN32_WINNT >= 0x0501)
typedef struct _SECURITY_LOGON_SESSION_DATA {
    ULONG               Size;
    LUID                LogonId;
    LSA_UNICODE_STRING  UserName;
    LSA_UNICODE_STRING  LogonDomain;
    LSA_UNICODE_STRING  AuthenticationPackage;
    ULONG               LogonType;
    ULONG               Session;
    PSID                Sid;
    LARGE_INTEGER       LogonTime;

    //
    // new for whistler:
    //

    LSA_UNICODE_STRING  LogonServer;
    LSA_UNICODE_STRING  DnsDomainName;
    LSA_UNICODE_STRING  Upn;

#if (_WIN32_WINNT >= 0x0600)

    //
    // new for LH
    //

    ULONG UserFlags;

    LSA_LAST_INTER_LOGON_INFO LastLogonInfo;
    LSA_UNICODE_STRING LogonScript;
    LSA_UNICODE_STRING ProfilePath;
    LSA_UNICODE_STRING HomeDirectory;
    LSA_UNICODE_STRING HomeDirectoryDrive;

    LARGE_INTEGER LogoffTime;
    LARGE_INTEGER KickOffTime;
    LARGE_INTEGER PasswordLastSet;
    LARGE_INTEGER PasswordCanChange;
    LARGE_INTEGER PasswordMustChange;

#endif
} SECURITY_LOGON_SESSION_DATA, * PSECURITY_LOGON_SESSION_DATA;

NTSTATUS
NTAPI
LsaEnumerateLogonSessions(
    _Out_ PULONG  LogonSessionCount,
    _Out_ PLUID * LogonSessionList
    );

NTSTATUS
NTAPI
LsaGetLogonSessionData(
    _In_ PLUID    LogonId,
    _Out_ PSECURITY_LOGON_SESSION_DATA * ppLogonSessionData
    );

#endif
NTSTATUS
NTAPI
LsaOpenPolicy(
    _In_opt_ PLSA_UNICODE_STRING SystemName,
    _In_ PLSA_OBJECT_ATTRIBUTES ObjectAttributes,
    _In_ ACCESS_MASK DesiredAccess,
    _Out_ PLSA_HANDLE PolicyHandle
    );


//
// Keep the following in sync with LSA_MAXIMUM_PER_CAP_CAPE_COUNT
// TBD: Reduce these multiple macros to one
//
#define MAXIMUM_CAPES_PER_CAP 0x7F

//
// The 32-bit flag in CAP are CAPE are segmented as follows
//
//  +----------------+---------------+-----------------+--------------------+
//  | Reserved Flags | Control Flags | Staged Policy's | Effective Policy's |
//  |                |               |       Flags     |        Flags       |
//  +----------------+---------------+-----------------+--------------------+
//  |     8-bits     |     8-bits    |      8-bits     |       8-bits       |
//  +----------------+---------------+-----------------+--------------------+
//
// Only the LSB of Control Flags, Staged and Effective Policy Flags are used
// All other bits are reserved and MUST be zero
//

//
// Effective Policy's Flags
//
#define CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG         0x00000001

//
// Staging Policy's Flags
//
#define CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG  0x00000100
#define STAGING_FLAG(Effective) ((Effective & 0xF) << 8)

//
// Control Flags
//
#define CENTRAL_ACCESS_POLICY_STAGED_FLAG                       0x00010000

#define CENTRAL_ACCESS_POLICY_VALID_FLAG_MASK                           \
        (                                                               \
            CENTRAL_ACCESS_POLICY_OWNER_RIGHTS_PRESENT_FLAG         |   \
            CENTRAL_ACCESS_POLICY_STAGED_OWNER_RIGHTS_PRESENT_FLAG  |   \
            CENTRAL_ACCESS_POLICY_STAGED_FLAG                           \
        )

//
// LsaSetCAPs Flags
//
#define LSASETCAPS_RELOAD_FLAG                      0x00000001

#define LSASETCAPS_VALID_FLAG_MASK                          \
        (                                                   \
            LSASETCAPS_RELOAD_FLAG                         \
        )

//
// CAPE structure that specifies the access policy and when it is applicable
//
// WARNING !
// Keep the fields of the following struct in sync with LSAPR_CENTRAL_ACCESS_POLICY_ENTRY
// struct in lsarpc.idl
//
//
typedef struct _CENTRAL_ACCESS_POLICY_ENTRY {

    LSA_UNICODE_STRING      Name;           // Unique name as in AD
    LSA_UNICODE_STRING      Description;
    LSA_UNICODE_STRING      ChangeId;
    ULONG                   LengthAppliesTo;
    _Field_size_bytes_(LengthAppliesTo)
    PUCHAR                  AppliesTo;      // Must satisfy to enforce CAPE
    ULONG                   LengthSD;
    _Field_size_bytes_(LengthSD)
    PSECURITY_DESCRIPTOR    SD;             // Effective policy
    ULONG                   LengthStagedSD;
    _Field_size_bytes_(LengthStagedSD)
    PSECURITY_DESCRIPTOR    StagedSD;       // Staged policy
    ULONG                   Flags;          // Reserved
}
CENTRAL_ACCESS_POLICY_ENTRY, *PCENTRAL_ACCESS_POLICY_ENTRY;

typedef const CENTRAL_ACCESS_POLICY_ENTRY *PCCENTRAL_ACCESS_POLICY_ENTRY;

//
// CAP structure that is a collection of CAPEs. Resources that are subject to
// Central Access Policies are associated with a CAP by a CAP ID placed in the SACL
// of the Resource. Applicable CAPEs of a CAP are enforced during access check.
//
// WARNING !
// Keep the fields of the following struct in sync with LSAPR_CENTRAL_ACCESS_POLICY
// struct in lsarpc.idl
//
//

typedef struct _CENTRAL_ACCESS_POLICY {

    PSID                            CAPID;
    LSA_UNICODE_STRING              Name;
    LSA_UNICODE_STRING              Description;
    LSA_UNICODE_STRING              ChangeId;
    ULONG                           Flags;
    ULONG                           CAPECount;
    _Field_size_(CAPECount)
    PCENTRAL_ACCESS_POLICY_ENTRY   *CAPEs;
}
CENTRAL_ACCESS_POLICY, *PCENTRAL_ACCESS_POLICY;

typedef const CENTRAL_ACCESS_POLICY *PCCENTRAL_ACCESS_POLICY;

NTSTATUS
NTAPI
LsaSetCAPs(
    _In_reads_opt_(CAPDNCount) PLSA_UNICODE_STRING CAPDNs,
    _In_ ULONG CAPDNCount,
    _In_ ULONG Flags
    );

NTSTATUS
NTAPI
LsaGetAppliedCAPIDs(
    _In_opt_ PLSA_UNICODE_STRING SystemName,
    _Outptr_result_buffer_(*CAPIDCount) PSID **CAPIDs,
    _Out_ PULONG CAPIDCount
    );

NTSTATUS
NTAPI
LsaQueryCAPs(
    _In_reads_opt_(CAPIDCount) PSID *CAPIDs,
    _In_ ULONG CAPIDCount,
    _Outptr_result_buffer_(*CAPCount) PCENTRAL_ACCESS_POLICY *CAPs,
    _Out_ PULONG CAPCount
    );


NTSTATUS
NTAPI
LsaQueryInformationPolicy(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ POLICY_INFORMATION_CLASS InformationClass,
    _Out_ PVOID *Buffer
    );

NTSTATUS
NTAPI
LsaSetInformationPolicy(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ POLICY_INFORMATION_CLASS InformationClass,
    _In_ PVOID Buffer
    );

NTSTATUS
NTAPI
LsaQueryDomainInformationPolicy(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ POLICY_DOMAIN_INFORMATION_CLASS InformationClass,
    _Out_ PVOID *Buffer
    );

NTSTATUS
NTAPI
LsaSetDomainInformationPolicy(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ POLICY_DOMAIN_INFORMATION_CLASS InformationClass,
    _In_opt_ PVOID Buffer
    );


NTSTATUS
NTAPI
LsaRegisterPolicyChangeNotification(
    _In_ POLICY_NOTIFICATION_INFORMATION_CLASS InformationClass,
    _In_ HANDLE  NotificationEventHandle
    );

NTSTATUS
NTAPI
LsaUnregisterPolicyChangeNotification(
    _In_ POLICY_NOTIFICATION_INFORMATION_CLASS InformationClass,
    _In_ HANDLE  NotificationEventHandle
    );



NTSTATUS
NTAPI
LsaEnumerateTrustedDomains(
    _In_ LSA_HANDLE PolicyHandle,
    _Inout_ PLSA_ENUMERATION_HANDLE EnumerationContext,
    _Out_ PVOID *Buffer,
    _In_ ULONG PreferedMaximumLength,
    _Out_ PULONG CountReturned
    );


NTSTATUS
NTAPI
LsaLookupNames(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ ULONG Count,
    _In_ PLSA_UNICODE_STRING Names,
    _Out_ PLSA_REFERENCED_DOMAIN_LIST *ReferencedDomains,
    _Out_ PLSA_TRANSLATED_SID *Sids
    );

#if (_WIN32_WINNT >= 0x0501)
NTSTATUS
NTAPI
LsaLookupNames2(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ ULONG Flags, // Reserved
    _In_ ULONG Count,
    _In_ PLSA_UNICODE_STRING Names,
    _Out_ PLSA_REFERENCED_DOMAIN_LIST *ReferencedDomains,
    _Out_ PLSA_TRANSLATED_SID2 *Sids
    );
#endif

NTSTATUS
NTAPI
LsaLookupSids(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ ULONG Count,
    _In_ PSID *Sids,
    _Out_ PLSA_REFERENCED_DOMAIN_LIST *ReferencedDomains,
    _Out_ PLSA_TRANSLATED_NAME *Names
    );

NTSTATUS
NTAPI
LsaLookupSids2(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ ULONG LookupOptions,
    _In_ ULONG Count,
    _In_ PSID *Sids,
    _Out_ PLSA_REFERENCED_DOMAIN_LIST *ReferencedDomains,
    _Out_ PLSA_TRANSLATED_NAME *Names
    );



#define SE_INTERACTIVE_LOGON_NAME           TEXT("SeInteractiveLogonRight")
#define SE_NETWORK_LOGON_NAME               TEXT("SeNetworkLogonRight")
#define SE_BATCH_LOGON_NAME                 TEXT("SeBatchLogonRight")
#define SE_SERVICE_LOGON_NAME               TEXT("SeServiceLogonRight")
#define SE_DENY_INTERACTIVE_LOGON_NAME      TEXT("SeDenyInteractiveLogonRight")
#define SE_DENY_NETWORK_LOGON_NAME          TEXT("SeDenyNetworkLogonRight")
#define SE_DENY_BATCH_LOGON_NAME            TEXT("SeDenyBatchLogonRight")
#define SE_DENY_SERVICE_LOGON_NAME          TEXT("SeDenyServiceLogonRight")
#if (_WIN32_WINNT >= 0x0501)
#define SE_REMOTE_INTERACTIVE_LOGON_NAME    TEXT("SeRemoteInteractiveLogonRight")
#define SE_DENY_REMOTE_INTERACTIVE_LOGON_NAME TEXT("SeDenyRemoteInteractiveLogonRight")
#endif

//
// This new API returns all the accounts with a certain privilege
//

NTSTATUS
NTAPI
LsaEnumerateAccountsWithUserRight(
    _In_ LSA_HANDLE PolicyHandle,
    _In_opt_ PLSA_UNICODE_STRING UserRight,
    _Out_ PVOID *Buffer,
    _Out_ PULONG CountReturned
    );

//
// These new APIs differ by taking a SID instead of requiring the caller
// to open the account first and passing in an account handle
//

NTSTATUS
NTAPI
LsaEnumerateAccountRights(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PSID AccountSid,
    _Outptr_result_buffer_(*CountOfRights) PLSA_UNICODE_STRING *UserRights,
    _Out_ PULONG CountOfRights
    );

NTSTATUS
NTAPI
LsaAddAccountRights(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PSID AccountSid,
    _In_reads_(CountOfRights) PLSA_UNICODE_STRING UserRights,
    _In_ ULONG CountOfRights
    );

NTSTATUS
NTAPI
LsaRemoveAccountRights(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PSID AccountSid,
    _In_ BOOLEAN AllRights,
    _In_reads_opt_(CountOfRights) PLSA_UNICODE_STRING UserRights,
    _In_ ULONG CountOfRights
    );

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// Local Security Policy - Trusted Domain Object API function prototypes     //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

NTSTATUS
NTAPI
LsaOpenTrustedDomainByName(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _In_ ACCESS_MASK DesiredAccess,
    _Out_ PLSA_HANDLE TrustedDomainHandle
    );


NTSTATUS
NTAPI
LsaQueryTrustedDomainInfo(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PSID TrustedDomainSid,
    _In_ TRUSTED_INFORMATION_CLASS InformationClass,
    _Out_ PVOID *Buffer
    );

NTSTATUS
NTAPI
LsaSetTrustedDomainInformation(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PSID TrustedDomainSid,
    _In_ TRUSTED_INFORMATION_CLASS InformationClass,
    _In_ PVOID Buffer
    );

NTSTATUS
NTAPI
LsaDeleteTrustedDomain(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PSID TrustedDomainSid
    );

NTSTATUS
NTAPI
LsaQueryTrustedDomainInfoByName(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _In_ TRUSTED_INFORMATION_CLASS InformationClass,
    _Out_ PVOID *Buffer
    );

NTSTATUS
NTAPI
LsaSetTrustedDomainInfoByName(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _In_ TRUSTED_INFORMATION_CLASS InformationClass,
    _In_ PVOID Buffer
    );

NTSTATUS
NTAPI
LsaEnumerateTrustedDomainsEx(
    _In_ LSA_HANDLE PolicyHandle,
    _Inout_ PLSA_ENUMERATION_HANDLE EnumerationContext,
    _Out_ PVOID *Buffer,
    _In_ ULONG PreferedMaximumLength,
    _Out_ PULONG CountReturned
    );

NTSTATUS
NTAPI
LsaCreateTrustedDomainEx(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PTRUSTED_DOMAIN_INFORMATION_EX TrustedDomainInformation,
    _In_ PTRUSTED_DOMAIN_AUTH_INFORMATION AuthenticationInformation,
    _In_ ACCESS_MASK DesiredAccess,
    _Out_ PLSA_HANDLE TrustedDomainHandle
    );

#if (_WIN32_WINNT >= 0x0501)
NTSTATUS
NTAPI
LsaQueryForestTrustInformation(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _Out_ PLSA_FOREST_TRUST_INFORMATION * ForestTrustInfo
    );

NTSTATUS
NTAPI
LsaSetForestTrustInformation(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _In_ PLSA_FOREST_TRUST_INFORMATION ForestTrustInfo,
    _In_ BOOLEAN CheckOnly,
    _Out_ PLSA_FOREST_TRUST_COLLISION_INFORMATION * CollisionInfo
    );

// #define TESTING_MATCHING_ROUTINE

#ifdef TESTING_MATCHING_ROUTINE

NTSTATUS
NTAPI
LsaForestTrustFindMatch(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ ULONG Type,
    _In_ PLSA_UNICODE_STRING Name,
    _Out_ PLSA_UNICODE_STRING * Match
    );

#endif
#endif

//
// This API sets the workstation password (equivalent of setting/getting
// the SSI_SECRET_NAME secret)
//

NTSTATUS
NTAPI
LsaStorePrivateData(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING KeyName,
    _In_opt_ PLSA_UNICODE_STRING PrivateData
    );

NTSTATUS
NTAPI
LsaRetrievePrivateData(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING KeyName,
    _Out_ PLSA_UNICODE_STRING * PrivateData
    );


ULONG
NTAPI
LsaNtStatusToWinError(
    _In_ NTSTATUS Status
    );

NTSTATUS
NTAPI
LsaQueryForestTrustInformation2(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _In_ LSA_FOREST_TRUST_RECORD_TYPE HighestRecordType,
    _Out_ PLSA_FOREST_TRUST_INFORMATION2 * ForestTrustInfo
    );

NTSTATUS
NTAPI
LsaSetForestTrustInformation2(
    _In_ LSA_HANDLE PolicyHandle,
    _In_ PLSA_UNICODE_STRING TrustedDomainName,
    _In_ LSA_FOREST_TRUST_RECORD_TYPE HighestRecordType,
    _In_ PLSA_FOREST_TRUST_INFORMATION2 ForestTrustInfo,
    _In_ BOOLEAN CheckOnly,
    _Out_ PLSA_FOREST_TRUST_COLLISION_INFORMATION * CollisionInfo
    );


//
// Define a symbol so we can tell if ntifs.h has been included.
//

// begin_ntifs
#ifndef _NTLSA_IFS_
#define _NTLSA_IFS_
#endif
// end_ntifs


//
// SPNEGO package stuff
//

enum NEGOTIATE_MESSAGES {
    NegEnumPackagePrefixes = 0,
    NegGetCallerName = 1,
    NegTransferCredentials = 2,
    NegMsgReserved1 = 3,    
    NegCallPackageMax
};

#define NEGOTIATE_MAX_PREFIX    32

typedef struct _NEGOTIATE_PACKAGE_PREFIX {
    ULONG_PTR   PackageId;
    PVOID       PackageDataA;
    PVOID       PackageDataW;
    ULONG_PTR   PrefixLen;
    UCHAR       Prefix[ NEGOTIATE_MAX_PREFIX ];
} NEGOTIATE_PACKAGE_PREFIX, * PNEGOTIATE_PACKAGE_PREFIX;

typedef struct _NEGOTIATE_PACKAGE_PREFIXES {
    ULONG       MessageType;
    ULONG       PrefixCount;
    ULONG       Offset;        // Offset to array of _PREFIX above
#if(_WIN32_WINNT >= 0x0502)
    ULONG       Pad;           // Align structure for 64-bit
#endif
} NEGOTIATE_PACKAGE_PREFIXES, *PNEGOTIATE_PACKAGE_PREFIXES;

typedef struct _NEGOTIATE_CALLER_NAME_REQUEST {
    ULONG       MessageType;
    LUID        LogonId;
} NEGOTIATE_CALLER_NAME_REQUEST, *PNEGOTIATE_CALLER_NAME_REQUEST;

typedef struct _NEGOTIATE_CALLER_NAME_RESPONSE {
    ULONG       MessageType;
    PWSTR       CallerName;
} NEGOTIATE_CALLER_NAME_RESPONSE, * PNEGOTIATE_CALLER_NAME_RESPONSE;

#ifndef _NTDEF_
typedef LSA_UNICODE_STRING UNICODE_STRING, *PUNICODE_STRING;
typedef LSA_STRING STRING, *PSTRING ;
#endif

#ifndef _DOMAIN_PASSWORD_INFORMATION_DEFINED
#define _DOMAIN_PASSWORD_INFORMATION_DEFINED
typedef struct _DOMAIN_PASSWORD_INFORMATION {
    USHORT MinPasswordLength;
    USHORT PasswordHistoryLength;
    ULONG PasswordProperties;
#if defined(MIDL_PASS)
    OLD_LARGE_INTEGER MaxPasswordAge;
    OLD_LARGE_INTEGER MinPasswordAge;
#else
    LARGE_INTEGER MaxPasswordAge;
    LARGE_INTEGER MinPasswordAge;
#endif
} DOMAIN_PASSWORD_INFORMATION, *PDOMAIN_PASSWORD_INFORMATION;
#endif

#if (_WIN32_WINNT >= 0x0501)
//
// PasswordProperties flags
//

#define DOMAIN_PASSWORD_COMPLEX             0x00000001L
#define DOMAIN_PASSWORD_NO_ANON_CHANGE      0x00000002L
#define DOMAIN_PASSWORD_NO_CLEAR_CHANGE     0x00000004L
#define DOMAIN_LOCKOUT_ADMINS               0x00000008L
#define DOMAIN_PASSWORD_STORE_CLEARTEXT     0x00000010L
#define DOMAIN_REFUSE_PASSWORD_CHANGE       0x00000020L
#if(_WIN32_WINNT >= 0x0502)
#define DOMAIN_NO_LM_OWF_CHANGE             0x00000040L
#endif
#endif



#ifndef _PASSWORD_NOTIFICATION_DEFINED
#define _PASSWORD_NOTIFICATION_DEFINED
typedef NTSTATUS (*PSAM_PASSWORD_NOTIFICATION_ROUTINE) (
    PUNICODE_STRING UserName,
    ULONG RelativeId,
    PUNICODE_STRING NewPassword
);

#define SAM_PASSWORD_CHANGE_NOTIFY_ROUTINE  "PasswordChangeNotify"

typedef BOOLEAN (*PSAM_INIT_NOTIFICATION_ROUTINE) (
    VOID
    );

#define SAM_INIT_NOTIFICATION_ROUTINE  "InitializeChangeNotify"

#define SAM_PASSWORD_FILTER_ROUTINE  "PasswordFilter"

typedef BOOLEAN (*PSAM_PASSWORD_FILTER_ROUTINE) (
    _In_ PUNICODE_STRING  AccountName,
    _In_ PUNICODE_STRING  FullName,
    _In_ PUNICODE_STRING Password,
    _In_ BOOLEAN SetOperation
    );


#endif // _PASSWORD_NOTIFICATION_DEFINED


/////////////////////////////////////////////////////////////////////////
//                                                                     //
// Name of the MSV1_0 authentication package                           //
//                                                                     //
/////////////////////////////////////////////////////////////////////////

#define MSV1_0_PACKAGE_NAME     "MICROSOFT_AUTHENTICATION_PACKAGE_V1_0"
#define MSV1_0_PACKAGE_NAMEW    L"MICROSOFT_AUTHENTICATION_PACKAGE_V1_0"
#define MSV1_0_PACKAGE_NAMEW_LENGTH sizeof(MSV1_0_PACKAGE_NAMEW) - sizeof(WCHAR)

//
// Location of MSV authentication package data
//
#define MSV1_0_SUBAUTHENTICATION_KEY "SYSTEM\\CurrentControlSet\\Control\\Lsa\\MSV1_0"
#define MSV1_0_SUBAUTHENTICATION_VALUE "Auth"


/////////////////////////////////////////////////////////////////////////
//                                                                     //
// Widely used MSV1_0 data types                                       //
//                                                                     //
/////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//       LOGON      Related Data Structures
//
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

//
// When a LsaLogonUser() call is dispatched to the MsV1_0 authentication
// package, the beginning of the AuthenticationInformation buffer is
// cast to a MSV1_0_LOGON_SUBMIT_TYPE to determine the type of logon
// being requested.  Similarly, upon return, the type of profile buffer
// can be determined by typecasting it to a MSV_1_0_PROFILE_BUFFER_TYPE.
//

//
//  MSV1.0 LsaLogonUser() submission message types.
//

typedef enum _MSV1_0_LOGON_SUBMIT_TYPE {
    MsV1_0InteractiveLogon = 2,
    MsV1_0Lm20Logon,
    MsV1_0NetworkLogon,
    MsV1_0SubAuthLogon,
    MsV1_0WorkstationUnlockLogon = 7,
    // defined in Windows Server 2008 and up
    MsV1_0S4ULogon = 12,
    MsV1_0VirtualLogon = 82,
    // defined in Windows 8 and up
    MsV1_0NoElevationLogon = 83,
    // defined in Windows 8.1 and up
    MsV1_0LuidLogon = 84,
} MSV1_0_LOGON_SUBMIT_TYPE, *PMSV1_0_LOGON_SUBMIT_TYPE;


//
//  MSV1.0 LsaLogonUser() profile buffer types.
//

typedef enum _MSV1_0_PROFILE_BUFFER_TYPE {
    MsV1_0InteractiveProfile = 2,
    MsV1_0Lm20LogonProfile,
    MsV1_0SmartCardProfile
} MSV1_0_PROFILE_BUFFER_TYPE, *PMSV1_0_PROFILE_BUFFER_TYPE;

//
// MsV1_0InteractiveLogon
//
// The AuthenticationInformation buffer of an LsaLogonUser() call to
// perform an interactive logon contains the following data structure:
//

typedef struct _MSV1_0_INTERACTIVE_LOGON {
    MSV1_0_LOGON_SUBMIT_TYPE MessageType;
    UNICODE_STRING LogonDomainName;
    UNICODE_STRING UserName;
    UNICODE_STRING Password;
} MSV1_0_INTERACTIVE_LOGON, *PMSV1_0_INTERACTIVE_LOGON;

//
// Where:
//
//     MessageType - Contains the type of logon being requested.  This
//         field must be set to MsV1_0InteractiveLogon.
//
//     UserName - Is a string representing the user's account name.  The
//         name may be up to 255 characters long.  The name is treated case
//         insensitive.
//
//     Password - Is a string containing the user's cleartext password.
//         The password may be up to 255 characters long and contain any
//         UNICODE value.
//
//


//
// The ProfileBuffer returned upon a successful logon of this type
// contains the following data structure:
//

typedef struct _MSV1_0_INTERACTIVE_PROFILE {
    MSV1_0_PROFILE_BUFFER_TYPE MessageType;
    USHORT LogonCount;
    USHORT BadPasswordCount;
    LARGE_INTEGER LogonTime;
    LARGE_INTEGER LogoffTime;
    LARGE_INTEGER KickOffTime;
    LARGE_INTEGER PasswordLastSet;
    LARGE_INTEGER PasswordCanChange;
    LARGE_INTEGER PasswordMustChange;
    UNICODE_STRING LogonScript;
    UNICODE_STRING HomeDirectory;
    UNICODE_STRING FullName;
    UNICODE_STRING ProfilePath;
    UNICODE_STRING HomeDirectoryDrive;
    UNICODE_STRING LogonServer;
    ULONG UserFlags;
} MSV1_0_INTERACTIVE_PROFILE, *PMSV1_0_INTERACTIVE_PROFILE;

//
// where:
//
//     MessageType - Identifies the type of profile data being returned.
//         Contains the type of logon being requested.  This field must
//         be set to MsV1_0InteractiveProfile.
//
//     LogonCount - Number of times the user is currently logged on.
//
//     BadPasswordCount - Number of times a bad password was applied to
//         the account since last successful logon.
//
//     LogonTime - Time when user last logged on.  This is an absolute
//         format NT standard time value.
//
//     LogoffTime - Time when user should log off.  This is an absolute
//         format NT standard time value.
//
//     KickOffTime - Time when system should force user logoff.  This is
//         an absolute format NT standard time value.
//
//     PasswordLastChanged - Time and date the password was last
//         changed.  This is an absolute format NT standard time
//         value.
//
//     PasswordCanChange - Time and date when the user can change the
//         password.  This is an absolute format NT time value.  To
//         prevent a password from ever changing, set this field to a
//         date very far into the future.
//
//     PasswordMustChange - Time and date when the user must change the
//         password.  If the user can never change the password, this
//         field is undefined.  This is an absolute format NT time
//         value.
//
//     LogonScript - The (relative) path to the account's logon
//         script.
//
//     HomeDirectory - The home directory for the user.
//


//
// MsV1_0Lm20Logon and MsV1_0NetworkLogon
//
// The AuthenticationInformation buffer of an LsaLogonUser() call to
// perform an network logon contains the following data structure:
//
// MsV1_0NetworkLogon logon differs from MsV1_0Lm20Logon in that the
// ParameterControl field exists.
//

#define MSV1_0_CHALLENGE_LENGTH 8
#define MSV1_0_USER_SESSION_KEY_LENGTH 16
#define MSV1_0_LANMAN_SESSION_KEY_LENGTH 8

//
// Values for ParameterControl.
//

#define MSV1_0_CLEARTEXT_PASSWORD_ALLOWED    0x02
#define MSV1_0_UPDATE_LOGON_STATISTICS       0x04
#define MSV1_0_RETURN_USER_PARAMETERS        0x08
#define MSV1_0_DONT_TRY_GUEST_ACCOUNT        0x10
#define MSV1_0_ALLOW_SERVER_TRUST_ACCOUNT    0x20
#define MSV1_0_RETURN_PASSWORD_EXPIRY        0x40
// this next flag says that CaseInsensitiveChallengeResponse
//  (aka LmResponse) contains a client challenge in the first 8 bytes
#define MSV1_0_USE_CLIENT_CHALLENGE          0x80
#define MSV1_0_TRY_GUEST_ACCOUNT_ONLY        0x100
#define MSV1_0_RETURN_PROFILE_PATH           0x200
#define MSV1_0_TRY_SPECIFIED_DOMAIN_ONLY     0x400
#define MSV1_0_ALLOW_WORKSTATION_TRUST_ACCOUNT 0x800
//#if (_WIN32_WINNT >= 0x0501) -- Disabled until IIS fixes their target version.
#define MSV1_0_DISABLE_PERSONAL_FALLBACK     0x00001000
#define MSV1_0_ALLOW_FORCE_GUEST             0x00002000
//#endif
#if (_WIN32_WINNT >= 0x0502)
#define MSV1_0_CLEARTEXT_PASSWORD_SUPPLIED   0x00004000
// Start
// Doesnt exist in Windows XP but does exist in Windows 2000 Security Rollup and up
#define MSV1_0_USE_DOMAIN_FOR_ROUTING_ONLY   0x00008000
#endif
#define MSV1_0_SUBAUTHENTICATION_DLL_EX      0x00100000
// Defined in Windows Server 2003 SP1 and above
#define MSV1_0_ALLOW_MSVCHAPV2               0x00010000

#if (_WIN32_WINNT >= 0x0600)

//Defined in Windows Server 2008 and up
#define MSV1_0_S4U2SELF                      0x00020000 // no password is needed
#define MSV1_0_CHECK_LOGONHOURS_FOR_S4U      0x00040000 // check logon hours for S4U logon
#endif

#if (_WIN32_WINNT >= 0x0602)
// Logon for internet users. To be used by calls from auth package
// directly, not from netlogon.
#define MSV1_0_INTERNET_DOMAIN               0x00080000
#endif

//
// The high order byte is a value indicating the SubAuthentication DLL.
//  Zero indicates no SubAuthentication DLL.
//
#define MSV1_0_SUBAUTHENTICATION_DLL         0xFF000000
#define MSV1_0_SUBAUTHENTICATION_DLL_SHIFT   24
#define MSV1_0_MNS_LOGON                     0x01000000

//
// This is the list of subauthentication dlls used in MS
//

#define MSV1_0_SUBAUTHENTICATION_DLL_RAS     2
#define MSV1_0_SUBAUTHENTICATION_DLL_IIS     132

typedef struct _MSV1_0_LM20_LOGON {
    MSV1_0_LOGON_SUBMIT_TYPE MessageType;
    UNICODE_STRING LogonDomainName;
    UNICODE_STRING UserName;
    UNICODE_STRING Workstation;
    UCHAR ChallengeToClient[MSV1_0_CHALLENGE_LENGTH];
    STRING CaseSensitiveChallengeResponse;
    STRING CaseInsensitiveChallengeResponse;
    ULONG ParameterControl;
} MSV1_0_LM20_LOGON, * PMSV1_0_LM20_LOGON;

//
// NT 5.0 SubAuth dlls can use this struct
//

typedef struct _MSV1_0_SUBAUTH_LOGON{
    MSV1_0_LOGON_SUBMIT_TYPE MessageType;
    UNICODE_STRING LogonDomainName;
    UNICODE_STRING UserName;
    UNICODE_STRING Workstation;
    UCHAR ChallengeToClient[MSV1_0_CHALLENGE_LENGTH];
    STRING AuthenticationInfo1;
    STRING AuthenticationInfo2;
    ULONG ParameterControl;
    ULONG SubAuthPackageId;
} MSV1_0_SUBAUTH_LOGON, * PMSV1_0_SUBAUTH_LOGON;

#if (_WIN32_WINNT >= 0x0600)

//
// s4u2self logon
//
// Defined in Windows Server 2008 and above

//
// request to enforce logon hours policy
//

#define MSV1_0_S4U_LOGON_FLAG_CHECK_LOGONHOURS 0x2

typedef struct _MSV1_0_S4U_LOGON {
    MSV1_0_LOGON_SUBMIT_TYPE MessageType;
    ULONG Flags;
    UNICODE_STRING UserPrincipalName; // username or username@domain
    UNICODE_STRING DomainName; // Optional: if missing, using the local machine
} MSV1_0_S4U_LOGON, *PMSV1_0_S4U_LOGON;

#endif

//
// Values for UserFlags.
//

#define LOGON_GUEST                 0x01
#define LOGON_NOENCRYPTION          0x02
#define LOGON_CACHED_ACCOUNT        0x04
#define LOGON_USED_LM_PASSWORD      0x08
#define LOGON_EXTRA_SIDS            0x20
#define LOGON_SUBAUTH_SESSION_KEY   0x40
#define LOGON_SERVER_TRUST_ACCOUNT  0x80
#define LOGON_NTLMV2_ENABLED        0x100       // says DC understands NTLMv2
#define LOGON_RESOURCE_GROUPS       0x200
#define LOGON_PROFILE_PATH_RETURNED 0x400
// Defined in Windows Server 2008 and above
#define LOGON_NT_V2                 0x800   // NT response was used for validation
#define LOGON_LM_V2                 0x1000  // LM response was used for validation
#define LOGON_NTLM_V2               0x2000  // LM response was used to authenticate but NT response was used to derive the session key

#if (_WIN32_WINNT >= 0x0600)

#define LOGON_OPTIMIZED             0x4000  // this is an optimized logon
#define LOGON_WINLOGON              0x8000  // the logon session was created for winlogon
#define LOGON_PKINIT               0x10000  // Kerberos PKINIT extension was used to authenticate the user
#define LOGON_NO_OPTIMIZED         0x20000  // optimized logon has been disabled for this account

#endif

#if (_WIN32_WINNT >= 0x0602)

#define LOGON_NO_ELEVATION         0x40000  // Do not allow elevation for this logon
#define LOGON_MANAGED_SERVICE      0x80000  // Managed service account

#endif

//
// The high order byte is reserved for return by SubAuthentication DLLs.
//

#define MSV1_0_SUBAUTHENTICATION_FLAGS 0xFF000000

// Values returned by the MSV1_0_MNS_LOGON SubAuthentication DLL
#define LOGON_GRACE_LOGON              0x01000000

typedef struct _MSV1_0_LM20_LOGON_PROFILE {
    MSV1_0_PROFILE_BUFFER_TYPE MessageType;
    LARGE_INTEGER KickOffTime;
    LARGE_INTEGER LogoffTime;
    ULONG UserFlags;
    UCHAR UserSessionKey[MSV1_0_USER_SESSION_KEY_LENGTH];
    UNICODE_STRING LogonDomainName;
    UCHAR LanmanSessionKey[MSV1_0_LANMAN_SESSION_KEY_LENGTH];
    UNICODE_STRING LogonServer;
    UNICODE_STRING UserParameters;
} MSV1_0_LM20_LOGON_PROFILE, * PMSV1_0_LM20_LOGON_PROFILE;


//
// Supplemental credentials structure used for passing credentials into
// MSV1_0 from other packages
//

#define MSV1_0_OWF_PASSWORD_LENGTH 16
#define MSV1_0_SHA_PASSWORD_LENGTH 20
#define MSV1_0_CREDENTIAL_KEY_LENGTH 20

#define MSV1_0_CRED_LM_PRESENT      0x0001
#define MSV1_0_CRED_NT_PRESENT      0x0002
#define MSV1_0_CRED_REMOVED         0x0004
#define MSV1_0_CRED_CREDKEY_PRESENT 0x0008
#define MSV1_0_CRED_SHA_PRESENT     0x0010

#define MSV1_0_CRED_VERSION             0
#define MSV1_0_CRED_VERSION_V2          2
#define MSV1_0_CRED_VERSION_V3          4
#define MSV1_0_CRED_VERSION_IUM         0xffff0001
#define MSV1_0_CRED_VERSION_REMOTE      0xffff0002
#define MSV1_0_CRED_VERSION_ARSO        0xffff0003
#define MSV1_0_CRED_VERSION_RESERVED_1  0xfffffffe
#define MSV1_0_CRED_VERSION_INVALID     0xffffffff

typedef enum _MSV1_0_CREDENTIAL_KEY_TYPE
{
    InvalidCredKey,
    DeprecatedIUMCredKey,
    DomainUserCredKey,
    LocalUserCredKey,
    ExternallySuppliedCredKey
} MSV1_0_CREDENTIAL_KEY_TYPE;

typedef struct _MSV1_0_CREDENTIAL_KEY {
    UCHAR Data[MSV1_0_CREDENTIAL_KEY_LENGTH];
} MSV1_0_CREDENTIAL_KEY, *PMSV1_0_CREDENTIAL_KEY;

typedef struct _MSV1_0_SUPPLEMENTAL_CREDENTIAL {
    ULONG Version;
    ULONG Flags;
    UCHAR LmPassword[MSV1_0_OWF_PASSWORD_LENGTH];
    UCHAR NtPassword[MSV1_0_OWF_PASSWORD_LENGTH];
} MSV1_0_SUPPLEMENTAL_CREDENTIAL, *PMSV1_0_SUPPLEMENTAL_CREDENTIAL;

// V2 Drops support for LM hash, but adds the credential key
typedef struct _MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2 {
    ULONG Version;
    ULONG Flags;
    UCHAR NtPassword[MSV1_0_OWF_PASSWORD_LENGTH];
    MSV1_0_CREDENTIAL_KEY CredentialKey;
} MSV1_0_SUPPLEMENTAL_CREDENTIAL_V2, *PMSV1_0_SUPPLEMENTAL_CREDENTIAL_V2;

typedef struct _MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3 {
    ULONG Version;
    ULONG Flags;
    MSV1_0_CREDENTIAL_KEY_TYPE CredentialKeyType;
    UCHAR NtPassword[MSV1_0_OWF_PASSWORD_LENGTH];
    MSV1_0_CREDENTIAL_KEY CredentialKey;
    UCHAR ShaPassword[MSV1_0_SHA_PASSWORD_LENGTH];
} MSV1_0_SUPPLEMENTAL_CREDENTIAL_V3, *PMSV1_0_SUPPLEMENTAL_CREDENTIAL_V3;

typedef struct _MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL {
    ULONG Version;
    ULONG EncryptedCredsSize;
#ifdef MIDL_PASS
    [size_is(EncryptedCredsSize)]
    UCHAR EncryptedCreds[*];
#else
    _Field_size_(EncryptedCredsSize)
    UCHAR EncryptedCreds[ANYSIZE_ARRAY];
#endif
} MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL, *PMSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL;

#define MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL_SIZE(Creds) \
    (FIELD_OFFSET(MSV1_0_IUM_SUPPLEMENTAL_CREDENTIAL, EncryptedCreds) + (Creds)->EncryptedCredsSize)

// For remote creds, NTLM will request the NTOWF from Kerberos. However, NTLM
// does need to get the CredentialKey from the credssp client system.
#pragma pack(push,1)
typedef struct _MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL {
    ULONG Version;
    ULONG Flags;
    MSV1_0_CREDENTIAL_KEY CredentialKey;
    MSV1_0_CREDENTIAL_KEY_TYPE CredentialKeyType;
    ULONG EncryptedCredsSize;
    UCHAR EncryptedCreds[ANYSIZE_ARRAY];
} MSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL, *PMSV1_0_REMOTE_SUPPLEMENTAL_CREDENTIAL;
#pragma pack(pop)

//
// NTLM3 definitions.
//

#define MSV1_0_NTLM3_RESPONSE_LENGTH 16
#define MSV1_0_NTLM3_OWF_LENGTH 16

//
// this is the longest amount of time we'll allow challenge response
// pairs to be used. Note that this also has to allow for worst case clock skew
//
#if (_WIN32_WINNT == 0x0500)
#define MSV1_0_MAX_NTLM3_LIFE 1800     // 30 minutes (in seconds)
#else
#define MSV1_0_MAX_NTLM3_LIFE 129600     // 36 hours (in seconds)
#endif
#define MSV1_0_MAX_AVL_SIZE 64000

#if (_WIN32_WINNT >= 0x0501)
//
// MsvAvFlags bit values
//
// Exists only after Windows 2000
//

#define MSV1_0_AV_FLAG_FORCE_GUEST                  0x00000001
#if (_WIN32_WINNT >= 0x0600)
#define MSV1_0_AV_FLAG_MIC_HANDSHAKE_MESSAGES       0x00000002 // the client supports
                                                               // hand-shake messages integrity
#if (_WIN32_WINNT >= 0x0601)
#define MSV1_0_AV_FLAG_UNVERIFIED_TARGET            0x00000004

#endif
#endif
#endif

// this is an MSV1_0 private data structure, defining the layout of an NTLM3 response, as sent by a
//  client in the NtChallengeResponse field of the NETLOGON_NETWORK_INFO structure. If can be differentiated
//  from an old style NT response by its length. This is crude, but it needs to pass through servers and
//  the servers' DCs that do not understand NTLM3 but that are willing to pass longer responses.
typedef struct _MSV1_0_NTLM3_RESPONSE {
    UCHAR Response[MSV1_0_NTLM3_RESPONSE_LENGTH]; // hash of OWF of password with all the following fields
    UCHAR RespType;     // id number of response; current is 1
    UCHAR HiRespType;   // highest id number understood by client
    USHORT Flags;       // reserved; must be sent as zero at this version
    ULONG MsgWord;      // 32 bit message from client to server (for use by auth protocol)
    ULONGLONG TimeStamp;    // time stamp when client generated response -- NT system time, quad part
    UCHAR ChallengeFromClient[MSV1_0_CHALLENGE_LENGTH];
    ULONG AvPairsOff;   // offset to start of AvPairs (to allow future expansion)
    UCHAR Buffer[1];    // start of buffer with AV pairs (or future stuff -- so use the offset)
} MSV1_0_NTLM3_RESPONSE, *PMSV1_0_NTLM3_RESPONSE;

#define MSV1_0_NTLM3_INPUT_LENGTH (sizeof(MSV1_0_NTLM3_RESPONSE) - MSV1_0_NTLM3_RESPONSE_LENGTH)
#if(_WIN32_WINNT >= 0x0502)
#define MSV1_0_NTLM3_MIN_NT_RESPONSE_LENGTH RTL_SIZEOF_THROUGH_FIELD(MSV1_0_NTLM3_RESPONSE, AvPairsOff)
#endif

typedef enum {
    MsvAvEOL,                 // end of list
    MsvAvNbComputerName,      // server's computer name -- NetBIOS
    MsvAvNbDomainName,        // server's domain name -- NetBIOS
    MsvAvDnsComputerName,     // server's computer name -- DNS
    MsvAvDnsDomainName,       // server's domain name -- DNS
#if (_WIN32_WINNT >= 0x0501)
    MsvAvDnsTreeName,         // server's tree name -- DNS
    MsvAvFlags,               // server's extended flags -- DWORD mask
#if (_WIN32_WINNT >= 0x0600)
    MsvAvTimestamp,           // contains the server's local time in FILETIME,
                              // (64 bit 100 ns ticks since 1602
                              // (UTC)) in little endian byte order
    MsvAvRestrictions,        // token restrictions
    MsvAvTargetName,
    MsvAvChannelBindings,
#endif
#endif
} MSV1_0_AVID;

typedef struct  _MSV1_0_AV_PAIR {
    USHORT AvId;
    USHORT AvLen;
    // Data is treated as byte array following structure
} MSV1_0_AV_PAIR, *PMSV1_0_AV_PAIR;



///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//       CALL PACKAGE Related Data Structures                                //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////


//
//  MSV1.0 LsaCallAuthenticationPackage() submission and response
//  message types.
//

typedef enum _MSV1_0_PROTOCOL_MESSAGE_TYPE {
    MsV1_0Lm20ChallengeRequest = 0,          // Both submission and response
    MsV1_0Lm20GetChallengeResponse,          // Both submission and response
    MsV1_0EnumerateUsers,                    // Both submission and response
    MsV1_0GetUserInfo,                       // Both submission and response
    MsV1_0ReLogonUsers,                      // Submission only
    MsV1_0ChangePassword,                    // Both submission and response
    MsV1_0ChangeCachedPassword,              // Both submission and response
    MsV1_0GenericPassthrough,                // Both submission and response
    MsV1_0CacheLogon,                        // Submission only, no response
    MsV1_0SubAuth,                           // Both submission and response
    MsV1_0DeriveCredential,                  // Both submission and response
    MsV1_0CacheLookup,                       // Both submission and response
#if (_WIN32_WINNT >= 0x0501)
    MsV1_0SetProcessOption,                  // Submission only, no response
#endif
#if (_WIN32_WINNT >= 0x0600)
    MsV1_0ConfigLocalAliases,
    MsV1_0ClearCachedCredentials,
#endif
#if (_WIN32_WINNT >= 0x0601)
    MsV1_0LookupToken,                        // Both submission and response
#endif
#if (_WIN32_WINNT >= 0x0602)
    MsV1_0ValidateAuth,
    MsV1_0CacheLookupEx,                      // Both submission and response
    MsV1_0GetCredentialKey,                   // Both submission and response
    MsV1_0SetThreadOption,                    // Submission only
#endif
#if (_WIN32_WINNT >= 0x0604)
    MsV1_0DecryptDpapiMasterKey,              // Both submission and response
    MsV1_0GetStrongCredentialKey,             // Both submission and response
#endif
#if (_WIN32_WINNT >= 0x0A00)
    MsV1_0TransferCred,
    MsV1_0ProvisionTbal,
    MsV1_0DeleteTbalSecrets,
#endif
} MSV1_0_PROTOCOL_MESSAGE_TYPE, *PMSV1_0_PROTOCOL_MESSAGE_TYPE;


typedef struct _MSV1_0_CHANGEPASSWORD_REQUEST {
    MSV1_0_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING DomainName;
    UNICODE_STRING AccountName;
    UNICODE_STRING OldPassword;
    UNICODE_STRING NewPassword;
    BOOLEAN        Impersonating;
} MSV1_0_CHANGEPASSWORD_REQUEST, *PMSV1_0_CHANGEPASSWORD_REQUEST;

typedef struct _MSV1_0_CHANGEPASSWORD_RESPONSE {
    MSV1_0_PROTOCOL_MESSAGE_TYPE MessageType;
    BOOLEAN PasswordInfoValid;
    DOMAIN_PASSWORD_INFORMATION DomainPasswordInfo;
} MSV1_0_CHANGEPASSWORD_RESPONSE, *PMSV1_0_CHANGEPASSWORD_RESPONSE;


#if(_WIN32_WINNT >= 0x0502)
//
// MsV1_0GenericPassthrough - for remoting a CallPackage to
// a domain controller on the specified domain
//

typedef struct _MSV1_0_PASSTHROUGH_REQUEST {
    MSV1_0_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING DomainName;
    UNICODE_STRING PackageName;
    ULONG DataLength;
    PUCHAR LogonData;
    ULONG Pad ;
} MSV1_0_PASSTHROUGH_REQUEST, *PMSV1_0_PASSTHROUGH_REQUEST;

typedef struct _MSV1_0_PASSTHROUGH_RESPONSE {
    MSV1_0_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG Pad;
    ULONG DataLength;
    PUCHAR ValidationData;
} MSV1_0_PASSTHROUGH_RESPONSE, *PMSV1_0_PASSTHROUGH_RESPONSE;
#endif


//
// MsV1_0SubAuthInfo submit buffer and response - for submitting a buffer to a
// specified Subauthentication Package during an LsaCallAuthenticationPackage().
// If this Subauthentication is to be done locally, then package this message
// in LsaCallAuthenticationPackage(). If this SubAuthentication needs to be done
// on the domain controller, then call LsaCallauthenticationPackage with the
// message type being MsV1_0GenericPassThrough and the LogonData in this struct
// should be a PMSV1_0_SUBAUTH_REQUEST
//

typedef struct _MSV1_0_SUBAUTH_REQUEST{
    MSV1_0_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG SubAuthPackageId;
    ULONG SubAuthInfoLength;
    PUCHAR SubAuthSubmitBuffer;
} MSV1_0_SUBAUTH_REQUEST, *PMSV1_0_SUBAUTH_REQUEST;

typedef struct _MSV1_0_SUBAUTH_RESPONSE{
    MSV1_0_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG SubAuthInfoLength;
    PUCHAR SubAuthReturnBuffer;
} MSV1_0_SUBAUTH_RESPONSE, *PMSV1_0_SUBAUTH_RESPONSE;


#if(_WIN32_WINNT >= 0x0501)
#define RtlGenRandom                    SystemFunction036
#endif
#if(_WIN32_WINNT >= 0x0500)
#define RtlEncryptMemory                SystemFunction040
#define RtlDecryptMemory                SystemFunction041
#endif

#if(_WIN32_WINNT >= 0x0501)
BOOLEAN
__stdcall
RtlGenRandom(
    _Out_writes_bytes_(RandomBufferLength) PVOID RandomBuffer,
    _In_ ULONG RandomBufferLength
    );
#endif

/*
 * #if(_WIN32_WINNT >= 0x0500) -- Disabled until WinHTTP fixes their target version.
 */

//
// The buffer passed into RtlEncryptMemory and RtlDecryptMemory
// must be a multiple of this length.
//

#define RTL_ENCRYPT_MEMORY_SIZE             8

//
// Allow Encrypt/Decrypt across process boundaries.
// eg: encrypted buffer passed across LPC to another process which calls RtlDecryptMemory.
//

#define RTL_ENCRYPT_OPTION_CROSS_PROCESS    0x01

//
// Allow Encrypt/Decrypt across callers with same LogonId.
// eg: encrypted buffer passed across LPC to another process which calls RtlDecryptMemory whilst impersonating.
//

#define RTL_ENCRYPT_OPTION_SAME_LOGON       0x02

//
// Allow callers to encrypt information to be decrypted only by a system process
//

#define RTL_ENCRYPT_OPTION_FOR_SYSTEM       0x04

NTSTATUS
__stdcall
RtlEncryptMemory(
    _Inout_updates_bytes_(MemorySize) PVOID Memory,
    _In_ ULONG MemorySize,
    _In_ ULONG OptionFlags
    );

NTSTATUS
__stdcall
RtlDecryptMemory(
    _Inout_updates_bytes_(MemorySize) PVOID Memory,
    _In_ ULONG MemorySize,
    _In_ ULONG OptionFlags
    );
//#endif


// Revision of the Kerberos Protocol.  MS uses Version 5, Revision 6

#define KERBEROS_VERSION    5
#define KERBEROS_REVISION   6



// Encryption Types:
// These encryption types are supported by the default MS KERBSUPP DLL
// as crypto systems.  Values over 127 are local values, and may be changed
// without notice.

#define KERB_ETYPE_NULL             0
#define KERB_ETYPE_DES_CBC_CRC      1
#define KERB_ETYPE_DES_CBC_MD4      2
#define KERB_ETYPE_DES_CBC_MD5      3
#define KERB_ETYPE_AES128_CTS_HMAC_SHA1_96    17
#define KERB_ETYPE_AES256_CTS_HMAC_SHA1_96    18
#define KERB_ETYPE_AES128_CTS_HMAC_SHA256     19
#define KERB_ETYPE_AES256_CTS_HMAC_SHA384     20


#define KERB_ETYPE_RC4_MD4          -128    // FFFFFF80
#define KERB_ETYPE_RC4_PLAIN2       -129
#define KERB_ETYPE_RC4_LM           -130
#define KERB_ETYPE_RC4_SHA          -131
#define KERB_ETYPE_DES_PLAIN        -132
#define KERB_ETYPE_RC4_HMAC_OLD     -133    // FFFFFF7B
#define KERB_ETYPE_RC4_PLAIN_OLD    -134
#define KERB_ETYPE_RC4_HMAC_OLD_EXP -135
#define KERB_ETYPE_RC4_PLAIN_OLD_EXP -136
#define KERB_ETYPE_RC4_PLAIN        -140
#define KERB_ETYPE_RC4_PLAIN_EXP    -141

//
// used internally by userapi.cxx
//

#define KERB_ETYPE_AES128_CTS_HMAC_SHA1_96_PLAIN    -148
#define KERB_ETYPE_AES256_CTS_HMAC_SHA1_96_PLAIN    -149

//
// Pkinit encryption types
//


#define KERB_ETYPE_DSA_SHA1_CMS                             9
#define KERB_ETYPE_RSA_MD5_CMS                              10
#define KERB_ETYPE_RSA_SHA1_CMS                             11
#define KERB_ETYPE_RC2_CBC_ENV                              12
#define KERB_ETYPE_RSA_ENV                                  13
#define KERB_ETYPE_RSA_ES_OEAP_ENV                          14
#define KERB_ETYPE_DES_EDE3_CBC_ENV                         15


//
// Deprecated
//

#define KERB_ETYPE_DSA_SIGN                                8
#define KERB_ETYPE_RSA_PRIV                                9
#define KERB_ETYPE_RSA_PUB                                 10
#define KERB_ETYPE_RSA_PUB_MD5                             11
#define KERB_ETYPE_RSA_PUB_SHA1                            12
#define KERB_ETYPE_PKCS7_PUB                               13

#if(_WIN32_WINNT >= 0x0502)
//
// Unsupported but defined types
//

#define KERB_ETYPE_DES3_CBC_MD5                             5
#define KERB_ETYPE_DES3_CBC_SHA1                            7
#define KERB_ETYPE_DES3_CBC_SHA1_KD                        16
#endif

//
// In use types
//

#define KERB_ETYPE_DES_CBC_MD5_NT                          20
#define KERB_ETYPE_RC4_HMAC_NT                             23
#define KERB_ETYPE_RC4_HMAC_NT_EXP                         24

// Checksum algorithms.
// These algorithms are keyed internally for our use.

#define KERB_CHECKSUM_NONE  0
#define KERB_CHECKSUM_CRC32         1
#define KERB_CHECKSUM_MD4           2
#define KERB_CHECKSUM_KRB_DES_MAC   4
#if (_WIN32_WINNT >= 0x0501)
#define KERB_CHECKSUM_KRB_DES_MAC_K 5
#endif
#define KERB_CHECKSUM_MD5           7
#define KERB_CHECKSUM_MD5_DES       8

#define KERB_CHECKSUM_SHA1_NEW      14           // defined in RFC3961
#define KERB_CHECKSUM_HMAC_SHA1_96_AES128  15
#define KERB_CHECKSUM_HMAC_SHA1_96_AES256  16

#define KERB_CHECKSUM_LM            -130
#define KERB_CHECKSUM_SHA1          -131
#define KERB_CHECKSUM_REAL_CRC32    -132
#define KERB_CHECKSUM_DES_MAC       -133
#define KERB_CHECKSUM_DES_MAC_MD5   -134
#define KERB_CHECKSUM_MD25          -135
#define KERB_CHECKSUM_RC4_MD5       -136
#define KERB_CHECKSUM_MD5_HMAC      -137                // used by netlogon
#define KERB_CHECKSUM_HMAC_MD5      -138                // used by Kerberos
#define KERB_CHECKSUM_SHA256        -139
#define KERB_CHECKSUM_SHA384        -140
#define KERB_CHECKSUM_SHA512        -141

//
// used internally by userapi.cxx
//

#define KERB_CHECKSUM_HMAC_SHA1_96_AES128_Ki -150
#define KERB_CHECKSUM_HMAC_SHA1_96_AES256_Ki -151

#define AUTH_REQ_ALLOW_FORWARDABLE      0x00000001
#define AUTH_REQ_ALLOW_PROXIABLE        0x00000002
#define AUTH_REQ_ALLOW_POSTDATE         0x00000004
#define AUTH_REQ_ALLOW_RENEWABLE        0x00000008
#define AUTH_REQ_ALLOW_NOADDRESS        0x00000010
#define AUTH_REQ_ALLOW_ENC_TKT_IN_SKEY  0x00000020
#define AUTH_REQ_ALLOW_VALIDATE         0x00000040
#define AUTH_REQ_VALIDATE_CLIENT        0x00000080
#define AUTH_REQ_OK_AS_DELEGATE         0x00000100
#define AUTH_REQ_PREAUTH_REQUIRED       0x00000200
#define AUTH_REQ_TRANSITIVE_TRUST       0x00000400
#if(_WIN32_WINNT >= 0x0502)
#define AUTH_REQ_ALLOW_S4U_DELEGATE     0x00000800
#endif


#define AUTH_REQ_PER_USER_FLAGS         (AUTH_REQ_ALLOW_FORWARDABLE | \
                                         AUTH_REQ_ALLOW_PROXIABLE | \
                                         AUTH_REQ_ALLOW_POSTDATE | \
                                         AUTH_REQ_ALLOW_RENEWABLE | \
                                         AUTH_REQ_ALLOW_VALIDATE )
//
// Ticket Flags:
//

#define KERB_TICKET_FLAGS_reserved          0x80000000
#define KERB_TICKET_FLAGS_forwardable       0x40000000
#define KERB_TICKET_FLAGS_forwarded         0x20000000
#define KERB_TICKET_FLAGS_proxiable         0x10000000
#define KERB_TICKET_FLAGS_proxy             0x08000000
#define KERB_TICKET_FLAGS_may_postdate      0x04000000
#define KERB_TICKET_FLAGS_postdated         0x02000000
#define KERB_TICKET_FLAGS_invalid           0x01000000
#define KERB_TICKET_FLAGS_renewable         0x00800000
#define KERB_TICKET_FLAGS_initial           0x00400000
#define KERB_TICKET_FLAGS_pre_authent       0x00200000
#define KERB_TICKET_FLAGS_hw_authent        0x00100000
#define KERB_TICKET_FLAGS_ok_as_delegate    0x00040000
#define KERB_TICKET_FLAGS_name_canonicalize 0x00010000
#if (_WIN32_WINNT == 0x0501)
#define KERB_TICKET_FLAGS_cname_in_pa_data  0x00040000
#endif
#define KERB_TICKET_FLAGS_enc_pa_rep        0x00010000
#define KERB_TICKET_FLAGS_reserved1         0x00000001



#if (_WIN32_WINNT >= 0x0501)
//
// Name types
//

#define KRB_NT_UNKNOWN   0                // Name type not known
#define KRB_NT_PRINCIPAL 1                // Just the name of the principal as in DCE, or for users
#define KRB_NT_PRINCIPAL_AND_ID -131      // Name of the principal and its SID.
#define KRB_NT_SRV_INST  2                // Service and other unique instance (krbtgt)
#define KRB_NT_SRV_INST_AND_ID -132       // SPN and SID
#define KRB_NT_SRV_HST   3                // Service with host name as instance (telnet, rcommands)
#define KRB_NT_SRV_XHST  4                // Service with host as remaining components
#define KRB_NT_UID       5                // Unique ID
#define KRB_NT_ENTERPRISE_PRINCIPAL 10    // UPN **ONLY**
#define KRB_NT_WELLKNOWN 11               // Well-known principal names
#define KRB_NT_ENT_PRINCIPAL_AND_ID -130  // UPN and SID

//
// MS extensions, negative according to the RFC
//

#define KRB_NT_MS_PRINCIPAL         -128        // NT4 style name

#define KRB_NT_MS_PRINCIPAL_AND_ID  -129        // nt4 style name with sid

#define KRB_NT_MS_BRANCH_ID         -133        // Branch ID

#define KERB_IS_MS_PRINCIPAL(_x_) (((_x_) <= KRB_NT_MS_PRINCIPAL) || ((_x_) >= KRB_NT_ENTERPRISE_PRINCIPAL))
#endif

#if (_WIN32_WINNT >= 0x0600)
#define KRB_NT_X500_PRINCIPAL 6           // Encoded X.500 Distingished name [RFC 2253]
#endif

#define KRB_WELLKNOWN_STRING L"WELLKNOWN"
#define KRB_ANONYMOUS_STRING L"ANONYMOUS"


#ifndef MICROSOFT_KERBEROS_NAME_A

#define MICROSOFT_KERBEROS_NAME_A   "Kerberos"
#define MICROSOFT_KERBEROS_NAME_W   L"Kerberos"
#ifdef WIN32_CHICAGO
#define MICROSOFT_KERBEROS_NAME MICROSOFT_KERBEROS_NAME_A
#else
#define MICROSOFT_KERBEROS_NAME MICROSOFT_KERBEROS_NAME_W
#endif // WIN32_CHICAGO
#endif // MICROSOFT_KERBEROS_NAME_A


/////////////////////////////////////////////////////////////////////////
//
// Quality of protection parameters for MakeSignature / EncryptMessage
//
/////////////////////////////////////////////////////////////////////////

//
// This flag indicates to EncryptMessage that the message is not to actually
// be encrypted, but a header/trailer are to be produced.
//

#define KERB_WRAP_NO_ENCRYPT 0x80000001

/////////////////////////////////////////////////////////////////////////
//
// LsaLogonUser parameters
//
/////////////////////////////////////////////////////////////////////////

typedef enum _KERB_LOGON_SUBMIT_TYPE {
    KerbInteractiveLogon = 2,
    KerbSmartCardLogon = 6,
    KerbWorkstationUnlockLogon = 7,
    KerbSmartCardUnlockLogon = 8,
    KerbProxyLogon = 9,
    KerbTicketLogon = 10,
    KerbTicketUnlockLogon = 11,
//#if (_WIN32_WINNT >= 0x0501) -- Disabled until IIS fixes their target version.
    KerbS4ULogon = 12,
//#endif
#if (_WIN32_WINNT >= 0x0600)     
    KerbCertificateLogon = 13, 
    KerbCertificateS4ULogon = 14,
    KerbCertificateUnlockLogon = 15,
#endif    
#if (_WIN32_WINNT >= 0x0602)     
    KerbNoElevationLogon = 83,
    KerbLuidLogon = 84,
#endif    
} KERB_LOGON_SUBMIT_TYPE, *PKERB_LOGON_SUBMIT_TYPE;

typedef struct _KERB_INTERACTIVE_LOGON {
    KERB_LOGON_SUBMIT_TYPE MessageType;
    UNICODE_STRING LogonDomainName;
    UNICODE_STRING UserName;
    UNICODE_STRING Password;
} KERB_INTERACTIVE_LOGON, *PKERB_INTERACTIVE_LOGON;

typedef struct _KERB_INTERACTIVE_UNLOCK_LOGON {
    KERB_INTERACTIVE_LOGON Logon;
    LUID LogonId;
} KERB_INTERACTIVE_UNLOCK_LOGON, *PKERB_INTERACTIVE_UNLOCK_LOGON;

typedef struct _KERB_SMART_CARD_LOGON {
    KERB_LOGON_SUBMIT_TYPE MessageType;
    UNICODE_STRING Pin;
    ULONG CspDataLength;
    PUCHAR CspData;
} KERB_SMART_CARD_LOGON, *PKERB_SMART_CARD_LOGON;

typedef struct _KERB_SMART_CARD_UNLOCK_LOGON {
    KERB_SMART_CARD_LOGON Logon;
    LUID LogonId;
} KERB_SMART_CARD_UNLOCK_LOGON, *PKERB_SMART_CARD_UNLOCK_LOGON;

#if (_WIN32_WINNT >= 0x0600) 

//
// let the KDC detect account mapping conflicts for the same certificate.
//

#define KERB_CERTIFICATE_LOGON_FLAG_CHECK_DUPLICATES            0x1
#define KERB_CERTIFICATE_LOGON_FLAG_USE_CERTIFICATE_INFO        0x2

typedef struct _KERB_CERTIFICATE_LOGON {
    KERB_LOGON_SUBMIT_TYPE MessageType; // KerbCertificateLogon
    UNICODE_STRING DomainName; // OPTIONAL, if supplied, used to locate the account forest
    UNICODE_STRING UserName;   // OPTIONAL, if supplied, used to locate the account
    UNICODE_STRING Pin;
    ULONG Flags;               // additional flags
    ULONG CspDataLength;
    PUCHAR CspData;            // contains the smartcard CSP data
} KERB_CERTIFICATE_LOGON, *PKERB_CERTIFICATE_LOGON;

typedef struct _KERB_CERTIFICATE_UNLOCK_LOGON {
    KERB_CERTIFICATE_LOGON Logon;
    LUID LogonId;
} KERB_CERTIFICATE_UNLOCK_LOGON, *PKERB_CERTIFICATE_UNLOCK_LOGON;

//
// let the KDC detect account mapping conflicts for the same certificate.
// 

#define KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_DUPLICATES 0x1
#define KERB_CERTIFICATE_S4U_LOGON_FLAG_CHECK_LOGONHOURS 0x2
#define KERB_CERTIFICATE_S4U_LOGON_FLAG_FAIL_IF_NT_AUTH_POLICY_REQUIRED 0x4
#define KERB_CERTIFICATE_S4U_LOGON_FLAG_IDENTIFY         0x8 // request for identify token, must have the same value as KERB_S4U_LOGON_FLAG_IDENTIFY

typedef struct _KERB_CERTIFICATE_S4U_LOGON {
    KERB_LOGON_SUBMIT_TYPE MessageType;
    ULONG Flags; 
    UNICODE_STRING UserPrincipalName; 
            // OPTIONAL, certificate mapping hints: username or username@domain
    UNICODE_STRING DomainName; // used to locate the forest
            // OPTIONAL, certificate mapping hints: if missing, using the local machine's domain
    ULONG CertificateLength;   // for the client certificate 
    PUCHAR Certificate;        // for the client certificate, BER encoded
} KERB_CERTIFICATE_S4U_LOGON, *PKERB_CERTIFICATE_S4U_LOGON;

#endif 

//
// Structure used for a ticket-only logon
//

typedef struct _KERB_TICKET_LOGON {
    KERB_LOGON_SUBMIT_TYPE MessageType;
    ULONG Flags;
    ULONG ServiceTicketLength;
    ULONG TicketGrantingTicketLength;
    PUCHAR ServiceTicket;               // REQUIRED: Service ticket "host"
    PUCHAR TicketGrantingTicket;        // OPTIONAL: User's encdoded in a KERB_CRED message, encrypted with session key from service ticket
} KERB_TICKET_LOGON, *PKERB_TICKET_LOGON;

//
// Flags for the ticket logon flags field
//

#define KERB_LOGON_FLAG_ALLOW_EXPIRED_TICKET 0x1
#define KERB_LOGON_FLAG_REDIRECTED           0x2

typedef struct _KERB_TICKET_UNLOCK_LOGON {
    KERB_TICKET_LOGON Logon;
    LUID LogonId;
} KERB_TICKET_UNLOCK_LOGON, *PKERB_TICKET_UNLOCK_LOGON;

//#if (_WIN32_WINNT >= 0x0501) -- Disabled until IIS fixes their target version.
//
//  Used for S4U Client requests
//
//

#if (_WIN32_WINNT >= 0x0600)

//
// request to enforce logon hours policy
//

#define KERB_S4U_LOGON_FLAG_CHECK_LOGONHOURS 0x2
#define KERB_S4U_LOGON_FLAG_IDENTIFY         0x8 // request for identify token

#endif

typedef struct _KERB_S4U_LOGON {
    KERB_LOGON_SUBMIT_TYPE MessageType;
    ULONG Flags;
    UNICODE_STRING ClientUpn;   // REQUIRED: UPN for client
    UNICODE_STRING ClientRealm; // Optional: Client Realm, if known
} KERB_S4U_LOGON, *PKERB_S4U_LOGON;
//#endif


//
// Use the same profile structure as MSV1_0
//
typedef enum _KERB_PROFILE_BUFFER_TYPE {
    KerbInteractiveProfile = 2,
    KerbSmartCardProfile = 4,
    KerbTicketProfile = 6
} KERB_PROFILE_BUFFER_TYPE, *PKERB_PROFILE_BUFFER_TYPE;


typedef struct _KERB_INTERACTIVE_PROFILE {
    KERB_PROFILE_BUFFER_TYPE MessageType;
    USHORT LogonCount;
    USHORT BadPasswordCount;
    LARGE_INTEGER LogonTime;
    LARGE_INTEGER LogoffTime;
    LARGE_INTEGER KickOffTime;
    LARGE_INTEGER PasswordLastSet;
    LARGE_INTEGER PasswordCanChange;
    LARGE_INTEGER PasswordMustChange;
    UNICODE_STRING LogonScript;
    UNICODE_STRING HomeDirectory;
    UNICODE_STRING FullName;
    UNICODE_STRING ProfilePath;
    UNICODE_STRING HomeDirectoryDrive;
    UNICODE_STRING LogonServer;
    ULONG UserFlags;
} KERB_INTERACTIVE_PROFILE, *PKERB_INTERACTIVE_PROFILE;


//
// For smart card, we return a smart card profile, which is an interactive
// profile plus a certificate
//

typedef struct _KERB_SMART_CARD_PROFILE {
    KERB_INTERACTIVE_PROFILE Profile;
    ULONG CertificateSize;
    PUCHAR CertificateData;
} KERB_SMART_CARD_PROFILE, *PKERB_SMART_CARD_PROFILE;


//
// For a ticket logon profile, we return the session key from the ticket
//

typedef struct KERB_CRYPTO_KEY {
    LONG KeyType;
    ULONG Length;
    PUCHAR Value;
} KERB_CRYPTO_KEY, *PKERB_CRYPTO_KEY;

typedef struct KERB_CRYPTO_KEY32 {
    LONG KeyType;
    ULONG Length;
    ULONG Offset;
} KERB_CRYPTO_KEY32, *PKERB_CRYPTO_KEY32;

typedef struct _KERB_TICKET_PROFILE {
    KERB_INTERACTIVE_PROFILE Profile;
    KERB_CRYPTO_KEY SessionKey;
} KERB_TICKET_PROFILE, *PKERB_TICKET_PROFILE;


typedef enum _KERB_PROTOCOL_MESSAGE_TYPE {
    KerbDebugRequestMessage = 0,
    KerbQueryTicketCacheMessage,
    KerbChangeMachinePasswordMessage,
    KerbVerifyPacMessage,
    KerbRetrieveTicketMessage,
    KerbUpdateAddressesMessage,
    KerbPurgeTicketCacheMessage,
    KerbChangePasswordMessage,
    KerbRetrieveEncodedTicketMessage,
    KerbDecryptDataMessage,
    KerbAddBindingCacheEntryMessage,
    KerbSetPasswordMessage,
    KerbSetPasswordExMessage,
#if (_WIN32_WINNT == 0x0500)
    KerbAddExtraCredentialsMessage = 17
#endif
#if (_WIN32_WINNT >= 0x0501)
    KerbVerifyCredentialsMessage,
    KerbQueryTicketCacheExMessage,
    KerbPurgeTicketCacheExMessage,
#endif
#if (_WIN32_WINNT >= 0x0502)
    KerbRefreshSmartcardCredentialsMessage,
    KerbAddExtraCredentialsMessage,
    KerbQuerySupplementalCredentialsMessage,
#endif
#if (_WIN32_WINNT >= 0x0600)
    KerbTransferCredentialsMessage,
    KerbQueryTicketCacheEx2Message,
    KerbSubmitTicketMessage,
    KerbAddExtraCredentialsExMessage,
#endif
#if (_WIN32_WINNT >= 0x0602)
    KerbQueryKdcProxyCacheMessage,
    KerbPurgeKdcProxyCacheMessage,
    KerbQueryTicketCacheEx3Message, 
    KerbCleanupMachinePkinitCredsMessage,
    KerbAddBindingCacheEntryExMessage,
    KerbQueryBindingCacheMessage,
    KerbPurgeBindingCacheMessage,
    KerbPinKdcMessage,
    KerbUnpinAllKdcsMessage,
    KerbQueryDomainExtendedPoliciesMessage,
    KerbQueryS4U2ProxyCacheMessage,
#endif
#if (_WIN32_WINNT >= 0x0A00)
    KerbRetrieveKeyTabMessage,
    KerbRefreshPolicyMessage,
    KerbPrintCloudKerberosDebugMessage,
    KerbNetworkTicketLogonMessage,
    KerbNlChangeMachinePasswordMessage,
#endif
} KERB_PROTOCOL_MESSAGE_TYPE, *PKERB_PROTOCOL_MESSAGE_TYPE;


//
// Used both for retrieving tickets and for querying ticket cache
//

typedef struct _KERB_QUERY_TKT_CACHE_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
} KERB_QUERY_TKT_CACHE_REQUEST, *PKERB_QUERY_TKT_CACHE_REQUEST;


typedef struct _KERB_TICKET_CACHE_INFO {
    UNICODE_STRING ServerName;
    UNICODE_STRING RealmName;
    LARGE_INTEGER StartTime;
    LARGE_INTEGER EndTime;
    LARGE_INTEGER RenewTime;
    LONG EncryptionType;
    ULONG TicketFlags;
} KERB_TICKET_CACHE_INFO, *PKERB_TICKET_CACHE_INFO;


#if (_WIN32_WINNT >= 0x0501)
typedef struct _KERB_TICKET_CACHE_INFO_EX {
    UNICODE_STRING ClientName;
    UNICODE_STRING ClientRealm;
    UNICODE_STRING ServerName;
    UNICODE_STRING ServerRealm;
    LARGE_INTEGER StartTime;
    LARGE_INTEGER EndTime;
    LARGE_INTEGER RenewTime;
    LONG EncryptionType;
    ULONG TicketFlags;
} KERB_TICKET_CACHE_INFO_EX, *PKERB_TICKET_CACHE_INFO_EX;
#endif

typedef struct _KERB_TICKET_CACHE_INFO_EX2 {
    UNICODE_STRING ClientName;
    UNICODE_STRING ClientRealm;
    UNICODE_STRING ServerName;
    UNICODE_STRING ServerRealm;
    LARGE_INTEGER StartTime;
    LARGE_INTEGER EndTime;
    LARGE_INTEGER RenewTime;
    LONG EncryptionType;
    ULONG TicketFlags;
    
    //
    // the following are new in KERB_TICKET_CACHE_INFO_EX2 
    //
    ULONG SessionKeyType;
    ULONG BranchId;
} KERB_TICKET_CACHE_INFO_EX2, *PKERB_TICKET_CACHE_INFO_EX2;

#if (_WIN32_WINNT >= 0x0602)
typedef struct _KERB_TICKET_CACHE_INFO_EX3 {
    UNICODE_STRING ClientName;
    UNICODE_STRING ClientRealm;
    UNICODE_STRING ServerName;
    UNICODE_STRING ServerRealm;
    LARGE_INTEGER  StartTime;
    LARGE_INTEGER  EndTime;
    LARGE_INTEGER  RenewTime;
    LONG           EncryptionType;
    ULONG          TicketFlags;
    ULONG          SessionKeyType;
    ULONG          BranchId;
    
    //
    // the following are new in KERB_TICKET_CACHE_INFO_EX3
    //

    ULONG          CacheFlags;
    UNICODE_STRING KdcCalled;
} KERB_TICKET_CACHE_INFO_EX3, *PKERB_TICKET_CACHE_INFO_EX3;
#endif

typedef struct _KERB_QUERY_TKT_CACHE_RESPONSE {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG CountOfTickets;
    KERB_TICKET_CACHE_INFO Tickets[ANYSIZE_ARRAY];
} KERB_QUERY_TKT_CACHE_RESPONSE, *PKERB_QUERY_TKT_CACHE_RESPONSE;

#if (_WIN32_WINNT >= 0x0502)
typedef struct _KERB_QUERY_TKT_CACHE_EX_RESPONSE {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG CountOfTickets;
    KERB_TICKET_CACHE_INFO_EX Tickets[ANYSIZE_ARRAY];
} KERB_QUERY_TKT_CACHE_EX_RESPONSE, *PKERB_QUERY_TKT_CACHE_EX_RESPONSE;
#endif

typedef struct _KERB_QUERY_TKT_CACHE_EX2_RESPONSE {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG CountOfTickets;
    KERB_TICKET_CACHE_INFO_EX2 Tickets[ANYSIZE_ARRAY];
} KERB_QUERY_TKT_CACHE_EX2_RESPONSE, *PKERB_QUERY_TKT_CACHE_EX2_RESPONSE;

#if (_WIN32_WINNT >= 0x0602)
typedef struct _KERB_QUERY_TKT_CACHE_EX3_RESPONSE {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG                      CountOfTickets;
    KERB_TICKET_CACHE_INFO_EX3 Tickets[ANYSIZE_ARRAY];
} KERB_QUERY_TKT_CACHE_EX3_RESPONSE, *PKERB_QUERY_TKT_CACHE_EX3_RESPONSE;
#endif

//
// Types for retrieving encoded ticket from the cache
//

#ifndef __SECHANDLE_DEFINED__
typedef struct _SecHandle
{
    ULONG_PTR dwLower ;
    ULONG_PTR dwUpper ;
} SecHandle, * PSecHandle ;

#define __SECHANDLE_DEFINED__
#endif // __SECHANDLE_DEFINED__

#if (_WIN32_WINNT >= 0x0501)
// Ticket Flags
#define KERB_USE_DEFAULT_TICKET_FLAGS       0x0

// CacheOptions
#define KERB_RETRIEVE_TICKET_DEFAULT           0x0
#endif
#define KERB_RETRIEVE_TICKET_DONT_USE_CACHE    0x1
#define KERB_RETRIEVE_TICKET_USE_CACHE_ONLY    0x2
#define KERB_RETRIEVE_TICKET_USE_CREDHANDLE    0x4
#if (_WIN32_WINNT >= 0x0501)
#define KERB_RETRIEVE_TICKET_AS_KERB_CRED      0x8
#define KERB_RETRIEVE_TICKET_WITH_SEC_CRED    0x10
#endif
#if (_WIN32_WINNT >= 0x0600)
#define KERB_RETRIEVE_TICKET_CACHE_TICKET     0x20
#endif

#if (_WIN32_WINNT >= 0x0601)
#define KERB_RETRIEVE_TICKET_MAX_LIFETIME     0x40
#endif

#if (_WIN32_WINNT >= 0x0501)
// Encryption Type options
#define KERB_ETYPE_DEFAULT 0x0 // don't specify etype in tkt req.

typedef struct _KERB_AUTH_DATA {
    ULONG Type;
    ULONG Length;
    PUCHAR Data;
} KERB_AUTH_DATA, *PKERB_AUTH_DATA;


typedef struct _KERB_NET_ADDRESS {
    ULONG Family;
    ULONG Length;
    PCHAR Address;
} KERB_NET_ADDRESS, *PKERB_NET_ADDRESS;


typedef struct _KERB_NET_ADDRESSES {
    ULONG Number;
    KERB_NET_ADDRESS Addresses[ANYSIZE_ARRAY];
} KERB_NET_ADDRESSES, *PKERB_NET_ADDRESSES;
#endif

//
// Types for the information about a ticket
//

typedef struct _KERB_EXTERNAL_NAME {
    SHORT NameType;
    USHORT NameCount;
    UNICODE_STRING Names[ANYSIZE_ARRAY];
} KERB_EXTERNAL_NAME, *PKERB_EXTERNAL_NAME;


typedef struct _KERB_EXTERNAL_TICKET {
    PKERB_EXTERNAL_NAME ServiceName;
    PKERB_EXTERNAL_NAME TargetName;
    PKERB_EXTERNAL_NAME ClientName;
    UNICODE_STRING DomainName;
    UNICODE_STRING TargetDomainName;
    UNICODE_STRING AltTargetDomainName;  // contains ClientDomainName
    KERB_CRYPTO_KEY SessionKey;
    ULONG TicketFlags;
    ULONG Flags;
    LARGE_INTEGER KeyExpirationTime;
    LARGE_INTEGER StartTime;
    LARGE_INTEGER EndTime;
    LARGE_INTEGER RenewUntil;
    LARGE_INTEGER TimeSkew;
    ULONG EncodedTicketSize;
    PUCHAR EncodedTicket;
} KERB_EXTERNAL_TICKET, *PKERB_EXTERNAL_TICKET;

typedef struct _KERB_RETRIEVE_TKT_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    UNICODE_STRING TargetName;
    ULONG TicketFlags;
    ULONG CacheOptions;
    LONG EncryptionType;
    SecHandle CredentialsHandle;
} KERB_RETRIEVE_TKT_REQUEST, *PKERB_RETRIEVE_TKT_REQUEST;

typedef struct _KERB_RETRIEVE_TKT_RESPONSE {
    KERB_EXTERNAL_TICKET Ticket;
} KERB_RETRIEVE_TKT_RESPONSE, *PKERB_RETRIEVE_TKT_RESPONSE;

//
// Used to purge entries from the ticket cache
//

typedef struct _KERB_PURGE_TKT_CACHE_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    UNICODE_STRING ServerName;
    UNICODE_STRING RealmName;
} KERB_PURGE_TKT_CACHE_REQUEST, *PKERB_PURGE_TKT_CACHE_REQUEST;

#if (_WIN32_WINNT >= 0x0501)
//
// Flags for purge requests
//

#define KERB_PURGE_ALL_TICKETS 1

typedef struct _KERB_PURGE_TKT_CACHE_EX_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    ULONG Flags;
    KERB_TICKET_CACHE_INFO_EX TicketTemplate;
} KERB_PURGE_TKT_CACHE_EX_REQUEST, *PKERB_PURGE_TKT_CACHE_EX_REQUEST;
#endif

typedef struct _KERB_SUBMIT_TKT_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    ULONG Flags;
    KERB_CRYPTO_KEY32 Key; // key to decrypt KERB_CRED
    ULONG KerbCredSize;
    ULONG KerbCredOffset;
} KERB_SUBMIT_TKT_REQUEST, *PKERB_SUBMIT_TKT_REQUEST;  

#if (_WIN32_WINNT >= 0x0602)

typedef struct _KERB_QUERY_KDC_PROXY_CACHE_REQUEST
{
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG                      Flags;  // reserved, must be 0
    LUID                       LogonId;
} KERB_QUERY_KDC_PROXY_CACHE_REQUEST, *PKERB_QUERY_KDC_PROXY_CACHE_REQUEST;

typedef struct _KDC_PROXY_CACHE_ENTRY_DATA
{
    ULONG64        SinceLastUsed;    // milliseconds since lastly used
    UNICODE_STRING DomainName;
    UNICODE_STRING ProxyServerName;
    UNICODE_STRING ProxyServerVdir;
    USHORT         ProxyServerPort;
    LUID           LogonId;          // per-credential cache
    UNICODE_STRING CredUserName;     // per-credential cache using supplied cred
    UNICODE_STRING CredDomainName;   // per-credential cache using supplied cred
    BOOLEAN        GlobalCache;
} KDC_PROXY_CACHE_ENTRY_DATA, *PKDC_PROXY_CACHE_ENTRY_DATA;

typedef struct _KERB_QUERY_KDC_PROXY_CACHE_RESPONSE
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       CountOfEntries;
    PKDC_PROXY_CACHE_ENTRY_DATA Entries;
} KERB_QUERY_KDC_PROXY_CACHE_RESPONSE, *PKERB_QUERY_KDC_PROXY_CACHE_RESPONSE;

typedef struct _KERB_PURGE_KDC_PROXY_CACHE_REQUEST 
{
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG                      Flags;  // reserved, must be 0
    LUID                       LogonId;
} KERB_PURGE_KDC_PROXY_CACHE_REQUEST, *PKERB_PURGE_KDC_PROXY_CACHE_REQUEST;

typedef struct _KERB_PURGE_KDC_PROXY_CACHE_RESPONSE
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       CountOfPurged;
} KERB_PURGE_KDC_PROXY_CACHE_RESPONSE, *PKERB_PURGE_KDC_PROXY_CACHE_RESPONSE;

//
// S4U2Proxy Cache Info
//

#define KERB_S4U2PROXY_CACHE_ENTRY_INFO_FLAG_NEGATIVE 0x1

typedef struct _KERB_S4U2PROXY_CACHE_ENTRY_INFO 
{
    UNICODE_STRING ServerName;
    ULONG          Flags;
    NTSTATUS       LastStatus;
    LARGE_INTEGER  Expiry;
} KERB_S4U2PROXY_CACHE_ENTRY_INFO, *PKERB_S4U2PROXY_CACHE_ENTRY_INFO;

#define KERB_S4U2PROXY_CRED_FLAG_NEGATIVE 0x1

typedef struct _KERB_S4U2PROXY_CRED
{
    UNICODE_STRING                   UserName;
    UNICODE_STRING                   DomainName;
    ULONG                            Flags;
    NTSTATUS                         LastStatus;
    LARGE_INTEGER                    Expiry;
    ULONG                            CountOfEntries;
    PKERB_S4U2PROXY_CACHE_ENTRY_INFO Entries;
} KERB_S4U2PROXY_CRED, *PKERB_S4U2PROXY_CRED;

typedef struct _KERB_QUERY_S4U2PROXY_CACHE_REQUEST 
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       Flags;  // reserved, must be 0
    LUID                        LogonId;
} KERB_QUERY_S4U2PROXY_CACHE_REQUEST, *PKERB_QUERY_S4U2PROXY_CACHE_REQUEST;

typedef struct _KERB_QUERY_S4U2PROXY_CACHE_RESPONSE
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       CountOfCreds;
    PKERB_S4U2PROXY_CRED        Creds;
} KERB_QUERY_S4U2PROXY_CACHE_RESPONSE, *PKERB_QUERY_S4U2PROXY_CACHE_RESPONSE;

#endif

#if (_WIN32_WINNT >= 0x0A00)

typedef struct _KERB_RETRIEVE_KEY_TAB_REQUEST
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       Flags;
    UNICODE_STRING              UserName;
    UNICODE_STRING              DomainName;
    UNICODE_STRING              Password;
} KERB_RETRIEVE_KEY_TAB_REQUEST, *PKERB_RETRIEVE_KEY_TAB_REQUEST;

typedef struct _KERB_RETRIEVE_KEY_TAB_RESPONSE
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       KeyTabLength;
    PUCHAR                      KeyTab;
} KERB_RETRIEVE_KEY_TAB_RESPONSE, *PKERB_RETRIEVE_KEY_TAB_RESPONSE;

#define KERB_REFRESH_POLICY_KERBEROS 0x1
#define KERB_REFRESH_POLICY_KDC 0x2

typedef struct _KERB_REFRESH_POLICY_REQUEST
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       Flags;
} KERB_REFRESH_POLICY_REQUEST, *PKERB_REFRESH_POLICY_REQUEST;

typedef struct _KERB_REFRESH_POLICY_RESPONSE
{
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       Flags;
} KERB_REFRESH_POLICY_RESPONSE, *PKERB_REFRESH_POLICY_RESPONSE;

typedef struct _KERB_CLOUD_KERBEROS_DEBUG_REQUEST
{
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
} KERB_CLOUD_KERBEROS_DEBUG_REQUEST, *PKERB_CLOUD_KERBEROS_DEBUG_REQUEST;

typedef struct _KERB_CLOUD_KERBEROS_DEBUG_RESPONSE
{
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    ULONG Version;
    ULONG Length; // specified in bytes
    ULONG Data[ANYSIZE_ARRAY];
} KERB_CLOUD_KERBEROS_DEBUG_RESPONSE, *PKERB_CLOUD_KERBEROS_DEBUG_RESPONSE;

#define KERB_CLOUD_KERBEROS_DEBUG_DATA_VERSION 1

typedef struct _KERB_CLOUD_KERBEROS_DEBUG_DATA_V0
{
    unsigned int EnabledByPolicy : 1;
    unsigned int AsRepCallbackPresent : 1;
    unsigned int AsRepCallbackUsed : 1;
    unsigned int CloudReferralTgtAvailable : 1;
    unsigned int SpnOracleConfigured : 1;
    unsigned int KdcProxyPresent : 1;
} KERB_CLOUD_KERBEROS_DEBUG_DATA_V0, *PKERB_CLOUD_KERBEROS_DEBUG_DATA_V0;

typedef struct _KERB_CLOUD_KERBEROS_DEBUG_DATA
{
    unsigned int EnabledByPolicy : 1;
    unsigned int AsRepCallbackPresent : 1;
    unsigned int AsRepCallbackUsed : 1;
    unsigned int CloudReferralTgtAvailable : 1;
    unsigned int SpnOracleConfigured : 1;
    unsigned int KdcProxyPresent : 1;
    unsigned int PublicKeyCredsPresent : 1;
    unsigned int PasswordKeysPresent : 1;
    unsigned int PasswordPresent : 1;
    unsigned int AsRepSourceCred : 8;
} KERB_CLOUD_KERBEROS_DEBUG_DATA, *PKERB_CLOUD_KERBEROS_DEBUG_DATA;

#endif


//
// KerbChangePassword
//
// KerbChangePassword changes the password on the KDC account plus
//  the password cache and logon credentials if applicable.
//
//

typedef struct _KERB_CHANGEPASSWORD_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING DomainName;
    UNICODE_STRING AccountName;
    UNICODE_STRING OldPassword;
    UNICODE_STRING NewPassword;
    BOOLEAN        Impersonating;
} KERB_CHANGEPASSWORD_REQUEST, *PKERB_CHANGEPASSWORD_REQUEST;



//
// KerbSetPassword
//
// KerbSetPassword changes the password on the KDC account plus
//  the password cache and logon credentials if applicable.
//
//
   
typedef struct _KERB_SETPASSWORD_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    SecHandle CredentialsHandle;
    ULONG Flags;
    UNICODE_STRING DomainName;
    UNICODE_STRING AccountName;
    UNICODE_STRING Password;
} KERB_SETPASSWORD_REQUEST, *PKERB_SETPASSWORD_REQUEST;


typedef struct _KERB_SETPASSWORD_EX_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    SecHandle CredentialsHandle;
    ULONG Flags;
    UNICODE_STRING AccountRealm;
    UNICODE_STRING AccountName;
    UNICODE_STRING Password;
    UNICODE_STRING ClientRealm;
    UNICODE_STRING ClientName;
    BOOLEAN        Impersonating;
    UNICODE_STRING KdcAddress;
    ULONG          KdcAddressType;
 } KERB_SETPASSWORD_EX_REQUEST, *PKERB_SETPASSWORD_EX_REQUEST;


//
// KerbNlChangeMachinePassword
//
// KerbNlChangeMachinePassword creates a password bound by CredGuard
//  and changes the password on the KDC machine account plus
//  the password cache and logon credentials if applicable.
//
//

typedef struct _KERB_CHANGEMACHINEPASSWORD_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    BOOLEAN        ForcePasswordChange;
} KERB_CHANGEMACHINEPASSWORD_REQUEST, *PKERB_CHANGEMACHINEPASSWORD_REQUEST;

                                                                   
#define DS_UNKNOWN_ADDRESS_TYPE         0 // anything *but* IP
#define KERB_SETPASS_USE_LOGONID        1
#define KERB_SETPASS_USE_CREDHANDLE     2


typedef struct _KERB_DECRYPT_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID LogonId;
    ULONG Flags;
    LONG CryptoType;
    LONG KeyUsage;
    KERB_CRYPTO_KEY Key;        // optional
    ULONG EncryptedDataSize;
    ULONG InitialVectorSize;
    PUCHAR InitialVector;
    PUCHAR EncryptedData;
} KERB_DECRYPT_REQUEST, *PKERB_DECRYPT_REQUEST;

//
// If set, use the primary key from the current logon session of the one provided in the LogonId field.
// Otherwise, use the Key in the KERB_DECRYPT_MESSAGE.

#define KERB_DECRYPT_FLAG_DEFAULT_KEY   0x00000001


typedef struct _KERB_DECRYPT_RESPONSE  {
        UCHAR DecryptedData[ANYSIZE_ARRAY];
} KERB_DECRYPT_RESPONSE, *PKERB_DECRYPT_RESPONSE;


//
// Request structure for adding a binding cache entry. TCB privilege
// is required for this operation.
//

typedef struct _KERB_ADD_BINDING_CACHE_ENTRY_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING RealmName;
    UNICODE_STRING KdcAddress;
    ULONG AddressType;                  //dsgetdc.h DS_NETBIOS_ADDRESS||DS_INET_ADDRESS
} KERB_ADD_BINDING_CACHE_ENTRY_REQUEST, *PKERB_ADD_BINDING_CACHE_ENTRY_REQUEST;

                       
#if (_WIN32_WINNT >= 0x0502)
//
// Request structure for reacquiring smartcard credentials for a 
// given LUID.
// Requires TCB.
//
typedef struct _KERB_REFRESH_SCCRED_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING             CredentialBlob;   // optional
    LUID                       LogonId;
    ULONG                      Flags;
} KERB_REFRESH_SCCRED_REQUEST, *PKERB_REFRESH_SCCRED_REQUEST;

//
// Flags for KERB_REFRESH_SCCRED_REQUEST
//
//      KERB_REFRESH_SCCRED_RELEASE
//      Release the smartcard handle for LUID
//
//      KERB_REFRESH_SCCRED_GETTGT
//      Use the certificate hash in the blob to get a TGT for the logon 
//      session.
//

#define KERB_REFRESH_SCCRED_RELEASE             0x0  
#define KERB_REFRESH_SCCRED_GETTGT              0x1  
#endif

#if (_WIN32_WINNT != 0x0501)
//
// Request structure for adding extra Server credentials to a given
// logon session.  Only applicable during AcceptSecurityContext, and
// requires TCB to alter "other" creds
//

typedef struct _KERB_ADD_CREDENTIALS_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING UserName;
    UNICODE_STRING DomainName;
    UNICODE_STRING Password;
    LUID  LogonId; // optional
    ULONG Flags;
} KERB_ADD_CREDENTIALS_REQUEST, *PKERB_ADD_CREDENTIALS_REQUEST;


#define KERB_REQUEST_ADD_CREDENTIAL     1
#define KERB_REQUEST_REPLACE_CREDENTIAL 2
#define KERB_REQUEST_REMOVE_CREDENTIAL  4
#define KERB_REQUEST_CRED_LOCAL_ACCOUNT 8
#endif

#if (_WIN32_WINNT >= 0x0600)

typedef struct _KERB_ADD_CREDENTIALS_REQUEST_EX {
    KERB_ADD_CREDENTIALS_REQUEST Credentials;
    //
    // new for Ex
    // 
    ULONG PrincipalNameCount;
    UNICODE_STRING PrincipalNames[ANYSIZE_ARRAY];
} KERB_ADD_CREDENTIALS_REQUEST_EX, *PKERB_ADD_CREDENTIALS_REQUEST_EX;


#endif

//
// Request structure for transferring credentials between 2 luids.
// Requires TCB.
//

typedef struct _KERB_TRANSFER_CRED_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID                       OriginLogonId;
    LUID                       DestinationLogonId;
    ULONG                      Flags;
} KERB_TRANSFER_CRED_REQUEST, *PKERB_TRANSFER_CRED_REQUEST;

#define KERB_TRANSFER_CRED_WITH_TICKETS        0x1
#define KERB_TRANSFER_CRED_CLEANUP_CREDENTIALS 0x2

#if (_WIN32_WINNT >= 0x0602)

typedef struct _KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST
{
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    LUID                       LogonId;
} KERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST, *PKERB_CLEANUP_MACHINE_PKINIT_CREDS_REQUEST;

#endif

#if (_WIN32_WINNT >= 0x0602)

typedef struct _KERB_BINDING_CACHE_ENTRY_DATA {
    ULONG64        DiscoveryTime;
    UNICODE_STRING RealmName;
    UNICODE_STRING KdcAddress;
    ULONG          AddressType;
    ULONG          Flags;
    ULONG          DcFlags;
    ULONG          CacheFlags;
    UNICODE_STRING KdcName;
} KERB_BINDING_CACHE_ENTRY_DATA, *PKERB_BINDING_CACHE_ENTRY_DATA;

typedef struct _KERB_QUERY_BINDING_CACHE_RESPONSE {
    KERB_PROTOCOL_MESSAGE_TYPE     MessageType;
    ULONG                          CountOfEntries;
    PKERB_BINDING_CACHE_ENTRY_DATA Entries;
} KERB_QUERY_BINDING_CACHE_RESPONSE, *PKERB_QUERY_BINDING_CACHE_RESPONSE;

typedef struct _KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE MessageType;
    UNICODE_STRING             RealmName;
    UNICODE_STRING             KdcAddress;
    ULONG                      AddressType;   // DS_NETBIOS_ADDRESS / DS_INET_ADDRESS
    ULONG                      DcFlags;
} KERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST, *PKERB_ADD_BINDING_CACHE_ENTRY_EX_REQUEST;

typedef struct _KERB_QUERY_BINDING_CACHE_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
} KERB_QUERY_BINDING_CACHE_REQUEST, *PKERB_QUERY_BINDING_CACHE_REQUEST;

typedef struct _KERB_PURGE_BINDING_CACHE_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
} KERB_PURGE_BINDING_CACHE_REQUEST, *PKERB_PURGE_BINDING_CACHE_REQUEST;

typedef struct _KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST {
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       Flags;         // reserved, must be 0
    UNICODE_STRING              DomainName;
} KERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST, *PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_REQUEST;

#define KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE_FLAG_DAC_DISABLED  0x1

typedef struct _KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE {
    KERB_PROTOCOL_MESSAGE_TYPE  MessageType;
    ULONG                       Flags;
    ULONG                       ExtendedPolicies;
    ULONG                       DsFlags;
} KERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE, *PKERB_QUERY_DOMAIN_EXTENDED_POLICIES_RESPONSE;

#endif

//

#if (_WIN32_WINNT >= 0x0602)  

//
// Marshaled KERB_CERTIFICATE_INFO
//

typedef enum _KERB_CERTIFICATE_INFO_TYPE 
{
    CertHashInfo = 1,

} KERB_CERTIFICATE_INFO_TYPE, *PKERB_CERTIFICATE_INFO_TYPE;

typedef struct _KERB_CERTIFICATE_HASHINFO 
{
    USHORT StoreNameLength;   // length in bytes, if not 0, must inclue L'\0', if 0, default to L"MY"
    USHORT HashLength;        // length in bytes
    // payload starts, store name followed by cert hash

} KERB_CERTIFICATE_HASHINFO, *PKERB_CERTIFICATE_HASHINFO;

typedef struct _KERB_CERTIFICATE_INFO
{
    ULONG  CertInfoSize;      // size of this structure w/ payload
    ULONG  InfoType;          // info type, currently CertHashInfo
    // payload starts, marshaled structure of InfoType

} KERB_CERTIFICATE_INFO, *PKERB_CERTIFICATE_INFO;

#endif





typedef struct _POLICY_AUDIT_SID_ARRAY {

    ULONG UsersCount;
#ifdef MIDL_PASS
    [size_is(UsersCount)] PAUDIT_SID_RPC* UserSidArray;
#else
    PSID* UserSidArray;
#endif

} POLICY_AUDIT_SID_ARRAY, *PPOLICY_AUDIT_SID_ARRAY;

typedef struct _AUDIT_POLICY_INFORMATION {

    GUID    AuditSubCategoryGuid;
    ULONG   AuditingInformation;
    GUID    AuditCategoryGuid;

} AUDIT_POLICY_INFORMATION, *PAUDIT_POLICY_INFORMATION;

typedef const PAUDIT_POLICY_INFORMATION PCAUDIT_POLICY_INFORMATION, LPCAUDIT_POLICY_INFORMATION;

#define AUDIT_SET_SYSTEM_POLICY                 (0x0001)
#define AUDIT_QUERY_SYSTEM_POLICY               (0x0002)
#define AUDIT_SET_USER_POLICY                   (0x0004)
#define AUDIT_QUERY_USER_POLICY                 (0x0008)
#define AUDIT_ENUMERATE_USERS                   (0x0010)
#define AUDIT_SET_MISC_POLICY                   (0x0020)
#define AUDIT_QUERY_MISC_POLICY                 (0x0040)

#define AUDIT_GENERIC_ALL            (STANDARD_RIGHTS_REQUIRED  |\
                                      AUDIT_SET_SYSTEM_POLICY   |\
                                      AUDIT_QUERY_SYSTEM_POLICY |\
                                      AUDIT_SET_USER_POLICY     |\
                                      AUDIT_QUERY_USER_POLICY   |\
                                      AUDIT_ENUMERATE_USERS     |\
                                      AUDIT_SET_MISC_POLICY     |\
                                      AUDIT_QUERY_MISC_POLICY)

#define AUDIT_GENERIC_READ           (STANDARD_RIGHTS_READ      |\
                                      AUDIT_QUERY_SYSTEM_POLICY |\
                                      AUDIT_QUERY_USER_POLICY   |\
                                      AUDIT_ENUMERATE_USERS     |\
                                      AUDIT_QUERY_MISC_POLICY)

#define AUDIT_GENERIC_WRITE          (STANDARD_RIGHTS_WRITE     |\
                                      AUDIT_SET_USER_POLICY     |\
                                      AUDIT_SET_MISC_POLICY     |\
                                      AUDIT_SET_SYSTEM_POLICY)

#define AUDIT_GENERIC_EXECUTE        (STANDARD_RIGHTS_EXECUTE)


BOOLEAN
NTAPI
AuditSetSystemPolicy(
    _In_reads_(dwPolicyCount) PCAUDIT_POLICY_INFORMATION pAuditPolicy,
    _In_ ULONG dwPolicyCount
    );

BOOLEAN
NTAPI
AuditSetPerUserPolicy(
    _In_ const PSID pSid,
    _In_reads_(dwPolicyCount) PCAUDIT_POLICY_INFORMATION pAuditPolicy,
    _In_ ULONG dwPolicyCount
    );

_Check_return_
BOOLEAN
NTAPI
AuditQuerySystemPolicy(
    _In_reads_(dwPolicyCount) const GUID* pSubCategoryGuids,
    _In_ ULONG dwPolicyCount,
    _Outptr_result_buffer_(dwPolicyCount)
    _When_(return != 0, __drv_allocatesMem(Mem)) PAUDIT_POLICY_INFORMATION* ppAuditPolicy
    );

_Check_return_
BOOLEAN
NTAPI
AuditQueryPerUserPolicy(
    _In_ const PSID pSid,
    _In_reads_(dwPolicyCount) const GUID* pSubCategoryGuids,
    _In_ ULONG dwPolicyCount,
    _Outptr_result_buffer_(dwPolicyCount)
    _When_(return != 0, __drv_allocatesMem(Mem)) PAUDIT_POLICY_INFORMATION* ppAuditPolicy
    );

_Check_return_
BOOLEAN
NTAPI
AuditEnumeratePerUserPolicy(
    _Out_ _When_(return != 0, __drv_allocatesMem(Mem)) PPOLICY_AUDIT_SID_ARRAY* ppAuditSidArray
    );

_Check_return_
BOOLEAN
NTAPI
AuditComputeEffectivePolicyBySid(
    _In_ const PSID pSid,
    _In_reads_(dwPolicyCount) const GUID* pSubCategoryGuids,
    _In_ ULONG dwPolicyCount,
    _Outptr_result_buffer_(dwPolicyCount)
    _When_(return != 0, __drv_allocatesMem(Mem)) PAUDIT_POLICY_INFORMATION* ppAuditPolicy
    );

_Check_return_
BOOLEAN
NTAPI
AuditComputeEffectivePolicyByToken(
    _In_ HANDLE hTokenHandle,
    _In_reads_(dwPolicyCount) const GUID* pSubCategoryGuids,
    _In_ ULONG dwPolicyCount,
    _Outptr_result_buffer_(dwPolicyCount)
    _When_(return != 0, __drv_allocatesMem(Mem)) PAUDIT_POLICY_INFORMATION* ppAuditPolicy
    );

_Check_return_
BOOLEAN
NTAPI
AuditEnumerateCategories(
    _Outptr_result_buffer_(*pdwCountReturned)
    _When_(return != 0, __drv_allocatesMem(Mem)) GUID** ppAuditCategoriesArray,
    _Out_ PULONG pdwCountReturned
    );

_Check_return_
BOOLEAN
NTAPI
AuditEnumerateSubCategories(
    _In_opt_ const GUID* pAuditCategoryGuid,
    _In_ BOOLEAN bRetrieveAllSubCategories,
    _Outptr_result_buffer_(*pdwCountReturned)
    _When_(return != 0, __drv_allocatesMem(Mem)) GUID** ppAuditSubCategoriesArray,
    _Out_ PULONG pdwCountReturned
    );

_Check_return_
BOOLEAN
NTAPI
AuditLookupCategoryNameW(
    _In_ const GUID* pAuditCategoryGuid,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PWSTR* ppszCategoryName
    );

_Check_return_
BOOLEAN
NTAPI
AuditLookupCategoryNameA(
    _In_ const GUID* pAuditCategoryGuid,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PSTR* ppszCategoryName
    );

#ifdef UNICODE
#define AuditLookupCategoryName  AuditLookupCategoryNameW
#else
#define AuditLookupCategoryName  AuditLookupCategoryNameA
#endif

_Check_return_
BOOLEAN
NTAPI
AuditLookupSubCategoryNameW(
    _In_ const GUID* pAuditSubCategoryGuid,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PWSTR* ppszSubCategoryName
    );

_Check_return_
BOOLEAN
NTAPI
AuditLookupSubCategoryNameA(
    _In_ const GUID* pAuditSubCategoryGuid,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PSTR* ppszSubCategoryName
    );

#ifdef UNICODE
#define AuditLookupSubCategoryName  AuditLookupSubCategoryNameW
#else
#define AuditLookupSubCategoryName  AuditLookupSubCategoryNameA
#endif

BOOLEAN
NTAPI
AuditLookupCategoryIdFromCategoryGuid(
    _In_ const GUID* pAuditCategoryGuid,
    _Out_ PPOLICY_AUDIT_EVENT_TYPE pAuditCategoryId
    );

BOOLEAN
NTAPI
AuditLookupCategoryGuidFromCategoryId(
    _In_ POLICY_AUDIT_EVENT_TYPE AuditCategoryId,
    _Out_ GUID* pAuditCategoryGuid
    );

BOOLEAN
NTAPI
AuditSetSecurity(
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

_Check_return_
BOOLEAN
NTAPI
AuditQuerySecurity(
    _In_ SECURITY_INFORMATION SecurityInformation,
    _Outptr_ _When_(return != 0, __drv_allocatesMem(Mem)) PSECURITY_DESCRIPTOR *ppSecurityDescriptor
    );

BOOLEAN
NTAPI
AuditSetGlobalSaclW(
    _In_ PCWSTR ObjectTypeName,
    _In_opt_ PACL Acl
    );

BOOLEAN
NTAPI
AuditSetGlobalSaclA(
    _In_ PCSTR ObjectTypeName,
    _In_opt_ PACL Acl
    );

#ifdef UNICODE
#define AuditSetGlobalSacl AuditSetGlobalSaclW
#else
#define AuditSetGlobalSacl AuditSetGlobalSaclA
#endif

_Check_return_
BOOLEAN
NTAPI
AuditQueryGlobalSaclW(
    _In_ PCWSTR ObjectTypeName,
    _Out_ _When_(return != 0, __drv_allocatesMem(Mem)) PACL *Acl
    );

_Check_return_
BOOLEAN
NTAPI
AuditQueryGlobalSaclA(
    _In_ PCSTR ObjectTypeName,
    _Out_ _When_(return != 0, __drv_allocatesMem(Mem)) PACL *Acl
    );

#ifdef UNICODE
#define AuditQueryGlobalSacl AuditQueryGlobalSaclW
#else
#define AuditQueryGlobalSacl AuditQueryGlobalSaclA
#endif

VOID
NTAPI
AuditFree(
    _In_ __drv_freesMem(Mem) _Post_invalid_ PVOID Buffer
    );


#if (_WIN32_WINNT >= 0x0601)

//
// Pku2u package name
//
               
#define PKU2U_PACKAGE_NAME_A          "pku2u"
#define PKU2U_PACKAGE_NAME            L"pku2u"
#define PKU2U_PACKAGE_NAME_W          PKU2U_PACKAGE_NAME

#endif // _WIN32_WINNT


#if (_WIN32_WINNT >= 0x0601)

// the following structure contains the ASN.1 encoded X.509 certificate 
typedef struct _PKU2U_CERT_BLOB {
    ULONG CertOffset; // each element is a byte 
    USHORT CertLength; // 
} PKU2U_CERT_BLOB, *PPKU2U_CERT_BLOB;

#define PKU2U_CREDUI_CONTEXT_VERSION  0x4154414454524543i64  // "CERTDATA"

typedef struct _PKU2U_CREDUI_CONTEXT {
    ULONG64 Version;
    USHORT cbHeaderLength;
    ULONG cbStructureLength;
    USHORT CertArrayCount; // followed by an array of PKU2U_CERT_BLOB
    ULONG CertArrayOffset; // offset to the array    
} PKU2U_CREDUI_CONTEXT, *PPKU2U_CREDUI_CONTEXT;

/////////////////////////////////////////////////////////////////////////
//
// LsaLogonUser parameters
//
/////////////////////////////////////////////////////////////////////////

typedef enum _PKU2U_LOGON_SUBMIT_TYPE {
    Pku2uCertificateS4ULogon = 14,
} PKU2U_LOGON_SUBMIT_TYPE, *PPKU2U_LOGON_SUBMIT_TYPE;

typedef struct _PKU2U_CERTIFICATE_S4U_LOGON {
    PKU2U_LOGON_SUBMIT_TYPE MessageType;
    ULONG Flags; 
    UNICODE_STRING UserPrincipalName; 
    _Reserved_ UNICODE_STRING DomainName; 
    ULONG CertificateLength;   // for the client certificate 
    _Field_size_bytes_(CertificateLength) PUCHAR Certificate; // for the client certificate, BER encoded
} PKU2U_CERTIFICATE_S4U_LOGON, *PPKU2U_CERTIFICATE_S4U_LOGON;

#endif // _WIN32_WINNT


#ifdef __cplusplus
}
#endif

#endif /* _NTSECAPI_ */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
