/*++

Copyright (c) 2000 Microsoft Corporation

Module Name:

    authz.h

Abstract:

    This module contains the authorization framework APIs and any public data
    structures needed to call these APIs.

Revision History:

    Created - March 2000

--*/

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#endif

#ifndef __AUTHZ_H__
#define __AUTHZ_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

#if !defined(_AUTHZ_)
#define AUTHZAPI DECLSPEC_IMPORT
#else 
#define AUTHZAPI
#endif

#ifndef MIDL_PASS
#include <windows.h>
#endif
#include <adtgen.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

//
// Flags which may be used at the time of client context creation using a sid.
//

#define AUTHZ_SKIP_TOKEN_GROUPS  0x2
#define AUTHZ_REQUIRE_S4U_LOGON  0x4
#define AUTHZ_COMPUTE_PRIVILEGES 0x8
              
DECLARE_HANDLE(AUTHZ_ACCESS_CHECK_RESULTS_HANDLE);
DECLARE_HANDLE(AUTHZ_CLIENT_CONTEXT_HANDLE);
DECLARE_HANDLE(AUTHZ_RESOURCE_MANAGER_HANDLE);
DECLARE_HANDLE(AUTHZ_AUDIT_EVENT_HANDLE);
DECLARE_HANDLE(AUTHZ_AUDIT_EVENT_TYPE_HANDLE);
DECLARE_HANDLE(AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE);

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
DECLARE_HANDLE(AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE);
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

typedef AUTHZ_ACCESS_CHECK_RESULTS_HANDLE    *PAUTHZ_ACCESS_CHECK_RESULTS_HANDLE;
typedef AUTHZ_CLIENT_CONTEXT_HANDLE          *PAUTHZ_CLIENT_CONTEXT_HANDLE;
typedef AUTHZ_RESOURCE_MANAGER_HANDLE        *PAUTHZ_RESOURCE_MANAGER_HANDLE;
typedef AUTHZ_AUDIT_EVENT_HANDLE             *PAUTHZ_AUDIT_EVENT_HANDLE;
typedef AUTHZ_AUDIT_EVENT_TYPE_HANDLE        *PAUTHZ_AUDIT_EVENT_TYPE_HANDLE;
typedef AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE *PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE;

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
typedef AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE *PAUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE;
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//
// Structure defining the access check request.
//

typedef struct _AUTHZ_ACCESS_REQUEST
{
    ACCESS_MASK DesiredAccess;

    //
    // To replace the principal self sid in the acl.
    //

    PSID PrincipalSelfSid;

    //
    // Object type list represented by an array of (level, guid) pair and the
    // number of elements in the array. This is a post-fix representation of the
    // object tree.
    // These fields should be set to NULL and 0 respectively except when per
    // property access is desired.
    //

    POBJECT_TYPE_LIST ObjectTypeList;
    DWORD ObjectTypeListLength;

    //
    // To support completely business rules based access. This will be passed as
    // input to the callback access check function. Access check algorithm does
    // not interpret these.
    //

    PVOID OptionalArguments;
    
} AUTHZ_ACCESS_REQUEST, *PAUTHZ_ACCESS_REQUEST;

//
// Structure to return the results of the access check call.
//

typedef struct _AUTHZ_ACCESS_REPLY
{
    //
    // The length of the array representing the object type list structure. If
    // no object type is used to represent the object, then the length must be
    // set to 1.
    //
    // Note: This parameter must be filled!
    //

    DWORD ResultListLength;

    //
    // Array of granted access masks. This memory is allocated by the RM. Access
    // check routines just fill in the values.
    //

    _Field_size_(ResultListLength) PACCESS_MASK GrantedAccessMask;
    
    //
    // Array of SACL evaluation results.  This memory is allocated by the RM, if SACL
    // evaluation results are desired. Access check routines just fill in the values.
    // Sacl evaluation will only be performed if auditing is requested.
    //
    
#define AUTHZ_GENERATE_SUCCESS_AUDIT 0x1
#define AUTHZ_GENERATE_FAILURE_AUDIT 0x2

    _Field_size_opt_(ResultListLength) PDWORD SaclEvaluationResults;
    
    //
    // Array of results for each element of the array. This memory is allocated
    // by the RM. Access check routines just fill in the values.
    //

    _Field_size_(ResultListLength) PDWORD Error;

} AUTHZ_ACCESS_REPLY, *PAUTHZ_ACCESS_REPLY;

