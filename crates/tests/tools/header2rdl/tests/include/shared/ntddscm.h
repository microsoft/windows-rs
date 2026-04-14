/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddscm.h

Abstract:

    This is the include file that defines common constants, types and control codes
    related to persistent memory, also known as storage-class memory (SCM).
    
    There are three types of persistent memory device objects.
    
    Physical persistent memory devices are the actual hardware that enable persistent memory
    to work. In Windows, their corresponding device objects can be used for health monitoring,
    firmware upgrade, secure erase etc., but they can't be used to read or write data. IOCTLs
    sent to physical persistent memory devices, or types that represent information about them,
    have the abbreviation "SCM_PD".
    
    Logical persistent memory devices are formed by combining one or more physical devices into
    a logical object that can perform reads and writes. A logical persistent memory device corresponds
    to a disk, and the usual disk interfaces should be used to query its health, do I/O and so on.
    The abbreviation that identifies IOCTLs and structures directed to logical persistent memory devices
    is "SCM_LD".
    
    Finally, the SCM bus driver is responsible for enumerating all physical and logical persistent memory
    devices on the system. The IOCTLs and structures related to it have "SCM_BUS" in their names.


--*/

#include <winapifamily.h>
#include <devioctl.h>

#ifdef __cplusplus
extern "C" {
#endif


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) 

//
// Interface GUIDs
//
// need these GUIDs outside conditional includes so that user can
//   #include <ntddscm.h> in precompiled header
//   #include <initguid.h> in a single source file
//   #include <ntddscm.h> in that source file a second time to instantiate the GUIDs
//
#ifdef DEFINE_GUID

//
// Supported device interfaces
//

// begin_wioctlguids

//
// This interface represents a physical persistent memory device, such as an NVDIMM.
// {4283609D-4DC2-43BE-BBB4-4F15DFCE2C61}
//
DEFINE_GUID(GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE, 0x4283609d, 0x4dc2, 0x43be, 0xbb, 0xb4, 0x4f, 0x15, 0xdf, 0xce, 0x2c, 0x61);

//
// When a physical device driver detects a change in the health status of a physical device,
// it triggers a PNP custom event (through TARGET_DEVICE_CUSTOM_NOTIFICATION) to alert any
// registered components. The custom event's GUID is GUID_SCM_PD_HEALTH_NOTIFICATION
// and its payload is SCM_PD_HEALTH_NOTIFICATION_DATA
// {9DA2D386-72F5-4EE3-8155-ECA0678E3B06}
//
DEFINE_GUID(GUID_SCM_PD_HEALTH_NOTIFICATION, 0x9da2d386, 0x72f5, 0x4ee3, 0x81, 0x55, 0xec, 0xa0, 0x67, 0x8e, 0x3b, 0x6);

//
// The passthrough protocol GUID for INVDIMM devices. The application and the driver use this value
// for the "ProtocolGuid" field of the SCM_PD_PASSTHROUGH_INPUT and SCM_PD_PASSTHROUGH_OUTPUT structures.
// {4309AC30-0D11-11E4-9191-0800200C9A66}
//
DEFINE_GUID(GUID_SCM_PD_PASSTHROUGH_INVDIMM, 0x4309AC30, 0x0D11, 0x11E4, 0x91, 0x91, 0x08, 0x00, 0x20, 0x0C, 0x9A, 0x66);

// end_wioctlguids

#endif // DEFINE_GUID

// begin_winioctl

#ifndef _NTDDSCM_H_
#define _NTDDSCM_H_

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/union
#pragma warning(disable:4214) // bit field types other than int


#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

//
// Functions 0 to 0x2FF are reserved for the bus device.
// Functions 0x300 to 0x5FF are reserved for the logical persistent memory device.
// Functions 0x600 to 0x7FF are reserved for the physical persistent memory device.
// Functions 0x800 and above are reserved for non-Microsoft users.
//

#define IOCTL_SCMBUS_BASE FILE_DEVICE_PERSISTENT_MEMORY

#define IOCTL_SCMBUS_DEVICE_FUNCTION_BASE           0x0
#define IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE      0x300
#define IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE     0x600

#define SCMBUS_FUNCTION(x)              (IOCTL_SCMBUS_DEVICE_FUNCTION_BASE + x)
#define SCM_LOGICAL_DEVICE_FUNCTION(x)  (IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE + x)
#define SCM_PHYSICAL_DEVICE_FUNCTION(x) (IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE + x)

//
// Persistent memory (SCM) bus device IOCTLs.
//

#define IOCTL_SCM_BUS_GET_LOGICAL_DEVICES           CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x00), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES          CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x01), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_GET_REGIONS                   CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x02), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_QUERY_PROPERTY                CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x03), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_SET_PROPERTY                  CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x05), METHOD_BUFFERED, FILE_WRITE_ACCESS)

//
// This IOCTL does not require any input nor produce any output data.
//
#define IOCTL_SCM_BUS_RUNTIME_FW_ACTIVATE           CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x04), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_BUS_REFRESH_NAMESPACE             CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x06), METHOD_BUFFERED, FILE_ANY_ACCESS)


//
// Logical persistent memory device IOCTLs.
//
#define IOCTL_SCM_LD_GET_INTERLEAVE_SET             CTL_CODE(IOCTL_SCMBUS_BASE, SCM_LOGICAL_DEVICE_FUNCTION(0x00), METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTLs exposed by physical persistent memory device objects.
//

#define IOCTL_SCM_PD_QUERY_PROPERTY                 CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x00), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_PD_FIRMWARE_DOWNLOAD              CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x01), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_FIRMWARE_ACTIVATE              CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x02), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_PASSTHROUGH                    CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x03), METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_UPDATE_MANAGEMENT_STATUS       CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x04), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_PD_REINITIALIZE_MEDIA             CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x05), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_SET_PROPERTY                   CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x06), METHOD_BUFFERED, FILE_WRITE_ACCESS)


