/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    usb4dbgioctl.h

Abstract:

    The USB4 debug interface definitions.

--*/

#pragma once

#include <pshpack1.h>


//
// USB4 Specfication Chapter 6.1
//

#define USB4_MAX_DEPTH 6

//
// USB4 Specfication Chapter 6.4.2.3 Table 6-2
//

typedef enum {

    USB4PathConfigurationSpace = 0,
    USB4AdapterConfigurationSpace = 1,
    USB4RouterConfigurationSpace = 2,
    USB4CounterConfigurationSpace = 3

} USB4_CONFIG_SPACE_TYPE;

//
// USB4 Specfication Chapter 6.5 Table 6-11
//

typedef enum {

    ErrConn = 0,
    ErrLink = 1,
    ErrAddr = 2,
    ErrAdp = 4,
    HpAck = 7,
    ErrEnum = 8,
    ErrNua = 9,
    ErrLen = 11,
    ErrHec = 12,
    ErrFc = 13,
    ErrPlug = 14,
    ErrLock = 15,
    DpBw = 32,

    //
    // V2 events
    //

    RopCmplt = 33,
    PopCmplt = 34,
    PcieWake = 35,
    DpConChange = 36,
    DpTxDiscovery = 37,
    LinkRecovery = 38,
    AsymLink = 39,

    //
    // The following are not spec defined but are useful for
    // software to convey additional status information from
    // the HRD to the DRD when completing requests.
    //

    PollingSkipped = 252,
    PollingTimeout = 253,
    StatusSuccess = 254, // Nominal success
    StatusUnknown = 255  // Nominal failure

} USB4_STATUS;

#define USB4_CONFIGURATION_REGISTERS_DW_LENGTH 60

// {981FCA05-60D3-4BB3-898E-497C580C4FB3}
DEFINE_GUID(USB4_HRD_DEBUG_INTERFACE,
    0x981fca05, 0x60d3, 0x4bb3, 0x89, 0x8e, 0x49, 0x7c, 0x58, 0xc, 0x4f, 0xb3);

#define USB4_HRD_DEBUG_INTERFACE_REFERENCE_STRING           L"\\DEBUGINTERFACE"

//
// USB4 HRD Debug IOCTL Function Codes
//

#define USB4_HRD_DEBUG_FUNCTION_READ_CONFIGURATION_SPACE    1131

#define IOCTL_USB4_HRD_DEBUG_READ_CONFIGURATION_SPACE  \
    CTL_CODE(FILE_DEVICE_USB4,\
    USB4_HRD_DEBUG_FUNCTION_READ_CONFIGURATION_SPACE,  \
    METHOD_BUFFERED,  \
    FILE_ANY_ACCESS)

typedef struct _USB4_HRD_DEBUG_ROUTE_STRING
{

    UINT8   Depth;
    UINT8   Route[USB4_MAX_DEPTH + 1];

} USB4_HRD_DEBUG_ROUTE_STRING, *PUSB4_HRD_DEBUG_ROUTE_STRING;

//
// Input buffer
//

typedef struct _USB4_HRD_DEBUG_READ_CONFIGURATION_SPACE_INPUT {

    USB4_HRD_DEBUG_ROUTE_STRING Route;
    UINT8                       AdapterNumber;
    USB4_CONFIG_SPACE_TYPE      ConfigurationSpaceType;
    UINT32                      DwOffset;
    UINT32                      DwLength;

} USB4_HRD_DEBUG_READ_CONFIGURATION_SPACE_INPUT,
  *PUSB4_HRD_DEBUG_READ_CONFIGURATION_SPACE_INPUT;

//
// Output buffer
//

typedef struct _USB4_HRD_DEBUG_READ_CONFIGURATION_SPACE_OUTPUT {

    USB4_STATUS     Usb4Status;
    UINT32          Data[USB4_CONFIGURATION_REGISTERS_DW_LENGTH];

} USB4_HRD_DEBUG_READ_CONFIGURATION_SPACE_OUTPUT,
  *PUSB4_HRD_DEBUG_READ_CONFIGURATION_SPACE_OUTPUT;


#include <poppack.h>

