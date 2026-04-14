
/*++

Copyright (c) 1996 Microsoft Corporation.  All rights reserved.

Module Name:

    clusapi.h

Abstract:

    This module defines the common management and application interface to
    the Microsoft Cluster Server services.

Revision History:

--*/

#ifndef _CLUSTER_API_
#define _CLUSTER_API_

#if ( !MIDL_PASS && !__midl )
#define CLUSTER_VERSION_FLAG_MIXED_MODE     0x00000001

#define CLUSTER_VERSION_UNKNOWN         0xFFFFFFFF

// these defines represent cluster numbers, not windows OS numbers
#define NT4_MAJOR_VERSION           1
#define NT4SP4_MAJOR_VERSION        2
#define NT5_MAJOR_VERSION           3
#define NT51_MAJOR_VERSION          4
#define NT6_MAJOR_VERSION           5
#define NT7_MAJOR_VERSION           6
#define NT8_MAJOR_VERSION           7
#define NT9_MAJOR_VERSION           8
#define NT10_MAJOR_VERSION          9
#define NT11_MAJOR_VERSION          10
#define NT12_MAJOR_VERSION          11
#define NT13_MAJOR_VERSION          12

// NT10 cluster upgrade versions (eg technical previews)
#define WS2016_TP4_UPGRADE_VERSION  6
#define WS2016_TP5_UPGRADE_VERSION  7
#define WS2016_RTM_UPGRADE_VERSION  8

// NT11 upgrade versions
#define RS3_UPGRADE_VERSION  1
#define RS4_UPGRADE_VERSION  2
#define RS5_UPGRADE_VERSION  3

// NT12 upgrade versions
#define NINETEEN_H1_UPGRADE_VERSION  1
#define NINETEEN_H2_UPGRADE_VERSION  2
#define MN_UPGRADE_VERSION           3
#define FE_UPGRADE_VERSION           4
#define FE_22H2_UPGRADE_VERSION      5

// NT13 upgrade versions
#define CA_UPGRADE_VERSION           1
#define NI_UPGRADE_VERSION           2
#define CU_UPGRADE_VERSION           3
#define ZN_UPGRADE_VERSION           4
#define GA_UPGRADE_VERSION           5
#define GE_UPGRADE_VERSION           6

#define HCI_UPGRADE_BIT 0x8000

#define CLUSREG_NAME_MIXED_MODE                    L"MixedMode"

#endif // ( !MIDL_PASS && !__midl )

#ifndef _IN_KERNEL_



#include <winerror.h>

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or FailoverCluster Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CLUSTER)


#define CLUSAPI_VERSION_SERVER2008   0x00000600
#define CLUSAPI_VERSION_SERVER2008R2 0x00000700
#define CLUSAPI_VERSION_WINDOWS8     0x00000701
#define CLUSAPI_VERSION_WINDOWSBLUE  0x00000702
#define CLUSAPI_VERSION_WINTHRESHOLD 0x00000703
#define CLUSAPI_VERSION_RS3          0x00000A00
#define CLUSAPI_VERSION_NI           0x00000A0C
// starting with CU use convention of 0x0000 + 2 digit major version + 2 digit minor version
// ie NT13_MAJOR_VERSION = 12 = 0x0C, CU_UPGRADE_VERSION = 3 = 0x03
#define CLUSAPI_VERSION_CU           0x00000C03
#define CLUSAPI_VERSION_ZN           0x00000C04
#define CLUSAPI_VERSION_GA           0x00000C05


#if (!defined(CLUSAPI_VERSION))
#if (!defined(NTDDI_VERSION) || (NTDDI_VERSION >= NTDDI_WIN11_GA))
#define CLUSAPI_VERSION  CLUSAPI_VERSION_GA
#elif (!defined(NTDDI_VERSION) || (NTDDI_VERSION >= NTDDI_WIN11_ZN))
#define CLUSAPI_VERSION  CLUSAPI_VERSION_ZN
#elif (!defined(NTDDI_VERSION) || (NTDDI_VERSION >= NTDDI_WIN10_CU))
#define CLUSAPI_VERSION CLUSAPI_VERSION_CU
#elif (!defined(NTDDI_VERSION) || (NTDDI_VERSION >= NTDDI_WIN10_NI))
#define CLUSAPI_VERSION CLUSAPI_VERSION_NI
#elif (!defined(NTDDI_VERSION) || (NTDDI_VERSION >= NTDDI_WIN10_RS3))
#define CLUSAPI_VERSION CLUSAPI_VERSION_RS3
#elif (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define CLUSAPI_VERSION CLUSAPI_VERSION_WINTHRESHOLD
#elif (NTDDI_VERSION >= NTDDI_WINBLUE)
#define CLUSAPI_VERSION CLUSAPI_VERSION_WINDOWSBLUE
#elif (NTDDI_VERSION >= NTDDI_WIN8)
#define CLUSAPI_VERSION CLUSAPI_VERSION_WINDOWS8
#elif (NTDDI_VERSION >= NTDDI_WIN7)
#define CLUSAPI_VERSION CLUSAPI_VERSION_SERVER2008R2
#else
#define CLUSAPI_VERSION CLUSAPI_VERSION_SERVER2008
#endif
#endif // !defined(CLUSAPI_VERSION)

#define CREATE_CLUSTER_VERSION CLUSAPI_VERSION_SERVER2008
#define CREATE_CLUSTER_MAJOR_VERSION_MASK 0xFFFFFF00

#ifdef __cplusplus
extern "C" {
#endif

#if ( !MIDL_PASS && !__midl )
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning( disable : 4200 ) // nonstandard extension used : zero-sized array in struct/union
#pragma warning( disable : 4201 ) // nonstandard extension used : nameless struct/union
#endif // MIDL_PASS

//
// General cluster definitions
//

#ifndef _CLUSTER_API_TYPES_
//
// Defined cluster handle types.
//
typedef struct _HCLUSTER *HCLUSTER;
typedef struct _HNODE *HNODE;
typedef struct _HRESOURCE *HRESOURCE;
typedef struct _HGROUP *HGROUP;
typedef struct _HNETWORK *HNETWORK;
typedef struct _HNETINTERFACE *HNETINTERFACE;
typedef struct _HCHANGE *HCHANGE;
typedef struct _HCLUSENUM *HCLUSENUM;
typedef struct _HGROUPENUM *HGROUPENUM;
typedef struct _HRESENUM *HRESENUM;
typedef struct _HNETWORKENUM *HNETWORKENUM;
typedef struct _HNODEENUM *HNODEENUM;
typedef struct _HNETINTERFACEENUM *HNETINTERFACEENUM;
typedef struct _HRESTYPEENUM *HRESTYPEENUM;
typedef struct _HREGBATCH *HREGBATCH;
typedef struct _HREGBATCHPORT *HREGBATCHPORT;
typedef struct _HREGBATCHNOTIFICATION *HREGBATCHNOTIFICATION;
typedef struct _HREGREADBATCH *HREGREADBATCH;
typedef struct _HREGREADBATCHREPLY *HREGREADBATCHREPLY;
typedef struct _HKEYVALUEBATCH *HKEYVALUEBATCH;
typedef struct _HKEYVALUEBATCHNOTIFICATION *HKEYVALUEBATCHNOTIFICATION;
typedef struct _HKEYVALUEREADBATCH *HKEYVALUEREADBATCH;
typedef struct _HKEYVALUEREADBATCHREPLY *HKEYVALUEREADBATCHREPLY;
typedef struct _HKEYVALUESTORE *HKEYVALUESTORE;

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
typedef struct _HNODEENUMEX *HNODEENUMEX;
typedef struct _HCLUSENUMEX *HCLUSENUMEX;
#endif

#if(CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)
typedef struct _HGROUPENUMEX *HGROUPENUMEX;
typedef struct _HRESENUMEX *HRESENUMEX;
#endif

#if(CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
typedef struct _HGROUPSET *HGROUPSET;
typedef struct _HGROUPSETENUM *HGROUPSETENUM;
#endif

#endif // _CLUSTER_API_TYPES_

//
// Definitions used in cluster management routines.
//

#define MAX_CLUSTERNAME_LENGTH      DNS_MAX_LABEL_LENGTH

#ifndef _CLUSTER_API_TYPES_
//
// Cluster-related structures and types
//
typedef enum CLUSTER_QUORUM_TYPE {
    OperationalQuorum,
    ModifyQuorum
} CLUSTER_QUORUM_TYPE;

#if ( !MIDL_PASS && !__midl )

typedef struct CLUSTERVERSIONINFO_NT4 {
    DWORD dwVersionInfoSize;
    WORD  MajorVersion;
    WORD  MinorVersion;
    WORD  BuildNumber;
    WCHAR szVendorId[64];
    WCHAR szCSDVersion[64];
}CLUSTERVERSIONINFO_NT4, *PCLUSTERVERSIONINFO_NT4;

typedef struct CLUSTERVERSIONINFO {
    DWORD dwVersionInfoSize;
    WORD  MajorVersion;
    WORD  MinorVersion;
    WORD  BuildNumber;
    WCHAR szVendorId[64];
    WCHAR szCSDVersion[64];
    DWORD dwClusterHighestVersion;
    DWORD dwClusterLowestVersion;
    DWORD dwFlags;
    DWORD dwReserved;
} CLUSTERVERSIONINFO, *LPCLUSTERVERSIONINFO, *PCLUSTERVERSIONINFO;


typedef struct CLUS_STARTING_PARAMS {
    DWORD   dwSize;
    BOOL    bForm;
    BOOL    bFirst;
} CLUS_STARTING_PARAMS, * PCLUS_STARTING_PARAMS;



#define CLUSTER_VERSION_UNKNOWN         0xFFFFFFFF

// these defines represent cluster numbers, not windows OS numbers
#define NT4_MAJOR_VERSION           1
#define NT4SP4_MAJOR_VERSION        2
#define NT5_MAJOR_VERSION           3
#define NT51_MAJOR_VERSION          4
#define NT6_MAJOR_VERSION           5
#define NT7_MAJOR_VERSION           6
#define NT8_MAJOR_VERSION           7
#define NT9_MAJOR_VERSION           8
#define NT10_MAJOR_VERSION          9

// And cluster upgrade versions (eg technical previews)
#define WS2016_TP4_UPGRADE_VERSION  6
#define WS2016_TP5_UPGRADE_VERSION  7
#define WS2016_RTM_UPGRADE_VERSION  8

//
// Version number macros
// Minor version has been renamed to upgrade version, but the minor version
// macro must remain for compatibility
//

#define CLUSTER_MAKE_VERSION( _maj, _min ) ((( _maj ) << 16 ) | ( _min ))
#define CLUSTER_GET_MAJOR_VERSION( _ver ) (( _ver ) >> 16 )
#define CLUSTER_GET_MINOR_VERSION( _ver ) (( _ver ) & 0xFFFF )
#define CLUSTER_GET_UPGRADE_VERSION( _ver ) (( _ver ) & 0xFFFF )

#endif // MIDL_PASS

//
// Interfaces for the cluster state on a node
//
#define CLUSTER_INSTALLED   0x00000001
#define CLUSTER_CONFIGURED  0x00000002
#define CLUSTER_RUNNING     0x00000010

typedef enum NODE_CLUSTER_STATE {
    ClusterStateNotInstalled                = 0x00000000,
    ClusterStateNotConfigured               = CLUSTER_INSTALLED,
    ClusterStateNotRunning                  = CLUSTER_INSTALLED | CLUSTER_CONFIGURED,
    ClusterStateRunning                     = CLUSTER_INSTALLED | CLUSTER_CONFIGURED | CLUSTER_RUNNING
} NODE_CLUSTER_STATE;

// Quorum mode flags for SetClusterQuorumResource API

#define CLUS_HYBRID_QUORUM          1024                // 0xFFFFFFFF
#define CLUS_NODE_MAJORITY_QUORUM   0                   // 0xFFFFFFFE
#define CLUS_LEGACY_QUORUM          (4 * 1024 * 1024)   // 0xFFFFFFFD

//
//  Resource state change reason related types and defines
//
#define CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1  1

typedef enum CLUSTER_RESOURCE_STATE_CHANGE_REASON {
    eResourceStateChangeReasonUnknown,
    eResourceStateChangeReasonMove,
    eResourceStateChangeReasonFailover,
    eResourceStateChangeReasonFailedMove,
    eResourceStateChangeReasonShutdown,
    eResourceStateChangeReasonRundown
} CLUSTER_RESOURCE_STATE_CHANGE_REASON;

typedef enum _CLUSTER_REG_COMMAND
{
    CLUSREG_COMMAND_NONE = 0,

    CLUSREG_SET_VALUE = 1,
    CLUSREG_CREATE_KEY,
    CLUSREG_DELETE_KEY,
    CLUSREG_DELETE_VALUE,
    CLUSREG_SET_KEY_SECURITY,
    CLUSREG_VALUE_DELETED,

    // Commands for read batch
    CLUSREG_READ_KEY,
    CLUSREG_READ_VALUE,
    CLUSREG_READ_ERROR,

    // Control command
        CLUSREG_CONTROL_COMMAND,

        // Write conditions
        CLUSREG_CONDITION_EXISTS,
        CLUSREG_CONDITION_NOT_EXISTS,
        CLUSREG_CONDITION_IS_EQUAL,
        CLUSREG_CONDITION_IS_NOT_EQUAL,
        CLUSREG_CONDITION_IS_GREATER_THAN,
        CLUSREG_CONDITION_IS_LESS_THAN,
        CLUSREG_CONDITION_KEY_EXISTS,
        CLUSREG_CONDITION_KEY_NOT_EXISTS,

    CLUSREG_LAST_COMMAND

} CLUSTER_REG_COMMAND;

#define  CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES        1
#define  CLUSREG_DATABASE_ISOLATE_READ                                2


#if ( !MIDL_PASS && !__midl )

typedef struct _CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    DWORD                                   dwSize;
    DWORD                                   dwVersion;
    CLUSTER_RESOURCE_STATE_CHANGE_REASON    eReason;
} CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT, *PCLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT;

typedef struct _CLUSTER_BATCH_COMMAND
{
    CLUSTER_REG_COMMAND Command;
    DWORD               dwOptions;
    LPCWSTR             wzName;
    BYTE CONST *        lpData;
    DWORD               cbData;
} CLUSTER_BATCH_COMMAND;

typedef struct _CLUSTER_READ_BATCH_COMMAND
{
    CLUSTER_REG_COMMAND Command;
    DWORD               dwOptions;
    LPCWSTR             wzSubkeyName;
    LPCWSTR             wzValueName;
    BYTE CONST *        lpData;
    DWORD               cbData;
} CLUSTER_READ_BATCH_COMMAND;


#endif // MIDL_PASS

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2

#define CLUSTER_ENUM_ITEM_VERSION_1 0x00000001
#define CLUSTER_ENUM_ITEM_VERSION   CLUSTER_ENUM_ITEM_VERSION_1

typedef struct _CLUSTER_ENUM_ITEM {
    DWORD dwVersion;
    DWORD dwType;
    DWORD cbId;
    LPWSTR lpszId;
    DWORD cbName;
    LPWSTR lpszName;
} CLUSTER_ENUM_ITEM, *PCLUSTER_ENUM_ITEM;

#endif // CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2

typedef enum {
    ClusGroupTypeCoreCluster        = 1,
    ClusGroupTypeAvailableStorage   = 2,
    ClusGroupTypeTemporary          = 3,
    ClusGroupTypeSharedVolume       = 4,
    ClusGroupTypeStoragePool        = 5,
    ClusGroupTypeFileServer         = 100,
    ClusGroupTypePrintServer        = 101,
    ClusGroupTypeDhcpServer         = 102,
    ClusGroupTypeDtc                = 103,
    ClusGroupTypeMsmq               = 104,
    ClusGroupTypeWins               = 105,
    ClusGroupTypeStandAloneDfs      = 106,
    ClusGroupTypeGenericApplication = 107,
    ClusGroupTypeGenericService     = 108,
    ClusGroupTypeGenericScript      = 109,
    ClusGroupTypeIScsiNameService   = 110,
    ClusGroupTypeVirtualMachine     = 111,
    ClusGroupTypeTsSessionBroker    = 112,
    ClusGroupTypeIScsiTarget        = 113,
    ClusGroupTypeScaleoutFileServer = 114,
    ClusGroupTypeVMReplicaBroker    = 115,
    ClusGroupTypeTaskScheduler      = 116,
    ClusGroupTypeClusterUpdateAgent = 117,
    ClusGroupTypeScaleoutCluster    = 118,
    ClusGroupTypeStorageReplica     = 119,
    ClusGroupTypeVMReplicaCoordinator        = 120,
    ClusGroupTypeCrossClusterOrchestrator = 121,
    ClusGroupTypeInfrastructureFileServer = 122,
    ClusGroupTypeCoreSddc           = 123,
    ClusGroupTypeUserManager        = 124,
    ClusGroupTypeKeyValueStoreManager = 125,
    ClusGroupTypeHcsVirtualMachine    = 126,
    ClusGroupTypeMetaVirtualMachine   = 127,
    ClusGroupTypeUnknown            = 9999
} CLUSGROUP_TYPE, *PCLUSGROUP_TYPE;

#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

#define CLUSTER_CREATE_GROUP_INFO_VERSION_1 0x00000001
#define CLUSTER_CREATE_GROUP_INFO_VERSION   CLUSTER_CREATE_GROUP_INFO_VERSION_1

typedef struct _CLUSTER_CREATE_GROUP_INFO {
    DWORD   dwVersion;
    CLUSGROUP_TYPE  groupType;
}CLUSTER_CREATE_GROUP_INFO, *PCLUSTER_CREATE_GROUP_INFO;
#endif

typedef enum
{
    CLUSTER_MGMT_POINT_TYPE_NONE = 0,
    CLUSTER_MGMT_POINT_TYPE_CNO,
    CLUSTER_MGMT_POINT_TYPE_DNS_ONLY,
    CLUSTER_MGMT_POINT_TYPE_CNO_ONLY
} CLUSTER_MGMT_POINT_TYPE;

typedef enum
{
    CLUSTER_MGMT_POINT_RESTYPE_AUTO = 0,
    CLUSTER_MGMT_POINT_RESTYPE_SNN = 1,
    CLUSTER_MGMT_POINT_RESTYPE_DNN = 2
} CLUSTER_MGMT_POINT_RESTYPE, *PCLUSTER_MGMT_POINT_RESTYPE;

typedef enum
{
    CLUSTER_CLOUD_TYPE_NONE     = 0,
    CLUSTER_CLOUD_TYPE_AZURE    = 1,


    CLUSTER_CLOUD_TYPE_MIXED    = 128,

    CLUSTER_CLOUD_TYPE_UNKNOWN  = -1
} CLUSTER_CLOUD_TYPE, *PCLUSTER_CLOUD_TYPE;


#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD

#define GROUPSET_READY_SETTING_DELAY              0x00000001
#define GROUPSET_READY_SETTING_ONLINE             0x00000002
#define GROUPSET_READY_SETTING_OS_HEARTBEAT       0x00000003
#define GROUPSET_READY_SETTING_APPLICATION_READY  0x00000004

typedef enum
{
    CLUS_GROUP_START_ALWAYS = 0,
    CLUS_GROUP_DO_NOT_START = 1,
    CLUS_GROUP_START_ALLOWED = 2
} CLUS_GROUP_START_SETTING;

typedef enum
{
    CLUS_AFFINITY_RULE_NONE = 0,
    CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN = 1,
    CLUS_AFFINITY_RULE_SAME_NODE = 2,
    CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN = 3,
    CLUS_AFFINITY_RULE_DIFFERENT_NODE = 4,

    CLUS_AFFINITY_RULE_MIN = CLUS_AFFINITY_RULE_NONE,
    CLUS_AFFINITY_RULE_MAX = CLUS_AFFINITY_RULE_DIFFERENT_NODE,
} CLUS_AFFINITY_RULE_TYPE;

typedef enum
{
    CLUS_ADAPTER_EXCLUSION_TYPE_IPPREFIX = 0,
    CLUS_ADAPTER_EXCLUSION_TYPE_DESCRIPTION = 1,
    CLUS_ADAPTER_EXCLUSION_TYPE_FRIENDLYNAME = 2,
} CLUS_ADAPTER_EXCLUSION_TYPE;

#define CLUS_GRP_MOVE_ALLOWED 0
#define CLUS_GRP_MOVE_LOCKED  1

#endif // CLUSAPI_VERSION_WINTHRESHOLD

#endif // _CLUSTER_API_TYPES_

//
// Interfaces for managing clusters
//

//
// Cluster API Specific Access Rights
//
#define CLUSAPI_READ_ACCESS     0x00000001L
#define CLUSAPI_CHANGE_ACCESS   0x00000002L
#define CLUSAPI_NO_ACCESS       0x00000004L
#define CLUSAPI_ALL_ACCESS (CLUSAPI_READ_ACCESS | CLUSAPI_CHANGE_ACCESS)

//
// Cluster API Access Control Type
//
#define CLUSTER_SET_ACCESS_TYPE_ALLOWED     0       //To add an allowed ACE
#define CLUSTER_SET_ACCESS_TYPE_DENIED      1       //To add a denied ACE
#define CLUSTER_DELETE_ACCESS_CONTROL_ENTRY 2       //To delete all the ACEs for a particular SID

//
// Return values for CLUSCTL_CLUSTER_CHECK_VOTER_DOWN and CLUSCTL_CLUSTER_CHECK_VOTER_EVICT
//
typedef enum CLUSTER_QUORUM_VALUE {
    CLUSTER_QUORUM_MAINTAINED = 0,
    CLUSTER_QUORUM_LOST = 1,
} CLUSTER_QUORUM_VALUE;

#if ( !MIDL_PASS && !__midl )

//
// Structure used to pass in the path to validate
//
typedef struct _CLUSTER_VALIDATE_PATH {
    WCHAR          szPath[];
} CLUSTER_VALIDATE_PATH, *PCLUSTER_VALIDATE_PATH;

//
// Structure used to pass in the directory to validate
//
typedef struct _CLUSTER_VALIDATE_DIRECTORY {
    WCHAR          szPath[];
} CLUSTER_VALIDATE_DIRECTORY, *PCLUSTER_VALIDATE_DIRECTORY;

//
// Structure used to pass in the network name to validate
//
typedef struct _CLUSTER_VALIDATE_NETNAME {
    WCHAR          szNetworkName[];
} CLUSTER_VALIDATE_NETNAME , *PCLUSTER_VALIDATE_NETNAME ;

//
// Structure used to pass in the file name to validate
//
typedef struct _CLUSTER_VALIDATE_CSV_FILENAME {
    WCHAR          szFileName[];
} CLUSTER_VALIDATE_CSV_FILENAME , *PCLUSTER_VALIDATE_CSV_FILENAME ;

//
// Structure used to return the status of a request to set the
// password on the account used by the Cluster Service on each
// cluster node.
//
typedef struct CLUSTER_SET_PASSWORD_STATUS {
    DWORD    NodeId;
    BOOLEAN  SetAttempted;
    DWORD    ReturnStatus;
} CLUSTER_SET_PASSWORD_STATUS, *PCLUSTER_SET_PASSWORD_STATUS;

#ifndef _CLUSTER_API_TYPES_
typedef struct _CLUSTER_IP_ENTRY
{
    PCWSTR          lpszIpAddress;
    DWORD           dwPrefixLength;
} CLUSTER_IP_ENTRY, *PCLUSTER_IP_ENTRY;

typedef struct _CREATE_CLUSTER_CONFIG
{
    DWORD                       dwVersion;
    PCWSTR                      lpszClusterName;
    DWORD                       cNodes;
    PCWSTR *                    ppszNodeNames;
    DWORD                       cIpEntries;
    PCLUSTER_IP_ENTRY           pIpEntries;
    BOOLEAN                     fEmptyCluster;
    CLUSTER_MGMT_POINT_TYPE     managementPointType;        // CLUSAPI Version >= CLUSAPI_VERSION_WINDOWSBLUE
    CLUSTER_MGMT_POINT_RESTYPE  managementPointResType;     // CLUSAPI Version >= CLUSAPI_VERSION_RS3
    PCWSTR                      pszUserName;                // CLUSAPU Version >= CLUSAPI_VERSION_GA
    PCWSTR                      pszPassword;                // CLUSAPU Version >= CLUSAPI_VERSION_GA
    PCWSTR                      pszDomain;                  // CLUSAPU Version >= CLUSAPI_VERSION_GA
} CREATE_CLUSTER_CONFIG, *PCREATE_CLUSTER_CONFIG;

// CLUSAPI Version >= CLUSAPI_VERSION_WINTHRESHOLD
typedef struct _CREATE_CLUSTER_NAME_ACCOUNT
{
    DWORD                       dwVersion;
    PCWSTR                      lpszClusterName;
    DWORD                       dwFlags;
    PCWSTR                      pszUserName;
    PCWSTR                      pszPassword;
    PCWSTR                      pszDomain;
    CLUSTER_MGMT_POINT_TYPE     managementPointType;
    CLUSTER_MGMT_POINT_RESTYPE  managementPointResType;        // CLUSAPI Version >= CLUSAPI_VERSION_RS3
    BOOLEAN                     bUpgradeVCOs;           // CLUSAPI Version >= CLUSAPI_VERSION_RS3, managementPointType==CLUSTER_MGMT_POINT_TYPE_CNO
} CREATE_CLUSTER_NAME_ACCOUNT, *PCREATE_CLUSTER_NAME_ACCOUNT;

// Cluster Version >= NT13.CU_UPGRADE_VERSION
// CLUSAPI >= CLUSAPI_VERSION_CU
typedef struct _REPAIR_CLUSTER_NAME_ACCOUNT_CONFIG
{
    DWORD                       dwVersion;
    DWORD                       dwFlags;
    PCWSTR                      pszUserName;
    PCWSTR                      pszPassword;
    PCWSTR                      pszDomain;
} REPAIR_CLUSTER_NAME_ACCOUNT_CONFIG, *PREPAIR_CLUSTER_NAME_ACCOUNT_CONFIG;



#endif // _CLUSTER_API_TYPES_

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

typedef BOOL(WINAPI* PCLUSAPI_PFN_REASON_HANDLER)(
    _In_ LPVOID lpParameter,
    _In_ HCLUSTER hCluster,
    _Out_ LPWSTR szReason,
    _Inout_ LPDWORD lpSize
    );

typedef struct _CLUSAPI_REASON_HANDLER {
    LPVOID lpParameter;
    PCLUSAPI_PFN_REASON_HANDLER pfnHandler;
} CLUSAPI_REASON_HANDLER, *PCLUSAPI_REASON_HANDLER;

PCLUSAPI_REASON_HANDLER
WINAPI
ClusapiSetReasonHandler(
    _In_ PCLUSAPI_REASON_HANDLER lpHandler
    );

typedef PCLUSAPI_REASON_HANDLER
(WINAPI* PCLUSAPI_SET_REASON_HANDLER)(
    _In_ PCLUSAPI_REASON_HANDLER lpHandler
    );

#endif

DWORD
WINAPI
GetNodeClusterState(
    _In_opt_    LPCWSTR lpszNodeName,
    _Out_       LPDWORD pdwClusterState
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_NODE_CLUSTER_STATE)(
    _In_opt_    LPCWSTR lpszNodeName,
    _Out_       LPDWORD pdwClusterState
    );

HCLUSTER
WINAPI
OpenCluster(
    _In_opt_ LPCWSTR lpszClusterName
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_OPEN_CLUSTER)(
    _In_opt_ LPCWSTR lpszClusterName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
HCLUSTER
WINAPI
OpenClusterEx(
    _In_opt_ LPCWSTR lpszClusterName,
    _In_ DWORD DesiredAccess,
    _Out_opt_ DWORD* GrantedAccess
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_OPEN_CLUSTER_EX)(
    _In_opt_  LPCWSTR lpszClusterName,
    _In_      DWORD   dwDesiredAccess,
    _Out_opt_ LPDWORD lpdwGrantedAccess
    );
#endif

BOOL
WINAPI
CloseCluster(
    _In_ HCLUSTER hCluster
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER)(
    _In_ HCLUSTER hCluster
    );

DWORD
WINAPI
SetClusterName(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNewClusterName
    );

typedef DWORD
(WINAPI * PCLUSAPI_SetClusterName)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNewClusterName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
SetClusterNameEx(
    _In_     HCLUSTER hCluster,
    _In_     LPCWSTR  lpszNewClusterName,
    _In_opt_ LPCWSTR  lpszReason
);

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_NAME_EX)(
    _In_     HCLUSTER hCluster,
    _In_     LPCWSTR  lpszNewClusterName,
    _In_opt_ LPCWSTR  lpszReason
);

#endif

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterInformation(
    _In_ HCLUSTER hCluster,
    _Out_writes_to_(*lpcchClusterName, *lpcchClusterName + 1) LPWSTR lpszClusterName,
    _Inout_ LPDWORD lpcchClusterName,
    _Out_opt_ LPCLUSTERVERSIONINFO lpClusterInfo
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_INFORMATION)(
    _In_ HCLUSTER hCluster,
    _Out_writes_to_(*lpcchClusterName, *lpcchClusterName + 1) LPWSTR lpszClusterName,
    _Inout_ LPDWORD lpcchClusterName,
    _Out_opt_ LPCLUSTERVERSIONINFO lpClusterInfo
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterQuorumResource(
    _In_ HCLUSTER hCluster,
    _Out_writes_to_(*lpcchResourceName, *lpcchResourceName + 1) LPWSTR lpszResourceName,
    _Inout_ LPDWORD lpcchResourceName,
    _Out_writes_to_(*lpcchDeviceName, *lpcchDeviceName + 1) LPWSTR lpszDeviceName,
    _Inout_ LPDWORD lpcchDeviceName,
    _Out_ LPDWORD lpdwMaxQuorumLogSize
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE)(
    _In_ HCLUSTER hCluster,
    _Out_writes_to_(*lpcchResourceName, *lpcchResourceName + 1) LPWSTR lpszResourceName,
    _Inout_ LPDWORD lpcchResourceName,
    _Out_writes_to_(*lpcchDeviceName, *lpcchDeviceName + 1) LPWSTR lpszDeviceName,
    _Inout_ LPDWORD lpcchDeviceName,
    _Out_ LPDWORD lpdwMaxQuorumLogSize
    );

DWORD
WINAPI
SetClusterQuorumResource(
    _In_     HRESOURCE hResource,
    _In_opt_ LPCWSTR   lpszDeviceName,
    _In_     DWORD     dwMaxQuoLogSize
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE)(
    _In_     HRESOURCE hResource,
    _In_opt_ LPCWSTR   lpszDeviceName,
    _In_     DWORD     dwMaxQuoLogSize
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
SetClusterQuorumResourceEx(
    _In_ HRESOURCE hResource,
    _In_opt_ LPCWSTR lpszDeviceName,
    _In_ DWORD dwMaxQuorumLogSize,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE_EX)(
    _In_ HRESOURCE hResource,
    _In_opt_ LPCWSTR lpszDeviceName,
    _In_ DWORD dwMaxQuorumLogSize,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
BackupClusterDatabase(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR  lpszPathName
    );

typedef DWORD
(WINAPI * PCLUSAPI_BACKUP_CLUSTER_DATABASE)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR  lpszPathName
    );

DWORD
WINAPI
RestoreClusterDatabase(
    _In_ LPCWSTR  lpszPathName,
    _In_ BOOL     bForce,
    _In_opt_ LPCWSTR  lpszQuorumDriveLetter
    );

typedef DWORD
(WINAPI * PCLUSAPI_RESTORE_CLUSTER_DATABASE)(
    _In_ LPCWSTR  lpszPathName,
    _In_ BOOL     bForce,
    _In_opt_ LPCWSTR  lpszQuorumDriveLetter
    );

DWORD
WINAPI
SetClusterNetworkPriorityOrder(
    _In_ HCLUSTER hCluster,
    _In_ DWORD NetworkCount,
    _In_reads_( NetworkCount ) HNETWORK NetworkList[]
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER)(
    _In_ HCLUSTER hCluster,
    _In_ DWORD NetworkCount,
    _In_reads_( NetworkCount ) HNETWORK NetworkList[]
    );

DWORD
WINAPI
SetClusterServiceAccountPassword(
    _In_ LPCWSTR lpszClusterName,
    _In_ LPCWSTR lpszNewPassword,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*lpcbReturnStatusBufferSize, *lpcbReturnStatusBufferSize) PCLUSTER_SET_PASSWORD_STATUS lpReturnStatusBuffer,
    _Inout_ LPDWORD lpcbReturnStatusBufferSize
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD)(
    _In_ LPCWSTR lpszClusterName,
    _In_ LPCWSTR lpszNewPassword,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*lpcbReturnStatusBufferSize, *lpcbReturnStatusBufferSize) PCLUSTER_SET_PASSWORD_STATUS lpReturnStatusBuffer,
    _Inout_ LPDWORD lpcbReturnStatusBufferSize
    );

