/*==========================================================================
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:   d3dnthal.h
 *  Content:    Direct3D HAL include file for NT
 *
 ***************************************************************************/

#ifndef _D3DNTHAL_H_
#define _D3DNTHAL_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <ddrawint.h>
#ifndef _WINDOWS_
#define _WINDOWS_
#include <d3dtypes.h>
#include <d3dcaps.h>
#undef _WINDOWS_
#else
#include <d3dtypes.h>
#include <d3dcaps.h>
#endif

/*
 * If the HAL driver does not implement clipping, it must reserve at least
 * this much space at the end of the LocalVertexBuffer for use by the HEL
 * clipping.  I.e. the vertex buffer contain dwNumVertices+dwNumClipVertices
 * vertices.  No extra space is needed by the HEL clipping in the
 * LocalHVertexBuffer.
 */
#define D3DNTHAL_NUMCLIPVERTICES    20

/*
 * If no dwNumVertices is given, this is what will be used.
 */
#define D3DNTHAL_DEFAULT_TL_NUM ((32 * 1024) / sizeof (D3DTLVERTEX))
#define D3DNTHAL_DEFAULT_H_NUM  ((32 * 1024) / sizeof (D3DHVERTEX))

/*
 * Description for a device.
 * This is used to describe a device that is to be created or to query
 * the current device.
 *
 * For DX5 and subsequent runtimes, D3DNTDEVICEDESC is a user-visible
 * structure that is not seen by the device drivers. The runtime
 * stitches a D3DNTDEVICEDESC together using the D3DNTDEVICEDESC_V1
 * embedded in the GLOBALDRIVERDATA and the extended caps queried
 * from the driver using GetDriverInfo.
 */

typedef struct _D3DNTHALDeviceDesc_V1
{
    DWORD               dwSize;                     // Size of D3DNTHALDEVICEDESC_V1 structure
    DWORD               dwFlags;                    // Indicates which fields have valid data
    D3DCOLORMODEL       dcmColorModel;              // Color model of device
    DWORD               dwDevCaps;                  // Capabilities of device
    D3DTRANSFORMCAPS    dtcTransformCaps;           // Capabilities of transform
    BOOL                bClipping;                  // Device can do 3D clipping
    D3DLIGHTINGCAPS     dlcLightingCaps;            // Capabilities of lighting
    D3DPRIMCAPS         dpcLineCaps;
    D3DPRIMCAPS         dpcTriCaps;
    DWORD               dwDeviceRenderBitDepth;     // One of DDBB_8, 16, etc..
    DWORD               dwDeviceZBufferBitDepth;    // One of DDBD_16, 32, etc..
    DWORD               dwMaxBufferSize;            // Maximum execute buffer size */
    DWORD               dwMaxVertexCount;           // Maximum vertex count */
} D3DNTHALDEVICEDESC_V1, *LPD3DNTHALDEVICEDESC_V1;

#define D3DNTHALDEVICEDESCSIZE_V1 (sizeof(D3DNTHALDEVICEDESC_V1))

/*
 * This is equivalent to the D3DNTDEVICEDESC understood by DX5, available only
 * from DX6. It is the same as D3DNTDEVICEDESC structure in DX5.
 * D3DNTDEVICEDESC is still the user-visible structure that is not seen by the
 * device drivers. The runtime stitches a D3DNTDEVICEDESC together using the
 * D3DNTDEVICEDESC_V1 embedded in the GLOBALDRIVERDATA and the extended caps
 * queried from the driver using GetDriverInfo.
 */

typedef struct _D3DNTHALDeviceDesc_V2
{
    DWORD               dwSize;                     // Size of D3DNTDEVICEDESC structure
    DWORD               dwFlags;                    // Indicates which fields have valid data
    D3DCOLORMODEL       dcmColorModel;              // Color model of device
    DWORD               dwDevCaps;                  // Capabilities of device
    D3DTRANSFORMCAPS    dtcTransformCaps;           // Capabilities of transform
    BOOL                bClipping;                  // Device can do 3D clipping
    D3DLIGHTINGCAPS     dlcLightingCaps;            // Capabilities of lighting
    D3DPRIMCAPS         dpcLineCaps;
    D3DPRIMCAPS         dpcTriCaps;
    DWORD               dwDeviceRenderBitDepth;     // One of DDBD_16, etc..
    DWORD               dwDeviceZBufferBitDepth;    // One of DDBD_16, 32, etc..
    DWORD               dwMaxBufferSize;            // Maximum execute buffer size
    DWORD               dwMaxVertexCount;           // Maximum vertex count

    DWORD               dwMinTextureWidth, dwMinTextureHeight;
    DWORD               dwMaxTextureWidth, dwMaxTextureHeight;
    DWORD               dwMinStippleWidth, dwMaxStippleWidth;
    DWORD               dwMinStippleHeight, dwMaxStippleHeight;

} D3DNTHALDEVICEDESC_V2, *LPD3DNTHALDEVICEDESC_V2;

#define D3DNTHALDEVICEDESCSIZE_V2 (sizeof(D3DNTHALDEVICEDESC_V2))

#if(DIRECT3D_VERSION >= 0x0700)
/*
 * This is equivalent to the D3DNTDEVICEDESC understood by DX6, available only
 * from DX6. It is the same as D3DNTDEVICEDESC structure in DX6.
 * D3DNTDEVICEDESC is still the user-visible structure that is not seen by the
 * device drivers. The runtime stitches a D3DNTDEVICEDESC together using the
 * D3DNTDEVICEDESC_V1 embedded in the GLOBALDRIVERDATA and the extended caps
 * queried from the driver using GetDriverInfo.
 */

typedef struct _D3DNTDeviceDesc_V3
{
    DWORD               dwSize;                     // Size of D3DNTDEVICEDESC structure
    DWORD               dwFlags;                    // Indicates which fields have valid data
    D3DCOLORMODEL       dcmColorModel;              // Color model of device
    DWORD               dwDevCaps;                  // Capabilities of device
    D3DTRANSFORMCAPS    dtcTransformCaps;           // Capabilities of transform
    BOOL                bClipping;                  // Device can do 3D clipping
    D3DLIGHTINGCAPS     dlcLightingCaps;            // Capabilities of lighting
    D3DPRIMCAPS         dpcLineCaps;
    D3DPRIMCAPS         dpcTriCaps;
    DWORD               dwDeviceRenderBitDepth;     // One of DDBD_16, etc..
    DWORD               dwDeviceZBufferBitDepth;    // One of DDBD_16, 32, etc..
    DWORD               dwMaxBufferSize;            // Maximum execute buffer size
    DWORD               dwMaxVertexCount;           // Maximum vertex count

    DWORD               dwMinTextureWidth, dwMinTextureHeight;
    DWORD               dwMaxTextureWidth, dwMaxTextureHeight;
    DWORD               dwMinStippleWidth, dwMaxStippleWidth;
    DWORD               dwMinStippleHeight, dwMaxStippleHeight;

    DWORD               dwMaxTextureRepeat;
    DWORD               dwMaxTextureAspectRatio;
    DWORD               dwMaxAnisotropy;
    D3DVALUE            dvGuardBandLeft;
    D3DVALUE            dvGuardBandTop;
    D3DVALUE            dvGuardBandRight;
    D3DVALUE            dvGuardBandBottom;
    D3DVALUE            dvExtentsAdjust;
    DWORD               dwStencilCaps;
    DWORD               dwFVFCaps;                  // low 4 bits: 0 implies TLVERTEX only, 1..8 imply FVF aware
    DWORD               dwTextureOpCaps;
    WORD                wMaxTextureBlendStages;
    WORD                wMaxSimultaneousTextures;
} D3DNTDEVICEDESC_V3, *LPD3DNTDEVICEDESC_V3;

#define D3DNTDEVICEDESCSIZE_V3 (sizeof(D3DNTDEVICEDESC_V3))
#endif /* DIRECT3D_VERSION >= 0x0700 */

/* --------------------------------------------------------------
 * Instantiated by the HAL driver on driver connection.
 *
 * Regarding dwNumVertices, specify 0 if you are relying on the HEL to do
 * everything and you do not need the resultant TLVertex buffer to reside
 * in device memory.
 * The HAL driver will be asked to allocate dwNumVertices + dwNumClipVertices
 * in the case described above.
 */
typedef struct _D3DNTHAL_GLOBALDRIVERDATA
{
    DWORD                   dwSize;                 // Size of this structure
    D3DNTHALDEVICEDESC_V1   hwCaps;                 // Capabilities of the hardware
    DWORD                   dwNumVertices;          // see following comment
    DWORD                   dwNumClipVertices;      // see following comment
    DWORD                   dwNumTextureFormats;    // Number of texture formats
    LPDDSURFACEDESC         lpTextureFormats;       // Pointer to texture formats
} D3DNTHAL_GLOBALDRIVERDATA;
typedef D3DNTHAL_GLOBALDRIVERDATA *LPD3DNTHAL_GLOBALDRIVERDATA;

#define D3DNTHAL_GLOBALDRIVERDATASIZE (sizeof(D3DNTHAL_GLOBALDRIVERDATA))

#if(DIRECT3D_VERSION >= 0x0700)
/* --------------------------------------------------------------
 * Extended caps introduced with DX5 and queried with
 * GetDriverInfo (GUID_D3DExtendedCaps).
 */
typedef struct _D3DNTHAL_D3DDX6EXTENDEDCAPS
{
    DWORD       dwSize;                             // Size of this structure

    DWORD       dwMinTextureWidth, dwMaxTextureWidth;
    DWORD       dwMinTextureHeight, dwMaxTextureHeight;
    DWORD       dwMinStippleWidth, dwMaxStippleWidth;
    DWORD       dwMinStippleHeight, dwMaxStippleHeight;

    // fields added for DX6
    DWORD       dwMaxTextureRepeat;
    DWORD       dwMaxTextureAspectRatio;
    DWORD       dwMaxAnisotropy;
    D3DVALUE    dvGuardBandLeft;
    D3DVALUE    dvGuardBandTop;
    D3DVALUE    dvGuardBandRight;
    D3DVALUE    dvGuardBandBottom;
    D3DVALUE    dvExtentsAdjust;
    DWORD       dwStencilCaps;
    DWORD       dwFVFCaps;                          // low 4 bits: 0 implies TLVERTEX only, 1..8 imply FVF aware
    DWORD       dwTextureOpCaps;
    WORD        wMaxTextureBlendStages;
    WORD        wMaxSimultaneousTextures;

} D3DNTHAL_D3DDX6EXTENDEDCAPS;
#endif /* DIRECT3D_VERSION >= 0x0700 */

/* --------------------------------------------------------------
 * Extended caps introduced with DX5 and queried with
 * GetDriverInfo (GUID_D3DExtendedCaps).
 */
typedef struct _D3DNTHAL_D3DEXTENDEDCAPS
{
    DWORD       dwSize;                         // Size of this structure
    DWORD       dwMinTextureWidth, dwMaxTextureWidth;
    DWORD       dwMinTextureHeight, dwMaxTextureHeight;
    DWORD       dwMinStippleWidth, dwMaxStippleWidth;
    DWORD       dwMinStippleHeight, dwMaxStippleHeight;

    // fields added for DX6
    DWORD       dwMaxTextureRepeat;
    DWORD       dwMaxTextureAspectRatio;
    DWORD       dwMaxAnisotropy;
    D3DVALUE    dvGuardBandLeft;
    D3DVALUE    dvGuardBandTop;
    D3DVALUE    dvGuardBandRight;
    D3DVALUE    dvGuardBandBottom;
    D3DVALUE    dvExtentsAdjust;
    DWORD       dwStencilCaps;
    DWORD       dwFVFCaps;                      // 0 implies TLVERTEX only, 1..8 imply full FVF aware */
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
} D3DNTHAL_D3DEXTENDEDCAPS;

typedef D3DNTHAL_D3DEXTENDEDCAPS *LPD3DNTHAL_D3DEXTENDEDCAPS;

#define D3DNTHAL_D3DEXTENDEDCAPSSIZE (sizeof(D3DNTHAL_D3DEXTENDEDCAPS))

// This is a temporary fix to make older NT drivers to compile
#define dvVertexProcessingCaps dwVertexProcessingCaps

#if(DIRECT3D_VERSION >= 0x0700)
typedef D3DNTHAL_D3DDX6EXTENDEDCAPS *LPD3DNTHAL_D3DDX6EXTENDEDCAPS;
#define D3DNTHAL_D3DDX6EXTENDEDCAPSSIZE (sizeof(D3DNTHAL_D3DDX6EXTENDEDCAPS))
#endif /* DIRECT3D_VERSION >= 0x0700 */