//
// The payload for a physical device health notification.
//
typedef struct _SCM_PD_HEALTH_NOTIFICATION_DATA {

    //
    // The GUID of the device reporting the health change.
    // This is the same GUID returned by IOCTL_SCM_PD_QUERY_PROPERTY with
    // ScmPhysicalDeviceProperty_DeviceInfo.
    //
    GUID  DeviceGuid;

} SCM_PD_HEALTH_NOTIFICATION_DATA, *PSCM_PD_HEALTH_NOTIFICATION_DATA;


//
// IOCTL_SCM_BUS_GET_LOGICAL_DEVICES
//
// Send this IOCTL to the ScmBus adapter to get a list of all the logical persistent memory
// devices on the system.
//
// Input Buffer:
//      None
//
// Output Buffer:
//      SCM_LOGICAL_DEVICES
//

#define SCM_MAX_SYMLINK_LEN_IN_CHARS 256

typedef struct _SCM_LOGICAL_DEVICE_INSTANCE {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure.
    //
    ULONG Size;

    //
    // The logical device GUID.
    //
    GUID DeviceGuid;

    //
    // Symbolic link that can be used to get a handle to the device.
    //
    WCHAR SymbolicLink[SCM_MAX_SYMLINK_LEN_IN_CHARS];

} SCM_LOGICAL_DEVICE_INSTANCE, *PSCM_LOGICAL_DEVICE_INSTANCE;

typedef struct _SCM_LOGICAL_DEVICES {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure, including all the elements in the
    // Devices array.
    //
    ULONG Size;

    //
    // The number of valid elements in the Devices array.
    //
    ULONG DeviceCount;

    //
    // Array of logical device instances.
    //
    SCM_LOGICAL_DEVICE_INSTANCE Devices[ANYSIZE_ARRAY];

} SCM_LOGICAL_DEVICES, *PSCM_LOGICAL_DEVICES;

//
// IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES
//
// Send this IOCTL to the ScmBus adapter to get a list of all the physical persistent memory
// devices on the system.
//
// Input Buffer:
//      None
//
// Output Buffer:
//      SCM_PHYSICAL_DEVICES
//
typedef struct _SCM_PHYSICAL_DEVICE_INSTANCE {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure.
    //
    ULONG Size;

    //
    // The NFIT handle of the physical device.
    //
    ULONG NfitHandle;

    //
    // Symbolic link that can be used to get a handle on the device.
    //
    WCHAR SymbolicLink[SCM_MAX_SYMLINK_LEN_IN_CHARS];

} SCM_PHYSICAL_DEVICE_INSTANCE, *PSCM_PHYSICAL_DEVICE_INSTANCE;

typedef struct _SCM_PHYSICAL_DEVICES {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure, including all the elements in the
    // Devices array.
    //
    ULONG Size;

    //
    // The number of valid elements in the Devices array.
    //
    ULONG DeviceCount;

    //
    // Array of physical device instances.
    //
    SCM_PHYSICAL_DEVICE_INSTANCE Devices[ANYSIZE_ARRAY];

} SCM_PHYSICAL_DEVICES, *PSCM_PHYSICAL_DEVICES;

//
// IOCTL_SCM_BUS_GET_REGIONS
//
// Send to a logical persistent memory device stack to get a list of all regions that make up
// the logical device.
//
// Send to a physical persistent memory device stack to get a list of all the regions that
// reside on that physical device.
//
// Input Buffer:
//      None
//
// Output Buffer:
//      SCM_REGIONS
//

typedef enum _SCM_REGION_FLAG {
    ScmRegionFlagNone = 0x0,

    //
    // Indicates this region is described by a label.
    //
    ScmRegionFlagLabel = 0x1

} SCM_REGION_FLAG, *PSCM_REGION_FLAG;

#define SCM_REGION_SPA_UNKNOWN MAXULONG64

typedef struct _SCM_REGION {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure.
    //
    ULONG Size;

    //
    // Bitmask of SCM_REGION_FLAG values.
    //
    ULONG Flags;

    //
    // The NFIT handle of the physical device for this region.
    //
    ULONG NfitHandle;

    //
    // The GUID of the logical device for this region, if any.
    //
    GUID LogicalDeviceGuid;

    //
    // The address range type (e.g. byte-addressable persistent memory).
    //
    GUID AddressRangeType;

    //
    // Regions that are associated with each other (e.g. part of an interleave
    // set) will share an associated ID.
    //
    ULONG AssociatedId;

    //
    // The total size of the region, in bytes.
    //
    ULONG64 Length;

    //
    // The starting device physical address of the region
    // within the physical device.
    //
    ULONG64 StartingDPA;

    //
    // The base system physical address.
    //
    ULONG64 BaseSPA;

    //
    // The region's offset from the base system physical address.
    // This field may be SCM_REGION_SPA_UNKNOWN if there is not enough
    // context to calculate the SPA offset for this particular region.
    //
    ULONG64 SPAOffset;

    //
    // The value of the Region Offset field from the associated Region Mapping
    // Structure.
    //
    ULONG64 RegionOffset;

} SCM_REGION, *PSCM_REGION;

typedef struct _SCM_REGIONS {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure, including all the elements in the
    // Regions array.
    //
    ULONG Size;

    //
    // The number of valid elements in the Regions array.
    //
    ULONG RegionCount;

    //
    // Array of regions for the logical or physical device.
    //
    SCM_REGION Regions[ANYSIZE_ARRAY];

} SCM_REGIONS, *PSCM_REGIONS;

//
// IOCTL_SCM_BUS_QUERY_PROPERTY
//
// Input Buffer:
//      An SCM_BUS_PROPERTY_QUERY structure that describes the type of query
//      being done, the property being queried, and any additional parameters
//      the query requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the query into. Since all
//      property descriptors can be cast into an SCM_BUS_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of queries
//

