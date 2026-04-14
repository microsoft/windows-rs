/*++

Copyright 2004(c) Microsoft Corporation. All rights reserved.

Module Name:

        USBDI.H

Abstract:

   Obsolete header.  Use usb.h.

Environment:

    Kernel & user mode

Revision History:

    09-29-95 : created
    02-01-04 : updated to use usb.h

--*/

#ifndef   __USBDI_H__
#define   __USBDI_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <usb.h>

#include <usbioctl.h>

//
// The following are deprecated definitions.  These should not be used
//

#define USBD_STATUS_CANCELLING      ((USBD_STATUS)0x00020000L)
#define USBD_STATUS_CANCELING       ((USBD_STATUS)0x00020000L)
#define USBD_STATUS_NO_MEMORY       ((USBD_STATUS)0x80000100L)
#define USBD_STATUS_ERROR           ((USBD_STATUS)0x80000000L)
#define USBD_STATUS_REQUEST_FAILED  ((USBD_STATUS)0x80000500L)
#define USBD_STATUS_HALTED          ((USBD_STATUS)0xC0000000L)


#define USBD_HALTED(Status)  ((ULONG)(Status) >> 30 == 3)
#define USBD_STATUS(Status) ((ULONG)(Status) & 0x0FFFFFFFL)

#define URB_FUNCTION_RESERVED0                      0x0016
#define URB_FUNCTION_RESERVED                       0x001D
#define URB_FUNCTION_LAST                           0x0029

#define USBD_PF_DOUBLE_BUFFER           0x00000002

#ifdef USBD_PF_VALID_MASK
#undef USBD_PF_VALID_MASK
#endif

#define USBD_PF_VALID_MASK    (USBD_PF_CHANGE_MAX_PACKET | \
                               USBD_PF_DOUBLE_BUFFER | \
                               USBD_PF_ENABLE_RT_THREAD_ACCESS | \
                               USBD_PF_MAP_ADD_TRANSFERS)

#define USBD_TRANSFER_DIRECTION_BIT             0
#define USBD_SHORT_TRANSFER_OK_BIT              1
#define USBD_START_ISO_TRANSFER_ASAP_BIT        2

#ifdef USBD_TRANSFER_DIRECTION
#undef USBD_TRANSFER_DIRECTION
#endif

#define USBD_TRANSFER_DIRECTION(x)      ((x) & USBD_TRANSFER_DIRECTION_IN)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /*  __USBDI_H__ */
