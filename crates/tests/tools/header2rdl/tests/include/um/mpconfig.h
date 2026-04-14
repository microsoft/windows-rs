//------------------------------------------------------------------------------
// File: MPConfig.h
//
// Desc: 
//
// Copyright (c) 1997 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __IMPConfig__
#define __IMPConfig__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

typedef enum _AM_ASPECT_RATIO_MODE
{
    AM_ARMODE_STRETCHED,	    // don't do any aspect ratio correction
    AM_ARMODE_LETTER_BOX,	    // letter box the video, paint background color in the excess region
    AM_ARMODE_CROP,		    // crop the video to the right aspect ratio
    AM_ARMODE_STRETCHED_AS_PRIMARY  // follow whatever the primary stream does (in terms of the mode as well as pict-aspect-ratio values)
} AM_ASPECT_RATIO_MODE;


DECLARE_INTERFACE_(IMixerPinConfig, IUnknown)
{
    // this function sets the position of the stream in the display window, assuming 
    // that the window coordinates are {0, 0, 10000, 10000}. Thus giving arguments
    // (0, 0, 5000, 5000) will put the stream in the top-left quarter. Any value greater
    // than 10000 is invalid.
    STDMETHOD (SetRelativePosition)(THIS_ 
				    IN DWORD dwLeft,
				    IN DWORD dwTop,
				    IN DWORD dwRight,
				    IN DWORD dwBottom
				   ) PURE;

    // this function gets the position of the stream in the display window, assuming 
    // that the window coordinates are {0, 0, 10000, 10000}. Thus if the values returned
    // are (0, 0, 5000, 5000), then it means that the stream is in the top-left quarter. 
    STDMETHOD (GetRelativePosition)(THIS_ 
				    OUT DWORD *pdwLeft,
				    OUT DWORD *pdwTop,
				    OUT DWORD *pdwRight,
				    OUT DWORD *pdwBottom
				   ) PURE;

    // this function sets the ZOrder of the stream. The ZOrder of 0 is the closest
    // to the eyes of the user, and increasing values imply greater distance.
    STDMETHOD (SetZOrder)(THIS_ 
			  IN DWORD dwZOrder
			 ) PURE;


    // this function gets the ZOrder of the stream. The ZOrder of 0 is the closest
    // to the eyes of the user, and increasing values imply greater distance.
    STDMETHOD (GetZOrder)(THIS_ 
			  OUT DWORD *pdwZOrder
			 ) PURE;

    // this function sets the colorkey being used by the stream. Setting this value on the 
    // primary stream sets the destination colorkey being used by the overlay surface. Setting
    // this value on the secondary pin makes sense only if the stream is transparent. By default
    // the destination colorkey is used as the colorkey for all transparent (secondary) streams.
    STDMETHOD (SetColorKey)(THIS_ 
			    IN COLORKEY *pColorKey
			   ) PURE;

    // this function gets the colorkey being used by the stream. Getting this value on the 
    // primary stream gets the destination colorkey being used by the overlay surface. Getting
    // this value on the secondary pin returns the colorkey being used by that particular stream.
    // When using this method, you are allowed to pass NULL for either pColorKey or pColor but 
    // not both.
    STDMETHOD (GetColorKey)(THIS_ 
			    OUT COLORKEY *pColorKey,
			    OUT DWORD *pColor
			   ) PURE;

    // this function sets the blending parameter which in turn defines, how the secondary stream 
    // is going to be blended with the primary stream. A value of 0 makes the secondary stream 
    // invisible, a value of 255 makes the primary stream invisible (in that region only ofcourse),
    // and any value inbetween, say x, blends the secondary and primary streams in the ratio x : (255-x).
    // If no value is set, the default is 255.
    // Any value less than 0 or greater than 255 is invalid. Calling this function on the primary
    // stream would result in a return value of E_UNEXPECTED.
    STDMETHOD (SetBlendingParameter)(THIS_ 
				     IN DWORD dwBlendingParameter
				    ) PURE;

    // this function gets the blending parameter which in turn defines, how the secondary stream 
    // is currently being blended with the primary stream. 
    STDMETHOD (GetBlendingParameter)(THIS_ 
				     OUT DWORD *pdwBlendingParameter
				    ) PURE;


    // this function is used to set the Aspect Ratio Correction mode on the pin. 
    // If the mode is set to letter box, black color is painted on the excess region 
    STDMETHOD (SetAspectRatioMode)(THIS_ 
				   IN AM_ASPECT_RATIO_MODE amAspectRatioMode
				  ) PURE;

    // this function is used to get the Aspect Ratio Correction mode on the pin.  
    STDMETHOD (GetAspectRatioMode)(THIS_ 
				   OUT AM_ASPECT_RATIO_MODE* pamAspectRatioMode
				  ) PURE;

    // this function sets the stream to be transparent. That means that the stream is not going
    // to occupy the whole of the rectangle (specified by SetRelativePosition), some of the region
    // is going to be transparent i.e. the stream underneath, is going to see through.
    // Calling this function on the primary stream would result in a return value of E_UNEXPECTED.
    STDMETHOD (SetStreamTransparent)(THIS_ 
				     IN BOOL bStreamTransparent
				    ) PURE;

    // this function is used to tell whether the stream is transparent or not. 
    STDMETHOD (GetStreamTransparent)(THIS_ 
				     OUT BOOL *pbStreamTransparent
				    ) PURE;
};


DECLARE_INTERFACE_(IMixerPinConfig2, IMixerPinConfig)
{
    // this functions sets the color-controls, if the vga chip supports it.
    STDMETHOD (SetOverlaySurfaceColorControls)(THIS_ 
					       IN LPDDCOLORCONTROL pColorControl
					      ) PURE;

    // this functions gets the color-controls. It also returns the capability of the vga hardware 
    // in the dwFlags value of the struct.
    STDMETHOD (GetOverlaySurfaceColorControls)(THIS_ 
					       OUT LPDDCOLORCONTROL pColorControl
					      ) PURE;
};

#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // #define __IMPConfig__