/* --------------------------------------------------------------
 * Argument to the HAL functions.
 */

typedef ULONG_PTR D3DINTHAL_BUFFERHANDLE, *LPD3DINTHAL_BUFFERHANDLE;

typedef struct _D3DNTHAL_CONTEXTCREATEDATA
{
    union
    {
        PDD_DIRECTDRAW_GLOBAL   lpDDGbl;    // in:  obsolete
        PDD_DIRECTDRAW_LOCAL    lpDDLcl;    // in:  Driver struct
    };
    union
    {
        PDD_SURFACE_LOCAL       lpDDS;      // in:  obsolete
        PDD_SURFACE_LOCAL       lpDDSLcl;   // in:  Surface to be used as target
    };
    union
    {
        PDD_SURFACE_LOCAL       lpDDSZ;     // in:  obsolete
        PDD_SURFACE_LOCAL       lpDDSZLcl;  // in:  Surface to be used as Z
    };
    DWORD                       dwPID;      // in:  Current process id
    ULONG_PTR                   dwhContext; // in/out: Context handle
    HRESULT                     ddrval;     // out: Return value
} D3DNTHAL_CONTEXTCREATEDATA;
typedef D3DNTHAL_CONTEXTCREATEDATA *LPD3DNTHAL_CONTEXTCREATEDATA;

