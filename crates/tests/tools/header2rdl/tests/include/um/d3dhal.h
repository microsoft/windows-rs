/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:   d3dhal.h
 *  Content:    Direct3D HAL include file
 *
 ***************************************************************************/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _D3DHAL_H_
#define _D3DHAL_H_
#include "ddraw.h"
#include "d3dtypes.h"
#include "d3dcaps.h"
#include "d3d.h"
#if(DIRECT3D_VERSION >= 0x0900 )
#include "d3d9.h"
#include "dxva9typ.h"
#else
#include "d3d8.h"
#endif

struct _D3DHAL_CALLBACKS;
typedef struct _D3DHAL_CALLBACKS D3DHAL_CALLBACKS, *LPD3DHAL_CALLBACKS;

struct _D3DHAL_CALLBACKS2;
typedef struct _D3DHAL_CALLBACKS2 D3DHAL_CALLBACKS2, *LPD3DHAL_CALLBACKS2;

struct _D3DHAL_CALLBACKS3;
typedef struct _D3DHAL_CALLBACKS3 D3DHAL_CALLBACKS3, *LPD3DHAL_CALLBACKS3;

typedef struct _DDRAWI_DIRECTDRAW_GBL FAR *LPDDRAWI_DIRECTDRAW_GBL;
typedef struct _DDRAWI_DIRECTDRAW_LCL FAR *LPDDRAWI_DIRECTDRAW_LCL;
struct _DDRAWI_DDRAWSURFACE_LCL;
typedef struct _DDRAWI_DDRAWSURFACE_LCL FAR *LPDDRAWI_DDRAWSURFACE_LCL;

/*
 * If the HAL driver does not implement clipping, it must reserve at least
 * this much space at the end of the LocalVertexBuffer for use by the HEL
 * clipping.  I.e. the vertex buffer contain dwNumVertices+dwNumClipVertices
 * vertices.  No extra space is needed by the HEL clipping in the
 * LocalHVertexBuffer.
 */
#define D3DHAL_NUMCLIPVERTICES  20

/*
 * These are a few special internal renderstates etc. that would
 * logically be in d3dtypes.h, but that file is external, so they are
 * here.
 */
#define D3DTSS_MAX_DX6 ((D3DTEXTURESTAGESTATETYPE)24)
#define D3DTSS_MAX_DX7 ((D3DTEXTURESTAGESTATETYPE)29)
#define D3DTSS_MAX_DX8 ((D3DTEXTURESTAGESTATETYPE)29)
#define D3DTSS_MAX_DX9 ((D3DTEXTURESTAGESTATETYPE)33)

#if( DIRECT3D_VERSION >= 0x0900 )
#define D3DTSS_MAX D3DTSS_MAX_DX9
#elif( DIRECT3D_VERSION >= 0x0800 )
#define D3DTSS_MAX D3DTSS_MAX_DX8
#elif( DIRECT3D_VERSION >= 0x0700 )
#define D3DTSS_MAX D3DTSS_MAX_DX7
#else
#define D3DTSS_MAX D3DTSS_MAX_DX6
#endif

/*
 * If DX8 driver wants to support pre-DX8 applications, it should use these
 * definitions for pre-DX8 world matrices
*/
#define D3DTRANSFORMSTATE_WORLD_DX7  1
#define D3DTRANSFORMSTATE_WORLD1_DX7 4
#define D3DTRANSFORMSTATE_WORLD2_DX7 5
#define D3DTRANSFORMSTATE_WORLD3_DX7 6

/*
 * Generally needed maximum state structure sizes.  Note that the copy of
 * these in refrasti.hpp must be kept in sync with these.
 */
#define D3DHAL_MAX_RSTATES_DX6 (256)
#define D3DHAL_MAX_RSTATES_DX7 (256)
#define D3DHAL_MAX_RSTATES_DX8 (256)
#define D3DHAL_MAX_RSTATES_DX9 (256)

#if( DIRECT3D_VERSION >= 0x0900 )
#define D3DHAL_MAX_RSTATES D3DHAL_MAX_RSTATES_DX9
#elif( DIRECT3D_VERSION >= 0x0800 )
#define D3DHAL_MAX_RSTATES D3DHAL_MAX_RSTATES_DX8
#elif( DIRECT3D_VERSION >= 0x0700 )
#define D3DHAL_MAX_RSTATES D3DHAL_MAX_RSTATES_DX7
#else
#define D3DHAL_MAX_RSTATES D3DHAL_MAX_RSTATES_DX6
#endif

#define D3D_MAXRENDERSTATES ((D3DRENDERSTATETYPE)D3DHAL_MAX_RSTATES)

/* Last state offset for combined render state and texture stage array + 1 */
#define D3DHAL_MAX_RSTATES_AND_STAGES \
    (D3DHAL_TSS_RENDERSTATEBASE + \
     D3DHAL_TSS_MAXSTAGES * D3DHAL_TSS_STATESPERSTAGE)
/* Last texture state ID */
#define D3DHAL_MAX_TEXTURESTATES (13)
/* Last texture state ID + 1 */
#define D3DHAL_TEXTURESTATEBUF_SIZE (D3DHAL_MAX_TEXTURESTATES+1)

/*
 * If no dwNumVertices is given, this is what will be used.
 */
#define D3DHAL_DEFAULT_TL_NUM   ((32 * 1024) / sizeof (D3DTLVERTEX))
#define D3DHAL_DEFAULT_H_NUM    ((32 * 1024) / sizeof (D3DHVERTEX))

/*
 * Description for a device.
 * This is used to describe a device that is to be created or to query
 * the current device.
 *
 * For DX5 and subsequent runtimes, D3DDEVICEDESC is a user-visible
 * structure that is not seen by the device drivers. The runtime
 * stitches a D3DDEVICEDESC together using the D3DDEVICEDESC_V1
 * embedded in the GLOBALDRIVERDATA and the extended caps queried
 * from the driver using GetDriverInfo.
 */

typedef struct _D3DDeviceDesc_V1 {
    DWORD        dwSize;             /* Size of D3DDEVICEDESC structure */
    DWORD        dwFlags;                /* Indicates which fields have valid data */
    D3DCOLORMODEL    dcmColorModel;      /* Color model of device */
    DWORD        dwDevCaps;              /* Capabilities of device */
    D3DTRANSFORMCAPS dtcTransformCaps;       /* Capabilities of transform */
    BOOL         bClipping;              /* Device can do 3D clipping */
    D3DLIGHTINGCAPS  dlcLightingCaps;        /* Capabilities of lighting */
    D3DPRIMCAPS      dpcLineCaps;
    D3DPRIMCAPS      dpcTriCaps;
    DWORD            dwDeviceRenderBitDepth; /* One of DDBD_16, etc.. */
    DWORD        dwDeviceZBufferBitDepth;/* One of DDBD_16, 32, etc.. */
    DWORD        dwMaxBufferSize;        /* Maximum execute buffer size */
    DWORD        dwMaxVertexCount;       /* Maximum vertex count */
} D3DDEVICEDESC_V1, *LPD3DDEVICEDESC_V1;

#define D3DDEVICEDESCSIZE_V1 (sizeof(D3DDEVICEDESC_V1))

/*
 * This is equivalent to the D3DDEVICEDESC understood by DX5, available only
 * from DX6. It is the same as D3DDEVICEDESC structure in DX5.
 * D3DDEVICEDESC is still the user-visible structure that is not seen by the
 * device drivers. The runtime stitches a D3DDEVICEDESC together using the
 * D3DDEVICEDESC_V1 embedded in the GLOBALDRIVERDATA and the extended caps
 * queried from the driver using GetDriverInfo.
 */

typedef struct _D3DDeviceDesc_V2 {
    DWORD        dwSize;             /* Size of D3DDEVICEDESC structure */
    DWORD        dwFlags;                /* Indicates which fields have valid data */
    D3DCOLORMODEL    dcmColorModel;      /* Color model of device */
    DWORD        dwDevCaps;              /* Capabilities of device */
    D3DTRANSFORMCAPS dtcTransformCaps;       /* Capabilities of transform */
    BOOL         bClipping;              /* Device can do 3D clipping */
    D3DLIGHTINGCAPS  dlcLightingCaps;        /* Capabilities of lighting */
    D3DPRIMCAPS      dpcLineCaps;
    D3DPRIMCAPS      dpcTriCaps;
    DWORD            dwDeviceRenderBitDepth; /* One of DDBD_16, etc.. */
    DWORD        dwDeviceZBufferBitDepth;/* One of DDBD_16, 32, etc.. */
    DWORD        dwMaxBufferSize;        /* Maximum execute buffer size */
    DWORD        dwMaxVertexCount;       /* Maximum vertex count */

    DWORD        dwMinTextureWidth, dwMinTextureHeight;
    DWORD        dwMaxTextureWidth, dwMaxTextureHeight;
    DWORD        dwMinStippleWidth, dwMaxStippleWidth;
    DWORD        dwMinStippleHeight, dwMaxStippleHeight;

} D3DDEVICEDESC_V2, *LPD3DDEVICEDESC_V2;

#define D3DDEVICEDESCSIZE_V2 (sizeof(D3DDEVICEDESC_V2))

#if(DIRECT3D_VERSION >= 0x0700)
/*
 * This is equivalent to the D3DDEVICEDESC understood by DX6, available only
 * from DX6. It is the same as D3DDEVICEDESC structure in DX6.
 * D3DDEVICEDESC is still the user-visible structure that is not seen by the
 * device drivers. The runtime stitches a D3DDEVICEDESC together using the
 * D3DDEVICEDESC_V1 embedded in the GLOBALDRIVERDATA and the extended caps
 * queried from the driver using GetDriverInfo.
 */

typedef struct _D3DDeviceDesc_V3 {
    DWORD        dwSize;             /* Size of D3DDEVICEDESC structure */
    DWORD        dwFlags;                /* Indicates which fields have valid data */
    D3DCOLORMODEL    dcmColorModel;      /* Color model of device */
    DWORD        dwDevCaps;              /* Capabilities of device */
    D3DTRANSFORMCAPS dtcTransformCaps;       /* Capabilities of transform */
    BOOL         bClipping;              /* Device can do 3D clipping */
    D3DLIGHTINGCAPS  dlcLightingCaps;        /* Capabilities of lighting */
    D3DPRIMCAPS      dpcLineCaps;
    D3DPRIMCAPS      dpcTriCaps;
    DWORD            dwDeviceRenderBitDepth; /* One of DDBD_16, etc.. */
    DWORD        dwDeviceZBufferBitDepth;/* One of DDBD_16, 32, etc.. */
    DWORD        dwMaxBufferSize;        /* Maximum execute buffer size */
    DWORD        dwMaxVertexCount;       /* Maximum vertex count */

    DWORD        dwMinTextureWidth, dwMinTextureHeight;
    DWORD        dwMaxTextureWidth, dwMaxTextureHeight;
    DWORD        dwMinStippleWidth, dwMaxStippleWidth;
    DWORD        dwMinStippleHeight, dwMaxStippleHeight;

    DWORD       dwMaxTextureRepeat;
    DWORD       dwMaxTextureAspectRatio;
    DWORD       dwMaxAnisotropy;
    D3DVALUE    dvGuardBandLeft;
    D3DVALUE    dvGuardBandTop;
    D3DVALUE    dvGuardBandRight;
    D3DVALUE    dvGuardBandBottom;
    D3DVALUE    dvExtentsAdjust;
    DWORD       dwStencilCaps;
    DWORD       dwFVFCaps;  /* low 4 bits: 0 implies TLVERTEX only, 1..8 imply FVF aware */
    DWORD       dwTextureOpCaps;
    WORD        wMaxTextureBlendStages;
    WORD        wMaxSimultaneousTextures;
} D3DDEVICEDESC_V3, *LPD3DDEVICEDESC_V3;

#define D3DDEVICEDESCSIZE_V3 (sizeof(D3DDEVICEDESC_V3))
#endif /* DIRECT3D_VERSION >= 0x0700 */

/* --------------------------------------------------------------
 * Instantiated by the HAL driver on driver connection.
 */
typedef struct _D3DHAL_GLOBALDRIVERDATA {
    DWORD       dwSize;         // Size of this structure
    D3DDEVICEDESC_V1    hwCaps;                 // Capabilities of the hardware
    DWORD       dwNumVertices;      // see following comment
    DWORD       dwNumClipVertices;  // see following comment
    DWORD       dwNumTextureFormats;    // Number of texture formats
    LPDDSURFACEDESC lpTextureFormats;   // Pointer to texture formats
} D3DHAL_GLOBALDRIVERDATA;

typedef D3DHAL_GLOBALDRIVERDATA *LPD3DHAL_GLOBALDRIVERDATA;

#define D3DHAL_GLOBALDRIVERDATASIZE (sizeof(D3DHAL_GLOBALDRIVERDATA))

#if(DIRECT3D_VERSION >= 0x0700)
/* --------------------------------------------------------------
 * Extended caps introduced with DX5 and queried with
 * GetDriverInfo (GUID_D3DExtendedCaps).
 */
typedef struct _D3DHAL_D3DDX6EXTENDEDCAPS {
    DWORD       dwSize;         // Size of this structure

    DWORD       dwMinTextureWidth, dwMaxTextureWidth;
    DWORD       dwMinTextureHeight, dwMaxTextureHeight;
    DWORD       dwMinStippleWidth, dwMaxStippleWidth;
    DWORD       dwMinStippleHeight, dwMaxStippleHeight;

    /* fields added for DX6 */
    DWORD       dwMaxTextureRepeat;
    DWORD       dwMaxTextureAspectRatio;
    DWORD       dwMaxAnisotropy;
    D3DVALUE    dvGuardBandLeft;
    D3DVALUE    dvGuardBandTop;
    D3DVALUE    dvGuardBandRight;
    D3DVALUE    dvGuardBandBottom;
    D3DVALUE    dvExtentsAdjust;
    DWORD       dwStencilCaps;
    DWORD       dwFVFCaps;  /* low 4 bits: 0 implies TLVERTEX only, 1..8 imply FVF aware */
    DWORD       dwTextureOpCaps;
    WORD        wMaxTextureBlendStages;
    WORD        wMaxSimultaneousTextures;

} D3DHAL_D3DDX6EXTENDEDCAPS;
#endif /* DIRECT3D_VERSION >= 0x0700 */

/* --------------------------------------------------------------
 * Extended caps introduced with DX5 and queried with
 * GetDriverInfo (GUID_D3DExtendedCaps).
 */
typedef struct _D3DHAL_D3DEXTENDEDCAPS {
    DWORD       dwSize;         // Size of this structure

    DWORD       dwMinTextureWidth, dwMaxTextureWidth;
    DWORD       dwMinTextureHeight, dwMaxTextureHeight;
    DWORD       dwMinStippleWidth, dwMaxStippleWidth;
    DWORD       dwMinStippleHeight, dwMaxStippleHeight;

    /* fields added for DX6 */
    DWORD       dwMaxTextureRepeat;
    DWORD       dwMaxTextureAspectRatio;
    DWORD       dwMaxAnisotropy;
    D3DVALUE    dvGuardBandLeft;
    D3DVALUE    dvGuardBandTop;
    D3DVALUE    dvGuardBandRight;
    D3DVALUE    dvGuardBandBottom;
    D3DVALUE    dvExtentsAdjust;
    DWORD       dwStencilCaps;
    DWORD       dwFVFCaps;  /* low 4 bits: 0 implies TLVERTEX only, 1..8 imply FVF aware */
    DWORD       dwTextureOpCaps;
    WORD        wMaxTextureBlendStages;
    WORD        wMaxSimultaneousTextures;

#if(DIRECT3D_VERSION >= 0x0700)
    /* fields added for DX7 */
    DWORD       dwMaxActiveLights;
    D3DVALUE    dvMaxVertexW;

    WORD        wMaxUserClipPlanes;
    WORD        wMaxVertexBlendMatrices;

    DWORD       dwVertexProcessingCaps;

    DWORD       dwReserved1;
    DWORD       dwReserved2;
    DWORD       dwReserved3;
    DWORD       dwReserved4;
#endif /* DIRECT3D_VERSION >= 0x0700 */
} D3DHAL_D3DEXTENDEDCAPS;

typedef D3DHAL_D3DEXTENDEDCAPS *LPD3DHAL_D3DEXTENDEDCAPS;
#define D3DHAL_D3DEXTENDEDCAPSSIZE (sizeof(D3DHAL_D3DEXTENDEDCAPS))

#if(DIRECT3D_VERSION >= 0x0700)
typedef D3DHAL_D3DDX6EXTENDEDCAPS *LPD3DHAL_D3DDX6EXTENDEDCAPS;
#define D3DHAL_D3DDX6EXTENDEDCAPSSIZE (sizeof(D3DHAL_D3DDX6EXTENDEDCAPS))
#endif /* DIRECT3D_VERSION >= 0x0700 */