typedef enum _SCM_BUS_QUERY_TYPE {
    ScmBusQuery_Descriptor = 0, // Retrieves the descriptor
    ScmBusQuery_IsSupported,    // Used to test whether the descriptor is supported

    ScmBusQuery_Max
} SCM_BUS_QUERY_TYPE, *PSCM_BUS_QUERY_TYPE;


//
// IOCTL_SCM_BUS_SET_PROPERTY
//
// Input Buffer:
//      An SCM_BUS_PROPERTY_SET structure that describes the type of set
//      being done, the property being set, and any additional parameters
//      the set requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the set into. Since all
//      property descriptors can be cast into an SCM_BUS_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of sets
//

typedef enum _SCM_BUS_SET_TYPE {
    ScmBusSet_Descriptor = 0, // Retrieves the descriptor
    ScmBusSet_IsSupported,    // Used to test whether the descriptor is supported

    ScmBusSet_Max
} SCM_BUS_SET_TYPE, *PSCM_BUS_SET_TYPE;


typedef enum _SCM_BUS_PROPERTY_ID {

    //
    // Runtime Firmware Activation Information.
    //
    ScmBusProperty_RuntimeFwActivationInfo = 0,

    //
    // Dedicated Memory Information.
    //
    ScmBusProperty_DedicatedMemoryInfo = 1,

    //
    // Activate/Deactivate the Dedicated Memory.
    //
    ScmBusProperty_DedicatedMemoryState = 2,

    ScmBusProperty_Max
} SCM_BUS_PROPERTY_ID, *PSCM_BUS_PROPERTY_ID;

//
// Query structure - additional parameters for specific queries can follow
// the header
//

typedef struct _SCM_BUS_PROPERTY_QUERY {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    ULONG Size;

    //
    // ID of the property being retrieved.
    //
    SCM_BUS_PROPERTY_ID PropertyId;

    //
    // The type of query being performed.
    //
    SCM_BUS_QUERY_TYPE QueryType;

    //
    // Space for additional parameters if necessary.
    //
    UCHAR AdditionalParameters[ANYSIZE_ARRAY];

} SCM_BUS_PROPERTY_QUERY, *PSCM_BUS_PROPERTY_QUERY;

//
// Output buffer for ScmBusProperty_RuntimeFwActivationInfo
//

//
// ScmBus Firmware Activation State
//
typedef enum _SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    ScmBusFirmwareActivationState_Idle = 0,     // NVDIMM is Idle for firmware update
    ScmBusFirmwareActivationState_Armed = 1,    // NVDIMM is armed to activate the staging firmware
    ScmBusFirmwareActivationState_Busy = 2      // NVDIMM firmware activation is underway

} SCM_BUS_FIRMWARE_ACTIVATION_STATE, *PSCM_BUS_FIRMWARE_ACTIVATION_STATE;

typedef struct _SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // Size of the data contained in this structure. If the output buffer is too small
    // to contain the requested information, the Size field indicates the length of the
    // output buffer the caller should provide in order to retrieve all the data.
    //
    ULONG Size;

    //
    // Indicates if runtime firmware activation is supported or not.
    //
    BOOLEAN RuntimeFwActivationSupported;

    //
    // Indicates the current Firmware activation state of the DIMMs.
    // Note: If any one of the NVDIMMs is Armed, the state is Armed.
    //
    SCM_BUS_FIRMWARE_ACTIVATION_STATE FirmwareActivationState;

    //
    // Firmware activation capabilities.
    //
    struct {

        //
        // Live activation supported with platform firmware managed processor and I/O quiesce.
        //
        ULONG FwManagedIoQuiesceFwActivationSupported : 1;

        //
        // Live activation supported with OS managed I/O quiesce (device idle) and platform managed processor quiesce.
        //
        ULONG OsManagedIoQuiesceFwActivationSupported : 1;

        //
        // Warm reset-based activation supported.
        //
        ULONG WarmResetBasedFwActivationSupported : 1;

        ULONG Reserved : 29;
    } FirmwareActivationCapability;

    //
    // Estimated firmware activation time in micro seconds.
    //
    ULONGLONG EstimatedFirmwareActivationTimeInUSecs;

    //
    // Estimated processor quiesce time during firmware activation in micro seconds.
    // 0 - no processor quiesce required.
    //
    ULONGLONG EstimatedProcessorAccessQuiesceTimeInUSecs;

    //
    // Estimated I/O access to host memory quiesce time during firmware activation in micro seconds.
    // 0 - no I/O quiesce required.
    //
    ULONGLONG EstimatedIOAccessQuiesceTimeInUSecs;

    //
    // Platform firmware supported Max I/O access to memory quiesce time during firmware activation in micro seconds.
    // 0 - Informaiton not available.
    //
    ULONGLONG PlatformSupportedMaxIOAccessQuiesceTimeInUSecs;

} SCM_BUS_RUNTIME_FW_ACTIVATION_INFO, *PSCM_BUS_RUNTIME_FW_ACTIVATION_INFO;

//
// Output buffer for ScmBusProperty_DedicatedMemoryInfo
//
typedef struct _SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {

    //
    // The dedicated memory device GUID.
    //
    GUID DeviceGuid;

    //
    // The dedicated memory device number.
    //
    ULONG DeviceNumber;

    struct {

        //
        // Indicates if the dedicated memory is created by registry settings.
        //
        ULONG ForcedByRegistry : 1;

        //
        // Indicates if the dedicated memory is initialized.
        //
        ULONG Initialized : 1;

        ULONG Reserved : 30;
    } Flags;

    //
    // The dedicated memory device size in bytes.
    //
    ULONGLONG DeviceSize;

} SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO, *PSCM_BUS_DEDICATED_MEMORY_DEVICE_INFO;

typedef struct _SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the data structure, including all the elements in the
    // Devices array.
    //
    ULONG Size;

    //
    // The number of valid elements in the Devices array.
    //
    ULONG DeviceCount;

    //
    // Array of dedicated memory devices info.
    //
    SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO Devices[ANYSIZE_ARRAY];

} SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO, *PSCM_BUS_DEDICATED_MEMORY_DEVICES_INFO;