typedef struct _D3DNTHAL_CONTEXTDESTROYDATA
{
    ULONG_PTR   dwhContext; // in:  Context handle
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_CONTEXTDESTROYDATA;
typedef D3DNTHAL_CONTEXTDESTROYDATA *LPD3DNTHAL_CONTEXTDESTROYDATA;

typedef struct _D3DNTHAL_CONTEXTDESTROYALLDATA
{
    DWORD       dwPID;      // in:  Process id to destroy contexts for
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_CONTEXTDESTROYALLDATA;
typedef D3DNTHAL_CONTEXTDESTROYALLDATA *LPD3DNTHAL_CONTEXTDESTROYALLDATA;

typedef struct _D3DNTHAL_SCENECAPTUREDATA
{
    ULONG_PTR   dwhContext; // in:  Context handle
    DWORD       dwFlag;     // in:  Indicates beginning or end
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_SCENECAPTUREDATA;
typedef D3DNTHAL_SCENECAPTUREDATA *LPD3DNTHAL_SCENECAPTUREDATA;

typedef struct _D3DNTHAL_TEXTURECREATEDATA
{
    ULONG_PTR   dwhContext; // in:  Context handle
    HANDLE      hDDS;       // in:  Handle to surface object
    ULONG_PTR   dwHandle;   // out: Handle to texture
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_TEXTURECREATEDATA;
typedef D3DNTHAL_TEXTURECREATEDATA *LPD3DNTHAL_TEXTURECREATEDATA;

typedef struct _D3DNTHAL_TEXTUREDESTROYDATA
{
    ULONG_PTR   dwhContext; // in:  Context handle
    ULONG_PTR   dwHandle;   // in:  Handle to texture
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_TEXTUREDESTROYDATA;
typedef D3DNTHAL_TEXTUREDESTROYDATA *LPD3DNTHAL_TEXTUREDESTROYDATA;

typedef struct _D3DNTHAL_TEXTURESWAPDATA
{
    ULONG_PTR   dwhContext; // in:  Context handle
    ULONG_PTR   dwHandle1;  // in:  Handle to texture 1
    ULONG_PTR   dwHandle2;  // in:  Handle to texture 2
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_TEXTURESWAPDATA;
typedef D3DNTHAL_TEXTURESWAPDATA *LPD3DNTHAL_TEXTURESWAPDATA;

typedef struct _D3DNTHAL_TEXTUREGETSURFDATA
{
    ULONG_PTR   dwhContext; // in:  Context handle
    HANDLE      hDDS;       // out: Handle to surface object
    ULONG_PTR   dwHandle;   // in:  Handle to texture
    HRESULT     ddrval;     // out: Return value
} D3DNTHAL_TEXTUREGETSURFDATA;
typedef D3DNTHAL_TEXTUREGETSURFDATA *LPD3DNTHAL_TEXTUREGETSURFDATA;

/* --------------------------------------------------------------
 * Flags for the data parameters.
 */

/*
 * SceneCapture()
 * This is used as an indication to the driver that a scene is about to
 * start or end, and that it should capture data if required.
 */
#define D3DNTHAL_SCENE_CAPTURE_START    0x00000000L
#define D3DNTHAL_SCENE_CAPTURE_END      0x00000001L

/* --------------------------------------------------------------
 * Return values from HAL functions.
 */

/*
 * The context passed in was bad.
 */
#define D3DNTHAL_CONTEXT_BAD        0x000000200L

/*
 * No more contexts left.
 */
#define D3DNTHAL_OUTOFCONTEXTS      0x000000201L

/* --------------------------------------------------------------
 * Direct3D HAL Table.
 * Instantiated by the HAL driver on connection.
 *
 * Calls take the form of:
 *  retcode = HalCall(HalCallData* lpData);
 */

typedef DWORD   (APIENTRY *LPD3DNTHAL_CONTEXTCREATECB)(LPD3DNTHAL_CONTEXTCREATEDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_CONTEXTDESTROYCB)(LPD3DNTHAL_CONTEXTDESTROYDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_CONTEXTDESTROYALLCB)(LPD3DNTHAL_CONTEXTDESTROYALLDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_SCENECAPTURECB)(LPD3DNTHAL_SCENECAPTUREDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_TEXTURECREATECB)(LPD3DNTHAL_TEXTURECREATEDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_TEXTUREDESTROYCB)(LPD3DNTHAL_TEXTUREDESTROYDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_TEXTURESWAPCB)(LPD3DNTHAL_TEXTURESWAPDATA);
typedef DWORD   (APIENTRY *LPD3DNTHAL_TEXTUREGETSURFCB)(LPD3DNTHAL_TEXTUREGETSURFDATA);

typedef struct _D3DNTHAL_CALLBACKS
{
    DWORD                           dwSize;

    // Device context
    LPD3DNTHAL_CONTEXTCREATECB      ContextCreate;
    LPD3DNTHAL_CONTEXTDESTROYCB     ContextDestroy;
    LPD3DNTHAL_CONTEXTDESTROYALLCB  ContextDestroyAll;

    // Scene Capture
    LPD3DNTHAL_SCENECAPTURECB       SceneCapture;

    // Execution
    LPVOID                          dwReserved10;       // Must be zero (was Execute)
    LPVOID                          dwReserved11;       // Must be zero (was ExecuteClipped)
    LPVOID                          dwReserved22;       // Must be zero (was RenderState)
    LPVOID                          dwReserved23;       // Must be zero (was RenderPrimitive)

    ULONG_PTR                       dwReserved;         // Must be zero

    // Textures
    LPD3DNTHAL_TEXTURECREATECB      TextureCreate;
    LPD3DNTHAL_TEXTUREDESTROYCB     TextureDestroy;
    LPD3DNTHAL_TEXTURESWAPCB        TextureSwap;
    LPD3DNTHAL_TEXTUREGETSURFCB     TextureGetSurf;

    LPVOID                          dwReserved12;       // Must be zero
    LPVOID                          dwReserved13;       // Must be zero
    LPVOID                          dwReserved14;       // Must be zero
    LPVOID                          dwReserved15;       // Must be zero
    LPVOID                          dwReserved16;       // Must be zero
    LPVOID                          dwReserved17;       // Must be zero
    LPVOID                          dwReserved18;       // Must be zero
    LPVOID                          dwReserved19;       // Must be zero
    LPVOID                          dwReserved20;       // Must be zero
    LPVOID                          dwReserved21;       // Must be zero

    // Pipeline state
    LPVOID                          dwReserved24;       // Was GetState;

    ULONG_PTR                       dwReserved0;        // Must be zero
    ULONG_PTR                       dwReserved1;        // Must be zero
    ULONG_PTR                       dwReserved2;        // Must be zero
    ULONG_PTR                       dwReserved3;        // Must be zero
    ULONG_PTR                       dwReserved4;        // Must be zero
    ULONG_PTR                       dwReserved5;        // Must be zero
    ULONG_PTR                       dwReserved6;        // Must be zero
    ULONG_PTR                       dwReserved7;        // Must be zero
    ULONG_PTR                       dwReserved8;        // Must be zero
    ULONG_PTR                       dwReserved9;        // Must be zero

} D3DNTHAL_CALLBACKS;
typedef D3DNTHAL_CALLBACKS *LPD3DNTHAL_CALLBACKS;

#define D3DNTHAL_SIZE_V1 sizeof(D3DNTHAL_CALLBACKS)

typedef struct _D3DNTHAL_SETRENDERTARGETDATA
{
    ULONG_PTR           dwhContext;     // in:  Context handle
    PDD_SURFACE_LOCAL   lpDDS;          // in:  new render target
    PDD_SURFACE_LOCAL   lpDDSZ;         // in:  new Z buffer
    HRESULT             ddrval;         // out: Return value
} D3DNTHAL_SETRENDERTARGETDATA;
typedef D3DNTHAL_SETRENDERTARGETDATA *LPD3DNTHAL_SETRENDERTARGETDATA;


typedef DWORD (APIENTRY *LPD3DNTHAL_SETRENDERTARGETCB)(LPD3DNTHAL_SETRENDERTARGETDATA);

typedef struct _D3DNTHAL_CALLBACKS2
{
    DWORD                           dwSize;             // size of struct
    DWORD                           dwFlags;            // flags for callbacks

    LPD3DNTHAL_SETRENDERTARGETCB    SetRenderTarget;
    LPVOID                          dwReserved1;        // was Clear
    LPVOID                          dwReserved2;        // was DrawOnePrimitive
    LPVOID                          dwReserved3;        // was DrawOneIndexedPrimitive
    LPVOID                          dwReserved4;        // was DrawPrimitives
} D3DNTHAL_CALLBACKS2;
typedef D3DNTHAL_CALLBACKS2 *LPD3DNTHAL_CALLBACKS2;

#define D3DNTHAL2_CB32_SETRENDERTARGET    0x00000001L


typedef struct _D3DNTHAL_CLEAR2DATA
{
    ULONG_PTR           dwhContext;     // in:  Context handle

    // dwFlags can contain D3DCLEAR_TARGET or D3DCLEAR_ZBUFFER
    DWORD               dwFlags;        // in:  surfaces to clear

    DWORD               dwFillColor;    // in:  Color value for rtarget
    D3DVALUE            dvFillDepth;    // in:  Depth value for
                                        //      Z-buffer (0.0-1.0)
    DWORD               dwFillStencil;  // in:  value used to clear stencil buffer

    LPD3DRECT           lpRects;        // in:  Rectangles to clear
    DWORD               dwNumRects;     // in:  Number of rectangles

    HRESULT             ddrval;         // out: Return value
} D3DNTHAL_CLEAR2DATA;
typedef D3DNTHAL_CLEAR2DATA FAR *LPD3DNTHAL_CLEAR2DATA;

typedef struct _D3DNTHAL_VALIDATETEXTURESTAGESTATEDATA
{
    ULONG_PTR           dwhContext;     // in:  Context handle
    DWORD               dwFlags;        // in:  Flags, currently set to 0
    ULONG_PTR           dwReserved;     //
    DWORD               dwNumPasses;    // out: Number of passes the hardware
                                        //      can perform the operation in
    HRESULT              ddrval;        // out: return value
} D3DNTHAL_VALIDATETEXTURESTAGESTATEDATA;
typedef D3DNTHAL_VALIDATETEXTURESTAGESTATEDATA FAR *LPD3DNTHAL_VALIDATETEXTURESTAGESTATEDATA;

//-----------------------------------------------------------------------------
// DrawPrimitives2 DDI
//-----------------------------------------------------------------------------

//
// Command structure for vertex buffer rendering
//

typedef struct _D3DNTHAL_DP2COMMAND
{
    BYTE        bCommand;               // vertex command
    BYTE        bReserved;
    union
    {
        WORD    wPrimitiveCount;        // primitive count for unconnected primitives
        WORD    wStateCount;            // count of render states to follow
    };
} D3DNTHAL_DP2COMMAND, *LPDNT3DHAL_DP2COMMAND;

//
// DrawPrimitives2 commands:
//

typedef enum _D3DNTHAL_DP2OPERATION
{
    D3DNTDP2OP_POINTS                   = 1,
    D3DNTDP2OP_INDEXEDLINELIST          = 2,
    D3DNTDP2OP_INDEXEDTRIANGLELIST      = 3,
    D3DNTDP2OP_RENDERSTATE              = 8,
    D3DNTDP2OP_LINELIST                 = 15,
    D3DNTDP2OP_LINESTRIP                = 16,
    D3DNTDP2OP_INDEXEDLINESTRIP         = 17,
    D3DNTDP2OP_TRIANGLELIST             = 18,
    D3DNTDP2OP_TRIANGLESTRIP            = 19,
    D3DNTDP2OP_INDEXEDTRIANGLESTRIP     = 20,
    D3DNTDP2OP_TRIANGLEFAN              = 21,
    D3DNTDP2OP_INDEXEDTRIANGLEFAN       = 22,
    D3DNTDP2OP_TRIANGLEFAN_IMM          = 23,
    D3DNTDP2OP_LINELIST_IMM             = 24,
    D3DNTDP2OP_TEXTURESTAGESTATE        = 25,
    D3DNTDP2OP_INDEXEDTRIANGLELIST2     = 26,
    D3DNTDP2OP_INDEXEDLINELIST2         = 27,
    D3DNTDP2OP_VIEWPORTINFO             = 28,
    D3DNTDP2OP_WINFO                    = 29,
    D3DNTDP2OP_SETPALETTE               = 30,
    D3DNTDP2OP_UPDATEPALETTE            = 31,
#if(DIRECT3D_VERSION >= 0x0700)
    //new for DX7
    D3DNTDP2OP_ZRANGE                   = 32,
    D3DNTDP2OP_SETMATERIAL              = 33,
    D3DNTDP2OP_SETLIGHT                 = 34,
    D3DNTDP2OP_CREATELIGHT              = 35,
    D3DNTDP2OP_SETTRANSFORM             = 36,
    D3DNTDP2OP_TEXBLT                   = 38,
    D3DNTDP2OP_STATESET                 = 39,
    D3DNTDP2OP_SETPRIORITY              = 40,
#endif /* DIRECT3D_VERSION >= 0x0700 */
    D3DNTDP2OP_SETRENDERTARGET          = 41,
    D3DNTDP2OP_CLEAR                    = 42,
#if(DIRECT3D_VERSION >= 0x0700)
    D3DNTDP2OP_SETTEXLOD                = 43,
    D3DNTDP2OP_SETCLIPPLANE             = 44,
#endif /* DIRECT3D_VERSION >= 0x0700 */
#if(DIRECT3D_VERSION >= 0x0800)
    D3DNTDP2OP_CREATEVERTEXSHADER       = 45,
    D3DNTDP2OP_DELETEVERTEXSHADER       = 46,
    D3DNTDP2OP_SETVERTEXSHADER          = 47,
    D3DNTDP2OP_SETVERTEXSHADERCONST     = 48,
    D3DNTDP2OP_SETSTREAMSOURCE          = 49,
    D3DNTDP2OP_SETSTREAMSOURCEUM        = 50,
    D3DNTDP2OP_SETINDICES               = 51,
    D3DNTDP2OP_DRAWPRIMITIVE            = 52,
    D3DNTDP2OP_DRAWINDEXEDPRIMITIVE     = 53,
    D3DNTDP2OP_CREATEPIXELSHADER        = 54,
    D3DNTDP2OP_DELETEPIXELSHADER        = 55,
    D3DNTDP2OP_SETPIXELSHADER           = 56,
    D3DNTDP2OP_SETPIXELSHADERCONST      = 57,
    D3DNTDP2OP_CLIPPEDTRIANGLEFAN       = 58,
    D3DNTDP2OP_DRAWPRIMITIVE2           = 59,
    D3DNTDP2OP_DRAWINDEXEDPRIMITIVE2    = 60,
    D3DNTDP2OP_DRAWRECTPATCH            = 61,
    D3DNTDP2OP_DRAWTRIPATCH             = 62,
    D3DNTDP2OP_VOLUMEBLT                = 63,
    D3DNTDP2OP_BUFFERBLT                = 64,
    D3DNTDP2OP_MULTIPLYTRANSFORM        = 65,
    D3DNTDP2OP_ADDDIRTYRECT             = 66,
    D3DNTDP2OP_ADDDIRTYBOX              = 67,
#endif /* DIRECT3D_VERSION >= 0x0800 */
#if(DIRECT3D_VERSION >= 0x0900)
    D3DNTDP2OP_CREATEVERTEXSHADERDECL   = 71,
    D3DNTDP2OP_DELETEVERTEXSHADERDECL   = 72,
    D3DNTDP2OP_SETVERTEXSHADERDECL      = 73,
    D3DNTDP2OP_CREATEVERTEXSHADERFUNC   = 74,
    D3DNTDP2OP_DELETEVERTEXSHADERFUNC   = 75,
    D3DNTDP2OP_SETVERTEXSHADERFUNC      = 76,
    D3DNTDP2OP_SETVERTEXSHADERCONSTI    = 77,
    D3DNTDP2OP_SETSCISSORRECT           = 79,
    D3DNTDP2OP_SETSTREAMSOURCE2         = 80,
    D3DNTDP2OP_BLT                      = 81,
    D3DNTDP2OP_COLORFILL                = 82,
    D3DNTDP2OP_SETVERTEXSHADERCONSTB    = 83,
    D3DNTDP2OP_CREATEQUERY              = 84,
    D3DNTDP2OP_SETRENDERTARGET2         = 85,
    D3DNTDP2OP_SETDEPTHSTENCIL          = 86,
    D3DNTDP2OP_RESPONSECONTINUE         = 87, // Can come only from driver
    D3DNTDP2OP_RESPONSEQUERY            = 88, // Can come only from driver
    D3DNTDP2OP_GENERATEMIPSUBLEVELS     = 89,
    D3DNTDP2OP_DELETEQUERY              = 90,
    D3DNTDP2OP_ISSUEQUERY               = 91,
    D3DNTDP2OP_SETPIXELSHADERCONSTI     = 93,
    D3DNTDP2OP_SETPIXELSHADERCONSTB     = 94,
    D3DNTDP2OP_SETSTREAMSOURCEFREQ      = 95,
    D3DNTDP2OP_SURFACEBLT               = 96,
    D3DNTDP2OP_SETCONVOLUTIONKERNELMONO = 97,
    D3DNTDP2OP_COMPOSERECTS             = 98,
#endif /* DIRECT3D_VERSION >= 0x0900 */
} D3DNTHAL_DP2OPERATION;

//
// DrawPrimitives2 point primitives
//

typedef struct _D3DNTHAL_DP2POINTS
{
    WORD    wCount;
    WORD    wVStart;
} D3DNTHAL_DP2POINTS;

//
// DrawPrimitives2 line primitives
//

typedef struct _D3DNTHAL_DP2STARTVERTEX
{
    WORD    wVStart;
} D3DNTHAL_DP2STARTVERTEX, *LPD3DNTHAL_DP2STARTVERTEX;

typedef struct _D3DNTHAL_DP2LINELIST
{
    WORD    wVStart;
} D3DNTHAL_DP2LINELIST;

typedef struct _D3DNTHAL_DP2INDEXEDLINELIST
{
    WORD    wV1;
    WORD    wV2;
} D3DNTHAL_DP2INDEXEDLINELIST;

typedef struct _D3DNTHAL_DP2LINESTRIP
{
    WORD    wVStart;
} D3DNTHAL_DP2LINESTRIP;

typedef struct _D3DNTHAL_DP2INDEXEDLINESTRIP
{
    WORD    wV[2];
} D3DNTHAL_DP2INDEXEDLINESTRIP;

//
// DrawPrimitives2 triangle primitives
//

typedef struct _D3DNTHAL_DP2TRIANGLELIST
{
    WORD    wVStart;
} D3DNTHAL_DP2TRIANGLELIST;

typedef struct _D3DNTHAL_DP2INDEXEDTRIANGLELIST
{
    WORD    wV1;
    WORD    wV2;
    WORD    wV3;
    WORD    wFlags;
} D3DNTHAL_DP2INDEXEDTRIANGLELIST;

typedef struct _D3DNTHAL_DP2INDEXEDTRIANGLELIST2 {
    WORD    wV1;
    WORD    wV2;
    WORD    wV3;
} D3DNTHAL_DP2INDEXEDTRIANGLELIST2, *LPD3DNTHAL_DP2INDEXEDTRIANGLELIST2;

typedef struct _D3DNTHAL_DP2TRIANGLESTRIP
{
    WORD    wVStart;
} D3DNTHAL_DP2TRIANGLESTRIP;

typedef struct _D3DNTHAL_DP2INDEXEDTRIANGLESTRIP
{
    WORD    wV[3];
} D3DNTHAL_DP2INDEXEDTRIANGLESTRIP;

typedef struct _D3DNTHAL_DP2TRIANGLEFAN
{
    WORD    wVStart;
} D3DNTHAL_DP2TRIANGLEFAN;

typedef struct _D3DNTHAL_DP2INDEXEDTRIANGLEFAN
{
    WORD    wV[3];
} D3DNTHAL_DP2INDEXEDTRIANGLEFAN;

typedef struct _D3DNTHAL_DP2TRIANGLEFAN_IMM
{
    DWORD   dwEdgeFlags;
} D3DNTHAL_DP2TRIANGLEFAN_IMM, *LPD3DNTHAL_DP2TRIANGLEFAN_IMM;

//
// DrawPrimitives2 Renderstate changes
//

typedef struct _D3DNTHAL_DP2RENDERSTATE
{
    D3DRENDERSTATETYPE  RenderState;
    union
    {
        D3DVALUE        fState;
        DWORD           dwState;
    };
} D3DNTHAL_DP2RENDERSTATE;
typedef D3DNTHAL_DP2RENDERSTATE  * LPD3DNTHAL_DP2RENDERSTATE;

typedef struct _D3DNTHAL_DP2TEXTURESTAGESTATE
{
    WORD    wStage;
    WORD    TSState;
    DWORD   dwValue;
} D3DNTHAL_DP2TEXTURESTAGESTATE;
typedef D3DNTHAL_DP2TEXTURESTAGESTATE  *LPD3DNTHAL_DP2TEXTURESTAGESTATE;

typedef struct _D3DNTHAL_DP2VIEWPORTINFO
{
    DWORD   dwX;
    DWORD   dwY;
    DWORD   dwWidth;
    DWORD   dwHeight;
} D3DNTHAL_DP2VIEWPORTINFO;
typedef D3DNTHAL_DP2VIEWPORTINFO  *LPD3DNTHAL_DP2VIEWPORTINFO;

typedef struct _D3DNTHAL_DP2WINFO
{
    D3DVALUE    dvWNear;
    D3DVALUE    dvWFar;
} D3DNTHAL_DP2WINFO;
typedef D3DNTHAL_DP2WINFO  *LPD3DNTHAL_DP2WINFO;

typedef struct _D3DNTHAL_DP2SETPALETTE
{
    DWORD   dwPaletteHandle;
    DWORD   dwPaletteFlags;
    DWORD   dwSurfaceHandle;
} D3DNTHAL_DP2SETPALETTE;
typedef D3DNTHAL_DP2SETPALETTE  *LPD3DNTHAL_DP2SETPALETTE;

typedef struct _D3DNTHAL_DP2UPDATEPALETTE
{
    DWORD   dwPaletteHandle;
    WORD    wStartIndex;
    WORD    wNumEntries;
} D3DNTHAL_DP2UPDATEPALETTE;
typedef D3DNTHAL_DP2UPDATEPALETTE  *LPD3DNTHAL_DP2UPDATEPALETTE;

typedef struct _D3DNTHAL_DP2SETRENDERTARGET
{
    DWORD   hRenderTarget;
    DWORD   hZBuffer;
} D3DNTHAL_DP2SETRENDERTARGET;
typedef D3DNTHAL_DP2SETRENDERTARGET  *LPD3DNTHAL_DP2SETRENDERTARGET;

#if(DIRECT3D_VERSION >= 0x0700)
// Values for dwOperations in the D3DHAL_DP2STATESET
#define D3DHAL_STATESETBEGIN    0
#define D3DHAL_STATESETEND      1
#define D3DHAL_STATESETDELETE   2
#define D3DHAL_STATESETEXECUTE  3
#define D3DHAL_STATESETCAPTURE  4

typedef struct _D3DNTHAL_DP2STATESET
{
    DWORD               dwOperation;
    DWORD               dwParam;        // State set handle passed with D3DHAL_STATESETBEGIN,
                                        // D3DHAL_STATESETEXECUTE, D3DHAL_STATESETDELETE
                                        // D3DHAL_STATESETCAPTURE
    D3DSTATEBLOCKTYPE   sbType;         // Type use with D3DHAL_STATESETBEGIN/END
} D3DNTHAL_DP2STATESET;
typedef D3DNTHAL_DP2STATESET  *LPD3DNTHAL_DP2STATESET;

//
// T&L Hal specific stuff
//
typedef struct _D3DNTHAL_DP2ZRANGE
{
    D3DVALUE    dvMinZ;
    D3DVALUE    dvMaxZ;
} D3DNTHAL_DP2ZRANGE;
typedef D3DNTHAL_DP2ZRANGE  *LPD3DNTHAL_DP2ZRANGE;

typedef D3DMATERIAL7 D3DNTHAL_DP2SETMATERIAL, *LPD3DNTHAL_DP2SETMATERIAL;

typedef struct _D3DNTHAL_DP2SETLIGHT
{
    DWORD       dwIndex;
    union {
        DWORD   lightData;
        DWORD   dwDataType;
    };
} D3DNTHAL_DP2SETLIGHT;
typedef D3DNTHAL_DP2SETLIGHT  *LPD3DNTHAL_DP2SETLIGHT;

typedef struct _D3DNTHAL_DP2SETCLIPPLANE
{
    DWORD       dwIndex;
    D3DVALUE    plane[4];
} D3DNTHAL_DP2SETCLIPPLANE;
typedef D3DNTHAL_DP2SETCLIPPLANE  *LPD3DNTHAL_DP2SETCLIPPLANE;

typedef struct _D3DNTHAL_DP2CREATELIGHT
{
    DWORD       dwIndex;
} D3DNTHAL_DP2CREATELIGHT;
typedef D3DNTHAL_DP2CREATELIGHT  *LPD3DNTHAL_DP2CREATELIGHT;

typedef struct _D3DNTHAL_DP2SETTRANSFORM
{
    D3DTRANSFORMSTATETYPE   xfrmType;
    D3DMATRIX               matrix;
} D3DNTHAL_DP2SETTRANSFORM;
typedef D3DNTHAL_DP2SETTRANSFORM  *LPD3DNTHAL_DP2SETTRANSFORM;

typedef struct _D3DNTHAL_DP2EXT
{
    DWORD   dwExtToken;
    DWORD   dwSize;
} D3DNTHAL_DP2EXT;
typedef D3DNTHAL_DP2EXT  *LPD3DNTHAL_DP2EXT;

typedef struct _D3DNTHAL_DP2TEXBLT
{
    DWORD   dwDDDestSurface;    // dest surface
    DWORD   dwDDSrcSurface;     // src surface
    POINT   pDest;
    RECTL   rSrc;               // src rect
    DWORD   dwFlags;            // blt flags
} D3DNTHAL_DP2TEXBLT;
typedef D3DNTHAL_DP2TEXBLT  *LPD3DNTHAL_DP2TEXBLT;

typedef struct _D3DNTHAL_DP2SETPRIORITY
{
    DWORD   dwDDDestSurface;    // dest surface
    DWORD   dwPriority;
} D3DNTHAL_DP2SETPRIORITY;
typedef D3DNTHAL_DP2SETPRIORITY  *LPD3DNTHAL_DP2SETPRIORITY;

typedef struct _D3DNTHAL_DP2CLEAR
{
    // dwFlags can contain D3DCLEAR_TARGET, D3DCLEAR_ZBUFFER, and/or D3DCLEAR_STENCIL
    DWORD       dwFlags;        // in:  surfaces to clear
    DWORD       dwFillColor;    // in:  Color value for rtarget
    D3DVALUE    dvFillDepth;    // in:  Depth value for Z buffer (0.0-1.0)
    DWORD       dwFillStencil;  // in:  value used to clear stencil buffer
    RECT        Rects[1];       // in:  Rectangles to clear
} D3DNTHAL_DP2CLEAR;
typedef D3DNTHAL_DP2CLEAR  *LPD3DNTHAL_DP2CLEAR;

typedef struct _D3DNTHAL_DP2SETTEXLOD
{
    DWORD   dwDDSurface;
    DWORD   dwLOD;
} D3DNTHAL_DP2SETTEXLOD;
typedef D3DNTHAL_DP2SETTEXLOD  *LPD3DNTHAL_DP2SETTEXLOD;

#endif /* DIRECT3D_VERSION >= 0x0700 */


typedef struct _D3DNTHAL_DRAWPRIMITIVES2DATA
{
    ULONG_PTR               dwhContext;             // in: Context handle
    DWORD                   dwFlags;                // in: flags (look below)
    DWORD                   dwVertexType;           // in: vertex type
    PDD_SURFACE_LOCAL       lpDDCommands;           // in: vertex buffer command data
    DWORD                   dwCommandOffset;        // in: offset to start of vb commands
    DWORD                   dwCommandLength;        // in: number of bytes of command data
    union
    {
        PDD_SURFACE_LOCAL   lpDDVertex;             // in: surface containing vertex data
        LPVOID              lpVertices;             // in: User mode pointer to vertices
    };
    DWORD                   dwVertexOffset;         // in: offset to start of vertex data
    DWORD                   dwVertexLength;         // in: number of vertices of vertex data
    DWORD                   dwReqVertexBufSize;     // in: number of bytes required for
                                                    //     the next vertex buffer
    DWORD                   dwReqCommandBufSize;    // in: number if bytes required for
                                                    //     the next commnand buffer
    LPDWORD                 lpdwRStates;            // in: Pointer to the array where render states are updated
    union
    {
        DWORD               dwVertexSize;           // in: Size of each vertex in bytes
        HRESULT             ddrval;                 // out: return value
    };
    DWORD                   dwErrorOffset;          // out: offset in LPDDVBCOMMAND to
                                                    //      first failed D3DNTHAL_VBCOMMAND
} D3DNTHAL_DRAWPRIMITIVES2DATA;
typedef D3DNTHAL_DRAWPRIMITIVES2DATA  FAR *LPD3DNTHAL_DRAWPRIMITIVES2DATA;

// Indicates that the lpVertices field in the DrawPrimitives2 data is
// valid, i.e. user allocated memory.
#define D3DNTHALDP2_USERMEMVERTICES     0x00000001L
// Indicates that the command buffer and vertex buffer are a system memory execute buffer
// resulting from the use of the Execute buffer API.
#define D3DNTHALDP2_EXECUTEBUFFER       0x00000002L

// The swap flags indicate if it is OK for the driver to swap the submitted buffers with new
// buffers and asyncronously work on the submitted buffers.
#define D3DNTHALDP2_SWAPVERTEXBUFFER    0x00000004L
#define D3DNTHALDP2_SWAPCOMMANDBUFFER   0x00000008L
// The requested flags are present if the new buffers which the driver can allocate need to be
// of atleast a given size. If any of these flags are set, the corresponding dwReq* field in
// D3DNTHAL_DRAWPRIMITIVES2DATA will also be set with the requested size in bytes.
#define D3DNTHALDP2_REQVERTEXBUFSIZE    0x00000010L
#define D3DNTHALDP2_REQCOMMANDBUFSIZE   0x00000020L
// These flags are set by the driver upon return from DrawPrimitives2 indicating if the new
// buffers are not in system memory.
#define D3DNTHALDP2_VIDMEMVERTEXBUF     0x00000040L
#define D3DNTHALDP2_VIDMEMCOMMANDBUF    0x00000080L


// Return values for the driver callback used in DP2 implementations
// Used by the driver to ask runtime to parse the execute buffer
#define D3DNTERR_COMMAND_UNPARSED       MAKE_DDHRESULT(3000)


typedef DWORD (APIENTRY *LPD3DNTHAL_CLEAR2CB)(LPD3DNTHAL_CLEAR2DATA);
typedef DWORD (APIENTRY *LPD3DNTHAL_VALIDATETEXTURESTAGESTATECB)(LPD3DNTHAL_VALIDATETEXTURESTAGESTATEDATA);
typedef DWORD (APIENTRY *LPD3DNTHAL_DRAWPRIMITIVES2CB)(LPD3DNTHAL_DRAWPRIMITIVES2DATA);

typedef struct _D3DNTHAL_CALLBACKS3
{
    DWORD                                   dwSize;                     // size of struct
    DWORD                                   dwFlags;                    // flags for callbacks

    LPD3DNTHAL_CLEAR2CB                     Clear2;
    LPVOID                                  lpvReserved;
    LPD3DNTHAL_VALIDATETEXTURESTAGESTATECB  ValidateTextureStageState;
    LPD3DNTHAL_DRAWPRIMITIVES2CB   DrawPrimitives2;
} D3DNTHAL_CALLBACKS3;
typedef D3DNTHAL_CALLBACKS3 *LPD3DNTHAL_CALLBACKS3;

#define D3DNTHAL3_CB32_CLEAR2                       0x00000001L
#define D3DNTHAL3_CB32_RESERVED                     0x00000002L
#define D3DNTHAL3_CB32_VALIDATETEXTURESTAGESTATE    0x00000004L
#define D3DNTHAL3_CB32_DRAWPRIMITIVES2              0x00000008L

// typedef for the Callback that the drivers can use to parse unknown commands
// passed to them via the DrawPrimitives2 callback. The driver obtains this
// callback thru a GetDriverInfo call with GUID_D3DParseUnknownCommandCallback
// made by ddraw somewhere around the initialization time.
typedef HRESULT (CALLBACK *PFND3DNTPARSEUNKNOWNCOMMAND)(LPVOID lpvCommands,
                                                        LPVOID *lplpvReturnedCommand);

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
#define D3DNTHAL_TSS_RENDERSTATEBASE    256UL

/*
 * Maximum number of stages allowed.
 */
#define D3DNTHAL_TSS_MAXSTAGES  8

/*
 * Number of state DWORDS per stage.
 */
#define D3DNTHAL_TSS_STATESPERSTAGE 64

/*
 * Texture handle's offset into the 32-DWORD cascade state vector
 */
#ifndef D3DTSS_TEXTUREMAP
#define D3DTSS_TEXTUREMAP 0
#endif

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
#ifndef D3DTSS_ADDRESSU
#define     D3DTSS_ADDRESSU         ((D3DTEXTURESTAGESTATETYPE)13)
#define     D3DTSS_ADDRESSV         ((D3DTEXTURESTAGESTATETYPE)14)
#define     D3DTSS_BORDERCOLOR      ((D3DTEXTURESTAGESTATETYPE)15)
#define     D3DTSS_MAGFILTER        ((D3DTEXTURESTAGESTATETYPE)16)
#define     D3DTSS_MINFILTER        ((D3DTEXTURESTAGESTATETYPE)17)
#define     D3DTSS_MIPFILTER        ((D3DTEXTURESTAGESTATETYPE)18)
#define     D3DTSS_MIPMAPLODBIAS    ((D3DTEXTURESTAGESTATETYPE)19)
#define     D3DTSS_MAXMIPLEVEL      ((D3DTEXTURESTAGESTATETYPE)20)
#define     D3DTSS_MAXANISOTROPY    ((D3DTEXTURESTAGESTATETYPE)21)
#define     D3DTSS_ADDRESSW         ((D3DTEXTURESTAGESTATETYPE)25)
#define     D3DTSS_SRGBTEXTURE      ((D3DTEXTURESTAGESTATETYPE)29)
#define     D3DTSS_ELEMENTINDEX     ((D3DTEXTURESTAGESTATETYPE)30)
#define     D3DTSS_DMAPOFFSET       ((D3DTEXTURESTAGESTATETYPE)31)
#endif

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

// These are line related states and caps that have been retired

typedef struct __NT_D3DLINEPATTERN {
    WORD    wRepeatFactor;
    WORD    wLinePattern;
} _NT_D3DLINEPATTERN;

#ifndef D3DPMISCCAPS_LINEPATTERNREP
#define D3DPMISCCAPS_LINEPATTERNREP     0x00000004L
#endif

// These are old filter caps that have been retired.
#ifndef D3DTEXF_FLATCUBIC
#define D3DTEXF_FLATCUBIC     ((D3DTEXTUREFILTERTYPE)4)
#endif

#ifndef D3DTEXF_GAUSSIANCUBIC
#define D3DTEXF_GAUSSIANCUBIC ((D3DTEXTUREFILTERTYPE)5)
#endif

#ifndef D3DRS_ZBIAS
#define D3DRS_ZBIAS           ((D3DRENDERSTATETYPE)47) // replaced by depthbias
#endif

#ifndef D3DRS_MAXVERTEXSHADERINST
#define D3DRS_MAXVERTEXSHADERINST 196 // DDI only: vs_3_0+ num instructions to execute.
#endif

#ifndef D3DRS_MAXPIXELSHADERINST
#define D3DRS_MAXPIXELSHADERINST  197 // DDI only: ps_3_0+ num instructions to execute.
#endif

#endif /* DIRECT3D_VERSION >= 0x0900 */

/*
 * DDI only renderstates.
 */
#define D3DRENDERSTATE_EVICTMANAGEDTEXTURES 61  // DDI render state only to Evict textures
#define D3DRENDERSTATE_SCENECAPTURE         62  // DDI only to replace SceneCapture

#define _NT_D3DRS_DELETERTPATCH       169 // DDI only to delete high order patch


// Default values for D3DRS_MAXVERTEXSHADERINST and D3DRS_MAXPIXELSHADERINST
#define D3DINFINITEINSTRUCTIONS 0xffffffff

#if(DIRECT3D_VERSION >= 0x0800 )

// New values for dwOperations in the D3DHAL_DP2STATESET
#define D3DNTHAL_STATESETCREATE    5

// This bit is the same as D3DCLEAR_RESERVED0 in d3d8types.h
// When set it means that driver has to cull rects against current viewport.
// The bit is set only for pure devices
//
#define D3DNTCLEAR_COMPUTERECTS   0x00000008l


typedef struct _D3DNTHAL_DP2MULTIPLYTRANSFORM
{
    D3DTRANSFORMSTATETYPE   xfrmType;
    D3DMATRIX               matrix;
} D3DNTHAL_DP2MULTIPLYTRANSFORM;
typedef D3DNTHAL_DP2MULTIPLYTRANSFORM  *LPD3DNTHAL_DP2MULTIPLYTRANSFORM;


// Used by SetVertexShader and DeleteVertexShader
typedef struct _D3DNTHAL_DP2VERTEXSHADER
{
    // Vertex shader handle.
    // The handle could be 0, meaning that the current vertex shader is invalid
    // (not set). When driver recieves handle 0, it should invalidate all
    // streams pointer
    DWORD   dwHandle;
} D3DNTHAL_DP2VERTEXSHADER;
typedef D3DNTHAL_DP2VERTEXSHADER  *LPD3DNTHAL_DP2VERTEXSHADER;

typedef struct _D3DNTHAL_DP2CREATEVERTEXSHADER
{
    DWORD   dwHandle;       // Shader handle
    DWORD   dwDeclSize;     // Shader declaration size in bytes
    DWORD   dwCodeSize;     // Shader code size in bytes
    // Declaration follows
    // Shader code follows
} D3DNTHAL_DP2CREATEVERTEXSHADER;
typedef D3DNTHAL_DP2CREATEVERTEXSHADER  *LPD3DNTHAL_DP2CREATEVERTEXSHADER;

typedef struct _D3DNTHAL_DP2SETVERTEXSHADERCONST
{
    DWORD   dwRegister;     // Const register to start copying
    DWORD   dwCount;        // Number of 4-float vectors to copy for D3DDP2OP_SETVERTEXSHADERCONST
                            // Number of 4-integer vectors to copy for D3DDP2OP_SETVERTEXSHADERCONSTI
                            // Number of BOOL values to copy for D3DDP2OP_SETVERTEXSHADERCONSTB
                            // Data follows
} D3DNTHAL_DP2SETVERTEXSHADERCONST;
typedef D3DNTHAL_DP2SETVERTEXSHADERCONST  *LPD3DNTHAL_DP2SETVERTEXSHADERCONST;

typedef struct _D3DNTHAL_DP2SETSTREAMSOURCE
{
    DWORD   dwStream;       // Stream index, starting from zero
    DWORD   dwVBHandle;     // Vertex buffer handle
    DWORD   dwStride;       // Vertex size in bytes
} D3DNTHAL_DP2SETSTREAMSOURCE;
typedef D3DNTHAL_DP2SETSTREAMSOURCE  *LPD3DNTHAL_DP2SETSTREAMSOURCE;

typedef struct _D3DNTHAL_DP2SETSTREAMSOURCEUM
{
    DWORD   dwStream;       // Stream index, starting from zero
    DWORD   dwStride;       // Vertex size in bytes
} D3DNTHAL_DP2SETSTREAMSOURCEUM;
typedef D3DNTHAL_DP2SETSTREAMSOURCEUM  *LPD3DNTHAL_DP2SETSTREAMSOURCEUM;

typedef struct _D3DNTHAL_DP2SETINDICES
{
    DWORD   dwVBHandle;     // Index buffer handle
    DWORD   dwStride;       // Index size in bytes (2 or 4)
} D3DNTHAL_DP2SETINDICES;
typedef D3DNTHAL_DP2SETINDICES  *LPD3DNTHAL_DP2SETINDICES;

typedef struct _D3DNTHAL_DP2DRAWPRIMITIVE
{
    D3DPRIMITIVETYPE    primType;
    DWORD               VStart;
    DWORD               PrimitiveCount;
} D3DNTHAL_DP2DRAWPRIMITIVE;
typedef D3DNTHAL_DP2DRAWPRIMITIVE  *LPD3DNTHAL_DP2DRAWPRIMITIVE;

typedef struct _D3DNTHAL_DP2DRAWINDEXEDPRIMITIVE
{
    D3DPRIMITIVETYPE    primType;
    INT                 BaseVertexIndex;    // Vertex which corresponds to index 0
    DWORD               MinIndex;           // Min vertex index in the vertex buffer
    DWORD               NumVertices;        // Number of vertices starting from MinIndex
    DWORD               StartIndex;         // Start index in the index buffer
    DWORD               PrimitiveCount;
} D3DNTHAL_DP2DRAWINDEXEDPRIMITIVE;
typedef D3DNTHAL_DP2DRAWINDEXEDPRIMITIVE  *LPD3DNTHAL_DP2DRAWINDEXEDPRIMITIVE;

typedef struct _D3DNTHAL_CLIPPEDTRIANGLEFAN
{
    DWORD   FirstVertexOffset;              // Offset in bytes in the current stream 0
    DWORD   dwEdgeFlags;
    DWORD   PrimitiveCount;
} D3DNTHAL_CLIPPEDTRIANGLEFAN;
typedef D3DNTHAL_CLIPPEDTRIANGLEFAN  *LPD3DNTHAL_CLIPPEDTRIANGLEFAN;

typedef struct _D3DNTHAL_DP2DRAWPRIMITIVE2
{
    D3DPRIMITIVETYPE    primType;
    DWORD               FirstVertexOffset;  // Offset in bytes in the stream 0
    DWORD               PrimitiveCount;
} D3DNTHAL_DP2DRAWPRIMITIVE2;
typedef D3DNTHAL_DP2DRAWPRIMITIVE2  *LPD3DNTHAL_DP2DRAWPRIMITIVE2;

typedef struct _D3DNTHAL_DP2DRAWINDEXEDPRIMITIVE2
{
    D3DPRIMITIVETYPE    primType;
    INT                 BaseVertexOffset;   // Stream 0 offset of the vertex which
                                            // corresponds to index 0. This offset could be
                                            // negative, but when an index is added to the
                                            // offset the result is positive
    DWORD               MinIndex;           // Min vertex index in the vertex buffer
    DWORD               NumVertices;        // Number of vertices starting from MinIndex
    DWORD               StartIndexOffset;   // Offset of the start index in the index buffer
    DWORD               PrimitiveCount;     // Number of triangles (points, lines)
} D3DNTHAL_DP2DRAWINDEXEDPRIMITIVE2;
typedef D3DNTHAL_DP2DRAWINDEXEDPRIMITIVE2  *LPD3DNTHAL_DP2DRAWINDEXEDPRIMITIVE2;

// Used by SetPixelShader and DeletePixelShader
typedef struct _D3DNTHAL_DP2PIXELSHADER
{
    // Pixel shader handle.
    // The handle could be 0, meaning that the current pixel shader is invalid
    // (not set).
    DWORD   dwHandle;
} D3DNTHAL_DP2PIXELSHADER;
typedef D3DNTHAL_DP2PIXELSHADER  *LPD3DNTHAL_DP2PIXELSHADER;

typedef struct _D3DNTHAL_DP2CREATEPIXELSHADER
{
    DWORD   dwHandle;       // Shader handle
    DWORD   dwCodeSize;     // Shader code size in bytes
    // Shader code follows
} D3DNTHAL_DP2CREATEPIXELSHADER;
typedef D3DNTHAL_DP2CREATEPIXELSHADER  *LPD3DNTHAL_DP2CREATEPIXELSHADER;

typedef struct _D3DNTHAL_DP2SETPIXELSHADERCONST
{
    DWORD   dwRegister;     // Const register to start copying
    DWORD   dwCount;        // Number of 4-float vectors to copy for D3DDP2OP_SETPIXELSHADERCONST
                            // Number of 4-integer vectors to copy for D3DDP2OP_SETPIXELSHADERCONSTI
                            // Number of BOOL values to copy for D3DDP2OP_SETPIXELSHADERCONSTB
    // Data follows
} D3DNTHAL_DP2SETPIXELSHADERCONST;
typedef D3DNTHAL_DP2SETPIXELSHADERCONST  *LPD3DNTHAL_DP2SETPIXELSHADERCONST;

// Flags that can be supplied to DRAWRECTPATCH and DRAWTRIPATCH
#define _NT_RTPATCHFLAG_HASSEGS     0x00000001L
#define _NT_RTPATCHFLAG_HASINFO     0x00000002L

typedef struct _D3DNTHAL_DP2DRAWRECTPATCH
{
    DWORD   Handle;
    DWORD   Flags;
    // Optionally followed by D3DFLOAT[4] NumSegments and/or D3DRECTPATCH_INFO
} D3DNTHAL_DP2DRAWRECTPATCH;
typedef D3DNTHAL_DP2DRAWRECTPATCH  *LPD3DNTHAL_DP2DRAWRECTPATCH;

typedef struct _D3DNTHAL_DP2DRAWTRIPATCH
{
    DWORD   Handle;
    DWORD   Flags;
    // Optionally followed by D3DFLOAT[3] NumSegments and/or D3DTRIPATCH_INFO
} D3DNTHAL_DP2DRAWTRIPATCH;
typedef D3DNTHAL_DP2DRAWTRIPATCH  *LPD3DNTHAL_DP2DRAWTRIPATCH;

typedef struct _D3DNTHAL_DP2VOLUMEBLT
{
    DWORD   dwDDDestSurface;    // dest surface
    DWORD   dwDDSrcSurface;     // src surface
    DWORD   dwDestX;            // dest X (width)
    DWORD   dwDestY;            // dest Y (height)
    DWORD   dwDestZ;            // dest Z (depth)
    D3DBOX  srcBox;             // src box
    DWORD   dwFlags;            // blt flags
} D3DNTHAL_DP2VOLUMEBLT;
typedef D3DNTHAL_DP2VOLUMEBLT  *LPD3DNTHAL_DP2VOLUMEBLT;

typedef struct _D3DNTHAL_DP2BUFFERBLT
{
    DWORD       dwDDDestSurface;    // dest surface
    DWORD       dwDDSrcSurface;     // src surface
    DWORD       dwOffset;           // Offset in the dest surface (in BYTES)
    D3DRANGE    rSrc;               // src range
    DWORD       dwFlags;            // blt flags
} D3DNTHAL_DP2BUFFERBLT;
typedef D3DNTHAL_DP2BUFFERBLT  *LPD3DNTHAL_DP2BUFFERBLT;

typedef struct _D3DNTHAL_DP2ADDDIRTYRECT
{
    DWORD   dwSurface;          // Driver managed surface
    RECTL   rDirtyArea;         // Area marked dirty
} D3DNTHAL_DP2ADDDIRTYRECT;
typedef D3DNTHAL_DP2ADDDIRTYRECT  *LPD3DNTHAL_DP2ADDDIRTYRECT;

typedef struct _D3DNTHAL_DP2ADDDIRTYBOX
{
    DWORD   dwSurface;          // Driver managed volume
    D3DBOX  DirtyBox;           // Box marked dirty
} D3DNTHAL_DP2ADDDIRTYBOX;
typedef D3DNTHAL_DP2ADDDIRTYBOX  *LPD3DNTHAL_DP2ADDDIRTYBOX;

#if(DIRECT3D_VERSION >= 0x0900 )

typedef struct _D3DNTHAL_DP2CREATEVERTEXSHADERDECL
{
    DWORD   dwHandle;               // Shader function handle
    DWORD   dwNumVertexElements;    // Number of vertex elements
                                    // D3DVERTEXELEMENT9 VertexElements[] that follow
} D3DNTHAL_DP2CREATEVERTEXSHADERDECL ;
typedef D3DNTHAL_DP2CREATEVERTEXSHADERDECL  *LPD3DNTHAL_DP2CREATEVERTEXSHADERDECL;

typedef struct _D3DNTHAL_DP2CREATEVERTEXSHADERFUNC
{
    DWORD   dwHandle;       // Shader function handle
    DWORD   dwSize;         // Shader function size in bytes
    // Shader declaration follows
} D3DNTHAL_DP2CREATEVERTEXSHADERFUNC ;
typedef D3DNTHAL_DP2CREATEVERTEXSHADERFUNC  *LPD3DNTHAL_DP2CREATEVERTEXSHADERFUNC;

typedef struct _D3DNTHAL_DP2SETSTREAMSOURCE2
{
    DWORD   dwStream;       // Stream index, starting from zero
    DWORD   dwVBHandle;     // Vertex buffer handle
    DWORD   dwOffset;       // Offset of the first vertex size in bytes
    DWORD   dwStride;       // Vertex size in bytes
} D3DNTHAL_DP2SETSTREAMSOURCE2;
typedef D3DNTHAL_DP2SETSTREAMSOURCE2  *LPD3DNTHAL_DP2SETSTREAMSOURCE2;

typedef struct _D3DNTHAL_DP2SETSTREAMSOURCEFREQ
{
    DWORD dwStream;     // Stream index, starting from zero
    DWORD dwDivider;    // Stream source divider
} D3DNTHAL_DP2SETSTREAMSOURCEFREQ;
typedef D3DNTHAL_DP2SETSTREAMSOURCEFREQ *LPD3DNTHAL_DP2SETSTREAMSOURCEFREQ;

#define D3DNTHAL_ROW_WEIGHTS  1
#define D3DNTHAL_COL_WEIGHTS  2
typedef struct _D3DNTHAL_DP2SETCONVOLUTIONKERNELMONO
{
    DWORD dwWidth;     // Kernel width
    DWORD dwHeight;    // Kernel height
    DWORD dwFlags;
    // If dwFlags & D3DNTHAL_ROW_WEIGHTS, then width floats follow. Otherwise row weights are 1.0.
    // If dwFlags & D3DNTHAL_COL_WEIGHTS, then height floats follow. Otherwise column weights are 1.0.
} D3DNTHAL_DP2SETCONVOLUTIONKERNELMONO;
typedef D3DNTHAL_DP2SETCONVOLUTIONKERNELMONO *LPD3DNTHAL_DP2SETCONVOLUTIONKERNELMONO;

typedef struct _D3DNTHAL_DP2COMPOSERECTS
{
    DWORD               SrcSurfaceHandle;
    DWORD               DstSurfaceHandle;
    DWORD               SrcRectDescsVBHandle;
    UINT                NumRects;
    DWORD               DstRectDescsVBHandle;
    D3DCOMPOSERECTSOP   Operation;
    INT                 XOffset;
    INT                 YOffset;
} D3DNTHAL_DP2COMPOSERECTS;
typedef D3DNTHAL_DP2COMPOSERECTS *LPD3DNTHAL_DP2COMPOSERECTS;

typedef RECT D3DNTHAL_DP2SETSCISSORRECT;
typedef D3DNTHAL_DP2SETSCISSORRECT *LPD3DNTHAL_DP2SETSCISSORRECT;

typedef struct _D3DNTHAL_DP2BLT
{
    DWORD   dwSource;         // Source surface
    RECTL   rSource;          // Source rectangle
    DWORD   dwSourceMipLevel; // Miplevel of lightweight surface
    DWORD   dwDest;           // Dest surface
    RECTL   rDest;            // Dest rectangle
    DWORD   dwDestMipLevel;   // Miplevel of lightweight surface
    DWORD   Flags;            // Can be DP2BLT_POINT, DP2BLT_LINEAR
} D3DNTHAL_DP2BLT;
typedef D3DNTHAL_DP2BLT  *LPD3DNTHAL_DP2BLT;

#define DP2BLT_POINT    0x00000001L
#define DP2BLT_LINEAR   0x00000002L

typedef struct _D3DNTHAL_DP2COLORFILL
{
    DWORD       dwSurface;  // Surface getting filled
    RECTL       rRect;      // Surface dimensions to fill
    D3DCOLOR    Color;      // A8R8G8B8 fill color
} D3DNTHAL_DP2COLORFILL;
typedef D3DNTHAL_DP2COLORFILL  *LPD3DNTHAL_DP2COLORFILL;

typedef struct _D3DNTHAL_DP2SURFACEBLT
{
    DWORD   dwSource;         // Source surface
    RECTL   rSource;          // Source rectangle
    DWORD   dwSourceMipLevel; // Miplevel of lightweight surface
    DWORD   dwDest;           // Dest surface
    RECTL   rDest;            // Dest rectangle
    DWORD   dwDestMipLevel;   // Miplevel of lightweight surface
    DWORD   Flags;            // No flags currently defined
} D3DNTHAL_DP2SURFACEBLT;
typedef D3DNTHAL_DP2SURFACEBLT  *LPD3DNTHAL_DP2SURFACEBLT;

typedef D3DNTHAL_DP2SETVERTEXSHADERCONST D3DNTHAL_DP2SETVERTEXSHADERCONSTB;
typedef D3DNTHAL_DP2SETVERTEXSHADERCONSTB  *LPD3DNTHAL_DP2SETVERTEXSHADERCONSTB;
typedef D3DNTHAL_DP2SETVERTEXSHADERCONST D3DNTHAL_DP2SETVERTEXSHADERCONSTI;
typedef D3DNTHAL_DP2SETVERTEXSHADERCONSTI  *LPD3DNTHAL_DP2SETVERTEXSHADERCONSTI;

typedef D3DNTHAL_DP2SETPIXELSHADERCONST D3DNTHAL_DP2SETPIXELSHADERCONSTB;
typedef D3DNTHAL_DP2SETPIXELSHADERCONSTB  *LPD3DNTHAL_DP2SETPIXELSHADERCONSTB;
typedef D3DNTHAL_DP2SETPIXELSHADERCONST D3DNTHAL_DP2SETPIXELSHADERCONSTI;
typedef D3DNTHAL_DP2SETPIXELSHADERCONSTI  *LPD3DNTHAL_DP2SETPIXELSHADERCONSTI;

typedef struct _D3DNTHAL_DP2CREATEQUERY
{
    DWORD           dwQueryID;
    D3DQUERYTYPE    QueryType;
} D3DNTHAL_DP2CREATEQUERY;
typedef D3DNTHAL_DP2CREATEQUERY  *LPD3DNTHAL_DP2CREATEQUERY;

typedef struct _D3DNTHAL_DP2DELETEQUERY
{
    DWORD   dwQueryID;
} D3DNTHAL_DP2DELETEQUERY;
typedef D3DNTHAL_DP2DELETEQUERY  *LPD3DNTHAL_DP2DELETEQUERY;

typedef struct _D3DNTHAL_DP2ISSUEQUERY
{
    DWORD   dwQueryID;
    DWORD   dwFlags;
} D3DNTHAL_DP2ISSUEQUERY;
typedef D3DNTHAL_DP2ISSUEQUERY  *LPD3DNTHAL_DP2ISSUEQUERY;

typedef struct _D3DNTHAL_DP2SETRENDERTARGET2
{
    DWORD   RTIndex;
    DWORD   hRenderTarget;
} D3DNTHAL_DP2SETRENDERTARGET2;
typedef D3DNTHAL_DP2SETRENDERTARGET2  *LPD3DNTHAL_DP2SETRENDERTARGET2;

typedef struct _D3DNTHAL_DP2SETDEPTHSTENCIL
{
    DWORD   hZBuffer;
} D3DNTHAL_DP2SETDEPTHSTENCIL;
typedef D3DNTHAL_DP2SETDEPTHSTENCIL  *LPD3DNTHAL_DP2SETDEPTHSTENCIL;

typedef struct _D3DNTHAL_DP2GENERATEMIPSUBLEVELS
{
    DWORD                   hSurface;
    D3DTEXTUREFILTERTYPE    Filter;
} D3DNTHAL_DP2GENERATEMIPSUBLEVELS;
typedef D3DNTHAL_DP2GENERATEMIPSUBLEVELS *LPD3DNTHAL_DP2GENERATEMIPSUBLEVELS;

#define DDBLT_EXTENDED_PRESENTATION_STRETCHFACTOR   0x00000010l /* old DDBLT_ALPHAEDGEBLEND */
//
// Command structure for driver responses:
//

typedef struct _D3DNTHAL_DP2RESPONSE
{
    BYTE    bCommand;       // response/ command id
    BYTE    bReserved;
    WORD    wStateCount;    // count of responses to follow
    DWORD   dwTotalSize;    // total size of response (including the DP2REPONSE struct) to enable skipping over.
} D3DNTHAL_DP2RESPONSE, *LPD3DNTHAL_DP2RESPONSE;

// Begin Responses
typedef struct _D3DNTHAL_DP2RESPONSEQUERY
{
    DWORD   dwQueryID;
    DWORD   dwSize;
} D3DNTHAL_DP2RESPONSEQUERY;
typedef D3DNTHAL_DP2RESPONSEQUERY  *LPD3DNTHAL_DP2RESPONSEQUERY;
// End Responses

#endif // (DIRECT3D_VERSION >= 0x0900 )


// Macros to access shader binary code

#define _NT_D3DSI_GETREGNUM(token)      (token & D3DSP_REGNUM_MASK)
#define _NT_D3DSI_GETOPCODE(command)    (command & D3DSI_OPCODE_MASK)
#define _NT_D3DSI_GETWRITEMASK(token)   (token & D3DSP_WRITEMASK_ALL)
#define _NT_D3DVS_GETSWIZZLECOMP(source, component) \
                                        (source >> ((component << 1) + 16) & 0x3)
#define _NT_D3DVS_GETSWIZZLE(token)     (token & D3DVS_SWIZZLE_MASK)
#define _NT_D3DVS_GETSRCMODIFIER(token) (token & D3DSP_SRCMOD_MASK)
#define _NT_D3DVS_GETADDRESSMODE(token) (token & D3DVS_ADDRESSMODE_MASK)

#if(DIRECT3D_VERSION < 0x0900)

#define _NT_D3DSI_GETREGTYPE(token)     ((D3DSHADER_PARAM_REGISTER_TYPE)(token & D3DSP_REGTYPE_MASK))

#else

#define _NT_D3DSI_GETREGTYPE(token)     ((D3DSHADER_PARAM_REGISTER_TYPE)(((token & D3DSP_REGTYPE_MASK) >> D3DSP_REGTYPE_SHIFT) | \
                                                                         ((token & D3DSP_REGTYPE_MASK2) >> D3DSP_REGTYPE_SHIFT2)))
#define _NT_D3DSI_GETUSAGE(token) ((token & D3DSP_DCL_USAGE_MASK) >> D3DSP_DCL_USAGE_SHIFT)
#define _NT_D3DSI_GETUSAGEINDEX(token) ((token & D3DSP_DCL_USAGEINDEX_MASK) >> D3DSP_DCL_USAGEINDEX_SHIFT)
#define _NT_D3DSI_GETINSTLENGTH(token)  ((token & D3DSI_INSTLENGTH_MASK) >> D3DSI_INSTLENGTH_SHIFT)
#define _NT_D3DSI_GETCOMPARISON(token)  ((D3DSHADER_COMPARISON)((token & D3DSHADER_COMPARISON_MASK) >> D3DSHADER_COMPARISON_SHIFT))
#define _NT_D3DSI_GETREGISTERPROPERTIES(token) (token & D3DSP_REGISTERPROPERTIES_MASK)
#define _NT_D3DSI_GETTEXTURETYPE(token) (token & D3DSP_TEXTURETYPE_MASK)
#define _NT_D3DSI_GETDSTMODIFIER(token) (token & D3DSP_DSTMOD_MASK))
#define _NT_D3DSI_GETSWIZZLECOMP(source, component)  (source >> ((component << 1) + 16) & 0x3)
#define _NT_D3DSI_GETSWIZZLE(token)  (token & D3DVS_SWIZZLE_MASK)
#define _NT_D3DSI_GETSRCMODIFIER(token) (token & D3DSP_SRCMOD_MASK)
#define _NT_D3DSI_GETADDRESSMODE(token) (token & D3DVS_ADDRESSMODE_MASK)

#ifdef __cplusplus
// This gets regtype, and also maps D3DSPR_CONSTn to D3DSPR_CONST (for easier parsing)
inline
D3DSHADER_PARAM_REGISTER_TYPE
_NT_D3DSI_GETREGTYPE_RESOLVING_CONSTANTS(
    DWORD   token)
{
    D3DSHADER_PARAM_REGISTER_TYPE RegType = _NT_D3DSI_GETREGTYPE(token);
    switch (RegType)
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
//           D3DSPR_CONST  is for c0-c2047
//           D3DSPR_CONST2 is for c2048-c4095
//           D3DSPR_CONST3 is for c4096-c6143
//           D3DSPR_CONST4 is for c6144-c8191
//
// For example if the instruction token specifies type D3DSPR_CONST4, reg# 3,
// the register number retrieved is 6147.
// For other register types, the register number is just returned unchanged.
inline
UINT
_NT_D3DSI_GETREGNUM_RESOLVING_CONSTANTS(
    DWORD   token)
{
    D3DSHADER_PARAM_REGISTER_TYPE RegType = _NT_D3DSI_GETREGTYPE(token);
    UINT RegNum = _NT_D3DSI_GETREGNUM(token);

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
typedef struct _DDNT_GETDRIVERINFO2DATA
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
} DDNT_GETDRIVERINFO2DATA;

//
// IMPORTANT NOTE: This GUID has exactly the same value as GUID_DDStereoMode
// and as such you must be very careful when using it. If your driver needs
// to handle both GetDriverInfo2 and DDStereoMode it must have a single
// check for the shared GUID and then distinguish between which use of that
// GUID is being requested.
//
#define _NT_GUID_GetDriverInfo2 (GUID_DDStereoMode)

//
// Magic value used to determine whether a GetDriverInfo call with the
// GUID GUID_GetDriverInfo2/GUID_DDStereoMode is a GetDriverInfo2 request
// or a query about stereo capabilities. This magic number is stored in
// the dwHeight field of the DD_STEREOMODE data structure.
//
#define _NT_D3DGDI2_MAGIC       (0xFFFFFFFFul)

//
// The types of information which can be requested from the driver via
// GetDriverInfo2.
//

#define _NT_D3DGDI2_TYPE_GETD3DCAPS8          (0x00000001ul) // Return the D3DCAPS8 data
#define _NT_D3DGDI2_TYPE_GETFORMATCOUNT       (0x00000002ul) // Return the number of supported formats
#define _NT_D3DGDI2_TYPE_GETFORMAT            (0x00000003ul) // Return a particular format
#define _NT_D3DGDI2_TYPE_DXVERSION            (0x00000004ul) // Notify driver of current DX Version
#define _NT_D3DGDI2_TYPE_DEFERRED_AGP_AWARE   (0x00000018ul) // Runtime is aware of deferred AGP frees, and will send following
#define _NT_D3DGDI2_TYPE_FREE_DEFERRED_AGP    (0x00000019ul) // Free any deferred-freed AGP allocations for this process
#define _NT_D3DGDI2_TYPE_DEFER_AGP_FREES      (0x00000020ul) // Start Deferring AGP frees for this process
#if(DIRECT3D_VERSION >= 0x0900)
#define _NT_D3DGDI2_TYPE_GETD3DCAPS9          (0x00000010ul) // Return the D3DCAPS9 data
#define _NT_D3DGDI2_TYPE_GETEXTENDEDMODECOUNT (0x00000011ul) // Return the number of supported extended mode
#define _NT_D3DGDI2_TYPE_GETEXTENDEDMODE      (0x00000012ul) // Return a particular extended mode
#define _NT_D3DGDI2_TYPE_GETADAPTERGROUP      (0x00000013ul) // Return a adapter group information

#define _NT_D3DGDI2_TYPE_GETMULTISAMPLEQUALITYLEVELS (0x00000016ul) // Return the number of multisample quality levels
#define _NT_D3DGDI2_TYPE_GETD3DQUERYCOUNT     (0x00000021ul) // Return the number of suported queries
#define _NT_D3DGDI2_TYPE_GETD3DQUERY          (0x00000022ul) // Return supported query
#define _NT_D3DGDI2_TYPE_GETDDIVERSION        (0x00000023ul) // Return DX9_DDI_VERSION
#endif // (DIRECT3D_VERSION >= 0x0900)

//
// This data structure is returned by the driver in response to a
// GetDriverInfo2 query with the type D3DGDI2_TYPE_GETFORMATCOUNT. It simply
// gives the number of surface formats supported by the driver. Currently this
// structure consists of a single member giving the number of supported
// surface formats.
//
typedef struct _DDNT_GETFORMATCOUNTDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwFormatCount;  // [out]    Number of supported surface formats
    DWORD                   dwReserved;     // Reserved
} DDNT_GETFORMATCOUNTDATA;

//
// This data structure is used to request a specific surface format from the
// driver. It is guaranteed that the requested format will be greater than or
// equal to zero and less that the format count reported by the driver from
// the preceeding D3DGDI2_TYPE_GETFORMATCOUNT request.
//
typedef struct _DDNT_GETFORMATDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwFormatIndex;  // [in]     The format to return
    DDPIXELFORMAT           format;         // [out]    The actual format
} DDNT_GETFORMATDATA;

//
// This data structure is used to notify drivers about the DirectX version
// number. This is the value that is denoted as DD_RUNTIME_VERSION in the
// DDK headers.
//
typedef struct _DDNT_DXVERSION
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwDXVersion;    // [in]     The Version of DX
    DWORD                   dwReserved;     // Reserved
} DDNT_DXVERSION;

// Informs driver that runtime will send a notification after last outstanding AGP
// lock has been released.
typedef struct _DDNT_DEFERRED_AGP_AWARE_DATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
} DDNT_DEFERRED_AGP_AWARE_DATA;

// Notification that the last AGP lock has been released. Driver can free all deferred AGP
// allocations for this process.
typedef struct _DDNT_FREE_DEFERRED_AGP_DATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwProcessId;    // [in] Process ID for whom to free deferred AGP
} DDNT_FREE_DEFERRED_AGP_DATA;

