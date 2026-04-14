/********************************************************************************
*                                                                               *
* mmeapi.h -- ApiSet Contract for api-ms-win-mm-mme-l1-1-0                      *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _MMEAPI_H_
#define _MMEAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <windows.h>
#endif // _CONTRACT_GEN

#include <mmsyscom.h> // mm common definitions

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef MMNOWAVE
/****************************************************************************

                        Waveform audio support

****************************************************************************/

/* waveform audio error return values */
#define WAVERR_BADFORMAT      (WAVERR_BASE + 0)    /* unsupported wave format */
#define WAVERR_STILLPLAYING   (WAVERR_BASE + 1)    /* still something playing */
#define WAVERR_UNPREPARED     (WAVERR_BASE + 2)    /* header not prepared */
#define WAVERR_SYNC           (WAVERR_BASE + 3)    /* device is synchronous */
#define WAVERR_LASTERROR      (WAVERR_BASE + 3)    /* last error in range */

/* waveform audio data types */
DECLARE_HANDLE(HWAVE);
DECLARE_HANDLE(HWAVEIN);
DECLARE_HANDLE(HWAVEOUT);
typedef HWAVEIN FAR *LPHWAVEIN;
typedef HWAVEOUT FAR *LPHWAVEOUT;
typedef DRVCALLBACK WAVECALLBACK;
typedef WAVECALLBACK FAR *LPWAVECALLBACK;

/* wave callback messages */
#define WOM_OPEN        MM_WOM_OPEN
#define WOM_CLOSE       MM_WOM_CLOSE
#define WOM_DONE        MM_WOM_DONE
#define WIM_OPEN        MM_WIM_OPEN
#define WIM_CLOSE       MM_WIM_CLOSE
#define WIM_DATA        MM_WIM_DATA

/* device ID for wave device mapper */
#define WAVE_MAPPER     ((UINT)-1)

/* flags for dwFlags parameter in waveOutOpen() and waveInOpen() */
#define  WAVE_FORMAT_QUERY                          0x0001
#define  WAVE_ALLOWSYNC                             0x0002
#if (WINVER >= 0x0400)
#define  WAVE_MAPPED                                0x0004
#define  WAVE_FORMAT_DIRECT                         0x0008
#define  WAVE_FORMAT_DIRECT_QUERY                   (WAVE_FORMAT_QUERY | WAVE_FORMAT_DIRECT)
#define  WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE   0x0010
#endif /* (WINVER >= 0x0400) */

/* wave data block header */
typedef struct wavehdr_tag {
    LPSTR       lpData;                 /* pointer to locked data buffer */
    DWORD       dwBufferLength;         /* length of data buffer */
    DWORD       dwBytesRecorded;        /* used for input only */
    DWORD_PTR   dwUser;                 /* for client's use */
    DWORD       dwFlags;                /* assorted flags (see defines) */
    DWORD       dwLoops;                /* loop control counter */
    struct wavehdr_tag FAR *lpNext;     /* reserved for driver */
    DWORD_PTR   reserved;               /* reserved for driver */
} WAVEHDR, *PWAVEHDR, NEAR *NPWAVEHDR, FAR *LPWAVEHDR;

/* flags for dwFlags field of WAVEHDR */
#define WHDR_DONE       0x00000001  /* done bit */
#define WHDR_PREPARED   0x00000002  /* set if this header has been prepared */
#define WHDR_BEGINLOOP  0x00000004  /* loop start block */
#define WHDR_ENDLOOP    0x00000008  /* loop end block */
#define WHDR_INQUEUE    0x00000010  /* reserved for driver */

/* waveform output device capabilities structure */
#ifdef _WIN32

typedef struct tagWAVEOUTCAPSA {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    CHAR    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    DWORD   dwFormats;             /* formats supported */
    WORD    wChannels;             /* number of sources supported */
    WORD    wReserved1;            /* packing */
    DWORD   dwSupport;             /* functionality supported by driver */
} WAVEOUTCAPSA, *PWAVEOUTCAPSA, *NPWAVEOUTCAPSA, *LPWAVEOUTCAPSA;
typedef struct tagWAVEOUTCAPSW {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    WCHAR   szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    DWORD   dwFormats;             /* formats supported */
    WORD    wChannels;             /* number of sources supported */
    WORD    wReserved1;            /* packing */
    DWORD   dwSupport;             /* functionality supported by driver */
} WAVEOUTCAPSW, *PWAVEOUTCAPSW, *NPWAVEOUTCAPSW, *LPWAVEOUTCAPSW;
#ifdef UNICODE
typedef WAVEOUTCAPSW WAVEOUTCAPS;
typedef PWAVEOUTCAPSW PWAVEOUTCAPS;
typedef NPWAVEOUTCAPSW NPWAVEOUTCAPS;
typedef LPWAVEOUTCAPSW LPWAVEOUTCAPS;
#else
typedef WAVEOUTCAPSA WAVEOUTCAPS;
typedef PWAVEOUTCAPSA PWAVEOUTCAPS;
typedef NPWAVEOUTCAPSA NPWAVEOUTCAPS;
typedef LPWAVEOUTCAPSA LPWAVEOUTCAPS;
#endif // UNICODE
typedef struct tagWAVEOUTCAPS2A {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    CHAR    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    DWORD   dwFormats;             /* formats supported */
    WORD    wChannels;             /* number of sources supported */
    WORD    wReserved1;            /* packing */
    DWORD   dwSupport;             /* functionality supported by driver */
    GUID    ManufacturerGuid;      /* for extensible MID mapping */
    GUID    ProductGuid;           /* for extensible PID mapping */
    GUID    NameGuid;              /* for name lookup in registry */
} WAVEOUTCAPS2A, *PWAVEOUTCAPS2A, *NPWAVEOUTCAPS2A, *LPWAVEOUTCAPS2A;
typedef struct tagWAVEOUTCAPS2W {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    WCHAR   szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    DWORD   dwFormats;             /* formats supported */
    WORD    wChannels;             /* number of sources supported */
    WORD    wReserved1;            /* packing */
    DWORD   dwSupport;             /* functionality supported by driver */
    GUID    ManufacturerGuid;      /* for extensible MID mapping */
    GUID    ProductGuid;           /* for extensible PID mapping */
    GUID    NameGuid;              /* for name lookup in registry */
} WAVEOUTCAPS2W, *PWAVEOUTCAPS2W, *NPWAVEOUTCAPS2W, *LPWAVEOUTCAPS2W;
#ifdef UNICODE
typedef WAVEOUTCAPS2W WAVEOUTCAPS2;
typedef PWAVEOUTCAPS2W PWAVEOUTCAPS2;
typedef NPWAVEOUTCAPS2W NPWAVEOUTCAPS2;
typedef LPWAVEOUTCAPS2W LPWAVEOUTCAPS2;
#else
typedef WAVEOUTCAPS2A WAVEOUTCAPS2;
typedef PWAVEOUTCAPS2A PWAVEOUTCAPS2;
typedef NPWAVEOUTCAPS2A NPWAVEOUTCAPS2;
typedef LPWAVEOUTCAPS2A LPWAVEOUTCAPS2;
#endif // UNICODE

#else
typedef struct waveoutcaps_tag {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    VERSION vDriverVersion;        /* version of the driver */
    char    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    DWORD   dwFormats;             /* formats supported */
    WORD    wChannels;             /* number of sources supported */
    DWORD   dwSupport;             /* functionality supported by driver */
} WAVEOUTCAPS, *PWAVEOUTCAPS, NEAR *NPWAVEOUTCAPS, FAR *LPWAVEOUTCAPS;
#endif

/* flags for dwSupport field of WAVEOUTCAPS */
#define WAVECAPS_PITCH          0x0001   /* supports pitch control */
#define WAVECAPS_PLAYBACKRATE   0x0002   /* supports playback rate control */
#define WAVECAPS_VOLUME         0x0004   /* supports volume control */
#define WAVECAPS_LRVOLUME       0x0008   /* separate left-right volume control */
#define WAVECAPS_SYNC           0x0010
#define WAVECAPS_SAMPLEACCURATE 0x0020

/* waveform input device capabilities structure */
#ifdef _WIN32

typedef struct tagWAVEINCAPSA {
    WORD    wMid;                    /* manufacturer ID */
    WORD    wPid;                    /* product ID */
    MMVERSION vDriverVersion;        /* version of the driver */
    CHAR    szPname[MAXPNAMELEN];    /* product name (NULL terminated string) */
    DWORD   dwFormats;               /* formats supported */
    WORD    wChannels;               /* number of channels supported */
    WORD    wReserved1;              /* structure packing */
} WAVEINCAPSA, *PWAVEINCAPSA, *NPWAVEINCAPSA, *LPWAVEINCAPSA;
typedef struct tagWAVEINCAPSW {
    WORD    wMid;                    /* manufacturer ID */
    WORD    wPid;                    /* product ID */
    MMVERSION vDriverVersion;        /* version of the driver */
    WCHAR   szPname[MAXPNAMELEN];    /* product name (NULL terminated string) */
    DWORD   dwFormats;               /* formats supported */
    WORD    wChannels;               /* number of channels supported */
    WORD    wReserved1;              /* structure packing */
} WAVEINCAPSW, *PWAVEINCAPSW, *NPWAVEINCAPSW, *LPWAVEINCAPSW;
#ifdef UNICODE
typedef WAVEINCAPSW WAVEINCAPS;
typedef PWAVEINCAPSW PWAVEINCAPS;
typedef NPWAVEINCAPSW NPWAVEINCAPS;
typedef LPWAVEINCAPSW LPWAVEINCAPS;
#else
typedef WAVEINCAPSA WAVEINCAPS;
typedef PWAVEINCAPSA PWAVEINCAPS;
typedef NPWAVEINCAPSA NPWAVEINCAPS;
typedef LPWAVEINCAPSA LPWAVEINCAPS;
#endif // UNICODE
typedef struct tagWAVEINCAPS2A {
    WORD    wMid;                    /* manufacturer ID */
    WORD    wPid;                    /* product ID */
    MMVERSION vDriverVersion;        /* version of the driver */
    CHAR    szPname[MAXPNAMELEN];    /* product name (NULL terminated string) */
    DWORD   dwFormats;               /* formats supported */
    WORD    wChannels;               /* number of channels supported */
    WORD    wReserved1;              /* structure packing */
    GUID    ManufacturerGuid;        /* for extensible MID mapping */
    GUID    ProductGuid;             /* for extensible PID mapping */
    GUID    NameGuid;                /* for name lookup in registry */
} WAVEINCAPS2A, *PWAVEINCAPS2A, *NPWAVEINCAPS2A, *LPWAVEINCAPS2A;
typedef struct tagWAVEINCAPS2W {
    WORD    wMid;                    /* manufacturer ID */
    WORD    wPid;                    /* product ID */
    MMVERSION vDriverVersion;        /* version of the driver */
    WCHAR   szPname[MAXPNAMELEN];    /* product name (NULL terminated string) */
    DWORD   dwFormats;               /* formats supported */
    WORD    wChannels;               /* number of channels supported */
    WORD    wReserved1;              /* structure packing */
    GUID    ManufacturerGuid;        /* for extensible MID mapping */
    GUID    ProductGuid;             /* for extensible PID mapping */
    GUID    NameGuid;                /* for name lookup in registry */
} WAVEINCAPS2W, *PWAVEINCAPS2W, *NPWAVEINCAPS2W, *LPWAVEINCAPS2W;
#ifdef UNICODE
typedef WAVEINCAPS2W WAVEINCAPS2;
typedef PWAVEINCAPS2W PWAVEINCAPS2;
typedef NPWAVEINCAPS2W NPWAVEINCAPS2;
typedef LPWAVEINCAPS2W LPWAVEINCAPS2;
#else
typedef WAVEINCAPS2A WAVEINCAPS2;
typedef PWAVEINCAPS2A PWAVEINCAPS2;
typedef NPWAVEINCAPS2A NPWAVEINCAPS2;
typedef LPWAVEINCAPS2A LPWAVEINCAPS2;
#endif // UNICODE

#else
typedef struct waveincaps_tag {
    WORD    wMid;                    /* manufacturer ID */
    WORD    wPid;                    /* product ID */
    VERSION vDriverVersion;          /* version of the driver */
    char    szPname[MAXPNAMELEN];    /* product name (NULL terminated string) */
    DWORD   dwFormats;               /* formats supported */
    WORD    wChannels;               /* number of channels supported */
} WAVEINCAPS, *PWAVEINCAPS, NEAR *NPWAVEINCAPS, FAR *LPWAVEINCAPS;
#endif

