/********************************************************************************
*                                                                               *
* mciapi.h -- ApiSet Contract for api-ms-win-mm-mci-l1-1-0                      *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif

#ifndef _MCIAPI_H_
#define _MCIAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#include <mmsyscom.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/****************************************************************************

                            MCI support

****************************************************************************/

#ifndef _MCIERROR_              /* MCIERROR is defined in some post 3.1 apps */
#define _MCIERROR_
typedef DWORD   MCIERROR;       /* error return code, 0 means no error */
#endif

#ifndef _MCIDEVICEID_           /* Same with MCIDEVICEID */
#define _MCIDEVICEID_
typedef UINT    MCIDEVICEID;    /* MCI device ID type */
#endif

typedef UINT (CALLBACK *YIELDPROC)(MCIDEVICEID mciId, DWORD dwYieldData);

/* MCI function prototypes */
#ifdef _WIN32

WINMMAPI
MCIERROR
WINAPI
mciSendCommandA(
    _In_ MCIDEVICEID mciId,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dwParam1,
    _In_opt_ DWORD_PTR dwParam2
    );

WINMMAPI
MCIERROR
WINAPI
mciSendCommandW(
    _In_ MCIDEVICEID mciId,
    _In_ UINT uMsg,
    _In_opt_ DWORD_PTR dwParam1,
    _In_opt_ DWORD_PTR dwParam2
    );

#ifdef UNICODE
#define mciSendCommand  mciSendCommandW
#else
#define mciSendCommand  mciSendCommandA
#endif // !UNICODE
WINMMAPI
MCIERROR
WINAPI
mciSendStringA(
    _In_ LPCSTR lpstrCommand,
    _Out_writes_opt_(uReturnLength) LPSTR lpstrReturnString,
    _In_ UINT uReturnLength,
    _In_opt_ HWND hwndCallback
    );

WINMMAPI
MCIERROR
WINAPI
mciSendStringW(
    _In_ LPCWSTR lpstrCommand,
    _Out_writes_opt_(uReturnLength) LPWSTR lpstrReturnString,
    _In_ UINT uReturnLength,
    _In_opt_ HWND hwndCallback
    );

#ifdef UNICODE
#define mciSendString  mciSendStringW
#else
#define mciSendString  mciSendStringA
#endif // !UNICODE
WINMMAPI
MCIDEVICEID
WINAPI
mciGetDeviceIDA(
    _In_ LPCSTR pszDevice
    );

WINMMAPI
MCIDEVICEID
WINAPI
mciGetDeviceIDW(
    _In_ LPCWSTR pszDevice
    );

#ifdef UNICODE
#define mciGetDeviceID  mciGetDeviceIDW
#else
#define mciGetDeviceID  mciGetDeviceIDA
#endif // !UNICODE
WINMMAPI
MCIDEVICEID
WINAPI
mciGetDeviceIDFromElementIDA(
    _In_ DWORD dwElementID,
    _In_ LPCSTR lpstrType
    );

WINMMAPI
MCIDEVICEID
WINAPI
mciGetDeviceIDFromElementIDW(
    _In_ DWORD dwElementID,
    _In_ LPCWSTR lpstrType
    );

#ifdef UNICODE
#define mciGetDeviceIDFromElementID  mciGetDeviceIDFromElementIDW
#else
#define mciGetDeviceIDFromElementID  mciGetDeviceIDFromElementIDA
#endif // !UNICODE
WINMMAPI
BOOL
WINAPI
mciGetErrorStringA(
    _In_ MCIERROR mcierr,
    _Out_writes_(cchText) LPSTR pszText,
    _In_ UINT cchText
    );

WINMMAPI
BOOL
WINAPI
mciGetErrorStringW(
    _In_ MCIERROR mcierr,
    _Out_writes_(cchText) LPWSTR pszText,
    _In_ UINT cchText
    );

#ifdef UNICODE
#define mciGetErrorString  mciGetErrorStringW
#else
#define mciGetErrorString  mciGetErrorStringA
#endif // !UNICODE

#else
MCIERROR WINAPI mciSendCommand(MCIDEVICEID mciId, UINT uMsg, DWORD dwParam1, DWORD dwParam2);
MCIERROR  WINAPI mciSendString(LPCSTR lpstrCommand, LPSTR lpstrReturnString, UINT uReturnLength, HWND hwndCallback);
MCIDEVICEID WINAPI mciGetDeviceID(LPCSTR pszDevice);
BOOL WINAPI mciGetErrorString(MCIERROR mcierr, LPSTR pszText, UINT cchText);
#endif

WINMMAPI
BOOL
WINAPI
mciSetYieldProc(
    _In_ MCIDEVICEID mciId,
    _In_opt_ YIELDPROC fpYieldProc,
    _In_ DWORD dwYieldData
    );

#if (WINVER >= 0x030a)
WINMMAPI
HTASK
WINAPI
mciGetCreatorTask(
    _In_ MCIDEVICEID mciId
    );

WINMMAPI
YIELDPROC
WINAPI
mciGetYieldProc(
    _In_ MCIDEVICEID mciId,
    _In_ LPDWORD pdwYieldData
    );

#endif /* ifdef WINVER >= 0x030a */

#if (WINVER < 0x030a)
WINMMAPI
BOOL
WINAPI
mciExecute(
    LPCSTR pszCommand
    );

#endif /* ifdef WINVER < 0x030a */

/* MCI error return values */
#define MCIERR_INVALID_DEVICE_ID        (MCIERR_BASE + 1)
#define MCIERR_UNRECOGNIZED_KEYWORD     (MCIERR_BASE + 3)
#define MCIERR_UNRECOGNIZED_COMMAND     (MCIERR_BASE + 5)
#define MCIERR_HARDWARE                 (MCIERR_BASE + 6)
#define MCIERR_INVALID_DEVICE_NAME      (MCIERR_BASE + 7)
#define MCIERR_OUT_OF_MEMORY            (MCIERR_BASE + 8)
#define MCIERR_DEVICE_OPEN              (MCIERR_BASE + 9)
#define MCIERR_CANNOT_LOAD_DRIVER       (MCIERR_BASE + 10)
#define MCIERR_MISSING_COMMAND_STRING   (MCIERR_BASE + 11)
#define MCIERR_PARAM_OVERFLOW           (MCIERR_BASE + 12)
#define MCIERR_MISSING_STRING_ARGUMENT  (MCIERR_BASE + 13)
#define MCIERR_BAD_INTEGER              (MCIERR_BASE + 14)
#define MCIERR_PARSER_INTERNAL          (MCIERR_BASE + 15)
#define MCIERR_DRIVER_INTERNAL          (MCIERR_BASE + 16)
#define MCIERR_MISSING_PARAMETER        (MCIERR_BASE + 17)
#define MCIERR_UNSUPPORTED_FUNCTION     (MCIERR_BASE + 18)
#define MCIERR_FILE_NOT_FOUND           (MCIERR_BASE + 19)
#define MCIERR_DEVICE_NOT_READY         (MCIERR_BASE + 20)
#define MCIERR_INTERNAL                 (MCIERR_BASE + 21)
#define MCIERR_DRIVER                   (MCIERR_BASE + 22)
#define MCIERR_CANNOT_USE_ALL           (MCIERR_BASE + 23)
#define MCIERR_MULTIPLE                 (MCIERR_BASE + 24)
#define MCIERR_EXTENSION_NOT_FOUND      (MCIERR_BASE + 25)
#define MCIERR_OUTOFRANGE               (MCIERR_BASE + 26)
#define MCIERR_FLAGS_NOT_COMPATIBLE     (MCIERR_BASE + 28)
#define MCIERR_FILE_NOT_SAVED           (MCIERR_BASE + 30)
#define MCIERR_DEVICE_TYPE_REQUIRED     (MCIERR_BASE + 31)
#define MCIERR_DEVICE_LOCKED            (MCIERR_BASE + 32)
#define MCIERR_DUPLICATE_ALIAS          (MCIERR_BASE + 33)
#define MCIERR_BAD_CONSTANT             (MCIERR_BASE + 34)
#define MCIERR_MUST_USE_SHAREABLE       (MCIERR_BASE + 35)
#define MCIERR_MISSING_DEVICE_NAME      (MCIERR_BASE + 36)
#define MCIERR_BAD_TIME_FORMAT          (MCIERR_BASE + 37)
#define MCIERR_NO_CLOSING_QUOTE         (MCIERR_BASE + 38)
#define MCIERR_DUPLICATE_FLAGS          (MCIERR_BASE + 39)
#define MCIERR_INVALID_FILE             (MCIERR_BASE + 40)
#define MCIERR_NULL_PARAMETER_BLOCK     (MCIERR_BASE + 41)
#define MCIERR_UNNAMED_RESOURCE         (MCIERR_BASE + 42)
#define MCIERR_NEW_REQUIRES_ALIAS       (MCIERR_BASE + 43)
#define MCIERR_NOTIFY_ON_AUTO_OPEN      (MCIERR_BASE + 44)
#define MCIERR_NO_ELEMENT_ALLOWED       (MCIERR_BASE + 45)
#define MCIERR_NONAPPLICABLE_FUNCTION   (MCIERR_BASE + 46)
#define MCIERR_ILLEGAL_FOR_AUTO_OPEN    (MCIERR_BASE + 47)
#define MCIERR_FILENAME_REQUIRED        (MCIERR_BASE + 48)
#define MCIERR_EXTRA_CHARACTERS         (MCIERR_BASE + 49)
#define MCIERR_DEVICE_NOT_INSTALLED     (MCIERR_BASE + 50)
#define MCIERR_GET_CD                   (MCIERR_BASE + 51)
#define MCIERR_SET_CD                   (MCIERR_BASE + 52)
#define MCIERR_SET_DRIVE                (MCIERR_BASE + 53)
#define MCIERR_DEVICE_LENGTH            (MCIERR_BASE + 54)
#define MCIERR_DEVICE_ORD_LENGTH        (MCIERR_BASE + 55)
#define MCIERR_NO_INTEGER               (MCIERR_BASE + 56)

