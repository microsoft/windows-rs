/*********************************************************************
*
* HINTAPI.H
*
*   Windows Terminal Server public hint API
*
*   Copyright (c) Microsoft Corporation. All rights reserved. 
*
**********************************************************************/

#ifndef _INC_HINTAPIPUB
#define _INC_HINTAPIPUB


#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef __cplusplus
extern "C" {
#endif


/* header files for imported files */
#include <propidl.h>


//
// Hint type definitions
//
#define RENDER_HINT_CLEAR            0x0
#define RENDER_HINT_VIDEO            0x1
#define RENDER_HINT_MAPPEDWINDOW     0x2


/*------------------------------------------------------------------*/
// Render Hints provided by applications for things like video
// 
// Parameters:
// pRenderHintID: Integer identifying a unique render hint for the process. 
//               This is primarily used for ongoing hints which need 
//               further updates and eventual clearing. E.g. a hint 
//               indicating start of video playback, updates to video
//               position and eventually clearing the hint when the video
//               stops. If a hint is to be updated, the API MUST be 
//               called with the same renderHintID as the original call
//               In first call for a hint, the value 0 MUST be passed in. 
//               API will generate a unique ID which MUST be used for 
//               updating the hint
//
// hwndOwner: The handle of window linked to lifetime of the rendering 
//            hint. This window is used in situations where a hint-target
//            is removed (e.g. application crash) without a formal 
//            clearing of the hint. 
//
// 
// renderHintType: Type of the render hint. The type dictates the 
//                 specific metadata for the hint indicated via 
//                 rednerHintData parameter. Currently following types are defined:
//
//                #define RENDER_HINT_CLEAR 0x00000000
//	              Hint type to indicate that the previous hint is cleared
//                #define RENDER_HINT_VIDEO 0x00000001
//                    Hint type to indicate presence of a video
//                #define RENDER_HINT_MAPPEDWINDOW 0x00000002
//                    Hint type to indicate presence of a window mapping
//
// cbHintDataLength: Length of the additional data corresponding to the hint. 
//
// pHintData:      Additional data corresponding to the hint. 
//
//                 For RENDER_HINT_CLEAR, this parameter MUST be set to NULL 
//                 and length MUST be set to 0
//
//                 For RENDER_HINT_VIDEO and RENDER_HINT_MAPPEDWINDOW, 
//                 the hint data MUST contain the RECT
//                 structure with the video co-ordinates and size. 
//                 These are in the client co-ordinates  w.r.t. hwndOwner paremeter.
//                 The hintDataSize MUST be sizeof(RECT)
/*------------------------------------------------------------------*/

HRESULT WINAPI WTSSetRenderHint(
  _Inout_  UINT64*             pRenderHintID,
  _In_     HWND  	       hwndOwner,
  _In_     DWORD 	       renderHintType,
  _In_     DWORD               cbHintDataLength,
  _In_reads_bytes_opt_(cbHintDataLength) BYTE*     pHintData
);

#ifdef __cplusplus
}
#endif

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* !_INC_HINTAPIPUB */

