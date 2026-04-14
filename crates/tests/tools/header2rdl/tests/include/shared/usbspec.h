//
//    Copyright (C) Microsoft.  All rights reserved.
//
//
// This header file contains definitions based on information contained
// in the following Universal Serial Bus Specifications:
//
// Universal Serial Bus Specification, Revision 1.1, September 23, 1998
// Universal Serial Bus Specification, Revision 2.0, April 27, 2000
// Universal Serial Bus 3.0 Specification, Revision 1.0, November 12, 2008
// Universal Serial Bus 3.1 Specification, Revision 1.0, July 26, 2013
//
// Refer to the referenced sections in these specifications for further
// information.
//

#ifndef   __USBSPEC_H__
#define   __USBSPEC_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#include <PSHPACK1.H>

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)   // nameless struct/union
#pragma warning(disable:4214)   // bit field types other than int

//
// These definitions are not USB specification definitions and do not
// belong in this header file.  They are only here to preserve
// compatibility with the previous usb200.h header file.
//
typedef enum _USB_DEVICE_SPEED {
    UsbLowSpeed = 0,
    UsbFullSpeed,
    UsbHighSpeed,
    UsbSuperSpeed
} USB_DEVICE_SPEED;

typedef enum _USB_DEVICE_TYPE {
    Usb11Device = 0,
    Usb20Device
} USB_DEVICE_TYPE;



//
// Chapter 9 USB Device Framework
//

//
// USB 1.1: 9.3 USB Device Requests, Table 9-2. Format of Setup Data
// USB 2.1: 9.3 USB Device Requests, Table 9-2. Format of Setup Data
// USB 3.0: 9.3 USB Device Requests, Table 9-2. Format of Setup Data
//
typedef union _BM_REQUEST_TYPE {
    struct _BM {
        UCHAR   Recipient:2;
        UCHAR   Reserved:3;
        UCHAR   Type:2;
        UCHAR   Dir:1;
#ifdef __cplusplus
        } s;
#else
        };
#endif
    UCHAR B;
} BM_REQUEST_TYPE, *PBM_REQUEST_TYPE;

typedef struct _USB_DEFAULT_PIPE_SETUP_PACKET {
    BM_REQUEST_TYPE bmRequestType;
    UCHAR bRequest;

    union _wValue {
        struct {
            UCHAR LowByte;
            UCHAR HiByte;
        };
        USHORT W;
    } wValue;

    union _wIndex {
        struct {
            UCHAR LowByte;
            UCHAR HiByte;
        };
        USHORT W;
    } wIndex;
    USHORT wLength;
} USB_DEFAULT_PIPE_SETUP_PACKET, *PUSB_DEFAULT_PIPE_SETUP_PACKET;

C_ASSERT(sizeof(USB_DEFAULT_PIPE_SETUP_PACKET) == 8);

//
// bmRequestType.Dir
//
#define BMREQUEST_HOST_TO_DEVICE        0
#define BMREQUEST_DEVICE_TO_HOST        1

//
// bmRequestType.Type
#define BMREQUEST_STANDARD              0
#define BMREQUEST_CLASS                 1
#define BMREQUEST_VENDOR                2

//
// bmRequestType.Recipient
//
#define BMREQUEST_TO_DEVICE             0
#define BMREQUEST_TO_INTERFACE          1
#define BMREQUEST_TO_ENDPOINT           2
#define BMREQUEST_TO_OTHER              3

//
// wValue for Get Descriptor request
//
#define USB_DESCRIPTOR_MAKE_TYPE_AND_INDEX(d, i) ((USHORT)((USHORT)d<<8 | i))

//
// USB 1.1: 9.4 Standard Device Requests, Table 9-4. Standard Request Codes
// USB 2.0: 9.4 Standard Device Requests, Table 9-4. Standard Request Codes
//
#define USB_REQUEST_GET_STATUS          0x00
#define USB_REQUEST_CLEAR_FEATURE       0x01
#define USB_REQUEST_SET_FEATURE         0x03
#define USB_REQUEST_SET_ADDRESS         0x05
#define USB_REQUEST_GET_DESCRIPTOR      0x06
#define USB_REQUEST_SET_DESCRIPTOR      0x07
#define USB_REQUEST_GET_CONFIGURATION   0x08
#define USB_REQUEST_SET_CONFIGURATION   0x09
#define USB_REQUEST_GET_INTERFACE       0x0A
#define USB_REQUEST_SET_INTERFACE       0x0B
#define USB_REQUEST_SYNC_FRAME          0x0C

#define USB_REQUEST_GET_FIRMWARE_STATUS 0x1A
#define USB_REQUEST_SET_FIRMWARE_STATUS 0x1B

//
// wValue field options for USB_REQUEST_GET_FIRMWARE_STATUS
//

#define USB_GET_FIRMWARE_ALLOWED_OR_DISALLOWED_STATE    0x00
#define USB_GET_FIRMWARE_HASH                           0x01

#define USB_DEVICE_FIRMWARE_HASH_LENGTH                 32

//
// wValue field options for USB_REQUEST_SET_FIRMWARE_STATUS
//

#define USB_DISALLOW_FIRMWARE_UPDATE    0x00
#define USB_ALLOW_FIRMWARE_UPDATE       0x01

//
// USB 3.0: 9.4 Standard Device Requests, Table 9-4. Standard Request Codes
//
#define USB_REQUEST_SET_SEL             0x30
#define USB_REQUEST_ISOCH_DELAY         0x31

//
// USB 1.1: 9.4 Standard Device Requests, Table 9-5. Descriptor Types
//
#define USB_DEVICE_DESCRIPTOR_TYPE                          0x01
#define USB_CONFIGURATION_DESCRIPTOR_TYPE                   0x02
#define USB_STRING_DESCRIPTOR_TYPE                          0x03
#define USB_INTERFACE_DESCRIPTOR_TYPE                       0x04
#define USB_ENDPOINT_DESCRIPTOR_TYPE                        0x05
//
// USB 2.0: 9.4 Standard Device Requests, Table 9-5. Descriptor Types
//
#define USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE                0x06
#define USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_TYPE       0x07
#define USB_INTERFACE_POWER_DESCRIPTOR_TYPE                 0x08
#define EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE      0x12