#define MCIERR_WAVE_OUTPUTSINUSE        (MCIERR_BASE + 64)
#define MCIERR_WAVE_SETOUTPUTINUSE      (MCIERR_BASE + 65)
#define MCIERR_WAVE_INPUTSINUSE         (MCIERR_BASE + 66)
#define MCIERR_WAVE_SETINPUTINUSE       (MCIERR_BASE + 67)
#define MCIERR_WAVE_OUTPUTUNSPECIFIED   (MCIERR_BASE + 68)
#define MCIERR_WAVE_INPUTUNSPECIFIED    (MCIERR_BASE + 69)
#define MCIERR_WAVE_OUTPUTSUNSUITABLE   (MCIERR_BASE + 70)
#define MCIERR_WAVE_SETOUTPUTUNSUITABLE (MCIERR_BASE + 71)
#define MCIERR_WAVE_INPUTSUNSUITABLE    (MCIERR_BASE + 72)
#define MCIERR_WAVE_SETINPUTUNSUITABLE  (MCIERR_BASE + 73)

#define MCIERR_SEQ_DIV_INCOMPATIBLE     (MCIERR_BASE + 80)
#define MCIERR_SEQ_PORT_INUSE           (MCIERR_BASE + 81)
#define MCIERR_SEQ_PORT_NONEXISTENT     (MCIERR_BASE + 82)
#define MCIERR_SEQ_PORT_MAPNODEVICE     (MCIERR_BASE + 83)
#define MCIERR_SEQ_PORT_MISCERROR       (MCIERR_BASE + 84)
#define MCIERR_SEQ_TIMER                (MCIERR_BASE + 85)
#define MCIERR_SEQ_PORTUNSPECIFIED      (MCIERR_BASE + 86)
#define MCIERR_SEQ_NOMIDIPRESENT        (MCIERR_BASE + 87)

#define MCIERR_NO_WINDOW                (MCIERR_BASE + 90)
#define MCIERR_CREATEWINDOW             (MCIERR_BASE + 91)
#define MCIERR_FILE_READ                (MCIERR_BASE + 92)
#define MCIERR_FILE_WRITE               (MCIERR_BASE + 93)

#define MCIERR_NO_IDENTITY              (MCIERR_BASE + 94)

/* all custom device driver errors must be >= than this value */
#define MCIERR_CUSTOM_DRIVER_BASE       (MCIERR_BASE + 256)

#define MCI_FIRST                       DRV_MCI_FIRST   /* 0x0800 */
/* MCI command message identifiers */
#define MCI_OPEN                        0x0803
#define MCI_CLOSE                       0x0804
#define MCI_ESCAPE                      0x0805
#define MCI_PLAY                        0x0806
#define MCI_SEEK                        0x0807
#define MCI_STOP                        0x0808
#define MCI_PAUSE                       0x0809
#define MCI_INFO                        0x080A
#define MCI_GETDEVCAPS                  0x080B
#define MCI_SPIN                        0x080C
#define MCI_SET                         0x080D
#define MCI_STEP                        0x080E
#define MCI_RECORD                      0x080F
#define MCI_SYSINFO                     0x0810
#define MCI_BREAK                       0x0811
#define MCI_SAVE                        0x0813
#define MCI_STATUS                      0x0814
#define MCI_CUE                         0x0830
#define MCI_REALIZE                     0x0840
#define MCI_WINDOW                      0x0841
#define MCI_PUT                         0x0842
#define MCI_WHERE                       0x0843
#define MCI_FREEZE                      0x0844
#define MCI_UNFREEZE                    0x0845
#define MCI_LOAD                        0x0850
#define MCI_CUT                         0x0851
#define MCI_COPY                        0x0852
#define MCI_PASTE                       0x0853
#define MCI_UPDATE                      0x0854
#define MCI_RESUME                      0x0855
#define MCI_DELETE                      0x0856

/* all custom MCI command messages must be >= than this value */
#define MCI_USER_MESSAGES               (DRV_MCI_FIRST + 0x400)
#define MCI_LAST                        0x0FFF

/* device ID for "all devices" */
#define MCI_ALL_DEVICE_ID               ((MCIDEVICEID)-1)

/* constants for predefined MCI device types */
#define MCI_DEVTYPE_VCR                 513 /* (MCI_STRING_OFFSET + 1) */
#define MCI_DEVTYPE_VIDEODISC           514 /* (MCI_STRING_OFFSET + 2) */
#define MCI_DEVTYPE_OVERLAY             515 /* (MCI_STRING_OFFSET + 3) */
#define MCI_DEVTYPE_CD_AUDIO            516 /* (MCI_STRING_OFFSET + 4) */
#define MCI_DEVTYPE_DAT                 517 /* (MCI_STRING_OFFSET + 5) */
#define MCI_DEVTYPE_SCANNER             518 /* (MCI_STRING_OFFSET + 6) */
#define MCI_DEVTYPE_ANIMATION           519 /* (MCI_STRING_OFFSET + 7) */
#define MCI_DEVTYPE_DIGITAL_VIDEO       520 /* (MCI_STRING_OFFSET + 8) */
#define MCI_DEVTYPE_OTHER               521 /* (MCI_STRING_OFFSET + 9) */
#define MCI_DEVTYPE_WAVEFORM_AUDIO      522 /* (MCI_STRING_OFFSET + 10) */
#define MCI_DEVTYPE_SEQUENCER           523 /* (MCI_STRING_OFFSET + 11) */

#define MCI_DEVTYPE_FIRST               MCI_DEVTYPE_VCR
#define MCI_DEVTYPE_LAST                MCI_DEVTYPE_SEQUENCER

#define MCI_DEVTYPE_FIRST_USER          0x1000
/* return values for 'status mode' command */
#define MCI_MODE_NOT_READY              (MCI_STRING_OFFSET + 12)
#define MCI_MODE_STOP                   (MCI_STRING_OFFSET + 13)
#define MCI_MODE_PLAY                   (MCI_STRING_OFFSET + 14)
#define MCI_MODE_RECORD                 (MCI_STRING_OFFSET + 15)
#define MCI_MODE_SEEK                   (MCI_STRING_OFFSET + 16)
#define MCI_MODE_PAUSE                  (MCI_STRING_OFFSET + 17)
#define MCI_MODE_OPEN                   (MCI_STRING_OFFSET + 18)

/* constants used in 'set time format' and 'status time format' commands */
#define MCI_FORMAT_MILLISECONDS         0
#define MCI_FORMAT_HMS                  1
#define MCI_FORMAT_MSF                  2
#define MCI_FORMAT_FRAMES               3
#define MCI_FORMAT_SMPTE_24             4
#define MCI_FORMAT_SMPTE_25             5
#define MCI_FORMAT_SMPTE_30             6
#define MCI_FORMAT_SMPTE_30DROP         7
#define MCI_FORMAT_BYTES                8
#define MCI_FORMAT_SAMPLES              9
#define MCI_FORMAT_TMSF                 10

