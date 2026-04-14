//------------------------------------------------------------------------------
// File: DXVA.h
//
// Desc: DirectX Video Acceleration header file.
//
// Copyright (c) 1999 - 2002, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __DIRECTX_VA__
#define __DIRECTX_VA__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif

#pragma warning(disable:4201) // named type definition in parentheses

#ifdef __cplusplus
extern "C" {
#endif

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

DEFINE_GUID(DXVA_ModeNone,    0x1b81be00, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH261_A,  0x1b81be01, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH261_B,  0x1b81be02, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

DEFINE_GUID(DXVA_ModeH263_A,  0x1b81be03, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH263_B,  0x1b81be04, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH263_C,  0x1b81be05, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH263_D,  0x1b81be06, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH263_E,  0x1b81be07, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH263_F,  0x1b81be08, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

DEFINE_GUID(DXVA_ModeMPEG1_A, 0x1b81be09, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeMPEG1_VLD, 0x6f3ec719, 0x3735,0x42cc,0x80,0x63,0x65,0xcc,0x3c,0xb3,0x66,0x16);

DEFINE_GUID(DXVA_ModeMPEG2_A, 0x1b81be0A, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeMPEG2_B, 0x1b81be0B, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeMPEG2_C, 0x1b81be0C, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeMPEG2_D, 0x1b81be0D, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeMPEG2and1_VLD, 0x86695f12, 0x340e,0x4f04,0x9f,0xd3,0x92,0x53,0xdd,0x32,0x74,0x60);

DEFINE_GUID(DXVA_ModeH264_A,  0x1b81be64, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH264_B,  0x1b81be65, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH264_C,  0x1b81be66, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH264_D,  0x1b81be67, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH264_E,  0x1b81be68, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH264_F,  0x1b81be69, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeH264_VLD_WithFMOASO_NoFGT, 0xd5f04ff9, 0x3418,0x45d8,0x95,0x61,0x32,0xa7,0x6a,0xae,0x2d,0xdd);

DEFINE_GUID(DXVA_ModeH264_VLD_Stereo_Progressive_NoFGT,     0xd79be8da, 0x0cf1,0x4c81,0xb8,0x2a,0x69,0xa4,0xe2,0x36,0xf4,0x3d);
DEFINE_GUID(DXVA_ModeH264_VLD_Stereo_NoFGT,                 0xf9aaccbb, 0xc2b6,0x4cfc,0x87,0x79,0x57,0x07,0xb1,0x76,0x05,0x52);
DEFINE_GUID(DXVA_ModeH264_VLD_Multiview_NoFGT,              0x705b9d82, 0x76cf,0x49d6,0xb7,0xe6,0xac,0x88,0x72,0xdb,0x01,0x3c);

DEFINE_GUID(DXVA_ModeWMV8_A,  0x1b81be80, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeWMV8_B,  0x1b81be81, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

DEFINE_GUID(DXVA_ModeWMV9_A,  0x1b81be90, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeWMV9_B,  0x1b81be91, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeWMV9_C,  0x1b81be94, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

DEFINE_GUID(DXVA_ModeVC1_A,   0x1b81beA0, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeVC1_B,   0x1b81beA1, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeVC1_C,   0x1b81beA2, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeVC1_D,   0x1b81beA3, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);
DEFINE_GUID(DXVA_ModeVC1_D2010,0x1b81beA4, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

DEFINE_GUID(DXVA_ModeMPEG4pt2_VLD_Simple,           0xefd64d74, 0xc9e8,0x41d7,0xa5,0xe9,0xe9,0xb0,0xe3,0x9f,0xa3,0x19);
DEFINE_GUID(DXVA_ModeMPEG4pt2_VLD_AdvSimple_NoGMC,  0xed418a9f, 0x010d,0x4eda,0x9a,0xe3,0x9a,0x65,0x35,0x8d,0x8d,0x2e);
DEFINE_GUID(DXVA_ModeMPEG4pt2_VLD_AdvSimple_GMC,    0xab998b5b, 0x4258,0x44a9,0x9f,0xeb,0x94,0xe5,0x97,0xa6,0xba,0xae);

DEFINE_GUID(DXVA_ModeHEVC_VLD_Main,         0x5b11d51b, 0x2f4c, 0x4452,0xbc,0xc3,0x09,0xf2,0xa1,0x16,0x0c,0xc0);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main10,       0x107af0e0, 0xef1a, 0x4d19,0xab,0xa8,0x67,0xa1,0x63,0x07,0x3d,0x13);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Monochrome,   0x0685b993, 0x3d8c, 0x43a0, 0x8b, 0x28, 0xd7, 0x4c, 0x2d, 0x68, 0x99, 0xa4);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Monochrome10, 0x142a1d0f, 0x69dd, 0x4ec9, 0x85, 0x91, 0xb1, 0x2f, 0xfc, 0xb9, 0x1a, 0x29);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main12,       0x1a72925f, 0x0c2c, 0x4f15, 0x96, 0xfb, 0xb1, 0x7d, 0x14, 0x73, 0x60, 0x3f);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main10_422,   0x0bac4fe5, 0x1532, 0x4429, 0xa8, 0x54, 0xf8, 0x4d, 0xe0, 0x49, 0x53, 0xdb);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main12_422,   0x55bcac81, 0xf311, 0x4093, 0xa7, 0xd0, 0x1c, 0xbc, 0x0b, 0x84, 0x9b, 0xee);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main_444,     0x4008018f, 0xf537, 0x4b36, 0x98, 0xcf, 0x61, 0xaf, 0x8a, 0x2c, 0x1a, 0x33);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main10_Ext,   0x9cc55490, 0xe37c, 0x4932, 0x86, 0x84, 0x49, 0x20, 0xf9, 0xf6, 0x40, 0x9c);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main10_444,   0x0dabeffa, 0x4458, 0x4602, 0xbc, 0x03, 0x07, 0x95, 0x65, 0x9d, 0x61, 0x7c);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main12_444,   0x9798634d, 0xfe9d, 0x48e5, 0xb4, 0xda, 0xdb, 0xec, 0x45, 0xb3, 0xdf, 0x01);
DEFINE_GUID(DXVA_ModeHEVC_VLD_Main16,       0xa4fbdbb0, 0xa113, 0x482b, 0xa2, 0x32, 0x63, 0x5c, 0xc0, 0x69, 0x7f, 0x6d);

DEFINE_GUID(DXVA_ModeVP9_VLD_Profile0,       0x463707f8, 0xa1d0, 0x4585, 0x87, 0x6d, 0x83, 0xaa, 0x6d, 0x60, 0xb8, 0x9e);
DEFINE_GUID(DXVA_ModeVP9_VLD_10bit_Profile2, 0xa4c749ef, 0x6ecf, 0x48aa, 0x84, 0x48, 0x50, 0xa7, 0xa1, 0x16, 0x5f, 0xf7);
DEFINE_GUID(DXVA_ModeVP8_VLD,                0x90b899ea, 0x3a62, 0x4705, 0x88, 0xb3, 0x8d, 0xf0, 0x4b, 0x27, 0x44, 0xe7);

DEFINE_GUID(DXVA_ModeAV1_VLD_Profile0,           0xb8be4ccb, 0xcf53, 0x46ba, 0x8d, 0x59, 0xd6, 0xb8, 0xa6, 0xda, 0x5d, 0x2a);
DEFINE_GUID(DXVA_ModeAV1_VLD_Profile1,           0x6936ff0f, 0x45b1, 0x4163, 0x9c, 0xc1, 0x64, 0x6e, 0xf6, 0x94, 0x61, 0x08);
DEFINE_GUID(DXVA_ModeAV1_VLD_Profile2,           0x0c5f2aa1, 0xe541, 0x4089, 0xbb, 0x7b, 0x98, 0x11, 0x0a, 0x19, 0xd7, 0xc8);
DEFINE_GUID(DXVA_ModeAV1_VLD_12bit_Profile2,     0x17127009, 0xa00f, 0x4ce1, 0x99, 0x4e, 0xbf, 0x40, 0x81, 0xf6, 0xf3, 0xf0);
DEFINE_GUID(DXVA_ModeAV1_VLD_12bit_Profile2_420, 0x2d80bed6, 0x9cac, 0x4835, 0x9e, 0x91, 0x32, 0x7b, 0xbc, 0x4f, 0x9e, 0xe8);

DEFINE_GUID(DXVA_ModeMJPEG_VLD_420,   0x725cb506, 0x0c29, 0x43c4, 0x94, 0x40, 0x8e, 0x93, 0x97, 0x90, 0x3a, 0x04);
DEFINE_GUID(DXVA_ModeMJPEG_VLD_422,   0x5b77b9cd, 0x1a35, 0x4c30, 0x9f, 0xd8, 0xef, 0x4b, 0x60, 0xc0, 0x35, 0xdd);
DEFINE_GUID(DXVA_ModeMJPEG_VLD_444,   0xd95161f9, 0x0d44, 0x47e6, 0xbc, 0xf5, 0x1b, 0xfb, 0xfb, 0x26, 0x8f, 0x97);
DEFINE_GUID(DXVA_ModeMJPEG_VLD_4444,  0xc91748d5, 0xfd18, 0x4aca, 0x9d, 0xb3, 0x3a, 0x66, 0x34, 0xab, 0x54, 0x7d);
DEFINE_GUID(DXVA_ModeJPEG_VLD_420,    0xcf782c83, 0xbef5, 0x4a2c, 0x87, 0xcb, 0x60, 0x19, 0xe7, 0xb1, 0x75, 0xac);
DEFINE_GUID(DXVA_ModeJPEG_VLD_422,    0xf04df417, 0xeee2, 0x4067, 0xa7, 0x78, 0xf3, 0x5c, 0x15, 0xab, 0x97, 0x21);
DEFINE_GUID(DXVA_ModeJPEG_VLD_444,    0x4cd00e17, 0x89ba, 0x48ef, 0xb9, 0xf9, 0xed, 0xcb, 0x82, 0x71, 0x3f, 0x65);

DEFINE_GUID(DXVA_NoEncrypt,   0x1b81beD0, 0xa0c7,0x11d3,0xb9,0x84,0x00,0xc0,0x4f,0x2e,0x73,0xc5);

#define DXVA_ModeWMV8_PostProc                  DXVA_ModeWMV8_A
#define DXVA_ModeWMV8_MoComp                    DXVA_ModeWMV8_B

#define DXVA_ModeWMV9_PostProc                  DXVA_ModeWMV9_A
#define DXVA_ModeWMV9_MoComp                    DXVA_ModeWMV9_B
#define DXVA_ModeWMV9_IDCT                      DXVA_ModeWMV9_C

#define DXVA_ModeVC1_PostProc                   DXVA_ModeVC1_A
#define DXVA_ModeVC1_MoComp                     DXVA_ModeVC1_B
#define DXVA_ModeVC1_IDCT                       DXVA_ModeVC1_C
#define DXVA_ModeVC1_VLD                        DXVA_ModeVC1_D

#define DXVA_ModeH264_MoComp_NoFGT              DXVA_ModeH264_A
#define DXVA_ModeH264_MoComp_FGT                DXVA_ModeH264_B
#define DXVA_ModeH264_IDCT_NoFGT                DXVA_ModeH264_C
#define DXVA_ModeH264_IDCT_FGT                  DXVA_ModeH264_D
#define DXVA_ModeH264_VLD_NoFGT                 DXVA_ModeH264_E
#define DXVA_ModeH264_VLD_FGT                   DXVA_ModeH264_F

