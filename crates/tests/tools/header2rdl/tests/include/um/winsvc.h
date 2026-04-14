/*++ BUILD Version: 0010    // Increment this if a change has global effects

Copyright (c) 1995-1998  Microsoft Corporation

Module Name:

    winsvc.h

Abstract:

    Header file for the Service Control Manager

Environment:

    User Mode - Win32

--*/
#ifndef _WINSVC_
#define _WINSVC_

#include <winapifamily.h>

//
// Define API decoration for direct importing of DLL references.
//

#if !defined(WINADVAPI)
#if !defined(_ADVAPI32_)
#define WINADVAPI DECLSPEC_IMPORT
#else
#define WINADVAPI
#endif
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// Constants
//

//
// Service database names
//

#define SERVICES_ACTIVE_DATABASEW      L"ServicesActive"
#define SERVICES_FAILED_DATABASEW      L"ServicesFailed"

#define SERVICES_ACTIVE_DATABASEA      "ServicesActive"
#define SERVICES_FAILED_DATABASEA      "ServicesFailed"

//
// Character to designate that a name is a group
//

#define SC_GROUP_IDENTIFIERW           L'+'
#define SC_GROUP_IDENTIFIERA           '+'

#ifdef UNICODE

#define SERVICES_ACTIVE_DATABASE       SERVICES_ACTIVE_DATABASEW
#define SERVICES_FAILED_DATABASE       SERVICES_FAILED_DATABASEW


#define SC_GROUP_IDENTIFIER            SC_GROUP_IDENTIFIERW

#else // ndef UNICODE

#define SERVICES_ACTIVE_DATABASE       SERVICES_ACTIVE_DATABASEA
#define SERVICES_FAILED_DATABASE       SERVICES_FAILED_DATABASEA

#define SC_GROUP_IDENTIFIER            SC_GROUP_IDENTIFIERA
#endif // ndef UNICODE


//
// Value to indicate no change to an optional parameter
//
#define SERVICE_NO_CHANGE              0xffffffff

//
// Service State -- for Enum Requests (Bit Mask)
//
#define SERVICE_ACTIVE                 0x00000001
#define SERVICE_INACTIVE               0x00000002
#define SERVICE_STATE_ALL              (SERVICE_ACTIVE   | \
                                        SERVICE_INACTIVE)

//
// Controls
//
#define SERVICE_CONTROL_STOP                   0x00000001
#define SERVICE_CONTROL_PAUSE                  0x00000002
#define SERVICE_CONTROL_CONTINUE               0x00000003
#define SERVICE_CONTROL_INTERROGATE            0x00000004
#define SERVICE_CONTROL_SHUTDOWN               0x00000005
#define SERVICE_CONTROL_PARAMCHANGE            0x00000006
#define SERVICE_CONTROL_NETBINDADD             0x00000007
#define SERVICE_CONTROL_NETBINDREMOVE          0x00000008
#define SERVICE_CONTROL_NETBINDENABLE          0x00000009
#define SERVICE_CONTROL_NETBINDDISABLE         0x0000000A
#define SERVICE_CONTROL_DEVICEEVENT            0x0000000B
#define SERVICE_CONTROL_HARDWAREPROFILECHANGE  0x0000000C
#define SERVICE_CONTROL_POWEREVENT             0x0000000D
#define SERVICE_CONTROL_SESSIONCHANGE          0x0000000E
#define SERVICE_CONTROL_PRESHUTDOWN            0x0000000F
#define SERVICE_CONTROL_TIMECHANGE             0x00000010
//#define SERVICE_CONTROL_USER_LOGOFF            0x00000011
#define SERVICE_CONTROL_TRIGGEREVENT           0x00000020
//reserved for internal use                    0x00000021
//reserved for internal use                    0x00000050
#define SERVICE_CONTROL_LOWRESOURCES           0x00000060
#define SERVICE_CONTROL_SYSTEMLOWRESOURCES     0x00000061

//
// Service State -- for CurrentState
//
#define SERVICE_STOPPED                        0x00000001
#define SERVICE_START_PENDING                  0x00000002
#define SERVICE_STOP_PENDING                   0x00000003
#define SERVICE_RUNNING                        0x00000004
#define SERVICE_CONTINUE_PENDING               0x00000005
#define SERVICE_PAUSE_PENDING                  0x00000006
#define SERVICE_PAUSED                         0x00000007

//
// Controls Accepted  (Bit Mask)
//
#define SERVICE_ACCEPT_STOP                    0x00000001
#define SERVICE_ACCEPT_PAUSE_CONTINUE          0x00000002
#define SERVICE_ACCEPT_SHUTDOWN                0x00000004
#define SERVICE_ACCEPT_PARAMCHANGE             0x00000008
#define SERVICE_ACCEPT_NETBINDCHANGE           0x00000010
#define SERVICE_ACCEPT_HARDWAREPROFILECHANGE   0x00000020
#define SERVICE_ACCEPT_POWEREVENT              0x00000040
#define SERVICE_ACCEPT_SESSIONCHANGE           0x00000080
#define SERVICE_ACCEPT_PRESHUTDOWN             0x00000100
#define SERVICE_ACCEPT_TIMECHANGE              0x00000200
#define SERVICE_ACCEPT_TRIGGEREVENT            0x00000400
#define SERVICE_ACCEPT_USER_LOGOFF             0x00000800
// reserved for internal use                   0x00001000
#define SERVICE_ACCEPT_LOWRESOURCES            0x00002000
#define SERVICE_ACCEPT_SYSTEMLOWRESOURCES      0x00004000

//
// Service Control Manager object specific access types
//
#define SC_MANAGER_CONNECT             0x0001
#define SC_MANAGER_CREATE_SERVICE      0x0002
#define SC_MANAGER_ENUMERATE_SERVICE   0x0004
#define SC_MANAGER_LOCK                0x0008
#define SC_MANAGER_QUERY_LOCK_STATUS   0x0010
#define SC_MANAGER_MODIFY_BOOT_CONFIG  0x0020

#define SC_MANAGER_ALL_ACCESS          (STANDARD_RIGHTS_REQUIRED      | \
                                        SC_MANAGER_CONNECT            | \
                                        SC_MANAGER_CREATE_SERVICE     | \
                                        SC_MANAGER_ENUMERATE_SERVICE  | \
                                        SC_MANAGER_LOCK               | \
                                        SC_MANAGER_QUERY_LOCK_STATUS  | \
                                        SC_MANAGER_MODIFY_BOOT_CONFIG)



//
// Service object specific access type
//
#define SERVICE_QUERY_CONFIG           0x0001
#define SERVICE_CHANGE_CONFIG          0x0002
#define SERVICE_QUERY_STATUS           0x0004
#define SERVICE_ENUMERATE_DEPENDENTS   0x0008
#define SERVICE_START                  0x0010
#define SERVICE_STOP                   0x0020
#define SERVICE_PAUSE_CONTINUE         0x0040
#define SERVICE_INTERROGATE            0x0080
#define SERVICE_USER_DEFINED_CONTROL   0x0100

#define SERVICE_ALL_ACCESS             (STANDARD_RIGHTS_REQUIRED     | \
                                        SERVICE_QUERY_CONFIG         | \
                                        SERVICE_CHANGE_CONFIG        | \
                                        SERVICE_QUERY_STATUS         | \
                                        SERVICE_ENUMERATE_DEPENDENTS | \
                                        SERVICE_START                | \
                                        SERVICE_STOP                 | \
                                        SERVICE_PAUSE_CONTINUE       | \
                                        SERVICE_INTERROGATE          | \
                                        SERVICE_USER_DEFINED_CONTROL)

//
// Service flags for QueryServiceStatusEx
//
#define SERVICE_RUNS_IN_SYSTEM_PROCESS  0x00000001

//
// Info levels for ChangeServiceConfig2 and QueryServiceConfig2
//
#define SERVICE_CONFIG_DESCRIPTION              1
#define SERVICE_CONFIG_FAILURE_ACTIONS          2
#define SERVICE_CONFIG_DELAYED_AUTO_START_INFO  3
#define SERVICE_CONFIG_FAILURE_ACTIONS_FLAG     4
#define SERVICE_CONFIG_SERVICE_SID_INFO         5
#define SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO 6
#define SERVICE_CONFIG_PRESHUTDOWN_INFO         7
#define SERVICE_CONFIG_TRIGGER_INFO             8
#define SERVICE_CONFIG_PREFERRED_NODE           9
// reserved                                     10
// reserved                                     11
#define SERVICE_CONFIG_LAUNCH_PROTECTED         12