#if(DIRECT3D_VERSION >= 0x0900)
//
// This data structure is returned by the driver in response to a
// GetDriverInfo2 query with the type D3DGDI2_TYPE_GETEXTENDEDMODECOUNT. It simply
// gives the number of extended video modes supported by the driver. Currently this
// structure consists of a single member giving the number of supported extended
// video modes.
//
typedef struct _DDNT_GETEXTENDEDMODECOUNTDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwModeCount;    // [out]    Number of supported extended video modes
    DWORD                   dwReserved;     // Reserved
} DDNT_GETEXTENDEDMODECOUNTDATA;

//
// This data structure is used to request a specific extended video mode from the
// driver. It is guaranteed that the requested format will be greater than or
// equal to zero and less that the format count reported by the driver from
// the preceeding D3DGDI2_TYPE_GETEXTENDEDMODECOUNT request.
//
typedef struct _DDNT_GETEXTENDEDMODEDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwModeIndex;    // [in]     The format to return
    D3DDISPLAYMODE          mode;           // [out]    The actual format
} DDNT_GETEXTENDEDMODEDATA;

//
// This data structure is used to request a adapter group information from the driver.
// A adapter group is a set of adapters which share video hardware (like video memory,
// 3D accelerator). Thus it is mainly for DualView video adapter. Direct3D runtime
// will share surface resources (like texture, vertex buffers) across adapters within
// a adapter group upon application's request.
//
typedef struct _DDNT_GETADAPTERGROUPDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;                   // [in/out] GetDriverInfo2 data
    ULONG_PTR               ulUniqueAdapterGroupId; // [out] The unique id of adapter group that this adapter belonging to
    DWORD                   dwReserved1;            // Reserved, must be 0
    DWORD                   dwReserved2;            // Reserved, must be 0
} DDNT_GETADAPTERGROUPDATA;