//
// Typedefs for callback functions to be provided by the resource manager.
//

//
// Callback access check function takes in
//     AuthzClientContext - a client context
//     pAce - pointer to a callback ace
//     pArgs - Optional arguments that were passed to AuthzAccessCheck thru
//             AuthzAccessRequest->OptionalArguments are passed back here.
//     pbAceApplicable - The resource manager must supply whether the ace should
//         be used in the computation of access evaluation
//
// Returns
//     TRUE if the API succeeded.
//     FALSE on any intermediate errors (like failed memory allocation)
//         In case of failure, the caller must use SetLastError(ErrorValue).
//

typedef BOOL (CALLBACK *PFN_AUTHZ_DYNAMIC_ACCESS_CHECK) (
                  _In_ AUTHZ_CLIENT_CONTEXT_HANDLE hAuthzClientContext,
                  _In_ PACE_HEADER                 pAce,
                  _In_opt_ PVOID                   pArgs,
                  _Inout_  PBOOL                   pbAceApplicable
                  );

//
// Callback compute dynamic groups function takes in
//     AuthzClientContext - a client context
//     pArgs - Optional arguments that supplied to AuthzInitializeClientContext*
//         thru DynamicGroupArgs are passed back here..
//     pSidAttrArray - To allocate and return an array of (sids, attribute)
//         pairs to be added to the normal part of the client context.
//     pSidCount - Number of elements in pSidAttrArray
//     pRestrictedSidAttrArray - To allocate and return an array of (sids, attribute)
//         pairs to be added to the restricted part of the client context.
//     pRestrictedSidCount - Number of elements in pRestrictedSidAttrArray
//
// Note:
//    Memory returned thru both these array will be freed by the callback
//    free function defined by the resource manager.
//
// Returns
//     TRUE if the API succeeded.
//     FALSE on any intermediate errors (like failed memory allocation)
//         In case of failure, the caller must use SetLastError(ErrorValue).
//

typedef BOOL (CALLBACK *PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS) (
                  _In_  AUTHZ_CLIENT_CONTEXT_HANDLE hAuthzClientContext,
                  _In_  PVOID                       Args,
                  _Out_ PSID_AND_ATTRIBUTES         *pSidAttrArray,
                  _Out_ PDWORD                      pSidCount,
                  _Out_ PSID_AND_ATTRIBUTES         *pRestrictedSidAttrArray,
                  _Out_ PDWORD                      pRestrictedSidCount
                  );

//
// Callback free function takes in
//     pSidAttrArray - To be freed. This has been allocated by the compute
//     dynamic groups function.
//

typedef VOID (CALLBACK *PFN_AUTHZ_FREE_DYNAMIC_GROUPS) (
                  _In_ PSID_AND_ATTRIBUTES pSidAttrArray
                  );

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
//
// Callback central access policy retrieval function takes in
//     AuthzClientContext - a client context
//     capid - CAPID of the central access policy to retrieve.
//     pArgs - Optional arguments that were passed to AuthzAccessCheck through
//         AuthzAccessRequest->OptionalArguments are passed back here.
//     pCentralAccessPolicyApplicable - The resource manager must indicate 
//         whether a central access policy should be used in access evaluation.
//     ppCentralAccessPolicy - Pointer to the CAP to be used in the 
//         computation of access evaluation. If NULL, the default CAP is applied.
//
// Returns
//     TRUE if the API succeeded.
//     FALSE on any intermediate errors (like failed memory allocation)
//         In case of failure, the caller must use SetLastError(ErrorValue).
//

typedef BOOL (CALLBACK *PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY) (
                _In_     AUTHZ_CLIENT_CONTEXT_HANDLE  hAuthzClientContext,
                _In_     PSID                         capid,
                _In_opt_ PVOID                        pArgs,
                _Out_    PBOOL                        pCentralAccessPolicyApplicable,
                _Out_    PVOID                       *ppCentralAccessPolicy
                );

//
// Callback central access policy free function takes in
//     pCentralAccessPolicy - To be freed. This memory has been allocated by 
//     the central access policy retrieval callback function.
//

typedef VOID (CALLBACK *PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY) (
                  _In_ PVOID pCentralAccessPolicy
                  );
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//
//  BEGIN: *** (Subject) Security attributes ***
//
//  WARNING!!!:
//      These #defines and data structures exactly mirror
//      the TOKEN_XXX definitions in ntseapi.w. Keep them
//      in sync.
//