//
// Info levels for NotifyServiceStatusChange
//
#define SERVICE_NOTIFY_STATUS_CHANGE_1          1
#define SERVICE_NOTIFY_STATUS_CHANGE_2          2

#define SERVICE_NOTIFY_STATUS_CHANGE            SERVICE_NOTIFY_STATUS_CHANGE_2

//
// Service notification masks
//
#define SERVICE_NOTIFY_STOPPED                  0x00000001
#define SERVICE_NOTIFY_START_PENDING            0x00000002
#define SERVICE_NOTIFY_STOP_PENDING             0x00000004
#define SERVICE_NOTIFY_RUNNING                  0x00000008
#define SERVICE_NOTIFY_CONTINUE_PENDING         0x00000010
#define SERVICE_NOTIFY_PAUSE_PENDING            0x00000020
#define SERVICE_NOTIFY_PAUSED                   0x00000040
#define SERVICE_NOTIFY_CREATED                  0x00000080
#define SERVICE_NOTIFY_DELETED                  0x00000100
#define SERVICE_NOTIFY_DELETE_PENDING           0x00000200

//
// The following defines are for service stop reason codes
//

//
// Stop reason flags. Update SERVICE_STOP_REASON_FLAG_MAX when
// new flags are added.
//
#define SERVICE_STOP_REASON_FLAG_MIN                            0x00000000
#define SERVICE_STOP_REASON_FLAG_UNPLANNED                      0x10000000
#define SERVICE_STOP_REASON_FLAG_CUSTOM                         0x20000000
#define SERVICE_STOP_REASON_FLAG_PLANNED                        0x40000000
#define SERVICE_STOP_REASON_FLAG_MAX                            0x80000000

//
// Microsoft major reasons. Update SERVICE_STOP_REASON_MAJOR_MAX when
// new codes are added.
//
#define SERVICE_STOP_REASON_MAJOR_MIN                           0x00000000
#define SERVICE_STOP_REASON_MAJOR_OTHER                         0x00010000
#define SERVICE_STOP_REASON_MAJOR_HARDWARE                      0x00020000
#define SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM               0x00030000
#define SERVICE_STOP_REASON_MAJOR_SOFTWARE                      0x00040000
#define SERVICE_STOP_REASON_MAJOR_APPLICATION                   0x00050000
#define SERVICE_STOP_REASON_MAJOR_NONE                          0x00060000
#define SERVICE_STOP_REASON_MAJOR_MAX                           0x00070000
#define SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM                    0x00400000
#define SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM                    0x00ff0000

//
// Microsoft minor reasons. Update SERVICE_STOP_REASON_MINOR_MAX when
// new codes are added.
//
#define SERVICE_STOP_REASON_MINOR_MIN                           0x00000000
#define SERVICE_STOP_REASON_MINOR_OTHER                         0x00000001
#define SERVICE_STOP_REASON_MINOR_MAINTENANCE                   0x00000002
#define SERVICE_STOP_REASON_MINOR_INSTALLATION                  0x00000003
#define SERVICE_STOP_REASON_MINOR_UPGRADE                       0x00000004
#define SERVICE_STOP_REASON_MINOR_RECONFIG                      0x00000005
#define SERVICE_STOP_REASON_MINOR_HUNG                          0x00000006
#define SERVICE_STOP_REASON_MINOR_UNSTABLE                      0x00000007
#define SERVICE_STOP_REASON_MINOR_DISK                          0x00000008
#define SERVICE_STOP_REASON_MINOR_NETWORKCARD                   0x00000009
#define SERVICE_STOP_REASON_MINOR_ENVIRONMENT                   0x0000000a
#define SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER               0x0000000b
#define SERVICE_STOP_REASON_MINOR_OTHERDRIVER                   0x0000000c
#define SERVICE_STOP_REASON_MINOR_SERVICEPACK                   0x0000000d
#define SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE               0x0000000e
#define SERVICE_STOP_REASON_MINOR_SECURITYFIX                   0x0000000f
#define SERVICE_STOP_REASON_MINOR_SECURITY                      0x00000010
#define SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY          0x00000011
#define SERVICE_STOP_REASON_MINOR_WMI                           0x00000012
#define SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL         0x00000013
#define SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL     0x00000014
#define SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL         0x00000015
#define SERVICE_STOP_REASON_MINOR_MMC                           0x00000016
#define SERVICE_STOP_REASON_MINOR_NONE                          0x00000017
#define SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT                   0x00000018
#define SERVICE_STOP_REASON_MINOR_MAX                           0x00000019
#define SERVICE_STOP_REASON_MINOR_MIN_CUSTOM                    0x00000100
#define SERVICE_STOP_REASON_MINOR_MAX_CUSTOM                    0x0000FFFF

//
// Info levels for ControlServiceEx
//
#define SERVICE_CONTROL_STATUS_REASON_INFO      1

//
// Service SID types supported
//
#define SERVICE_SID_TYPE_NONE                                   0x00000000
#define SERVICE_SID_TYPE_UNRESTRICTED                           0x00000001
#define SERVICE_SID_TYPE_RESTRICTED                             ( 0x00000002 | SERVICE_SID_TYPE_UNRESTRICTED )

//
// Service trigger types
//
#define SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL               1
#define SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY                2
#define SERVICE_TRIGGER_TYPE_DOMAIN_JOIN                            3
#define SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT                    4
#define SERVICE_TRIGGER_TYPE_GROUP_POLICY                           5
#define SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT                       6
#define SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE             7
#define SERVICE_TRIGGER_TYPE_CUSTOM                                 20
#define SERVICE_TRIGGER_TYPE_AGGREGATE                              30

//
// Service trigger data types
//
#define SERVICE_TRIGGER_DATA_TYPE_BINARY                            1
#define SERVICE_TRIGGER_DATA_TYPE_STRING                            2
#define SERVICE_TRIGGER_DATA_TYPE_LEVEL                             3
#define SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY                       4
#define SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL                       5

//
//  Service start reason
//
#define SERVICE_START_REASON_DEMAND                                 0x00000001
#define SERVICE_START_REASON_AUTO                                   0x00000002
#define SERVICE_START_REASON_TRIGGER                                0x00000004
#define SERVICE_START_REASON_RESTART_ON_FAILURE                     0x00000008
#define SERVICE_START_REASON_DELAYEDAUTO                            0x00000010

//
//  Service dynamic information levels
//
#define SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON              1

//
// Service LaunchProtected types supported
//
#define SERVICE_LAUNCH_PROTECTED_NONE                               0
#define SERVICE_LAUNCH_PROTECTED_WINDOWS                            1
#define SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT                      2
#define SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT                  3

//
//  NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID & NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID are used with
//  SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY trigger.
//
DEFINE_GUID ( /* 4f27f2de-14e2-430b-a549-7cd48cbc8245 */
    NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID,
    0x4f27f2de,
    0x14e2,
    0x430b,
    0xa5, 0x49, 0x7c, 0xd4, 0x8c, 0xbc, 0x82, 0x45
  );

DEFINE_GUID ( /* cc4ba62a-162e-4648-847a-b6bdf993e335 */
    NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID,
    0xcc4ba62a,
    0x162e,
    0x4648,
    0x84, 0x7a, 0xb6, 0xbd, 0xf9, 0x93, 0xe3, 0x35
  );

//
//  DOMAIN_JOIN_GUID & DOMAIN_LEAVE_GUID are used with SERVICE_TRIGGER_TYPE_DOMAIN_JOIN trigger.
//
DEFINE_GUID ( /* 1ce20aba-9851-4421-9430-1ddeb766e809 */
    DOMAIN_JOIN_GUID,
    0x1ce20aba,
    0x9851,
    0x4421,
    0x94, 0x30, 0x1d, 0xde, 0xb7, 0x66, 0xe8, 0x09
  );

DEFINE_GUID ( /* ddaf516e-58c2-4866-9574-c3b615d42ea1 */
    DOMAIN_LEAVE_GUID,
    0xddaf516e,
    0x58c2,
    0x4866,
    0x95, 0x74, 0xc3, 0xb6, 0x15, 0xd4, 0x2e, 0xa1
  );