/* MCI time format conversion macros */
#define MCI_MSF_MINUTE(msf)             ((BYTE)(msf))
#define MCI_MSF_SECOND(msf)             ((BYTE)(((WORD)(msf)) >> 8))
#define MCI_MSF_FRAME(msf)              ((BYTE)((msf)>>16))

#define MCI_MAKE_MSF(m, s, f)           ((DWORD)(((BYTE)(m) | \
                                                  ((WORD)(s)<<8)) | \
                                                 (((DWORD)(BYTE)(f))<<16)))

#define MCI_TMSF_TRACK(tmsf)            ((BYTE)(tmsf))
#define MCI_TMSF_MINUTE(tmsf)           ((BYTE)(((WORD)(tmsf)) >> 8))
#define MCI_TMSF_SECOND(tmsf)           ((BYTE)((tmsf)>>16))
#define MCI_TMSF_FRAME(tmsf)            ((BYTE)((tmsf)>>24))

#define MCI_MAKE_TMSF(t, m, s, f)       ((DWORD)(((BYTE)(t) | \
                                                  ((WORD)(m)<<8)) | \
                                                 (((DWORD)(BYTE)(s) | \
                                                   ((WORD)(f)<<8))<<16)))

#define MCI_HMS_HOUR(hms)               ((BYTE)(hms))
#define MCI_HMS_MINUTE(hms)             ((BYTE)(((WORD)(hms)) >> 8))
#define MCI_HMS_SECOND(hms)             ((BYTE)((hms)>>16))

#define MCI_MAKE_HMS(h, m, s)           ((DWORD)(((BYTE)(h) | \
                                                  ((WORD)(m)<<8)) | \
                                                 (((DWORD)(BYTE)(s))<<16)))

/* flags for wParam of MM_MCINOTIFY message */
#define MCI_NOTIFY_SUCCESSFUL           0x0001
#define MCI_NOTIFY_SUPERSEDED           0x0002
#define MCI_NOTIFY_ABORTED              0x0004
#define MCI_NOTIFY_FAILURE              0x0008

/* common flags for dwFlags parameter of MCI command messages */
#define MCI_NOTIFY                      0x00000001L
#define MCI_WAIT                        0x00000002L
#define MCI_FROM                        0x00000004L
#define MCI_TO                          0x00000008L
#define MCI_TRACK                       0x00000010L

/* flags for dwFlags parameter of MCI_OPEN command message */
#define MCI_OPEN_SHAREABLE              0x00000100L
#define MCI_OPEN_ELEMENT                0x00000200L
#define MCI_OPEN_ALIAS                  0x00000400L
#define MCI_OPEN_ELEMENT_ID             0x00000800L
#define MCI_OPEN_TYPE_ID                0x00001000L
#define MCI_OPEN_TYPE                   0x00002000L

/* flags for dwFlags parameter of MCI_SEEK command message */
#define MCI_SEEK_TO_START               0x00000100L
#define MCI_SEEK_TO_END                 0x00000200L

/* flags for dwFlags parameter of MCI_STATUS command message */
#define MCI_STATUS_ITEM                 0x00000100L
#define MCI_STATUS_START                0x00000200L

/* flags for dwItem field of the MCI_STATUS_PARMS parameter block */
#define MCI_STATUS_LENGTH               0x00000001L
#define MCI_STATUS_POSITION             0x00000002L
#define MCI_STATUS_NUMBER_OF_TRACKS     0x00000003L
#define MCI_STATUS_MODE                 0x00000004L
#define MCI_STATUS_MEDIA_PRESENT        0x00000005L
#define MCI_STATUS_TIME_FORMAT          0x00000006L
#define MCI_STATUS_READY                0x00000007L
#define MCI_STATUS_CURRENT_TRACK        0x00000008L

/* flags for dwFlags parameter of MCI_INFO command message */
#define MCI_INFO_PRODUCT                0x00000100L
#define MCI_INFO_FILE                   0x00000200L
#define MCI_INFO_MEDIA_UPC              0x00000400L
#define MCI_INFO_MEDIA_IDENTITY         0x00000800L
#define MCI_INFO_NAME                   0x00001000L
#define MCI_INFO_COPYRIGHT              0x00002000L

/* flags for dwFlags parameter of MCI_GETDEVCAPS command message */
#define MCI_GETDEVCAPS_ITEM             0x00000100L

/* flags for dwItem field of the MCI_GETDEVCAPS_PARMS parameter block */
#define MCI_GETDEVCAPS_CAN_RECORD       0x00000001L
#define MCI_GETDEVCAPS_HAS_AUDIO        0x00000002L
#define MCI_GETDEVCAPS_HAS_VIDEO        0x00000003L
#define MCI_GETDEVCAPS_DEVICE_TYPE      0x00000004L
#define MCI_GETDEVCAPS_USES_FILES       0x00000005L
#define MCI_GETDEVCAPS_COMPOUND_DEVICE  0x00000006L
#define MCI_GETDEVCAPS_CAN_EJECT        0x00000007L
#define MCI_GETDEVCAPS_CAN_PLAY         0x00000008L
#define MCI_GETDEVCAPS_CAN_SAVE         0x00000009L

/* flags for dwFlags parameter of MCI_SYSINFO command message */
#define MCI_SYSINFO_QUANTITY            0x00000100L
#define MCI_SYSINFO_OPEN                0x00000200L
#define MCI_SYSINFO_NAME                0x00000400L
#define MCI_SYSINFO_INSTALLNAME         0x00000800L

/* flags for dwFlags parameter of MCI_SET command message */
#define MCI_SET_DOOR_OPEN               0x00000100L
#define MCI_SET_DOOR_CLOSED             0x00000200L
#define MCI_SET_TIME_FORMAT             0x00000400L
#define MCI_SET_AUDIO                   0x00000800L
#define MCI_SET_VIDEO                   0x00001000L
#define MCI_SET_ON                      0x00002000L
#define MCI_SET_OFF                     0x00004000L

/* flags for dwAudio field of MCI_SET_PARMS or MCI_SEQ_SET_PARMS */
#define MCI_SET_AUDIO_ALL               0x00000000L
#define MCI_SET_AUDIO_LEFT              0x00000001L
#define MCI_SET_AUDIO_RIGHT             0x00000002L

/* flags for dwFlags parameter of MCI_BREAK command message */
#define MCI_BREAK_KEY                   0x00000100L
#define MCI_BREAK_HWND                  0x00000200L
#define MCI_BREAK_OFF                   0x00000400L

/* flags for dwFlags parameter of MCI_RECORD command message */
#define MCI_RECORD_INSERT               0x00000100L
#define MCI_RECORD_OVERWRITE            0x00000200L

/* flags for dwFlags parameter of MCI_SAVE command message */
#define MCI_SAVE_FILE                   0x00000100L

/* flags for dwFlags parameter of MCI_LOAD command message */
#define MCI_LOAD_FILE                   0x00000100L

/* generic parameter block for MCI command messages with no special parameters */
typedef struct tagMCI_GENERIC_PARMS {
    DWORD_PTR   dwCallback;
} MCI_GENERIC_PARMS, *PMCI_GENERIC_PARMS, FAR *LPMCI_GENERIC_PARMS;

/* parameter block for MCI_OPEN command message */
#ifdef _WIN32

typedef struct tagMCI_OPEN_PARMSA {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCSTR     lpstrDeviceType;
    LPCSTR     lpstrElementName;
    LPCSTR     lpstrAlias;
} MCI_OPEN_PARMSA, *PMCI_OPEN_PARMSA, *LPMCI_OPEN_PARMSA;
typedef struct tagMCI_OPEN_PARMSW {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCWSTR    lpstrDeviceType;
    LPCWSTR    lpstrElementName;
    LPCWSTR    lpstrAlias;
} MCI_OPEN_PARMSW, *PMCI_OPEN_PARMSW, *LPMCI_OPEN_PARMSW;
#ifdef UNICODE
typedef MCI_OPEN_PARMSW MCI_OPEN_PARMS;
typedef PMCI_OPEN_PARMSW PMCI_OPEN_PARMS;
typedef LPMCI_OPEN_PARMSW LPMCI_OPEN_PARMS;
#else
typedef MCI_OPEN_PARMSA MCI_OPEN_PARMS;
typedef PMCI_OPEN_PARMSA PMCI_OPEN_PARMS;
typedef LPMCI_OPEN_PARMSA LPMCI_OPEN_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_OPEN_PARMS {
    DWORD       dwCallback;
    MCIDEVICEID wDeviceID;
    WORD        wReserved0;
    LPCSTR      lpstrDeviceType;
    LPCSTR      lpstrElementName;
    LPCSTR      lpstrAlias;
} MCI_OPEN_PARMS, FAR *LPMCI_OPEN_PARMS;
#endif

/* parameter block for MCI_PLAY command message */
typedef struct tagMCI_PLAY_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrom;
    DWORD       dwTo;
} MCI_PLAY_PARMS, *PMCI_PLAY_PARMS, FAR *LPMCI_PLAY_PARMS;

/* parameter block for MCI_SEEK command message */
typedef struct tagMCI_SEEK_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwTo;
} MCI_SEEK_PARMS, *PMCI_SEEK_PARMS, FAR *LPMCI_SEEK_PARMS;

/* parameter block for MCI_STATUS command message */
typedef struct tagMCI_STATUS_PARMS {
    DWORD_PTR   dwCallback;
    DWORD_PTR   dwReturn;
    DWORD       dwItem;
    DWORD       dwTrack;
} MCI_STATUS_PARMS, *PMCI_STATUS_PARMS, FAR * LPMCI_STATUS_PARMS;

/* parameter block for MCI_INFO command message */
#ifdef _WIN32

typedef struct tagMCI_INFO_PARMSA {
    DWORD_PTR dwCallback;
    LPSTR     lpstrReturn;
    DWORD     dwRetSize;
} MCI_INFO_PARMSA, * LPMCI_INFO_PARMSA;
typedef struct tagMCI_INFO_PARMSW {
    DWORD_PTR dwCallback;
    LPWSTR    lpstrReturn;
    DWORD     dwRetSize;
} MCI_INFO_PARMSW, * LPMCI_INFO_PARMSW;
#ifdef UNICODE
typedef MCI_INFO_PARMSW MCI_INFO_PARMS;
typedef LPMCI_INFO_PARMSW LPMCI_INFO_PARMS;
#else
typedef MCI_INFO_PARMSA MCI_INFO_PARMS;
typedef LPMCI_INFO_PARMSA LPMCI_INFO_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_INFO_PARMS {
    DWORD   dwCallback;
    LPSTR   lpstrReturn;
    DWORD   dwRetSize;
} MCI_INFO_PARMS, FAR * LPMCI_INFO_PARMS;
#endif

/* parameter block for MCI_GETDEVCAPS command message */
typedef struct tagMCI_GETDEVCAPS_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwReturn;
    DWORD       dwItem;
} MCI_GETDEVCAPS_PARMS, *PMCI_GETDEVCAPS_PARMS, FAR * LPMCI_GETDEVCAPS_PARMS;

/* parameter block for MCI_SYSINFO command message */
#ifdef _WIN32

typedef struct tagMCI_SYSINFO_PARMSA {
    DWORD_PTR   dwCallback;
    LPSTR       lpstrReturn;
    DWORD       dwRetSize;
    DWORD       dwNumber;
    UINT        wDeviceType;
} MCI_SYSINFO_PARMSA, *PMCI_SYSINFO_PARMSA, * LPMCI_SYSINFO_PARMSA;
typedef struct tagMCI_SYSINFO_PARMSW {
    DWORD_PTR   dwCallback;
    LPWSTR      lpstrReturn;
    DWORD       dwRetSize;
    DWORD       dwNumber;
    UINT        wDeviceType;
} MCI_SYSINFO_PARMSW, *PMCI_SYSINFO_PARMSW, * LPMCI_SYSINFO_PARMSW;
#ifdef UNICODE
typedef MCI_SYSINFO_PARMSW MCI_SYSINFO_PARMS;
typedef PMCI_SYSINFO_PARMSW PMCI_SYSINFO_PARMS;
typedef LPMCI_SYSINFO_PARMSW LPMCI_SYSINFO_PARMS;
#else
typedef MCI_SYSINFO_PARMSA MCI_SYSINFO_PARMS;
typedef PMCI_SYSINFO_PARMSA PMCI_SYSINFO_PARMS;
typedef LPMCI_SYSINFO_PARMSA LPMCI_SYSINFO_PARMS;
#endif // UNICODE
#else
typedef struct tagMCI_SYSINFO_PARMS {
    DWORD   dwCallback;
    LPSTR   lpstrReturn;
    DWORD   dwRetSize;
    DWORD   dwNumber;
    WORD    wDeviceType;
    WORD    wReserved0;
} MCI_SYSINFO_PARMS, FAR * LPMCI_SYSINFO_PARMS;
#endif

/* parameter block for MCI_SET command message */
typedef struct tagMCI_SET_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwTimeFormat;
    DWORD       dwAudio;
} MCI_SET_PARMS, *PMCI_SET_PARMS, FAR *LPMCI_SET_PARMS;

/* parameter block for MCI_BREAK command message */
typedef struct tagMCI_BREAK_PARMS {
    DWORD_PTR   dwCallback;
#ifdef _WIN32
    int         nVirtKey;
    HWND        hwndBreak;
#else
    short       nVirtKey;
    WORD        wReserved0;             /* padding for Win 16 */
    HWND        hwndBreak;
    WORD        wReserved1;             /* padding for Win 16 */
#endif
} MCI_BREAK_PARMS, *PMCI_BREAK_PARMS, FAR * LPMCI_BREAK_PARMS;

/* parameter block for MCI_SAVE command message */
#ifdef _WIN32

typedef struct tagMCI_SAVE_PARMSA {
    DWORD_PTR    dwCallback;
    LPCSTR       lpfilename;
} MCI_SAVE_PARMSA, *PMCI_SAVE_PARMSA, * LPMCI_SAVE_PARMSA;
typedef struct tagMCI_SAVE_PARMSW {
    DWORD_PTR    dwCallback;
    LPCWSTR      lpfilename;
} MCI_SAVE_PARMSW, *PMCI_SAVE_PARMSW, * LPMCI_SAVE_PARMSW;
#ifdef UNICODE
typedef MCI_SAVE_PARMSW MCI_SAVE_PARMS;
typedef PMCI_SAVE_PARMSW PMCI_SAVE_PARMS;
typedef LPMCI_SAVE_PARMSW LPMCI_SAVE_PARMS;
#else
typedef MCI_SAVE_PARMSA MCI_SAVE_PARMS;
typedef PMCI_SAVE_PARMSA PMCI_SAVE_PARMS;
typedef LPMCI_SAVE_PARMSA LPMCI_SAVE_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_SAVE_PARMS {
    DWORD_PTR   dwCallback;
    LPCSTR      lpfilename;
} MCI_SAVE_PARMS, FAR * LPMCI_SAVE_PARMS;
#endif

/* parameter block for MCI_LOAD command message */
#ifdef _WIN32

typedef struct tagMCI_LOAD_PARMSA {
    DWORD_PTR    dwCallback;
    LPCSTR       lpfilename;
} MCI_LOAD_PARMSA, *PMCI_LOAD_PARMSA, * LPMCI_LOAD_PARMSA;
typedef struct tagMCI_LOAD_PARMSW {
    DWORD_PTR    dwCallback;
    LPCWSTR      lpfilename;
} MCI_LOAD_PARMSW, *PMCI_LOAD_PARMSW, * LPMCI_LOAD_PARMSW;
#ifdef UNICODE
typedef MCI_LOAD_PARMSW MCI_LOAD_PARMS;
typedef PMCI_LOAD_PARMSW PMCI_LOAD_PARMS;
typedef LPMCI_LOAD_PARMSW LPMCI_LOAD_PARMS;
#else
typedef MCI_LOAD_PARMSA MCI_LOAD_PARMS;
typedef PMCI_LOAD_PARMSA PMCI_LOAD_PARMS;
typedef LPMCI_LOAD_PARMSA LPMCI_LOAD_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_LOAD_PARMS {
    DWORD   dwCallback;
    LPCSTR  lpfilename;
} MCI_LOAD_PARMS, FAR * LPMCI_LOAD_PARMS;
#endif

/* parameter block for MCI_RECORD command message */
typedef struct tagMCI_RECORD_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrom;
    DWORD       dwTo;
} MCI_RECORD_PARMS, FAR *LPMCI_RECORD_PARMS;

/* MCI extensions for videodisc devices */

/* flag for dwReturn field of MCI_STATUS_PARMS */
/* MCI_STATUS command, (dwItem == MCI_STATUS_MODE) */
#define MCI_VD_MODE_PARK                (MCI_VD_OFFSET + 1)

/* flag for dwReturn field of MCI_STATUS_PARMS */
/* MCI_STATUS command, (dwItem == MCI_VD_STATUS_MEDIA_TYPE) */
#define MCI_VD_MEDIA_CLV                (MCI_VD_OFFSET + 2)
#define MCI_VD_MEDIA_CAV                (MCI_VD_OFFSET + 3)
#define MCI_VD_MEDIA_OTHER              (MCI_VD_OFFSET + 4)

