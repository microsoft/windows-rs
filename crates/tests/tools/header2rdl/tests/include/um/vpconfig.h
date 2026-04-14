//------------------------------------------------------------------------------
// File: VPConfig.h
//
// Desc: An interface exposed by the decoder to help it and the filter
//       configuring the videoport to communicate.
//
// Copyright (c) 1992 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __IVPConfig__
#define __IVPConfig__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

// IVPBaseConfig
DECLARE_INTERFACE_(IVPBaseConfig, IUnknown)
{
    // gets the various connection information structures (guid, portwidth)
    // in an array of structures. If the pointer to the array is NULL, first 
    // parameter returns the total number of formats supported.
    STDMETHOD (GetConnectInfo)(THIS_
			       _Inout_ LPDWORD pdwNumConnectInfo,
			       _Out_writes_to_opt_(*pdwNumConnectInfo, *pdwNumConnectInfo) LPDDVIDEOPORTCONNECT pddVPConnectInfo
			      ) PURE;

    // sets the connection entry chosen (0, 1, .. ,(dwNumProposedEntries-1))
    STDMETHOD (SetConnectInfo)(THIS_
			       IN DWORD dwChosenEntry
			      ) PURE;

    // gets various data parameters, includes dimensionnal info
    STDMETHOD (GetVPDataInfo)(THIS_
			      IN OUT LPAMVPDATAINFO pamvpDataInfo
			     ) PURE;

    // retrives maximum pixels per second rate expected for a given 
    // format and a given scaling factor. If decoder does not support 
    // those scaling factors, then it gives the rate and the nearest 
    // scaling factors.
    STDMETHOD (GetMaxPixelRate)(THIS_
				IN OUT LPAMVPSIZE pamvpSize,
				OUT LPDWORD pdwMaxPixelsPerSecond
			       ) PURE;

    // informs the callee of the videoformats supported by the videoport
    STDMETHOD (InformVPInputFormats)(THIS_
				     IN DWORD dwNumFormats,
				     IN LPDDPIXELFORMAT pDDPixelFormats
				    ) PURE;

    // gets the various formats supported by the decoder in an array
    // of structures. If the pointer to the array is NULL, first parameter
    // returns the total number of formats supported.
    STDMETHOD (GetVideoFormats)(THIS_
				_Inout_ LPDWORD pdwNumFormats,
				_Out_writes_to_opt_(*pdwNumFormats, *pdwNumFormats) LPDDPIXELFORMAT pddPixelFormats
			       ) PURE;

    // sets the format entry chosen (0, 1, .. ,(dwNumProposedEntries-1))
    STDMETHOD (SetVideoFormat)(THIS_
			       IN DWORD dwChosenEntry
			      ) PURE;

    // asks the decoder to treat even fields like odd fields and visa versa
    STDMETHOD (SetInvertPolarity)(THIS
				 ) PURE;

    // the mixer uses this function to determine if the callee wants
    // the vpmixer to use its overlay surface and if so to get a pointer to it
    STDMETHOD (GetOverlaySurface)(THIS_
				  OUT LPDIRECTDRAWSURFACE* ppddOverlaySurface
				 ) PURE;

    // sets the direct draw kernel handle
    STDMETHOD (SetDirectDrawKernelHandle)(THIS_
					  IN ULONG_PTR dwDDKernelHandle
					 ) PURE;

    // sets the video port id
    STDMETHOD (SetVideoPortID)(THIS_
			       IN DWORD dwVideoPortID
			      ) PURE;

    // sets the direct draw surface kernel handle
    STDMETHOD (SetDDSurfaceKernelHandles)(THIS_
					  IN DWORD cHandles,
					  IN ULONG_PTR *rgDDKernelHandles
					 ) PURE;

    // Tells driver about surface created on its behalf by ovmixer/vbisurf and 
    // returned from videoport/ddraw. Should always return NOERROR or E_NOIMPL. 
    // dwPitch is the pitch of the surface (distance in pixels between the start 
    // pixels of two consecutive lines of the surface). (dwXOrigin, dwYOrigin) 
    // are the (X, Y) coordinates of the pixel at which valid data starts.  
    STDMETHOD (SetSurfaceParameters)(THIS_
                    IN DWORD dwPitch,
                    IN DWORD dwXOrigin,
                    IN DWORD dwYOrigin
                    ) PURE;
};

// IVPConfig
DECLARE_INTERFACE_(IVPConfig, IVPBaseConfig)
{
	// the mixer uses this function to determine if the callee wants
	// the mixer to decimate VIDEO data at its own descrition
	STDMETHOD (IsVPDecimationAllowed)(THIS_
					  OUT LPBOOL pbIsDecimationAllowed
					 ) PURE;

	// sets the scaling factors. If decoder does not support these,
	// then it sets the values to the nearest factors it can support
	STDMETHOD (SetScalingFactors)(THIS_
				      IN LPAMVPSIZE pamvpSize
				     ) PURE;
};

// IVPVBIConfig
DECLARE_INTERFACE_(IVPVBIConfig, IVPBaseConfig)
{
};

#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __IVPConfig__
