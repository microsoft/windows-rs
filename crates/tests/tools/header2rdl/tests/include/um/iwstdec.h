//------------------------------------------------------------------------------
// File: iwstdec.h
//
// Desc: WST Decoder related definitions and interfaces for ActiveMovie
//
// Copyright (c) 1999 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------
// 

#ifndef __IWSTDEC__
#define __IWSTDEC__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
//  Some data types used as WST decoder parameters by the interface
//
typedef struct _AM_WST_PAGE {
	DWORD	dwPageNr ;
	DWORD	dwSubPageNr ;
	BYTE	*pucPageData;
} AM_WST_PAGE, *PAM_WST_PAGE ;

typedef enum _AM_WST_LEVEL {
	AM_WST_LEVEL_1_5 = 0
} AM_WST_LEVEL, *PAM_WST_LEVEL ;

typedef enum _AM_WST_SERVICE {
	AM_WST_SERVICE_None = 0,
	AM_WST_SERVICE_Text,
	AM_WST_SERVICE_IDS,
	AM_WST_SERVICE_Invalid
} AM_WST_SERVICE, *PAM_WST_SERVICE ;

typedef enum _AM_WST_STATE {
	AM_WST_STATE_Off = 0,
	AM_WST_STATE_On
} AM_WST_STATE, *PAM_WST_STATE ;

typedef enum _AM_WST_STYLE {
	AM_WST_STYLE_None = 0,
	AM_WST_STYLE_Invers
} AM_WST_STYLE, *PAM_WST_STYLE ;

typedef enum _AM_WST_DRAWBGMODE {
	AM_WST_DRAWBGMODE_Opaque,
	AM_WST_DRAWBGMODE_Transparent
} AM_WST_DRAWBGMODE, *PAM_WST_DRAWBGMODE ;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

//
//  WST Decoder standard COM interface
//
DECLARE_INTERFACE_(IAMWstDecoder, IUnknown)
{
	//
	// Decoder options to be used by apps
	//

	// What is the decoder's level
	STDMETHOD(GetDecoderLevel)(THIS_ AM_WST_LEVEL *lpLevel) PURE ;  

//		STDMETHOD(SetDecoderLevel)(THIS_ AM_WST_LEVEL Level) PURE ;  

	// Which of the services is being currently used
	STDMETHOD(GetCurrentService)(THIS_ AM_WST_SERVICE *lpService) PURE ;  
//		STDMETHOD(SetCurrentService)(THIS_ AM_WST_SERVICE Service) PURE ;  

	// Query/Set the service state (On/Off)
	// supported state values are AM_WSTState_On and AM_WSTState_Off
	STDMETHOD(GetServiceState)(THIS_ AM_WST_STATE *lpState) PURE ;  
	STDMETHOD(SetServiceState)(THIS_ AM_WST_STATE State) PURE ;  

	//
	// Output options to be used by downstream filters
	//

	// What size, bitdepth etc should the output video be
	STDMETHOD(GetOutputFormat)(THIS_ LPBITMAPINFOHEADER lpbmih) PURE ;
	// GetOutputFormat() method, if successful, returns 
	// 1.  S_FALSE if no output format has so far been defined by downstream filters
	// 2.  S_OK if an output format has already been defined by downstream filters
	STDMETHOD(SetOutputFormat)(THIS_ LPBITMAPINFO lpbmi) PURE ;

	// Specify physical color to be used in colorkeying the background 
	// for overlay mixing
	STDMETHOD(GetBackgroundColor)(THIS_ DWORD *pdwPhysColor) PURE ;
	STDMETHOD(SetBackgroundColor)(THIS_ DWORD dwPhysColor) PURE ;

	// Specify if whole output bitmap should be redrawn for each sample
	STDMETHOD(GetRedrawAlways)(THIS_ LPBOOL lpbOption) PURE ;
	STDMETHOD(SetRedrawAlways)(THIS_ BOOL bOption) PURE ;

	// Specify if the caption text background should be opaque/transparent
	STDMETHOD(GetDrawBackgroundMode)(THIS_ AM_WST_DRAWBGMODE *lpMode) PURE ;
	STDMETHOD(SetDrawBackgroundMode)(THIS_ AM_WST_DRAWBGMODE Mode) PURE ;
	// supported mode values are AM_WST_DrawBGMode_Opaque and
	// AM_WST_DrawBGMode_Transparent

	STDMETHOD(SetAnswerMode)(THIS_ BOOL bAnswer) PURE ;
	STDMETHOD(GetAnswerMode)(THIS_ BOOL* pbAnswer) PURE ;

	STDMETHOD(SetHoldPage)(THIS_ BOOL bHoldPage) PURE ;
	STDMETHOD(GetHoldPage)(THIS_ BOOL* pbHoldPage) PURE ;

	STDMETHOD(GetCurrentPage)(THIS_ PAM_WST_PAGE pWstPage) PURE;
	STDMETHOD(SetCurrentPage)(THIS_ AM_WST_PAGE WstPage) PURE;

} ;

#ifdef __cplusplus
}
#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __IWSTDEC__