//
//  FIREWALL_PORT_OPEN_GUID & FIREWALL_PORT_CLOSE_GUID are used with
//  SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT trigger.
//
DEFINE_GUID ( /* b7569e07-8421-4ee0-ad10-86915afdad09 */
    FIREWALL_PORT_OPEN_GUID,
    0xb7569e07,
    0x8421,
    0x4ee0,
    0xad, 0x10, 0x86, 0x91, 0x5a, 0xfd, 0xad, 0x09
  );

DEFINE_GUID ( /* a144ed38-8e12-4de4-9d96-e64740b1a524 */
    FIREWALL_PORT_CLOSE_GUID,
    0xa144ed38,
    0x8e12,
    0x4de4,
    0x9d, 0x96, 0xe6, 0x47, 0x40, 0xb1, 0xa5, 0x24
  );

//
//  MACHINE_POLICY_PRESENT_GUID & USER_POLICY_PRESENT_GUID are used with
//  SERVICE_TRIGGER_TYPE_GROUP_POLICY trigger.
//
DEFINE_GUID ( /* 659FCAE6-5BDB-4DA9-B1FF-CA2A178D46E0 */
    MACHINE_POLICY_PRESENT_GUID,
    0x659FCAE6,
    0x5BDB,
    0x4DA9,
    0xB1, 0xFF, 0xCA, 0x2A, 0x17, 0x8D, 0x46, 0xE0
  );

DEFINE_GUID ( /* 54FB46C8-F089-464C-B1FD-59D1B62C3B50 */
    USER_POLICY_PRESENT_GUID,
    0x54FB46C8,
    0xF089,
    0x464C,
    0xB1, 0xFD, 0x59, 0xD1, 0xB6, 0x2C, 0x3B, 0x50
  );

//
// RPC_INTERFACE_EVENT_GUID, NAMED_PIPE_EVENT_GUID & TCP_PORT_EVENT_GUID are
// used with SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT trigger.

DEFINE_GUID ( /* bc90d167-9470-4139-a9ba-be0bbbf5b74d */
    RPC_INTERFACE_EVENT_GUID,
    0xbc90d167,
    0x9470,
    0x4139,
    0xa9, 0xba, 0xbe, 0x0b, 0xbb, 0xf5, 0xb7, 0x4d
  );

DEFINE_GUID ( /* 1f81d131-3fac-4537-9e0c-7e7b0c2f4b55 */
    NAMED_PIPE_EVENT_GUID,
    0x1f81d131,
    0x3fac,
    0x4537,
    0x9e, 0x0c, 0x7e, 0x7b, 0x0c, 0x2f, 0x4b, 0x55
  );

//
// CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID is used with SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE
//
DEFINE_GUID ( /* 2d7a2816-0c5e-45fc-9ce7-570e5ecde9c9 */
    CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID,
    0x2d7a2816,
    0x0c5e,
    0x45fc,
    0x9c, 0xe7, 0x57, 0x0e, 0x5e, 0xcd, 0xe9, 0xc9
  );

//
// Service notification trigger identifier
//
typedef struct
{
    DWORD Data[2];
} SERVICE_TRIGGER_CUSTOM_STATE_ID;

typedef struct _SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    union {
        SERVICE_TRIGGER_CUSTOM_STATE_ID CustomStateId;
        struct {
            DWORD DataOffset;
            BYTE Data[1];
        } s;
    } u;
} SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM, *LPSERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM;

//
// Service trigger actions
//
#define SERVICE_TRIGGER_ACTION_SERVICE_START                        1
#define SERVICE_TRIGGER_ACTION_SERVICE_STOP                         2


//
// argv[1] passed into ServiceMain of trigger started services
//
#define SERVICE_TRIGGER_STARTED_ARGUMENT                L"TriggerStarted"

//
// Service description string
//
typedef struct _SERVICE_DESCRIPTIONA {
    LPSTR       lpDescription;
} SERVICE_DESCRIPTIONA, *LPSERVICE_DESCRIPTIONA;
//
// Service description string
//
typedef struct _SERVICE_DESCRIPTIONW {
    LPWSTR      lpDescription;
} SERVICE_DESCRIPTIONW, *LPSERVICE_DESCRIPTIONW;
#ifdef UNICODE
typedef SERVICE_DESCRIPTIONW SERVICE_DESCRIPTION;
typedef LPSERVICE_DESCRIPTIONW LPSERVICE_DESCRIPTION;
#else
typedef SERVICE_DESCRIPTIONA SERVICE_DESCRIPTION;
typedef LPSERVICE_DESCRIPTIONA LPSERVICE_DESCRIPTION;
#endif // UNICODE

//
// Actions to take on service failure
//
typedef enum _SC_ACTION_TYPE {
        SC_ACTION_NONE          = 0,
        SC_ACTION_RESTART       = 1,
        SC_ACTION_REBOOT        = 2,
        SC_ACTION_RUN_COMMAND   = 3,
        SC_ACTION_OWN_RESTART   = 4
} SC_ACTION_TYPE;

typedef struct _SC_ACTION {
    SC_ACTION_TYPE  Type;
    DWORD           Delay;
} SC_ACTION, *LPSC_ACTION;

typedef struct _SERVICE_FAILURE_ACTIONSA {
    DWORD       dwResetPeriod;
    LPSTR       lpRebootMsg;
    LPSTR       lpCommand;
#ifdef __midl
    [range(0, 1024)]
#endif
    DWORD       cActions;
#ifdef __midl
    [size_is(cActions)]
#endif
    SC_ACTION * lpsaActions;
} SERVICE_FAILURE_ACTIONSA, *LPSERVICE_FAILURE_ACTIONSA;
typedef struct _SERVICE_FAILURE_ACTIONSW {
    DWORD       dwResetPeriod;
    LPWSTR      lpRebootMsg;
    LPWSTR      lpCommand;
#ifdef __midl
    [range(0, 1024)]
#endif
    DWORD       cActions;
#ifdef __midl
    [size_is(cActions)]
#endif
    SC_ACTION * lpsaActions;
} SERVICE_FAILURE_ACTIONSW, *LPSERVICE_FAILURE_ACTIONSW;
#ifdef UNICODE
typedef SERVICE_FAILURE_ACTIONSW SERVICE_FAILURE_ACTIONS;
typedef LPSERVICE_FAILURE_ACTIONSW LPSERVICE_FAILURE_ACTIONS;
#else
typedef SERVICE_FAILURE_ACTIONSA SERVICE_FAILURE_ACTIONS;
typedef LPSERVICE_FAILURE_ACTIONSA LPSERVICE_FAILURE_ACTIONS;
#endif // UNICODE

//
// Service delayed autostart info setting
//
typedef struct _SERVICE_DELAYED_AUTO_START_INFO {
    BOOL       fDelayedAutostart;      // Delayed autostart flag
} SERVICE_DELAYED_AUTO_START_INFO, *LPSERVICE_DELAYED_AUTO_START_INFO;

//
// Service failure actions flag setting
//
typedef struct _SERVICE_FAILURE_ACTIONS_FLAG {
    BOOL       fFailureActionsOnNonCrashFailures;       // Failure actions flag
} SERVICE_FAILURE_ACTIONS_FLAG, *LPSERVICE_FAILURE_ACTIONS_FLAG;

//
// Service SID info setting
//
typedef struct _SERVICE_SID_INFO {
    DWORD       dwServiceSidType;     // Service SID type
} SERVICE_SID_INFO, *LPSERVICE_SID_INFO;

//
// Service required privileges information
//
typedef struct _SERVICE_REQUIRED_PRIVILEGES_INFOA {
    LPSTR       pmszRequiredPrivileges;             // Required privileges multi-sz
} SERVICE_REQUIRED_PRIVILEGES_INFOA, *LPSERVICE_REQUIRED_PRIVILEGES_INFOA;
//
// Service required privileges information
//
typedef struct _SERVICE_REQUIRED_PRIVILEGES_INFOW {
    LPWSTR      pmszRequiredPrivileges;             // Required privileges multi-sz
} SERVICE_REQUIRED_PRIVILEGES_INFOW, *LPSERVICE_REQUIRED_PRIVILEGES_INFOW;
#ifdef UNICODE
typedef SERVICE_REQUIRED_PRIVILEGES_INFOW SERVICE_REQUIRED_PRIVILEGES_INFO;
typedef LPSERVICE_REQUIRED_PRIVILEGES_INFOW LPSERVICE_REQUIRED_PRIVILEGES_INFO;
#else
typedef SERVICE_REQUIRED_PRIVILEGES_INFOA SERVICE_REQUIRED_PRIVILEGES_INFO;
typedef LPSERVICE_REQUIRED_PRIVILEGES_INFOA LPSERVICE_REQUIRED_PRIVILEGES_INFO;
#endif // UNICODE

