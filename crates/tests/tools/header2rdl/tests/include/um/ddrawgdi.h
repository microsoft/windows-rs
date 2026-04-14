/******************************Module*Header*******************************\
* Module Name: ddrawgdi.h
*
* Structures and defines for the private entry points in GDI to support
* DirectDraw.
*
* Copyright (c) Microsoft Corporation. All rights reserved.
\**************************************************************************/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef  COMBOX_SANDBOX
#define DX_LONGHORN_PRESERVEDC
#endif

// We rename the actual entry points for added protection against anyone
// trying to call our private entry points directly:

#ifndef NODDRAWGDI
#define DdCreateDirectDrawObject            GdiEntry1
#define DdQueryDirectDrawObject             GdiEntry2
#define DdDeleteDirectDrawObject            GdiEntry3
#define DdCreateSurfaceObject               GdiEntry4
#define DdDeleteSurfaceObject               GdiEntry5
#define DdResetVisrgn                       GdiEntry6
#define DdGetDC                             GdiEntry7
#define DdReleaseDC                         GdiEntry8
#define DdCreateDIBSection                  GdiEntry9
#define DdReenableDirectDrawObject          GdiEntry10
#define DdAttachSurface                     GdiEntry11
#define DdUnattachSurface                   GdiEntry12
#define DdQueryDisplaySettingsUniqueness    GdiEntry13
#define DdGetDxHandle                       GdiEntry14
#define DdSetGammaRamp                      GdiEntry15
#define DdSwapTextureHandles                GdiEntry16

#ifdef DX_LONGHORN_PRESERVEDC
#define DdChangeSurfacePointer              GdiEntry17
#endif // DX_LONGHORN_PRESERVEDC

#endif

BOOL
APIENTRY
DdCreateDirectDrawObject(
    LPDDRAWI_DIRECTDRAW_GBL pDirectDrawGlobal,
    HDC                     hdc
    );

typedef struct _D3DHAL_CALLBACKS FAR *LPD3DHAL_CALLBACKS;
typedef struct _D3DHAL_GLOBALDRIVERDATA FAR *LPD3DHAL_GLOBALDRIVERDATA;

BOOL
APIENTRY
DdQueryDirectDrawObject(
    LPDDRAWI_DIRECTDRAW_GBL     pDirectDrawGlobal,
    LPDDHALINFO                 pHalInfo,
    LPDDHAL_DDCALLBACKS         pDDCallbacks,
    LPDDHAL_DDSURFACECALLBACKS  pDDSurfaceCallbacks,
    LPDDHAL_DDPALETTECALLBACKS  pDDPaletteCallbacks,
    LPD3DHAL_CALLBACKS          pD3dCallbacks,
    LPD3DHAL_GLOBALDRIVERDATA   pD3dDriverData,
    LPDDHAL_DDEXEBUFCALLBACKS   pD3dBufferCallbacks,
    LPDDSURFACEDESC             pD3dTextureFormats,
    LPDWORD                     pdwFourCC,         // Can be NULL
    LPVIDMEM                    pvmList            // Can be NULL
    );

BOOL
APIENTRY
DdDeleteDirectDrawObject(
    LPDDRAWI_DIRECTDRAW_GBL   pDirectDrawGlobal
    );

BOOL
APIENTRY
DdCreateSurfaceObject(
    LPDDRAWI_DDRAWSURFACE_LCL pSurfaceLocal,
    BOOL                      bPrimarySurface
    );

BOOL
APIENTRY
DdDeleteSurfaceObject(
    LPDDRAWI_DDRAWSURFACE_LCL pSurfaceLocal
    );

BOOL
APIENTRY
DdResetVisrgn(
    LPDDRAWI_DDRAWSURFACE_LCL pSurfaceLocal,
    HWND                      hWnd
    );

HDC
APIENTRY
DdGetDC(
    LPDDRAWI_DDRAWSURFACE_LCL pSurfaceLocal,
    LPPALETTEENTRY            pColorTable
    );

BOOL
APIENTRY
DdReleaseDC(
    LPDDRAWI_DDRAWSURFACE_LCL pSurfaceLocal
    );

HBITMAP
APIENTRY
DdCreateDIBSection(
    HDC               hdc,
    CONST BITMAPINFO* pbmi,
    UINT              iUsage,
    VOID**            ppvBits,
    HANDLE            hSectionApp,
    DWORD             dwOffset
    );

BOOL
APIENTRY
DdReenableDirectDrawObject(
    LPDDRAWI_DIRECTDRAW_GBL pDirectDrawGlobal,
    BOOL*                   pbNewMode
    );

BOOL
APIENTRY
DdAttachSurface(
    LPDDRAWI_DDRAWSURFACE_LCL   pSurfaceFrom,
    LPDDRAWI_DDRAWSURFACE_LCL   pSurfaceTo
    );

VOID
APIENTRY
DdUnattachSurface(
    LPDDRAWI_DDRAWSURFACE_LCL   pSurface,
    LPDDRAWI_DDRAWSURFACE_LCL   pSurfaceAttached
    );

ULONG
APIENTRY
DdQueryDisplaySettingsUniqueness(
    VOID
    );

HANDLE
APIENTRY
DdGetDxHandle(
    LPDDRAWI_DIRECTDRAW_LCL	pDDraw,
    LPDDRAWI_DDRAWSURFACE_LCL   pSurface,
    BOOL			bRelease
    );

BOOL
APIENTRY
DdSetGammaRamp(
    LPDDRAWI_DIRECTDRAW_LCL	pDDraw,
    HDC                         hdc,
    LPVOID                      lpGammaRamp
    );



DWORD 
APIENTRY
DdSwapTextureHandles(
    LPDDRAWI_DIRECTDRAW_LCL	pDDraw,
    LPDDRAWI_DDRAWSURFACE_LCL  pDDSLcl1,
    LPDDRAWI_DDRAWSURFACE_LCL  pDDSLcl2
    );

#ifdef DX_LONGHORN_PRESERVEDC

DWORD
APIENTRY
DdChangeSurfacePointer(
    LPDDRAWI_DDRAWSURFACE_LCL   pSurfaceLocal,
    LPDDRAWI_DIRECTDRAW_GBL     pDirectDrawGlobal,
    LPVOID                      pSurfacePointer
    );

#endif // DX_LONGHORN_PRESERVEDC

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