#if(DIRECT3D_VERSION >= 0x0900)
typedef struct _D3DCAPS8
{
    /* Device Info */
    D3DDEVTYPE  DeviceType;
    UINT    AdapterOrdinal;

    /* Caps from DX7 Draw */
    DWORD   Caps;
    DWORD   Caps2;
    DWORD   Caps3;
    DWORD   PresentationIntervals;

    /* Cursor Caps */
    DWORD   CursorCaps;

    /* 3D Device Caps */
    DWORD   DevCaps;

    DWORD   PrimitiveMiscCaps;
    DWORD   RasterCaps;
    DWORD   ZCmpCaps;
    DWORD   SrcBlendCaps;
    DWORD   DestBlendCaps;
    DWORD   AlphaCmpCaps;
    DWORD   ShadeCaps;
    DWORD   TextureCaps;
    DWORD   TextureFilterCaps;          // D3DPTFILTERCAPS for IDirect3DTexture8's
    DWORD   CubeTextureFilterCaps;      // D3DPTFILTERCAPS for IDirect3DCubeTexture8's
    DWORD   VolumeTextureFilterCaps;    // D3DPTFILTERCAPS for IDirect3DVolumeTexture8's
    DWORD   TextureAddressCaps;         // D3DPTADDRESSCAPS for IDirect3DTexture8's
    DWORD   VolumeTextureAddressCaps;   // D3DPTADDRESSCAPS for IDirect3DVolumeTexture8's

    DWORD   LineCaps;                   // D3DLINECAPS

    DWORD   MaxTextureWidth, MaxTextureHeight;
    DWORD   MaxVolumeExtent;

    DWORD   MaxTextureRepeat;
    DWORD   MaxTextureAspectRatio;
    DWORD   MaxAnisotropy;
    float   MaxVertexW;

    float   GuardBandLeft;
    float   GuardBandTop;
    float   GuardBandRight;
    float   GuardBandBottom;

    float   ExtentsAdjust;
    DWORD   StencilCaps;

    DWORD   FVFCaps;
    DWORD   TextureOpCaps;
    DWORD   MaxTextureBlendStages;
    DWORD   MaxSimultaneousTextures;

    DWORD   VertexProcessingCaps;
    DWORD   MaxActiveLights;
    DWORD   MaxUserClipPlanes;
    DWORD   MaxVertexBlendMatrices;
    DWORD   MaxVertexBlendMatrixIndex;

    float   MaxPointSize;

    DWORD   MaxPrimitiveCount;          // max number of primitives per DrawPrimitive call
    DWORD   MaxVertexIndex;
    DWORD   MaxStreams;
    DWORD   MaxStreamStride;            // max stride for SetStreamSource

    DWORD   VertexShaderVersion;
    DWORD   MaxVertexShaderConst;       // number of vertex shader constant registers

    DWORD   PixelShaderVersion;
    float   MaxPixelShaderValue;        // max value of pixel shader arithmetic component
} D3DCAPS8;

typedef D3DLIGHT9 D3DLIGHT8;
typedef D3DMATERIAL9 D3DMATERIAL8;
typedef D3DVIEWPORT9 D3DVIEWPORT8;
#define D3DRS_EDGEANTIALIAS D3DRS_RESERVED0
#ifndef D3DPRASTERCAPS_ANTIALIASEDGES
#define D3DPRASTERCAPS_ANTIALIASEDGES D3DPRASTERCAPS_RESERVED0
#endif /* D3DPRASTERCAPS_ANTIALIASEDGES */
#ifndef D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE
#define D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE 0x00800000L
#endif /* D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE */
#ifndef D3DPRASTERCAPS_PAT
#define D3DPRASTERCAPS_PAT 0x00000008L  // equal to D3DPRASTERCAPS_RESERVED2
#endif


// This cap should not be used in DX9+ drivers
//
#ifndef D3DVTXPCAPS_NO_VSDT_UBYTE4
#define D3DVTXPCAPS_NO_VSDT_UBYTE4      0x00000080L /* device does not support D3DVSDT_UBYTE4 */
#endif

// These are old filter caps that have been retired.
#define D3DTEXF_FLATCUBIC     4
#define D3DTEXF_GAUSSIANCUBIC 5

// These are line related states and caps that have been retired
typedef struct _D3DLINEPATTERN {
    WORD    wRepeatFactor;
    WORD    wLinePattern;
} D3DLINEPATTERN;

#define D3DPMISCCAPS_LINEPATTERNREP     0x00000004L

#endif /* DIRECT3D_VERSION >= 0x0900 */

/* --------------------------------------------------------------
 * Argument to the HAL functions.
 *
 * !!! When this structure is changed, D3DHAL_CONTEXTCREATEDATA in
 *  windows\published\ntgdistr.h also must be changed to be the same size !!!
 *
 */

typedef struct _D3DHAL_CONTEXTCREATEDATA
{
    union
    {
        LPDDRAWI_DIRECTDRAW_GBL lpDDGbl;    // in:  Driver struct (legacy)
        LPDDRAWI_DIRECTDRAW_LCL lpDDLcl;    // in:  For DX7 driver onwards
    };

    union
    {
        LPDIRECTDRAWSURFACE lpDDS;      // in:  Surface to be used as target
        LPDDRAWI_DDRAWSURFACE_LCL lpDDSLcl; // For DX7 onwards
    };

    union
    {
        LPDIRECTDRAWSURFACE lpDDSZ;     // in:  Surface to be used as Z
        LPDDRAWI_DDRAWSURFACE_LCL lpDDSZLcl; // For DX7 onwards
    };

    union
    {
        DWORD       dwPID;      // in:  Current process id
        ULONG_PTR dwrstates;  // Must be larger enough to hold a pointer as
                              // we can return a pointer in this field
    };
    ULONG_PTR       dwhContext; // out: Context handle
    HRESULT     ddrval;     // out: Return value
} D3DHAL_CONTEXTCREATEDATA;
typedef D3DHAL_CONTEXTCREATEDATA *LPD3DHAL_CONTEXTCREATEDATA;

typedef struct _D3DHAL_CONTEXTDESTROYDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    HRESULT     ddrval;     // out: Return value
} D3DHAL_CONTEXTDESTROYDATA;
typedef D3DHAL_CONTEXTDESTROYDATA *LPD3DHAL_CONTEXTDESTROYDATA;

typedef struct _D3DHAL_CONTEXTDESTROYALLDATA
{
    DWORD       dwPID;      // in:  Process id to destroy contexts for
    HRESULT     ddrval;     // out: Return value
} D3DHAL_CONTEXTDESTROYALLDATA;
typedef D3DHAL_CONTEXTDESTROYALLDATA *LPD3DHAL_CONTEXTDESTROYALLDATA;

typedef struct _D3DHAL_SCENECAPTUREDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    DWORD       dwFlag;     // in:  Indicates beginning or end
    HRESULT     ddrval;     // out: Return value
} D3DHAL_SCENECAPTUREDATA;
typedef D3DHAL_SCENECAPTUREDATA *LPD3DHAL_SCENECAPTUREDATA;

typedef struct _D3DHAL_RENDERSTATEDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    DWORD       dwOffset;   // in:  Where to find states in buffer
    DWORD       dwCount;    // in:  How many states to process
    LPDIRECTDRAWSURFACE lpExeBuf;   // in:  Execute buffer containing data
    HRESULT     ddrval;     // out: Return value
} D3DHAL_RENDERSTATEDATA;
typedef D3DHAL_RENDERSTATEDATA *LPD3DHAL_RENDERSTATEDATA;

typedef struct _D3DHAL_RENDERPRIMITIVEDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    DWORD       dwOffset;   // in:  Where to find primitive data in buffer
    DWORD       dwStatus;   // in/out: Condition branch status
    LPDIRECTDRAWSURFACE lpExeBuf;   // in:  Execute buffer containing data
    DWORD       dwTLOffset; // in:  Byte offset in lpTLBuf for start of vertex data
    LPDIRECTDRAWSURFACE lpTLBuf;    // in:  Execute buffer containing TLVertex data
    D3DINSTRUCTION  diInstruction;  // in:  Primitive instruction
    HRESULT     ddrval;     // out: Return value
} D3DHAL_RENDERPRIMITIVEDATA;
typedef D3DHAL_RENDERPRIMITIVEDATA *LPD3DHAL_RENDERPRIMITIVEDATA;

typedef struct _D3DHAL_TEXTURECREATEDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    LPDIRECTDRAWSURFACE lpDDS;      // in:  Pointer to surface object
    DWORD       dwHandle;   // out: Handle to texture
    HRESULT     ddrval;     // out: Return value
} D3DHAL_TEXTURECREATEDATA;
typedef D3DHAL_TEXTURECREATEDATA *LPD3DHAL_TEXTURECREATEDATA;

typedef struct _D3DHAL_TEXTUREDESTROYDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    DWORD       dwHandle;   // in:  Handle to texture
    HRESULT     ddrval;     // out: Return value
} D3DHAL_TEXTUREDESTROYDATA;
typedef D3DHAL_TEXTUREDESTROYDATA *LPD3DHAL_TEXTUREDESTROYDATA;

typedef struct _D3DHAL_TEXTURESWAPDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    DWORD       dwHandle1;  // in:  Handle to texture 1
    DWORD       dwHandle2;  // in:  Handle to texture 2
    HRESULT     ddrval;     // out: Return value
} D3DHAL_TEXTURESWAPDATA;
typedef D3DHAL_TEXTURESWAPDATA *LPD3DHAL_TEXTURESWAPDATA;

typedef struct _D3DHAL_TEXTUREGETSURFDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    ULONG_PTR    lpDDS;      // out: Pointer to surface object
    DWORD       dwHandle;   // in:  Handle to texture
    HRESULT     ddrval;     // out: Return value
} D3DHAL_TEXTUREGETSURFDATA;
typedef D3DHAL_TEXTUREGETSURFDATA *LPD3DHAL_TEXTUREGETSURFDATA;

typedef struct _D3DHAL_GETSTATEDATA
{
    ULONG_PTR       dwhContext; // in:  Context handle
    DWORD       dwWhich;    // in:  Transform, lighting or render?
    D3DSTATE        ddState;    // in/out: State.
    HRESULT     ddrval;     // out: Return value
} D3DHAL_GETSTATEDATA;
typedef D3DHAL_GETSTATEDATA *LPD3DHAL_GETSTATEDATA;


/* --------------------------------------------------------------
 * Direct3D HAL Table.
 * Instantiated by the HAL driver on connection.
 *
 * Calls take the form of:
 *  retcode = HalCall(HalCallData* lpData);
 */

typedef DWORD   (__stdcall *LPD3DHAL_CONTEXTCREATECB)   (LPD3DHAL_CONTEXTCREATEDATA);
typedef DWORD   (__stdcall *LPD3DHAL_CONTEXTDESTROYCB)  (LPD3DHAL_CONTEXTDESTROYDATA);
typedef DWORD   (__stdcall *LPD3DHAL_CONTEXTDESTROYALLCB) (LPD3DHAL_CONTEXTDESTROYALLDATA);
typedef DWORD   (__stdcall *LPD3DHAL_SCENECAPTURECB)    (LPD3DHAL_SCENECAPTUREDATA);
typedef DWORD   (__stdcall *LPD3DHAL_RENDERSTATECB) (LPD3DHAL_RENDERSTATEDATA);
typedef DWORD   (__stdcall *LPD3DHAL_RENDERPRIMITIVECB) (LPD3DHAL_RENDERPRIMITIVEDATA);
typedef DWORD   (__stdcall *LPD3DHAL_TEXTURECREATECB)   (LPD3DHAL_TEXTURECREATEDATA);
typedef DWORD   (__stdcall *LPD3DHAL_TEXTUREDESTROYCB)  (LPD3DHAL_TEXTUREDESTROYDATA);
typedef DWORD   (__stdcall *LPD3DHAL_TEXTURESWAPCB) (LPD3DHAL_TEXTURESWAPDATA);
typedef DWORD   (__stdcall *LPD3DHAL_TEXTUREGETSURFCB)  (LPD3DHAL_TEXTUREGETSURFDATA);
typedef DWORD   (__stdcall *LPD3DHAL_GETSTATECB)    (LPD3DHAL_GETSTATEDATA);


/*
 * Regarding dwNumVertices, specify 0 if you are relying on the HEL to do
 * everything and you do not need the resultant TLVertex buffer to reside
 * in device memory.
 * The HAL driver will be asked to allocate dwNumVertices + dwNumClipVertices
 * in the case described above.
 */

typedef struct _D3DHAL_CALLBACKS
{
    DWORD           dwSize;

    // Device context
    LPD3DHAL_CONTEXTCREATECB    ContextCreate;
    LPD3DHAL_CONTEXTDESTROYCB   ContextDestroy;
    LPD3DHAL_CONTEXTDESTROYALLCB ContextDestroyAll;

    // Scene Capture
    LPD3DHAL_SCENECAPTURECB SceneCapture;

    LPVOID                      lpReserved10;           // Must be zero
    LPVOID                      lpReserved11;           // Must be zero

    // Execution
    LPD3DHAL_RENDERSTATECB  RenderState;
    LPD3DHAL_RENDERPRIMITIVECB  RenderPrimitive;

    DWORD           dwReserved;     // Must be zero

    // Textures
    LPD3DHAL_TEXTURECREATECB    TextureCreate;
    LPD3DHAL_TEXTUREDESTROYCB   TextureDestroy;
    LPD3DHAL_TEXTURESWAPCB  TextureSwap;
    LPD3DHAL_TEXTUREGETSURFCB   TextureGetSurf;

    LPVOID                      lpReserved12;           // Must be zero
    LPVOID                      lpReserved13;           // Must be zero
    LPVOID                      lpReserved14;           // Must be zero
    LPVOID                      lpReserved15;           // Must be zero
    LPVOID                      lpReserved16;           // Must be zero
    LPVOID                      lpReserved17;           // Must be zero
    LPVOID                      lpReserved18;           // Must be zero
    LPVOID                      lpReserved19;           // Must be zero
    LPVOID                      lpReserved20;           // Must be zero
    LPVOID                      lpReserved21;           // Must be zero

    // Pipeline state
    LPD3DHAL_GETSTATECB     GetState;

    DWORD           dwReserved0;        // Must be zero
    DWORD           dwReserved1;        // Must be zero
    DWORD           dwReserved2;        // Must be zero
    DWORD           dwReserved3;        // Must be zero
    DWORD           dwReserved4;        // Must be zero
    DWORD           dwReserved5;        // Must be zero
    DWORD           dwReserved6;        // Must be zero
    DWORD           dwReserved7;        // Must be zero
    DWORD           dwReserved8;        // Must be zero
    DWORD           dwReserved9;        // Must be zero

} D3DHAL_CALLBACKS;
typedef D3DHAL_CALLBACKS *LPD3DHAL_CALLBACKS;

#define D3DHAL_SIZE_V1 sizeof( D3DHAL_CALLBACKS )


typedef struct _D3DHAL_SETRENDERTARGETDATA
{
    ULONG_PTR               dwhContext;     // in:  Context handle
    union
    {
        LPDIRECTDRAWSURFACE lpDDS;          // in:  new render target
        LPDDRAWI_DDRAWSURFACE_LCL lpDDSLcl;
    };

    union
    {
        LPDIRECTDRAWSURFACE lpDDSZ;         // in:  new Z buffer
        LPDDRAWI_DDRAWSURFACE_LCL lpDDSZLcl;
    };

    HRESULT             ddrval;         // out: Return value
} D3DHAL_SETRENDERTARGETDATA;
typedef D3DHAL_SETRENDERTARGETDATA FAR *LPD3DHAL_SETRENDERTARGETDATA;

// This bit is the same as D3DCLEAR_RESERVED0 in d3d8types.h
// When set it means that driver has to cull rects against current viewport.
// The bit is set only for pure devices
//
#define D3DCLEAR_COMPUTERECTS   0x00000008l

typedef struct _D3DHAL_CLEARDATA
{
    ULONG_PTR               dwhContext;     // in:  Context handle

    // dwFlags can contain D3DCLEAR_TARGET or D3DCLEAR_ZBUFFER
    DWORD               dwFlags;        // in:  surfaces to clear

    DWORD               dwFillColor;    // in:  Color value for rtarget
    DWORD               dwFillDepth;    // in:  Depth value for Z buffer

    LPD3DRECT           lpRects;        // in:  Rectangles to clear
    DWORD               dwNumRects;     // in:  Number of rectangles

    HRESULT             ddrval;         // out: Return value
} D3DHAL_CLEARDATA;
typedef D3DHAL_CLEARDATA FAR *LPD3DHAL_CLEARDATA;

