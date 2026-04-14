/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    cfg.h

Abstract:

    This module contains the common Configuration Manager definitions for
    both user mode and kernel mode code.

Revision History:

--*/

#ifndef _CFG_INCLUDED_
#define _CFG_INCLUDED_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#ifdef _MSC_VER
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) /* #if not_defined treated as #if 0 */
#endif
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// The following definitions are also used by kernel mode code to
// set up the registry.
//
//
// VetoType used in
//      CM_Disable_DevNode
//      CM_Uninstall_DevNode
//      CM_Query_And_Remove_SubTree
//
typedef enum _PNP_VETO_TYPE {
    PNP_VetoTypeUnknown,            // Name is unspecified
    PNP_VetoLegacyDevice,           // Name is an Instance Path
    PNP_VetoPendingClose,           // Name is an Instance Path
    PNP_VetoWindowsApp,             // Name is a Module
    PNP_VetoWindowsService,         // Name is a Service
    PNP_VetoOutstandingOpen,        // Name is an Instance Path
    PNP_VetoDevice,                 // Name is an Instance Path
    PNP_VetoDriver,                 // Name is a Driver Service Name
    PNP_VetoIllegalDeviceRequest,   // Name is an Instance Path
    PNP_VetoInsufficientPower,      // Name is unspecified
    PNP_VetoNonDisableable,         // Name is an Instance Path
    PNP_VetoLegacyDriver,           // Name is a Service
    PNP_VetoInsufficientRights,     // Name is unspecified
    PNP_VetoAlreadyRemoved,         // Name is unspecified
} PNP_VETO_TYPE, *PPNP_VETO_TYPE;


