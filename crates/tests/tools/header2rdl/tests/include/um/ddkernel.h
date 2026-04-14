/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:	ddkernel.h
 *  Content:	APIs for accessing kernel mode support.
 *
 ***************************************************************************/

#ifndef __DDKM_INCLUDED__
#define __DDKM_INCLUDED__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if defined( _WIN32 )  && !defined( _NO_COM )
#define COM_NO_WINDOWS_H
#include <objbase.h>
#else
#define IUnknown	    void
#undef  CO_E_NOTINITIALIZED
#define CO_E_NOTINITIALIZED 0x800401F0L
#endif

#ifdef __cplusplus
extern "C" {
#endif

/*
 * GUIDS used by to get kernel interfaces
 */
#if defined( _WIN32 ) && !defined( _NO_COM )
DEFINE_GUID( IID_IDirectDrawKernel,             0x8D56C120,0x6A08,0x11D0,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8 );
DEFINE_GUID( IID_IDirectDrawSurfaceKernel,      0x60755DA0,0x6A40,0x11D0,0x9B,0x06,0x00,0xA0,0xC9,0x03,0xA3,0xB8 );

#endif

/*============================================================================
 *
 * DirectDraw Structures
 *
 * Various structures used to invoke the kernel API functions.
 *
 *==========================================================================*/

typedef struct IDirectDrawKernel		FAR *LPDIRECTDRAWKERNEL;
typedef struct IDirectDrawSurfaceKernel		FAR *LPDIRECTDRAWSURFACEKERNEL;
typedef struct _DDKERNELCAPS			FAR *LPDDKERNELCAPS;


/*
 * INTERACES FOLLOW:
 *	IDirectDrawKernel
 *	IVideoPort
 */

/*
 * IDirectDrawKernel
 */
#if defined( _WIN32 ) && !defined( _NO_COM )
#undef INTERFACE
#define INTERFACE IDirectDrawKernel
DECLARE_INTERFACE_( IDirectDrawKernel, IUnknown )
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID FAR * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    /*** IDirectDraw methods ***/
    STDMETHOD(GetCaps)(THIS_ LPDDKERNELCAPS) PURE;
    STDMETHOD(GetKernelHandle)(THIS_ PULONG_PTR) PURE;
    STDMETHOD(ReleaseKernelHandle)(THIS) PURE;
};

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IDirectDrawKernel_GetCaps(p, a)             (p)->lpVtbl->GetCaps(p, a)
#define IDirectDrawKernel_GetKernelHandle(p, a)     (p)->lpVtbl->GetKernelHandle(p, a)
#define IDirectDrawKernel_ReleaseKernelHandle(p)    (p)->lpVtbl->ReleaseKernelHandle(p)
#endif

#endif

/*
 * IDirectDrawSurfaceKernel
 */
#if defined( _WIN32 ) && !defined( _NO_COM )
#undef INTERFACE
#define INTERFACE IDirectDrawSurfaceKernel
DECLARE_INTERFACE_( IDirectDrawSurfaceKernel, IUnknown )
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID FAR * ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS)  PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;
    /*** IDirectDraw methods ***/
    STDMETHOD(GetKernelHandle)(THIS_ PULONG_PTR) PURE;
    STDMETHOD(ReleaseKernelHandle)(THIS) PURE;
};

#if !defined(__cplusplus) || defined(CINTERFACE)
#define IDirectDrawSurfaceKernel_GetKernelHandle(p, a)     (p)->lpVtbl->GetKernelHandle(p, a)
#define IDirectDrawSurfaceKernel_ReleaseKernelHandle(p)    (p)->lpVtbl->ReleaseKernelHandle(p)
#endif

#endif


/*
 * DDKERNELCAPS
 */
typedef struct _DDKERNELCAPS
{
    DWORD dwSize;			// size of the DDKERNELCAPS structure
    DWORD dwCaps;                       // Contains the DDKERNELCAPS_XXX flags
    DWORD dwIRQCaps;                    // Contains the DDIRQ_XXX flags
} DDKERNELCAPS, FAR *LPDDKERNELCAPS;



/****************************************************************************
 *
 * DDKERNELCAPS CAPS
 *
 ****************************************************************************/

/*
 * Indicates that the device supports field skipping.
 */
#define DDKERNELCAPS_SKIPFIELDS			0x00000001l

/*
 * Indicates that the device can support software autoflipping.
 */
#define DDKERNELCAPS_AUTOFLIP			0x00000002l