//
// USB 3.0: 9.4 Standard Device Requests, Table 9-5. Descriptor Types
//
#define USB_OTG_DESCRIPTOR_TYPE                                     0x09
#define USB_DEBUG_DESCRIPTOR_TYPE                                   0x0A
#define USB_INTERFACE_ASSOCIATION_DESCRIPTOR_TYPE                   0x0B
#define USB_BOS_DESCRIPTOR_TYPE                                     0x0F
#define USB_DEVICE_CAPABILITY_DESCRIPTOR_TYPE                       0x10
#define USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_TYPE           0x30
//
// USB 3.1: 9.4 Standard Device Requests, Table 9-6. Descriptor Types
//
#define USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE 0x31
//
// Legacy definitions, do not use.
//
#define USB_RESERVED_DESCRIPTOR_TYPE                        0x06
#define USB_CONFIG_POWER_DESCRIPTOR_TYPE                    0x07

//
// USB 1.1: 9.4 Standard Device Requests, Table 9-6. Standard Feature Selectors
//
#define USB_FEATURE_ENDPOINT_STALL              0x00
#define USB_FEATURE_REMOTE_WAKEUP               0x01
//
// USB 2.0: 9.4 Standard Device Requests, Table 9-6. Standard Feature Selectors
//
#define USB_FEATURE_TEST_MODE                   0x02
//
// USB 3.0: 9.4 Standard Device Requests, Table 9-6. Standard Feature Selectors
//
#define USB_FEATURE_FUNCTION_SUSPEND            0x00
#define USB_FEATURE_U1_ENABLE                   0x30
#define USB_FEATURE_U2_ENABLE                   0x31
#define USB_FEATURE_LTM_ENABLE                  0x32
//
// USB 3.1: 9.4 Standard Device Requests, Table 9-7. Standard Feature Selectors
//
#define USB_FEATURE_LDM_ENABLE                  0x35

//
// USBPD Rev2.0: 9.3.1 Class-specific Device Requests, Table 9-8 PD. Class Feature Selectors
//
#define USB_FEATURE_BATTERY_WAKE_MASK           0x28
#define USB_FEATURE_OS_IS_PD_AWARE              0x29
#define USB_FEATURE_POLICY_MODE                 0x2A
#define USB_FEATURE_CHARGING_POLICY             0x36

//
// USBPD Rev2.0: 9.4.5.4 CHARGING_POLICY Feature Selector
//
#define USB_CHARGING_POLICY_DEFAULT             0x00
#define USB_CHARGING_POLICY_ICCHPF              0x01
#define USB_CHARGING_POLICY_ICCLPF              0x02
#define USB_CHARGING_POLICY_NO_POWER            0x03

//
// USB 3.1: 10.16.2.6 Get Port Status, Table 10-12. Port Status Type Codes
//
#define USB_STATUS_PORT_STATUS                  0x00
#define USB_STATUS_PD_STATUS                    0x01
#define USB_STATUS_EXT_PORT_STATUS              0x02

//
// USB 1.1: 9.4.5 Get Status, Figure 9-4. Information Returned by a GetStatus() Request to a Device
// USB 2.0: 9.4.5 Get Status, Figure 9-4. Information Returned by a GetStatus() Request to a Device
//
#define USB_GETSTATUS_SELF_POWERED              0x01
#define USB_GETSTATUS_REMOTE_WAKEUP_ENABLED     0x02
//
// USB 3.0: 9.4.5 Get Status, Figure 9-5. Information Returned by a GetStatus() Request to a Device
//
#define USB_GETSTATUS_U1_ENABLE                 0x04
#define USB_GETSTATUS_U2_ENABLE                 0x08
#define USB_GETSTATUS_LTM_ENABLE                0x10

typedef union _USB_DEVICE_STATUS {
    USHORT  AsUshort16;
    struct {
        USHORT  SelfPowered:1;
        USHORT  RemoteWakeup:1;
        USHORT  U1Enable:1;     // (USB 1.1, USB 2.0 Reserved)
        USHORT  U2Enable:1;     // (USB 1.1, USB 2.0 Reserved)
        USHORT  LtmEnable:1;    // (USB 1.1, USB 2.0 Reserved)
        USHORT  Reserved:11;
    };
} USB_DEVICE_STATUS, *PUSB_DEVICE_STATUS;

C_ASSERT(sizeof(USB_DEVICE_STATUS) == sizeof(USHORT));

//
// USB 3.0: 9.4.5 Get Status, Figure 9-6. Information Returned by a GetStatus() Request to an Interface
//
typedef union _USB_INTERFACE_STATUS {
    USHORT  AsUshort16;
    struct {
        USHORT  RemoteWakeupCapable:1;
        USHORT  RemoteWakeupEnabled:1;
        USHORT  Reserved:14;
    };
} USB_INTERFACE_STATUS, *PUSB_INTERFACE_STATUS;

C_ASSERT(sizeof(USB_INTERFACE_STATUS) == sizeof(USHORT));

//
// USB 1.1: 9.4.5 Get Status, Figure 9-6. Information Returned by a GetStatus() Request to an Endpoint
// USB 2.0: 9.4.5 Get Status, Figure 9-6. Information Returned by a GetStatus() Request to an Endpoint
// USB 3.0: 9.4.5 Get Status, Figure 9-7. Information Returned by a GetStatus() Request to an Endpoint
//
typedef union _USB_ENDPOINT_STATUS {
    USHORT  AsUshort16;
    struct {
        USHORT  Halt:1;
        USHORT  Reserved:15;
    };
} USB_ENDPOINT_STATUS, *PUSB_ENDPOINT_STATUS;

C_ASSERT(sizeof(USB_ENDPOINT_STATUS) == sizeof(USHORT));


typedef struct _USB_COMMON_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
} USB_COMMON_DESCRIPTOR, *PUSB_COMMON_DESCRIPTOR;

//
// USB 1.1: 9.6.1 Device, Table 9-7. Standard Device Descriptor
// USB 2.0: 9.6.1 Device, Table 9-8. Standard Device Descriptor
// USB 3.0: 9.6.1 Device, Table 9-8. Standard Device Descriptor
//
typedef struct _USB_DEVICE_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    USHORT  bcdUSB;
    UCHAR   bDeviceClass;
    UCHAR   bDeviceSubClass;
    UCHAR   bDeviceProtocol;
    UCHAR   bMaxPacketSize0;
    USHORT  idVendor;
    USHORT  idProduct;
    USHORT  bcdDevice;
    UCHAR   iManufacturer;
    UCHAR   iProduct;
    UCHAR   iSerialNumber;
    UCHAR   bNumConfigurations;
} USB_DEVICE_DESCRIPTOR, *PUSB_DEVICE_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_DESCRIPTOR) == 18);