//
//  Security attribute data types ...
//

#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID   0x00

#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64     0x01
#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64    0x02

//
//  Case insensitive attribute value string by default.
//  Unless the flag AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE
//  is set indicating otherwise.
//

#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING    0x03

//
//  Fully-qualified binary name.
//

typedef struct _AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    ULONG64 Version;
    PWSTR   pName;
} AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE, *PAUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE;

#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN      0x04

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID       0x05

#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN   0x06
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//
//  This is the 'catch all' type. The attribute manipulation
//  code really doesn't care about the actual format of the
//  value. Value subtypes are defined only for this type.
//  Value subtypes permit easy addition of new subtypes
//  without having to change the attribute manipulation
//  (and WOW64 thunking!) code.
//

typedef struct _AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    PVOID   pValue;         //  Pointer is BYTE aligned.
    ULONG   ValueLength;    //  In bytes
} AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
    *PAUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;

#define AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING  0x10

//
//  Attribute operations that can be specified for a 'set' API:
//


typedef enum {

    //
    //  No-op
    //

    AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE = 0,

    //
    //  Delete all existing security attributes and their values in
    //  the NT token and replace it with the specified attributes/values.
    //  If attributes to replace with are not specified, all existing
    //  attributes and values are deleted.
    //
    //  This operation can be specified at most once and must be the
    //  only operation specified.
    //

    AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL,

    //
    //  Add a new attribute or a new value to an existing attribute.
    //  If the value specified for any attribute already exists for
    //  that attribute, the call fails.
    //

    AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD,

    //
    //  Delete the specified value(s) of the specified attribute(s).
    //  If the last value is deleted from an attribute, the attribute
    //  itself is removed. If no matching attribute name was found, no
    //  modifications are done and the call fails. If no value is specified
    //  for the attribute, the attribute itself will be deleted.
    //

    AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE,

    //
    //  The value(s) of the specified security attribute(s) completely
    //  replace(s) the existing value(s) of the attribute(s). If the
    //  attribute does not already exist, it is added.  When no value
    //  is specified, the attribute is deleted, if it exists; otherwise,
    //  the operation is simply ignored and no failure is reported.
    //

    AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE
} AUTHZ_SECURITY_ATTRIBUTE_OPERATION, *PAUTHZ_SECURITY_ATTRIBUTE_OPERATION;

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
//
//  SID operations that can be specified for a 'set' API:
//

typedef enum {

    //
    //  No-op
    //

    AUTHZ_SID_OPERATION_NONE = 0,

    //
    //  Delete all existing SIDs in the NT token and replace them with 
    //  the specified SIDs.
    //  If the SIDs to replace with are not specified, all existing
    //  SIDs are deleted.
    //
    //  This operation can be specified at most once and must be the
    //  only operation specified.
    //

    AUTHZ_SID_OPERATION_REPLACE_ALL,

    //
    //  Add a new SID.
    //  If the SID specified already exists, the call fails.
    //

    AUTHZ_SID_OPERATION_ADD,

    //
    //  Delete the specified SID(s).
    //  If no matching SID was found, no modifications are done and 
    //  the call fails.
    //

    AUTHZ_SID_OPERATION_DELETE,

    //
    //  The specified SID(s) completely replace(s) the existing SID(s). 
    //  If the SID does not already exist, it is added.
    //

    AUTHZ_SID_OPERATION_REPLACE
} AUTHZ_SID_OPERATION, *PAUTHZ_SID_OPERATION;
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//
//  An individual security attribute.
//

