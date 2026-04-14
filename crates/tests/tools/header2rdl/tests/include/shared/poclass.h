/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    poclass.h

Abstract:

    Defines power policy device driver interfaces.



Revision History:

--*/

//
// GUIDs are defined outside ifdef __POCLASS_ so that they can be instantiated
// easily using <initguid.h>.
//

//
// Custom device properties...
//

#include <devpropdef.h>
#include <winapifamily.h>

//
// This is of type DEVPROP_TYPE_UINT32 and represents the NT processor
// number.
//

DEFINE_DEVPROPKEY(PROCESSOR_NUMBER_PKEY,
                  0x5724c81d,
                  0xd5af,
                  0x4c1f,
                  0xa1, 0x03, 0xa0, 0x6e, 0x28, 0xf2, 0x04, 0xc6,
                  1);

//
// Power management policy device GUIDs
//

DEFINE_GUID( GUID_DEVICE_BATTERY,           0x72631e54L, 0x78A4, 0x11d0, 0xbc, 0xf7, 0x00, 0xaa, 0x00, 0xb7, 0xb3, 0x2a );
DEFINE_GUID( GUID_DEVICE_APPLICATIONLAUNCH_BUTTON, 0x629758eel, 0x986e, 0x4d9e, 0x8e, 0x47, 0xde, 0x27, 0xf8, 0xab, 0x05, 0x4d );
DEFINE_GUID( GUID_DEVICE_SYS_BUTTON,        0x4AFA3D53L, 0x74A7, 0x11d0, 0xbe, 0x5e, 0x00, 0xA0, 0xC9, 0x06, 0x28, 0x57 );
DEFINE_GUID( GUID_DEVICE_LID,               0x4AFA3D52L, 0x74A7, 0x11d0, 0xbe, 0x5e, 0x00, 0xA0, 0xC9, 0x06, 0x28, 0x57 );
DEFINE_GUID( GUID_DEVICE_THERMAL_ZONE,      0x4AFA3D51L, 0x74A7, 0x11d0, 0xbe, 0x5e, 0x00, 0xA0, 0xC9, 0x06, 0x28, 0x57 );
DEFINE_GUID( GUID_DEVICE_FAN,               0x05ecd13dL, 0x81da, 0x4a2a, 0x8a, 0x4c, 0x52, 0x4f, 0x23, 0xdd, 0x4d, 0xc9 );
DEFINE_GUID( GUID_DEVICE_PROCESSOR,         0x97fadb10L, 0x4e33, 0x40ae, 0x35, 0x9c, 0x8b, 0xef, 0x02, 0x9d, 0xbd, 0xd0 );
DEFINE_GUID( GUID_DEVICE_MEMORY,            0x3fd0f03dL, 0x92e0, 0x45fb, 0xb7, 0x5c, 0x5e, 0xd8, 0xff, 0xb0, 0x10, 0x21 );
DEFINE_GUID( GUID_DEVICE_ACPI_TIME,         0x97f99bf6L, 0x4497, 0x4f18, 0xbb, 0x22, 0x4b, 0x9f, 0xb2, 0xfb, 0xef, 0x9c );
DEFINE_GUID( GUID_DEVICE_MESSAGE_INDICATOR, 0XCD48A365L, 0xfa94, 0x4ce2, 0xa2, 0x32, 0xa1, 0xb7, 0x64, 0xe5, 0xd8, 0xb4 );
DEFINE_GUID( GUID_DEVICE_POWER_ADAPTER,     0xf76c6c62L, 0x7dea, 0x43cd, 0x86, 0x89, 0xd9, 0xa4, 0xaf, 0x3d, 0x85, 0x57 );

// copied from hidclass.h
DEFINE_GUID( GUID_CLASS_INPUT,              0x4D1E55B2L, 0xF16F, 0x11CF, 0x88, 0xCB, 0x00, 0x11, 0x11, 0x00, 0x00, 0x30 );