//
// With the exception of the HUB device class, USB class codes are not
// defined in the core USB 1.1, 2.0, 3.0 specifications.
//
#define USB_DEVICE_CLASS_RESERVED               0x00
#define USB_DEVICE_CLASS_AUDIO                  0x01
#define USB_DEVICE_CLASS_COMMUNICATIONS         0x02
#define USB_DEVICE_CLASS_HUMAN_INTERFACE        0x03
#define USB_DEVICE_CLASS_MONITOR                0x04
#define USB_DEVICE_CLASS_PHYSICAL_INTERFACE     0x05
#define USB_DEVICE_CLASS_POWER                  0x06
#define USB_DEVICE_CLASS_IMAGE                  0x06
#define USB_DEVICE_CLASS_PRINTER                0x07
#define USB_DEVICE_CLASS_STORAGE                0x08
#define USB_DEVICE_CLASS_HUB                    0x09
#define USB_DEVICE_CLASS_CDC_DATA               0x0A
#define USB_DEVICE_CLASS_SMART_CARD             0x0B
#define USB_DEVICE_CLASS_CONTENT_SECURITY       0x0D
#define USB_DEVICE_CLASS_VIDEO                  0x0E
#define USB_DEVICE_CLASS_PERSONAL_HEALTHCARE    0x0F
#define USB_DEVICE_CLASS_AUDIO_VIDEO            0x10
#define USB_DEVICE_CLASS_BILLBOARD              0x11
#define USB_DEVICE_CLASS_DIAGNOSTIC_DEVICE      0xDC
#define USB_DEVICE_CLASS_WIRELESS_CONTROLLER    0xE0
#define USB_DEVICE_CLASS_MISCELLANEOUS          0xEF
#define USB_DEVICE_CLASS_APPLICATION_SPECIFIC   0xFE
#define USB_DEVICE_CLASS_VENDOR_SPECIFIC        0xFF

//
// USB 2.0: 9.6.2 Device_Qualifier, Table 9-9. Device_Qualifier Descriptor
//
typedef struct _USB_DEVICE_QUALIFIER_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    USHORT  bcdUSB;
    UCHAR   bDeviceClass;
    UCHAR   bDeviceSubClass;
    UCHAR   bDeviceProtocol;
    UCHAR   bMaxPacketSize0;
    UCHAR   bNumConfigurations;
    UCHAR   bReserved;
} USB_DEVICE_QUALIFIER_DESCRIPTOR, *PUSB_DEVICE_QUALIFIER_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_QUALIFIER_DESCRIPTOR) == 10);

//
// USB 3.0: 9.6.2 Binary Device Object Store (BOS), Table 9-9. BOS Descriptor
//
typedef struct _USB_BOS_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    USHORT  wTotalLength;
    UCHAR   bNumDeviceCaps;
} USB_BOS_DESCRIPTOR, *PUSB_BOS_DESCRIPTOR;

C_ASSERT(sizeof(USB_BOS_DESCRIPTOR) == 5);

//
// USB 3.1: 9.6.2 Binary Device Object Store (BOS), Table 9-14. Device Capability Type Codes
//
#define USB_DEVICE_CAPABILITY_WIRELESS_USB                0x01
#define USB_DEVICE_CAPABILITY_USB20_EXTENSION             0x02
#define USB_DEVICE_CAPABILITY_SUPERSPEED_USB              0x03
#define USB_DEVICE_CAPABILITY_CONTAINER_ID                0x04
#define USB_DEVICE_CAPABILITY_PLATFORM                    0x05
#define USB_DEVICE_CAPABILITY_POWER_DELIVERY              0x06
#define USB_DEVICE_CAPABILITY_BATTERY_INFO                0x07
#define USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT            0x08
#define USB_DEVICE_CAPABILITY_PD_PROVIDER_PORT            0x09
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB          0x0A
#define USB_DEVICE_CAPABILITY_PRECISION_TIME_MEASUREMENT  0x0B
#define USB_DEVICE_CAPABILITY_BILLBOARD                   0x0D
#define USB_DEVICE_CAPABILITY_FIRMWARE_STATUS             0x11

//
// USB 2.0 ECN: Link Power Management (LPM), 3. Framework: USB Device Capabilities - USB 2.0 Extension,
//     Table 3-1. USB Device Capabilities - USB 2.0 Extension Descriptor
// USB 3.0: 9.6.2.1 USB 2.0 Extension, Table 9-12. USB 2.0 Extension Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    union {
        ULONG       AsUlong;
        struct {
            ULONG   Reserved:1;
            ULONG   LPMCapable:1;
            ULONG   BESLAndAlternateHIRDSupported:1;
            ULONG   BaselineBESLValid:1;
            ULONG   DeepBESLValid:1;
            ULONG   Reserved1:3;
            ULONG   BaselineBESL:4;
            ULONG   DeepBESL:4;
            ULONG   Reserved2:16;
        };
    }       bmAttributes;
} USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR) == 7);

#define USB_DEVICE_CAPABILITY_USB20_EXTENSION_BMATTRIBUTES_RESERVED_MASK 0xFFFF00E1

//
// USBPD Rev2.0: 9.2.1 USB Power Delivery Capability Descriptor, Table 9-2. USB Power Delivery Capability Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bReserved;
    union {
        ULONG       AsUlong;
        struct {
            ULONG   Reserved1:1;
            ULONG   BatteryCharging:1;
            ULONG   USBPowerDelivery:1;
            ULONG   Provider:1;
            ULONG   Consumer:1;
            ULONG   ChargingPolicy:1;
            ULONG   TypeCCurrent:1;
            ULONG   Reserved2:1;
            ULONG   ACSupply:1;
            ULONG   Battery:1;
            ULONG   Other:1;
            ULONG   NumBatteries:3;
            ULONG   UsesVbus:1;
            ULONG   Reserved3:17;
        };
    }        bmAttributes;
    USHORT   bmProviderPorts;
    USHORT   bmConsumerPorts;
    USHORT   bcdBCVersion;
    USHORT   bcdPDVersion;
    USHORT   bcdUSBTypeCVersion;
} USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR) == 18);

//
// USBPD Rev2.0: 9.2.3 PD Consumer Port Capability Descriptor, Table 9-4. PD Consumer Port Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bReserved;
    union {
        USHORT       AsUshort;
        struct {
            USHORT   BatteryCharging:1;
            USHORT   USBPowerDelivery:1;
            USHORT   USBTypeCCurrent:1;
            USHORT   Reserved:13;
        };
    }        bmCapabilities;
    USHORT   wMinVoltage;
    USHORT   wMaxVoltage;
    USHORT   wReserved;
    ULONG    dwMaxOperatingPower;
    ULONG    dwMaxPeakPower;
    ULONG    dwMaxPeakPowerTime;
} USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR) == 24);