//
// Set structure - additional parameters for specific sets can follow
// the header
//

typedef struct _SCM_BUS_PROPERTY_SET {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    ULONG Size;

    //
    // ID of the property being set.
    //
    SCM_BUS_PROPERTY_ID PropertyId;

    //
    // The type of set being performed.
    //
    SCM_BUS_SET_TYPE SetType;

    //
    // Space for additional parameters if necessary.
    //
    UCHAR AdditionalParameters[ANYSIZE_ARRAY];

} SCM_BUS_PROPERTY_SET, *PSCM_BUS_PROPERTY_SET;

//
// Input AdditionalParameters for ScmBusProperty_DedicatedMemoryState
//

typedef struct _SCM_BUS_DEDICATED_MEMORY_STATE {

    //
    // Dedicated Memory state (Deactivate - FALSE, Activate - TRUE).
    //
    BOOLEAN ActivateState;
} SCM_BUS_DEDICATED_MEMORY_STATE, *PSCM_BUS_DEDICATED_MEMORY_STATE;


//
// Definitions for interfaces related to logical persistent memory devices (LDs).
//

//
// IOCTL_SCM_LD_GET_INTERLEAVE_SET
//
// This IOCTL retrieves the interleave set of a logical persistent memory disk. The interleave set
// is comprised of one or more physical persistent memory devices.
//
// Input Buffer:
//      None.
//
// Output Buffer:
//      SCM_LD_INTERLEAVE_SET_INFO
//

typedef struct _SCM_INTERLEAVED_PD_INFO {

    //
    // An identifier for the physical device that comes from the NFIT table and
    // is unique on the local system.
    //
    ULONG DeviceHandle;

    //
    // A GUID that uniquely identifies the physical device on the system.
    //
    GUID DeviceGuid;

} SCM_INTERLEAVED_PD_INFO, *PSCM_INTERLEAVED_PD_INFO;

typedef struct _SCM_LD_INTERLEAVE_SET_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // Total size of the structure, in bytes, including the InterleaveSet array.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    ULONG Size;

    //
    // The number of elements in the InterleaveSet array.
    //
    ULONG InterleaveSetSize;

    //
    // Information about the physical devices that make up this interleave
    // set.
    //
    SCM_INTERLEAVED_PD_INFO InterleaveSet[ANYSIZE_ARRAY];

} SCM_LD_INTERLEAVE_SET_INFO, *PSCM_LD_INTERLEAVE_SET_INFO;


//
// Definitions for interfaces related to physical persistent memory devices (PDs).
//

//
// IOCTL_SCM_PD_QUERY_PROPERTY
//
// Input Buffer:
//      An SCM_PD_PROPERTY_QUERY structure that describes the type of query
//      being done, the property being queried, and any additional parameters
//      the query requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the query into. Since all
//      property descriptors can be cast into an SCM_PD_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of queries
//

typedef enum _SCM_PD_QUERY_TYPE {
    ScmPhysicalDeviceQuery_Descriptor = 0, // Retrieves the descriptor
    ScmPhysicalDeviceQuery_IsSupported, // Used to test whether the descriptor is supported

    ScmPhysicalDeviceQuery_Max
} SCM_PD_QUERY_TYPE, *PSCM_PD_QUERY_TYPE;


//
// IOCTL_SCM_PD_SET_PROPERTY
//
// Input Buffer:
//      An SCM_PD_PROPERTY_SET structure that describes the type of set
//      being done, the property being set, and any additional parameters
//      the set requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the set into. Since all
//      property descriptors can be cast into an SCM_PD_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of sets
//

typedef enum _SCM_PD_SET_TYPE {
    ScmPhysicalDeviceSet_Descriptor = 0, // Retrieves the descriptor
    ScmPhysicalDeviceSet_IsSupported, // Used to test whether the descriptor is supported

    ScmPhysicalDeviceSet_Max
} SCM_PD_SET_TYPE, *PSCM_PD_SET_TYPE;


typedef enum _SCM_PD_PROPERTY_ID {

    //
    // General information about the device.
    //
    ScmPhysicalDeviceProperty_DeviceInfo = 0,

    //
    // Information about the device's health.
    //
    ScmPhysicalDeviceProperty_ManagementStatus,

    //
    // Firmware-related information.
    //
    ScmPhysicalDeviceProperty_FirmwareInfo,

    //
    // Returns a string that identifies where the device is located
    // on the local system.
    //
    ScmPhysicalDeviceProperty_LocationString,

    //
    // Returns a series of device-specific information, which give more detail
    // on the device's status.
    //
    ScmPhysicalDeviceProperty_DeviceSpecificInfo,

    //
    // Returns a identifying handle of the physical device, which comes from
    // the NFIT table.
    //
    ScmPhysicalDeviceProperty_DeviceHandle,

    //
    // Returns a string that identifies the replacement unit housing
    // the device on the local system.
    //
    ScmPhysicalDeviceProperty_FruIdString,

    //
    // Returns runtime firmware activation information.
    //
    ScmPhysicalDeviceProperty_RuntimeFwActivationInfo,

    //
    // Runtime firmware activation arm state.
    //
    ScmPhysicalDeviceProperty_RuntimeFwActivationArmState,

    ScmPhysicalDeviceProperty_Max
} SCM_PD_PROPERTY_ID, *PSCM_PD_PROPERTY_ID;


//
// Query structure - additional parameters for specific queries can follow
// the header
//

typedef struct _SCM_PD_PROPERTY_QUERY {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    ULONG Size;

    //
    // ID of the property being retrieved.
    //
    SCM_PD_PROPERTY_ID PropertyId;

    //
    // The type of query being performed.
    //
    SCM_PD_QUERY_TYPE QueryType;

    //
    // Space for additional parameters if necessary.
    //
    UCHAR AdditionalParameters[ANYSIZE_ARRAY];

} SCM_PD_PROPERTY_QUERY, *PSCM_PD_PROPERTY_QUERY;