typedef struct _D3DHAL_DRAWONEPRIMITIVEDATA
{
    ULONG_PTR               dwhContext;     // in:  Context handle

    DWORD               dwFlags;        // in:  flags

    D3DPRIMITIVETYPE    PrimitiveType;  // in:  type of primitive to draw
    union{
    D3DVERTEXTYPE       VertexType;     // in:  type of vertices
    DWORD               dwFVFControl;   // in:  FVF control DWORD
    };
    LPVOID              lpvVertices;    // in:  pointer to vertices
    DWORD               dwNumVertices;  // in:  number of vertices

    DWORD               dwReserved;     // in:  reserved

    HRESULT             ddrval;         // out: Return value

} D3DHAL_DRAWONEPRIMITIVEDATA;
typedef D3DHAL_DRAWONEPRIMITIVEDATA *LPD3DHAL_DRAWONEPRIMITIVEDATA;


typedef struct _D3DHAL_DRAWONEINDEXEDPRIMITIVEDATA
{
    ULONG_PTR               dwhContext;     // in: Context handle

    DWORD               dwFlags;        // in: flags word

    // Primitive and vertex type
    D3DPRIMITIVETYPE    PrimitiveType;  // in: primitive type
    union{
    D3DVERTEXTYPE       VertexType;     // in: vertex type
    DWORD               dwFVFControl;   // in:  FVF control DWORD
    };

    // Vertices
    LPVOID              lpvVertices;    // in: vertex data
    DWORD               dwNumVertices;  // in: vertex count

    // Indices
    LPWORD              lpwIndices;     // in: index data
    DWORD               dwNumIndices;   // in: index count

    HRESULT             ddrval;         // out: Return value
} D3DHAL_DRAWONEINDEXEDPRIMITIVEDATA;
typedef D3DHAL_DRAWONEINDEXEDPRIMITIVEDATA *LPD3DHAL_DRAWONEINDEXEDPRIMITIVEDATA;


typedef struct _D3DHAL_DRAWPRIMCOUNTS
{
    WORD wNumStateChanges;
    WORD wPrimitiveType;
    WORD wVertexType;
    WORD wNumVertices;
} D3DHAL_DRAWPRIMCOUNTS, *LPD3DHAL_DRAWPRIMCOUNTS;

typedef struct _D3DHAL_DRAWPRIMITIVESDATA
{
    ULONG_PTR               dwhContext;     // in:  Context handle

    DWORD               dwFlags;

    //
    // Data block:
    //
    // Consists of interleaved D3DHAL_DRAWPRIMCOUNTS, state change pairs,
    // and primitive drawing commands.
    //
    //  D3DHAL_DRAWPRIMCOUNTS: gives number of state change pairs and
    //          the information on the primitive to draw.
    //          wPrimitiveType is of type D3DPRIMITIVETYPE. Drivers
    //              must support all 7 of the primitive types specified
    //              in the DrawPrimitive API.
    //          Currently, wVertexType will always be D3DVT_TLVERTEX.
    //          If the wNumVertices member is 0, then the driver should
    //              return after doing the state changing. This is the
    //              terminator for the command stream.
    // state change pairs: DWORD pairs specify the state changes that
    //          the driver should effect before drawing the primitive.
    //          wNumStateChanges can be 0, in which case the next primitive
    //          should be drawn without any state changes in between.
    //          If present, the state change pairs are NOT aligned, they
    //          immediately follow the PRIMCOUNTS structure.
    // vertex data (if any): is 32-byte aligned.
    //
    // If a primcounts structure follows (i.e. if wNumVertices was nonzero
    // in the previous one), then it will immediately follow the state
    // changes or vertex data with no alignment padding.
    //

    LPVOID              lpvData;

    DWORD               dwFVFControl;   // in:  FVF control DWORD

    HRESULT             ddrval;         // out: Return value
} D3DHAL_DRAWPRIMITIVESDATA;
typedef D3DHAL_DRAWPRIMITIVESDATA *LPD3DHAL_DRAWPRIMITIVESDATA;

typedef DWORD (CALLBACK *LPD3DHAL_SETRENDERTARGETCB) (LPD3DHAL_SETRENDERTARGETDATA);
typedef DWORD (CALLBACK *LPD3DHAL_CLEARCB)           (LPD3DHAL_CLEARDATA);
typedef DWORD (CALLBACK *LPD3DHAL_DRAWONEPRIMITIVECB)   (LPD3DHAL_DRAWONEPRIMITIVEDATA);
typedef DWORD (CALLBACK *LPD3DHAL_DRAWONEINDEXEDPRIMITIVECB) (LPD3DHAL_DRAWONEINDEXEDPRIMITIVEDATA);
typedef DWORD (CALLBACK *LPD3DHAL_DRAWPRIMITIVESCB)   (LPD3DHAL_DRAWPRIMITIVESDATA);

typedef struct _D3DHAL_CALLBACKS2
{
    DWORD                       dwSize;                 // size of struct
    DWORD                       dwFlags;                // flags for callbacks
    LPD3DHAL_SETRENDERTARGETCB  SetRenderTarget;
    LPD3DHAL_CLEARCB            Clear;
    LPD3DHAL_DRAWONEPRIMITIVECB DrawOnePrimitive;
    LPD3DHAL_DRAWONEINDEXEDPRIMITIVECB DrawOneIndexedPrimitive;
    LPD3DHAL_DRAWPRIMITIVESCB    DrawPrimitives;
} D3DHAL_CALLBACKS2;
typedef D3DHAL_CALLBACKS2 *LPD3DHAL_CALLBACKS2;

#define D3DHAL_CALLBACKS2SIZE       sizeof(D3DHAL_CALLBACKS2)

#define D3DHAL2_CB32_SETRENDERTARGET    0x00000001L
#define D3DHAL2_CB32_CLEAR              0x00000002L
#define D3DHAL2_CB32_DRAWONEPRIMITIVE   0x00000004L
#define D3DHAL2_CB32_DRAWONEINDEXEDPRIMITIVE 0x00000008L
#define D3DHAL2_CB32_DRAWPRIMITIVES     0x00000010L

/* --------------------------------------------------------------
 * D3DCallbacks3 - queried with GetDriverInfo (GUID_D3DCallbacks3).
 *
 * Clear2 - enables stencil clears (exposed to the API in
 *      IDirect3DViewport3::Clear2
 * ValidateTextureStageState - evaluates the context's current state (including
 *      multitexture) and returns an error if the hardware cannot
 *      accelerate the current state vector.
 * DrawPrimitives2 - Renders primitives, and changes device state specified
 *                   in the command buffer.
 *
 * Multitexture-aware drivers must implement both ValidateTextureStageState.
 */

typedef struct _D3DHAL_CLEAR2DATA
{
    ULONG_PTR           dwhContext;     // in:  Context handle

  // dwFlags can contain D3DCLEAR_TARGET, D3DCLEAR_ZBUFFER, and/or D3DCLEAR_STENCIL
    DWORD               dwFlags;        // in:  surfaces to clear

    DWORD               dwFillColor;    // in:  Color value for rtarget
    D3DVALUE            dvFillDepth;    // in:  Depth value for Z buffer (0.0-1.0)
    DWORD               dwFillStencil;  // in:  value used to clear stencil buffer

    LPD3DRECT           lpRects;        // in:  Rectangles to clear
    DWORD               dwNumRects;     // in:  Number of rectangles

    HRESULT             ddrval;         // out: Return value
} D3DHAL_CLEAR2DATA;
typedef D3DHAL_CLEAR2DATA FAR *LPD3DHAL_CLEAR2DATA;

typedef struct _D3DHAL_VALIDATETEXTURESTAGESTATEDATA
{
    ULONG_PTR           dwhContext;     // in:  Context handle
    DWORD               dwFlags;        // in:  Flags, currently set to 0
    ULONG_PTR           dwReserved;     //
    DWORD               dwNumPasses;    // out: Number of passes the hardware
                                        //      can perform the operation in
    HRESULT             ddrval;         // out: return value
} D3DHAL_VALIDATETEXTURESTAGESTATEDATA;
typedef D3DHAL_VALIDATETEXTURESTAGESTATEDATA *LPD3DHAL_VALIDATETEXTURESTAGESTATEDATA;

//-----------------------------------------------------------------------------
// DrawPrimitives2 DDI
//-----------------------------------------------------------------------------

//
// Command structure for vertex buffer rendering
//

typedef struct _D3DHAL_DP2COMMAND
{
    BYTE bCommand;           // vertex command
    BYTE bReserved;
    union
    {
        WORD wPrimitiveCount;   // primitive count for unconnected primitives
        WORD wStateCount;     // count of render states to follow
    };
} D3DHAL_DP2COMMAND, *LPD3DHAL_DP2COMMAND;

//
// DrawPrimitives2 commands:
//

typedef enum _D3DHAL_DP2OPERATION
{
    D3DDP2OP_POINTS               = 1,
    D3DDP2OP_INDEXEDLINELIST      = 2,
    D3DDP2OP_INDEXEDTRIANGLELIST  = 3,
    D3DDP2OP_RENDERSTATE          = 8,
    D3DDP2OP_LINELIST             = 15,
    D3DDP2OP_LINESTRIP            = 16,
    D3DDP2OP_INDEXEDLINESTRIP     = 17,
    D3DDP2OP_TRIANGLELIST         = 18,
    D3DDP2OP_TRIANGLESTRIP        = 19,
    D3DDP2OP_INDEXEDTRIANGLESTRIP = 20,
    D3DDP2OP_TRIANGLEFAN          = 21,
    D3DDP2OP_INDEXEDTRIANGLEFAN   = 22,
    D3DDP2OP_TRIANGLEFAN_IMM      = 23,
    D3DDP2OP_LINELIST_IMM         = 24,
    D3DDP2OP_TEXTURESTAGESTATE    = 25,     // Has edge flags and called from Execute
    D3DDP2OP_INDEXEDTRIANGLELIST2 = 26,
    D3DDP2OP_INDEXEDLINELIST2     = 27,
    D3DDP2OP_VIEWPORTINFO         = 28,
    D3DDP2OP_WINFO                = 29,
// two below are for pre-DX7 interface apps running DX7 driver
    D3DDP2OP_SETPALETTE           = 30,
    D3DDP2OP_UPDATEPALETTE        = 31,
#if(DIRECT3D_VERSION >= 0x0700)
    // New for DX7
    D3DDP2OP_ZRANGE               = 32,
    D3DDP2OP_SETMATERIAL          = 33,
    D3DDP2OP_SETLIGHT             = 34,
    D3DDP2OP_CREATELIGHT          = 35,
    D3DDP2OP_SETTRANSFORM         = 36,
    D3DDP2OP_TEXBLT               = 38,
    D3DDP2OP_STATESET             = 39,
    D3DDP2OP_SETPRIORITY          = 40,
#endif /* DIRECT3D_VERSION >= 0x0700 */
    D3DDP2OP_SETRENDERTARGET      = 41,
    D3DDP2OP_CLEAR                = 42,
#if(DIRECT3D_VERSION >= 0x0700)
    D3DDP2OP_SETTEXLOD            = 43,
    D3DDP2OP_SETCLIPPLANE         = 44,
#endif /* DIRECT3D_VERSION >= 0x0700 */
#if(DIRECT3D_VERSION >= 0x0800)
    D3DDP2OP_CREATEVERTEXSHADER   = 45,
    D3DDP2OP_DELETEVERTEXSHADER   = 46,
    D3DDP2OP_SETVERTEXSHADER      = 47,
    D3DDP2OP_SETVERTEXSHADERCONST = 48,
    D3DDP2OP_SETSTREAMSOURCE      = 49,
    D3DDP2OP_SETSTREAMSOURCEUM    = 50,
    D3DDP2OP_SETINDICES           = 51,
    D3DDP2OP_DRAWPRIMITIVE        = 52,
    D3DDP2OP_DRAWINDEXEDPRIMITIVE = 53,
    D3DDP2OP_CREATEPIXELSHADER    = 54,
    D3DDP2OP_DELETEPIXELSHADER    = 55,
    D3DDP2OP_SETPIXELSHADER       = 56,
    D3DDP2OP_SETPIXELSHADERCONST  = 57,
    D3DDP2OP_CLIPPEDTRIANGLEFAN   = 58,
    D3DDP2OP_DRAWPRIMITIVE2       = 59,
    D3DDP2OP_DRAWINDEXEDPRIMITIVE2= 60,
    D3DDP2OP_DRAWRECTPATCH        = 61,
    D3DDP2OP_DRAWTRIPATCH         = 62,
    D3DDP2OP_VOLUMEBLT            = 63,
    D3DDP2OP_BUFFERBLT            = 64,
    D3DDP2OP_MULTIPLYTRANSFORM    = 65,
    D3DDP2OP_ADDDIRTYRECT         = 66,
    D3DDP2OP_ADDDIRTYBOX          = 67,
#endif /* DIRECT3D_VERSION >= 0x0800 */
#if(DIRECT3D_VERSION >= 0x0900)
    D3DDP2OP_CREATEVERTEXSHADERDECL   = 71,
    D3DDP2OP_DELETEVERTEXSHADERDECL   = 72,
    D3DDP2OP_SETVERTEXSHADERDECL      = 73,
    D3DDP2OP_CREATEVERTEXSHADERFUNC   = 74,
    D3DDP2OP_DELETEVERTEXSHADERFUNC   = 75,
    D3DDP2OP_SETVERTEXSHADERFUNC      = 76,
    D3DDP2OP_SETVERTEXSHADERCONSTI    = 77,
    D3DDP2OP_SETSCISSORRECT           = 79,
    D3DDP2OP_SETSTREAMSOURCE2         = 80,
    D3DDP2OP_BLT                      = 81,
    D3DDP2OP_COLORFILL                = 82,
    D3DDP2OP_SETVERTEXSHADERCONSTB    = 83,
    D3DDP2OP_CREATEQUERY              = 84,
    D3DDP2OP_SETRENDERTARGET2         = 85,
    D3DDP2OP_SETDEPTHSTENCIL          = 86,
    D3DDP2OP_RESPONSECONTINUE         = 87, /* Can come only from driver */
    D3DDP2OP_RESPONSEQUERY            = 88, /* Can come only from driver */
    D3DDP2OP_GENERATEMIPSUBLEVELS     = 89,
    D3DDP2OP_DELETEQUERY              = 90,
    D3DDP2OP_ISSUEQUERY               = 91,
    D3DDP2OP_SETPIXELSHADERCONSTI     = 93,
    D3DDP2OP_SETPIXELSHADERCONSTB     = 94,
    D3DDP2OP_SETSTREAMSOURCEFREQ      = 95,
    D3DDP2OP_SURFACEBLT               = 96,
    D3DDP2OP_SETCONVOLUTIONKERNELMONO = 97,
    D3DDP2OP_COMPOSERECTS             = 98,
#endif /* DIRECT3D_VERSION >= 0x0900 */

} D3DHAL_DP2OPERATION;

//
// DrawPrimitives2 point primitives
//

typedef struct _D3DHAL_DP2POINTS
{
    WORD wCount;
    WORD wVStart;
} D3DHAL_DP2POINTS, *LPD3DHAL_DP2POINTS;

//
// DrawPrimitives2 line primitives
//

typedef struct _D3DHAL_DP2STARTVERTEX
{
    WORD wVStart;
} D3DHAL_DP2STARTVERTEX, *LPD3DHAL_DP2STARTVERTEX;

typedef struct _D3DHAL_DP2LINELIST
{
    WORD wVStart;
} D3DHAL_DP2LINELIST, *LPD3DHAL_DP2LINELIST;

typedef struct _D3DHAL_DP2INDEXEDLINELIST
{
    WORD wV1;
    WORD wV2;
} D3DHAL_DP2INDEXEDLINELIST, *LPD3DHAL_DP2INDEXEDLINELIST;

typedef struct _D3DHAL_DP2LINESTRIP
{
    WORD wVStart;
} D3DHAL_DP2LINESTRIP, *LPD3DHAL_DP2LINESTRIP;

typedef struct _D3DHAL_DP2INDEXEDLINESTRIP
{
    WORD wV[2];
} D3DHAL_DP2INDEXEDLINESTRIP, *LPD3DHAL_DP2INDEXEDLINESTRIP;

//
// DrawPrimitives2 triangle primitives
//

typedef struct _D3DHAL_DP2TRIANGLELIST
{
    WORD wVStart;
} D3DHAL_DP2TRIANGLELIST, *LPD3DHAL_DP2TRIANGLELIST;

typedef struct _D3DHAL_DP2INDEXEDTRIANGLELIST
{
    WORD wV1;
    WORD wV2;
    WORD wV3;
    WORD wFlags;
} D3DHAL_DP2INDEXEDTRIANGLELIST, *LPD3DHAL_DP2INDEXEDTRIANGLELIST;