#define DXVA_RESTRICTED_MODE_UNRESTRICTED       0xFFFF
#define DXVA_RESTRICTED_MODE_H261_A             1
#define DXVA_RESTRICTED_MODE_H261_B             2

#define DXVA_RESTRICTED_MODE_H263_A             3
#define DXVA_RESTRICTED_MODE_H263_B             4
#define DXVA_RESTRICTED_MODE_H263_C             5
#define DXVA_RESTRICTED_MODE_H263_D             6
#define DXVA_RESTRICTED_MODE_H263_E             7
#define DXVA_RESTRICTED_MODE_H263_F             8

#define DXVA_RESTRICTED_MODE_MPEG1_A            9

#define DXVA_RESTRICTED_MODE_MPEG2_A            0xA
#define DXVA_RESTRICTED_MODE_MPEG2_B            0xB
#define DXVA_RESTRICTED_MODE_MPEG2_C            0xC
#define DXVA_RESTRICTED_MODE_MPEG2_D            0xD
#define DXVA_RESTRICTED_MODE_MPEG1_VLD          0x10
#define DXVA_RESTRICTED_MODE_MPEG2and1_VLD      0x11

#define DXVA_RESTRICTED_MODE_H264_A             0x64
#define DXVA_RESTRICTED_MODE_H264_B             0x65
#define DXVA_RESTRICTED_MODE_H264_C             0x66
#define DXVA_RESTRICTED_MODE_H264_D             0x67
#define DXVA_RESTRICTED_MODE_H264_E             0x68
#define DXVA_RESTRICTED_MODE_H264_F             0x69

#define DXVA_RESTRICTED_MODE_H264_VLD_WITHFMOASO_NOFGT             0x70

#define DXVA_RESTRICTED_MODE_H264_VLD_STEREO_PROGRESSIVE_NOFGT  0x71
#define DXVA_RESTRICTED_MODE_H264_VLD_STEREO_NOFGT              0x72
#define DXVA_RESTRICTED_MODE_H264_VLD_MULTIVIEW_NOFGT           0x73

#define DXVA_RESTRICTED_MODE_WMV8_A             0x80
#define DXVA_RESTRICTED_MODE_WMV8_B             0x81

#define DXVA_RESTRICTED_MODE_WMV9_A             0x90
#define DXVA_RESTRICTED_MODE_WMV9_B             0x91
#define DXVA_RESTRICTED_MODE_WMV9_C             0x94

#define DXVA_RESTRICTED_MODE_VC1_A              0xA0
#define DXVA_RESTRICTED_MODE_VC1_B              0xA1
#define DXVA_RESTRICTED_MODE_VC1_C              0xA2
#define DXVA_RESTRICTED_MODE_VC1_D              0xA3

#define DXVA_RESTRICTED_MODE_VC1_D2010              0xA4

#define DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_SIMPLE            0xB0
#define DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_ADV_SIMPLE_NOGMC  0xB1
#define DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_ADV_SIMPLE_GMC    0xB2

#define DXVA_RESTRICTED_MODE_WMV8_POSTPROC      DXVA_RESTRICTED_MODE_WMV8_A
#define DXVA_RESTRICTED_MODE_WMV8_MOCOMP        DXVA_RESTRICTED_MODE_WMV8_B

#define DXVA_RESTRICTED_MODE_WMV9_POSTPROC      DXVA_RESTRICTED_MODE_WMV9_A
#define DXVA_RESTRICTED_MODE_WMV9_MOCOMP        DXVA_RESTRICTED_MODE_WMV9_B
#define DXVA_RESTRICTED_MODE_WMV9_IDCT          DXVA_RESTRICTED_MODE_WMV9_C

#define DXVA_RESTRICTED_MODE_VC1_POSTPROC       DXVA_RESTRICTED_MODE_VC1_A
#define DXVA_RESTRICTED_MODE_VC1_MOCOMP         DXVA_RESTRICTED_MODE_VC1_B
#define DXVA_RESTRICTED_MODE_VC1_IDCT           DXVA_RESTRICTED_MODE_VC1_C
#define DXVA_RESTRICTED_MODE_VC1_VLD            DXVA_RESTRICTED_MODE_VC1_D

#define DXVA_RESTRICTED_MODE_H264_MOCOMP_NOFGT  DXVA_RESTRICTED_MODE_H264_A
#define DXVA_RESTRICTED_MODE_H264_MOCOMP_FGT    DXVA_RESTRICTED_MODE_H264_B
#define DXVA_RESTRICTED_MODE_H264_IDCT_NOFGT    DXVA_RESTRICTED_MODE_H264_C
#define DXVA_RESTRICTED_MODE_H264_IDCT_FGT      DXVA_RESTRICTED_MODE_H264_D
#define DXVA_RESTRICTED_MODE_H264_VLD_NOFGT     DXVA_RESTRICTED_MODE_H264_E
#define DXVA_RESTRICTED_MODE_H264_VLD_FGT       DXVA_RESTRICTED_MODE_H264_F

#define DXVA_COMPBUFFER_TYPE_THAT_IS_NOT_USED    0
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

/* H.264/AVC Additional buffer types */
#define DXVA_MOTION_VECTOR_BUFFER               16
#define DXVA_FILM_GRAIN_BUFFER                  17

#define DXVA_NUM_TYPES_COMP_BUFFERS             18

/* values for bDXVA_Func */
#define DXVA_PICTURE_DECODING_FUNCTION        1
#define DXVA_ALPHA_BLEND_DATA_LOAD_FUNCTION   2
#define DXVA_ALPHA_BLEND_COMBINATION_FUNCTION 3
#define DXVA_PICTURE_RESAMPLE_FUNCTION        4
#define DXVA_DEBLOCKING_FILTER_FUNCTION       5
#define DXVA_FILM_GRAIN_SYNTHESIS_FUNCTION    6
#define DXVA_STATUS_REPORTING_FUNCTION        7

/* values returned from Execute command in absence of read-back */
#define DXVA_EXECUTE_RETURN_OK                  0
#define DXVA_EXECUTE_RETURN_DATA_ERROR_MINOR    1
#define DXVA_EXECUTE_RETURN_DATA_ERROR_SIGNIF   2
#define DXVA_EXECUTE_RETURN_DATA_ERROR_SEVERE   3
#define DXVA_EXECUTE_RETURN_OTHER_ERROR_SEVERE  4


#pragma pack(push, BeforeDXVApacking, 1)


typedef struct _DXVA_ConnectMode {
  GUID guidMode;
  WORD wRestrictedMode;
} DXVA_ConnectMode, *LPDXVA_ConnectMode;


typedef DWORD DXVA_ConfigQueryOrReplyFunc, *LPDXVA_ConfigQueryOrReplyFunc;

#define DXVA_QUERYORREPLYFUNCFLAG_DECODER_PROBE_QUERY     0xFFFFF1
#define DXVA_QUERYORREPLYFUNCFLAG_DECODER_LOCK_QUERY      0xFFFFF5
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_COPY     0xFFFFF8
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_PLUS     0xFFFFF9
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_OK_COPY      0xFFFFFC
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_FALSE_PLUS  0xFFFFFB
#define DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_FALSE_PLUS   0xFFFFFF

#define readDXVA_QueryOrReplyFuncFlag(ptr)        ((*(ptr)) >> 8)

#define readDXVA_QueryOrReplyFuncFlag_ACCEL(ptr)  (((*(ptr)) >> 11) & 1)
#define readDXVA_QueryOrReplyFuncFlag_LOCK(ptr)   (((*(ptr)) >> 10) & 1)
#define readDXVA_QueryOrReplyFuncFlag_BAD(ptr)    (((*(ptr)) >>  9) & 1)
#define readDXVA_QueryOrReplyFuncFlag_PLUS(ptr)   (((*(ptr)) >>  8) & 1)

#define readDXVA_QueryOrReplyFuncFunc(ptr)        ((*(ptr)) & 0xFF)

#define writeDXVA_QueryOrReplyFunc(ptr, flg, fnc) ((*(ptr)) = ((flg) << 8) | (fnc))

#define setDXVA_QueryOrReplyFuncFlag(ptr, flg) ((*(ptr)) |= ((flg) << 8))
#define setDXVA_QueryOrReplyFuncFunc(ptr, fnc) ((*(ptr)) |= (fnc));

typedef DWORD DXVA_EncryptProtocolFunc, *LPDXVA_EncryptProtocolFunc;

#define DXVA_ENCRYPTPROTOCOLFUNCFLAG_HOST    0xFFFF00
#define DXVA_ENCRYPTPROTOCOLFUNCFLAG_ACCEL   0xFFFF08

#define readDXVA_EncryptProtocolFuncFlag(ptr)        ((*(ptr)) >> 8)
#define readDXVA_EncryptProtocolFuncFlag_ACCEL(ptr)  (((*(ptr)) >> 11) & 1)

#define readDXVA_EncryptProtocolFuncFunc(ptr)        ((*(ptr)) & 0xFF)

#define writeDXVA_EncryptProtocolFunc(ptr, flg, fnc) ((*(ptr)) = ((flg) << 8) | (fnc))

#define setDXVA_EncryptProtocolFuncFlag(ptr, flg) ((*(ptr)) |= ((flg) << 8))
#define setDXVA_EncryptProtocolFuncFunc(ptr, fnc) ((*(ptr)) |= (fnc));

typedef struct _DXVA_EncryptProtocolHeader {
  DXVA_EncryptProtocolFunc dwFunction;
  DWORD ReservedBits[3];
  GUID  guidEncryptProtocol;
} DXVA_EncryptProtocolHeader, *LPDXVA_EncryptProtocolHeader;

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


/* Picture Decoding Parameters */

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



/* Picture Resampling */

typedef struct _DXVA_PicResample {

  WORD  wPicResampleSourcePicIndex;
  WORD  wPicResampleDestPicIndex;

  WORD  wPicResampleRcontrol;
  BYTE  bPicResampleExtrapWidth;
  BYTE  bPicResampleExtrapHeight;

  DWORD dwPicResampleSourceWidth;
  DWORD dwPicResampleSourceHeight;

  DWORD dwPicResampleDestWidth;
  DWORD dwPicResampleDestHeight;

  DWORD dwPicResampleFullDestWidth;
  DWORD dwPicResampleFullDestHeight;

} DXVA_PicResample, *LPDXVA_PicResample;


#define DXVA_CHROMA_FORMAT_420 1
#define DXVA_CHROMA_FORMAT_422 2
#define DXVA_CHROMA_FORMAT_444 3

#define DXVA_PICTURE_STRUCTURE_TOP_FIELD    1
#define DXVA_PICTURE_STRUCTURE_BOTTOM_FIELD 2
#define DXVA_PICTURE_STRUCTURE_FRAME        3

#define DXVA_BIDIRECTIONAL_AVERAGING_MPEG2_ROUND 0
#define DXVA_BIDIRECTIONAL_AVERAGING_H263_TRUNC  1

#define DXVA_MV_PRECISION_AND_CHROMA_RELATION_MPEG2  0
#define DXVA_MV_PRECISION_AND_CHROMA_RELATION_H263   1
#define DXVA_MV_PRECISION_AND_CHROMA_RELATION_H261   2

