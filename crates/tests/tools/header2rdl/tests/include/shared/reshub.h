/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

    THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY
    KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR
    PURPOSE.

Module Name:

    reshub.h

Abstract:

    Defines the structures, prototypes and definitions required by consumers of
    Resource Hub services.

--*/

#ifndef __RES_HUB_H__
#define __RES_HUB_H__


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef RESHUB_USE_HELPER_ROUTINES

//
// Fix for value-less UMDF_DRIVER identifier in UMDF 1.x.
//
#if defined(__wudfddi_h__) && defined(UMDF_DRIVER)
#undef UMDF_DRIVER
#define UMDF_DRIVER 1
#endif

#ifdef UMDF_DRIVER
#if (UMDF_DRIVER == 1)
#include <winternl.h>
#endif
#include <strsafe.h>
#else
#include <ntstrsafe.h>
#endif

#endif // RESHUB_USE_HELPER_ROUTINES

#ifdef __cplusplus
extern "C" {
#endif

//
// Define the name for the Resource Hub device.
//

#ifdef UMDF_DRIVER
#define RESOURCE_HUB_DEVICE_NAME L"\\\\.\\RESOURCE_HUB"
#else
#define RESOURCE_HUB_DEVICE_NAME L"\\Device\\RESOURCE_HUB"
#endif

//
// Define the symbolic name target for the Resource Hub device for use by UMDF
// drivers.

#define RESOURCE_HUB_SYMBOLIC_NAME L"\\DosDevices\\RESOURCE_HUB"


#define RESOURCE_HUB_DEVICE_NAME_PREFIX RESOURCE_HUB_DEVICE_NAME L"\\"

//
// For connections, the file name to open a provider via the resource hub is a
// 16-digit hexadecimal representation of the ID.
//
// RESOURCE_HUB_CONNECTION_FILE_SIZE and RESOURCE_HUB_CONNECTION_PATH_SIZE both
// include space for NULL termination.
//

#define RESOURCE_HUB_CONNECTION_FILE_SIZE \
    ((sizeof(LARGE_INTEGER) * 2 * sizeof(WCHAR)) + sizeof(UNICODE_NULL))

#define RESOURCE_HUB_CONNECTION_PATH_SIZE      \
    (sizeof(RESOURCE_HUB_DEVICE_NAME_PREFIX) + \
     RESOURCE_HUB_CONNECTION_FILE_SIZE -       \
     sizeof(UNICODE_NULL))

#define RESOURCE_HUB_CONNECTION_FILE_CHARS \
    ((RESOURCE_HUB_CONNECTION_FILE_SIZE + sizeof(WCHAR) - 1) / sizeof(WCHAR))

#define RESOURCE_HUB_CONNECTION_PATH_CHARS \
    ((RESOURCE_HUB_CONNECTION_PATH_SIZE + sizeof(WCHAR) - 1) / sizeof(WCHAR))

#define RESOURCE_HUB_FILE_SIZE RESOURCE_HUB_CONNECTION_FILE_SIZE
#define RESOURCE_HUB_PATH_SIZE RESOURCE_HUB_CONNECTION_PATH_SIZE
#define RESOURCE_HUB_FILE_CHARS RESOURCE_HUB_CONNECTION_FILE_CHARS
#define RESOURCE_HUB_PATH_CHARS RESOURCE_HUB_CONNECTION_PATH_CHARS



#define FILE_DEVICE_RESOURCE_HUB (FILE_DEVICE_BUS_EXTENDER)

//
// Definitions to interact with the resource hub.
//

#define IOCTL_RH_QUERY_CONNECTION_PROPERTIES \
    CTL_CODE(FILE_DEVICE_RESOURCE_HUB,       \
             0x0,                            \
             METHOD_BUFFERED,                \
             FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_RH_ALLOCATE_CONNECTION            \
    CTL_CODE(FILE_DEVICE_RESOURCE_HUB,          \
             0x1,                               \
             METHOD_BUFFERED,                   \
             FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_RH_FREE_CONNECTION                \
    CTL_CODE(FILE_DEVICE_RESOURCE_HUB,          \
             0x2,                               \
             METHOD_BUFFERED,                   \
             FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_RH_UPDATE_CONNECTION_PROPERTIES   \
    CTL_CODE(FILE_DEVICE_RESOURCE_HUB,          \
             0x3,                               \
             METHOD_BUFFERED,                   \
             FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY   \
    CTL_CODE(FILE_DEVICE_RESOURCE_HUB,              \
             0x4,                                   \
             METHOD_BUFFERED,                       \
             FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY  \
    CTL_CODE(FILE_DEVICE_RESOURCE_HUB,              \
             0x5,                                   \
             METHOD_BUFFERED,                       \
             FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define RH_QUERY_CONNECTION_PROPERTIES_INPUT_VERSION 1
#define RH_QUERY_CONNECTION_PROPERTIES_OUTPUT_VERSION 1

#define RH_ALLOCATE_CONNECTION_INPUT_VERSION 1
#define RH_ALLOCATE_CONNECTION_OUTPUT_VERSION 1

#define RH_FREE_CONNECTION_INPUT_VERSION 1
#define RH_FREE_CONNECTION_OUTPUT_VERSION 1

#define RH_UPDATE_CONNECTION_PROPERTIES_INPUT_VERSION 1
#define RH_UPDATE_CONNECTION_PROPERTIES_OUTPUT_VERSION 1

#define RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_VERSION 1
#define RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_VERSION 1

#define RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_VERSION 1
#define RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_VERSION 1

/*------------------------------------------------------------------------------
 *
 * Update Connection Masks
 *
 *----------------------------------------------------------------------------*/

 #define RH_UPDATE_CONNECTIONAL_MASK_ALL 0xFFFF

/*------------------------------------------------------------------------------
 *
 * Connection property definitions
 *
 *----------------------------------------------------------------------------*/

//
// Function Config
//

#define FUNCTION_CONFIG_DESCRIPTOR 0x8d

//
// Function config descriptor revision and type field values.
//

#define PNP_FUNCTION_CONFIG_DESCRIPTOR_REVISION 0x1
#define PNP_FUNCTION_CONFIG_DESCRIPTOR_REVISION_MINIMUM 0x1

//
// GPIO
//

#define GPIO_INTERRUPT_IO_DESCRIPTOR 0x8c

#ifndef INVALID_PIN_NUMBER

#define INVALID_PIN_NUMBER 0xFFFF

#endif

//
// GPIO descriptor revision and type field values.
//

#define PNP_GPIO_IRQ_DESCRIPTOR_REVISION 0x1
#define PNP_GPIO_IRQ_DESCRIPTOR_TYPE_INTERRUPT 0x0
#define PNP_GPIO_IRQ_DESCRIPTOR_TYPE_IO 0x1
#define PNP_GPIO_IRQ_DESCRIPTOR_REVISION_MINIMUM (0x1)

//
// IRQ Flags for GPIO interrupt descriptor.
//

#define PNP_GPIO_IRQ_RESOURCE_CONSUMER_ONLY 0x01
#define PNP_GPIO_IRQ_MODE                   0x01
#define PNP_GPIO_IRQ_POLARITY               0x06
#define PNP_GPIO_IRQ_SHARED                 0x08
#define PNP_GPIO_IRQ_WAKE_HINT              0x10

#ifndef PNP_GPIO_IRQ_MODE_EDGE

#define PNP_GPIO_IRQ_MODE_EDGE 0x01
#define PNP_GPIO_IRQ_MODE_LEVEL 0x00
#define PNP_GPIO_IRQ_POLARITY_LOW 0x02
#define PNP_GPIO_IRQ_POLARITY_HIGH 0x00
#define PNP_GPIO_IRQ_POLARITY_BOTH 0x04

#endif

//
// Serial Peripheral Bus
//

#define SERIAL_BUS_DESCRIPTOR 0x8e
#define SERIAL_BUS_DESCRIPTOR_REVISION 0x01
#define SERIAL_BUS_DESCRIPTOR_REVISION_V2 0x02
#define SERIAL_BUS_I2C_DESCRIPTOR_TYPE 1
#define SERIAL_BUS_SPI_DESCRIPTOR_TYPE 2
#define SERIAL_BUS_UART_DESCRIPTOR_TYPE 3

//
// General flags for SPB resources.
//

#define SERIAL_BUS_FLAG_SHARED_DESCRIPTOR (0x4)

/*------------------------------------------------------------------------------
 *
 * IOCTL_RH_QUERY_CONNECTION_PROPERTIES parameters
 *
 *----------------------------------------------------------------------------*/

typedef enum _RH_QUERY_CONNECTION_PROPERTIES_INPUT_TYPE {
    ConnectionIdType,
    InterruptVectorType
} RH_QUERY_CONNECTION_PROPERTIES_INPUT_TYPE,
  *PRH_QUERY_CONNECTION_PROPERTIES_INPUT_TYPE;

typedef struct _RH_QUERY_CONNECTION_PROPERTIES_INPUT_BUFFER {
    ULONG Version;
    RH_QUERY_CONNECTION_PROPERTIES_INPUT_TYPE QueryType;
    union {
        LARGE_INTEGER ConnectionId;
        ULONG InterruptVector;
   } u;

} RH_QUERY_CONNECTION_PROPERTIES_INPUT_BUFFER,
  *PRH_QUERY_CONNECTION_PROPERTIES_INPUT_BUFFER;

typedef struct _RH_QUERY_CONNECTION_PROPERTIES_OUTPUT_BUFFER {
    ULONG Version;
    ULONG PropertiesLength;
    UCHAR ConnectionProperties[ANYSIZE_ARRAY];

} RH_QUERY_CONNECTION_PROPERTIES_OUTPUT_BUFFER,
  *PRH_QUERY_CONNECTION_PROPERTIES_OUTPUT_BUFFER;

/*------------------------------------------------------------------------------
*
* IOCTL_RH_ALLOCATE_CONNECTION and IOCTL_RH_UPDATE_CONNECTION_PROPERTIES
* parameters
*
*----------------------------------------------------------------------------*/

typedef enum _RH_CONNECTION_PROPERTIES_INPUT_TYPE {
    GpioIoType,
    GpioInterruptType,
    SpiConnectionType,
    I2cConnectionType,
    UartConnectionType
} RH_CONNECTION_PROPERTIES_INPUT_TYPE,
  *PRH_CONNECTION_PROPERTIES_INPUT_TYPE;

//
// _RH_GPIO_IO_CONNECTION_PARAMETERS only includes mutable parameters.
// Fields not present in this structure must not be changed.
//

typedef struct _RH_GPIO_IO_CONNECTION_PARAMETERS {
    LARGE_INTEGER ConnectionId;

    //
    // The UpdateMask field is used to determine which parameters
    // should be updated. Currently, all parameters are updated
    // by default.
    //

    USHORT UpdateMask;

    //
    // Configurable parameters.
    //

    UCHAR PinConfiguration;
    USHORT DebounceTimeout;
    USHORT DriveStrength;
} RH_GPIO_IO_CONNECTION_PARAMETERS,
  *PRH_GPIO_IO_CONNECTION_PARAMETERS;

//
// _RH_GPIO_INTERRUPT_CONNECTION_PARAMETERS only includes mutable parameters.
// Fields not present in this structure must not be changed.
//

typedef struct _RH_GPIO_INTERRUPT_CONNECTION_PARAMETERS {
    ULONG InterruptVector;

    //
    // The UpdateMask field is used to determine which parameters
    // should be updated. Currently, all parameters are updated
    // by default.
    //

    USHORT UpdateMask;

    //
    // Configurable parameters.
    //

     UCHAR InterruptMode;
     UCHAR InterruptPolarity;
     UCHAR PinConfiguration;
     USHORT DebounceTimeout;
} RH_GPIO_INTERRUPT_CONNECTION_PARAMETERS,
  *PRH_GPIO_INTERRUPT_CONNECTION_PARAMETERS;

//
// _RH_SPI_CONNECTION_PARAMETERS only includes mutable parameters.
// Fields not present in this structure must not be changed.
//

typedef struct _RH_SPI_CONNECTION_PARAMETERS {
    LARGE_INTEGER ConnectionId;

    //
    // The UpdateMask field is used to determine which parameters
    // should be updated. Currently, all parameters are updated
    // by default.
    //

    USHORT UpdateMask;

    //
    // Configurable parameters.
    //

    USHORT DeviceSelection;
    ULONG ConnectionSpeed;
    UCHAR ClockPolarity;
    UCHAR ClockPhase;
    UCHAR DevicePolarity;
    UCHAR DataBitLength;
} RH_SPI_CONNECTION_PARAMETERS,
  *PRH_SPI_CONNECTION_PARAMETERS;

//
// _RH_I2C_CONNECTION_PARAMETERS only includes mutable parameters.
// Fields not present in this structure must not be changed.
//

typedef struct _RH_I2C_CONNECTION_PARAMETERS {
    LARGE_INTEGER ConnectionId;

    //
    // The UpdateMask field is used to determine which parameters
    // should be updated. Currently, all parameters are updated
    // by default.
    //

    USHORT UpdateMask;

    //
    // Configurable parameters.
    //

    USHORT TypeSpecificFlags;
    ULONG ConnectionSpeed;
    USHORT SlaveAddress;
} RH_I2C_CONNECTION_PARAMETERS,
  *PRH_I2C_CONNECTION_PARAMETERS;

//
// _RH_UART_CONNECTION_PARAMETERS only includes mutable parameters.
// Fields not present in this structure must not be changed.
//

typedef struct _RH_UART_CONNECTION_PARAMETERS {
    LARGE_INTEGER ConnectionId;

    //
    // The UpdateMask field is used to determine which parameters
    // should be updated. Currently, all parameters are updated
    // by default.
    //

    USHORT UpdateMask;

    //
    // Configurable parameters.
    //

    ULONG BaudRate;
    USHORT TypeSpecificFlags;
    USHORT RxBufferSize;
    USHORT TxBufferSize;
    UCHAR Parity;
} RH_UART_CONNECTION_PARAMETERS,
  *PRH_UART_CONNECTION_PARAMETERS;

typedef struct _RH_ALLOCATE_UPDATE_CONNECTION_INPUT_BUFFER {
    ULONG Version;
    RH_CONNECTION_PROPERTIES_INPUT_TYPE ConnectionType;
    union {
        RH_GPIO_IO_CONNECTION_PARAMETERS IoConnection;
        RH_GPIO_INTERRUPT_CONNECTION_PARAMETERS InterruptConnection;
        RH_SPI_CONNECTION_PARAMETERS SpiConnection;
        RH_I2C_CONNECTION_PARAMETERS I2cConnection;
        RH_UART_CONNECTION_PARAMETERS UartConnection;
    } u;
} RH_ALLOCATE_UPDATE_CONNECTION_INPUT_BUFFER,
  *PRH_ALLOCATE_UPDATE_CONNECTION_INPUT_BUFFER;

typedef struct _RH_ALLOCATE_CONNECTION_OUTPUT_BUFFER {
    ULONG Version;
    LARGE_INTEGER ConnectionId;
} RH_ALLOCATE_CONNECTION_OUTPUT_BUFFER,
  *PRH_ALLOCATE_CONNECTION_OUTPUT_BUFFER;

typedef struct _RH_UPDATE_CONNECTION_PROPERTIES_OUTPUT_BUFFER {
    ULONG Version;
} RH_UPDATE_CONNECTION_PROPERTIES_OUTPUT_BUFFER,
  *PRH_UPDATE_CONNECTION_PROPERTIES_OUTPUT_BUFFER;

/*------------------------------------------------------------------------------
*
* IOCTL_RH_FREE_CONNECTION parameters
*
*----------------------------------------------------------------------------*/

typedef struct _RH_FREE_CONNECTION_INPUT_BUFFER {
    ULONG Version;
    LARGE_INTEGER ConnectionId;
} RH_FREE_CONNECTION_INPUT_BUFFER,
  *PRH_FREE_CONNECTION_INPUT_BUFFER;

//
// Currently, the output buffer is empty and is left for future expansion.
//

typedef struct _RH_FREE_CONNECTION_OUTPUT_BUFFER {
    ULONG Version;
} RH_FREE_CONNECTION_OUTPUT_BUFFER,
  *PRH_FREE_CONNECTION_OUTPUT_BUFFER;

/*------------------------------------------------------------------------------
 *
 * IOCTL_RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY parameters
 *
 *----------------------------------------------------------------------------*/

typedef struct _RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_BUFFER {
    ULONG Version;
    ULONG InterruptVector;
} RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_BUFFER,
  *PRH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_BUFFER;

typedef struct _RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_BUFFER {
    ULONG Version;
    UCHAR InitialPolarity;
} RH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_BUFFER,
  *PRH_QUERY_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_BUFFER;

/*------------------------------------------------------------------------------
 *
 * IOCTL_RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY parameters
 *
 *----------------------------------------------------------------------------*/

typedef struct _RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_BUFFER {
    ULONG Version;
    ULONG InterruptVector;
    BOOLEAN InitialPolarity;
} RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_BUFFER,
  *PRH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_INPUT_BUFFER;

//
// Currently this is basically empty but is left for future expansion.
//

typedef struct _RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_BUFFER {
    ULONG Version;
} RH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_BUFFER,
  *PRH_UPDATE_ACTIVE_BOTH_INITIAL_POLARITY_OUTPUT_BUFFER;

/*------------------------------------------------------------------------------
 *
 * Connection property structures
 *
 *----------------------------------------------------------------------------*/

#include "pshpack1.h"

//
// Length of GPIO common descriptor.
//

#define PNP_GPIO_INTERRUPT_IO_DESCRIPTOR_LENGTH \
    sizeof(PNP_GPIO_INTERRUPT_IO_DESCRIPTOR)

//
// GPIO IRQ and IO descriptor [fixed block]
//

typedef struct _PNP_GPIO_INTERRUPT_IO_DESCRIPTOR {
    UCHAR   Tag;                        // 10001100B, Large item name = 0xC
    USHORT  Length;                     // Length of the descriptor = 12 (min)
    UCHAR   Revision;                   // Descriptor format revision
    UCHAR   DescriptorType;             // Interrupt or IO descriptor
    USHORT  GeneralFlags;               // Generic Flags
    USHORT  InterruptIoFlags;           // Flags depending on descriptor type
    UCHAR   PinConfiguration;           // Pull configuration for the pin
    USHORT  DriveStrength;              // Drive strength
    USHORT  DebounceTimeout;            // Debounce timeout
    USHORT  PinTableOffset;             // Offset to start of pin table.
    UCHAR   ResourceSourceIndex;        // Index of the resource producer
    USHORT  ResourceSourceOffset;       // Offset to resource name string
    USHORT  VendorDataOffset;           // Offset to start of vendor data
    USHORT  VendorDataLength;           // Length of vendor data field

    //
    // ...followed by Pin table (USHORT)
    // ...followed RESOURCE_NAME string
    // ...followed by optional vendor data.
    //
    // Note the offsets in the fixed block should be used to access this data.
    //

} PNP_GPIO_INTERRUPT_IO_DESCRIPTOR, *PPNP_GPIO_INTERRUPT_IO_DESCRIPTOR;

//
// Length of function config descriptor.
//

#define PNP_FUNCTION_CONFIG_DESCRIPTOR_LENGTH \
    sizeof(PNP_FUNCTION_CONFIG_DESCRIPTOR)

//
// Large vendor descriptor.
//

typedef struct _PNP_LARGE_VENDOR_DESCRIPTOR {
    UCHAR   Tag;                              // 10001100B, Large item name = 0x4
    USHORT  Length;                           // Length of the descriptor
    UCHAR   SubType;                          // UUID specific subtype
    GUID    UUID;                             // UUID
    UCHAR   VendorDescriptor[ANYSIZE_ARRAY];  // Vendor descriptor data
} PNP_LARGE_VENDOR_DESCRIPTOR, *PPNP_LARGE_VENDOR_DESCRIPTOR;

//
// Length of function config descriptor [fixed block]
//

#define PNP_LARGE_VENDOR_DESCRIPTOR_LENGTH \
    sizeof(PNP_LARGE_VENDOR_DESCRIPTOR)

//
// Function config descriptor [fixed block]
//

typedef struct _PNP_FUNCTION_CONFIG_DESCRIPTOR {
    UCHAR   Tag;                        // 10001100B, Large item name = 0xD
    USHORT  Length;                     // Length of the descriptor = 12 (min)
    UCHAR   Revision;                   // Descriptor format revision
    USHORT  Flags;                      // Flags
    UCHAR   PinConfiguration;           // Pull configuration for the pin
    USHORT  FunctionNumber;             // Function config number for the pin
    USHORT  PinTableOffset;             // Offset to start of pin table.
    UCHAR   ResourceSourceIndex;        // Index of the resource producer
    USHORT  ResourceSourceOffset;       // Offset to resource name string
    USHORT  VendorDataOffset;           // Offset to start of vendor data
    USHORT  VendorDataLength;           // Length of vendor data field

    //
    // ...followed by Pin table (USHORT)
    // ...followed RESOURCE_NAME string
    // ...followed by optional vendor data.
    //
    // Note the offsets in the fixed block should be used to access this data.
    //

} PNP_FUNCTION_CONFIG_DESCRIPTOR, *PPNP_FUNCTION_CONFIG_DESCRIPTOR;

//
// Length of SPB common descriptor.
//

#define PNP_SERIAL_BUS_DESCRIPTOR_LENGTH sizeof(PNP_SERIAL_BUS_DESCRIPTOR)

//
// Serial Peripheral Bus common descriptor
//

typedef struct _PNP_SERIAL_BUS_DESCRIPTOR {
    UCHAR Tag;
    USHORT Length;
    UCHAR RevisionId;
    UCHAR ResourceSourceIndex;
    UCHAR SerialBusType;
    UCHAR GeneralFlags;
    USHORT TypeSpecificFlags;
    UCHAR TypeSpecificRevisionId;
    USHORT TypeDataLength;
} PNP_SERIAL_BUS_DESCRIPTOR, *PPNP_SERIAL_BUS_DESCRIPTOR;

#include "poppack.h"

/*------------------------------------------------------------------------------
 *
 * Resource Hub Helper Macros
 *
 *----------------------------------------------------------------------------*/

#ifdef RESHUB_USE_HELPER_ROUTINES

/*------------------------------------------------------------------------------
 *
 * For Connections
 *
 *----------------------------------------------------------------------------*/

#ifdef UMDF_DRIVER

_When_(SUCCEEDED(hr), _Post_satisfies_(NT_SUCCESS(return)))
_When_(FAILED(hr), _Post_satisfies_(!NT_SUCCESS(return)))
NTSTATUS
FORCEINLINE
RESOURCE_HUB_HRESULT_TO_NTSTATUS (
    _In_ HRESULT hr
    )
{
    switch(hr)
    {

    case STRSAFE_E_INVALID_PARAMETER:
        return STATUS_INVALID_PARAMETER;

    case STRSAFE_E_INSUFFICIENT_BUFFER:
        return STATUS_BUFFER_OVERFLOW;

    case STRSAFE_E_END_OF_FILE:
        return STATUS_END_OF_FILE;

    default:
        if(SUCCEEDED(hr)) {
            return STATUS_SUCCESS;

        } else {
            return STATUS_UNSUCCESSFUL;
        }
    }
}

static
NTSTATUS
RESOURCE_HUB_STRING_PRINTF (
    _Inout_ STRSAFE_LPWSTR DestinationString,
    _In_ size_t DestinationSizeInBytes,
    _In_ _Printf_format_string_ STRSAFE_LPCWSTR pszFormat,
    ...
    )
{

    HRESULT hr;
    va_list argList;

    va_start(argList, pszFormat);
    hr = StringCbVPrintfExW(DestinationString,
                            DestinationSizeInBytes,
                            NULL,
                            NULL,
                            0,
                            pszFormat,
                            argList);

    return RESOURCE_HUB_HRESULT_TO_NTSTATUS(hr);
}

static
NTSTATUS
RESOURCE_HUB_UNICODE_STRING_PRINTF (
    _Inout_ PUNICODE_STRING DestinationString,
    _In_ _Printf_format_string_ STRSAFE_LPCWSTR pszFormat,
    ...
    )
{

    HRESULT hr;
    STRSAFE_LPWSTR pszDestEnd;
    va_list argList;

    va_start(argList, pszFormat);
    hr = StringCbVPrintfExW(DestinationString->Buffer,
                            DestinationString->MaximumLength,
                            &pszDestEnd,
                            NULL,
                            0,
                            pszFormat,
                            argList);

    if (SUCCEEDED(hr)) {
        DestinationString->Length = (USHORT)(
            (pszDestEnd - DestinationString->Buffer) * sizeof(WCHAR));
    }

    return RESOURCE_HUB_HRESULT_TO_NTSTATUS(hr);
}

#if (UMDF_DRIVER)

NTSTATUS
FORCEINLINE
RESOURCE_HUB_UNICODE_STRING_INIT (
    _Out_ PUNICODE_STRING DestinationString,
    _In_opt_ STRSAFE_LPCWSTR pszSrc
    )
{

    HRESULT hr;
    size_t cbLength;

    if (pszSrc) {
        hr = StringCbLengthW(pszSrc, STRSAFE_MAX_CCH * sizeof(TCHAR), &cbLength);
        if (SUCCEEDED(hr)) {
            DestinationString->Length = (USHORT)cbLength;
            DestinationString->MaximumLength = (USHORT)(cbLength + sizeof(wchar_t));
            DestinationString->Buffer = (LPWSTR)pszSrc;
        }
    }
    else {
        hr = STATUS_SUCCESS;
        DestinationString->Length = 0;
        DestinationString->MaximumLength = 0;
        DestinationString->Buffer = NULL;
    }

    return RESOURCE_HUB_HRESULT_TO_NTSTATUS(hr);
}

#else

#define RESOURCE_HUB_UNICODE_STRING_INIT RtlUnicodeStringInit

#endif

#ifdef DBG

#define RESOURCE_HUB_ASSERT(_exp) \
    ((!(_exp)) ? \
        (__annotation(L"Debug", L"AssertFail", L#_exp), \
         DebugBreak(), FALSE) : \
        TRUE)

#else

#define RESOURCE_HUB_ASSERT(_exp)

#endif // DBG

#else

static
NTSTATUS
RESOURCE_HUB_STRING_PRINTF (
    _Inout_ NTSTRSAFE_PWSTR DestinationString,
    _In_ size_t DestinationSizeInBytes,
    _In_ _Printf_format_string_ NTSTRSAFE_PWSTR pszFormat,
    ...
    )
{

    va_list argList;

    va_start(argList, pszFormat);
    return RtlStringCbVPrintfExW(DestinationString,
                                 DestinationSizeInBytes,
                                 NULL,
                                 NULL,
                                 0,
                                 pszFormat,
                                 argList);
}

static
NTSTATUS
RESOURCE_HUB_UNICODE_STRING_PRINTF (
    _Inout_ PUNICODE_STRING DestinationString,
    _In_ _Printf_format_string_ NTSTRSAFE_PWSTR pszFormat,
    ...
    )
{

    va_list argList;
    NTSTRSAFE_PWSTR pszDestEnd;
    NTSTATUS status;

    va_start(argList, pszFormat);
    status = RtlStringCbVPrintfExW(DestinationString->Buffer,
                                   DestinationString->MaximumLength,
                                   &pszDestEnd,
                                   NULL,
                                   0,
                                   pszFormat,
                                   argList);

    if (NT_SUCCESS(status)) {
        DestinationString->Length = (USHORT)(
            (pszDestEnd - DestinationString->Buffer) * sizeof(WCHAR));
    }

    return status;
}

#define RESOURCE_HUB_UNICODE_STRING_INIT RtlUnicodeStringInit
#define RESOURCE_HUB_ASSERT(_exp) NT_ASSERT(_exp)

#endif

NTSTATUS
FORCEINLINE
RESOURCE_HUB_ID_TO_FILE_NAME(
    _In_ ULONG IdLowPart,
    _In_ ULONG IdHighPart,
    _Out_writes_bytes_(RESOURCE_HUB_CONNECTION_FILE_SIZE)
        PWCHAR FileName
    )
{

    LARGE_INTEGER Id;

    Id.LowPart = IdLowPart;
    Id.HighPart = IdHighPart;
    return RESOURCE_HUB_STRING_PRINTF(FileName,
                                      RESOURCE_HUB_FILE_SIZE,
                                      L"%0*I64x",
                                      (ULONG)(sizeof(LARGE_INTEGER) * 2),
                                      Id.QuadPart);

}

NTSTATUS
FORCEINLINE
RESOURCE_HUB_CREATE_PATH_FROM_ID (
    _Inout_ PUNICODE_STRING FileName,
    _In_ ULONG IdLowPart,
    _In_ ULONG IdHighPart
    )
{

    WCHAR FileNameSuffix[RESOURCE_HUB_CONNECTION_FILE_CHARS];
    NTSTATUS Status;

    RESOURCE_HUB_ASSERT(FileName->MaximumLength >=
            RESOURCE_HUB_CONNECTION_PATH_SIZE);

    Status = RESOURCE_HUB_ID_TO_FILE_NAME(IdLowPart,
                                          IdHighPart,
                                          FileNameSuffix);

    if (NT_SUCCESS(Status)) {
        Status = RESOURCE_HUB_UNICODE_STRING_PRINTF(
            FileName,
            L"%s%s",
            RESOURCE_HUB_DEVICE_NAME_PREFIX,
            FileNameSuffix);
    }

    return Status;
}

NTSTATUS
FORCEINLINE
RESOURCE_HUB_ID_FROM_FILE_NAME_WITH_SUBPATH (
    _In_z_ LPCWSTR FileName,
    _Out_ PLARGE_INTEGER Id,
    _Out_ LPWSTR *NextPathElement
    )
{

    UNICODE_STRING HighPart;
    USHORT Index;
#if !defined(UMDF_DRIVER) || (UMDF_DRIVER >= 2)
    UNICODE_STRING LowPart;
#endif
    NTSTATUS Status;

    Id->QuadPart = 0;
    *NextPathElement = (LPWSTR)FileName;

    Status = RESOURCE_HUB_UNICODE_STRING_INIT(&HighPart, FileName);
    if (!NT_SUCCESS(Status)) {
        return Status;

    } else if ((RESOURCE_HUB_CONNECTION_FILE_SIZE - sizeof(UNICODE_NULL)) >
               HighPart.Length) {

        Status = STATUS_INVALID_PARAMETER;
        return Status;

    } else {
        HighPart.Length = (RESOURCE_HUB_CONNECTION_FILE_SIZE - sizeof(UNICODE_NULL));
    }

    for (Index = 0; Index < (HighPart.Length / sizeof(WCHAR)); ++Index) {
        if ((HighPart.Buffer[Index] >= L'0' && HighPart.Buffer[Index] <= L'9') ||
            (HighPart.Buffer[Index] >= L'a' && HighPart.Buffer[Index] <= L'f') ||
            (HighPart.Buffer[Index] >= L'A' && HighPart.Buffer[Index] <= L'F')) {

            continue;

        } else {


            Status = STATUS_INVALID_PARAMETER;
            break;
        }
    }

    if (!NT_SUCCESS(Status)) {
        return Status;
    }

    *NextPathElement = &HighPart.Buffer[Index];
    if ((*NextPathElement)[0] != L'\\' && (*NextPathElement)[0] != L'\0') {
        return STATUS_INVALID_PARAMETER;
    }

#if defined(UMDF_DRIVER) && (UMDF_DRIVER == 1)

    Id->QuadPart = _wcstoi64(HighPart.Buffer, NextPathElement, 16);

    RESOURCE_HUB_ASSERT(
            ((ULONG_PTR)*NextPathElement - (ULONG_PTR)HighPart.Buffer) ==
              HighPart.Length);

#else

    //
    // Split into two strings of eight characters each to translate into two
    // 32-bit integers.
    //

    HighPart.Length = 2 * sizeof(Id->HighPart) * sizeof(WCHAR);
    LowPart.Buffer = (PWSTR)((ULONG_PTR)HighPart.Buffer + HighPart.Length);
    LowPart.MaximumLength = HighPart.MaximumLength - HighPart.Length;
    LowPart.Length = 2 * sizeof(Id->LowPart) * sizeof(WCHAR);

    Status = RtlUnicodeStringToInteger(&HighPart, 16, (PULONG)&Id->HighPart);
    if (!NT_SUCCESS(Status)) {
        return Status;
    }

    Status = RtlUnicodeStringToInteger(&LowPart, 16, (PULONG)&Id->LowPart);
    if (!NT_SUCCESS(Status)) {
        return Status;
    }

#endif

    if ((*NextPathElement)[0] == L'\\') {
        ++(*NextPathElement);
    }

    return Status;
}

NTSTATUS
FORCEINLINE
RESOURCE_HUB_ID_FROM_FILE_NAME (
    _In_z_ LPCWSTR FileName,
    _Out_ PLARGE_INTEGER Id
    )
{

    LPWSTR NextPathElement;
    NTSTATUS Status;

    Status = RESOURCE_HUB_ID_FROM_FILE_NAME_WITH_SUBPATH(FileName,
                                                         Id,
                                                         &NextPathElement);

    return Status;
}


#endif // RESHUB_USE_HELPER_ROUTINES

#ifdef __cplusplus
}
#endif

#endif // NTDDI_VERSION


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif // __RES_HUB_H__