//
// DevInst problem values, returned by call to CM_Get_DevInst_Status
//
#define CM_PROB_NOT_CONFIGURED             (0x00000001)   // no config for device
#define CM_PROB_DEVLOADER_FAILED           (0x00000002)   // service load failed
#define CM_PROB_OUT_OF_MEMORY              (0x00000003)   // out of memory
#define CM_PROB_ENTRY_IS_WRONG_TYPE        (0x00000004)   //
#define CM_PROB_LACKED_ARBITRATOR          (0x00000005)   //
#define CM_PROB_BOOT_CONFIG_CONFLICT       (0x00000006)   // boot config conflict
#define CM_PROB_FAILED_FILTER              (0x00000007)   //
#define CM_PROB_DEVLOADER_NOT_FOUND        (0x00000008)   // Devloader not found
#define CM_PROB_INVALID_DATA               (0x00000009)   // Invalid ID
#define CM_PROB_FAILED_START               (0x0000000A)   //
#define CM_PROB_LIAR                       (0x0000000B)   //
#define CM_PROB_NORMAL_CONFLICT            (0x0000000C)   // config conflict
#define CM_PROB_NOT_VERIFIED               (0x0000000D)   //
#define CM_PROB_NEED_RESTART               (0x0000000E)   // requires restart
#define CM_PROB_REENUMERATION              (0x0000000F)   //
#define CM_PROB_PARTIAL_LOG_CONF           (0x00000010)   //
#define CM_PROB_UNKNOWN_RESOURCE           (0x00000011)   // unknown res type
#define CM_PROB_REINSTALL                  (0x00000012)   //
#define CM_PROB_REGISTRY                   (0x00000013)   //
#define CM_PROB_VXDLDR                     (0x00000014)   // WINDOWS 95 ONLY
#define CM_PROB_WILL_BE_REMOVED            (0x00000015)   // devinst will remove
#define CM_PROB_DISABLED                   (0x00000016)   // devinst is disabled
#define CM_PROB_DEVLOADER_NOT_READY        (0x00000017)   // Devloader not ready
#define CM_PROB_DEVICE_NOT_THERE           (0x00000018)   // device doesn't exist
#define CM_PROB_MOVED                      (0x00000019)   //
#define CM_PROB_TOO_EARLY                  (0x0000001A)   //
#define CM_PROB_NO_VALID_LOG_CONF          (0x0000001B)   // no valid log config
#define CM_PROB_FAILED_INSTALL             (0x0000001C)   // install failed
#define CM_PROB_HARDWARE_DISABLED          (0x0000001D)   // device disabled
#define CM_PROB_CANT_SHARE_IRQ             (0x0000001E)   // can't share IRQ
#define CM_PROB_FAILED_ADD                 (0x0000001F)   // driver failed add
#define CM_PROB_DISABLED_SERVICE           (0x00000020)   // service's Start = 4
#define CM_PROB_TRANSLATION_FAILED         (0x00000021)   // resource translation failed
#define CM_PROB_NO_SOFTCONFIG              (0x00000022)   // no soft config
#define CM_PROB_BIOS_TABLE                 (0x00000023)   // device missing in BIOS table
#define CM_PROB_IRQ_TRANSLATION_FAILED     (0x00000024)   // IRQ translator failed
#define CM_PROB_FAILED_DRIVER_ENTRY        (0x00000025)   // DriverEntry() failed.
#define CM_PROB_DRIVER_FAILED_PRIOR_UNLOAD (0x00000026)   // Driver should have unloaded.
#define CM_PROB_DRIVER_FAILED_LOAD         (0x00000027)   // Driver load unsuccessful.
#define CM_PROB_DRIVER_SERVICE_KEY_INVALID (0x00000028)   // Error accessing driver's service key
#define CM_PROB_LEGACY_SERVICE_NO_DEVICES  (0x00000029)   // Loaded legacy service created no devices
#define CM_PROB_DUPLICATE_DEVICE           (0x0000002A)   // Two devices were discovered with the same name
#define CM_PROB_FAILED_POST_START          (0x0000002B)   // The drivers set the device state to failed
#define CM_PROB_HALTED                     (0x0000002C)   // This device was failed post start via usermode
#define CM_PROB_PHANTOM                    (0x0000002D)   // The devinst currently exists only in the registry
#define CM_PROB_SYSTEM_SHUTDOWN            (0x0000002E)   // The system is shutting down
#define CM_PROB_HELD_FOR_EJECT             (0x0000002F)   // The device is offline awaiting removal
#define CM_PROB_DRIVER_BLOCKED             (0x00000030)   // One or more drivers is blocked from loading
#define CM_PROB_REGISTRY_TOO_LARGE         (0x00000031)   // System hive has grown too large
#define CM_PROB_SETPROPERTIES_FAILED       (0x00000032)   // Failed to apply one or more registry properties  
#define CM_PROB_WAITING_ON_DEPENDENCY      (0x00000033)   // Device is stalled waiting on a dependency to start
#define CM_PROB_UNSIGNED_DRIVER            (0x00000034)   // Failed load driver due to unsigned image.
#define CM_PROB_USED_BY_DEBUGGER           (0x00000035)   // Device is being used by kernel debugger
#define CM_PROB_DEVICE_RESET               (0x00000036)   // Device is being reset
#define CM_PROB_CONSOLE_LOCKED             (0x00000037)   // Device is blocked while console is locked
#define CM_PROB_NEED_CLASS_CONFIG          (0x00000038)   // Device needs extended class configuration to start
#define CM_PROB_GUEST_ASSIGNMENT_FAILED    (0x00000039)   // Assignment to guest partition failed

#define NUM_CM_PROB_V1                     (0x00000025)
#define NUM_CM_PROB_V2                     (0x00000032)
#define NUM_CM_PROB_V3                     (0x00000033)
#define NUM_CM_PROB_V4                     (0x00000034)
#define NUM_CM_PROB_V5                     (0x00000035)
#define NUM_CM_PROB_V6                     (0x00000036)
#define NUM_CM_PROB_V7                     (0x00000037)
#define NUM_CM_PROB_V8                     (0x00000039)
#define NUM_CM_PROB_V9                     (0x0000003A)

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

#define NUM_CM_PROB NUM_CM_PROB_V9

#elif (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#define NUM_CM_PROB NUM_CM_PROB_V8

#elif (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define NUM_CM_PROB NUM_CM_PROB_V7

#elif (NTDDI_VERSION >= NTDDI_WINBLUE)

#define NUM_CM_PROB NUM_CM_PROB_V6

#elif (NTDDI_VERSION >= NTDDI_WIN7)

#define NUM_CM_PROB NUM_CM_PROB_V5

#elif (NTDDI_VERSION >= NTDDI_WS08)

#define NUM_CM_PROB NUM_CM_PROB_V4

#elif (NTDDI_VERSION >= NTDDI_WS03)

#define NUM_CM_PROB NUM_CM_PROB_V3

#elif (NTDDI_VERSION >= NTDDI_WINXP)