DWORD
WINAPI
ClusterControl(
    _In_ HCLUSTER hCluster,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_CONTROL)(
    _In_ HCLUSTER hCluster,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ClusterControlEx(
    _In_ HCLUSTER hCluster,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_CONTROL_EX)(
    _In_ HCLUSTER hCluster,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

typedef enum _CLUSTER_UPGRADE_PHASE {
    ClusterUpgradePhaseInitialize              = 1,
    ClusterUpgradePhaseValidatingUpgrade       = 2,
    ClusterUpgradePhaseUpgradingComponents     = 3,
    ClusterUpgradePhaseInstallingNewComponents = 4,
    ClusterUpgradePhaseUpgradeComplete         = 5
} CLUSTER_UPGRADE_PHASE;

typedef BOOL
(WINAPI *PCLUSTER_UPGRADE_PROGRESS_CALLBACK)(
    PVOID pvCallbackArg,
    CLUSTER_UPGRADE_PHASE eUpgradePhase
    );

DWORD
WINAPI
ClusterUpgradeFunctionalLevel(
    _In_ HCLUSTER hCluster,
    _In_ BOOL perform,
    _In_opt_ PCLUSTER_UPGRADE_PROGRESS_CALLBACK pfnProgressCallback,
    _In_opt_ PVOID pvCallbackArg
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_UPGRADE)(
    _In_ HCLUSTER hCluster,
    _In_ BOOL perform,
    _In_opt_ PCLUSTER_UPGRADE_PROGRESS_CALLBACK pfnProgressCallback,
    _In_opt_ PVOID pvCallbackArg
    );


#endif // MIDL_PASS

//
// Cluster Event Notification API
//

#ifndef _CLUSTER_API_TYPES_
//
// Cluster notification enums original
//
typedef enum CLUSTER_CHANGE {
    CLUSTER_CHANGE_NODE_STATE               = 0x00000001,
    CLUSTER_CHANGE_NODE_DELETED             = 0x00000002,
    CLUSTER_CHANGE_NODE_ADDED               = 0x00000004,
    CLUSTER_CHANGE_NODE_PROPERTY            = 0x00000008,

    CLUSTER_CHANGE_REGISTRY_NAME            = 0x00000010,
    CLUSTER_CHANGE_REGISTRY_ATTRIBUTES      = 0x00000020,
    CLUSTER_CHANGE_REGISTRY_VALUE           = 0x00000040,
    CLUSTER_CHANGE_REGISTRY_SUBTREE         = 0x00000080,

    CLUSTER_CHANGE_RESOURCE_STATE           = 0x00000100,
    CLUSTER_CHANGE_RESOURCE_DELETED         = 0x00000200,
    CLUSTER_CHANGE_RESOURCE_ADDED           = 0x00000400,
    CLUSTER_CHANGE_RESOURCE_PROPERTY        = 0x00000800,

    CLUSTER_CHANGE_GROUP_STATE              = 0x00001000,
    CLUSTER_CHANGE_GROUP_DELETED            = 0x00002000,
    CLUSTER_CHANGE_GROUP_ADDED              = 0x00004000,
    CLUSTER_CHANGE_GROUP_PROPERTY           = 0x00008000,

    CLUSTER_CHANGE_RESOURCE_TYPE_DELETED    = 0x00010000,
    CLUSTER_CHANGE_RESOURCE_TYPE_ADDED      = 0x00020000,
    CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY   = 0x00040000,

    CLUSTER_CHANGE_CLUSTER_RECONNECT        = 0x00080000,

    CLUSTER_CHANGE_NETWORK_STATE            = 0x00100000,
    CLUSTER_CHANGE_NETWORK_DELETED          = 0x00200000,
    CLUSTER_CHANGE_NETWORK_ADDED            = 0x00400000,
    CLUSTER_CHANGE_NETWORK_PROPERTY         = 0x00800000,

    CLUSTER_CHANGE_NETINTERFACE_STATE       = 0x01000000,
    CLUSTER_CHANGE_NETINTERFACE_DELETED     = 0x02000000,
    CLUSTER_CHANGE_NETINTERFACE_ADDED       = 0x04000000,
    CLUSTER_CHANGE_NETINTERFACE_PROPERTY    = 0x08000000,

    CLUSTER_CHANGE_QUORUM_STATE             = 0x10000000,
    CLUSTER_CHANGE_CLUSTER_STATE            = 0x20000000,
    CLUSTER_CHANGE_CLUSTER_PROPERTY         = 0x40000000,


    CLUSTER_CHANGE_HANDLE_CLOSE             = 0x80000000,

    CLUSTER_CHANGE_ALL                      = (CLUSTER_CHANGE_NODE_STATE                |
                                               CLUSTER_CHANGE_NODE_DELETED              |
                                               CLUSTER_CHANGE_NODE_ADDED                |
                                               CLUSTER_CHANGE_NODE_PROPERTY             |
                                               CLUSTER_CHANGE_REGISTRY_NAME             |
                                               CLUSTER_CHANGE_REGISTRY_ATTRIBUTES       |
                                               CLUSTER_CHANGE_REGISTRY_VALUE            |
                                               CLUSTER_CHANGE_REGISTRY_SUBTREE          |
                                               CLUSTER_CHANGE_RESOURCE_STATE            |
                                               CLUSTER_CHANGE_RESOURCE_DELETED          |
                                               CLUSTER_CHANGE_RESOURCE_ADDED            |
                                               CLUSTER_CHANGE_RESOURCE_PROPERTY         |
                                               CLUSTER_CHANGE_GROUP_STATE               |
                                               CLUSTER_CHANGE_GROUP_DELETED             |
                                               CLUSTER_CHANGE_GROUP_ADDED               |
                                               CLUSTER_CHANGE_GROUP_PROPERTY            |
                                               CLUSTER_CHANGE_RESOURCE_TYPE_DELETED     |
                                               CLUSTER_CHANGE_RESOURCE_TYPE_ADDED       |
                                               CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY    |
                                               CLUSTER_CHANGE_NETWORK_STATE             |
                                               CLUSTER_CHANGE_NETWORK_DELETED           |
                                               CLUSTER_CHANGE_NETWORK_ADDED             |
                                               CLUSTER_CHANGE_NETWORK_PROPERTY          |
                                               CLUSTER_CHANGE_NETINTERFACE_STATE        |
                                               CLUSTER_CHANGE_NETINTERFACE_DELETED      |
                                               CLUSTER_CHANGE_NETINTERFACE_ADDED        |
                                               CLUSTER_CHANGE_NETINTERFACE_PROPERTY     |
                                               CLUSTER_CHANGE_QUORUM_STATE              |
                                               CLUSTER_CHANGE_CLUSTER_STATE             |
                                               CLUSTER_CHANGE_CLUSTER_PROPERTY          |
                                               CLUSTER_CHANGE_CLUSTER_RECONNECT         |
                                               CLUSTER_CHANGE_HANDLE_CLOSE)

} CLUSTER_CHANGE;

#if ( CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8 )
//
// Cluster notification enums V2
//
typedef enum CLUSTER_NOTIFICATIONS_VERSION {
    CLUSTER_NOTIFICATIONS_V1    =    0x00000001,
    CLUSTER_NOTIFICATIONS_V2    =    0x00000002
} CLUSTER_NOTIFICATIONS_VERSION;

typedef enum CLUSTER_CHANGE_CLUSTER_V2 {
    CLUSTER_CHANGE_CLUSTER_RECONNECT_V2             =    0x00000001,
    CLUSTER_CHANGE_CLUSTER_STATE_V2                 =    0x00000002,
    CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2           =    0x00000004,
    CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2          =    0x00000008,
    CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2         =    0x00000010,
    CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2            =    0x00000020,
    CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2   =    0x00000040,
    CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2       =    0x00000080,
    CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2      =    0x00000100,
    CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2    =    0x00000200,
    CLUSTER_CHANGE_CLUSTER_RENAME_V2                =    0x00000400,
    CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2            =    0x00000800,
    CLUSTER_CHANGE_CLUSTER_UPGRADED_V2              =    0x00001000,
    CLUSTER_CHANGE_CLUSTER_ALL_V2                   =    (CLUSTER_CHANGE_CLUSTER_RECONNECT_V2             |
                                                          CLUSTER_CHANGE_CLUSTER_STATE_V2                 |
                                                          CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2           |
                                                          CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2          |
                                                          CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2         |
                                                          CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2            |
                                                          CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2   |
                                                          CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2       |
                                                          CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2      |
                                                          CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2    |
                                                          CLUSTER_CHANGE_CLUSTER_RENAME_V2                |
                                                          CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2            |
                                                          CLUSTER_CHANGE_CLUSTER_UPGRADED_V2)
} CLUSTER_CHANGE_CLUSTER_V2;

typedef enum CLUSTER_CHANGE_GROUP_V2 {
    CLUSTER_CHANGE_GROUP_DELETED_V2            =    0x00000001,
    CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2    =    0x00000002,
    CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2   =    0x00000004,
    CLUSTER_CHANGE_GROUP_STATE_V2              =    0x00000008,
    CLUSTER_CHANGE_GROUP_OWNER_NODE_V2         =    0x00000010,
    CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2   =    0x00000020,
    CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2     =    0x00000040,
    CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2    =    0x00000080,
    CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2      =    0x00000100,
    CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2       =    0x00000200,
    CLUSTER_CHANGE_GROUP_ALL_V2                =    (CLUSTER_CHANGE_GROUP_DELETED_V2            |
                                                     CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2    |
                                                     CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2   |
                                                     CLUSTER_CHANGE_GROUP_STATE_V2              |
                                                     CLUSTER_CHANGE_GROUP_OWNER_NODE_V2         |
                                                     CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2   |
                                                     CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2     |
                                                     CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2    |
                                                     CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2      |
                                                     CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2)
} CLUSTER_CHANGE_GROUP_V2;

typedef enum CLUSTER_CHANGE_GROUPSET_V2 {
    CLUSTER_CHANGE_GROUPSET_DELETED_v2            = 0x00000001,
    CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2    = 0x00000002,
    CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2   = 0x00000004,
    CLUSTER_CHANGE_GROUPSET_STATE_V2              = 0x00000008,
    CLUSTER_CHANGE_GROUPSET_GROUP_ADDED           = 0x00000010,
    CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED         = 0x00000020,
    CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2       = 0x00000040,
    CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2         = 0x00000080,
    CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2       = 0x00000100,
    CLUSTER_CHANGE_GROUPSET_ALL_V2                = (CLUSTER_CHANGE_GROUPSET_DELETED_v2 |
                                                       CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2 |
                                                       CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2 |
                                                       CLUSTER_CHANGE_GROUPSET_STATE_V2 |
                                                       CLUSTER_CHANGE_GROUPSET_GROUP_ADDED |
                                                       CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED |
                                                       CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2 |
                                                       CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2 |
                                                       CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2)
} CLUSTER_CHANGE_GROUPSET_V2;

typedef enum CLUSTER_CHANGE_RESOURCE_V2 {
    CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2    =    0x00000001,
    CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2   =    0x00000002,
    CLUSTER_CHANGE_RESOURCE_STATE_V2              =    0x00000004,
    CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2        =    0x00000008,
    CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2       =    0x00000010,
    CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2         =    0x00000020,
    CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2    =    0x00000040,
    CLUSTER_CHANGE_RESOURCE_DELETED_V2            =    0x00000080,
    CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2       =    0x00000100,
    CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2       =    0x00000200,
    CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2     =    0X00000400,
    CLUSTER_CHANGE_RESOURCE_ALL_V2                =    (CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2    |
                                                        CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2   |
                                                        CLUSTER_CHANGE_RESOURCE_STATE_V2              |
                                                        CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2        |
                                                        CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2       |
                                                        CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2         |
                                                        CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2    |
                                                        CLUSTER_CHANGE_RESOURCE_DELETED_V2            |
                                                        CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2       |
                                                        CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2       |
                                                        CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2 )
} CLUSTER_CHANGE_RESOURCE_V2;

typedef enum CLUSTER_CHANGE_RESOURCE_TYPE_V2 {
    CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2            =    0x00000001,
    CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2    =    0x00000002,
    CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2   =    0x00000004,
    CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2    =    0x00000008,
    CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2       =    0x00000010,
    CLUSTER_RESOURCE_TYPE_SPECIFIC_V2                  =    0x00000020,
    CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2                =    (CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2            |
                                                             CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2    |
                                                             CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2   |
                                                             CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2    |
                                                             CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2       |
                                                             CLUSTER_RESOURCE_TYPE_SPECIFIC_V2)
} CLUSTER_CHANGE_RESOURCE_TYPE_V2;

typedef enum CLUSTER_CHANGE_NETINTERFACE_V2 {
    CLUSTER_CHANGE_NETINTERFACE_DELETED_V2            =    0x00000001,
    CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2    =    0x00000002,
    CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2   =    0x00000004,
    CLUSTER_CHANGE_NETINTERFACE_STATE_V2              =    0x00000008,
    CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2       =    0x00000010,
    CLUSTER_CHANGE_NETINTERFACE_ALL_V2                =    (CLUSTER_CHANGE_NETINTERFACE_DELETED_V2            |
                                                            CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2    |
                                                            CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2   |
                                                            CLUSTER_CHANGE_NETINTERFACE_STATE_V2              |
                                                            CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2)
} CLUSTER_CHANGE_NETINTERFACE_V2;

typedef enum CLUSTER_CHANGE_NETWORK_V2 {
    CLUSTER_CHANGE_NETWORK_DELETED_V2            =    0x00000001,
    CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2    =    0x00000002,
    CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2   =    0x00000004,
    CLUSTER_CHANGE_NETWORK_STATE_V2              =    0x00000008,
    CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2       =    0x00000010,
    CLUSTER_CHANGE_NETWORK_ALL_V2                =    (CLUSTER_CHANGE_NETWORK_DELETED_V2            |
                                                       CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2    |
                                                       CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2   |
                                                       CLUSTER_CHANGE_NETWORK_STATE_V2              |
                                                       CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2)
} CLUSTER_CHANGE_NETWORK_V2;

typedef enum CLUSTER_CHANGE_NODE_V2 {
    CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2   =    0x00000001,
    CLUSTER_CHANGE_NODE_DELETED_V2              =    0x00000002,
    CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2      =    0x00000004,
    CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2     =    0x00000008,
    CLUSTER_CHANGE_NODE_STATE_V2                =    0x00000010,
    CLUSTER_CHANGE_NODE_GROUP_GAINED_V2         =    0x00000020,
    CLUSTER_CHANGE_NODE_GROUP_LOST_V2           =    0x00000040,
    CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2         =    0x00000080,
    CLUSTER_CHANGE_NODE_ALL_V2                  =    (CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2   |
                                                      CLUSTER_CHANGE_NODE_DELETED_V2              |
                                                      CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2      |
                                                      CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2     |
                                                      CLUSTER_CHANGE_NODE_STATE_V2                |
                                                      CLUSTER_CHANGE_NODE_GROUP_GAINED_V2         |
                                                      CLUSTER_CHANGE_NODE_GROUP_LOST_V2           |
                                                      CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2)
} CLUSTER_CHANGE_NODE_V2;

typedef enum CLUSTER_CHANGE_REGISTRY_V2 {
    CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2   =    0x00000001,
    CLUSTER_CHANGE_REGISTRY_NAME_V2         =    0x00000002,
    CLUSTER_CHANGE_REGISTRY_SUBTREE_V2      =    0x00000004,
    CLUSTER_CHANGE_REGISTRY_VALUE_V2        =    0x00000008,
    CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2 =    0x00000010,
    CLUSTER_CHANGE_REGISTRY_ALL_V2          =    (CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2   |
                                                  CLUSTER_CHANGE_REGISTRY_NAME_V2         |
                                                  CLUSTER_CHANGE_REGISTRY_SUBTREE_V2      |
                                                  CLUSTER_CHANGE_REGISTRY_VALUE_V2        |
                                                  CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2)
} CLUSTER_CHANGE_REGISTRY_V2;

typedef enum CLUSTER_CHANGE_QUORUM_V2 {
    CLUSTER_CHANGE_QUORUM_STATE_V2   =    0x00000001,
    CLUSTER_CHANGE_QUORUM_ALL_V2     =    (CLUSTER_CHANGE_QUORUM_STATE_V2)
} CLUSTER_CHANGE_QUORUM_V2;

typedef enum CLUSTER_CHANGE_SHARED_VOLUME_V2 {
    CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2   = 0x00000001,
    CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2   = 0x00000002,
    CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2 = 0x00000004,
    CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2     = (CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2   |
                                               CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2   |
                                               CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2)
} CLUSTER_CHANGE_SHARED_VOLUME_V2;

typedef enum CLUSTER_CHANGE_SPACEPORT_V2 {
    CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2             = 0x00000001,
} CLUSTER_CHANGE_SPACEPORT_V2;

typedef enum CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 {
    CLUSTER_CHANGE_UPGRADE_NODE_PREPARE     = 0x00000001,
    CLUSTER_CHANGE_UPGRADE_NODE_COMMIT      = 0x00000002,
    CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT  = 0x00000004,
    CLUSTER_CHANGE_UPGRADE_ALL              = (CLUSTER_CHANGE_UPGRADE_NODE_PREPARE    |
                                               CLUSTER_CHANGE_UPGRADE_NODE_COMMIT     |
                                               CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT)
} CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2;

typedef enum CLUSTER_OBJECT_TYPE {
    CLUSTER_OBJECT_TYPE_NONE                =    0x00000000,
    CLUSTER_OBJECT_TYPE_CLUSTER             =    0x00000001,
    CLUSTER_OBJECT_TYPE_GROUP               =    0x00000002,
    CLUSTER_OBJECT_TYPE_RESOURCE            =    0x00000003,
    CLUSTER_OBJECT_TYPE_RESOURCE_TYPE       =    0x00000004,
    CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE   =    0x00000005,
    CLUSTER_OBJECT_TYPE_NETWORK             =    0x00000006,
    CLUSTER_OBJECT_TYPE_NODE                =    0x00000007,
    CLUSTER_OBJECT_TYPE_REGISTRY            =    0x00000008,
    CLUSTER_OBJECT_TYPE_QUORUM              =    0x00000009,
    CLUSTER_OBJECT_TYPE_SHARED_VOLUME       =    0x0000000a,
    CLUSTER_OBJECT_TYPE_GROUPSET            =    0x0000000d,
    CLUSTER_OBJECT_TYPE_AFFINITYRULE        =    0x00000010,
    CLUSTER_OBJECT_TYPE_FAULTDOMAIN         =    0x00000011,

} CLUSTER_OBJECT_TYPE;


typedef enum CLUSTERSET_OBJECT_TYPE {
    CLUSTERSET_OBJECT_TYPE_NONE             =    0x00000000,
    CLUSTERSET_OBJECT_TYPE_MEMBER           =    0x00000001,
    CLUSTERSET_OBJECT_TYPE_WORKLOAD         =    0x00000002,
    CLUSTERSET_OBJECT_TYPE_DATABASE         =    0x00000003,
} CLUSTERSET_OBJECT_TYPE;
//
// Cluster notification structs V2
//
typedef struct _NOTIFY_FILTER_AND_TYPE
{
    DWORD dwObjectType;     // Uses CLUSTER_OBJECT_TYPE, but used for RPC so left
                            // as DWORD
    LONGLONG FilterFlags;
} NOTIFY_FILTER_AND_TYPE, *PNOTIFY_FILTER_AND_TYPE;

// Membership info returned as an array of up nodes
typedef struct _CLUSTER_MEMBERSHIP_INFO {
    BOOL  HasQuorum;
    DWORD UpnodesSize;
    BYTE  Upnodes[1];
} CLUSTER_MEMBERSHIP_INFO, *PCLUSTER_MEMBERSHIP_INFO;

#endif // (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)

#endif // _CLUSTER_API_TYPES_

#if ( !MIDL_PASS && !__midl )
#if ( CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8 )
//
// V2 Notifications DLL Functions
//
HCHANGE
WINAPI
CreateClusterNotifyPortV2(
    _In_  HCHANGE hChange,
    _In_  HCLUSTER hCluster,
    _In_  NOTIFY_FILTER_AND_TYPE * Filters,
    _In_  DWORD dwFilterCount,
    _In_  DWORD_PTR dwNotifyKey
);

typedef HCHANGE
(WINAPI * PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2)(
    _In_  HCHANGE hChange,
    _In_  HCLUSTER hCluster,
    _In_  NOTIFY_FILTER_AND_TYPE * Filters,
    _In_  DWORD dwFilterCount,
    _In_  DWORD_PTR dwNotifyKey
    );

DWORD
WINAPI
RegisterClusterNotifyV2(
    _In_  HCHANGE hChange,
    _In_  NOTIFY_FILTER_AND_TYPE Filter,
    _In_  HANDLE hObject,
    _In_  DWORD_PTR dwNotifyKey
);

typedef DWORD
(WINAPI * PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2)(
    _In_  HCHANGE hChange,
    _In_  NOTIFY_FILTER_AND_TYPE Filter,
    _In_  HANDLE hObject,
    _In_  DWORD_PTR dwNotifyKey
    );

DWORD
WINAPI
GetNotifyEventHandle(
    _In_  HCHANGE  hChange,
    _Out_ LPHANDLE lphTargetEvent
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2)(
    _In_  HCHANGE  hChange,
    _Out_ LPHANDLE lphTargetEvent
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterNotifyV2(
    _In_                                                            HCHANGE hChange,
    _Out_                                                           DWORD_PTR* lpdwNotifyKey,
    _Inout_opt_                                                     PNOTIFY_FILTER_AND_TYPE pFilterAndType,
    _Inout_updates_bytes_opt_(*lpbBufferSize)                              BYTE* buffer,
    _Inout_opt_                                                     LPDWORD lpbBufferSize,
    _Inout_updates_to_opt_(*lpcchObjectId, *lpcchObjectId + 1 )    LPWSTR lpszObjectId,
    _Inout_opt_                                                     LPDWORD lpcchObjectId,
    _Inout_updates_to_opt_(*lpcchParentId, *lpcchParentId + 1 )    LPWSTR lpszParentId,
    _Inout_opt_                                                     LPDWORD lpcchParentId,
    _Inout_updates_to_opt_(*lpcchName, *lpcchName + 1 )            LPWSTR lpszName,
    _Inout_opt_                                                     LPDWORD lpcchName,
    _Inout_updates_to_opt_(*lpcchType, *lpcchType + 1 )            LPWSTR lpszType,
    _Inout_opt_                                                     LPDWORD lpcchType,
    _In_opt_                                                        DWORD dwMilliseconds
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_NOTIFY_V2)(
    _In_      HCHANGE hChange,
    _Out_     DWORD_PTR* lpdwNotifyKey,
    _Inout_opt_ PNOTIFY_FILTER_AND_TYPE pFilterAndType,
    _Inout_opt_ BYTE* buffer,
    _Inout_opt_ LPDWORD lpcchBufferSize,
    _Inout_opt_ LPWSTR lpszObjectId,
    _Inout_opt_ LPDWORD lpcchObjectId,
    _Inout_opt_ LPWSTR lpszParentId,
    _Inout_opt_ LPDWORD lpcchParentId,
    _Inout_opt_ LPWSTR lpszName,
    _Inout_opt_ LPDWORD lpcchName,
    _Inout_opt_ LPWSTR lpszType,
    _Inout_opt_ LPDWORD lpcchType,
    _In_opt_  DWORD dwMilliseconds
    );


#endif // (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)

//
// Begin original notification DLL function defs
//
HCHANGE
WINAPI
CreateClusterNotifyPort(
    _In_ HCHANGE hChange,
    _In_ HCLUSTER hCluster,
    _In_ DWORD dwFilter,
    _In_ DWORD_PTR dwNotifyKey
    );

typedef HCHANGE
(WINAPI * PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT)(
    _In_ HCHANGE hChange,
    _In_ HCLUSTER hCluster,
    _In_ DWORD dwFilter,
    _In_ DWORD_PTR dwNotifyKey
    );

DWORD
WINAPI
RegisterClusterNotify(
    _In_ HCHANGE hChange,
    _In_ DWORD dwFilterType,
    _In_ HANDLE hObject,
    _In_ DWORD_PTR dwNotifyKey
    );

typedef DWORD
(WINAPI * PCLUSAPI_REGISTER_CLUSTER_NOTIFY)(
    _In_ HCHANGE hChange,
    _In_ DWORD dwFilterType,
    _In_ HANDLE hObject,
    _In_ DWORD_PTR dwNotifyKey
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterNotify(
    _In_ HCHANGE hChange,
    _Out_ DWORD_PTR *lpdwNotifyKey,
    _Out_ LPDWORD lpdwFilterType,
    _Out_writes_(*lpcchName) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName,
    _In_ DWORD dwMilliseconds
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_NOTIFY)(
    _In_ HCHANGE hChange,
    _Out_ DWORD_PTR *lpdwNotifyKey,
    _Out_ LPDWORD lpdwFilterType,
    _Out_writes_to_opt_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName,
    _In_ DWORD dwMilliseconds
    );

BOOL
WINAPI
CloseClusterNotifyPort(
    _In_ HCHANGE hChange
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT)(
    _In_ HCHANGE hChange
    );

#endif // MIDL_PASS

//
// Enumeration routines
//

#ifndef _CLUSTER_API_TYPES_
//
// Define enumerable types
//
typedef enum CLUSTER_ENUM {
    CLUSTER_ENUM_NODE                   = 0x00000001,
    CLUSTER_ENUM_RESTYPE                = 0x00000002,
    CLUSTER_ENUM_RESOURCE               = 0x00000004,
    CLUSTER_ENUM_GROUP                  = 0x00000008,
    CLUSTER_ENUM_NETWORK                = 0x00000010,
    CLUSTER_ENUM_NETINTERFACE           = 0x00000020,
    #if (CLUSAPI_VERSION >= CLUSAPI_VERSION_ZN)
    CLUSTER_ENUM_CAPACITY_NODE          = 0x10000000,
    #endif
    CLUSTER_ENUM_SHARED_VOLUME_GROUP    = 0x20000000,
    CLUSTER_ENUM_SHARED_VOLUME_RESOURCE = 0x40000000,
    CLUSTER_ENUM_INTERNAL_NETWORK       = 0x80000000,

    CLUSTER_ENUM_ALL                = (CLUSTER_ENUM_NODE      |
                                       CLUSTER_ENUM_RESTYPE   |
                                       CLUSTER_ENUM_RESOURCE  |
                                       CLUSTER_ENUM_GROUP     |
                                       CLUSTER_ENUM_NETWORK   |
                                       CLUSTER_ENUM_NETINTERFACE)

} CLUSTER_ENUM;

#endif // _CLUSTER_API_TYPES_

#if ( !MIDL_PASS && !__midl )
HCLUSENUM
WINAPI
ClusterOpenEnum(
    _In_ HCLUSTER hCluster,
    _In_ DWORD dwType
    );

typedef HCLUSENUM
(WINAPI * PCLUSAPI_CLUSTER_OPEN_ENUM)(
    _In_ HCLUSTER hCluster,
    _In_ DWORD dwType
    );

DWORD
WINAPI
ClusterGetEnumCount(
    _In_ HCLUSENUM hEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GET_ENUM_COUNT)(
    _In_ HCLUSENUM hEnum
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterEnum(
    _In_ HCLUSENUM hEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_ENUM)(
    _In_ HCLUSENUM hEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterCloseEnum(
    _In_ HCLUSENUM hEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_CLOSE_ENUM)(
    _In_ HCLUSENUM hEnum
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)

HCLUSENUMEX
WINAPI
ClusterOpenEnumEx(
    _In_ HCLUSTER hCluster,
    _In_ DWORD dwType,
    _In_opt_ PVOID pOptions
    );

typedef HCLUSENUMEX
(WINAPI * PCLUSAPI_CLUSTER_OPEN_ENUM_EX)(
        _In_ HCLUSTER hCluster,
        _In_ DWORD dwType,
        _In_opt_ PVOID pOptions
        );

DWORD
WINAPI
ClusterGetEnumCountEx(
    _In_ HCLUSENUMEX hClusterEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX)(
    _In_ HCLUSENUMEX hClusterEnum
    );

DWORD
WINAPI
ClusterEnumEx(
    _In_ HCLUSENUMEX hClusterEnum,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_ENUM_EX)(
    _In_ HCLUSENUMEX hClusterEnum,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

DWORD
WINAPI
ClusterCloseEnumEx(
    _In_ HCLUSENUMEX hClusterEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_CLOSE_ENUM_EX)(
    _In_ HCLUSENUMEX hClusterEnum
    );

#endif // CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)

// Group Set StatusInformation Flags; These flags are set only by Cluster Service Only;

#define CLUSGROUPSET_STATUS_GROUPS_PENDING                                          0x0000000000000001
#define CLUSGROUPSET_STATUS_GROUPS_ONLINE                                           0x0000000000000002
#define CLUSGROUPSET_STATUS_OS_HEARTBEAT                                            0x0000000000000004
#define CLUSGROUPSET_STATUS_APPLICATION_READY                                       0x0000000000000008

HGROUPSET
WINAPI
CreateClusterGroupSet(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR groupSetName
    );

typedef HGROUPSET
(WINAPI * PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupSetName
    );

HGROUPSET
WINAPI
OpenClusterGroupSet(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupSetName
    );

typedef HGROUPSET
(WINAPI * PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupSetName
    );

BOOL
WINAPI
CloseClusterGroupSet(
    _In_ HGROUPSET hGroupSet
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET)(
    _In_ HGROUPSET hGroupSet
    );

DWORD
WINAPI
DeleteClusterGroupSet(
    _In_ HGROUPSET hGroupSet
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET)(
    _In_ HGROUPSET hGroupSet
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
DeleteClusterGroupSetEx(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET_EX)(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
ClusterAddGroupToGroupSet(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUP hGroup
    );

DWORD
WINAPI
ClusterAddGroupToGroupSetWithDomains(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUP hGroup,
    _In_ DWORD faultDomain,
    _In_ DWORD updateDomain
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET)(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUP hGroup
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ClusterAddGroupToGroupSetWithDomainsEx(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUP hGroup,
    _In_ DWORD faultDomain,
    _In_ DWORD updateDomain,
    _In_opt_ LPCWSTR lpszReason
    );


typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUPSET_WITH_DOMAINS_EX)(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUP hGroup,
    _In_ DWORD faultDomain,
    _In_ DWORD updateDomain,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
ClusterRemoveGroupFromGroupSet(
    _In_ HGROUP hGroup
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET)(
    _In_ HGROUPSET hGroupSet
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ClusterRemoveGroupFromGroupSetEx(
    _In_ HGROUP hGroup,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET_EX)(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
ClusterGroupSetControl(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL)(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

__success( return == ERROR_SUCCESS )
DWORD
WINAPI
ClusterGroupSetControlEx(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL_EX)(
    _In_ HGROUPSET hGroupSet,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
AddClusterGroupDependency(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUP hProviderGroup
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY)(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUP hProviderGroup
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
AddClusterGroupDependencyEx(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUP hProviderGroup,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY_EX)(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUP hProviderGroup,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
SetGroupDependencyExpression(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszDependencyExpression
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION)(
    _In_ HGROUP hGroupSet,
    _In_ LPCWSTR lpszDependencyExpression
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
SetGroupDependencyExpressionEx(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszDependencyExpression,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION_EX)(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszDependencyExpression,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
RemoveClusterGroupDependency(
    _In_ HGROUP hGroup,
    _In_ HGROUP hDependsOn
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY)(
    _In_ HGROUP hGroup,
    _In_ HGROUP hDependsOn
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
RemoveClusterGroupDependencyEx(
    _In_ HGROUP hGroup,
    _In_ HGROUP hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY_EX)(
    _In_ HGROUP hGroup,
    _In_ HGROUP hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
AddClusterGroupSetDependency(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ HGROUPSET hProviderGroupSet
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY)(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ HGROUPSET hProviderGroupSet
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
AddClusterGroupSetDependencyEx(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ HGROUPSET hProviderGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX)(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ HGROUPSET hProviderGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
SetClusterGroupSetDependencyExpression(
    _In_ HGROUPSET hGroupSet,
    _In_ LPCWSTR lpszDependencyExprssion
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION)(
    _In_ HGROUPSET hGroupSet,
    _In_ LPCWSTR lpszDependencyExpression
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
SetClusterGroupSetDependencyExpressionEx(
    _In_ HGROUPSET hGroupSet,
    _In_ LPCWSTR lpszDependencyExpression,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION_EX)(
    _In_ HGROUPSET hGroupSet,
    _In_ LPCWSTR lpszDependencyExpression,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
RemoveClusterGroupSetDependency(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUPSET hDependsOn
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY)(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUPSET hDependsOn
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
RemoveClusterGroupSetDependencyEx(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUPSET hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX)(
    _In_ HGROUPSET hGroupSet,
    _In_ HGROUPSET hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
AddClusterGroupToGroupSetDependency(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUPSET hProviderGroupSet
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY)(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUPSET hProviderGroupSet
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
AddClusterGroupToGroupSetDependencyEx(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUPSET hProviderGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX)(
    _In_ HGROUP hDependentGroup,
    _In_ HGROUPSET hProviderGroupSet,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
RemoveClusterGroupToGroupSetDependency(
    _In_ HGROUP hGroup,
    _In_ HGROUPSET hDependsOn
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY)(
    _In_ HGROUP hGroup,
    _In_ HGROUPSET hDependsOn
    );


#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
RemoveClusterGroupToGroupSetDependencyEx(
    _In_ HGROUP hGroup,
    _In_ HGROUPSET hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX)(
    _In_ HGROUP hGroup,
    _In_ HGROUPSET hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

HGROUPSETENUM
WINAPI
    ClusterGroupSetOpenEnum(
    IN HCLUSTER hCluster
    );

DWORD
WINAPI
    ClusterGroupSetGetEnumCount(
    IN HGROUPSETENUM hGroupSetEnum
    );

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
    ClusterGroupSetEnum(
    _In_ HGROUPSETENUM hGroupSetEnum,
    _In_ DWORD dwIndex,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterGroupSetCloseEnum(
    IN HGROUPSETENUM hGroupSetEnum
    );

HCLUSTER
WINAPI
GetClusterFromGroupSet(
    _In_ HGROUPSET hGroupSet
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET)(
    _In_ HGROUPSET hGroupSet
    );

DWORD
WINAPI
AddCrossClusterGroupSetDependency(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ LPCWSTR lpRemoteClusterName,
    _In_ LPCWSTR lpRemoteGroupSetName
);

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY)(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ LPCWSTR lpRemoteClusterName,
    _In_ LPCWSTR lpRemoteGroupSetName
);

DWORD
WINAPI
RemoveCrossClusterGroupSetDependency(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ LPCWSTR lpRemoteClusterName,
    _In_ LPCWSTR lpRemoteGroupSetName
);

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY)(
    _In_ HGROUPSET hDependentGroupSet,
    _In_ LPCWSTR lpRemoteClusterName,
    _In_ LPCWSTR lpRemoteGroupSetName
);

#define CLUSTER_AVAILABILITY_SET_CONFIG_V1 0x00000001

typedef struct CLUSTER_AVAILABILITY_SET_CONFIG
{
    DWORD dwVersion;
    DWORD dwUpdateDomains;
    DWORD dwFaultDomains;
    BOOL  bReserveSpareNode;
} CLUSTER_AVAILABILITY_SET_CONFIG, *PCLUSTER_AVAILABILITY_SET_CONFIG;

HGROUPSET
WINAPI
CreateClusterAvailabilitySet(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpAvailabilitySetName,
    _In_ PCLUSTER_AVAILABILITY_SET_CONFIG pAvailabilitySetConfig
);

typedef HGROUPSET
(WINAPI *PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpAvailabilitySetName,
    _In_ PCLUSTER_AVAILABILITY_SET_CONFIG pAvailabilitySetConfig
);

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterNodeReplacement(
    __in HCLUSTER   hCluster,
    __in LPCWSTR    lpszNodeNameCurrent,
    __in LPCWSTR    lpszNodeNameNew);


// affinity rule

DWORD
WINAPI
ClusterCreateAffinityRule(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName,
    __in CLUS_AFFINITY_RULE_TYPE ruleType
);

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE)(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName,
    __in CLUS_AFFINITY_RULE_TYPE ruleType
);

DWORD
WINAPI
ClusterRemoveAffinityRule(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName
);

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE)(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName
);

DWORD
WINAPI
ClusterAddGroupToAffinityRule(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName,
    __in HGROUP hGroup
);

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE)(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName,
    __in HGROUP hGroup
);

DWORD
WINAPI
ClusterRemoveGroupFromAffinityRule(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName,
    __in HGROUP hGroup
);

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE)(
    __in HCLUSTER hCluster,
    __in LPCWSTR ruleName,
    __in HGROUP hGroup
);

DWORD
WINAPI
ClusterAffinityRuleControl(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR affinityRuleName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR affinityRuleName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
);


#endif //(CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)

#endif // MIDL_PASS


#ifndef _CLUSTER_API_TYPES_
//
// Define enumerable node types
//
#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)

typedef enum CLUSTER_NODE_ENUM {
    CLUSTER_NODE_ENUM_NETINTERFACES           = 0x00000001,
    CLUSTER_NODE_ENUM_GROUPS                  = 0x00000002,
    CLUSTER_NODE_ENUM_PREFERRED_GROUPS        = 0x00000004,

    CLUSTER_NODE_ENUM_ALL                     = (CLUSTER_NODE_ENUM_NETINTERFACES |
                                                 CLUSTER_NODE_ENUM_GROUPS)
} CLUSTER_NODE_ENUM;

#else

typedef enum CLUSTER_NODE_ENUM {
    CLUSTER_NODE_ENUM_NETINTERFACES = 0x00000001,

    CLUSTER_NODE_ENUM_ALL           = (CLUSTER_NODE_ENUM_NETINTERFACES)

} CLUSTER_NODE_ENUM;

#endif

//
// Node-related structures and types.
//
typedef enum CLUSTER_NODE_STATE {
    ClusterNodeStateUnknown = -1,
    ClusterNodeUp,
    ClusterNodeDown,
    ClusterNodePaused,
    ClusterNodeJoining
} CLUSTER_NODE_STATE;

#if (CLUSAPI_VERSION > CLUSAPI_VERSION_WINTHRESHOLD)

//
// StorageNode-related structures and types.
//
typedef enum CLUSTER_STORAGENODE_STATE {
    ClusterStorageNodeStateUnknown = 0,
    ClusterStorageNodeUp,
    ClusterStorageNodeDown,
    ClusterStorageNodePaused,
    ClusterStorageNodeStarting,
    ClusterStorageNodeStopping,
} CLUSTER_STORAGENODE_STATE;

#endif

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)

typedef enum CLUSTER_NODE_DRAIN_STATUS {
    NodeDrainStatusNotInitiated = 0,
    NodeDrainStatusInProgress,
    NodeDrainStatusCompleted,
    NodeDrainStatusFailed,
    ClusterNodeDrainStatusCount
} CLUSTER_NODE_DRAIN_STATUS;

#endif //CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)

typedef enum CLUSTER_NODE_STATUS
{
    NodeStatusNormal                    =  0x0,
    NodeStatusIsolated                  =  0x1,
    NodeStatusQuarantined               =  0x2,
    NodeStatusDrainInProgress           =  0x4,
    NodeStatusDrainCompleted            =  0x8,
    NodeStatusDrainFailed               =  0x10,
    NodeStatusAvoidPlacement            =  0x20,
    NodeStatusMax                       =  (NodeStatusIsolated | NodeStatusQuarantined | NodeStatusDrainFailed | NodeStatusAvoidPlacement)
} CLUSTER_NODE_STATUS;

#endif //CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

typedef enum CLUSTER_NODE_FAILBACK_STATUS {
    NodeFailbackStatusNotInitiated = 0,
    NodeFailbackStatusInProgress,
    NodeFailbackStatusCompleted,
    NodeFailbackStatusFailed,
    ClusterNodeFailbackStatusCount
} CLUSTER_NODE_FAILBACK_STATUS;

#endif //CLUSAPI_VERSION >= CLUSAPI_VERSION_NI

#endif // _CLUSTER_API_TYPES_

//
// Interfaces for managing the nodes of a cluster.
//

#if ( !MIDL_PASS && !__midl )
HNODE
WINAPI
OpenClusterNode(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNodeName
    );

typedef HNODE
(WINAPI * PCLUSAPI_OPEN_CLUSTER_NODE)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNodeName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
HNODE
WINAPI
OpenClusterNodeEx(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszNodeName,
    _In_      DWORD dwDesiredAccess,
    _Out_opt_ DWORD* lpdwGrantedAccess
    );

typedef HNODE
(WINAPI * PCLUSAPI_OPEN_CLUSTER_NODE_EX)(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszNodeName,
    _In_      DWORD   dwDesiredAccess,
    _Out_opt_ LPDWORD lpdwGrantedAccess
    );
#endif

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
HNODE
WINAPI
OpenClusterNodeById(
    __in HCLUSTER hCluster,
    __in DWORD nodeId
    );

typedef HNODE
(WINAPI * PCLUSAPI_OPEN_NODE_BY_ID)(
    __in HCLUSTER hCluster,
    __in DWORD nodeId
    );
#endif

BOOL
WINAPI
CloseClusterNode(
    _In_ HNODE hNode
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_NODE)(
    _In_ HNODE hNode
    );

CLUSTER_NODE_STATE
WINAPI
GetClusterNodeState(
    _In_ HNODE hNode
    );

typedef CLUSTER_NODE_STATE
(WINAPI * PCLUSAPI_GET_CLUSTER_NODE_STATE)(
    _In_ HNODE hNode
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterNodeId(
    _In_opt_ HNODE hNode,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszNodeId,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_NODE_ID)(
    _In_opt_ HNODE hNode,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszNodeId,
    _Inout_ LPDWORD lpcchName
    );

#define GetCurrentClusterNodeId(_lpszNodeId_, _lpcchName_) \
    GetClusterNodeId(NULL, (_lpszNodeId_), (_lpcchName_))

HCLUSTER
WINAPI
GetClusterFromNode(
    _In_ HNODE hNode
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_GET_CLUSTER_FROM_NODE)(
    _In_ HNODE hNode
    );

DWORD
WINAPI
PauseClusterNode(
    _In_ HNODE hNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_PAUSE_CLUSTER_NODE)(
    _In_ HNODE hNode
    );

DWORD
WINAPI
ResumeClusterNode(
    _In_ HNODE hNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_RESUME_CLUSTER_NODE)(
    _In_ HNODE hNode
    );

DWORD
WINAPI
EvictClusterNode(
    _In_ HNODE hNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_EVICT_CLUSTER_NODE)(
    _In_ HNODE hNode
    );

HNETINTERFACEENUM
WINAPI
ClusterNetInterfaceOpenEnum(
   _In_ HCLUSTER hCluster,
   _In_opt_ LPCWSTR lpszNodeName,
   _In_opt_ LPCWSTR lpszNetworkName
    );

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterNetInterfaceEnum(
    _In_ HNETINTERFACEENUM hNetInterfaceEnum,
    _In_ DWORD dwIndex,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
);

DWORD
WINAPI
ClusterNetInterfaceCloseEnum(
  _In_ HNETINTERFACEENUM hNetInterfaceEnum
);

HNODEENUM
WINAPI
ClusterNodeOpenEnum(
    _In_ HNODE hNode,
    _In_ DWORD dwType
    );

typedef HNODEENUM
(WINAPI * PCLUSAPI_CLUSTER_NODE_OPEN_ENUM)(
    _In_ HNODE hNode,
    _In_ DWORD dwType
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)

HNODEENUMEX
WINAPI
ClusterNodeOpenEnumEx(
    _In_ HNODE hNode,
    _In_ DWORD dwType,
    _In_opt_ PVOID pOptions
    );

typedef HNODEENUMEX
(WINAPI * PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX)(
        _In_ HNODE hNode,
        _In_ DWORD dwType,
        _In_opt_ PVOID pOptions
        );

DWORD
WINAPI
ClusterNodeGetEnumCountEx(
    _In_ HNODEENUMEX hNodeEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX)(
    _In_ HNODEENUMEX hNodeEnum
    );

DWORD
WINAPI
ClusterNodeEnumEx(
    _In_ HNODEENUMEX hNodeEnum,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_ENUM_EX)(
    _In_ HNODEENUMEX hNodeEnum,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

DWORD
WINAPI
ClusterNodeCloseEnumEx(
    _In_ HNODEENUMEX hNodeEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX)(
    _In_ HNODEENUMEX hNodeEnum
    );

#endif

DWORD
WINAPI
ClusterNodeGetEnumCount(
    _In_ HNODEENUM hNodeEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT)(
    _In_ HNODEENUM hNodeEnum
    );

DWORD
WINAPI
ClusterNodeCloseEnum(
    _In_ HNODEENUM hNodeEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM)(
    _In_ HNODEENUM hNodeEnum
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterNodeEnum(
    _In_ HNODEENUM hNodeEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_ENUM)(
    _In_ HNODEENUM hNodeEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
EvictClusterNodeEx(
    _In_ HNODE hNode,
    _In_ DWORD dwTimeOut,
    _Out_ HRESULT * phrCleanupStatus
    );

typedef DWORD
(WINAPI * PCLUSAPI_EVICT_CLUSTER_NODE_EX)(
    _In_ HNODE hNode,
    _In_ DWORD dwTimeOut,
    _Out_ HRESULT * phrCleanupStatus
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
EvictClusterNodeEx2(
    _In_ HNODE hNode,
    _In_ DWORD dwTimeout,
    _Out_ HRESULT* phrCleanupStatus,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_EVICT_CLUSTER_NODE_EX2)(
    _In_ HNODE hNode,
    _In_ DWORD dwTimeout,
    _Out_ HRESULT* phrCleanupStatus,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

#endif // MIDL_PASS


//
// Interfaces for managing the resource types in a cluster
//

#if ( !MIDL_PASS && !__midl )
HKEY
WINAPI
GetClusterResourceTypeKey(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszTypeName,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszTypeName,
    _In_ REGSAM samDesired
    );

#endif // MIDL_PASS

#ifndef _CLUSTER_API_TYPES_
//
// Define enumerable group types
//
typedef enum CLUSTER_GROUP_ENUM {
    CLUSTER_GROUP_ENUM_CONTAINS     = 0x00000001,
    CLUSTER_GROUP_ENUM_NODES        = 0x00000002,

    CLUSTER_GROUP_ENUM_ALL          = (CLUSTER_GROUP_ENUM_CONTAINS |
                                       CLUSTER_GROUP_ENUM_NODES)
} CLUSTER_GROUP_ENUM;

//
// Interfaces for managing the failover groups in a cluster.
//
typedef enum CLUSTER_GROUP_STATE {
    ClusterGroupStateUnknown = -1,
    ClusterGroupOnline,
    ClusterGroupOffline,
    ClusterGroupFailed,
    ClusterGroupPartialOnline,
    ClusterGroupPending
} CLUSTER_GROUP_STATE;

typedef enum CLUSTER_GROUP_PRIORITY
{
    PriorityDisabled = 0, // Groups with disabled priorities do not auto-start
    PriorityLow = 1000,
    PriorityMedium = 2000,
    PriorityHigh  = 3000,
} CLUSTER_GROUP_PRIORITY;

typedef enum CLUSTER_GROUP_AUTOFAILBACK_TYPE
{
    ClusterGroupPreventFailback = 0,
    ClusterGroupAllowFailback,
    ClusterGroupFailbackTypeCount
} CLUSTER_GROUP_AUTOFAILBACK_TYPE, CGAFT;


#if CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

#define CLUSTER_GROUP_ENUM_ITEM_VERSION_1 0x00000001
#define CLUSTER_GROUP_ENUM_ITEM_VERSION   CLUSTER_GROUP_ENUM_ITEM_VERSION_1

typedef struct _CLUSTER_GROUP_ENUM_ITEM {
    DWORD dwVersion;
    DWORD cbId;
    LPWSTR lpszId;
    DWORD cbName;
    LPWSTR lpszName;
    CLUSTER_GROUP_STATE state;
    DWORD cbOwnerNode;
    LPWSTR lpszOwnerNode;
    DWORD dwFlags;
    DWORD cbProperties;
    PVOID pProperties;
    DWORD cbRoProperties;
    PVOID pRoProperties;
} CLUSTER_GROUP_ENUM_ITEM, *PCLUSTER_GROUP_ENUM_ITEM;

#define CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1 0x00000001
#define CLUSTER_RESOURCE_ENUM_ITEM_VERSION   CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1

typedef struct _CLUSTER_RESOURCE_ENUM_ITEM {
    DWORD dwVersion;
    DWORD cbId;
    LPWSTR lpszId;
    DWORD cbName;
    LPWSTR lpszName;
    DWORD cbOwnerGroupName;
    LPWSTR lpszOwnerGroupName;
    DWORD cbOwnerGroupId;
    LPWSTR lpszOwnerGroupId;
    DWORD cbProperties;
    PVOID pProperties;
    DWORD cbRoProperties;
    PVOID pRoProperties;
} CLUSTER_RESOURCE_ENUM_ITEM, *PCLUSTER_RESOURCE_ENUM_ITEM;

#endif  // CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8

#endif // _CLUSTER_API_TYPES_

#if ( !MIDL_PASS && !__midl )
HGROUP
WINAPI
CreateClusterGroup(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupName
    );

typedef HGROUP
(WINAPI * PCLUSAPI_CREATE_CLUSTER_GROUP)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupName
    );

HGROUP
WINAPI
OpenClusterGroup(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupName
    );

typedef HGROUP
(WINAPI * PCLUSAPI_OPEN_CLUSTER_GROUP)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
HGROUP
WINAPI
OpenClusterGroupEx(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszGroupName,
    _In_      DWORD dwDesiredAccess,
    _Out_opt_ DWORD* lpdwGrantedAccess
    );

typedef HGROUP
(WINAPI * PCLUSAPI_OPEN_CLUSTER_GROUP_EX)(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszGroupName,
    _In_      DWORD   dwDesiredAccess,
    _Out_opt_ LPDWORD lpdwGrantedAccess
    );

#endif

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)

// flags for PauseClusterNodeEx
#define CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR  0x00000001
#define CLUSAPI_NODE_AVOID_PLACEMENT                            0x00000002
#define CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE               0x00000004


DWORD
WINAPI
PauseClusterNodeEx(
    _In_ HNODE hNode,
    _In_ BOOL bDrainNode,
    _In_ DWORD dwPauseFlags,
    _In_opt_ HNODE hNodeDrainTarget
    );

typedef DWORD
(WINAPI * PCLUSAPI_PAUSE_CLUSTER_NODE_EX)(
    _In_ HNODE hNode,
    _In_ BOOL bDrainNode,
    _In_ DWORD dwPauseFlags,
    _In_opt_ HNODE hNodeDrainTarget
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
PauseClusterNodeEx2(
    _In_ HNODE hNode,
    _In_ BOOL bDrainNode,
    _In_ DWORD dwPauseFlags,
    _In_opt_ HNODE hNodeDrainTarget,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_PAUSE_CLUSTER_NODE_EX2)(
    _In_ HNODE hNode,
    _In_ BOOL bDrainNode,
    _In_ DWORD dwPauseFlags,
    _In_opt_ HNODE hNodeDrainTarget,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

typedef enum CLUSTER_NODE_RESUME_FAILBACK_TYPE
{
    DoNotFailbackGroups = 0,
    FailbackGroupsImmediately,
    FailbackGroupsPerPolicy,
    ClusterNodeResumeFailbackTypeCount
} CLUSTER_NODE_RESUME_FAILBACK_TYPE;

// flags for ResumeClusterNodeEx
#define CLUSAPI_NODE_RESUME_FAILBACK_STORAGE                   0x00000001
#define CLUSAPI_NODE_RESUME_FAILBACK_VMS                       0x00000002
#define CLUSAPI_NODE_RESUME_FAILBACK_PINNED_VMS_ONLY           0x00000004
#define CLUSAPI_NODE_RESUME_FAILBACK_VMS_FORCEFULLY            0x00000008


DWORD
WINAPI
ResumeClusterNodeEx(
    _In_ HNODE hNode,
    _In_ CLUSTER_NODE_RESUME_FAILBACK_TYPE eResumeFailbackType,
    _In_ DWORD dwResumeFlagsReserved
    );

typedef DWORD
(WINAPI * PCLUSAPI_RESUME_CLUSTER_NODE_EX)(
    _In_ HNODE hNode,
    _In_ CLUSTER_NODE_RESUME_FAILBACK_TYPE eResumeFailbackType,
    _In_ DWORD dwResumeFlagsReserved
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ResumeClusterNodeEx2(
    _In_ HNODE hNode,
    _In_ CLUSTER_NODE_RESUME_FAILBACK_TYPE eResumeFailbackType,
    _In_ DWORD dwResumeFlagsReserved,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_RESUME_CLUSTER_NODE_EX2)(
    _In_ HNODE hNode,
    _In_ CLUSTER_NODE_RESUME_FAILBACK_TYPE eResumeFailbackType,
    _In_ DWORD dwResumeFlagsReserved,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

// Group StatusInformation Flags; These flags are set only by Cluster Service Only;
#define CLUSGRP_STATUS_LOCKED_MODE                                             0x0000000000000001
#define CLUSGRP_STATUS_PREEMPTED                                               0x0000000000000002
#define CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE                               0x0000000000000004
#define CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING                              0x0000000000000008
#define CLUSGRP_STATUS_WAITING_TO_START                                        0x0000000000000010
#define CLUSGRP_STATUS_EMBEDDED_FAILURE                                        0x0000000000000020
#define CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT                    0x0000000000000040
#define CLUSGRP_STATUS_NETWORK_FAILURE                                         0x0000000000000080
#define CLUSGRP_STATUS_UNMONITORED                                             0x0000000000000100
#define CLUSGRP_STATUS_OS_HEARTBEAT                                            0x0000000000000200
#define CLUSGRP_STATUS_APPLICATION_READY                                       0x0000000000000400
#define CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER                            0x0000000000000800
#define CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES                                0x0000000000001000

HGROUP
WINAPI
CreateClusterGroupEx(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupName,
    _In_opt_ PCLUSTER_CREATE_GROUP_INFO pGroupInfo
    );

typedef HGROUP
(WINAPI * PCLUSAPI_CREATE_CLUSTER_GROUPEX)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszGroupName,
    _In_opt_ PCLUSTER_CREATE_GROUP_INFO pGroupInfo
    );

HGROUPENUMEX
WINAPI
ClusterGroupOpenEnumEx(
    _In_ HCLUSTER hCluster,
    _In_reads_bytes_opt_(cbProperties)LPCWSTR lpszProperties,
    _In_ DWORD cbProperties,
    _In_reads_bytes_opt_(cbRoProperties) LPCWSTR lpszRoProperties,
    _In_ DWORD cbRoProperties,
    _In_ DWORD dwFlags
    );

typedef HGROUPENUMEX
(WINAPI * PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX)(
    _In_ HCLUSTER hCluster,
    _In_reads_bytes_opt_(cbProperties)LPCWSTR lpszProperties,
    _In_ DWORD cbProperties,
    _In_reads_bytes_opt_(cbRoProperties) LPCWSTR lpszRoProperties,
    _In_ DWORD cbRoProperties,
    _In_ DWORD dwFlags
    );

DWORD
WINAPI
ClusterGroupGetEnumCountEx(
    _In_ HGROUPENUMEX hGroupEnumEx
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX)(
    _In_ HGROUPENUMEX hGroupEnumEx
    );

DWORD
WINAPI
ClusterGroupEnumEx(
    _In_ HGROUPENUMEX hGroupEnumEx,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_GROUP_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_ENUM_EX)(
    _In_ HGROUPENUMEX hGroupEnumEx,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_GROUP_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

DWORD
WINAPI
ClusterGroupCloseEnumEx(
    _In_ HGROUPENUMEX hGroupEnumEx
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX)(
    _In_ HGROUPENUMEX hGroupEnumEx
    );

// Resource StatusInformation Flags; These are set by Cluster Service Only;
#define CLUSRES_STATUS_LOCKED_MODE                                             0x0000000000000001
#define CLUSRES_STATUS_EMBEDDED_FAILURE                                        0x0000000000000002
#define CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU                          0x0000000000000004
#define CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY                       0x0000000000000008
#define CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES            0x0000000000000010
#define CLUSRES_STATUS_NETWORK_FAILURE                                         0x0000000000000020
#define CLUSRES_STATUS_UNMONITORED                                             0x0000000000000040
#define CLUSRES_STATUS_OS_HEARTBEAT                                            0x0000000000000080
#define CLUSRES_STATUS_APPLICATION_READY                                       0x0000000000000100
#define CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER                            0x0000000000000200

HRESENUMEX
WINAPI
ClusterResourceOpenEnumEx(
    _In_ HCLUSTER hCluster,
    _In_reads_bytes_opt_(cbProperties)LPCWSTR lpszProperties,
    _In_ DWORD cbProperties,
    _In_reads_bytes_opt_(cbRoProperties) LPCWSTR lpszRoProperties,
    _In_ DWORD cbRoProperties,
    _In_ DWORD dwFlags
    );

typedef HRESENUMEX
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX)(
    _In_ HCLUSTER hCluster,
    _In_reads_bytes_opt_(cbProperties)LPCWSTR lpszProperties,
    _In_ DWORD cbProperties,
    _In_reads_bytes_opt_(cbRoProperties) LPCWSTR lpszRoProperties,
    _In_ DWORD cbRoProperties,
    _In_ DWORD dwFlags
    );

DWORD
WINAPI
ClusterResourceGetEnumCountEx(
    _In_ HRESENUMEX hResourceEnumEx
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX)(
    _In_ HRESENUMEX hResourceEnumEx
    );

DWORD
WINAPI
ClusterResourceEnumEx(
    _In_ HRESENUMEX hResourceEnumEx,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_RESOURCE_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX)(
    _In_ HRESENUMEX hResourceEnumEx,
    _In_ DWORD dwIndex,
    _Inout_ PCLUSTER_RESOURCE_ENUM_ITEM pItem,
    _Inout_ LPDWORD cbItem
    );

DWORD
WINAPI
ClusterResourceCloseEnumEx(
    _In_ HRESENUMEX hResourceEnumEx
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX)(
    _In_ HRESENUMEX hResourceEnumEx
    );

#define CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS 0x00000001
#define CLUSAPI_GROUP_ONLINE_SYNCHRONOUS            0x00000002
#define CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE     0x00000004
#define CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE   0x00000008

DWORD WINAPI OnlineClusterGroupEx(
    _In_       HGROUP hGroup,
    _In_opt_   HNODE  hDestinationNode,
    _In_       DWORD  dwOnlineFlags,
    _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
    _In_       DWORD  cbInBufferSize
    );


#define CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS 0x00000001

DWORD WINAPI OfflineClusterGroupEx(
    _In_       HGROUP hGroup,
    _In_       DWORD  dwOfflineFlags,
    _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
    _In_       DWORD  cbInBufferSize
    );


#define CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS          0x00000001
#define CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE  0x00000002
#define CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM            0x00000004
#define CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE              0x00000008

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
OnlineClusterGroupEx2(
    _In_       HGROUP hGroup,
    _In_opt_   HNODE  hDestinationNode,
    _In_       DWORD  dwOnlineFlags,
    _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
    _In_       DWORD  cbInBufferSize,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
OfflineClusterGroupEx2(
    __in       HGROUP hGroup,
    __in       DWORD  dwOfflineFlags,
    __in_opt   PBYTE  lpInBuffer,
    __in       DWORD  cbInBufferSize,
    _In_opt_   LPCWSTR lpszReason
    );

#endif


#define CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE            0x00000020


DWORD WINAPI OnlineClusterResourceEx(
  _In_       HRESOURCE hResource,
  _In_       DWORD     dwOnlineFlags,
  _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
  _In_       DWORD     cbInBufferSize
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
OnlineClusterResourceEx2(
    _In_ HRESOURCE hResource,
    _In_ DWORD dwOnlineFlags,
    _In_reads_bytes_opt_(cbInBufferSize) PBYTE lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

#define CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS             0x00000001
#define CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION             0x00000002
#define CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE     0x00000004

// these are passed through the RHS offline resource call for V2 and later resources
#define CLUSAPI_RESOURCE_OFFLINE_REASON_NONE            0x00000000
#define CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN         0x00000001
#define CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING          0x00000002
#define CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED  0x00000004
#define CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED   0x00000008
#define CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED 0x00000010
#define CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED       0x00000020
#define CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN   0x00000040

DWORD WINAPI OfflineClusterResourceEx(
  _In_       HRESOURCE hResource,
  _In_       DWORD     dwOfflineFlags,
  _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
  _In_       DWORD     cbInBufferSize
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
OfflineClusterResourceEx2(
    _In_       HRESOURCE hResource,
    _In_       DWORD     dwOfflineFlags,
    _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
    _In_       DWORD     cbInBufferSize,
    _In_opt_   LPCWSTR   lpszReason
    );

#endif

#define CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS           0x00000001
#define CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR   0x00000002
#define CLUSAPI_GROUP_MOVE_QUEUE_ENABLED                    0x00000004
#define CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START              0x00000008
#define CLUSAPI_GROUP_MOVE_FAILBACK                         0x00000010
#define CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE             0x00000020


#define CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV     0x0000000000000001
#define CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS           (CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV)

DWORD
WINAPI
MoveClusterGroupEx(
  _In_       HGROUP hGroup,
  _In_opt_   HNODE  hDestinationNode,
  _In_       DWORD  dwMoveFlags,
  _In_reads_bytes_opt_(cbInBufferSize) PBYTE  lpInBuffer,
  _In_       DWORD  cbInBufferSize
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
MoveClusterGroupEx2(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hDestinationNode,
    _In_ DWORD dwMoveFlags,
    _In_reads_bytes_opt_(cbInBufferSize) PBYTE lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _In_opt_ LPCWSTR lpszReason
    );

#endif


DWORD WINAPI CancelClusterGroupOperation(
  _In_       HGROUP hGroup,
  _In_       DWORD  dwCancelFlags_RESERVED
    );

DWORD WINAPI RestartClusterResource(
    _In_ HRESOURCE hResource,
    _In_ DWORD dwFlags
    );

typedef DWORD
(WINAPI * PCLUSAPI_RESTART_CLUSTER_RESOURCE)(
    HRESOURCE hResource,
    DWORD dwFlags
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
RestartClusterResourceEx(
    _In_ HRESOURCE hResource,
    _In_ DWORD dwFlags,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_RESTART_CLUSTER_RESOURCE_EX)(
    HRESOURCE hResource,
    DWORD dwFlags
    );

#endif

#endif // (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)

BOOL
WINAPI
CloseClusterGroup(
    _In_ HGROUP hGroup
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_GROUP)(
    _In_ HGROUP hGroup
    );

HCLUSTER
WINAPI
GetClusterFromGroup(
    _In_ HGROUP hGroup
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_GET_CLUSTER_FROM_GROUP)(
    _In_ HGROUP hGroup
    );

_Success_(return >= 0) //!= ClusterGroupStateUnknown
CLUSTER_GROUP_STATE
WINAPI
GetClusterGroupState(
    _In_ HGROUP hGroup,
    _Out_writes_to_opt_(*lpcchNodeName, *lpcchNodeName + 1) LPWSTR lpszNodeName,
    _Inout_opt_ LPDWORD lpcchNodeName
    );

typedef CLUSTER_GROUP_STATE
(WINAPI * PCLUSAPI_GET_CLUSTER_GROUP_STATE)(
    _In_ HGROUP hGroup,
    _Out_writes_to_opt_(*lpcchNodeName, *lpcchNodeName + 1) LPWSTR lpszNodeName,
    _Inout_opt_ LPDWORD lpcchNodeName
    );

DWORD
WINAPI
SetClusterGroupName(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszGroupName
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_GROUP_NAME)(
    HGROUP hGroup,
    LPCWSTR lpszGroupName
    );

DWORD
WINAPI
SetClusterGroupNodeList(
    _In_ HGROUP hGroup,
    _In_ DWORD NodeCount,
    _In_reads_opt_( NodeCount ) HNODE NodeList[]
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST)(
    _In_ HGROUP hGroup,
    _In_ DWORD NodeCount,
    _In_reads_opt_( NodeCount ) HNODE NodeList[]
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
SetClusterGroupNameEx(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszGroupName,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_GROUP_NAME_EX)(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszGroupName,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
SetClusterGroupNodeListEx(
    __in HGROUP hGroup,
    __in DWORD NodeCount,
    __in_ecount( NodeCount ) HNODE NodeList[],
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST_EX)(
    _In_ HGROUP hGroup,
    _In_ DWORD NodeCount,
    _In_reads_opt_( NodeCount ) HNODE NodeList[],
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
OnlineClusterGroup(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hDestinationNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_ONLINE_CLUSTER_GROUP)(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hDestinationNode
    );

DWORD
WINAPI
MoveClusterGroup(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hDestinationNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_MOVE_CLUSTER_GROUP)(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hDestinationNode
    );

DWORD
WINAPI
OfflineClusterGroup(
    _In_ HGROUP hGroup
    );

typedef DWORD
(WINAPI * PCLUSAPI_OFFLINE_CLUSTER_GROUP)(
    HGROUP hGroup
    );

DWORD
WINAPI
DeleteClusterGroup(
    _In_ HGROUP hGroup
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_GROUP)(
    HGROUP hGroup
    );

DWORD
WINAPI
DestroyClusterGroup(
    _In_ HGROUP hGroup
    );

typedef DWORD
(WINAPI * PCLUSAPI_DESTROY_CLUSTER_GROUP)(
    HGROUP hGroup
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
DeleteClusterGroupEx(
    _In_ HGROUP hGroup,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_GROUP_EX)(
    _In_ HGROUP hGroup,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
DestroyClusterGroupEx(
    _In_ HGROUP hGroup,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_DESTROY_CLUSTER_GROUP_EX)(
    _In_ HGROUP hGroup,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

HGROUPENUM
WINAPI
ClusterGroupOpenEnum(
    _In_ HGROUP hGroup,
    _In_ DWORD dwType
    );

typedef HGROUPENUM
(WINAPI * PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM)(
    HGROUP hGroup,
    DWORD dwType
    );

DWORD
WINAPI
ClusterGroupGetEnumCount(
    _In_ HGROUPENUM hGroupEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT)(
    _In_ HGROUPENUM hGroupEnum
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterGroupEnum(
    _In_ HGROUPENUM hGroupEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszResourceName,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_ENUM)(
    _In_ HGROUPENUM hGroupEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszResourceName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterGroupCloseEnum(
    _In_ HGROUPENUM hGroupEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM)(
    HGROUPENUM hGroupEnum
    );

#endif // MIDL_PASS


//
// Definitions used in resource management routines.
//

#ifndef _CLUSTER_API_TYPES_
//
// Resource-related structures and types
//
typedef enum CLUSTER_RESOURCE_STATE {
    ClusterResourceStateUnknown = -1,
    ClusterResourceInherited,
    ClusterResourceInitializing,
    ClusterResourceOnline,
    ClusterResourceOffline,
    ClusterResourceFailed,
    ClusterResourcePending = 128,
    ClusterResourceOnlinePending,
    ClusterResourceOfflinePending
} CLUSTER_RESOURCE_STATE;

typedef enum CLUSTER_RESOURCE_RESTART_ACTION {
    ClusterResourceDontRestart = 0,
    ClusterResourceRestartNoNotify,
    ClusterResourceRestartNotify,
    ClusterResourceRestartActionCount
} CLUSTER_RESOURCE_RESTART_ACTION, CRRA;

typedef enum CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION {
    ClusterResourceEmbeddedFailureActionNone = 0,
    ClusterResourceEmbeddedFailureActionLogOnly = 1,
    ClusterResourceEmbeddedFailureActionRecover
} CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION;

//
// Flags for resource creation
//
typedef enum CLUSTER_RESOURCE_CREATE_FLAGS {
    CLUSTER_RESOURCE_DEFAULT_MONITOR   = 0,
    CLUSTER_RESOURCE_SEPARATE_MONITOR  = 1,
    CLUSTER_RESOURCE_VALID_FLAGS       = CLUSTER_RESOURCE_SEPARATE_MONITOR
} CLUSTER_RESOURCE_CREATE_FLAGS;

typedef enum CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE {
    ClusterSharedVolumeSnapshotStateUnknown,
    ClusterSharedVolumePrepareForHWSnapshot,
    ClusterSharedVolumeHWSnapshotCompleted,
    ClusterSharedVolumePrepareForFreeze
} CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE;

#endif // _CLUSTER_API_TYPES_

//
// Interfaces for managing the resources in a cluster
//

#if ( !MIDL_PASS && !__midl )
HRESOURCE
WINAPI
CreateClusterResource(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszResourceName,
    _In_ LPCWSTR lpszResourceType,
    _In_ DWORD dwFlags
    );

typedef HRESOURCE
(WINAPI * PCLUSAPI_CREATE_CLUSTER_RESOURCE)(
    HGROUP hGroup,
    LPCWSTR lpszResourceName,
    LPCWSTR lpszResourceType,
    DWORD dwFlags
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

HRESOURCE
WINAPI
CreateClusterResourceEx(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszResourceName,
    _In_ LPCWSTR lpszResourceType,
    _In_ DWORD dwFlags,
    _In_opt_ LPCWSTR lpszReason
    );

typedef HRESOURCE
(WINAPI * PCLUSAPI_CREATE_CLUSTER_RESOURCE_EX)(
    _In_ HGROUP hGroup,
    _In_ LPCWSTR lpszResourceName,
    _In_ LPCWSTR lpszResourceType,
    _In_ DWORD dwFlags,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

HRESOURCE
WINAPI
OpenClusterResource(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceName
    );

typedef HRESOURCE
(WINAPI * PCLUSAPI_OPEN_CLUSTER_RESOURCE)(
    HCLUSTER hCluster,
    LPCWSTR lpszResourceName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
HRESOURCE
WINAPI
OpenClusterResourceEx(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszResourceName,
    _In_      DWORD dwDesiredAccess,
    _Out_opt_ DWORD* lpdwGrantedAccess
    );

typedef HRESOURCE
(WINAPI * PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX)(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszResourceName,
    _In_      DWORD   dwDesiredAccess,
    _Out_opt_ LPDWORD lpdwGrantedAccess
    );
#endif

BOOL
WINAPI
CloseClusterResource(
    _In_ HRESOURCE hResource
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_RESOURCE)(
    HRESOURCE hResource
    );

HCLUSTER
WINAPI
GetClusterFromResource(
    _In_ HRESOURCE hResource
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_GET_CLUSTER_FROM_RESOURCE)(
    _In_ HRESOURCE hResource
    );

DWORD
WINAPI
DeleteClusterResource(
    _In_ HRESOURCE hResource
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_RESOURCE)(
    HRESOURCE hResource
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
DeleteClusterResourceEx(
    _In_ HRESOURCE hResource,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_RESOURCE_EX)(
    _In_ HRESOURCE hResource,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

_Success_(return >= 0) // != ClusterResourceStateUnknown
CLUSTER_RESOURCE_STATE
WINAPI
GetClusterResourceState(
    _In_ HRESOURCE hResource,
    _Out_writes_to_opt_(*lpcchNodeName, *lpcchNodeName) LPWSTR lpszNodeName,
    _Inout_opt_ LPDWORD lpcchNodeName,
    _Out_writes_to_opt_(*lpcchGroupName, *lpcchGroupName) LPWSTR lpszGroupName,
    _Inout_opt_ LPDWORD lpcchGroupName
    );

typedef CLUSTER_RESOURCE_STATE
(WINAPI * PCLUSAPI_GET_CLUSTER_RESOURCE_STATE)(
    _In_ HRESOURCE hResource,
    _Out_writes_to_opt_(*lpcchNodeName, *lpcchNodeName) LPWSTR lpszNodeName,
    _Inout_opt_ LPDWORD lpcchNodeName,
    _Out_writes_to_opt_(*lpcchGroupName, *lpcchGroupName) LPWSTR lpszGroupName,
    _Inout_opt_ LPDWORD lpcchGroupName
    );

DWORD
WINAPI
SetClusterResourceName(
    _In_ HRESOURCE hResource,
    _In_ LPCWSTR lpszResourceName
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_RESOURCE_NAME)(
    HRESOURCE hResource,
    LPCWSTR lpszResourceName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
SetClusterResourceNameEx(
    _In_ HRESOURCE hResource,
    _In_ LPCWSTR lpszResourceName,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_RESOURCE_NAME_EX)(
    _In_ HRESOURCE hResource,
    _In_ LPCWSTR lpszResourceName,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
FailClusterResource(
    _In_ HRESOURCE hResource
    );

typedef DWORD
(WINAPI * PCLUSAPI_FAIL_CLUSTER_RESOURCE)(
    HRESOURCE hResource
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
FailClusterResourceEx(
    _In_ HRESOURCE hResource,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_FAIL_CLUSTER_RESOURCE_EX)(
    _In_ HRESOURCE hResource,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
OnlineClusterResource(
    _In_ HRESOURCE hResource
    );

typedef DWORD
(WINAPI * PCLUSAPI_ONLINE_CLUSTER_RESOURCE)(
    HRESOURCE hResource
    );

DWORD
WINAPI
OfflineClusterResource(
    _In_ HRESOURCE hResource
    );

typedef DWORD
(WINAPI * PCLUSAPI_OFFLINE_CLUSTER_RESOURCE)(
    HRESOURCE hResource
    );

DWORD
WINAPI
ChangeClusterResourceGroup(
    _In_ HRESOURCE hResource,
    _In_ HGROUP hGroup
    );

typedef DWORD
(WINAPI * PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP)(
    HRESOURCE hResource,
    HGROUP hGroup
    );

DWORD
WINAPI
ChangeClusterResourceGroupEx(
    _In_ HRESOURCE hResource,
    _In_ HGROUP hGroup,
    _In_ ULONGLONG Flags
    );

typedef DWORD
(WINAPI * PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX)(
    HRESOURCE hResource,
    HGROUP hGroup,
    _In_ ULONGLONG Flags
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ChangeClusterResourceGroupEx2(
    _In_ HRESOURCE hResource,
    _In_ HGROUP hGroup,
    _In_ ULONGLONG Flags,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX2)(
    _In_ HRESOURCE hResource,
    _In_ HGROUP hGroup,
    _In_ ULONGLONG Flags,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
AddClusterResourceNode(
    _In_ HRESOURCE hResource,
    _In_ HNODE hNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE)(
    HRESOURCE hResource,
    HNODE hNode
    );

DWORD
WINAPI
RemoveClusterResourceNode(
    _In_ HRESOURCE hResource,
    _In_ HNODE hNode
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE)(
    HRESOURCE hResource,
    HNODE hNode
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
AddClusterResourceNodeEx(
    _In_     HRESOURCE hResource,
    _In_     HNODE hNode,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE_EX)(
    _In_     HRESOURCE hResource,
    _In_     HNODE hNode,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
RemoveClusterResourceNodeEx(
    _In_ HRESOURCE hResource,
    _In_ HNODE hNode,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE_EX)(
    _In_ HRESOURCE hResource,
    _In_ HNODE hNode,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
AddClusterResourceDependency(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hDependsOn
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY)(
    HRESOURCE hResource,
    HRESOURCE hDependsOn
    );

DWORD
WINAPI
RemoveClusterResourceDependency(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hDependsOn
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY)(
    HRESOURCE hResource,
    HRESOURCE hDependsOn
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
AddClusterResourceDependencyEx(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY_EX)(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
RemoveClusterResourceDependencyEx(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY_EX)(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hDependsOn,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

DWORD
WINAPI
SetClusterResourceDependencyExpression(
    _In_ HRESOURCE hResource,
    _In_ LPCWSTR lpszDependencyExpression
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION)(
    _In_ HRESOURCE hResource,
    _In_ LPCWSTR lpszDependencyExpression
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterResourceDependencyExpression(
    _In_ HRESOURCE hResource,
    _Out_writes_to_opt_(*lpcchDependencyExpression, *lpcchDependencyExpression + 1)
        LPWSTR lpszDependencyExpression,
    _Inout_ LPDWORD lpcchDependencyExpression
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION)(
    _In_ HRESOURCE hResource,
    _Out_writes_to_opt_(*lpcchDependencyExpression, *lpcchDependencyExpression + 1)
        LPWSTR lpszDependencyExpression,
    _Inout_ LPDWORD lpcchDependencyExpression
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
DWORD
WINAPI
AddResourceToClusterSharedVolumes(
    _In_ HRESOURCE hResource
    );
#endif

typedef DWORD
(WINAPI * PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES)(
    _In_ HRESOURCE hResource
    );


#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
DWORD
WINAPI
RemoveResourceFromClusterSharedVolumes(
    _In_ HRESOURCE hResource
    );
#endif

typedef DWORD
(WINAPI * PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES)(
    _In_ HRESOURCE hResource
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
DWORD
WINAPI
IsFileOnClusterSharedVolume(
    _In_ LPCWSTR lpszPathName,
    _Out_ PBOOL pbFileIsOnSharedVolume
    );
#endif

typedef DWORD
(WINAPI *PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME)(
    _In_ LPCWSTR lpszPathName,
    _Out_ PBOOL pbFileIsOnSharedVolume
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
DWORD
WINAPI
ClusterSharedVolumeSetSnapshotState(
    _In_ GUID    guidSnapshotSet,
    _In_ LPCWSTR lpszVolumeName,
    _In_ CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE state
    );
#endif

typedef DWORD
(WINAPI *PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE)(
    _In_ GUID    guidSnapshotSet,
    _In_ LPCWSTR lpszVolumeName,
    _In_ CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE state
    );

BOOL
WINAPI
CanResourceBeDependent(
    _In_ HRESOURCE hResource,
    _In_ HRESOURCE hResourceDependent
    );

typedef BOOL
(WINAPI * PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT)(
    HRESOURCE hResource,
    HRESOURCE hResourceDependent
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceControl(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_CONTROL)(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceControlAsUser(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceTypeControl(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceTypeControlAsUser(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterGroupControl(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_CONTROL)(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceControlEx(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_CONTROL_EX)(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceControlAsUserEx(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_CONTROL_AS_USER_EX)(
    _In_ HRESOURCE hResource,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(cbInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD cbInBufferSize,
    _Out_writes_bytes_to_opt_(cbOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD cbOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceTypeControlEx(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_EX)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceTypeControlAsUserEx(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_AS_USER_EX)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterGroupControlEx(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_GROUP_CONTROL_EX)(
    _In_ HGROUP hGroup,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterNodeControl(
    _In_ HNODE hNode,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_CONTROL)(
    _In_ HNODE hNode,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

__success(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterNodeControlEx(
    _In_ HNODE hNode,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NODE_CONTROL_EX)(
    _In_ HNODE hNode,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

_Success_(return != FALSE)
BOOL
WINAPI
GetClusterResourceNetworkName(
    _In_ HRESOURCE hResource,
    _Out_writes_to_(*nSize, *nSize + 1) LPWSTR lpBuffer,
    _Inout_ LPDWORD nSize
    );

typedef BOOL
(WINAPI * PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME)(
    _In_ HRESOURCE hResource,
    _Out_writes_to_(*nSize, *nSize + 1) LPWSTR lpBuffer,
    _Inout_ LPDWORD nSize
    );


#endif // MIDL_PASS


//
// Cluster control properties
//

#ifndef _CLUSTER_API_TYPES_
//
// Cluster Control Property Data - Types (a WORD)
//
typedef enum CLUSTER_PROPERTY_TYPE {
    CLUSPROP_TYPE_UNKNOWN = -1,
    CLUSPROP_TYPE_ENDMARK = 0,
    CLUSPROP_TYPE_LIST_VALUE,
    CLUSPROP_TYPE_RESCLASS,
    CLUSPROP_TYPE_RESERVED1,
    CLUSPROP_TYPE_NAME,
    CLUSPROP_TYPE_SIGNATURE,
    CLUSPROP_TYPE_SCSI_ADDRESS,
    CLUSPROP_TYPE_DISK_NUMBER,
    CLUSPROP_TYPE_PARTITION_INFO,
    CLUSPROP_TYPE_FTSET_INFO,
    CLUSPROP_TYPE_DISK_SERIALNUMBER,
    CLUSPROP_TYPE_DISK_GUID,
    CLUSPROP_TYPE_DISK_SIZE,
    CLUSPROP_TYPE_PARTITION_INFO_EX,
    CLUSPROP_TYPE_PARTITION_INFO_EX2,
    CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR,


    CLUSPROP_TYPE_USER=32768
} CLUSTER_PROPERTY_TYPE;

//
// Cluster Control Property Data - Formats (a WORD)
//
typedef enum CLUSTER_PROPERTY_FORMAT {
    CLUSPROP_FORMAT_UNKNOWN = 0,
    CLUSPROP_FORMAT_BINARY,
    CLUSPROP_FORMAT_DWORD,
    CLUSPROP_FORMAT_SZ,
    CLUSPROP_FORMAT_EXPAND_SZ,
    CLUSPROP_FORMAT_MULTI_SZ,
    CLUSPROP_FORMAT_ULARGE_INTEGER,
    CLUSPROP_FORMAT_LONG,
    CLUSPROP_FORMAT_EXPANDED_SZ,
    CLUSPROP_FORMAT_SECURITY_DESCRIPTOR,
    CLUSPROP_FORMAT_LARGE_INTEGER,
    CLUSPROP_FORMAT_WORD,
    CLUSPROP_FORMAT_FILETIME,
    CLUSPROP_FORMAT_VALUE_LIST,
    CLUSPROP_FORMAT_PROPERTY_LIST,

    CLUSPROP_FORMAT_USER=32768
} CLUSTER_PROPERTY_FORMAT;

#endif // _CLUSTER_API_TYPES_

//
// Cluster Control Property Data - Syntax
//
#define CLUSPROP_SYNTAX_VALUE( type, format ) ((DWORD) ((type << 16) | format))

#ifndef _CLUSTER_API_TYPES_

typedef enum CLUSTER_PROPERTY_SYNTAX {

    CLUSPROP_SYNTAX_ENDMARK         = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_ENDMARK, CLUSPROP_FORMAT_UNKNOWN ),
    CLUSPROP_SYNTAX_NAME            = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_NAME, CLUSPROP_FORMAT_SZ ),
    CLUSPROP_SYNTAX_RESCLASS        = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_RESCLASS, CLUSPROP_FORMAT_DWORD ),

    CLUSPROP_SYNTAX_LIST_VALUE_SZ                   = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_SZ ),
    CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ            = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_EXPAND_SZ ),
    CLUSPROP_SYNTAX_LIST_VALUE_DWORD                = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_DWORD ),
    CLUSPROP_SYNTAX_LIST_VALUE_BINARY               = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_BINARY ),
    CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ             = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_MULTI_SZ ),
    CLUSPROP_SYNTAX_LIST_VALUE_LONG                 = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_LONG ),
    CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ          = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_EXPANDED_SZ ),
    CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR  = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_SECURITY_DESCRIPTOR ),
    CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER        = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_LARGE_INTEGER ),
    CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER       = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_ULARGE_INTEGER ),
    CLUSPROP_SYNTAX_LIST_VALUE_WORD                 = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_WORD ),
    CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST        = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_PROPERTY_LIST ),
    CLUSPROP_SYNTAX_LIST_VALUE_FILETIME             = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_LIST_VALUE, CLUSPROP_FORMAT_FILETIME ),

    // Storage syntax values

    CLUSPROP_SYNTAX_DISK_SIGNATURE       = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_SIGNATURE, CLUSPROP_FORMAT_DWORD ),
    CLUSPROP_SYNTAX_SCSI_ADDRESS         = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_SCSI_ADDRESS, CLUSPROP_FORMAT_DWORD ),
    CLUSPROP_SYNTAX_DISK_NUMBER          = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_DISK_NUMBER, CLUSPROP_FORMAT_DWORD ),
    CLUSPROP_SYNTAX_PARTITION_INFO       = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_PARTITION_INFO, CLUSPROP_FORMAT_BINARY ),
    CLUSPROP_SYNTAX_FTSET_INFO           = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_FTSET_INFO, CLUSPROP_FORMAT_BINARY ),
    CLUSPROP_SYNTAX_DISK_SERIALNUMBER    = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_DISK_SERIALNUMBER, CLUSPROP_FORMAT_SZ ),
    CLUSPROP_SYNTAX_DISK_GUID            = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_DISK_GUID, CLUSPROP_FORMAT_SZ ),
    CLUSPROP_SYNTAX_DISK_SIZE            = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_DISK_SIZE, CLUSPROP_FORMAT_ULARGE_INTEGER ),
    CLUSPROP_SYNTAX_PARTITION_INFO_EX    = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_PARTITION_INFO_EX, CLUSPROP_FORMAT_BINARY ),
    CLUSPROP_SYNTAX_PARTITION_INFO_EX2   = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_PARTITION_INFO_EX2, CLUSPROP_FORMAT_BINARY ),
    CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR = CLUSPROP_SYNTAX_VALUE( CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR, CLUSPROP_FORMAT_BINARY ),


} CLUSTER_PROPERTY_SYNTAX;

#endif // _CLUSTER_API_TYPES_


#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)

#define GROUP_FAILURE_INFO_VERSION_1        0x1

typedef struct GROUP_FAILURE_INFO {
    DWORD dwFailoverAttemptsRemaining;
    DWORD dwFailoverPeriodRemaining; // in sec
} GROUP_FAILURE_INFO, *PGROUP_FAILURE_INFO;

typedef struct GROUP_FAILURE_INFO_BUFFER {
    DWORD dwVersion;
    GROUP_FAILURE_INFO Info;
} GROUP_FAILURE_INFO_BUFFER, *PGROUP_FAILURE_INFO_BUFFER;


#define RESOURCE_FAILURE_INFO_VERSION_1     0x1

typedef struct RESOURCE_FAILURE_INFO {
    DWORD dwRestartAttemptsRemaining;
    DWORD dwRestartPeriodRemaining; // in sec
} RESOURCE_FAILURE_INFO, *PRESOURCE_FAILURE_INFO;


typedef struct RESOURCE_FAILURE_INFO_BUFFER {
    DWORD dwVersion;
    RESOURCE_FAILURE_INFO Info;
} RESOURCE_FAILURE_INFO_BUFFER, *PRESOURCE_FAILURE_INFO_BUFFER;

typedef struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    BOOL isTerminalFailure;
    DWORD restartPeriodRemaining;
} RESOURCE_TERMINAL_FAILURE_INFO_BUFFER, *PRESOURCE_TERMINAL_FAILURE_INFO_BUFFER;


#endif //(CLUSAPI_VERSION >= CLUSAPI_VERSION_WINDOWS8)


//
// Define Cluster Control Code access methods
//
#define CLUS_ACCESS_ANY        0
#define CLUS_ACCESS_READ    0x01
#define CLUS_ACCESS_WRITE   0x02

//
// Define Cluster Control Code modification actions
//
#define CLUS_NO_MODIFY      0
#define CLUS_MODIFY         0x01

//
// Define Cluster Control Code Global actions
//
#define CLUS_NOT_GLOBAL     0
#define CLUS_GLOBAL         0x01

#ifndef _CLUSTER_API_TYPES_
//
// Define Cluster Control Code target objects
//
typedef enum CLUSTER_CONTROL_OBJECT {
    CLUS_OBJECT_INVALID=0,
    CLUS_OBJECT_RESOURCE,
    CLUS_OBJECT_RESOURCE_TYPE,
    CLUS_OBJECT_GROUP,
    CLUS_OBJECT_NODE,
    CLUS_OBJECT_NETWORK,
    CLUS_OBJECT_NETINTERFACE,
    CLUS_OBJECT_CLUSTER,
    CLUS_OBJECT_GROUPSET,
    CLUS_OBJECT_AFFINITYRULE,
    CLUS_OBJECT_USER=128
} CLUSTER_CONTROL_OBJECT;

#endif // _CLUSTER_API_TYPES_


//
// Macro to generate full cluster control codes
//
//  31      24 23 22 21 20 19       16 15                    2 1    0
// +----------+--+--+--+--+-----------+-----------------------+------+
// |  OBJECT  |G |M |U |I       CLUSTER CONTROL CODES         |ACCESS|
// +----------+--+--+--+--+-----------+-----------------------+------+
//
// OBJECT - Object identifier (8 bits)
// G - Global bit (operation must be performed on all nodes of cluster)
// M - Modify bit (code causes a modification, may cause event notification)
// U - User code bit (splits the control codes into 2 spaces each 2^^19 in size)
// I - Internal code bit (only for non-user control codes)
// CLUSTER CONTROL CODES - 2^^18 (256 thousand possible control codes)
// ACCESS - Access mode (2 bits)
//

//
// Define control code shifts
//
#define CLUSCTL_ACCESS_SHIFT         0
#define CLUSCTL_FUNCTION_SHIFT       2
#define CLCTL_INTERNAL_SHIFT        20
#define CLCTL_USER_SHIFT            21
#define CLCTL_MODIFY_SHIFT          22
#define CLCTL_GLOBAL_SHIFT          23
#define CLUSCTL_OBJECT_SHIFT        24

//
// Define control code masks
//
#define CLCTL_INTERNAL_MASK             (1<<CLCTL_INTERNAL_SHIFT)
#define CLCTL_USER_MASK                 (1<<CLCTL_USER_SHIFT)
#define CLCTL_MODIFY_MASK               (1<<CLCTL_MODIFY_SHIFT)
#define CLCTL_GLOBAL_MASK               (1<<CLCTL_GLOBAL_SHIFT)
#define CLUSCTL_CONTROL_CODE_MASK       0x3FFFFF // Includes access mask
#define CLUSCTL_OBJECT_MASK             0xFF
#define CLUSCTL_ACCESS_MODE_MASK        0x03

//
// Cluster Control function codes (a DWORD)
//
#define CLCTL_CLUSTER_BASE  0           // Start of cluster defined functions
#define CLCTL_USER_BASE     (1<<CLCTL_USER_SHIFT) // Start of user functions

#define CLCTL_EXTERNAL_CODE( Function, Access, Modify ) ( \
    ((Access) << CLUSCTL_ACCESS_SHIFT) | \
    ((CLCTL_CLUSTER_BASE + Function) << CLUSCTL_FUNCTION_SHIFT) | \
    ((Modify) << CLCTL_MODIFY_SHIFT) )

#define CLCTL_INTERNAL_CODE( Function, Access, Modify ) ( \
    ((Access) << CLUSCTL_ACCESS_SHIFT) | \
    CLCTL_INTERNAL_MASK | \
    ((CLCTL_CLUSTER_BASE + Function) << CLUSCTL_FUNCTION_SHIFT) | \
    ((Modify) << CLCTL_MODIFY_SHIFT) )

//External codes:
//A control code that applications can use to initiate an operation on a cluster object. External control codes are passed as the dwControlCode parameter to control code functions.

#ifndef _CLUSTER_API_TYPES_
typedef enum CLCTL_CODES {
    //
    // External control codes
    //
    CLCTL_UNKNOWN                           = CLCTL_EXTERNAL_CODE( 0, CLUS_ACCESS_ANY, CLUS_NO_MODIFY ),
    CLCTL_GET_CHARACTERISTICS               = CLCTL_EXTERNAL_CODE( 1, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_FLAGS                         = CLCTL_EXTERNAL_CODE( 2, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_CLASS_INFO                    = CLCTL_EXTERNAL_CODE( 3, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_REQUIRED_DEPENDENCIES         = CLCTL_EXTERNAL_CODE( 4, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_ARB_TIMEOUT                   = CLCTL_EXTERNAL_CODE( 5, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_FAILURE_INFO                  = CLCTL_EXTERNAL_CODE( 6, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_GET_NAME                          = CLCTL_EXTERNAL_CODE( 10, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_RESOURCE_TYPE                 = CLCTL_EXTERNAL_CODE( 11, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_NODE                          = CLCTL_EXTERNAL_CODE( 12, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_NETWORK                       = CLCTL_EXTERNAL_CODE( 13, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_ID                            = CLCTL_EXTERNAL_CODE( 14, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_FQDN                          = CLCTL_EXTERNAL_CODE( 15, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME  = CLCTL_EXTERNAL_CODE( 16, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_CHECK_VOTER_EVICT                 = CLCTL_EXTERNAL_CODE( 17, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_CHECK_VOTER_DOWN                  = CLCTL_EXTERNAL_CODE( 18, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_SHUTDOWN                          = CLCTL_EXTERNAL_CODE( 19, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_ENUM_COMMON_PROPERTIES            = CLCTL_EXTERNAL_CODE( 20, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_RO_COMMON_PROPERTIES          = CLCTL_EXTERNAL_CODE( 21, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_COMMON_PROPERTIES             = CLCTL_EXTERNAL_CODE( 22, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_SET_COMMON_PROPERTIES             = CLCTL_EXTERNAL_CODE( 23, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_VALIDATE_COMMON_PROPERTIES        = CLCTL_EXTERNAL_CODE( 24, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_COMMON_PROPERTY_FMTS          = CLCTL_EXTERNAL_CODE( 25, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS = CLCTL_EXTERNAL_CODE( 26, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_CHECK_VOTER_EVICT_WITNESS         = CLCTL_EXTERNAL_CODE( 27, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_CHECK_VOTER_DOWN_WITNESS          = CLCTL_EXTERNAL_CODE( 28, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_ENUM_PRIVATE_PROPERTIES           = CLCTL_EXTERNAL_CODE( 30, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_RO_PRIVATE_PROPERTIES         = CLCTL_EXTERNAL_CODE( 31, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_PRIVATE_PROPERTIES            = CLCTL_EXTERNAL_CODE( 32, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_SET_PRIVATE_PROPERTIES            = CLCTL_EXTERNAL_CODE( 33, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_VALIDATE_PRIVATE_PROPERTIES       = CLCTL_EXTERNAL_CODE( 34, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_PRIVATE_PROPERTY_FMTS         = CLCTL_EXTERNAL_CODE( 35, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS= CLCTL_EXTERNAL_CODE( 36, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_ADD_REGISTRY_CHECKPOINT           = CLCTL_EXTERNAL_CODE( 40, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_DELETE_REGISTRY_CHECKPOINT        = CLCTL_EXTERNAL_CODE( 41, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_GET_REGISTRY_CHECKPOINTS          = CLCTL_EXTERNAL_CODE( 42, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_ADD_CRYPTO_CHECKPOINT             = CLCTL_EXTERNAL_CODE( 43, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_DELETE_CRYPTO_CHECKPOINT          = CLCTL_EXTERNAL_CODE( 44, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_GET_CRYPTO_CHECKPOINTS            = CLCTL_EXTERNAL_CODE( 45, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_RESOURCE_UPGRADE_DLL              = CLCTL_EXTERNAL_CODE( 46, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT     = CLCTL_EXTERNAL_CODE( 47, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT     = CLCTL_EXTERNAL_CODE( 48, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_GET_LOADBAL_PROCESS_LIST          = CLCTL_EXTERNAL_CODE( 50, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_SET_ACCOUNT_ACCESS                = CLCTL_EXTERNAL_CODE( 60, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_GET_NETWORK_NAME                  = CLCTL_EXTERNAL_CODE( 90, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),
    CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN  = CLCTL_EXTERNAL_CODE( 91, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),
    CLCTL_NETNAME_REGISTER_DNS_RECORDS      = CLCTL_EXTERNAL_CODE( 92, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_GET_DNS_NAME                      = CLCTL_EXTERNAL_CODE( 93, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),
    CLCTL_NETNAME_SET_PWD_INFO              = CLCTL_EXTERNAL_CODE( 94, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_NETNAME_DELETE_CO                 = CLCTL_EXTERNAL_CODE( 95, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_NETNAME_VALIDATE_VCO              = CLCTL_EXTERNAL_CODE( 96, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),
    CLCTL_NETNAME_RESET_VCO                 = CLCTL_EXTERNAL_CODE( 97, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),
    CLCTL_NETNAME_REPAIR_VCO                = CLCTL_EXTERNAL_CODE( 99, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),

    CLCTL_STORAGE_GET_DISK_INFO             = CLCTL_EXTERNAL_CODE( 100, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_AVAILABLE_DISKS       = CLCTL_EXTERNAL_CODE( 101, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_IS_PATH_VALID             = CLCTL_EXTERNAL_CODE( 102, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_SYNC_CLUSDISK_DB          = CLCTL_EXTERNAL_CODE( 103, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STORAGE_GET_DISK_NUMBER_INFO      = CLCTL_EXTERNAL_CODE( 104, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_QUERY_DELETE                      = CLCTL_EXTERNAL_CODE( 110, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_IPADDRESS_RENEW_LEASE             = CLCTL_EXTERNAL_CODE( 111, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_IPADDRESS_RELEASE_LEASE           = CLCTL_EXTERNAL_CODE( 112, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_QUERY_MAINTENANCE_MODE            = CLCTL_EXTERNAL_CODE( 120, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_SET_MAINTENANCE_MODE              = CLCTL_EXTERNAL_CODE( 121, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STORAGE_SET_DRIVELETTER           = CLCTL_EXTERNAL_CODE( 122, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STORAGE_GET_DRIVELETTERS          = CLCTL_EXTERNAL_CODE( 123, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_DISK_INFO_EX          = CLCTL_EXTERNAL_CODE( 124, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX    = CLCTL_EXTERNAL_CODE( 125, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_DISK_INFO_EX2         = CLCTL_EXTERNAL_CODE( 126, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT   = CLCTL_EXTERNAL_CODE( 127, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_STORAGE_REMAP_DRIVELETTER         = CLCTL_EXTERNAL_CODE( 128, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_DISKID                = CLCTL_EXTERNAL_CODE( 129, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_IS_CLUSTERABLE            = CLCTL_EXTERNAL_CODE( 130, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_REMOVE_VM_OWNERSHIP       = CLCTL_EXTERNAL_CODE( 131, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STORAGE_GET_MOUNTPOINTS           = CLCTL_EXTERNAL_CODE( 132, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_DIRTY                 = CLCTL_EXTERNAL_CODE( 134, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    // Codes 135-136 are available for storage after the two previous squatters were made internal.
    CLCTL_STORAGE_GET_SHARED_VOLUME_INFO    = CLCTL_EXTERNAL_CODE( 137, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_IS_CSV_FILE               = CLCTL_EXTERNAL_CODE( 138, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_RESOURCEID            = CLCTL_EXTERNAL_CODE( 139, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_VALIDATE_PATH                     = CLCTL_EXTERNAL_CODE( 140, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_VALIDATE_NETNAME                  = CLCTL_EXTERNAL_CODE( 141, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_VALIDATE_DIRECTORY                = CLCTL_EXTERNAL_CODE( 142, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_BATCH_BLOCK_KEY                   = CLCTL_EXTERNAL_CODE( 143, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_BATCH_UNBLOCK_KEY                 = CLCTL_EXTERNAL_CODE( 144, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_FILESERVER_SHARE_ADD              = CLCTL_EXTERNAL_CODE( 145, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_FILESERVER_SHARE_DEL              = CLCTL_EXTERNAL_CODE( 146, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_FILESERVER_SHARE_MODIFY           = CLCTL_EXTERNAL_CODE( 147, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_FILESERVER_SHARE_REPORT           = CLCTL_EXTERNAL_CODE( 148, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),

    CLCTL_NETNAME_GET_OU_FOR_VCO            = CLCTL_EXTERNAL_CODE( 155, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    // Codes 160-161 are available for storage after the two previous squatters were made internal.
    CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO     = CLCTL_EXTERNAL_CODE( 162, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO    = CLCTL_EXTERNAL_CODE( 163, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_GET_SHARED_VOLUME_ID              = CLCTL_EXTERNAL_CODE( 164, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_SET_CSV_MAINTENANCE_MODE          = CLCTL_EXTERNAL_CODE( 165, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_SET_SHARED_VOLUME_BACKUP_MODE     = CLCTL_EXTERNAL_CODE( 166, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES = CLCTL_EXTERNAL_CODE( 167, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_STORAGE_GET_SHARED_VOLUME_STATES  = CLCTL_EXTERNAL_CODE( 168, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STORAGE_IS_SHARED_VOLUME          = CLCTL_EXTERNAL_CODE( 169, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_GET_CLUSDB_TIMESTAMP              = CLCTL_EXTERNAL_CODE( 170, CLUS_ACCESS_READ,  CLUS_NO_MODIFY ),

    // this control has CLUS_MODIFY bit set and will trigger property change notification, that is the only purpose of this control
    CLCTL_RW_MODIFY_NOOP                    = CLCTL_EXTERNAL_CODE( 171, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_IS_QUORUM_BLOCKED                 = CLCTL_EXTERNAL_CODE( 172, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_POOL_GET_DRIVE_INFO               = CLCTL_EXTERNAL_CODE( 173, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_GET_GUM_LOCK_OWNER                = CLCTL_EXTERNAL_CODE( 174, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_STUCK_NODES                   = CLCTL_EXTERNAL_CODE( 175, CLUS_ACCESS_READ,  CLUS_NO_MODIFY),
    CLCTL_INJECT_GEM_FAULT                    = CLCTL_EXTERNAL_CODE( 176, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_INTRODUCE_GEM_REPAIR_DELAY        = CLCTL_EXTERNAL_CODE( 177, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_SEND_DUMMY_GEM_MESSAGES           = CLCTL_EXTERNAL_CODE( 178, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_BLOCK_GEM_SEND_RECV                    = CLCTL_EXTERNAL_CODE( 179, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_GET_GEMID_VECTOR                  = CLCTL_EXTERNAL_CODE( 180, CLUS_ACCESS_READ, CLUS_NO_MODIFY),

    CLCTL_ADD_CRYPTO_CHECKPOINT_EX          = CLCTL_EXTERNAL_CODE( 181, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    // gets the last time the group moved
    CLCTL_GROUP_GET_LAST_MOVE_TIME          = CLCTL_EXTERNAL_CODE( 182, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_SET_STORAGE_CONFIGURATION         = CLCTL_EXTERNAL_CODE( 184, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_GET_STORAGE_CONFIGURATION         = CLCTL_EXTERNAL_CODE( 185, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES     = CLCTL_EXTERNAL_CODE( 186, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_REMOVE_NODE                       = CLCTL_EXTERNAL_CODE( 187, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_IS_FEATURE_INSTALLED              = CLCTL_EXTERNAL_CODE( 188, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_IS_S2D_FEATURE_SUPPORTED          = CLCTL_EXTERNAL_CODE( 189, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO    = CLCTL_EXTERNAL_CODE( 190, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_CLUSBFLT_PATHS        = CLCTL_EXTERNAL_CODE( 191, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO     = CLCTL_EXTERNAL_CODE( 192, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_CLEAR_NODE_CONNECTION_INFO        = CLCTL_EXTERNAL_CODE( 193, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_SET_DNS_DOMAIN                    = CLCTL_EXTERNAL_CODE( 194, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    // Network health.  STATUS = status matrix now, Extended = future use.
    CTCTL_GET_ROUTESTATUS_BASIC             = CLCTL_EXTERNAL_CODE( 195, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CTCTL_GET_ROUTESTATUS_EXTENDED          = CLCTL_EXTERNAL_CODE( 196, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CTCTL_GET_FAULT_DOMAIN_STATE            = CLCTL_EXTERNAL_CODE( 197, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_NETNAME_SET_PWD_INFOEX            = CLCTL_EXTERNAL_CODE( 198, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),

    CLCTL_GET_NODE_NETWORK_CONNECTIVITY     = CLCTL_EXTERNAL_CODE( 199, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),


    // Control codes 2000 to 2999 are reserved.

    CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT   = CLCTL_EXTERNAL_CODE( 2040, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    //
    // Cloud Witness Controls
    //

    // takes in a property list of AccountName, PrimaryToken
    CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS = CLCTL_EXTERNAL_CODE( 2104, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    // takes in a proeprty list of PrimaryToken
    CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN  = CLCTL_EXTERNAL_CODE( 2105, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    //
    // Rolling upgrade controls
    //
    CLCTL_RESOURCE_PREPARE_UPGRADE             = CLCTL_EXTERNAL_CODE( 2106, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_RESOURCE_UPGRADE_COMPLETED           = CLCTL_EXTERNAL_CODE( 2107, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    // takes in a property list of AccountName, PrimaryKey
    CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY = CLCTL_EXTERNAL_CODE( 2108, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    // takes in a proeprty list of PrimaryKey
    CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY  = CLCTL_EXTERNAL_CODE( 2109, CLUS_ACCESS_WRITE, CLUS_MODIFY ),


    //
    // Storage Replication
    //
    CLCTL_REPLICATION_ADD_REPLICATION_GROUP           = CLCTL_EXTERNAL_CODE(2128, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_LOG_INFO                    = CLCTL_EXTERNAL_CODE(2129, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS           = CLCTL_EXTERNAL_CODE(2130, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS   = CLCTL_EXTERNAL_CODE(2131, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS   = CLCTL_EXTERNAL_CODE(2132, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_REPLICATED_DISKS            = CLCTL_EXTERNAL_CODE(2133, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_REPLICA_VOLUMES             = CLCTL_EXTERNAL_CODE(2134, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_LOG_VOLUME                  = CLCTL_EXTERNAL_CODE(2135, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_RESOURCE_GROUP              = CLCTL_EXTERNAL_CODE(2136, CLUS_ACCESS_READ, CLUS_NO_MODIFY),
    CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO   = CLCTL_EXTERNAL_CODE(2137, CLUS_ACCESS_READ, CLUS_NO_MODIFY),


    // Get Filetime of last state change
    CLCTL_GET_STATE_CHANGE_TIME                   = CLCTL_EXTERNAL_CODE( 2903, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
    CLCTL_SET_CLUSTER_S2D_ENABLED                 = CLCTL_EXTERNAL_CODE( 2904, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    // 2906 is free and can be reused
    CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES = CLCTL_EXTERNAL_CODE( 2907, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
#endif
    CLCTL_GROUPSET_GET_GROUPS                       = CLCTL_EXTERNAL_CODE( 2908, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GROUPSET_GET_PROVIDER_GROUPS              = CLCTL_EXTERNAL_CODE( 2909, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS           = CLCTL_EXTERNAL_CODE( 2910, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GROUP_GET_PROVIDER_GROUPS                 = CLCTL_EXTERNAL_CODE( 2911, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GROUP_GET_PROVIDER_GROUPSETS              = CLCTL_EXTERNAL_CODE( 2912, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GROUP_SET_CCF_FROM_MASTER                 = CLCTL_EXTERNAL_CODE( 2913, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER            = CLCTL_EXTERNAL_CODE( 2914, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER            = CLCTL_EXTERNAL_CODE( 2915, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED        = CLCTL_EXTERNAL_CODE( 2916, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_SCALEOUT_COMMAND                          = CLCTL_EXTERNAL_CODE( 2917, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_SCALEOUT_CONTROL                          = CLCTL_EXTERNAL_CODE( 2918, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_SCALEOUT_GET_CLUSTERS                     = CLCTL_EXTERNAL_CODE( 2919, CLUS_ACCESS_READ, CLUS_MODIFY ),


    CLCTL_RELOAD_AUTOLOGGER_CONFIG                  = CLCTL_EXTERNAL_CODE( 2932, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_RENAME_SHARED_VOLUME              = CLCTL_EXTERNAL_CODE( 2933, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID         = CLCTL_EXTERNAL_CODE( 2934, CLUS_ACCESS_WRITE, CLUS_NO_MODIFY ),
    CLCTL_ENUM_AFFINITY_RULE_NAMES                  = CLCTL_EXTERNAL_CODE( 2935, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_GET_NODES_IN_FD                           = CLCTL_EXTERNAL_CODE( 2936, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_FORCE_DB_FLUSH                            = CLCTL_EXTERNAL_CODE( 2937, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

//Internal codes:
//A control code used by the Cluster service to notify a resource DLL of changes to the cluster environment. Applications cannot use internal control codes; they must use external control codes.


    //
    // Internal control codes
    //
    CLCTL_DELETE                            = CLCTL_INTERNAL_CODE( 1, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_INSTALL_NODE                      = CLCTL_INTERNAL_CODE( 2, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_EVICT_NODE                        = CLCTL_INTERNAL_CODE( 3, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_ADD_DEPENDENCY                    = CLCTL_INTERNAL_CODE( 4, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_REMOVE_DEPENDENCY                 = CLCTL_INTERNAL_CODE( 5, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_ADD_OWNER                         = CLCTL_INTERNAL_CODE( 6, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_REMOVE_OWNER                      = CLCTL_INTERNAL_CODE( 7, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    //************ Hole here at 8
    CLCTL_SET_NAME                          = CLCTL_INTERNAL_CODE( 9, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_CLUSTER_NAME_CHANGED              = CLCTL_INTERNAL_CODE( 10, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_CLUSTER_VERSION_CHANGED           = CLCTL_INTERNAL_CODE( 11, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_FIXUP_ON_UPGRADE                  = CLCTL_INTERNAL_CODE( 12, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STARTING_PHASE1                   = CLCTL_INTERNAL_CODE( 13, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STARTING_PHASE2                   = CLCTL_INTERNAL_CODE( 14, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_HOLD_IO                           = CLCTL_INTERNAL_CODE( 15, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_RESUME_IO                         = CLCTL_INTERNAL_CODE( 16, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_FORCE_QUORUM                      = CLCTL_INTERNAL_CODE( 17, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_INITIALIZE                        = CLCTL_INTERNAL_CODE( 18, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_STATE_CHANGE_REASON               = CLCTL_INTERNAL_CODE( 19, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_PROVIDER_STATE_CHANGE             = CLCTL_INTERNAL_CODE( 20, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_LEAVING_GROUP                     = CLCTL_INTERNAL_CODE( 21, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_JOINING_GROUP                     = CLCTL_INTERNAL_CODE( 22, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_FSWITNESS_GET_EPOCH_INFO          = CLCTL_INTERNAL_CODE( 23, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_FSWITNESS_SET_EPOCH_INFO          = CLCTL_INTERNAL_CODE( 24, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
    CLCTL_FSWITNESS_RELEASE_LOCK            = CLCTL_INTERNAL_CODE( 25, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_NETNAME_CREDS_NOTIFYCAM           = CLCTL_INTERNAL_CODE( 26, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_NOTIFY_QUORUM_STATUS              = CLCTL_INTERNAL_CODE( 31, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN      = CLCTL_INTERNAL_CODE( 32, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

    CLCTL_UNDELETE                          = CLCTL_INTERNAL_CODE( 33, CLUS_ACCESS_WRITE, CLUS_MODIFY ),

    CLCTL_GET_OPERATION_CONTEXT             = CLCTL_INTERNAL_CODE( 2106, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_NOTIFY_OWNER_CHANGE               = CLCTL_INTERNAL_CODE( 2120, CLUS_ACCESS_WRITE, CLUS_MODIFY ),
#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
    CLCTL_VALIDATE_CHANGE_GROUP             = CLCTL_INTERNAL_CODE( 2121, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
#endif

    CLCTL_CHECK_DRAIN_VETO                  = CLCTL_INTERNAL_CODE( 2123, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),
    CLCTL_NOTIFY_DRAIN_COMPLETE             = CLCTL_INTERNAL_CODE( 2124, CLUS_ACCESS_READ, CLUS_NO_MODIFY ),

} CLCTL_CODES;

#endif // _CLUSTER_API_TYPES_

//
// Define macros to generate object specific control codes
//
#define CLUSCTL_RESOURCE_CODE( Function ) ( \
    ((CLUS_OBJECT_RESOURCE << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_RESOURCE_TYPE_CODE( Function ) ( \
    ((CLUS_OBJECT_RESOURCE_TYPE << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_GROUP_CODE( Function ) ( \
    ((CLUS_OBJECT_GROUP << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_NODE_CODE( Function ) ( \
    ((CLUS_OBJECT_NODE << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_NETWORK_CODE( Function ) ( \
    ((CLUS_OBJECT_NETWORK << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_NETINTERFACE_CODE( Function ) ( \
    ((CLUS_OBJECT_NETINTERFACE << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_CLUSTER_CODE( Function ) ( \
    ((CLUS_OBJECT_CLUSTER << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_GROUPSET_CODE( Function ) ( \
    ((CLUS_OBJECT_GROUPSET << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_AFFINITYRULE_CODE( Function ) ( \
    ((CLUS_OBJECT_AFFINITYRULE << CLUSCTL_OBJECT_SHIFT) | Function) )

#define CLUSCTL_USER_CODE( Function, Object ) ( \
     ( (Object) << CLUSCTL_OBJECT_SHIFT)  |  CLCTL_USER_BASE  |  (Function << CLUSCTL_FUNCTION_SHIFT)  )

//
// Define macros to get the function, object, access mode, or User Base flag out
// of a control code
//
#define CLUSCTL_GET_CONTROL_FUNCTION( ControlCode ) \
    ((ControlCode >> CLUSCTL_ACCESS_SHIFT) & CLUSCTL_CONTROL_CODE_MASK)

#define CLUSCTL_GET_ACCESS_MODE( ControlCode ) \
    ((ControlCode >> CLUSCTL_ACCESS_SHIFT) & CLUSCTL_ACCESS_MODE_MASK)

#define CLUSCTL_GET_CONTROL_OBJECT( ControlCode ) \
    ((ControlCode >> CLUSCTL_OBJECT_SHIFT) & CLUSCTL_OBJECT_MASK)

#define CLUSCTL_GET_USER( ControlCode ) \
    ((ControlCode & CLCTL_USER_MASK) >> CLCTL_USER_SHIFT)

#ifndef _CLUSTER_API_TYPES_
//
// Cluster Control Codes for Resources
//
typedef enum CLUSCTL_RESOURCE_CODES {

    //
    // External resource codes
    //
    CLUSCTL_RESOURCE_UNKNOWN =
        CLUSCTL_RESOURCE_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_RESOURCE_GET_CHARACTERISTICS =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_CHARACTERISTICS ),

    CLUSCTL_RESOURCE_GET_FLAGS =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_FLAGS ),

    CLUSCTL_RESOURCE_GET_CLASS_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_CLASS_INFO ),

    CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_REQUIRED_DEPENDENCIES ),

    CLUSCTL_RESOURCE_GET_NAME =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_NAME ),

    CLUSCTL_RESOURCE_GET_ID =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_ID ),

    CLUSCTL_RESOURCE_GET_RESOURCE_TYPE =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_RESOURCE_TYPE ),

    CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),

    CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_REGISTRY_CHECKPOINT ),

    CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT =
        CLUSCTL_RESOURCE_CODE( CLCTL_DELETE_REGISTRY_CHECKPOINT ),

    CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_REGISTRY_CHECKPOINTS ),

    CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_CRYPTO_CHECKPOINT ),

    CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT =
        CLUSCTL_RESOURCE_CODE( CLCTL_DELETE_CRYPTO_CHECKPOINT ),

    CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_CRYPTO_CHECKPOINT_EX ),

    CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_CRYPTO_CHECKPOINTS ),

    CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_LOADBAL_PROCESS_LIST ),

    CLUSCTL_RESOURCE_GET_NETWORK_NAME =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_NETWORK_NAME ),

    CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN ),

    CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_SET_PWD_INFO ),

    CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_SET_PWD_INFOEX ),

    CLUSCTL_RESOURCE_NETNAME_DELETE_CO =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_DELETE_CO ),

    CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_VALIDATE_VCO ),

    CLUSCTL_RESOURCE_NETNAME_RESET_VCO =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_RESET_VCO ),

    CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_REPAIR_VCO ),

    CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_REGISTER_DNS_RECORDS ),

    CLUSCTL_RESOURCE_GET_DNS_NAME =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_DNS_NAME ),

    CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_DISK_INFO ),

    CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_DISK_NUMBER_INFO ),

    CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_IS_PATH_VALID ),

    CLUSCTL_RESOURCE_QUERY_DELETE =
        CLUSCTL_RESOURCE_CODE( CLCTL_QUERY_DELETE ),

    CLUSCTL_RESOURCE_UPGRADE_DLL =
        CLUSCTL_RESOURCE_CODE( CLCTL_RESOURCE_UPGRADE_DLL ),

    CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE =
        CLUSCTL_RESOURCE_CODE( CLCTL_IPADDRESS_RENEW_LEASE ),

    CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE =
        CLUSCTL_RESOURCE_CODE( CLCTL_IPADDRESS_RELEASE_LEASE ),

    CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT ),

    CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT ),

    CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE =
        CLUSCTL_RESOURCE_CODE( CLCTL_QUERY_MAINTENANCE_MODE ),

    CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_MAINTENANCE_MODE ),

    CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_SET_DRIVELETTER ),

    CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_DISK_INFO_EX ),

    CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2 =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_DISK_INFO_EX2 ),


    CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_MOUNTPOINTS ),

    CLUSCTL_RESOURCE_STORAGE_GET_DIRTY =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_DIRTY ),

    CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_SHARED_VOLUME_INFO ),

    CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_CSV_MAINTENANCE_MODE ),

    CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO =
        CLUSCTL_RESOURCE_CODE( CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO ),

    CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO =
        CLUSCTL_RESOURCE_CODE( CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO ),

    CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_SHARED_VOLUME_BACKUP_MODE ),

    CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES ),

    // get diagnostics info about resource failures
    CLUSCTL_RESOURCE_GET_FAILURE_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_FAILURE_INFO ),

    CLUSCTL_RESOURCE_STORAGE_GET_DISKID =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_DISKID ),

    CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_GET_SHARED_VOLUME_STATES ),

    CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_IS_SHARED_VOLUME ),

    CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED =
        CLUSCTL_RESOURCE_CODE( CLCTL_IS_QUORUM_BLOCKED ),

    CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_POOL_GET_DRIVE_INFO ),

    CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN ),

    CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_SET_PWD_INFO ),

    CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_SET_PWD_INFOEX ),


    //
    // Internal resource codes
    //
    CLUSCTL_RESOURCE_DELETE =
        CLUSCTL_RESOURCE_CODE( CLCTL_DELETE ),

    CLUSCTL_RESOURCE_UNDELETE =
            CLUSCTL_RESOURCE_CODE( CLCTL_UNDELETE ),

    CLUSCTL_RESOURCE_INSTALL_NODE =
        CLUSCTL_RESOURCE_CODE( CLCTL_INSTALL_NODE ),

    CLUSCTL_RESOURCE_EVICT_NODE =
        CLUSCTL_RESOURCE_CODE( CLCTL_EVICT_NODE ),

    CLUSCTL_RESOURCE_ADD_DEPENDENCY =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_DEPENDENCY ),

    CLUSCTL_RESOURCE_REMOVE_DEPENDENCY =
        CLUSCTL_RESOURCE_CODE( CLCTL_REMOVE_DEPENDENCY ),

    CLUSCTL_RESOURCE_ADD_OWNER =
        CLUSCTL_RESOURCE_CODE( CLCTL_ADD_OWNER ),

    CLUSCTL_RESOURCE_REMOVE_OWNER =
        CLUSCTL_RESOURCE_CODE( CLCTL_REMOVE_OWNER ),

    CLUSCTL_RESOURCE_SET_NAME =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_NAME ),

    CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED =
        CLUSCTL_RESOURCE_CODE( CLCTL_CLUSTER_NAME_CHANGED ),

    CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED =
        CLUSCTL_RESOURCE_CODE( CLCTL_CLUSTER_VERSION_CHANGED ),

    CLUSCTL_RESOURCE_FORCE_QUORUM =
        CLUSCTL_RESOURCE_CODE( CLCTL_FORCE_QUORUM ),

    CLUSCTL_RESOURCE_INITIALIZE =
        CLUSCTL_RESOURCE_CODE( CLCTL_INITIALIZE ),

    CLUSCTL_RESOURCE_STATE_CHANGE_REASON =
        CLUSCTL_RESOURCE_CODE( CLCTL_STATE_CHANGE_REASON ),

    CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE =
        CLUSCTL_RESOURCE_CODE( CLCTL_PROVIDER_STATE_CHANGE ),

    CLUSCTL_RESOURCE_LEAVING_GROUP =
        CLUSCTL_RESOURCE_CODE( CLCTL_LEAVING_GROUP ),

    CLUSCTL_RESOURCE_JOINING_GROUP =
        CLUSCTL_RESOURCE_CODE( CLCTL_JOINING_GROUP ),

    CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_FSWITNESS_GET_EPOCH_INFO ),

    CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO =
        CLUSCTL_RESOURCE_CODE( CLCTL_FSWITNESS_SET_EPOCH_INFO ),

    CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK =
        CLUSCTL_RESOURCE_CODE( CLCTL_FSWITNESS_RELEASE_LOCK ),

    CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM =
        CLUSCTL_RESOURCE_CODE( CLCTL_NETNAME_CREDS_NOTIFYCAM ),

    /*
        CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT

        input is GET_OPERATION_CONTEXT_PARAMS

        output should be a property list containing
        CLUSRES_GET_OPERATION_CONTEXT_FLAGS

        and 0->many properties with the name being a resource type name
        and the value being what will be passed along with it
    */

    CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_OPERATION_CONTEXT ),

    CLUSCTL_RESOURCE_RW_MODIFY_NOOP =
        CLUSCTL_RESOURCE_CODE( CLCTL_RW_MODIFY_NOOP ),

    CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS =
        CLUSCTL_RESOURCE_CODE( CLCTL_NOTIFY_QUORUM_STATUS ),

    CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE =
        CLUSCTL_RESOURCE_CODE( CLCTL_NOTIFY_OWNER_CHANGE ),

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
    CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP =
        CLUSCTL_RESOURCE_CODE( CLCTL_VALIDATE_CHANGE_GROUP ),

    CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_RENAME_SHARED_VOLUME ),

    CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID ),
#endif


    CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN = CLUSCTL_RESOURCE_CODE( CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN ),

    CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY = CLUSCTL_RESOURCE_CODE( CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY ),

    CLUSCTL_RESOURCE_PREPARE_UPGRADE =
        CLUSCTL_RESOURCE_CODE( CLCTL_RESOURCE_PREPARE_UPGRADE ),

    CLUSCTL_RESOURCE_UPGRADE_COMPLETED =
        CLUSCTL_RESOURCE_CODE( CLCTL_RESOURCE_UPGRADE_COMPLETED ),

    CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_STATE_CHANGE_TIME ),

    CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER ),

    CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER =
        CLUSCTL_RESOURCE_CODE( CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER ),

    CLUSCTL_RESOURCE_SCALEOUT_COMMAND =
        CLUSCTL_RESOURCE_CODE( CLCTL_SCALEOUT_COMMAND ),
    CLUSCTL_RESOURCE_SCALEOUT_CONTROL =
        CLUSCTL_RESOURCE_CODE( CLCTL_SCALEOUT_CONTROL ),
    CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS =
        CLUSCTL_RESOURCE_CODE( CLCTL_SCALEOUT_GET_CLUSTERS ),


    CLUSCTL_RESOURCE_CHECK_DRAIN_VETO =
        CLUSCTL_RESOURCE_CODE( CLCTL_CHECK_DRAIN_VETO ),

    CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE =
        CLUSCTL_RESOURCE_CODE( CLCTL_NOTIFY_DRAIN_COMPLETE ),

    CLUSCTL_RESOURCE_GET_NODES_IN_FD =
        CLUSCTL_RESOURCE_CODE( CLCTL_GET_NODES_IN_FD ),

} CLUSCTL_RESOURCE_CODES;

//
// Cluster Control Codes for Resource Types
//
typedef enum CLUSCTL_RESOURCE_TYPE_CODES {

    // External
    CLUSCTL_RESOURCE_TYPE_UNKNOWN =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS  =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_CHARACTERISTICS ),

    CLUSCTL_RESOURCE_TYPE_GET_FLAGS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_FLAGS ),

    CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_CLASS_INFO ),

    CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_REQUIRED_DEPENDENCIES ),

    CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_ARB_TIMEOUT ),

    CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS ),

    CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),

    CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS ),

    CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_REGISTRY_CHECKPOINTS ),

    CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_GET_CRYPTO_CHECKPOINTS ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_GET_AVAILABLE_DISKS ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_SYNC_CLUSDISK_DB ),

    CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_NETNAME ),

    CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_NETNAME_GET_OU_FOR_VCO ),

    CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_PATH ),

    CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_DIRECTORY ),

    CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_PATH ),

    CLUSCTL_RESOURCE_TYPE_QUERY_DELETE =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_QUERY_DELETE ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_GET_DRIVELETTERS ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_REMAP_DRIVELETTER ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_GET_DISKID ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_GET_RESOURCEID ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_IS_CLUSTERABLE ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_REMOVE_VM_OWNERSHIP ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE =
        CLUSCTL_RESOURCE_CODE( CLCTL_STORAGE_IS_CSV_FILE ),

    CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_VALIDATE_PATH ),

    // Internal
    CLUSCTL_RESOURCE_TYPE_INSTALL_NODE =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_INSTALL_NODE ),

    CLUSCTL_RESOURCE_TYPE_EVICT_NODE =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_EVICT_NODE ),

    CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_CLUSTER_VERSION_CHANGED ),

    CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_FIXUP_ON_UPGRADE ),

    CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1 =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STARTING_PHASE1 ),

    CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2 =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STARTING_PHASE2 ),

    CLUSCTL_RESOURCE_TYPE_HOLD_IO =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_HOLD_IO ),

    CLUSCTL_RESOURCE_TYPE_RESUME_IO =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_RESUME_IO ),

    CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_REPLICATED_DISKS ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_REPLICA_VOLUMES ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_LOG_VOLUME ),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_RESOURCE_GROUP),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_GET_LOG_INFO),

    CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_REPLICATION_ADD_REPLICATION_GROUP ),


    CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS = CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS ),
    CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY = CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY ),


    CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_RESOURCE_PREPARE_UPGRADE ),

    CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_RESOURCE_UPGRADE_COMPLETED ),

    CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN =
        CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN ),

    CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO =
            CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_CHECK_DRAIN_VETO ),

    CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE =
            CLUSCTL_RESOURCE_TYPE_CODE( CLCTL_NOTIFY_DRAIN_COMPLETE ),

} CLUSCTL_RESOURCE_TYPE_CODES;


    // To be used with Control Code CLUSCTL_RESOURCE_TYPE_GET_LOCAL_NODE_SRIOV_INFO Only:
    typedef struct NodeSriovInfo {
        DWORD VFTotal;
        DWORD VFUsed;
        DWORD QPTotal;
        DWORD QPUsed;
    } NodeSriovInfo;


//
// Cluster Control Codes for Groups
//
typedef enum CLUSCTL_GROUP_CODES {

    // External
    CLUSCTL_GROUP_UNKNOWN =
        CLUSCTL_GROUP_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_GROUP_GET_CHARACTERISTICS =
        CLUSCTL_GROUP_CODE( CLCTL_GET_CHARACTERISTICS ),

    CLUSCTL_GROUP_GET_FLAGS =
        CLUSCTL_GROUP_CODE( CLCTL_GET_FLAGS ),

    CLUSCTL_GROUP_GET_NAME =
        CLUSCTL_GROUP_CODE( CLCTL_GET_NAME ),

    CLUSCTL_GROUP_GET_ID =
        CLUSCTL_GROUP_CODE( CLCTL_GET_ID ),

    CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_GROUP_GET_COMMON_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_GROUP_SET_COMMON_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_GROUP_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_GROUP_QUERY_DELETE =
        CLUSCTL_GROUP_CODE( CLCTL_QUERY_DELETE ),

    CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS=
        CLUSCTL_GROUP_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS=
        CLUSCTL_GROUP_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),

    CLUSCTL_GROUP_GET_FAILURE_INFO =
        CLUSCTL_GROUP_CODE( CLCTL_GET_FAILURE_INFO ),


    // last time a group moved see struct
    // input - nothing
    // output - see CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT
    CLUSCTL_GROUP_GET_LAST_MOVE_TIME =
        CLUSCTL_GROUP_CODE( CLCTL_GROUP_GET_LAST_MOVE_TIME ),


    CLUSCTL_GROUP_SET_CCF_FROM_MASTER =
        CLUSCTL_GROUP_CODE( CLCTL_GROUP_SET_CCF_FROM_MASTER ),

    // Internal

} CLUSCTL_GROUP_CODES;

//
// Cluster Control Codes for Nodes
//
typedef enum CLUSCTL_NODE_CODES {

    // External
    CLUSCTL_NODE_UNKNOWN =
        CLUSCTL_NODE_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_NODE_GET_CHARACTERISTICS =
        CLUSCTL_NODE_CODE( CLCTL_GET_CHARACTERISTICS ),

    CLUSCTL_NODE_GET_FLAGS =
        CLUSCTL_NODE_CODE( CLCTL_GET_FLAGS ),

    CLUSCTL_NODE_GET_NAME =
        CLUSCTL_NODE_CODE( CLCTL_GET_NAME ),

    CLUSCTL_NODE_GET_ID =
        CLUSCTL_NODE_CODE( CLCTL_GET_ID ),

    CLUSCTL_NODE_ENUM_COMMON_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_NODE_GET_COMMON_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_NODE_SET_COMMON_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_NODE_GET_PRIVATE_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_NODE_SET_PRIVATE_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_NODE_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS=
        CLUSCTL_NODE_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS=
        CLUSCTL_NODE_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),

    CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME =
        CLUSCTL_NODE_CODE( CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME ),

    CLUSCTL_NODE_GET_STUCK_NODES =
        CLUSCTL_NODE_CODE( CLCTL_GET_STUCK_NODES),

    CLUSCTL_NODE_INJECT_GEM_FAULT =
        CLUSCTL_NODE_CODE( CLCTL_INJECT_GEM_FAULT),

    CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY =
        CLUSCTL_NODE_CODE(CLCTL_INTRODUCE_GEM_REPAIR_DELAY),

    CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES =
        CLUSCTL_NODE_CODE(CLCTL_SEND_DUMMY_GEM_MESSAGES),

    CLUSCTL_NODE_BLOCK_GEM_SEND_RECV =
        CLUSCTL_NODE_CODE(CLCTL_BLOCK_GEM_SEND_RECV),

    CLUSCTL_NODE_GET_GEMID_VECTOR =
        CLUSCTL_NODE_CODE(CLCTL_GET_GEMID_VECTOR),
} CLUSCTL_NODE_CODES;

//
// Cluster Control Codes for Networks
//
typedef enum CLUSCTL_NETWORK_CODES {

    // External
    CLUSCTL_NETWORK_UNKNOWN =
        CLUSCTL_NETWORK_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_NETWORK_GET_CHARACTERISTICS =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_CHARACTERISTICS ),

    CLUSCTL_NETWORK_GET_FLAGS =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_FLAGS ),

    CLUSCTL_NETWORK_GET_NAME =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_NAME ),

    CLUSCTL_NETWORK_GET_ID =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_ID ),

    CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_NETWORK_GET_COMMON_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_NETWORK_SET_COMMON_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_NETWORK_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS=
        CLUSCTL_NETWORK_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS=
        CLUSCTL_NETWORK_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),


} CLUSCTL_NETWORK_CODES;

//
// Cluster Control Codes for Network Interfaces
//
typedef enum CLUSCTL_NETINTERFACE_CODES {

    // External
    CLUSCTL_NETINTERFACE_UNKNOWN =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_CHARACTERISTICS ),

    CLUSCTL_NETINTERFACE_GET_FLAGS =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_FLAGS ),

    CLUSCTL_NETINTERFACE_GET_NAME =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_NAME ),

    CLUSCTL_NETINTERFACE_GET_ID =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_ID ),

    CLUSCTL_NETINTERFACE_GET_NODE =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_NODE ),

    CLUSCTL_NETINTERFACE_GET_NETWORK =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_NETWORK ),

    CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_NETINTERFACE_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS=
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS=
        CLUSCTL_NETINTERFACE_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),


} CLUSCTL_NETINTERFACE_CODES;

//
// Cluster Control Codes for Clusters
//
typedef enum CLUSCTL_CLUSTER_CODES {

    // External
    CLUSCTL_CLUSTER_UNKNOWN =
        CLUSCTL_CLUSTER_CODE( CLCTL_UNKNOWN ),

    CLUSCTL_CLUSTER_GET_FQDN =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_FQDN ),

    CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION =
        CLUSCTL_CLUSTER_CODE( CLCTL_SET_STORAGE_CONFIGURATION ),

    CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_STORAGE_CONFIGURATION ),

    CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES ),

    CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_ENUM_COMMON_PROPERTIES ),

    CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_VALIDATE_COMMON_PROPERTIES ),

    CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_ENUM_PRIVATE_PROPERTIES ),

    CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_RO_PRIVATE_PROPERTIES ),

    CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_PRIVATE_PROPERTIES ),

    CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_SET_PRIVATE_PROPERTIES ),

    CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES =
        CLUSCTL_CLUSTER_CODE( CLCTL_VALIDATE_PRIVATE_PROPERTIES ),

    CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS=
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_COMMON_PROPERTY_FMTS ),

    CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS=
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_PRIVATE_PROPERTY_FMTS ),

    CLUSCTL_CLUSTER_CHECK_VOTER_EVICT=
        CLUSCTL_CLUSTER_CODE( CLCTL_CHECK_VOTER_EVICT ),

    CLUSCTL_CLUSTER_CHECK_VOTER_DOWN=
        CLUSCTL_CLUSTER_CODE( CLCTL_CHECK_VOTER_DOWN ),

    CLUSCTL_CLUSTER_SHUTDOWN=
        CLUSCTL_CLUSTER_CODE( CLCTL_SHUTDOWN ),

    CLUSCTL_CLUSTER_BATCH_BLOCK_KEY =
        CLUSCTL_CLUSTER_CODE( CLCTL_BATCH_BLOCK_KEY ),

    CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY =
        CLUSCTL_CLUSTER_CODE( CLCTL_BATCH_UNBLOCK_KEY ),

    CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_SHARED_VOLUME_ID ),

    CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_CLUSDB_TIMESTAMP ),

    CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_GUM_LOCK_OWNER ),

    CLUSCTL_CLUSTER_REMOVE_NODE =
        CLUSCTL_CLUSTER_CODE( CLCTL_REMOVE_NODE),

    CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS =
        CLUSCTL_CLUSTER_CODE( CLCTL_SET_ACCOUNT_ACCESS),

    CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO =
        CLUSCTL_CLUSTER_CODE ( CLCTL_CLEAR_NODE_CONNECTION_INFO ),

    CLUSCTL_CLUSTER_SET_DNS_DOMAIN =
        CLUSCTL_CLUSTER_CODE ( CLCTL_SET_DNS_DOMAIN ),


#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
    CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED =
        CLUSCTL_CLUSTER_CODE( CLCTL_SET_CLUSTER_S2D_ENABLED ),

    CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES =
        CLUSCTL_CLUSTER_CODE( CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES ),

    CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME =
        CLUSCTL_CLUSTER_CODE( CLCTL_STORAGE_RENAME_SHARED_VOLUME ),

    CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID =
        CLUSCTL_CLUSTER_CODE( CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID ),
#endif


    CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG  =
        CLUSCTL_CLUSTER_CODE( CLCTL_RELOAD_AUTOLOGGER_CONFIG  ),

    CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES =
        CLUSCTL_CLUSTER_CODE( CLCTL_ENUM_AFFINITY_RULE_NAMES ),

    CLUSCTL_CLUSTER_GET_NODES_IN_FD =
        CLUSCTL_CLUSTER_CODE( CLCTL_GET_NODES_IN_FD ),

    CLUSCTL_CLUSTER_FORCE_FLUSH_DB =
        CLUSCTL_CLUSTER_CODE( CLCTL_FORCE_DB_FLUSH ),

    CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN =
        CLUSCTL_CLUSTER_CODE( CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN ),


    CLUSCTL_CLUSTER_CHECK_VOTER_EVICT_WITNESS =
        CLUSCTL_CLUSTER_CODE( CLCTL_CHECK_VOTER_EVICT_WITNESS ),

    CLUSCTL_CLUSTER_CHECK_VOTER_DOWN_WITNESS =
        CLUSCTL_CLUSTER_CODE( CLCTL_CHECK_VOTER_DOWN_WITNESS ),

} CLUSCTL_CLUSTER_CODES;

//
// Cluster Control Codes for GroupSets
//
typedef enum CLUSCTL_GROUPSET_CODES {
    CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES =
        CLUSCTL_GROUPSET_CODE( CLCTL_GET_COMMON_PROPERTIES ),

    CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES =
        CLUSCTL_GROUPSET_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

    CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES =
        CLUSCTL_GROUPSET_CODE( CLCTL_SET_COMMON_PROPERTIES ),

    CLUSCTL_GROUPSET_GET_GROUPS =
        CLUSCTL_GROUPSET_CODE( CLCTL_GROUPSET_GET_GROUPS ),
    CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS =
        CLUSCTL_GROUPSET_CODE( CLCTL_GROUPSET_GET_PROVIDER_GROUPS ),
    CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS =
        CLUSCTL_GROUPSET_CODE( CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS ),
    CLUSCTL_GROUP_GET_PROVIDER_GROUPS =
        CLUSCTL_GROUPSET_CODE( CLCTL_GROUP_GET_PROVIDER_GROUPS ),
    CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS =
        CLUSCTL_GROUPSET_CODE( CLCTL_GROUP_GET_PROVIDER_GROUPSETS ),

    CLUSCTL_GROUPSET_GET_ID =
        CLUSCTL_GROUPSET_CODE( CLCTL_GET_ID ),


} CLUSCTL_GROUPSET_CODES;

//
// Cluster control codes for Affinity Rules
//
typedef enum CLUSCTL_AFFINITYRULE_CODES {
        CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES =
                CLUSCTL_AFFINITYRULE_CODE( CLCTL_GET_COMMON_PROPERTIES ),

        CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES =
                CLUSCTL_AFFINITYRULE_CODE( CLCTL_GET_RO_COMMON_PROPERTIES ),

        CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES =
                CLUSCTL_AFFINITYRULE_CODE( CLCTL_SET_COMMON_PROPERTIES ),

        CLUSCTL_AFFINITYRULE_GET_ID =
                CLUSCTL_AFFINITYRULE_CODE( CLCTL_GET_ID ),

        CLUSCTL_AFFINITYRULE_GET_GROUPNAMES =
                CLUSCTL_AFFINITYRULE_CODE( CLCTL_GROUPSET_GET_GROUPS ),

} CLUSCTL_AFFINITYRULE_CODES;


//
// Cluster Resource Class types
//
typedef enum CLUSTER_RESOURCE_CLASS {
    CLUS_RESCLASS_UNKNOWN = 0,
    CLUS_RESCLASS_STORAGE,
    CLUS_RESCLASS_NETWORK,
    CLUS_RESCLASS_USER = 32768
} CLUSTER_RESOURCE_CLASS;

//
// Define Resource SubClass bits
//
// legacy subclass struct
//
typedef enum CLUS_RESSUBCLASS {
    CLUS_RESSUBCLASS_SHARED =                       0x80000000
} CLUS_RESSUBCLASS;

typedef enum CLUS_RESSUBCLASS_STORAGE {
    CLUS_RESSUBCLASS_STORAGE_SHARED_BUS    =           0x80000000,
    CLUS_RESSUBCLASS_STORAGE_DISK          =           0x40000000,
    CLUS_RESSUBCLASS_STORAGE_REPLICATION   =           0x10000000
} CLUS_RESSUBCLASS_STORAGE;

typedef enum CLUS_RESSUBCLASS_NETWORK {
    CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL =    0x80000000    // Identifies IP address providers
} CLUS_RESSUBCLASS_NETWORK;

//
// Cluster Characteristics used by resource types and resources
//
typedef enum CLUS_CHARACTERISTICS {
    CLUS_CHAR_UNKNOWN                       = 0x00000000,
    CLUS_CHAR_QUORUM                        = 0x00000001,
    CLUS_CHAR_DELETE_REQUIRES_ALL_NODES     = 0x00000002,
    CLUS_CHAR_LOCAL_QUORUM                  = 0x00000004,       // deprecated in Vista
    CLUS_CHAR_LOCAL_QUORUM_DEBUG            = 0x00000008,       // deprecated in Vista
    CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON  = 0x00000010,
    CLUS_CHAR_BROADCAST_DELETE              = 0x00000020,
    CLUS_CHAR_SINGLE_CLUSTER_INSTANCE       = 0x00000040,       // only one resource of this type allowed per cluster
    CLUS_CHAR_SINGLE_GROUP_INSTANCE         = 0x00000080,       // only one resource of this type allowed per group
    CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP= 0x00000100,
    CLUS_CHAR_PLACEMENT_DATA                = 0x00000200,
    CLUS_CHAR_MONITOR_DETACH                = 0x00000400,
    CLUS_CHAR_MONITOR_REATTACH              = 0x00000800,
    CLUS_CHAR_OPERATION_CONTEXT             = 0x00001000,
    CLUS_CHAR_CLONES                        = 0x00002000,
    CLUS_CHAR_NOT_PREEMPTABLE               = 0x00004000,
    CLUS_CHAR_NOTIFY_NEW_OWNER              = 0x00008000,
    CLUS_CHAR_SUPPORTS_UNMONITORED_STATE    = 0x00010000,
    CLUS_CHAR_INFRASTRUCTURE                = 0x00020000,       // The resource type is for infrastructure and is not for roles
    CLUS_CHAR_VETO_DRAIN                    = 0x00040000,
    CLUS_CHAR_DRAIN_LOCAL_OFFLINE			= 0x00080000

} CLUS_CHARACTERISTICS;

//
// Cluster Flags
//
typedef enum CLUS_FLAGS {
    CLUS_FLAG_CORE          = 0x00000001
}  CLUS_FLAGS;


//
// Cluster Resource Property Helper Structures
//

#if ( !MIDL_PASS && !__midl )

// Property syntax.  Used for property names and values.
typedef union CLUSPROP_SYNTAX {
    DWORD dw;
    struct {
        WORD wFormat;
        WORD wType;
    } DUMMYSTRUCTNAME;
} CLUSPROP_SYNTAX, *PCLUSPROP_SYNTAX;

// Property value.
typedef struct CLUSPROP_VALUE {
    CLUSPROP_SYNTAX Syntax;
    DWORD           cbLength;
} CLUSPROP_VALUE, *PCLUSPROP_VALUE;

// Binary property value.
#ifdef __cplusplus
typedef struct CLUSPROP_BINARY : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_BINARY {
    CLUSPROP_VALUE;
#endif
    BYTE            rgb[];
} CLUSPROP_BINARY, *PCLUSPROP_BINARY;

// WORD property value.
#ifdef __cplusplus
typedef struct CLUSPROP_WORD : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_WORD {
    CLUSPROP_VALUE;
#endif
    WORD            w;
} CLUSPROP_WORD, *PCLUSPROP_WORD;

// DWORD property value.
#ifdef __cplusplus
typedef struct CLUSPROP_DWORD : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_DWORD {
    CLUSPROP_VALUE;
#endif
    DWORD           dw;
} CLUSPROP_DWORD, *PCLUSPROP_DWORD;

// LONG property value.
#ifdef __cplusplus
typedef struct CLUSPROP_LONG : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_LONG {
    CLUSPROP_VALUE;
#endif
    LONG           l;
} CLUSPROP_LONG, *PCLUSPROP_LONG;

// String property value.
#ifdef __cplusplus
typedef struct CLUSPROP_SZ : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_SZ {
    CLUSPROP_VALUE;
#endif
    WCHAR           sz[];
} CLUSPROP_SZ, *PCLUSPROP_SZ;

// Multiple string property value.
typedef CLUSPROP_SZ CLUSPROP_MULTI_SZ, *PCLUSPROP_MULTI_SZ;

// Property name.
typedef CLUSPROP_SZ CLUSPROP_PROPERTY_NAME, *PCLUSPROP_PROPERTY_NAME;

// Unsigned large Integer property value.
#ifdef __cplusplus
typedef struct CLUSPROP_ULARGE_INTEGER
    : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_ULARGE_INTEGER {
    CLUSPROP_VALUE;
#endif
    ULARGE_INTEGER li;
} CLUSPROP_ULARGE_INTEGER;

typedef CLUSPROP_ULARGE_INTEGER UNALIGNED *PCLUSPROP_ULARGE_INTEGER;

// Signed large Integer property value.
#ifdef __cplusplus
typedef struct CLUSPROP_LARGE_INTEGER
    : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_LARGE_INTEGER {
    CLUSPROP_VALUE;
#endif
    LARGE_INTEGER li;
} CLUSPROP_LARGE_INTEGER;

typedef CLUSPROP_LARGE_INTEGER UNALIGNED *PCLUSPROP_LARGE_INTEGER;

// Security Descriptor property value.
#ifdef __cplusplus
typedef struct CLUSPROP_SECURITY_DESCRIPTOR : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_SECURITY_DESCRIPTOR {
    CLUSPROP_VALUE;
#endif
    union {
        SECURITY_DESCRIPTOR_RELATIVE    sd;
        BYTE                            rgbSecurityDescriptor[];
    } DUMMYUNIONNAME;
} CLUSPROP_SECURITY_DESCRIPTOR, *PCLUSPROP_SECURITY_DESCRIPTOR;

// FILETIME Time property value.
#ifdef __cplusplus
typedef struct CLUSPROP_FILETIME
    : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_FILETIME {
    CLUSPROP_VALUE;
#endif
    FILETIME ft;
} CLUSPROP_FILETIME, *PCLUSPROP_FILETIME;

// Resource class info returned by CLCTL_GET_CLASS_INFO control functions.
typedef struct CLUS_RESOURCE_CLASS_INFO {
    union {
        struct {
            union {
                DWORD                   dw;
                CLUSTER_RESOURCE_CLASS  rc;
                } DUMMYUNIONNAME;
            DWORD           SubClass;
        } DUMMYSTRUCTNAME;
        ULARGE_INTEGER      li;
    } DUMMYUNIONNAME;
} CLUS_RESOURCE_CLASS_INFO, *PCLUS_RESOURCE_CLASS_INFO;

// Resource class property value.
#ifdef __cplusplus
typedef struct CLUSPROP_RESOURCE_CLASS
    : public CLUSPROP_VALUE {
#else
typedef struct CLUSPROP_RESOURCE_CLASS {
    CLUSPROP_VALUE;
#endif
    CLUSTER_RESOURCE_CLASS rc;
} CLUSPROP_RESOURCE_CLASS, *PCLUSPROP_RESOURCE_CLASS;

// Resource class info property value.
#ifdef __cplusplus
typedef struct CLUSPROP_RESOURCE_CLASS_INFO
    : public CLUSPROP_VALUE
    , public CLUS_RESOURCE_CLASS_INFO {
#else
typedef struct CLUSPROP_RESOURCE_CLASS_INFO {
    CLUSPROP_VALUE;
    CLUS_RESOURCE_CLASS_INFO;
#endif
} CLUSPROP_RESOURCE_CLASS_INFO, *PCLUSPROP_RESOURCE_CLASS_INFO;

// One entry from list returned by CLCTL_GET_REQUIRED_DEPENDENCIES control functions.
typedef union CLUSPROP_REQUIRED_DEPENDENCY {
    CLUSPROP_VALUE          Value;
    CLUSPROP_RESOURCE_CLASS ResClass;
    CLUSPROP_SZ             ResTypeName;
} CLUSPROP_REQUIRED_DEPENDENCY, *PCLUSPROP_REQUIRED_DEPENDENCY;

typedef CLUSPROP_DWORD CLUSPROP_DISK_NUMBER, *PCLUSPROP_DISK_NUMBER;

#endif // MIDL_PASS
#endif // _CLUSTER_API_TYPES_

#ifndef _CLUSTER_API_TYPES_

// Disk partition information flags.
typedef enum CLUSPROP_PIFLAGS {
    CLUSPROP_PIFLAG_STICKY          = 0x00000001,
    CLUSPROP_PIFLAG_REMOVABLE       = 0x00000002,
    CLUSPROP_PIFLAG_USABLE          = 0x00000004,
    CLUSPROP_PIFLAG_DEFAULT_QUORUM  = 0x00000008,
    CLUSPROP_PIFLAG_USABLE_FOR_CSV  = 0x00000010,
    CLUSPROP_PIFLAG_ENCRYPTION_ENABLED = 0x00000020,
    CLUSPROP_PIFLAG_RAW             = 0x00000040,
    CLUSPROP_PIFLAG_UNKNOWN         = 0x80000000
} CLUSPROP_PIFLAGS;

#if ( !MIDL_PASS && !__midl )
//force quorum information, useful for QON type resources
//to be able to continue operation without the quorum
typedef struct CLUS_FORCE_QUORUM_INFO {
    DWORD           dwSize;             // size of this struct including the nodes list.
    DWORD           dwNodeBitMask;      // a bit mask representing the max assumed node set
    DWORD           dwMaxNumberofNodes; // the number of bits set in the mask
    WCHAR           multiszNodeList[1]; // Multi sz list of nodes
} CLUS_FORCE_QUORUM_INFO, *PCLUS_FORCE_QUORUM_INFO;


// Disk partition information.
typedef struct CLUS_PARTITION_INFO {
    DWORD           dwFlags;
    WCHAR           szDeviceName[MAX_PATH];
    WCHAR           szVolumeLabel[MAX_PATH];
    DWORD           dwSerialNumber;
    DWORD           rgdwMaximumComponentLength;
    DWORD           dwFileSystemFlags;
    WCHAR           szFileSystem[32];
} CLUS_PARTITION_INFO, *PCLUS_PARTITION_INFO;

// Disk partition information ex

// NOTE: property lists are 32b aligned which means this structure could be returned with a starting
// address that is 32b. aligned, i.e., an address ending in 0, 4, 8 or 0xC. The distance to the
// ULARGE_INTEGER members are properly aligned when the address of the structure ends with 0 or 8 but
// are unaligned when the structure addresses ends with 4 or 0xC. Since it is unpredictable as to
// the alignment of the structure's address, the developer must always access the ULARGE_INTEGER members
// with unaligned pointers or copy the data to another, aligned structure.

typedef struct CLUS_PARTITION_INFO_EX {
    DWORD           dwFlags;
    WCHAR           szDeviceName[MAX_PATH];
    WCHAR           szVolumeLabel[MAX_PATH];
    DWORD           dwSerialNumber;
    DWORD           rgdwMaximumComponentLength;
    DWORD           dwFileSystemFlags;
    WCHAR           szFileSystem[32];
    ULARGE_INTEGER  TotalSizeInBytes;
    ULARGE_INTEGER  FreeSizeInBytes;
    DWORD           DeviceNumber;
    DWORD           PartitionNumber;
    GUID            VolumeGuid;
} CLUS_PARTITION_INFO_EX, *PCLUS_PARTITION_INFO_EX;

// Disk partition information ex V2 contains these members in addition to ex.
typedef struct CLUS_PARTITION_INFO_EX2 {
    GUID            GptPartitionId;
    WCHAR           szPartitionName[MAX_PATH];
    DWORD           EncryptionFlags;
} CLUS_PARTITION_INFO_EX2, *PCLUS_PARTITION_INFO_EX2;

// Bitmask for volume encryption flags (used in CLUS_PARTITION_INFO_EX2)
#define BitLockerEnabled       (0x00000001L)
#define BitLockerDecrypted     (0x00000004L)
#define BitlockerEncrypted     (0x00000008L)
#define BitLockerDecrypting    (0x00000010L)
#define BitlockerEncrypting    (0x00000020L)
#define BitLockerPaused        (0x00000040L)
#define BitLockerStopped       (0x00000080L)
#define BitLockerFlagsAll      (BitLockerEnabled | BitLockerDecrypted | BitlockerEncrypted | BitLockerDecrypting | BitlockerEncrypting | BitLockerPaused | BitLockerStopped)

typedef enum _CLUSTER_CSV_VOLUME_FAULT_STATE {
    VolumeStateNoFaults       = 0x00000000,
    VolumeStateNoDirectIO     = 0x00000001,
    VolumeStateNoAccess       = 0x00000002,
    VolumeStateInMaintenance  = 0x00000004,
    VolumeStateDismounted     = 0x00000008
} CLUSTER_CSV_VOLUME_FAULT_STATE, *PCLUSTER_CSV_VOLUME_FAULT_STATE;

typedef enum _CLUSTER_SHARED_VOLUME_BACKUP_STATE {
    VolumeBackupNone        = 0x00000000,
    VolumeBackupInProgress  = 0x00000001
} CLUSTER_SHARED_VOLUME_BACKUP_STATE, *PCLUSTER_SHARED_VOLUME_BACKUP_STATE;

typedef struct _CLUS_CSV_VOLUME_INFO {
    ULARGE_INTEGER                 VolumeOffset;
    DWORD                          PartitionNumber;
    CLUSTER_CSV_VOLUME_FAULT_STATE FaultState;
    CLUSTER_SHARED_VOLUME_BACKUP_STATE BackupState;
    WCHAR                          szVolumeFriendlyName[MAX_PATH];
    WCHAR                          szVolumeName[50]; // CSV volume name
} CLUS_CSV_VOLUME_INFO, *PCLUS_CSV_VOLUME_INFO;

typedef struct _CLUS_CSV_VOLUME_NAME {
    LARGE_INTEGER                  VolumeOffset;
    WCHAR                          szVolumeName[MAX_PATH];
    WCHAR                          szRootPath[MAX_PATH+3];
} CLUS_CSV_VOLUME_NAME, *PCLUS_CSV_VOLUME_NAME;

typedef enum _CLUSTER_SHARED_VOLUME_STATE
{
    SharedVolumeStateUnavailable = 0,
    SharedVolumeStatePaused      = 1,
    SharedVolumeStateActive      = 2,
    SharedVolumeStateActiveRedirected = 3,
    SharedVolumeStateActiveVolumeRedirected = 4

} CLUSTER_SHARED_VOLUME_STATE, *PCLUSTER_SHARED_VOLUME_STATE;

typedef struct _CLUSTER_SHARED_VOLUME_STATE_INFO
{
    WCHAR szVolumeName[MAX_PATH];
    WCHAR szNodeName[MAX_PATH];
    CLUSTER_SHARED_VOLUME_STATE VolumeState;
} CLUSTER_SHARED_VOLUME_STATE_INFO, *PCLUSTER_SHARED_VOLUME_STATE_INFO;

// Bit mask values for CSV redirected IO reason - upto 64 reasons are supported.
#define RedirectedIOReasonUserRequest            0x0000000000000001
#define RedirectedIOReasonUnsafeFileSystemFilter 0x0000000000000002
#define RedirectedIOReasonUnsafeVolumeFilter     0x0000000000000004
#define RedirectedIOReasonFileSystemTiering      0x0000000000000008
#define RedirectedIOReasonBitLockerInitializing  0x0000000000000010
#define RedirectedIOReasonReFs                   0x0000000000000020
#define RedirectedIOReasonMax                    0x8000000000000000

#define VolumeRedirectedIOReasonNoDiskConnectivity      0x0000000000000001
#define VolumeRedirectedIOReasonStorageSpaceNotAttached 0x0000000000000002
#define VolumeRedirectedIOReasonVolumeReplicationEnabled 0x0000000000000004
#define VolumeRedirectedIOReasonMax                     0x8000000000000000

typedef struct _CLUSTER_SHARED_VOLUME_STATE_INFO_EX
{
    WCHAR szVolumeName[MAX_PATH];
    WCHAR szNodeName[MAX_PATH];
    CLUSTER_SHARED_VOLUME_STATE VolumeState;
    WCHAR szVolumeFriendlyName[MAX_PATH];
    ULONGLONG RedirectedIOReason;
    ULONGLONG VolumeRedirectedIOReason;
} CLUSTER_SHARED_VOLUME_STATE_INFO_EX, *PCLUSTER_SHARED_VOLUME_STATE_INFO_EX;


#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD)
typedef enum _CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE
{
    ClusterSharedVolumeRenameInputTypeNone,
    ClusterSharedVolumeRenameInputTypeVolumeOffset,
    ClusterSharedVolumeRenameInputTypeVolumeId,
    ClusterSharedVolumeRenameInputTypeVolumeName,
    ClusterSharedVolumeRenameInputTypeVolumeGuid
} CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE, *PCLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE;

typedef struct _CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME
{
    CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE InputType;
    union
    {
        ULONGLONG VolumeOffset;
        WCHAR VolumeId[MAX_PATH];
        WCHAR VolumeName[MAX_PATH];
        WCHAR VolumeGuid[50];
    };
} CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME, *PCLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME;

typedef struct _CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME
{
    WCHAR NewVolumeName[MAX_PATH];
} CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME, *PCLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME;

typedef struct _CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME
{
    WCHAR NewVolumeName[MAX_PATH];
    WCHAR NewVolumeGuid[50];
} CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME, *PCLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME;

#ifdef __cplusplus
typedef struct _CLUSTER_SHARED_VOLUME_RENAME_INPUT
    : public CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME
    , public CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
#else
typedef struct _CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME;
    CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME;
#endif
} CLUSTER_SHARED_VOLUME_RENAME_INPUT, *PCLUSTER_SHARED_VOLUME_RENAME_INPUT;

#ifdef __cplusplus
typedef struct _CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT
    : public CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME
    , public CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
#else
typedef struct _CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME;
    CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME;
#endif
} CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT, *PCLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT;

#endif // CLUSAPI_VERSION >= CLUSAPI_VERSION_WINTHRESHOLD

typedef struct _CLUS_CHKDSK_INFO {
    DWORD PartitionNumber;
    DWORD ChkdskState;
    DWORD FileIdCount;
    ULONGLONG FileIdList[1];   // variable length array
} CLUS_CHKDSK_INFO, *PCLUS_CHKDSK_INFO;

typedef struct _CLUS_DISK_NUMBER_INFO {
    DWORD DiskNumber;
    DWORD BytesPerSector;
} CLUS_DISK_NUMBER_INFO, *PCLUS_DISK_NUMBER_INFO;

typedef struct _CLUS_SHARED_VOLUME_BACKUP_MODE {
    CLUSTER_SHARED_VOLUME_BACKUP_STATE BackupState;
    DWORD DelayTimerInSecs;
    WCHAR VolumeName[MAX_PATH];
} CLUS_SHARED_VOLUME_BACKUP_MODE, *PCLUS_SHARED_VOLUME_BACKUP_MODE;

// Disk partition information property value.
#ifdef __cplusplus
typedef struct CLUSPROP_PARTITION_INFO
    : public CLUSPROP_VALUE
    , public CLUS_PARTITION_INFO {
#else
typedef struct CLUSPROP_PARTITION_INFO {
    CLUSPROP_VALUE;
    CLUS_PARTITION_INFO;
#endif
} CLUSPROP_PARTITION_INFO, *PCLUSPROP_PARTITION_INFO;

// Disk partition information ex property value.
#ifdef __cplusplus
typedef struct CLUSPROP_PARTITION_INFO_EX
    : public CLUSPROP_VALUE
    , public CLUS_PARTITION_INFO_EX {
#else
typedef struct CLUSPROP_PARTITION_INFO_EX {
    CLUSPROP_VALUE;
    CLUS_PARTITION_INFO_EX;
#endif
} CLUSPROP_PARTITION_INFO_EX;

typedef CLUSPROP_PARTITION_INFO_EX UNALIGNED *PCLUSPROP_PARTITION_INFO_EX;

// Disk partition information for CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2
#ifdef __cplusplus
typedef struct CLUSPROP_PARTITION_INFO_EX2
    : public CLUSPROP_PARTITION_INFO_EX
    , public CLUS_PARTITION_INFO_EX2 {
#else
typedef struct CLUSPROP_PARTITION_INFO_EX2 {
    CLUSPROP_PARTITION_INFO_EX;
    CLUS_PARTITION_INFO_EX2;
#endif
} CLUSPROP_PARTITION_INFO_EX2;

typedef CLUSPROP_PARTITION_INFO_EX2 UNALIGNED *PCLUSPROP_PARTITION_INFO_EX2;

//
// FT set information.
//
typedef struct CLUS_FTSET_INFO {
    DWORD           dwRootSignature;
    DWORD           dwFtType;
} CLUS_FTSET_INFO, *PCLUS_FTSET_INFO;

// Disk partition information property value.
#ifdef __cplusplus
typedef struct CLUSPROP_FTSET_INFO
    : public CLUSPROP_VALUE
    , public CLUS_FTSET_INFO {
#else
typedef struct CLUSPROP_FTSET_INFO {
    CLUSPROP_VALUE;
    CLUS_FTSET_INFO;
#endif
} CLUSPROP_FTSET_INFO, *PCLUSPROP_FTSET_INFO;

// Disk Signature property value.
typedef CLUSPROP_DWORD CLUSPROP_DISK_SIGNATURE, *PCLUSPROP_DISK_SIGNATURE;

// SCSI Address.
typedef struct CLUS_SCSI_ADDRESS {
    union {
        struct {
            UCHAR PortNumber;
            UCHAR PathId;
            UCHAR TargetId;
            UCHAR Lun;
        } DUMMYSTRUCTNAME;
        DWORD   dw;
    } DUMMYUNIONNAME;
} CLUS_SCSI_ADDRESS, *PCLUS_SCSI_ADDRESS;

// SCSI Address property value.
#ifdef __cplusplus
typedef struct CLUSPROP_SCSI_ADDRESS
    : public CLUSPROP_VALUE
    , public CLUS_SCSI_ADDRESS {
#else
typedef struct CLUSPROP_SCSI_ADDRESS {
    CLUSPROP_VALUE;
    CLUS_SCSI_ADDRESS;
#endif
} CLUSPROP_SCSI_ADDRESS, *PCLUSPROP_SCSI_ADDRESS;

//
// input structure for CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN
//
typedef struct CLUS_NETNAME_VS_TOKEN_INFO {
    DWORD ProcessID;
    DWORD DesiredAccess;
    BOOL  InheritHandle;
} CLUS_NETNAME_VS_TOKEN_INFO, CLUS_VS_TOKEN_INFO, *PCLUS_NETNAME_VS_TOKEN_INFO, *PCLUS_VS_TOKEN_INFO;

//
// input structure for CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO and for CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO
//
#define MAX_OBJECTID 64
#define MAX_CO_PASSWORD_LENGTH 16
#define GUID_PRESENT 0x1
#define CREATEDC_PRESENT 0x2

// (PWLEN/sizeof(wchar_t))-1 is the max allowed password size, remember to add 1 to buffers to hold the null
#define MAX_CO_PASSWORD_LENGTHEX 127

// including the null
#define MAX_CO_PASSWORD_STORAGEEX 128

#define MAX_CREATINGDC_LENGTH 256

typedef struct CLUS_NETNAME_PWD_INFO {
    DWORD Flags;
    WCHAR Password[MAX_CO_PASSWORD_LENGTH];
    WCHAR CreatingDC[MAX_CREATINGDC_LENGTH+2];  // including the '\\' prefix
    WCHAR ObjectGuid[MAX_OBJECTID];                          //
} CLUS_NETNAME_PWD_INFO, *PCLUS_NETNAME_PWD_INFO, CLUS_RLUA_PWD_INFO, *PCLUS_RLUA_PWD_INFO;

//
// input structure for CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX and for CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX
//
typedef struct CLUS_NETNAME_PWD_INFOEX {
    DWORD Flags;
    WCHAR Password[MAX_CO_PASSWORD_STORAGEEX];
    WCHAR CreatingDC[MAX_CREATINGDC_LENGTH+2];  // including the '\\' prefix
    WCHAR ObjectGuid[MAX_OBJECTID];                          //
} CLUS_NETNAME_PWD_INFOEX, *PCLUS_NETNAME_PWD_INFOEX, CLUS_RLUA_PWD_INFOEX, *PCLUS_RLUA_PWD_INFOEX;


//
// input structure for CLUSCTL_RESOURCE_DNN_SEND_LEADER_STATUS and for CLUSCTL_RESOURCE_DNN_REFRESH_CLONES
//
typedef struct CLUS_DNN_LEADER_STATUS {
    BOOL IsOnline; // Indicates if the leader is online or is shutting down
    BOOL IsFileServerPresent; // Indicates if a file server is depending on the netname
} CLUS_DNN_LEADER_STATUS , *PCLUS_DNN_LEADER_STATUS;

//
// input structure for CLUSCTL_RESOURCE_DNN_UPDATE_SODAFS_CLONE_STATUS
//
typedef struct CLUS_DNN_SODAFS_CLONE_STATUS {
    DWORD NodeId; // Indicates the cluster assigned node id of the SODAFS clone
    CLUSTER_RESOURCE_STATE Status; // Indicates the status of the clone (Online/Failed)
} CLUS_DNN_SODAFS_CLONE_STATUS , *PCLUS_DNN_SODAFS_CLONE_STATUS;

//
// A single IP info entry consisting of the node and the IP address
//
typedef struct CLUS_NETNAME_IP_INFO_ENTRY {
    DWORD NodeId; // Node to which this IP belongs to
    DWORD AddressSize;
    BYTE Address[ANYSIZE_ARRAY]; // The actual IP info
} CLUS_NETNAME_IP_INFO_ENTRY, *PCLUS_NETNAME_IP_INFO_ENTRY;

//
// input structure for CLUSCTL_RESOURCE_NETNAME_SEND_IP_INFO_FOR_MULTICHANNEL
//
#define DNS_LENGTH 64
typedef struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    WCHAR  szName[DNS_LENGTH]; // The actual name
    DWORD NumEntries; // Number of CLUS_NETNAME_IP_INFO_ENTRY
    CLUS_NETNAME_IP_INFO_ENTRY IpInfo[ANYSIZE_ARRAY]; // The actual IP Info as an array
} CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL, *PCLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL;

//
// input/output structure associated with Maintenance Mode
//
typedef struct CLUS_MAINTENANCE_MODE_INFO {
    BOOL  InMaintenance;
} CLUS_MAINTENANCE_MODE_INFO, *PCLUS_MAINTENANCE_MODE_INFO;

typedef struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    BOOL  InMaintenance;
    WCHAR VolumeName[MAX_PATH];
} CLUS_CSV_MAINTENANCE_MODE_INFO, *PCLUS_CSV_MAINTENANCE_MODE_INFO;

#define MAINTENANCE_MODE_V2_SIG 0xabbaf00f

typedef enum _MAINTENANCE_MODE_TYPE_ENUM {
    MaintenanceModeTypeDisableIsAliveCheck=1,
    MaintenanceModeTypeOfflineResource=2,
    MaintenanceModeTypeUnclusterResource=3,
} MAINTENANCE_MODE_TYPE_ENUM, *PMAINTENANCE_MODE_TYPE_ENUM;

typedef struct _CLUS_MAINTENANCE_MODE_INFOEX {
    BOOL                        InMaintenance;
    MAINTENANCE_MODE_TYPE_ENUM  MaintainenceModeType;
    CLUSTER_RESOURCE_STATE      InternalState;
    DWORD                       Signature;
} CLUS_MAINTENANCE_MODE_INFOEX, *PCLUS_MAINTENANCE_MODE_INFOEX;

typedef struct _CLUS_SET_MAINTENANCE_MODE_INPUT {
    BOOL  InMaintenance;
    DWORD ExtraParameterSize;
    BYTE  ExtraParameter[1];
} CLUS_SET_MAINTENANCE_MODE_INPUT, *PCLUS_SET_MAINTENANCE_MODE_INPUT;

typedef struct _CLUS_STORAGE_SET_DRIVELETTER {
    DWORD    PartitionNumber;
    DWORD    DriveLetterMask;
} CLUS_STORAGE_SET_DRIVELETTER, *PCLUS_STORAGE_SET_DRIVELETTER;

typedef struct _CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    DWORD    AvailDrivelettersMask;
} CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS, *PCLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS;

typedef struct _CLUS_STORAGE_REMAP_DRIVELETTER {
    DWORD    CurrentDriveLetterMask;
    DWORD    TargetDriveLetterMask;
} CLUS_STORAGE_REMAP_DRIVELETTER, *PCLUS_STORAGE_REMAP_DRIVELETTER;

typedef struct _CLUS_PROVIDER_STATE_CHANGE_INFO {
    DWORD                   dwSize;             // size of this struct including the provider name.
    CLUSTER_RESOURCE_STATE  resourceState;
    WCHAR                   szProviderId[1];
} CLUS_PROVIDER_STATE_CHANGE_INFO, *PCLUS_PROVIDER_STATE_CHANGE_INFO;

// Cluster set create fileserver control input.
typedef struct _CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    WCHAR FileServerName[16];
} CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT, *PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT;

// Cluster set create fileserver control output.
typedef struct _CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    WCHAR FileServerName[MAX_PATH];
} CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT, *PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT;

// Beginning of a property list.
typedef struct CLUSPROP_LIST {
    DWORD                   nPropertyCount;
    CLUSPROP_PROPERTY_NAME  PropertyName;
} CLUSPROP_LIST, *PCLUSPROP_LIST;

//
// values for IP Address' EnableNetbios property
//
typedef enum CLUSPROP_IPADDR_ENABLENETBIOS {
    CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED = 0,
    CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED,
    CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC
} CLUSPROP_IPADDR_ENABLENETBIOS;

//
// List of change notifications for File Server resource.
//
typedef enum _FILESHARE_CHANGE_ENUM {
    FILESHARE_CHANGE_NONE,
    FILESHARE_CHANGE_ADD,
    FILESHARE_CHANGE_DEL,
    FILESHARE_CHANGE_MODIFY
} FILESHARE_CHANGE_ENUM, *PFILESHARE_CHANGE_ENUM;

// Copied from Lmcons.h
#define NNLEN       80                  // Net name length (share name)
typedef struct _FILESHARE_CHANGE {
    FILESHARE_CHANGE_ENUM   Change;
    WCHAR                   ShareName[NNLEN+4];
} FILESHARE_CHANGE, *PFILESHARE_CHANGE;

#pragma warning(push)
#pragma warning(disable: 4200)
typedef struct _FILESHARE_CHANGE_LIST {
    DWORD               NumEntries;
    FILESHARE_CHANGE    ChangeEntry[0];
} FILESHARE_CHANGE_LIST, *PFILESHARE_CHANGE_LIST;
#pragma warning(pop)

typedef struct _CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT
{
    ULONGLONG   GetTickCount64;
    SYSTEMTIME  GetSystemTime;
    DWORD       NodeId;
} CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT, *PCLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT;


// Helper for building or parsing a property list buffer.
typedef union CLUSPROP_BUFFER_HELPER {
    BYTE *                          pb;
    WORD *                          pw;
    DWORD *                         pdw;
    LONG *                          pl;
    LPWSTR                          psz;
    PCLUSPROP_LIST                  pList;
    PCLUSPROP_SYNTAX                pSyntax;
    PCLUSPROP_PROPERTY_NAME         pName;
    PCLUSPROP_VALUE                 pValue;
    PCLUSPROP_BINARY                pBinaryValue;
    PCLUSPROP_WORD                  pWordValue;
    PCLUSPROP_DWORD                 pDwordValue;
    PCLUSPROP_LONG                  pLongValue;
    PCLUSPROP_ULARGE_INTEGER        pULargeIntegerValue;
    PCLUSPROP_LARGE_INTEGER         pLargeIntegerValue;
    PCLUSPROP_SZ                    pStringValue;
    PCLUSPROP_MULTI_SZ              pMultiSzValue;
    PCLUSPROP_SECURITY_DESCRIPTOR   pSecurityDescriptor;
    PCLUSPROP_RESOURCE_CLASS        pResourceClassValue;
    PCLUSPROP_RESOURCE_CLASS_INFO   pResourceClassInfoValue;
    PCLUSPROP_DISK_SIGNATURE        pDiskSignatureValue;
    PCLUSPROP_SCSI_ADDRESS          pScsiAddressValue;
    PCLUSPROP_DISK_NUMBER           pDiskNumberValue;
    PCLUSPROP_PARTITION_INFO        pPartitionInfoValue;
    PCLUSPROP_REQUIRED_DEPENDENCY   pRequiredDependencyValue;
    PCLUSPROP_PARTITION_INFO_EX     pPartitionInfoValueEx;
    PCLUSPROP_PARTITION_INFO_EX2    pPartitionInfoValueEx2;
    PCLUSPROP_FILETIME              pFileTimeValue;
} CLUSPROP_BUFFER_HELPER, *PCLUSPROP_BUFFER_HELPER;

#endif // MIDL_PASS

#endif // _CLUSTER_API_TYPES_

// Macro for aligning CLUSPROP buffers on a DWORD boundary.
#define ALIGN_CLUSPROP( count ) ((count + 3) & ~3)

// Macros for declaring array format values
#define CLUSPROP_BINARY_DECLARE( name, cb ) \
    struct {                                \
        CLUSPROP_SYNTAX Syntax;             \
        DWORD           cbLength;           \
        BYTE            rgb[(cb + 3) & ~3]; \
    } name

#define CLUSPROP_SZ_DECLARE( name, cch )    \
    struct {                                \
        CLUSPROP_SYNTAX Syntax;             \
        DWORD           cbLength;           \
        WCHAR           sz[(cch + 1) & ~1]; \
    } name

#define CLUSPROP_PROPERTY_NAME_DECLARE( name, cch ) CLUSPROP_SZ_DECLARE( name, cch )



//
// Cluster resource property enumeration.
//

#ifndef _CLUSTER_API_TYPES_
//
// Define enumerable types
//
typedef enum CLUSTER_RESOURCE_ENUM {
    CLUSTER_RESOURCE_ENUM_DEPENDS   = 0x00000001,
    CLUSTER_RESOURCE_ENUM_PROVIDES  = 0x00000002,
    CLUSTER_RESOURCE_ENUM_NODES     = 0x00000004,

    CLUSTER_RESOURCE_ENUM_ALL       = (CLUSTER_RESOURCE_ENUM_DEPENDS  |
                                       CLUSTER_RESOURCE_ENUM_PROVIDES |
                                       CLUSTER_RESOURCE_ENUM_NODES)
} CLUSTER_RESOURCE_ENUM;

typedef enum CLUSTER_RESOURCE_TYPE_ENUM {
    CLUSTER_RESOURCE_TYPE_ENUM_NODES     = 0x00000001,
    CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES = 0x00000002,

    CLUSTER_RESOURCE_TYPE_ENUM_ALL       = (CLUSTER_RESOURCE_TYPE_ENUM_NODES |
                                            CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES)
} CLUSTER_RESOURCE_TYPE_ENUM;

#endif // _CLUSTER_API_TYPES_

#if ( !MIDL_PASS && !__midl )
HRESENUM
WINAPI
ClusterResourceOpenEnum(
    _In_ HRESOURCE hResource,
    _In_ DWORD dwType
    );

typedef HRESENUM
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM)(
    HRESOURCE hResource,
    DWORD dwType
    );

DWORD
WINAPI
ClusterResourceGetEnumCount(
    _In_ HRESENUM hResEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT)(
    _In_ HRESENUM hResEnum
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceEnum(
    _In_ HRESENUM hResEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_ENUM)(
    _In_ HRESENUM hResEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterResourceCloseEnum(
    _In_ HRESENUM hResEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM)(
    HRESENUM hResEnum
    );

DWORD
WINAPI
CreateClusterResourceType(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_ LPCWSTR lpszDisplayName,
    _In_ LPCWSTR lpszResourceTypeDll,
    _In_ DWORD dwLooksAlivePollInterval,
    _In_ DWORD dwIsAlivePollInterval
    );

typedef DWORD
(WINAPI * PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_ LPCWSTR lpszDisplayName,
    _In_ LPCWSTR lpszResourceTypeDll,
    _In_ DWORD dwLooksAlivePollInterval,
    _In_ DWORD dwIsAlivePollInterval
    );

DWORD
WINAPI
DeleteClusterResourceType(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE)(
    HCLUSTER hCluster,
    LPCWSTR lpszResourceTypeName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
CreateClusterResourceTypeEx(
    _In_     HCLUSTER hCluster,
    _In_     LPCWSTR lpszResourceTypeName,
    _In_     LPCWSTR lpszDisplayName,
    _In_     LPCWSTR lpszResourceTypeDll,
    _In_     DWORD dwLooksAlivePollInterval,
    _In_     DWORD dwIsAlivePollInterval,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE_EX)(
    _In_     HCLUSTER hCluster,
    _In_     LPCWSTR lpszResourceTypeName,
    _In_     LPCWSTR lpszDisplayName,
    _In_     LPCWSTR lpszResourceTypeDll,
    _In_     DWORD dwLooksAlivePollInterval,
    _In_     DWORD dwIsAlivePollInterval,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
DeleteClusterResourceTypeEx(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszTypeName,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE_EX)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszTypeName,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

HRESTYPEENUM
WINAPI
ClusterResourceTypeOpenEnum(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_ DWORD dwType
    );

typedef HRESTYPEENUM
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszResourceTypeName,
    _In_ DWORD dwType
    );

DWORD
WINAPI
ClusterResourceTypeGetEnumCount(
    _In_ HRESTYPEENUM hResTypeEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT)(
    _In_ HRESTYPEENUM hResTypeEnum
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterResourceTypeEnum(
    _In_ HRESTYPEENUM hResTypeEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM)(
    _In_ HRESTYPEENUM hResTypeEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterResourceTypeCloseEnum(
    _In_ HRESTYPEENUM hResTypeEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM)(
    _In_ HRESTYPEENUM hResTypeEnum
    );

#endif // MIDL_PASS



//
// Network-related structures and types.
//

#ifndef _CLUSTER_API_TYPES_
//
// Define enumerable group types
//
typedef enum CLUSTER_NETWORK_ENUM {
    CLUSTER_NETWORK_ENUM_NETINTERFACES  = 0x00000001,

    CLUSTER_NETWORK_ENUM_ALL            = CLUSTER_NETWORK_ENUM_NETINTERFACES
} CLUSTER_NETWORK_ENUM;

typedef enum CLUSTER_NETWORK_STATE {
    ClusterNetworkStateUnknown = -1,
    ClusterNetworkUnavailable,
    ClusterNetworkDown,
    ClusterNetworkPartitioned,
    ClusterNetworkUp
} CLUSTER_NETWORK_STATE;

// Role the network plays in the cluster.  This is a bitmask.
typedef enum CLUSTER_NETWORK_ROLE {
    ClusterNetworkRoleNone              = 0,
    ClusterNetworkRoleInternalUse       = 0x00000001,
    ClusterNetworkRoleClientAccess      = 0x00000002,
    ClusterNetworkRoleInternalAndClient = 0x00000003
} CLUSTER_NETWORK_ROLE;

#endif // _CLUSTER_API_TYPES_

//
// Interfaces for managing the networks of a cluster.
//

#if ( !MIDL_PASS && !__midl )
HNETWORK
WINAPI
OpenClusterNetwork(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNetworkName
    );

typedef HNETWORK
(WINAPI * PCLUSAPI_OPEN_CLUSTER_NETWORK)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNetworkName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
HNETWORK
WINAPI
OpenClusterNetworkEx(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszNetworkName,
    _In_      DWORD dwDesiredAccess,
    _Out_opt_ DWORD* lpdwGrantedAccess
    );

typedef HNETWORK
(WINAPI * PCLUSAPI_OPEN_CLUSTER_NETWORK_EX)(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszNetworkName,
    _In_      DWORD   dwDesiredAccess,
    _Out_opt_ LPDWORD lpdwGrantedAccess
    );
#endif

BOOL
WINAPI
CloseClusterNetwork(
    _In_ HNETWORK hNetwork
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_NETWORK)(
    _In_ HNETWORK hNetwork
    );

HCLUSTER
WINAPI
GetClusterFromNetwork(
    _In_ HNETWORK hNetwork
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_GET_CLUSTER_FROM_NETWORK)(
    _In_ HNETWORK hNetwork
    );

HNETWORKENUM
WINAPI
ClusterNetworkOpenEnum(
    _In_ HNETWORK hNetwork,
    _In_ DWORD dwType
    );

typedef HNETWORKENUM
(WINAPI * PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM)(
    _In_ HNETWORK hNetwork,
    _In_ DWORD dwType
    );

DWORD
WINAPI
ClusterNetworkGetEnumCount(
    _In_ HNETWORKENUM hNetworkEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT)(
    _In_ HNETWORKENUM hNetworkEnum
    );

_Success_ (return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterNetworkEnum(
    _In_ HNETWORKENUM hNetworkEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NETWORK_ENUM)(
    _In_ HNETWORKENUM hNetworkEnum,
    _In_ DWORD dwIndex,
    _Out_ LPDWORD lpdwType,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterNetworkCloseEnum(
    _In_ HNETWORKENUM hNetworkEnum
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM)(
    _In_ HNETWORKENUM hNetworkEnum
    );

CLUSTER_NETWORK_STATE
WINAPI
GetClusterNetworkState(
    _In_ HNETWORK hNetwork
    );

typedef CLUSTER_NETWORK_STATE
(WINAPI * PCLUSAPI_GET_CLUSTER_NETWORK_STATE)(
    _In_ HNETWORK hNetwork
    );

DWORD
WINAPI
SetClusterNetworkName(
    _In_ HNETWORK hNetwork,
    _In_ LPCWSTR lpszName
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_NETWORK_NAME)(
    _In_ HNETWORK hNetwork,
    _In_ LPCWSTR lpszName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
SetClusterNetworkNameEx(
    _In_ HNETWORK hNetwork,
    _In_ LPCWSTR lpszName,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_SET_CLUSTER_NETWORK_NAME_EX)(
    _In_ HNETWORK hNetwork,
    _In_ LPCWSTR lpszName,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterNetworkId(
    _In_ HNETWORK hNetwork,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszNetworkId,
    _Inout_ LPDWORD lpcchName
    );

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_NETWORK_ID)(
    _In_ HNETWORK hNetwork,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszNetworkId,
    _Inout_ LPDWORD lpcchName
    );

DWORD
WINAPI
ClusterNetworkControl(
    _In_ HNETWORK hNetwork,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NETWORK_CONTROL)(
    _In_ HNETWORK hNetwork,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ClusterNetworkControlEx(
    _In_ HNETWORK hNetwork,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NETWORK_CONTROL_EX)(
    _In_ HNETWORK hNetwork,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

#endif // MIDL_PASS


#ifndef _CLUSTER_API_TYPES_
//
// Network interface-related structures and types.
//
typedef enum CLUSTER_NETINTERFACE_STATE {
    ClusterNetInterfaceStateUnknown = -1,
    ClusterNetInterfaceUnavailable,
    ClusterNetInterfaceFailed,
    ClusterNetInterfaceUnreachable,
    ClusterNetInterfaceUp
} CLUSTER_NETINTERFACE_STATE;

#endif // _CLUSTER_API_TYPES_

//
// Interfaces for managing the network interfaces of a cluster.
//

#if ( !MIDL_PASS && !__midl )
HNETINTERFACE
WINAPI
OpenClusterNetInterface(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszInterfaceName
    );

typedef HNETINTERFACE
(WINAPI * PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszInterfaceName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_SERVER2008R2)
HNETINTERFACE
WINAPI
OpenClusterNetInterfaceEx(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszInterfaceName,
    _In_      DWORD dwDesiredAccess,
    _Out_opt_ DWORD* lpdwGrantedAccess
    );

typedef HNETINTERFACE
(WINAPI * PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX)(
    _In_      HCLUSTER hCluster,
    _In_opt_  LPCWSTR lpszNetInterfaceName,
    _In_      DWORD   dwDesiredAccess,
    _Out_opt_ LPDWORD lpdwGrantedAccess
    );
#endif

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterNetInterface(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNodeName,
    _In_ LPCWSTR lpszNetworkName,
    _Out_writes_to_(*lpcchInterfaceName, *lpcchInterfaceName + 1) LPWSTR lpszInterfaceName,
    _Inout_ LPDWORD lpcchInterfaceName
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetClusterNetInterfaceEx(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNodeName,
    _In_ LPCWSTR lpszNetworkName,
    _Out_writes_to_(*lpcbInterfaceListBufSize, *lpcbInterfaceListBufSize) LPWSTR lpmszInterfaceNameList,
    _Inout_ LPDWORD lpcbInterfaceListBufSize
);

typedef DWORD
(WINAPI * PCLUSAPI_GET_CLUSTER_NET_INTERFACE)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR lpszNodeName,
    _In_ LPCWSTR lpszNetworkName,
    _Out_writes_to_opt_(*lpcchInterfaceName, *lpcchInterfaceName + 1) LPWSTR lpszInterfaceName,
    _Inout_ LPDWORD lpcchInterfaceName
    );

BOOL
WINAPI
CloseClusterNetInterface(
    _In_ HNETINTERFACE hNetInterface
    );

typedef BOOL
(WINAPI * PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE)(
    _In_ HNETINTERFACE hNetInterface
    );

HCLUSTER
WINAPI
GetClusterFromNetInterface(
    _In_ HNETINTERFACE hNetInterface
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE)(
    _In_ HNETINTERFACE hNetInterface
    );

CLUSTER_NETINTERFACE_STATE
WINAPI
GetClusterNetInterfaceState(
    _In_ HNETINTERFACE hNetInterface
    );

typedef CLUSTER_NETINTERFACE_STATE
(WINAPI * PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE)(
    _In_ HNETINTERFACE hNetInterface
    );

DWORD
WINAPI
ClusterNetInterfaceControl(
    _In_ HNETINTERFACE hNetInterface,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL)(
    _In_ HNETINTERFACE hNetInterface,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ClusterNetInterfaceControlEx(
    _In_ HNETINTERFACE hNetInterface,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL_EX)(
    _In_ HNETINTERFACE hNetInterface,
    _In_opt_ HNODE hHostNode,
    _In_ DWORD dwControlCode,
    _In_reads_bytes_opt_(nInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD nInBufferSize,
    _Out_writes_bytes_to_opt_(nOutBufferSize, *lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD nOutBufferSize,
    _Out_opt_ LPDWORD lpBytesReturned,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

#endif // MIDL_PASS


//
// Cluster registry update and access routines
//

#if ( !MIDL_PASS && !__midl )
HKEY
WINAPI
GetClusterKey(
    _In_ HCLUSTER hCluster,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_KEY)(
     HCLUSTER hCluster,
     REGSAM samDesired
     );

HKEY
WINAPI
GetClusterGroupKey(
    _In_ HGROUP hGroup,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_GROUP_KEY)(
    HGROUP hGroup,
    REGSAM samDesired
    );

HKEY
WINAPI
GetClusterResourceKey(
    _In_ HRESOURCE hResource,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_RESOURCE_KEY)(
    HRESOURCE hResource,
    REGSAM samDesired
    );

HKEY
WINAPI
GetClusterNodeKey(
    _In_ HNODE hNode,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_NODE_KEY)(
    HNODE hNode,
    REGSAM samDesired
    );

HKEY
WINAPI
GetClusterNetworkKey(
    _In_ HNETWORK hNetwork,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_NETWORK_KEY)(
    _In_ HNETWORK hNetwork,
    _In_ REGSAM samDesired
    );

HKEY
WINAPI
GetClusterNetInterfaceKey(
    _In_ HNETINTERFACE hNetInterface,
    _In_ REGSAM samDesired
    );

typedef HKEY
(WINAPI * PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY)(
    _In_ HNETINTERFACE hNetInterface,
    _In_ REGSAM samDesired
    );

LONG
WINAPI
ClusterRegCreateKey(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszSubKey,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_CREATE_KEY)(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszSubKey,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

LONG
WINAPI
ClusterRegCreateKeyEx(
    __in HKEY hKey,
    __in LPCWSTR lpSubKey,
    __in DWORD dwOptions,
    __in REGSAM samDesired,
    __in_opt LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    __out PHKEY phkResult,
    __out_opt LPDWORD lpdwDisposition,
    _In_opt_ LPCWSTR lpszReason
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_CREATE_KEY_EX)(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszSubKey,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

LONG
WINAPI
ClusterRegOpenKey(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszSubKey,
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_OPEN_KEY)(
    HKEY hKey,
    LPCWSTR lpszSubKey,
    REGSAM samDesired,
    PHKEY phkResult
    );

LONG
WINAPI
ClusterRegDeleteKey(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszSubKey
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_DELETE_KEY)(
    HKEY hKey,
    LPCWSTR lpszSubKey
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

LONG
WINAPI
ClusterRegDeleteKeyEx(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey,
    _In_opt_ LPCWSTR lpszReason
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_DELETE_KEY_EX)(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

LONG
WINAPI
ClusterRegCloseKey(
    _In_ HKEY hKey
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_CLOSE_KEY)(
    HKEY hKey
    );

_Success_(return == ERROR_SUCCESS)
LONG
WINAPI
ClusterRegEnumKey(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName,
    _Out_opt_ PFILETIME lpftLastWriteTime
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_ENUM_KEY)(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_(*lpcchName, *lpcchName + 1) LPWSTR lpszName,
    _Inout_ LPDWORD lpcchName,
    _Out_ PFILETIME lpftLastWriteTime
    );

DWORD
WINAPI
ClusterRegSetValue(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _In_ DWORD dwType,
    _In_ CONST BYTE* lpData,
    _In_ DWORD cbData
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REG_SET_VALUE)(
    HKEY hKey,
    LPCWSTR lpszValueName,
    DWORD dwType,
    CONST BYTE* lpData,
    DWORD cbData
    );

DWORD
WINAPI
ClusterRegDeleteValue(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REG_DELETE_VALUE)(
    HKEY hKey,
    LPCWSTR lpszValueName
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

DWORD
WINAPI
ClusterRegSetValueEx(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _In_ DWORD dwType,
    _In_ CONST BYTE* lpData,
    _In_ DWORD cbData,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REG_SET_VALUE_EX)(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _In_ DWORD dwType,
    _In_ CONST BYTE* lpData,
    _In_ DWORD cbData,
    _In_opt_ LPCWSTR lpszReason
    );

DWORD
WINAPI
ClusterRegDeleteValueEx(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _In_opt_ LPCWSTR lpszReason
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REG_DELETE_VALUE_EX)(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

LONG
WINAPI
ClusterRegQueryValue(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _Out_opt_ LPDWORD lpdwValueType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) LPBYTE lpData,
    _Inout_opt_ LPDWORD lpcbData
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_QUERY_VALUE)(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpszValueName,
    _Out_opt_ LPDWORD lpdwValueType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) LPBYTE lpData,
    _Inout_opt_ LPDWORD lpcbData
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
ClusterRegEnumValue(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_(*lpcchValueName, *lpcchValueName + 1) LPWSTR lpszValueName,
    _Inout_ LPDWORD lpcchValueName,
    _Out_opt_ LPDWORD lpdwType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) LPBYTE lpData,
    _Inout_opt_ LPDWORD lpcbData
    );

typedef DWORD
(WINAPI * PCLUSAPI_CLUSTER_REG_ENUM_VALUE)(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_(*lpcchValueName, *lpcchValueName + 1) LPWSTR lpszValueName,
    _Inout_ LPDWORD lpcchValueName,
    _Out_ LPDWORD lpdwType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) LPBYTE lpData,
    _Inout_opt_ LPDWORD lpcbData
    );

LONG
WINAPI
ClusterRegQueryInfoKey(
    _In_ HKEY hKey,
    _In_ LPDWORD lpcSubKeys,
    _In_ LPDWORD lpcchMaxSubKeyLen,
    _In_ LPDWORD lpcValues,
    _In_ LPDWORD lpcchMaxValueNameLen,
    _In_ LPDWORD lpcbMaxValueLen,
    _In_ LPDWORD lpcbSecurityDescriptor,
    _In_ PFILETIME lpftLastWriteTime
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY)(
    HKEY hKey,
    LPDWORD lpcSubKeys,
    LPDWORD lpcbMaxSubKeyLen,
    LPDWORD lpcValues,
    LPDWORD lpcbMaxValueNameLen,
    LPDWORD lpcbMaxValueLen,
    LPDWORD lpcbSecurityDescriptor,
    PFILETIME lpftLastWriteTime
    );

LONG
WINAPI
ClusterRegGetKeySecurity (
    _In_ HKEY hKey,
    _In_ SECURITY_INFORMATION RequestedInformation,
    _Out_writes_bytes_to_(*lpcbSecurityDescriptor, *lpcbSecurityDescriptor) PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Inout_ LPDWORD lpcbSecurityDescriptor
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY)(
    _In_ HKEY hKey,
    _In_ SECURITY_INFORMATION RequestedInformation,
    _Out_writes_bytes_to_(*lpcbSecurityDescriptor, *lpcbSecurityDescriptor) PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Inout_ LPDWORD lpcbSecurityDescriptor
    );

LONG
WINAPI
ClusterRegSetKeySecurity(
    _In_ HKEY hKey,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY)(
    HKEY hKey,
    SECURITY_INFORMATION SecurityInformation,
    PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

#if (CLUSAPI_VERSION >= CLUSAPI_VERSION_NI)

LONG
WINAPI
ClusterRegSetKeySecurityEx(
    _In_ HKEY hKey,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_opt_ LPCWSTR lpszReason
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY_EX)(
    HKEY hKey,
    SECURITY_INFORMATION SecurityInformation,
    PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _In_opt_ LPCWSTR lpszReason
    );

#endif

LONG
WINAPI
ClusterRegSyncDatabase(
    _In_ HCLUSTER hCluster,
    _In_ DWORD flags
    );

typedef LONG
(WINAPI * PCLUSAPI_CLUSTER_REG_SYNC_DATABASE)(
    HCLUSTER hCluster,
    DWORD flags
    );

LONG WINAPI ClusterRegCreateBatch(
    _In_opt_ HKEY hKey,
    _Out_ HREGBATCH* pHREGBATCH
    );

typedef LONG
(WINAPI *PCLUSAPI_CLUSTER_REG_CREATE_BATCH)(
    _In_opt_ HKEY hKey,
    _Out_ HREGBATCH* pHREGBATCH
    );

LONG WINAPI ClusterRegBatchAddCommand(
    _In_ HREGBATCH hRegBatch,
    _In_ CLUSTER_REG_COMMAND dwCommand,
    _In_opt_ LPCWSTR wzName,
    _In_ DWORD dwOptions,
    _In_reads_bytes_opt_(cbData) VOID CONST * lpData,
    _In_ DWORD cbData
    );

typedef LONG
(WINAPI *PCLUSTER_REG_BATCH_ADD_COMMAND)(
    _In_ HREGBATCH hRegBatch,
    _In_ CLUSTER_REG_COMMAND dwCommand,
    _In_opt_ LPWSTR wzName,
    _In_ DWORD dwOptions,
    _In_reads_bytes_opt_(cbData) VOID CONST * lpData,
    _In_ DWORD cbData
    );

LONG WINAPI ClusterRegCloseBatch(
    _In_ HREGBATCH hRegBatch,
    _In_ BOOL bCommit,
    _Out_opt_ INT * failedCommandNumber
    );

LONG WINAPI ClusterRegCloseBatchEx(
    _In_ HREGBATCH hRegBatch,
    _In_ DWORD flags,
    _Out_opt_ INT * failedCommandNumber
    );

typedef LONG
(WINAPI *PCLUSTER_REG_CLOSE_BATCH)(
    _In_ HREGBATCH hRegBatch,
    _In_ BOOL bCommit,
    _Out_opt_ INT * failedCommandNumber
    );

LONG WINAPI ClusterRegBatchReadCommand(
    _In_ HREGBATCHNOTIFICATION hBatchNotification,
    _Out_ CLUSTER_BATCH_COMMAND * pBatchCommand);

typedef LONG
(WINAPI *PCLUSTER_REG_BATCH_READ_COMMAND)(
    _In_ HREGBATCHNOTIFICATION hBatchNotification,
    _Out_ CLUSTER_BATCH_COMMAND * pBatchCommand);

LONG WINAPI ClusterRegBatchCloseNotification(
    _In_ HREGBATCHNOTIFICATION hBatchNotification);

typedef LONG
(WINAPI *PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION)(
    _In_ HREGBATCHNOTIFICATION hBatchNotification);

LONG WINAPI ClusterRegCreateBatchNotifyPort(
    _In_ HKEY hKey,
    _Out_ HREGBATCHPORT * phBatchNotifyPort);

typedef LONG
(WINAPI *PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT)(
    _In_ HKEY hKey,
    _Out_ HREGBATCHPORT * phBatchNotifyPort);

LONG WINAPI ClusterRegCloseBatchNotifyPort(
    _In_ HREGBATCHPORT hBatchNotifyPort);

typedef LONG
(WINAPI *PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT)(
    _In_ HREGBATCHPORT hBatchNotifyPort);

LONG WINAPI ClusterRegGetBatchNotification(
    _In_ HREGBATCHPORT hBatchNotify,
    _Out_ HREGBATCHNOTIFICATION * phBatchNotification);

typedef LONG
(WINAPI *PCLUSTER_REG_GET_BATCH_NOTIFICATION)(
    _In_ HREGBATCHPORT hBatchNotify,
    _Out_ HREGBATCHNOTIFICATION * phBatchNotification);


LONG WINAPI ClusterRegCreateReadBatch(
    _In_ HKEY hKey,
    _Out_ HREGREADBATCH* phRegReadBatch);

typedef LONG
(WINAPI *PCLUSTER_REG_CREATE_READ_BATCH)(
    _In_ HKEY hKey,
    _Out_ HREGREADBATCH* phRegReadBatch);

LONG WINAPI ClusterRegReadBatchAddCommand(
    _In_ HREGREADBATCH hRegReadBatch,
    _In_ LPCWSTR wzSubkeyName,
    _In_ LPCWSTR wzValueName);

typedef LONG
(WINAPI *PCLUSTER_REG_READ_BATCH_ADD_COMMAND)(
    _In_ HREGREADBATCH hRegReadBatch,
    _In_ LPCWSTR wzSubkeyName,
    _In_ LPCWSTR wzValueName);

LONG WINAPI ClusterRegCloseReadBatch(
    _In_ HREGREADBATCH hRegReadBatch,
    _Out_ HREGREADBATCHREPLY * phRegReadBatchReply);

typedef LONG
(WINAPI *PCLUSTER_REG_CLOSE_READ_BATCH)(
    _In_ HREGREADBATCH hRegReadBatch,
    _Out_ HREGREADBATCHREPLY * phRegReadBatchReply);

LONG WINAPI ClusterRegCloseReadBatchEx(
    _In_ HREGREADBATCH hRegReadBatch,
        _In_ DWORD flags,
    _Out_ HREGREADBATCHREPLY * phRegReadBatchReply);

typedef LONG
(WINAPI *PCLUSTER_REG_CLOSE_READ_BATCH_EX)(
    _In_ HREGREADBATCH hRegReadBatch,
        _In_ DWORD flags,
    _Out_ HREGREADBATCHREPLY * phRegReadBatchReply);

LONG WINAPI ClusterRegReadBatchReplyNextCommand(
    _In_ HREGREADBATCHREPLY hRegReadBatchReply,
    _Out_ CLUSTER_READ_BATCH_COMMAND * pBatchCommand);

typedef LONG
(WINAPI *PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND)(
    _In_ HREGREADBATCHREPLY hRegReadBatchReply,
    _Out_ CLUSTER_READ_BATCH_COMMAND * pBatchCommand);

LONG WINAPI ClusterRegCloseReadBatchReply(
    _In_ HREGREADBATCHREPLY hRegReadBatchReply);

typedef LONG
(WINAPI *PCLUSTER_REG_CLOSE_READ_BATCH_REPLY)(
    _In_ HREGREADBATCHREPLY hRegReadBatchReply);

DWORD
WINAPI
ClusterSetAccountAccess(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR szAccountSID,          //Takes in String SID or an account name
    _In_ DWORD dwAccess,
    _In_ DWORD dwControlType
);

typedef DWORD
(WINAPI *PCLUSTER_SET_ACCOUNT_ACCESS)(
    _In_ HCLUSTER hCluster,
    _In_ LPCWSTR szAccountSID,
    _In_ DWORD dwAccess,
    _In_ DWORD dwControlType
);


//
// Cluster Create, Add Node and Destroy Cluster APIs
//

//
// Phases that cluster setup operations through.
// Phase numbers may arrive in any order via the callback function.
//

typedef enum _CLUSTER_SETUP_PHASE {

    ClusterSetupPhaseInitialize                     = 1,

    ClusterSetupPhaseValidateNodeState              = 100,
    ClusterSetupPhaseValidateNetft                  = 102,
    ClusterSetupPhaseValidateClusDisk               = 103,
    ClusterSetupPhaseConfigureClusSvc               = 104,
    ClusterSetupPhaseStartingClusSvc                = 105,

    ClusterSetupPhaseQueryClusterNameAccount        = 106,
    ClusterSetupPhaseValidateClusterNameAccount     = 107,
    ClusterSetupPhaseCreateClusterAccount           = 108,
    ClusterSetupPhaseConfigureClusterAccount        = 109,

    ClusterSetupPhaseFormingCluster                 = 200,
    ClusterSetupPhaseAddClusterProperties           = 201,
    ClusterSetupPhaseCreateResourceTypes            = 202,
    ClusterSetupPhaseCreateGroups                   = 203,
    ClusterSetupPhaseCreateIPAddressResources       = 204,
    ClusterSetupPhaseCreateNetworkName              = 205,
    ClusterSetupPhaseClusterGroupOnline             = 206,

    ClusterSetupPhaseGettingCurrentMembership       = 300,
    ClusterSetupPhaseAddNodeToCluster               = 301,
    ClusterSetupPhaseNodeUp                         = 302,
    ClusterSetupPhaseApplyNetworkATCIntents         = 303,

    ClusterSetupPhaseMoveGroup                      = 400,
    ClusterSetupPhaseDeleteGroup                    = 401,
    ClusterSetupPhaseCleanupCOs                     = 402,
    ClusterSetupPhaseOfflineGroup                   = 403,
    ClusterSetupPhaseEvictNode                      = 404,
    ClusterSetupPhaseCleanupNode                    = 405,
    ClusterSetupPhaseCoreGroupCleanup               = 406,

    ClusterSetupPhaseRepairCNOAccount               = 500,
    ClusterSetupPhaseRepairDNSPermissions           = 501,

    ClusterSetupPhaseFailureCleanup                 = 999

} CLUSTER_SETUP_PHASE;

//
// used to delineate between phases
//
typedef enum _CLUSTER_SETUP_PHASE_TYPE {

    ClusterSetupPhaseStart                          = 1,
    ClusterSetupPhaseContinue                       = 2,
    ClusterSetupPhaseEnd                            = 3,
    ClusterSetupPhaseReport                         = 4

} CLUSTER_SETUP_PHASE_TYPE;

typedef enum _CLUSTER_SETUP_PHASE_SEVERITY {

    ClusterSetupPhaseInformational                  = 1,
    ClusterSetupPhaseWarning                        = 2,
    ClusterSetupPhaseFatal                          = 3

} CLUSTER_SETUP_PHASE_SEVERITY;

typedef BOOL
(WINAPI *PCLUSTER_SETUP_PROGRESS_CALLBACK)(
    PVOID                           pvCallbackArg,
    CLUSTER_SETUP_PHASE             eSetupPhase,
    CLUSTER_SETUP_PHASE_TYPE        ePhaseType,
    CLUSTER_SETUP_PHASE_SEVERITY    ePhaseSeverity,
    DWORD                           dwPercentComplete,
    _In_opt_ PCWSTR                 lpszObjectName,
    DWORD                           dwStatus );

HCLUSTER
WINAPI
CreateCluster(
    _In_ PCREATE_CLUSTER_CONFIG pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_CREATE_CLUSTER)(
    _In_ PCREATE_CLUSTER_CONFIG pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg
    );

HCLUSTER
WINAPI
CreateClusterCNOless(
    _In_ PCREATE_CLUSTER_CONFIG pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg
    );

typedef HCLUSTER
(WINAPI * PCLUSAPI_CREATE_CLUSTER_CNOLESS)(
    _In_ PCREATE_CLUSTER_CONFIG pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg
    );

DWORD
WINAPI
CreateClusterNameAccount(
    _In_ HCLUSTER    hCluster,
    _In_ PCREATE_CLUSTER_NAME_ACCOUNT pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg
    );

typedef DWORD
(WINAPI * PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT)(
    _In_ HCLUSTER    hCluster,
    _In_ PCREATE_CLUSTER_NAME_ACCOUNT pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg
    );

DWORD
WINAPI
RemoveClusterNameAccount(
    _In_ HCLUSTER    hCluster,
    _In_ BOOL        bDeleteComputerObjects
);

DWORD
WINAPI
RepairClusterNameAccount(
    _In_ HCLUSTER hCluster,
    _In_ PREPAIR_CLUSTER_NAME_ACCOUNT_CONFIG pConfig,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK pfnProgressCallback,
    _In_opt_ PVOID              pvCallbackArg);

typedef DWORD
(WINAPI * PCLUSAPI_REPAIR_CLUSTER_NAME_ACCOUNT)(
    _In_ HCLUSTER    hCluster
    );

DWORD
WINAPI
DetermineCNOResTypeFromNodelist(
    _In_ DWORD                          cNodes,
    _In_ PCWSTR *                       ppszNodeNames,
    _Out_ CLUSTER_MGMT_POINT_RESTYPE*   pCNOResType
);


DWORD
WINAPI
DetermineCNOResTypeFromCluster(
    _In_ HCLUSTER                       hCluster,
    _Out_ CLUSTER_MGMT_POINT_RESTYPE*   pCNOResType
);

DWORD
WINAPI
DetermineClusterCloudTypeFromNodelist(
    _In_ DWORD                  cNodes,
    _In_ PCWSTR *               ppszNodeNames,
    _Out_ PCLUSTER_CLOUD_TYPE   pCloudType
);


DWORD
WINAPI
DetermineClusterCloudTypeFromCluster(
    _In_ HCLUSTER               hCluster,
    _Out_ PCLUSTER_CLOUD_TYPE   pCloudType
);

DWORD
WINAPI
GetNodeCloudTypeDW(
    _In_ PCWSTR  ppszNodeName,
    __out DWORD* NodeCloudType);

typedef DWORD (WINAPI *PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT)(
    _In_ HCLUSTER    hCluster
);

DWORD
WINAPI
RegisterClusterResourceTypeNotifyV2 (
    __in  HCHANGE hChange,
    __in  HCLUSTER hCluster,
    __in  LONGLONG Flags,
    __in  LPCWSTR resTypeName,
    __in  DWORD_PTR dwNotifyKey
    );

HNODE
WINAPI
AddClusterNode(
    _In_ HCLUSTER    hCluster,
    _In_ PCWSTR      lpszNodeName,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg
    );

DWORD
WINAPI
AddClusterStorageNode(
    _In_ HCLUSTER    hCluster,
    _In_ PCWSTR      lpszNodeName,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg,
    _In_opt_ LPCWSTR lpszClusterStorageNodeDescription,
    _In_opt_ LPCWSTR lpszClusterStorageNodeLocation
    );

HNODE
WINAPI
AddClusterNodeEx(
    _In_ HCLUSTER    hCluster,
    _In_ PCWSTR      lpszNodeName,
    _In_ DWORD       dwFlags,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg
    );

DWORD
WINAPI
RemoveClusterStorageNode(
    __in HCLUSTER hCluster,
    __in LPCWSTR lpszClusterStorageEnclosureName,
    __in DWORD dwTimeout,
    __in DWORD dwFlags
    );

typedef HNODE
(WINAPI * PCLUSAPI_ADD_CLUSTER_NODE)(
    _In_ HCLUSTER    hCluster,
    _In_ PCWSTR      lpszNodeName,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg
    );

typedef HNODE
(WINAPI * PCLUSAPI_ADD_CLUSTER_NODE_EX)(
    _In_ HCLUSTER    hCluster,
    _In_ PCWSTR      lpszNodeName,
    _In_ DWORD       dwFlags,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg
    );

DWORD
WINAPI
DestroyCluster(
    _In_ HCLUSTER    hCluster,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg,
    _In_     BOOL    fdeleteVirtualComputerObjects
    );

typedef DWORD
(WINAPI * PCLUSAPI_DESTROY_CLUSTER)(
    _In_ HCLUSTER    hCluster,
    _In_opt_ PCLUSTER_SETUP_PROGRESS_CALLBACK   pfnProgressCallback,
    _In_opt_ PVOID   pvCallbackArg,
    _In_     BOOL    fdeleteVirtualComputerObjects
    );


//
// Standard Resource Type Names, these are duplicated in resapi.w's CLUSTER_ROLE struct. Be sure
// to update that struct when adding restype names.
//

#define CLUS_RESTYPE_NAME_GENAPP                L"Generic Application"
#define CLUS_RESTYPE_NAME_GENSVC                L"Generic Service"
#define CLUS_RESTYPE_NAME_GENSCRIPT             L"Generic Script"
#define CLUS_RESTYPE_NAME_IPADDR                L"IP Address"
#define CLUS_RESTYPE_NAME_NETNAME               L"Network Name"
#define CLUS_RESTYPE_NAME_DNN                   L"Distributed Network Name"
#define CLUS_RESTYPE_NAME_FILESHR               L"File Share"
#define CLUS_RESTYPE_NAME_PRTSPLR               L"Print Spooler"
#define CLUS_RESTYPE_NAME_IPV6_NATIVE           L"IPv6 Address"
#define CLUS_RESTYPE_NAME_IPV6_TUNNEL           L"IPv6 Tunnel Address"
#define CLUS_RESTYPE_NAME_VSSTASK               L"Volume Shadow Copy Service Task"
#define CLUS_RESTYPE_NAME_WINS                  L"WINS Service"
#define CLUS_RESTYPE_NAME_DHCP                  L"DHCP Service"
#define CLUS_RESTYPE_NAME_MSMQ                  L"Microsoft Message Queue Server"
#define CLUS_RESTYPE_NAME_NEW_MSMQ              L"MSMQ"
#define CLUS_RESTYPE_NAME_MSMQ_TRIGGER          L"MSMQTriggers"
#define CLUS_RESTYPE_NAME_MSDTC                 L"Distributed Transaction Coordinator"
#define CLUS_RESTYPE_NAME_NFS                   L"NFS Share"
#define CLUS_RESTYPE_NAME_NETWORK_FILE_SYSTEM   L"Network File System"
#define CLUS_RESTYPE_NAME_ISNS                  L"Microsoft iSNS"
#define CLUS_RESTYPE_NAME_HARDDISK              L"Physical Disk"
#define CLUS_RESTYPE_NAME_PHYS_DISK             CLUS_RESTYPE_NAME_HARDDISK
#define CLUS_RESTYPE_NAME_FSWITNESS             L"File Share Witness"
#define CLUS_RESTYPE_NAME_FILESERVER            L"File Server"
#define CLUS_RESTYPE_NAME_SODAFILESERVER        L"Scale Out File Server"
#define CLUS_RESTYPE_NAME_DFS                   L"Distributed File System"
#define CLUS_RESTYPE_NAME_DFSR                  L"DFS Replicated Folder"
#define CLUS_RESTYPE_NAME_VM                    L"Virtual Machine"
#define CLUS_RESTYPE_NAME_VM_CONFIG             L"Virtual Machine Configuration"
#define CLUS_RESTYPE_NAME_ISCSITARGET           L"iSCSI Target Server"
#define CLUS_RESTYPE_NAME_STORAGE_POOL          L"Storage Pool"
#define CLUS_RESTYPE_NAME_TASKSCHEDULER         L"Task Scheduler"
#define CLUS_RESTYPE_NAME_VMREPLICA_BROKER      L"Virtual Machine Replication Broker"
#define CLUS_RESTYPE_NAME_VMREPLICA_COORDINATOR      L"Virtual Machine Replication Coordinator"
#define CLUS_RESTYPE_NAME_NFS_V2                L"Network File System"
#define CLUS_RESTYPE_NAME_NFS_MSNS              L"NFS Multi Server Namespace"
#define CLUS_RESTYPE_NAME_CAU                   L"ClusterAwareUpdatingResource"
#define CLUS_RESTYPE_NAME_NV_PROVIDER_ADDRESS   L"Provider Address"
#define CLUS_RESTYPE_NAME_NAT                   L"Nat"
#define CLUS_RESTYPE_NAME_STORAGE_POLICIES      L"Storage Policies"
#define CLUS_RESTYPE_NAME_STORQOS               L"Storage QoS Policy Manager"
#define CLUS_RESTYPE_NAME_HEALTH_SERVICE        L"Health Service"
#define CLUS_RESTYPE_NAME_VM_WMI                L"Virtual Machine Cluster WMI"
#define CLUS_RESTYPE_NAME_SDDC_MANAGEMENT       L"SDDC Management"
#define CLUS_RESTYPE_NAME_HCSVM                 L"HCS Virtual Machine"

#define CLUS_RESTYPE_NAME_VIRTUAL_IPV4          L"Disjoint IPv4 Address"
#define CLUS_RESTYPE_NAME_VIRTUAL_IPV6          L"Disjoint IPv6 Address"
#define CLUS_RESTYPE_NAME_CLOUD_WITNESS         L"Cloud Witness"
#define CLUS_RESTYPE_NAME_STORAGE_REPLICA       L"Storage Replica"
#define CLUS_RESTYPE_NAME_CROSS_CLUSTER         L"Cross Cluster Dependency Orchestrator"

#define CLUS_RESTYPE_NAME_SCALEOUT_MASTER       L"Scaleout Master"
#define CLUS_RESTYPE_NAME_SCALEOUT_WORKER       L"Scaleout Worker"

#define CLUS_RESTYPE_NAME_CONTAINER             L"Container"

#define CLUS_RES_NAME_SCALEOUT_MASTER           L"Scaleout Master"
#define CLUS_RES_NAME_SCALEOUT_WORKER           L"Scaleout Worker"

#define CLUS_RESTYPE_NAME_KEY_VALUE_STORE       L"Key Value Store"


//
// Cluster common property names
//

#define CLUSREG_NAME_CLUS_DESC                     L"Description"
#define CLUSREG_NAME_CLUS_SD                       L"Security Descriptor"
#define CLUSREG_NAME_CLUS_DEFAULT_NETWORK_ROLE     L"DefaultNetworkRole"
#define CLUSREG_NAME_QUORUM_ARBITRATION_TIMEOUT    L"QuorumArbitrationTimeMax"
#define CLUSTER_HANG_TIMEOUT_KEYNAME               L"ClusSvcHangTimeout"
#define CLUSTER_HANG_RECOVERY_ACTION_KEYNAME       L"HangRecoveryAction"
#define CLUSTER_CSA_VSS_STATE                      L"BackupInProgress"
#define CLUSTER_REQUEST_REPLY_TIMEOUT              L"RequestReplyTimeout"
#define CLUSTER_WITNESS_FAILED_RESTART_INTERVAL    L"WitnessRestartInterval"
#define CLUSTER_WITNESS_DATABASE_WRITE_TIMEOUT     L"WitnessDatabaseWriteTimeout"
#define CLUSTER_ADD_EVICT_DELAY                    L"AddEvictDelay"
#define CLUSREG_NAME_FIXQUORUM                     L"FixQuorum"
#define CLUSREG_NAME_PREVENTQUORUM                 L"PreventQuorum"
#define CLUSREG_NAME_IGNORE_PERSISTENT_STATE       L"IgnorePersistentStateOnStartup"
#define CLUSTER_SHARED_VOLUMES_ROOT                L"SharedVolumesRoot"
#define ENABLE_CLUSTER_SHARED_VOLUMES              L"EnableSharedVolumes"
#define CLUSTER_SHARED_VOLUME_VSS_WRITER_OPERATION_TIMEOUT   L"SharedVolumeVssWriterOperationTimeout"
#define USE_CLIENT_ACCESS_NETWORKS_FOR_CSV         L"UseClientAccessNetworksForSharedVolumes"
#define CLUSTER_CSV_COMPATIBLE_FILTERS             L"SharedVolumeCompatibleFilters"
#define CLUSTER_CSV_INCOMPATIBLE_FILTERS           L"SharedVolumeIncompatibleFilters"
#define CLUSTER_GROUP_WAIT_DELAY                   L"ClusterGroupWaitDelay"
#define MINIMUM_PREEMPTOR_PRIORITY                 L"MinimumPreemptorPriority"
#define MINIMUM_NEVER_PREEMPT_PRIORITY             L"MinimumNeverPreemptPriority"
#define CLUSTER_ENFORCED_ANTIAFFINITY              L"ClusterEnforcedAntiaffinity"
#define CLUSREG_NAME_SHUTDOWN_TIMEOUT_MINUTES      L"ShutdownTimeoutInMinutes"
#define CLUSREG_NAME_CSV_MDS_SD                    L"SharedVolumeSecurityDescriptor"
#define CLUSREG_NAME_FAILOVER_MOVE_MIGRATION_TYPE  L"FailoverMoveMigrationType"
#define CLUSREG_NAME_CSV_BLOCK_CACHE               L"BlockCacheSize"
#define CLUSREG_NAME_ROUTE_HISTORY_LENGTH          L"RouteHistoryLength"
#define CLUSREG_NAME_LAST_RECENT_EVENTS_RESET_TIME L"RecentEventsResetTime"
#define CLUSREG_NAME_DRAIN_ON_SHUTDOWN             L"DrainOnShutdown"
#define CLUSREG_NAME_NETFT_IPSEC_ENABLED           L"NetftIPSecEnabled"
#define CLUSREG_NAME_WITNESS_DYNAMIC_WEIGHT        L"WitnessDynamicWeight"
#define CLUSREG_NAME_MESSAGE_BUFFER_LENGTH         L"MessageBufferLength"
#define CLUSREG_NAME_DATABASE_READ_WRITE_MODE      L"DatabaseReadWriteMode"
#define CLUSREG_NAME_FUNCTIONAL_LEVEL              L"ClusterFunctionalLevel"
#define CLUSREG_NAME_UPGRADE_VERSION               L"ClusterUpgradeVersion"
#define CLUSREG_NAME_RESILIENCY_LEVEL              L"ResiliencyLevel"
#define CLUSREG_NAME_RESILIENCY_DEFAULT_SECONDS    L"ResiliencyDefaultPeriod"
#define CLUSREG_NAME_QUARANTINE_THRESHOLD          L"QuarantineThreshold"
#define CLUSREG_NAME_QUARANTINE_DURATION           L"QuarantineDuration"
#define CLUSTER_S2D_ENABLED                        L"S2DEnabled"
#define CLUSTER_S2D_BUS_TYPES                      L"S2DBusTypes"
#define CLUSTER_S2D_CACHE_PAGE_SIZE_KBYTES         L"S2DCachePageSizeKBytes"
#define CLUSTER_S2D_OPTIMIZATIONS                  L"S2DOptimizations"
#define CLUSTER_S2D_IO_LATENCY_THRESHOLD           L"S2DIOLatencyThreshold"
#define CLUSTER_S2D_CACHE_DESIRED_STATE            L"S2DCacheDesiredState"
#define CLUSTER_S2D_CACHE_METADATA_RESERVE         L"S2DCacheMetadataReserveBytes"
#define CLUSTER_S2D_CACHE_FLASH_RESERVE_PERCENT    L"S2DCacheFlashReservePercent"
#define CLUSTER_S2D_CACHE_BEHAVIOR_FLAGS           L"S2DCacheBehavior"
#define CLUSTER_NAME_PREFERRED_SITE                L"PreferredSite"
#define CLUSTER_NAME_AUTO_BALANCER_MODE            L"AutoBalancerMode"
#define CLUSTER_NAME_AUTO_BALANCER_LEVEL           L"AutoBalancerLevel"
#define CLUSREG_NAME_GROUP_DEPENDENCY_TIMEOUT      L"GroupDependencyTimeout"
#define CLUSREG_NAME_PLACEMENT_OPTIONS             L"PlacementOptions"
#define CLUSREG_NAME_ENABLED_EVENT_LOGS            L"EnabledEventLogs"
#define CLUSREG_NAME_MAX_PARALLEL_MIGRATIONS       L"MaximumParallelMigrations"
#define CLUSREG_NAME_ACCELERATED_NETWORKING_ENABLED      L"AcceleratedNetworkingEnabled"
#define CLUSREG_NAME_ACCELERATED_NETWORKING_NODE_RESERVE L"AcceleratedNetworkingNodeReserve"

//
// Properties and defaults for single and multi subnet delays and thresholds.
//

#define CLUSREG_NAME_SAME_SUBNET_DELAY              L"SameSubnetDelay"
#define CLUSREG_NAME_CROSS_SUBNET_DELAY             L"CrossSubnetDelay"
#define CLUSREG_NAME_CROSS_SITE_DELAY               L"CrossSiteDelay"
#define CLUSREG_NAME_SAME_SUBNET_THRESHOLD          L"SameSubnetThreshold"
#define CLUSREG_NAME_CROSS_SUBNET_THRESHOLD         L"CrossSubnetThreshold"
#define CLUSREG_NAME_CROSS_SITE_THRESHOLD           L"CrossSiteThreshold"
#define CLUSREG_NAME_PLUMB_ALL_CROSS_SUBNET_ROUTES  L"PlumbAllCrossSubnetRoutes"

//
// Node common property names
//

#define CLUSREG_NAME_NODE_NAME                      L"NodeName"
#define CLUSREG_NAME_NODE_HIGHEST_VERSION           L"NodeHighestVersion"
#define CLUSREG_NAME_NODE_LOWEST_VERSION            L"NodeLowestVersion"
#define CLUSREG_NAME_NODE_DESC                      L"Description"
#define CLUSREG_NAME_NODE_MAJOR_VERSION             L"MajorVersion"
#define CLUSREG_NAME_NODE_MINOR_VERSION             L"MinorVersion"
#define CLUSREG_NAME_NODE_BUILD_NUMBER              L"BuildNumber"
#define CLUSREG_NAME_NODE_CSDVERSION                L"CSDVersion"
#define CLUSREG_NAME_NODE_WEIGHT                    L"NodeWeight"
#define CLUSREG_NAME_NODE_DYNAMIC_WEIGHT            L"DynamicWeight"
#define CLUSREG_NAME_NODE_IS_PRIMARY                L"IsPrimary"
#define CLUSREG_NAME_NODE_DRAIN_STATUS              L"NodeDrainStatus"
#define CLUSREG_NAME_NODE_DRAIN_TARGET              L"NodeDrainTarget"
#define CLUSREG_NAME_NODE_NEEDS_PQ                  L"NeedsPreventQuorum"
#define CLUSREG_NAME_NODE_FDID                      L"FaultDomainId"
#define CLUSREG_NAME_NODE_STATUS_INFO               L"StatusInformation"
#define CLUSREG_NAME_NODE_FAULT_DOMAIN              L"FaultDomain"
#define CLUSREG_NAME_NODE_MODEL                     L"Model"
#define CLUSREG_NAME_NODE_SERIALNUMBER              L"SerialNumber"
#define CLUSREG_NAME_NODE_MANUFACTURER              L"Manufacturer"
#define CLUSREG_NAME_NODE_UNIQUEID                  L"UniqueID"
#define CLUSREG_NAME_NODE_DRAIN_ERROR_CODE          L"DrainErrorCode"
#define CLUSREG_NAME_NODE_FAILBACK_STATUS           L"NodeFailbackStatus"
#define CLUSREG_NAME_NODE_FAILBACK_ERROR_CODE       L"FailbackErrorCode"
#define CLUSREG_NAME_NODE_HYPERTHREADING_ENABLED    L"HyperthreadingEnabled"

//
// Group common property names
//

#define CLUSREG_NAME_GRP_NAME                              L"Name"
#define CLUSREG_NAME_GRP_TYPE                              L"GroupType"
#define CLUSREG_NAME_GRP_DESC                              L"Description"
#define CLUSREG_NAME_GRP_PERSISTENT_STATE                  L"PersistentState"
#define CLUSREG_NAME_GRP_FAILBACK_TYPE                     L"AutoFailbackType"
#define CLUSREG_NAME_GRP_FAILBACK_WIN_START                L"FailbackWindowStart"
#define CLUSREG_NAME_GRP_FAILBACK_WIN_END                  L"FailbackWindowEnd"
#define CLUSREG_NAME_GRP_FAILOVER_THRESHOLD                L"FailoverThreshold"
#define CLUSREG_NAME_GRP_FAILOVER_PERIOD                   L"FailoverPeriod"
#define CLUSREG_NAME_GRP_PRIORITY                          L"Priority"
#define CLUSREG_NAME_GRP_DEFAULT_OWNER                     L"DefaultOwner"
#define CLUSREG_NAME_GRP_STATUS_INFORMATION                L"StatusInformation"
#define CLUSREG_NAME_GRP_ANTI_AFFINITY_CLASS_NAME          L"AntiAffinityClassNames"
#define CLUSREG_NAME_GRP_START_DELAY                       L"GroupStartDelay"
#define CLUSREG_NAME_GRP_CCF_EPOCH                         L"CCFEpoch"
#define CLUSREG_NAME_GRP_CCF_EPOCH_HIGH                    L"CCFEpochHigh"
#define CLUSREG_NAME_GRP_RESILIENCY_PERIOD                 L"ResiliencyPeriod"
#define CLUSREG_NAME_GRP_PREFERRED_SITE                    L"PreferredSite"
#define CLUSREG_NAME_GRP_COLD_START_SETTING                L"ColdStartSetting"
#define CLUSREG_NAME_GRP_FAULT_DOMAIN                      L"FaultDomain"
#define CLUSREG_NAME_GRP_UPDATE_DOMAIN                     L"UpdateDomain"
#define CLUSREG_NAME_GRP_PLACEMENT_OPTIONS                 L"PlacementOptions"
#define CLUSREG_NAME_GRP_LOCK_MOVE                         L"LockedFromMoving"

//
// Resource common property names
//

#define CLUSREG_NAME_RES_NAME                              L"Name"
#define CLUSREG_NAME_RES_TYPE                              L"Type"
#define CLUSREG_NAME_RES_DESC                              L"Description"
#define CLUSREG_NAME_RES_SEPARATE_MONITOR                  L"SeparateMonitor"
#define CLUSREG_NAME_RES_PERSISTENT_STATE                  L"PersistentState"
#define CLUSREG_NAME_RES_LOOKS_ALIVE                       L"LooksAlivePollInterval"
#define CLUSREG_NAME_RES_IS_ALIVE                          L"IsAlivePollInterval"
#define CLUSREG_NAME_RES_RESTART_ACTION                    L"RestartAction"
#define CLUSREG_NAME_RES_RESTART_THRESHOLD                 L"RestartThreshold"
#define CLUSREG_NAME_RES_RESTART_PERIOD                    L"RestartPeriod"
#define CLUSREG_NAME_RES_RESTART_DELAY                     L"RestartDelay"
#define CLUSREG_NAME_RES_RETRY_PERIOD_ON_FAILURE           L"RetryPeriodOnFailure"
#define CLUSREG_NAME_RES_EMBEDDED_FAILURE_ACTION           L"EmbeddedFailureAction"
#define CLUSREG_NAME_RES_PENDING_TIMEOUT                   L"PendingTimeout"
#define CLUSREG_NAME_RES_DEADLOCK_TIMEOUT                  L"DeadlockTimeout"
#define CLUSREG_NAME_RES_MONITOR_PID                       L"MonitorProcessId"
#define CLUSREG_NAME_RES_STATUS_INFORMATION                L"StatusInformation"
#define CLUSREG_NAME_RES_LAST_OPERATION_STATUS_CODE        L"LastOperationStatusCode"
#define CLUSREG_NAME_RES_DATA1                             L"ResourceSpecificData1"
#define CLUSREG_NAME_RES_DATA2                             L"ResourceSpecificData2"
#define CLUSREG_NAME_RES_STATUS                            L"ResourceSpecificStatus"


//
// Resource Type common property names
//

#define CLUSREG_NAME_RESTYPE_NAME               L"Name"
#define CLUSREG_NAME_RESTYPE_DESC               L"Description"
#define CLUSREG_NAME_RESTYPE_DLL_NAME           L"DllName"
#define CLUSREG_NAME_RESTYPE_ADMIN_EXTENSIONS   L"AdminExtensions"
#define CLUSREG_NAME_RESTYPE_LOOKS_ALIVE        CLUSREG_NAME_RES_LOOKS_ALIVE
#define CLUSREG_NAME_RESTYPE_IS_ALIVE           CLUSREG_NAME_RES_IS_ALIVE
#define CLUSREG_NAME_RESTYPE_PENDING_TIMEOUT    CLUSREG_NAME_RES_PENDING_TIMEOUT
#define CLUSREG_NAME_RESTYPE_DEADLOCK_TIMEOUT   CLUSREG_NAME_RES_DEADLOCK_TIMEOUT
#define CLUSREG_NAME_RESTYPE_DUMP_POLICY        L"DumpPolicy"
#define CLUSREG_NAME_RESTYPE_DUMP_LOG_QUERY     L"DumpLogQuery"
#define CLUSREG_NAME_RESTYPE_DUMP_SERVICES      L"DumpServices"
#define CLUSREG_NAME_RESTYPE_ENABLED_EVENT_LOGS L"EnabledEventLogs"
#define CLUSREG_NAME_RESTYPE_MAX_MONITORS       L"MaximumMonitors"
#define CLUSREG_NAME_RESTYPE_WPR_START_AFTER    L"WprStartAfter"
#define CLUSREG_NAME_RESTYPE_WPR_PROFILES       L"WprProfiles"

//
// Network common property names
//

#define CLUSREG_NAME_NET_NAME               L"Name"
#define CLUSREG_NAME_NET_IPV6_ADDRESSES     L"IPv6Addresses"
#define CLUSREG_NAME_NET_IPV6_PREFIXLENGTHS L"IPv6PrefixLengths"
#define CLUSREG_NAME_NET_IPV4_ADDRESSES     L"IPv4Addresses"
#define CLUSREG_NAME_NET_IPV4_PREFIXLENGTHS L"IPv4PrefixLengths"
#define CLUSREG_NAME_NET_ADDRESS            L"Address"
#define CLUSREG_NAME_NET_ADDRESS_MASK       L"AddressMask"
#define CLUSREG_NAME_NET_DESC               L"Description"
#define CLUSREG_NAME_NET_ROLE               L"Role"
#define CLUSREG_NAME_NET_SPEED              L"LinkSpeed"
#define CLUSREG_NAME_NET_RDMA_CAPABLE       L"RdmaCapable"
#define CLUSREG_NAME_NET_RSS_CAPABLE        L"RssCapable"
#define CLUSREG_NAME_NET_METRIC             L"Metric"
#define CLUSREG_NAME_NET_AUTOMETRIC         L"AutoMetric"


//
// Network Interface common property names
//

#define CLUSREG_NAME_NETIFACE_NAME              L"Name"
#define CLUSREG_NAME_NETIFACE_NODE              L"Node"
#define CLUSREG_NAME_NETIFACE_NETWORK           L"Network"
#define CLUSREG_NAME_NETIFACE_ADAPTER_NAME      L"Adapter"
#define CLUSREG_NAME_NETIFACE_ADAPTER_ID        L"AdapterId"
#define CLUSREG_NAME_NETIFACE_DHCP_ENABLED      L"DhcpEnabled"
#define CLUSREG_NAME_NETIFACE_IPV6_ADDRESSES    L"IPv6Addresses"
#define CLUSREG_NAME_NETIFACE_IPV4_ADDRESSES    L"IPv4Addresses"
#define CLUSREG_NAME_NETIFACE_ADDRESS           L"Address"
#define CLUSREG_NAME_NETIFACE_DESC              L"Description"

//
// GroupSet common property names
//

#define CLUSREG_NAME_GROUPSET_NAME                  L"Name"
#define CLUSREG_NAME_GROUPSET_STARTUP_SETTING       L"StartupSetting"
#define CLUSREG_NAME_GROUPSET_STARTUP_COUNT         L"StartupCount"
#define CLUSREG_NAME_GROUPSET_STARTUP_DELAY         L"StartupDelay"
#define CLUSREG_NAME_GROUPSET_IS_GLOBAL             L"IsGlobal"
#define CLUSREG_NAME_GROUPSET_STATUS_INFORMATION    L"StatusInformation"
#define CLUSREG_NAME_GROUPSET_IS_AVAILABILITY_SET   L"IsAvailabilitySet"
#define CLUSREG_NAME_GROUPSET_UPDATE_DOMAINS        L"UpdateDomains"
#define CLUSREG_NAME_GROUPSET_FAULT_DOMAINS         L"FaultDomains"
#define CLUSREG_NAME_GROUPSET_RESERVE_NODE  L"ReserveSpareNode"
#define CLUSREG_NAME_GROUPSET_AVAILABILITY_SET_INDEX_TO_NODE_MAPPING L"NodeDomainInfo"

//
// Affinity rule property names
//
#define CLUSREG_NAME_AFFINITYRULE_NAME              L"Name"
#define CLUSREG_NAME_AFFINITYRULE_TYPE              L"RuleType"
#define CLUSREG_NAME_AFFINITYRULE_GROUPS            L"Groups"
#define CLUSREG_NAME_AFFINITYRULE_ENABLED           L"Enabled"
#define CLUSREG_NAME_AFFINITYRULE_SOFTANTIAFFINITY  L"SoftAntiAffinity"

//
// Resource private property names
//

//
// Common to All Resources interested in using Cluster Awareness of Application Memory Usage
//
#define CLUSREG_NAME_START_MEMORY               L"StartMemory"
#define CLUSREG_NAME_VIRTUAL_NUMA_COUNT         L"VirtualNumaCount"
#define CLUSREG_NAME_DDA_DEVICE_ALLOCATIONS     L"DdaDeviceAllocations"
#define CLUSREG_NAME_GPUP_DEVICE_ALLOCATIONS    L"GpupDeviceAllocations"

//
// Physical Disk
//

#define CLUSREG_NAME_PHYSDISK_DISKIDTYPE       L"DiskIdType"
#define CLUSREG_NAME_PHYSDISK_DISKSIGNATURE    L"DiskSignature"
#define CLUSREG_NAME_PHYSDISK_DISKIDGUID       L"DiskIdGuid"
#define CLUSREG_NAME_PHYSDISK_DISKRUNCHKDSK    L"DiskRunChkDsk"
#define CLUSREG_NAME_PHYSDISK_DISKUNIQUEIDS    L"DiskUniqueIds"
#define CLUSREG_NAME_PHYSDISK_DISKVOLUMEINFO   L"DiskVolumeInfo"
#define CLUSREG_NAME_PHYSDISK_DISKARBTYPE      L"DiskArbType"
#define CLUSREG_NAME_PHYSDISK_DISKARBINTERVAL  L"DiskArbInterval"
#define CLUSREG_NAME_PHYSDISK_DISKPATH         L"DiskPath"
#define CLUSREG_NAME_PHYSDISK_DISKRELOAD       L"DiskReload"
#define CLUSREG_NAME_PHYSDISK_MAINTMODE        L"MaintenanceMode"
#define CLUSREG_NAME_PHYSDISK_DISKIODELAY      L"MaxIoLatency"
#define CLUSREG_NAME_PHYSDISK_MIGRATEFIXUP     L"MigrateDriveLetters"
#define CLUSREG_NAME_PHYSDISK_CSVWRITETHROUGH  L"CsvEnforceWriteThrough"
#define CLUSREG_NAME_PHYSDISK_CSVBLOCKCACHE    L"EnableBlockCache"
#define CLUSREG_NAME_PHYSDISK_FASTONLINEARBITRATE L"FastOnlineArbitrate"
#define CLUSREG_NAME_PHYSDISK_SPACEIDGUID      L"VirtualDiskId"
#define CLUSREG_NAME_STORAGESPACE_POOLIDGUID   L"PoolId"
#define CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTDIFFAREASIZE L"SnapshotDiffSize"
#define CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTAGELIMIT L"SnapshotAgeLimit"
#define CLUSREG_NAME_PHYSDISK_DISKGUID         L"DiskGuid"
#define CLUSREG_NAME_PHYSDISK_VOLSNAPACTIVATETIMEOUT L"VolsnapActivateTimeout"
#define CLUSREG_NAME_PHYSDISK_DISKRECOVERYACTION L"DiskRecoveryAction"


#define CLUSREG_NAME_STORAGESPACE_NAME                  L"VirtualDiskName"
#define CLUSREG_NAME_STORAGESPACE_DESCRIPTION           L"VirtualDiskDescription"
#define CLUSREG_NAME_STORAGESPACE_HEALTH                L"VirtualDiskHealth"
#define CLUSREG_NAME_STORAGESPACE_STATE                 L"VirtualDiskState"
#define CLUSREG_NAME_STORAGESPACE_PROVISIONING          L"VirtualDiskProvisioning"
#define CLUSREG_NAME_STORAGESPACE_RESILIENCYTYPE        L"VirtualDiskResiliencyType"
#define CLUSREG_NAME_STORAGESPACE_RESILIENCYCOLUMNS     L"VirtualDiskResiliencyColumns"
#define CLUSREG_NAME_STORAGESPACE_RESILIENCYINTERLEAVE  L"VirtualDiskResiliencyInterleave"


//
// Generic Application
//

#define CLUSREG_NAME_GENAPP_COMMAND_LINE            L"CommandLine"
#define CLUSREG_NAME_GENAPP_CURRENT_DIRECTORY       L"CurrentDirectory"
#define CLUSREG_NAME_GENAPP_USE_NETWORK_NAME        L"UseNetworkName"

//
// Generic Script
//

#define CLUSREG_NAME_GENSCRIPT_SCRIPT_FILEPATH      L"ScriptFilepath"


//
// Generic Service
//

#define CLUSREG_NAME_GENSVC_SERVICE_NAME            L"ServiceName"
#define CLUSREG_NAME_GENSVC_STARTUP_PARAMS          L"StartupParameters"
#define CLUSREG_NAME_GENSVC_USE_NETWORK_NAME        L"UseNetworkName"


//
// IPv4 Address
//

#define CLUSREG_NAME_IPADDR_NETWORK                     L"Network"
#define CLUSREG_NAME_IPADDR_ADDRESS                     L"Address"
#define CLUSREG_NAME_IPADDR_SUBNET_MASK                 L"SubnetMask"
#define CLUSREG_NAME_IPADDR_ENABLE_NETBIOS              L"EnableNetBIOS"
#define CLUSREG_NAME_IPADDR_OVERRIDE_ADDRMATCH          L"OverrideAddressMatch"
#define CLUSREG_NAME_IPADDR_ENABLE_DHCP                 L"EnableDhcp"
#define CLUSREG_NAME_IPADDR_LEASE_OBTAINED_TIME         L"LeaseObtainedTime"
#define CLUSREG_NAME_IPADDR_LEASE_TERMINATES_TIME       L"LeaseExpiresTime"
#define CLUSREG_NAME_IPADDR_T1                          L"T1"
#define CLUSREG_NAME_IPADDR_T2                          L"T2"
#define CLUSREG_NAME_IPADDR_DHCP_SERVER                 L"DhcpServer"
#define CLUSREG_NAME_IPADDR_DHCP_ADDRESS                L"DhcpAddress"
#define CLUSREG_NAME_IPADDR_DHCP_SUBNET_MASK            L"DhcpSubnetMask"
#define CLUSREG_NAME_IPADDR_SHARED_NETNAME              L"SharedNetname"
#define CLUSREG_NAME_IPADDR_PROBE_PORT                  L"ProbePort"
#define CLUSREG_NAME_IPADDR_PROBE_FAILURE_THRESHOLD     L"ProbeFailureThreshold"

//
// IPv6 Address
//

#define CLUSREG_NAME_IPV6_NATIVE_NETWORK            L"Network"
#define CLUSREG_NAME_IPV6_NATIVE_ADDRESS            L"Address"
#define CLUSREG_NAME_IPV6_NATIVE_PREFIX_LENGTH      L"PrefixLength"

#define CLUSREG_NAME_IPV6_TUNNEL_ADDRESS            L"Address"
#define CLUSREG_NAME_IPV6_TUNNEL_TUNNELTYPE         L"TunnelType"


//
// Network Name
//
#define CLUSREG_NAME_NETNAME_NAME                   L"Name"
#define CLUSREG_NAME_NETNAME_CREATING_DC            L"CreatingDC"
#define CLUSREG_NAME_NETNAME_OBJECT_ID              L"ObjectGUID"
#define CLUSREG_NAME_NETNAME_DNS_NAME               L"DnsName"
#define CLUSREG_NAME_NETNAME_REMAP_PIPE_NAMES       L"RemapPipeNames"
#define CLUSREG_NAME_NETNAME_RESOURCE_DATA          L"ResourceData"
#define CLUSREG_NAME_NETNAME_STATUS_NETBIOS         L"StatusNetBIOS"
#define CLUSREG_NAME_NETNAME_STATUS_DNS             L"StatusDNS"
#define CLUSREG_NAME_NETNAME_STATUS_KERBEROS        L"StatusKerberos"
#define CLUSREG_NAME_NETNAME_VCO_CONTAINER          L"VcoContainer"
#define CLUSREG_NAME_NETNAME_LAST_DNS_UPDATE        L"LastDNSUpdateTime"
#define CLUSREG_NAME_NETNAME_CONTAINERGUID          L"CryptoContainerGUID"
#define CLUSREG_NAME_NETNAME_HOST_TTL               L"HostRecordTTL"
#define CLUSREG_NAME_NETNAME_PUBLISH_PTR            L"PublishPTRRecords"
#define CLUSREG_NAME_NETNAME_REMOVEVCO_ONDELETE     L"DeleteVcoOnResCleanup"
#define CLUSREG_NAME_NETNAME_REGISTER_ALL_IP        L"RegisterAllProvidersIP"
#define CLUSREG_KEYNAME_OBJECTGUIDS                 L"ObjectGUIDs"
#define CLUSREG_NAME_NETNAME_EXCLUDE_NETWORKS       L"ExcludeNetworks"
#define CLUSREG_NAME_NETNAME_ALIASES                L"Aliases"
#define CLUSREG_NAME_NETNAME_IN_USE_NETWORKS        L"InUseNetworks"
#define CLUSREG_NAME_NETNAME_DNS_SUFFIX             L"DnsSuffix"
#define CLUSREG_NAME_NETNAME_AD_AWARE               L"ADAware"
#define CLUSREG_NAME_NETNAME_DNN_DISABLE_CLONES     L"DisableClones"

//
// Print Spooler
//

#define CLUSREG_NAME_PRTSPOOL_DEFAULT_SPOOL_DIR     L"DefaultSpoolDirectory"
#define CLUSREG_NAME_PRTSPOOL_TIMEOUT               L"JobCompletionTimeout"

//
// File Share
//

#define CLUSREG_NAME_FILESHR_SERVER_NAME            L"ServerName"
#define CLUSREG_NAME_FILESHR_SHARE_NAME             L"ShareName"
#define CLUSREG_NAME_FILESHR_PATH                   L"Path"
#define CLUSREG_NAME_FILESHR_REMARK                 L"Remark"
#define CLUSREG_NAME_FILESHR_MAX_USERS              L"MaxUsers"
#define CLUSREG_NAME_FILESHR_SD                     L"Security Descriptor"
#define CLUSREG_NAME_FILESHR_SHARE_SUBDIRS          L"ShareSubDirs"
#define CLUSREG_NAME_FILESHR_HIDE_SUBDIR_SHARES     L"HideSubDirShares"
#define CLUSREG_NAME_FILESHR_IS_DFS_ROOT            L"IsDfsRoot"
#define CLUSREG_NAME_FILESHR_SHARE_FLAGS            L"ShareFlags"
#define CLUSREG_NAME_FILESHR_CA_TIMEOUT             L"CATimeout"
#define CLUSREG_NAME_FILESHR_QOS_FLOWSCOPE          L"QosFlowScope"
#define CLUSREG_NAME_FILESHR_QOS_POLICYID           L"QosPolicyId"

//
// DHCP Service
//

#define CLUSREG_NAME_DHCP_DATABASE_PATH             L"DatabasePath"
#define CLUSREG_NAME_DHCP_BACKUP_PATH               L"BackupPath"
#define CLUSREG_NAME_LOG_FILE_PATH                  L"LogFilePath"


//
// WINS Service
//

#define CLUSREG_NAME_WINS_DATABASE_PATH             L"DatabasePath"
#define CLUSREG_NAME_WINS_BACKUP_PATH               L"BackupPath"


//
// Volume Shadow Copy Service Task
//

#define CLUSREG_NAME_VSSTASK_CURRENTDIRECTORY       L"CurrentDirectory"
#define CLUSREG_NAME_VSSTASK_APPNAME                L"ApplicationName"
#define CLUSREG_NAME_VSSTASK_APPPARAMS              L"ApplicationParams"
#define CLUSREG_NAME_VSSTASK_TRIGGERARRAY           L"TriggerArray"

//
// File Share Quorum Witness Resource
//

#define CLUSREG_NAME_FSWITNESS_SHARE_PATH           L"SharePath"
#define CLUSREG_NAME_FSWITNESS_ARB_DELAY            L"ArbitrationDelay"
#define CLUSREG_NAME_FSWITNESS_IMPERSONATE_CNO      L"ImpersonateCNO"

//
// Storage Pool
//
#define CLUSREG_NAME_STORAGESPACE_POOLNAME              L"Name"
#define CLUSREG_NAME_STORAGESPACE_POOLDESC              L"Description"
#define CLUSREG_NAME_STORAGESPACE_POOLDRIVEIDS          L"DriveIds"
#define CLUSREG_NAME_STORAGESPACE_POOLHEALTH            L"Health"
#define CLUSREG_NAME_STORAGESPACE_POOLSTATE             L"State"
#define CLUSREG_NAME_STORAGESPACE_POOLTOTALCAPACITY     L"TotalCapacity"
#define CLUSREG_NAME_STORAGESPACE_POOLCONSUMEDCAPACITY  L"ConsumedCapacity"

#define CLUSREG_NAME_STORAGESPACE_POOLARBITRATE         L"Arbitrate"
#define CLUSREG_NAME_STORAGESPACE_POOLREEVALTIMEOUT     L"ReEvaluatePlacementTimeout"
#define CLUSREG_NAME_STORAGESPACE_POOLQUORUMSHARE       L"PoolQuorumShare"
#define CLUSREG_NAME_STORAGESPACE_POOLQUORUMUSERACCOUNT L"PoolQuorumUserAccount"

//
// Scale Out File Server
//
#define CLUSREG_NAME_SOFS_SMBASYMMETRYMODE              L"SmbAsymmetryMode"

//
// VIP Address
//
#define CLUSREG_NAME_VIP_PREFIX_LENGTH            L"PrefixLength"
#define CLUSREG_NAME_VIP_ADAPTER_NAME             L"AdapterName"
#define CLUSREG_NAME_VIP_ADDRESS                  L"Address"
#define CLUSREG_NAME_VIP_VSID                     L"VSID"
#define CLUSREG_NAME_VIP_RDID                     L"RDID"


//
// Cloud Witness
//
#define CLUSREG_NAME_CLOUDWITNESS_PRIMARY_TOKEN         L"PrimaryToken"
#define CLUSREG_NAME_CLOUDWITNESS_PRIMARY_KEY           L"PrimaryKey"
#define CLUSREG_NAME_CLOUDWITNESS_ACCOUNT_NAME          L"AccountName"
#define CLUSREG_NAME_CLOUDWITNESS_ENDPOINT_INFO         L"EndpointInfo"
#define CLUSREG_NAME_CLOUDWITNESS_CONTAINER_NAME        L"ContainerName"
#define CLUSREG_NAME_CLOUDWITNESS_MANAGED_IDENTITY      L"IsManagedIdentity"
#define CLOUD_WITNESS_CONTAINER_NAME                    L"msft-cloud-witness"

// Storage Replica
#define CLUS_NAME_RES_TYPE_SOURCE_RESID                 L"SourceResourceId"
#define CLUS_NAME_RES_TYPE_TARGET_RESID                 L"TargetResourceId"
#define CLUS_NAME_RES_TYPE_SOURCE_VOLUMES               L"SourceVolumes"
#define CLUS_NAME_RES_TYPE_TARGET_VOLUMES               L"TargetVolumes"
#define CLUS_NAME_RES_TYPE_DATA_RESID                   L"DataResourceId"
#define CLUS_NAME_RES_TYPE_LOG_RESID                    L"LogResourceId"
#define CLUS_NAME_RES_TYPE_LOG_VOLUME                   L"LogVolume"
#define CLUS_NAME_RES_TYPE_REPLICATION_GROUPID          L"ReplicationGroupId"
#define CLUS_NAME_RES_TYPE_CLUSTER_GROUPID              L"ClusterGroupId"
#define CLUS_NAME_RES_TYPE_REPLICATION_GROUP_TYPE       L"ReplicationClusterGroupType"
#define CLUS_NAME_RES_TYPE_MINIMUM_LOG_SIZE             L"MinimumLogSizeInBytes"
#define CLUS_NAME_RES_TYPE_UNIT_LOG_SIZE_CHANGE         L"UnitOfLogSizeChangeInBytes"
#define CLUS_NAME_RES_TYPE_LOG_MULTIPLE                 L"LogSizeMultiple"

// Key Value Store
#define CLUSREG_NAME_KEYVALUESTORE_NAME                 L"KeyValueStores"
#define CLUSREG_NAME_KEYVALUESTORE_MANAGERNAME          L"ManagerName"
#define CLUSREG_NAME_KEYVALUESTORE_MANAGERPATH          L"ManagerPath"


typedef enum PLACEMENT_OPTIONS {
    PLACEMENT_OPTIONS_MIN_VALUE                     = 0x00000000,
    PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS     = PLACEMENT_OPTIONS_MIN_VALUE,
    PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY     = 0x00000001,
    PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS          = 0x00000002,
    PLACEMENT_OPTIONS_DONT_USE_MEMORY               = 0x00000004,
    PLACEMENT_OPTIONS_DONT_USE_CPU                  = 0x00000008,
    PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK      = 0x00000010,
    PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK                       = 0x00000020,
    PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE                   = 0x00000040,
    PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK       = 0x00000080,
    PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE   = 0x00000100,
    PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY                              = 0x00000200,
    PLACEMENT_OPTIONS_ALL                           = (PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY      |
                                                      PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS            |
                                                      PLACEMENT_OPTIONS_DONT_USE_MEMORY                 |
                                                      PLACEMENT_OPTIONS_DONT_USE_CPU                    |
                                                      PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK        |
                                                      PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK                 |
                                                      PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE             |
                                                      PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK |
                                                      PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE |
                                                      PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY)
} PLACEMENT_OPTIONS;

typedef enum GRP_PLACEMENT_OPTIONS {
    GRP_PLACEMENT_OPTIONS_MIN_VALUE                     = 0x00000000,
    GRP_PLACEMENT_OPTIONS_DEFAULT                       = GRP_PLACEMENT_OPTIONS_MIN_VALUE,
    GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING         = 0x00000001,
    GRP_PLACEMENT_OPTIONS_ALL                           = (GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING)
} GRP_PLACEMENT_OPTIONS;

#define SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO   0x00000001

typedef struct _SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO
{
     ULONGLONG PartitionOffset;                   /**< Offset of the partition in the disk */
     ULONG     Capabilities;                      /**< Capabilities of replicated partition*/
} SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO, *PSR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO;

typedef struct _SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY
{
   DWORD                                        Count;                /**< Count of all replicated partitions on the disk*/
   SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO         PartitionArray[1];    /**< Variable size array of all replicated partitions on the disk*/
} SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY, *PSR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY;

//
// Storage Replica structures and enums used to get replication information from the replication resource types
//
typedef enum _SR_REPLICATED_DISK_TYPE
{
    SrReplicatedDiskTypeNone                = 0,
    SrReplicatedDiskTypeSource              = 1,
    SrReplicatedDiskTypeLogSource           = 2,
    SrReplicatedDiskTypeDestination         = 3,
    SrReplicatedDiskTypeLogDestination      = 4,
    SrReplicatedDiskTypeNotInParthership    = 5,
    SrReplicatedDiskTypeLogNotInParthership = 6,
    SrReplicatedDiskTypeOther
} SR_REPLICATED_DISK_TYPE, *PSR_REPLICATED_DISK_TYPE;

typedef enum _SR_DISK_REPLICATION_ELIGIBLE
{
    SrDiskReplicationEligibleNone                       = 0,
    SrDiskReplicationEligibleYes                        = 1,
    SrDiskReplicationEligibleOffline                    = 2,
    SrDiskReplicationEligibleNotGpt                     = 3,
    SrDiskReplicationEligiblePartitionLayoutMismatch    = 4,
    SrDiskReplicationEligibleInsufficientFreeSpace      = 5,
    SrDiskReplicationEligibleNotInSameSite              = 6,
    SrDiskReplicationEligibleInSameSite                 = 7,
    SrDiskReplicationEligibleFileSystemNotSupported     = 8,
    SrDiskReplicationEligibleAlreadyInReplication       = 9,
    SrDiskReplicationEligibleSameAsSpecifiedDisk        = 10,
    SrDiskReplicationEligibleOther                      = 9999
} SR_DISK_REPLICATION_ELIGIBLE, *PSR_DISK_REPLICATION_ELIGIBLE;

typedef struct _SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS
{
    GUID        DataDiskGuid;            /**< Cluster resource identifier of data disk.*/
    BOOLEAN     IncludeOfflineDisks;     /**< When TRUE, the result set includes all the offline disks in Available Storage group.*/
} SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS, *PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS;

typedef struct _SR_RESOURCE_TYPE_ELIGIBLE_TARGET_DATADISKS
{
    GUID        SourceDataDiskGuid;         /**< Cluster resource identifier of data disk.*/
    GUID        TargetReplicationGroupGuid; /**< Replication group to which one of returned set of disk may be added.*/
    BOOLEAN     SkipConnectivityCheck;      /**< When TRUE, even the disks that are connected to same nodes as the source disk are included in result set.*/
    BOOLEAN     IncludeOfflineDisks;        /**< When TRUE, the result set includes offline disks in Available Storage group.*/
} SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS, *PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS;

typedef struct _SR_RESOURCE_TYPE_ELIGIBLE_SOURCE_DATADISKS
{
    GUID        DataDiskGuid;                     /**< Cluster resource identifier of data disk.*/
    BOOLEAN     IncludeAvailableStoargeDisks;     /**< When TRUE, the result set includes disks in Available Storage group.*/
} SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS, *PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS;

typedef struct _SR_RESOURCE_TYPE_DISK_INFO
{
    SR_DISK_REPLICATION_ELIGIBLE        Reason;                  /**< Number of disks in the result set.*/
    GUID                                DiskGuid;                /**< Cluster resource identifier of disk.*/
} SR_RESOURCE_TYPE_DISK_INFO, *PSR_RESOURCE_TYPE_DISK_INFO;

typedef struct _SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT
{
    USHORT                      Count;             /**< Number of disks in the result set.*/
    SR_RESOURCE_TYPE_DISK_INFO  DiskInfo[1];       /**< Cluster resource identifier and related information about the disk.*/
} SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT, *PSR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT;

typedef struct _SR_RESOURCE_TYPE_REPLICATED_DISK
{
    SR_REPLICATED_DISK_TYPE  Type;                            /**< Type of the replicated disk.*/
    GUID                     ClusterDiskResourceGuid;         /**< Cluster resource identifier of disk.*/
    GUID                     ReplicationGroupId;              /**< Replication group identifier.*/
    WCHAR                    ReplicationGroupName[MAX_PATH];  /**< Name of the replication group name.*/
} SR_RESOURCE_TYPE_REPLICATED_DISK, *PSR_RESOURCE_TYPE_REPLICATED_DISK;

typedef struct _SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT
{
    USHORT Count;                                        /**< Number of replicated disks in the result set.*/
    SR_RESOURCE_TYPE_REPLICATED_DISK ReplicatedDisks[1]; /**< Array of replicated disks.*/
} SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT, *PSR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT;

typedef struct _SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP
{
    WCHAR       ReplicationGroupName[MAX_PATH];          /**< The name of the replication group to create*/
    WCHAR       Description[MAX_PATH];                   /**< A text description of the group*/
    WCHAR       LogPath[MAX_PATH];                       /**< Full path of the log container*/
    ULONGLONG   MaxLogSizeInBytes;                       /**< The maximum size of the log file in Bytes*/
    USHORT      LogType;                                 /**< Whether the log is file based CLFS log (1) or RAW SR log (2)*/
    DWORD       ReplicationMode;                         /**< Whether the replication is performed synchronously(1) or asynchronously(2)*/
    DWORD       MinimumPartnersInSync;                   /**< Minimum number of synchronous Replication Partners to be actively in sync before allowing data access by applications on the primary Replica*/
    BOOLEAN     EnableWriteConsistency;                  /**< Set true to enable write consistency*/
    BOOLEAN     EnableEncryption;                        /**< true to enable encryption; otherwise, false*/
    BOOLEAN     EnableCompression;                       /**< true to enable compression; otherwise, false*/
    WCHAR       CertificateThumbprint[MAX_PATH];         /**< The certificate thumbprint*/
    ULONG       VolumeNameCount;                         /**< Count of number of volumes in \ref VolumeNames field*/
    WCHAR       VolumeNames[ANYSIZE_ARRAY][MAX_PATH];    /**< A collection of volume names*/
} SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP, *PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP;

typedef struct _SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT
{
    DWORD       Result;                                  /**< Result code*/
    WCHAR       ErrorString[MAX_PATH];                   /**< Buffer that contains error string from remote CIM calls.*/
} SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT, *PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT;


//
// Input structure for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2 / CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2
//
typedef struct _CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    DWORD   dwFlags;
    GUID    guidPoolFilter;
} CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT, *PCLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT;

#define CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO 0x00000001
#define CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL  0x00000002
#define CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS  0x00000004

#if _MSC_VER >= 1200
#pragma warning(pop)              // restore 4200/4201
#else
#pragma warning( default : 4200 ) // nonstandard extension used : zero-sized array in struct/union
#pragma warning( default : 4201 ) // nonstandard extension used : nameless struct/union
#endif
#endif // MIDL_PASS

#ifdef __cplusplus
} // extern "C"

#endif

#ifndef _CLUSTER_API_TYPES_
#define _CLUSTER_API_TYPES_
#endif // _CLUSTER_API_TYPES_


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CLUSTER) */
#pragma endregion

#endif // _IN_KERNEL_
#endif // _CLUSTER_API_