//
// USB 3.0: 9.6.2.2 SuperSpeed USB Device Capability, Table 9-13. SuperSpeed Device Capabilities Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bmAttributes;           // needs bitfield definitions
    USHORT  wSpeedsSupported;       // needs bitfield definitions
    UCHAR   bFunctionalitySupport;
    UCHAR   bU1DevExitLat;
    USHORT  wU2DevExitLat;
} USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR;

#define USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_RESERVED_MASK 0xFD
#define USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_LTM_CAPABLE   0x02

#define USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_RESERVED_MASK  0xFFF0
#define USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_LOW            0x0001
#define USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_FULL           0x0002
#define USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_HIGH           0x0004
#define USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_SUPER          0x0008

#define USB_DEVICE_CAPABILITY_SUPERSPEED_U1_DEVICE_EXIT_MAX_VALUE 0x0A

#define USB_DEVICE_CAPABILITY_SUPERSPEED_U2_DEVICE_EXIT_MAX_VALUE 0x07FF

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR) == 10);

#define USB_DEVICE_CAPABILITY_MAX_U1_LATENCY    0x0A
#define USB_DEVICE_CAPABILITY_MAX_U2_LATENCY    0x07FF

//
// USB 3.1: 9.6.2.5 SuperSpeedPlus USB Device Capability, Table 9-19. SuperSpeedPlus Device Capabilities Descriptor
//
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_BPS              0
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_KBPS             1
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_MBPS             2
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_GBPS             3

#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_SYMMETRIC       0
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_ASYMMETRIC      1
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_RX               0
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_TX               1

#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SS          0
#define USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SSP         1

typedef union _USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    ULONG   AsUlong32;
    struct {
        ULONG   SublinkSpeedAttrID:4;
        ULONG   LaneSpeedExponent:2;
        ULONG   SublinkTypeMode:1;
        ULONG   SublinkTypeDir:1;
        ULONG   Reserved:6;
        ULONG   LinkProtocol:2;
        ULONG   LaneSpeedMantissa:16;
    };
} USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED, *PUSB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED;

typedef struct _USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bReserved;
    union {
        ULONG        AsUlong;
        struct {
            ULONG    SublinkSpeedAttrCount:5;
            ULONG    SublinkSpeedIDCount:4;
            ULONG    Reserved:23;
        };
    }       bmAttributes;
    union {
        USHORT       AsUshort;
        struct {
            USHORT   SublinkSpeedAttrID:4;
            USHORT   Reserved:4;
            USHORT   MinRxLaneCount:4;
            USHORT   MinTxLaneCount:4;
        };
    }       wFunctionalitySupport;
    USHORT  wReserved;
    USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED   bmSublinkSpeedAttr[1];
} USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR) == 16);

//
// USB 3.0: 9.6.2.3 Container ID, Table 9-14. Container ID Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bReserved;
    UCHAR   ContainerID[16];
} USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR) == 20);

//
// USB Device Capability Platform Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bReserved;
    GUID    PlatformCapabilityUuid;
    UCHAR   CapabililityData[1];
} USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR) == 21);

//
// USB Billboard 1.0: Section 3.1.5.2, Table 3-6. Billboard Capability Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   iAddtionalInfoURL;
    UCHAR   bNumberOfAlternateModes;
    UCHAR   bPreferredAlternateMode;
    union {
        USHORT       AsUshort;
        struct {
            USHORT   VConnPowerNeededForFullFunctionality:3;
            USHORT   Reserved:12;
            USHORT   NoVconnPowerRequired:1;
        };
    } VconnPower;
    UCHAR   bmConfigured[32];
    ULONG   bReserved;
    struct {
        USHORT  wSVID;
        UCHAR   bAlternateMode;
        UCHAR   iAlternateModeSetting;
    } AlternateMode[1];
} USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR) == 48);

//
// USB 3.2 ECN: USB FW Update
//
typedef struct _USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
    UCHAR   bcdDescriptorVersion;
    union {
        ULONG        AsUlong;
        struct {
            ULONG    GetFirmwareImageHashSupport:1;
            ULONG    DisallowFirmwareUpdateSupport:1;
            ULONG    Reserved:30;
        };
    }       bmAttributes;
} USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR) == 8);

// {D8DD60DF-4589-4CC7-9CD2-659D9E648A9F}
DEFINE_GUID(GUID_USB_MSOS20_PLATFORM_CAPABILITY_ID,
0xD8DD60DF, 0x4589, 0x4CC7, 0x9C, 0xD2, 0x65, 0x9D, 0x9E, 0x64, 0x8A, 0x9F);

//
// USB 3.0: 9.6.2 Binary Device Object Store, Table 9-10. Device Capability Descriptor
//
typedef struct _USB_DEVICE_CAPABILITY_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bDevCapabilityType;
} USB_DEVICE_CAPABILITY_DESCRIPTOR, *PUSB_DEVICE_CAPABILITY_DESCRIPTOR;

C_ASSERT(sizeof(USB_DEVICE_CAPABILITY_DESCRIPTOR) == 3);
//
// USB 1.1: 9.6.2 Configuration, Table 9-8. Standard Configuration Descriptor
// USB 2.0: 9.6.3 Configuration, Table 9-10. Standard Configuration Descriptor
// USB 3.0: 9.6.3 Configuration, Table 9-15. Standard Configuration Descriptor
//
typedef struct _USB_CONFIGURATION_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    USHORT  wTotalLength;
    UCHAR   bNumInterfaces;
    UCHAR   bConfigurationValue;
    UCHAR   iConfiguration;
    UCHAR   bmAttributes;
    UCHAR   MaxPower;
} USB_CONFIGURATION_DESCRIPTOR, *PUSB_CONFIGURATION_DESCRIPTOR;

C_ASSERT(sizeof(USB_CONFIGURATION_DESCRIPTOR) == 9);

//
// Configuration Descriptor bmAttributes bit definitions
//
#define USB_CONFIG_POWERED_MASK                   0xC0
#define USB_CONFIG_BUS_POWERED                    0x80
#define USB_CONFIG_SELF_POWERED                   0x40
#define USB_CONFIG_REMOTE_WAKEUP                  0x20
#define USB_CONFIG_RESERVED                       0x1F

