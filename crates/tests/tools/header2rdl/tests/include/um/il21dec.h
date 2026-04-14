//------------------------------------------------------------------------------
// File: IL21Dec.h
//
// Desc: Line 21 Decoder related definitions and interfaces for ActiveMovie.
//
// Copyright (c) 1992 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __IL21DEC__
#define __IL21DEC__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



//
//  Some enum data types used as line 21 decoder params by the interface
//
typedef enum _AM_LINE21_CCLEVEL {  // should we use TC1, TC2 in stead?
    AM_L21_CCLEVEL_TC2 = 0
} AM_LINE21_CCLEVEL, *PAM_LINE21_CCLEVEL ;

typedef enum _AM_LINE21_CCSERVICE {
    AM_L21_CCSERVICE_None = 0,
    AM_L21_CCSERVICE_Caption1,
    AM_L21_CCSERVICE_Caption2,
    AM_L21_CCSERVICE_Text1,
    AM_L21_CCSERVICE_Text2,
    AM_L21_CCSERVICE_XDS,
    AM_L21_CCSERVICE_DefChannel = 10,
    AM_L21_CCSERVICE_Invalid
} AM_LINE21_CCSERVICE, *PAM_LINE21_CCSERVICE ;

typedef enum _AM_LINE21_CCSTATE {
    AM_L21_CCSTATE_Off = 0,
    AM_L21_CCSTATE_On
} AM_LINE21_CCSTATE, *PAM_LINE21_CCSTATE ;

typedef enum _AM_LINE21_CCSTYLE {
    AM_L21_CCSTYLE_None = 0,
    AM_L21_CCSTYLE_PopOn,
    AM_L21_CCSTYLE_PaintOn,
    AM_L21_CCSTYLE_RollUp
} AM_LINE21_CCSTYLE, *PAM_LINE21_CCSTYLE ;

typedef enum _AM_LINE21_DRAWBGMODE {
    AM_L21_DRAWBGMODE_Opaque,
    AM_L21_DRAWBGMODE_Transparent
} AM_LINE21_DRAWBGMODE, *PAM_LINE21_DRAWBGMODE ;


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

//
//  Line 21 Decoder standard COM interface
//
DECLARE_INTERFACE_(IAMLine21Decoder, IUnknown)
{
    //
    // Decoder options to be used by apps
    //

    // What is the decoder's level
    STDMETHOD(GetDecoderLevel)(THIS_ AM_LINE21_CCLEVEL *lpLevel) PURE ;  
    // supported level value is AM_L21Level_TC2 only
    // skipping the SetDecoderLevel( )

    // Which of the services is being currently used
    STDMETHOD(GetCurrentService)(THIS_ AM_LINE21_CCSERVICE *lpService) PURE ;  
    STDMETHOD(SetCurrentService)(THIS_ AM_LINE21_CCSERVICE Service) PURE ;  
    // supported service values are AM_L21Service_Caption1, 
    // AM_L21Service_Caption2, AM_L21Service_Text1, AM_L21Service_Text2, 
    // AM_L21Service_XDS, AM_L21Service_None)

    // Query/Set the service state (On/Off)
    // supported state values are AM_L21State_On and AM_L21State_Off
    STDMETHOD(GetServiceState)(THIS_ AM_LINE21_CCSTATE *lpState) PURE ;  
    STDMETHOD(SetServiceState)(THIS_ AM_LINE21_CCSTATE State) PURE ;  

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
    STDMETHOD(GetDrawBackgroundMode)(THIS_ AM_LINE21_DRAWBGMODE *lpMode) PURE ;
    STDMETHOD(SetDrawBackgroundMode)(THIS_ AM_LINE21_DRAWBGMODE Mode) PURE ;
    // supported mode values are AM_L21_DrawBGMode_Opaque and
    // AM_L21_DrawBGMode_Transparent

} ;

#ifdef __cplusplus
}
#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __IL21DEC__
