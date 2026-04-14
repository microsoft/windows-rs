/*++

Copyright (c) Microsoft Corporation. All Rights Reserved.

Module Name:

    ddrawint.h

Abstract:

    Private entry points, defines and types for Windows NT DirectDraw
    driver interface.  Corresponds to Windows' 'ddrawi.h' file.

    The structure names for NT are different from that of Win95.  Use
    dx95type.h to aid in porting DirectX code from Win95 to NT.

--*/

#ifndef __DD_INCLUDED__
#define __DD_INCLUDED__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * These GUIDs are used to identify driver info structures, not interfaces,
 * so the prefix GUID_ is used instead of IID_.
 */

DEFINE_GUID( GUID_MiscellaneousCallbacks,       0xefd60cc0, 0x49e7, 0x11d0, 0x88, 0x9d, 0x0, 0xaa, 0x0, 0xbb, 0xb7, 0x6a);
DEFINE_GUID( GUID_Miscellaneous2Callbacks,      0x406B2F00, 0x3E5A, 0x11D1, 0xB6, 0x40, 0x00, 0xAA, 0x00, 0xA1, 0xF9, 0x6A);
DEFINE_GUID( GUID_VideoPortCallbacks,           0xefd60cc1, 0x49e7, 0x11d0, 0x88, 0x9d, 0x0, 0xaa, 0x0, 0xbb, 0xb7, 0x6a);
DEFINE_GUID( GUID_ColorControlCallbacks,        0xefd60cc2, 0x49e7, 0x11d0, 0x88, 0x9d, 0x0, 0xaa, 0x0, 0xbb, 0xb7, 0x6a);
DEFINE_GUID( GUID_MotionCompCallbacks,          0xb1122b40, 0x5dA5, 0x11d1, 0x8f, 0xcF, 0x00, 0xc0, 0x4f, 0xc2, 0x9b, 0x4e);
DEFINE_GUID( GUID_VideoPortCaps,                0xefd60cc3, 0x49e7, 0x11d0, 0x88, 0x9d, 0x0, 0xaa, 0x0, 0xbb, 0xb7, 0x6a);
DEFINE_GUID( GUID_D3DCaps,                      0x7bf06991, 0x8794, 0x11d0, 0x91, 0x39, 0x08, 0x00, 0x36, 0xd2, 0xef, 0x02);
DEFINE_GUID( GUID_D3DExtendedCaps, 		0x7de41f80, 0x9d93, 0x11d0, 0x89, 0xab, 0x00, 0xa0, 0xc9, 0x05, 0x41, 0x29);
DEFINE_GUID( GUID_D3DCallbacks,                 0x7bf06990, 0x8794, 0x11d0, 0x91, 0x39, 0x08, 0x00, 0x36, 0xd2, 0xef, 0x02);
DEFINE_GUID( GUID_D3DCallbacks2,                0xba584e1, 0x70b6, 0x11d0, 0x88, 0x9d, 0x0, 0xaa, 0x0, 0xbb, 0xb7, 0x6a);
DEFINE_GUID( GUID_D3DCallbacks3,                0xddf41230, 0xec0a, 0x11d0, 0xa9, 0xb6, 0x00, 0xaa, 0x00, 0xc0, 0x99, 0x3e);
DEFINE_GUID( GUID_NonLocalVidMemCaps,           0x86c4fa80, 0x8d84, 0x11d0, 0x94, 0xe8, 0x00, 0xc0, 0x4f, 0xc3, 0x41, 0x37);
DEFINE_GUID( GUID_KernelCallbacks,              0x80863800, 0x6B06, 0x11D0, 0x9B, 0x06, 0x0, 0xA0, 0xC9, 0x03, 0xA3, 0xB8);
DEFINE_GUID( GUID_KernelCaps,                   0xFFAA7540, 0x7AA8, 0x11D0, 0x9B, 0x06, 0x00, 0xA0, 0xC9, 0x03, 0xA3, 0xB8);
DEFINE_GUID( GUID_ZPixelFormats,                0x93869880, 0x36cf, 0x11d1, 0x9b, 0x1b, 0x0, 0xaa, 0x0, 0xbb, 0xb8, 0xae);
DEFINE_GUID( GUID_DDMoreCaps,                   0x880baf30, 0xb030, 0x11d0, 0x8e, 0xa7, 0x00, 0x60, 0x97, 0x97, 0xea, 0x5b);
DEFINE_GUID( GUID_D3DParseUnknownCommandCallback, 0x2e04ffa0, 0x98e4, 0x11d1, 0x8c, 0xe1, 0x0, 0xa0, 0xc9, 0x6, 0x29, 0xa8);
DEFINE_GUID( GUID_NTCallbacks,                  0x6fe9ecde, 0xdf89, 0x11d1, 0x9d, 0xb0, 0x00, 0x60, 0x08, 0x27, 0x71, 0xba);
DEFINE_GUID( GUID_DDMoreSurfaceCaps,            0x3b8a0466, 0xf269, 0x11d1, 0x88, 0x0b, 0x0, 0xc0, 0x4f, 0xd9, 0x30, 0xc5);
DEFINE_GUID( GUID_GetHeapAlignment,             0x42e02f16, 0x7b41, 0x11d2, 0x8b, 0xff, 0x0, 0xa0, 0xc9, 0x83, 0xea, 0xf6);
DEFINE_GUID( GUID_UpdateNonLocalHeap,           0x42e02f17, 0x7b41, 0x11d2, 0x8b, 0xff, 0x0, 0xa0, 0xc9, 0x83, 0xea, 0xf6);
DEFINE_GUID( GUID_NTPrivateDriverCaps,          0xfad16a23, 0x7b66, 0x11d2, 0x83, 0xd7, 0x0, 0xc0, 0x4f, 0x7c, 0xe5, 0x8c);
DEFINE_GUID( GUID_DDStereoMode,                 0xf828169c, 0xa8e8, 0x11d2, 0xa1, 0xf2, 0x0, 0xa0, 0xc9, 0x83, 0xea, 0xf6);
DEFINE_GUID( GUID_VPE2Callbacks,                0x52882147, 0x2d47, 0x469a, 0xa0, 0xd1, 0x3, 0x45, 0x58, 0x90, 0xf6, 0xc8);

#ifndef GUID_DEFS_ONLY

#ifndef _NO_DDRAWINT_NO_COM
#ifndef _NO_COM
#define _NO_COM
#include "ddraw.h"
#include "dvp.h"
#undef _NO_COM
#else
#include "ddraw.h"
#include "dvp.h"
#endif
#else
#include "ddraw.h"
#include "dvp.h"
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define MAKE_HRESULT(sev,fac,code) \
    ((HRESULT) (((unsigned long)(sev)<<31) | ((unsigned long)(fac)<<16) | ((unsigned long)(code))) )

/*
 * offset into video meory
 */
typedef ULONG_PTR FLATPTR;

/*
 * maximum number of surfaces that can be autoflipped between
 */
#define MAX_AUTOFLIP_BUFFERS    10

/*
 * Indicates the surface is D3D buffer, i.e., a linear chunk of
 * memory that holds a Direct3D structure. A driver reports this
 * cap to indicate that it can create buffers in video memory and
 * Direct3D uses this bit to request buffers. However, it is not
 * visible to the API.
 */
#define DDSCAPS_EXECUTEBUFFER DDSCAPS_RESERVED2
#define DDSCAPS_COMMANDBUFFER DDSCAPS_RESERVED3
#define DDSCAPS_VERTEXBUFFER DDSCAPS_RESERVED4


/*
 * This DDPF flag is used to indicate a DX8+ format capability entry in
 * the texture format list. It is not visible to applications.
 */
#define DDPF_D3DFORMAT                                          0x00200000l

/*
 * List of operations supported on formats in DX8+ texture list.
 * See the DX8 DDK for a complete description of these flags.
 */
#define D3DFORMAT_OP_TEXTURE                    0x00000001L
#define D3DFORMAT_OP_VOLUMETEXTURE              0x00000002L
#define D3DFORMAT_OP_CUBETEXTURE                0x00000004L
#define D3DFORMAT_OP_OFFSCREEN_RENDERTARGET     0x00000008L
#define D3DFORMAT_OP_SAME_FORMAT_RENDERTARGET   0x00000010L
#define D3DFORMAT_OP_ZSTENCIL                   0x00000040L
#define D3DFORMAT_OP_ZSTENCIL_WITH_ARBITRARY_COLOR_DEPTH 0x00000080L

// This format can be used as a render target if the current display mode
// is the same depth if the alpha channel is ignored. e.g. if the device 
// can render to A8R8G8B8 when the display mode is X8R8G8B8, then the
// format op list entry for A8R8G8B8 should have this cap. 
#define D3DFORMAT_OP_SAME_FORMAT_UP_TO_ALPHA_RENDERTARGET 0x00000100L

// This format contains DirectDraw support (including Flip).  This flag
// should not to be set on alpha formats.
#define D3DFORMAT_OP_DISPLAYMODE                0x00000400L

// The rasterizer can support some level of Direct3D support in this format
// and implies that the driver can create a Context in this mode (for some 
// render target format).  When this flag is set, the D3DFORMAT_OP_DISPLAYMODE
// flag must also be set.
#define D3DFORMAT_OP_3DACCELERATION             0x00000800L

// If the driver needs a private format to be D3D or driver manageable,
// then it needs to tell D3D the pixelsize in bits per pixel by setting
// dwPrivateFormatBitCount in DDPIXELFORMAT and by setting the below
// format op. If the below format op is not set, then D3D or the driver
// will NOT be allowed to manage the format.
#define D3DFORMAT_OP_PIXELSIZE                  0x00001000L

// Indicates that this format can be converted to any RGB format for which
// D3DFORMAT_MEMBEROFGROUP_ARGB is specified
#define D3DFORMAT_OP_CONVERT_TO_ARGB            0x00002000L

// Indicates that this format can be used to create offscreen plain surfaces.
#define D3DFORMAT_OP_OFFSCREENPLAIN             0x00004000L

// Indicated that this format can be read as an SRGB texture (meaning that the
// sampler will linearize the looked up data)
#define D3DFORMAT_OP_SRGBREAD                   0x00008000L

// Indicates that this format can be used in the bumpmap instructions
#define D3DFORMAT_OP_BUMPMAP                    0x00010000L

// Indicates that this format can be sampled by the displacement map sampler
#define D3DFORMAT_OP_DMAP                       0x00020000L

// Indicates that this format cannot be used with texture filtering
#define D3DFORMAT_OP_NOFILTER                   0x00040000L

// Indicates that format conversions are supported to this RGB format if
// D3DFORMAT_OP_CONVERT_TO_ARGB is specified in the source format.
#define D3DFORMAT_MEMBEROFGROUP_ARGB            0x00080000L

// Indicated that this format can be written as an SRGB target (meaning that the
// pixel pipe will DE-linearize data on output to format)
#define D3DFORMAT_OP_SRGBWRITE                  0x00100000L

// Indicates that this format cannot be used with alpha blending
#define D3DFORMAT_OP_NOALPHABLEND               0x00200000L

//Indicates that the device can auto-generated sublevels for resources
//of this format
#define D3DFORMAT_OP_AUTOGENMIPMAP              0x00400000L

// Indicates that this format cannot be used by vertex texture sampler
#define D3DFORMAT_OP_VERTEXTEXTURE              0x00800000L 

// Indicates that this format supports neither texture coordinate wrap modes, nor mipmapping
#define D3DFORMAT_OP_NOTEXCOORDWRAPNORMIP		0x01000000L

/*
 * pre-declare pointers to structs containing data for DDHAL driver fns
 */
typedef struct _DD_CREATEPALETTEDATA *PDD_CREATEPALETTEDATA;
typedef struct _DD_CREATESURFACEDATA *PDD_CREATESURFACEDATA;
typedef struct _DD_CANCREATESURFACEDATA *PDD_CANCREATESURFACEDATA;
typedef struct _DD_WAITFORVERTICALBLANKDATA *PDD_WAITFORVERTICALBLANKDATA;
typedef struct _DD_DESTROYDRIVERDATA *PDD_DESTROYDRIVERDATA;
typedef struct _DD_SETMODEDATA *PDD_SETMODEDATA;
typedef struct _DD_DRVSETCOLORKEYDATA *PDD_DRVSETCOLORKEYDATA;
typedef struct _DD_GETSCANLINEDATA *PDD_GETSCANLINEDATA;
typedef struct _DD_MAPMEMORYDATA *PDD_MAPMEMORYDATA;