// This data structure used to request the supported quality levels for a given sample count,
// presentation type, and pixel format.
typedef struct _DDNT_MULTISAMPLEQUALITYLEVELSDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           //[in/out] GetDriverInfo2 data
    D3DFORMAT               Format;         //[in] Format of multi-sampled render-target
    BOOL                    bFlip  :  1;    //[in] FALSE means blt-style resolution
    D3DMULTISAMPLE_TYPE     MSType : 31;    //[in]
    DWORD                   QualityLevels;  //[out]
} DDNT_MULTISAMPLEQUALITYLEVELSDATA;

typedef struct _DDNT_GETD3DQUERYCOUNTDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                   dwNumQueries;   // [out]    Number of queries
} DDNT_GETD3DQUERYCOUNTDATA;

typedef struct _DDNT_GETD3DQUERYDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    union
    {
        DWORD               dwQueryIndex; // [in] Index of cap
        D3DQUERYTYPE        QueryType;    // [out] Query cap
    };
} DDNT_GETD3DQUERYDATA;

typedef struct _DDNT_GETDDIVERSIONDATA
{
    DDNT_GETDRIVERINFO2DATA gdi2;           // [in/out] GetDriverInfo2 data
    DWORD                 dwDXVersion;      // [in] DX Version (9 for DX9, etc.)
    DWORD                 dwDDIVersion;     // [out] DX9_DDI_VERSION
} DDNT_GETDDIVERSIONDATA;