typedef struct _D3DHAL_DP2INDEXEDTRIANGLELIST2
{
    WORD wV1;
    WORD wV2;
    WORD wV3;
} D3DHAL_DP2INDEXEDTRIANGLELIST2, *LPD3DHAL_DP2INDEXEDTRIANGLELIST2;

typedef struct _D3DHAL_DP2TRIANGLESTRIP
{
    WORD wVStart;
} D3DHAL_DP2TRIANGLESTRIP, *LPD3DHAL_DP2TRIANGLESTRIP;

typedef struct _D3DHAL_DP2INDEXEDTRIANGLESTRIP
{
    WORD wV[3];
} D3DHAL_DP2INDEXEDTRIANGLESTRIP, *LPD3DHAL_DP2INDEXEDTRIANGLESTRIP;

typedef struct _D3DHAL_DP2TRIANGLEFAN
{
    WORD wVStart;
} D3DHAL_DP2TRIANGLEFAN, *LPD3DHAL_DP2TRIANGLEFAN;

typedef struct _D3DHAL_DP2INDEXEDTRIANGLEFAN
{
    WORD wV[3];
} D3DHAL_DP2INDEXEDTRIANGLEFAN, *LPD3DHAL_DP2INDEXEDTRIANGLEFAN;

typedef struct _D3DHAL_DP2TRIANGLEFAN_IMM
{
    DWORD dwEdgeFlags;
} D3DHAL_DP2TRIANGLEFAN_IMM;

typedef D3DHAL_DP2TRIANGLEFAN_IMM  *LPD3DHAL_DP2TRIANGLEFAN_IMM;

//
// DrawPrimitives2 Renderstate changes
//

typedef struct _D3DHAL_DP2RENDERSTATE
{
    D3DRENDERSTATETYPE RenderState;
    union
    {
        D3DVALUE dvState;
        DWORD dwState;
    };
} D3DHAL_DP2RENDERSTATE;
typedef D3DHAL_DP2RENDERSTATE  * LPD3DHAL_DP2RENDERSTATE;

typedef struct _D3DHAL_DP2TEXTURESTAGESTATE
{
    WORD wStage;
    WORD TSState;
    DWORD dwValue;
} D3DHAL_DP2TEXTURESTAGESTATE;
typedef D3DHAL_DP2TEXTURESTAGESTATE  *LPD3DHAL_DP2TEXTURESTAGESTATE;

typedef struct _D3DHAL_DP2VIEWPORTINFO
{
    DWORD dwX;
    DWORD dwY;
    DWORD dwWidth;
    DWORD dwHeight;
} D3DHAL_DP2VIEWPORTINFO;
typedef D3DHAL_DP2VIEWPORTINFO  *LPD3DHAL_DP2VIEWPORTINFO;

typedef struct _D3DHAL_DP2WINFO
{
    D3DVALUE        dvWNear;
    D3DVALUE        dvWFar;
} D3DHAL_DP2WINFO;
typedef D3DHAL_DP2WINFO  *LPD3DHAL_DP2WINFO;

typedef struct _D3DHAL_DP2SETPALETTE
{
    DWORD dwPaletteHandle;
    DWORD dwPaletteFlags;
    DWORD dwSurfaceHandle;
} D3DHAL_DP2SETPALETTE;
typedef D3DHAL_DP2SETPALETTE  *LPD3DHAL_DP2SETPALETTE;

typedef struct _D3DHAL_DP2UPDATEPALETTE
{
    DWORD dwPaletteHandle;
    WORD  wStartIndex;
    WORD  wNumEntries;
} D3DHAL_DP2UPDATEPALETTE;
typedef D3DHAL_DP2UPDATEPALETTE  *LPD3DHAL_DP2UPDATEPALETTE;

typedef struct _D3DHAL_DP2SETRENDERTARGET
{
    DWORD hRenderTarget;
    DWORD hZBuffer;
} D3DHAL_DP2SETRENDERTARGET;
typedef D3DHAL_DP2SETRENDERTARGET  *LPD3DHAL_DP2SETRENDERTARGET;

#if(DIRECT3D_VERSION >= 0x0700)
// Values for dwOperations in the D3DHAL_DP2STATESET
#define D3DHAL_STATESETBEGIN     0
#define D3DHAL_STATESETEND       1
#define D3DHAL_STATESETDELETE    2
#define D3DHAL_STATESETEXECUTE   3
#define D3DHAL_STATESETCAPTURE   4
#endif /* DIRECT3D_VERSION >= 0x0700 */
#if(DIRECT3D_VERSION >= 0x0800)
#define D3DHAL_STATESETCREATE    5
#endif /* DIRECT3D_VERSION >= 0x0800 */
#if(DIRECT3D_VERSION >= 0x0700)

typedef struct _D3DHAL_DP2STATESET
{
    DWORD               dwOperation;
    DWORD               dwParam;  // State set handle passed with D3DHAL_STATESETBEGIN,
                                  // D3DHAL_STATESETEXECUTE, D3DHAL_STATESETDELETE
                                  // D3DHAL_STATESETCAPTURE
    D3DSTATEBLOCKTYPE   sbType;   // Type use with D3DHAL_STATESETBEGIN/END
} D3DHAL_DP2STATESET;
typedef D3DHAL_DP2STATESET  *LPD3DHAL_DP2STATESET;
//
// T&L Hal specific stuff
//
typedef struct _D3DHAL_DP2ZRANGE
{
    D3DVALUE    dvMinZ;
    D3DVALUE    dvMaxZ;
} D3DHAL_DP2ZRANGE;
typedef D3DHAL_DP2ZRANGE  *LPD3DHAL_DP2ZRANGE;

typedef D3DMATERIAL7 D3DHAL_DP2SETMATERIAL, *LPD3DHAL_DP2SETMATERIAL;

// Values for dwDataType in D3DHAL_DP2SETLIGHT
#define D3DHAL_SETLIGHT_ENABLE   0
#define D3DHAL_SETLIGHT_DISABLE  1
// If this is set, light data will be passed in after the
// D3DLIGHT7 structure
#define D3DHAL_SETLIGHT_DATA     2

typedef struct _D3DHAL_DP2SETLIGHT
{
    DWORD     dwIndex;
    DWORD     dwDataType;
} D3DHAL_DP2SETLIGHT;
typedef D3DHAL_DP2SETLIGHT  *LPD3DHAL_DP2SETLIGHT;

typedef struct _D3DHAL_DP2SETCLIPPLANE
{
    DWORD     dwIndex;
    D3DVALUE  plane[4];
} D3DHAL_DP2SETCLIPPLANE;
typedef D3DHAL_DP2SETCLIPPLANE  *LPD3DHAL_DP2SETCLIPPLANE;

typedef struct _D3DHAL_DP2CREATELIGHT
{
    DWORD dwIndex;
} D3DHAL_DP2CREATELIGHT;
typedef D3DHAL_DP2CREATELIGHT  *LPD3DHAL_DP2CREATELIGHT;

typedef struct _D3DHAL_DP2SETTRANSFORM
{
    D3DTRANSFORMSTATETYPE xfrmType;
    D3DMATRIX matrix;
} D3DHAL_DP2SETTRANSFORM;
typedef D3DHAL_DP2SETTRANSFORM  *LPD3DHAL_DP2SETTRANSFORM;

typedef struct _D3DHAL_DP2MULTIPLYTRANSFORM
{
    D3DTRANSFORMSTATETYPE xfrmType;
    D3DMATRIX matrix;
} D3DHAL_DP2MULTIPLYTRANSFORM;
typedef D3DHAL_DP2MULTIPLYTRANSFORM  *LPD3DHAL_DP2MULTIPLYTRANSFORM;

typedef struct _D3DHAL_DP2EXT
{
    DWORD dwExtToken;
    DWORD dwSize;
} D3DHAL_DP2EXT;
typedef D3DHAL_DP2EXT  *LPD3DHAL_DP2EXT;

typedef struct _D3DHAL_DP2TEXBLT
{
    DWORD   dwDDDestSurface;// dest surface
    DWORD   dwDDSrcSurface; // src surface
    POINT   pDest;
    RECTL   rSrc;       // src rect
    DWORD   dwFlags;    // blt flags
} D3DHAL_DP2TEXBLT;
typedef D3DHAL_DP2TEXBLT  *LPD3DHAL_DP2TEXBLT;

typedef struct _D3DHAL_DP2SETPRIORITY
{
    DWORD dwDDSurface;
    DWORD dwPriority;
} D3DHAL_DP2SETPRIORITY;
typedef D3DHAL_DP2SETPRIORITY  *LPD3DHAL_DP2SETPRIORITY;
#endif /* DIRECT3D_VERSION >= 0x0700 */

typedef struct _D3DHAL_DP2CLEAR
{
  // dwFlags can contain D3DCLEAR_TARGET, D3DCLEAR_ZBUFFER, and/or D3DCLEAR_STENCIL
    DWORD               dwFlags;        // in:  surfaces to clear
    DWORD               dwFillColor;    // in:  Color value for rtarget
    D3DVALUE            dvFillDepth;    // in:  Depth value for Z buffer (0.0-1.0)
    DWORD               dwFillStencil;  // in:  value used to clear stencil buffer
    RECT                Rects[1];       // in:  Rectangles to clear
} D3DHAL_DP2CLEAR;
typedef D3DHAL_DP2CLEAR  *LPD3DHAL_DP2CLEAR;


#if(DIRECT3D_VERSION >= 0x0700)
typedef struct _D3DHAL_DP2SETTEXLOD
{
    DWORD dwDDSurface;
    DWORD dwLOD;
} D3DHAL_DP2SETTEXLOD;
typedef D3DHAL_DP2SETTEXLOD  *LPD3DHAL_DP2SETTEXLOD;
#endif /* DIRECT3D_VERSION >= 0x0700 */

#if(DIRECT3D_VERSION >= 0x0800)

// Used by SetVertexShader, DeleteVertexShader
// SetVertexShaderDecl, DeleteVertexShaderDecl,
// SetVertexShaderFunc, DeleteVertexShaderFunc
typedef struct _D3DHAL_DP2VERTEXSHADER
{
    // Vertex shader handle.
    // The handle could be 0, meaning that the current vertex shader is invalid
    // (not set). When driver recieves handle 0, it should invalidate all
    // streams pointer
#endif /* DIRECT3D_VERSION >= 0x0800 */
#if(DIRECT3D_VERSION >= 0x0900)
    // When SetVertexShaderDecl is used, the dwHandle could be a legacy FVF
    // handle or a DX9 declaration handle. Bit 0 is set for DX9 declaration.
    //
    // When SetVertexShaderFunc is used and dwHandle is zero, this means fixed
    // function pipeline
#endif /* DIRECT3D_VERSION >= 0x0900 */
#if(DIRECT3D_VERSION >= 0x0800)
    DWORD dwHandle;
} D3DHAL_DP2VERTEXSHADER;
typedef D3DHAL_DP2VERTEXSHADER  *LPD3DHAL_DP2VERTEXSHADER;

typedef struct _D3DHAL_DP2CREATEVERTEXSHADER
{
    DWORD dwHandle;     // Shader handle
    DWORD dwDeclSize;   // Shader declaration size in bytes
    DWORD dwCodeSize;   // Shader code size in bytes
    // Declaration follows
    // Shader code follows
} D3DHAL_DP2CREATEVERTEXSHADER;
typedef D3DHAL_DP2CREATEVERTEXSHADER  *LPD3DHAL_DP2CREATEVERTEXSHADER;

// Used with all types of vertex shader constants
typedef struct _D3DHAL_DP2SETVERTEXSHADERCONST
{
    DWORD dwRegister;   // Const register to start copying
    DWORD dwCount;      // Number of 4-float vectors to copy for D3DDP2OP_SETVERTEXSHADERCONST
                        // Number of 4-integer vectors to copy for D3DDP2OP_SETVERTEXSHADERCONSTI
                        // Number of BOOL values to copy for D3DDP2OP_SETVERTEXSHADERCONSTB
    // Data follows
} D3DHAL_DP2SETVERTEXSHADERCONST;
typedef D3DHAL_DP2SETVERTEXSHADERCONST  *LPD3DHAL_DP2SETVERTEXSHADERCONST;

typedef struct _D3DHAL_DP2SETSTREAMSOURCE
{
    DWORD dwStream;     // Stream index, starting from zero
    DWORD dwVBHandle;   // Vertex buffer handle
    DWORD dwStride;     // Vertex size in bytes
} D3DHAL_DP2SETSTREAMSOURCE;
typedef D3DHAL_DP2SETSTREAMSOURCE  *LPD3DHAL_DP2SETSTREAMSOURCE;

typedef struct _D3DHAL_DP2SETSTREAMSOURCEUM
{
    DWORD dwStream;     // Stream index, starting from zero
    DWORD dwStride;     // Vertex size in bytes
} D3DHAL_DP2SETSTREAMSOURCEUM;
typedef D3DHAL_DP2SETSTREAMSOURCEUM  *LPD3DHAL_DP2SETSTREAMSOURCEUM;

typedef struct _D3DHAL_DP2SETINDICES
{
    DWORD dwVBHandle;           // Index buffer handle
    DWORD dwStride;             // Index size in bytes (2 or 4)
} D3DHAL_DP2SETINDICES;
typedef D3DHAL_DP2SETINDICES  *LPD3DHAL_DP2SETINDICES;

typedef struct _D3DHAL_DP2DRAWPRIMITIVE
{
    D3DPRIMITIVETYPE primType;
    DWORD VStart;
    DWORD PrimitiveCount;
} D3DHAL_DP2DRAWPRIMITIVE;
typedef D3DHAL_DP2DRAWPRIMITIVE  *LPD3DHAL_DP2DRAWPRIMITIVE;

typedef struct _D3DHAL_DP2DRAWINDEXEDPRIMITIVE
{
    D3DPRIMITIVETYPE primType;
    INT BaseVertexIndex;          // Vertex which corresponds to index 0
    DWORD MinIndex;                 // Min vertex index in the vertex buffer
    DWORD NumVertices;              // Number of vertices starting from MinIndex
    DWORD StartIndex;               // Start index in the index buffer
    DWORD PrimitiveCount;
} D3DHAL_DP2DRAWINDEXEDPRIMITIVE;
typedef D3DHAL_DP2DRAWINDEXEDPRIMITIVE  *LPD3DHAL_DP2DRAWINDEXEDPRIMITIVE;

typedef struct _D3DHAL_CLIPPEDTRIANGLEFAN
{
    DWORD FirstVertexOffset;            // Offset in bytes in the current stream 0
    DWORD dwEdgeFlags;
    DWORD PrimitiveCount;
} D3DHAL_CLIPPEDTRIANGLEFAN;
typedef D3DHAL_CLIPPEDTRIANGLEFAN  *LPD3DHAL_CLIPPEDTRIANGLEFAN;

typedef struct _D3DHAL_DP2DRAWPRIMITIVE2
{
    D3DPRIMITIVETYPE primType;
    DWORD FirstVertexOffset;            // Offset in bytes in the stream 0
    DWORD PrimitiveCount;
} D3DHAL_DP2DRAWPRIMITIVE2;
typedef D3DHAL_DP2DRAWPRIMITIVE2  *LPD3DHAL_DP2DRAWPRIMITIVE2;

typedef struct _D3DHAL_DP2DRAWINDEXEDPRIMITIVE2
{
    D3DPRIMITIVETYPE primType;
    INT   BaseVertexOffset;     // Stream 0 offset of the vertex which
                                // corresponds to index 0. This offset could be
                                // negative, but when an index is added to the
                                // offset the result is positive
    DWORD MinIndex;             // Min vertex index in the vertex buffer
    DWORD NumVertices;          // Number of vertices starting from MinIndex
    DWORD StartIndexOffset;     // Offset of the start index in the index buffer
    DWORD PrimitiveCount;       // Number of triangles (points, lines)
} D3DHAL_DP2DRAWINDEXEDPRIMITIVE2;
typedef D3DHAL_DP2DRAWINDEXEDPRIMITIVE2  *LPD3DHAL_DP2DRAWINDEXEDPRIMITIVE2;

// Used by SetPixelShader and DeletePixelShader
typedef struct _D3DHAL_DP2PIXELSHADER
{
    // Pixel shader handle.
    // The handle could be 0, meaning that the current pixel shader is invalid
    // (not set).
    DWORD dwHandle;
} D3DHAL_DP2PIXELSHADER;
typedef D3DHAL_DP2PIXELSHADER  *LPD3DHAL_DP2PIXELSHADER;

typedef struct _D3DHAL_DP2CREATEPIXELSHADER
{
    DWORD dwHandle;     // Shader handle
    DWORD dwCodeSize;   // Shader code size in bytes
    // Shader code follows
} D3DHAL_DP2CREATEPIXELSHADER;
typedef D3DHAL_DP2CREATEPIXELSHADER  *LPD3DHAL_DP2CREATEPIXELSHADER;