//
// Set structure - additional parameters for specific sets can follow
// the header
//

typedef struct _SCM_PD_PROPERTY_SET {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    ULONG Size;

    //
    // ID of the property being retrieved.
    //
    SCM_PD_PROPERTY_ID PropertyId;

    //
    // The type of set being performed.
    //
    SCM_PD_SET_TYPE SetType;

    //
    // Space for additional parameters if necessary.
    //
    UCHAR AdditionalParameters[ANYSIZE_ARRAY];

} SCM_PD_PROPERTY_SET, *PSCM_PD_PROPERTY_SET;

//
// Input AdditionalParameters for ScmPhysicalDeviceProperty_RuntimeFwActivationArmState
//

typedef struct _SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {

    //
    // Runtime firmware activation arm state (Disarm - FALSE, Arm - TRUE).
    //
    BOOLEAN ArmState;
} SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE, *PSCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE;

//
// Standard property descriptor header. All property pages should use this
// as their first element or should contain these two elements
//

typedef struct _SCM_PD_DESCRIPTOR_HEADER {

    //
    // The sizeof() of the entire descriptor (not just the header).
    //
    ULONG Version;

    //
    // The size of the entire descriptor (not just the header).
    //
    ULONG Size;
} SCM_PD_DESCRIPTOR_HEADER, *PSCM_PD_DESCRIPTOR_HEADER;

//
// Output buffer for ScmPhysicalDeviceProperty_DeviceHandle & ScmPhysicalDeviceQuery_Descriptor
//

//
// The ScmPhysicalDeviceProperty_DeviceHandle property gets identifying information about
// a physical device.
//
typedef struct _SCM_PD_DEVICE_HANDLE {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the structure.
    //
    ULONG Size;

    //
    // A GUID that uniquely identifies the physical device, based on hardware information.
    //
    GUID DeviceGuid;

    //
    // A handle, exposed in the NFIT table, that uniquely identifies the physical device on a local
    // system.
    //
    ULONG DeviceHandle;

} SCM_PD_DEVICE_HANDLE, *PSCM_PD_DEVICE_HANDLE;

//
// Output buffer for ScmPhysicalDeviceProperty_DeviceInfo & ScmPhysicalDeviceQuery_Descriptor
//

#define MAX_INTERFACE_CODES 8
#define SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES 32

#define SCM_PD_MEMORY_SIZE_UNKNOWN MAXULONG64

typedef struct _SCM_PD_DEVICE_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // The total size of the structure, including the serial number at the end.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    ULONG Size;
    
    //
    // A GUID that uniquely identifies the physical device, based on hardware information.
    //
    GUID DeviceGuid;
    
    //
    // The number of times this device went through an unsafe shutdown (i.e. a shutdown
    // that might have led to data loss).
    //
    ULONG UnsafeShutdownCount;

        
    //
    // The combined size of all the persistent memory regions of the physical device.
    //
    ULONG64 PersistentMemorySizeInBytes;

    //
    // The total size of the volatile memory this device contains, if any.
    // May be SCM_PD_MEMORY_SIZE_UNKNOWN if it is not reported by the platform.
    //
    ULONG64 VolatileMemorySizeInBytes;

    //
    // The total capacity of this memory device, including persistent and any
    // volatile memory.
    // May be SCM_PD_MEMORY_SIZE_UNKNOWN if it is not reported by the platform.
    //
    ULONG64 TotalMemorySizeInBytes;
    
    //
    // The number of the slot in which the physical device is installed on the system.
    //
    ULONG SlotNumber;
    
    //
    // A handle, exposed in the NFIT table, that uniquely identifies the physical device on a local
    // system.
    //
    ULONG DeviceHandle;

    //
    // The unique ID for this physical device as indicated in the SMBIOS.
    //
    USHORT PhysicalId;

    //
    // An physical device can have regions that implement different format interface
    // codes. This is a list of all format interface codes on this physical device.
    //
    UCHAR  NumberOfFormatInterfaceCodes;
    USHORT FormatInterfaceCodes[MAX_INTERFACE_CODES];

    //
    // Vendor and product IDs.
    //
    ULONG VendorId;
    ULONG ProductId;
    ULONG SubsystemDeviceId;
    ULONG SubsystemVendorId;
    UCHAR ManufacturingLocation;
    UCHAR ManufacturingWeek; // *Not* in BCD format.
    UCHAR ManufacturingYear; // *Not* in BCD format.
    ULONG SerialNumber4Byte; // 4-byte serial number as defined in the JEDEC SPD spec and reported in the NFIT.

    //
    // The physical device's serial number, as a string.
    //
    ULONG SerialNumberLengthInChars;
    _Field_size_(SerialNumberLengthInChars) CHAR SerialNumber[ANYSIZE_ARRAY];
} SCM_PD_DEVICE_INFO, *PSCM_PD_DEVICE_INFO;

//
// Output buffer for ScmPhysicalDeviceProperty_DeviceSpecificInfo & ScmPhysicalDeviceQuery_Descriptor
//

//
// A device specific property is a key-value pair where the key is a string
// and the value is a number.
//
#define SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS 128

typedef struct _SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    // NULL-terminated string.
    WCHAR Name[SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS];
    LONGLONG Value;
} SCM_PD_DEVICE_SPECIFIC_PROPERTY, *PSCM_PD_DEVICE_SPECIFIC_PROPERTY;

//
// The physical device driver fills in this structure with arbitrary numeric information.
//
typedef struct _SCM_PD_DEVICE_SPECIFIC_INFO {
    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the structure, including the DeviceSpecificProperties array.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    ULONG Size;

    //
    // The number of elements in the DeviceSpecificProperties array.
    //
    ULONG NumberOfProperties;
    
    //
    // A series of device-specific properties filled in by the driver.
    //
    SCM_PD_DEVICE_SPECIFIC_PROPERTY DeviceSpecificProperties[ANYSIZE_ARRAY];
} SCM_PD_DEVICE_SPECIFIC_INFO, *PSCM_PD_DEVICE_SPECIFIC_INFO;