#define DX9_DDI_VERSION   4


#endif // (DIRECT3D_VERSION >= 0x0900 )

#if(DIRECT3D_VERSION >= 0x0900)

// GetDriverState IDs - D3DDEVINFO structures used for query mechanism in public headers

// This was eliminated in DX9 but was exposed in DX8.1 so the drivers still need it
#define D3DFMT_W11V11U10    (D3DFORMAT)65
#endif // (DIRECT3D_VERSION >= 0x0900 )

// New Caps that are not API visible that the driver exposes.
#define _NT_D3DDEVCAPS_HWVERTEXBUFFER       0x02000000L // Device supports Driver Allocated Vertex Buffers
#define _NT_D3DDEVCAPS_HWINDEXBUFFER        0x04000000L // Device supports Driver Allocated Index Buffers
#define _NT_D3DDEVCAPS_SUBVOLUMELOCK        0x08000000L // Device supports locking a part of volume texture
#define _NT_D3DPMISCCAPS_FOGINFVF           0x00002000L // Device supports separate fog value in the FVF

// New FVF flags that are not API visible but accessed by the driver
// Note, that D3DFVF_RESERVED2 includes this flag and should not be used for validation

#define _NT_D3DFVF_FOG                      0x00002000L // There is a separate fog value in the FVF vertex

