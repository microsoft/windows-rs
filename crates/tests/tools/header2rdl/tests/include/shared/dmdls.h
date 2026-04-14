/************************************************************************
*                                                                       *
*   dmdls.h -- DLS download definitions for DirectMusic API's           *
*                                                                       *
*   Copyright (c) Microsoft Corporation.  All rights reserved.          *
*                                                                       *
************************************************************************/

#ifndef _DMDLS_
#define _DMDLS_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include "dls1.h"

typedef long PCENT;     /* Pitch cents */
typedef long GCENT;     /* Gain cents */
typedef long TCENT;     /* Time cents */
typedef long PERCENT;   /* Per.. cent! */

typedef LONGLONG REFERENCE_TIME;
typedef REFERENCE_TIME *LPREFERENCE_TIME;
typedef long            MUSIC_TIME;

#ifndef MAKEFOURCC
#define MAKEFOURCC(ch0, ch1, ch2, ch3)                              \
                ((DWORD)(BYTE)(ch0) | ((DWORD)(BYTE)(ch1) << 8) |   \
                ((DWORD)(BYTE)(ch2) << 16) | ((DWORD)(BYTE)(ch3) << 24 ))


typedef DWORD           FOURCC;         /* a four character code */
#endif

typedef struct _DMUS_DOWNLOADINFO
{
    DWORD dwDLType;                     /* Instrument or Wave */
    DWORD dwDLId;                       /* Unique identifier to tag this download. */
    DWORD dwNumOffsetTableEntries;      /* Number of index in the offset address table. */
    DWORD cbSize;                       /* Total size of this memory chunk. */
} DMUS_DOWNLOADINFO;

#define DMUS_DOWNLOADINFO_INSTRUMENT        1
#define DMUS_DOWNLOADINFO_WAVE              2
#define DMUS_DOWNLOADINFO_INSTRUMENT2       3   /* New version for better DLS2 support. */

#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */

/* Support for oneshot and streaming wave data */
#define DMUS_DOWNLOADINFO_WAVEARTICULATION  4   /* Wave articulation data */
#define DMUS_DOWNLOADINFO_STREAMINGWAVE     5   /* One chunk of a streaming */
#define DMUS_DOWNLOADINFO_ONESHOTWAVE       6

#endif

#define DMUS_DEFAULT_SIZE_OFFSETTABLE   1

/* Flags for DMUS_INSTRUMENT's ulFlags member */

#define DMUS_INSTRUMENT_GM_INSTRUMENT   (1 << 0)

typedef struct _DMUS_OFFSETTABLE
{
    ULONG ulOffsetTable[DMUS_DEFAULT_SIZE_OFFSETTABLE];
} DMUS_OFFSETTABLE;

typedef struct _DMUS_INSTRUMENT
{
    ULONG           ulPatch;
    ULONG           ulFirstRegionIdx;
    ULONG           ulGlobalArtIdx;         /* If zero the instrument does not have an articulation */
    ULONG           ulFirstExtCkIdx;        /* If zero no 3rd party entenstion chunks associated with the instrument */
    ULONG           ulCopyrightIdx;         /* If zero no Copyright information associated with the instrument */
    ULONG           ulFlags;
} DMUS_INSTRUMENT;

typedef struct _DMUS_REGION
{
    RGNRANGE        RangeKey;
    RGNRANGE        RangeVelocity;
    USHORT          fusOptions;
    USHORT          usKeyGroup;
    ULONG           ulRegionArtIdx;         /* If zero the region does not have an articulation */
    ULONG           ulNextRegionIdx;        /* If zero no more regions */
    ULONG           ulFirstExtCkIdx;        /* If zero no 3rd party entenstion chunks associated with the region */
    WAVELINK        WaveLink;
    WSMPL           WSMP;                   /*  If WSMP.cSampleLoops > 1 then a WLOOP is included */
    WLOOP           WLOOP[1];
} DMUS_REGION;

typedef struct _DMUS_LFOPARAMS
{
    PCENT       pcFrequency;
    TCENT       tcDelay;
    GCENT       gcVolumeScale;
    PCENT       pcPitchScale;
    GCENT       gcMWToVolume;
    PCENT       pcMWToPitch;
} DMUS_LFOPARAMS;

typedef struct _DMUS_VEGPARAMS
{
    TCENT       tcAttack;
    TCENT       tcDecay;
    PERCENT     ptSustain;
    TCENT       tcRelease;
    TCENT       tcVel2Attack;
    TCENT       tcKey2Decay;
} DMUS_VEGPARAMS;