/* defines for dwFormat field of WAVEINCAPS and WAVEOUTCAPS */
#define WAVE_INVALIDFORMAT     0x00000000       /* invalid format */
#define WAVE_FORMAT_1M08       0x00000001       /* 11.025 kHz, Mono,   8-bit  */
#define WAVE_FORMAT_1S08       0x00000002       /* 11.025 kHz, Stereo, 8-bit  */
#define WAVE_FORMAT_1M16       0x00000004       /* 11.025 kHz, Mono,   16-bit */
#define WAVE_FORMAT_1S16       0x00000008       /* 11.025 kHz, Stereo, 16-bit */
#define WAVE_FORMAT_2M08       0x00000010       /* 22.05  kHz, Mono,   8-bit  */
#define WAVE_FORMAT_2S08       0x00000020       /* 22.05  kHz, Stereo, 8-bit  */
#define WAVE_FORMAT_2M16       0x00000040       /* 22.05  kHz, Mono,   16-bit */
#define WAVE_FORMAT_2S16       0x00000080       /* 22.05  kHz, Stereo, 16-bit */
#define WAVE_FORMAT_4M08       0x00000100       /* 44.1   kHz, Mono,   8-bit  */
#define WAVE_FORMAT_4S08       0x00000200       /* 44.1   kHz, Stereo, 8-bit  */
#define WAVE_FORMAT_4M16       0x00000400       /* 44.1   kHz, Mono,   16-bit */
#define WAVE_FORMAT_4S16       0x00000800       /* 44.1   kHz, Stereo, 16-bit */

#define WAVE_FORMAT_44M08      0x00000100       /* 44.1   kHz, Mono,   8-bit  */
#define WAVE_FORMAT_44S08      0x00000200       /* 44.1   kHz, Stereo, 8-bit  */
#define WAVE_FORMAT_44M16      0x00000400       /* 44.1   kHz, Mono,   16-bit */
#define WAVE_FORMAT_44S16      0x00000800       /* 44.1   kHz, Stereo, 16-bit */
#define WAVE_FORMAT_48M08      0x00001000       /* 48     kHz, Mono,   8-bit  */
#define WAVE_FORMAT_48S08      0x00002000       /* 48     kHz, Stereo, 8-bit  */
#define WAVE_FORMAT_48M16      0x00004000       /* 48     kHz, Mono,   16-bit */
#define WAVE_FORMAT_48S16      0x00008000       /* 48     kHz, Stereo, 16-bit */
#define WAVE_FORMAT_96M08      0x00010000       /* 96     kHz, Mono,   8-bit  */
#define WAVE_FORMAT_96S08      0x00020000       /* 96     kHz, Stereo, 8-bit  */
#define WAVE_FORMAT_96M16      0x00040000       /* 96     kHz, Mono,   16-bit */
#define WAVE_FORMAT_96S16      0x00080000       /* 96     kHz, Stereo, 16-bit */

#ifndef WAVE_FORMAT_PCM

/* OLD general waveform format structure (information common to all formats) */
typedef struct waveformat_tag {
    WORD    wFormatTag;        /* format type */
    WORD    nChannels;         /* number of channels (i.e. mono, stereo, etc.) */
    DWORD   nSamplesPerSec;    /* sample rate */
    DWORD   nAvgBytesPerSec;   /* for buffer estimation */
    WORD    nBlockAlign;       /* block size of data */
} WAVEFORMAT, *PWAVEFORMAT, NEAR *NPWAVEFORMAT, FAR *LPWAVEFORMAT;

/* flags for wFormatTag field of WAVEFORMAT */
#define WAVE_FORMAT_PCM     1

/* specific waveform format structure for PCM data */
typedef struct pcmwaveformat_tag {
    WAVEFORMAT  wf;
    WORD        wBitsPerSample;
} PCMWAVEFORMAT, *PPCMWAVEFORMAT, NEAR *NPPCMWAVEFORMAT, FAR *LPPCMWAVEFORMAT;
#endif /* WAVE_FORMAT_PCM */

#ifndef _WAVEFORMATEX_
#define _WAVEFORMATEX_

/*
 *  extended waveform format structure used for all non-PCM formats. this
 *  structure is common to all non-PCM formats.
 */
typedef struct tWAVEFORMATEX
{
    WORD        wFormatTag;         /* format type */
    WORD        nChannels;          /* number of channels (i.e. mono, stereo...) */
    DWORD       nSamplesPerSec;     /* sample rate */
    DWORD       nAvgBytesPerSec;    /* for buffer estimation */
    WORD        nBlockAlign;        /* block size of data */
    WORD        wBitsPerSample;     /* number of bits per sample of mono data */
    WORD        cbSize;             /* the count in bytes of the size of */
                                    /* extra information (after cbSize) */
} WAVEFORMATEX, *PWAVEFORMATEX, NEAR *NPWAVEFORMATEX, FAR *LPWAVEFORMATEX;

#endif /* _WAVEFORMATEX_ */
typedef const WAVEFORMATEX FAR *LPCWAVEFORMATEX;

/* waveform audio function prototypes */
WINMMAPI
UINT
WINAPI
waveOutGetNumDevs(
    void
    );

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
waveOutGetDevCapsA(
    _In_ UINT_PTR uDeviceID,
    _Out_ LPWAVEOUTCAPSA pwoc,
    _In_ UINT cbwoc
    );

WINMMAPI
MMRESULT
WINAPI
waveOutGetDevCapsW(
    _In_ UINT_PTR uDeviceID,
    _Out_ LPWAVEOUTCAPSW pwoc,
    _In_ UINT cbwoc
    );

#ifdef UNICODE
#define waveOutGetDevCaps  waveOutGetDevCapsW
#else
#define waveOutGetDevCaps  waveOutGetDevCapsA
#endif // !UNICODE

#else
WINMMAPI MMRESULT WINAPI waveOutGetDevCaps( UINT uDeviceID, LPWAVEOUTCAPS pwoc, UINT cbwoc);
#endif

#if (WINVER >= 0x0400)
WINMMAPI
MMRESULT
WINAPI
waveOutGetVolume(
    _In_opt_ HWAVEOUT hwo,
    _Out_ LPDWORD pdwVolume
    );

WINMMAPI
MMRESULT
WINAPI
waveOutSetVolume(
    _In_opt_ HWAVEOUT hwo,
    _In_ DWORD dwVolume
    );

#else
WINMMAPI MMRESULT WINAPI waveOutGetVolume(UINT uId, LPDWORD pdwVolume);
WINMMAPI MMRESULT WINAPI waveOutSetVolume(UINT uId, DWORD dwVolume);
#endif

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
waveOutGetErrorTextA(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPSTR pszText,
    _In_ UINT cchText
    );

WINMMAPI
MMRESULT
WINAPI
waveOutGetErrorTextW(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPWSTR pszText,
    _In_ UINT cchText
    );

#ifdef UNICODE
#define waveOutGetErrorText  waveOutGetErrorTextW
#else
#define waveOutGetErrorText  waveOutGetErrorTextA
#endif // !UNICODE

#else
MMRESULT WINAPI waveOutGetErrorText(MMRESULT mmrError, LPSTR pszText, UINT cchText);
#endif

WINMMAPI
MMRESULT
WINAPI
waveOutOpen(
    _Out_opt_ LPHWAVEOUT phwo,
    _In_ UINT uDeviceID,
    _In_ LPCWAVEFORMATEX pwfx,
    _In_opt_ DWORD_PTR dwCallback,
    _In_opt_ DWORD_PTR dwInstance,
    _In_ DWORD fdwOpen
    );

WINMMAPI
MMRESULT
WINAPI
waveOutClose(
    _In_ HWAVEOUT hwo
    );

WINMMAPI
MMRESULT
WINAPI
waveOutPrepareHeader(
    _In_ HWAVEOUT hwo,
    _Inout_updates_bytes_(cbwh) LPWAVEHDR pwh,
    _In_ UINT cbwh
    );

WINMMAPI
MMRESULT
WINAPI
waveOutUnprepareHeader(
    _In_ HWAVEOUT hwo,
    _Inout_updates_bytes_(cbwh) LPWAVEHDR pwh,
    _In_ UINT cbwh
    );

WINMMAPI
MMRESULT
WINAPI
waveOutWrite(
    _In_ HWAVEOUT hwo,
    _Inout_updates_bytes_(cbwh) LPWAVEHDR pwh,
    _In_ UINT cbwh
    );

WINMMAPI
MMRESULT
WINAPI
waveOutPause(
    _In_ HWAVEOUT hwo
    );

WINMMAPI
MMRESULT
WINAPI
waveOutRestart(
    _In_ HWAVEOUT hwo
    );

WINMMAPI
MMRESULT
WINAPI
waveOutReset(
    _In_ HWAVEOUT hwo
    );

WINMMAPI
MMRESULT
WINAPI
waveOutBreakLoop(
    _In_ HWAVEOUT hwo
    );

WINMMAPI
MMRESULT
WINAPI
waveOutGetPosition(
    _In_ HWAVEOUT hwo,
    _Inout_updates_bytes_(cbmmt) LPMMTIME pmmt,
    _In_ UINT cbmmt
    );

WINMMAPI
MMRESULT
WINAPI
waveOutGetPitch(
    _In_ HWAVEOUT hwo,
    _Out_ LPDWORD pdwPitch
    );

WINMMAPI
MMRESULT
WINAPI
waveOutSetPitch(
    _In_ HWAVEOUT hwo,
    _In_ DWORD dwPitch
    );

WINMMAPI
MMRESULT
WINAPI
waveOutGetPlaybackRate(
    _In_ HWAVEOUT hwo,
    _Out_ LPDWORD pdwRate
    );

WINMMAPI
MMRESULT
WINAPI
waveOutSetPlaybackRate(
    _In_ HWAVEOUT hwo,
    _In_ DWORD dwRate
    );

WINMMAPI
MMRESULT
WINAPI
waveOutGetID(
    _In_ HWAVEOUT hwo,
    _Out_ LPUINT puDeviceID
    );

#if (WINVER >= 0x030a)
#ifdef _WIN32
WINMMAPI
MMRESULT
WINAPI
waveOutMessage(
    _In_opt_ HWAVEOUT hwo,
    _In_ UINT uMsg,
    _In_ DWORD_PTR dw1,
    _In_ DWORD_PTR dw2
    );

#else
DWORD WINAPI waveOutMessage(HWAVEOUT hwo, UINT uMsg, DWORD dw1, DWORD dw2);
#endif
#endif /* ifdef WINVER >= 0x030a */

WINMMAPI
UINT
WINAPI
waveInGetNumDevs(
    void
    );

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
waveInGetDevCapsA(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbwic) LPWAVEINCAPSA pwic,
    _In_ UINT cbwic
    );

WINMMAPI
MMRESULT
WINAPI
waveInGetDevCapsW(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbwic) LPWAVEINCAPSW pwic,
    _In_ UINT cbwic
    );

#ifdef UNICODE
#define waveInGetDevCaps  waveInGetDevCapsW
#else
#define waveInGetDevCaps  waveInGetDevCapsA
#endif // !UNICODE

#else
MMRESULT WINAPI waveInGetDevCaps(UINT uDeviceID, LPWAVEINCAPS pwic, UINT cbwic);
#endif

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
waveInGetErrorTextA(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPSTR pszText,
    _In_ UINT cchText
    );

WINMMAPI
MMRESULT
WINAPI
waveInGetErrorTextW(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPWSTR pszText,
    _In_ UINT cchText
    );

#ifdef UNICODE
#define waveInGetErrorText  waveInGetErrorTextW
#else
#define waveInGetErrorText  waveInGetErrorTextA
#endif // !UNICODE

#else
MMRESULT WINAPI waveInGetErrorText(MMRESULT mmrError, LPSTR pszText, UINT cchText);
#endif

WINMMAPI
MMRESULT
WINAPI
waveInOpen(
    _Out_opt_ LPHWAVEIN phwi,
    _In_ UINT uDeviceID,
    _In_ LPCWAVEFORMATEX pwfx,
    _In_opt_ DWORD_PTR dwCallback,
    _In_opt_ DWORD_PTR dwInstance,
    _In_ DWORD fdwOpen
    );