typedef struct _SCM_PD_FIRMWARE_SLOT_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG   Version;
    
    //
    // Size of the data contained in this structure.
    //
    ULONG   Size;

    UCHAR   SlotNumber;
    UCHAR   ReadOnly : 1;
    UCHAR   Reserved0 : 7;
    UCHAR   Reserved1[6];

    UCHAR   Revision[SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES];

} SCM_PD_FIRMWARE_SLOT_INFO, *PSCM_PD_FIRMWARE_SLOT_INFO;


//
// Output buffer for ScmPhysicalDeviceQuery_Descriptor & ScmPhysicalDeviceProperty_FirmwareInfo
//
typedef struct _SCM_PD_FIRMWARE_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // Size of the data contained in this structure, including the Slots
    // array. If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    ULONG Size;


    //
    // The firmware slot that is currently active.
    // 
    UCHAR ActiveSlot;

    //
    // The slot that will become active once the device is reset. A value of 0xFF means
    // there is no slot waiting to be activated.
    //
    UCHAR NextActiveSlot;

    UCHAR SlotCount;
    
    _Field_size_(SlotCount) SCM_PD_FIRMWARE_SLOT_INFO Slots[ANYSIZE_ARRAY];

} SCM_PD_FIRMWARE_INFO, *PSCM_PD_FIRMWARE_INFO;


//
// Constants for ScmPhysicalDeviceProperty_ManagementStatus, which can be queried via
// ScmPhysicalDeviceQuery_Descriptor.
//

//
// Health states.
//
typedef enum _SCM_PD_HEALTH_STATUS {
    ScmPhysicalDeviceHealth_Unknown = 0,
    ScmPhysicalDeviceHealth_Unhealthy,
    ScmPhysicalDeviceHealth_Warning,
    ScmPhysicalDeviceHealth_Healthy,

    ScmPhysicalDeviceHealth_Max
} SCM_PD_HEALTH_STATUS, *PSCM_PD_HEALTH_STATUS;

//
// Operational states.
//
typedef enum _SCM_PD_OPERATIONAL_STATUS {
    ScmPhysicalDeviceOpStatus_Unknown = 0,
    ScmPhysicalDeviceOpStatus_Ok,
    ScmPhysicalDeviceOpStatus_PredictingFailure,
    ScmPhysicalDeviceOpStatus_InService,
    ScmPhysicalDeviceOpStatus_HardwareError,
    ScmPhysicalDeviceOpStatus_NotUsable,
    ScmPhysicalDeviceOpStatus_TransientError,
    ScmPhysicalDeviceOpStatus_Missing,

    ScmPhysicalDeviceOpStatus_Max
} SCM_PD_OPERATIONAL_STATUS, *PSCM_PD_OPERATIONAL_STATUS;

//
// Operational reasons.
//
typedef enum _SCM_PD_OPERATIONAL_STATUS_REASON {
    ScmPhysicalDeviceOpReason_Unknown = 0,
    ScmPhysicalDeviceOpReason_Media,
    ScmPhysicalDeviceOpReason_ThresholdExceeded,
    ScmPhysicalDeviceOpReason_LostData,
    ScmPhysicalDeviceOpReason_EnergySource,
    ScmPhysicalDeviceOpReason_Configuration,
    ScmPhysicalDeviceOpReason_DeviceController,
    ScmPhysicalDeviceOpReason_MediaController,
    ScmPhysicalDeviceOpReason_Component,
    ScmPhysicalDeviceOpReason_BackgroundOperation,
    ScmPhysicalDeviceOpReason_InvalidFirmware,
    ScmPhysicalDeviceOpReason_HealthCheck,
    ScmPhysicalDeviceOpReason_LostDataPersistence,
    ScmPhysicalDeviceOpReason_DisabledByPlatform,
    ScmPhysicalDeviceOpReason_PermanentError,
    ScmPhysicalDeviceOpReason_LostWritePersistence,
    ScmPhysicalDeviceOpReason_FatalError,
    ScmPhysicalDeviceOpReason_DataPersistenceLossImminent,
    ScmPhysicalDeviceOpReason_WritePersistenceLossImminent,
    ScmPhysicalDeviceOpReason_MediaRemainingSpareBlock,
    ScmPhysicalDeviceOpReason_PerformanceDegradation,
    ScmPhysicalDeviceOpReason_ExcessiveTemperature,
    ScmPhysicalDeviceOpReason_InternalFailure,

    ScmPhysicalDeviceOpReason_Max
} SCM_PD_OPERATIONAL_STATUS_REASON, *PSCM_PD_OPERATIONAL_STATUS_REASON;

//
// Output buffer for ScmPhysicalDeviceProperty_ManagementStatus & ScmPhysicalDeviceQuery_Descriptor
//

#define SCM_PD_MAX_OPERATIONAL_STATUS    16

typedef struct _SCM_PD_MANAGEMENT_STATUS {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the structure, including operational status reasons
    // that didn't fit in the caller's array. If the output buffer is too small to
    // contain the requested information, the Size field indicates the length of the
    // output buffer the caller should provide in order to retrieve all the data.
    //
    ULONG Size;

    //
    // Health status.
    //
    SCM_PD_HEALTH_STATUS Health;

    //
    // The number of operational statuses returned.
    //
    ULONG NumberOfOperationalStatus;

    //
    // The number of additional reasons returned.
    //
    ULONG NumberOfAdditionalReasons;

    //
    // Operational statuses. The primary operational status is the first element
    // in the array. There are NumberOfOperationalStatus valid elements in the array.
    //
    SCM_PD_OPERATIONAL_STATUS OperationalStatus[SCM_PD_MAX_OPERATIONAL_STATUS];

    //
    // Additional reasons. There are NumberOfAdditionalReasons valid elements in the array.
    //
    _Field_size_(NumberOfAdditionalReasons) SCM_PD_OPERATIONAL_STATUS_REASON AdditionalReasons[ANYSIZE_ARRAY];

} SCM_PD_MANAGEMENT_STATUS, *PSCM_PD_MANAGEMENT_STATUS;