// Flags that drivers need to expose for DX8 but were removed from the DX9 headers
#if(DIRECT3D_VERSION >= 0x0900)
#ifndef D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE
#define D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE 0x00800000L
#endif /* D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE */
#ifndef D3DPRASTERCAPS_PAT
#define D3DPRASTERCAPS_PAT D3DPRASTERCAPS_RESERVED2
#endif
#endif

//
// This stuff is not API visible but should be DDI visible.
// Should be in Sync with d3d8types.h
//
#define _NT_D3DFMT_D32      (D3DFORMAT)71
#define _NT_D3DFMT_S1D15    (D3DFORMAT)72
#define _NT_D3DFMT_D15S1    (D3DFORMAT)73
#define _NT_D3DFMT_S8D24    (D3DFORMAT)74
#define _NT_D3DFMT_D24S8    (D3DFORMAT)75
#define _NT_D3DFMT_X8D24    (D3DFORMAT)76
#define _NT_D3DFMT_D24X8    (D3DFORMAT)77
#define _NT_D3DFMT_X4S4D24  (D3DFORMAT)78
#define _NT_D3DFMT_D24X4S4  (D3DFORMAT)79

//---------------- Vertex shader defines -------------------------------
// Vertex Shader register limits. D3D device must provide at least
// specified number of registers

// This one was used in DX8 only.
#define _NT_D3DVS_MAXINSTRUCTIONCOUNT_V1_1  128

// Max number of labels in a shader
#define _NT_D3DVS_LABEL_MAX_V3_0            2048

// Max number of output texture coordinates
#define _NT_D3DVS_TCRDOUTREG_MAX_V1_1       8
#define _NT_D3DVS_TCRDOUTREG_MAX_V2_0       8
#define _NT_D3DVS_TCRDOUTREG_MAX_V2_1       8
#define _NT_D3DVS_OUTPUTREG_MAX_V3_0       12
#define _NT_D3DVS_OUTPUTREG_MAX_SW_DX9     16

// Max number of output attributes (colors)
#define _NT_D3DVS_ATTROUTREG_MAX_V1_1       2
#define _NT_D3DVS_ATTROUTREG_MAX_V2_0       2
#define _NT_D3DVS_ATTROUTREG_MAX_V2_1       2

// Max number of input registers
#define _NT_D3DVS_INPUTREG_MAX_V1_1         16
#define _NT_D3DVS_INPUTREG_MAX_V2_0         16
#define _NT_D3DVS_INPUTREG_MAX_V2_1         16
#define _NT_D3DVS_INPUTREG_MAX_V3_0         16