typedef struct _DD_DESTROYPALETTEDATA *PDD_DESTROYPALETTEDATA;
typedef struct _DD_SETENTRIESDATA *PDD_SETENTRIESDATA;

typedef struct _DD_BLTDATA *PDD_BLTDATA;
typedef struct _DD_LOCKDATA *PDD_LOCKDATA;
typedef struct _DD_UNLOCKDATA *PDD_UNLOCKDATA;
typedef struct _DD_UPDATEOVERLAYDATA *PDD_UPDATEOVERLAYDATA;
typedef struct _DD_SETOVERLAYPOSITIONDATA *PDD_SETOVERLAYPOSITIONDATA;
typedef struct _DD_SETPALETTEDATA *PDD_SETPALETTEDATA;
typedef struct _DD_FLIPDATA *PDD_FLIPDATA;
typedef struct _DD_DESTROYSURFACEDATA *PDD_DESTROYSURFACEDATA;
typedef struct _DD_SETCLIPLISTDATA *PDD_SETCLIPLISTDATA;
typedef struct _DD_ADDATTACHEDSURFACEDATA *PDD_ADDATTACHEDSURFACEDATA;
typedef struct _DD_SETCOLORKEYDATA *PDD_SETCOLORKEYDATA;
typedef struct _DD_GETBLTSTATUSDATA *PDD_GETBLTSTATUSDATA;
typedef struct _DD_GETFLIPSTATUSDATA *PDD_GETFLIPSTATUSDATA;

typedef struct _DD_CANCREATEVPORTDATA *PDD_CANCREATEVPORTDATA;
typedef struct _DD_CREATEVPORTDATA *PDD_CREATEVPORTDATA;
typedef struct _DD_FLIPVPORTDATA *PDD_FLIPVPORTDATA;
typedef struct _DD_GETVPORTCONNECTDATA *PDD_GETVPORTCONNECTDATA;
typedef struct _DD_GETVPORTBANDWIDTHDATA *PDD_GETVPORTBANDWIDTHDATA;
typedef struct _DD_GETVPORTINPUTFORMATDATA *PDD_GETVPORTINPUTFORMATDATA;
typedef struct _DD_GETVPORTOUTPUTFORMATDATA *PDD_GETVPORTOUTPUTFORMATDATA;
typedef struct _DD_GETVPORTAUTOFLIPSURFACEDATA *PDD_GETVPORTAUTOFLIPSURFACEDATA;
typedef struct _DD_GETVPORTFIELDDATA *PDD_GETVPORTFIELDDATA;
typedef struct _DD_GETVPORTLINEDATA *PDD_GETVPORTLINEDATA;
typedef struct _DD_DESTROYVPORTDATA *PDD_DESTROYVPORTDATA;
typedef struct _DD_GETVPORTFLIPSTATUSDATA *PDD_GETVPORTFLIPSTATUSDATA;
typedef struct _DD_UPDATEVPORTDATA *PDD_UPDATEVPORTDATA;
typedef struct _DD_WAITFORVPORTSYNCDATA *PDD_WAITFORVPORTSYNCDATA;
typedef struct _DD_GETVPORTSIGNALDATA *PDD_GETVPORTSIGNALDATA;
typedef struct _DD_VPORTCOLORDATA *PDD_VPORTCOLORDATA;

typedef struct _DD_COLORCONTROLDATA *PDD_COLORCONTROLDATA;

typedef struct _DD_GETAVAILDRIVERMEMORYDATA *PDD_GETAVAILDRIVERMEMORYDATA;

typedef struct _DD_FREEDRIVERMEMORYDATA *PDD_FREEDRIVERMEMORYDATA;
typedef struct _DD_SETEXCLUSIVEMODEDATA *PDD_SETEXCLUSIVEMODEDATA;
typedef struct _DD_FLIPTOGDISURFACEDATA *PDD_FLIPTOGDISURFACEDATA;

typedef struct _DD_GETDRIVERINFODATA *PDD_GETDRIVERINFODATA;

typedef struct _DD_SYNCSURFACEDATA *PDD_SYNCSURFACEDATA;
typedef struct _DD_SYNCVIDEOPORTDATA *PDD_SYNCVIDEOPORTDATA;

typedef struct _DD_GETMOCOMPGUIDSDATA *PDD_GETMOCOMPGUIDSDATA;
typedef struct _DD_GETMOCOMPFORMATSDATA *PDD_GETMOCOMPFORMATSDATA;
typedef struct _DD_CREATEMOCOMPDATA *PDD_CREATEMOCOMPDATA;
typedef struct _DD_GETMOCOMPCOMPBUFFDATA *PDD_GETMOCOMPCOMPBUFFDATA;
typedef struct _DD_GETINTERNALMOCOMPDATA *PDD_GETINTERNALMOCOMPDATA;
typedef struct _DD_BEGINMOCOMPFRAMEDATA *PDD_BEGINMOCOMPFRAMEDATA;
typedef struct _DD_ENDMOCOMPFRAMEDATA *PDD_ENDMOCOMPFRAMEDATA;
typedef struct _DD_RENDERMOCOMPDATA *PDD_RENDERMOCOMPDATA;
typedef struct _DD_QUERYMOCOMPSTATUSDATA *PDD_QUERYMOCOMPSTATUSDATA;
typedef struct _DD_DESTROYMOCOMPDATA *PDD_DESTROYMOCOMPDATA;

// Miscelleneous2 callbacks
typedef struct _DD_CREATESURFACEEXDATA *PDD_CREATESURFACEEXDATA;
typedef struct _DD_GETDRIVERSTATEDATA *PDD_GETDRIVERSTATEDATA;
typedef struct _DD_DESTROYDDLOCALDATA *PDD_DESTROYDDLOCALDATA;
typedef struct _DD_MORESURFACECAPS *PDD_MORESURFACECAPS;
typedef struct _DD_STEREOMODE *PDD_STEREOMODE;
typedef struct _DD_UPDATENONLOCALHEAPDATA *PDD_UPDATENONLOCALHEAPDATA;



/*
 * The following structure is defined in dmemmgr.h
 */
struct _DD_GETHEAPALIGNMENTDATA;
typedef struct _DD_GETHEAPALIGNMENTDATA *PDD_GETHEAPALIGNMENTDATA;

/*
 * value in the fpVidMem; indicates dwBlockSize is valid (surface object)
 */
#define DDHAL_PLEASEALLOC_BLOCKSIZE     0x00000002l
#define DDHAL_PLEASEALLOC_USERMEM       0x00000004l

/*
 * video memory data structures (passed in DD_HALINFO)
 */
typedef struct _VIDEOMEMORY
{
    DWORD               dwFlags;        // flags
    FLATPTR             fpStart;        // start of memory chunk
    union
    {
        FLATPTR         fpEnd;          // end of memory chunk
        DWORD           dwWidth;        // width of chunk (rectanglar memory)
    };
    DDSCAPS             ddsCaps;        // what this memory CANNOT be used for
    DDSCAPS             ddsCapsAlt;     // what this memory CANNOT be used for if it must
    union
    {
        struct _VMEMHEAP *lpHeap;       // heap pointer, used by DDRAW
        DWORD           dwHeight;       // height of chunk (rectanguler memory)
    };
} VIDEOMEMORY;
typedef VIDEOMEMORY *LPVIDEOMEMORY;

/*
 * flags for vidmem struct
 */
#define VIDMEM_ISLINEAR         0x00000001l     // heap is linear
#define VIDMEM_ISRECTANGULAR    0x00000002l     // heap is rectangular
#define VIDMEM_ISHEAP           0x00000004l     // heap is preallocated by driver
#define VIDMEM_ISNONLOCAL       0x00000008l     // heap populated with non-local video memory
#define VIDMEM_ISWC             0x00000010l     // heap populated with write combining memory
#define VIDMEM_HEAPDISABLED     0x00000020l     // heap disabled

typedef struct _VIDEOMEMORYINFO
{
    FLATPTR             fpPrimary;              // offset to primary surface
    DWORD               dwFlags;                // flags
    DWORD               dwDisplayWidth;         // current display width
    DWORD               dwDisplayHeight;        // current display height
    LONG                lDisplayPitch;          // current display pitch
    DDPIXELFORMAT       ddpfDisplay;            // pixel format of display
    DWORD               dwOffscreenAlign;       // byte alignment for offscreen surfaces
    DWORD               dwOverlayAlign;         // byte alignment for overlays
    DWORD               dwTextureAlign;         // byte alignment for textures
    DWORD               dwZBufferAlign;         // byte alignment for z buffers
    DWORD               dwAlphaAlign;           // byte alignment for alpha
    PVOID               pvPrimary;              // kernel-mode pointer to primary surface
} VIDEOMEMORYINFO;
typedef VIDEOMEMORYINFO *LPVIDEOMEMORYINFO;

/*
 * These structures contain the entry points in the display driver that
 * DDRAW will call.   Entries that the display driver does not care about
 * should be NULL.   Passed to DDRAW in DD_HALINFO.
 */
typedef struct _DD_DIRECTDRAW_GLOBAL *PDD_DIRECTDRAW_GLOBAL;
typedef struct _DD_SURFACE_GLOBAL *PDD_SURFACE_GLOBAL;
typedef struct _DD_PALETTE_GLOBAL *PDD_PALETTE_GLOBAL;
typedef struct _DD_CLIPPER_GLOBAL *PDD_CLIPPER_GLOBAL;
typedef struct _DD_DIRECTDRAW_LOCAL *PDD_DIRECTDRAW_LOCAL;
typedef struct _DD_SURFACE_LOCAL *PDD_SURFACE_LOCAL;
typedef struct _DD_SURFACE_MORE *PDD_SURFACE_MORE;
typedef struct _DD_SURFACE_INT *PDD_SURFACE_INT;
typedef struct _DD_VIDEOPORT_LOCAL *PDD_VIDEOPORT_LOCAL;
typedef struct _DD_PALETTE_LOCAL *PDD_PALETTE_LOCAL;
typedef struct _DD_CLIPPER_LOCAL *PDD_CLIPPER_LOCAL;
typedef struct _DD_MOTIONCOMP_LOCAL *PDD_MOTIONCOMP_LOCAL;

/*
 * DIRECTDRAW object callbacks
 */
typedef DWORD   (APIENTRY *PDD_SETCOLORKEY)(PDD_DRVSETCOLORKEYDATA );
typedef DWORD   (APIENTRY *PDD_CANCREATESURFACE)(PDD_CANCREATESURFACEDATA );
typedef DWORD   (APIENTRY *PDD_WAITFORVERTICALBLANK)(PDD_WAITFORVERTICALBLANKDATA );
typedef DWORD   (APIENTRY *PDD_CREATESURFACE)(PDD_CREATESURFACEDATA);
typedef DWORD   (APIENTRY *PDD_DESTROYDRIVER)(PDD_DESTROYDRIVERDATA);
typedef DWORD   (APIENTRY *PDD_SETMODE)(PDD_SETMODEDATA);
typedef DWORD   (APIENTRY *PDD_CREATEPALETTE)(PDD_CREATEPALETTEDATA);
typedef DWORD   (APIENTRY *PDD_GETSCANLINE)(PDD_GETSCANLINEDATA);
typedef DWORD   (APIENTRY *PDD_MAPMEMORY)(PDD_MAPMEMORYDATA);

typedef DWORD   (APIENTRY *PDD_GETDRIVERINFO)(PDD_GETDRIVERINFODATA);

typedef struct DD_CALLBACKS
{
    DWORD                       dwSize;
    DWORD                       dwFlags;
    PDD_DESTROYDRIVER           DestroyDriver;
    PDD_CREATESURFACE           CreateSurface;
    PDD_SETCOLORKEY             SetColorKey;
    PDD_SETMODE                 SetMode;
    PDD_WAITFORVERTICALBLANK    WaitForVerticalBlank;
    PDD_CANCREATESURFACE        CanCreateSurface;
    PDD_CREATEPALETTE           CreatePalette;
    PDD_GETSCANLINE             GetScanLine;
    PDD_MAPMEMORY               MapMemory;
} DD_CALLBACKS;