// {DBE4373D-3C81-40cb-ACE4-E0E5D05F0C9F}
DEFINE_GUID(GUID_DEVINTERFACE_THERMAL_COOLING,
0xdbe4373d, 0x3c81, 0x40cb, 0xac, 0xe4, 0xe0, 0xe5, 0xd0, 0x5f, 0xc, 0x9f);

// {927EC093-69A4-4bc0-BD02-711664714463}
DEFINE_GUID(GUID_DEVINTERFACE_THERMAL_MANAGER,
0x927ec093, 0x69a4, 0x4bc0, 0xbd, 0x2, 0x71, 0x16, 0x64, 0x71, 0x44, 0x63);

// {8f366301-091e-4056-b92f-958b27625fce}
DEFINE_GUID(GUID_DEVINTERFACE_POWER_LIMIT,
0x8f366301, 0x091e, 0x4056, 0xb9, 0x2f, 0x95, 0x8b, 0x27, 0x62, 0x5f, 0xce);

// {2a6c8538-7895-4d56-8567-795d3844858a}
DEFINE_GUID(GUID_DEVINTERFACE_TEMPERATURE_SENSOR,
0x2a6c8538, 0x7895, 0x4d56, 0x85, 0x67, 0x79, 0x5d, 0x38, 0x44, 0x85, 0x8a);

// {2ed8544a-8eef-4033-b2a0-04aaa507cecb}
DEFINE_GUID(GUID_DEVINTERFACE_CUSTOMIZED_IO,
0x2ed8544a, 0x8eef, 0x4033, 0xb2, 0xa0, 0x04, 0xaa, 0xa5, 0x07, 0xce, 0xcb);

#ifndef _POCLASS_
#define _POCLASS_

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Battery driver interface (devices of registrying as GUID_DEVICE_BATTERY)
//

typedef enum {
    BatteryInformation,
    BatteryGranularityInformation,
    BatteryTemperature,
    BatteryEstimatedTime,
    BatteryDeviceName,
    BatteryManufactureDate,
    BatteryManufactureName,
    BatteryUniqueID,
    BatterySerialNumber
} BATTERY_QUERY_INFORMATION_LEVEL;

typedef struct _BATTERY_QUERY_INFORMATION {
    ULONG                           BatteryTag;
    BATTERY_QUERY_INFORMATION_LEVEL InformationLevel;
    ULONG                           AtRate;
} BATTERY_QUERY_INFORMATION, *PBATTERY_QUERY_INFORMATION;

typedef struct _BATTERY_INFORMATION {
    ULONG       Capabilities;
    UCHAR       Technology;
    UCHAR       Reserved[3];
    UCHAR       Chemistry[4];
    ULONG       DesignedCapacity;
    ULONG       FullChargedCapacity;
    ULONG       DefaultAlert1;
    ULONG       DefaultAlert2;
    ULONG       CriticalBias;
    ULONG       CycleCount;
} BATTERY_INFORMATION, *PBATTERY_INFORMATION;

// BATTERY_INFORMATION.*Capacity constants
#define BATTERY_UNKNOWN_CAPACITY 0xFFFFFFFF
#define UNKNOWN_CAPACITY         BATTERY_UNKNOWN_CAPACITY

// BATTERY_INFORMATION.Capabilities flags
#define BATTERY_SYSTEM_BATTERY               0x80000000
#define BATTERY_CAPACITY_RELATIVE            0x40000000
#define BATTERY_IS_SHORT_TERM                0x20000000
#define BATTERY_SEALED                       0x10000000
#define BATTERY_SET_CHARGE_SUPPORTED         0x00000001
#define BATTERY_SET_DISCHARGE_SUPPORTED      0x00000002
#define BATTERY_SET_CHARGINGSOURCE_SUPPORTED 0x00000004
#define BATTERY_SET_CHARGER_ID_SUPPORTED     0x00000008

//
// BatteryEstimatedTime constant
//
#define BATTERY_UNKNOWN_TIME 0xFFFFFFFF

#define BATTERY_UNKNOWN_CURRENT 0xFFFFFFFF
#define UNKNOWN_CURRENT BATTERY_UNKNOWN_CURRENT

