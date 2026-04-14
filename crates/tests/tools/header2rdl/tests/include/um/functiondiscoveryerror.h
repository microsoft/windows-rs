//
//    Copyright (C) Microsoft.  All rights reserved.
//
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <winerror.h>

// Error codes
//
//

// MessageId: E_FDPAIRING_NOCONNECTION
//
// MessageText:
//
// The device has rejected the connection.
//
#define E_FDPAIRING_NOCONNECTION        _HRESULT_TYPEDEF_(0x8FD00001L)

// MessageId: E_FDPAIRING_NOCONNECTION
//
// MessageText:
//
// The device has indicated a hardware failure.
//
#define E_FDPAIRING_HWFAILURE           _HRESULT_TYPEDEF_(0x8FD00002L)

// MessageId: E_FDPAIRING_AUTHFAILURE
//
// MessageText:
//
// The device authentication has failed.  Either the device has rejected the authentication or you rejected the authentication.
//
#define E_FDPAIRING_AUTHFAILURE         _HRESULT_TYPEDEF_(0x8FD00003L)

// MessageId: E_FDPAIRING_CONNECTTIMEOUT
//
// MessageText:
//
// The time to finish the authentication has expired on the device.
//
#define E_FDPAIRING_CONNECTTIMEOUT      _HRESULT_TYPEDEF_(0x8FD00004L)

// MessageId: E_FDPAIRING_TOOMANYCONNECTIONS
//
// MessageText:
//
// The device has indicated that it cannot accept more incoming connections.
//
#define E_FDPAIRING_TOOMANYCONNECTIONS  _HRESULT_TYPEDEF_(0x8FD00005L)

// MessageId: E_FDPAIRING_AUTHNOTALLOWED
//
// MessageText:
//
// The device has indicated that the authentication is not allowed.
//
#define E_FDPAIRING_AUTHNOTALLOWED      _HRESULT_TYPEDEF_(0x8FD00006L)

// MessageId: E_FDPAIRING_AUTHNOTALLOWED
//
// MessageText:
//
// The Pnp-X Bus Enumerator service is disabled.
//
#define E_FDPAIRING_IPBUSDISABLED      _HRESULT_TYPEDEF_(0x8FD00007L)

// MessageId: E_FDPAIRING_NOPROFILES
//
// MessageText:
//
// Windows does not have any network profiles for this device to use.
//
#define E_FDPAIRING_NOPROFILES         _HRESULT_TYPEDEF_(0x8FD00008L)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