typedef struct _D3DHAL_DP2SETPIXELSHADERCONST
{
    DWORD dwRegister;   // Const register to start copying
    DWORD dwCount;      // Number of 4-float vectors to copy for D3DDP2OP_SETPIXELSHADERCONST
                        // Number of 4-integer vectors to copy for D3DDP2OP_SETPIXELSHADERCONSTI
                        // Number of BOOL values to copy for D3DDP2OP_SETPIXELSHADERCONSTB
    // Data follows
} D3DHAL_DP2SETPIXELSHADERCONST;
typedef D3DHAL_DP2SETPIXELSHADERCONST  *LPD3DHAL_DP2SETPIXELSHADERCONST;

// Flags that can be supplied to DRAWRECTPATCH and DRAWTRIPATCH
#define RTPATCHFLAG_HASSEGS  0x00000001L
#define RTPATCHFLAG_HASINFO  0x00000002L

typedef struct _D3DHAL_DP2DRAWRECTPATCH
{
    DWORD Handle;
    DWORD Flags;
    // Optionally followed by D3DFLOAT[4] NumSegments and/or D3DRECTPATCH_INFO
} D3DHAL_DP2DRAWRECTPATCH;
typedef D3DHAL_DP2DRAWRECTPATCH  *LPD3DHAL_DP2DRAWRECTPATCH;

typedef struct _D3DHAL_DP2DRAWTRIPATCH
{
    DWORD Handle;
    DWORD Flags;
    // Optionally followed by D3DFLOAT[3] NumSegments and/or D3DTRIPATCH_INFO
} D3DHAL_DP2DRAWTRIPATCH;
typedef D3DHAL_DP2DRAWTRIPATCH  *LPD3DHAL_DP2DRAWTRIPATCH;

typedef struct _D3DHAL_DP2VOLUMEBLT
{
    DWORD   dwDDDestSurface;// dest surface
    DWORD   dwDDSrcSurface; // src surface
    DWORD   dwDestX;        // dest X (width)
    DWORD   dwDestY;        // dest Y (height)
    DWORD   dwDestZ;        // dest Z (depth)
    D3DBOX  srcBox;         // src box
    DWORD   dwFlags;        // blt flags
} D3DHAL_DP2VOLUMEBLT;
typedef D3DHAL_DP2VOLUMEBLT  *LPD3DHAL_DP2VOLUMEBLT;

typedef struct _D3DHAL_DP2BUFFERBLT
{
    DWORD     dwDDDestSurface; // dest surface
    DWORD     dwDDSrcSurface;  // src surface
    DWORD     dwOffset;        // Offset in the dest surface (in BYTES)
    D3DRANGE  rSrc;            // src range
    DWORD     dwFlags;         // blt flags
} D3DHAL_DP2BUFFERBLT;
typedef D3DHAL_DP2BUFFERBLT  *LPD3DHAL_DP2BUFFERBLT;

typedef struct _D3DHAL_DP2ADDDIRTYRECT
{
    DWORD     dwSurface;      // Driver managed surface
    RECTL     rDirtyArea;     // Area marked dirty
} D3DHAL_DP2ADDDIRTYRECT;
typedef D3DHAL_DP2ADDDIRTYRECT  *LPD3DHAL_DP2ADDDIRTYRECT;

typedef struct _D3DHAL_DP2ADDDIRTYBOX
{
    DWORD     dwSurface;      // Driver managed volume
    D3DBOX    DirtyBox;       // Box marked dirty
} D3DHAL_DP2ADDDIRTYBOX;
typedef D3DHAL_DP2ADDDIRTYBOX  *LPD3DHAL_DP2ADDDIRTYBOX;

#endif /* DIRECT3D_VERSION >= 0x0800 */

#if(DIRECT3D_VERSION >= 0x0900)

typedef struct _D3DHAL_DP2CREATEVERTEXSHADERDECL
{
    DWORD dwHandle;                         // Shader function handle
    DWORD dwNumVertexElements;              // Number of vertex elements
    // D3DVERTEXELEMENT9 VertexElements[];   Vertex elements follow
} D3DHAL_DP2CREATEVERTEXSHADERDECL ;
typedef D3DHAL_DP2CREATEVERTEXSHADERDECL  *LPD3DHAL_DP2CREATEVERTEXSHADERDECL;

typedef struct _D3DHAL_DP2CREATEVERTEXSHADERFUNC
{
    DWORD dwHandle;     // Shader function handle
    DWORD dwSize;       // Shader function size in bytes
    // Shader declaration follows
} D3DHAL_DP2CREATEVERTEXSHADERFUNC ;
typedef D3DHAL_DP2CREATEVERTEXSHADERFUNC  *LPD3DHAL_DP2CREATEVERTEXSHADERFUNC;

typedef struct _D3DHAL_DP2SETSTREAMSOURCE2
{
    DWORD dwStream;     // Stream index, starting from zero
    DWORD dwVBHandle;   // Vertex buffer handle
    DWORD dwOffset;     // Offset of the first vertex size in bytes
    DWORD dwStride;     // Vertex size in bytes
} D3DHAL_DP2SETSTREAMSOURCE2;
typedef D3DHAL_DP2SETSTREAMSOURCE2  *LPD3DHAL_DP2SETSTREAMSOURCE2;

typedef struct _D3DHAL_DP2SETSTREAMSOURCEFREQ
{
    DWORD dwStream;     // Stream index, starting from zero
    DWORD dwDivider;    // Stream source divider
} D3DHAL_DP2SETSTREAMSOURCEFREQ;
typedef D3DHAL_DP2SETSTREAMSOURCEFREQ *LPD3DHAL_DP2SETSTREAMSOURCEFREQ;

#define D3DHAL_ROW_WEIGHTS  1
#define D3DHAL_COL_WEIGHTS  2
typedef struct _D3DHAL_DP2SETCONVOLUTIONKERNELMONO
{
    DWORD dwWidth;     // Kernel width
    DWORD dwHeight;    // Kernel height
    DWORD dwFlags;
    // If dwFlags & D3DHAL_ROW_WEIGHTS, then width floats follow. Otherwise row weights are 1.0.
    // If dwFlags & D3DHAL_COL_WEIGHTS, then height floats follow. Otherwise column weights are 1.0.
} D3DHAL_DP2SETCONVOLUTIONKERNELMONO;
typedef D3DHAL_DP2SETCONVOLUTIONKERNELMONO *LPD3DHAL_DP2SETCONVOLUTIONKERNELMONO;

typedef struct _D3DHAL_DP2COMPOSERECTS
{
    DWORD               SrcSurfaceHandle;
    DWORD               DstSurfaceHandle;
    DWORD               SrcRectDescsVBHandle;
    UINT                NumRects;
    DWORD               DstRectDescsVBHandle;
    D3DCOMPOSERECTSOP   Operation;
    INT                 XOffset;
    INT                 YOffset;
} D3DHAL_DP2COMPOSERECTS;
typedef D3DHAL_DP2COMPOSERECTS *LPD3DHAL_DP2COMPOSERECTS;

typedef RECT D3DHAL_DP2SETSCISSORRECT;
typedef D3DHAL_DP2SETSCISSORRECT *LPD3DHAL_DP2SETSCISSORRECT;

typedef struct _D3DHAL_DP2BLT
{
    DWORD dwSource;         // Source surface
    RECTL rSource;          // Source rectangle
    DWORD dwSourceMipLevel; // Miplevel of lightweight surface
    DWORD dwDest;           // Dest surface
    RECTL rDest;            // Dest rectangle
    DWORD dwDestMipLevel;   // Miplevel of lightweight surface
    DWORD Flags;            // Can be DP2BLT_POINT, DP2BLT_LINEAR
} D3DHAL_DP2BLT;
typedef D3DHAL_DP2BLT  *LPD3DHAL_DP2BLT;

#define DP2BLT_POINT    0x00000001L
#define DP2BLT_LINEAR   0x00000002L

typedef struct _D3DHAL_DP2COLORFILL
{
    DWORD    dwSurface; // Surface getting filled
    RECTL    rRect;     // Surface dimensions to fill
    D3DCOLOR Color;     // A8R8G8B8 fill color
} D3DHAL_DP2COLORFILL;
typedef D3DHAL_DP2COLORFILL  *LPD3DHAL_DP2COLORFILL;

typedef struct _D3DHAL_DP2SURFACEBLT
{
    DWORD dwSource;         // Source surface
    RECTL rSource;          // Source rectangle
    DWORD dwSourceMipLevel; // Miplevel of lightweight surface
    DWORD dwDest;           // Dest surface
    RECTL rDest;            // Dest rectangle
    DWORD dwDestMipLevel;   // Miplevel of lightweight surface
    DWORD Flags;            // No flags currently defined
} D3DHAL_DP2SURFACEBLT;
typedef D3DHAL_DP2SURFACEBLT  *LPD3DHAL_DP2SURFACEBLT;

typedef D3DHAL_DP2SETVERTEXSHADERCONST D3DHAL_DP2SETVERTEXSHADERCONSTI;
typedef D3DHAL_DP2SETVERTEXSHADERCONST  *LPD3DHAL_DP2SETVERTEXSHADERCONSTI;
typedef D3DHAL_DP2SETVERTEXSHADERCONST D3DHAL_DP2SETVERTEXSHADERCONSTB;
typedef D3DHAL_DP2SETVERTEXSHADERCONSTB  *LPD3DHAL_DP2SETVERTEXSHADERCONSTB;

typedef D3DHAL_DP2SETPIXELSHADERCONST D3DHAL_DP2SETPIXELSHADERCONSTI;
typedef D3DHAL_DP2SETPIXELSHADERCONST  *LPD3DHAL_DP2SETPIXELSHADERCONSTI;
typedef D3DHAL_DP2SETPIXELSHADERCONST D3DHAL_DP2SETPIXELSHADERCONSTB;
typedef D3DHAL_DP2SETPIXELSHADERCONSTB  *LPD3DHAL_DP2SETPIXELSHADERCONSTB;

typedef struct _D3DHAL_DP2CREATEQUERY
{
    DWORD dwQueryID;
    D3DQUERYTYPE QueryType;
} D3DHAL_DP2CREATEQUERY;
typedef D3DHAL_DP2CREATEQUERY  *LPD3DHAL_DP2CREATEQUERY;

typedef struct _D3DHAL_DP2DELETEQUERY
{
    DWORD dwQueryID;
} D3DHAL_DP2DELETEQUERY;
typedef D3DHAL_DP2DELETEQUERY  *LPD3DHAL_DP2DELETEQUERY;

typedef struct _D3DHAL_DP2ISSUEQUERY
{
    DWORD dwQueryID;
    DWORD dwFlags;
} D3DHAL_DP2ISSUEQUERY;
typedef D3DHAL_DP2ISSUEQUERY  *LPD3DHAL_DP2ISSUEQUERY;

typedef struct _D3DHAL_DP2SETRENDERTARGET2
{
    DWORD RTIndex;
    DWORD hRenderTarget;
} D3DHAL_DP2SETRENDERTARGET2;
typedef D3DHAL_DP2SETRENDERTARGET2  *LPD3DHAL_DP2SETRENDERTARGET2;

typedef struct _D3DHAL_DP2SETDEPTHSTENCIL
{
    DWORD hZBuffer;
} D3DHAL_DP2SETDEPTHSTENCIL;
typedef D3DHAL_DP2SETDEPTHSTENCIL  *LPD3DHAL_DP2SETDEPTHSTENCIL;

typedef struct _D3DHAL_DP2GENERATEMIPSUBLEVELS
{
    DWORD                hSurface;
    D3DTEXTUREFILTERTYPE Filter;
} D3DHAL_DP2GENERATEMIPSUBLEVELS;
typedef D3DHAL_DP2GENERATEMIPSUBLEVELS *LPD3DHAL_DP2GENERATEMIPSUBLEVELS;

//
// Command structure for driver responses:
//

typedef struct _D3DHAL_DP2RESPONSE
{
    BYTE bCommand;     /* response/ command id */
    BYTE bReserved;
    WORD wStateCount;  /* count of responses to follow */
    DWORD dwTotalSize; /* total size of response (including the DP2REPONSE struct) to enable skipping over. */
} D3DHAL_DP2RESPONSE, *LPD3DHAL_DP2RESPONSE;

/* Begin Responses */
typedef struct _D3DHAL_DP2RESPONSEQUERY
{
    DWORD dwQueryID;
    DWORD dwSize;
} D3DHAL_DP2RESPONSEQUERY;
typedef D3DHAL_DP2RESPONSEQUERY  *LPD3DHAL_DP2RESPONSEQUERY;
/* End Responses */

#endif /* DIRECT3D_VERSION >= 0x0900 */

typedef struct _D3DHAL_DRAWPRIMITIVES2DATA {
    ULONG_PTR             dwhContext;           // in: Context handle
    DWORD             dwFlags;              // in: flags
    DWORD             dwVertexType;         // in: vertex type
    LPDDRAWI_DDRAWSURFACE_LCL lpDDCommands; // in: vertex buffer command data
    DWORD             dwCommandOffset;      // in: offset to start of vertex buffer commands
    DWORD             dwCommandLength;      // in: number of bytes of command data
    union
    { // based on D3DHALDP2_USERMEMVERTICES flag
       LPDDRAWI_DDRAWSURFACE_LCL lpDDVertex;// in: surface containing vertex data
       LPVOID lpVertices;                   // in: User mode pointer to vertices
    };
    DWORD             dwVertexOffset;       // in: offset to start of vertex data
    DWORD             dwVertexLength;       // in: number of vertices of vertex data
    DWORD             dwReqVertexBufSize;   // in: number of bytes required for the next vertex buffer
    DWORD             dwReqCommandBufSize;  // in: number of bytes required for the next commnand buffer
    LPDWORD           lpdwRStates;          // in: Pointer to the array where render states are updated
    union
    {
       DWORD          dwVertexSize;         // in: Size of each vertex in bytes
       HRESULT        ddrval;               // out: return value
    };
    DWORD             dwErrorOffset;        // out: offset in lpDDCommands to
                                            //      first D3DHAL_COMMAND not handled
} D3DHAL_DRAWPRIMITIVES2DATA;
typedef D3DHAL_DRAWPRIMITIVES2DATA  *LPD3DHAL_DRAWPRIMITIVES2DATA;

// Macros to access shader binary code
#define D3DSI_GETREGNUM(token)  (token & D3DSP_REGNUM_MASK)
#define D3DSI_GETOPCODE(command) (command & D3DSI_OPCODE_MASK)
#define D3DSI_GETWRITEMASK(token) (token & D3DSP_WRITEMASK_ALL)
#define D3DVS_GETSWIZZLECOMP(source, component)  (source >> ((component << 1) + 16) & 0x3)
#define D3DVS_GETSWIZZLE(token)  (token & D3DVS_SWIZZLE_MASK)
#define D3DVS_GETSRCMODIFIER(token) (token & D3DSP_SRCMOD_MASK)
#define D3DVS_GETADDRESSMODE(token) (token & D3DVS_ADDRESSMODE_MASK)

#if(DIRECT3D_VERSION < 0x0900)

#define D3DSI_GETREGTYPE(token) ((D3DSHADER_PARAM_REGISTER_TYPE)(token & D3DSP_REGTYPE_MASK))

#else

#define D3DSI_GETREGTYPE(token) ((D3DSHADER_PARAM_REGISTER_TYPE)(((token & D3DSP_REGTYPE_MASK) >> D3DSP_REGTYPE_SHIFT) | \
                                 ((token & D3DSP_REGTYPE_MASK2) >> D3DSP_REGTYPE_SHIFT2)))
#define D3DSI_GETUSAGE(token) ((token & D3DSP_DCL_USAGE_MASK) >> D3DSP_DCL_USAGE_SHIFT)
#define D3DSI_GETUSAGEINDEX(token) ((token & D3DSP_DCL_USAGEINDEX_MASK) >> D3DSP_DCL_USAGEINDEX_SHIFT)
#define D3DSI_GETINSTLENGTH(token) ((token & D3DSI_INSTLENGTH_MASK) >> D3DSI_INSTLENGTH_SHIFT)
#define D3DSI_GETCOMPARISON(token) ((D3DSHADER_COMPARISON)((token & D3DSHADER_COMPARISON_MASK) >> D3DSHADER_COMPARISON_SHIFT))
#define D3DSI_GETREGISTERPROPERTIES(token) (token & D3DSP_REGISTERPROPERTIES_MASK)
#define D3DSI_GETTEXTURETYPE(token) (token & D3DSP_TEXTURETYPE_MASK)
#define D3DSI_GETDSTMODIFIER(token) (token & D3DSP_DSTMOD_MASK)
#define D3DSI_GETSWIZZLECOMP(source, component)  (source >> ((component << 1) + 16) & 0x3)
#define D3DSI_GETSWIZZLE(token)  (token & D3DVS_SWIZZLE_MASK)
#define D3DSI_GETSRCMODIFIER(token) (token & D3DSP_SRCMOD_MASK)
#define D3DSI_GETADDRESSMODE(token) (token & D3DVS_ADDRESSMODE_MASK)