// AC type explicitly set to maintain backwards compatibility
// (the previous enum member BatteryChargingSourceType_None was removed).
typedef enum _BATTERY_CHARGING_SOURCE_TYPE {
    BatteryChargingSourceType_AC = 1,
    BatteryChargingSourceType_USB,
    BatteryChargingSourceType_Wireless,
    BatteryChargingSourceType_Max
} BATTERY_CHARGING_SOURCE_TYPE, *PBATTERY_CHARGING_SOURCE_TYPE;

typedef struct _BATTERY_CHARGING_SOURCE {
    BATTERY_CHARGING_SOURCE_TYPE Type;
    ULONG MaxCurrent;
} BATTERY_CHARGING_SOURCE, *PBATTERY_CHARGING_SOURCE;

typedef GUID BATTERY_CHARGER_ID, *PBATTERY_CHARGER_ID;

// New structure for Windows Phone specific feature: Source Change Notification.
typedef struct _BATTERY_CHARGING_SOURCE_INFORMATION {
    BATTERY_CHARGING_SOURCE_TYPE Type;
    BOOLEAN SourceOnline;
} BATTERY_CHARGING_SOURCE_INFORMATION, *PBATTERY_CHARGING_SOURCE_INFORMATION;

//
// Physical USB port types.
//

typedef enum _USB_CHARGER_PORT
{
    UsbChargerPort_Legacy,
    UsbChargerPort_TypeC,
    UsbChargerPort_Max
} USB_CHARGER_PORT, *PUSB_CHARGER_PORT;

//
// IOCTL_BATTERY_SET_INFORMATION information levels
//

typedef enum {
    BatteryCriticalBias,
    BatteryCharge,
    BatteryDischarge,
    BatteryChargingSource,
    BatteryChargerId,
    BatteryChargerStatus
} BATTERY_SET_INFORMATION_LEVEL;

//
// Generic Payload for IOCTL_BATTERY_SET_INFORMATION.
//

typedef struct _BATTERY_SET_INFORMATION {
    ULONG                         BatteryTag;
    BATTERY_SET_INFORMATION_LEVEL InformationLevel;
    UCHAR                         Buffer[1];
} BATTERY_SET_INFORMATION, *PBATTERY_SET_INFORMATION;

//
// Payload for IOCTL_BATTERY_SET_INFORMATION information level
// BatteryChargerStatus.
//
// CAD uses this structure to identify charger source properties to the Battery.
//

typedef struct _BATTERY_CHARGER_STATUS
{
    //
    // Identifies the charger type.
    //

    BATTERY_CHARGING_SOURCE_TYPE Type;

    //
    // Start of variable data area. Data contained here will be specific to
    // charging source type identified by Type.
    //

    ULONG VaData[1];

} BATTERY_CHARGER_STATUS, *PBATTERY_CHARGER_STATUS;

//
// Payload for IOCTL_BATTERY_SET_INFORMATION/BatteryChargerStatus (CAD->Battery)
// when Type is set to BatteryChargingSourceType_USB.
//
// CAD uses this structure to identify charger source properties to the Battery.
//

typedef struct _BATTERY_USB_CHARGER_STATUS
{
    //
    // Identifies the charger type.
    // This value will be set to BatteryChargingSourceType_USB.
    //

    BATTERY_CHARGING_SOURCE_TYPE Type;

    //
    // Should not be interpreted by BM.
    //

    ULONG Reserved;

    //
    // Flags.
    //

    ULONG Flags;

    //
    // Indicates maximum current that can be drawn from this charging port in mA.
    // Note that this value may be zero when OemCharger value is not GUID_EMPTY,
    // in such case zero should be interpreted as unknown value.
    //

    ULONG MaxCurrent;

    //
    // Indicates constant voltage available on the charger terminal in mV.
    // Note that this value may be zero when OemCharger value is not GUID_EMPTY,
    // in such case zero should be interpreted as unknown value.
    //

    ULONG Voltage;

    //
    // USB Port Type.
    //

    USB_CHARGER_PORT PortType;

    //
    // Identifies a particular USB port on the system.
    // When set to 0 indicates that port information is unavailable.
    //

    ULONGLONG PortId;

    //
    // Power source specific information:
    //
    // Type                           | PowerSourceInformation
    // -------------------------------|-----------------------
    // BatteryChargingSourceType_USB  | USBFN_PORT_TYPE
    // <other values>                 | <undefined>
    //
    // N.B. On 64-bit systems, the high order 32-bits are always reset and must
    //      be unused. In other words, this value must not exceed 0xffffffff.
    //      This is to ensure semantic compatibility with 32-bit applications on
    //      64-bit systems.
    //

    PVOID PowerSourceInformation;

    //
    // Represents OEM proprietary charger type when not GUID_EMPTY, this value is
    // passed from charging port uninterpreted by CAD.
    //

    BATTERY_CHARGER_ID OemCharger;

} BATTERY_USB_CHARGER_STATUS, *PBATTERY_USB_CHARGER_STATUS;