typedef struct _AUTHZ_SECURITY_ATTRIBUTE_V1 {

    //
    //  Name of the attribute.
    //  Case insensitive Windows Unicode string.
    //

    PWSTR   pName;

    //
    //  Data type of attribute.
    //

    USHORT  ValueType;

    //
    //  Pass 0 in a set operation and check for 0 in
    //  a get operation.
    //

    USHORT  Reserved;

//
//  Attribute must not be inherited across process spawns.
//

#define AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE      0x0001


//
//  Attribute value is compared in a case sensitive way. It is valid with string value
//  or composite type containing string value. For other types of value, this flag
//  will be ignored. Currently, it is valid with the two types:
//  AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING and AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN.
//
#define AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE         0x0002

#define AUTHZ_SECURITY_ATTRIBUTE_VALID_FLAGS   (    \
                        AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE |  \
                        AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE     \
                        )

    ULONG   Flags;

    //
    //  Number of values.
    //

    ULONG   ValueCount;

    //
    //  The actual value itself.
    //
    union {
        PLONG64                                         pInt64;
        PULONG64                                        pUint64;
        PWSTR                                           *ppString;
        PAUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE            pFqbn;
        PAUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE    pOctetString;
    } Values;
} AUTHZ_SECURITY_ATTRIBUTE_V1, *PAUTHZ_SECURITY_ATTRIBUTE_V1;


//
//  Set of security attributes.
//

typedef struct _AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {

    //
    //  Versioning. The interpretation of the pointers in the
    //  Attribute field below is dependent on the version field.
    //
    //  Get operations return the version while the set operation
    //  MUST specify the version of the data structure passed in.
    //

#define AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1    1

#define AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION       \
    AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1

    //
    //  MUST BE first.
    //

    USHORT  Version;

    //
    //  Pass 0 in set operations and ignore on get operations.
    //

    USHORT  Reserved;

    ULONG   AttributeCount;

    union {
        PAUTHZ_SECURITY_ATTRIBUTE_V1    pAttributeV1;
    } Attribute;
} AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
    *PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION;




//
// Valid flags for AuthzAccessCheck
//

#define AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD 0x00000001

AUTHZAPI
BOOL
WINAPI
AuthzAccessCheck(
    _In_      DWORD                              Flags,
    _In_      AUTHZ_CLIENT_CONTEXT_HANDLE        hAuthzClientContext,
    _In_      PAUTHZ_ACCESS_REQUEST              pRequest,
    _In_opt_  AUTHZ_AUDIT_EVENT_HANDLE           hAuditEvent,
    _In_      PSECURITY_DESCRIPTOR               pSecurityDescriptor,
    _In_reads_opt_(OptionalSecurityDescriptorCount)
              PSECURITY_DESCRIPTOR               *OptionalSecurityDescriptorArray,
    _In_      DWORD                              OptionalSecurityDescriptorCount,
    _Inout_   PAUTHZ_ACCESS_REPLY                pReply,
    _Out_opt_ PAUTHZ_ACCESS_CHECK_RESULTS_HANDLE phAccessCheckResults
    );

AUTHZAPI
BOOL
WINAPI
AuthzCachedAccessCheck(
    _In_     DWORD                             Flags,
    _In_     AUTHZ_ACCESS_CHECK_RESULTS_HANDLE hAccessCheckResults,
    _In_     PAUTHZ_ACCESS_REQUEST             pRequest,
    _In_opt_ AUTHZ_AUDIT_EVENT_HANDLE          hAuditEvent,
     _Inout_ PAUTHZ_ACCESS_REPLY               pReply
    );

AUTHZAPI
BOOL
WINAPI
AuthzOpenObjectAudit(
    _In_     DWORD                       Flags,
    _In_     AUTHZ_CLIENT_CONTEXT_HANDLE hAuthzClientContext,
    _In_     PAUTHZ_ACCESS_REQUEST       pRequest,
    _In_     AUTHZ_AUDIT_EVENT_HANDLE    hAuditEvent,
    _In_     PSECURITY_DESCRIPTOR        pSecurityDescriptor,
    _In_reads_opt_(OptionalSecurityDescriptorCount)
             PSECURITY_DESCRIPTOR        *OptionalSecurityDescriptorArray OPTIONAL,
    _In_     DWORD                       OptionalSecurityDescriptorCount,
    _In_     PAUTHZ_ACCESS_REPLY         pReply
    );

AUTHZAPI
BOOL
WINAPI
AuthzFreeHandle(
    _Inout_ AUTHZ_ACCESS_CHECK_RESULTS_HANDLE hAccessCheckResults
    );

//
// Flags for AuthzInitializeResourceManager and AuthzInitializeResourceManagerEx
//

#define AUTHZ_RM_FLAG_NO_AUDIT 0x1
#define AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION 0x2
#define AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES 0x4
#define AUTHZ_VALID_RM_INIT_FLAGS (AUTHZ_RM_FLAG_NO_AUDIT | \
        AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION | \
        AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES)

