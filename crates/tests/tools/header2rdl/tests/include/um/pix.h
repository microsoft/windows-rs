/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:       pix.h
 *  Content:    PIX include file
 *
 ****************************************************************************/

#pragma once

#ifndef _PIX_H_
#define _PIX_H_

// 
// PIX MARKER EVENT API
// 
// The PIX marker event APIs are intended to allow a developer to annotate sections of their code for use with
// debugging or performance measurement tools. These APIs are available across Windows devices. The three APIs are:
// 
//   void PIXBeginEvent(<interface>* pContext, UINT64 Metadata, LPCWSTR pFormat, ...)
//   void PIXEndEvent(<interface>* pContext)
//   void PIXSetMarker(<interface>* pContext, UINT64 Metadata, LPCWSTR pFormat, ...)
// 
// Examples of the allowed types for <interface> are: ID3D11DeviceContext2, ID3DUserDefinedAnnotation, 
// ID3D12GraphicsCommandList, and ID3D12CommandQueue. Developers can pass in a color value into the Metadata parameter
// using the PIX_COLOR or PIX_COLOR_INDEX macros.
// 
// The PIX marker event APIs are also available as ANSI versions in addition to the Unicode overloads specified above.
// Note that there may be additional device-specific APIs available. Review the SDK documentation for more information.
// 


// Use these functions to specify colors to pass as metadata to a PIX event/marker API.
// Use PIX_COLOR() to specify a particular color for an event.
// Or, use PIX_COLOR_INDEX() to specify a set of unique event categories, and let PIX choose
// the colors to represent each category.
inline UINT PIX_COLOR(BYTE r, BYTE g, BYTE b) { return 0xff000000 | (r << 16) | (g << 8) | b; }
inline UINT PIX_COLOR_INDEX(BYTE i) { return 0x00000000 | i; }
const UINT PIX_COLOR_DEFAULT = PIX_COLOR_INDEX(0);


// These headers are implementation details only - developers should not include pix_xbox.h or pix_win.h directly.
#if defined(_XBOX_ONE) || WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_TV_TITLE)
#include "pix_xbox.h"
#else
#include "pix_win.h"
#endif // _XBOX_ONE

#endif // _PIX_H_