#define MCI_VD_FORMAT_TRACK             0x4001

/* flags for dwFlags parameter of MCI_PLAY command message */
#define MCI_VD_PLAY_REVERSE             0x00010000L
#define MCI_VD_PLAY_FAST                0x00020000L
#define MCI_VD_PLAY_SPEED               0x00040000L
#define MCI_VD_PLAY_SCAN                0x00080000L
#define MCI_VD_PLAY_SLOW                0x00100000L

/* flag for dwFlags parameter of MCI_SEEK command message */
#define MCI_VD_SEEK_REVERSE             0x00010000L

/* flags for dwItem field of MCI_STATUS_PARMS parameter block */
#define MCI_VD_STATUS_SPEED             0x00004002L
#define MCI_VD_STATUS_FORWARD           0x00004003L
#define MCI_VD_STATUS_MEDIA_TYPE        0x00004004L
#define MCI_VD_STATUS_SIDE              0x00004005L
#define MCI_VD_STATUS_DISC_SIZE         0x00004006L

/* flags for dwFlags parameter of MCI_GETDEVCAPS command message */
#define MCI_VD_GETDEVCAPS_CLV           0x00010000L
#define MCI_VD_GETDEVCAPS_CAV           0x00020000L

#define MCI_VD_SPIN_UP                  0x00010000L
#define MCI_VD_SPIN_DOWN                0x00020000L

/* flags for dwItem field of MCI_GETDEVCAPS_PARMS parameter block */
#define MCI_VD_GETDEVCAPS_CAN_REVERSE   0x00004002L
#define MCI_VD_GETDEVCAPS_FAST_RATE     0x00004003L
#define MCI_VD_GETDEVCAPS_SLOW_RATE     0x00004004L
#define MCI_VD_GETDEVCAPS_NORMAL_RATE   0x00004005L

/* flags for the dwFlags parameter of MCI_STEP command message */
#define MCI_VD_STEP_FRAMES              0x00010000L
#define MCI_VD_STEP_REVERSE             0x00020000L

/* flag for the MCI_ESCAPE command message */
#define MCI_VD_ESCAPE_STRING            0x00000100L

/* parameter block for MCI_PLAY command message */
typedef struct tagMCI_VD_PLAY_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrom;
    DWORD       dwTo;
    DWORD       dwSpeed;
} MCI_VD_PLAY_PARMS, *PMCI_VD_PLAY_PARMS, FAR *LPMCI_VD_PLAY_PARMS;

/* parameter block for MCI_STEP command message */
typedef struct tagMCI_VD_STEP_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrames;
} MCI_VD_STEP_PARMS, *PMCI_VD_STEP_PARMS, FAR *LPMCI_VD_STEP_PARMS;

/* parameter block for MCI_ESCAPE command message */
#ifdef _WIN32

typedef struct tagMCI_VD_ESCAPE_PARMSA {
    DWORD_PTR   dwCallback;
    LPCSTR      lpstrCommand;
} MCI_VD_ESCAPE_PARMSA, *PMCI_VD_ESCAPE_PARMSA, *LPMCI_VD_ESCAPE_PARMSA;
typedef struct tagMCI_VD_ESCAPE_PARMSW {
    DWORD_PTR   dwCallback;
    LPCWSTR     lpstrCommand;
} MCI_VD_ESCAPE_PARMSW, *PMCI_VD_ESCAPE_PARMSW, *LPMCI_VD_ESCAPE_PARMSW;
#ifdef UNICODE
typedef MCI_VD_ESCAPE_PARMSW MCI_VD_ESCAPE_PARMS;
typedef PMCI_VD_ESCAPE_PARMSW PMCI_VD_ESCAPE_PARMS;
typedef LPMCI_VD_ESCAPE_PARMSW LPMCI_VD_ESCAPE_PARMS;
#else
typedef MCI_VD_ESCAPE_PARMSA MCI_VD_ESCAPE_PARMS;
typedef PMCI_VD_ESCAPE_PARMSA PMCI_VD_ESCAPE_PARMS;
typedef LPMCI_VD_ESCAPE_PARMSA LPMCI_VD_ESCAPE_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_VD_ESCAPE_PARMS {
    DWORD   dwCallback;
    LPCSTR  lpstrCommand;
} MCI_VD_ESCAPE_PARMS, FAR *LPMCI_VD_ESCAPE_PARMS;
#endif

/* MCI extensions for CD audio devices */

/* flags for the dwItem field of the MCI_STATUS_PARMS parameter block */
#define MCI_CDA_STATUS_TYPE_TRACK       0x00004001L

/* flags for the dwReturn field of MCI_STATUS_PARMS parameter block */
/* MCI_STATUS command, (dwItem == MCI_CDA_STATUS_TYPE_TRACK) */
#define MCI_CDA_TRACK_AUDIO             (MCI_CD_OFFSET + 0)
#define MCI_CDA_TRACK_OTHER             (MCI_CD_OFFSET + 1)

/* MCI extensions for waveform audio devices */

#define MCI_WAVE_PCM                    (MCI_WAVE_OFFSET + 0)
#define MCI_WAVE_MAPPER                 (MCI_WAVE_OFFSET + 1)

/* flags for the dwFlags parameter of MCI_OPEN command message */
#define MCI_WAVE_OPEN_BUFFER            0x00010000L

/* flags for the dwFlags parameter of MCI_SET command message */
#define MCI_WAVE_SET_FORMATTAG          0x00010000L
#define MCI_WAVE_SET_CHANNELS           0x00020000L
#define MCI_WAVE_SET_SAMPLESPERSEC      0x00040000L
#define MCI_WAVE_SET_AVGBYTESPERSEC     0x00080000L
#define MCI_WAVE_SET_BLOCKALIGN         0x00100000L
#define MCI_WAVE_SET_BITSPERSAMPLE      0x00200000L

/* flags for the dwFlags parameter of MCI_STATUS, MCI_SET command messages */
#define MCI_WAVE_INPUT                  0x00400000L
#define MCI_WAVE_OUTPUT                 0x00800000L

/* flags for the dwItem field of MCI_STATUS_PARMS parameter block */
#define MCI_WAVE_STATUS_FORMATTAG       0x00004001L
#define MCI_WAVE_STATUS_CHANNELS        0x00004002L
#define MCI_WAVE_STATUS_SAMPLESPERSEC   0x00004003L
#define MCI_WAVE_STATUS_AVGBYTESPERSEC  0x00004004L
#define MCI_WAVE_STATUS_BLOCKALIGN      0x00004005L
#define MCI_WAVE_STATUS_BITSPERSAMPLE   0x00004006L
#define MCI_WAVE_STATUS_LEVEL           0x00004007L

/* flags for the dwFlags parameter of MCI_SET command message */
#define MCI_WAVE_SET_ANYINPUT           0x04000000L
#define MCI_WAVE_SET_ANYOUTPUT          0x08000000L

/* flags for the dwFlags parameter of MCI_GETDEVCAPS command message */
#define MCI_WAVE_GETDEVCAPS_INPUTS      0x00004001L
#define MCI_WAVE_GETDEVCAPS_OUTPUTS     0x00004002L

/* parameter block for MCI_OPEN command message */
#ifdef _WIN32

typedef struct tagMCI_WAVE_OPEN_PARMSA {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCSTR     lpstrDeviceType;
    LPCSTR     lpstrElementName;
    LPCSTR     lpstrAlias;
    DWORD   dwBufferSeconds;
} MCI_WAVE_OPEN_PARMSA, *PMCI_WAVE_OPEN_PARMSA, *LPMCI_WAVE_OPEN_PARMSA;
typedef struct tagMCI_WAVE_OPEN_PARMSW {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCWSTR    lpstrDeviceType;
    LPCWSTR    lpstrElementName;
    LPCWSTR    lpstrAlias;
    DWORD   dwBufferSeconds;
} MCI_WAVE_OPEN_PARMSW, *PMCI_WAVE_OPEN_PARMSW, *LPMCI_WAVE_OPEN_PARMSW;
#ifdef UNICODE
typedef MCI_WAVE_OPEN_PARMSW MCI_WAVE_OPEN_PARMS;
typedef PMCI_WAVE_OPEN_PARMSW PMCI_WAVE_OPEN_PARMS;
typedef LPMCI_WAVE_OPEN_PARMSW LPMCI_WAVE_OPEN_PARMS;
#else
typedef MCI_WAVE_OPEN_PARMSA MCI_WAVE_OPEN_PARMS;
typedef PMCI_WAVE_OPEN_PARMSA PMCI_WAVE_OPEN_PARMS;
typedef LPMCI_WAVE_OPEN_PARMSA LPMCI_WAVE_OPEN_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_WAVE_OPEN_PARMS {
    DWORD   dwCallback;
    MCIDEVICEID wDeviceID;
    WORD        wReserved0;
    LPCSTR      lpstrDeviceType;
    LPCSTR      lpstrElementName;
    LPCSTR      lpstrAlias;
    DWORD       dwBufferSeconds;
} MCI_WAVE_OPEN_PARMS, FAR *LPMCI_WAVE_OPEN_PARMS;
#endif

