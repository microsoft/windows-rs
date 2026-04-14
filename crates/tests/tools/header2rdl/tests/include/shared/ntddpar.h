/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddpar.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the Parallel device.


--*/

//
// Interface GUID
//
//
// need these GUIDs outside conditional includes so that user can
//   #include <ntddpar.h>  in precompiled header
//   #include <initguid.h> in a single source file
//   #include <ntddpar.h>  in that source file a second time to instantiate the GUIDs
//
// #ifdef WANT_WDM
#ifndef FAR
#define FAR
#endif

#ifdef DEFINE_GUID

#if (NTDDI_VERSION >= NTDDI_WINXP) // Windows XP Version

DEFINE_GUID(GUID_DEVINTERFACE_PARALLEL, 0x97F76EF0, 0xF883, 0x11D0, 0xAF, 0x1F, 0x00, 0x00, 0xF8, 0x00, 0x84, 0x5C);
DEFINE_GUID(GUID_DEVINTERFACE_PARCLASS, 0x811FC6A5, 0xF728, 0x11D0, 0xA5, 0x37, 0x00, 0x00, 0xF8, 0x75, 0x3E, 0xD1);

//
// Obsolete device interface class GUID names.
// (use of above GUID_DEVINTERFACE_* names is recommended).
//

#define GUID_PARALLEL_DEVICE  GUID_DEVINTERFACE_PARALLEL
#define GUID_PARCLASS_DEVICE  GUID_DEVINTERFACE_PARCLASS

#else // if ( NTDDI_VERSION ... )

DEFINE_GUID(GUID_PARALLEL_DEVICE, 0x97F76EF0, 0xF883, 0x11D0, 0xAF, 0x1F, 0x00, 0x00, 0xF8, 0x00, 0x84, 0x5C);
DEFINE_GUID(GUID_PARCLASS_DEVICE, 0x811FC6A5, 0xF728, 0x11D0, 0xA5, 0x37, 0x00, 0x00, 0xF8, 0x75, 0x3E, 0xD1);

#endif // if ( NTDDI_VERSION ... )

#endif
// #endif

#ifndef _NTDDPAR_
#define _NTDDPAR_
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_PAR_BASE                  FILE_DEVICE_PARALLEL_PORT
#define IOCTL_PAR_QUERY_INFORMATION     CTL_CODE(FILE_DEVICE_PARALLEL_PORT,1,METHOD_BUFFERED,FILE_ANY_ACCESS)
#define IOCTL_PAR_SET_INFORMATION       CTL_CODE(FILE_DEVICE_PARALLEL_PORT,2,METHOD_BUFFERED,FILE_ANY_ACCESS)

//
// Returns NULL terminated device ID string
//
#define IOCTL_PAR_QUERY_DEVICE_ID       CTL_CODE(FILE_DEVICE_PARALLEL_PORT,3,METHOD_BUFFERED,FILE_ANY_ACCESS)

//
// Returns buffer size required for a call to IOCTL_PAR_QUERY_DEVICE_ID 
//   to succeed. This includes device ID size plus space for terminating NULL.
//
#define IOCTL_PAR_QUERY_DEVICE_ID_SIZE  CTL_CODE(FILE_DEVICE_PARALLEL_PORT,4,METHOD_BUFFERED,FILE_ANY_ACCESS)

#define IOCTL_IEEE1284_GET_MODE         CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 5, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_IEEE1284_NEGOTIATE        CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 6, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_SET_WRITE_ADDRESS     CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 7, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_SET_READ_ADDRESS      CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 8, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_GET_DEVICE_CAPS       CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 9, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_GET_DEFAULT_MODES     CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 10, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_PING                  CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 11, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// Similar to IOCTL_PAR_QUERY_DEVICE_ID above, but includes (i.e., does 
//   not discard) the two byte size prefix returned by the device.
//
#define IOCTL_PAR_QUERY_RAW_DEVICE_ID   CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 12, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_ECP_HOST_RECOVERY     CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 13, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_GET_READ_ADDRESS      CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 14, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_GET_WRITE_ADDRESS     CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 15, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_PAR_TEST                  CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 20, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_PAR_IS_PORT_FREE          CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 21, METHOD_BUFFERED, FILE_ANY_ACCESS)

#if (NTDDI_VERSION >= NTDDI_WINXP) // Windows XP Version

// returns Location of the port - generally of the form: LPTx or LPTx.y or LPTx.y-z
#define IOCTL_PAR_QUERY_LOCATION        CTL_CODE(FILE_DEVICE_PARALLEL_PORT, 22, METHOD_BUFFERED, FILE_ANY_ACCESS)

#endif // if (NTDDI_VERSION ... )

//
// NtDeviceIoControlFile InputBuffer/OutputBuffer record structures for
// this device.
//

typedef struct _PAR_QUERY_INFORMATION{
       UCHAR Status;
} PAR_QUERY_INFORMATION, *PPAR_QUERY_INFORMATION;

typedef struct _PAR_SET_INFORMATION{
       UCHAR Init;
} PAR_SET_INFORMATION, *PPAR_SET_INFORMATION;

#define PARALLEL_INIT            0x1
#define PARALLEL_AUTOFEED        0x2
#define PARALLEL_PAPER_EMPTY     0x4
#define PARALLEL_OFF_LINE        0x8
#define PARALLEL_POWER_OFF       0x10
#define PARALLEL_NOT_CONNECTED   0x20
#define PARALLEL_BUSY            0x40
#define PARALLEL_SELECTED        0x80

//
// This is the structure returned by IOCTL_PAR_QUERY_DEVICE_ID_SIZE.
//

typedef struct _PAR_DEVICE_ID_SIZE_INFORMATION {
    ULONG   DeviceIdSize;
} PAR_DEVICE_ID_SIZE_INFORMATION, *PPAR_DEVICE_ID_SIZE_INFORMATION;


//
// These constants are used for usReadMask and usWriteMask components of the 
// PARCLASS_NEGOTIATION_MASK structure that is used for:
//
// IOCTL_IEEE1284_NEGOTIATE, 
// IOCTL_IEEE1284_GET_MODE, and 
// IOCTL_PAR_GET_DEFAULT_MODES.
//

typedef struct _PARCLASS_NEGOTIATION_MASK {
	USHORT      usReadMask;
	USHORT      usWriteMask;
} PARCLASS_NEGOTIATION_MASK, *PPARCLASS_NEGOTIATION_MASK;

#define NONE                0x0000
#define CENTRONICS          0x0001       /* Write Only */
#define IEEE_COMPATIBILITY  0x0002       /* Write Only */
#define NIBBLE              0x0004       /* Read Only */
#define CHANNEL_NIBBLE      0x0008       /* Read Only */
#define BYTE_BIDIR          0x0010       /* Read Only */
#define EPP_HW              0x0020
#define EPP_SW              0x0040
#define EPP_ANY             0x0060
#define BOUNDED_ECP         0x0080
#define ECP_HW_NOIRQ        0x0100      /* HWECP PIO */
#define ECP_HW_IRQ          0x0200      /* HWECP with IRQ */
#define ECP_SW              0x0400
#define ECP_ANY             0x0780

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif  // _NTDDPAR_