//
// BATTERY_USB_CHARGER_STATUS.Flags field.
// Indicates Type-C port running in the Default USB mode.
//
#define BATTERY_USB_CHARGER_STATUS_FN_DEFAULT_USB                   0x00000001

//
// BATTERY_USB_CHARGER_STATUS.Flags field.
// Indicates Type-C port has established an explicit contract of Power Delivery with the port partner.
//
#define BATTERY_USB_CHARGER_STATUS_UCM_PD                           0x00000002


typedef struct _BATTERY_WAIT_STATUS {
    ULONG       BatteryTag;
    ULONG       Timeout;
    ULONG       PowerState;
    ULONG       LowCapacity;
    ULONG       HighCapacity;
} BATTERY_WAIT_STATUS, *PBATTERY_WAIT_STATUS;

typedef struct _BATTERY_STATUS {
    ULONG       PowerState;
    ULONG       Capacity;
    ULONG       Voltage;
    LONG        Rate;
} BATTERY_STATUS, *PBATTERY_STATUS;

typedef union _POWER_ADAPTER_POWER_STATES {
    struct {
        ULONG Online : 1;
        ULONG RecState : 2;
        ULONG Reserved : 29;
    } States;

    ULONG AsUlong;
} POWER_ADAPTER_POWER_STATES, *PPOWER_ADAPTER_POWER_STATES;

typedef struct _POWER_ADAPTER_STATUS {
    UCHAR                      Version;
    UCHAR                      Reserved[3];
    POWER_ADAPTER_POWER_STATES PowerState;

    //
    // All power members are in mW.
    //

    ULONG                      PeakPower;
    ULONG                      MaxOutputPower;
    ULONG                      MaxInputPower;
    ULONG64                    RecStartTime;
    ULONG64                    RecEndTime;
} POWER_ADAPTER_STATUS, *PPOWER_ADAPTER_STATUS;

#define POWER_ADAPTER_REC_TIME_NOT_AVAILABLE 0xFFFFFFFFFFFFFFFFULL

typedef struct _POWER_ADAPTER_SET_STATUS_BUFFER {
    UCHAR Version;
    BOOLEAN RecOverride;
    UCHAR Reserved[2];
} POWER_ADAPTER_SET_STATUS_BUFFER, *PPOWER_ADAPTER_SET_STATUS_BUFFER;

//
// Power adapter charge requirements.
// - Type: 1 for USB-C, 0 for others.
// - If an adapter that delivers less than MinimumPower it will not charge the device.
// - If between MinimumPower and NominalPower will charge slowly.
// - MaximumPower should always charge the device.
// - MinimumPower, NominalPower and MinimumPower should be in mW.
//

typedef struct _POWER_ADAPTER_CHARGE_REQUIREMENT {
    ULONG       AcAdapterType;
    ULONG       MinimumPower;
    ULONG       NominalPower;
    ULONG       MaximumPower;
} POWER_ADAPTER_CHARGE_REQUIREMENT, *PPOWER_ADAPTER_CHARGE_REQUIREMENT;