typedef DD_CALLBACKS *PDD_CALLBACKS;

#define DDHAL_CB32_DESTROYDRIVER        0x00000001l
#define DDHAL_CB32_CREATESURFACE        0x00000002l
#define DDHAL_CB32_SETCOLORKEY          0x00000004l
#define DDHAL_CB32_SETMODE              0x00000008l
#define DDHAL_CB32_WAITFORVERTICALBLANK 0x00000010l
#define DDHAL_CB32_CANCREATESURFACE     0x00000020l
#define DDHAL_CB32_CREATEPALETTE        0x00000040l
#define DDHAL_CB32_GETSCANLINE          0x00000080l
#define DDHAL_CB32_MAPMEMORY            0x80000000l

// This structure can be queried from the driver from NT5 onward
// using GetDriverInfo with GUID_MiscellaneousCallbacks

typedef DWORD   (APIENTRY *PDD_GETAVAILDRIVERMEMORY)(PDD_GETAVAILDRIVERMEMORYDATA);

typedef struct _DD_MISCELLANEOUSCALLBACKS {
    DWORD                               dwSize;
    DWORD                               dwFlags;
    PDD_GETAVAILDRIVERMEMORY            GetAvailDriverMemory;
} DD_MISCELLANEOUSCALLBACKS, *PDD_MISCELLANEOUSCALLBACKS;

#define DDHAL_MISCCB32_GETAVAILDRIVERMEMORY    0x00000001l

// DDHAL_DDMISCELLANEOUS2CALLBACKS:
//   This structure can be queried from the driver from DX7 onward
//   using GetDriverInfo with GUID_Miscellaneous2Callbacks

typedef DWORD   (APIENTRY *PDD_ALPHABLT)(PDD_BLTDATA);
typedef DWORD   (APIENTRY *PDD_CREATESURFACEEX)(PDD_CREATESURFACEEXDATA);
typedef DWORD   (APIENTRY *PDD_GETDRIVERSTATE)(PDD_GETDRIVERSTATEDATA);
typedef DWORD   (APIENTRY *PDD_DESTROYDDLOCAL)(PDD_DESTROYDDLOCALDATA);

typedef struct _DD_MISCELLANEOUS2CALLBACKS {
    DWORD                               dwSize;
    DWORD                               dwFlags;
    PDD_ALPHABLT                        AlphaBlt;
    PDD_CREATESURFACEEX                 CreateSurfaceEx;
    PDD_GETDRIVERSTATE                  GetDriverState;
    PDD_DESTROYDDLOCAL                  DestroyDDLocal;
} DD_MISCELLANEOUS2CALLBACKS, *PDD_MISCELLANEOUS2CALLBACKS;

#define DDHAL_MISC2CB32_ALPHABLT                 0x00000001l
#define DDHAL_MISC2CB32_CREATESURFACEEX          0x00000002l
#define DDHAL_MISC2CB32_GETDRIVERSTATE           0x00000004l
#define DDHAL_MISC2CB32_DESTROYDDLOCAL           0x00000008l

// This is used in the CreateSurfaceEx callback to indicate that the
// SwapHandle emulation is being done
#define DDHAL_CREATESURFACEEX_SWAPHANDLES      0x00000001l

// This structure can be queried from the driver from NT5 onward
// using GetDriverInfo with GUID_NTCallbacks

typedef DWORD   (APIENTRY *PDD_FREEDRIVERMEMORY)(PDD_FREEDRIVERMEMORYDATA);
typedef DWORD   (APIENTRY *PDD_SETEXCLUSIVEMODE)(PDD_SETEXCLUSIVEMODEDATA);
typedef DWORD   (APIENTRY *PDD_FLIPTOGDISURFACE)(PDD_FLIPTOGDISURFACEDATA);

typedef struct _DD_NTCALLBACKS {
    DWORD                   dwSize;
    DWORD                   dwFlags;
    PDD_FREEDRIVERMEMORY    FreeDriverMemory;
    PDD_SETEXCLUSIVEMODE    SetExclusiveMode;
    PDD_FLIPTOGDISURFACE    FlipToGDISurface;
} DD_NTCALLBACKS, *PDD_NTCALLBACKS;

#define DDHAL_NTCB32_FREEDRIVERMEMORY   0x00000001l
#define DDHAL_NTCB32_SETEXCLUSIVEMODE   0x00000002l
#define DDHAL_NTCB32_FLIPTOGDISURFACE   0x00000004l

/*
 * DIRECTDRAWPALETTE object callbacks
 */
typedef DWORD   (APIENTRY *PDD_PALCB_DESTROYPALETTE)(PDD_DESTROYPALETTEDATA );
typedef DWORD   (APIENTRY *PDD_PALCB_SETENTRIES)(PDD_SETENTRIESDATA );

typedef struct DD_PALETTECALLBACKS
{
    DWORD                       dwSize;
    DWORD                       dwFlags;
    PDD_PALCB_DESTROYPALETTE    DestroyPalette;
    PDD_PALCB_SETENTRIES        SetEntries;
} DD_PALETTECALLBACKS;

typedef DD_PALETTECALLBACKS *PDD_PALETTECALLBACKS;

#define DDHAL_PALCB32_DESTROYPALETTE    0x00000001l
#define DDHAL_PALCB32_SETENTRIES        0x00000002l

/*
 * DIRECTDRAWSURFACE object callbacks
 */
typedef DWORD   (APIENTRY *PDD_SURFCB_LOCK)(PDD_LOCKDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_UNLOCK)(PDD_UNLOCKDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_BLT)(PDD_BLTDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_UPDATEOVERLAY)(PDD_UPDATEOVERLAYDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_SETOVERLAYPOSITION)(PDD_SETOVERLAYPOSITIONDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_SETPALETTE)(PDD_SETPALETTEDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_FLIP)(PDD_FLIPDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_DESTROYSURFACE)(PDD_DESTROYSURFACEDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_SETCLIPLIST)(PDD_SETCLIPLISTDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_ADDATTACHEDSURFACE)(PDD_ADDATTACHEDSURFACEDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_SETCOLORKEY)(PDD_SETCOLORKEYDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_GETBLTSTATUS)(PDD_GETBLTSTATUSDATA);
typedef DWORD   (APIENTRY *PDD_SURFCB_GETFLIPSTATUS)(PDD_GETFLIPSTATUSDATA);


typedef struct DD_SURFACECALLBACKS
{
    DWORD                               dwSize;
    DWORD                               dwFlags;
    PDD_SURFCB_DESTROYSURFACE           DestroySurface;
    PDD_SURFCB_FLIP                     Flip;
    PDD_SURFCB_SETCLIPLIST              SetClipList;
    PDD_SURFCB_LOCK                     Lock;
    PDD_SURFCB_UNLOCK                   Unlock;
    PDD_SURFCB_BLT                      Blt;
    PDD_SURFCB_SETCOLORKEY              SetColorKey;
    PDD_SURFCB_ADDATTACHEDSURFACE       AddAttachedSurface;
    PDD_SURFCB_GETBLTSTATUS             GetBltStatus;
    PDD_SURFCB_GETFLIPSTATUS            GetFlipStatus;
    PDD_SURFCB_UPDATEOVERLAY            UpdateOverlay;
    PDD_SURFCB_SETOVERLAYPOSITION       SetOverlayPosition;
    LPVOID                              reserved4;
    PDD_SURFCB_SETPALETTE               SetPalette;
} DD_SURFACECALLBACKS;
typedef DD_SURFACECALLBACKS *PDD_SURFACECALLBACKS;

#define DDHAL_SURFCB32_DESTROYSURFACE           0x00000001l
#define DDHAL_SURFCB32_FLIP                     0x00000002l
#define DDHAL_SURFCB32_SETCLIPLIST              0x00000004l
#define DDHAL_SURFCB32_LOCK                     0x00000008l
#define DDHAL_SURFCB32_UNLOCK                   0x00000010l
#define DDHAL_SURFCB32_BLT                      0x00000020l
#define DDHAL_SURFCB32_SETCOLORKEY              0x00000040l
#define DDHAL_SURFCB32_ADDATTACHEDSURFACE       0x00000080l
#define DDHAL_SURFCB32_GETBLTSTATUS             0x00000100l
#define DDHAL_SURFCB32_GETFLIPSTATUS            0x00000200l
#define DDHAL_SURFCB32_UPDATEOVERLAY            0x00000400l
#define DDHAL_SURFCB32_SETOVERLAYPOSITION       0x00000800l
#define DDHAL_SURFCB32_RESERVED4                0x00001000l
#define DDHAL_SURFCB32_SETPALETTE               0x00002000l

/*
 * DIRECTVIDEOPORT object callbacks
 */
typedef DWORD (APIENTRY *PDD_VPORTCB_CANCREATEVIDEOPORT)(PDD_CANCREATEVPORTDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_CREATEVIDEOPORT)(PDD_CREATEVPORTDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_FLIP)(PDD_FLIPVPORTDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETBANDWIDTH)(PDD_GETVPORTBANDWIDTHDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETINPUTFORMATS)(PDD_GETVPORTINPUTFORMATDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETOUTPUTFORMATS)(PDD_GETVPORTOUTPUTFORMATDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETAUTOFLIPSURF)(PDD_GETVPORTAUTOFLIPSURFACEDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETFIELD)(PDD_GETVPORTFIELDDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETLINE)(PDD_GETVPORTLINEDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETVPORTCONNECT)(PDD_GETVPORTCONNECTDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_DESTROYVPORT)(PDD_DESTROYVPORTDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETFLIPSTATUS)(PDD_GETVPORTFLIPSTATUSDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_UPDATE)(PDD_UPDATEVPORTDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_WAITFORSYNC)(PDD_WAITFORVPORTSYNCDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_GETSIGNALSTATUS)(PDD_GETVPORTSIGNALDATA);
typedef DWORD (APIENTRY *PDD_VPORTCB_COLORCONTROL)(PDD_VPORTCOLORDATA);

typedef struct DD_VIDEOPORTCALLBACKS
{
    DWORD                               dwSize;
    DWORD                               dwFlags;
    PDD_VPORTCB_CANCREATEVIDEOPORT      CanCreateVideoPort;
    PDD_VPORTCB_CREATEVIDEOPORT         CreateVideoPort;
    PDD_VPORTCB_FLIP                    FlipVideoPort;
    PDD_VPORTCB_GETBANDWIDTH            GetVideoPortBandwidth;
    PDD_VPORTCB_GETINPUTFORMATS         GetVideoPortInputFormats;
    PDD_VPORTCB_GETOUTPUTFORMATS        GetVideoPortOutputFormats;
    LPVOID                              lpReserved1;
    PDD_VPORTCB_GETFIELD                GetVideoPortField;
    PDD_VPORTCB_GETLINE                 GetVideoPortLine;
    PDD_VPORTCB_GETVPORTCONNECT         GetVideoPortConnectInfo;
    PDD_VPORTCB_DESTROYVPORT            DestroyVideoPort;
    PDD_VPORTCB_GETFLIPSTATUS           GetVideoPortFlipStatus;
    PDD_VPORTCB_UPDATE                  UpdateVideoPort;
    PDD_VPORTCB_WAITFORSYNC             WaitForVideoPortSync;
    PDD_VPORTCB_GETSIGNALSTATUS         GetVideoSignalStatus;
    PDD_VPORTCB_COLORCONTROL            ColorControl;
} DD_VIDEOPORTCALLBACKS;

typedef DD_VIDEOPORTCALLBACKS *PDD_VIDEOPORTCALLBACKS;