AUTHZAPI
BOOL
WINAPI
AuthzInitializeResourceManager(
    _In_     DWORD                            Flags,
    _In_opt_ PFN_AUTHZ_DYNAMIC_ACCESS_CHECK   pfnDynamicAccessCheck,
    _In_opt_ PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS pfnComputeDynamicGroups,
    _In_opt_ PFN_AUTHZ_FREE_DYNAMIC_GROUPS    pfnFreeDynamicGroups,
    _In_opt_ PCWSTR                           szResourceManagerName,
    _Out_    PAUTHZ_RESOURCE_MANAGER_HANDLE   phAuthzResourceManager
    );

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#define AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1 1

typedef struct _AUTHZ_RPC_INIT_INFO_CLIENT
{
    USHORT                             version;
    PWSTR                              ObjectUuid;
    PWSTR                              ProtSeq;
    PWSTR                              NetworkAddr;
    PWSTR                              Endpoint;
    PWSTR                              Options;
    PWSTR                              ServerSpn;
} AUTHZ_RPC_INIT_INFO_CLIENT, *PAUTHZ_RPC_INIT_INFO_CLIENT;

//
// Versioning enables future updates of authz resource manager initialization 
// info structure.
//
#define AUTHZ_INIT_INFO_VERSION_V1 1

//
// Authz resource manager initialization info structure.
//
// version - authz resource manager initialization info structure version. 
//
// szResourceManagerName - the name of the resource manager.
//
// pfnDynamicAccessCheck - Pointer to the RM supplied access check function to be
// called when a callback ACE is encountered by the access check algorithm.
//
// pfnComputeDynamicGroups - Pointer to the RM supplied function to compute
// groups to be added to the client context at the time of its creation.
//
// pfnFreeDynamicGroups - Pointer to the function to free the memory allocated
// by the pfnComputeDynamicGroups function.
//
// pfnGetCentralAccessPolicy - Pointer to the function to be called when 
// a CAPID ACE is encountered by the access check algorithm.
//
// pfnFreeCentralAccessPolicy - Pointer to the function to free the memory allocated
// by the pfnGetCentralAccessPolicy function.
//

typedef struct _AUTHZ_INIT_INFO
{
    USHORT                                  version;
    PCWSTR                                  szResourceManagerName;
    PFN_AUTHZ_DYNAMIC_ACCESS_CHECK          pfnDynamicAccessCheck;
    PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS        pfnComputeDynamicGroups;
    PFN_AUTHZ_FREE_DYNAMIC_GROUPS           pfnFreeDynamicGroups;
    PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY     pfnGetCentralAccessPolicy;
    PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY    pfnFreeCentralAccessPolicy;
} AUTHZ_INIT_INFO, *PAUTHZ_INIT_INFO;

AUTHZAPI
BOOL
WINAPI
AuthzInitializeResourceManagerEx(
    _In_opt_ DWORD                          Flags,
    _In_opt_ PAUTHZ_INIT_INFO               pAuthzInitInfo,
    _Out_    PAUTHZ_RESOURCE_MANAGER_HANDLE phAuthzResourceManager
    );



AUTHZAPI
BOOL
WINAPI
AuthzInitializeRemoteResourceManager(
    _In_  PAUTHZ_RPC_INIT_INFO_CLIENT    pRpcInitInfo,
    _Out_ PAUTHZ_RESOURCE_MANAGER_HANDLE phAuthzResourceManager
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

AUTHZAPI
BOOL
WINAPI
AuthzFreeResourceManager(
    _In_ AUTHZ_RESOURCE_MANAGER_HANDLE hAuthzResourceManager
    );

AUTHZAPI
BOOL
WINAPI
AuthzInitializeContextFromToken(
    _In_      DWORD                         Flags,
    _In_      HANDLE                        TokenHandle,
    _In_      AUTHZ_RESOURCE_MANAGER_HANDLE hAuthzResourceManager,
    _In_opt_  PLARGE_INTEGER                pExpirationTime,
    _In_      LUID                          Identifier,
    _In_opt_  PVOID                         DynamicGroupArgs,
    _Out_     PAUTHZ_CLIENT_CONTEXT_HANDLE  phAuthzClientContext
    );

AUTHZAPI
BOOL
WINAPI
AuthzInitializeContextFromSid(
    _In_      DWORD                         Flags,
    _In_      PSID                          UserSid,
    _In_      AUTHZ_RESOURCE_MANAGER_HANDLE hAuthzResourceManager,
    _In_opt_  PLARGE_INTEGER                pExpirationTime,
    _In_      LUID                          Identifier,
    _In_opt_  PVOID                         DynamicGroupArgs,
    _Out_     PAUTHZ_CLIENT_CONTEXT_HANDLE  phAuthzClientContext
    );

AUTHZAPI
BOOL
WINAPI
AuthzInitializeContextFromAuthzContext(
    _In_      DWORD                        Flags,
    _In_      AUTHZ_CLIENT_CONTEXT_HANDLE  hAuthzClientContext,
    _In_opt_  PLARGE_INTEGER               pExpirationTime,
    _In_      LUID                         Identifier,
    _In_      PVOID                        DynamicGroupArgs,
    _Out_     PAUTHZ_CLIENT_CONTEXT_HANDLE phNewAuthzClientContext
    );

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)