#define DXVA_SCAN_METHOD_ZIG_ZAG              0
#define DXVA_SCAN_METHOD_ALTERNATE_VERTICAL   1
#define DXVA_SCAN_METHOD_ALTERNATE_HORIZONTAL 2
#define DXVA_SCAN_METHOD_ARBITRARY            3

#define DXVA_BITSTREAM_CONCEALMENT_NEED_UNLIKELY 0
#define DXVA_BITSTREAM_CONCEALMENT_NEED_MILD     1
#define DXVA_BITSTREAM_CONCEALMENT_NEED_LIKELY   2
#define DXVA_BITSTREAM_CONCEALMENT_NEED_SEVERE   3

#define DXVA_BITSTREAM_CONCEALMENT_METHOD_UNSPECIFIED 0
#define DXVA_BITSTREAM_CONCEALMENT_METHOD_INTRA       1
#define DXVA_BITSTREAM_CONCEALMENT_METHOD_FORWARD     2
#define DXVA_BITSTREAM_CONCEALMENT_METHOD_BACKWARD    3


/* Buffer Description Data */

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


/* Off-Host IDCT Coefficient Data Structures */

typedef struct _DXVA_TCoef4Group {
  BYTE  TCoefIDX[4];
  SHORT TCoefValue[4];
} DXVA_TCoef4Group, *LPDXVA_TCoef4Group;

typedef struct _DXVA_TCoefSingle {
  WORD  wIndexWithEOB;
  SHORT TCoefValue;
} DXVA_TCoefSingle, *LPDXVA_TCoefSingle;

/* Macros for Reading EOB and Index Values */

#define readDXVA_TCoefSingleIDX(ptr) ((ptr)->wIndexWithEOB >> 1)
#define readDXVA_TCoefSingleEOB(ptr) ((ptr)->wIndexWithEOB & 1)

/* Macro for Writing EOB and Index Values */

#define writeDXVA_TCoefSingleIndexWithEOB(ptr, idx, eob) ((ptr)->wIndexWithEOB = ((idx) << 1) | (eob))
#define setDXVA_TCoefSingleIDX(ptr, idx) ((ptr)->wIndexWithEOB |= ((idx) << 1))
#define setDXVA_TCoefSingleEOB(ptr)      ((ptr)->wIndexWithEOB |= 1)

/* Spatial-Domain Residual Difference Blocks */

#define DXVA_USUAL_BLOCK_WIDTH   8
#define DXVA_USUAL_BLOCK_HEIGHT  8
#define DXVA_USUAL_BLOCK_SIZE   (DXVA_USUAL_BLOCK_WIDTH * DXVA_USUAL_BLOCK_HEIGHT)

typedef SHORT       DXVA_Sample16[DXVA_USUAL_BLOCK_SIZE];
typedef signed char DXVA_Sample8 [DXVA_USUAL_BLOCK_SIZE];

/* Deblocking Filter Control Structure */

typedef BYTE DXVA_DeblockingEdgeControl;

typedef DXVA_DeblockingEdgeControl * LPDXVA_DeblockingEdgeControl;

/* Macros for Reading STRENGTH and FilterOn */

#define readDXVA_EdgeFilterStrength(ptr) ((*(ptr)) >> 1)
#define readDXVA_EdgeFilterOn(ptr)       ((*(ptr)) & 1)

/* Macro for Writing STRENGTH and FilterOn */

#define writeDXVA_DeblockingEdgeControl(ptr, str, fon) ((*(ptr)) = ((str) << 1) | (fon))
#define setDXVA_EdgeFilterStrength(ptr, str)           ((*(ptr)) |= ((str) << 1))
#define setDXVA_EdgeFilterOn(ptr)                      ((*(ptr)) |= 1)


/* Macroblock Control Command Data Structures */

typedef struct _DXVA_MVvalue {
  SHORT horz, vert;
} DXVA_MVvalue, *LPDXVA_MVvalue;


/* Inverse Quantization Matrices */

typedef struct _DXVA_QmatrixData {
  BYTE bNewQmatrix[4]; /* intra Y, inter Y, intra chroma, inter chroma */
  WORD Qmatrix[4][DXVA_USUAL_BLOCK_WIDTH * DXVA_USUAL_BLOCK_HEIGHT];
} DXVA_QmatrixData, *LPDXVA_QmatrixData;


/* Slice Control Buffer Data */

typedef struct _DXVA_SliceInfo {
  WORD  wHorizontalPosition;
  WORD  wVerticalPosition;
  DWORD dwSliceBitsInBuffer;
  DWORD dwSliceDataLocation;
  BYTE  bStartCodeBitOffset;
  BYTE  bReservedBits;
  WORD  wMBbitOffset;
  WORD  wNumberMBsInSlice;
  WORD  wQuantizerScaleCode;
  WORD  wBadSliceChopping;
} DXVA_SliceInfo, *LPDXVA_SliceInfo;


#define DXVA_NumMV_OBMC_off_BinPBwith4MV_off    4
#define DXVA_NumMV_OBMC_off_BinPBwith4MV_on  (4+1)
#define DXVA_NumMV_OBMC_on__BinPB_off         (10)
#define DXVA_NumMV_OBMC_on__BinPB_on          (11) /* not current standards */

#define DXVA_NumBlocksPerMB_420 (4+2+0)
#define DXVA_NumBlocksPerMB_422 (4+2+2)
#define DXVA_NumBlocksPerMB_444 (4+4+4)

/* Basic form for I pictures */
/* Host Residual Differences */
typedef struct _DXVA_MBctrl_I_HostResidDiff_1 {
  WORD  wMBaddress;
  WORD  wMBtype;
  DWORD dwMB_SNL;
  WORD  wPatternCode;
  WORD  wPC_Overflow; /* zero if not overflow format */
  DWORD dwReservedBits2;
} DXVA_MBctrl_I_HostResidDiff_1;

/* Basic form for I pictures */
/* Off-Host IDCT, 4:2:0 sampling */
typedef struct _DXVA_MBctrl_I_OffHostIDCT_1 {
  WORD  wMBaddress;
  WORD  wMBtype;
  DWORD dwMB_SNL;
  WORD  wPatternCode;
  BYTE  bNumCoef[DXVA_NumBlocksPerMB_420];
} DXVA_MBctrl_I_OffHostIDCT_1;

/* Basic form for P and B pictures */
/* Should also be used for concealment MVs in MPEG-2 I pictures */
/* Without OBMC, without BinPB and 4MV together, without MV RPS */
/* Host Residual Differences */
typedef struct _DXVA_MBctrl_P_HostResidDiff_1 {
  WORD         wMBaddress;
  WORD         wMBtype;
  DWORD        dwMB_SNL;
  WORD         wPatternCode;
  WORD         wPC_Overflow; /* zero if not overflow format */
  DWORD        dwReservedBits2;
  DXVA_MVvalue MVector[DXVA_NumMV_OBMC_off_BinPBwith4MV_off];
} DXVA_MBctrl_P_HostResidDiff_1;

/* Basic form for P and B pictures */
/* Without OBMC, without BinPB and 4MV together, without MV RPS */
/* Off-Host IDCT, 4:2:0 sampling */
typedef struct _DXVA_MBctrl_P_OffHostIDCT_1 {
  WORD         wMBaddress;
  WORD         wMBtype;
  DWORD        dwMB_SNL;
  WORD         wPatternCode;
  BYTE         bNumCoef[DXVA_NumBlocksPerMB_420];
  DXVA_MVvalue MVector[DXVA_NumMV_OBMC_off_BinPBwith4MV_off];
} DXVA_MBctrl_P_OffHostIDCT_1;

/* How to load alpha blending graphic data */
typedef struct _DXVA_ConfigAlphaLoad {

  // Operation Indicated
  DXVA_ConfigQueryOrReplyFunc dwFunction;

  // Alignment
  DWORD dwReservedBits[3];

  BYTE  bConfigDataType;
} DXVA_ConfigAlphaLoad, *LPDXVA_ConfigAlphaLoad;

#define DXVA_CONFIG_DATA_TYPE_IA44  0
#define DXVA_CONFIG_DATA_TYPE_AI44  1
#define DXVA_CONFIG_DATA_TYPE_DPXD  2
#define DXVA_CONFIG_DATA_TYPE_AYUV  3


/* How to combine alpha blending graphic data */
typedef struct _DXVA_ConfigAlphaCombine {

  // Operation Indicated
  DXVA_ConfigQueryOrReplyFunc dwFunction;

  // Alignment
  DWORD dwReservedBits[3];

  BYTE  bConfigBlendType;
  BYTE  bConfigPictureResizing;
  BYTE  bConfigOnlyUsePicDestRectArea;
  BYTE  bConfigGraphicResizing;
  BYTE  bConfigWholePlaneAlpha;

} DXVA_ConfigAlphaCombine, *LPDXVA_ConfigAlphaCombine;

#define DXVA_CONFIG_BLEND_TYPE_FRONT_BUFFER   0
#define DXVA_CONFIG_BLEND_TYPE_BACK_HARDWARE  1


/* AYUV sample for 16-entry YUV palette or graphic surface */

typedef struct _DXVA_AYUVsample2 {
  BYTE bCrValue;
  BYTE bCbValue;
  BYTE bY_Value;
  BYTE bSampleAlpha8;
} DXVA_AYUVsample2, *LPDXVA_AYUVsample2;

/* Macros for IA44 alpha blending surface samples */

typedef BYTE DXVA_IA44sample, *LPDXVA_IA44sample;

#define readDXVA_IA44index(ptr) (((*(ptr)) & 0xF0) >> 4)
#define readDXVA_IA44alpha(ptr)  ((*(ptr)) & 0x0F)

#define writeDXVA_IA44(ptr, idx, alpha) ((*(ptr)) = (((idx) << 4) | (alpha)))

#define setDXVA_IA44index(ptr, idx)    ((*(ptr)) |= ((idx) << 4))
#define setDXVA_IA44alpha(ptr, alpha)  ((*(ptr)) |= (alpha))

/* Macros for AI44 alpha blending surface samples */

typedef BYTE DXVA_AI44sample, *LPDXVA_AI44sample;

#define readDXVA_AI44index(ptr)  ((*(ptr)) & 0x0F)
#define readDXVA_AI44alpha(ptr) (((*(ptr)) & 0xF0) >> 4)

#define writeDXVA_AI44(ptr, idx, alpha) ((*(ptr)) = (((alpha) << 4) | (idx)))

#define setDXVA_AI44index(ptr, idx)    ((*(ptr)) |= (idx))
#define setDXVA_AI44alpha(ptr, alpha)  ((*(ptr)) |= ((alpha) << 4))


/* Highlight data structure */

typedef struct _DXVA_Highlight {
  WORD wHighlightActive;
  WORD wHighlightIndices;
  WORD wHighlightAlphas;
  RECT HighlightRect;
} DXVA_Highlight, *LPDXVA_Highlight;

typedef BYTE DXVA_DPXD,  *LPDXVA_DPXD;
typedef WORD DXVA_DCCMD, *LPDXVA_DCCMD;

/* Alpha blend combination */

typedef struct _DXVA_BlendCombination {
  WORD wPictureSourceIndex;
  WORD wBlendedDestinationIndex;
  RECT PictureSourceRect16thPel;
  RECT PictureDestinationRect;
  RECT GraphicSourceRect;
  RECT GraphicDestinationRect;
  WORD wBlendDelay;
  BYTE bBlendOn;
  BYTE bWholePlaneAlpha;
  DXVA_AYUVsample2 OutsideYUVcolor;
} DXVA_BlendCombination, *LPDXVA_BlendCombination;

/* H.264/AVC-specific structures */

/* H.264/AVC picture entry data structure */
typedef struct _DXVA_PicEntry_H264 {
  union {
    struct {
      UCHAR  Index7Bits      : 7;
      UCHAR  AssociatedFlag  : 1;
    };
    UCHAR  bPicEntry;
  };
} DXVA_PicEntry_H264, *LPDXVA_PicEntry_H264;  /* 1 byte */

/* H.264/AVC picture parameters structure */
typedef struct _DXVA_PicParams_H264 {
  USHORT  wFrameWidthInMbsMinus1;
  USHORT  wFrameHeightInMbsMinus1;
  DXVA_PicEntry_H264  CurrPic; /* flag is bot field flag */
  UCHAR   num_ref_frames;

  union {
    struct {
      USHORT  field_pic_flag                 : 1;
      USHORT  MbaffFrameFlag                 : 1;
      USHORT  residual_colour_transform_flag : 1;
      USHORT  sp_for_switch_flag             : 1;
      USHORT  chroma_format_idc              : 2;
      USHORT  RefPicFlag                     : 1;
      USHORT  constrained_intra_pred_flag    : 1;

      USHORT  weighted_pred_flag             : 1;
      USHORT  weighted_bipred_idc            : 2;
      USHORT  MbsConsecutiveFlag             : 1;
      USHORT  frame_mbs_only_flag            : 1;
      USHORT  transform_8x8_mode_flag        : 1;
      USHORT  MinLumaBipredSize8x8Flag       : 1;
      USHORT  IntraPicFlag                   : 1;
    };
    USHORT  wBitFields;
  };
  UCHAR  bit_depth_luma_minus8;
  UCHAR  bit_depth_chroma_minus8;

  USHORT Reserved16Bits;
  UINT   StatusReportFeedbackNumber;

  DXVA_PicEntry_H264  RefFrameList[16]; /* flag LT */
  INT    CurrFieldOrderCnt[2];
  INT    FieldOrderCntList[16][2];

  CHAR   pic_init_qs_minus26;
  CHAR   chroma_qp_index_offset;   /* also used for QScb */
  CHAR   second_chroma_qp_index_offset; /* also for QScr */
  UCHAR  ContinuationFlag;

/* remainder for parsing */
  CHAR   pic_init_qp_minus26;
  UCHAR  num_ref_idx_l0_active_minus1;
  UCHAR  num_ref_idx_l1_active_minus1;
  UCHAR  Reserved8BitsA;

  USHORT FrameNumList[16];
  UINT   UsedForReferenceFlags;
  USHORT NonExistingFrameFlags;
  USHORT frame_num;

  UCHAR  log2_max_frame_num_minus4;
  UCHAR  pic_order_cnt_type;
  UCHAR  log2_max_pic_order_cnt_lsb_minus4;
  UCHAR  delta_pic_order_always_zero_flag;

  UCHAR  direct_8x8_inference_flag;
  UCHAR  entropy_coding_mode_flag;
  UCHAR  pic_order_present_flag;
  UCHAR  num_slice_groups_minus1;

  UCHAR  slice_group_map_type;
  UCHAR  deblocking_filter_control_present_flag;
  UCHAR  redundant_pic_cnt_present_flag;
  UCHAR  Reserved8BitsB;

  USHORT slice_group_change_rate_minus1;

  UCHAR  SliceGroupMap[810]; /* 4b/sgmu, Size BT.601 */

} DXVA_PicParams_H264, *LPDXVA_PicParams_H264;

/* H.264/AVC quantization weighting matrix data structure */
typedef struct _DXVA_Qmatrix_H264 {
  UCHAR  bScalingLists4x4[6][16];
  UCHAR  bScalingLists8x8[2][64];

} DXVA_Qmatrix_H264, *LPDXVA_Qmatrix_H264;

/* H.264/AVC slice control data structure - short form */
typedef struct _DXVA_Slice_H264_Short {
  UINT   BSNALunitDataLocation; /* type 1..5 */
  UINT   SliceBytesInBuffer; /* for off-host parse */
  USHORT wBadSliceChopping;  /* for off-host parse */
} DXVA_Slice_H264_Short, *LPDXVA_Slice_H264_Short;

/* H.264/AVC picture entry data structure - long form */
typedef struct _DXVA_Slice_H264_Long {
  UINT   BSNALunitDataLocation; /* type 1..5 */
  UINT   SliceBytesInBuffer; /* for off-host parse */
  USHORT wBadSliceChopping;  /* for off-host parse */

  USHORT first_mb_in_slice;
  USHORT NumMbsForSlice;

  USHORT BitOffsetToSliceData; /* after CABAC alignment */

  UCHAR  slice_type;
  UCHAR  luma_log2_weight_denom;
  UCHAR  chroma_log2_weight_denom;
  UCHAR  num_ref_idx_l0_active_minus1;
  UCHAR  num_ref_idx_l1_active_minus1;
  CHAR   slice_alpha_c0_offset_div2;
  CHAR   slice_beta_offset_div2;
  UCHAR  Reserved8Bits;
  DXVA_PicEntry_H264 RefPicList[2][32]; /* L0 & L1 */
  SHORT  Weights[2][32][3][2]; /* L0 & L1; Y, Cb, Cr */
  CHAR   slice_qs_delta;
                               /* rest off-host parse */
  CHAR   slice_qp_delta;
  UCHAR  redundant_pic_cnt;
  UCHAR  direct_spatial_mv_pred_flag;
  UCHAR  cabac_init_idc;
  UCHAR  disable_deblocking_filter_idc;
  USHORT slice_id;
} DXVA_Slice_H264_Long, *LPDXVA_Slice_H264_Long;

/* H.264/AVC macroblock control command data structure */
typedef struct _DXVA_MBctrl_H264 {
  union {
    struct {
      UINT  bSliceID                : 8;   /* 1 byte */
      UINT  MbType5Bits             : 5;
      UINT  IntraMbFlag             : 1;
      UINT  mb_field_decoding_flag  : 1;
      UINT  transform_size_8x8_flag : 1;   /* 2 bytes */
      UINT  HostResidDiff           : 1;
      UINT  DcBlockCodedCrFlag      : 1;
      UINT  DcBlockCodedCbFlag      : 1;
      UINT  DcBlockCodedYFlag       : 1;
      UINT  FilterInternalEdgesFlag : 1;
      UINT  FilterLeftMbEdgeFlag    : 1;
      UINT  FilterTopMbEdgeFlag     : 1;
      UINT  ReservedBit             : 1;
      UINT  bMvQuantity             : 8;   /* 4 bytes */
    };
    UINT  dwMBtype;                    /* 4 bytes so far */
  };
  USHORT  CurrMbAddr;                  /* 6 bytes so far */
  USHORT  wPatternCode[3];/* YCbCr, 16 4x4 blks, 1b each */
                                      /* 12 bytes so far */
  UCHAR   bQpPrime[3];    /* Y, Cb, Cr, need just 7b QpY */
  UCHAR   bMBresidDataQuantity;
  ULONG   dwMBdataLocation;  /* offset into resid buffer */
                                      /* 20 bytes so far */
  union {
    struct {
/* start here for Intra MB's  (9 useful bytes in branch) */
      USHORT LumaIntraPredModes[4];/* 16 blocks, 4b each */
                                      /* 28 bytes so far */
      union {
        struct {
          UCHAR  intra_chroma_pred_mode : 2;
          UCHAR  IntraPredAvailFlags    : 5;
          UCHAR  ReservedIntraBit       : 1;
        };
        UCHAR  bMbIntraStruct;        /* 29 bytes so far */
      };
      UCHAR ReservedIntra24Bits[3];   /* 32 bytes total  */
    };
    struct {
/* start here for non-Intra MB's (12 bytes in branch)    */
      UCHAR  bSubMbShapes;          /* 4 subMbs, 2b each */
      UCHAR  bSubMbPredModes;       /* 4 subMBs, 2b each */
                                      /* 22 bytes so far */
      USHORT wMvBuffOffset;     /* offset into MV buffer */
      UCHAR  bRefPicSelect[2][4];     /* 32 bytes total */
    };
  };
} DXVA_MBctrl_H264, *LPDXVA_MBctrl_H264;

/* H.264/AVC IndexA and IndexB data structure */
typedef struct _DXVA_DeblockIndexAB_H264 {
  UCHAR  bIndexAinternal; /* 6b - could get from MB CC */
  UCHAR  bIndexBinternal; /* 6b - could get from MB CC */

  UCHAR  bIndexAleft0;
  UCHAR  bIndexBleft0;

  UCHAR  bIndexAleft1;
  UCHAR  bIndexBleft1;

  UCHAR  bIndexAtop0;
  UCHAR  bIndexBtop0;

  UCHAR  bIndexAtop1;
  UCHAR  bIndexBtop1;
} DXVA_DeblockIndexAB_H264, *LPDXVA_DeblockIndexAB_H264;
                                /* 10 bytes in struct */

/* H.264/AVC deblocking filter control data structure */
typedef struct _DXVA_Deblock_H264 {
  USHORT  CurrMbAddr; /* dup info */   /* 2 bytes so far */
  union {
    struct {
      UCHAR  ReservedBit                : 1;
      UCHAR  FieldModeCurrentMbFlag     : 1; /* dup info */
      UCHAR  FieldModeLeftMbFlag        : 1;
      UCHAR  FieldModeAboveMbFlag       : 1;
      UCHAR  FilterInternal8x8EdgesFlag : 1;
      UCHAR  FilterInternal4x4EdgesFlag : 1;
      UCHAR  FilterLeftMbEdgeFlag       : 1;
      UCHAR  FilterTopMbEdgeFlag        : 1;
    };
    UCHAR  FirstByte;
  };
  UCHAR  Reserved8Bits;      /* 4 bytes so far */

  UCHAR  bbSinternalLeftVert; /* 2 bits per bS */
  UCHAR  bbSinternalMidVert;

  UCHAR  bbSinternalRightVert;
  UCHAR  bbSinternalTopHorz;  /* 8 bytes so far */

  UCHAR  bbSinternalMidHorz;
  UCHAR  bbSinternalBotHorz;       /* 10 bytes so far */

  USHORT wbSLeft0; /* 4 bits per bS (1 wasted) */
  USHORT wbSLeft1; /* 4 bits per bS (1 wasted) */

  USHORT wbSTop0;  /* 4 bits per bS (1 wasted) */
  USHORT wbSTop1;  /* 4b (2 wasted)  18 bytes so far*/

  DXVA_DeblockIndexAB_H264  IndexAB[3]; /* Y, Cb, Cr */

} DXVA_Deblock_H264, *LPDXVA_Deblock_H264;/* 48 bytes */

/* H.264/AVC film grain characteristics data structure */
typedef struct _DXVA_FilmGrainCharacteristics {

  USHORT  wFrameWidthInMbsMinus1;
  USHORT  wFrameHeightInMbsMinus1;

  DXVA_PicEntry_H264  InPic; /* flag is bot field flag */
  DXVA_PicEntry_H264  OutPic; /* flag is field pic flag */

  USHORT PicOrderCnt_offset;
  INT    CurrPicOrderCnt;
  UINT   StatusReportFeedbackNumber;

  UCHAR model_id;
  UCHAR separate_colour_description_present_flag;
  UCHAR film_grain_bit_depth_luma_minus8;
  UCHAR film_grain_bit_depth_chroma_minus8;

  UCHAR film_grain_full_range_flag;
  UCHAR film_grain_colour_primaries;
  UCHAR film_grain_transfer_characteristics;
  UCHAR film_grain_matrix_coefficients;

  UCHAR blending_mode_id;
  UCHAR log2_scale_factor;

  UCHAR comp_model_present_flag[4];
  UCHAR num_intensity_intervals_minus1[4];
  UCHAR num_model_values_minus1[4];

  UCHAR intensity_interval_lower_bound[3][16];
  UCHAR intensity_interval_upper_bound[3][16];
  SHORT comp_model_value[3][16][8];
} DXVA_FilmGrainChar_H264, *LPDXVA_FilmGrainChar_H264;

/* H.264/AVC status reporting data structure */
typedef struct _DXVA_Status_H264 {
  UINT   StatusReportFeedbackNumber;
  DXVA_PicEntry_H264  CurrPic; /* flag is bot field flag */
  UCHAR  field_pic_flag;
  UCHAR  bDXVA_Func;
  UCHAR  bBufType;
  UCHAR  bStatus;
  UCHAR  bReserved8Bits;
  USHORT wNumMbsAffected;
} DXVA_Status_H264, *LPDXVA_Status_H264;

/* H.264 MVC picture parameters structure */
typedef struct _DXVA_PicParams_H264_MVC {
  USHORT  wFrameWidthInMbsMinus1;
  USHORT  wFrameHeightInMbsMinus1;
  DXVA_PicEntry_H264  CurrPic; /* flag is bot field flag */
  UCHAR   num_ref_frames;

  union {
    struct {
      USHORT  field_pic_flag                 : 1;
      USHORT  MbaffFrameFlag                 : 1;
      USHORT  residual_colour_transform_flag : 1;
      USHORT  sp_for_switch_flag             : 1;
      USHORT  chroma_format_idc              : 2;
      USHORT  RefPicFlag                     : 1;
      USHORT  constrained_intra_pred_flag    : 1;

      USHORT  weighted_pred_flag             : 1;
      USHORT  weighted_bipred_idc            : 2;
      USHORT  MbsConsecutiveFlag             : 1;
      USHORT  frame_mbs_only_flag            : 1;
      USHORT  transform_8x8_mode_flag        : 1;
      USHORT  MinLumaBipredSize8x8Flag       : 1;
      USHORT  IntraPicFlag                   : 1;
    };
    USHORT  wBitFields;
  };
  UCHAR  bit_depth_luma_minus8;
  UCHAR  bit_depth_chroma_minus8;

  USHORT Reserved16Bits;
  UINT   StatusReportFeedbackNumber;

  DXVA_PicEntry_H264  RefFrameList[16]; /* flag LT */
  INT    CurrFieldOrderCnt[2];
  INT    FieldOrderCntList[16][2];

  CHAR   pic_init_qs_minus26;
  CHAR   chroma_qp_index_offset;   /* also used for QScb */
  CHAR   second_chroma_qp_index_offset; /* also for QScr */
  UCHAR  ContinuationFlag;

/* remainder for parsing */
  CHAR   pic_init_qp_minus26;
  UCHAR  num_ref_idx_l0_active_minus1;
  UCHAR  num_ref_idx_l1_active_minus1;
  UCHAR  Reserved8BitsA;

  USHORT FrameNumList[16];
  UINT   UsedForReferenceFlags;
  USHORT NonExistingFrameFlags;
  USHORT frame_num;

  UCHAR  log2_max_frame_num_minus4;
  UCHAR  pic_order_cnt_type;
  UCHAR  log2_max_pic_order_cnt_lsb_minus4;
  UCHAR  delta_pic_order_always_zero_flag;

  UCHAR  direct_8x8_inference_flag;
  UCHAR  entropy_coding_mode_flag;
  UCHAR  pic_order_present_flag;
  UCHAR  num_slice_groups_minus1;

  UCHAR  slice_group_map_type;
  UCHAR  deblocking_filter_control_present_flag;
  UCHAR  redundant_pic_cnt_present_flag;
  UCHAR  Reserved8BitsB;

  USHORT slice_group_change_rate_minus1;
  /* SliceGroupMap is not needed for MVC, as MVC is for high profile only */

  /* Following are H.264 MVC Specific parameters */
  UCHAR   num_views_minus1;
  USHORT  view_id[16];
  UCHAR   num_anchor_refs_l0[16];
  USHORT  anchor_ref_l0[16][16];
  UCHAR   num_anchor_refs_l1[16];
  USHORT  anchor_ref_l1[16][16];
  UCHAR   num_non_anchor_refs_l0[16];
  USHORT  non_anchor_ref_l0[16][16];
  UCHAR   num_non_anchor_refs_l1[16];
  USHORT  non_anchor_ref_l1[16][16];

  USHORT curr_view_id; 
  UCHAR  anchor_pic_flag;
  UCHAR  inter_view_flag;
  USHORT ViewIDList[16];

} DXVA_PicParams_H264_MVC, *LPDXVA_PicParams_H264_MVC;

/* VC-1 status reporting data structure */
typedef struct _DXVA_Status_VC1 {
  USHORT StatusReportFeedbackNumber;
  WORD   wDecodedPictureIndex;
  WORD   wDeblockedPictureIndex;
  UCHAR  bPicStructure;
  UCHAR  bBufType;
  UCHAR  bStatus;
  UCHAR  bReserved8Bits;
  USHORT wNumMbsAffected;
} DXVA_Status_VC1, *LPDXVA_Status_VC1;


/* MPEG4PT2 Picture Parameter structure */
typedef struct _DXVA_PicParams_MPEG4_PART2 {
    UCHAR  short_video_header;
    UCHAR  vop_coding_type;
    UCHAR  vop_quant;
    WORD   wDecodedPictureIndex;
    WORD   wDeblockedPictureIndex;
    WORD   wForwardRefPictureIndex;
    WORD   wBackwardRefPictureIndex;
    USHORT vop_time_increment_resolution;
    UINT   TRB[2];
    UINT   TRD[2];

    union {
        struct {
            USHORT  unPicPostProc                 : 2;
            USHORT  interlaced                    : 1;
            USHORT  quant_type                    : 1;
            USHORT  quarter_sample                : 1;
            USHORT  resync_marker_disable         : 1;
            USHORT  data_partitioned              : 1;
            USHORT  reversible_vlc                : 1;
            USHORT  reduced_resolution_vop_enable : 1;
            USHORT  vop_coded                     : 1;
            USHORT  vop_rounding_type             : 1;
            USHORT  intra_dc_vlc_thr              : 3;
            USHORT  top_field_first               : 1;
            USHORT  alternate_vertical_scan_flag  : 1;
        };
        USHORT wPicFlagBitFields;
    };
    UCHAR  profile_and_level_indication;
    UCHAR  video_object_layer_verid;
    WORD   vop_width;
    WORD   vop_height;
    union {
        struct {
            USHORT  sprite_enable               : 2;
            USHORT  no_of_sprite_warping_points : 6;
            USHORT  sprite_warping_accuracy     : 2;
        };
        USHORT wSpriteBitFields;
    };
    SHORT warping_mv[4][2];
    union {
        struct {
            UCHAR  vop_fcode_forward   : 3;
            UCHAR  vop_fcode_backward  : 3;
        };
        UCHAR wFcodeBitFields;
    };
    USHORT  StatusReportFeedbackNumber;
    USHORT  Reserved16BitsA;
    USHORT  Reserved16BitsB;
} DXVA_PicParams_MPEG4_PART2, *LPDXVA_PicParams_MPEG4_PART2;


/* HEVC Picture Entry structure */
typedef struct _DXVA_PicEntry_HEVC
{ 
    union 
    { 
        struct 
        { 
            UCHAR Index7Bits : 7; 
            UCHAR AssociatedFlag : 1; 
        }; 
        UCHAR bPicEntry; 
    }; 
} DXVA_PicEntry_HEVC, *LPDXVA_PicEntry_HEVC;

/* HEVC Picture Parameter structure */
typedef struct _DXVA_PicParams_HEVC {
    USHORT      PicWidthInMinCbsY;
    USHORT      PicHeightInMinCbsY;
    union {
        struct {
            USHORT  chroma_format_idc                       : 2;
            USHORT  separate_colour_plane_flag              : 1;
            USHORT  bit_depth_luma_minus8                   : 3; 
            USHORT  bit_depth_chroma_minus8                 : 3;
            USHORT  log2_max_pic_order_cnt_lsb_minus4       : 4;
            USHORT  NoPicReorderingFlag                     : 1;
            USHORT  NoBiPredFlag                            : 1;
            USHORT  ReservedBits1                            : 1;
        };
        USHORT wFormatAndSequenceInfoFlags;
    };
    DXVA_PicEntry_HEVC  CurrPic;
    UCHAR   sps_max_dec_pic_buffering_minus1;
    UCHAR   log2_min_luma_coding_block_size_minus3;
    UCHAR   log2_diff_max_min_luma_coding_block_size;
    UCHAR   log2_min_transform_block_size_minus2;
    UCHAR   log2_diff_max_min_transform_block_size;
    UCHAR   max_transform_hierarchy_depth_inter;
    UCHAR   max_transform_hierarchy_depth_intra;
    UCHAR   num_short_term_ref_pic_sets;
    UCHAR   num_long_term_ref_pics_sps;
    UCHAR   num_ref_idx_l0_default_active_minus1;
    UCHAR   num_ref_idx_l1_default_active_minus1;
    CHAR    init_qp_minus26;
    UCHAR   ucNumDeltaPocsOfRefRpsIdx;
    USHORT  wNumBitsForShortTermRPSInSlice;
    USHORT  ReservedBits2;

    union {
        struct {
            UINT32  scaling_list_enabled_flag                    : 1;
            UINT32  amp_enabled_flag                            : 1;
            UINT32  sample_adaptive_offset_enabled_flag         : 1;
            UINT32  pcm_enabled_flag                            : 1;
            UINT32  pcm_sample_bit_depth_luma_minus1            : 4;
            UINT32  pcm_sample_bit_depth_chroma_minus1          : 4;
            UINT32  log2_min_pcm_luma_coding_block_size_minus3  : 2;
            UINT32  log2_diff_max_min_pcm_luma_coding_block_size : 2;
            UINT32  pcm_loop_filter_disabled_flag                : 1;
            UINT32  long_term_ref_pics_present_flag             : 1;
            UINT32  sps_temporal_mvp_enabled_flag               : 1;
            UINT32  strong_intra_smoothing_enabled_flag         : 1;
            UINT32  dependent_slice_segments_enabled_flag       : 1;
            UINT32  output_flag_present_flag                    : 1;
            UINT32  num_extra_slice_header_bits                 : 3;
            UINT32  sign_data_hiding_enabled_flag               : 1;
            UINT32  cabac_init_present_flag                     : 1;
            UINT32  ReservedBits3                               : 5;
        };
        UINT32 dwCodingParamToolFlags;
    };

    union {
        struct {
            UINT32  constrained_intra_pred_flag                 : 1;
            UINT32  transform_skip_enabled_flag                 : 1;
            UINT32  cu_qp_delta_enabled_flag                    : 1;
            UINT32  pps_slice_chroma_qp_offsets_present_flag    : 1;
            UINT32  weighted_pred_flag                          : 1;
            UINT32  weighted_bipred_flag                        : 1;
            UINT32  transquant_bypass_enabled_flag              : 1;
            UINT32  tiles_enabled_flag                          : 1;
            UINT32  entropy_coding_sync_enabled_flag            : 1;
            UINT32  uniform_spacing_flag                        : 1;
            UINT32  loop_filter_across_tiles_enabled_flag       : 1;
            UINT32  pps_loop_filter_across_slices_enabled_flag  : 1;
            UINT32  deblocking_filter_override_enabled_flag     : 1;
            UINT32  pps_deblocking_filter_disabled_flag         : 1;
            UINT32  lists_modification_present_flag             : 1;
            UINT32  slice_segment_header_extension_present_flag : 1;
            UINT32  IrapPicFlag                                 : 1;
            UINT32  IdrPicFlag                                  : 1;
            UINT32  IntraPicFlag                                : 1;
            UINT32  ReservedBits4                               : 13;
        };
        UINT32 dwCodingSettingPicturePropertyFlags;
    };
    CHAR    pps_cb_qp_offset;
    CHAR    pps_cr_qp_offset;
    UCHAR   num_tile_columns_minus1;
    UCHAR   num_tile_rows_minus1;
    USHORT  column_width_minus1[19];
    USHORT  row_height_minus1[21];
    UCHAR   diff_cu_qp_delta_depth;
    CHAR    pps_beta_offset_div2;
    CHAR    pps_tc_offset_div2;
    UCHAR   log2_parallel_merge_level_minus2;
    INT     CurrPicOrderCntVal;
    DXVA_PicEntry_HEVC	RefPicList[15];
    UCHAR   ReservedBits5;
    INT     PicOrderCntValList[15];
    UCHAR   RefPicSetStCurrBefore[8];
    UCHAR   RefPicSetStCurrAfter[8];
    UCHAR   RefPicSetLtCurr[8];
    USHORT  ReservedBits6;
    USHORT  ReservedBits7;
    UINT    StatusReportFeedbackNumber;
} DXVA_PicParams_HEVC, *LPDXVA_PicParams_HEVC;

/* HEVC Picture Parameter structure for Range Extensions */
typedef struct _DXVA_PicParams_HEVC_RangeExt {
    DXVA_PicParams_HEVC params;

    union {
        struct {
            USHORT transform_skip_rotation_enabled_flag    : 1;
            USHORT transform_skip_context_enabled_flag     : 1;
            USHORT implicit_rdpcm_enabled_flag             : 1;
            USHORT explicit_rdpcm_enabled_flag             : 1;
            USHORT extended_precision_processing_flag      : 1;
            USHORT intra_smoothing_disabled_flag           : 1;
            USHORT persistent_rice_adaptation_enabled_flag : 1;
            USHORT high_precision_offsets_enabled_flag     : 1;
            USHORT cabac_bypass_alignment_enabled_flag     : 1;
            USHORT cross_component_prediction_enabled_flag : 1;
            USHORT chroma_qp_offset_list_enabled_flag      : 1; 
            USHORT ReservedBits8                           : 5;
        };
        USHORT dwRangeExtensionFlags;
    };

    UCHAR diff_cu_chroma_qp_offset_depth;
    UCHAR log2_sao_offset_scale_luma;
    UCHAR log2_sao_offset_scale_chroma;
    UCHAR log2_max_transform_skip_block_size_minus2;
    CHAR cb_qp_offset_list[6];
    CHAR cr_qp_offset_list[6];
    UCHAR chroma_qp_offset_list_len_minus1;
    USHORT ReservedBits9;

} DXVA_PicParams_HEVC_RangeExt, *LPDXVA_PicParams_HEVC_RangeExt;

/* HEVC Quantizatiuon Matrix structure */
typedef struct _DXVA_Qmatrix_HEVC 
{
    UCHAR ucScalingLists0[6][16]; 
    UCHAR ucScalingLists1[6][64]; 
    UCHAR ucScalingLists2[6][64]; 
    UCHAR ucScalingLists3[2][64]; 
    UCHAR ucScalingListDCCoefSizeID2[6];
    UCHAR ucScalingListDCCoefSizeID3[2];
} DXVA_Qmatrix_HEVC, *LPDXVA_Qmatrix_HEVC;


/* HEVC Slice Control Structure */
typedef struct _DXVA_Slice_HEVC_Short 
{ 
    UINT    BSNALunitDataLocation; 
    UINT    SliceBytesInBuffer; 
    USHORT  wBadSliceChopping; 
} DXVA_Slice_HEVC_Short, *LPDXVA_Slice_HEVC_Short;

/* H.265/HEVC status reporting data structure */
typedef struct _DXVA_Status_HEVC {
    USHORT StatusReportFeedbackNumber;
    DXVA_PicEntry_HEVC CurrPic;
    UCHAR  bBufType;
    UCHAR  bStatus;
    UCHAR  bReserved8Bits;
    USHORT wNumMbsAffected;
} DXVA_Status_HEVC, *LPDXVA_Status_HEVC;

#pragma pack(push, 16)


typedef DXVA_MBctrl_I_HostResidDiff_1 *
       LPDXVA_MBctrl_I_HostResidDiff_1;

typedef DXVA_MBctrl_I_OffHostIDCT_1 *
       LPDXVA_MBctrl_I_OffHostIDCT_1;

typedef DXVA_MBctrl_P_HostResidDiff_1 *
       LPDXVA_MBctrl_P_HostResidDiff_1;

typedef DXVA_MBctrl_P_OffHostIDCT_1 *
       LPDXVA_MBctrl_P_OffHostIDCT_1;


#pragma pack(pop)

/* VPx-specific structures */

/* VPx picture entry data structure */
typedef struct _DXVA_PicEntry_VPx {
    union {
        struct {
            UCHAR Index7Bits : 7;
            UCHAR AssociatedFlag : 1;
        };
        UCHAR bPicEntry;
    };
} DXVA_PicEntry_VPx, *LPDXVA_PicEntry_VPx;

/* VP9 segmentation structure */
typedef struct _segmentation_VP9 {
    union {
        struct {
            UCHAR enabled : 1;
            UCHAR update_map : 1;
            UCHAR temporal_update : 1;
            UCHAR abs_delta : 1;
            UCHAR ReservedSegmentFlags4Bits : 4;
        };
        UCHAR wSegmentInfoFlags;
    };
    UCHAR tree_probs[7];
    UCHAR pred_probs[3];
    SHORT feature_data[8][4];
    UCHAR feature_mask[8];
} DXVA_segmentation_VP9;

/* VP9 picture parameters structure */
typedef struct _DXVA_PicParams_VP9 {
    DXVA_PicEntry_VPx    CurrPic;
    UCHAR                profile;
    union {
        struct {
            USHORT frame_type : 1;
            USHORT show_frame : 1;
            USHORT error_resilient_mode : 1;
            USHORT subsampling_x : 1;
            USHORT subsampling_y : 1;
            USHORT extra_plane : 1;
            USHORT refresh_frame_context : 1;
            USHORT frame_parallel_decoding_mode : 1;
            USHORT intra_only : 1;
            USHORT frame_context_idx : 2;
            USHORT reset_frame_context : 2;
            USHORT allow_high_precision_mv : 1;
            USHORT ReservedFormatInfo2Bits : 2;
        };
        USHORT wFormatAndPictureInfoFlags;
    };
    UINT  width;
    UINT  height;
    UCHAR BitDepthMinus8Luma;
    UCHAR BitDepthMinus8Chroma;
    UCHAR interp_filter;
    UCHAR Reserved8Bits;
    DXVA_PicEntry_VPx  ref_frame_map[8];
    UINT  ref_frame_coded_width[8];
    UINT  ref_frame_coded_height[8];
    DXVA_PicEntry_VPx  frame_refs[3];
    CHAR  ref_frame_sign_bias[4];
    CHAR  filter_level;
    CHAR  sharpness_level;
    union {
        struct {
            UCHAR mode_ref_delta_enabled : 1;
            UCHAR mode_ref_delta_update : 1;
            UCHAR use_prev_in_find_mv_refs : 1;
            UCHAR ReservedControlInfo5Bits : 5;
        };
        UCHAR wControlInfoFlags;
    };
    CHAR   ref_deltas[4];
    CHAR   mode_deltas[2];
    SHORT  base_qindex;
    CHAR   y_dc_delta_q;
    CHAR   uv_dc_delta_q;
    CHAR   uv_ac_delta_q;
    DXVA_segmentation_VP9 stVP9Segments;
    UCHAR  log2_tile_cols;
    UCHAR  log2_tile_rows;
    USHORT uncompressed_header_size_byte_aligned;
    USHORT first_partition_size;
    USHORT Reserved16Bits;
    UINT   Reserved32Bits;
    UINT   StatusReportFeedbackNumber;
} DXVA_PicParams_VP9, *LPDXVA_PicParams_VP9;

/* VP8 segmentation structure */
typedef struct _segmentation_VP8 {
    union {
        struct {
            UCHAR segmentation_enabled : 1;
            UCHAR update_mb_segmentation_map : 1;
            UCHAR update_mb_segmentation_data : 1;
            UCHAR mb_segement_abs_delta : 1;
            UCHAR ReservedSegmentFlags4Bits : 4;
        };
        UCHAR wSegmentFlags;
    };
    CHAR  segment_feature_data[2][4];
    UCHAR mb_segment_tree_probs[3];
} DXVA_segmentation_VP8;

/* VP8 picture parameters structure */
typedef struct _DXVA_PicParams_VP8 {
    UINT first_part_size;
    UINT width;
    UINT height;
    DXVA_PicEntry_VPx  CurrPic;
    union {
        struct {
            UCHAR frame_type : 1;
            UCHAR version : 3;
            UCHAR show_frame : 1;
            UCHAR clamp_type : 1;
            UCHAR ReservedFrameTag3Bits : 2;
        };
        UCHAR wFrameTagFlags;
    };
    DXVA_segmentation_VP8  stVP8Segments;
    UCHAR filter_type;
    UCHAR filter_level;
    UCHAR sharpness_level;
    UCHAR mode_ref_lf_delta_enabled;
    UCHAR mode_ref_lf_delta_update;
    CHAR  ref_lf_deltas[4];
    CHAR  mode_lf_deltas[4];
    UCHAR log2_nbr_of_dct_partitions;
    UCHAR base_qindex;
    CHAR  y1dc_delta_q;
    CHAR  y2dc_delta_q;
    CHAR  y2ac_delta_q;
    CHAR  uvdc_delta_q;
    CHAR  uvac_delta_q;
    DXVA_PicEntry_VPx alt_fb_idx;
    DXVA_PicEntry_VPx gld_fb_idx;
    DXVA_PicEntry_VPx lst_fb_idx;
    UCHAR  ref_frame_sign_bias_golden;
    UCHAR  ref_frame_sign_bias_altref;
    UCHAR  refresh_entropy_probs;
    UCHAR  vp8_coef_update_probs[4][8][3][11];
    UCHAR  mb_no_coeff_skip;
    UCHAR  prob_skip_false;
    UCHAR  prob_intra;
    UCHAR  prob_last;
    UCHAR  prob_golden;
    UCHAR  intra_16x16_prob[4];
    UCHAR  intra_chroma_prob[3];
    UCHAR  vp8_mv_update_probs[2][19];
    USHORT ReservedBits1;
    USHORT ReservedBits2;
    USHORT ReservedBits3;
    UINT   StatusReportFeedbackNumber;
} DXVA_PicParams_VP8, *LPDXVA_PicParams_VP8;