// Max number of temp registers
#define _NT_D3DVS_TEMPREG_MAX_V1_1          12
#define _NT_D3DVS_TEMPREG_MAX_V2_0          12
#define _NT_D3DVS_TEMPREG_MAX_V2_1          32
#define _NT_D3DVS_TEMPREG_MAX_V3_0          32

// Max number of constant float vector registers
#define _NT_D3DVS_CONSTREG_MAX_V1_1         96
#define _NT_D3DVS_CONSTREG_MAX_V2_0         8192
#define _NT_D3DVS_CONSTREG_MAX_V2_1         8192
#define _NT_D3DVS_CONSTREG_MAX_V3_0         8192

// The number of INTEGER constants for software is limited only by the binary opcode specification: ;internal
#define _NT_D3DVS_CONSTINTREG_MAX_SW_DX9    2048

// Max number of integer constant registers
#define _NT_D3DVS_CONSTINTREG_MAX_V2_0      16
#define _NT_D3DVS_CONSTINTREG_MAX_V2_1      16
#define _NT_D3DVS_CONSTINTREG_MAX_V3_0      16

// The number of BOOL constants for software is limited only by the binary opcode specification: ;internal
#define _NT_D3DVS_CONSTBOOLREG_MAX_SW_DX9   2048

// Max number of BOOL constant registers
#define _NT_D3DVS_CONSTBOOLREG_MAX_V2_0     16
#define _NT_D3DVS_CONSTBOOLREG_MAX_V2_1     16
#define _NT_D3DVS_CONSTBOOLREG_MAX_V3_0     16

// Max number of vector address registers
#define _NT_D3DVS_ADDRREG_MAX_V1_1          1
#define _NT_D3DVS_ADDRREG_MAX_V2_0          1
#define _NT_D3DVS_ADDRREG_MAX_V2_1          1
#define _NT_D3DVS_ADDRREG_MAX_V3_0          1

// Max absolute value of the loop step
#define _NT_D3DVS_MAXLOOPSTEP_V2_0          128
#define _NT_D3DVS_MAXLOOPSTEP_V2_1          128
#define _NT_D3DVS_MAXLOOPSTEP_V3_0          128

// Max absolute value of the loop initial valuep
#define _NT_D3DVS_MAXLOOPINITVALUE_V2_0     255
#define _NT_D3DVS_MAXLOOPINITVALUE_V2_1     255
#define _NT_D3DVS_MAXLOOPINITVALUE_V3_0     255

// Max loop interation count
#define _NT_D3DVS_MAXLOOPITERATIONCOUNT_V2_0 255
#define _NT_D3DVS_MAXLOOPITERATIONCOUNT_V2_1 255
#define _NT_D3DVS_MAXLOOPITERATIONCOUNT_V3_0 255

// Number of PREDICATE registers
#define _NT_D3DVS_PREDICATE_MAX_V2_1         1
#define _NT_D3DVS_PREDICATE_MAX_V3_0         1

//---------------- End vertex shader defines -------------------------------


//---------------- Pixel shader defines ----------------------------------
// Pixel Shader register limits. D3D device must provide at least
// specified number of registers

// Number of INPUT registers based on shader version
#define _NT_D3DPS_INPUTREG_MAX_V1_1         2
#define _NT_D3DPS_INPUTREG_MAX_V1_2         2
#define _NT_D3DPS_INPUTREG_MAX_V1_3         2
#define _NT_D3DPS_INPUTREG_MAX_V1_4         2
#define _NT_D3DPS_INPUTREG_MAX_V2_0         2
#define _NT_D3DPS_INPUTREG_MAX_V2_1         2
#define _NT_D3DPS_INPUTREG_MAX_V3_0         12

// Number of TEMP registers based on shader version
#define _NT_D3DPS_TEMPREG_MAX_V1_1          2
#define _NT_D3DPS_TEMPREG_MAX_V1_2          2
#define _NT_D3DPS_TEMPREG_MAX_V1_3          2
#define _NT_D3DPS_TEMPREG_MAX_V1_4          6
#define _NT_D3DPS_TEMPREG_MAX_V2_0          12
#define _NT_D3DPS_TEMPREG_MAX_V2_1          32
#define _NT_D3DPS_TEMPREG_MAX_V3_0          32

// Number of TEXTURE registers based on shader version
#define _NT_D3DPS_TEXTUREREG_MAX_V1_1       4
#define _NT_D3DPS_TEXTUREREG_MAX_V1_2       4
#define _NT_D3DPS_TEXTUREREG_MAX_V1_3       4
#define _NT_D3DPS_TEXTUREREG_MAX_V1_4       6
#define _NT_D3DPS_TEXTUREREG_MAX_V2_0       8
#define _NT_D3DPS_TEXTUREREG_MAX_V2_1       8
#define _NT_D3DPS_TEXTUREREG_MAX_V3_0       0

// Number of COLOROUT registers based on shader version
#define _NT_D3DPS_COLOROUT_MAX_V2_0         4
#define _NT_D3DPS_COLOROUT_MAX_V2_1         4
#define _NT_D3DPS_COLOROUT_MAX_V3_0         4

// Number of PREDICATE registers based on shader version
#define _NT_D3DPS_PREDICATE_MAX_V2_1         1
#define _NT_D3DPS_PREDICATE_MAX_V3_0         1

// The number of FLOAT constants for software is limited only by the binary opcode specification: ;internal
#define _NT_D3DPS_CONSTREG_MAX_SW_DX9       8192

// Number of FLOAT constants based on shader version
#define _NT_D3DPS_CONSTREG_MAX_V1_1         8
#define _NT_D3DPS_CONSTREG_MAX_V1_2         8
#define _NT_D3DPS_CONSTREG_MAX_V1_3         8
#define _NT_D3DPS_CONSTREG_MAX_V1_4         8
#define _NT_D3DPS_CONSTREG_MAX_V2_0         32
#define _NT_D3DPS_CONSTREG_MAX_V2_1         32
#define _NT_D3DPS_CONSTREG_MAX_V3_0         224

// The number of BOOL constants for software is limited only by the binary opcode specification: ;internal
#define _NT_D3DPS_CONSTBOOLREG_MAX_SW_DX9   2048

// Max number of pixel shader hardware BOOL constant registers
#define _NT_D3DPS_CONSTBOOLREG_MAX_V2_1     16
#define _NT_D3DPS_CONSTBOOLREG_MAX_V3_0     16

// The number of INTEGER constants for software is limited only by the binary opcode specification: ;internal
#define _NT_D3DPS_CONSTINTREG_MAX_SW_DX9   2048

// Max number of pixel shader hardware INTEGER constant registers
#define _NT_D3DPS_CONSTINTREG_MAX_V2_1     16
#define _NT_D3DPS_CONSTINTREG_MAX_V3_0     16

// Max absolute value for loop step
#define _NT_D3DPS_MAXLOOPSTEP_V2_1          128
#define _NT_D3DPS_MAXLOOPSTEP_V3_0          128

// Max absolute value for loop initial value
#define _NT_D3DPS_MAXLOOPINITVALUE_V2_1     255
#define _NT_D3DPS_MAXLOOPINITVALUE_V3_0     255

// Max loop interation count
#define _NT_D3DPS_MAXLOOPITERATIONCOUNT_V2_1 255
#define _NT_D3DPS_MAXLOOPITERATIONCOUNT_V3_0 255

//---------------- End pixel shader defines -------------------------------

// Pixel Shader DX8 register limits. D3D device will have at most these
// specified number of registers
//
// Sync up d3d?dm.hpp with D3DPS_*_MAX_DX8 ;internal
#define _NT_D3DPS_INPUTREG_MAX_DX8         8
#define _NT_D3DPS_TEMPREG_MAX_DX8          8
#define _NT_D3DPS_CONSTREG_MAX_DX8         8
#define _NT_D3DPS_TEXTUREREG_MAX_DX8       8

#endif /* DIRECT3D_VERSION >= 0x0800 */

#if(DIRECT3D_VERSION >= 0x0900)

// bit declarations for _Type fields
#define D3DVSDT_FLOAT1          0x00    // 1D float expanded to (value, 0., 0., 1.)
#define D3DVSDT_FLOAT2          0x01    // 2D float expanded to (value, value, 0., 1.)
#define D3DVSDT_FLOAT3          0x02    // 3D float expanded to (value, value, value, 1.)
#define D3DVSDT_FLOAT4          0x03    // 4D float
#define D3DVSDT_D3DCOLOR        0x04    // 4D packed unsigned bytes mapped to 0. to 1. range
                                        // Input is in D3DCOLOR format (ARGB) expanded to (R, G, B, A)
#define D3DVSDT_UBYTE4          0x05    // 4D unsigned byte
#define D3DVSDT_SHORT2          0x06    // 2D signed short expanded to (value, value, 0., 1.)
#define D3DVSDT_SHORT4          0x07    // 4D signed short

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
    D3DVSD_TOKEN_NOP            = 0,    // NOP or extension
    D3DVSD_TOKEN_STREAM,                // stream selector
    D3DVSD_TOKEN_STREAMDATA,            // stream data definition (map to vertex input memory)
    D3DVSD_TOKEN_TESSELLATOR,           // vertex input memory from tessellator
    D3DVSD_TOKEN_CONSTMEM,              // constant memory from shader
    D3DVSD_TOKEN_EXT,                   // extension
    D3DVSD_TOKEN_END            = 7,    // end-of-array (requires all DWORD bits to be 1)
    D3DVSD_FORCE_DWORD          = 0x7fffffff,// force 32-bit size enum
} D3DVSD_TOKENTYPE;
#endif // __COMMONHALDEFINES

#define D3DVSD_TOKENTYPESHIFT       29
#define D3DVSD_TOKENTYPEMASK        (7 << D3DVSD_TOKENTYPESHIFT)

#define D3DVSD_STREAMNUMBERSHIFT    0
#define D3DVSD_STREAMNUMBERMASK     (0xF << D3DVSD_STREAMNUMBERSHIFT)

#define D3DVSD_DATALOADTYPESHIFT    28
#define D3DVSD_DATALOADTYPEMASK     (0x1 << D3DVSD_DATALOADTYPESHIFT)

#define D3DVSD_DATATYPESHIFT        16
#define D3DVSD_DATATYPEMASK         (0xF << D3DVSD_DATATYPESHIFT)

#define D3DVSD_SKIPCOUNTSHIFT       16
#define D3DVSD_SKIPCOUNTMASK        (0xF << D3DVSD_SKIPCOUNTSHIFT)

#define D3DVSD_VERTEXREGSHIFT       0
#define D3DVSD_VERTEXREGMASK        (0x1F << D3DVSD_VERTEXREGSHIFT)

#define D3DVSD_VERTEXREGINSHIFT     20
#define D3DVSD_VERTEXREGINMASK      (0xF << D3DVSD_VERTEXREGINSHIFT)

#define D3DVSD_CONSTCOUNTSHIFT      25
#define D3DVSD_CONSTCOUNTMASK       (0xF << D3DVSD_CONSTCOUNTSHIFT)

#define D3DVSD_CONSTADDRESSSHIFT    0
#define D3DVSD_CONSTADDRESSMASK     (0x7F << D3DVSD_CONSTADDRESSSHIFT)

#define D3DVSD_CONSTRSSHIFT         16
#define D3DVSD_CONSTRSMASK          (0x1FFF << D3DVSD_CONSTRSSHIFT)

#define D3DVSD_EXTCOUNTSHIFT        24
#define D3DVSD_EXTCOUNTMASK         (0x1F << D3DVSD_EXTCOUNTSHIFT)

#define D3DVSD_EXTINFOSHIFT         0
#define D3DVSD_EXTINFOMASK          (0xFFFFFF << D3DVSD_EXTINFOSHIFT)

#define D3DVSD_MAKETOKENTYPE(tokenType) ((tokenType << D3DVSD_TOKENTYPESHIFT) & D3DVSD_TOKENTYPEMASK)

// macros for generation of CreateVertexShader Declaration token array

// Set current stream
// _StreamNumber [0..(MaxStreams-1)] stream to get data from
//
#define D3DVSD_STREAM( _StreamNumber ) \
    (D3DVSD_MAKETOKENTYPE(D3DVSD_TOKEN_STREAM) | (_StreamNumber))

// Set tessellator stream
//
#define D3DVSD_STREAMTESSSHIFT  28
#define D3DVSD_STREAMTESSMASK   (1 << D3DVSD_STREAMTESSSHIFT)
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
#define D3DVSD_END()    0xFFFFFFFF

// Generates NOP token
#define D3DVSD_NOP()    0x00000000

#endif /* DIRECT3D_VERSION >= 0x0900 */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _D3DNTHAL_H */
