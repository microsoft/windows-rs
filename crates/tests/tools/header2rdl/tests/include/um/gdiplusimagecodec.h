/**************************************************************************\
*
* Copyright (c) 2000-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusImageCodec.h
*
* Abstract:
*
*   GDI+ Codec Image APIs
*
\**************************************************************************/

#ifndef _GDIPLUSIMAGECODEC_H
#define _GDIPLUSIMAGECODEC_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//--------------------------------------------------------------------------
// Codec Management APIs
//--------------------------------------------------------------------------

inline Status 
GetImageDecodersSize(
    _Out_ UINT *numDecoders,
    _Out_ _Out_range_(>=, (*numDecoders) * sizeof(ImageCodecInfo)) UINT *size)
{
    return DllExports::GdipGetImageDecodersSize(numDecoders, size);
}


inline Status 
GetImageDecoders(
    _In_ UINT numDecoders,
    _In_ UINT size,
    _Out_writes_bytes_(size) ImageCodecInfo *decoders)
{
    return DllExports::GdipGetImageDecoders(numDecoders, size, decoders);
}


inline Status 
GetImageEncodersSize(
    _Out_ UINT *numEncoders, 
    _Out_ _Out_range_(>=, (*numEncoders) * sizeof(ImageCodecInfo)) UINT *size)
{
    return DllExports::GdipGetImageEncodersSize(numEncoders, size);
}


inline Status 
GetImageEncoders(
    _In_ UINT numEncoders,
    _In_ IN UINT size,
    _Out_writes_bytes_(size) ImageCodecInfo *encoders)
{
    return DllExports::GdipGetImageEncoders(numEncoders, size, encoders);
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _GDIPLUSIMAGECODEC_H
