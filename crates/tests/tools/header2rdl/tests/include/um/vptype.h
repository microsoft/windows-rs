//------------------------------------------------------------------------------
// File: VPType.h
//
// Desc: This file includes all the data structures defined for the IVPConfig
//       interface.
//
// Copyright (c) 1997 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __IVPType__
#define __IVPType__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

    // enum to specify the criterion, which the vpmixer is supposed to use
    // in order to select the video format
    typedef enum _AMVP_SELECT_FORMAT_BY
    {
	AMVP_DO_NOT_CARE,
	AMVP_BEST_BANDWIDTH,
	AMVP_INPUT_SAME_AS_OUTPUT
    } AMVP_SELECT_FORMAT_BY;

    // enum to specify the various mode
    typedef enum _AMVP_MODE
    {	
	AMVP_MODE_WEAVE,
	AMVP_MODE_BOBINTERLEAVED,
	AMVP_MODE_BOBNONINTERLEAVED,
	AMVP_MODE_SKIPEVEN,
	AMVP_MODE_SKIPODD
    } AMVP_MODE;

    // struct to specify the width and height. The context could be anything
    // such as scaling cropping etc.
    typedef struct _AMVPSIZE
    {
	DWORD			dwWidth;				// the width
	DWORD			dwHeight;				// the height
    } AMVPSIZE, *LPAMVPSIZE;

    // struct to specify the dimensional characteristics of the input stream
    typedef struct _AMVPDIMINFO
    {
	DWORD			dwFieldWidth;				// Field height of the data
	DWORD			dwFieldHeight;				// Field width of the data
	DWORD			dwVBIWidth;				// Width of the VBI data
	DWORD			dwVBIHeight;				// Height of the VBI data
	RECT			rcValidRegion;				// The vaild rectangle, used for cropping
    } AMVPDIMINFO, *LPAMVPDIMINFO;

    // struct to specify the various data specific characteristics of the input stream
    typedef struct _AMVPDATAINFO
    {
	DWORD			dwSize;					// Size of the struct
	DWORD			dwMicrosecondsPerField;			// Time taken by each field
	AMVPDIMINFO		amvpDimInfo;				// Dimensional Information 
	DWORD			dwPictAspectRatioX;			// X dimension of Picture Aspect Ratio
	DWORD			dwPictAspectRatioY;			// Y dimension of Picture Aspect Ratio
	BOOL			bEnableDoubleClock;			// Videoport should enable double clocking
	BOOL			bEnableVACT;				// Videoport should use an external VACT signal
	BOOL			bDataIsInterlaced;			// Indicates that the signal is interlaced
	LONG			lHalfLinesOdd;				// number of halflines in the odd field
	BOOL			bFieldPolarityInverted;			// Device inverts the polarity by default
	DWORD			dwNumLinesInVREF;			// Number of lines of data in VREF 
	LONG			lHalfLinesEven;				// number of halflines in the even field
	DWORD			dwReserved1;				// Reserved for future use
    } AMVPDATAINFO, *LPAMVPDATAINFO; 


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __IVPType__
