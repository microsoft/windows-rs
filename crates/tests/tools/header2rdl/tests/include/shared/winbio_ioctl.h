/*++

Copyright (c) 2007  Microsoft Corporation

Module Name:

    winbio_ioctl.h

Abstract:
    Biometric device class/port IOCTL codes. This file is required for all code
    user mode and kernel mode, using Biometric IOCTL's, defines, data structures

Revision History:

--*/

#if (NTDDI_VERSION >= NTDDI_WIN7)

#include "winbio_types.h"

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#ifndef FILE_DEVICE_BIOMETRIC
#define FILE_DEVICE_BIOMETRIC   0x00000044
#else
#if 0x00000044 != FILE_DEVICE_BIOMETRIC
#error "Incorrect Biometric Device Definition"
#endif
#endif

//
// {E2B5183A-99EA-4cc3-AD6B-80CA8D715B80}
//
DEFINE_GUID( GUID_DEVINTERFACE_BIOMETRIC_READER,
             0xe2b5183a, 0x99ea, 0x4cc3, 0xad, 0x6b, 0x80, 0xca, 0x8d, 0x71, 0x5b, 0x80);


//
///////////////////////////////////////////////////////////////////////////////
//
//  Biometric Device IOCTLs
//

#define BIO_CTL_CODE(code)                      CTL_CODE(FILE_DEVICE_BIOMETRIC, \
                                                (code), \
                                                METHOD_BUFFERED, \
                                                FILE_ANY_ACCESS)

//
// Mandatory IOCTLs
//
#define IOCTL_BIOMETRIC_GET_ATTRIBUTES                  BIO_CTL_CODE(0x001)
#define IOCTL_BIOMETRIC_RESET                           BIO_CTL_CODE(0x002)
#define IOCTL_BIOMETRIC_CALIBRATE                       BIO_CTL_CODE(0x003)
#define IOCTL_BIOMETRIC_GET_SENSOR_STATUS               BIO_CTL_CODE(0x004)
#define IOCTL_BIOMETRIC_CAPTURE_DATA                    BIO_CTL_CODE(0x005)

//
// Optional IOCTL for updating the firmware
//
#define IOCTL_BIOMETRIC_UPDATE_FIRMWARE                 BIO_CTL_CODE(0x006)

//
// Optional IOCTL for retrieving supported hash algorthims
// Capability:  WINBIO_CAPABILITY_PROCESSING, WINBIO_CAPABILITY_ENCRYPTION
//
#define IOCTL_BIOMETRIC_GET_SUPPORTED_ALGORITHMS        BIO_CTL_CODE(0x007)

//
// Optional IOCTLs to set indicator status
// Capability:  WINBIO_CAPABILITY_INDICATOR
//
#define IOCTL_BIOMETRIC_GET_INDICATOR                   BIO_CTL_CODE(0x008)
#define IOCTL_BIOMETRIC_SET_INDICATOR                   BIO_CTL_CODE(0x009)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

//
// Optional IOCTL to retrieve sensor-type information
// in a vendor-specific format
//
#define IOCTL_BIOMETRIC_GET_PRIVATE_SENSOR_TYPE         BIO_CTL_CODE(0x00A)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// Optional IOCTLs for support of the Secure Connection Protocol (SCP) V1
// Capability: WINBIO_CAPABILITY_SCP_V1
//
#define IOCTL_BIOMETRIC_CONNECT_SECURE                  BIO_CTL_CODE(0x00B)
#define IOCTL_BIOMETRIC_CAPTURE_ENCRYPTED_DATA          BIO_CTL_CODE(0x00C)

//
// Optional IOCTL for wake-on-touch support.
// Capability: WINBIO_CAPABILITY_WAKE
// The WBDI driver must hold this IOCTL in non-power-managed IO queue.
// The Windows Biometric Framework will send this IOCTL on transition to a low
// power state. When the sensor wishes to wake the host, it should complete this
// IOCTL.
//
#define IOCTL_BIOMETRIC_NOTIFY_WAKE                    BIO_CTL_CODE(0x00D)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// Vendor control codes are specified at 0x800:
//
//
#define IOCTL_BIOMETRIC_VENDOR                          CTL_CODE(FILE_DEVICE_BIOMETRIC, 0x800, 0, 0)


//
///////////////////////////////////////////////////////////////////////////////
//
// IOCTL payload and type definitions
//

//
// WINBIO version
//

#define WINBIO_WBDI_MAJOR_VERSION 0x1
#define WINBIO_WBDI_MINOR_VERSION 0x0


//
// OUT payload for IOCTL_BIOMETRIC_GET_ATTRIBUTES
//
typedef struct _WINBIO_SENSOR_ATTRIBUTES {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_VERSION WinBioVersion;
    WINBIO_BIOMETRIC_TYPE SensorType;
    WINBIO_BIOMETRIC_SENSOR_SUBTYPE SensorSubType;
    WINBIO_CAPABILITIES Capabilities;
    WINBIO_STRING ManufacturerName;
    WINBIO_STRING ModelName;
    WINBIO_STRING SerialNumber;
    WINBIO_VERSION FirmwareVersion;
    DWORD SupportedFormatEntries;
    WINBIO_REGISTERED_FORMAT SupportedFormat[1]; // Defined as a length of 1 to avoid compiler warning.
} WINBIO_SENSOR_ATTRIBUTES, *PWINBIO_SENSOR_ATTRIBUTES;

