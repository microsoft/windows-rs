#ifndef _D3DHALEX_H
#define _D3DHALEX_H

//-----------------------------------------------------------------------------
//
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  File:       d3dhalex.h
//  Content:    Direct3D HAL Extensions and Helpers include file
//              This file contains definitions and macros which although not
//              essential for building a driver are useful helpers and
//              utilities.
//
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
//
// Macros to help with handling the new GetDriverInfo2 mechanism. The following
// macros assist with handling the GetDriverInfo2 sub-call of GetDriverInfo.
// Two of the macros are simplified ways of differentiating between
// GetDriverInfo2 calls and DDStereoMode calls. The others are simplified ways
// of getting the data structures associated with those two calls.
//
// The following code fragment demonstrates how to handle GetDriverInfo2 using
// these macros. Compare this with the code fragment in d3dhal.h
//
// D3DCAPS8 myD3DCaps8;
//
// DWORD CALLBACK 
// DdGetDriverInfo(LPDDHAL_GETDRIVERINFODATA lpData)
// {
//     if (MATCH_GUID((lpData->guidInfo), GUID_GetDriverInfo2) )
//     {
//         ASSERT(NULL != lpData);
//         ASSERT(NULL != lpData->lpvData);
//
//         // Is this a call to GetDriverInfo2 or DDStereoMode?
//         if (D3DGDI_IS_GDI2(lpData))
//         {
//             // Yes, its a call to GetDriverInfo2, fetch the
//             // DD_GETDRIVERINFO2DATA data structure.
//             DD_GETDRIVERINFO2DATA* pgdi2 = D3DGDI_GET_GDI2_DATA(lpData);
//             ASSERT(NULL != pgdi2);
//
//             // What type of request is this?
//             switch (pgdi2->dwType)
//             {
//             case D3DGDI2_TYPE_GETD3DCAPS8:
//                 {
//                     // The runtime is requesting the DX8 D3D caps so
//                     // copy them over now.
//
//                     // It should be noted that the dwExpectedSize field
//                     // of DD_GETDRIVERINFODATA is not used for
//                     // GetDriverInfo2 calls and should be ignored.
//                     size_t copySize = min(sizeof(myD3DCaps8), pgdi2->dwExpectedSize);
//                     memcpy(lpData->lpvData, &myD3DCaps8, copySize);
//                     lpData->dwActualSize = copySize;
//                     lpData->ddRVal       = DD_OK;
//                     return DDHAL_DRIVER_HANDLED;
//                 }
//             default:
//                 // For any other GetDriverInfo2 types not handled
//                 // or understood by the driver set an ddRVal of
//                 // DDERR_CURRENTLYNOTAVAIL and return
//                 // DDHAL_DRIVER_HANDLED.
//                 return DDHAL_DRIVER_HANDLED;
//             }
//         }
//         else
//         {
//             // It must be a call a request for stereo mode support.
//             // Fetch the stereo mode data
//             DD_STEREOMODE* pStereoMode = D3DGDI_GET_STEREOMODE_DATA(pData);
//             ASSERT(NULL != pStereoMode);
//
//             // Process the stereo mode request...
//             lpData->dwActualSize = sizeof(DD_STEREOMODE);
//             lpData->ddRVal       = DD_OK;
//             return DDHAL_DRIVER_HANDLED;
//         }
//     }
//
//     // Handle any other device GUIDs...
//
// } // DdGetDriverInfo
//
//-----------------------------------------------------------------------------

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Macros to determine what type of call a call to GetDriverInfo with the
// GUID GUID_GetDriverInfo2/GUID_DDStereoMode. A GetDriverInfo2 call or
// a DDStereoMode call.
//
#define D3DGDI_IS_GDI2(pData) \
    ((((DD_GETDRIVERINFO2DATA*)(pData->lpvData))->dwMagic)  == D3DGDI2_MAGIC)

#define D3DGDI_IS_STEREOMODE(pData) \
    ((((DD_STEREOMODE*)        (pData->lpvData))->dwHeight) != D3DGDI2_MAGIC)

//
// Macros to return the appropriate GetDriverInfo data structure for a
// call to GetDriverInfo with the GUID GUID_GetDriverInfo2/GUID_DDStereoMode.
//
#define D3DGDI_GET_GDI2_DATA(pData) \
    (D3DGDI_IS_GDI2(pData) ? (((DD_GETDRIVERINFO2DATA*)(pData->lpvData))) : NULL)

#define D3DGDI_GET_STEREOMODE_DATA(pData) \
    (D3DGDI_IS_STEREOMODE(pData) ? (((DD_STEREOMODE*)(pData->lpvData)))   : NULL)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _D3DHALEX_H */