AUTHZAPI
BOOL
WINAPI
AuthzInitializeCompoundContext(
    _In_ AUTHZ_CLIENT_CONTEXT_HANDLE UserContext,
    _In_ AUTHZ_CLIENT_CONTEXT_HANDLE DeviceContext,
    _Out_ PAUTHZ_CLIENT_CONTEXT_HANDLE phCompoundContext
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

AUTHZAPI
BOOL
WINAPI
AuthzAddSidsToContext(
    _In_      AUTHZ_CLIENT_CONTEXT_HANDLE  hAuthzClientContext,
    _In_opt_  PSID_AND_ATTRIBUTES          Sids,
    _In_      DWORD                        SidCount,
    _In_opt_  PSID_AND_ATTRIBUTES          RestrictedSids,
    _In_      DWORD                        RestrictedSidCount,
    _Out_     PAUTHZ_CLIENT_CONTEXT_HANDLE phNewAuthzClientContext
    );

//
//  API to modify security attributes in AUTHZ client context.
//

AUTHZAPI
BOOL
WINAPI
AuthzModifySecurityAttributes(
    _In_     AUTHZ_CLIENT_CONTEXT_HANDLE             hAuthzClientContext,
    _In_
    _When_(pAttributes != NULL && *pOperations != AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL, _In_reads_(pAttributes->AttributeCount))
             PAUTHZ_SECURITY_ATTRIBUTE_OPERATION     pOperations,
    _In_opt_ PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION  pAttributes
    );

//
// Enumeration type to be used to specify the type of information to be
// retrieved from an existing AuthzClientContext.
//

typedef enum _AUTHZ_CONTEXT_INFORMATION_CLASS
{
    AuthzContextInfoUserSid = 1,
    AuthzContextInfoGroupsSids,
    AuthzContextInfoRestrictedSids,
    AuthzContextInfoPrivileges,
    AuthzContextInfoExpirationTime,
    AuthzContextInfoServerContext,
    AuthzContextInfoIdentifier,
    AuthzContextInfoSource,
    AuthzContextInfoAll,
    AuthzContextInfoAuthenticationId,
    AuthzContextInfoSecurityAttributes,
    AuthzContextInfoDeviceSids,
    AuthzContextInfoUserClaims,
    AuthzContextInfoDeviceClaims,
    AuthzContextInfoAppContainerSid,
    AuthzContextInfoCapabilitySids
} AUTHZ_CONTEXT_INFORMATION_CLASS;

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
AUTHZAPI
BOOL
WINAPI
AuthzModifyClaims(
    _In_     AUTHZ_CLIENT_CONTEXT_HANDLE            hAuthzClientContext,
    _In_     AUTHZ_CONTEXT_INFORMATION_CLASS        ClaimClass,
    _In_
    _When_(pClaims != NULL && *pClaimOperations != AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL, _In_reads_(pClaims->AttributeCount))
             PAUTHZ_SECURITY_ATTRIBUTE_OPERATION    pClaimOperations,
    _In_opt_ PAUTHZ_SECURITY_ATTRIBUTES_INFORMATION pClaims
    );

AUTHZAPI
BOOL
WINAPI
AuthzModifySids(
    _In_     AUTHZ_CLIENT_CONTEXT_HANDLE            hAuthzClientContext,
    _In_     AUTHZ_CONTEXT_INFORMATION_CLASS        SidClass,
    _In_
    _When_(pSids != NULL && *pSidOperations != AUTHZ_SID_OPERATION_REPLACE_ALL, _In_reads_(pSids->GroupCount))
             PAUTHZ_SID_OPERATION                   pSidOperations,
    _In_opt_ PTOKEN_GROUPS                          pSids
    );

AUTHZAPI
BOOL
WINAPI
AuthzSetAppContainerInformation(
    _In_     AUTHZ_CLIENT_CONTEXT_HANDLE            hAuthzClientContext,
    _In_     PSID                                   pAppContainerSid,
    _In_     DWORD                                  CapabilityCount,
    _In_reads_opt_(CapabilityCount)
             PSID_AND_ATTRIBUTES                    pCapabilitySids
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

AUTHZAPI
BOOL
WINAPI
AuthzGetInformationFromContext(
    _In_  AUTHZ_CLIENT_CONTEXT_HANDLE     hAuthzClientContext,
    _In_  AUTHZ_CONTEXT_INFORMATION_CLASS InfoClass,
    _In_  DWORD                           BufferSize,
    _Out_ PDWORD                          pSizeRequired,
    _Out_ PVOID                           Buffer
);

AUTHZAPI
BOOL
WINAPI
AuthzFreeContext(
    _In_ AUTHZ_CLIENT_CONTEXT_HANDLE hAuthzClientContext
    );


//
// Valid flags that may be used in AuthzInitializeObjectAccessAuditEvent().
//

#define AUTHZ_NO_SUCCESS_AUDIT                   0x00000001
#define AUTHZ_NO_FAILURE_AUDIT                   0x00000002
#define AUTHZ_NO_ALLOC_STRINGS                   0x00000004

#define AUTHZ_WPD_CATEGORY_FLAG                  0x00000010

#define AUTHZ_VALID_OBJECT_ACCESS_AUDIT_FLAGS    (AUTHZ_NO_SUCCESS_AUDIT | \
                                                  AUTHZ_NO_FAILURE_AUDIT | \
                                                  AUTHZ_NO_ALLOC_STRINGS | \
                                                  AUTHZ_WPD_CATEGORY_FLAG)
                             
AUTHZAPI
BOOL
WINAPI
AuthzInitializeObjectAccessAuditEvent(
    _In_    DWORD                         Flags,
    _In_    AUTHZ_AUDIT_EVENT_TYPE_HANDLE hAuditEventType,
    _In_    PWSTR                         szOperationType,
    _In_    PWSTR                         szObjectType,
    _In_    PWSTR                         szObjectName,
    _In_    PWSTR                         szAdditionalInfo,
    _Out_   PAUTHZ_AUDIT_EVENT_HANDLE     phAuditEvent,
    _In_    DWORD                         dwAdditionalParameterCount,
    ...
    );
    
AUTHZAPI
BOOL
WINAPI
AuthzInitializeObjectAccessAuditEvent2(
    _In_    DWORD                         Flags,
    _In_    AUTHZ_AUDIT_EVENT_TYPE_HANDLE hAuditEventType,
    _In_    PWSTR                         szOperationType,
    _In_    PWSTR                         szObjectType,
    _In_    PWSTR                         szObjectName,
    _In_    PWSTR                         szAdditionalInfo,
    _In_    PWSTR                         szAdditionalInfo2,
    _Out_   PAUTHZ_AUDIT_EVENT_HANDLE     phAuditEvent,
    _In_    DWORD                         dwAdditionalParameterCount,
    ...
    );

//
// Enumeration type to be used to specify the type of information to be
// retrieved from an existing AUTHZ_AUDIT_EVENT_HANDLE.
//

typedef enum _AUTHZ_AUDIT_EVENT_INFORMATION_CLASS
{
    AuthzAuditEventInfoFlags = 1,
    AuthzAuditEventInfoOperationType,
    AuthzAuditEventInfoObjectType,
    AuthzAuditEventInfoObjectName,
    AuthzAuditEventInfoAdditionalInfo,
} AUTHZ_AUDIT_EVENT_INFORMATION_CLASS;

AUTHZAPI
BOOL
WINAPI
AuthzFreeAuditEvent(
    _In_ AUTHZ_AUDIT_EVENT_HANDLE hAuditEvent
    );

//
// Support for SACL evaluation
//

AUTHZAPI
BOOL 
WINAPI
AuthzEvaluateSacl(
    _In_  AUTHZ_CLIENT_CONTEXT_HANDLE AuthzClientContext,
    _In_  PAUTHZ_ACCESS_REQUEST       pRequest,
    _In_  PACL                        Sacl,
    _In_  ACCESS_MASK                 GrantedAccess,
    _In_  BOOL                        AccessGranted,
    _Out_ PBOOL                       pbGenerateAudit
    );

//
// Support for generic auditing.
//

typedef struct _AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET
{
    PWSTR szObjectTypeName;
    DWORD dwOffset;
} AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET, *PAUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET;

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4201) //  nonstandard extension used : nameless struct/union
#endif