//
// USB 2.0 ECN: USB ECN : Interface Association Descriptor, 9.X.Y Interface Association,
//     Table 9-Z. Standard Interface Association Descriptor
// USB 3.0: 9.6.4 Interface Association, Table 9-16. Standard Interface Association Descriptor
//
typedef struct _USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bFirstInterface;
    UCHAR   bInterfaceCount;
    UCHAR   bFunctionClass;
    UCHAR   bFunctionSubClass;
    UCHAR   bFunctionProtocol;
    UCHAR   iFunction;
} USB_INTERFACE_ASSOCIATION_DESCRIPTOR, *PUSB_INTERFACE_ASSOCIATION_DESCRIPTOR;

C_ASSERT(sizeof(USB_INTERFACE_ASSOCIATION_DESCRIPTOR) == 8);

//
// USB 1.1: 9.6.3 Interface, Table 9-9. Standard Interface Descriptor
// USB 2.0: 9.6.5 Interface, Table 9-12. Standard Interface Descriptor
// USB 3.0: 9.6.5 Interface, Table 9-17. Standard Interface Descriptor
//
typedef struct _USB_INTERFACE_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bInterfaceNumber;
    UCHAR   bAlternateSetting;
    UCHAR   bNumEndpoints;
    UCHAR   bInterfaceClass;
    UCHAR   bInterfaceSubClass;
    UCHAR   bInterfaceProtocol;
    UCHAR   iInterface;
} USB_INTERFACE_DESCRIPTOR, *PUSB_INTERFACE_DESCRIPTOR;

C_ASSERT(sizeof(USB_INTERFACE_DESCRIPTOR) == 9);

//
// USB 1.1: 9.6.4 Endpoint, Table 9-10. Standard Endpoint Descriptor
// USB 2.0: 9.6.6 Endpoint, Table 9-13. Standard Endpoint Descriptor
// USB 3.0: 9.6.6 Endpoint, Table 9-18. Standard Endpoint Descriptor
//
typedef struct _USB_ENDPOINT_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bEndpointAddress;
    UCHAR   bmAttributes;
    USHORT  wMaxPacketSize;
    UCHAR   bInterval;
} USB_ENDPOINT_DESCRIPTOR, *PUSB_ENDPOINT_DESCRIPTOR;

C_ASSERT(sizeof(USB_ENDPOINT_DESCRIPTOR) == 7);

//
// USB_ENDPOINT_DESCRIPTOR bEndpointAddress bit 7
//
#define USB_ENDPOINT_DIRECTION_MASK               0x80
#define USB_ENDPOINT_DIRECTION_OUT(addr)          (!((addr) & USB_ENDPOINT_DIRECTION_MASK))
#define USB_ENDPOINT_DIRECTION_IN(addr)           ((addr) & USB_ENDPOINT_DIRECTION_MASK)

#define USB_ENDPOINT_ADDRESS_MASK                 0x0F

//
// USB_ENDPOINT_DESCRIPTOR bmAttributes bits 0-1
//
#define USB_ENDPOINT_TYPE_MASK                    0x03
#define USB_ENDPOINT_TYPE_CONTROL                 0x00
#define USB_ENDPOINT_TYPE_ISOCHRONOUS             0x01
#define USB_ENDPOINT_TYPE_BULK                    0x02
#define USB_ENDPOINT_TYPE_INTERRUPT               0x03

//
// USB_ENDPOINT_DESCRIPTOR bmAttributes bits 7-2
//

#define USB_ENDPOINT_TYPE_BULK_RESERVED_MASK      0xFC
#define USB_ENDPOINT_TYPE_CONTROL_RESERVED_MASK   0xFC
#define USB_20_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK   0xFC
#define USB_30_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK   0xCC
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_RESERVED_MASK    0xC0

#define USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK         0x30
#define USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_PERIODIC     0x00
#define USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_NOTIFICATION 0x10
#define USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED10   0x20
#define USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED11   0x30
#define USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE(bmAttr)      (bmAttr & USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK)

#define USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK               0x0C
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_NO_SYNCHRONIZATION 0x00
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ASYNCHRONOUS       0x04
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ADAPTIVE           0x08
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_SYNCHRONOUS        0x0C
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION(bmAttr)            (bmAttr & USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK)

#define USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK                            0x30
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_DATA_ENDOINT                    0x00
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_FEEDBACK_ENDPOINT               0x10
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_IMPLICIT_FEEDBACK_DATA_ENDPOINT 0x20
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_RESERVED                        0x30
#define USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE(bmAttr)                         (bmAttr & USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK)

//
// USB_ENDPOINT_DESCRIPTOR wMaxPacketSize
//
typedef union _USB_HIGH_SPEED_MAXPACKET {
    struct _MP {
        USHORT  MaxPacket:11;   // 0-10
        USHORT  HSmux:2;        // 11-12
        USHORT  Reserved:3;     // 13-15
    };
    USHORT us;
} USB_HIGH_SPEED_MAXPACKET, *PUSB_HIGH_SPEED_MAXPACKET;

C_ASSERT(sizeof(USB_HIGH_SPEED_MAXPACKET) == 2);

#define USB_ENDPOINT_SUPERSPEED_BULK_MAX_PACKET_SIZE       1024
#define USB_ENDPOINT_SUPERSPEED_CONTROL_MAX_PACKET_SIZE     512
#define USB_ENDPOINT_SUPERSPEED_ISO_MAX_PACKET_SIZE        1024
#define USB_ENDPOINT_SUPERSPEED_INTERRUPT_MAX_PACKET_SIZE  1024

//
// USB 1.1: 9.6.5 String, Table 9-12. UNICODE String Descriptor
// USB 2.0: 9.6.7 String, Table 9-16. UNICODE String Descriptor
// USB 3.0: 9.6.8 String, Table 9-22. UNICODE String Descriptor
//
typedef struct _USB_STRING_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    WCHAR   bString[1];
} USB_STRING_DESCRIPTOR, *PUSB_STRING_DESCRIPTOR;

#define MAXIMUM_USB_STRING_LENGTH   255

//
// USB 3.0: 9.6.7 SuperSpeed Endpoint Companion, Table 9-20. SuperSpeed Endpoint Companion Descriptor
//
typedef struct _USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    UCHAR       bLength;
    UCHAR       bDescriptorType;
    UCHAR       bMaxBurst;
    union {
        UCHAR       AsUchar;
        struct {
            UCHAR   MaxStreams:5;
            UCHAR   Reserved1:3;
        } Bulk;
        struct {
            UCHAR   Mult:2;
            UCHAR   Reserved2:5;
            UCHAR   SspCompanion:1;
        } Isochronous;
    }           bmAttributes;
    USHORT      wBytesPerInterval;
} USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR, *PUSB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR;

