/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    hwn.h

Abstract:

    Header file that defines structures, prototypes and definitions required
    by consumers of Hardware Notification services.

Environment:

    Kernel mode or User mode

--*/

#ifndef __HWN_H__
#define __HWN_H__

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef __cplusplus
extern "C" {
#endif

//
// Define IOCTLs to get and set Hardware Notifications.
//

#define IOCTL_HWN_SET_STATE                                           \
    CTL_CODE(FILE_DEVICE_UNKNOWN, 0, METHOD_BUFFERED, FILE_WRITE_ACCESS)

#define IOCTL_HWN_GET_STATE                                           \
    CTL_CODE(FILE_DEVICE_UNKNOWN, 1, METHOD_BUFFERED, FILE_READ_ACCESS |  \
             FILE_WRITE_ACCESS)

DEFINE_GUID (HWN_DEVINTERFACE,
    0x81ca6602, 0xdae3, 0x476f, 0x8e, 0x3a, 0x60, 0x76, 0x6b, 0x2f, 0xff, 0x1b);

DEFINE_GUID (HWN_DEVINTERFACE_VIBRATOR,
    0x825e8fd3, 0x6467, 0x4a5b, 0xab, 0xfb, 0xbb, 0x01, 0x07, 0x34, 0xff, 0x4d);

DEFINE_GUID (HWN_DEVINTERFACE_NLED,
    0x6b2a25e2, 0xaaf5, 0x482c, 0x99, 0xa5, 0x62, 0x05, 0xcd, 0xcc, 0x17, 0x6a);

#define HWN_SUPPORTED         0xAAAA
#define HWN_NOT_SUPPORTED     0x5555
#define HWN_CURRENT_MTE_NOT_SUPPORTED 0xFFFFFFFF

//
// The various states devices can be set to
//

typedef enum _HWN_STATE
{
    HWN_OFF = 0,
    HWN_ON = 1,
    HWN_BLINK = 2,
    HWN_TOTAL_STATES
} HWN_STATE;

//
// The various types of devices supported
//

typedef enum _HWN_TYPE
{
    HWN_UNKNOWN = 0,
    HWN_LED = 1,
    HWN_VIBRATOR = 2,
    HWN_TOTAL_TYPES
} HWN_TYPE;

//
// Defines the capabilities or settings a client driver may support.
// Specifically, it is used as the index to HwNSettings[] member of the
// HWN_SETTINGS structure.
//

typedef enum _HWN_SETTINGS_OPTIONS
{
    HWN_INTENSITY = 0,            // HwN intensity (0 - 100%)
    HWN_PERIOD = 1,               // HwN period in milliseconds
    HWN_DUTY_CYCLE = 2,           // HwN duty cycle (0 - 100%)
    HWN_CYCLE_COUNT = 3,          // HwN number of repetitions of cycle
    HWN_CYCLE_GRANULARITY = 4,    // HwN adjustment granularity - read only
    HWN_CURRENT_MTE_RESERVED = 5, // HwN device current in mA for MTE use only
    HWN_TOTAL_SETTINGS
} HWN_SETTINGS_OPTIONS;

//
// A structure for getting data from or setting data to a device
//

typedef struct _HWN_SETTINGS
{
    ULONG HwNId;                            // HwN ID number, 0 is first
    HWN_TYPE HwNType;                       // specifies type of a device
    ULONG HwNSettings[HWN_TOTAL_SETTINGS];  // holds the current device's settings
    HWN_STATE OffOnBlink;                   // 0-off, 1-on, 2-blink
} HWN_SETTINGS, *PHWN_SETTINGS;

#define HWN_SETTINGS_SIZE (sizeof(HWN_SETTINGS))

//
// Defines a structure for a header of each data packet
//
// Data packet format:
//
// |-------------------|
// | HWN_HEADER        |
// |-------------------|
// | HWN_SETTINGS[0]   |
// |-------------------|
// | HWN_SETTINGS[1]   |
// |-------------------|
// | HWN_SETTINGS[N]   |
// |-------------------|
//

typedef struct _HWN_HEADER
{
    ULONG HwNPayloadSize;       //Total size of a payload
    ULONG HwNPayloadVersion;    //Version number of a payload
    ULONG HwNRequests;          //Number of requests in a payload
    HWN_SETTINGS HwNSettingsInfo[ANYSIZE_ARRAY];
} HWN_HEADER, *PHWN_HEADER;

#define HWN_HEADER_SIZE (sizeof(HWN_HEADER)-sizeof(HWN_SETTINGS))

#ifdef __cplusplus
}
#endif

#endif

#endif