//
// Output buffer for ScmPhysicalDeviceQuery_Descriptor & ScmPhysicalDeviceProperty_LocationString
//
typedef struct _SCM_PD_LOCATION_STRING {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the structure, including the buffer for the Unicode
    // string. If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    ULONG Size;

    //
    // The Unicode string that represents the physical location of this physical device.
    //
    WCHAR Location[ANYSIZE_ARRAY];

} SCM_PD_LOCATION_STRING, *PSCM_PD_LOCATION_STRING;

//
// Output buffer for ScmPhysicalDeviceQuery_Descriptor & ScmPhysicalDeviceProperty_FruIdString
//
typedef struct _SCM_PD_FRU_ID_STRING {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // The total size of the structure, including the buffer for the fru id string.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    ULONG Size;

    //
    // The string that represents the fru id of this physical device.
    //
    ULONG IdentifierSize;
    UCHAR Identifier[ANYSIZE_ARRAY];

} SCM_PD_FRU_ID_STRING, *PSCM_PD_FRU_ID_STRING;

//
// Firmware update IOCTLs.
//

//
// Signals that the firmware image regions contained in the SCM_PD_FIRMWARE_DOWNLOAD
// structure are the last ones of the image. The physical device driver finishes the firmware update
// operation when this flag is set.
//
#define SCM_PD_FIRMWARE_LAST_DOWNLOAD 0x1

//
// Input buffer for IOCTL_SCM_PD_FIRMWARE_DOWNLOAD.
//
typedef struct _SCM_PD_FIRMWARE_DOWNLOAD {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // The structure size, including the image.
    //
    ULONG Size;
    
    //
    // Additional information about the region being download, such as whether it
    // is the last region in the image.
    //
    ULONG Flags;
    
    //
    // The firmware slot being upgraded.
    //
    UCHAR Slot;
    
    UCHAR Reserved[3];

    //
    // The offset of this region of the firmware image.
    //
    ULONG64 Offset;

    //
    // The size of the FirmwareImage array.
    //
    ULONG FirmwareImageSizeInBytes;

    //
    // The firmware region being downloaded to the device.
    //
    UCHAR FirmwareImage[ANYSIZE_ARRAY];

} SCM_PD_FIRMWARE_DOWNLOAD, *PSCM_PD_FIRMWARE_DOWNLOAD;

//
// Input buffer for IOCTL_SCM_PD_FIRMWARE_ACTIVATE.
//
typedef struct _SCM_PD_FIRMWARE_ACTIVATE {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // Total size of the structure
    //
    ULONG Size;

    //
    // Reserved. Callers should set to 0.
    //
    ULONG Flags;
    
    //
    // The slot that contains the firmware image being activated.
    //
    UCHAR Slot;

} SCM_PD_FIRMWARE_ACTIVATE, *PSCM_PD_FIRMWARE_ACTIVATE;

//
// Output buffer for ScmPhysicalDeviceProperty_RuntimeFwActivationInfo
//

//
// ScmBus Physical Device Last Firmware Activation Status
//
typedef enum _SCM_PD_LAST_FW_ACTIVATION_STATUS {
    ScmPdLastFwActivationStatus_None = 0,                   // No Firmware Activation performed. Reset to None on boot
    ScmPdLastFwActivationStatus_Success = 1,                // Success
    ScmPdLastFwActivationStatus_FwNotFound = 2,             // No new staged firmware to activate
    ScmPdLastFwActivationStatus_ColdRebootRequired = 3,     // NVDIMM Reset required to activate staged firmware
    ScmPdLastFwActivaitonStatus_ActivationInProgress = 4,   // Media disabled during firmware activation
    ScmPdLastFwActivaitonStatus_Retry = 5,                  // Activation aborted due to throttling. Retry recommended
    ScmPdLastFwActivaitonStatus_FwUnsupported = 6,          // Staged firmware version does not meet activation requirements
    ScmPdLastFwActivaitonStatus_UnknownError = 7            // Unclassified firmware activation error

} SCM_PD_LAST_FW_ACTIVATION_STATUS, *PSCM_PD_LAST_FW_ACTIVATION_STATUS;

//
// ScmBus Physical Device Firmware Activation State
//
typedef enum _SCM_PD_FIRMWARE_ACTIVATION_STATE {
    ScmPdFirmwareActivationState_Idle = 0,  // NVDIMM is Idle for firmware update
    ScmPdFirmwareActivationState_Armed = 1, // NVDIMM is armed to activate the staging firmware
    ScmPdFirmwareActivationState_Busy = 2   // NVDIMM firmware activation is underway

} SCM_PD_FIRMWARE_ACTIVATION_STATE, *PSCM_PD_FIRMWARE_ACTIVATION_STATE;

typedef struct _SCM_PD_RUNTIME_FW_ACTIVATION_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;

    //
    // Size of the data contained in this structure. If the output buffer is too small
    // to contain the requested information, the Size field indicates the length of the
    // output buffer the caller should provide in order to retrieve all the data.
    //
    ULONG Size;

    //
    // This provides the previous Firmware Activation status.
    //
    SCM_PD_LAST_FW_ACTIVATION_STATUS LastFirmwareActivationStatus;

    //
    // Indicates the current Firmware activation state of the DIMM.
    //
    SCM_PD_FIRMWARE_ACTIVATION_STATE FirmwareActivationState;

} SCM_PD_RUNTIME_FW_ACTIVATION_INFO, *PSCM_PD_RUNTIME_FW_ACTIVATION_INFO;

//
// IOCTL_SCM_PD_PASSTHROUGH
//
// This IOCTL sends a vendor-specific command to a physical device and returns its
// output.
//
// Input buffer:
//      SCM_PD_PASSTHROUGH_INPUT
//      The input structure contains another, physical device-type specific structure.
//
// Output buffer:
//      SCM_PD_PASSTHROUGH_OUTPUT
//      The output structure contains another, physical device-type specific structure.
//