#define DDHAL_VPORT32_CANCREATEVIDEOPORT        0x00000001l
#define DDHAL_VPORT32_CREATEVIDEOPORT           0x00000002l
#define DDHAL_VPORT32_FLIP                      0x00000004l
#define DDHAL_VPORT32_GETBANDWIDTH              0x00000008l
#define DDHAL_VPORT32_GETINPUTFORMATS           0x00000010l
#define DDHAL_VPORT32_GETOUTPUTFORMATS          0x00000020l
#define DDHAL_VPORT32_GETAUTOFLIPSURF           0x00000040l
#define DDHAL_VPORT32_GETFIELD                  0x00000080l
#define DDHAL_VPORT32_GETLINE                   0x00000100l
#define DDHAL_VPORT32_GETCONNECT                0x00000200l
#define DDHAL_VPORT32_DESTROY                   0x00000400l
#define DDHAL_VPORT32_GETFLIPSTATUS             0x00000800l
#define DDHAL_VPORT32_UPDATE                    0x00001000l
#define DDHAL_VPORT32_WAITFORSYNC               0x00002000l
#define DDHAL_VPORT32_GETSIGNALSTATUS           0x00004000l
#define DDHAL_VPORT32_COLORCONTROL              0x00008000l

/*
 * DIRECTDRAWCOLORCONTROL object callbacks
 */
typedef DWORD (APIENTRY *PDD_COLORCB_COLORCONTROL)(PDD_COLORCONTROLDATA);

typedef struct _DD_COLORCONTROLCALLBACKS
{
    DWORD                               dwSize;
    DWORD                               dwFlags;
    PDD_COLORCB_COLORCONTROL            ColorControl;
} DD_COLORCONTROLCALLBACKS;

typedef DD_COLORCONTROLCALLBACKS *PDD_COLORCONTROLCALLBACKS;

#define DDHAL_COLOR_COLORCONTROL                0x00000001l

/*
 * DIRECTDRAWSURFACEKERNEL object callbacks
 * This structure can be queried from the driver from DX5 onward
 * using GetDriverInfo with GUID_KernelCallbacks
 */
typedef DWORD (APIENTRY *PDD_KERNELCB_SYNCSURFACE)(PDD_SYNCSURFACEDATA);
typedef DWORD (APIENTRY *PDD_KERNELCB_SYNCVIDEOPORT)(PDD_SYNCVIDEOPORTDATA);

typedef struct DD_KERNELCALLBACKS
{
    DWORD                               dwSize;
    DWORD                               dwFlags;
    PDD_KERNELCB_SYNCSURFACE            SyncSurfaceData;
    PDD_KERNELCB_SYNCVIDEOPORT          SyncVideoPortData;
} DD_KERNELCALLBACKS, *PDD_KERNELCALLBACKS;

#define DDHAL_KERNEL_SYNCSURFACEDATA            0x00000001l
#define DDHAL_KERNEL_SYNCVIDEOPORTDATA          0x00000002l

/*
 * DIRECTDRAWVIDEO object callbacks
 */
typedef DWORD (APIENTRY *PDD_MOCOMPCB_GETGUIDS)( PDD_GETMOCOMPGUIDSDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_GETFORMATS)( PDD_GETMOCOMPFORMATSDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_CREATE)( PDD_CREATEMOCOMPDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_GETCOMPBUFFINFO)( PDD_GETMOCOMPCOMPBUFFDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_GETINTERNALINFO)( PDD_GETINTERNALMOCOMPDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_BEGINFRAME)( PDD_BEGINMOCOMPFRAMEDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_ENDFRAME)( PDD_ENDMOCOMPFRAMEDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_RENDER)( PDD_RENDERMOCOMPDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_QUERYSTATUS)( PDD_QUERYMOCOMPSTATUSDATA);
typedef DWORD (APIENTRY *PDD_MOCOMPCB_DESTROY)( PDD_DESTROYMOCOMPDATA);

typedef struct DD_MOTIONCOMPCALLBACKS
{
    DWORD                           dwSize;
    DWORD                           dwFlags;
    PDD_MOCOMPCB_GETGUIDS           GetMoCompGuids;
    PDD_MOCOMPCB_GETFORMATS         GetMoCompFormats;
    PDD_MOCOMPCB_CREATE             CreateMoComp;
    PDD_MOCOMPCB_GETCOMPBUFFINFO    GetMoCompBuffInfo;
    PDD_MOCOMPCB_GETINTERNALINFO    GetInternalMoCompInfo;
    PDD_MOCOMPCB_BEGINFRAME         BeginMoCompFrame;
    PDD_MOCOMPCB_ENDFRAME           EndMoCompFrame;
    PDD_MOCOMPCB_RENDER             RenderMoComp;
    PDD_MOCOMPCB_QUERYSTATUS        QueryMoCompStatus;
    PDD_MOCOMPCB_DESTROY            DestroyMoComp;
} DD_MOTIONCOMPCALLBACKS;
typedef DD_MOTIONCOMPCALLBACKS *PDD_MOTIONCOMPCALLBACKS;

#define DDHAL_MOCOMP32_GETGUIDS                 0x00000001
#define DDHAL_MOCOMP32_GETFORMATS               0x00000002
#define DDHAL_MOCOMP32_CREATE                   0x00000004
#define DDHAL_MOCOMP32_GETCOMPBUFFINFO          0x00000008
#define DDHAL_MOCOMP32_GETINTERNALINFO          0x00000010
#define DDHAL_MOCOMP32_BEGINFRAME               0x00000020
#define DDHAL_MOCOMP32_ENDFRAME                 0x00000040
#define DDHAL_MOCOMP32_RENDER                   0x00000080
#define DDHAL_MOCOMP32_QUERYSTATUS              0x00000100
#define DDHAL_MOCOMP32_DESTROY                  0x00000200

/*
 * CALLBACK RETURN VALUES
 *
 * these are values returned by the driver from the above callback routines
 */
/*
 * indicates that the display driver didn't do anything with the call
 */
#define DDHAL_DRIVER_NOTHANDLED         0x00000000l

/*
 * indicates that the display driver handled the call; HRESULT value is valid
 */
#define DDHAL_DRIVER_HANDLED            0x00000001l

/*
 * indicates that the display driver couldn't handle the call because it
 * ran out of color key hardware resources
 */
#define DDHAL_DRIVER_NOCKEYHW           0x00000002l

/*
 * Capabilities structure for non-local video memory
 */
typedef struct _DD_NONLOCALVIDMEMCAPS
{
    DWORD   dwSize;
    DWORD   dwNLVBCaps;           // driver specific capabilities for non-local->local vidmem blts
    DWORD   dwNLVBCaps2;          // more driver specific capabilities non-local->local vidmem blts
    DWORD   dwNLVBCKeyCaps;       // driver color key capabilities for non-local->local vidmem blts
    DWORD   dwNLVBFXCaps;         // driver FX capabilities for non-local->local blts
    DWORD   dwNLVBRops[DD_ROP_SPACE]; // ROPS supported for non-local->local blts
} DD_NONLOCALVIDMEMCAPS;
typedef struct _DD_NONLOCALVIDMEMCAPS *PDD_NONLOCALVIDMEMCAPS;

/*
 * DDRAW internal version of DIRECTDRAWPALETTE object; it has data after the vtable
 */
typedef struct _DD_PALETTE_GLOBAL
{
    ULONG_PTR                   dwReserved1;    // reserved for use by display driver
} DD_PALETTE_GLOBAL;

typedef struct _DD_PALETTE_LOCAL
{
    ULONG                       dwReserved0;    // reserved for future expansion
    ULONG_PTR                   dwReserved1;    // reserved for use by display driver
} DD_PALETTE_LOCAL;

/*
 * DDRAW internal version of DIRECTDRAWCLIPPER object; it has data after the vtable
 */
typedef struct _DD_CLIPPER_GLOBAL
{
    ULONG_PTR                   dwReserved1;    // reserved for use by display driver
} DD_CLIPPER_GLOBAL;

typedef struct _DD_CLIPPER_LOCAL
{
    ULONG_PTR                   dwReserved1;    // reserved for use by display driver
} DD_CLIPPER_LOCAL;

typedef struct _DD_ATTACHLIST *PDD_ATTACHLIST;
typedef struct _DD_ATTACHLIST
{
    PDD_ATTACHLIST              lpLink;         // link to next attached surface
    PDD_SURFACE_LOCAL           lpAttached;     // attached surface local object
} DD_ATTACHLIST;

/*
 * DDRAW surface interface struct
 */
typedef struct _DD_SURFACE_INT
{
    PDD_SURFACE_LOCAL           lpLcl;          // pointer to interface data
} DD_SURFACE_INT;

/*
 * DDRAW internal version of DIRECTDRAWSURFACE struct
 *
 * the GBL structure is global data for all duplicate objects
 */
typedef struct _DD_SURFACE_GLOBAL
{
    union 
    {
        DWORD                   dwBlockSizeY;   // block size that display driver requested (return)
        LONG                    lSlicePitch;    // slice pitch for volume textures
    };

    union 
    {
        LPVIDEOMEMORY           lpVidMemHeap;   // heap vidmem was alloc'ed from
        DWORD                   dwBlockSizeX;   // block size that display driver requested (return)
        DWORD                   dwUserMemSize;  // user-mode memory size that display driver requested (return)
    };

    FLATPTR                     fpVidMem;       // pointer to video memory
    union
    {
        LONG                    lPitch;         // pitch of surface
        DWORD                   dwLinearSize;   // linear size of non-rectangular surface
    };
    LONG                        yHint;          // y-coordinate of surface
    LONG                        xHint;          // x-coordinate of surface
    DWORD                       wHeight;        // height of surface
    DWORD                       wWidth;         // width of surface
    ULONG_PTR                   dwReserved1;    // reserved for use by display driver
    DDPIXELFORMAT               ddpfSurface;    // pixel format of surface
    FLATPTR                     fpHeapOffset;   // raw offset in source heap
    HANDLE                      hCreatorProcess;// opaque identifier for creating process
} DD_SURFACE_GLOBAL;

/*
 * a structure holding additional LCL surface information (to maintain some
 * compatibility with Win95).
 */
typedef struct _DD_SURFACE_MORE
{
    DWORD                       dwMipMapCount;      // number of mip-map levels
    PDD_VIDEOPORT_LOCAL         lpVideoPort;        // video port currently writing data to this surface
    DWORD                       dwOverlayFlags;     // current overlay flags
    DDSCAPSEX                   ddsCapsEx;          // more surface capabilities
    DWORD                       dwSurfaceHandle;    // cookie for use with CreateSurfaceEx DDI
} DD_SURFACE_MORE, *PDD_SURFACE_MORE;

/*
 * the LCL structure is local data for each individual surface object
 */
typedef struct _DD_SURFACE_LOCAL
{
    PDD_SURFACE_GLOBAL          lpGbl;            // pointer to surface shared data
    DWORD                       dwFlags;          // flags
    DDSCAPS                     ddsCaps;          // capabilities of surface
    ULONG_PTR                   dwReserved1;      // reserved for use by display driver
    union
    {
        DDCOLORKEY              ddckCKSrcOverlay; // color key for source overlay use
        DDCOLORKEY              ddckCKSrcBlt;     // color key for source blt and texture use
    };
    union
    {
        DDCOLORKEY              ddckCKDestOverlay;// color key for destination overlay use
        DDCOLORKEY              ddckCKDestBlt;    // color key for destination blt
    };
    PDD_SURFACE_MORE            lpSurfMore;       // pointer to additional local data
    PDD_ATTACHLIST              lpAttachList;     // link to surfaces we attached to
    PDD_ATTACHLIST              lpAttachListFrom; // link to surfaces that attached to us
    RECT                        rcOverlaySrc;     // Overlay source rectangle relative to surface
} DD_SURFACE_LOCAL;

#define DDRAWISURF_HASCKEYSRCBLT        0x00000800L     // surface has CKSrcBlt
#define DDRAWISURF_HASPIXELFORMAT       0x00002000L     // surface structure has pixel format data
#define DDRAWISURF_HASOVERLAYDATA       0x00004000L     // surface structure has overlay data
#define DDRAWISURF_FRONTBUFFER          0x04000000L     // surface was originally a front buffer
#define DDRAWISURF_BACKBUFFER           0x08000000L     // surface was originally backbuffer
#define DDRAWISURF_INVALID              0x10000000L     // surface has been invalidated by mode set
#define DDRAWISURF_DRIVERMANAGED        0x40000000L     // surface is a driver managed texture (D3D)

/*
 * More driver capabilities (in addition to those described in DDCORECAPS).
 * This struct contains the caps bits added to the DDCAPS structure in DX6.
 */