/* VPx slice control data structure - short form */
typedef struct _DXVA_Slice_VPx_Short {
    UINT   BSNALunitDataLocation;
    UINT   SliceBytesInBuffer;
    USHORT wBadSliceChopping;
} DXVA_Slice_VPx_Short, *LPDXVA_Slice_VPx_Short;

/* VPx status reporting data structure */
typedef struct _DXVA_Status_VPx {
    UINT  StatusReportFeedbackNumber;
    DXVA_PicEntry_VPx CurrPic;
    UCHAR  bBufType;
    UCHAR  bStatus;
    UCHAR  bReserved8Bits;
    USHORT wNumMbsAffected;
} DXVA_Status_VPx, *LPDXVA_Status_VPx;

/* AV1 specific structures */

#ifndef _DIRECTX_AV1_VA_
#define _DIRECTX_AV1_VA_

/* AV1 picture entry data structure */
typedef struct _DXVA_PicEntry_AV1 {

    UINT width;
    UINT height;

    // Global motion parameters
    INT wmmat[6];
    union {
        struct {
            UCHAR wminvalid : 1;
            UCHAR wmtype : 2;
            UCHAR Reserved : 5;
        };
        UCHAR GlobalMotionFlags;
    } DUMMYUNIONNAME;

    UCHAR Index;
    UINT16 Reserved16Bits;

} DXVA_PicEntry_AV1, *LPDXVA_PicEntry_AV1;

/* AV1 picture parameters structure */
typedef struct _DXVA_PicParams_AV1 {
    UINT width;
    UINT height;

    UINT max_width;
    UINT max_height;

    UCHAR CurrPicTextureIndex;
    UCHAR superres_denom;
    UCHAR bitdepth;
    UCHAR seq_profile;

    // Tiles:
    struct {
        UCHAR cols;
        UCHAR rows;
        USHORT context_update_id;
        USHORT widths[64];
        USHORT heights[64];
    } tiles;

    // Coding Tools
    union {
        struct {
            UINT use_128x128_superblock : 1;
            UINT intra_edge_filter : 1;
            UINT interintra_compound : 1;
            UINT masked_compound : 1;
            UINT warped_motion : 1;
            UINT dual_filter : 1;
            UINT jnt_comp : 1;
            UINT screen_content_tools : 1;
            UINT integer_mv : 1;
            UINT cdef : 1;
            UINT restoration : 1;
            UINT film_grain : 1;
            UINT intrabc : 1;
            UINT high_precision_mv : 1;
            UINT switchable_motion_mode : 1;
            UINT filter_intra : 1;
            UINT disable_frame_end_update_cdf : 1;
            UINT disable_cdf_update : 1;
            UINT reference_mode : 1;
            UINT skip_mode : 1;
            UINT reduced_tx_set : 1;
            UINT superres : 1;
            UINT tx_mode : 2;
            UINT use_ref_frame_mvs : 1;
            UINT enable_ref_frame_mvs : 1;
            UINT reference_frame_update : 1;
            UINT Reserved : 5;
        };
        UINT32 CodingParamToolFlags;
    } coding;

    // Format & Picture Info flags
    union {
        struct {
            UCHAR frame_type : 2;
            UCHAR show_frame : 1;
            UCHAR showable_frame : 1;
            UCHAR subsampling_x : 1;
            UCHAR subsampling_y : 1;
            UCHAR mono_chrome : 1;
            UCHAR Reserved : 1;
        };
        UCHAR FormatAndPictureInfoFlags;
    } format;

    // References
    UCHAR primary_ref_frame;
    UCHAR order_hint;
    UCHAR order_hint_bits;

    DXVA_PicEntry_AV1 frame_refs[7];
    UCHAR RefFrameMapTextureIndex[8];

    // Loop filter parameters
    struct {
        UCHAR filter_level[2];
        UCHAR filter_level_u;
        UCHAR filter_level_v;

        UCHAR sharpness_level;
        union {
            struct {
                UCHAR mode_ref_delta_enabled : 1;
                UCHAR mode_ref_delta_update : 1;
                UCHAR delta_lf_multi : 1;
                UCHAR delta_lf_present : 1;
                UCHAR Reserved : 4;
            };
            UCHAR ControlFlags;
        } DUMMYUNIONNAME;
        CHAR ref_deltas[8];
        CHAR mode_deltas[2];
        UCHAR delta_lf_res;
        UCHAR frame_restoration_type[3];
        USHORT log2_restoration_unit_size[3];
        UINT16 Reserved16Bits;
    } loop_filter;

    // Quantization
    struct {
        union {
            struct {
                UCHAR delta_q_present : 1;
                UCHAR delta_q_res : 2;
                UCHAR Reserved : 5;
            };
            UCHAR ControlFlags;
        } DUMMYUNIONNAME;

        UCHAR base_qindex;
        CHAR y_dc_delta_q;
        CHAR u_dc_delta_q;
        CHAR v_dc_delta_q;
        CHAR u_ac_delta_q;
        CHAR v_ac_delta_q;
        // using_qmatrix:
        UCHAR qm_y;
        UCHAR qm_u;
        UCHAR qm_v;
        UINT16 Reserved16Bits;
    } quantization;

    // Cdef parameters
    struct {
        union {
            struct {
                UCHAR damping : 2;
                UCHAR bits : 2;
                UCHAR Reserved : 4;
            };
            UCHAR ControlFlags;
        } DUMMYUNIONNAME;

        union {
            struct {
                UCHAR primary : 6;
                UCHAR secondary : 2;
            };
            UCHAR combined;
        } y_strengths[8];

        union {
            struct {
                UCHAR primary : 6;
                UCHAR secondary : 2;
            };
            UCHAR combined;
        } uv_strengths[8];

    } cdef;

    UCHAR interp_filter;

    // Segmentation
    struct {
        union {
            struct {
                UCHAR enabled : 1;
                UCHAR update_map : 1;
                UCHAR update_data : 1;
                UCHAR temporal_update : 1;
                UCHAR Reserved : 4;
            };
            UCHAR ControlFlags;
        } DUMMYUNIONNAME;
        UCHAR Reserved24Bits[3];

        union {
            struct {
                UCHAR alt_q : 1;
                UCHAR alt_lf_y_v : 1;
                UCHAR alt_lf_y_h : 1;
                UCHAR alt_lf_u : 1;
                UCHAR alt_lf_v : 1;
                UCHAR ref_frame : 1;
                UCHAR skip : 1;
                UCHAR globalmv : 1;
            };
            UCHAR mask;
        } feature_mask[8];

        SHORT feature_data[8][8];

    } segmentation;

    struct {
        union {
            struct {
                USHORT apply_grain : 1;
                USHORT scaling_shift_minus8 : 2;
                USHORT chroma_scaling_from_luma : 1;
                USHORT ar_coeff_lag : 2;
                USHORT ar_coeff_shift_minus6 : 2;
                USHORT grain_scale_shift : 2;
                USHORT overlap_flag : 1;
                USHORT clip_to_restricted_range : 1;
                USHORT matrix_coeff_is_identity : 1;
                USHORT Reserved : 3;
            };
            USHORT ControlFlags;
        } DUMMYUNIONNAME;

        USHORT grain_seed;
        UCHAR scaling_points_y[14][2];
        UCHAR num_y_points;
        UCHAR scaling_points_cb[10][2];
        UCHAR num_cb_points;
        UCHAR scaling_points_cr[10][2];
        UCHAR num_cr_points;
        UCHAR ar_coeffs_y[24];
        UCHAR ar_coeffs_cb[25];
        UCHAR ar_coeffs_cr[25];
        UCHAR cb_mult;
        UCHAR cb_luma_mult;
        UCHAR cr_mult;
        UCHAR cr_luma_mult;
        UCHAR Reserved8Bits;
        SHORT cb_offset;
        SHORT cr_offset;
    } film_grain;

    UINT   Reserved32Bits;
    UINT   StatusReportFeedbackNumber;
} DXVA_PicParams_AV1, *LPDXVA_PicParams_AV1;

/* AV1 tile structure */
typedef struct _DXVA_Tile_AV1 {
    UINT   DataOffset;
    UINT   DataSize;
    USHORT row;
    USHORT column;
    UINT16 Reserved16Bits;
    UCHAR anchor_frame;
    UCHAR Reserved8Bits;
} DXVA_Tile_AV1, *LPDXVA_Tile_AV1;

/* AV1 status reporting data structure */
typedef struct _DXVA_Status_AV1 {
    UINT  StatusReportFeedbackNumber;
    DXVA_PicEntry_AV1 CurrPic;
    UCHAR  BufType;
    UCHAR  Status;
    UCHAR  Reserved8Bits;
    USHORT NumMbsAffected;
} DXVA_Status_AV1, *LPDXVA_Status_AV1;

#endif // _DIRECTX_AV1_VA_

/* MJPEG specific structures */

#ifndef _DIRECTX_MJPEG_VA_
#define _DIRECTX_MJPEG_VA_

/* MJPEG picture parameters structure */
typedef struct _DXVA_PicParams_MJPEG {
    UINT width;
    UINT height;
    UCHAR numComponents;
    UCHAR bitDepth;
    USHORT reserved16Bits;
    UCHAR quantizationTableSelector[4];
    UINT scanOffset[4];
    UINT scanSize[4];
    UCHAR componentIdentifier[4];
    USHORT restartInterval;
    USHORT reserved16Bits2;
    UINT reserved32Bits;
    UINT statusReportFeedbackNumber;
} DXVA_PicParams_MJPEG, *LPDXVA_PicParams_MJPEG;

/* MJPEG quantization table structure */
typedef struct _DXVA_QMatrix_MJPEG {

    UINT16 quantvals[4][64];

} DXVA_QMatrix_MJPEG, *LPDXVA_QMatrix_MJPEG;

/* MJPEG quantization table structure */
typedef struct _DXVA_HuffmanTable_MJPEG {

    UINT8 bits_ac[4][16];
    UINT8 table_ac[4][256];

    UINT8 bits_dc[4][16];
    UINT8 table_dc[4][256];

} DXVA_HuffmanTable_MJPEG, *LPDXVA_HuffmanTable_MJPEG;

#endif // _DIRECTX_MJPEG_VA_

/*
 * Other forms of pictures are constructed in the obvious way
 * from the above by adjusting the number of residual difference
 * blocks, the number of motion vectors per macroblock, etc.
 */

#define readDXVA_MBskipsFollowing(ptr)       (((ptr)->dwMB_SNL & 0xFF000000) >> 24)
#define readDXVA_MBdataLocation(ptr)         (((ptr)->dwMB_SNL & 0x00FFFFFF))

