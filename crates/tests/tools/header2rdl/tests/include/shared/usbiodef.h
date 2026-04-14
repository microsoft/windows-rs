/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

        USBIODEF.H

Abstract:

        Common header file for all USB IOCTLs defined for
        the core stack.  We define them in this single header file
        so that we can maintain backward compatibilty with older
        versions of the stack.


        We divide the IOCTLS supported by the stack as follows:

        kernel IOCTLS:


        user IOCTLS:

            IOCTLS Handled by HCD (PORT) FDO
            IOCTLS Handled by HUB FDO
            IOCTLS Handled by USB (DEVICE) PDO

Environment:

    kernel & user mode

Revision History:


--*/

#ifndef   __USBIODEF_H__
#define   __USBIODEF_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


/*
    Function codes for kernel mode IOCTLs with DeviceType : DEVICE_TYPE_USB (a.k.a DEVICE_TYPE_UNKNOWN)

    The following codes are valid only if passed as in
    the icControlCode parameter for
    IRP_MJ_INTERNAL_DEVICE_CONTROL

*/
#define USB_SUBMIT_URB              0
#define USB_RESET_PORT              1
#define USB_GET_ROOTHUB_PDO         3
#define USB_GET_PORT_STATUS         4
#define USB_ENABLE_PORT             5
#define USB_GET_HUB_COUNT           6
#define USB_CYCLE_PORT              7
#define USB_GET_HUB_NAME            8
#define USB_IDLE_NOTIFICATION       9
#define USB_RECORD_FAILURE          10
#define USB_GET_BUS_INFO            264
#define USB_GET_CONTROLLER_NAME     265
#define USB_GET_BUSGUID_INFO        266
#define USB_GET_PARENT_HUB_INFO     267
#define USB_GET_DEVICE_HANDLE       268
#define USB_GET_DEVICE_HANDLE_EX    269
#define USB_GET_TT_DEVICE_HANDLE    270
#define USB_GET_TOPOLOGY_ADDRESS    271
#define USB_IDLE_NOTIFICATION_EX    272
#define USB_REQ_GLOBAL_SUSPEND      273
#define USB_REQ_GLOBAL_RESUME       274
#define USB_GET_HUB_CONFIG_INFO     275
#define USB_FAIL_GET_STATUS         280

/*
    Function codes for kernel mode IOCTLs with DeviceType : DEVICE_TYPE_USBEX

    The following codes are valid only if passed as in
    the icControlCode parameter for
    IRP_MJ_INTERNAL_DEVICE_CONTROL

    The range 0 - 2047 is reserved for use by Microsoft.
    The range 0 - 1023 is used for Public Ioctls defined by Microsoft.

*/

#define USB_REGISTER_COMPOSITE_DEVICE              0
#define USB_UNREGISTER_COMPOSITE_DEVICE            1
#define USB_REQUEST_REMOTE_WAKE_NOTIFICATION       2


/*
    Function codes for user mode IOCTLs

    The following codes are valid only if passed as in
    the icControlCode parameter for
    IRP_MJ_DEVICE_CONTROL
    hence, they are callable by user mode applications
*/
#define HCD_GET_STATS_1                     255
#define HCD_DIAGNOSTIC_MODE_ON              256
#define HCD_DIAGNOSTIC_MODE_OFF             257
#define HCD_GET_ROOT_HUB_NAME               258
#define HCD_GET_DRIVERKEY_NAME              265
#define HCD_GET_STATS_2                     266
#define HCD_DISABLE_PORT                    268
#define HCD_ENABLE_PORT                     269
#define HCD_USER_REQUEST                    270
#define HCD_TRACE_READ_REQUEST              275


#define USB_GET_NODE_INFORMATION                            258
#define USB_GET_NODE_CONNECTION_INFORMATION                 259
#define USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION             260
#define USB_GET_NODE_CONNECTION_NAME                        261
#define USB_DIAG_IGNORE_HUBS_ON                             262
#define USB_DIAG_IGNORE_HUBS_OFF                            263
#define USB_GET_NODE_CONNECTION_DRIVERKEY_NAME              264
#define USB_GET_HUB_CAPABILITIES                            271
#define USB_GET_NODE_CONNECTION_ATTRIBUTES                  272
#define USB_HUB_CYCLE_PORT                                  273
#define USB_GET_NODE_CONNECTION_INFORMATION_EX              274
#define USB_RESET_HUB                                       275
#define USB_GET_HUB_CAPABILITIES_EX                         276
#define USB_GET_HUB_INFORMATION_EX                          277
#define USB_GET_PORT_CONNECTOR_PROPERTIES                   278
#define USB_GET_NODE_CONNECTION_INFORMATION_EX_V2           279

#define USB_GET_TRANSPORT_CHARACTERISTICS                   281
#define USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE   282
#define USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE      283
#define USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE 284

#define USB_START_TRACKING_FOR_TIME_SYNC                    285
#define USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC          286
#define USB_STOP_TRACKING_FOR_TIME_SYNC                     287

#define USB_GET_DEVICE_CHARACTERISTICS                      288

#define USB_GET_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION  289

// IOCTL codes starting here and beyond are for windows' internal use
#define USB_RESERVED_USER_BASE                              1024

/*
USB specific GUIDs
*/


/* f18a0e88-c30c-11d0-8815-00a0c906bed8 */
DEFINE_GUID(GUID_DEVINTERFACE_USB_HUB,    0xf18a0e88, 0xc30c, 0x11d0, 0x88, 0x15, 0x00, \
             0xa0, 0xc9, 0x06, 0xbe, 0xd8);