typedef struct _DD_MORECAPS
{
    DWORD   dwSize; 		    // size of DDMORECAPS structure
    DWORD   dwAlphaCaps;	    // driver-specific alpha caps for overlays & Vmem->Vmem blts
    DWORD   dwSVBAlphaCaps;	    // driver-specific alpha capabilities for System->Vmem blts
    DWORD   dwVSBAlphaCaps;	    // driver-specific alpha capabilities for Vmem->System blts
    DWORD   dwSSBAlphaCaps;	    // driver-specific alpha capabilities for System->System blts
    DWORD   dwFilterCaps;           // driver-specific filter caps for overlays & Vmem->Vmem blts
    DWORD   dwSVBFilterCaps;        // driver-specific filter capabilities for System->Vmem blts
    DWORD   dwVSBFilterCaps;        // driver-specific filter capabilities for Vmem->System blts
    DWORD   dwSSBFilterCaps;        // driver-specific filter capabilities for System->System blts
} DD_MORECAPS;

typedef DD_MORECAPS *PDD_MORECAPS;

/*
 * rop stuff
 */
#define ROP_HAS_SOURCE          0x00000001l
#define ROP_HAS_PATTERN         0x00000002l
#define ROP_HAS_SOURCEPATTERN   ROP_HAS_SOURCE | ROP_HAS_PATTERN

/*
 * This structure mirrors the first entries of the DDCAPS but is of a fixed
 * size and will not grow as DDCAPS grows. This is the structure your driver
 * returns in DDCOREINFO. Additional caps will be requested via a GetDriverInfo
 * call.
 */
typedef struct _DDNTCORECAPS
{
    DWORD       dwSize;                 // size of the DDDRIVERCAPS structure
    DWORD       dwCaps;                 // driver specific capabilities
    DWORD       dwCaps2;                // more driver specific capabilites
    DWORD       dwCKeyCaps;             // color key capabilities of the surface
    DWORD       dwFXCaps;               // driver specific stretching and effects capabilites
    DWORD       dwFXAlphaCaps;          // alpha driver specific capabilities
    DWORD       dwPalCaps;              // palette capabilities
    DWORD       dwSVCaps;               // stereo vision capabilities
    DWORD       dwAlphaBltConstBitDepths;       // DDBD_2,4,8
    DWORD       dwAlphaBltPixelBitDepths;       // DDBD_1,2,4,8
    DWORD       dwAlphaBltSurfaceBitDepths;     // DDBD_1,2,4,8
    DWORD       dwAlphaOverlayConstBitDepths;   // DDBD_2,4,8
    DWORD       dwAlphaOverlayPixelBitDepths;   // DDBD_1,2,4,8
    DWORD       dwAlphaOverlaySurfaceBitDepths; // DDBD_1,2,4,8
    DWORD       dwZBufferBitDepths;             // DDBD_8,16,24,32
    DWORD       dwVidMemTotal;          // total amount of video memory
    DWORD       dwVidMemFree;           // amount of free video memory
    DWORD       dwMaxVisibleOverlays;   // maximum number of visible overlays
    DWORD       dwCurrVisibleOverlays;  // current number of visible overlays
    DWORD       dwNumFourCCCodes;       // number of four cc codes
    DWORD       dwAlignBoundarySrc;     // source rectangle alignment
    DWORD       dwAlignSizeSrc;         // source rectangle byte size
    DWORD       dwAlignBoundaryDest;    // dest rectangle alignment
    DWORD       dwAlignSizeDest;        // dest rectangle byte size
    DWORD       dwAlignStrideAlign;     // stride alignment
    DWORD       dwRops[DD_ROP_SPACE];   // ROPS supported
    DDSCAPS     ddsCaps;                // DDSCAPS structure has all the general capabilities
    DWORD       dwMinOverlayStretch;    // minimum overlay stretch factor multiplied by 1000, eg 1000 == 1.0, 1300 == 1.3
    DWORD       dwMaxOverlayStretch;    // maximum overlay stretch factor multiplied by 1000, eg 1000 == 1.0, 1300 == 1.3
    DWORD       dwMinLiveVideoStretch;  // minimum live video stretch factor multiplied by 1000, eg 1000 == 1.0, 1300 == 1.3
    DWORD       dwMaxLiveVideoStretch;  // maximum live video stretch factor multiplied by 1000, eg 1000 == 1.0, 1300 == 1.3
    DWORD       dwMinHwCodecStretch;    // minimum hardware codec stretch factor multiplied by 1000, eg 1000 == 1.0, 1300 == 1.3
    DWORD       dwMaxHwCodecStretch;    // maximum hardware codec stretch factor multiplied by 1000, eg 1000 == 1.0, 1300 == 1.3
    DWORD       dwReserved1;            // reserved
    DWORD       dwReserved2;            // reserved
    DWORD       dwReserved3;            // reserved
    DWORD       dwSVBCaps;              // driver specific capabilities for System->Vmem blts
    DWORD       dwSVBCKeyCaps;          // driver color key capabilities for System->Vmem blts
    DWORD       dwSVBFXCaps;            // driver FX capabilities for System->Vmem blts
    DWORD       dwSVBRops[DD_ROP_SPACE];// ROPS supported for System->Vmem blts
    DWORD       dwVSBCaps;              // driver specific capabilities for Vmem->System blts
    DWORD       dwVSBCKeyCaps;          // driver color key capabilities for Vmem->System blts
    DWORD       dwVSBFXCaps;            // driver FX capabilities for Vmem->System blts
    DWORD       dwVSBRops[DD_ROP_SPACE];// ROPS supported for Vmem->System blts
    DWORD       dwSSBCaps;              // driver specific capabilities for System->System blts
    DWORD       dwSSBCKeyCaps;          // driver color key capabilities for System->System blts
    DWORD       dwSSBFXCaps;            // driver FX capabilities for System->System blts
    DWORD       dwSSBRops[DD_ROP_SPACE];// ROPS supported for System->System blts
    DWORD       dwMaxVideoPorts;        // maximum number of usable video ports
    DWORD       dwCurrVideoPorts;       // current number of video ports used
    DWORD       dwSVBCaps2;             // more driver specific capabilities for System->Vmem blts
} DDNTCORECAPS, *PDDNTCORECAPS;

/*
 * structure for D3D buffer callbacks
 */
typedef struct _DD_D3DBUFCALLBACKS
{
    DWORD dwSize;
    DWORD dwFlags;
    PDD_CANCREATESURFACE        CanCreateD3DBuffer;
    PDD_CREATESURFACE           CreateD3DBuffer;
    PDD_SURFCB_DESTROYSURFACE   DestroyD3DBuffer;
    PDD_SURFCB_LOCK             LockD3DBuffer;
    PDD_SURFCB_UNLOCK           UnlockD3DBuffer;
} DD_D3DBUFCALLBACKS, *PDD_D3DBUFCALLBACKS;

#define DDHAL_EXEBUFCB32_CANCREATEEXEBUF    0x00000001l
#define DDHAL_EXEBUFCB32_CREATEEXEBUF       0x00000002l
#define DDHAL_EXEBUFCB32_DESTROYEXEBUF      0x00000004l
#define DDHAL_EXEBUFCB32_LOCKEXEBUF         0x00000008l
#define DDHAL_EXEBUFCB32_UNLOCKEXEBUF       0x00000010l

/*
 * NT friendly names
 */
#define DDHAL_D3DBUFCB32_CANCREATED3DBUF    DDHAL_EXEBUFCB32_CANCREATEEXEBUF    
#define DDHAL_D3DBUFCB32_CREATED3DBUF       DDHAL_EXEBUFCB32_CREATEEXEBUF       
#define DDHAL_D3DBUFCB32_DESTROYD3DBUF      DDHAL_EXEBUFCB32_DESTROYEXEBUF      
#define DDHAL_D3DBUFCB32_LOCKD3DBUF         DDHAL_EXEBUFCB32_LOCKEXEBUF         
#define DDHAL_D3DBUFCB32_UNLOCKD3DBUF       DDHAL_EXEBUFCB32_UNLOCKEXEBUF       


/*
 * structure for display driver to call DDHAL_Create with
 * the _V4 version was used by NT4 drivers
 */
typedef struct _DD_HALINFO_V4
{
    DWORD                       dwSize;
    VIDEOMEMORYINFO             vmiData;                // video memory info
    DDNTCORECAPS                ddCaps;                 // hw specific caps
    PDD_GETDRIVERINFO           GetDriverInfo;          // callback for querying driver data
    DWORD                       dwFlags;                // create flags
} DD_HALINFO_V4, *PDD_HALINFO_V4;

typedef struct _DD_HALINFO
{
    DWORD                       dwSize;
    VIDEOMEMORYINFO             vmiData;                // video memory info
    DDNTCORECAPS                ddCaps;                 // hw specific caps
    PDD_GETDRIVERINFO           GetDriverInfo;          // callback for querying driver data
    DWORD                       dwFlags;                // create flags
    LPVOID                      lpD3DGlobalDriverData;  // D3D global Data
    LPVOID                      lpD3DHALCallbacks;      // D3D callbacks
    PDD_D3DBUFCALLBACKS         lpD3DBufCallbacks;      // Buffer callbacks
} DD_HALINFO, *PDD_HALINFO;

#define DDHALINFO_GETDRIVERINFOSET      0x00000004l     // indicates that GetDriverInfo is set
#define DDHALINFO_GETDRIVERINFO2        0x00000008l     // indicates driver support GetDriverInfo2 variant
                                                        // of GetDriverInfo. New for DX 8.0


/*
 * DDRAW version of DirectDraw object;
 *
 */
typedef struct _DD_DIRECTDRAW_GLOBAL
{
    VOID*                       dhpdev;         // driver's private PDEV pointer
    ULONG_PTR                   dwReserved1;    // reserved for use by display driver
    ULONG_PTR                   dwReserved2;    // reserved for use by display driver
    LPDDVIDEOPORTCAPS           lpDDVideoPortCaps;// Info returned by the HAL (an array if more than one video port)
} DD_DIRECTDRAW_GLOBAL;

typedef struct _DD_DIRECTDRAW_LOCAL
{
    PDD_DIRECTDRAW_GLOBAL       lpGbl;            // pointer to data
} DD_DIRECTDRAW_LOCAL;

typedef struct _DD_VIDEOPORT_LOCAL
{
    PDD_DIRECTDRAW_LOCAL        lpDD;             // pointer to DIRECTDRAW_LCL
    DDVIDEOPORTDESC             ddvpDesc;         // description used at create time
    DDVIDEOPORTINFO             ddvpInfo;         // most recent video port info
    PDD_SURFACE_INT             lpSurface;        // surface receiving the data
    PDD_SURFACE_INT             lpVBISurface;     // surface receiving the VBI data
    DWORD                       dwNumAutoflip;    // Number of current autoflip surfaces
    DWORD                       dwNumVBIAutoflip; // Number of VBI surfaces currently being autoflipped
    ULONG_PTR                   dwReserved1;      // Reserved for display driver
    ULONG_PTR                   dwReserved2;      // Reserved for display driver
    ULONG_PTR                   dwReserved3;      // Reserved for display driver
} DD_VIDEOPORT_LOCAL;

#define DDRAWIVPORT_ON                  0x00000001      // Video port is pumping data
#define DDRAWIVPORT_SOFTWARE_AUTOFLIP   0x00000002      // Video port cannot use hardware autoflip
#define DDRAWIVPORT_COLORKEYANDINTERP   0x00000004      // Overlay cannot bob and colorkey at same time

typedef struct _DD_MOTIONCOMP_LOCAL
{
    PDD_DIRECTDRAW_LOCAL        lpDD;             // pointer to DIRECTDRAW_LCL
    GUID                            guid;
    DWORD                           dwUncompWidth;
    DWORD                           dwUncompHeight;
    DDPIXELFORMAT                   ddUncompPixelFormat;
    DWORD                           dwDriverReserved1;
    DWORD                           dwDriverReserved2;
    DWORD                           dwDriverReserved3;
    LPVOID                          lpDriverReserved1;
    LPVOID                          lpDriverReserved2;
    LPVOID                          lpDriverReserved3;
} DD_MOTIONCOMP_LOCAL;


/*
 * More driver surface capabilities (in addition to those described in DDCORECAPS).
 * This struct contains the caps bits added to the DDCAPS.ddsCaps structure in DX6.
 */