C_ASSERT(sizeof(USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR) == 6);

#define USB_SUPERSPEED_ISOCHRONOUS_MAX_MULTIPLIER 2

//
// USB 3.1: 9.6.8 SuperSpeedPlus Isoch Endpoint Companion, Table 9-27. SuperSpeedPlus Isoch Endpoint Companion Descriptor
//
typedef struct _USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    UCHAR       bLength;
    UCHAR       bDescriptorType;
    USHORT      wReserved;
    ULONG       dwBytesPerInterval;
} USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR, *PUSB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR;

C_ASSERT(sizeof(USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR) == 8);

#define USB_SUPERSPEEDPLUS_ISOCHRONOUS_MIN_BYTESPERINTERVAL 0xC001
#define USB_SUPERSPEEDPLUS_ISOCHRONOUS_MAX_BYTESPERINTERVAL 0xFFFFFF

//
// USB 2.0: 9.6.6.1	eUSB2 Isochronous Endpoint Companion Descriptor, Table 9-X. eUSB2 Isochronous Endpoint Companion Descriptor
//
typedef struct _EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    UCHAR       bLength;
    UCHAR       bDescriptorType;
    USHORT      wMaxPacketSize;
    ULONG       dwBytesPerInterval;
} EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR , *PEUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR;

C_ASSERT(sizeof(EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR) == 8);

#define USB_HIGHSPEED_EUSB2_ISOCHRONOUS_MIN_BYTESPERINTERVAL 0x0C01
#define USB_HIGHSPEED_EUSB2_ISOCHRONOUS_MAX_BYTESPERINTERVAL 0x1800


//
// Chapter 11 Hub Specification (USB 1.1, USB 2.0)
// Chapter 10 Hub, Host Downstream Port, and Device Upstream Port Specification (3.0)
//

//
// USB 1.1: 11.15.2.1 Hub Descriptor, Table 11-8. Hub Descriptor
// USB 2.0: 11.23.2.1 Hub Descriptor, Table 11-13. Hub Descriptor
//
typedef struct _USB_HUB_DESCRIPTOR {
    UCHAR   bDescriptorLength;
    UCHAR   bDescriptorType;
    UCHAR   bNumberOfPorts;
    USHORT  wHubCharacteristics;
    UCHAR   bPowerOnToPowerGood;
    UCHAR   bHubControlCurrent;
    UCHAR   bRemoveAndPowerMask[64];
} USB_HUB_DESCRIPTOR, *PUSB_HUB_DESCRIPTOR;

#define USB_20_HUB_DESCRIPTOR_TYPE      0x29

//
// USB 3.0: 10.13.2.1 Hub Descriptor, Table 10-3. SuperSpeed Hub Descriptor
//
typedef struct _USB_30_HUB_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bNumberOfPorts;
    USHORT  wHubCharacteristics;
    UCHAR   bPowerOnToPowerGood;
    UCHAR   bHubControlCurrent;
    UCHAR   bHubHdrDecLat;
    USHORT  wHubDelay;
    USHORT  DeviceRemovable;
} USB_30_HUB_DESCRIPTOR, *PUSB_30_HUB_DESCRIPTOR;

#define USB_30_HUB_DESCRIPTOR_TYPE      0x2A

//
// USB 1.1: 11.16.2 Class-specific Requests, Table 11-11. Hub Class Request Codes
//
#define USB_REQUEST_GET_STATE           0x02
//
// USB 2.0: 11.24.2 Class-specific Requests, Table 11-16. Hub Class Request Codes
//
#define USB_REQUEST_CLEAR_TT_BUFFER     0x08
#define USB_REQUEST_RESET_TT            0x09
#define USB_REQUEST_GET_TT_STATE        0x0A
#define USB_REQUEST_STOP_TT             0x0B
//
// USB 3.0: 10.14.2 Class-specific Requests, Table 10-6. Hub Class Request Codes
//
#define USB_REQUEST_SET_HUB_DEPTH       0x0C
#define USB_REQUEST_GET_PORT_ERR_COUNT  0x0D

//
// USB 1.1: 11.16.2 Class-specific Requests, Table 11-12. Hub Class Feature Selectors
// USB 2.0: 11.24.2 Class-specific Requests, Table 11-17. Hub Class Feature Selectors
// USB 3.0: 10.14.2 Class-specific Requests, Table 10-7. Hub Class Feature Selectors
//
/*
typedef enum _USB_HUB_FEATURE_SELECTOR {
    C_HUB_LOCAL_POWER       = 0,
    C_HUB_OVER_CURRENT      = 1
} USB_HUB_FEATURE_SELECTOR, *PUSB_HUB_FEATURE_SELECTOR;

typedef enum _USB_PORT_FEATURE_SELECTOR {
    PORT_CONNECTION         = 0,
    PORT_ENABLE             = 1,
    PORT_SUSPEND            = 2,
    PORT_OVER_CURRENT       = 3,
    PORT_RESET              = 4,
    PORT_LINK_STATE         = 5,
    PORT_POWER              = 8,
    PORT_LOW_SPEED          = 9,
    C_PORT_CONNECTION       = 16,
    C_PORT_ENABLE           = 17,
    C_PORT_SUSPEND          = 18,
    C_PORT_OVER_CURRENT     = 19,
    C_PORT_RESET            = 20,
    PORT_TEST               = 21,
    PORT_INDICATOR          = 22,
    PORT_U1_TIMEOUT         = 23,
    PORT_U2_TIMEOUT         = 24,
    C_PORT_LINK_STATE       = 25,
    C_PORT_CONFIG_ERROR     = 26,
    PORT_REMOTE_WAKE_MASK   = 27,
    BH_PORT_RESET           = 28,
    C_BH_PORT_RESET         = 29,
    FORCE_LINKPM_ACCEPT     = 30
} USB_PORT_FEATURE_SELECTOR, *PUSB_PORT_FEATURE_SELECTOR;
*/

//
// USB 1.1: 11.16.2.5 Get Hub Status, Table 11-13. Hub Status Field, wHubStatus
// USB 2.0: 11.24.2.6 Get Hub Status, Table 11-19. Hub Status Field, wHubStatus
// USB 3.0, 10.14.2.4 Get Hub Status, Table 10-8. Hub Status Field, wHubStatus
//
typedef union _USB_HUB_STATUS {
    USHORT  AsUshort16;
    struct {
        USHORT  LocalPowerLost:1;
        USHORT  OverCurrent:1;
        USHORT  Reserved:14;
    };
} USB_HUB_STATUS, *PUSB_HUB_STATUS;