//
// Service preshutdown timeout setting
//
typedef struct _SERVICE_PRESHUTDOWN_INFO {
    DWORD       dwPreshutdownTimeout;   // Timeout in msecs
} SERVICE_PRESHUTDOWN_INFO, *LPSERVICE_PRESHUTDOWN_INFO;

//
//  Service trigger data item
//
typedef struct _SERVICE_TRIGGER_SPECIFIC_DATA_ITEM
{
    DWORD   dwDataType; // Data type -- one of SERVICE_TRIGGER_DATA_TYPE_* constants
#ifdef __midl
    [range(0, 1024)]
#endif
    DWORD   cbData;     // Size of trigger specific data
#ifdef __midl
    [size_is(cbData)]
#endif
    PBYTE   pData;      // Trigger specific data
} SERVICE_TRIGGER_SPECIFIC_DATA_ITEM, *PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM;

//
//  Trigger-specific information
//
typedef struct _SERVICE_TRIGGER
{
    DWORD                       dwTriggerType;              // One of SERVICE_TRIGGER_TYPE_* constants
    DWORD                       dwAction;                   // One of SERVICE_TRIGGER_ACTION_* constants
    GUID    *                   pTriggerSubtype;            // Provider GUID if the trigger type is SERVICE_TRIGGER_TYPE_CUSTOM
                                                            // Device class interface GUID if the trigger type is
                                                            // SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL
                                                            // Aggregate identifier GUID if type is aggregate.
#ifdef __midl
    [range(0, 64)]
#endif
    DWORD                       cDataItems;                 // Number of data items in pDataItems array
#ifdef __midl
    [size_is(cDataItems)]
#endif
    PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM  pDataItems;       // Trigger specific data
} SERVICE_TRIGGER, *PSERVICE_TRIGGER;

//
// Service trigger information
//
typedef struct _SERVICE_TRIGGER_INFO {
#ifdef __midl
    [range(0, 64)]
#endif
    DWORD                   cTriggers;  // Number of triggers in the pTriggers array
#ifdef __midl
    [size_is(cTriggers)]
#endif
    PSERVICE_TRIGGER        pTriggers;  // Array of triggers
    PBYTE                   pReserved;  // Reserved, must be NULL
} SERVICE_TRIGGER_INFO, *PSERVICE_TRIGGER_INFO;

#define SC_AGGREGATE_STORAGE_KEY L"System\\CurrentControlSet\\Control\\ServiceAggregatedEvents"

//
// Preferred node information
//
typedef struct _SERVICE_PREFERRED_NODE_INFO {
    USHORT                  usPreferredNode;    // Preferred node
    BOOLEAN                 fDelete;            // Delete the preferred node setting
} SERVICE_PREFERRED_NODE_INFO, *LPSERVICE_PREFERRED_NODE_INFO;

//
// Time change information
//
typedef struct _SERVICE_TIMECHANGE_INFO {
    LARGE_INTEGER   liNewTime;      // New time
    LARGE_INTEGER   liOldTime;      // Old time
} SERVICE_TIMECHANGE_INFO, *PSERVICE_TIMECHANGE_INFO;

//
// Service launch protected setting
//
typedef struct _SERVICE_LAUNCH_PROTECTED_INFO {
    DWORD       dwLaunchProtected;     // Service launch protected
} SERVICE_LAUNCH_PROTECTED_INFO, *PSERVICE_LAUNCH_PROTECTED_INFO;

//
// Handle Types
//

DECLARE_HANDLE(SC_HANDLE);
typedef SC_HANDLE   *LPSC_HANDLE;

DECLARE_HANDLE(SERVICE_STATUS_HANDLE);

//
// Info levels for QueryServiceStatusEx
//

typedef enum _SC_STATUS_TYPE {
    SC_STATUS_PROCESS_INFO      = 0
} SC_STATUS_TYPE;

//
// Info levels for EnumServicesStatusEx
//
typedef enum _SC_ENUM_TYPE {
    SC_ENUM_PROCESS_INFO        = 0
} SC_ENUM_TYPE;


//
// Service Status Structures
//

typedef struct _SERVICE_STATUS {
    DWORD   dwServiceType;
    DWORD   dwCurrentState;
    DWORD   dwControlsAccepted;
    DWORD   dwWin32ExitCode;
    DWORD   dwServiceSpecificExitCode;
    DWORD   dwCheckPoint;
    DWORD   dwWaitHint;
} SERVICE_STATUS, *LPSERVICE_STATUS;

typedef struct _SERVICE_STATUS_PROCESS {
    DWORD   dwServiceType;
    DWORD   dwCurrentState;
    DWORD   dwControlsAccepted;
    DWORD   dwWin32ExitCode;
    DWORD   dwServiceSpecificExitCode;
    DWORD   dwCheckPoint;
    DWORD   dwWaitHint;
    DWORD   dwProcessId;
    DWORD   dwServiceFlags;
} SERVICE_STATUS_PROCESS, *LPSERVICE_STATUS_PROCESS;


//
// Service Status Enumeration Structure
//

typedef struct _ENUM_SERVICE_STATUSA {
    LPSTR             lpServiceName;
    LPSTR             lpDisplayName;
    SERVICE_STATUS    ServiceStatus;
} ENUM_SERVICE_STATUSA, *LPENUM_SERVICE_STATUSA;
typedef struct _ENUM_SERVICE_STATUSW {
    LPWSTR            lpServiceName;
    LPWSTR            lpDisplayName;
    SERVICE_STATUS    ServiceStatus;
} ENUM_SERVICE_STATUSW, *LPENUM_SERVICE_STATUSW;
#ifdef UNICODE
typedef ENUM_SERVICE_STATUSW ENUM_SERVICE_STATUS;
typedef LPENUM_SERVICE_STATUSW LPENUM_SERVICE_STATUS;
#else
typedef ENUM_SERVICE_STATUSA ENUM_SERVICE_STATUS;
typedef LPENUM_SERVICE_STATUSA LPENUM_SERVICE_STATUS;
#endif // UNICODE

typedef struct _ENUM_SERVICE_STATUS_PROCESSA {
    LPSTR                     lpServiceName;
    LPSTR                     lpDisplayName;
    SERVICE_STATUS_PROCESS    ServiceStatusProcess;
} ENUM_SERVICE_STATUS_PROCESSA, *LPENUM_SERVICE_STATUS_PROCESSA;
typedef struct _ENUM_SERVICE_STATUS_PROCESSW {
    LPWSTR                    lpServiceName;
    LPWSTR                    lpDisplayName;
    SERVICE_STATUS_PROCESS    ServiceStatusProcess;
} ENUM_SERVICE_STATUS_PROCESSW, *LPENUM_SERVICE_STATUS_PROCESSW;
#ifdef UNICODE
typedef ENUM_SERVICE_STATUS_PROCESSW ENUM_SERVICE_STATUS_PROCESS;
typedef LPENUM_SERVICE_STATUS_PROCESSW LPENUM_SERVICE_STATUS_PROCESS;
#else
typedef ENUM_SERVICE_STATUS_PROCESSA ENUM_SERVICE_STATUS_PROCESS;
typedef LPENUM_SERVICE_STATUS_PROCESSA LPENUM_SERVICE_STATUS_PROCESS;
#endif // UNICODE

//
// Structures for the Lock API functions
//

typedef LPVOID  SC_LOCK;

