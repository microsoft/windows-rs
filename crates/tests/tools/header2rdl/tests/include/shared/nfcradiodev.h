/*++

Copyright (C) Microsoft Corporation. All rights reserved.

Module Name:

    NfcRadioDev.h

Abstract:

    IOCTL codes and structure common to the NFC Radio Device Interface

--*/
#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

    //{4d51e930-750d-4a36-a9f791dc540fcd30}
    DEFINE_GUID(GUID_NFC_RADIO_MEDIA_DEVICE_INTERFACE, 0x4d51e930, 0x750d, 0x4a36, 0xa9, 0xf7, 0x91, 0xdc, 0x54, 0x0f, 0xcd, 0x30);

    // {EF8BA08F-148D-4116-83EF-A2679DFC3FA5}
    DEFINE_GUID(GUID_NFCSE_RADIO_MEDIA_DEVICE_INTERFACE, 0xef8ba08f, 0x148d, 0x4116, 0x83, 0xef, 0xa2, 0x67, 0x9d, 0xfc, 0x3f, 0xa5);

//
// IOCTL definitions.
//
#define NFCRMDDI_IOCTL_BASE 0x50

#define IOCTL_NFCRM_SET_RADIO_STATE \
    CTL_CODE(FILE_DEVICE_NFP, (NFCRMDDI_IOCTL_BASE + 0x11), METHOD_BUFFERED, FILE_ANY_ACCESS) // Input: NFCRM_SET_RADIO_STATE
#define IOCTL_NFCRM_QUERY_RADIO_STATE \
    CTL_CODE(FILE_DEVICE_NFP, (NFCRMDDI_IOCTL_BASE + 0x12), METHOD_BUFFERED, FILE_ANY_ACCESS) // Output: NFCRM_RADIO_STATE
#define IOCTL_NFCSERM_SET_RADIO_STATE \
    CTL_CODE(FILE_DEVICE_NFP, (NFCRMDDI_IOCTL_BASE + 0x13), METHOD_BUFFERED, FILE_ANY_ACCESS) // Input: NFCRM_SET_RADIO_STATE
#define IOCTL_NFCSERM_QUERY_RADIO_STATE \
    CTL_CODE(FILE_DEVICE_NFP, (NFCRMDDI_IOCTL_BASE + 0x14), METHOD_BUFFERED, FILE_ANY_ACCESS) // Output: NFCRM_RADIO_STATE

#include <PSHPACK1.H>

    typedef struct _NFCRM_SET_RADIO_STATE
    {
        BOOLEAN SystemStateUpdate; // TRUE if this is a system state update
        BOOLEAN MediaRadioOn;
    } NFCRM_SET_RADIO_STATE, *PNFCRM_SET_RADIO_STATE;

    typedef struct _NFCRM_RADIO_STATE
    {
        BOOLEAN MediaRadioOn;
    } NFCRM_RADIO_STATE, *PNFCRM_RADIO_STATE;

#include <POPPACK.H>

#endif // NTDDI_WINBLUE

#ifdef __cplusplus
}
#endif