typedef struct _DD_MORESURFACECAPS
{
    DWORD       dwSize;             // size of DDMORESURFACECAPS structure
    DDSCAPSEX   ddsCapsMore;
    /*
     * The DDMORESURFACECAPS struct is of variable size. The following list may be
     * filled in by DX6-aware drivers (see DDVERSIONINFO) to restrict their
     * video memory heaps (those which are exposed to DirectDraw) to
     * certain sets of DDSCAPS_ bits. Thse entries are exactly analogous to
     * the ddsCaps and ddsCapsAlt members of the VIDMEM structures listed in
     * the VIDMEMINFO.pvmList member of DDHALINFO.vmiData. There should be
     * exactly DDHALINFO.vmiData.dwNumHeaps copies of tagExtendedHeapRestrictions
     * in this struct. The size of this struct is thus:
     *  DDMORESURFACECAPS.dwSize = sizeof(DDMORESURFACECAPS) +
     *          (DDHALINFO.vmiData.dwNumHeaps-1) * sizeof(DDSCAPSEX)*2;
     * Note the -1 accounts for the fact that DDMORESURFACECAPS is declared to have 1
     * tagExtendedHeapRestrictions member.
     */
    struct tagNTExtendedHeapRestrictions
    {
        DDSCAPSEX   ddsCapsEx;
        DDSCAPSEX   ddsCapsExAlt;
    } ddsExtendedHeapRestrictions[1];
} DD_MORESURFACECAPS;

// for DX7, we check each mode in the driver if it supports 
// Stereo, driver returns DD_OK if mode is ok for stereo
typedef struct _DD_STEREOMODE
{
    DWORD       dwSize;             // size of DDSTEREOMODECAPS structure

    DWORD       dwHeight;
    DWORD       dwWidth;
    DWORD       dwBpp;
    DWORD       dwRefreshRate;

    BOOL        bSupported;

} DD_STEREOMODE;

typedef struct _DD_UPDATENONLOCALHEAPDATA
{
    PDD_DIRECTDRAW_GLOBAL      lpDD;                // driver struct
    DWORD                      dwHeap;              // heap index
    FLATPTR                    fpGARTLin;           // linear GART address of start of heap
    FLATPTR                    fpGARTDev;           // high physical GART address of start of heap
    ULONG_PTR                  ulPolicyMaxBytes;    // maximum amount of AGP memory to use
    HRESULT                    ddRVal;              // return value
    VOID*                      UpdateNonLocalHeap;  // Unused: Win95 compatibility
} DD_UPDATENONLOCALHEAPDATA;

/*
 * Private caps that the driver passes to change DirectDraw behavior.
 * These caps are not exposed to the application
 */

typedef struct DD_NTPRIVATEDRIVERCAPS
{
    DWORD                               dwSize;
    DWORD                               dwPrivateCaps;
} DD_NTPRIVATEDRIVERCAPS;

// Driver wants DD_CREATESURFACEDATA.lplpSList to contain a list of
// surfaces to create rather than always a single surface.
#define DDHAL_PRIVATECAP_ATOMICSURFACECREATION 0x00000001l

// Driver wants to be notified when creating a primary surface.
#define DDHAL_PRIVATECAP_NOTIFYPRIMARYCREATION  0x00000002l

#define DDHAL_PRIVATECAP_RESERVED1              0x00000004l

/////////////////////////////////////////////////////////////////////////////
// NT Note:
//
// The following structures must match, field for field, the corresponding
// structures as declared in 'ddrawi.h.'  We cannot simply use the same
// structures because the sub-structures such as DD_DIRECTDRAW_GLOBAL are
// different, and have to be properly typed for the drivers.
//
/////////////////////////////////////////////////////////////////////////////

/****************************************************************************
 *
 * DDHAL structures for Surface Object callbacks
 *
 ***************************************************************************/

/*
 * This special flag is seen only by drivers.  The DD runtime sets this
 * bit in DDHAL_BLTDATA.dwFlags if the dwAFlags and ddrgbaScaleFactors
 * members at the end of the DDHAL_BLTDATA structure are valid.
 * The flag is always set if the DDHAL_BLTDATA structure is passed to
 * the driver via the AlphaBlt HAL callback; otherwise, the flag is zero.
 */
#define DDBLT_AFLAGS 		0x80000000L

/*
 * This flag will be set in DDHAL_BLTDATA.dwAFlags if the call was originated
 * by the AlphaBlt API method. If the call was originated by the Blt API,
 * this flag will not be set.
 * Drivers which have a unified Blt/AlphaBlt DDI can use this flag to distinguish
 * between the two API calls.
 */
#define DDABLT_SRCOVERDEST      0x00000001L

/*
 * structure for passing information to DDHAL Blt fn
 */
typedef struct _DD_BLTDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDDestSurface;// dest surface
    RECTL                       rDest;          // dest rect
    PDD_SURFACE_LOCAL           lpDDSrcSurface; // src surface
    RECTL                       rSrc;           // src rect
    DWORD                       dwFlags;        // blt flags
    DWORD                       dwROPFlags;     // ROP flags (valid for ROPS only)
    DDBLTFX                     bltFX;          // blt FX
    HRESULT                     ddRVal;         // return value
    VOID*                       Blt;            // Unused: Win95 compatibility
    BOOL                        IsClipped;      // clipped blt?
    RECTL                       rOrigDest;      // unclipped dest rect
                                                // (only valid if IsClipped)
    RECTL                       rOrigSrc;       // unclipped src rect
                                                // (only valid if IsClipped)
    DWORD                       dwRectCnt;      // count of dest rects
                                                // (only valid if IsClipped)
    LPRECT                      prDestRects;    // array of dest rects
                                                // (only valid if IsClipped)
    DWORD                       dwAFlags;       // DDABLT_ flags (for AlphaBlt DDI)
    DDARGB                      ddargbScaleFactors;  // ARGB scaling factors (AlphaBlt)
} DD_BLTDATA;

/*
 * structure for passing information to DDHAL Lock fn
 */
typedef struct _DD_LOCKDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    DWORD                       bHasRect;       // rArea is valid
    RECTL                       rArea;          // area being locked
    LPVOID                      lpSurfData;     // pointer to screen memory (return value)
    HRESULT                     ddRVal;         // return value
    VOID*                       Lock;           // Unused: Win95 compatibility
    DWORD                       dwFlags;        // DDLOCK flags
    FLATPTR                     fpProcess;      // process start address
} DD_LOCKDATA;

/*
 * structure for passing information to DDHAL Unlock fn
 */
typedef struct _DD_UNLOCKDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    HRESULT                     ddRVal;         // return value
    VOID*                       Unlock;         // Unused: Win95 compatibility
} DD_UNLOCKDATA;

/*
 * structure for passing information to DDHAL UpdateOverlay fn
 */
typedef struct _DD_UPDATEOVERLAYDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDDestSurface;// dest surface
    RECTL                       rDest;          // dest rect
    PDD_SURFACE_LOCAL           lpDDSrcSurface; // src surface
    RECTL                       rSrc;           // src rect
    DWORD                       dwFlags;        // flags
    DDOVERLAYFX                 overlayFX;      // overlay FX
    HRESULT                     ddRVal;         // return value
    VOID*                       UpdateOverlay;  // Unused: Win95 compatibility
} DD_UPDATEOVERLAYDATA;

/*
 * structure for passing information to DDHAL UpdateOverlay fn
 */
typedef struct _DD_SETOVERLAYPOSITIONDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSrcSurface; // src surface
    PDD_SURFACE_LOCAL           lpDDDestSurface;// dest surface
    LONG                        lXPos;          // x position
    LONG                        lYPos;          // y position
    HRESULT                     ddRVal;         // return value
    VOID*                       SetOverlayPosition; // Unused: Win95 compatibility
} DD_SETOVERLAYPOSITIONDATA;
/*
 * structure for passing information to DDHAL SetPalette fn
 */
typedef struct _DD_SETPALETTEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    PDD_PALETTE_GLOBAL          lpDDPalette;    // palette to set to surface
    HRESULT                     ddRVal;         // return value
    VOID*                       SetPalette;     // Unused: Win95 compatibility
    BOOL                        Attach;         // attach this palette?
} DD_SETPALETTEDATA;

/*
 * structure for passing information to DDHAL Flip fn
 */
typedef struct _DD_FLIPDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpSurfCurr;     // current surface
    PDD_SURFACE_LOCAL           lpSurfTarg;     // target surface (to flip to)
    DWORD                       dwFlags;        // flags
    HRESULT                     ddRVal;         // return value
    VOID*                       Flip;           // Unused: Win95 compatibility
    PDD_SURFACE_LOCAL           lpSurfCurrLeft;     // left target surface (to flip to)
    PDD_SURFACE_LOCAL           lpSurfTargLeft;     // left target surface (to flip to)
} DD_FLIPDATA;

/*
 * structure for passing information to DDHAL DestroySurface fn
 */
typedef struct _DD_DESTROYSURFACEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    HRESULT                     ddRVal;         // return value
    VOID*                       DestroySurface;// Unused: Win95 compatibility
} DD_DESTROYSURFACEDATA;

/*
 * structure for passing information to DDHAL SetClipList fn
 */
typedef struct _DD_SETCLIPLISTDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    HRESULT                     ddRVal;         // return value
    VOID*                       SetClipList;    // Unused: Win95 compatibility
} DD_SETCLIPLISTDATA;

/*
 * structure for passing information to DDHAL AddAttachedSurface fn
 */
typedef struct _DD_ADDATTACHEDSURFACEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    PDD_SURFACE_LOCAL           lpSurfAttached; // surface to attach
    HRESULT                     ddRVal;         // return value
    VOID*                       AddAttachedSurface; // Unused: Win95 compatibility
} DD_ADDATTACHEDSURFACEDATA;

/*
 * structure for passing information to DDHAL SetColorKey fn
 */
typedef struct _DD_SETCOLORKEYDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    DWORD                       dwFlags;        // flags
    DDCOLORKEY                  ckNew;          // new color key
    HRESULT                     ddRVal;         // return value
    VOID*                       SetColorKey;    // Unused: Win95 compatibility
} DD_SETCOLORKEYDATA;

/*
 * structure for passing information to DDHAL GetBltStatus fn
 */
typedef struct _DD_GETBLTSTATUSDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    DWORD                       dwFlags;        // flags
    HRESULT                     ddRVal;         // return value
    VOID*                       GetBltStatus;   // Unused: Win95 compatibility
} DD_GETBLTSTATUSDATA;

/*
 * structure for passing information to DDHAL GetFlipStatus fn
 */
typedef struct _DD_GETFLIPSTATUSDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    DWORD                       dwFlags;        // flags
    HRESULT                     ddRVal;         // return value
    VOID*                       GetFlipStatus;  // Unused: Win95 compatibility
} DD_GETFLIPSTATUSDATA;

/****************************************************************************
 *
 * DDHAL structures for Palette Object callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL DestroyPalette fn
 */
typedef struct _DD_DESTROYPALETTEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_PALETTE_GLOBAL          lpDDPalette;    // palette struct
    HRESULT                     ddRVal;         // return value
    VOID*                       DestroyPalette; // Unused: Win95 compatibility
} DD_DESTROYPALETTEDATA;

/*
 * structure for passing information to DDHAL SetEntries fn
 */
typedef struct _DD_SETENTRIESDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_PALETTE_GLOBAL          lpDDPalette;    // palette struct
    DWORD                       dwBase;         // base palette index
    DWORD                       dwNumEntries;   // number of palette entries
    LPPALETTEENTRY              lpEntries;      // color table
    HRESULT                     ddRVal;         // return value
    VOID*                       SetEntries;     // Unused: Win95 compatibility
} DD_SETENTRIESDATA;

/****************************************************************************
 *
 * DDHAL structures for Driver Object callbacks
 *
 ***************************************************************************/

typedef DDSURFACEDESC* PDD_SURFACEDESC;

/*
 * structure for passing information to DDHAL CreateSurface fn
 */
typedef struct _DD_CREATESURFACEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACEDESC             lpDDSurfaceDesc;// description of surface being created
    PDD_SURFACE_LOCAL           *lplpSList;     // list of created surface objects
    DWORD                       dwSCnt;         // number of surfaces in SList
    HRESULT                     ddRVal;         // return value
    VOID*                       CreateSurface;  // Unused: Win95 compatibility
} DD_CREATESURFACEDATA;