//
// The WINBIO_DATA structure associates a length, in
// bytes, with an arbitrary block of contiguous memory.
//
typedef struct _WINBIO_DATA{
    DWORD Size;
    BYTE Data[1]; // Defined as a length of 1 to avoid compiler warning.
} WINBIO_DATA, *PWINBIO_DATA;


//
// IN payload for IOCTL_BIOMETRIC_UPDATE_FIRMWARE
//
typedef struct _WINBIO_UPDATE_FIRMWARE {
    DWORD PayloadSize;
    WINBIO_DATA FirmwareData;
} WINBIO_UPDATE_FIRMWARE, *PWINBIO_UPDATE_FIRMWARE;


//
// OUT payload for IOCTL_BIOMETRIC_CALIBRATE
//
typedef struct _WINBIO_CALIBRATION_INFO {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_DATA CalibrationData;
} WINBIO_CALIBRATION_INFO, *PWINBIO_CALIBRATION_INFO;


//
// OUT payload for IOCTL_BIOMETRIC_GET_SENSOR_STATUS
//
typedef struct _WINBIO_DIAGNOSTICS {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_SENSOR_STATUS SensorStatus;
    WINBIO_DATA VendorDiagnostics;
} WINBIO_DIAGNOSTICS, *PWINBIO_DIAGNOSTICS;


//
// OUT payload for IOCTL_BIOMETRIC_RESET
// OUT payload for IOCTL_BIOMETRIC_UPDATE_FIRMWARE
//
typedef struct _WINBIO_BLANK_PAYLOAD {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
} WINBIO_BLANK_PAYLOAD, *PWINBIO_BLANK_PAYLOAD;


//
// Parameters for a capture operation.
// IN payload for IOCTL_BIOMETRIC_CAPTURE_DATA
//

typedef struct _WINBIO_CAPTURE_PARAMETERS {
    DWORD PayloadSize;
    WINBIO_BIR_PURPOSE Purpose;
    WINBIO_REGISTERED_FORMAT Format;
    WINBIO_UUID VendorFormat;
    WINBIO_BIR_DATA_FLAGS Flags;
} WINBIO_CAPTURE_PARAMETERS, *PWINBIO_CAPTURE_PARAMETERS;

//
// A container for biometric captured data.
// OUT payload for IOCTL_BIOMETRIC_CAPTURE_DATA
//
typedef struct _WINBIO_CAPTURE_DATA {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_SENSOR_STATUS SensorStatus;
    WINBIO_REJECT_DETAIL RejectDetail;
    WINBIO_DATA CaptureData;
} WINBIO_CAPTURE_DATA, *PWINBIO_CAPTURE_DATA;


//
// OUT payload for IOCTL_BIOMETRIC_GET_SUPPORTED_ALGORITHMS
//
typedef struct _WINBIO_SUPPORTED_ALGORITHMS {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    DWORD NumberOfAlgorithms;
    WINBIO_DATA AlgorithmData;
} WINBIO_SUPPORTED_ALGORITHMS, *PWINBIO_SUPPORTED_ALGORITHMS;


//
// OUT payload for IOCTL_GET_INDICATOR
//
typedef struct _WINBIO_GET_INDICATOR {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_INDICATOR_STATUS IndicatorStatus;
} WINBIO_GET_INDICATOR, *PWINBIO_GET_INDICATOR;


//
// IN payload for IOCTL_SET_INDICATOR
//
typedef struct _WINBIO_SET_INDICATOR {
    DWORD PayloadSize;
    WINBIO_INDICATOR_STATUS IndicatorStatus;
} WINBIO_SET_INDICATOR, *PWINBIO_SET_INDICATOR;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

//
// OUT payload for IOCTL_BIOMETRIC_GET_PRIVATE_SENSOR_TYPE
//
typedef struct _WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_DATA PrivateSensorTypeInfo;
} WINBIO_PRIVATE_SENSOR_TYPE_INFO, *PWINBIO_PRIVATE_SENSOR_TYPE_INFO;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// IN payload for IOCTL_BIOMETRIC_CONNECT_SECURE is a WINBIO_SECURE_CONNECTION_PARAMS structure
//

//
// OUT payload for IOCTL_BIOMETRIC_CONNECT_SECURE is a WINBIO_SECURE_CONNECTION_DATA structure
//

//
// IN payload for IOCTL_BIOMETRIC_CAPTURE_ENCRYPTED_DATA
//
typedef struct _WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    DWORD PayloadSize;
    WINBIO_BIR_PURPOSE Purpose;
    WINBIO_REGISTERED_FORMAT Format;
    WINBIO_UUID VendorFormat;
    WINBIO_BIR_DATA_FLAGS Flags;
    DWORD NonceSize;
    // Nonce[NonceSize]
} WINBIO_ENCRYPTED_CAPTURE_PARAMS, *PWINBIO_ENCRYPTED_CAPTURE_PARAMS;

//
// OUT payload for IOCTL_BIOMETRIC_CAPTURE_ENCRYPTED_DATA is a WINBIO_CAPTURE_DATA structure
//

//
// OUT payload for IOCTL_BIOMETRIC_NOTIFY_WAKE
//
typedef struct _WINBIO_NOTIFY_WAKE {
    DWORD PayloadSize;
    HRESULT WinBioHresult;
    WINBIO_WAKE_REASON Reason;
} WINBIO_NOTIFY_WAKE, *PWINBIO_NOTIFY_WAKE;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
