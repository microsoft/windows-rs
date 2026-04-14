//
//    Copyright (C) Microsoft.  All rights reserved.
//

#if defined(_MSC_VER) && (_MSC_VER >= 1020)

#pragma once

#endif  // _MSC_VER

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

// Applications use this interface to denote the beginning and ending of a 
// "frame" of captured DXGI/D3D10+ commands for examination within a debugging 
// tool such as Visual Studio 2013. Absent such calls, a frame is defined by 
// Visual Studio as beginning and ending with the IDXGISwapChain*::Present* calls.
//
//  To retrieve a pointer to this interface, call 
//
//     ComPtr<IDXGraphicsAnalysis> graphicsAnalysis;
//     DXGIGetDebugInterface1(0, IID_PPV_ARGS(&graphicsAnalysis));
//
// The function will only succeed when the application is being run under 
// "graphics analysis" in Visual Studio (or other similar tools), so applications
// must be sure to handle failure from DXGIGetDebugInterface1 gracefully.
//
// Further note that DXGIGetDebugInterface1 is tagged as a development-only capability,
// which implies that linking to this function will cause the application to fail
// Windows store certification. Consequently, it is recommended that usage of that
// function be compiled only for pre-release code.

interface DECLSPEC_UUID("9f251514-9d4d-4902-9d60-18988ab7d4b5") DECLSPEC_NOVTABLE IDXGraphicsAnalysis : public IUnknown
{
    STDMETHOD_(void, BeginCapture)() PURE;
    STDMETHOD_(void, EndCapture)() PURE;
}; 

#endif // NTDDI_VERSION

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion