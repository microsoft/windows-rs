/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:   dxva9typ.h
 *  Content:    Direct3D include file
 *
 ****************************************************************************/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef _DXVA9TYP_H_
#define _DXVA9TYP_H_

#ifndef DIRECT3D_VERSION
#define DIRECT3D_VERSION         0x0900
#endif  //DIRECT3D_VERSION

// include this file content only if compiling for DX9 interfaces
#if(DIRECT3D_VERSION >= 0x0900)


#include <stdlib.h>

#define COM_NO_WINDOWS_H
#include <objbase.h>

#include <windows.h>

#ifdef __DIRECTX_VA_COPP_ONLY
#define __DIRECTX_VA_DECODER__
#define __DIRECTX_VA_PROCAMPCONTROL__
#define __DIRECTX_VA_DEINTERLACE__
#endif

#ifndef DXVABit
#define DXVABit(__x) (1 << __x)
#endif


// -------------------------------------------------------------------------
//
// The definitions that follow describe the DirectX Video Acceleration
// decoding interface.
// This interface is accessable via the IAMVideoAccelerator interface.
//
// -------------------------------------------------------------------------
//
#ifndef __DIRECTX_VA_DECODER__
#define __DIRECTX_VA_DECODER__

/* AYUV sample for 16-entry YUV palette or graphic surface */

typedef struct _DXVA_AYUVsample2 {
    BYTE bCrValue;
    BYTE bCbValue;
    BYTE bY_Value;
    BYTE bSampleAlpha8;
} DXVA_AYUVsample2, *LPDXVA_AYUVsample2;

DEFINE_GUID(DXVAp_ModeMPEG2_A, 0x1b81be0A, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVAp_ModeMPEG2_C, 0x1b81be0C, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVAp_NoEncrypt,   0x1b81beD0, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

#pragma pack(push, BeforeDXVApacking, 1)

typedef struct _DXVA_BufferDescription {
  DWORD dwTypeIndex;
  DWORD dwBufferIndex;
  DWORD dwDataOffset;
  DWORD dwDataSize;
  DWORD dwFirstMBaddress;
  DWORD dwNumMBsInBuffer;
  DWORD dwWidth;
  DWORD dwHeight;
  DWORD dwStride;
  DWORD dwReservedBits;
} DXVA_BufferDescription, *LPDXVA_BufferDescription;

typedef DWORD DXVA_ConfigQueryOrReplyFunc, *LPDXVA_ConfigQueryOrReplyFunc;

#define DXVA_QUERYORREPLYFUNCFLAG_DECODER_PROBE_QUERY     0xFFFFF1
#define DXVA_QUERYORREPLYFUNCFLAG_DECODER_LOCK_QUERY      0xFFFFF5
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_COPY     0xFFFFF8
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_PLUS     0xFFFFF9
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_OK_COPY      0xFFFFFC
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_FALSE_PLUS  0xFFFFFB
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_FALSE_PLUS   0xFFFFFF

#define DXVA_PICTURE_DECODE_BUFFER               1
#define DXVA_MACROBLOCK_CONTROL_BUFFER           2
#define DXVA_RESIDUAL_DIFFERENCE_BUFFER          3
#define DXVA_DEBLOCKING_CONTROL_BUFFER           4
#define DXVA_INVERSE_QUANTIZATION_MATRIX_BUFFER  5
#define DXVA_SLICE_CONTROL_BUFFER                6
#define DXVA_BITSTREAM_DATA_BUFFER               7
#define DXVA_AYUV_BUFFER                         8
#define DXVA_IA44_SURFACE_BUFFER                 9
#define DXVA_DPXD_SURFACE_BUFFER                10
#define DXVA_HIGHLIGHT_BUFFER                   11
#define DXVA_DCCMD_SURFACE_BUFFER               12
#define DXVA_ALPHA_BLEND_COMBINATION_BUFFER     13
#define DXVA_PICTURE_RESAMPLE_BUFFER            14
#define DXVA_READ_BACK_BUFFER                   15

typedef struct _DXVA_ConfigPictureDecode {

  // Operation Indicated
  DXVA_ConfigQueryOrReplyFunc dwFunction;

  // Alignment
  DWORD dwReservedBits[3];

  // Encryption GUIDs
  GUID guidConfigBitstreamEncryption;
  GUID guidConfigMBcontrolEncryption;
  GUID guidConfigResidDiffEncryption;

  // Bitstream Processing Indicator
  BYTE bConfigBitstreamRaw;

  // Macroblock Control Config
  BYTE bConfigMBcontrolRasterOrder;

  // Host Resid Diff Config
  BYTE bConfigResidDiffHost;
  BYTE bConfigSpatialResid8;
  BYTE bConfigResid8Subtraction;
  BYTE bConfigSpatialHost8or9Clipping;
  BYTE bConfigSpatialResidInterleaved;
  BYTE bConfigIntraResidUnsigned;

  // Accelerator Resid Diff Config
  BYTE bConfigResidDiffAccelerator;
  BYTE bConfigHostInverseScan;
  BYTE bConfigSpecificIDCT;
  BYTE bConfig4GroupedCoefs;
} DXVA_ConfigPictureDecode, *LPDXVA_ConfigPictureDecode;

typedef struct _DXVA_PictureParameters {

  WORD wDecodedPictureIndex;
  WORD wDeblockedPictureIndex;

  WORD wForwardRefPictureIndex;
  WORD wBackwardRefPictureIndex;

  WORD wPicWidthInMBminus1;
  WORD wPicHeightInMBminus1;

  BYTE bMacroblockWidthMinus1;
  BYTE bMacroblockHeightMinus1;

  BYTE bBlockWidthMinus1;
  BYTE bBlockHeightMinus1;

  BYTE bBPPminus1;

  BYTE bPicStructure;
  BYTE bSecondField;
  BYTE bPicIntra;
  BYTE bPicBackwardPrediction;

  BYTE bBidirectionalAveragingMode;
  BYTE bMVprecisionAndChromaRelation;
  BYTE bChromaFormat;

  BYTE bPicScanFixed;
  BYTE bPicScanMethod;
  BYTE bPicReadbackRequests;

  BYTE bRcontrol;
  BYTE bPicSpatialResid8;
  BYTE bPicOverflowBlocks;
  BYTE bPicExtrapolation;

  BYTE bPicDeblocked;
  BYTE bPicDeblockConfined;
  BYTE bPic4MVallowed;
  BYTE bPicOBMC;
  BYTE bPicBinPB;
  BYTE bMV_RPS;

  BYTE bReservedBits;

  WORD wBitstreamFcodes;
  WORD wBitstreamPCEelements;
  BYTE bBitstreamConcealmentNeed;
  BYTE bBitstreamConcealmentMethod;

} DXVA_PictureParameters, *LPDXVA_PictureParameters;

#pragma pack(pop, BeforeDXVApacking)

#endif  /* __DIRECTX_VA_DECODER__ */


#ifndef __DIRECTX_VA_DECODER9__
#define __DIRECTX_VA_DECODER9__
// -------------------------------------------------------------------------
// Decoding data types used with RenderMoComp
// -------------------------------------------------------------------------
//

typedef struct _DXVAUncompDataInfo
{
    DWORD       UncompWidth;    /* Width of uncompressed data */
    DWORD       UncompHeight;   /* Height of uncompressed data */
    D3DFORMAT   UncompFormat;   /* Format of uncompressed data */
} DXVAUncompDataInfo;

typedef struct _DXVACompBufferInfo
{
    DWORD       NumCompBuffers;     /* Number of buffers reqd for compressed data */
    DWORD       WidthToCreate;      /* Width of surface to create */
    DWORD       HeightToCreate;     /* Height of surface to create */
    DWORD       BytesToAllocate;    /* Total number of bytes used by each surface */
    DWORD       Usage;              /* Usage used to create the compressed buffer */
    D3DPOOL     Pool;               /* Pool where the compressed buffer belongs */
    D3DFORMAT   Format;             /* Format used to create the compressed buffer */
} DXVACompBufferInfo;

typedef struct _DXVABufferInfo
{
    VOID*               pCompSurface;   /* Pointer to buffer containing compressed data */
    DWORD               DataOffset;     /* Offset of relevant data from the beginning of buffer */
    DWORD               DataSize;       /* Size of relevant data */
} DXVABufferInfo;

#endif  /* __DIRECTX_VA_DECODER9__ */


// -------------------------------------------------------------------------
//
// D3DFORMAT describes a pixel memory layout, DXVA sample format contains
// additional information that describes how the pixels should be interpreted.
//
// DXVA Extended color data - occupies the SampleFormat DWORD
// data fields.
// -------------------------------------------------------------------------
#ifndef __DIRECTX_VA_SAMPLEFORMAT__
#define __DIRECTX_VA_SAMPLEFORMAT__

typedef enum _DXVA_SampleFormat {
    DXVA_SampleFormatMask = 0xFF,   // 8 bits used for DXVA Sample format
    DXVA_SampleUnknown = 0,
    DXVA_SamplePreviousFrame = 1,
    DXVA_SampleProgressiveFrame = 2,
    DXVA_SampleFieldInterleavedEvenFirst = 3,
    DXVA_SampleFieldInterleavedOddFirst = 4,
    DXVA_SampleFieldSingleEven = 5,
    DXVA_SampleFieldSingleOdd = 6,
    DXVA_SampleSubStream = 7
} DXVA_SampleFormat;

#define DXVA_ExtractSampleFormat(_sf) ((_sf) & (DXVA_SampleFormatMask))

#define DXVA_ExtractExtColorData(_sf, _Mask, _Shift) \
    (((_sf) & (_Mask)) >> (_Shift))

#define DXVABitMask(__n) (~((~0) << __n))
#define DXVA_ExtColorData_ShiftBase 8
#define DXVAColorMask(__bits,__base) (DXVABitMask(__bits) << (__base))

typedef enum _DXVA_VideoTransferFunction
{
    DXVA_VideoTransFuncShift = (DXVA_ExtColorData_ShiftBase + 19),
    DXVA_VideoTransFuncMask = DXVAColorMask(5, DXVA_VideoTransFuncShift),

    DXVA_VideoTransFunc_Unknown = 0,
    DXVA_VideoTransFunc_10 = 1,
    DXVA_VideoTransFunc_18 = 2,
    DXVA_VideoTransFunc_20 = 3,
    DXVA_VideoTransFunc_22 = 4,
    DXVA_VideoTransFunc_22_709  = 5,
    DXVA_VideoTransFunc_22_240M = 6,
    DXVA_VideoTransFunc_22_8bit_sRGB = 7,
    DXVA_VideoTransFunc_28 = 8
} DXVA_VideoTransferFunction;

typedef enum _DXVA_VideoPrimaries
{
    DXVA_VideoPrimariesShift = (DXVA_ExtColorData_ShiftBase + 14),
    DXVA_VideoPrimariesMask = DXVAColorMask(5, DXVA_VideoPrimariesShift),

    DXVA_VideoPrimaries_Unknown = 0,
    DXVA_VideoPrimaries_reserved = 1,
    DXVA_VideoPrimaries_BT709 = 2,
    DXVA_VideoPrimaries_BT470_2_SysM = 3,
    DXVA_VideoPrimaries_BT470_2_SysBG = 4,
    DXVA_VideoPrimaries_SMPTE170M = 5,
    DXVA_VideoPrimaries_SMPTE240M = 6,
    DXVA_VideoPrimaries_EBU3213 = 7,
    DXVA_VideoPrimaries_SMPTE_C = 8
} DXVA_VideoPrimaries;

typedef enum _DXVA_VideoLighting
{
    DXVA_VideoLightingShift = (DXVA_ExtColorData_ShiftBase + 10),
    DXVA_VideoLightingMask = DXVAColorMask(4, DXVA_VideoLightingShift),

    DXVA_VideoLighting_Unknown = 0,
    DXVA_VideoLighting_bright = 1,
    DXVA_VideoLighting_office = 2,
    DXVA_VideoLighting_dim = 3,
    DXVA_VideoLighting_dark = 4
} DXVA_VideoLighting;

typedef enum _DXVA_VideoTransferMatrix
{
    DXVA_VideoTransferMatrixShift = (DXVA_ExtColorData_ShiftBase + 7),
    DXVA_VideoTransferMatrixMask = DXVAColorMask(3, DXVA_VideoTransferMatrixShift),

    DXVA_VideoTransferMatrix_Unknown = 0,
    DXVA_VideoTransferMatrix_BT709 = 1,
    DXVA_VideoTransferMatrix_BT601 = 2,
    DXVA_VideoTransferMatrix_SMPTE240M = 3
} DXVA_VideoTransferMatrix;

typedef enum _DXVA_NominalRange
{
    DXVA_NominalRangeShift = (DXVA_ExtColorData_ShiftBase + 4),
    DXVA_NominalRangeMask = DXVAColorMask(3, DXVA_NominalRangeShift),

    DXVA_NominalRange_Unknown = 0,
    DXVA_NominalRange_Normal = 1,
    DXVA_NominalRange_Wide = 2,

    DXVA_NominalRange_0_255 = 1,
    DXVA_NominalRange_16_235 = 2,
    DXVA_NominalRange_48_208 = 3
} DXVA_NominalRange;

typedef enum _DXVA_VideoChromaSubsampling
{
    DXVA_VideoChromaSubsamplingShift = (DXVA_ExtColorData_ShiftBase + 0),
    DXVA_VideoChromaSubsamplingMask = DXVAColorMask(4, DXVA_VideoChromaSubsamplingShift),

    DXVA_VideoChromaSubsampling_Unknown = 0,
    DXVA_VideoChromaSubsampling_ProgressiveChroma = 0x8,
    DXVA_VideoChromaSubsampling_Horizontally_Cosited = 0x4,
    DXVA_VideoChromaSubsampling_Vertically_Cosited = 0x2,
    DXVA_VideoChromaSubsampling_Vertically_AlignedChromaPlanes = 0x1,
    // 4:2:0 variations
    DXVA_VideoChromaSubsampling_MPEG2  =   DXVA_VideoChromaSubsampling_Horizontally_Cosited |
                                           DXVA_VideoChromaSubsampling_Vertically_AlignedChromaPlanes, 

    DXVA_VideoChromaSubsampling_MPEG1  =   DXVA_VideoChromaSubsampling_Vertically_AlignedChromaPlanes, 

    DXVA_VideoChromaSubsampling_DV_PAL  =  DXVA_VideoChromaSubsampling_Horizontally_Cosited |
                                           DXVA_VideoChromaSubsampling_Vertically_Cosited,
    // 4:4:4, 4:2:2, 4:1:1
    DXVA_VideoChromaSubsampling_Cosited =  DXVA_VideoChromaSubsampling_Horizontally_Cosited |
                                           DXVA_VideoChromaSubsampling_Vertically_Cosited |
                                           DXVA_VideoChromaSubsampling_Vertically_AlignedChromaPlanes,

} DXVA_VideoChromaSubsampling;

typedef struct _DXVA_ExtendedFormat
{
    UINT                        SampleFormat : 8;           // See DXVA_SampleFormat
    UINT                        VideoChromaSubsampling : 4; // See DXVA_VideoChromaSubSampling
    DXVA_NominalRange           NominalRange : 3;           // See DXVA_NominalRange
    DXVA_VideoTransferMatrix    VideoTransferMatrix : 3;    // See DXVA_VideoTransferMatrix
    DXVA_VideoLighting          VideoLighting : 4;          // See DXVA_VideoLighting
    DXVA_VideoPrimaries         VideoPrimaries : 5;         // See DXVA_VideoPrimaries
    DXVA_VideoTransferFunction  VideoTransferFunction : 5;  // See DXVA_VideoTransferFunction
} DXVA_ExtendedFormat;

#endif  /* __DIRECTX_VA_SAMPLEFORMAT__ */


// -------------------------------------------------------------------------
//
// The definitions that follow describe the video de-interlace interface
// between the VMR and the graphics device driver.  This interface is not
// accessable via the IAMVideoAccelerator interface.
//
// -------------------------------------------------------------------------
//
#ifndef __DIRECTX_VA_DEINTERLACE__
#define __DIRECTX_VA_DEINTERLACE__

#ifndef REFERENCE_TME
    typedef LONGLONG REFERENCE_TIME;
#endif

DEFINE_GUID(DXVAp_DeinterlaceBobDevice,          0x335aa36e,0x7884,0x43a4,0x9c,0x91,0x7f,0x87,0xfa,0xf3,0xe3,0x7e);
DEFINE_GUID(DXVAp_DeinterlaceContainerDevice,    0x0e85cb93,0x3046,0x4ff0,0xae,0xcc,0xd5,0x8c,0xb5,0xf0,0x35,0xfd);

#define DXVA_DeinterlaceBobDevice               DXVAp_DeinterlaceBobDevice
#define DXVA_DeinterlaceContainerDevice         DXVAp_DeinterlaceContainerDevice

#if (DIRECT3D_VERSION < 0x0800) || !defined(DIRECT3D_VERSION)
typedef DWORD D3DFORMAT;
enum {
    D3DPOOL_DEFAULT                 = 0,
    D3DPOOL_MANAGED                 = 1,
    D3DPOOL_SYSTEMMEM               = 2,
    D3DPOOL_SCRATCH                 = 3,
    D3DPOOL_LOCALVIDMEM             = 4,
    D3DPOOL_NONLOCALVIDMEM          = 5,
    D3DPOOL_FORCE_DWORD             = 0x7fffffff
};
#endif

// -------------------------------------------------------------------------
// data structures shared by User mode and Kernel mode.
// -------------------------------------------------------------------------
//

typedef struct _DXVA_Frequency {
    DWORD Numerator;
    DWORD Denominator;
} DXVA_Frequency;

typedef struct _DXVA_VideoDesc {
    DWORD               Size;
    DWORD               SampleWidth;
    DWORD               SampleHeight;
    DWORD               SampleFormat; // also contains extend color data
    D3DFORMAT           d3dFormat;
    DXVA_Frequency      InputSampleFreq;
    DXVA_Frequency      OutputFrameFreq;
} DXVA_VideoDesc, *LPDXVA_VideoDesc;

typedef enum _DXVA_VideoProcessCaps {
    DXVA_VideoProcess_None                  = 0x0000,
    DXVA_VideoProcess_YUV2RGB               = 0x0001,
    DXVA_VideoProcess_StretchX              = 0x0002,
    DXVA_VideoProcess_StretchY              = 0x0004,
    DXVA_VideoProcess_AlphaBlend            = 0x0008,
    DXVA_VideoProcess_SubRects              = 0x0010,
    DXVA_VideoProcess_SubStreams            = 0x0020,
    DXVA_VideoProcess_SubStreamsExtended    = 0x0040,
    DXVA_VideoProcess_YUV2RGBExtended       = 0x0080,
    DXVA_VideoProcess_AlphaBlendExtended    = 0x0100
} DXVA_VideoProcessCaps;

typedef enum _DXVA_DeinterlaceTech {

    // the algorithm is unknown or proprietary
    DXVA_DeinterlaceTech_Unknown                = 0x0000,

    // the algorithm creates the missing lines by repeating
    // the line either above or below it - this method will look very jaggy and
    // isn't recommended
    DXVA_DeinterlaceTech_BOBLineReplicate       = 0x0001,

    // The algorithm creates the missing lines by vertically stretching each
    // video field by a factor of two by averaging two lines
    DXVA_DeinterlaceTech_BOBVerticalStretch     = 0x0002,

    // or using a [-1, 9, 9, -1]/16 filter across four lines.
    DXVA_DeinterlaceTech_BOBVerticalStretch4Tap = 0x0100,

    // the pixels in the missing line are recreated by a median filtering operation
    DXVA_DeinterlaceTech_MedianFiltering        = 0x0004,

    // the pixels in the missing line are recreated by an edge filter.
    // In this process, spatial directional filters are applied to determine
    // the orientation of edges in the picture content, and missing
    // pixels are created by filtering along (rather than across) the
    // detected edges.
    DXVA_DeinterlaceTech_EdgeFiltering          = 0x0010,

    // the pixels in the missing line are recreated by switching on a field by
    // field basis between using either spatial or temporal interpolation
    // depending on the amount of motion.
    DXVA_DeinterlaceTech_FieldAdaptive          = 0x0020,

    // the pixels in the missing line are recreated by switching on a pixel by pixel
    // basis between using either spatial or temporal interpolation depending on
    // the amount of motion..
    DXVA_DeinterlaceTech_PixelAdaptive          = 0x0040,

    // Motion Vector Steering  identifies objects within a sequence of video
    // fields.  The missing pixels are recreated after first aligning the
    // movement axes of the individual objects in the scene to make them
    // parallel with the time axis.
    DXVA_DeinterlaceTech_MotionVectorSteered      = 0x0080

} DXVA_DeinterlaceTech;

typedef struct _DXVA_VideoSample {
    REFERENCE_TIME      rtStart;
    REFERENCE_TIME      rtEnd;
    DXVA_SampleFormat   SampleFormat;   // only lower 8 bits used
    VOID*               lpDDSSrcSurface;
} DXVA_VideoSample, *LPDXVA_VideoSample;

// -------------------------------------------------------------------------
// DeinterlaceBltEx declarations
// -------------------------------------------------------------------------
//

typedef enum _DXVA_SampleFlags {
    DXVA_SampleFlagsMask = DXVABit(3)|DXVABit(2)|DXVABit(1)|DXVABit(0),

    DXVA_SampleFlag_Palette_Changed         = 0x0001,
    DXVA_SampleFlag_SrcRect_Changed         = 0x0002,
    DXVA_SampleFlag_DstRect_Changed         = 0x0004,
    DXVA_SampleFlag_ColorData_Changed       = 0x0008,
} DXVA_SampleFlags;

typedef enum _DXVA_DestinationFlags {
    DXVA_DestinationFlagMask = DXVABit(3)|DXVABit(2)|DXVABit(1)|DXVABit(0),

    DXVA_DestinationFlag_Background_Changed = 0x0001,
    DXVA_DestinationFlag_TargetRect_Changed = 0x0002,
    DXVA_DestinationFlag_ColorData_Changed  = 0x0004,
    DXVA_DestinationFlag_Alpha_Changed      = 0x0008
} DXVA_DestinationFlags;

typedef struct _DXVA_VideoSample2 {
#ifdef _WIN64
    DWORD               Size;
    DWORD               Reserved;
#endif
    REFERENCE_TIME      rtStart;
    REFERENCE_TIME      rtEnd;
    DWORD               SampleFormat;   // cast to DXVA_ExtendedFormat, or use Extract macros
    DWORD               SampleFlags;
    VOID*               lpDDSSrcSurface;
    RECT                rcSrc;
    RECT                rcDst;
    DXVA_AYUVsample2    Palette[16];
} DXVA_VideoSample2, *LPDXVA_VideoSample2;

typedef struct _DXVA_DeinterlaceCaps {
    DWORD                   Size;
    DWORD                   NumPreviousOutputFrames;
    DWORD                   InputPool;
    DWORD                   NumForwardRefSamples;
    DWORD                   NumBackwardRefSamples;
    D3DFORMAT               d3dOutputFormat;
    DXVA_VideoProcessCaps   VideoProcessingCaps;
    DXVA_DeinterlaceTech    DeinterlaceTechnology;
} DXVA_DeinterlaceCaps, *LPDXVA_DeinterlaceCaps;

// -------------------------------------------------------------------------
// Data types used with RenderMoComp in kernel mode
// -------------------------------------------------------------------------
//

// Function codes for RenderMoComp

#define MAX_DEINTERLACE_SURFACES                        32

#ifdef _WIN64
//
// These structures are used for thunking 32 bit DeinterlaceBltEx calls on
// 64 bit drivers.
//
typedef struct _DXVA_VideoSample32 {
    REFERENCE_TIME      rtStart;
    REFERENCE_TIME      rtEnd;
    DWORD               SampleFormat;
    DWORD               SampleFlags;
    DWORD               lpDDSSrcSurface;  // 32 bit pointer size
    RECT                rcSrc;
    RECT                rcDst;
    DXVA_AYUVsample2    Palette[16];
    // DWORD Pad;
    // 4 bytes of padding added by the compiler to align the struct to 8 bytes.
} DXVA_VideoSample32;

typedef struct _DXVA_DeinterlaceBltEx32 {
    DWORD               Size;
    DXVA_AYUVsample2    BackgroundColor;
    RECT                rcTarget;
    REFERENCE_TIME      rtTarget;
    DWORD               NumSourceSurfaces;
    FLOAT               Alpha;
    DXVA_VideoSample32  Source[MAX_DEINTERLACE_SURFACES];
    DWORD               DestinationFormat;
    DWORD               DestinationFlags;
} DXVA_DeinterlaceBltEx32;
#endif

typedef struct _DXVA_DeinterlaceBlt {
    DWORD               Size;
    DWORD               Reserved;
    REFERENCE_TIME      rtTarget;
    RECT                DstRect;
    RECT                SrcRect;
    DWORD               NumSourceSurfaces;
    FLOAT               Alpha;
    DXVA_VideoSample    Source[MAX_DEINTERLACE_SURFACES];
} DXVA_DeinterlaceBlt;

#define DXVA_DeinterlaceBltFnCode                     0x01
// lpInput => DXVA_DeinterlaceBlt*
// lpOuput => NULL /* not currently used */

typedef struct _DXVA_DeinterlaceBltEx {
    DWORD               Size;
    DXVA_AYUVsample2    BackgroundColor;
    RECT                rcTarget;
    REFERENCE_TIME      rtTarget;
    DWORD               NumSourceSurfaces;
    FLOAT               Alpha;
    DXVA_VideoSample2   Source[MAX_DEINTERLACE_SURFACES];
    DWORD               DestinationFormat;
    DWORD               DestinationFlags;
} DXVA_DeinterlaceBltEx;

#define DXVA_DeinterlaceBltExFnCode                   0x02
// lpInput => DXVA_DeinterlaceBltEx*
// lpOuput => NULL /* not currently used */

#define MAX_DEINTERLACE_DEVICE_GUIDS                    32
typedef struct _DXVA_DeinterlaceQueryAvailableModes {
    DWORD               Size;
    DWORD               NumGuids;
    GUID                Guids[MAX_DEINTERLACE_DEVICE_GUIDS];
} DXVA_DeinterlaceQueryAvailableModes;

#define DXVA_DeinterlaceQueryAvailableModesFnCode     0x01
// lpInput => DXVA_VideoDesc*
// lpOuput => DXVA_DeinterlaceQueryAvailableModes*

typedef struct _DXVA_DeinterlaceQueryModeCaps {
    DWORD               Size;
    GUID                Guid;
    DXVA_VideoDesc      VideoDesc;
} DXVA_DeinterlaceQueryModeCaps;

#define DXVA_DeinterlaceQueryModeCapsFnCode           0x02
// lpInput => DXVA_DeinterlaceQueryModeCaps*
// lpOuput => DXVA_DeinterlaceCaps*

#endif /* __DIRECTX_VA_DEINTERLACE__ */

// -------------------------------------------------------------------------
//
// The definitions that follow describe the video ProcAmp interface
// between the VMR and the graphics device driver.  This interface is not
// accessable via the IAMVideoAccelerator interface.
//
// -------------------------------------------------------------------------
//
#ifndef __DIRECTX_VA_PROCAMPCONTROL__
#define __DIRECTX_VA_PROCAMPCONTROL__

DEFINE_GUID(DXVA_ProcAmpControlDevice,
    0x9f200913,0x2ffd,0x4056,0x9f,0x1e,0xe1,0xb5,0x08,0xf2,0x2d,0xcf);

typedef enum _DXVA_ProcAmpControlProp {
    DXVA_ProcAmp_None       = 0x0000,
    DXVA_ProcAmp_Brightness = 0x0001,
    DXVA_ProcAmp_Contrast   = 0x0002,
    DXVA_ProcAmp_Hue        = 0x0004,
    DXVA_ProcAmp_Saturation = 0x0008
} DXVA_ProcAmpControlProp;

typedef struct _DXVA_ProcAmpControlCaps {
    DWORD                   Size;
    DWORD                   InputPool;
    D3DFORMAT               d3dOutputFormat;
    DWORD                   ProcAmpControlProps;// see DXVA_ProcAmpControlProp
    DWORD                   VideoProcessingCaps;// see DXVA_VideoProcessCaps
} DXVA_ProcAmpControlCaps, *LPDXVA_ProcAmpControlCaps;

#define DXVA_ProcAmpControlQueryCapsFnCode             0x03
// lpInput => DXVA_VideoDesc*
// lpOuput => DXVA_ProcAmpControlCaps*

typedef struct _DXVA_ProcAmpControlQueryRange {
    DWORD                   Size;
    DXVA_ProcAmpControlProp ProcAmpControlProp;
    DXVA_VideoDesc          VideoDesc;
} DXVA_ProcAmpControlQueryRange, *LPDXVA_ProcAmpControlQueryRange;

typedef struct _DXVA_VideoPropertyRange {
    FLOAT   MinValue;
    FLOAT   MaxValue;
    FLOAT   DefaultValue;
    FLOAT   StepSize;
} DXVA_VideoPropertyRange, *LPDXVA_VideoPropertyRange;

#define DXVA_ProcAmpControlQueryRangeFnCode            0x04
// lpInput => DXVA_ProcAmpControlQueryRange*
// lpOuput => DXVA_VideoPropertyRange*

typedef struct _DXVA_ProcAmpControlBlt {
    DWORD               Size;
    RECT                DstRect;
    RECT                SrcRect;
    FLOAT               Alpha;
    FLOAT               Brightness;
    FLOAT               Contrast;
    FLOAT               Hue;
    FLOAT               Saturation;
} DXVA_ProcAmpControlBlt;

#define DXVA_ProcAmpControlBltFnCode                   0x01
// lpInput => DXVA_ProcAmpControlBlt*
// lpOuput => NULL /* not currently used */

#endif /* __DIRECTX_VA_PROCAMPCONTROL__ */

// -------------------------------------------------------------------------
//
// The definitions that follow describe the Certified Output Protection
// Protocol between the VMR and the graphics device driver.  This interface
// is not accessable via the IAMVideoAccelerator interface.
//
// -------------------------------------------------------------------------
//
#ifndef __DIRECTX_VA_CERTOUTPUTPROTECT__
#define __DIRECTX_VA_CERTOUTPUTPROTECT__

DEFINE_GUID(DXVA_COPPDevice,
    0xd2457add,0x8999,0x45ed,0x8a,0x8a,0xd1,0xaa,0x04,0x7b,0xa4,0xd5);

// -------------------------------------------------------------------------
// COPPGetCertificateLength
// -------------------------------------------------------------------------
#define DXVA_COPPGetCertificateLengthFnCode         0x01
// lpInput => NULL
// lpOuput => DWORD*

// -------------------------------------------------------------------------
// COPPKeyExchange
// -------------------------------------------------------------------------
#define DXVA_COPPKeyExchangeFnCode                  0x02
// lpInputData => NULL
// lpOuputData => GUID*

// -------------------------------------------------------------------------
// COPPSequenceStart
// -------------------------------------------------------------------------
typedef struct _DXVA_COPPSignature {
    UCHAR   Signature[256];
} DXVA_COPPSignature, *LPDXVA_COPPSignature;

#define DXVA_COPPSequenceStartFnCode                0x03
// lpInputData => DXVA_COPPSignature*
// lpOuputData => NULL

// -------------------------------------------------------------------------
// COPPCommand
// -------------------------------------------------------------------------
typedef struct _DXVA_COPPCommand {
    GUID    macKDI;             //   16 bytes
    GUID    guidCommandID;      //   16 bytes
    ULONG   dwSequence;         //    4 bytes
    ULONG   cbSizeData;         //    4 bytes
    UCHAR   CommandData[4056];  // 4056 bytes (4056+4+4+16+16 = 4096)
}  DXVA_COPPCommand, *LPDXVA_COPPCommand;

#define DXVA_COPPCommandFnCode                      0x04
// lpInputData => DXVA_COPPCommand*
// lpOuputData => NULL

DEFINE_GUID(DXVA_COPPSetProtectionLevel,
    0x9bb9327c,0x4eb5,0x4727,0x9f,0x00,0xb4,0x2b,0x09,0x19,0xc0,0xda);

typedef struct _DXVA_COPPSetProtectionLevelCmdData {
    ULONG   ProtType;
    ULONG   ProtLevel;
    ULONG   ExtendedInfoChangeMask;
    ULONG   ExtendedInfoData;
} DXVA_COPPSetProtectionLevelCmdData;

// Set the HDCP protection level - (0 - 1 DWORD, 4 bytes)

typedef enum _COPP_HDCP_Protection_Level {
    COPP_HDCP_Level0    = 0,
    COPP_HDCP_LevelMin  = COPP_HDCP_Level0,
    COPP_HDCP_Level1    = 1,
    COPP_HDCP_LevelMax  = COPP_HDCP_Level1,
    COPP_HDCP_ForceDWORD = 0x7fffffff
} COPP_HDCP_Protection_Level;

typedef enum _COPP_CGMSA_Protection_Level {
    COPP_CGMSA_Disabled = 0,
    COPP_CGMSA_LevelMin = COPP_CGMSA_Disabled,
    COPP_CGMSA_CopyFreely = 1,
    COPP_CGMSA_CopyNoMore = 2,
    COPP_CGMSA_CopyOneGeneration   = 3,
    COPP_CGMSA_CopyNever = 4,
    COPP_CGMSA_RedistributionControlRequired = 0x08,
    COPP_CGMSA_LevelMax = (COPP_CGMSA_RedistributionControlRequired + COPP_CGMSA_CopyNever),
    COPP_CGMSA_ForceDWORD = 0x7fffffff
} COPP_CGMSA_Protection_Level;

typedef enum _COPP_ACP_Protection_Level {
    COPP_ACP_Level0     = 0,
    COPP_ACP_LevelMin   = COPP_ACP_Level0,
    COPP_ACP_Level1     = 1,
    COPP_ACP_Level2     = 2,
    COPP_ACP_Level3     = 3,
    COPP_ACP_LevelMax   = COPP_ACP_Level3,
    COPP_ACP_ForceDWORD = 0x7fffffff
} COPP_ACP_Protection_Level;

#define COPP_NoProtectionLevelAvailable  -1
#define COPP_DefaultProtectionLevel 0

//
// Bit flags of possible protection types.  Note that it is possible to apply
// different protection settings to a single connector.
//
enum {
    COPP_ProtectionType_Unknown      = 0x80000000,
    COPP_ProtectionType_None         = 0x00000000,
    COPP_ProtectionType_HDCP         = 0x00000001, 	
    COPP_ProtectionType_ACP          = 0x00000002,
    COPP_ProtectionType_CGMSA        = 0x00000004,
    COPP_ProtectionType_Mask         = 0x80000007,
    COPP_ProtectionType_Reserved     = 0x7FFFFFF8
};

DEFINE_GUID(DXVA_COPPSetSignaling,
    0x9a631a5, 0xd684, 0x4c60, 0x8e, 0x4d, 0xd3, 0xbb, 0xf, 0xb, 0xe3, 0xee);

typedef struct _DXVA_COPPSetSignalingCmdData {
    ULONG   ActiveTVProtectionStandard;           // See COPP_TVProtectionStandard
    ULONG   AspectRatioChangeMask1;
    ULONG   AspectRatioData1;                     // See COPP_ImageAspectRatio_EN300294 for ETSI EN 300 294 values
    ULONG   AspectRatioChangeMask2;
    ULONG   AspectRatioData2;
    ULONG   AspectRatioChangeMask3;
    ULONG   AspectRatioData3;
    ULONG   ExtendedInfoChangeMask[4];
    ULONG   ExtendedInfoData[4];
    ULONG   Reserved;
} DXVA_COPPSetSignalingCmdData;

// Add format enum and data enum
typedef enum _COPP_TVProtectionStandard {
    COPP_ProtectionStandard_Unknown                         = 0x80000000,
    COPP_ProtectionStandard_None                            = 0x00000000,
    COPP_ProtectionStandard_IEC61880_525i                   = 0x00000001,
    COPP_ProtectionStandard_IEC61880_2_525i                 = 0x00000002,
    COPP_ProtectionStandard_IEC62375_625p                   = 0x00000004,
    COPP_ProtectionStandard_EIA608B_525                     = 0x00000008,
    COPP_ProtectionStandard_EN300294_625i                   = 0x00000010,
    COPP_ProtectionStandard_CEA805A_TypeA_525p              = 0x00000020,
    COPP_ProtectionStandard_CEA805A_TypeA_750p              = 0x00000040,
    COPP_ProtectionStandard_CEA805A_TypeA_1125i             = 0x00000080,
    COPP_ProtectionStandard_CEA805A_TypeB_525p              = 0x00000100,
    COPP_ProtectionStandard_CEA805A_TypeB_750p              = 0x00000200,
    COPP_ProtectionStandard_CEA805A_TypeB_1125i             = 0x00000400,
    COPP_ProtectionStandard_ARIBTRB15_525i                  = 0x00000800,
    COPP_ProtectionStandard_ARIBTRB15_525p                  = 0x00001000,
    COPP_ProtectionStandard_ARIBTRB15_750p                  = 0x00002000,
    COPP_ProtectionStandard_ARIBTRB15_1125i                 = 0x00004000,
    COPP_ProtectionStandard_Mask                            = 0x80007FFF,
    COPP_ProtectionStandard_Reserved                        = 0x7FFF8000
} COPP_TVProtectionStandard;

#define COPP_ImageAspectRatio_EN300294_Mask                 0x00000007

typedef enum _COPP_ImageAspectRatio_EN300294 {
    COPP_AspectRatio_EN300294_FullFormat4by3                = 0,
    COPP_AspectRatio_EN300294_Box14by9Center                = 1,
    COPP_AspectRatio_EN300294_Box14by9Top                   = 2,
    COPP_AspectRatio_EN300294_Box16by9Center                = 3,
    COPP_AspectRatio_EN300294_Box16by9Top                   = 4,
    COPP_AspectRatio_EN300294_BoxGT16by9Center              = 5,
    COPP_AspectRatio_EN300294_FullFormat4by3ProtectedCenter = 6,
    COPP_AspectRatio_EN300294_FullFormat16by9Anamorphic     = 7,
    COPP_AspectRatio_ForceDWORD                             = 0x7fffffff
} COPP_ImageAspectRatio_EN300294;

// -------------------------------------------------------------------------
// COPPQueryStatus
// -------------------------------------------------------------------------
typedef struct _DXVA_COPPStatusInput {
    GUID    rApp;               //   16 bytes
    GUID    guidStatusRequestID;//   16 bytes
    ULONG   dwSequence;         //    4 bytes
    ULONG   cbSizeData;         //    4 bytes
    UCHAR   StatusData[4056];   // 4056 bytes (4056+4+4+16+16 = 4096)
} DXVA_COPPStatusInput, *LPDXVA_COPPStatusInput;

typedef struct _DXVA_COPPStatusOutput {
    GUID    macKDI;             //   16 bytes
    ULONG   cbSizeData;         //    4 bytes
    UCHAR   COPPStatus[4076];   // 4076 bytes (4076+16+4 = 4096)
} DXVA_COPPStatusOutput, *LPDXVA_COPPStatusOutput;

typedef enum _COPP_StatusFlags {
    COPP_StatusNormal           = 0x00,
    COPP_LinkLost               = 0x01,
    COPP_RenegotiationRequired  = 0x02,
    COPP_StatusFlagsReserved    = 0xFFFFFFFC
} COPP_StatusFlags;

typedef struct _DXVA_COPPStatusData {
    GUID    rApp;
    ULONG   dwFlags;    // See COPP_StatusFlags above
    ULONG   dwData;
    ULONG   ExtendedInfoValidMask;
    ULONG   ExtendedInfoData;
} DXVA_COPPStatusData;

typedef struct _DXVA_COPPStatusDisplayData {
    GUID    rApp;
    ULONG   dwFlags;    // See COPP_StatusFlags above
    ULONG   DisplayWidth;
    ULONG   DisplayHeight;
    ULONG   Format;     // also contains extended color data
    ULONG   d3dFormat;
    ULONG   FreqNumerator;
    ULONG   FreqDenominator;
} DXVA_COPPStatusDisplayData;

typedef enum _COPP_StatusHDCPFlags {
    COPP_HDCPRepeater       = 0x01,
    COPP_HDCPFlagsReserved  = 0xFFFFFFFE
} COPP_StatusHDCPFlags;

typedef struct _DXVA_COPPStatusHDCPKeyData {
    GUID    rApp;
    ULONG   dwFlags;        // See COPP_StatusFlags above
    ULONG   dwHDCPFlags;    // See COPP_StatusHDCPFlags above
    GUID    BKey;           // Lower 40 bits
    GUID    Reserved1;
    GUID    Reserved2;
} DXVA_COPPStatusHDCPKeyData;

#define DXVA_COPPQueryStatusFnCode 0x05
// lpInputData => DXVA_COPPStatusInput*
// lpOuputData => DXVA_COPPStatusOutput*

//
// Status GUID and enumerations
//
DEFINE_GUID(DXVA_COPPQueryConnectorType,
      0x81d0bfd5,0x6afe,0x48c2,0x99,0xc0,0x95,0xa0,0x8f,0x97,0xc5,0xda);

typedef enum _COPP_ConnectorType {
    COPP_ConnectorType_Unknown = -1,
    COPP_ConnectorType_VGA = 0,
    COPP_ConnectorType_SVideo = 1,
    COPP_ConnectorType_CompositeVideo = 2,
    COPP_ConnectorType_ComponentVideo = 3,
    COPP_ConnectorType_DVI = 4,
    COPP_ConnectorType_HDMI = 5,
    COPP_ConnectorType_LVDS = 6,
    COPP_ConnectorType_TMDS = 7,
    COPP_ConnectorType_D_JPN = 8,
    COPP_ConnectorType_Internal = 0x80000000,   // can be combined with the other connector types
    COPP_ConnectorType_ForceDWORD = 0x7fffffff  /* force 32-bit size enum */
} COPP_ConnectorType;

DEFINE_GUID(DXVA_COPPQueryProtectionType,
    0x38f2a801,0x9a6c,0x48bb,0x91,0x07,0xb6,0x69,0x6e,0x6f,0x17,0x97);

DEFINE_GUID(DXVA_COPPQueryLocalProtectionLevel,
    0xb2075857,0x3eda,0x4d5d,0x88,0xdb,0x74,0x8f,0x8c,0x1a,0x05,0x49);

DEFINE_GUID(DXVA_COPPQueryGlobalProtectionLevel,
    0x1957210a,0x7766,0x452a,0xb9,0x9a,0xd2,0x7a,0xed,0x54,0xf0,0x3a);

DEFINE_GUID(DXVA_COPPQueryDisplayData,
    0xd7bf1ba3,0xad13,0x4f8e,0xaf,0x98,0x0d,0xcb,0x3c,0xa2,0x04,0xcc);

DEFINE_GUID(DXVA_COPPQueryHDCPKeyData,
    0xdb59d74, 0xa992, 0x492e, 0xa0, 0xbd, 0xc2, 0x3f, 0xda, 0x56, 0x4e, 0x0);

DEFINE_GUID(DXVA_COPPQueryBusData,
    0xc6f4d673, 0x6174, 0x4184, 0x8e, 0x35, 0xf6, 0xdb, 0x52, 0x0, 0xbc, 0xba);

typedef enum _COPP_BusType {
    COPP_BusType_Unknown    = 0,
    COPP_BusType_PCI        = 1,
    COPP_BusType_PCIX       = 2,
    COPP_BusType_PCIExpress = 3,
    COPP_BusType_AGP        = 4,
    COPP_BusType_Integrated = 0x80000000, // can be combined with the other bus types
    COPP_BusType_ForceDWORD = 0x7fffffff  /* force 32-bit size enum */
} COPP_BusType;

DEFINE_GUID(DXVA_COPPQuerySignaling,
    0x6629a591, 0x3b79, 0x4cf3, 0x92, 0x4a, 0x11, 0xe8, 0xe7, 0x81, 0x16, 0x71);

typedef struct _DXVA_COPPStatusSignalingCmdData {
    GUID    rApp;
    ULONG   dwFlags;                                // See COPP_StatusFlags above
    ULONG   AvailableTVProtectionStandards;         // See COPP_TVProtectionStandard
    ULONG   ActiveTVProtectionStandard;             // See COPP_TVProtectionStandard
    ULONG   TVType;
    ULONG   AspectRatioValidMask1;
    ULONG   AspectRatioData1;                       // See COPP_AspectRatio_EN300294 for ETSI EN 300 294 values
    ULONG   AspectRatioValidMask2;
    ULONG   AspectRatioData2;
    ULONG   AspectRatioValidMask3;
    ULONG   AspectRatioData3;
    ULONG   ExtendedInfoValidMask[4];
    ULONG   ExtendedInfoData[4];
} DXVA_COPPStatusSignalingCmdData;

#endif /* __DIRECTX_VA_CERTOUTPUTPROTECT__ */

#endif /* (DIRECT3D_VERSION >= 0x0900) */
#endif /* _DXVA9TYP_H_ */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