WINMMAPI
MMRESULT
WINAPI
waveInClose(
    _In_ HWAVEIN hwi
    );

WINMMAPI
MMRESULT
WINAPI
waveInPrepareHeader(
    _In_ HWAVEIN hwi,
    _Inout_updates_bytes_(cbwh) LPWAVEHDR pwh,
    _In_ UINT cbwh
    );

WINMMAPI
MMRESULT
WINAPI
waveInUnprepareHeader(
    _In_ HWAVEIN hwi,
    _Inout_updates_bytes_(cbwh) LPWAVEHDR pwh,
    _In_ UINT cbwh
    );

WINMMAPI
MMRESULT
WINAPI
waveInAddBuffer(
    _In_ HWAVEIN hwi,
    _Inout_updates_bytes_(cbwh) LPWAVEHDR pwh,
    _In_ UINT cbwh
    );

WINMMAPI
MMRESULT
WINAPI
waveInStart(
    _In_ HWAVEIN hwi
    );

WINMMAPI
MMRESULT
WINAPI
waveInStop(
    _In_ HWAVEIN hwi
    );

WINMMAPI
MMRESULT
WINAPI
waveInReset(
    _In_ HWAVEIN hwi
    );

WINMMAPI
MMRESULT
WINAPI
waveInGetPosition(
    _In_ HWAVEIN hwi,
    _Inout_updates_bytes_(cbmmt) LPMMTIME pmmt,
    _In_ UINT cbmmt
    );

WINMMAPI
MMRESULT
WINAPI
waveInGetID(
    _In_ HWAVEIN hwi,
    _In_ LPUINT puDeviceID
    );

#if (WINVER >= 0x030a)
#ifdef _WIN32
WINMMAPI
MMRESULT
WINAPI
waveInMessage(
    _In_opt_ HWAVEIN hwi,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dw1,
    _In_opt_ DWORD_PTR dw2
    );

#else
DWORD WINAPI waveInMessage(HWAVEIN hwi, UINT uMsg, DWORD dw1, DWORD dw2);
#endif
#endif /* ifdef WINVER >= 0x030a */

#endif  /* ifndef MMNOWAVE */

#ifndef MMNOMIDI
/****************************************************************************

                            MIDI audio support

****************************************************************************/

/* MIDI error return values */
#define MIDIERR_UNPREPARED    (MIDIERR_BASE + 0)   /* header not prepared */
#define MIDIERR_STILLPLAYING  (MIDIERR_BASE + 1)   /* still something playing */
#define MIDIERR_NOMAP         (MIDIERR_BASE + 2)   /* no configured instruments */
#define MIDIERR_NOTREADY      (MIDIERR_BASE + 3)   /* hardware is still busy */
#define MIDIERR_NODEVICE      (MIDIERR_BASE + 4)   /* port no longer connected */
#define MIDIERR_INVALIDSETUP  (MIDIERR_BASE + 5)   /* invalid MIF */
#define MIDIERR_BADOPENMODE   (MIDIERR_BASE + 6)   /* operation unsupported w/ open mode */
#define MIDIERR_DONT_CONTINUE (MIDIERR_BASE + 7)   /* thru device 'eating' a message */
#define MIDIERR_LASTERROR     (MIDIERR_BASE + 7)   /* last error in range */

/* MIDI audio data types */
DECLARE_HANDLE(HMIDI);
DECLARE_HANDLE(HMIDIIN);
DECLARE_HANDLE(HMIDIOUT);
DECLARE_HANDLE(HMIDISTRM);
typedef HMIDI FAR *LPHMIDI;
typedef HMIDIIN FAR *LPHMIDIIN;
typedef HMIDIOUT FAR *LPHMIDIOUT;
typedef HMIDISTRM FAR *LPHMIDISTRM;
typedef DRVCALLBACK MIDICALLBACK;
typedef MIDICALLBACK FAR *LPMIDICALLBACK;
#define MIDIPATCHSIZE   128
typedef WORD PATCHARRAY[MIDIPATCHSIZE];
typedef WORD FAR *LPPATCHARRAY;
typedef WORD KEYARRAY[MIDIPATCHSIZE];
typedef WORD FAR *LPKEYARRAY;

/* MIDI callback messages */
#define MIM_OPEN        MM_MIM_OPEN
#define MIM_CLOSE       MM_MIM_CLOSE
#define MIM_DATA        MM_MIM_DATA
#define MIM_LONGDATA    MM_MIM_LONGDATA
#define MIM_ERROR       MM_MIM_ERROR
#define MIM_LONGERROR   MM_MIM_LONGERROR
#define MOM_OPEN        MM_MOM_OPEN
#define MOM_CLOSE       MM_MOM_CLOSE
#define MOM_DONE        MM_MOM_DONE

#if(WINVER >= 0x0400)
#define MIM_MOREDATA      MM_MIM_MOREDATA
#define MOM_POSITIONCB    MM_MOM_POSITIONCB
#endif /* WINVER >= 0x0400 */

/* device ID for MIDI mapper */
#define MIDIMAPPER     ((UINT)-1)
#define MIDI_MAPPER    ((UINT)-1)

#if(WINVER >= 0x0400)
/* flags for dwFlags parm of midiInOpen() */
#define MIDI_IO_STATUS      0x00000020L
#endif /* WINVER >= 0x0400 */

/* flags for wFlags parm of midiOutCachePatches(), midiOutCacheDrumPatches() */
#define MIDI_CACHE_ALL      1
#define MIDI_CACHE_BESTFIT  2
#define MIDI_CACHE_QUERY    3
#define MIDI_UNCACHE        4

/* MIDI output device capabilities structure */
#ifdef _WIN32

typedef struct tagMIDIOUTCAPSA {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    CHAR    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    WORD    wTechnology;           /* type of device */
    WORD    wVoices;               /* # of voices (internal synth only) */
    WORD    wNotes;                /* max # of notes (internal synth only) */
    WORD    wChannelMask;          /* channels used (internal synth only) */
    DWORD   dwSupport;             /* functionality supported by driver */
} MIDIOUTCAPSA, *PMIDIOUTCAPSA, *NPMIDIOUTCAPSA, *LPMIDIOUTCAPSA;
typedef struct tagMIDIOUTCAPSW {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    WCHAR   szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    WORD    wTechnology;           /* type of device */
    WORD    wVoices;               /* # of voices (internal synth only) */
    WORD    wNotes;                /* max # of notes (internal synth only) */
    WORD    wChannelMask;          /* channels used (internal synth only) */
    DWORD   dwSupport;             /* functionality supported by driver */
} MIDIOUTCAPSW, *PMIDIOUTCAPSW, *NPMIDIOUTCAPSW, *LPMIDIOUTCAPSW;
#ifdef UNICODE
typedef MIDIOUTCAPSW MIDIOUTCAPS;
typedef PMIDIOUTCAPSW PMIDIOUTCAPS;
typedef NPMIDIOUTCAPSW NPMIDIOUTCAPS;
typedef LPMIDIOUTCAPSW LPMIDIOUTCAPS;
#else
typedef MIDIOUTCAPSA MIDIOUTCAPS;
typedef PMIDIOUTCAPSA PMIDIOUTCAPS;
typedef NPMIDIOUTCAPSA NPMIDIOUTCAPS;
typedef LPMIDIOUTCAPSA LPMIDIOUTCAPS;
#endif // UNICODE
typedef struct tagMIDIOUTCAPS2A {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    CHAR    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    WORD    wTechnology;           /* type of device */
    WORD    wVoices;               /* # of voices (internal synth only) */
    WORD    wNotes;                /* max # of notes (internal synth only) */
    WORD    wChannelMask;          /* channels used (internal synth only) */
    DWORD   dwSupport;             /* functionality supported by driver */
    GUID    ManufacturerGuid;      /* for extensible MID mapping */
    GUID    ProductGuid;           /* for extensible PID mapping */
    GUID    NameGuid;              /* for name lookup in registry */
} MIDIOUTCAPS2A, *PMIDIOUTCAPS2A, *NPMIDIOUTCAPS2A, *LPMIDIOUTCAPS2A;
typedef struct tagMIDIOUTCAPS2W {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    MMVERSION vDriverVersion;      /* version of the driver */
    WCHAR   szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    WORD    wTechnology;           /* type of device */
    WORD    wVoices;               /* # of voices (internal synth only) */
    WORD    wNotes;                /* max # of notes (internal synth only) */
    WORD    wChannelMask;          /* channels used (internal synth only) */
    DWORD   dwSupport;             /* functionality supported by driver */
    GUID    ManufacturerGuid;      /* for extensible MID mapping */
    GUID    ProductGuid;           /* for extensible PID mapping */
    GUID    NameGuid;              /* for name lookup in registry */
} MIDIOUTCAPS2W, *PMIDIOUTCAPS2W, *NPMIDIOUTCAPS2W, *LPMIDIOUTCAPS2W;
#ifdef UNICODE
typedef MIDIOUTCAPS2W MIDIOUTCAPS2;
typedef PMIDIOUTCAPS2W PMIDIOUTCAPS2;
typedef NPMIDIOUTCAPS2W NPMIDIOUTCAPS2;
typedef LPMIDIOUTCAPS2W LPMIDIOUTCAPS2;
#else
typedef MIDIOUTCAPS2A MIDIOUTCAPS2;
typedef PMIDIOUTCAPS2A PMIDIOUTCAPS2;
typedef NPMIDIOUTCAPS2A NPMIDIOUTCAPS2;
typedef LPMIDIOUTCAPS2A LPMIDIOUTCAPS2;
#endif // UNICODE

#else
typedef struct midioutcaps_tag {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    VERSION vDriverVersion;        /* version of the driver */
    char    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    WORD    wTechnology;           /* type of device */
    WORD    wVoices;               /* # of voices (internal synth only) */
    WORD    wNotes;                /* max # of notes (internal synth only) */
    WORD    wChannelMask;          /* channels used (internal synth only) */
    DWORD   dwSupport;             /* functionality supported by driver */
} MIDIOUTCAPS, *PMIDIOUTCAPS, NEAR *NPMIDIOUTCAPS, FAR *LPMIDIOUTCAPS;
#endif

/* flags for wTechnology field of MIDIOUTCAPS structure */
#define MOD_MIDIPORT    1  /* output port */
#define MOD_SYNTH       2  /* generic internal synth */
#define MOD_SQSYNTH     3  /* square wave internal synth */
#define MOD_FMSYNTH     4  /* FM internal synth */
#define MOD_MAPPER      5  /* MIDI mapper */
#define MOD_WAVETABLE   6  /* hardware wavetable synth */
#define MOD_SWSYNTH     7  /* software synth */

/* flags for dwSupport field of MIDIOUTCAPS structure */
#define MIDICAPS_VOLUME          0x0001  /* supports volume control */
#define MIDICAPS_LRVOLUME        0x0002  /* separate left-right volume control */
#define MIDICAPS_CACHE           0x0004
#if(WINVER >= 0x0400)
#define MIDICAPS_STREAM          0x0008  /* driver supports midiStreamOut directly */
#endif /* WINVER >= 0x0400 */

/* MIDI input device capabilities structure */
#ifdef _WIN32