typedef struct _AUTHZ_SOURCE_SCHEMA_REGISTRATION
{
    DWORD dwFlags;
    PWSTR szEventSourceName;
    PWSTR szEventMessageFile;
    PWSTR szEventSourceXmlSchemaFile;
    PWSTR szEventAccessStringsFile;
    PWSTR szExecutableImagePath;

    //
    // The meaning of the data is defined by dwFlags. Make sure
    // new types are pointers.
    //

    union
    {
        PVOID pReserved;

        //
        // Must be supplied when dwFlags contains AUTHZ_MIGRATED_LEGACY_PUBLISHER
        //

        GUID* pProviderGuid;
    } DUMMYUNIONNAME;

    DWORD dwObjectTypeNameCount;
    AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET ObjectTypeNames[ANYSIZE_ARRAY];
} AUTHZ_SOURCE_SCHEMA_REGISTRATION, *PAUTHZ_SOURCE_SCHEMA_REGISTRATION;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#define AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES 0x1

AUTHZAPI
BOOL 
WINAPI
AuthzInstallSecurityEventSource(
    _In_ DWORD                             dwFlags,
    _In_ PAUTHZ_SOURCE_SCHEMA_REGISTRATION pRegistration
    );

AUTHZAPI
BOOL
WINAPI
AuthzUninstallSecurityEventSource(
    _In_ DWORD  dwFlags,
    _In_ PCWSTR szEventSourceName
    );