/* parameter block for MCI_DELETE command message */
typedef struct tagMCI_WAVE_DELETE_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrom;
    DWORD       dwTo;
} MCI_WAVE_DELETE_PARMS, *PMCI_WAVE_DELETE_PARMS, FAR *LPMCI_WAVE_DELETE_PARMS;

/* parameter block for MCI_SET command message */
typedef struct tagMCI_WAVE_SET_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwTimeFormat;
    DWORD       dwAudio;
#ifdef _WIN32
    UINT    wInput;
    UINT    wOutput;
#else
    WORD    wInput;
    WORD    wReserved0;
    WORD    wOutput;
    WORD    wReserved1;
#endif
    WORD    wFormatTag;
    WORD    wReserved2;
    WORD    nChannels;
    WORD    wReserved3;
    DWORD   nSamplesPerSec;
    DWORD   nAvgBytesPerSec;
    WORD    nBlockAlign;
    WORD    wReserved4;
    WORD    wBitsPerSample;
    WORD    wReserved5;
} MCI_WAVE_SET_PARMS, *PMCI_WAVE_SET_PARMS, FAR * LPMCI_WAVE_SET_PARMS;

/* MCI extensions for MIDI sequencer devices */

/* flags for the dwReturn field of MCI_STATUS_PARMS parameter block */
/* MCI_STATUS command, (dwItem == MCI_SEQ_STATUS_DIVTYPE) */
#define     MCI_SEQ_DIV_PPQN            (0 + MCI_SEQ_OFFSET)
#define     MCI_SEQ_DIV_SMPTE_24        (1 + MCI_SEQ_OFFSET)
#define     MCI_SEQ_DIV_SMPTE_25        (2 + MCI_SEQ_OFFSET)
#define     MCI_SEQ_DIV_SMPTE_30DROP    (3 + MCI_SEQ_OFFSET)
#define     MCI_SEQ_DIV_SMPTE_30        (4 + MCI_SEQ_OFFSET)

/* flags for the dwMaster field of MCI_SEQ_SET_PARMS parameter block */
/* MCI_SET command, (dwFlags == MCI_SEQ_SET_MASTER) */
#define     MCI_SEQ_FORMAT_SONGPTR      0x4001
#define     MCI_SEQ_FILE                0x4002
#define     MCI_SEQ_MIDI                0x4003
#define     MCI_SEQ_SMPTE               0x4004
#define     MCI_SEQ_NONE                65533
#define     MCI_SEQ_MAPPER              65535

/* flags for the dwItem field of MCI_STATUS_PARMS parameter block */
#define MCI_SEQ_STATUS_TEMPO            0x00004002L
#define MCI_SEQ_STATUS_PORT             0x00004003L
#define MCI_SEQ_STATUS_SLAVE            0x00004007L
#define MCI_SEQ_STATUS_MASTER           0x00004008L
#define MCI_SEQ_STATUS_OFFSET           0x00004009L
#define MCI_SEQ_STATUS_DIVTYPE          0x0000400AL
#define MCI_SEQ_STATUS_NAME             0x0000400BL
#define MCI_SEQ_STATUS_COPYRIGHT        0x0000400CL

/* flags for the dwFlags parameter of MCI_SET command message */
#define MCI_SEQ_SET_TEMPO               0x00010000L
#define MCI_SEQ_SET_PORT                0x00020000L
#define MCI_SEQ_SET_SLAVE               0x00040000L
#define MCI_SEQ_SET_MASTER              0x00080000L
#define MCI_SEQ_SET_OFFSET              0x01000000L

/* parameter block for MCI_SET command message */
typedef struct tagMCI_SEQ_SET_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwTimeFormat;
    DWORD       dwAudio;
    DWORD       dwTempo;
    DWORD       dwPort;
    DWORD       dwSlave;
    DWORD       dwMaster;
    DWORD       dwOffset;
} MCI_SEQ_SET_PARMS, *PMCI_SEQ_SET_PARMS, FAR * LPMCI_SEQ_SET_PARMS;

/* MCI extensions for animation devices */

/* flags for dwFlags parameter of MCI_OPEN command message */
#define MCI_ANIM_OPEN_WS                0x00010000L
#define MCI_ANIM_OPEN_PARENT            0x00020000L
#define MCI_ANIM_OPEN_NOSTATIC          0x00040000L

/* flags for dwFlags parameter of MCI_PLAY command message */
#define MCI_ANIM_PLAY_SPEED             0x00010000L
#define MCI_ANIM_PLAY_REVERSE           0x00020000L
#define MCI_ANIM_PLAY_FAST              0x00040000L
#define MCI_ANIM_PLAY_SLOW              0x00080000L
#define MCI_ANIM_PLAY_SCAN              0x00100000L

/* flags for dwFlags parameter of MCI_STEP command message */
#define MCI_ANIM_STEP_REVERSE           0x00010000L
#define MCI_ANIM_STEP_FRAMES            0x00020000L

/* flags for dwItem field of MCI_STATUS_PARMS parameter block */
#define MCI_ANIM_STATUS_SPEED           0x00004001L
#define MCI_ANIM_STATUS_FORWARD         0x00004002L
#define MCI_ANIM_STATUS_HWND            0x00004003L
#define MCI_ANIM_STATUS_HPAL            0x00004004L
#define MCI_ANIM_STATUS_STRETCH         0x00004005L

/* flags for the dwFlags parameter of MCI_INFO command message */
#define MCI_ANIM_INFO_TEXT              0x00010000L

/* flags for dwItem field of MCI_GETDEVCAPS_PARMS parameter block */
#define MCI_ANIM_GETDEVCAPS_CAN_REVERSE 0x00004001L
#define MCI_ANIM_GETDEVCAPS_FAST_RATE   0x00004002L
#define MCI_ANIM_GETDEVCAPS_SLOW_RATE   0x00004003L
#define MCI_ANIM_GETDEVCAPS_NORMAL_RATE 0x00004004L
#define MCI_ANIM_GETDEVCAPS_PALETTES    0x00004006L
#define MCI_ANIM_GETDEVCAPS_CAN_STRETCH 0x00004007L
#define MCI_ANIM_GETDEVCAPS_MAX_WINDOWS 0x00004008L

/* flags for the MCI_REALIZE command message */
#define MCI_ANIM_REALIZE_NORM           0x00010000L
#define MCI_ANIM_REALIZE_BKGD           0x00020000L

/* flags for dwFlags parameter of MCI_WINDOW command message */
#define MCI_ANIM_WINDOW_HWND            0x00010000L
#define MCI_ANIM_WINDOW_STATE           0x00040000L
#define MCI_ANIM_WINDOW_TEXT            0x00080000L
#define MCI_ANIM_WINDOW_ENABLE_STRETCH  0x00100000L
#define MCI_ANIM_WINDOW_DISABLE_STRETCH 0x00200000L

/* flags for hWnd field of MCI_ANIM_WINDOW_PARMS parameter block */
/* MCI_WINDOW command message, (dwFlags == MCI_ANIM_WINDOW_HWND) */
#define MCI_ANIM_WINDOW_DEFAULT         0x00000000L

/* flags for dwFlags parameter of MCI_PUT command message */
#define MCI_ANIM_RECT                   0x00010000L
#define MCI_ANIM_PUT_SOURCE             0x00020000L
#define MCI_ANIM_PUT_DESTINATION        0x00040000L

/* flags for dwFlags parameter of MCI_WHERE command message */
#define MCI_ANIM_WHERE_SOURCE           0x00020000L
#define MCI_ANIM_WHERE_DESTINATION      0x00040000L

/* flags for dwFlags parameter of MCI_UPDATE command message */
#define MCI_ANIM_UPDATE_HDC             0x00020000L

/* parameter block for MCI_OPEN command message */
#ifdef _WIN32

