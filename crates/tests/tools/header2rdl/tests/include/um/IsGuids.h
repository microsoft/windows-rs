/*
 * isguids.h - Internet Shortcut GUID definitions.
 *
 * Copyright (c) Microsoft Corporation. All rights reserved.
 */


/* GUIDs
 ********/

#pragma once

#ifndef _ISGUIDS_H_
#define _ISGUIDS_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DEFINE_GUID(CLSID_InternetShortcut,       0xFBF23B40L, 0xE3F0, 0x101B, 0x84, 0x88, 0x00, 0xAA, 0x00, 0x3E, 0x56, 0xF8);

DEFINE_GUID(IID_IUniformResourceLocatorA, 0xFBF23B80L, 0xE3F0, 0x101B, 0x84, 0x88, 0x00, 0xAA, 0x00, 0x3E, 0x56, 0xF8);
DEFINE_GUID(IID_IUniformResourceLocatorW, 0xCABB0DA0L, 0xDA57, 0x11CF, 0x99, 0x74, 0x00, 0x20, 0xAF, 0xD7, 0x97, 0x62);

#ifdef UNICODE
#define IID_IUniformResourceLocator     IID_IUniformResourceLocatorW
#else
#define IID_IUniformResourceLocator     IID_IUniformResourceLocatorA
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