// Battery Status Constants
#define BATTERY_UNKNOWN_VOLTAGE 0xFFFFFFFF
#define BATTERY_UNKNOWN_RATE    0x80000000
#define UNKNOWN_RATE            BATTERY_UNKNOWN_RATE
#define UNKNOWN_VOLTAGE         BATTERY_UNKNOWN_VOLTAGE

// PowerState flags
#define BATTERY_POWER_ON_LINE                       0x00000001
#define BATTERY_DISCHARGING                         0x00000002
#define BATTERY_CHARGING                            0x00000004
#define BATTERY_CRITICAL                            0x00000008

// Max battery driver BATTERY_QUERY_INFORMATION_LEVEL string storage
// size in bytes.
#define MAX_BATTERY_STRING_SIZE 128

// Struct for accessing the packed date format in BatteryManufactureDate.
typedef struct _BATTERY_MANUFACTURE_DATE
{
    UCHAR   Day;
    UCHAR   Month;
    USHORT  Year;
} BATTERY_MANUFACTURE_DATE, *PBATTERY_MANUFACTURE_DATE;

// battery

#define IOCTL_BATTERY_QUERY_TAG         \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x10, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_BATTERY_QUERY_INFORMATION \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x11, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_BATTERY_SET_INFORMATION   \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x12, METHOD_BUFFERED, FILE_WRITE_ACCESS)

#define IOCTL_BATTERY_QUERY_STATUS      \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x13, METHOD_BUFFERED, FILE_READ_ACCESS)

// New ioctl for Windows Phone specific feature: Source Change Notificaton.
#define IOCTL_BATTERY_CHARGING_SOURCE_CHANGE    \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x14, METHOD_BUFFERED, FILE_READ_ACCESS)


#define BATTERY_TAG_INVALID     0

//
// Customized I/O
//