/*5e9adaef-f879-473f-b807-4e5ea77d1b1c*/
DEFINE_GUID(GUID_DEVINTERFACE_USB_BILLBOARD, 0x5e9adaef, 0xf879, 0x473f, 0xb8, 0x07, 0x4e, \
            0x5e, 0xa7, 0x7d, 0x1b, 0x1c);

/* A5DCBF10-6530-11D2-901F-00C04FB951ED */
DEFINE_GUID(GUID_DEVINTERFACE_USB_DEVICE, 0xA5DCBF10L, 0x6530, 0x11D2, 0x90, 0x1F, 0x00, \
             0xC0, 0x4F, 0xB9, 0x51, 0xED);

/* 3ABF6F2D-71C4-462a-8A92-1E6861E6AF27 */
DEFINE_GUID(GUID_DEVINTERFACE_USB_HOST_CONTROLLER, 0x3abf6f2d, 0x71c4, 0x462a, 0x8a, 0x92, 0x1e, \
             0x68, 0x61, 0xe6, 0xaf, 0x27);

/* 4E623B20-CB14-11D1-B331-00A0C959BBD2 */
DEFINE_GUID(GUID_USB_WMI_STD_DATA, 0x4E623B20L, 0xCB14, 0x11D1, 0xB3, 0x31, 0x00,\
             0xA0, 0xC9, 0x59, 0xBB, 0xD2);

/* 4E623B20-CB14-11D1-B331-00A0C959BBD2 */
DEFINE_GUID(GUID_USB_WMI_STD_NOTIFICATION, 0x4E623B20L, 0xCB14, 0x11D1, 0xB3, 0x31, 0x00,\
             0xA0, 0xC9, 0x59, 0xBB, 0xD2);

#if (_WIN32_WINNT >= 0x0600)
/*
    For windows longhorn and later
*/

/* 66C1AA3C-499F-49a0-A9A5-61E2359F6407 */
DEFINE_GUID(GUID_USB_WMI_DEVICE_PERF_INFO, 0x66c1aa3c, 0x499f, 0x49a0, 0xa9, 0xa5, 0x61, 0xe2,\
             0x35, 0x9f, 0x64, 0x7);

// {9C179357-DC7A-4f41-B66B-323B9DDCB5B1}
DEFINE_GUID(GUID_USB_WMI_NODE_INFO,
0x9c179357, 0xdc7a, 0x4f41, 0xb6, 0x6b, 0x32, 0x3b, 0x9d, 0xdc, 0xb5, 0xb1);

/* 3a61881b-b4e6-4bf9-ae0f-3cd8f394e52f */
DEFINE_GUID(GUID_USB_WMI_TRACING,
0x3a61881b, 0xb4e6, 0x4bf9, 0xae, 0xf, 0x3c, 0xd8, 0xf3, 0x94, 0xe5, 0x2f);

// {681EB8AA-403D-452c-9F8A-F0616FAC9540}
DEFINE_GUID(GUID_USB_TRANSFER_TRACING,
0x681eb8aa, 0x403d, 0x452c, 0x9f, 0x8a, 0xf0, 0x61, 0x6f, 0xac, 0x95, 0x40);

// {D5DE77A6-6AE9-425c-B1E2-F5615FD348A9}
DEFINE_GUID(GUID_USB_PERFORMANCE_TRACING,
0xd5de77a6, 0x6ae9, 0x425c, 0xb1, 0xe2, 0xf5, 0x61, 0x5f, 0xd3, 0x48, 0xa9);

// {9BBBF831-A2F2-43B4-96D1-86944B5914B3}
DEFINE_GUID(GUID_USB_WMI_SURPRISE_REMOVAL_NOTIFICATION,
0x9bbbf831, 0xa2f2, 0x43b4, 0x96, 0xd1, 0x86, 0x94, 0x4b, 0x59, 0x14, 0xb3);

#endif

/*
Obsolete device interface class GUID names.
(use of above GUID_DEVINTERFACE_* names is recommended).
--*/

#define GUID_CLASS_USBHUB               GUID_DEVINTERFACE_USB_HUB
#define GUID_CLASS_USB_DEVICE           GUID_DEVINTERFACE_USB_DEVICE
#define GUID_CLASS_USB_HOST_CONTROLLER  GUID_DEVINTERFACE_USB_HOST_CONTROLLER

#define FILE_DEVICE_USB         FILE_DEVICE_UNKNOWN

/*
    common macro used by IOCTL header files
*/
#define USB_CTL(id)  CTL_CODE(FILE_DEVICE_USB,  \
                                      (id), \
                                      METHOD_BUFFERED,  \
                                      FILE_ANY_ACCESS)

#define USB_KERNEL_CTL(id)  CTL_CODE(FILE_DEVICE_USB,  \
                                      (id), \
                                      METHOD_NEITHER,  \
                                      FILE_ANY_ACCESS)

#define USB_KERNEL_CTL_BUFFERED(id)  CTL_CODE(FILE_DEVICE_USB,  \
                                      (id), \
                                      METHOD_BUFFERED,  \
                                      FILE_ANY_ACCESS)

/*
    structures common to both usbioctl.h and usbdrivr.h
*/


#if (_WIN32_WINNT >= 0x0501)

/*
    used by IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION.
    Available on windows XP and later
*/


_IRQL_requires_max_(PASSIVE_LEVEL)
typedef
VOID
(*USB_IDLE_CALLBACK)(
    _In_ PVOID Context
    );

typedef struct _USB_IDLE_CALLBACK_INFO {
    USB_IDLE_CALLBACK IdleCallback;
    PVOID IdleContext;
} USB_IDLE_CALLBACK_INFO, *PUSB_IDLE_CALLBACK_INFO;

#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif //__USBIODEF_H__