typedef struct tagMCI_ANIM_OPEN_PARMSA {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCSTR      lpstrDeviceType;
    LPCSTR      lpstrElementName;
    LPCSTR      lpstrAlias;
    DWORD   dwStyle;
    HWND    hWndParent;
} MCI_ANIM_OPEN_PARMSA, *PMCI_ANIM_OPEN_PARMSA, *LPMCI_ANIM_OPEN_PARMSA;
typedef struct tagMCI_ANIM_OPEN_PARMSW {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCWSTR     lpstrDeviceType;
    LPCWSTR     lpstrElementName;
    LPCWSTR     lpstrAlias;
    DWORD   dwStyle;
    HWND    hWndParent;
} MCI_ANIM_OPEN_PARMSW, *PMCI_ANIM_OPEN_PARMSW, *LPMCI_ANIM_OPEN_PARMSW;
#ifdef UNICODE
typedef MCI_ANIM_OPEN_PARMSW MCI_ANIM_OPEN_PARMS;
typedef PMCI_ANIM_OPEN_PARMSW PMCI_ANIM_OPEN_PARMS;
typedef LPMCI_ANIM_OPEN_PARMSW LPMCI_ANIM_OPEN_PARMS;
#else
typedef MCI_ANIM_OPEN_PARMSA MCI_ANIM_OPEN_PARMS;
typedef PMCI_ANIM_OPEN_PARMSA PMCI_ANIM_OPEN_PARMS;
typedef LPMCI_ANIM_OPEN_PARMSA LPMCI_ANIM_OPEN_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_ANIM_OPEN_PARMS {
    DWORD   dwCallback;
    MCIDEVICEID wDeviceID;
    WORD        wReserved0;
    LPCSTR      lpstrDeviceType;
    LPCSTR      lpstrElementName;
    LPCSTR      lpstrAlias;
    DWORD       dwStyle;
    HWND        hWndParent;
    WORD        wReserved1;
} MCI_ANIM_OPEN_PARMS, FAR *LPMCI_ANIM_OPEN_PARMS;
#endif

/* parameter block for MCI_PLAY command message */
typedef struct tagMCI_ANIM_PLAY_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrom;
    DWORD       dwTo;
    DWORD       dwSpeed;
} MCI_ANIM_PLAY_PARMS, *PMCI_ANIM_PLAY_PARMS, FAR *LPMCI_ANIM_PLAY_PARMS;

/* parameter block for MCI_STEP command message */
typedef struct tagMCI_ANIM_STEP_PARMS {
    DWORD_PTR   dwCallback;
    DWORD       dwFrames;
} MCI_ANIM_STEP_PARMS, *PMCI_ANIM_STEP_PARMS, FAR *LPMCI_ANIM_STEP_PARMS;

/* parameter block for MCI_WINDOW command message */
#ifdef _WIN32

typedef struct tagMCI_ANIM_WINDOW_PARMSA {
    DWORD_PTR   dwCallback;
    HWND        hWnd;
    UINT        nCmdShow;
    LPCSTR     lpstrText;
} MCI_ANIM_WINDOW_PARMSA, *PMCI_ANIM_WINDOW_PARMSA, * LPMCI_ANIM_WINDOW_PARMSA;
typedef struct tagMCI_ANIM_WINDOW_PARMSW {
    DWORD_PTR   dwCallback;
    HWND        hWnd;
    UINT        nCmdShow;
    LPCWSTR    lpstrText;
} MCI_ANIM_WINDOW_PARMSW, *PMCI_ANIM_WINDOW_PARMSW, * LPMCI_ANIM_WINDOW_PARMSW;
#ifdef UNICODE
typedef MCI_ANIM_WINDOW_PARMSW MCI_ANIM_WINDOW_PARMS;
typedef PMCI_ANIM_WINDOW_PARMSW PMCI_ANIM_WINDOW_PARMS;
typedef LPMCI_ANIM_WINDOW_PARMSW LPMCI_ANIM_WINDOW_PARMS;
#else
typedef MCI_ANIM_WINDOW_PARMSA MCI_ANIM_WINDOW_PARMS;
typedef PMCI_ANIM_WINDOW_PARMSA PMCI_ANIM_WINDOW_PARMS;
typedef LPMCI_ANIM_WINDOW_PARMSA LPMCI_ANIM_WINDOW_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_ANIM_WINDOW_PARMS {
    DWORD   dwCallback;
    HWND    hWnd;
    WORD    wReserved1;
    WORD    nCmdShow;
    WORD    wReserved2;
    LPCSTR  lpstrText;
} MCI_ANIM_WINDOW_PARMS, FAR * LPMCI_ANIM_WINDOW_PARMS;
#endif

/* parameter block for MCI_PUT, MCI_UPDATE, MCI_WHERE command messages */
typedef struct tagMCI_ANIM_RECT_PARMS {
    DWORD_PTR   dwCallback;
#ifdef MCI_USE_OFFEXT
    POINT   ptOffset;
    POINT   ptExtent;
#else   /* ifdef MCI_USE_OFFEXT */
    RECT    rc;
#endif  /* ifdef MCI_USE_OFFEXT */
} MCI_ANIM_RECT_PARMS;
typedef MCI_ANIM_RECT_PARMS * PMCI_ANIM_RECT_PARMS;
typedef MCI_ANIM_RECT_PARMS FAR * LPMCI_ANIM_RECT_PARMS;

/* parameter block for MCI_UPDATE PARMS */
typedef struct tagMCI_ANIM_UPDATE_PARMS {
    DWORD_PTR   dwCallback;
    RECT        rc;
    HDC         hDC;
} MCI_ANIM_UPDATE_PARMS, *PMCI_ANIM_UPDATE_PARMS, FAR * LPMCI_ANIM_UPDATE_PARMS;

/* MCI extensions for video overlay devices */

/* flags for dwFlags parameter of MCI_OPEN command message */
#define MCI_OVLY_OPEN_WS                0x00010000L
#define MCI_OVLY_OPEN_PARENT            0x00020000L

/* flags for dwFlags parameter of MCI_STATUS command message */
#define MCI_OVLY_STATUS_HWND            0x00004001L
#define MCI_OVLY_STATUS_STRETCH         0x00004002L

/* flags for dwFlags parameter of MCI_INFO command message */
#define MCI_OVLY_INFO_TEXT              0x00010000L

/* flags for dwItem field of MCI_GETDEVCAPS_PARMS parameter block */
#define MCI_OVLY_GETDEVCAPS_CAN_STRETCH 0x00004001L
#define MCI_OVLY_GETDEVCAPS_CAN_FREEZE  0x00004002L
#define MCI_OVLY_GETDEVCAPS_MAX_WINDOWS 0x00004003L

/* flags for dwFlags parameter of MCI_WINDOW command message */
#define MCI_OVLY_WINDOW_HWND            0x00010000L
#define MCI_OVLY_WINDOW_STATE           0x00040000L
#define MCI_OVLY_WINDOW_TEXT            0x00080000L
#define MCI_OVLY_WINDOW_ENABLE_STRETCH  0x00100000L
#define MCI_OVLY_WINDOW_DISABLE_STRETCH 0x00200000L

/* flags for hWnd parameter of MCI_OVLY_WINDOW_PARMS parameter block */
#define MCI_OVLY_WINDOW_DEFAULT         0x00000000L

/* flags for dwFlags parameter of MCI_PUT command message */
#define MCI_OVLY_RECT                   0x00010000L
#define MCI_OVLY_PUT_SOURCE             0x00020000L
#define MCI_OVLY_PUT_DESTINATION        0x00040000L
#define MCI_OVLY_PUT_FRAME              0x00080000L
#define MCI_OVLY_PUT_VIDEO              0x00100000L

/* flags for dwFlags parameter of MCI_WHERE command message */
#define MCI_OVLY_WHERE_SOURCE           0x00020000L
#define MCI_OVLY_WHERE_DESTINATION      0x00040000L
#define MCI_OVLY_WHERE_FRAME            0x00080000L
#define MCI_OVLY_WHERE_VIDEO            0x00100000L

/* parameter block for MCI_OPEN command message */
#ifdef _WIN32