#define IOCTL_QUERY_CUSTOMIZED_IO_CAPABILITIES      \
        CTL_CODE(FILE_DEVICE_BATTERY, 0xA0, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_QUERY_CUSTOMIZED_INPUT_FROM_PLATFORM  \
        CTL_CODE(FILE_DEVICE_BATTERY, 0xA1, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_SEND_CUSTOMIZED_OUTPUT_TO_PLATFORM    \
        CTL_CODE(FILE_DEVICE_BATTERY, 0xA2, METHOD_BUFFERED, FILE_WRITE_ACCESS)

typedef struct _CUSTOMIZED_IO_CAPABILITIES {
    ULONG SupportedInputs;
    ULONG SupportedOutputs;
} CUSTOMIZED_IO_CAPABILITIES, *PCUSTOMIZED_IO_CAPABILITIES;

typedef struct _CUSTOMIZED_IO_QUERY_INPUT_RETURN {
    ULONG FunctionId;
    ULONG ErrorCode;
    ULONG Value;
} CUSTOMIZED_IO_QUERY_INPUT_RETURN, *PCUSTOMIZED_IO_QUERY_INPUT_RETURN;

typedef struct _CUSTOMIZED_IO_SEND_OUTPUT_BUFFER {
    ULONG FunctionId;
    ULONG Value;
} CUSTOMIZED_IO_SEND_OUTPUT_BUFFER, *PCUSTOMIZED_IO_SEND_OUTPUT_BUFFER;

//
// Thermal Zone driver interface (devices of registrying as GUID_DEVICE_THERMAL_ZONE)
//

#define MAX_ACTIVE_COOLING_LEVELS       10

//
// This structure has been depricated and the THERMAL_INFORMATION_EX
// structure should be used. THERMAL_INFORMATION has been left here for
// backward compatibility with the thermal WMI interface.
//
typedef struct _THERMAL_INFORMATION {
    ULONG           ThermalStamp;
    ULONG           ThermalConstant1;
    ULONG           ThermalConstant2;
    KAFFINITY       Processors;
    ULONG           SamplingPeriod;
    ULONG           CurrentTemperature;
    ULONG           PassiveTripPoint;
    ULONG           CriticalTripPoint;
    UCHAR           ActiveTripPointCount;
    ULONG           ActiveTripPoint[MAX_ACTIVE_COOLING_LEVELS];
} THERMAL_INFORMATION, *PTHERMAL_INFORMATION;

#define ACTIVE_COOLING          0x0
#define PASSIVE_COOLING         0x1

typedef struct _THERMAL_WAIT_READ {
    ULONG Timeout;
    ULONG LowTemperature;
    ULONG HighTemperature;
} THERMAL_WAIT_READ, *PTHERMAL_WAIT_READ;

#define THERMAL_WAIT_READ_TIMEOUT_IMMEDIATE         0
#define THERMAL_WAIT_READ_TIMEOUT_NONE              0xFFFFFFFFul

#define TZ_ACTIVATION_REASON_THERMAL      0x00000001
#define TZ_ACTIVATION_REASON_CURRENT      0x00000002

typedef struct _THERMAL_POLICY {
    ULONG           Version;
    BOOLEAN         WaitForUpdate;
    BOOLEAN         Hibernate;
    BOOLEAN         Critical;
    BOOLEAN         ThermalStandby;
    ULONG           ActivationReasons;
    ULONG           PassiveLimit;
    ULONG           ActiveLevel;
    BOOLEAN         OverThrottled;
} THERMAL_POLICY, *PTHERMAL_POLICY;

#define THERMAL_POLICY_VERSION_1          1
#define THERMAL_POLICY_VERSION_2          2

// thermal

#define IOCTL_THERMAL_QUERY_INFORMATION \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x20, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_THERMAL_SET_COOLING_POLICY\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x21, METHOD_BUFFERED, FILE_WRITE_ACCESS)

#define IOCTL_RUN_ACTIVE_COOLING_METHOD\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x22, METHOD_BUFFERED, FILE_WRITE_ACCESS)

#define IOCTL_THERMAL_SET_PASSIVE_LIMIT\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x23, METHOD_BUFFERED, FILE_WRITE_ACCESS)

#define IOCTL_THERMAL_READ_TEMPERATURE\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x24, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_THERMAL_READ_POLICY\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x25, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// Lid class driver functions
//

#define IOCTL_QUERY_LID\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x30, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// Switch class driver functions
//

#define IOCTL_NOTIFY_SWITCH_EVENT\
        CTL_CODE(FILE_DEVICE_BATTERY, 0x40, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// System button class driver functions
//

#define IOCTL_GET_SYS_BUTTON_CAPS       \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x50, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_GET_SYS_BUTTON_EVENT      \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x51, METHOD_BUFFERED, FILE_READ_ACCESS)

#define SYS_BUTTON_POWER        0x00000001
#define SYS_BUTTON_SLEEP        0x00000002
#define SYS_BUTTON_LID          0x00000004
#define SYS_BUTTON_WAKE         0x80000000

//
// Lid-specific state embedded in the button event irp.
//

#define SYS_BUTTON_LID_STATE_MASK 0x00030000
#define SYS_BUTTON_LID_OPEN       0x00010000
#define SYS_BUTTON_LID_CLOSED     0x00020000
#define SYS_BUTTON_LID_INITIAL    0x00040000
#define SYS_BUTTON_LID_CHANGED    0x00080000

//
// Processor object class driver functions
//

typedef struct {
    ULONG   PhysicalID;
    ULONG   PBlkAddress;
    UCHAR   PBlkLength;
} PROCESSOR_OBJECT_INFO, *PPROCESSOR_OBJECT_INFO;

typedef struct {
    ULONG   PhysicalID;
    ULONG   PBlkAddress;
    UCHAR   PBlkLength;
    ULONG   InitialApicId;
} PROCESSOR_OBJECT_INFO_EX, *PPROCESSOR_OBJECT_INFO_EX;



#define IOCTL_GET_PROCESSOR_OBJ_INFO        \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x60, METHOD_BUFFERED, FILE_READ_ACCESS)


#ifndef _WINDOWS_

//
// Thermal client interface (devices implementing
// GUID_THERMAL_COOLING_INTERFACE)
//

typedef
_Function_class_(DEVICE_ACTIVE_COOLING)
VOID
DEVICE_ACTIVE_COOLING (
    _Inout_opt_ PVOID Context,
    _In_ BOOLEAN Engaged
    );

typedef DEVICE_ACTIVE_COOLING *PDEVICE_ACTIVE_COOLING;

typedef
_Function_class_(DEVICE_PASSIVE_COOLING)
VOID
DEVICE_PASSIVE_COOLING (
    _Inout_opt_ PVOID Context,
    _In_ ULONG Percentage
    );

typedef DEVICE_PASSIVE_COOLING *PDEVICE_PASSIVE_COOLING;

typedef struct _THERMAL_COOLING_INTERFACE {

    //
    // generic interface header
    //

    USHORT Size;
    USHORT Version;
    PVOID Context;
    PINTERFACE_REFERENCE    InterfaceReference;
    PINTERFACE_DEREFERENCE  InterfaceDereference;

    //
    // Thermal cooling interface
    //

    ULONG Flags;
    PDEVICE_ACTIVE_COOLING ActiveCooling;
    PDEVICE_PASSIVE_COOLING PassiveCooling;

} THERMAL_COOLING_INTERFACE, *PTHERMAL_COOLING_INTERFACE;


#define THERMAL_COOLING_INTERFACE_VERSION 1

//
// The following THERMAL_DEVICE_* definitions are deprecated and should not be
// used. Use THERMAL_COOLING_* instead.
//

#define THERMAL_DEVICE_INTERFACE_VERSION 1

typedef THERMAL_COOLING_INTERFACE THERMAL_DEVICE_INTERFACE;
typedef THERMAL_COOLING_INTERFACE *PTHERMAL_DEVICE_INTERFACE;

typedef enum _THERMAL_DEVICE_INTERFACE_FLAGS {
    ThermalDeviceFlagActiveCooling = 1,
    ThermalDeviceFlagPassiveCooling = 2
} THERMAL_DEVICE_INTERFACE_FLAGS, *PTHERMAL_DEVICE_INTERFACE_FLAGS;

//
// Power limit types.
//

typedef
_Function_class_(QUERY_POWER_LIMIT_ATTRIBUTES)
NTSTATUS
QUERY_POWER_LIMIT_ATTRIBUTES (
    _Inout_opt_ PVOID Context,
    _In_ ULONG BufferCount,
    _Inout_opt_ PVOID Buffer,
    _Out_ PULONG AttributeCount
    );

typedef QUERY_POWER_LIMIT_ATTRIBUTES *PQUERY_POWER_LIMIT_ATTRIBUTES;

typedef
_Function_class_(SET_POWER_LIMIT)
NTSTATUS
SET_POWER_LIMIT (
    _Inout_opt_ PVOID Context,
    _In_ ULONG ValueCount,
    _In_ PVOID Values
    );

typedef SET_POWER_LIMIT *PSET_POWER_LIMIT;

typedef
_Function_class_(QUERY_POWER_LIMIT)
NTSTATUS
QUERY_POWER_LIMIT (
    _Inout_opt_ PVOID Context,
    _In_ ULONG ValueCount,
    _Inout_ PVOID Values
    );

typedef QUERY_POWER_LIMIT *PQUERY_POWER_LIMIT;

//
// Power limit client interface (devices implementing GUID_POWER_LIMIT_INTERFACE)
//

typedef struct _POWER_LIMIT_INTERFACE {

    //
    // Generic interface header
    //

    USHORT Size;
    USHORT Version;
    PVOID Context;
    PINTERFACE_REFERENCE    InterfaceReference;
    PINTERFACE_DEREFERENCE  InterfaceDereference;

    //
    // Power limit interface
    //

    ULONG DomainCount;
    PQUERY_POWER_LIMIT_ATTRIBUTES QueryAttributes;
    PSET_POWER_LIMIT              SetPowerLimit;
    PQUERY_POWER_LIMIT            QueryPowerLimit;
} POWER_LIMIT_INTERFACE, *PPOWER_LIMIT_INTERFACE;

#define POWER_LIMIT_INTERFACE_VERSION 1

#endif // _WINDOWS_

//
// Message indicator class driver functions
//
#define IOCTL_SET_SYS_MESSAGE_INDICATOR     \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x70, METHOD_BUFFERED, FILE_WRITE_ACCESS)

//
// ACPI Time device functions
//
#define IOCTL_SET_WAKE_ALARM_VALUE      \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x80, METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SET_WAKE_ALARM_POLICY     \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x81, METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_GET_WAKE_ALARM_VALUE      \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x82, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_GET_WAKE_ALARM_POLICY     \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x83, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

typedef struct _WAKE_ALARM_INFORMATION {

    //
    // Timer type, IN.
    //   0x00000000 - AC Timer
    //   0x00000001 - DC Timer
    //

    ULONG TimerIdentifier;

    //
    // Timeout value, IN for SET functions, OUT for GET functions
    //   Values 0x00000000 to 0xfffffffe represent the timeout in seconds.
    //   0xffffffff disables a timer or indicates a timer countdown completion.
    //

    ULONG Timeout;

} WAKE_ALARM_INFORMATION, *PWAKE_ALARM_INFORMATION;

//
// ACPI Real Time Clock functions
//

#define ACPI_TIME_ADJUST_DAYLIGHT 0x01
#define ACPI_TIME_IN_DAYLIGHT 0x02
#define ACPI_TIME_ZONE_UNKNOWN 0x7FF

#define IOCTL_ACPI_GET_REAL_TIME     \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x84, METHOD_BUFFERED, FILE_READ_DATA)

#define IOCTL_ACPI_SET_REAL_TIME     \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x85, METHOD_BUFFERED, FILE_WRITE_ACCESS)

#define IOCTL_GET_WAKE_ALARM_SYSTEM_POWERSTATE    \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x86, METHOD_BUFFERED, FILE_READ_DATA)

typedef struct _ACPI_REAL_TIME {
    UINT16 Year;          // Year : 2010 - 9999
    UINT8 Month;          // Month : 1 - 12
    UINT8 Day;            // Day : 1 - 31
    UINT8 Hour;           // Hour : 0 - 23
    UINT8 Minute;         // Minute : 0 - 59
    UINT8 Second;         // Second : 0 - 59
    UINT8 Valid;          // 0 - Time is not valid (request failed); 1 - Time is valid
    UINT16 Milliseconds;  // Milliseconds :  1-1000
    INT16 TimeZone;       // TimeZone : -1440 to 1440 or 2047
    UINT8 DayLight;       // DayLight : 0-1
    UINT8 Reserved1[3];   // Reserved: Must be 0
} ACPI_REAL_TIME, *PACPI_REAL_TIME;

//
// ACPI Time and Alarm Device Capabilities
//

#define IOCTL_GET_ACPI_TIME_AND_ALARM_CAPABILITIES \
        CTL_CODE(FILE_DEVICE_BATTERY, 0x87, METHOD_BUFFERED, FILE_READ_ACCESS)

typedef enum _ACPI_TIME_RESOLUTION {
    AcpiTimeResolutionMilliseconds = 0,
    AcpiTimeResolutionSeconds,
    AcpiTimeResolutionMax
} ACPI_TIME_RESOLUTION, *PACPI_TIME_RESOLUTION;

typedef struct _ACPI_TIME_AND_ALARM_CAPABILITIES {
    BOOLEAN AcWakeSupported;
    BOOLEAN DcWakeSupported;

    BOOLEAN S4AcWakeSupported;
    BOOLEAN S4DcWakeSupported;

    BOOLEAN S5AcWakeSupported;
    BOOLEAN S5DcWakeSupported;

    BOOLEAN S4S5WakeStatusSupported;

    ULONG DeepestWakeSystemState;

    BOOLEAN RealTimeFeaturesSupported;
    ACPI_TIME_RESOLUTION RealTimeResolution;
} ACPI_TIME_AND_ALARM_CAPABILITIES, *PACPI_TIME_AND_ALARM_CAPABILITIES;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _POCLASS_