#ifdef __cplusplus
// This gets regtype, and also maps D3DSPR_CONSTn to D3DSPR_CONST (for easier parsing)
inline D3DSHADER_PARAM_REGISTER_TYPE D3DSI_GETREGTYPE_RESOLVING_CONSTANTS(DWORD token)
{
    D3DSHADER_PARAM_REGISTER_TYPE RegType = D3DSI_GETREGTYPE(token);
    switch(RegType)
    {
    case D3DSPR_CONST4:
    case D3DSPR_CONST3:
    case D3DSPR_CONST2:
        return D3DSPR_CONST;
    default:
        return RegType;
    }
}

// The inline function below retrieves register number for an opcode,
// taking into account that: if the type is a
// D3DSPR_CONSTn, the register number needs to be remapped.
//
//           D3DSPR_CONST is for c0-c2047
//           D3DSPR_CONST2 is for c2048-c4095
//           D3DSPR_CONST3 is for c4096-c6143
//           D3DSPR_CONST4 is for c6144-c8191
//
// For example if the instruction token specifies type D3DSPR_CONST4, reg# 3,
// the register number retrieved is 6147.
// For other register types, the register number is just returned unchanged.
inline UINT D3DSI_GETREGNUM_RESOLVING_CONSTANTS(DWORD token)
{
    D3DSHADER_PARAM_REGISTER_TYPE RegType = D3DSI_GETREGTYPE(token);
    UINT RegNum = D3DSI_GETREGNUM(token);
    switch(RegType)
    {
    case D3DSPR_CONST4:
        return RegNum + 6144;
    case D3DSPR_CONST3:
        return RegNum + 4096;
    case D3DSPR_CONST2:
        return RegNum + 2048;
    default:
        return RegNum;
    }
}
#endif // __cplusplus

#endif

// Indicates that the lpVertices field in the DrawPrimitives2 data is
// valid, i.e. user allocated memory.
#define D3DHALDP2_USERMEMVERTICES   0x00000001L
// Indicates that the command buffer and vertex buffer are a system memory execute buffer
// resulting from the use of the Execute buffer API.
#define D3DHALDP2_EXECUTEBUFFER     0x00000002L
// The swap flags indicate if it is OK for the driver to swap the submitted buffers with new
// buffers and asyncronously work on the submitted buffers.
#define D3DHALDP2_SWAPVERTEXBUFFER  0x00000004L
#define D3DHALDP2_SWAPCOMMANDBUFFER 0x00000008L
// The requested flags are present if the new buffers which the driver can allocate need to be
// of atleast a given size. If any of these flags are set, the corresponding dwReq* field in
// D3DHAL_DRAWPRIMITIVES2DATA will also be set with the requested size in bytes.
#define D3DHALDP2_REQVERTEXBUFSIZE  0x00000010L
#define D3DHALDP2_REQCOMMANDBUFSIZE 0x00000020L
// These flags are set by the driver upon return from DrawPrimitives2 indicating if the new
// buffers are not in system memory.
#define D3DHALDP2_VIDMEMVERTEXBUF   0x00000040L
#define D3DHALDP2_VIDMEMCOMMANDBUF  0x00000080L

// Used by the driver to ask runtime to parse the execute buffer
#define D3DERR_COMMAND_UNPARSED              MAKE_DDHRESULT(3000)

typedef DWORD (CALLBACK *LPD3DHAL_CLEAR2CB)        (LPD3DHAL_CLEAR2DATA);
typedef DWORD (CALLBACK *LPD3DHAL_VALIDATETEXTURESTAGESTATECB)(LPD3DHAL_VALIDATETEXTURESTAGESTATEDATA);
typedef DWORD (CALLBACK *LPD3DHAL_DRAWPRIMITIVES2CB)  (LPD3DHAL_DRAWPRIMITIVES2DATA);

typedef struct _D3DHAL_CALLBACKS3
{
    DWORD   dwSize;         // size of struct
    DWORD   dwFlags;        // flags for callbacks
    LPD3DHAL_CLEAR2CB                       Clear2;
    LPVOID                                  lpvReserved;
    LPD3DHAL_VALIDATETEXTURESTAGESTATECB    ValidateTextureStageState;
    LPD3DHAL_DRAWPRIMITIVES2CB              DrawPrimitives2;
} D3DHAL_CALLBACKS3;
typedef D3DHAL_CALLBACKS3 *LPD3DHAL_CALLBACKS3;
#define D3DHAL_CALLBACKS3SIZE       sizeof(D3DHAL_CALLBACKS3)

//  bit definitions for D3DHAL
#define D3DHAL3_CB32_CLEAR2                      0x00000001L
#define D3DHAL3_CB32_RESERVED                    0x00000002L
#define D3DHAL3_CB32_VALIDATETEXTURESTAGESTATE   0x00000004L
#define D3DHAL3_CB32_DRAWPRIMITIVES2             0x00000008L

/* --------------------------------------------------------------
 * Texture stage renderstate mapping definitions.
 *
 * 256 renderstate slots [256, 511] are reserved for texture processing
 * stage controls, which provides for 8 texture processing stages each
 * with 32 DWORD controls.
 *
 * The renderstates within each stage are indexed by the
 * D3DTEXTURESTAGESTATETYPE enumerants by adding the appropriate
 * enumerant to the base for a given texture stage.
 *
 * Note, "state overrides" bias the renderstate by 256, so the two
 * ranges overlap.  Overrides are enabled for exebufs only, so all
 * this means is that Texture3 cannot be used with exebufs.
 */

/*
 * Base of all texture stage state values in renderstate array.
 */
#define D3DHAL_TSS_RENDERSTATEBASE 256UL

/*
 * Maximum number of stages allowed.
 */
#define D3DHAL_TSS_MAXSTAGES 8

/*
 * Number of state DWORDS per stage.
 */
#define D3DHAL_TSS_STATESPERSTAGE 64

/*
 * Texture handle's offset into the 32-DWORD cascade state vector
 */
#define D3DTSS_TEXTUREMAP 0


#if(DIRECT3D_VERSION >= 0x0900)
/* --------------------------------------------------------------
 * Texture sampler renderstate.
 *
 * D3DSAMPLERSTATETYPE (D3DSAMP_*) sampler states exist to
 * separate sampler state from the rest of the D3DTSS_* states.
 * D3DSAMP_* states are only visible at the API level;
 * the runtime simply maps these to D3DTSS_* for drivers.
 *
 */

/*
 * Maximum number of texture samplers allowed.
 *
 * If this number gets bigger than 32, some retooling
 * will be needed, as DWORD bitfields are used all over the place.
 */
#define D3DHAL_SAMPLER_MAXSAMP          16

/*
 * Maximum number of samplers in vertex shaders (must be power of 2)
 */
#define D3DHAL_SAMPLER_MAXVERTEXSAMP     4

/*
 * Number of state DWORDS per sampler.
 */
#define D3DHAL_SAMPLER_STATESPERSAMP    D3DSAMP_MAX


/*
 * D3DTSS_* states that have been removed from the D3DTEXTURESTAGESTATETYPE
 * and turned into the D3DSAMP_* enum D3DTEXTURESAMPLERTYPE.
 * These defines allow D3DSAMP_* to be mapped to D3DTSS_* through the DDI
 * so that drivers can simply understand D3DTSS_* and do not have to know
 * about D3DSAMP_* at all.
 * These defines are now labelled as D3DTSS_RESERVEDn in the public
 * header definition of D3DTEXTURESTAGESTATETYPE.
 */
#define     D3DTSS_ADDRESSU        ((D3DTEXTURESTAGESTATETYPE)13)
#define     D3DTSS_ADDRESSV        ((D3DTEXTURESTAGESTATETYPE)14)
#define     D3DTSS_BORDERCOLOR     ((D3DTEXTURESTAGESTATETYPE)15)
#define     D3DTSS_MAGFILTER       ((D3DTEXTURESTAGESTATETYPE)16)
#define     D3DTSS_MINFILTER       ((D3DTEXTURESTAGESTATETYPE)17)
#define     D3DTSS_MIPFILTER       ((D3DTEXTURESTAGESTATETYPE)18)
#define     D3DTSS_MIPMAPLODBIAS   ((D3DTEXTURESTAGESTATETYPE)19)
#define     D3DTSS_MAXMIPLEVEL     ((D3DTEXTURESTAGESTATETYPE)20)
#define     D3DTSS_MAXANISOTROPY   ((D3DTEXTURESTAGESTATETYPE)21)
#define     D3DTSS_ADDRESSW        ((D3DTEXTURESTAGESTATETYPE)25)
#define     D3DTSS_SRGBTEXTURE     ((D3DTEXTURESTAGESTATETYPE)29)
#define     D3DTSS_ELEMENTINDEX    ((D3DTEXTURESTAGESTATETYPE)30)
#define     D3DTSS_DMAPOFFSET      ((D3DTEXTURESTAGESTATETYPE)31)

// These renderstates were retired in DX8:
#ifndef D3DRS_SOFTWAREVERTEXPROCESSING
#define D3DRS_SOFTWAREVERTEXPROCESSING ((D3DRENDERSTATETYPE)153)
#endif

// These renderstates were retired in DX9:
#ifndef D3DRS_PATCHSEGMENTS
#define D3DRS_LINEPATTERN   ((D3DRENDERSTATETYPE)10)
#define D3DRS_ZVISIBLE      ((D3DRENDERSTATETYPE)30)
#define D3DRS_PATCHSEGMENTS ((D3DRENDERSTATETYPE)164)
#endif

#endif /* DIRECT3D_VERSION >= 0x0900 */

/* --------------------------------------------------------------
 * Flags for the data parameters.
 */

/*
 * SceneCapture()
 * This is used as an indication to the driver that a scene is about to
 * start or end, and that it should capture data if required.
 */
#define D3DHAL_SCENE_CAPTURE_START  0x00000000L
#define D3DHAL_SCENE_CAPTURE_END    0x00000001L

/*
 * Execute()
 */

/*
 * Use the instruction stream starting at dwOffset.
 */
#define D3DHAL_EXECUTE_NORMAL       0x00000000L

/*
 * Use the optional instruction override (diInstruction) and return
 * after completion.  dwOffset is the offset to the first primitive.
 */
#define D3DHAL_EXECUTE_OVERRIDE     0x00000001L

/*
 * GetState()
 * The driver will get passed a flag in dwWhich specifying which module
 * the state must come from.  The driver then fills in ulArg[1] with the
 * appropriate value depending on the state type given in ddState.
 */

/*
 * The following are used to get the state of a particular stage of the
 * pipeline.
 */
#define D3DHALSTATE_GET_TRANSFORM   0x00000001L
#define D3DHALSTATE_GET_LIGHT       0x00000002L
#define D3DHALSTATE_GET_RENDER      0x00000004L


/* --------------------------------------------------------------
 * Return values from HAL functions.
 */

/*
 * The context passed in was bad.
 */
#define D3DHAL_CONTEXT_BAD      0x000000200L

/*
 * No more contexts left.
 */
#define D3DHAL_OUTOFCONTEXTS        0x000000201L

/*
 * Execute() and ExecuteClipped()
 */

/*
 * Executed to completion via early out.
 *  (e.g. totally clipped)
 */
#define D3DHAL_EXECUTE_ABORT        0x00000210L

/*
 * An unhandled instruction code was found (e.g. D3DOP_TRANSFORM).
 * The dwOffset parameter must be set to the offset of the unhandled
 * instruction.
 *
 * Only valid from Execute()
 */
#define D3DHAL_EXECUTE_UNHANDLED    0x00000211L

// typedef for the Callback that the drivers can use to parse unknown commands
// passed to them via the DrawPrimitives2 callback. The driver obtains this
// callback thru a GetDriverInfo call with GUID_D3DParseUnknownCommandCallback
// made by ddraw somewhere around the initialization time.
typedef HRESULT (CALLBACK *PFND3DPARSEUNKNOWNCOMMAND) (LPVOID lpvCommands,
                                         LPVOID *lplpvReturnedCommand);


/*
 * DDI only renderstates.
 */
#define D3DRENDERSTATE_EVICTMANAGEDTEXTURES 61  // DDI render state only to Evict textures
#define D3DRENDERSTATE_SCENECAPTURE     62      // DDI only to replace SceneCapture
#define D3DRS_DELETERTPATCH       169     // DDI only to delete high order patch
#define D3DRS_MAXVERTEXSHADERINST ((D3DRENDERSTATETYPE)196) // DDI only: vs_3_0+ num instructions to execute.
#define D3DRS_MAXPIXELSHADERINST  ((D3DRENDERSTATETYPE)197) // DDI only: ps_3_0+ num instructions to execute.
#define D3DRS_ZBIAS               ((D3DRENDERSTATETYPE)47) // replaced by depthbias

// Default values for D3DRS_MAXVERTEXSHADERINST and D3DRS_MAXPIXELSHADERINST
#define D3DINFINITEINSTRUCTIONS 0xffffffff

//-----------------------------------------------------------------------------
//
// DirectX 8.0's new driver info querying mechanism.
//
// How to handle the new driver info query mechanism.
//
// DirectX 8.0 utilizes an extension to GetDriverInfo() to query for
// additional information from the driver. Currently this mechanism is only
// used for querying for DX8 style D3D caps but it may be used for other
// information over time.
//
// This extension to GetDriverInfo takes the form of a GetDriverInfo call
// with the GUID GUID_GetDriverInfo2. When a GetDriverInfo call with this
// GUID is received by the driver the driver must check the data passed
// in the lpvData field of the DD_GETDRIVERINFODATA data structure to see
// what information is being requested.
//
// It is important to note that the GUID GUID_GetDriverInfo2 is, in fact,
// the same as the GUID_DDStereoMode. If you driver doesn't handle
// GUID_DDStereoMode this is not an issue. However, if you wish your driver
// to handle GUID_DDStereoMode as well as GUID_GetDriverInfo2 special action
// must be taken. When a call tp GetDriverInfo with the GUID
// GUID_GetDriverInfo2/GUID_DDStereoMode is made the runtime sets the
// dwHeight field of the DD_STEREOMODE structure to the special value
// D3DGDI2_MAGIC. In this way you can determine when the request is a
// stereo mode call or a GetDriverInfo2 call. The dwHeight field of
// DD_STEREOMODE corresponds to the dwMagic field of the
// DD_GETDRIVERINFO2DATA structure.
//
// The dwExpectedSize field of the DD_GETDRIVERINFODATA structure is not
// used by when a GetDriverInfo2 request is being made and should be
// ignored. The actual expected size of the data is found in the
// dwExpectedSize of the DD_GETDRIVERINFO2DATA structure.
//
// Once the driver has determined that this is a call to
// GetDriverInfo2 it must then determine the type of information being
// requested by the runtime. This type is contained in the dwType field
// of the DD_GETDRIVERINFO2DATA data structure.
//
// Finally, once the driver knows this is a GetDriverInfo2 request of a
// particular type it can copy the requested data into the data buffer.
// It is important to note that the lpvData field of the DD_GETDRIVERINFODATA
// data structure points to data buffer in which to copy your data. lpvData
// also points to the DD_GETDRIVERINFO2DATA structure. This means that the
// data returned by the driver will overwrite the DD_GETDRIVERINFO2DATA
// structure and, hence, the DD_GETDRIVERINFO2DATA structure occupies the
// first few DWORDs of the buffer.
//
// The following code fragment demonstrates how to handle GetDriverInfo2.
//
// D3DCAPS8 myD3DCaps8;
//
// DWORD CALLBACK
// DdGetDriverInfo(LPDDHAL_GETDRIVERINFODATA lpData)
// {
//     if (MATCH_GUID((lpData->guidInfo), GUID_GetDriverInfo2) )
//     {
//         ASSERT(NULL != lpData);
//         ASSERT(NULL != lpData->lpvData);
//
//         // Is this a call to GetDriverInfo2 or DDStereoMode?
//         if (((DD_GETDRIVERINFO2DATA*)(lpData->lpvData))->dwMagic == D3DGDI2_MAGIC)
//         {
//             // Yes, its a call to GetDriverInfo2, fetch the
//             // DD_GETDRIVERINFO2DATA data structure.
//             DD_GETDRIVERINFO2DATA* pgdi2 = lpData->lpvData;
//             ASSERT(NULL != pgdi2);
//
//             // What type of request is this?
//             switch (pgdi2->dwType)
//             {
//             case D3DGDI2_TYPE_GETD3DCAPS8:
//                 {
//                     // The runtime is requesting the DX8 D3D caps so
//                     // copy them over now.
//
//                     // It should be noted that the dwExpectedSize field
//                     // of DD_GETDRIVERINFODATA is not used for
//                     // GetDriverInfo2 calls and should be ignored.
//                     size_t copySize = min(sizeof(myD3DCaps8), pgdi2->dwExpectedSize);
//                     memcpy(lpData->lpvData, &myD3DCaps8, copySize);
//                     lpData->dwActualSize = copySize;
//                     lpData->ddRVal       = DD_OK;
//                     return DDHAL_DRIVER_HANDLED;
//                 }
//             default:
//                 // For any other GetDriverInfo2 types not handled
//                 // or understood by the driver set an ddRVal of
//                 // DDERR_CURRENTLYNOTAVAIL and return
//                 // DDHAL_DRIVER_HANDLED.
//                 lpData->dwActualSize = 0;
//                 lpData->ddRVal       = DDERR_CURRENTLYNOTAVAIL;
//                 return DDHAL_DRIVER_HANDLED;
//             }
//         }
//         else
//         {
//             // It must be a call a request for stereo mode support.
//             // Fetch the stereo mode data
//             DD_STEREOMODE* pStereoMode = lpData->lpvData;
//             ASSERT(NULL != pStereoMode);
//
//             // Process the stereo mode request...
//             lpData->dwActualSize = sizeof(DD_STEREOMODE);
//             lpData->ddRVal       = DD_OK;
//             return DDHAL_DRIVER_HANDLED;
//         }
//     }
//
//     // Handle any other device GUIDs...
//
// } // DdGetDriverInfo
//
//-----------------------------------------------------------------------------