/*
 * structure for passing information to DDHAL CanCreateSurface fn
 */
typedef struct _DD_CANCREATESURFACEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;                   // driver struct
    PDD_SURFACEDESC             lpDDSurfaceDesc;        // description of surface being created
    DWORD                       bIsDifferentPixelFormat;// pixel format differs from primary surface
    HRESULT                     ddRVal;                 // return value
    VOID*                       CanCreateSurface;       // Unused: Win95 compatibility
} DD_CANCREATESURFACEDATA;

/*
 * structure for passing information to DDHAL CreatePalette fn
 */
typedef struct _DD_CREATEPALETTEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_PALETTE_GLOBAL          lpDDPalette;    // ddraw palette struct
    LPPALETTEENTRY              lpColorTable;   // colors to go in palette
    HRESULT                     ddRVal;         // return value
    VOID*                       CreatePalette;  // Unused: Win95 compatibility
    BOOL                        is_excl;        // process has exclusive mode
} DD_CREATEPALETTEDATA;

/*
 * Return if the vertical blank is in progress
 */
#define DDWAITVB_I_TESTVB                       0x80000006l

/*
 * structure for passing information to DDHAL WaitForVerticalBlank fn
 */
typedef struct _DD_WAITFORVERTICALBLANKDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    DWORD                       dwFlags;        // flags
    DWORD                       bIsInVB;        // is in vertical blank
    ULONG_PTR                   hEvent;         // event
    HRESULT                     ddRVal;         // return value
    VOID*                       WaitForVerticalBlank; // Unused: Win95 compatibility
} DD_WAITFORVERTICALBLANKDATA;

/*
 * structure for passing information to DDHAL driver SetColorKey fn
 */
typedef struct _DD_DRVSETCOLORKEYDATA
{
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface struct
    DWORD                       dwFlags;        // flags
    DDCOLORKEY                  ckNew;          // new color key
    HRESULT                     ddRVal;         // return value
    VOID*                       SetColorKey;    // Unused: Win95 compatibility
} DD_DRVSETCOLORKEYDATA;

/*
 * structure for passing information to DDHAL GetScanLine fn
 */
typedef struct _DD_GETSCANLINEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    DWORD                       dwScanLine;     // returned scan line
    HRESULT                     ddRVal;         // return value
    VOID*                       GetScanLine;    // Unused: Win95 compatibility
} DD_GETSCANLINEDATA;

/*
 * structure for passing information to DDHAL MapMemory fn
 */
typedef struct _DD_MAPMEMORYDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    BOOL                        bMap;           // TRUE if map; FALSe if un-map
    HANDLE                      hProcess;       // process handle
    FLATPTR                     fpProcess;      // returned address in process' address space
    HRESULT                     ddRVal;         // return value
} DD_MAPMEMORYDATA;

/****************************************************************************
 *
 * DDHAL structures for VideoPort callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL CanCreateVideoPort fn
 */
typedef struct _DD_CANCREATEVPORTDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;               // driver struct
    LPDDVIDEOPORTDESC           lpDDVideoPortDesc;
    HRESULT                     ddRVal;             // return value
    VOID*                       CanCreateVideoPort; // Unused: Win95 compatibility
} DD_CANCREATEVPORTDATA;

/*
 * structure for passing information to DDHAL CreateVideoPort fn
 */
typedef struct _DD_CREATEVPORTDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;              // driver struct
    LPDDVIDEOPORTDESC           lpDDVideoPortDesc;
    PDD_VIDEOPORT_LOCAL         lpVideoPort;       // Video port created
    HRESULT                     ddRVal;            // return value
    VOID*                       CreateVideoPort;   // Unused: Win95 compatibility
} DD_CREATEVPORTDATA;

/*
 * structure for passing information to DDHAL FlipVideoPort fn
 */
typedef struct _DD_FLIPVPORTDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;          // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;   // Video port object
    PDD_SURFACE_LOCAL           lpSurfCurr;    // current surface
    PDD_SURFACE_LOCAL           lpSurfTarg;    // target surface
    HRESULT                     ddRVal;        // return value
    VOID*                       FlipVideoPort; // Unused: Win95 compatibility
} DD_FLIPVPORTDATA;

/*
 * structure for passing information to DDHAL GetVideoPortBandwidth fn
 */
typedef struct _DD_GETVPORTBANDWIDTHDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;                  // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;           // Video port object
    LPDDPIXELFORMAT             lpddpfFormat;          // Format for bandwidth
    DWORD                       dwWidth;
    DWORD                       dwHeight;
    DWORD                       dwFlags;               // Prescale factor for bandwidth
    LPDDVIDEOPORTBANDWIDTH      lpBandwidth;           // Returned bandwidth parameters
    HRESULT                     ddRVal;                // return value
    VOID*                       GetVideoPortBandwidth; // Unused: Win95 compatibility
} DD_GETVPORTBANDWIDTHDATA;

/*
 * structure for passing information to DDHAL GetVideoPortInputFormats fn
 */
typedef struct _DD_GETVPORTINPUTFORMATDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;                     // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;              // Video port object
    DWORD                       dwFlags;                  // VBI, regular, or both
    LPDDPIXELFORMAT             lpddpfFormat;             // Array of formats
    DWORD                       dwNumFormats;             // # of formats in array
    HRESULT                     ddRVal;                   // return value
    VOID*                       GetVideoPortInputFormats; // Unused: Win95 compatibility
} DD_GETVPORTINPUTFORMATDATA;

/*
 * structure for passing information to DDHAL GetVideoPortOutputFormats fn
 */
typedef struct _DD_GETVPORTOUTPUTFORMATDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;                     // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;              // Video port object
    DWORD                       dwFlags;                  // VBI, regular, or both
    LPDDPIXELFORMAT             lpddpfInputFormat;        // Input format
    LPDDPIXELFORMAT             lpddpfOutputFormats;      // Array of output formats
    DWORD                       dwNumFormats;             // # of formats in array
    HRESULT                     ddRVal;                   // return value
    VOID*                       GetVideoPortInputFormats; // Unused: Win95 compatibility
} DD_GETVPORTOUTPUTFORMATDATA;

/*
 * structure for passing information to DDHAL GetVideoPortField fn
 */
typedef struct _DD_GETVPORTFIELDDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;              // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;       // Video port object
    BOOL                        bField;            // TRUE if even
    HRESULT                     ddRVal;            // return value
    VOID*                       GetVideoPortField; // Unused: Win95 compatibility
} DD_GETVPORTFIELDDATA;

/*
 * structure for passing information to DDHAL GetVideoPortLine fn
 */
typedef struct _DD_GETVPORTLINEDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;             // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;      // Video port object
    DWORD                       dwLine;           // Current line counter
    HRESULT                     ddRVal;           // return value
    VOID*                       GetVideoPortLine; // Unused: Win95 compatibility
} DD_GETVPORTLINEDATA;

/*
 * structure for passing information to DDHAL GetVideoPortConnectInfo fn
 */
typedef struct _DD_GETVPORTCONNECTDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;                    // driver struct
    DWORD                       dwPortId;                // ID of desired video port
    LPDDVIDEOPORTCONNECT        lpConnect;               // Array of DDVIDEOPORTCONNECT structures
    DWORD                       dwNumEntries;            // # of structures in array
    HRESULT                     ddRVal;                  // return value
    VOID*                       GetVideoPortConnectInfo; // Unused: Win95 compatibility
} DD_GETVPORTCONNECTDATA;

/*
 * structure for passing information to DDHAL DestroyVideoPort fn
 */
typedef struct _DD_DESTROYVPORTDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;             // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;      // Video port object
    HRESULT                     ddRVal;           // return value
    VOID*                       DestroyVideoPort; // Unused: Win95 compatibility
} DD_DESTROYVPORTDATA;

/*
 * structure for passing information to DDHAL GetVideoPortFlipStatus fn
 */
typedef struct _DD_GETVPORTFLIPSTATUSDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;                   // driver struct
    FLATPTR                     fpSurface;              // surface struct
    HRESULT                     ddRVal;                 // return value
    VOID*                       GetVideoPortFlipStatus; // Unused: Win95 compatibility
} DD_GETVPORTFLIPSTATUSDATA;

typedef DDVIDEOPORTINFO*   PDD_VIDEOPORTINFO;
/*
 * structure for passing information to DDHAL UpdateVideoPort fn
 */
typedef struct _DD_UPDATEVPORTDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;             // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;      // Video port object
    PDD_SURFACE_INT            *lplpDDSurface;    // surface struct
    PDD_SURFACE_INT            *lplpDDVBISurface; // VBI surface structure
    PDD_VIDEOPORTINFO           lpVideoInfo;      // Video information
    DWORD                       dwFlags;          // DDRAWI_VPORTSTART, DDRAWI_VPORTSTOP, DDRAWI_VPORTUPDATE
    DWORD                       dwNumAutoflip;    // # of autoflip surfaces. If > 1, lpDDSurface is an array.
    DWORD                       dwNumVBIAutoflip; // # of autoflip surfaces. If > 1, lpDDVBISurface is an array.
    HRESULT                     ddRVal;           // return value
    VOID*                       UpdateVideoPort;  // Unused: Win95 compatibility
} DD_UPDATEVPORTDATA;

#define DDRAWI_VPORTSTART       0x0001
#define DDRAWI_VPORTSTOP        0x0002
#define DDRAWI_VPORTUPDATE      0x0003

/*
 * structure for passing information to DDHAL WaitForVideoPortSync fn
 */
typedef struct _DD_WAITFORVPORTSYNCDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;            // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;     // Video port object
    DWORD                       dwFlags;         // DDVPEVENT_XXXX
    DWORD                       dwLine;
    DWORD                       dwTimeOut;       // Max time to wait before returning
    HRESULT                     ddRVal;          // return value
    VOID*                       UpdateVideoPort; // Unused: Win95 compatibility
} DD_WAITFORVPORTSYNCDATA;

/*
 * structure for passing information to DDHAL GetVideoSignalStatus fn
 */
typedef struct _DD_GETVPORTSIGNALDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;                 // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;          // Video port object
    DWORD                       dwStatus;             // Video signal status
    HRESULT                     ddRVal;               // return value
    VOID*                       GetVideoSignalStatus; // Unused: Win95 compatibility
} DD_GETVPORTSIGNALDATA;

/*
 * structure for passing information to DDHAL GetVideoSignalStatus fn
 */
typedef struct _DD_VPORTCOLORDATA
{
    PDD_DIRECTDRAW_LOCAL        lpDD;         // driver struct
    PDD_VIDEOPORT_LOCAL         lpVideoPort;  // Video port object
    DWORD                       dwFlags;      // Video signal status
    LPDDCOLORCONTROL            lpColorData;
    HRESULT                     ddRVal;       // return value
    VOID*                       ColorControl; // Unused: Win95 compatibility
} DD_VPORTCOLORDATA;

#define DDRAWI_VPORTGETCOLOR    0x0001
#define DDRAWI_VPORTSETCOLOR    0x0002

/****************************************************************************
 *
 * DDHAL structures for Color Control callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL ColorControl fn
 */
typedef struct _DD_COLORCONTROLDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;    // surface
    LPDDCOLORCONTROL            lpColorData;    // color control information
    DWORD                       dwFlags;        // DDRAWI_GETCOLOR/DDRAWI_SETCOLOR
    HRESULT                     ddRVal;         // return value
    VOID*                       ColorControl;   // Unused: Win95 compatibility
} DD_COLORCONTROLDATA;

#define DDRAWI_GETCOLOR         0x0001
#define DDRAWI_SETCOLOR         0x0002

/****************************************************************************
 *
 * DDHAL structure for GetDriverData callback
 *
 ***************************************************************************/

typedef struct _DD_GETDRIVERINFODATA {

    // Input fields filled in by DirectDraw

    VOID*                       dhpdev;         // Driver context
    DWORD                       dwSize;         // Size of this structure
    DWORD                       dwFlags;        // Flags
    GUID                        guidInfo;       // GUID that DirectX is querying for
    DWORD                       dwExpectedSize; // Size of callbacks structure expected by DirectDraw.
    PVOID                       lpvData;        // Buffer that will receive the requested data

    // Output fields filled in by driver

    DWORD                       dwActualSize;   // Size of callbacks structure expected by driver
    HRESULT                     ddRVal;         // Return value from driver

} DD_GETDRIVERINFODATA;