typedef struct _DMUS_PEGPARAMS
{
    TCENT       tcAttack;
    TCENT       tcDecay;
    PERCENT     ptSustain;
    TCENT       tcRelease;
    TCENT       tcVel2Attack;
    TCENT       tcKey2Decay;
    PCENT       pcRange;
} DMUS_PEGPARAMS;

typedef struct _DMUS_MSCPARAMS
{
    PERCENT     ptDefaultPan;
} DMUS_MSCPARAMS;

typedef struct _DMUS_ARTICPARAMS
{
    DMUS_LFOPARAMS   LFO;
    DMUS_VEGPARAMS   VolEG;
    DMUS_PEGPARAMS   PitchEG;
    DMUS_MSCPARAMS   Misc;
} DMUS_ARTICPARAMS;

typedef struct _DMUS_ARTICULATION           /* Articulation chunk for DMUS_DOWNLOADINFO_INSTRUMENT format. */
{
    ULONG           ulArt1Idx;              /* DLS Level 1 articulation chunk */
    ULONG           ulFirstExtCkIdx;        /* 3rd party extenstion chunks associated with the articulation */
} DMUS_ARTICULATION;

typedef struct _DMUS_ARTICULATION2          /* Articulation chunk for DMUS_DOWNLOADINFO_INSTRUMENT2 format. */
{
    ULONG           ulArtIdx;               /* DLS Level 1/2 articulation chunk */
    ULONG           ulFirstExtCkIdx;        /* 3rd party extenstion chunks associated with the articulation */
    ULONG           ulNextArtIdx;           /* Additional articulation chunks */
} DMUS_ARTICULATION2;

#define DMUS_MIN_DATA_SIZE 4
/*  The actual number is determined by cbSize of struct _DMUS_EXTENSIONCHUNK */

typedef struct _DMUS_EXTENSIONCHUNK
{
    ULONG           cbSize;                      /*  Size of extension chunk  */
    ULONG           ulNextExtCkIdx;              /*  If zero no more 3rd party entenstion chunks */
    FOURCC          ExtCkID;
    BYTE            byExtCk[DMUS_MIN_DATA_SIZE]; /*  The actual number that follows is determined by cbSize */
} DMUS_EXTENSIONCHUNK;

/*  The actual number is determined by cbSize of struct _DMUS_COPYRIGHT */

typedef struct _DMUS_COPYRIGHT
{
    ULONG           cbSize;                             /*  Size of copyright information */
    BYTE            byCopyright[DMUS_MIN_DATA_SIZE];    /*  The actual number that follows is determined by cbSize */
} DMUS_COPYRIGHT;

typedef struct _DMUS_WAVEDATA
{
    ULONG           cbSize;
    BYTE            byData[DMUS_MIN_DATA_SIZE];
} DMUS_WAVEDATA;

typedef struct _DMUS_WAVE
{
    ULONG           ulFirstExtCkIdx;    /* If zero no 3rd party entenstion chunks associated with the wave */
    ULONG           ulCopyrightIdx;     /* If zero no Copyright information associated with the wave */
    ULONG           ulWaveDataIdx;      /* Location of actual wave data. */
    WAVEFORMATEX    WaveformatEx;
} DMUS_WAVE;

typedef struct _DMUS_NOTERANGE *LPDMUS_NOTERANGE;
typedef struct _DMUS_NOTERANGE
{
    DWORD           dwLowNote;  /* Sets the low note for the range of MIDI note events to which the instrument responds.*/
    DWORD           dwHighNote; /* Sets the high note for the range of MIDI note events to which the instrument responds.*/
} DMUS_NOTERANGE;

#if (NTDDI_VERSION >= NTDDI_WINXP) /* Windows XP or greater */

typedef struct _DMUS_WAVEARTDL
{
    ULONG               ulDownloadIdIdx;    /* Download ID's of each buffer */
    ULONG               ulBus;              /* Playback bus */
    ULONG               ulBuffers;          /* Buffers */
    ULONG               ulMasterDLId;       /* Download ID of master voice of subordinate group */
    USHORT              usOptions;          /* Same as DLS2 region options */
}   DMUS_WAVEARTDL,
    *LPDMUS_WAVEARTDL;

typedef struct _DMUS_WAVEDL
{
    ULONG               cbWaveData;         /* Bytes of wave data */
}   DMUS_WAVEDL,
    *LPDMUS_WAVEDL;

#endif /* NTDDI_VERSION >= NTDDI_WINXP */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _DMDLS_ */
