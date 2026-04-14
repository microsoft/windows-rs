//*@@@+++@@@@******************************************************************
//
// Microsoft Windows
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//*@@@---@@@@******************************************************************

/******************************Module*Header*******************************\
*
* Module Name: DLNAMetadataProviderProperties.h
*
* DLNA Metadata Plug-In Properties
*
\**************************************************************************/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if NTDDI_VERSION >= NTDDI_WIN8

#ifndef INITGUID
#define INITGUID
#endif
#include <guiddef.h>

#include <devpropdef.h>

// Property keys GUID base.
// {88AD39DB-0D0C-4A38-8435-4043826B5C91}

DEFINE_DEVPROPKEY( DEVPKEY_Device_PacketWakeSupported,        0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x0 );    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_SendPacketWakeSupported,    0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x1 );    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_UDN,                        0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x6 );    // DEVPROP_TYPE_STRING
DEFINE_DEVPROPKEY( DEVPKEY_Device_SupportsAudio,              0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x8 );    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_SupportsVideo,              0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x9 );    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_SupportsImages,             0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0xA );    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_SinkProtocolInfo,           0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0xE );    // DEVPROP_TYPE_STRING

// DLNA device class and device capabilities <dlna:X_DLNADOC>
DEFINE_DEVPROPKEY( DEVPKEY_Device_DLNADOC,                    0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0xF );    // DEVPROP_TYPE_STRING

// DLNA capabilities <dlna:X_DLNACAP>
DEFINE_DEVPROPKEY( DEVPKEY_Device_DLNACAP,                    0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x10 );   // DEVPROP_TYPE_STRING

DEFINE_DEVPROPKEY( DEVPKEY_Device_SupportsSearch,             0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x11 );   // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_SupportsMute,               0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x12 );   // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY( DEVPKEY_Device_MaxVolume,                  0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x13 );   // DEVPROP_TYPE_UINT32
DEFINE_DEVPROPKEY( DEVPKEY_Device_SupportsSetNextAVT,         0x88AD39DB, 0x0D0C, 0x4A38, 0x84, 0x35, 0x40, 0x43, 0x82, 0x6B, 0x5C, 0x91, 0x14 );   // DEVPROP_TYPE_BOOLEAN

#endif // NTDDI_VERSION >= NTDDI_WIN8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