/****************************************************************************
 *
 * DDHAL structure for misc. driver callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL GetAvailDriverMemory fn
 */
typedef struct _DD_GETAVAILDRIVERMEMORYDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;            // driver struct
    DDSCAPS                     DDSCaps;         // caps for type of surface memory
    DWORD                       dwTotal;         // total memory for this kind of surface
    DWORD                       dwFree;          // free memory for this kind of surface
    HRESULT                     ddRVal;          // return value
    VOID*                       GetAvailDriverMemory; // Unused: Win95 compatibility
} DD_GETAVAILDRIVERMEMORYDATA;


/****************************************************************************
 *
 * DDHAL structures for NT callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL FreeDriverMemory fn
 */
typedef struct _DD_FREEDRIVERMEMORYDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;            // driver struct
    PDD_SURFACE_LOCAL           lpDDSurface;     // surface object trying to be created
    HRESULT                     ddRVal;          // return value
    VOID*                       FreeDriverMemory;// Unused: Win95 compatibility
} DD_FREEDRIVERMEMORYDATA;

/*
 * structure for passing information to DDHAL SetExclusiveMode fn
 */
typedef struct _DD_SETEXCLUSIVEMODEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    DWORD                       dwEnterExcl;    // TRUE if entering exclusive mode, FALSE is leaving
    DWORD                       dwReserved;     // reserved for future use
    HRESULT                     ddRVal;         // return value
    VOID*                       SetExclusiveMode; // Unused: Win95 compatibility
} DD_SETEXCLUSIVEMODEDATA;

/*
 * structure for passing information to DDHAL FlipToGDISurface fn
 */
typedef struct _DD_FLIPTOGDISURFACEDATA
{
    PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
    DWORD                       dwToGDI;        // TRUE if flipping to the GDI surface, FALSE if flipping away
    DWORD                       dwReserved;     // reserved for future use
    HRESULT                     ddRVal;         // return value
    VOID*                       FlipToGDISurface; // Unused: Win95 compatibility
} DD_FLIPTOGDISURFACEDATA;

/****************************************************************************
 *
 * DDHAL structure for kernel callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL SyncSurfaceData fn
 */
typedef struct _DD_SYNCSURFACEDATA
{
    PDD_DIRECTDRAW_LOCAL   lpDD;        // driver struct
    PDD_SURFACE_LOCAL      lpDDSurface; // Surface to sync with
    DWORD       dwSurfaceOffset;        // Offset in frame buffer of surface
    ULONG_PTR    fpLockPtr;              // Surface lock ptr
    LONG        lPitch;                 // Surface pitch
    DWORD       dwOverlayOffset;        // Added to dwSurfaceOffset for origin, clipping, etc.
    ULONG       dwDriverReserved1;      // Reserved for the HAL
    ULONG       dwDriverReserved2;      // Reserved for the HAL
    ULONG       dwDriverReserved3;      // Reserved for the HAL
    ULONG       dwDriverReserved4;      // Reserved for the HAL
    HRESULT     ddRVal;
} DD_SYNCSURFACEDATA;

/*
 * structure for passing information to DDHAL SyncVideoPortData fn
 */
typedef struct _DD_SYNCVIDEOPORTDATA
{
    PDD_DIRECTDRAW_LOCAL    lpDD;       // driver struct
    PDD_VIDEOPORT_LOCAL     lpVideoPort;// Video port object
    DWORD       dwOriginOffset;         // Start address relative to surface
    DWORD       dwHeight;               // Height of total video region (per field)
    DWORD       dwVBIHeight;            // Height of VBI region (per field)
    ULONG       dwDriverReserved1;      // Reserved for the HAL
    ULONG       dwDriverReserved2;      // Reserved for the HAL
    ULONG       dwDriverReserved3;      // Reserved for the HAL
    HRESULT     ddRVal;
} DD_SYNCVIDEOPORTDATA;

/****************************************************************************
 *
 * DDHAL structure for motion comp callbacks
 *
 ***************************************************************************/

/*
 * structure for passing information to DDHAL GetMoCompGuids
 */
typedef struct _DD_GETMOCOMPGUIDSDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    DWORD                     dwNumGuids;
    GUID*                     lpGuids;
    HRESULT                   ddRVal;
} DD_GETMOCOMPGUIDSDATA;

/*
 * structure for passing information to DDHAL GetMoCompFormats
 */
typedef struct _DD_GETMOCOMPFORMATSDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    GUID*                     lpGuid;
    DWORD                     dwNumFormats;
    LPDDPIXELFORMAT           lpFormats;
    HRESULT                   ddRVal;
} DD_GETMOCOMPFORMATSDATA;

/*
 * structure for passing information to DDHAL CreateMoComp
 */
typedef struct _DD_CREATEMOCOMPDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    PDD_MOTIONCOMP_LOCAL      lpMoComp;
    GUID*                     lpGuid;
    DWORD                     dwUncompWidth;
    DWORD                     dwUncompHeight;
    DDPIXELFORMAT             ddUncompPixelFormat;
    LPVOID                    lpData;
    DWORD                     dwDataSize;
    HRESULT                   ddRVal;
} DD_CREATEMOCOMPDATA;

/*
 * structure for passing information to DDHAL GetMoCompBuffInfo
 */
typedef struct _DDCOMPBUFFERINFO
{
    DWORD                     dwSize;             // [in]   size of the struct
    DWORD                     dwNumCompBuffers;   // [out]  number of buffers required for compressed data
    DWORD                     dwWidthToCreate;    // [out]    Width of surface to create
    DWORD                     dwHeightToCreate;   // [out]    Height of surface to create
    DWORD                     dwBytesToAllocate;  // [out]    Total number of bytes used by each surface
    DDSCAPS2                  ddCompCaps;         // [out]    caps to create surfaces to store compressed data
    DDPIXELFORMAT             ddPixelFormat;      // [out]  format to create surfaces to store compressed data
} DDCOMPBUFFERINFO, *LPDDCOMPBUFFERINFO;

typedef struct _DD_GETMOCOMPCOMPBUFFDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    GUID*                     lpGuid;
    DWORD                     dwWidth;            // [in]   width of uncompressed data
    DWORD                     dwHeight;           // [in]   height of uncompressed data
    DDPIXELFORMAT             ddPixelFormat;      // [in]   pixel-format of uncompressed data
    DWORD                     dwNumTypesCompBuffs;// [in/out] number of memory types required for comp buffers
    LPDDCOMPBUFFERINFO        lpCompBuffInfo;     // [in]   driver supplied info regarding comp buffers (allocated by client)
    HRESULT                   ddRVal;             // [out]
} DD_GETMOCOMPCOMPBUFFDATA;

/*
 * structure for passing information to DDHAL GetMoCompBuffInfo
 */
typedef struct _DD_GETINTERNALMOCOMPDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    GUID*                     lpGuid;
    DWORD                     dwWidth;            // [in]   width of uncompressed data
    DWORD                     dwHeight;           // [in]   height of uncompressed data
    DDPIXELFORMAT             ddPixelFormat;      // [in]   pixel-format of uncompressed data
    DWORD                     dwScratchMemAlloc;  // [out]  amount of scratch memory will the hal allocate for its private use
    HRESULT                   ddRVal;             // [out]
} DD_GETINTERNALMOCOMPDATA;

/*
 * structure for passing information to DDHAL BeginMoCompFrame
 */
typedef struct _DD_BEGINMOCOMPFRAMEDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    PDD_MOTIONCOMP_LOCAL      lpMoComp;
    PDD_SURFACE_LOCAL         lpDestSurface;        // [in]  destination buffer in which to decoding this frame
    DWORD                     dwInputDataSize;      // [in]  size of other misc input data to begin frame
    LPVOID                    lpInputData;          // [in]  pointer to misc input data
    DWORD                     dwOutputDataSize;     // [in]  size of other misc output data to begin frame
    LPVOID                    lpOutputData;         // [in]  pointer to output misc data (allocated by client)
    HRESULT                   ddRVal;               // [out]
} DD_BEGINMOCOMPFRAMEDATA;

/*
 * structure for passing information to DDHAL EndMoCompFrame
 */
typedef struct _DD_ENDMOCOMPFRAMEDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    PDD_MOTIONCOMP_LOCAL      lpMoComp;
    LPVOID                    lpInputData;
    DWORD                     dwInputDataSize;
    HRESULT                   ddRVal;
} DD_ENDMOCOMPFRAMEDATA;

/*
 * structure for passing information to DDHAL RenderMoComp
 */
typedef struct _DDMOCOMPBUFFERINFO
{
    DWORD                     dwSize;         // [in]    size of the struct
    PDD_SURFACE_LOCAL         lpCompSurface;  // [in]    pointer to buffer containing compressed data
    DWORD                     dwDataOffset;   // [in]    offset of relevant data from the beginning of buffer
    DWORD                     dwDataSize;     // [in]    size of relevant data
    LPVOID                    lpPrivate;      // Reserved by DirectDraw
} DDMOCOMPBUFFERINFO, *LPDDMOCOMPBUFFERINFO;

typedef struct _DD_RENDERMOCOMPDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    PDD_MOTIONCOMP_LOCAL      lpMoComp;
    DWORD                     dwNumBuffers;     // [in]  Number of entries in the lpMacroBlockInfo array
    LPDDMOCOMPBUFFERINFO      lpBufferInfo;     // [in]  Surfaces containing macro block info
    DWORD                     dwFunction;       // [in]  Function
    LPVOID                    lpInputData;
    DWORD                     dwInputDataSize;
    LPVOID                    lpOutputData;
    DWORD                     dwOutputDataSize;
    HRESULT                   ddRVal;           // [out]
} DD_RENDERMOCOMPDATA;

/*
 * structure for passing information to DDHAL QueryMoCompStatus
 */
typedef struct _DD_QUERYMOCOMPSTATUSDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    PDD_MOTIONCOMP_LOCAL      lpMoComp;
    PDD_SURFACE_LOCAL         lpSurface;        // [in]  Surface being queried
    DWORD                     dwFlags;          // [in]  DDMCQUERY_XXX flags
    HRESULT                   ddRVal;            // [out]
} DD_QUERYMOCOMPSTATUSDATA;

#define DDMCQUERY_READ          0x00000001

/*
 * structure for passing information to DDHAL DestroyVideo
 */
typedef struct _DD_DESTROYMOCOMPDATA
{
    PDD_DIRECTDRAW_LOCAL      lpDD;
    PDD_MOTIONCOMP_LOCAL      lpMoComp;
    HRESULT                   ddRVal;
} DD_DESTROYMOCOMPDATA;


/****************************************************************************
 *
 * DDHAL structures for Miscellaneous2 callbacks
 *
 ***************************************************************************/
// This DDI is called by the kernel only.
typedef struct _DD_CREATESURFACEEXDATA
{
    DWORD                       dwFlags;
    PDD_DIRECTDRAW_LOCAL        lpDDLcl;        // driver struct
    PDD_SURFACE_LOCAL           lpDDSLcl;       // created surface
                                                // objects
    HRESULT                     ddRVal;         // return value
} DD_CREATESURFACEEXDATA;

// This DDI is used by both ddraw and d3d to obtain information from
// the driver.
typedef struct _DD_GETDRIVERSTATEDATA
{
    DWORD                       dwFlags;        // Flags to indicate the data
                                                // required
    union
    {
        PDD_DIRECTDRAW_GLOBAL       lpDD;           // driver struct
        DWORD_PTR                   dwhContext;     // d3d context
    };
    LPDWORD                     lpdwStates;     // ptr to the state data
                                                // to be filled in by the
                                                // driver
    DWORD                       dwLength;
    HRESULT                     ddRVal;         // return value
} DD_GETDRIVERSTATEDATA;

typedef struct _DD_DESTROYDDLOCALDATA
{
    DWORD dwFlags;
    PDD_DIRECTDRAW_LOCAL pDDLcl;
    HRESULT  ddRVal;
} DD_DESTROYDDLOCALDATA;


#ifdef __cplusplus
};
#endif

#endif  // GUID_DEFS_ONLY


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