#define NUM_CM_PROB NUM_CM_PROB_V2

#elif (NTDDI_VERSION >= WIN2K)

#define NUM_CM_PROB NUM_CM_PROB_V1

#endif

//
// Device Instance status flags, returned by call to CM_Get_DevInst_Status
//
#define DN_ROOT_ENUMERATED (0x00000001) // Was enumerated by ROOT
#define DN_DRIVER_LOADED   (0x00000002) // Has Register_Device_Driver
#define DN_ENUM_LOADED     (0x00000004) // Has Register_Enumerator
#define DN_STARTED         (0x00000008) // Is currently configured
#define DN_MANUAL          (0x00000010) // Manually installed
#define DN_NEED_TO_ENUM    (0x00000020) // May need reenumeration
#define DN_NOT_FIRST_TIME  (0x00000040) // Has received a config
#define DN_HARDWARE_ENUM   (0x00000080) // Enum generates hardware ID
#define DN_LIAR            (0x00000100) // Lied about can reconfig once
#define DN_HAS_MARK        (0x00000200) // Not CM_Create_DevInst lately
#define DN_HAS_PROBLEM     (0x00000400) // Need device installer
#define DN_FILTERED        (0x00000800) // Is filtered
#define DN_MOVED           (0x00001000) // Has been moved
#define DN_DISABLEABLE     (0x00002000) // Can be disabled
#define DN_REMOVABLE       (0x00004000) // Can be removed
#define DN_PRIVATE_PROBLEM (0x00008000) // Has a private problem
#define DN_MF_PARENT       (0x00010000) // Multi function parent
#define DN_MF_CHILD        (0x00020000) // Multi function child
#define DN_WILL_BE_REMOVED (0x00040000) // DevInst is being removed

//
// Windows 4 OPK2 Flags
//
#define DN_NOT_FIRST_TIMEE  0x00080000  // S: Has received a config enumerate
#define DN_STOP_FREE_RES    0x00100000  // S: When child is stopped, free resources
#define DN_REBAL_CANDIDATE  0x00200000  // S: Don't skip during rebalance
#define DN_BAD_PARTIAL      0x00400000  // S: This devnode's log_confs do not have same resources
#define DN_NT_ENUMERATOR    0x00800000  // S: This devnode's is an NT enumerator
#define DN_NT_DRIVER        0x01000000  // S: This devnode's is an NT driver
//
// Windows 4.1 Flags
//
#define DN_NEEDS_LOCKING    0x02000000  // S: Devnode need lock resume processing
#define DN_ARM_WAKEUP       0x04000000  // S: Devnode can be the wakeup device
#define DN_APM_ENUMERATOR   0x08000000  // S: APM aware enumerator
#define DN_APM_DRIVER       0x10000000  // S: APM aware driver
#define DN_SILENT_INSTALL   0x20000000  // S: Silent install
#define DN_NO_SHOW_IN_DM    0x40000000  // S: No show in device manager
#define DN_BOOT_LOG_PROB    0x80000000  // S: Had a problem during preassignment of boot log conf

//
// Windows NT Flags
//
// These are overloaded on top of unused Win 9X flags
//

#if (NTDDI_VERSION >= NTDDI_WIN2K)

#define DN_NEED_RESTART                 DN_LIAR                 // System needs to be restarted for this Devnode to work properly

#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)

#define DN_DRIVER_BLOCKED               DN_NOT_FIRST_TIME       // One or more drivers are blocked from loading for this Devnode
#define DN_LEGACY_DRIVER                DN_MOVED                // This device is using a legacy driver
#define DN_CHILD_WITH_INVALID_ID        DN_HAS_MARK             // One or more children have invalid ID(s)

#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define DN_DEVICE_DISCONNECTED          DN_NEEDS_LOCKING        // The function driver for a device reported that the device is not connected.  Typically this means a wireless device is out of range.

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10)

#define DN_QUERY_REMOVE_PENDING         DN_MF_PARENT            // Device is part of a set of related devices collectively pending query-removal
#define DN_QUERY_REMOVE_ACTIVE          DN_MF_CHILD             // Device is actively engaged in a query-remove IRP

#endif