C_ASSERT(sizeof(USB_HUB_STATUS) == sizeof(USHORT));

//
// USB 1.1: 11.16.2.5 Get Hub Status, Table 11-14. Hub Change Field, wHubChange
// USB 2.0: 11.24.2.6 Get Hub Status, Table 11-20. Hub Change Field, wHubChange
// USB 3.0, 10.14.2.4 Get Hub Status, Table 10-9. Hub Change Field, wHubChange
//
typedef union _USB_HUB_CHANGE {
    USHORT  AsUshort16;
    struct {
        USHORT  LocalPowerChange:1;
        USHORT  OverCurrentChange:1;
        USHORT  Reserved:14;
    };
} USB_HUB_CHANGE, *PUSB_HUB_CHANGE;

C_ASSERT(sizeof(USB_HUB_CHANGE) == sizeof(USHORT));

typedef union _USB_HUB_STATUS_AND_CHANGE {
    ULONG   AsUlong32;
    struct {
        USB_HUB_STATUS      HubStatus;          // 0-15
        USB_HUB_CHANGE      HubChange;          // 16-32
    };
} USB_HUB_STATUS_AND_CHANGE, *PUSB_HUB_STATUS_AND_CHANGE;

C_ASSERT(sizeof(USB_HUB_STATUS_AND_CHANGE) == sizeof(ULONG));

//
// USB 1.1: 11.16.2.6.1 Port Status Bits, Table 11-15. Port Status Field, wPortStatus
// USB 2.0: 11.24.2.7.1 Port Status Bits, Table 11-21. Port Status Field, wPortStatus
//
typedef union _USB_20_PORT_STATUS {
    USHORT   AsUshort16;
    struct {
        USHORT  CurrentConnectStatus:1;         // 0
        USHORT  PortEnabledDisabled:1;          // 1
        USHORT  Suspend:1;                      // 2
        USHORT  OverCurrent:1;                  // 3
        USHORT  Reset:1;                        // 4
        USHORT  L1:1;                           // 5
        USHORT  Reserved0:2;                    // 6-7
        USHORT  PortPower:1;                    // 8
        USHORT  LowSpeedDeviceAttached:1;       // 9
        USHORT  HighSpeedDeviceAttached:1;      // 10   (USB 1.1 Reserved)
        USHORT  PortTestMode:1;                 // 11   (USB 1.1 Reserved)
        USHORT  PortIndicatorControl:1;         // 12   (USB 1.1 Reserved)
        USHORT  Reserved1:3;                    // 13-15
    };
}  USB_20_PORT_STATUS, *PUSB_20_PORT_STATUS;

C_ASSERT(sizeof(USB_20_PORT_STATUS) == sizeof(USHORT));

#define USB_PORT_STATUS_CONNECT         0x0001
#define USB_PORT_STATUS_ENABLE          0x0002
#define USB_PORT_STATUS_SUSPEND         0x0004
#define USB_PORT_STATUS_OVER_CURRENT    0x0008
#define USB_PORT_STATUS_RESET           0x0010
#define USB_PORT_STATUS_POWER           0x0100
#define USB_PORT_STATUS_LOW_SPEED       0x0200
#define USB_PORT_STATUS_HIGH_SPEED      0x0400

//
// USB 1.1: 11.16.2.6.2 Port Status Change Bits, Table 11-16. Port Change Field, wPortChange
// USB 2.0: 11.24.2.7.2 Port Status Change Bits, Table 11-22. Port Change Field, wPortChange
//
typedef union _USB_20_PORT_CHANGE {
    USHORT   AsUshort16;
    struct {
        USHORT  ConnectStatusChange:1;          // 0
        USHORT  PortEnableDisableChange:1;      // 1
        USHORT  SuspendChange:1;                // 2
        USHORT  OverCurrentIndicatorChange:1;   // 3
        USHORT  ResetChange:1;                  // 4
        USHORT  Reserved2:11;                   // 5-15
    };
} USB_20_PORT_CHANGE, *PUSB_20_PORT_CHANGE;

C_ASSERT(sizeof(USB_20_PORT_CHANGE) == sizeof(USHORT));

//
// USB 3.0: 10.14.2.6.1 Port Status Bits, Table 10-10. Port Status Field, wPortStatus
//
typedef union _USB_30_PORT_STATUS {
    USHORT   AsUshort16;
    struct {
        USHORT  CurrentConnectStatus:1;         // 0
        USHORT  PortEnabledDisabled:1;          // 1
        USHORT  Reserved0:1;                    // 2
        USHORT  OverCurrent:1;                  // 3
        USHORT  Reset:1;                        // 4
        USHORT  PortLinkState:4;                // 5-8
        USHORT  PortPower:1;                    // 9
        USHORT  NegotiatedDeviceSpeed:3;        // 10-12
        USHORT  Reserved1:3;                    // 13-15
    };
} USB_30_PORT_STATUS, *PUSB_30_PORT_STATUS;

C_ASSERT(sizeof(USB_30_PORT_STATUS) == sizeof(USHORT));

#define PORT_LINK_STATE_U0                      0
#define PORT_LINK_STATE_U1                      1
#define PORT_LINK_STATE_U2                      2
#define PORT_LINK_STATE_U3                      3
#define PORT_LINK_STATE_DISABLED                4
#define PORT_LINK_STATE_RX_DETECT               5
#define PORT_LINK_STATE_INACTIVE                6
#define PORT_LINK_STATE_POLLING                 7
#define PORT_LINK_STATE_RECOVERY                8
#define PORT_LINK_STATE_HOT_RESET               9
#define PORT_LINK_STATE_COMPLIANCE_MODE         10
#define PORT_LINK_STATE_LOOPBACK                11
#define PORT_LINK_STATE_TEST_MODE               11 // xHCI-specific, replacing LOOPBACK

//
// USB 3.0: 10.14.2.6.2 Port Status Change Bits, Table 10-11. Port Change Field, wPortChange
//
typedef union _USB_30_PORT_CHANGE {
    USHORT   AsUshort16;
    struct {
        USHORT  ConnectStatusChange:1;          // 0
        USHORT  Reserved2:2;                    // 1-2
        USHORT  OverCurrentIndicatorChange:1;   // 3
        USHORT  ResetChange:1;                  // 4
        USHORT  BHResetChange:1;                // 5
        USHORT  PortLinkStateChange:1;          // 6
        USHORT  PortConfigErrorChange:1;        // 7
        USHORT  Reserved3:8;                    // 8-15
    };
} USB_30_PORT_CHANGE, *PUSB_30_PORT_CHANGE;