typedef struct _SCM_PD_PASSTHROUGH_INPUT {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // The size of the structure, including the Data field, in bytes.
    //
    ULONG Size;

    //
    // This GUID defines which command protocol is being used. The driver will
    // check this field to make sure the application is sending commands for
    // device types that the driver understands.
    //
    GUID ProtocolGuid;

    //
    // The size, in bytes, of the data field.
    //
    ULONG DataSize;

    //
    // The physical device-type specific structure which contains the passthrough command.
    //
    UCHAR Data[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_INPUT, *PSCM_PD_PASSTHROUGH_INPUT;

typedef struct _SCM_PD_PASSTHROUGH_OUTPUT {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // The size of the structure, including the Data field, in bytes.
    // The caller is responsible for knowing how large the output buffer
    // will be. The common approach of sending the IOCTL twice - once to learn
    // the total required size, and again to retrieve the data - isn't recommended
    // here because of the performance impact of executing a passthrough command.
    //
    ULONG Size;

    //
    // This GUID defines which command protocol is being used. The application should
    // check this field to make sure the driver is using a protocol that it understands.
    //
    GUID ProtocolGuid;

    //
    // The size, in bytes, of the data field.
    //
    ULONG DataSize;

    //
    // The physical device-type specific structure which contains the output of the passthrough command.
    //
    UCHAR Data[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_OUTPUT, *PSCM_PD_PASSTHROUGH_OUTPUT;

//
// Passthrough structures and definitions for INVDIMM devices.
//

//
// This structure defines the input of an INVDIMM command. The application sending a passthrough
// command uses this structure as the "Data" field of the SCM_PD_PASSTHROUGH_INPUT structure.
//
typedef struct _SCM_PD_PASSTHROUGH_INVDIMM_INPUT {

    //
    // The command's opcode.
    //
    ULONG Opcode;

    //
    // The length, in bytes, of the OpcodeParameters field.
    // This can be zero, but the size of this structure must always be equal to
    // or greater than sizeof(SCM_PD_PASSTHROUGH_INVDIMM_INPUT).
    //
    ULONG OpcodeParametersLength;

    //
    // The opcode input payload, if any.
    //
    UCHAR OpcodeParameters[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_INVDIMM_INPUT, *PSCM_PD_PASSTHROUGH_INVDIMM_INPUT;

//
// This structure defines the output of an INVDIMM command. The driver will respond to
// a passthrough request by bundling this structure in the "Data" field of the
// SCM_PD_PASSTHROUGH_OUTPUT structure.
//
typedef struct _SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {

    //
    // The general status of the command (see the INVDIMM _DSM specification for details).
    //
    USHORT GeneralStatus;

    //
    // The extended status of the command (see the INVDIMM _DSM specification for details).
    //
    USHORT ExtendedStatus;

    //
    // The length, in bytes, of the OutputData field. Even when this is zero, the total
    // size of this structure will be equal to or greater than sizeof(SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT).
    //
    ULONG OutputDataLength;

    //
    // The data returned by the device in response to the command.
    //
    UCHAR OutputData[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT, *PSCM_PD_PASSTHROUGH_INVDIMM_OUTPUT;

//
// IOCTL_SCM_PD_REINITIALIZE_MEDIA
//
// This IOCTL reinitializes the media of a physical persistent memory device, which erases
// all the data on it.
//
// Input buffer:
//      SCM_PD_REINITIALIZE_MEDIA_INPUT
//
// Output buffer:
//      SCM_PD_REINITIALIZE_MEDIA_OUTPUT
//
typedef struct _SCM_PD_REINITIALIZE_MEDIA_INPUT {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // The size of the structure, in bytes.
    //
    ULONG Size;
    
    //
    // If Overwrite is set to 1, the physical persistent memory device will
    // overwrite the entire media, which might take several minutes.
    // If it is set to 0, the physical persistent device may do a crypto-erase or some
    // other quicker form of clearing the data on the media.
    //
    struct {
        ULONG Overwrite : 1;
    } Options;
} SCM_PD_REINITIALIZE_MEDIA_INPUT, *PSCM_PD_REINITIALIZE_MEDIA_INPUT;

typedef enum _SCM_PD_MEDIA_REINITIALIZATION_STATUS {

    //
    // The media reinitialization was successful and the device is ready for use.
    //
    ScmPhysicalDeviceReinit_Success = 0,
    
    //
    // The media reinitialization was successful, but the device requires a reboot before being used.
    //
    ScmPhysicalDeviceReinit_RebootNeeded,
    
    //
    // The media reinitialization was successful, but the device requires a cold boot before being used.
    //
    ScmPhysicalDeviceReinit_ColdBootNeeded,

    ScmPhysicalDeviceReinit_Max
} SCM_PD_MEDIA_REINITIALIZATION_STATUS, *PSCM_PD_MEDIA_REINITIALIZATION_STATUS;

typedef struct _SCM_PD_REINITIALIZE_MEDIA_OUTPUT {

    //
    // Sizeof() of this structure serves as the version.
    //
    ULONG Version;
    
    //
    // The size of the structure, in bytes.
    //
    ULONG Size;
    
    //
    // The detailed status of the reinitialization operation, in case it
    // was successful. If it failed, the IOCTL itself will fail and callers
    // should not look at the returned status code instead of this field.
    //
    SCM_PD_MEDIA_REINITIALIZATION_STATUS Status;
} SCM_PD_REINITIALIZE_MEDIA_OUTPUT, *PSCM_PD_REINITIALIZE_MEDIA_OUTPUT;


#endif // NTDDI_WIN10_RS5

#pragma warning(pop)


#endif // _NTDDSCM_H_

// end_winioctl

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) | WINAPI_PARTITION_SYSTEM) */

#pragma endregion

#ifdef __cplusplus
}
#endif