#define DN_CHANGEABLE_FLAGS (DN_NOT_FIRST_TIME+\
                DN_HARDWARE_ENUM+\
                DN_HAS_MARK+\
                DN_DISABLEABLE+\
                DN_REMOVABLE+\
                DN_MF_CHILD+\
                DN_MF_PARENT+\
                DN_NOT_FIRST_TIMEE+\
                DN_STOP_FREE_RES+\
                DN_REBAL_CANDIDATE+\
                DN_NT_ENUMERATOR+\
                DN_NT_DRIVER+\
                DN_SILENT_INSTALL+\
                DN_NO_SHOW_IN_DM)

//
// Logical configuration Priority values
//
// These priority values are used in user-mode calls to CM_Add_Empty_Log_Conf.
// Drivers may also specify priority values for a given IO_RESOURCE_LIST
// structure by including a ConfigData member union as the first
// IO_RESOURCE_DESCRIPTOR in the IO_RESOURCE_LIST. In this case, the descriptor
// type would be CmResourceTypeConfigData.
//
#define LCPRI_FORCECONFIG     (0x00000000) // Coming from a forced config
#define LCPRI_BOOTCONFIG      (0x00000001) // Coming from a boot config
#define LCPRI_DESIRED         (0x00002000) // Preferable (better performance)
#define LCPRI_NORMAL          (0x00003000) // Workable (acceptable performance)
#define LCPRI_LASTBESTCONFIG  (0x00003FFF) // CM only--do not use
#define LCPRI_SUBOPTIMAL      (0x00005000) // Not desired, but will work
#define LCPRI_LASTSOFTCONFIG  (0x00007FFF) // CM only--do not use
#define LCPRI_RESTART         (0x00008000) // Need to restart
#define LCPRI_REBOOT          (0x00009000) // Need to reboot
#define LCPRI_POWEROFF        (0x0000A000) // Need to shutdown/power-off
#define LCPRI_HARDRECONFIG    (0x0000C000) // Need to change a jumper
#define LCPRI_HARDWIRED       (0x0000E000) // Cannot be changed
#define LCPRI_IMPOSSIBLE      (0x0000F000) // Impossible configuration
#define LCPRI_DISABLED        (0x0000FFFF) // Disabled configuration
#define MAX_LCPRI             (0x0000FFFF) // Maximum known LC Priority

//
// Device panel sides
//
#define CM_DEVICE_PANEL_SIDE_UNKNOWN            (0x00000000)
#define CM_DEVICE_PANEL_SIDE_TOP                (0x00000001)
#define CM_DEVICE_PANEL_SIDE_BOTTOM             (0x00000002)
#define CM_DEVICE_PANEL_SIDE_LEFT               (0x00000003)
#define CM_DEVICE_PANEL_SIDE_RIGHT              (0x00000004)
#define CM_DEVICE_PANEL_SIDE_FRONT              (0x00000005)
#define CM_DEVICE_PANEL_SIDE_BACK               (0x00000006)

//
// Device panel edges
//
#define CM_DEVICE_PANEL_EDGE_UNKNOWN            (0x00000000)
#define CM_DEVICE_PANEL_EDGE_TOP                (0x00000001)
#define CM_DEVICE_PANEL_EDGE_BOTTOM             (0x00000002)
#define CM_DEVICE_PANEL_EDGE_LEFT               (0x00000003)
#define CM_DEVICE_PANEL_EDGE_RIGHT              (0x00000004)
                                        
//
// Device panel shapes
//
#define CM_DEVICE_PANEL_SHAPE_UNKNOWN           (0x00000000)
#define CM_DEVICE_PANEL_SHAPE_RECTANGLE         (0x00000001)
#define CM_DEVICE_PANEL_SHAPE_OVAL              (0x00000002)

//
// Device panel orientations
//
#define CM_DEVICE_PANEL_ORIENTATION_HORIZONTAL  (0x00000000)
#define CM_DEVICE_PANEL_ORIENTATION_VERTICAL    (0x00000001)

//
// Device panel joint types
//
#define CM_DEVICE_PANEL_JOINT_TYPE_UNKNOWN      (0x00000000)
#define CM_DEVICE_PANEL_JOINT_TYPE_PLANAR       (0x00000001)
#define CM_DEVICE_PANEL_JOINT_TYPE_HINGE        (0x00000002)
#define CM_DEVICE_PANEL_JOINT_TYPE_PIVOT        (0x00000003)
#define CM_DEVICE_PANEL_JOINT_TYPE_SWIVEL       (0x00000004)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef _MSC_VER
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
#endif

#endif // _CFG_INCLUDED_