/*
 * Indicates that the device can switch between bob and weave.
 */
#define DDKERNELCAPS_SETSTATE			0x00000004l

/*
 * Indicates that a client can gain direct access to the frame buffer.
 */
#define DDKERNELCAPS_LOCK			0x00000008l

/*
 * Indicates that a client can manually flip the video port.
 */
#define DDKERNELCAPS_FLIPVIDEOPORT		0x00000010l

/*
 * Indicates that a client can manually flip the overlay.
 */
#define DDKERNELCAPS_FLIPOVERLAY		0x00000020l

/*
 * Indicates that the device supports a video port capture interface
 * capable of transfering data to system memory.
 */
#define DDKERNELCAPS_CAPTURE_SYSMEM		0x00000040l

/*
 * Indicates that the device supports a video port capture interface
 * capable of transfering data to non-local video memory.
 */
#define DDKERNELCAPS_CAPTURE_NONLOCALVIDMEM	0x00000080l

/*
 * Indicates that the device can report the polarity (even/odd) of
 * the curent video field.
 */
#define DDKERNELCAPS_FIELDPOLARITY		0x00000100l

/*
 * Indicates that the device supports inverting the DIBs while capturing
 * the data.
 */
#define DDKERNELCAPS_CAPTURE_INVERTED		0x00000200l

/****************************************************************************
 *
 * DDKERNELCAPS IRQ CAPS
 *
 ****************************************************************************/

/*
 * The device can generate display VSYNC IRQs
 */
#define DDIRQ_DISPLAY_VSYNC			0x00000001l

/*
 * Reserved
 */
#define DDIRQ_RESERVED1				0x00000002l

/*
 * The device can generate video ports VSYNC IRQs using video port 0
 */
#define DDIRQ_VPORT0_VSYNC			0x00000004l

/*
 * The device can generate video ports line IRQs using video port 0
 */
#define DDIRQ_VPORT0_LINE			0x00000008l

/*
 * The device can generate video ports VSYNC IRQs using video port 1
 */
#define DDIRQ_VPORT1_VSYNC			0x00000010l

/*
 * The device can generate video ports line IRQs using video port 1
 */
#define DDIRQ_VPORT1_LINE			0x00000020l

/*
 * The device can generate video ports VSYNC IRQs using video port 2
 */
#define DDIRQ_VPORT2_VSYNC			0x00000040l

/*
 * The device can generate video ports line IRQs using video port 2
 */
#define DDIRQ_VPORT2_LINE			0x00000080l

/*
 * The device can generate video ports VSYNC IRQs using video port 3
 */
#define DDIRQ_VPORT3_VSYNC			0x00000100l

/*
 * The device can generate video ports line IRQs using video port 3
 */
#define DDIRQ_VPORT3_LINE			0x00000200l

/*
 * The device can generate video ports VSYNC IRQs using video port 4
 */
#define DDIRQ_VPORT4_VSYNC			0x00000400l

/*
 * The device can generate video ports line IRQs using video port 4
 */
#define DDIRQ_VPORT4_LINE			0x00000800l

/*
 * The device can generate video ports VSYNC IRQs using video port 5
 */
#define DDIRQ_VPORT5_VSYNC			0x00001000l

/*
 * The device can generate video ports line IRQs using video port 5
 */
#define DDIRQ_VPORT5_LINE			0x00002000l

/*
 * The device can generate video ports VSYNC IRQs using video port 6
 */
#define DDIRQ_VPORT6_VSYNC			0x00004000l

/*
 * The device can generate video ports line IRQs using video port 6
 */
#define DDIRQ_VPORT6_LINE			0x00008000l

/*
 * The device can generate video ports VSYNC IRQs using video port 7
 */
#define DDIRQ_VPORT7_VSYNC			0x00010000l

/*
 * The device can generate video ports line IRQs using video port 7
 */
#define DDIRQ_VPORT7_LINE			0x00020000l

/*
 * The device can generate video ports VSYNC IRQs using video port 8
 */
#define DDIRQ_VPORT8_VSYNC			0x00040000l

/*
 * The device can generate video ports line IRQs using video port 8
 */
#define DDIRQ_VPORT8_LINE			0x00080000l

/*
 * The device can generate video ports VSYNC IRQs using video port 9
 */
#define DDIRQ_VPORT9_VSYNC			0x00010000l

/*
 * The device can generate video ports line IRQs using video port 9
 */
#define DDIRQ_VPORT9_LINE			0x00020000l



#ifdef __cplusplus
};
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif



