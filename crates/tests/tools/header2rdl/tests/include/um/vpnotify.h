//------------------------------------------------------------------------------
// File: VPNotify.h
//
// Desc: 
//
// Copyright (c) 1997 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __IVPNotify__
#define __IVPNotify__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

// interface IVPBaseNotify
DECLARE_INTERFACE_(IVPBaseNotify, IUnknown)
{
	// this function initializes the reconnection to the decoder. 
	STDMETHOD (RenegotiateVPParameters)(THIS
					   ) PURE;

};

// interface IVPNotify
DECLARE_INTERFACE_(IVPNotify, IVPBaseNotify)
{
    // function to set the mode (bob, weave etc)
    STDMETHOD (SetDeinterlaceMode)(THIS_ 
				   IN AMVP_MODE mode
				  ) PURE;

    // function to get the mode (bob, weave etc)
    STDMETHOD (GetDeinterlaceMode)(THIS_ 
				   OUT AMVP_MODE *pMode
				  ) PURE;
};

// interface IVPNotify
DECLARE_INTERFACE_(IVPNotify2, IVPNotify)
{
// function to set the mode (bob, weave etc)
    STDMETHOD (SetVPSyncMaster)(THIS_ 
				   IN BOOL bVPSyncMaster
				  ) PURE;

    // function to get the mode (bob, weave etc)
    STDMETHOD (GetVPSyncMaster)(THIS_ 
				   OUT BOOL *pbVPSyncMaster
				  ) PURE;

    /*
    // this function sets the directdraw surface that the mixer is supposed to use.
    STDMETHOD (SetDirectDrawSurface)(THIS_ 
				  IN LPDIRECTDRAWSURFACE pDirectDrawSurface
				 ) PURE;

    // this function gets the directdraw surface that the mixer is using
    STDMETHOD (GetDirectDrawSurface)(THIS_ 
				  OUT LPDIRECTDRAWSURFACE *ppDirectDrawSurface
				 ) PURE;

    // this functions sets the color-controls, if the chip supports it.
    STDMETHOD (SetVPColorControls)(THIS_ 
				 IN LPDDCOLORCONTROL pColorControl
				) PURE;

    // this functions also returns the capability of the hardware in the dwFlags
    // value of the struct.
    STDMETHOD (GetVPColorControls)(THIS_ 
				 OUT LPDDCOLORCONTROL *ppColorControl
				) PURE;
    */
};


// interface IVPVBINotify
DECLARE_INTERFACE_(IVPVBINotify, IVPBaseNotify)
{
};

#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __IVPNotify__