C_ASSERT(sizeof(USB_30_PORT_CHANGE) == sizeof(USHORT));

typedef union _USB_PORT_STATUS {
    USHORT  AsUshort16;
    USB_20_PORT_STATUS  Usb20PortStatus;
    USB_30_PORT_STATUS  Usb30PortStatus;
} USB_PORT_STATUS, *PUSB_PORT_STATUS;

C_ASSERT(sizeof(USB_PORT_STATUS) == sizeof(USHORT));

typedef union _USB_PORT_CHANGE {
    USHORT  AsUshort16;
    USB_20_PORT_CHANGE  Usb20PortChange;
    USB_30_PORT_CHANGE  Usb30PortChange;
} USB_PORT_CHANGE, *PUSB_PORT_CHANGE;

C_ASSERT(sizeof(USB_PORT_CHANGE) == sizeof(USHORT));

typedef union _USB_PORT_EXT_STATUS {
    ULONG   AsUlong32;
    struct {
        ULONG  RxSublinkSpeedID:4;
        ULONG  TxSublinkSpeedID:4;
        ULONG  RxLaneCount:4;
        ULONG  TxLaneCount:4;
        ULONG  Reserved:16;
    };
} USB_PORT_EXT_STATUS, *PUSB_PORT_EXT_STATUS;

C_ASSERT(sizeof(USB_PORT_EXT_STATUS) == sizeof(ULONG));

typedef union _USB_PORT_STATUS_AND_CHANGE {
    ULONG   AsUlong32;
    struct {
        USB_PORT_STATUS     PortStatus;         // 0-15
        USB_PORT_CHANGE     PortChange;         // 16-31
    };
} USB_PORT_STATUS_AND_CHANGE, *PUSB_PORT_STATUS_AND_CHANGE;

C_ASSERT(sizeof(USB_PORT_STATUS_AND_CHANGE) == sizeof(ULONG));

typedef union _USB_PORT_EXT_STATUS_AND_CHANGE {
    ULONG64   AsUlong64;
    struct {
        USB_PORT_STATUS_AND_CHANGE  PortStatusChange; // 0-31
        USB_PORT_EXT_STATUS         PortExtStatus;    // 32-63
    };
} USB_PORT_EXT_STATUS_AND_CHANGE, *PUSB_PORT_EXT_STATUS_AND_CHANGE;

C_ASSERT(sizeof(USB_PORT_EXT_STATUS_AND_CHANGE) == sizeof(ULONG64));

//
// USB 3.0: 10.14.2.10 Set Port Feature, Table 10-14. Downstream Port Remote Wake Mask Encoding
//
typedef union _USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    UCHAR   AsUchar8;
    struct {
        UCHAR   ConnectRemoteWakeEnable:1;      // 0
        UCHAR   DisconnectRemoteWakeEnable:1;   // 1
        UCHAR   OverCurrentRemoteWakeEnable:1;  // 2
        UCHAR   Reserved0:5;                    // 3-7
    };
} USB_HUB_30_PORT_REMOTE_WAKE_MASK, *PUSB_HUB_30_PORT_REMOTE_WAKE_MASK;

C_ASSERT(sizeof(USB_HUB_30_PORT_REMOTE_WAKE_MASK) == sizeof(UCHAR));

//
// USB 3.0: 9.4.9 Set Feature, Table 9-7. Suspend Options
//
typedef union _USB_FUNCTION_SUSPEND_OPTIONS {
    UCHAR  AsUchar;
    struct {
        UCHAR  PowerState:1;                    // 0
        UCHAR  RemoteWakeEnabled:1;             // 1
        UCHAR  Reserved:6;                      // 2-7
    };
} USB_FUNCTION_SUSPEND_OPTIONS, *PUSB_FUNCTION_SUSPEND_OPTIONS;

C_ASSERT(sizeof(USB_FUNCTION_SUSPEND_OPTIONS) == sizeof(UCHAR));

//
// USB Interface Power Management Specification definitions.
//
// The USB Interface Power Management Specification was never released
// and these definitions should not be used.  They are only included
// here to preserve compatibility with the previous usb100.h header
// file.
//
#define USB_FEATURE_INTERFACE_POWER_D0  0x0002
#define USB_FEATURE_INTERFACE_POWER_D1  0x0003
#define USB_FEATURE_INTERFACE_POWER_D2  0x0004
#define USB_FEATURE_INTERFACE_POWER_D3  0x0005

#define USB_SUPPORT_D0_COMMAND          0x01
#define USB_SUPPORT_D1_COMMAND          0x02
#define USB_SUPPORT_D2_COMMAND          0x04
#define USB_SUPPORT_D3_COMMAND          0x08

#define USB_SUPPORT_D1_WAKEUP           0x10
#define USB_SUPPORT_D2_WAKEUP           0x20

typedef struct _USB_CONFIGURATION_POWER_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   SelfPowerConsumedD0[3];
    UCHAR   bPowerSummaryId;
    UCHAR   bBusPowerSavingD1;
    UCHAR   bSelfPowerSavingD1;
    UCHAR   bBusPowerSavingD2;
    UCHAR   bSelfPowerSavingD2;
    UCHAR   bBusPowerSavingD3;
    UCHAR   bSelfPowerSavingD3;
    USHORT  TransitionTimeFromD1;
    USHORT  TransitionTimeFromD2;
    USHORT  TransitionTimeFromD3;
} USB_CONFIGURATION_POWER_DESCRIPTOR, *PUSB_CONFIGURATION_POWER_DESCRIPTOR;

typedef struct _USB_INTERFACE_POWER_DESCRIPTOR {
    UCHAR   bLength;
    UCHAR   bDescriptorType;
    UCHAR   bmCapabilitiesFlags;
    UCHAR   bBusPowerSavingD1;
    UCHAR   bSelfPowerSavingD1;
    UCHAR   bBusPowerSavingD2;
    UCHAR   bSelfPowerSavingD2;
    UCHAR   bBusPowerSavingD3;
    UCHAR   bSelfPowerSavingD3;
    USHORT  TransitionTimeFromD1;
    USHORT  TransitionTimeFromD2;
    USHORT  TransitionTimeFromD3;
} USB_INTERFACE_POWER_DESCRIPTOR, *PUSB_INTERFACE_POWER_DESCRIPTOR;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#include <POPPACK.H>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif
