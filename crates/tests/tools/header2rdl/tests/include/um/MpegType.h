//------------------------------------------------------------------------------
// File: MPEGType.h
//
// Desc: MPEG system stream compound type definition
//
// Copyright (c) 1996 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __MPEGTYPE__
#define __MPEGTYPE__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

//
//  AM_MPEGSYSTEMTYPE defines the format block contents for
//  data of type MEDIATYPE_MPEG1System when the format
//  block GUID is FORMAT_MPEG1System
//
//  The format block consists of elements of type
//  AM_MPEGSYSTEMTYPE up to the length of the format block
//  Each format block is 8-byte aligned from the start of
//  the format block
//

typedef struct tagAM_MPEGSTREAMTYPE
{
    DWORD             dwStreamId;     // Stream id of stream to process
    DWORD             dwReserved;     // 8-byte alignment
    AM_MEDIA_TYPE     mt;             // Type for substream - pbFormat is NULL
    BYTE              bFormat[1];     // Format data
} AM_MPEGSTREAMTYPE;

typedef struct tagAM_MPEGSYSTEMTYPE
{
    DWORD             dwBitRate;      // Bits per second
    DWORD             cStreams;       // Number of streams
    AM_MPEGSTREAMTYPE Streams[1];
} AM_MPEGSYSTEMTYPE;

//
//  Helper macros for AM_MPEGSTREAMTYPE
//
#define AM_MPEGSTREAMTYPE_ELEMENTLENGTH(pStreamType)  \
    FIELD_OFFSET(AM_MPEGSTREAMTYPE, bFormat[(pStreamType)->mt.cbFormat])
#define AM_MPEGSTREAMTYPE_NEXT(pStreamType)           \
    ((AM_MPEGSTREAMTYPE *)((PBYTE)(pStreamType) +     \
     ((AM_MPEGSTREAMTYPE_ELEMENTLENGTH(pStreamType) + 7) & ~7)))

//
// IMpegAudioDecoder
//

// Values for DualMode
#define AM_MPEG_AUDIO_DUAL_MERGE 0
#define AM_MPEG_AUDIO_DUAL_LEFT  1
#define AM_MPEG_AUDIO_DUAL_RIGHT 2

DECLARE_INTERFACE_(IMpegAudioDecoder, IUnknown) {

    STDMETHOD(get_FrequencyDivider) (THIS_
                           _Out_ unsigned long *pDivider   /* [out] */
                           ) PURE;

    STDMETHOD(put_FrequencyDivider) (THIS_
                           unsigned long Divider     /* [in] */
                           ) PURE;

    STDMETHOD(get_DecoderAccuracy) (THIS_
                           _Out_ unsigned long *pAccuracy  /* [out] */
                           ) PURE;

    STDMETHOD(put_DecoderAccuracy) (THIS_
                           unsigned long Accuracy    /* [in] */
                           ) PURE;

    STDMETHOD(get_Stereo) (THIS_
                           _Out_ unsigned long *pStereo    /* [out] */
                           ) PURE;

    STDMETHOD(put_Stereo) (THIS_
                           unsigned long Stereo      /* [in] */
                           ) PURE;

    STDMETHOD(get_DecoderWordSize) (THIS_
                           _Out_ unsigned long *pWordSize  /* [out] */
                           ) PURE;

    STDMETHOD(put_DecoderWordSize) (THIS_
                           unsigned long WordSize    /* [in] */
                           ) PURE;

    STDMETHOD(get_IntegerDecode) (THIS_
                           _Out_ unsigned long *pIntDecode /* [out] */
                           ) PURE;

    STDMETHOD(put_IntegerDecode) (THIS_
                           unsigned long IntDecode   /* [in] */
                           ) PURE;

    STDMETHOD(get_DualMode) (THIS_
                           unsigned long *pIntDecode /* [out] */
                           ) PURE;

    STDMETHOD(put_DualMode) (THIS_
                           unsigned long IntDecode   /* [in] */
                           ) PURE;

    STDMETHOD(get_AudioFormat) (THIS_
                           _Out_ MPEG1WAVEFORMAT *lpFmt    /* [out] */
                           ) PURE;
};

#ifdef __cplusplus
}
#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __MPEGTYPE__