typedef struct tagMIDIINCAPSA {
    WORD        wMid;                   /* manufacturer ID */
    WORD        wPid;                   /* product ID */
    MMVERSION   vDriverVersion;         /* version of the driver */
    CHAR        szPname[MAXPNAMELEN];   /* product name (NULL terminated string) */
#if (WINVER >= 0x0400)
    DWORD   dwSupport;             /* functionality supported by driver */
#endif
} MIDIINCAPSA, *PMIDIINCAPSA, *NPMIDIINCAPSA, *LPMIDIINCAPSA;
typedef struct tagMIDIINCAPSW {
    WORD        wMid;                   /* manufacturer ID */
    WORD        wPid;                   /* product ID */
    MMVERSION   vDriverVersion;         /* version of the driver */
    WCHAR       szPname[MAXPNAMELEN];   /* product name (NULL terminated string) */
#if (WINVER >= 0x0400)
    DWORD   dwSupport;             /* functionality supported by driver */
#endif
} MIDIINCAPSW, *PMIDIINCAPSW, *NPMIDIINCAPSW, *LPMIDIINCAPSW;
#ifdef UNICODE
typedef MIDIINCAPSW MIDIINCAPS;
typedef PMIDIINCAPSW PMIDIINCAPS;
typedef NPMIDIINCAPSW NPMIDIINCAPS;
typedef LPMIDIINCAPSW LPMIDIINCAPS;
#else
typedef MIDIINCAPSA MIDIINCAPS;
typedef PMIDIINCAPSA PMIDIINCAPS;
typedef NPMIDIINCAPSA NPMIDIINCAPS;
typedef LPMIDIINCAPSA LPMIDIINCAPS;
#endif // UNICODE
typedef struct tagMIDIINCAPS2A {
    WORD        wMid;                   /* manufacturer ID */
    WORD        wPid;                   /* product ID */
    MMVERSION   vDriverVersion;         /* version of the driver */
    CHAR        szPname[MAXPNAMELEN];   /* product name (NULL terminated string) */
#if (WINVER >= 0x0400)
    DWORD       dwSupport;              /* functionality supported by driver */
#endif
    GUID        ManufacturerGuid;       /* for extensible MID mapping */
    GUID        ProductGuid;            /* for extensible PID mapping */
    GUID        NameGuid;               /* for name lookup in registry */
} MIDIINCAPS2A, *PMIDIINCAPS2A, *NPMIDIINCAPS2A, *LPMIDIINCAPS2A;
typedef struct tagMIDIINCAPS2W {
    WORD        wMid;                   /* manufacturer ID */
    WORD        wPid;                   /* product ID */
    MMVERSION   vDriverVersion;         /* version of the driver */
    WCHAR       szPname[MAXPNAMELEN];   /* product name (NULL terminated string) */
#if (WINVER >= 0x0400)
    DWORD       dwSupport;              /* functionality supported by driver */
#endif
    GUID        ManufacturerGuid;       /* for extensible MID mapping */
    GUID        ProductGuid;            /* for extensible PID mapping */
    GUID        NameGuid;               /* for name lookup in registry */
} MIDIINCAPS2W, *PMIDIINCAPS2W, *NPMIDIINCAPS2W, *LPMIDIINCAPS2W;
#ifdef UNICODE
typedef MIDIINCAPS2W MIDIINCAPS2;
typedef PMIDIINCAPS2W PMIDIINCAPS2;
typedef NPMIDIINCAPS2W NPMIDIINCAPS2;
typedef LPMIDIINCAPS2W LPMIDIINCAPS2;
#else
typedef MIDIINCAPS2A MIDIINCAPS2;
typedef PMIDIINCAPS2A PMIDIINCAPS2;
typedef NPMIDIINCAPS2A NPMIDIINCAPS2;
typedef LPMIDIINCAPS2A LPMIDIINCAPS2;
#endif // UNICODE

#else
typedef struct midiincaps_tag {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    VERSION vDriverVersion;        /* version of the driver */
    char    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
#if (WINVER >= 0x0400)
    DWORD   dwSupport;             /* functionality supported by driver */
#endif
} MIDIINCAPS, *PMIDIINCAPS, NEAR *NPMIDIINCAPS, FAR *LPMIDIINCAPS;
#endif

/* MIDI data block header */
typedef struct midihdr_tag {
    LPSTR       lpData;               /* pointer to locked data block */
    DWORD       dwBufferLength;       /* length of data in data block */
    DWORD       dwBytesRecorded;      /* used for input only */
    DWORD_PTR   dwUser;               /* for client's use */
    DWORD       dwFlags;              /* assorted flags (see defines) */
    struct midihdr_tag far *lpNext;   /* reserved for driver */
    DWORD_PTR   reserved;             /* reserved for driver */
#if (WINVER >= 0x0400)
    DWORD       dwOffset;             /* Callback offset into buffer */
    DWORD_PTR   dwReserved[8];        /* Reserved for MMSYSTEM */
#endif
} MIDIHDR, *PMIDIHDR, NEAR *NPMIDIHDR, FAR *LPMIDIHDR;

#if(WINVER >= 0x0400)
typedef struct midievent_tag
{
    DWORD       dwDeltaTime;          /* Ticks since last event */
    DWORD       dwStreamID;           /* Reserved; must be zero */
    DWORD       dwEvent;              /* Event type and parameters */
    DWORD       dwParms[1];           /* Parameters if this is a long event */
} MIDIEVENT;

typedef struct midistrmbuffver_tag
{
    DWORD       dwVersion;                  /* Stream buffer format version */
    DWORD       dwMid;                      /* Manufacturer ID as defined in MMREG.H */
    DWORD       dwOEMVersion;               /* Manufacturer version for custom ext */
} MIDISTRMBUFFVER;
#endif /* WINVER >= 0x0400 */

/* flags for dwFlags field of MIDIHDR structure */
#define MHDR_DONE       0x00000001       /* done bit */
#define MHDR_PREPARED   0x00000002       /* set if header prepared */
#define MHDR_INQUEUE    0x00000004       /* reserved for driver */
#define MHDR_ISSTRM     0x00000008       /* Buffer is stream buffer */
#if(WINVER >= 0x0400)
/* */
/* Type codes which go in the high byte of the event DWORD of a stream buffer */
/* */
/* Type codes 00-7F contain parameters within the low 24 bits */
/* Type codes 80-FF contain a length of their parameter in the low 24 */
/* bits, followed by their parameter data in the buffer. The event */
/* DWORD contains the exact byte length; the parm data itself must be */
/* padded to be an even multiple of 4 bytes long. */
/* */

#define MEVT_F_SHORT        0x00000000L
#define MEVT_F_LONG         0x80000000L
#define MEVT_F_CALLBACK     0x40000000L

#define MEVT_EVENTTYPE(x)   ((BYTE)(((x)>>24)&0xFF))
#define MEVT_EVENTPARM(x)   ((DWORD)((x)&0x00FFFFFFL))

#define MEVT_SHORTMSG       ((BYTE)0x00)    /* parm = shortmsg for midiOutShortMsg */
#define MEVT_TEMPO          ((BYTE)0x01)    /* parm = new tempo in microsec/qn     */
#define MEVT_NOP            ((BYTE)0x02)    /* parm = unused; does nothing         */

/* 0x04-0x7F reserved */

#define MEVT_LONGMSG        ((BYTE)0x80)    /* parm = bytes to send verbatim       */
#define MEVT_COMMENT        ((BYTE)0x82)    /* parm = comment data                 */
#define MEVT_VERSION        ((BYTE)0x84)    /* parm = MIDISTRMBUFFVER struct       */

/* 0x81-0xFF reserved */

#define MIDISTRM_ERROR      (-2)

/* */
/* Structures and defines for midiStreamProperty */
/* */
#define MIDIPROP_SET        0x80000000L
#define MIDIPROP_GET        0x40000000L

/* These are intentionally both non-zero so the app cannot accidentally */
/* leave the operation off and happen to appear to work due to default */
/* action. */

#define MIDIPROP_TIMEDIV    0x00000001L
#define MIDIPROP_TEMPO      0x00000002L

typedef struct midiproptimediv_tag
{
    DWORD       cbStruct;
    DWORD       dwTimeDiv;
} MIDIPROPTIMEDIV, FAR *LPMIDIPROPTIMEDIV;

typedef struct midiproptempo_tag
{
    DWORD       cbStruct;
    DWORD       dwTempo;
} MIDIPROPTEMPO, FAR *LPMIDIPROPTEMPO;

#endif /* WINVER >= 0x0400 */

/* MIDI function prototypes */
WINMMAPI
UINT
WINAPI
midiOutGetNumDevs(
    void
    );