//
// The data structure which is passed to the driver when GetDriverInfo is
// called with a GUID of GUID_GetDriverInfo2.
//
// NOTE: Although the fields listed below are all read only this data
// structure is actually the first four DWORDs of the data buffer into
// which the driver writes the requested infomation. As such, these fields
// (and the entire data structure) are overwritten by the data returned by
// the driver.
//
typedef struct _DD_GETDRIVERINFO2DATA
{
    DWORD       dwReserved;     // Reserved Field.
                                // Driver should not read or write this field.

    DWORD       dwMagic;        // Magic Number. Has the value D3DGDI2_MAGIC if
                                // this is a GetDriverInfo2 call. Otherwise
                                // this structure is, in fact, a DD_STEREOMODE
                                // call.
                                // Driver should only read this field.

    DWORD       dwType;         // Type of information requested. This field
                                // contains one of the DDGDI2_TYPE_ #defines
                                // listed below.
                                // Driver should only read (not write) this
                                // field.

    DWORD       dwExpectedSize; // Expected size of the information requested.
                                // Driver should only read (not write) this
                                // field.

    // The remainder of the data buffer (beyond the first four DWORDs)
    // follows here.
} DD_GETDRIVERINFO2DATA;

//
// IMPORTANT NOTE: This GUID has exactly the same value as GUID_DDStereoMode
// and as such you must be very careful when using it. If your driver needs
// to handle both GetDriverInfo2 and DDStereoMode it must have a single
// check for the shared GUID and then distinguish between which use of that
// GUID is being requested.
//
#define GUID_GetDriverInfo2 (GUID_DDStereoMode)

//
// Magic value used to determine whether a GetDriverInfo call with the
// GUID GUID_GetDriverInfo2/GUID_DDStereoMode is a GetDriverInfo2 request
// or a query about stereo capabilities. This magic number is stored in
// the dwHeight field of the DD_STEREOMODE data structure.
//
#define D3DGDI2_MAGIC       (0xFFFFFFFFul)

//
// The types of information which can be requested from the driver via
// GetDriverInfo2.
//

#define D3DGDI2_TYPE_GETD3DCAPS8            (0x00000001ul) // Return the D3DCAPS8 data
#define D3DGDI2_TYPE_GETFORMATCOUNT         (0x00000002ul) // Return the number of supported formats
#define D3DGDI2_TYPE_GETFORMAT              (0x00000003ul) // Return a particular format
#define D3DGDI2_TYPE_DXVERSION              (0x00000004ul) // Notify driver of current DX Version
#define D3DGDI2_TYPE_GETD3DCAPS9            (0x00000010ul) // Return the D3DCAPS9 data
#define D3DGDI2_TYPE_GETEXTENDEDMODECOUNT   (0x00000011ul) // Return the number of supported extended mode
#define D3DGDI2_TYPE_GETEXTENDEDMODE        (0x00000012ul) // Return a particular extended mode
#define D3DGDI2_TYPE_GETADAPTERGROUP        (0x00000013ul) // Return a adapter group information
#define D3DGDI2_TYPE_GETMULTISAMPLEQUALITYLEVELS (0x00000016ul) // Return the number of multisample quality levels
#define D3DGDI2_TYPE_DEFERRED_AGP_AWARE     (0x00000018ul) // Runtime is aware of deferred AGP frees, and will send following (NT only)
#define D3DGDI2_TYPE_FREE_DEFERRED_AGP      (0x00000019ul) // Free any deferred-freed AGP allocations for this process (NT only)
#define D3DGDI2_TYPE_DEFER_AGP_FREES        (0x00000020ul) // Start defering AGP frees for this process
#define D3DGDI2_TYPE_GETD3DQUERYCOUNT       (0x00000021ul) // Return the number of supported queries
#define D3DGDI2_TYPE_GETD3DQUERY            (0x00000022ul) // Return supported query
#define D3DGDI2_TYPE_GETDDIVERSION          (0x00000023ul) // Return DX9_DDI_VERSION

//
// This data structure is returned by the driver in response to a
// GetDriverInfo2 query with the type D3DGDI2_TYPE_GETFORMATCOUNT. It simply
// gives the number of surface formats supported by the driver. Currently this
// structure consists of a single member giving the number of supported
// surface formats.
//
typedef struct _DD_GETFORMATCOUNTDATA
{
    DD_GETDRIVERINFO2DATA gdi2;          // [in/out] GetDriverInfo2 data
    DWORD                 dwFormatCount; // [out]    Number of supported surface formats
    DWORD                 dwReserved;    // Reserved
} DD_GETFORMATCOUNTDATA;

//
// This data structure is used to request a specific surface format from the
// driver. It is guaranteed that the requested format will be greater than or
// equal to zero and less that the format count reported by the driver from
// the preceeding D3DGDI2_TYPE_GETFORMATCOUNT request.
//
typedef struct _DD_GETFORMATDATA
{
    DD_GETDRIVERINFO2DATA gdi2;             // [in/out] GetDriverInfo2 data
    DWORD                 dwFormatIndex;    // [in]     The format to return
    DDPIXELFORMAT         format;           // [out]    The actual format
} DD_GETFORMATDATA;

//
// This data structure is used to notify drivers about the DirectX version
// number. This is the value that is denoted as DD_RUNTIME_VERSION in the
// DDK headers.
//
typedef struct _DD_DXVERSION
{
    DD_GETDRIVERINFO2DATA gdi2;             // [in/out] GetDriverInfo2 data
    DWORD                 dwDXVersion;      // [in]     The Version of DX
    DWORD                 dwReserved;       // Reserved
} DD_DXVERSION;

// Informs driver that runtime will send a notification after last outstanding AGP
// lock has been released.
typedef struct _DD_DEFERRED_AGP_AWARE_DATA
{
    DD_GETDRIVERINFO2DATA gdi2;        // [in/out] GetDriverInfo2 data
} DD_DEFERRED_AGP_AWARE_DATA;

// Notification that the last AGP lock has been released. Driver can free all deferred AGP
// allocations for this process.
typedef struct _DD_FREE_DEFERRED_AGP_DATA
{
    DD_GETDRIVERINFO2DATA gdi2;        // [in/out] GetDriverInfo2 data
    DWORD dwProcessId;                   // [in] Process ID for whom to free deferred AGP
} DD_FREE_DEFERRED_AGP_DATA;

#if(DIRECT3D_VERSION >= 0x0900)
//
// This data structure is returned by the driver in response to a
// GetDriverInfo2 query with the type D3DGDI2_TYPE_GETEXTENDEDMODECOUNT. It simply
// gives the number of extended video modes supported by the driver. Currently this
// structure consists of a single member giving the number of supported extended
// video modes.
//
typedef struct _DD_GETEXTENDEDMODECOUNTDATA
{
    DD_GETDRIVERINFO2DATA gdi2;          // [in/out] GetDriverInfo2 data
    DWORD                 dwModeCount;   // [out]    Number of supported extended video modes
    DWORD                 dwReserved;    // Reserved
} DD_GETEXTENDEDMODECOUNTDATA;

//
// This data structure is used to request a specific extended video mode from the
// driver. It is guaranteed that the requested format will be greater than or
// equal to zero and less that the format count reported by the driver from
// the preceeding D3DGDI2_TYPE_GETEXTENDEDMODECOUNT request.
//
typedef struct _DD_GETEXTENDEDMODEDATA
{
    DD_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                 dwModeIndex;    // [in]     The format to return
    D3DDISPLAYMODE        mode;           // [out]    The actual format
} DD_GETEXTENDEDMODEDATA;

//
// This data structure is used to request a adapter group information from the driver.
// A adapter group is a set of adapters which share video hardware (like video memory,
// 3D accelerator). Thus it is mainly for DualView video adapter. Direct3D runtime
// will share surface resources (like texture, vertex buffers) across adapters within
// a adapter group upon application's request.
//
typedef struct _DD_GETADAPTERGROUPDATA
{
    DD_GETDRIVERINFO2DATA gdi2;                   // [in/out] GetDriverInfo2 data
    ULONG_PTR             ulUniqueAdapterGroupId; // [out] The unique id of adapter group that this adapter belonging to
    DWORD                 dwReserved1;            // Reserved, must be 0
    DWORD                 dwReserved2;            // Reserved, must be 0
} DD_GETADAPTERGROUPDATA;

typedef struct _DD_MULTISAMPLEQUALITYLEVELSDATA
{
    DD_GETDRIVERINFO2DATA gdi2;          //[in/out] GetDriverInfo2 data
    D3DFORMAT             Format;        //[in] Format of multi-sampled render-target
    BOOL                  bFlip  :  1;   //[in] FALSE means blt-style resolution
    D3DMULTISAMPLE_TYPE   MSType : 31;   //[in]
    DWORD                 QualityLevels; //[out]
} DD_MULTISAMPLEQUALITYLEVELSDATA;

typedef struct _DD_GETD3DQUERYCOUNTDATA
{
    DD_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                 dwNumQueries;   // [out]    Number of queries
} DD_GETD3DQUERYCOUNTDATA;

typedef struct _DD_GETD3DQUERYDATA
{
    DD_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    union
    {
        DWORD                 dwQueryIndex; // [in] Index of cap
        D3DQUERYTYPE          QueryType;    // [out] Query cap
    };
} DD_GETD3DQUERYDATA;

typedef struct _DD_GETDDIVERSIONDATA
{
    DD_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                 dwDXVersion;    // [in] DX version (9 for DX9, etc.)
    DWORD                 dwDDIVersion;   // [out] DX9_DDI_VERSION
} DD_GETDDIVERSIONDATA;

#define DX9_DDI_VERSION     4

#endif /* DIRECT3D_VERSION >= 0x0900 */

#if(DIRECT3D_VERSION >= 0x0900)

// GetDriverState IDs - D3DDEVINFO structures used for query mechanism in public headers

#define D3DDEVINFOID_VCACHE                 4   /* Used with D3DDEVINFOID_VCACHE */

// This was eliminated in DX9 but was exposed in DX8.1 so the drivers still need it
#define D3DFMT_W11V11U10 (D3DFORMAT)65

#endif /* DIRECT3D_VERSION >= 0x0900 */

// New Caps that are not API visible that the driver exposes.
#define D3DDEVCAPS_HWVERTEXBUFFER       0x02000000L /* Device supports Driver Allocated Vertex Buffers*/
#define D3DDEVCAPS_HWINDEXBUFFER        0x04000000L /* Device supports Driver Allocated Index Buffers*/
#define D3DDEVCAPS_SUBVOLUMELOCK        0x08000000L /* Device supports locking a part of volume texture*/
#ifndef D3DPMISCCAPS_FOGINFVF
#define D3DPMISCCAPS_FOGINFVF           0x00002000L /* Device supports separate fog value in the FVF */
#endif
#ifndef D3DFVF_FOG
#define D3DFVF_FOG                      0x00002000L /* There is a separate fog value in the FVF vertex */
#endif

//
// This stuff is not API visible but should be DDI visible.
// Should be in Sync with d3d8types.h
//
#define D3DFMT_D32    (D3DFORMAT)71
#define D3DFMT_S1D15  (D3DFORMAT)72
#define D3DFMT_D15S1  (D3DFORMAT)73
#define D3DFMT_S8D24  (D3DFORMAT)74
#define D3DFMT_D24S8  (D3DFORMAT)75
#define D3DFMT_X8D24  (D3DFORMAT)76
#define D3DFMT_D24X8 (D3DFORMAT)77
#define D3DFMT_X4S4D24 (D3DFORMAT)78
#define D3DFMT_D24X4S4 (D3DFORMAT)79


//-------------- Vertex shader defines --------------------------------
// Vertex Shader register limits. D3D device must provide at least
// specified number of registers

// This one was used by DX8 only.
#define D3DVS_MAXINSTRUCTIONCOUNT_V1_1  128

// Max number of labels in a shader
#define D3DVS_LABEL_MAX_V3_0            2048

// Max number of output texture coordinates
#define D3DVS_TCRDOUTREG_MAX_V1_1       8
#define D3DVS_TCRDOUTREG_MAX_V2_0       8
#define D3DVS_TCRDOUTREG_MAX_V2_1       8
#define D3DVS_OUTPUTREG_MAX_V3_0       12
#define D3DVS_OUTPUTREG_MAX_SW_DX9     16

// Max number of output registers
#define D3DVS_OUTPUTREG_MAX_V3_0        12

// Max number of output attributes (colors)
#define D3DVS_ATTROUTREG_MAX_V1_1       2
#define D3DVS_ATTROUTREG_MAX_V2_0       2
#define D3DVS_ATTROUTREG_MAX_V2_1       2

// Max number of input registers
#define D3DVS_INPUTREG_MAX_V1_1         16
#define D3DVS_INPUTREG_MAX_V2_0         16
#define D3DVS_INPUTREG_MAX_V2_1         16
#define D3DVS_INPUTREG_MAX_V3_0         16

// Max number of temp registers
#define D3DVS_TEMPREG_MAX_V1_1          12
#define D3DVS_TEMPREG_MAX_V2_0          12
#define D3DVS_TEMPREG_MAX_V2_1          32
#define D3DVS_TEMPREG_MAX_V3_0          32

// Max number of constant float vector registers
#define D3DVS_CONSTREG_MAX_V1_1         96
#define D3DVS_CONSTREG_MAX_V2_0         8192
#define D3DVS_CONSTREG_MAX_V2_1         8192
#define D3DVS_CONSTREG_MAX_V3_0         8192

#define D3DVS_CONSTINTREG_MAX_SW_DX9    2048

// Max number of integer constant registers
#define D3DVS_CONSTINTREG_MAX_V2_0      16
#define D3DVS_CONSTINTREG_MAX_V2_1      16
#define D3DVS_CONSTINTREG_MAX_V3_0      16

#define D3DVS_CONSTBOOLREG_MAX_SW_DX9   2048

// Max number of BOOL constant registers
#define D3DVS_CONSTBOOLREG_MAX_V2_0     16
#define D3DVS_CONSTBOOLREG_MAX_V2_1     16
#define D3DVS_CONSTBOOLREG_MAX_V3_0     16

// Max number of vector address registers
#define D3DVS_ADDRREG_MAX_V1_1          1
#define D3DVS_ADDRREG_MAX_V2_0          1
#define D3DVS_ADDRREG_MAX_V2_1          1
#define D3DVS_ADDRREG_MAX_V3_0          1

// Max absolute value of the loop step
#define D3DVS_MAXLOOPSTEP_V2_0          128
#define D3DVS_MAXLOOPSTEP_V2_1          128
#define D3DVS_MAXLOOPSTEP_V3_0          128

// Max absolute value of the loop initial valuep
#define D3DVS_MAXLOOPINITVALUE_V2_0     255
#define D3DVS_MAXLOOPINITVALUE_V2_1     255
#define D3DVS_MAXLOOPINITVALUE_V3_0     255