typedef struct _QUERY_SERVICE_LOCK_STATUSA {
    DWORD   fIsLocked;
    LPSTR   lpLockOwner;
    DWORD   dwLockDuration;
} QUERY_SERVICE_LOCK_STATUSA, *LPQUERY_SERVICE_LOCK_STATUSA;
typedef struct _QUERY_SERVICE_LOCK_STATUSW {
    DWORD   fIsLocked;
    LPWSTR  lpLockOwner;
    DWORD   dwLockDuration;
} QUERY_SERVICE_LOCK_STATUSW, *LPQUERY_SERVICE_LOCK_STATUSW;
#ifdef UNICODE
typedef QUERY_SERVICE_LOCK_STATUSW QUERY_SERVICE_LOCK_STATUS;
typedef LPQUERY_SERVICE_LOCK_STATUSW LPQUERY_SERVICE_LOCK_STATUS;
#else
typedef QUERY_SERVICE_LOCK_STATUSA QUERY_SERVICE_LOCK_STATUS;
typedef LPQUERY_SERVICE_LOCK_STATUSA LPQUERY_SERVICE_LOCK_STATUS;
#endif // UNICODE



//
// Query Service Configuration Structure
//

typedef struct _QUERY_SERVICE_CONFIGA {
    DWORD   dwServiceType;
    DWORD   dwStartType;
    DWORD   dwErrorControl;
    LPSTR   lpBinaryPathName;
    LPSTR   lpLoadOrderGroup;
    DWORD   dwTagId;
    LPSTR   lpDependencies;
    LPSTR   lpServiceStartName;
    LPSTR   lpDisplayName;
} QUERY_SERVICE_CONFIGA, *LPQUERY_SERVICE_CONFIGA;
typedef struct _QUERY_SERVICE_CONFIGW {
    DWORD   dwServiceType;
    DWORD   dwStartType;
    DWORD   dwErrorControl;
    LPWSTR  lpBinaryPathName;
    LPWSTR  lpLoadOrderGroup;
    DWORD   dwTagId;
    LPWSTR  lpDependencies;
    LPWSTR  lpServiceStartName;
    LPWSTR  lpDisplayName;
} QUERY_SERVICE_CONFIGW, *LPQUERY_SERVICE_CONFIGW;
#ifdef UNICODE
typedef QUERY_SERVICE_CONFIGW QUERY_SERVICE_CONFIG;
typedef LPQUERY_SERVICE_CONFIGW LPQUERY_SERVICE_CONFIG;
#else
typedef QUERY_SERVICE_CONFIGA QUERY_SERVICE_CONFIG;
typedef LPQUERY_SERVICE_CONFIGA LPQUERY_SERVICE_CONFIG;
#endif // UNICODE



//
// Function Prototype for the Service Main Function
//

typedef VOID WINAPI SERVICE_MAIN_FUNCTIONW (
    DWORD dwNumServicesArgs,
    LPWSTR *lpServiceArgVectors
    );

typedef VOID WINAPI SERVICE_MAIN_FUNCTIONA (
    DWORD dwNumServicesArgs,
    LPTSTR *lpServiceArgVectors
    );

#ifdef UNICODE
#define SERVICE_MAIN_FUNCTION SERVICE_MAIN_FUNCTIONW
#else
#define SERVICE_MAIN_FUNCTION SERVICE_MAIN_FUNCTIONA
#endif //UNICODE

typedef VOID (WINAPI *LPSERVICE_MAIN_FUNCTIONW)(
    DWORD   dwNumServicesArgs,
    LPWSTR  *lpServiceArgVectors
    );

typedef VOID (WINAPI *LPSERVICE_MAIN_FUNCTIONA)(
    DWORD   dwNumServicesArgs,
    LPSTR   *lpServiceArgVectors
    );

#ifdef UNICODE
#define LPSERVICE_MAIN_FUNCTION LPSERVICE_MAIN_FUNCTIONW
#else
#define LPSERVICE_MAIN_FUNCTION LPSERVICE_MAIN_FUNCTIONA
#endif //UNICODE


//
// Service Start Table
//

typedef struct _SERVICE_TABLE_ENTRYA {
    LPSTR                       lpServiceName;
    LPSERVICE_MAIN_FUNCTIONA    lpServiceProc;
}SERVICE_TABLE_ENTRYA, *LPSERVICE_TABLE_ENTRYA;
typedef struct _SERVICE_TABLE_ENTRYW {
    LPWSTR                      lpServiceName;
    LPSERVICE_MAIN_FUNCTIONW    lpServiceProc;
}SERVICE_TABLE_ENTRYW, *LPSERVICE_TABLE_ENTRYW;
#ifdef UNICODE
typedef SERVICE_TABLE_ENTRYW SERVICE_TABLE_ENTRY;
typedef LPSERVICE_TABLE_ENTRYW LPSERVICE_TABLE_ENTRY;
#else
typedef SERVICE_TABLE_ENTRYA SERVICE_TABLE_ENTRY;
typedef LPSERVICE_TABLE_ENTRYA LPSERVICE_TABLE_ENTRY;
#endif // UNICODE

//
// Prototype for the Service Control Handler Function
//

typedef VOID WINAPI HANDLER_FUNCTION (
    DWORD    dwControl
    );

typedef DWORD WINAPI HANDLER_FUNCTION_EX (
    DWORD    dwControl,
    DWORD    dwEventType,
    LPVOID   lpEventData,
    LPVOID   lpContext
    );

typedef VOID (WINAPI *LPHANDLER_FUNCTION)(
    DWORD    dwControl
    );

typedef DWORD (WINAPI *LPHANDLER_FUNCTION_EX)(
    DWORD    dwControl,
    DWORD    dwEventType,
    LPVOID   lpEventData,
    LPVOID   lpContext
    );

//
// Service notification parameters
//
typedef
VOID
( CALLBACK * PFN_SC_NOTIFY_CALLBACK ) (
    _In_ PVOID pParameter
    );

//
//  Each new notify structure is a superset of the older version
//
typedef struct _SERVICE_NOTIFY_1 {
    DWORD                   dwVersion;
    PFN_SC_NOTIFY_CALLBACK  pfnNotifyCallback;
    PVOID                   pContext;
    DWORD                   dwNotificationStatus;
    SERVICE_STATUS_PROCESS  ServiceStatus;
} SERVICE_NOTIFY_1, *PSERVICE_NOTIFY_1;

typedef struct _SERVICE_NOTIFY_2A {
    DWORD                   dwVersion;
    PFN_SC_NOTIFY_CALLBACK  pfnNotifyCallback;
    PVOID                   pContext;
    DWORD                   dwNotificationStatus;
    SERVICE_STATUS_PROCESS  ServiceStatus;
    DWORD                   dwNotificationTriggered;
    LPSTR                   pszServiceNames;
} SERVICE_NOTIFY_2A, *PSERVICE_NOTIFY_2A;
typedef struct _SERVICE_NOTIFY_2W {
    DWORD                   dwVersion;
    PFN_SC_NOTIFY_CALLBACK  pfnNotifyCallback;
    PVOID                   pContext;
    DWORD                   dwNotificationStatus;
    SERVICE_STATUS_PROCESS  ServiceStatus;
    DWORD                   dwNotificationTriggered;
    LPWSTR                  pszServiceNames;
} SERVICE_NOTIFY_2W, *PSERVICE_NOTIFY_2W;
#ifdef UNICODE
typedef SERVICE_NOTIFY_2W SERVICE_NOTIFY_2;
typedef PSERVICE_NOTIFY_2W PSERVICE_NOTIFY_2;
#else
typedef SERVICE_NOTIFY_2A SERVICE_NOTIFY_2;
typedef PSERVICE_NOTIFY_2A PSERVICE_NOTIFY_2;
#endif // UNICODE

typedef SERVICE_NOTIFY_2A SERVICE_NOTIFYA, *PSERVICE_NOTIFYA;
typedef SERVICE_NOTIFY_2W SERVICE_NOTIFYW, *PSERVICE_NOTIFYW;
#ifdef UNICODE
typedef SERVICE_NOTIFYW SERVICE_NOTIFY;
typedef PSERVICE_NOTIFYW PSERVICE_NOTIFY;
#else
typedef SERVICE_NOTIFYA SERVICE_NOTIFY;
typedef PSERVICE_NOTIFYA PSERVICE_NOTIFY;
#endif // UNICODE