#if (WINVER >= 0x0400)
WINMMAPI
MMRESULT
WINAPI
midiStreamOpen(
    _Out_ LPHMIDISTRM phms,
    _Inout_updates_(cMidi) LPUINT puDeviceID,
    _In_ DWORD cMidi,
    _In_opt_ DWORD_PTR dwCallback,
    _In_opt_ DWORD_PTR dwInstance,
    _In_ DWORD fdwOpen
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamClose(
    _In_ HMIDISTRM hms
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamProperty(
    _In_ HMIDISTRM hms,
    _Inout_updates_bytes_(sizeof(DWORD)+sizeof(DWORD)) LPBYTE lppropdata,
    _In_ DWORD dwProperty
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamPosition(
    _In_ HMIDISTRM hms,
    _Out_writes_bytes_(cbmmt) LPMMTIME lpmmt,
    _In_ UINT cbmmt
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamOut(
    _In_ HMIDISTRM hms,
    _Out_writes_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamPause(
    _In_ HMIDISTRM hms
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamRestart(
    _In_ HMIDISTRM hms
    );

WINMMAPI
MMRESULT
WINAPI
midiStreamStop(
    _In_ HMIDISTRM hms
    );

#ifdef _WIN32
WINMMAPI
MMRESULT
WINAPI
midiConnect(
    _In_ HMIDI hmi,
    _In_ HMIDIOUT hmo,
    _In_opt_ LPVOID pReserved
    );

WINMMAPI
MMRESULT
WINAPI
midiDisconnect(
    _In_ HMIDI hmi,
    _In_ HMIDIOUT hmo,
    _In_opt_ LPVOID pReserved
    );

#endif
#endif /* WINVER >= 0x0400 */

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
midiOutGetDevCapsA(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbmoc) LPMIDIOUTCAPSA pmoc,
    _In_ UINT cbmoc
    );

WINMMAPI
MMRESULT
WINAPI
midiOutGetDevCapsW(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbmoc) LPMIDIOUTCAPSW pmoc,
    _In_ UINT cbmoc
    );

#ifdef UNICODE
#define midiOutGetDevCaps  midiOutGetDevCapsW
#else
#define midiOutGetDevCaps  midiOutGetDevCapsA
#endif // !UNICODE

#else
MMRESULT WINAPI midiOutGetDevCaps(UINT uDeviceID, LPMIDIOUTCAPS pmoc, UINT cbmoc);
#endif

#if (WINVER >= 0x0400)
WINMMAPI
MMRESULT
WINAPI
midiOutGetVolume(
    _In_opt_ HMIDIOUT hmo,
    _Out_ LPDWORD pdwVolume
    );

WINMMAPI
MMRESULT
WINAPI
midiOutSetVolume(
    _In_opt_ HMIDIOUT hmo,
    _In_ DWORD dwVolume
    );

#else
WINMMAPI MMRESULT WINAPI midiOutGetVolume(UINT uId, LPDWORD pdwVolume);
WINMMAPI MMRESULT WINAPI midiOutSetVolume(UINT uId, DWORD dwVolume);
#endif

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
midiOutGetErrorTextA(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPSTR pszText,
    _In_ UINT cchText
    );

WINMMAPI
MMRESULT
WINAPI
midiOutGetErrorTextW(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPWSTR pszText,
    _In_ UINT cchText
    );

#ifdef UNICODE
#define midiOutGetErrorText  midiOutGetErrorTextW
#else
#define midiOutGetErrorText  midiOutGetErrorTextA
#endif // !UNICODE

#else
WINMMAPI MMRESULT WINAPI midiOutGetErrorText(MMRESULT mmrError, LPSTR pszText, UINT cchText);
#endif

WINMMAPI
MMRESULT
WINAPI
midiOutOpen(
    _Out_ LPHMIDIOUT phmo,
    _In_ UINT uDeviceID,
    _In_opt_ DWORD_PTR dwCallback,
    _In_opt_ DWORD_PTR dwInstance,
    _In_ DWORD fdwOpen
    );

WINMMAPI
MMRESULT
WINAPI
midiOutClose(
    _In_ HMIDIOUT hmo
    );

WINMMAPI
MMRESULT
WINAPI
midiOutPrepareHeader(
    _In_ HMIDIOUT hmo,
    _Inout_updates_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiOutUnprepareHeader(
    _In_ HMIDIOUT hmo,
    _Inout_updates_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiOutShortMsg(
    _In_ HMIDIOUT hmo,
    _In_ DWORD dwMsg
    );

WINMMAPI
MMRESULT
WINAPI
midiOutLongMsg(
    _In_ HMIDIOUT hmo,
    _In_reads_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiOutReset(
    _In_ HMIDIOUT hmo
    );

WINMMAPI
MMRESULT
WINAPI
midiOutCachePatches(
    _In_ HMIDIOUT hmo,
    _In_ UINT uBank,
    _In_reads_(MIDIPATCHSIZE) LPWORD pwpa,
    _In_ UINT fuCache
    );

WINMMAPI
MMRESULT
WINAPI
midiOutCacheDrumPatches(
    _In_ HMIDIOUT hmo,
    _In_ UINT uPatch,
    _In_reads_(MIDIPATCHSIZE) LPWORD pwkya,
    _In_ UINT fuCache
    );

WINMMAPI
MMRESULT
WINAPI
midiOutGetID(
    _In_ HMIDIOUT hmo,
    _Out_ LPUINT puDeviceID
    );

#if (WINVER >= 0x030a)
#ifdef _WIN32
WINMMAPI
MMRESULT
WINAPI
midiOutMessage(
    _In_opt_ HMIDIOUT hmo,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dw1,
    _In_opt_ DWORD_PTR dw2
    );

#else
DWORD WINAPI midiOutMessage(HMIDIOUT hmo, UINT uMsg, DWORD dw1, DWORD dw2);
#endif
#endif /* ifdef WINVER >= 0x030a */

WINMMAPI
UINT
WINAPI
midiInGetNumDevs(
    void
    );

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
midiInGetDevCapsA(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbmic) LPMIDIINCAPSA pmic,
    _In_ UINT cbmic
    );

WINMMAPI
MMRESULT
WINAPI
midiInGetDevCapsW(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbmic) LPMIDIINCAPSW pmic,
    _In_ UINT cbmic
    );

#ifdef UNICODE
#define midiInGetDevCaps  midiInGetDevCapsW
#else
#define midiInGetDevCaps  midiInGetDevCapsA
#endif // !UNICODE
WINMMAPI
MMRESULT
WINAPI
midiInGetErrorTextA(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPSTR pszText,
    _In_ UINT cchText
    );

WINMMAPI
MMRESULT
WINAPI
midiInGetErrorTextW(
    _In_ MMRESULT mmrError,
    _Out_writes_(cchText) LPWSTR pszText,
    _In_ UINT cchText
    );

#ifdef UNICODE
#define midiInGetErrorText  midiInGetErrorTextW
#else
#define midiInGetErrorText  midiInGetErrorTextA
#endif // !UNICODE

#else
MMRESULT WINAPI midiInGetDevCaps(UINT uDeviceID, LPMIDIINCAPS pmic, UINT cbmic);
WINMMAPI MMRESULT WINAPI midiInGetErrorText(MMRESULT mmrError, _Out_writes_(cchText) LPSTR pszText, UINT cchText);
#endif

WINMMAPI
MMRESULT
WINAPI
midiInOpen(
    _Out_ LPHMIDIIN phmi,
    _In_ UINT uDeviceID,
    _In_opt_ DWORD_PTR dwCallback,
    _In_opt_ DWORD_PTR dwInstance,
    _In_ DWORD fdwOpen
    );

WINMMAPI
MMRESULT
WINAPI
midiInClose(
    _In_ HMIDIIN hmi
    );

WINMMAPI
MMRESULT
WINAPI
midiInPrepareHeader(
    _In_ HMIDIIN hmi,
    _Inout_updates_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiInUnprepareHeader(
    _In_ HMIDIIN hmi,
    _Inout_updates_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiInAddBuffer(
    _In_ HMIDIIN hmi,
    _Out_writes_bytes_(cbmh) LPMIDIHDR pmh,
    _In_ UINT cbmh
    );

WINMMAPI
MMRESULT
WINAPI
midiInStart(
    _In_ HMIDIIN hmi
    );

WINMMAPI
MMRESULT
WINAPI
midiInStop(
    _In_ HMIDIIN hmi
    );

WINMMAPI
MMRESULT
WINAPI
midiInReset(
    _In_ HMIDIIN hmi
    );

WINMMAPI
MMRESULT
WINAPI
midiInGetID(
    _In_ HMIDIIN hmi,
    _Out_ LPUINT puDeviceID
    );

#if (WINVER >= 0x030a)
#ifdef _WIN32
WINMMAPI
MMRESULT
WINAPI
midiInMessage(
    _In_opt_ HMIDIIN hmi,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dw1,
    _In_opt_ DWORD_PTR dw2
    );

#else
DWORD WINAPI midiInMessage(HMIDIIN hmi, UINT uMsg, DWORD dw1, DWORD dw2);
#endif
#endif /* ifdef WINVER >= 0x030a */

#endif  /* ifndef MMNOMIDI */

#ifndef MMNOAUX
/****************************************************************************

                        Auxiliary audio support

****************************************************************************/

/* device ID for aux device mapper */
#define AUX_MAPPER     ((UINT)-1)

/* Auxiliary audio device capabilities structure */
#ifdef _WIN32

typedef struct tagAUXCAPSA {
    WORD        wMid;                /* manufacturer ID */
    WORD        wPid;                /* product ID */
    MMVERSION   vDriverVersion;      /* version of the driver */
    CHAR        szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    WORD        wTechnology;         /* type of device */
    WORD        wReserved1;          /* padding */
    DWORD       dwSupport;           /* functionality supported by driver */
} AUXCAPSA, *PAUXCAPSA, *NPAUXCAPSA, *LPAUXCAPSA;
typedef struct tagAUXCAPSW {
    WORD        wMid;                /* manufacturer ID */
    WORD        wPid;                /* product ID */
    MMVERSION   vDriverVersion;      /* version of the driver */
    WCHAR       szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    WORD        wTechnology;         /* type of device */
    WORD        wReserved1;          /* padding */
    DWORD       dwSupport;           /* functionality supported by driver */
} AUXCAPSW, *PAUXCAPSW, *NPAUXCAPSW, *LPAUXCAPSW;
#ifdef UNICODE
typedef AUXCAPSW AUXCAPS;
typedef PAUXCAPSW PAUXCAPS;
typedef NPAUXCAPSW NPAUXCAPS;
typedef LPAUXCAPSW LPAUXCAPS;
#else
typedef AUXCAPSA AUXCAPS;
typedef PAUXCAPSA PAUXCAPS;
typedef NPAUXCAPSA NPAUXCAPS;
typedef LPAUXCAPSA LPAUXCAPS;
#endif // UNICODE
typedef struct tagAUXCAPS2A {
    WORD        wMid;                /* manufacturer ID */
    WORD        wPid;                /* product ID */
    MMVERSION   vDriverVersion;      /* version of the driver */
    CHAR        szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    WORD        wTechnology;         /* type of device */
    WORD        wReserved1;          /* padding */
    DWORD       dwSupport;           /* functionality supported by driver */
    GUID        ManufacturerGuid;    /* for extensible MID mapping */
    GUID        ProductGuid;         /* for extensible PID mapping */
    GUID        NameGuid;            /* for name lookup in registry */
} AUXCAPS2A, *PAUXCAPS2A, *NPAUXCAPS2A, *LPAUXCAPS2A;
typedef struct tagAUXCAPS2W {
    WORD        wMid;                /* manufacturer ID */
    WORD        wPid;                /* product ID */
    MMVERSION   vDriverVersion;      /* version of the driver */
    WCHAR       szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    WORD        wTechnology;         /* type of device */
    WORD        wReserved1;          /* padding */
    DWORD       dwSupport;           /* functionality supported by driver */
    GUID        ManufacturerGuid;    /* for extensible MID mapping */
    GUID        ProductGuid;         /* for extensible PID mapping */
    GUID        NameGuid;            /* for name lookup in registry */
} AUXCAPS2W, *PAUXCAPS2W, *NPAUXCAPS2W, *LPAUXCAPS2W;
#ifdef UNICODE
typedef AUXCAPS2W AUXCAPS2;
typedef PAUXCAPS2W PAUXCAPS2;
typedef NPAUXCAPS2W NPAUXCAPS2;
typedef LPAUXCAPS2W LPAUXCAPS2;
#else
typedef AUXCAPS2A AUXCAPS2;
typedef PAUXCAPS2A PAUXCAPS2;
typedef NPAUXCAPS2A NPAUXCAPS2;
typedef LPAUXCAPS2A LPAUXCAPS2;
#endif // UNICODE

#else
typedef struct auxcaps_tag {
    WORD    wMid;                  /* manufacturer ID */
    WORD    wPid;                  /* product ID */
    VERSION vDriverVersion;        /* version of the driver */
    char    szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    WORD    wTechnology;           /* type of device */
    DWORD   dwSupport;             /* functionality supported by driver */
} AUXCAPS, *PAUXCAPS, NEAR *NPAUXCAPS, FAR *LPAUXCAPS;
#endif

/* flags for wTechnology field in AUXCAPS structure */
#define AUXCAPS_CDAUDIO    1       /* audio from internal CD-ROM drive */
#define AUXCAPS_AUXIN      2       /* audio from auxiliary input jacks */

/* flags for dwSupport field in AUXCAPS structure */
#define AUXCAPS_VOLUME          0x0001  /* supports volume control */
#define AUXCAPS_LRVOLUME        0x0002  /* separate left-right volume control */

/* auxiliary audio function prototypes */
WINMMAPI
UINT
WINAPI
auxGetNumDevs(
    void
    );

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
auxGetDevCapsA(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbac) LPAUXCAPSA pac,
    _In_ UINT cbac
    );

WINMMAPI
MMRESULT
WINAPI
auxGetDevCapsW(
    _In_ UINT_PTR uDeviceID,
    _Out_writes_bytes_(cbac) LPAUXCAPSW pac,
    _In_ UINT cbac
    );

#ifdef UNICODE
#define auxGetDevCaps  auxGetDevCapsW
#else
#define auxGetDevCaps  auxGetDevCapsA
#endif // !UNICODE

#else
MMRESULT WINAPI auxGetDevCaps(UINT uDeviceID, LPAUXCAPS pac, UINT cbac);
#endif
WINMMAPI
MMRESULT
WINAPI
auxSetVolume(
    _In_ UINT uDeviceID,
    _In_ DWORD dwVolume
    );

WINMMAPI
MMRESULT
WINAPI
auxGetVolume(
    _In_ UINT uDeviceID,
    _Out_ LPDWORD pdwVolume
    );

#if (WINVER >= 0x030a)
#ifdef _WIN32
WINMMAPI
MMRESULT
WINAPI
auxOutMessage(
    _In_ UINT uDeviceID,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dw1,
    _In_opt_ DWORD_PTR dw2
    );

#else
DWORD WINAPI auxOutMessage(UINT uDeviceID, UINT uMsg, DWORD dw1, DWORD dw2);
#endif
#endif /* ifdef WINVER >= 0x030a */

#endif  /* ifndef MMNOAUX */

#ifndef MMNOMIXER
/****************************************************************************

                            Mixer Support

****************************************************************************/

DECLARE_HANDLE(HMIXEROBJ);
typedef HMIXEROBJ FAR *LPHMIXEROBJ;

DECLARE_HANDLE(HMIXER);
typedef HMIXER     FAR *LPHMIXER;

#define MIXER_SHORT_NAME_CHARS   16
#define MIXER_LONG_NAME_CHARS    64

/* */
/*  MMRESULT error return values specific to the mixer API */
/* */
/* */
#define MIXERR_INVALLINE            (MIXERR_BASE + 0)
#define MIXERR_INVALCONTROL         (MIXERR_BASE + 1)
#define MIXERR_INVALVALUE           (MIXERR_BASE + 2)
#define MIXERR_LASTERROR            (MIXERR_BASE + 2)

#define MIXER_OBJECTF_HANDLE    0x80000000L
#define MIXER_OBJECTF_MIXER     0x00000000L
#define MIXER_OBJECTF_HMIXER    (MIXER_OBJECTF_HANDLE|MIXER_OBJECTF_MIXER)
#define MIXER_OBJECTF_WAVEOUT   0x10000000L
#define MIXER_OBJECTF_HWAVEOUT  (MIXER_OBJECTF_HANDLE|MIXER_OBJECTF_WAVEOUT)
#define MIXER_OBJECTF_WAVEIN    0x20000000L
#define MIXER_OBJECTF_HWAVEIN   (MIXER_OBJECTF_HANDLE|MIXER_OBJECTF_WAVEIN)
#define MIXER_OBJECTF_MIDIOUT   0x30000000L
#define MIXER_OBJECTF_HMIDIOUT  (MIXER_OBJECTF_HANDLE|MIXER_OBJECTF_MIDIOUT)
#define MIXER_OBJECTF_MIDIIN    0x40000000L
#define MIXER_OBJECTF_HMIDIIN   (MIXER_OBJECTF_HANDLE|MIXER_OBJECTF_MIDIIN)
#define MIXER_OBJECTF_AUX       0x50000000L

WINMMAPI
UINT
WINAPI
mixerGetNumDevs(
    void
    );

#ifdef _WIN32

typedef struct tagMIXERCAPSA {
    WORD            wMid;                   /* manufacturer id */
    WORD            wPid;                   /* product id */
    MMVERSION       vDriverVersion;         /* version of the driver */
    CHAR            szPname[MAXPNAMELEN];   /* product name */
    DWORD           fdwSupport;             /* misc. support bits */
    DWORD           cDestinations;          /* count of destinations */
} MIXERCAPSA, *PMIXERCAPSA, *LPMIXERCAPSA;
typedef struct tagMIXERCAPSW {
    WORD            wMid;                   /* manufacturer id */
    WORD            wPid;                   /* product id */
    MMVERSION       vDriverVersion;         /* version of the driver */
    WCHAR           szPname[MAXPNAMELEN];   /* product name */
    DWORD           fdwSupport;             /* misc. support bits */
    DWORD           cDestinations;          /* count of destinations */
} MIXERCAPSW, *PMIXERCAPSW, *LPMIXERCAPSW;
#ifdef UNICODE
typedef MIXERCAPSW MIXERCAPS;
typedef PMIXERCAPSW PMIXERCAPS;
typedef LPMIXERCAPSW LPMIXERCAPS;
#else
typedef MIXERCAPSA MIXERCAPS;
typedef PMIXERCAPSA PMIXERCAPS;
typedef LPMIXERCAPSA LPMIXERCAPS;
#endif // UNICODE
typedef struct tagMIXERCAPS2A {
    WORD            wMid;                   /* manufacturer id */
    WORD            wPid;                   /* product id */
    MMVERSION       vDriverVersion;         /* version of the driver */
    CHAR            szPname[MAXPNAMELEN];   /* product name */
    DWORD           fdwSupport;             /* misc. support bits */
    DWORD           cDestinations;          /* count of destinations */
    GUID            ManufacturerGuid;       /* for extensible MID mapping */
    GUID            ProductGuid;            /* for extensible PID mapping */
    GUID            NameGuid;               /* for name lookup in registry */
} MIXERCAPS2A, *PMIXERCAPS2A, *LPMIXERCAPS2A;
typedef struct tagMIXERCAPS2W {
    WORD            wMid;                   /* manufacturer id */
    WORD            wPid;                   /* product id */
    MMVERSION       vDriverVersion;         /* version of the driver */
    WCHAR           szPname[MAXPNAMELEN];   /* product name */
    DWORD           fdwSupport;             /* misc. support bits */
    DWORD           cDestinations;          /* count of destinations */
    GUID            ManufacturerGuid;       /* for extensible MID mapping */
    GUID            ProductGuid;            /* for extensible PID mapping */
    GUID            NameGuid;               /* for name lookup in registry */
} MIXERCAPS2W, *PMIXERCAPS2W, *LPMIXERCAPS2W;
#ifdef UNICODE
typedef MIXERCAPS2W MIXERCAPS2;
typedef PMIXERCAPS2W PMIXERCAPS2;
typedef LPMIXERCAPS2W LPMIXERCAPS2;
#else
typedef MIXERCAPS2A MIXERCAPS2;
typedef PMIXERCAPS2A PMIXERCAPS2;
typedef LPMIXERCAPS2A LPMIXERCAPS2;
#endif // UNICODE

#else
typedef struct tMIXERCAPS {
    WORD            wMid;                   /* manufacturer id */
    WORD            wPid;                   /* product id */
    VERSION         vDriverVersion;         /* version of the driver */
    char            szPname[MAXPNAMELEN];   /* product name */
    DWORD           fdwSupport;             /* misc. support bits */
    DWORD           cDestinations;          /* count of destinations */
} MIXERCAPS, *PMIXERCAPS, FAR *LPMIXERCAPS;
#endif

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
mixerGetDevCapsA(
    _In_ UINT_PTR uMxId,
    _Out_writes_bytes_(cbmxcaps) LPMIXERCAPSA pmxcaps,
    _In_ UINT cbmxcaps
    );

WINMMAPI
MMRESULT
WINAPI
mixerGetDevCapsW(
    _In_ UINT_PTR uMxId,
    _Out_writes_bytes_(cbmxcaps) LPMIXERCAPSW pmxcaps,
    _In_ UINT cbmxcaps
    );

#ifdef UNICODE
#define mixerGetDevCaps  mixerGetDevCapsW
#else
#define mixerGetDevCaps  mixerGetDevCapsA
#endif // !UNICODE

#else
MMRESULT WINAPI mixerGetDevCaps(UINT uMxId, LPMIXERCAPS pmxcaps, UINT cbmxcaps);
#endif

WINMMAPI
MMRESULT
WINAPI
mixerOpen(
    _Out_opt_ LPHMIXER phmx,
    _In_ UINT uMxId,
    _In_opt_ DWORD_PTR dwCallback,
    _In_opt_ DWORD_PTR dwInstance,
    _In_ DWORD fdwOpen
    );

WINMMAPI
MMRESULT
WINAPI
mixerClose(
    _In_ HMIXER hmx
    );

WINMMAPI
DWORD
WINAPI
mixerMessage(
    _In_opt_ HMIXER hmx,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dwParam1,
    _In_opt_ DWORD_PTR dwParam2
    );

#ifdef _WIN32

typedef struct tagMIXERLINEA {
    DWORD       cbStruct;               /* size of MIXERLINE structure */
    DWORD       dwDestination;          /* zero based destination index */
    DWORD       dwSource;               /* zero based source index (if source) */
    DWORD       dwLineID;               /* unique line id for mixer device */
    DWORD       fdwLine;                /* state/information about line */
    DWORD_PTR   dwUser;                 /* driver specific information */
    DWORD       dwComponentType;        /* component type line connects to */
    DWORD       cChannels;              /* number of channels line supports */
    DWORD       cConnections;           /* number of connections [possible] */
    DWORD       cControls;              /* number of controls at this line */
    CHAR        szShortName[MIXER_SHORT_NAME_CHARS];
    CHAR        szName[MIXER_LONG_NAME_CHARS];
    struct {
        DWORD       dwType;                 /* MIXERLINE_TARGETTYPE_xxxx */
        DWORD       dwDeviceID;             /* target device ID of device type */
        WORD        wMid;                   /* of target device */
        WORD        wPid;                   /*      " */
        MMVERSION   vDriverVersion;         /*      " */
        CHAR        szPname[MAXPNAMELEN];   /*      " */
    } Target;
} MIXERLINEA, *PMIXERLINEA, *LPMIXERLINEA;
typedef struct tagMIXERLINEW {
    DWORD       cbStruct;               /* size of MIXERLINE structure */
    DWORD       dwDestination;          /* zero based destination index */
    DWORD       dwSource;               /* zero based source index (if source) */
    DWORD       dwLineID;               /* unique line id for mixer device */
    DWORD       fdwLine;                /* state/information about line */
    DWORD_PTR   dwUser;                 /* driver specific information */
    DWORD       dwComponentType;        /* component type line connects to */
    DWORD       cChannels;              /* number of channels line supports */
    DWORD       cConnections;           /* number of connections [possible] */
    DWORD       cControls;              /* number of controls at this line */
    WCHAR       szShortName[MIXER_SHORT_NAME_CHARS];
    WCHAR       szName[MIXER_LONG_NAME_CHARS];
    struct {
        DWORD       dwType;                 /* MIXERLINE_TARGETTYPE_xxxx */
        DWORD       dwDeviceID;             /* target device ID of device type */
        WORD        wMid;                   /* of target device */
        WORD        wPid;                   /*      " */
        MMVERSION   vDriverVersion;         /*      " */
        WCHAR       szPname[MAXPNAMELEN];   /*      " */
    } Target;
} MIXERLINEW, *PMIXERLINEW, *LPMIXERLINEW;
#ifdef UNICODE
typedef MIXERLINEW MIXERLINE;
typedef PMIXERLINEW PMIXERLINE;
typedef LPMIXERLINEW LPMIXERLINE;
#else
typedef MIXERLINEA MIXERLINE;
typedef PMIXERLINEA PMIXERLINE;
typedef LPMIXERLINEA LPMIXERLINE;
#endif // UNICODE

#else
typedef struct tMIXERLINE {
    DWORD       cbStruct;               /* size of MIXERLINE structure */
    DWORD       dwDestination;          /* zero based destination index */
    DWORD       dwSource;               /* zero based source index (if source) */
    DWORD       dwLineID;               /* unique line id for mixer device */
    DWORD       fdwLine;                /* state/information about line */
    DWORD       dwUser;                 /* driver specific information */
    DWORD       dwComponentType;        /* component type line connects to */
    DWORD       cChannels;              /* number of channels line supports */
    DWORD       cConnections;           /* number of connections [possible] */
    DWORD       cControls;              /* number of controls at this line */
    char        szShortName[MIXER_SHORT_NAME_CHARS];
    char        szName[MIXER_LONG_NAME_CHARS];
    struct {
        DWORD   dwType;                 /* MIXERLINE_TARGETTYPE_xxxx */
        DWORD   dwDeviceID;             /* target device ID of device type */
        WORD    wMid;                   /* of target device */
        WORD    wPid;                   /*      " */
        VERSION vDriverVersion;         /*      " */
        char    szPname[MAXPNAMELEN];   /*      " */
    } Target;
} MIXERLINE, *PMIXERLINE, FAR *LPMIXERLINE;
#endif

/* */
/*  MIXERLINE.fdwLine */
/* */
/* */
#define MIXERLINE_LINEF_ACTIVE              0x00000001L
#define MIXERLINE_LINEF_DISCONNECTED        0x00008000L
#define MIXERLINE_LINEF_SOURCE              0x80000000L

/* */
/*  MIXERLINE.dwComponentType */
/* */
/*  component types for destinations and sources */
/* */
/* */
#define MIXERLINE_COMPONENTTYPE_DST_FIRST       0x00000000L
#define MIXERLINE_COMPONENTTYPE_DST_UNDEFINED   (MIXERLINE_COMPONENTTYPE_DST_FIRST + 0)
#define MIXERLINE_COMPONENTTYPE_DST_DIGITAL     (MIXERLINE_COMPONENTTYPE_DST_FIRST + 1)
#define MIXERLINE_COMPONENTTYPE_DST_LINE        (MIXERLINE_COMPONENTTYPE_DST_FIRST + 2)
#define MIXERLINE_COMPONENTTYPE_DST_MONITOR     (MIXERLINE_COMPONENTTYPE_DST_FIRST + 3)
#define MIXERLINE_COMPONENTTYPE_DST_SPEAKERS    (MIXERLINE_COMPONENTTYPE_DST_FIRST + 4)
#define MIXERLINE_COMPONENTTYPE_DST_HEADPHONES  (MIXERLINE_COMPONENTTYPE_DST_FIRST + 5)
#define MIXERLINE_COMPONENTTYPE_DST_TELEPHONE   (MIXERLINE_COMPONENTTYPE_DST_FIRST + 6)
#define MIXERLINE_COMPONENTTYPE_DST_WAVEIN      (MIXERLINE_COMPONENTTYPE_DST_FIRST + 7)
#define MIXERLINE_COMPONENTTYPE_DST_VOICEIN     (MIXERLINE_COMPONENTTYPE_DST_FIRST + 8)
#define MIXERLINE_COMPONENTTYPE_DST_LAST        (MIXERLINE_COMPONENTTYPE_DST_FIRST + 8)

#define MIXERLINE_COMPONENTTYPE_SRC_FIRST       0x00001000L
#define MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED   (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 0)
#define MIXERLINE_COMPONENTTYPE_SRC_DIGITAL     (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 1)
#define MIXERLINE_COMPONENTTYPE_SRC_LINE        (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 2)
#define MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE  (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 3)
#define MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 4)
#define MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 5)
#define MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE   (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 6)
#define MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER   (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 7)
#define MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT     (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 8)
#define MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY   (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 9)
#define MIXERLINE_COMPONENTTYPE_SRC_ANALOG      (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 10)
#define MIXERLINE_COMPONENTTYPE_SRC_LAST        (MIXERLINE_COMPONENTTYPE_SRC_FIRST + 10)

/* */
/*  MIXERLINE.Target.dwType */
/* */
/* */
#define MIXERLINE_TARGETTYPE_UNDEFINED      0
#define MIXERLINE_TARGETTYPE_WAVEOUT        1
#define MIXERLINE_TARGETTYPE_WAVEIN         2
#define MIXERLINE_TARGETTYPE_MIDIOUT        3
#define MIXERLINE_TARGETTYPE_MIDIIN         4
#define MIXERLINE_TARGETTYPE_AUX            5

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
mixerGetLineInfoA(
    _In_opt_ HMIXEROBJ hmxobj,
    _Inout_ LPMIXERLINEA pmxl,
    _In_ DWORD fdwInfo
    );

WINMMAPI
MMRESULT
WINAPI
mixerGetLineInfoW(
    _In_opt_ HMIXEROBJ hmxobj,
    _Inout_ LPMIXERLINEW pmxl,
    _In_ DWORD fdwInfo
    );

#ifdef UNICODE
#define mixerGetLineInfo  mixerGetLineInfoW
#else
#define mixerGetLineInfo  mixerGetLineInfoA
#endif // !UNICODE

#else
MMRESULT WINAPI mixerGetLineInfo(HMIXEROBJ hmxobj, LPMIXERLINE pmxl, DWORD fdwInfo);
#endif

#define MIXER_GETLINEINFOF_DESTINATION      0x00000000L
#define MIXER_GETLINEINFOF_SOURCE           0x00000001L
#define MIXER_GETLINEINFOF_LINEID           0x00000002L
#define MIXER_GETLINEINFOF_COMPONENTTYPE    0x00000003L
#define MIXER_GETLINEINFOF_TARGETTYPE       0x00000004L

#define MIXER_GETLINEINFOF_QUERYMASK        0x0000000FL

WINMMAPI
MMRESULT
WINAPI
mixerGetID(
    _In_opt_ HMIXEROBJ hmxobj,
    _Out_ UINT  FAR * puMxId,
    _In_ DWORD fdwId
    );

/* */
/*  MIXERCONTROL */
/* */
/* */
#ifdef _WIN32

typedef struct tagMIXERCONTROLA {
    DWORD           cbStruct;           /* size in bytes of MIXERCONTROL */
    DWORD           dwControlID;        /* unique control id for mixer device */
    DWORD           dwControlType;      /* MIXERCONTROL_CONTROLTYPE_xxx */
    DWORD           fdwControl;         /* MIXERCONTROL_CONTROLF_xxx */
    DWORD           cMultipleItems;     /* if MIXERCONTROL_CONTROLF_MULTIPLE set */
    CHAR            szShortName[MIXER_SHORT_NAME_CHARS];
    CHAR            szName[MIXER_LONG_NAME_CHARS];
    union {
        struct {
            LONG    lMinimum;           /* signed minimum for this control */
            LONG    lMaximum;           /* signed maximum for this control */
        } DUMMYSTRUCTNAME;
        struct {
            DWORD   dwMinimum;          /* unsigned minimum for this control */
            DWORD   dwMaximum;          /* unsigned maximum for this control */
        } DUMMYSTRUCTNAME2;
        DWORD       dwReserved[6];
    } Bounds;
    union {
        DWORD       cSteps;             /* # of steps between min & max */
        DWORD       cbCustomData;       /* size in bytes of custom data */
        DWORD       dwReserved[6];      /* !!! needed? we have cbStruct.... */
    } Metrics;
} MIXERCONTROLA, *PMIXERCONTROLA, *LPMIXERCONTROLA;
typedef struct tagMIXERCONTROLW {
    DWORD           cbStruct;           /* size in bytes of MIXERCONTROL */
    DWORD           dwControlID;        /* unique control id for mixer device */
    DWORD           dwControlType;      /* MIXERCONTROL_CONTROLTYPE_xxx */
    DWORD           fdwControl;         /* MIXERCONTROL_CONTROLF_xxx */
    DWORD           cMultipleItems;     /* if MIXERCONTROL_CONTROLF_MULTIPLE set */
    WCHAR           szShortName[MIXER_SHORT_NAME_CHARS];
    WCHAR           szName[MIXER_LONG_NAME_CHARS];
    union {
        struct {
            LONG    lMinimum;           /* signed minimum for this control */
            LONG    lMaximum;           /* signed maximum for this control */
        } DUMMYSTRUCTNAME;
        struct {
            DWORD   dwMinimum;          /* unsigned minimum for this control */
            DWORD   dwMaximum;          /* unsigned maximum for this control */
        } DUMMYSTRUCTNAME2;
        DWORD       dwReserved[6];
    } Bounds;
    union {
        DWORD       cSteps;             /* # of steps between min & max */
        DWORD       cbCustomData;       /* size in bytes of custom data */
        DWORD       dwReserved[6];      /* !!! needed? we have cbStruct.... */
    } Metrics;
} MIXERCONTROLW, *PMIXERCONTROLW, *LPMIXERCONTROLW;
#ifdef UNICODE
typedef MIXERCONTROLW MIXERCONTROL;
typedef PMIXERCONTROLW PMIXERCONTROL;
typedef LPMIXERCONTROLW LPMIXERCONTROL;
#else
typedef MIXERCONTROLA MIXERCONTROL;
typedef PMIXERCONTROLA PMIXERCONTROL;
typedef LPMIXERCONTROLA LPMIXERCONTROL;
#endif // UNICODE

#else
typedef struct tMIXERCONTROL {
    DWORD           cbStruct;           /* size in bytes of MIXERCONTROL */
    DWORD           dwControlID;        /* unique control id for mixer device */
    DWORD           dwControlType;      /* MIXERCONTROL_CONTROLTYPE_xxx */
    DWORD           fdwControl;         /* MIXERCONTROL_CONTROLF_xxx */
    DWORD           cMultipleItems;     /* if MIXERCONTROL_CONTROLF_MULTIPLE set */
    char            szShortName[MIXER_SHORT_NAME_CHARS];
    char            szName[MIXER_LONG_NAME_CHARS];
    union {
        struct {
            LONG    lMinimum;           /* signed minimum for this control */
            LONG    lMaximum;           /* signed maximum for this control */
        } DUMMYSTRUCTNAME;
        struct {
            DWORD   dwMinimum;          /* unsigned minimum for this control */
            DWORD   dwMaximum;          /* unsigned maximum for this control */
        } DUMMYSTRUCTNAME2;
        DWORD       dwReserved[6];
    } Bounds;
    union {
        DWORD       cSteps;             /* # of steps between min & max */
        DWORD       cbCustomData;       /* size in bytes of custom data */
        DWORD       dwReserved[6];      /* !!! needed? we have cbStruct.... */
    } Metrics;
} MIXERCONTROL, *PMIXERCONTROL, FAR *LPMIXERCONTROL;
#endif

/* */
/*  MIXERCONTROL.fdwControl */
/* */
/* */
#define MIXERCONTROL_CONTROLF_UNIFORM   0x00000001L
#define MIXERCONTROL_CONTROLF_MULTIPLE  0x00000002L
#define MIXERCONTROL_CONTROLF_DISABLED  0x80000000L

/* */
/*  MIXERCONTROL_CONTROLTYPE_xxx building block defines */
/* */
/* */
#define MIXERCONTROL_CT_CLASS_MASK          0xF0000000L
#define MIXERCONTROL_CT_CLASS_CUSTOM        0x00000000L
#define MIXERCONTROL_CT_CLASS_METER         0x10000000L
#define MIXERCONTROL_CT_CLASS_SWITCH        0x20000000L
#define MIXERCONTROL_CT_CLASS_NUMBER        0x30000000L
#define MIXERCONTROL_CT_CLASS_SLIDER        0x40000000L
#define MIXERCONTROL_CT_CLASS_FADER         0x50000000L
#define MIXERCONTROL_CT_CLASS_TIME          0x60000000L
#define MIXERCONTROL_CT_CLASS_LIST          0x70000000L

#define MIXERCONTROL_CT_SUBCLASS_MASK       0x0F000000L

#define MIXERCONTROL_CT_SC_SWITCH_BOOLEAN   0x00000000L
#define MIXERCONTROL_CT_SC_SWITCH_BUTTON    0x01000000L

#define MIXERCONTROL_CT_SC_METER_POLLED     0x00000000L

#define MIXERCONTROL_CT_SC_TIME_MICROSECS   0x00000000L
#define MIXERCONTROL_CT_SC_TIME_MILLISECS   0x01000000L

#define MIXERCONTROL_CT_SC_LIST_SINGLE      0x00000000L
#define MIXERCONTROL_CT_SC_LIST_MULTIPLE    0x01000000L

#define MIXERCONTROL_CT_UNITS_MASK          0x00FF0000L
#define MIXERCONTROL_CT_UNITS_CUSTOM        0x00000000L
#define MIXERCONTROL_CT_UNITS_BOOLEAN       0x00010000L
#define MIXERCONTROL_CT_UNITS_SIGNED        0x00020000L
#define MIXERCONTROL_CT_UNITS_UNSIGNED      0x00030000L
#define MIXERCONTROL_CT_UNITS_DECIBELS      0x00040000L /* in 10ths */
#define MIXERCONTROL_CT_UNITS_PERCENT       0x00050000L /* in 10ths */

/* */
/*  Commonly used control types for specifying MIXERCONTROL.dwControlType */
/* */

#define MIXERCONTROL_CONTROLTYPE_CUSTOM         (MIXERCONTROL_CT_CLASS_CUSTOM | MIXERCONTROL_CT_UNITS_CUSTOM)
#define MIXERCONTROL_CONTROLTYPE_BOOLEANMETER   (MIXERCONTROL_CT_CLASS_METER | MIXERCONTROL_CT_SC_METER_POLLED | MIXERCONTROL_CT_UNITS_BOOLEAN)
#define MIXERCONTROL_CONTROLTYPE_SIGNEDMETER    (MIXERCONTROL_CT_CLASS_METER | MIXERCONTROL_CT_SC_METER_POLLED | MIXERCONTROL_CT_UNITS_SIGNED)
#define MIXERCONTROL_CONTROLTYPE_PEAKMETER      (MIXERCONTROL_CONTROLTYPE_SIGNEDMETER + 1)
#define MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER  (MIXERCONTROL_CT_CLASS_METER | MIXERCONTROL_CT_SC_METER_POLLED | MIXERCONTROL_CT_UNITS_UNSIGNED)
#define MIXERCONTROL_CONTROLTYPE_BOOLEAN        (MIXERCONTROL_CT_CLASS_SWITCH | MIXERCONTROL_CT_SC_SWITCH_BOOLEAN | MIXERCONTROL_CT_UNITS_BOOLEAN)
#define MIXERCONTROL_CONTROLTYPE_ONOFF          (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 1)
#define MIXERCONTROL_CONTROLTYPE_MUTE           (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 2)
#define MIXERCONTROL_CONTROLTYPE_MONO           (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 3)
#define MIXERCONTROL_CONTROLTYPE_LOUDNESS       (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 4)
#define MIXERCONTROL_CONTROLTYPE_STEREOENH      (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 5)
#define MIXERCONTROL_CONTROLTYPE_BASS_BOOST     (MIXERCONTROL_CONTROLTYPE_BOOLEAN + 0x00002277)
#define MIXERCONTROL_CONTROLTYPE_BUTTON         (MIXERCONTROL_CT_CLASS_SWITCH | MIXERCONTROL_CT_SC_SWITCH_BUTTON | MIXERCONTROL_CT_UNITS_BOOLEAN)
#define MIXERCONTROL_CONTROLTYPE_DECIBELS       (MIXERCONTROL_CT_CLASS_NUMBER | MIXERCONTROL_CT_UNITS_DECIBELS)
#define MIXERCONTROL_CONTROLTYPE_SIGNED         (MIXERCONTROL_CT_CLASS_NUMBER | MIXERCONTROL_CT_UNITS_SIGNED)
#define MIXERCONTROL_CONTROLTYPE_UNSIGNED       (MIXERCONTROL_CT_CLASS_NUMBER | MIXERCONTROL_CT_UNITS_UNSIGNED)
#define MIXERCONTROL_CONTROLTYPE_PERCENT        (MIXERCONTROL_CT_CLASS_NUMBER | MIXERCONTROL_CT_UNITS_PERCENT)
#define MIXERCONTROL_CONTROLTYPE_SLIDER         (MIXERCONTROL_CT_CLASS_SLIDER | MIXERCONTROL_CT_UNITS_SIGNED)
#define MIXERCONTROL_CONTROLTYPE_PAN            (MIXERCONTROL_CONTROLTYPE_SLIDER + 1)
#define MIXERCONTROL_CONTROLTYPE_QSOUNDPAN      (MIXERCONTROL_CONTROLTYPE_SLIDER + 2)
#define MIXERCONTROL_CONTROLTYPE_FADER          (MIXERCONTROL_CT_CLASS_FADER | MIXERCONTROL_CT_UNITS_UNSIGNED)
#define MIXERCONTROL_CONTROLTYPE_VOLUME         (MIXERCONTROL_CONTROLTYPE_FADER + 1)
#define MIXERCONTROL_CONTROLTYPE_BASS           (MIXERCONTROL_CONTROLTYPE_FADER + 2)
#define MIXERCONTROL_CONTROLTYPE_TREBLE         (MIXERCONTROL_CONTROLTYPE_FADER + 3)
#define MIXERCONTROL_CONTROLTYPE_EQUALIZER      (MIXERCONTROL_CONTROLTYPE_FADER + 4)
#define MIXERCONTROL_CONTROLTYPE_SINGLESELECT   (MIXERCONTROL_CT_CLASS_LIST | MIXERCONTROL_CT_SC_LIST_SINGLE | MIXERCONTROL_CT_UNITS_BOOLEAN)
#define MIXERCONTROL_CONTROLTYPE_MUX            (MIXERCONTROL_CONTROLTYPE_SINGLESELECT + 1)
#define MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT (MIXERCONTROL_CT_CLASS_LIST | MIXERCONTROL_CT_SC_LIST_MULTIPLE | MIXERCONTROL_CT_UNITS_BOOLEAN)
#define MIXERCONTROL_CONTROLTYPE_MIXER          (MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT + 1)
#define MIXERCONTROL_CONTROLTYPE_MICROTIME      (MIXERCONTROL_CT_CLASS_TIME | MIXERCONTROL_CT_SC_TIME_MICROSECS | MIXERCONTROL_CT_UNITS_UNSIGNED)
#define MIXERCONTROL_CONTROLTYPE_MILLITIME      (MIXERCONTROL_CT_CLASS_TIME | MIXERCONTROL_CT_SC_TIME_MILLISECS | MIXERCONTROL_CT_UNITS_UNSIGNED)

/* */
/*  MIXERLINECONTROLS */
/* */
#ifdef _WIN32

typedef struct tagMIXERLINECONTROLSA {
    DWORD           cbStruct;       /* size in bytes of MIXERLINECONTROLS */
    DWORD           dwLineID;       /* line id (from MIXERLINE.dwLineID) */
    union {
        DWORD       dwControlID;    /* MIXER_GETLINECONTROLSF_ONEBYID */
        DWORD       dwControlType;  /* MIXER_GETLINECONTROLSF_ONEBYTYPE */
    } DUMMYUNIONNAME;
    DWORD           cControls;      /* count of controls pmxctrl points to */
    DWORD           cbmxctrl;       /* size in bytes of _one_ MIXERCONTROL */
    LPMIXERCONTROLA pamxctrl;       /* pointer to first MIXERCONTROL array */
} MIXERLINECONTROLSA, *PMIXERLINECONTROLSA, *LPMIXERLINECONTROLSA;
typedef struct tagMIXERLINECONTROLSW {
    DWORD           cbStruct;       /* size in bytes of MIXERLINECONTROLS */
    DWORD           dwLineID;       /* line id (from MIXERLINE.dwLineID) */
    union {
        DWORD       dwControlID;    /* MIXER_GETLINECONTROLSF_ONEBYID */
        DWORD       dwControlType;  /* MIXER_GETLINECONTROLSF_ONEBYTYPE */
    } DUMMYUNIONNAME;
    DWORD           cControls;      /* count of controls pmxctrl points to */
    DWORD           cbmxctrl;       /* size in bytes of _one_ MIXERCONTROL */
    LPMIXERCONTROLW pamxctrl;       /* pointer to first MIXERCONTROL array */
} MIXERLINECONTROLSW, *PMIXERLINECONTROLSW, *LPMIXERLINECONTROLSW;
#ifdef UNICODE
typedef MIXERLINECONTROLSW MIXERLINECONTROLS;
typedef PMIXERLINECONTROLSW PMIXERLINECONTROLS;
typedef LPMIXERLINECONTROLSW LPMIXERLINECONTROLS;
#else
typedef MIXERLINECONTROLSA MIXERLINECONTROLS;
typedef PMIXERLINECONTROLSA PMIXERLINECONTROLS;
typedef LPMIXERLINECONTROLSA LPMIXERLINECONTROLS;
#endif // UNICODE

#else
typedef struct tMIXERLINECONTROLS {
    DWORD           cbStruct;       /* size in bytes of MIXERLINECONTROLS */
    DWORD           dwLineID;       /* line id (from MIXERLINE.dwLineID) */
    union {
        DWORD       dwControlID;    /* MIXER_GETLINECONTROLSF_ONEBYID */
        DWORD       dwControlType;  /* MIXER_GETLINECONTROLSF_ONEBYTYPE */
    };
    DWORD           cControls;      /* count of controls pmxctrl points to */
    DWORD           cbmxctrl;       /* size in bytes of _one_ MIXERCONTROL */
    LPMIXERCONTROL  pamxctrl;       /* pointer to first MIXERCONTROL array */
} MIXERLINECONTROLS, *PMIXERLINECONTROLS, FAR *LPMIXERLINECONTROLS;
#endif

/* */
/* */
/* */
#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
mixerGetLineControlsA(
    _In_opt_ HMIXEROBJ hmxobj,
    _Inout_ LPMIXERLINECONTROLSA pmxlc,
    _In_ DWORD fdwControls
    );

WINMMAPI
MMRESULT
WINAPI
mixerGetLineControlsW(
    _In_opt_ HMIXEROBJ hmxobj,
    _Inout_ LPMIXERLINECONTROLSW pmxlc,
    _In_ DWORD fdwControls
    );

#ifdef UNICODE
#define mixerGetLineControls  mixerGetLineControlsW
#else
#define mixerGetLineControls  mixerGetLineControlsA
#endif // !UNICODE

#else
MMRESULT WINAPI mixerGetLineControls(HMIXEROBJ hmxobj, LPMIXERLINECONTROLS pmxlc, DWORD fdwControls);
#endif

#define MIXER_GETLINECONTROLSF_ALL          0x00000000L
#define MIXER_GETLINECONTROLSF_ONEBYID      0x00000001L
#define MIXER_GETLINECONTROLSF_ONEBYTYPE    0x00000002L

#define MIXER_GETLINECONTROLSF_QUERYMASK    0x0000000FL

typedef struct tMIXERCONTROLDETAILS {
    DWORD           cbStruct;       /* size in bytes of MIXERCONTROLDETAILS */
    DWORD           dwControlID;    /* control id to get/set details on */
    DWORD           cChannels;      /* number of channels in paDetails array */
    union {
        HWND        hwndOwner;      /* for MIXER_SETCONTROLDETAILSF_CUSTOM */
        DWORD       cMultipleItems; /* if _MULTIPLE, the number of items per channel */
    } DUMMYUNIONNAME;
    DWORD           cbDetails;      /* size of _one_ details_XX struct */
    LPVOID          paDetails;      /* pointer to array of details_XX structs */
} MIXERCONTROLDETAILS, *PMIXERCONTROLDETAILS, FAR *LPMIXERCONTROLDETAILS;

/* */
/*  MIXER_GETCONTROLDETAILSF_LISTTEXT */
/* */
/* */
#ifdef _WIN32

typedef struct tagMIXERCONTROLDETAILS_LISTTEXTA {
    DWORD           dwParam1;
    DWORD           dwParam2;
    CHAR            szName[MIXER_LONG_NAME_CHARS];
} MIXERCONTROLDETAILS_LISTTEXTA, *PMIXERCONTROLDETAILS_LISTTEXTA, *LPMIXERCONTROLDETAILS_LISTTEXTA;
typedef struct tagMIXERCONTROLDETAILS_LISTTEXTW {
    DWORD           dwParam1;
    DWORD           dwParam2;
    WCHAR           szName[MIXER_LONG_NAME_CHARS];
} MIXERCONTROLDETAILS_LISTTEXTW, *PMIXERCONTROLDETAILS_LISTTEXTW, *LPMIXERCONTROLDETAILS_LISTTEXTW;
#ifdef UNICODE
typedef MIXERCONTROLDETAILS_LISTTEXTW MIXERCONTROLDETAILS_LISTTEXT;
typedef PMIXERCONTROLDETAILS_LISTTEXTW PMIXERCONTROLDETAILS_LISTTEXT;
typedef LPMIXERCONTROLDETAILS_LISTTEXTW LPMIXERCONTROLDETAILS_LISTTEXT;
#else
typedef MIXERCONTROLDETAILS_LISTTEXTA MIXERCONTROLDETAILS_LISTTEXT;
typedef PMIXERCONTROLDETAILS_LISTTEXTA PMIXERCONTROLDETAILS_LISTTEXT;
typedef LPMIXERCONTROLDETAILS_LISTTEXTA LPMIXERCONTROLDETAILS_LISTTEXT;
#endif // UNICODE

#else
typedef struct tMIXERCONTROLDETAILS_LISTTEXT {
    DWORD           dwParam1;
    DWORD           dwParam2;
    char            szName[MIXER_LONG_NAME_CHARS];
} MIXERCONTROLDETAILS_LISTTEXT, *PMIXERCONTROLDETAILS_LISTTEXT, FAR *LPMIXERCONTROLDETAILS_LISTTEXT;
#endif

/* */
/*  MIXER_GETCONTROLDETAILSF_VALUE */
/* */
/* */
typedef struct tMIXERCONTROLDETAILS_BOOLEAN {
    LONG            fValue;
}       MIXERCONTROLDETAILS_BOOLEAN,
      *PMIXERCONTROLDETAILS_BOOLEAN,
 FAR *LPMIXERCONTROLDETAILS_BOOLEAN;

typedef struct tMIXERCONTROLDETAILS_SIGNED {
    LONG            lValue;
}       MIXERCONTROLDETAILS_SIGNED,
      *PMIXERCONTROLDETAILS_SIGNED,
 FAR *LPMIXERCONTROLDETAILS_SIGNED;

typedef struct tMIXERCONTROLDETAILS_UNSIGNED {
    DWORD           dwValue;
}       MIXERCONTROLDETAILS_UNSIGNED,
      *PMIXERCONTROLDETAILS_UNSIGNED,
 FAR *LPMIXERCONTROLDETAILS_UNSIGNED;

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
mixerGetControlDetailsA(
    _In_opt_ HMIXEROBJ hmxobj,
    _Inout_ LPMIXERCONTROLDETAILS pmxcd,
    _In_ DWORD fdwDetails
    );

WINMMAPI
MMRESULT
WINAPI
mixerGetControlDetailsW(
    _In_opt_ HMIXEROBJ hmxobj,
    _Inout_ LPMIXERCONTROLDETAILS pmxcd,
    _In_ DWORD fdwDetails
    );

#ifdef UNICODE
#define mixerGetControlDetails  mixerGetControlDetailsW
#else
#define mixerGetControlDetails  mixerGetControlDetailsA
#endif // !UNICODE

#else
MMRESULT WINAPI mixerGetControlDetails(HMIXEROBJ hmxobj, LPMIXERCONTROLDETAILS pmxcd, DWORD fdwDetails);
#endif

#define MIXER_GETCONTROLDETAILSF_VALUE      0x00000000L
#define MIXER_GETCONTROLDETAILSF_LISTTEXT   0x00000001L

#define MIXER_GETCONTROLDETAILSF_QUERYMASK  0x0000000FL

WINMMAPI
MMRESULT
WINAPI
mixerSetControlDetails(
    _In_opt_ HMIXEROBJ hmxobj,
    _In_ LPMIXERCONTROLDETAILS pmxcd,
    _In_ DWORD fdwDetails
    );

#define MIXER_SETCONTROLDETAILSF_VALUE      0x00000000L
#define MIXER_SETCONTROLDETAILSF_CUSTOM     0x00000001L

#define MIXER_SETCONTROLDETAILSF_QUERYMASK  0x0000000FL

#endif /* ifndef MMNOMIXER */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _MMEAPI_H_

