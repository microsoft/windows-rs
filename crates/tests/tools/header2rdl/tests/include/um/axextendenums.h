//------------------------------------------------------------------------------
// File: AXExtendenums.h
//
// Desc: axextend.idl enums that must be exposed in managed code
//
// Copyright (c) 1992 - 2004, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------

#ifndef AXEXTEND_ENUM_H
#define AXEXTEND_ENUM_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// !!!! do not #pragma once, we use this file twice(once for native and once for mgd) in managed interop
#include <exposeenums2managed.h>

//
//---------------------------------------------------------------------
// AnalogVideoStandard enum
//---------------------------------------------------------------------

FLAGS TAG(AnalogVideoStandard)
{
    AnalogVideo_None     = 0x00000000,  // This is a digital sensor
    AnalogVideo_NTSC_M   = 0x00000001,  //        75 IRE Setup
    AnalogVideo_NTSC_M_J = 0x00000002,  // Japan,  0 IRE Setup
    AnalogVideo_NTSC_433 = 0x00000004,

    AnalogVideo_PAL_B    = 0x00000010,
    AnalogVideo_PAL_D    = 0x00000020,
    AnalogVideo_PAL_G    = 0x00000040,
    AnalogVideo_PAL_H    = 0x00000080,
    AnalogVideo_PAL_I    = 0x00000100,
    AnalogVideo_PAL_M    = 0x00000200,
    AnalogVideo_PAL_N    = 0x00000400,

    AnalogVideo_PAL_60   = 0x00000800,

    AnalogVideo_SECAM_B  = 0x00001000,
    AnalogVideo_SECAM_D  = 0x00002000,
    AnalogVideo_SECAM_G  = 0x00004000,
    AnalogVideo_SECAM_H  = 0x00008000,
    AnalogVideo_SECAM_K  = 0x00010000,
    AnalogVideo_SECAM_K1 = 0x00020000,
    AnalogVideo_SECAM_L  = 0x00040000,
    AnalogVideo_SECAM_L1 = 0x00080000,

    AnalogVideo_PAL_N_COMBO             // Argentina
                            = 0x00100000,

    AnalogVideoMask_MCE_NTSC = AnalogVideo_NTSC_M | // add pal n, n combo, m and 60 since they can be done on a ntsc tuner
                        AnalogVideo_NTSC_M_J |
                        AnalogVideo_NTSC_433 |
                        AnalogVideo_PAL_M |
                        AnalogVideo_PAL_N |
                        AnalogVideo_PAL_60 |
                        AnalogVideo_PAL_N_COMBO,

    AnalogVideoMask_MCE_PAL = AnalogVideo_PAL_B | // pull out pal n, n combo, m and 60
                        AnalogVideo_PAL_D |
                        AnalogVideo_PAL_G |
                        AnalogVideo_PAL_H |
                        AnalogVideo_PAL_I,

    AnalogVideoMask_MCE_SECAM = AnalogVideo_SECAM_B | // secam looks ok
                        AnalogVideo_SECAM_D  |
                        AnalogVideo_SECAM_G  |
                        AnalogVideo_SECAM_H  |
                        AnalogVideo_SECAM_K  |
                        AnalogVideo_SECAM_K1 |
                        AnalogVideo_SECAM_L  |
                        AnalogVideo_SECAM_L1,
} AnalogVideoStandard;

//---------------------------------------------------------------------
// TunerInputType enum
//---------------------------------------------------------------------

ENUM16 TAG(TunerInputType)
{
    TunerInputCable,
    TunerInputAntenna
} TunerInputType;

#include <unexposeenums2managed.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
// end of file axextendenum.h