//
// Service control status reason parameters
//
typedef struct _SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    DWORD                   dwReason;
    LPSTR                   pszComment;
    SERVICE_STATUS_PROCESS  ServiceStatus;
} SERVICE_CONTROL_STATUS_REASON_PARAMSA, *PSERVICE_CONTROL_STATUS_REASON_PARAMSA;
//
// Service control status reason parameters
//
typedef struct _SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    DWORD                   dwReason;
    LPWSTR                  pszComment;
    SERVICE_STATUS_PROCESS  ServiceStatus;
} SERVICE_CONTROL_STATUS_REASON_PARAMSW, *PSERVICE_CONTROL_STATUS_REASON_PARAMSW;
#ifdef UNICODE
typedef SERVICE_CONTROL_STATUS_REASON_PARAMSW SERVICE_CONTROL_STATUS_REASON_PARAMS;
typedef PSERVICE_CONTROL_STATUS_REASON_PARAMSW PSERVICE_CONTROL_STATUS_REASON_PARAMS;
#else
typedef SERVICE_CONTROL_STATUS_REASON_PARAMSA SERVICE_CONTROL_STATUS_REASON_PARAMS;
typedef PSERVICE_CONTROL_STATUS_REASON_PARAMSA PSERVICE_CONTROL_STATUS_REASON_PARAMS;
#endif // UNICODE

//
//  Service start reason
//
typedef struct _SERVICE_START_REASON {
    DWORD                   dwReason;
} SERVICE_START_REASON, *PSERVICE_START_REASON;

///////////////////////////////////////////////////////////////////////////
// API Function Prototypes
///////////////////////////////////////////////////////////////////////////

WINADVAPI
BOOL
WINAPI
ChangeServiceConfigA(
    _In_        SC_HANDLE    hService,
    _In_        DWORD        dwServiceType,
    _In_        DWORD        dwStartType,
    _In_        DWORD        dwErrorControl,
    _In_opt_    LPCSTR     lpBinaryPathName,
    _In_opt_    LPCSTR     lpLoadOrderGroup,
    _Out_opt_   LPDWORD      lpdwTagId,
    _In_opt_    LPCSTR     lpDependencies,
    _In_opt_    LPCSTR     lpServiceStartName,
    _In_opt_    LPCSTR     lpPassword,
    _In_opt_    LPCSTR     lpDisplayName
    );
WINADVAPI
BOOL
WINAPI
ChangeServiceConfigW(
    _In_        SC_HANDLE    hService,
    _In_        DWORD        dwServiceType,
    _In_        DWORD        dwStartType,
    _In_        DWORD        dwErrorControl,
    _In_opt_    LPCWSTR     lpBinaryPathName,
    _In_opt_    LPCWSTR     lpLoadOrderGroup,
    _Out_opt_   LPDWORD      lpdwTagId,
    _In_opt_    LPCWSTR     lpDependencies,
    _In_opt_    LPCWSTR     lpServiceStartName,
    _In_opt_    LPCWSTR     lpPassword,
    _In_opt_    LPCWSTR     lpDisplayName
    );
#ifdef UNICODE
#define ChangeServiceConfig  ChangeServiceConfigW
#else
#define ChangeServiceConfig  ChangeServiceConfigA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
ChangeServiceConfig2A(
    _In_        SC_HANDLE    hService,
    _In_        DWORD        dwInfoLevel,
    _In_opt_    LPVOID       lpInfo
    );
WINADVAPI
BOOL
WINAPI
ChangeServiceConfig2W(
    _In_        SC_HANDLE    hService,
    _In_        DWORD        dwInfoLevel,
    _In_opt_    LPVOID       lpInfo
    );
#ifdef UNICODE
#define ChangeServiceConfig2  ChangeServiceConfig2W
#else
#define ChangeServiceConfig2  ChangeServiceConfig2A
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
CloseServiceHandle(
    _In_        SC_HANDLE   hSCObject
    );

WINADVAPI
BOOL
WINAPI
ControlService(
    _In_        SC_HANDLE           hService,
    _In_        DWORD               dwControl,
    _Out_       LPSERVICE_STATUS    lpServiceStatus
    );

_Must_inspect_result_
WINADVAPI
SC_HANDLE
WINAPI
CreateServiceA(
    _In_        SC_HANDLE    hSCManager,
    _In_        LPCSTR     lpServiceName,
    _In_opt_    LPCSTR     lpDisplayName,
    _In_        DWORD        dwDesiredAccess,
    _In_        DWORD        dwServiceType,
    _In_        DWORD        dwStartType,
    _In_        DWORD        dwErrorControl,
    _In_opt_    LPCSTR     lpBinaryPathName,
    _In_opt_    LPCSTR     lpLoadOrderGroup,
    _Out_opt_   LPDWORD      lpdwTagId,
    _In_opt_    LPCSTR     lpDependencies,
    _In_opt_    LPCSTR     lpServiceStartName,
    _In_opt_    LPCSTR     lpPassword
    );
_Must_inspect_result_
WINADVAPI
SC_HANDLE
WINAPI
CreateServiceW(
    _In_        SC_HANDLE    hSCManager,
    _In_        LPCWSTR     lpServiceName,
    _In_opt_    LPCWSTR     lpDisplayName,
    _In_        DWORD        dwDesiredAccess,
    _In_        DWORD        dwServiceType,
    _In_        DWORD        dwStartType,
    _In_        DWORD        dwErrorControl,
    _In_opt_    LPCWSTR     lpBinaryPathName,
    _In_opt_    LPCWSTR     lpLoadOrderGroup,
    _Out_opt_   LPDWORD      lpdwTagId,
    _In_opt_    LPCWSTR     lpDependencies,
    _In_opt_    LPCWSTR     lpServiceStartName,
    _In_opt_    LPCWSTR     lpPassword
    );