// Max loop interation count
#define D3DVS_MAXLOOPITERATIONCOUNT_V2_0 255
#define D3DVS_MAXLOOPITERATIONCOUNT_V2_1 255
#define D3DVS_MAXLOOPITERATIONCOUNT_V3_0 255

// Number of PREDICATE registers
#define D3DVS_PREDICATE_MAX_V2_1         1
#define D3DVS_PREDICATE_MAX_V3_0         1

//---------------- End vertex shader defines -------------------------------


//---------------- Pixel shader defines ------------------------------------
// Pixel Shader register limits. D3D device must provide at least
// specified number of registers

// Number of INPUT registers based on shader version
#define D3DPS_INPUTREG_MAX_V1_1         2
#define D3DPS_INPUTREG_MAX_V1_2         2
#define D3DPS_INPUTREG_MAX_V1_3         2
#define D3DPS_INPUTREG_MAX_V1_4         2
#define D3DPS_INPUTREG_MAX_V2_0         2
#define D3DPS_INPUTREG_MAX_V2_1         2
#define D3DPS_INPUTREG_MAX_V3_0         10
#define D3DPS_INPUTREG_MAX_SW_DX9       14

// Number of TEMP registers based on shader version
#define D3DPS_TEMPREG_MAX_V1_1          2
#define D3DPS_TEMPREG_MAX_V1_2          2
#define D3DPS_TEMPREG_MAX_V1_3          2
#define D3DPS_TEMPREG_MAX_V1_4          6
#define D3DPS_TEMPREG_MAX_V2_0          12
#define D3DPS_TEMPREG_MAX_V2_1          32
#define D3DPS_TEMPREG_MAX_V3_0          32

// Number of TEXTURE registers based on shader version
#define D3DPS_TEXTUREREG_MAX_V1_1       4
#define D3DPS_TEXTUREREG_MAX_V1_2       4
#define D3DPS_TEXTUREREG_MAX_V1_3       4
#define D3DPS_TEXTUREREG_MAX_V1_4       6
#define D3DPS_TEXTUREREG_MAX_V2_0       8
#define D3DPS_TEXTUREREG_MAX_V2_1       8
#define D3DPS_TEXTUREREG_MAX_V3_0       0

// Number of COLOROUT registers based on shader version
#define D3DPS_COLOROUT_MAX_V2_0         4
#define D3DPS_COLOROUT_MAX_V2_1         4
#define D3DPS_COLOROUT_MAX_V3_0         4

// Number of PREDICATE registers based on shader version
#define D3DPS_PREDICATE_MAX_V2_1         1
#define D3DPS_PREDICATE_MAX_V3_0         1

// Number of FLOAT constants based on shader version
#define D3DPS_CONSTREG_MAX_V1_1         8
#define D3DPS_CONSTREG_MAX_V1_2         8
#define D3DPS_CONSTREG_MAX_V1_3         8
#define D3DPS_CONSTREG_MAX_V1_4         8
#define D3DPS_CONSTREG_MAX_V2_0         32
#define D3DPS_CONSTREG_MAX_V2_1         32
#define D3DPS_CONSTREG_MAX_V3_0         224
#define D3DPS_CONSTREG_MAX_SW_DX9       8192

// Max number of pixel shader hardware BOOL constant registers
#define D3DPS_CONSTBOOLREG_MAX_V2_1     16
#define D3DPS_CONSTBOOLREG_MAX_V3_0     16
#define D3DPS_CONSTBOOLREG_MAX_SW_DX9   2048

// Max number of pixel shader hardware INTEGER constant registers
#define D3DPS_CONSTINTREG_MAX_V2_1     16
#define D3DPS_CONSTINTREG_MAX_V3_0     16
#define D3DPS_CONSTINTREG_MAX_SW_DX9   2048

// Max absolute value for loop step
#define D3DPS_MAXLOOPSTEP_V2_1          128
#define D3DPS_MAXLOOPSTEP_V3_0          128

// Max absolute value for loop initial value
#define D3DPS_MAXLOOPINITVALUE_V2_1     255
#define D3DPS_MAXLOOPINITVALUE_V3_0     255

// Max loop interation count
#define D3DPS_MAXLOOPITERATIONCOUNT_V2_1 255
#define D3DPS_MAXLOOPITERATIONCOUNT_V3_0 255

//---------------- End pixel shader defines -------------------------------

// Pixel Shader DX8 register limits. D3D device will have at most these
// specified number of registers
//
#define D3DPS_INPUTREG_MAX_DX8         8
#define D3DPS_TEMPREG_MAX_DX8          8
#define D3DPS_CONSTREG_MAX_DX8         8
#define D3DPS_TEXTUREREG_MAX_DX8       8

#if(DIRECT3D_VERSION >= 0x0900)

// bit declarations for _Type fields
#define D3DVSDT_FLOAT1      0x00    // 1D float expanded to (value, 0., 0., 1.)
#define D3DVSDT_FLOAT2      0x01    // 2D float expanded to (value, value, 0., 1.)
#define D3DVSDT_FLOAT3      0x02    // 3D float expanded to (value, value, value, 1.)
#define D3DVSDT_FLOAT4      0x03    // 4D float
#define D3DVSDT_D3DCOLOR    0x04    // 4D packed unsigned bytes mapped to 0. to 1. range
                                    // Input is in D3DCOLOR format (ARGB) expanded to (R, G, B, A)
#define D3DVSDT_UBYTE4      0x05    // 4D unsigned byte
#define D3DVSDT_SHORT2      0x06    // 2D signed short expanded to (value, value, 0., 1.)
#define D3DVSDT_SHORT4      0x07    // 4D signed short

#define D3DVSDE_POSITION        0
#define D3DVSDE_BLENDWEIGHT     1
#define D3DVSDE_BLENDINDICES    2
#define D3DVSDE_NORMAL          3
#define D3DVSDE_PSIZE           4
#define D3DVSDE_DIFFUSE         5
#define D3DVSDE_SPECULAR        6
#define D3DVSDE_TEXCOORD0       7
#define D3DVSDE_TEXCOORD1       8
#define D3DVSDE_TEXCOORD2       9
#define D3DVSDE_TEXCOORD3       10
#define D3DVSDE_TEXCOORD4       11
#define D3DVSDE_TEXCOORD5       12
#define D3DVSDE_TEXCOORD6       13
#define D3DVSDE_TEXCOORD7       14
#define D3DVSDE_POSITION2       15
#define D3DVSDE_NORMAL2         16

/* DX8 style vertex declaration

Vertex Shader Declaration

The declaration portion of a vertex shader defines the static external
interface of the shader.  The information in the declaration includes:

- Assignments of vertex shader input registers to data streams.  These
assignments bind a specific vertex register to a single component within a
vertex stream.  A vertex stream element is identified by a byte offset
within the stream and a type.  The type specifies the arithmetic data type
plus the dimensionality (1, 2, 3, or 4 values).  Stream data which is
less than 4 values are always expanded out to 4 values with zero or more
0.F values and one 1.F value.

- Assignment of vertex shader input registers to implicit data from the
primitive tessellator.  This controls the loading of vertex data which is
not loaded from a stream, but rather is generated during primitive
tessellation prior to the vertex shader.

- Loading data into the constant memory at the time a shader is set as the
current shader.  Each token specifies values for one or more contiguous 4
DWORD constant registers.  This allows the shader to update an arbitrary
subset of the constant memory, overwriting the device state (which
contains the current values of the constant memory).  Note that these
values can be subsequently overwritten (between DrawPrimitive calls)
during the time a shader is bound to a device via the
SetVertexShaderConstant method.


Declaration arrays are single-dimensional arrays of DWORDs composed of
multiple tokens each of which is one or more DWORDs.  The single-DWORD
token value 0xFFFFFFFF is a special token used to indicate the end of the
declaration array.  The single DWORD token value 0x00000000 is a NOP token
with is ignored during the declaration parsing.  Note that 0x00000000 is a
valid value for DWORDs following the first DWORD for multiple word tokens.

[31:29] TokenType
    0x0 - NOP (requires all DWORD bits to be zero)
    0x1 - stream selector
    0x2 - stream data definition (map to vertex input memory)
    0x3 - vertex input memory from tessellator
    0x4 - constant memory from shader
    0x5 - extension
    0x6 - reserved
    0x7 - end-of-array (requires all DWORD bits to be 1)

NOP Token (single DWORD token)
    [31:29] 0x0
    [28:00] 0x0

Stream Selector (single DWORD token)
    [31:29] 0x1
    [28]    indicates whether this is a tessellator stream
    [27:04] 0x0
    [03:00] stream selector (0..15)

Stream Data Definition (single DWORD token)
    Vertex Input Register Load
      [31:29] 0x2
      [28]    0x0
      [27:20] 0x0
      [19:16] type (dimensionality and data type)
      [15:04] 0x0
      [03:00] vertex register address (0..15)
    Data Skip (no register load)
      [31:29] 0x2
      [28]    0x1
      [27:20] 0x0
      [19:16] count of DWORDS to skip over (0..15)
      [15:00] 0x0
    Vertex Input Memory from Tessellator Data (single DWORD token)
      [31:29] 0x3
      [28]    indicates whether data is normals or u/v
      [27:24] 0x0
      [23:20] vertex register address (0..15)
      [19:16] type (dimensionality)
      [15:04] 0x0
      [03:00] vertex register address (0..15)

Constant Memory from Shader (multiple DWORD token)
    [31:29] 0x4
    [28:25] count of 4*DWORD constants to load (0..15)
    [24:07] 0x0
    [06:00] constant memory address (0..95)

Extension Token (single or multiple DWORD token)
    [31:29] 0x5
    [28:24] count of additional DWORDs in token (0..31)
    [23:00] extension-specific information

End-of-array token (single DWORD token)
    [31:29] 0x7
    [28:00] 0x1fffffff

The stream selector token must be immediately followed by a contiguous set of stream data definition tokens.  This token sequence fully defines that stream, including the set of elements within the stream, the order in which the elements appear, the type of each element, and the vertex register into which to load an element.
Streams are allowed to include data which is not loaded into a vertex register, thus allowing data which is not used for this shader to exist in the vertex stream.  This skipped data is defined only by a count of DWORDs to skip over, since the type information is irrelevant.
The token sequence:
Stream Select: stream=0
Stream Data Definition (Load): type=FLOAT3; register=3
Stream Data Definition (Load): type=FLOAT3; register=4
Stream Data Definition (Skip): count=2
Stream Data Definition (Load): type=FLOAT2; register=7

defines stream zero to consist of 4 elements, 3 of which are loaded into registers and the fourth skipped over.  Register 3 is loaded with the first three DWORDs in each vertex interpreted as FLOAT data.  Register 4 is loaded with the 4th, 5th, and 6th DWORDs interpreted as FLOAT data.  The next two DWORDs (7th and 8th) are skipped over and not loaded into any vertex input register.   Register 7 is loaded with the 9th and 10th DWORDS interpreted as FLOAT data.
Placing of tokens other than NOPs between the Stream Selector and Stream Data Definition tokens is disallowed.

*/

#ifndef __COMMONHALDEFINES
#define __COMMONHALDEFINES

typedef enum _D3DVSD_TOKENTYPE
{
    D3DVSD_TOKEN_NOP        = 0,    // NOP or extension
    D3DVSD_TOKEN_STREAM,            // stream selector
    D3DVSD_TOKEN_STREAMDATA,        // stream data definition (map to vertex input memory)
    D3DVSD_TOKEN_TESSELLATOR,       // vertex input memory from tessellator
    D3DVSD_TOKEN_CONSTMEM,          // constant memory from shader
    D3DVSD_TOKEN_EXT,               // extension
    D3DVSD_TOKEN_END = 7,           // end-of-array (requires all DWORD bits to be 1)
    D3DVSD_FORCE_DWORD = 0x7fffffff,// force 32-bit size enum
} D3DVSD_TOKENTYPE;
#endif // __COMMONHALDEFINES

#define D3DVSD_TOKENTYPESHIFT   29
#define D3DVSD_TOKENTYPEMASK    (7 << D3DVSD_TOKENTYPESHIFT)

#define D3DVSD_STREAMNUMBERSHIFT 0
#define D3DVSD_STREAMNUMBERMASK (0xF << D3DVSD_STREAMNUMBERSHIFT)

#define D3DVSD_DATALOADTYPESHIFT 28
#define D3DVSD_DATALOADTYPEMASK (0x1 << D3DVSD_DATALOADTYPESHIFT)

#define D3DVSD_DATATYPESHIFT 16
#define D3DVSD_DATATYPEMASK (0xF << D3DVSD_DATATYPESHIFT)

#define D3DVSD_SKIPCOUNTSHIFT 16
#define D3DVSD_SKIPCOUNTMASK (0xF << D3DVSD_SKIPCOUNTSHIFT)

#define D3DVSD_VERTEXREGSHIFT 0
#define D3DVSD_VERTEXREGMASK (0x1F << D3DVSD_VERTEXREGSHIFT)

#define D3DVSD_VERTEXREGINSHIFT 20
#define D3DVSD_VERTEXREGINMASK (0xF << D3DVSD_VERTEXREGINSHIFT)

#define D3DVSD_CONSTCOUNTSHIFT 25
#define D3DVSD_CONSTCOUNTMASK (0xF << D3DVSD_CONSTCOUNTSHIFT)

#define D3DVSD_CONSTADDRESSSHIFT 0
#define D3DVSD_CONSTADDRESSMASK (0x7F << D3DVSD_CONSTADDRESSSHIFT)

#define D3DVSD_CONSTRSSHIFT 16
#define D3DVSD_CONSTRSMASK (0x1FFF << D3DVSD_CONSTRSSHIFT)

#define D3DVSD_EXTCOUNTSHIFT 24
#define D3DVSD_EXTCOUNTMASK (0x1F << D3DVSD_EXTCOUNTSHIFT)

#define D3DVSD_EXTINFOSHIFT 0
#define D3DVSD_EXTINFOMASK (0xFFFFFF << D3DVSD_EXTINFOSHIFT)

#define D3DVSD_MAKETOKENTYPE(tokenType) ((tokenType << D3DVSD_TOKENTYPESHIFT) & D3DVSD_TOKENTYPEMASK)

// macros for generation of CreateVertexShader Declaration token array

// Set current stream
// _StreamNumber [0..(MaxStreams-1)] stream to get data from
//
#define D3DVSD_STREAM( _StreamNumber ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_STREAM) | (_StreamNumber))

// Set tessellator stream
//
#define D3DVSD_STREAMTESSSHIFT 28
#define D3DVSD_STREAMTESSMASK (1 << D3DVSD_STREAMTESSSHIFT)
#define D3DVSD_STREAM_TESS( ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_STREAM) | (D3DVSD_STREAMTESSMASK))

// bind single vertex register to vertex element from vertex stream
//
// _VertexRegister [0..15] address of the vertex register
// _Type [D3DVSDT_*] dimensionality and arithmetic data type

#define D3DVSD_REG( _VertexRegister, _Type ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_STREAMDATA) |            \
     ((_Type) << D3DVSD_DATATYPESHIFT) | (_VertexRegister))

// Skip _DWORDCount DWORDs in vertex
//
#define D3DVSD_SKIP( _DWORDCount ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_STREAMDATA) | 0x10000000 | \
     ((_DWORDCount) << D3DVSD_SKIPCOUNTSHIFT))

// load data into vertex shader constant memory
//
// _ConstantAddress [0..95] - address of constant array to begin filling data
// _Count [0..15] - number of constant vectors to load (4 DWORDs each)
// followed by 4*_Count DWORDS of data
//
#define D3DVSD_CONST( _ConstantAddress, _Count ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_CONSTMEM) | \
     ((_Count) << D3DVSD_CONSTCOUNTSHIFT) | (_ConstantAddress))

// enable tessellator generated normals
//
// _VertexRegisterIn  [0..15] address of vertex register whose input stream
//                            will be used in normal computation
// _VertexRegisterOut [0..15] address of vertex register to output the normal to
//
#define D3DVSD_TESSNORMAL( _VertexRegisterIn, _VertexRegisterOut ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_TESSELLATOR) | \
     ((_VertexRegisterIn) << D3DVSD_VERTEXREGINSHIFT) | \
     ((0x02) << D3DVSD_DATATYPESHIFT) | (_VertexRegisterOut))

// enable tessellator generated surface parameters
//
// _VertexRegister [0..15] address of vertex register to output parameters
//
#define D3DVSD_TESSUV( _VertexRegister ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_TESSELLATOR) | 0x10000000 | \
     ((0x01) << D3DVSD_DATATYPESHIFT) | (_VertexRegister))

// Generates END token
//
#define D3DVSD_END() 0xFFFFFFFF

// Generates NOP token
#define D3DVSD_NOP() 0x00000000

#endif /* DIRECT3D_VERSION >= 0x0900 */

#endif /* _D3DHAL_H */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