#define writeDXVA_MB_SNL(ptr, skips, dloc)   ((ptr)->dwMB_SNL = (((skips) << 24) | (dloc)))
#define setDXVA_MBskipsFollowing(ptr, skips) ((ptr)->dwMB_SNL |= ((skips) << 24))
#define setDXVA_MBdataLocation(ptr, dloc)    ((ptr)->dwMB_SNL |= (dloc))

#define readDXVA_MvertFieldSel_3(ptr)    (((ptr)->wMBtype & 0x8000) >> 15)
#define readDXVA_MvertFieldSel_2(ptr)    (((ptr)->wMBtype & 0x4000) >> 14)
#define readDXVA_MvertFieldSel_1(ptr)    (((ptr)->wMBtype & 0x2000) >> 13)
#define readDXVA_MvertFieldSel_0(ptr)    (((ptr)->wMBtype & 0x1000) >> 12)
#define readDXVA_ReservedBits(ptr)       (((ptr)->wMBtype & 0x0800) >> 11)
#define readDXVA_HostResidDiff(ptr)      (((ptr)->wMBtype & 0x0400) >> 10)
#define readDXVA_MotionType(ptr)         (((ptr)->wMBtype & 0x0300) >>  8)
#define readDXVA_MBscanMethod(ptr)       (((ptr)->wMBtype & 0x00C0) >>  6)
#define readDXVA_FieldResidual(ptr)      (((ptr)->wMBtype & 0x0020) >>  5)
#define readDXVA_H261LoopFilter(ptr)     (((ptr)->wMBtype & 0x0010) >>  4)
#define readDXVA_Motion4MV(ptr)          (((ptr)->wMBtype & 0x0008) >>  3)
#define readDXVA_MotionBackward(ptr)     (((ptr)->wMBtype & 0x0004) >>  2)
#define readDXVA_MotionForward(ptr)      (((ptr)->wMBtype & 0x0002) >>  1)
#define readDXVA_IntraMacroblock(ptr)    (((ptr)->wMBtype & 0x0001))

#define setDXVA_MvertFieldSel_3(ptr)     ((ptr)->wMBtype |= 0x8000)
#define setDXVA_MvertFieldSel_2(ptr)     ((ptr)->wMBtype |= 0x4000)
#define setDXVA_MvertFieldSel_1(ptr)     ((ptr)->wMBtype |= 0x2000)
#define setDXVA_MvertFieldSel_0(ptr)     ((ptr)->wMBtype |= 0x1000)
#define setDXVA_ReservedBits(ptr)        ((ptr)->wMBtype |= 0x0800)
#define setDXVA_HostResidDiff(ptr)       ((ptr)->wMBtype |= 0x0400)
#define setDXVA_MotionType(ptr, value)   ((ptr)->wMBtype |= ((value) << 8))
#define setDXVA_MBscanMethod(ptr, value) ((ptr)->wMBtype |= ((value) << 6))
#define setDXVA_FieldResidual(ptr)       ((ptr)->wMBtype |= 0x0020)
#define setDXVA_H261LoopFilter(ptr)      ((ptr)->wMBtype |= 0x0010)
#define setDXVA_Motion4MV(ptr)           ((ptr)->wMBtype |= 0x0008)
#define setDXVA_MotionBackward(ptr)      ((ptr)->wMBtype |= 0x0004)
#define setDXVA_MotionForward(ptr)       ((ptr)->wMBtype |= 0x0002)
#define setDXVA_IntraMacroblock(ptr)     ((ptr)->wMBtype |= 0x0001)

#define readDXVA_Y___0coded(ptr)        (((ptr)->wPatternCode & 0x0800) >> 11)
#define readDXVA_Y___1coded(ptr)        (((ptr)->wPatternCode & 0x0400) >> 10)
#define readDXVA_Y___2coded(ptr)        (((ptr)->wPatternCode & 0x0200) >>  9)
#define readDXVA_Y___3coded(ptr)        (((ptr)->wPatternCode & 0x0100) >>  8)
#define readDXVA_Cb__4coded(ptr)        (((ptr)->wPatternCode & 0x0080) >>  7)
#define readDXVA_Cr__5coded(ptr)        (((ptr)->wPatternCode & 0x0040) >>  6)
#define readDXVA_Cb__6coded(ptr)        (((ptr)->wPatternCode & 0x0020) >>  5)
#define readDXVA_Cr__7coded(ptr)        (((ptr)->wPatternCode & 0x0010) >>  4)
#define readDXVA_Cb__8coded(ptr)        (((ptr)->wPatternCode & 0x0008) >>  3)
#define readDXVA_Cb__9coded(ptr)        (((ptr)->wPatternCode & 0x0004) >>  2)
#define readDXVA_Cr_10coded(ptr)        (((ptr)->wPatternCode & 0x0002) >>  1)
#define readDXVA_Cr_11coded(ptr)        (((ptr)->wPatternCode & 0x0001))

#define readDXVA_Y___0oflow(ptr)        (((ptr)->wPC_Overflow & 0x0800) >> 11)
#define readDXVA_Y___1oflow(ptr)        (((ptr)->wPC_Overflow & 0x0400) >> 10)
#define readDXVA_Y___2oflow(ptr)        (((ptr)->wPC_Overflow & 0x0200) >>  9)
#define readDXVA_Y___3oflow(ptr)        (((ptr)->wPC_Overflow & 0x0100) >>  8)
#define readDXVA_Cb__4oflow(ptr)        (((ptr)->wPC_Overflow & 0x0080) >>  7)
#define readDXVA_Cr__5oflow(ptr)        (((ptr)->wPC_Overflow & 0x0040) >>  6)
#define readDXVA_Cb__6oflow(ptr)        (((ptr)->wPC_Overflow & 0x0020) >>  5)
#define readDXVA_Cr__7oflow(ptr)        (((ptr)->wPC_Overflow & 0x0010) >>  4)
#define readDXVA_Cb__8oflow(ptr)        (((ptr)->wPC_Overflow & 0x0008) >>  3)
#define readDXVA_Cb__9oflow(ptr)        (((ptr)->wPC_Overflow & 0x0004) >>  2)
#define readDXVA_Cr_10oflow(ptr)        (((ptr)->wPC_Overflow & 0x0002) >>  1)
#define readDXVA_Cr_11oflow(ptr)        (((ptr)->wPC_Overflow & 0x0001))

#pragma pack(pop, BeforeDXVApacking)
#endif  /*   __DIRECTX_VA_DECODER__   */


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

#endif



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

typedef LONGLONG REFERENCE_TIME;

DEFINE_GUID(DXVA_DeinterlaceBobDevice,
    0x335aa36e,0x7884,0x43a4,0x9c,0x91,0x7f,0x87,0xfa,0xf3,0xe3,0x7e);

DEFINE_GUID(DXVA_DeinterlaceContainerDevice,
    0x0e85cb93,0x3046,0x4ff0,0xae,0xcc,0xd5,0x8c,0xb5,0xf0,0x35,0xfd);


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

#endif /*  __DIRECTX_VA_DEINTERLACE__ */


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

#endif /*  __DIRECTX_VA_PROCAMPCONTROL__ */


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

typedef enum _COPP_DPCP_Protection_Level {
    COPP_DPCP_Level0        = 0,
    COPP_DPCP_LevelMin      = COPP_DPCP_Level0,
    COPP_DPCP_Level1        = 1,
    COPP_DPCP_LevelMax      = COPP_DPCP_Level1,
    COPP_DPCP_ForceDWORD    = 0x7fffffff
} COPP_DPCP_Protection_Level;

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
    COPP_ProtectionType_DPCP         = 0x00000010,
    COPP_ProtectionType_Mask         = 0x80000017,
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
    COPP_ConnectorType_SDI = 9,
    COPP_ConnectorType_DisplayPortExternal = 10,
    COPP_ConnectorType_DisplayPortEmbedded = 11,
    COPP_ConnectorType_UDIExternal = 12,
    COPP_ConnectorType_UDIEmbedded = 13,
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

#ifdef __cplusplus
}
#endif

#ifdef __DXVA1_DEPRECATED_INTERFACES__   

#if !defined(__cplusplus)
#error C++ compiler required.
#endif

/* IID_IDirect3DVideoDevice9 */
DEFINE_GUID(IID_IDirect3DVideoDevice9, 
0x694036ac, 0x542a, 0x4a3a, 0x9a, 0x32, 0x53, 0xbc, 0x20, 0x0, 0x2c, 0x1b);

/* IID_IDirect3DDXVADevice9 */
DEFINE_GUID(IID_IDirect3DDXVADevice9, 
0x9f00c3d3, 0x5ab6, 0x465f, 0xb9, 0x55, 0x9f, 0xe, 0xbb, 0x2c, 0x56, 0x6);

interface IDirect3DVideoDevice9;
interface IDirect3DDXVADevice9;

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

#undef INTERFACE
#define INTERFACE IDirect3DVideoDevice9

DECLARE_INTERFACE_(IDirect3DVideoDevice9, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, void** ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    /*** IDirect3DVideoDevice9 methods ***/
    STDMETHOD(CreateSurface)(THIS_ UINT Width,UINT Height,UINT BackBuffers,
                             D3DFORMAT Format,D3DPOOL Pool,DWORD Usage,
                             IDirect3DSurface9** ppSurface,
                             HANDLE* pSharedHandle) PURE;
    STDMETHOD(GetDXVACompressedBufferInfo)(THIS_ GUID* pGuid,
                                           DXVAUncompDataInfo* pUncompData,
                                           DWORD* pNumBuffers,
                                           DXVACompBufferInfo* pBufferInfo) PURE;
    STDMETHOD(GetDXVAGuids)(THIS_ DWORD* pNumGuids,GUID* pGuids) PURE;
    STDMETHOD(GetDXVAInternalInfo)(THIS_ GUID* pGuid,
                                   DXVAUncompDataInfo* pUncompData,
                                   DWORD* pMemoryUsed) PURE;
    STDMETHOD(GetUncompressedDXVAFormats)(THIS_ GUID* pGuid,
                                          DWORD* pNumFormats,
                                          D3DFORMAT* pFormats) PURE;
    STDMETHOD(CreateDXVADevice)(THIS_ GUID* pGuid,
                                DXVAUncompDataInfo* pUncompData,
                                LPVOID pData,DWORD DataSize,
                                IDirect3DDXVADevice9** ppDXVADevice) PURE;
};

#undef INTERFACE
#define INTERFACE IDirect3DDXVADevice9

DECLARE_INTERFACE_(IDirect3DDXVADevice9, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, void** ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    /*** IDirect3DDXVADevice9 methods ***/
    STDMETHOD(BeginFrame)(THIS_ IDirect3DSurface9* pDstSurface,
                          DWORD SizeInputData,VOID* pInputData,
                          DWORD* pSizeOutputData,VOID* pOutputData) PURE;
    STDMETHOD(EndFrame)(THIS_ DWORD SizeMiscData,VOID* pMiscData) PURE;
    STDMETHOD(Execute)(THIS_ DWORD FunctionNum,VOID* pInputData,
                       DWORD InputSize,VOID* OuputData,DWORD OutputSize,
                       DWORD NumBuffers,DXVABufferInfo* pBufferInfo) PURE;
    STDMETHOD(QueryStatus)(THIS_ IDirect3DSurface9* pSurface,DWORD Flags) PURE;
};

#endif /* __DXVA1_DEPRECATED_INTERFACES__ */        

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* __DIRECTX_VA__ */