#ifdef UNICODE
#define CreateService  CreateServiceW
#else
#define CreateService  CreateServiceA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
DeleteService(
    _In_        SC_HANDLE   hService
    );

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
EnumDependentServicesA(
    _In_            SC_HANDLE               hService,
    _In_            DWORD                   dwServiceState,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPENUM_SERVICE_STATUSA  lpServices,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded,
    _Out_           LPDWORD                 lpServicesReturned
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
EnumDependentServicesW(
    _In_            SC_HANDLE               hService,
    _In_            DWORD                   dwServiceState,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPENUM_SERVICE_STATUSW  lpServices,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded,
    _Out_           LPDWORD                 lpServicesReturned
    );
#ifdef UNICODE
#define EnumDependentServices  EnumDependentServicesW
#else
#define EnumDependentServices  EnumDependentServicesA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
EnumServicesStatusA(
    _In_            SC_HANDLE               hSCManager,
    _In_            DWORD                   dwServiceType,
    _In_            DWORD                   dwServiceState,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPENUM_SERVICE_STATUSA  lpServices,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded,
    _Out_           LPDWORD                 lpServicesReturned,
    _Inout_opt_     LPDWORD                 lpResumeHandle
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
EnumServicesStatusW(
    _In_            SC_HANDLE               hSCManager,
    _In_            DWORD                   dwServiceType,
    _In_            DWORD                   dwServiceState,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPENUM_SERVICE_STATUSW  lpServices,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded,
    _Out_           LPDWORD                 lpServicesReturned,
    _Inout_opt_     LPDWORD                 lpResumeHandle
    );
#ifdef UNICODE
#define EnumServicesStatus  EnumServicesStatusW
#else
#define EnumServicesStatus  EnumServicesStatusA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
EnumServicesStatusExA(
    _In_            SC_HANDLE               hSCManager,
    _In_            SC_ENUM_TYPE            InfoLevel,
    _In_            DWORD                   dwServiceType,
    _In_            DWORD                   dwServiceState,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPBYTE                  lpServices,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded,
    _Out_           LPDWORD                 lpServicesReturned,
    _Inout_opt_     LPDWORD                 lpResumeHandle,
    _In_opt_        LPCSTR                pszGroupName
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
EnumServicesStatusExW(
    _In_            SC_HANDLE               hSCManager,
    _In_            SC_ENUM_TYPE            InfoLevel,
    _In_            DWORD                   dwServiceType,
    _In_            DWORD                   dwServiceState,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPBYTE                  lpServices,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded,
    _Out_           LPDWORD                 lpServicesReturned,
    _Inout_opt_     LPDWORD                 lpResumeHandle,
    _In_opt_        LPCWSTR                pszGroupName
    );
#ifdef UNICODE
#define EnumServicesStatusEx  EnumServicesStatusExW
#else
#define EnumServicesStatusEx  EnumServicesStatusExA
#endif // !UNICODE

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
GetServiceKeyNameA(
    _In_            SC_HANDLE               hSCManager,
    _In_            LPCSTR                lpDisplayName,
    _Out_writes_opt_(*lpcchBuffer)
                    LPSTR                 lpServiceName,
    _Inout_         LPDWORD                 lpcchBuffer
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
GetServiceKeyNameW(
    _In_            SC_HANDLE               hSCManager,
    _In_            LPCWSTR                lpDisplayName,
    _Out_writes_opt_(*lpcchBuffer)
                    LPWSTR                 lpServiceName,
    _Inout_         LPDWORD                 lpcchBuffer
    );
#ifdef UNICODE
#define GetServiceKeyName  GetServiceKeyNameW
#else
#define GetServiceKeyName  GetServiceKeyNameA
#endif // !UNICODE

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
GetServiceDisplayNameA(
    _In_            SC_HANDLE               hSCManager,
    _In_            LPCSTR                lpServiceName,
    _Out_writes_opt_(*lpcchBuffer)
                    LPSTR                 lpDisplayName,
    _Inout_         LPDWORD                 lpcchBuffer
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
GetServiceDisplayNameW(
    _In_            SC_HANDLE               hSCManager,
    _In_            LPCWSTR                lpServiceName,
    _Out_writes_opt_(*lpcchBuffer)
                    LPWSTR                 lpDisplayName,
    _Inout_         LPDWORD                 lpcchBuffer
    );
#ifdef UNICODE
#define GetServiceDisplayName  GetServiceDisplayNameW
#else
#define GetServiceDisplayName  GetServiceDisplayNameA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINADVAPI
SC_LOCK
WINAPI
LockServiceDatabase(
    _In_            SC_HANDLE               hSCManager
    );

WINADVAPI
BOOL
WINAPI
NotifyBootConfigStatus(
    _In_            BOOL                    BootAcceptable
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Must_inspect_result_
WINADVAPI
SC_HANDLE
WINAPI
OpenSCManagerA(
    _In_opt_        LPCSTR                lpMachineName,
    _In_opt_        LPCSTR                lpDatabaseName,
    _In_            DWORD                   dwDesiredAccess
    );
_Must_inspect_result_
WINADVAPI
SC_HANDLE
WINAPI
OpenSCManagerW(
    _In_opt_        LPCWSTR                lpMachineName,
    _In_opt_        LPCWSTR                lpDatabaseName,
    _In_            DWORD                   dwDesiredAccess
    );
#ifdef UNICODE
#define OpenSCManager  OpenSCManagerW
#else
#define OpenSCManager  OpenSCManagerA
#endif // !UNICODE

_Must_inspect_result_
WINADVAPI
SC_HANDLE
WINAPI
OpenServiceA(
    _In_            SC_HANDLE               hSCManager,
    _In_            LPCSTR                lpServiceName,
    _In_            DWORD                   dwDesiredAccess
    );
_Must_inspect_result_
WINADVAPI
SC_HANDLE
WINAPI
OpenServiceW(
    _In_            SC_HANDLE               hSCManager,
    _In_            LPCWSTR                lpServiceName,
    _In_            DWORD                   dwDesiredAccess
    );
#ifdef UNICODE
#define OpenService  OpenServiceW
#else
#define OpenService  OpenServiceA
#endif // !UNICODE

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceConfigA(
    _In_            SC_HANDLE               hService,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPQUERY_SERVICE_CONFIGA lpServiceConfig,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceConfigW(
    _In_            SC_HANDLE               hService,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPQUERY_SERVICE_CONFIGW lpServiceConfig,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded
    );
#ifdef UNICODE
#define QueryServiceConfig  QueryServiceConfigW
#else
#define QueryServiceConfig  QueryServiceConfigA
#endif // !UNICODE

_When_(dwInfoLevel == SERVICE_CONFIG_DESCRIPTION,
       _At_(cbBufSize, _In_range_(>=, sizeof(LPSERVICE_DESCRIPTIONA))))
_When_(dwInfoLevel == SERVICE_CONFIG_FAILURE_ACTIONS,
       _At_(cbBufSize, _In_range_(>=, sizeof(LPSERVICE_FAILURE_ACTIONSA))))
_When_(dwInfoLevel == SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO,
       _At_(cbBufSize, _In_range_(>=,
                                  sizeof(LPSERVICE_REQUIRED_PRIVILEGES_INFOA))))
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceConfig2A(
    _In_            SC_HANDLE               hService,
    _In_            DWORD                   dwInfoLevel,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPBYTE                  lpBuffer,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded
    );
_When_(dwInfoLevel == SERVICE_CONFIG_DESCRIPTION,
       _At_(cbBufSize, _In_range_(>=, sizeof(LPSERVICE_DESCRIPTIONW))))
_When_(dwInfoLevel == SERVICE_CONFIG_FAILURE_ACTIONS,
       _At_(cbBufSize, _In_range_(>=, sizeof(LPSERVICE_FAILURE_ACTIONSW))))
_When_(dwInfoLevel == SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO,
       _At_(cbBufSize, _In_range_(>=,
                                  sizeof(LPSERVICE_REQUIRED_PRIVILEGES_INFOW))))
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceConfig2W(
    _In_            SC_HANDLE               hService,
    _In_            DWORD                   dwInfoLevel,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPBYTE                  lpBuffer,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded
    );
#ifdef UNICODE
#define QueryServiceConfig2  QueryServiceConfig2W
#else
#define QueryServiceConfig2  QueryServiceConfig2A
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceLockStatusA(
    _In_            SC_HANDLE                       hSCManager,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPQUERY_SERVICE_LOCK_STATUSA    lpLockStatus,
    _In_            DWORD                           cbBufSize,
    _Out_           LPDWORD                         pcbBytesNeeded
    );
_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceLockStatusW(
    _In_            SC_HANDLE                       hSCManager,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPQUERY_SERVICE_LOCK_STATUSW    lpLockStatus,
    _In_            DWORD                           cbBufSize,
    _Out_           LPDWORD                         pcbBytesNeeded
    );
#ifdef UNICODE
#define QueryServiceLockStatus  QueryServiceLockStatusW
#else
#define QueryServiceLockStatus  QueryServiceLockStatusA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceObjectSecurity(
    _In_            SC_HANDLE               hService,
    _In_            SECURITY_INFORMATION    dwSecurityInformation,
    _Out_writes_bytes_opt_(cbBufSize)
                    PSECURITY_DESCRIPTOR    lpSecurityDescriptor,
    _In_            DWORD                   cbBufSize,
    _Out_           LPDWORD                 pcbBytesNeeded
    );

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceStatus(
    _In_            SC_HANDLE           hService,
    _Out_           LPSERVICE_STATUS    lpServiceStatus
    );

_Must_inspect_result_
WINADVAPI
BOOL
WINAPI
QueryServiceStatusEx(
    _In_            SC_HANDLE           hService,
    _In_            SC_STATUS_TYPE      InfoLevel,
    _Out_writes_bytes_opt_(cbBufSize)
                    LPBYTE              lpBuffer,
    _In_            DWORD               cbBufSize,
    _Out_           LPDWORD             pcbBytesNeeded
    );

_Must_inspect_result_
WINADVAPI
SERVICE_STATUS_HANDLE
WINAPI
RegisterServiceCtrlHandlerA(
    _In_    LPCSTR                    lpServiceName,
    _In_    __callback
            LPHANDLER_FUNCTION          lpHandlerProc
    );
_Must_inspect_result_
WINADVAPI
SERVICE_STATUS_HANDLE
WINAPI
RegisterServiceCtrlHandlerW(
    _In_    LPCWSTR                    lpServiceName,
    _In_    __callback
            LPHANDLER_FUNCTION          lpHandlerProc
    );
#ifdef UNICODE
#define RegisterServiceCtrlHandler  RegisterServiceCtrlHandlerW
#else
#define RegisterServiceCtrlHandler  RegisterServiceCtrlHandlerA
#endif // !UNICODE

_Must_inspect_result_
WINADVAPI
SERVICE_STATUS_HANDLE
WINAPI
RegisterServiceCtrlHandlerExA(
    _In_    LPCSTR                    lpServiceName,
    _In_    __callback
            LPHANDLER_FUNCTION_EX       lpHandlerProc,
    _In_opt_ LPVOID                     lpContext
    );
_Must_inspect_result_
WINADVAPI
SERVICE_STATUS_HANDLE
WINAPI
RegisterServiceCtrlHandlerExW(
    _In_    LPCWSTR                    lpServiceName,
    _In_    __callback
            LPHANDLER_FUNCTION_EX       lpHandlerProc,
    _In_opt_ LPVOID                     lpContext
    );
#ifdef UNICODE
#define RegisterServiceCtrlHandlerEx  RegisterServiceCtrlHandlerExW
#else
#define RegisterServiceCtrlHandlerEx  RegisterServiceCtrlHandlerExA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
SetServiceObjectSecurity(
    _In_        SC_HANDLE               hService,
    _In_        SECURITY_INFORMATION    dwSecurityInformation,
    _In_        PSECURITY_DESCRIPTOR    lpSecurityDescriptor
    );

WINADVAPI
BOOL
WINAPI
SetServiceStatus(
    _In_        SERVICE_STATUS_HANDLE   hServiceStatus,
    _In_        LPSERVICE_STATUS        lpServiceStatus
    );

WINADVAPI
BOOL
WINAPI
StartServiceCtrlDispatcherA(
    _In_ CONST  SERVICE_TABLE_ENTRYA    *lpServiceStartTable
    );
WINADVAPI
BOOL
WINAPI
StartServiceCtrlDispatcherW(
    _In_ CONST  SERVICE_TABLE_ENTRYW    *lpServiceStartTable
    );
#ifdef UNICODE
#define StartServiceCtrlDispatcher  StartServiceCtrlDispatcherW
#else
#define StartServiceCtrlDispatcher  StartServiceCtrlDispatcherA
#endif // !UNICODE


WINADVAPI
BOOL
WINAPI
StartServiceA(
    _In_            SC_HANDLE            hService,
    _In_            DWORD                dwNumServiceArgs,
    _In_reads_opt_(dwNumServiceArgs)
                    LPCSTR             *lpServiceArgVectors
    );
WINADVAPI
BOOL
WINAPI
StartServiceW(
    _In_            SC_HANDLE            hService,
    _In_            DWORD                dwNumServiceArgs,
    _In_reads_opt_(dwNumServiceArgs)
                    LPCWSTR             *lpServiceArgVectors
    );
#ifdef UNICODE
#define StartService  StartServiceW
#else
#define StartService  StartServiceA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINADVAPI
BOOL
WINAPI
UnlockServiceDatabase(
    _In_            SC_LOCK             ScLock
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_VISTA)

WINADVAPI
DWORD
WINAPI
NotifyServiceStatusChangeA (
    _In_        SC_HANDLE               hService,
    _In_        DWORD                   dwNotifyMask,
    _In_        PSERVICE_NOTIFYA        pNotifyBuffer
    );
WINADVAPI
DWORD
WINAPI
NotifyServiceStatusChangeW (
    _In_        SC_HANDLE               hService,
    _In_        DWORD                   dwNotifyMask,
    _In_        PSERVICE_NOTIFYW        pNotifyBuffer
    );
#ifdef UNICODE
#define NotifyServiceStatusChange  NotifyServiceStatusChangeW
#else
#define NotifyServiceStatusChange  NotifyServiceStatusChangeA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
ControlServiceExA(
    _In_        SC_HANDLE               hService,
    _In_        DWORD                   dwControl,
    _In_        DWORD                   dwInfoLevel,
    _Inout_     PVOID                   pControlParams
    );
WINADVAPI
BOOL
WINAPI
ControlServiceExW(
    _In_        SC_HANDLE               hService,
    _In_        DWORD                   dwControl,
    _In_        DWORD                   dwInfoLevel,
    _Inout_     PVOID                   pControlParams
    );
#ifdef UNICODE
#define ControlServiceEx  ControlServiceExW
#else
#define ControlServiceEx  ControlServiceExA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
QueryServiceDynamicInformation (
    _In_        SERVICE_STATUS_HANDLE   hServiceStatus,
    _In_        DWORD                   dwInfoLevel,
    _Outptr_    PVOID           *       ppDynamicInfo
    );

#endif // NTDDI_VERSION >= NTDDI_VISTA

#if (NTDDI_VERSION >= NTDDI_WIN8)

//
// Service status change notification API
//

typedef enum _SC_EVENT_TYPE {
    SC_EVENT_DATABASE_CHANGE,
    SC_EVENT_PROPERTY_CHANGE,
    SC_EVENT_STATUS_CHANGE
} SC_EVENT_TYPE, *PSC_EVENT_TYPE;

typedef
VOID
CALLBACK
SC_NOTIFICATION_CALLBACK (
    _In_        DWORD                   dwNotify,
    _In_opt_    PVOID                   pCallbackContext
    );
typedef SC_NOTIFICATION_CALLBACK* PSC_NOTIFICATION_CALLBACK;

typedef struct _SC_NOTIFICATION_REGISTRATION* PSC_NOTIFICATION_REGISTRATION;

WINADVAPI
DWORD
WINAPI
SubscribeServiceChangeNotifications (
    _In_      SC_HANDLE                      hService,
    _In_      SC_EVENT_TYPE                  eEventType,
    _In_      PSC_NOTIFICATION_CALLBACK      pCallback,
    _In_opt_  PVOID                          pCallbackContext,
    _Out_     PSC_NOTIFICATION_REGISTRATION* pSubscription
    );

WINADVAPI
VOID
WINAPI
UnsubscribeServiceChangeNotifications (
    _In_      PSC_NOTIFICATION_REGISTRATION pSubscription
    );

WINADVAPI
DWORD
WINAPI
WaitServiceState (
    _In_      SC_HANDLE  hService,
    _In_      DWORD      dwNotify,
    _In_opt_  DWORD      dwTimeout,
    _In_opt_  HANDLE     hCancelEvent
    );

#endif // NTDDI_VERSION >= NTDDI_WIN8

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// Service state type enums
//

typedef enum SERVICE_REGISTRY_STATE_TYPE {
    ServiceRegistryStateParameters = 0,
    ServiceRegistryStatePersistent = 1,
    MaxServiceRegistryStateType = 2,
} SERVICE_REGISTRY_STATE_TYPE;

_Must_inspect_result_
DWORD
WINAPI
GetServiceRegistryStateKey(
    _In_ SERVICE_STATUS_HANDLE ServiceStatusHandle,
    _In_ SERVICE_REGISTRY_STATE_TYPE StateType,
    _In_ DWORD AccessMask,
    _Out_ HKEY *ServiceStateKey
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS4

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

typedef enum SERVICE_DIRECTORY_TYPE {
    ServiceDirectoryPersistentState = 0,
    ServiceDirectoryTypeMax = 1,
} SERVICE_DIRECTORY_TYPE;

_Must_inspect_result_
DWORD
WINAPI
GetServiceDirectory(
    _In_ SERVICE_STATUS_HANDLE hServiceStatus,
    _In_ SERVICE_DIRECTORY_TYPE eDirectoryType,
    _Out_writes_opt_(cchPathBufferLength) PWCHAR lpPathBuffer,
    _In_ DWORD cchPathBufferLength,
    _Out_ DWORD *lpcchRequiredBufferLength
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS5

#if (NTDDI_VERSION >= NTDDI_WIN10_MN)

typedef enum SERVICE_SHARED_REGISTRY_STATE_TYPE {
    ServiceSharedRegistryPersistentState = 0
} SERVICE_SHARED_REGISTRY_STATE_TYPE;

_Must_inspect_result_
DWORD
WINAPI
GetSharedServiceRegistryStateKey(
    _In_ SC_HANDLE ServiceHandle,
    _In_ SERVICE_SHARED_REGISTRY_STATE_TYPE StateType,
    _In_ DWORD AccessMask,
    _Out_ HKEY *ServiceStateKey
    );

typedef enum SERVICE_SHARED_DIRECTORY_TYPE {
    ServiceSharedDirectoryPersistentState = 0
} SERVICE_SHARED_DIRECTORY_TYPE;

_Must_inspect_result_
DWORD
WINAPI
GetSharedServiceDirectory(
    _In_ SC_HANDLE ServiceHandle,
    _In_ SERVICE_SHARED_DIRECTORY_TYPE DirectoryType,
    _Out_writes_to_opt_(PathBufferLength, *RequiredBufferLength) PWCHAR PathBuffer,
    _In_ DWORD PathBufferLength,
    _Out_ DWORD *RequiredBufferLength
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_MN

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // _WINSVC_