AUTHZAPI
BOOL
WINAPI
AuthzEnumerateSecurityEventSources(
    _In_     DWORD                             dwFlags,
    _Out_    PAUTHZ_SOURCE_SCHEMA_REGISTRATION Buffer,
    _Out_    PDWORD                            pdwCount,
     _Inout_ PDWORD                            pdwLength
    );
    
AUTHZAPI
BOOL
WINAPI
AuthzRegisterSecurityEventSource(
    _In_  DWORD                                 dwFlags,
    _In_  PCWSTR                                szEventSourceName,
    _Out_ PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE phEventProvider
    );
    
AUTHZAPI
BOOL
WINAPI
AuthzUnregisterSecurityEventSource(
    _In_     DWORD                                 dwFlags,
     _Inout_ PAUTHZ_SECURITY_EVENT_PROVIDER_HANDLE phEventProvider
    );

AUTHZAPI
BOOL
WINAPI
AuthzReportSecurityEvent(
    _In_     DWORD                                dwFlags,
     _Inout_ AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE hEventProvider,
    _In_     DWORD                                dwAuditId,
    _In_opt_ PSID                                 pUserSid,
    _In_     DWORD                                dwCount,
    ...    
    );

AUTHZAPI
BOOL
WINAPI
AuthzReportSecurityEventFromParams(
    _In_     DWORD                                dwFlags,
     _Inout_ AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE hEventProvider,
    _In_     DWORD                                dwAuditId,
    _In_opt_ PSID                                 pUserSid,
    _In_     PAUDIT_PARAMS                        pParams
    );

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN8)
_Success_(return != FALSE)
AUTHZAPI
BOOL
WINAPI
AuthzRegisterCapChangeNotification(
    _Out_ PAUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE phCapChangeSubscription, 
    _In_ LPTHREAD_START_ROUTINE pfnCapChangeCallback, 
    _In_opt_ PVOID pCallbackContext
    );

AUTHZAPI
BOOL
WINAPI
AuthzUnregisterCapChangeNotification(
    _In_ AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE hCapChangeSubscription
    );

AUTHZAPI
BOOL
WINAPI
AuthzFreeCentralAccessPolicyCache(
    void
    );
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* __AUTHZ_H__ */

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
