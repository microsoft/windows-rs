/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1990-1999  Microsoft Corporation

Module Name:

    ntddtdi.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the Transport driver interface device.

--*/

#ifndef _NTDDTDI_
#define _NTDDTDI_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
// Device Name - this string is the name of the device.  It is the name
// that should be passed to NtOpenFile when accessing the device.
//
// Note:  For devices that support multiple units, it should be suffixed
//        with the Ascii representation of the unit number.
//

#define DD_TDI_DEVICE_NAME "\\Device\\UNKNOWN"


//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define _TDI_CONTROL_CODE(request,method) \
            CTL_CODE(FILE_DEVICE_TRANSPORT, request, method, FILE_ANY_ACCESS)

#define IOCTL_TDI_ACCEPT                _TDI_CONTROL_CODE( 0, METHOD_BUFFERED )
#define IOCTL_TDI_CONNECT               _TDI_CONTROL_CODE( 1, METHOD_BUFFERED )
#define IOCTL_TDI_DISCONNECT            _TDI_CONTROL_CODE( 2, METHOD_BUFFERED )
#define IOCTL_TDI_LISTEN                _TDI_CONTROL_CODE( 3, METHOD_BUFFERED )
#define IOCTL_TDI_QUERY_INFORMATION     _TDI_CONTROL_CODE( 4, METHOD_OUT_DIRECT )
#define IOCTL_TDI_RECEIVE               _TDI_CONTROL_CODE( 5, METHOD_OUT_DIRECT )
#define IOCTL_TDI_RECEIVE_DATAGRAM      _TDI_CONTROL_CODE( 6, METHOD_OUT_DIRECT )
#define IOCTL_TDI_SEND                  _TDI_CONTROL_CODE( 7, METHOD_IN_DIRECT )
#define IOCTL_TDI_SEND_DATAGRAM         _TDI_CONTROL_CODE( 8, METHOD_IN_DIRECT )
#define IOCTL_TDI_SET_EVENT_HANDLER     _TDI_CONTROL_CODE( 9, METHOD_BUFFERED )
#define IOCTL_TDI_SET_INFORMATION       _TDI_CONTROL_CODE( 10, METHOD_IN_DIRECT )
#define IOCTL_TDI_ASSOCIATE_ADDRESS     _TDI_CONTROL_CODE( 11, METHOD_BUFFERED )
#define IOCTL_TDI_DISASSOCIATE_ADDRESS  _TDI_CONTROL_CODE( 12, METHOD_BUFFERED )
#define IOCTL_TDI_ACTION                _TDI_CONTROL_CODE( 13, METHOD_OUT_DIRECT )

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // ndef _NTDDTDI_