typedef struct tagMCI_OVLY_OPEN_PARMSA {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCSTR      lpstrDeviceType;
    LPCSTR      lpstrElementName;
    LPCSTR      lpstrAlias;
    DWORD   dwStyle;
    HWND    hWndParent;
} MCI_OVLY_OPEN_PARMSA, *PMCI_OVLY_OPEN_PARMSA, *LPMCI_OVLY_OPEN_PARMSA;
typedef struct tagMCI_OVLY_OPEN_PARMSW {
    DWORD_PTR   dwCallback;
    MCIDEVICEID wDeviceID;
    LPCWSTR     lpstrDeviceType;
    LPCWSTR     lpstrElementName;
    LPCWSTR     lpstrAlias;
    DWORD   dwStyle;
    HWND    hWndParent;
} MCI_OVLY_OPEN_PARMSW, *PMCI_OVLY_OPEN_PARMSW, *LPMCI_OVLY_OPEN_PARMSW;
#ifdef UNICODE
typedef MCI_OVLY_OPEN_PARMSW MCI_OVLY_OPEN_PARMS;
typedef PMCI_OVLY_OPEN_PARMSW PMCI_OVLY_OPEN_PARMS;
typedef LPMCI_OVLY_OPEN_PARMSW LPMCI_OVLY_OPEN_PARMS;
#else
typedef MCI_OVLY_OPEN_PARMSA MCI_OVLY_OPEN_PARMS;
typedef PMCI_OVLY_OPEN_PARMSA PMCI_OVLY_OPEN_PARMS;
typedef LPMCI_OVLY_OPEN_PARMSA LPMCI_OVLY_OPEN_PARMS;
#endif // UNICODE

#else
typedef struct tagMCI_OVLY_OPEN_PARMS {
    DWORD   dwCallback;
    MCIDEVICEID wDeviceID;
    WORD        wReserved0;
    LPCSTR      lpstrDeviceType;
    LPCSTR      lpstrElementName;
    LPCSTR      lpstrAlias;
    DWORD       dwStyle;
    HWND        hWndParent;
    WORD        wReserved1;
} MCI_OVLY_OPEN_PARMS, FAR *LPMCI_OVLY_OPEN_PARMS;
#endif

/* parameter block for MCI_WINDOW command message */
#ifdef _WIN32

typedef struct tagMCI_OVLY_WINDOW_PARMSA {
    DWORD_PTR   dwCallback;
    HWND        hWnd;
    UINT        nCmdShow;
    LPCSTR      lpstrText;
} MCI_OVLY_WINDOW_PARMSA, *PMCI_OVLY_WINDOW_PARMSA, * LPMCI_OVLY_WINDOW_PARMSA;
typedef struct tagMCI_OVLY_WINDOW_PARMSW {
    DWORD_PTR   dwCallback;
    HWND        hWnd;
    UINT        nCmdShow;
    LPCWSTR     lpstrText;
} MCI_OVLY_WINDOW_PARMSW, *PMCI_OVLY_WINDOW_PARMSW, * LPMCI_OVLY_WINDOW_PARMSW;
#ifdef UNICODE
typedef MCI_OVLY_WINDOW_PARMSW MCI_OVLY_WINDOW_PARMS;
typedef PMCI_OVLY_WINDOW_PARMSW PMCI_OVLY_WINDOW_PARMS;
typedef LPMCI_OVLY_WINDOW_PARMSW LPMCI_OVLY_WINDOW_PARMS;
#else
typedef MCI_OVLY_WINDOW_PARMSA MCI_OVLY_WINDOW_PARMS;
typedef PMCI_OVLY_WINDOW_PARMSA PMCI_OVLY_WINDOW_PARMS;
typedef LPMCI_OVLY_WINDOW_PARMSA LPMCI_OVLY_WINDOW_PARMS;
#endif // UNICODE
#else
typedef struct tagMCI_OVLY_WINDOW_PARMS {
    DWORD   dwCallback;
    HWND    hWnd;
    WORD    wReserved1;
    UINT    nCmdShow;
    WORD    wReserved2;
    LPCSTR  lpstrText;
} MCI_OVLY_WINDOW_PARMS, FAR * LPMCI_OVLY_WINDOW_PARMS;
#endif

/* parameter block for MCI_PUT, MCI_UPDATE, and MCI_WHERE command messages */
typedef struct tagMCI_OVLY_RECT_PARMS {
    DWORD_PTR   dwCallback;
#ifdef MCI_USE_OFFEXT
    POINT   ptOffset;
    POINT   ptExtent;
#else   /* ifdef MCI_USE_OFFEXT */
    RECT    rc;
#endif  /* ifdef MCI_USE_OFFEXT */
} MCI_OVLY_RECT_PARMS, *PMCI_OVLY_RECT_PARMS, FAR * LPMCI_OVLY_RECT_PARMS;

/* parameter block for MCI_SAVE command message */
#ifdef _WIN32

typedef struct tagMCI_OVLY_SAVE_PARMSA {
    DWORD_PTR   dwCallback;
    LPCSTR      lpfilename;
    RECT        rc;
} MCI_OVLY_SAVE_PARMSA, *PMCI_OVLY_SAVE_PARMSA, * LPMCI_OVLY_SAVE_PARMSA;
typedef struct tagMCI_OVLY_SAVE_PARMSW {
    DWORD_PTR   dwCallback;
    LPCWSTR     lpfilename;
    RECT        rc;
} MCI_OVLY_SAVE_PARMSW, *PMCI_OVLY_SAVE_PARMSW, * LPMCI_OVLY_SAVE_PARMSW;
#ifdef UNICODE
typedef MCI_OVLY_SAVE_PARMSW MCI_OVLY_SAVE_PARMS;
typedef PMCI_OVLY_SAVE_PARMSW PMCI_OVLY_SAVE_PARMS;
typedef LPMCI_OVLY_SAVE_PARMSW LPMCI_OVLY_SAVE_PARMS;
#else
typedef MCI_OVLY_SAVE_PARMSA MCI_OVLY_SAVE_PARMS;
typedef PMCI_OVLY_SAVE_PARMSA PMCI_OVLY_SAVE_PARMS;
typedef LPMCI_OVLY_SAVE_PARMSA LPMCI_OVLY_SAVE_PARMS;
#endif // UNICODE
#else
typedef struct tagMCI_OVLY_SAVE_PARMS {
    DWORD   dwCallback;
    LPCSTR  lpfilename;
    RECT    rc;
} MCI_OVLY_SAVE_PARMS, FAR * LPMCI_OVLY_SAVE_PARMS;
#endif

/* parameter block for MCI_LOAD command message */
#ifdef _WIN32

typedef struct tagMCI_OVLY_LOAD_PARMSA {
    DWORD_PTR   dwCallback;
    LPCSTR      lpfilename;
    RECT    rc;
} MCI_OVLY_LOAD_PARMSA, *PMCI_OVLY_LOAD_PARMSA, * LPMCI_OVLY_LOAD_PARMSA;
typedef struct tagMCI_OVLY_LOAD_PARMSW {
    DWORD_PTR   dwCallback;
    LPCWSTR     lpfilename;
    RECT    rc;
} MCI_OVLY_LOAD_PARMSW, *PMCI_OVLY_LOAD_PARMSW, * LPMCI_OVLY_LOAD_PARMSW;
#ifdef UNICODE
typedef MCI_OVLY_LOAD_PARMSW MCI_OVLY_LOAD_PARMS;
typedef PMCI_OVLY_LOAD_PARMSW PMCI_OVLY_LOAD_PARMS;
typedef LPMCI_OVLY_LOAD_PARMSW LPMCI_OVLY_LOAD_PARMS;
#else
typedef MCI_OVLY_LOAD_PARMSA MCI_OVLY_LOAD_PARMS;
typedef PMCI_OVLY_LOAD_PARMSA PMCI_OVLY_LOAD_PARMS;
typedef LPMCI_OVLY_LOAD_PARMSA LPMCI_OVLY_LOAD_PARMS;
#endif // UNICODE
#else
typedef struct tagMCI_OVLY_LOAD_PARMS {
    DWORD   dwCallback;
    LPCSTR  lpfilename;
    RECT    rc;
} MCI_OVLY_LOAD_PARMS, FAR * LPMCI_OVLY_LOAD_PARMS;
#endif

//
// APIs moved from mmddk.h - function prototypes for MCI driver functions
//
DWORD_PTR
APIENTRY
mciGetDriverData(
    MCIDEVICEID wDeviceID
    );

UINT
APIENTRY
mciLoadCommandResource(
    HANDLE hInstance,
    LPCWSTR lpResName,
    UINT wType
    );

BOOL
APIENTRY
mciSetDriverData(
    MCIDEVICEID wDeviceID,
    DWORD_PTR dwData
    );

UINT
APIENTRY
mciDriverYield(
    MCIDEVICEID wDeviceID
    );

BOOL
APIENTRY
mciDriverNotify(
    HANDLE hwndCallback,
    MCIDEVICEID wDeviceID,
    UINT uStatus
    );

BOOL
APIENTRY
mciFreeCommandResource(
    UINT wTable
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _MCIAPI_H_

