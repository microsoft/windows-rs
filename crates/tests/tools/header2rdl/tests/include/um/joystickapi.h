/********************************************************************************
*                                                                               *
* joystickapi.h -- ApiSet Contract for api-ms-win-mm-joystick-l1-1-0            *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _JOYSTICKAPI_H_
#define _JOYSTICKAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#include <mmsyscom.h> // mm common definitions

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef MMNOJOY
/****************************************************************************

                            Joystick support

****************************************************************************/

/* joystick error return values */
#define JOYERR_NOERROR        (0)                  /* no error */
#define JOYERR_PARMS          (JOYERR_BASE+5)      /* bad parameters */
#define JOYERR_NOCANDO        (JOYERR_BASE+6)      /* request not completed */
#define JOYERR_UNPLUGGED      (JOYERR_BASE+7)      /* joystick is unplugged */

/* constants used with JOYINFO and JOYINFOEX structures and MM_JOY* messages */
#define JOY_BUTTON1         0x0001
#define JOY_BUTTON2         0x0002
#define JOY_BUTTON3         0x0004
#define JOY_BUTTON4         0x0008
#define JOY_BUTTON1CHG      0x0100
#define JOY_BUTTON2CHG      0x0200
#define JOY_BUTTON3CHG      0x0400
#define JOY_BUTTON4CHG      0x0800

/* constants used with JOYINFOEX */
#define JOY_BUTTON5         0x00000010l
#define JOY_BUTTON6         0x00000020l
#define JOY_BUTTON7         0x00000040l
#define JOY_BUTTON8         0x00000080l
#define JOY_BUTTON9         0x00000100l
#define JOY_BUTTON10        0x00000200l
#define JOY_BUTTON11        0x00000400l
#define JOY_BUTTON12        0x00000800l
#define JOY_BUTTON13        0x00001000l
#define JOY_BUTTON14        0x00002000l
#define JOY_BUTTON15        0x00004000l
#define JOY_BUTTON16        0x00008000l
#define JOY_BUTTON17        0x00010000l
#define JOY_BUTTON18        0x00020000l
#define JOY_BUTTON19        0x00040000l
#define JOY_BUTTON20        0x00080000l
#define JOY_BUTTON21        0x00100000l
#define JOY_BUTTON22        0x00200000l
#define JOY_BUTTON23        0x00400000l
#define JOY_BUTTON24        0x00800000l
#define JOY_BUTTON25        0x01000000l
#define JOY_BUTTON26        0x02000000l
#define JOY_BUTTON27        0x04000000l
#define JOY_BUTTON28        0x08000000l
#define JOY_BUTTON29        0x10000000l
#define JOY_BUTTON30        0x20000000l
#define JOY_BUTTON31        0x40000000l
#define JOY_BUTTON32        0x80000000l

/* constants used with JOYINFOEX structure */
#define JOY_POVCENTERED         (WORD) -1
#define JOY_POVFORWARD          0
#define JOY_POVRIGHT            9000
#define JOY_POVBACKWARD         18000
#define JOY_POVLEFT             27000

#define JOY_RETURNX             0x00000001l
#define JOY_RETURNY             0x00000002l
#define JOY_RETURNZ             0x00000004l
#define JOY_RETURNR             0x00000008l
#define JOY_RETURNU             0x00000010l     /* axis 5 */
#define JOY_RETURNV             0x00000020l     /* axis 6 */
#define JOY_RETURNPOV           0x00000040l
#define JOY_RETURNBUTTONS       0x00000080l
#define JOY_RETURNRAWDATA       0x00000100l
#define JOY_RETURNPOVCTS        0x00000200l
#define JOY_RETURNCENTERED      0x00000400l
#define JOY_USEDEADZONE         0x00000800l
#define JOY_RETURNALL           (JOY_RETURNX | JOY_RETURNY | JOY_RETURNZ | \
                                 JOY_RETURNR | JOY_RETURNU | JOY_RETURNV | \
                                 JOY_RETURNPOV | JOY_RETURNBUTTONS)
#define JOY_CAL_READALWAYS      0x00010000l
#define JOY_CAL_READXYONLY      0x00020000l
#define JOY_CAL_READ3           0x00040000l
#define JOY_CAL_READ4           0x00080000l
#define JOY_CAL_READXONLY       0x00100000l
#define JOY_CAL_READYONLY       0x00200000l
#define JOY_CAL_READ5           0x00400000l
#define JOY_CAL_READ6           0x00800000l
#define JOY_CAL_READZONLY       0x01000000l
#define JOY_CAL_READRONLY       0x02000000l
#define JOY_CAL_READUONLY       0x04000000l
#define JOY_CAL_READVONLY       0x08000000l

/* joystick ID constants */
#define JOYSTICKID1         0
#define JOYSTICKID2         1

/* joystick driver capabilites */
#define JOYCAPS_HASZ            0x0001
#define JOYCAPS_HASR            0x0002
#define JOYCAPS_HASU            0x0004
#define JOYCAPS_HASV            0x0008
#define JOYCAPS_HASPOV          0x0010
#define JOYCAPS_POV4DIR         0x0020
#define JOYCAPS_POVCTS          0x0040

/* joystick device capabilities data structure */
#ifdef _WIN32

typedef struct tagJOYCAPSA {
    WORD    wMid;                /* manufacturer ID */
    WORD    wPid;                /* product ID */
    CHAR    szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    UINT    wXmin;               /* minimum x position value */
    UINT    wXmax;               /* maximum x position value */
    UINT    wYmin;               /* minimum y position value */
    UINT    wYmax;               /* maximum y position value */
    UINT    wZmin;               /* minimum z position value */
    UINT    wZmax;               /* maximum z position value */
    UINT    wNumButtons;         /* number of buttons */
    UINT    wPeriodMin;          /* minimum message period when captured */
    UINT    wPeriodMax;          /* maximum message period when captured */
#if (WINVER >= 0x0400)
    UINT    wRmin;               /* minimum r position value */
    UINT    wRmax;               /* maximum r position value */
    UINT    wUmin;               /* minimum u (5th axis) position value */
    UINT    wUmax;               /* maximum u (5th axis) position value */
    UINT    wVmin;               /* minimum v (6th axis) position value */
    UINT    wVmax;               /* maximum v (6th axis) position value */
    UINT    wCaps;               /* joystick capabilites */
    UINT    wMaxAxes;            /* maximum number of axes supported */
    UINT    wNumAxes;            /* number of axes in use */
    UINT    wMaxButtons;         /* maximum number of buttons supported */
    CHAR    szRegKey[MAXPNAMELEN];/* registry key */
    CHAR    szOEMVxD[MAX_JOYSTICKOEMVXDNAME]; /* OEM VxD in use */
#endif
} JOYCAPSA, *PJOYCAPSA, *NPJOYCAPSA, *LPJOYCAPSA;
typedef struct tagJOYCAPSW {
    WORD    wMid;                /* manufacturer ID */
    WORD    wPid;                /* product ID */
    WCHAR   szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    UINT    wXmin;               /* minimum x position value */
    UINT    wXmax;               /* maximum x position value */
    UINT    wYmin;               /* minimum y position value */
    UINT    wYmax;               /* maximum y position value */
    UINT    wZmin;               /* minimum z position value */
    UINT    wZmax;               /* maximum z position value */
    UINT    wNumButtons;         /* number of buttons */
    UINT    wPeriodMin;          /* minimum message period when captured */
    UINT    wPeriodMax;          /* maximum message period when captured */
#if (WINVER >= 0x0400)
    UINT    wRmin;               /* minimum r position value */
    UINT    wRmax;               /* maximum r position value */
    UINT    wUmin;               /* minimum u (5th axis) position value */
    UINT    wUmax;               /* maximum u (5th axis) position value */
    UINT    wVmin;               /* minimum v (6th axis) position value */
    UINT    wVmax;               /* maximum v (6th axis) position value */
    UINT    wCaps;               /* joystick capabilites */
    UINT    wMaxAxes;            /* maximum number of axes supported */
    UINT    wNumAxes;            /* number of axes in use */
    UINT    wMaxButtons;         /* maximum number of buttons supported */
    WCHAR   szRegKey[MAXPNAMELEN];/* registry key */
    WCHAR   szOEMVxD[MAX_JOYSTICKOEMVXDNAME]; /* OEM VxD in use */
#endif
} JOYCAPSW, *PJOYCAPSW, *NPJOYCAPSW, *LPJOYCAPSW;
#ifdef UNICODE
typedef JOYCAPSW JOYCAPS;
typedef PJOYCAPSW PJOYCAPS;
typedef NPJOYCAPSW NPJOYCAPS;
typedef LPJOYCAPSW LPJOYCAPS;
#else
typedef JOYCAPSA JOYCAPS;
typedef PJOYCAPSA PJOYCAPS;
typedef NPJOYCAPSA NPJOYCAPS;
typedef LPJOYCAPSA LPJOYCAPS;
#endif // UNICODE
typedef struct tagJOYCAPS2A {
    WORD    wMid;                /* manufacturer ID */
    WORD    wPid;                /* product ID */
    CHAR    szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    UINT    wXmin;               /* minimum x position value */
    UINT    wXmax;               /* maximum x position value */
    UINT    wYmin;               /* minimum y position value */
    UINT    wYmax;               /* maximum y position value */
    UINT    wZmin;               /* minimum z position value */
    UINT    wZmax;               /* maximum z position value */
    UINT    wNumButtons;         /* number of buttons */
    UINT    wPeriodMin;          /* minimum message period when captured */
    UINT    wPeriodMax;          /* maximum message period when captured */
    UINT    wRmin;               /* minimum r position value */
    UINT    wRmax;               /* maximum r position value */
    UINT    wUmin;               /* minimum u (5th axis) position value */
    UINT    wUmax;               /* maximum u (5th axis) position value */
    UINT    wVmin;               /* minimum v (6th axis) position value */
    UINT    wVmax;               /* maximum v (6th axis) position value */
    UINT    wCaps;               /* joystick capabilites */
    UINT    wMaxAxes;            /* maximum number of axes supported */
    UINT    wNumAxes;            /* number of axes in use */
    UINT    wMaxButtons;         /* maximum number of buttons supported */
    CHAR    szRegKey[MAXPNAMELEN];/* registry key */
    CHAR    szOEMVxD[MAX_JOYSTICKOEMVXDNAME]; /* OEM VxD in use */
    GUID    ManufacturerGuid;    /* for extensible MID mapping */
    GUID    ProductGuid;         /* for extensible PID mapping */
    GUID    NameGuid;            /* for name lookup in registry */
} JOYCAPS2A, *PJOYCAPS2A, *NPJOYCAPS2A, *LPJOYCAPS2A;
typedef struct tagJOYCAPS2W {
    WORD    wMid;                /* manufacturer ID */
    WORD    wPid;                /* product ID */
    WCHAR   szPname[MAXPNAMELEN];/* product name (NULL terminated string) */
    UINT    wXmin;               /* minimum x position value */
    UINT    wXmax;               /* maximum x position value */
    UINT    wYmin;               /* minimum y position value */
    UINT    wYmax;               /* maximum y position value */
    UINT    wZmin;               /* minimum z position value */
    UINT    wZmax;               /* maximum z position value */
    UINT    wNumButtons;         /* number of buttons */
    UINT    wPeriodMin;          /* minimum message period when captured */
    UINT    wPeriodMax;          /* maximum message period when captured */
    UINT    wRmin;               /* minimum r position value */
    UINT    wRmax;               /* maximum r position value */
    UINT    wUmin;               /* minimum u (5th axis) position value */
    UINT    wUmax;               /* maximum u (5th axis) position value */
    UINT    wVmin;               /* minimum v (6th axis) position value */
    UINT    wVmax;               /* maximum v (6th axis) position value */
    UINT    wCaps;               /* joystick capabilites */
    UINT    wMaxAxes;            /* maximum number of axes supported */
    UINT    wNumAxes;            /* number of axes in use */
    UINT    wMaxButtons;         /* maximum number of buttons supported */
    WCHAR   szRegKey[MAXPNAMELEN];/* registry key */
    WCHAR   szOEMVxD[MAX_JOYSTICKOEMVXDNAME]; /* OEM VxD in use */
    GUID    ManufacturerGuid;    /* for extensible MID mapping */
    GUID    ProductGuid;         /* for extensible PID mapping */
    GUID    NameGuid;            /* for name lookup in registry */
} JOYCAPS2W, *PJOYCAPS2W, *NPJOYCAPS2W, *LPJOYCAPS2W;
#ifdef UNICODE
typedef JOYCAPS2W JOYCAPS2;
typedef PJOYCAPS2W PJOYCAPS2;
typedef NPJOYCAPS2W NPJOYCAPS2;
typedef LPJOYCAPS2W LPJOYCAPS2;
#else
typedef JOYCAPS2A JOYCAPS2;
typedef PJOYCAPS2A PJOYCAPS2;
typedef NPJOYCAPS2A NPJOYCAPS2;
typedef LPJOYCAPS2A LPJOYCAPS2;
#endif // UNICODE

#else
typedef struct joycaps_tag {
    WORD wMid;                  /* manufacturer ID */
    WORD wPid;                  /* product ID */
    char szPname[MAXPNAMELEN];  /* product name (NULL terminated string) */
    UINT wXmin;                 /* minimum x position value */
    UINT wXmax;                 /* maximum x position value */
    UINT wYmin;                 /* minimum y position value */
    UINT wYmax;                 /* maximum y position value */
    UINT wZmin;                 /* minimum z position value */
    UINT wZmax;                 /* maximum z position value */
    UINT wNumButtons;           /* number of buttons */
    UINT wPeriodMin;            /* minimum message period when captured */
    UINT wPeriodMax;            /* maximum message period when captured */
#if (WINVER >= 0x0400)
    UINT wRmin;                 /* minimum r position value */
    UINT wRmax;                 /* maximum r position value */
    UINT wUmin;                 /* minimum u (5th axis) position value */
    UINT wUmax;                 /* maximum u (5th axis) position value */
    UINT wVmin;                 /* minimum v (6th axis) position value */
    UINT wVmax;                 /* maximum v (6th axis) position value */
    UINT wCaps;                 /* joystick capabilites */
    UINT wMaxAxes;              /* maximum number of axes supported */
    UINT wNumAxes;              /* number of axes in use */
    UINT wMaxButtons;           /* maximum number of buttons supported */
    char szRegKey[MAXPNAMELEN]; /* registry key */
    char szOEMVxD[MAX_JOYSTICKOEMVXDNAME]; /* OEM VxD in use */
#endif
} JOYCAPS, *PJOYCAPS, NEAR *NPJOYCAPS, FAR *LPJOYCAPS;
#endif

/* joystick information data structure */
typedef struct joyinfo_tag {
    UINT wXpos;                 /* x position */
    UINT wYpos;                 /* y position */
    UINT wZpos;                 /* z position */
    UINT wButtons;              /* button states */
} JOYINFO, *PJOYINFO, NEAR *NPJOYINFO, FAR *LPJOYINFO;

#if (WINVER >= 0x0400)
typedef struct joyinfoex_tag {
    DWORD dwSize;                /* size of structure */
    DWORD dwFlags;               /* flags to indicate what to return */
    DWORD dwXpos;                /* x position */
    DWORD dwYpos;                /* y position */
    DWORD dwZpos;                /* z position */
    DWORD dwRpos;                /* rudder/4th axis position */
    DWORD dwUpos;                /* 5th axis position */
    DWORD dwVpos;                /* 6th axis position */
    DWORD dwButtons;             /* button states */
    DWORD dwButtonNumber;        /* current button number pressed */
    DWORD dwPOV;                 /* point of view state */
    DWORD dwReserved1;           /* reserved for communication between winmm & driver */
    DWORD dwReserved2;           /* reserved for future expansion */
} JOYINFOEX, *PJOYINFOEX, NEAR *NPJOYINFOEX, FAR *LPJOYINFOEX;
#endif

/* joystick function prototypes */

#if (WINVER >= 0x0400)
WINMMAPI
MMRESULT
WINAPI
joyGetPosEx(
    _In_ UINT uJoyID,
    _Out_ LPJOYINFOEX pji
    );

#endif /* WINVER >= 0x0400 */

WINMMAPI
UINT
WINAPI
joyGetNumDevs(
    void
    );

#ifdef _WIN32

WINMMAPI
MMRESULT
WINAPI
joyGetDevCapsA(
    _In_ UINT_PTR uJoyID,
    _Out_writes_bytes_(cbjc) LPJOYCAPSA pjc,
    _In_ UINT cbjc
    );

WINMMAPI
MMRESULT
WINAPI
joyGetDevCapsW(
    _In_ UINT_PTR uJoyID,
    _Out_writes_bytes_(cbjc) LPJOYCAPSW pjc,
    _In_ UINT cbjc
    );

#ifdef UNICODE
#define joyGetDevCaps  joyGetDevCapsW
#else
#define joyGetDevCaps  joyGetDevCapsA
#endif // !UNICODE

#else
MMRESULT WINAPI joyGetDevCaps(UINT uJoyID, LPJOYCAPS pjc, UINT cbjc);
#endif

WINMMAPI
MMRESULT
WINAPI
joyGetPos(
    _In_ UINT uJoyID,
    _Out_ LPJOYINFO pji
    );

WINMMAPI
MMRESULT
WINAPI
joyGetThreshold(
    _In_ UINT uJoyID,
    _Out_ LPUINT puThreshold
    );

WINMMAPI
MMRESULT
WINAPI
joyReleaseCapture(
    _In_ UINT uJoyID
    );

WINMMAPI
MMRESULT
WINAPI
joySetCapture(
    _In_ HWND hwnd,
    _In_ UINT uJoyID,
    _In_ UINT uPeriod,
    _In_ BOOL fChanged
    );

WINMMAPI
MMRESULT
WINAPI
joySetThreshold(
    _In_ UINT uJoyID,
    _In_ UINT uThreshold
    );

#if (WINVER >= 0x0400)
WINMMAPI
MMRESULT
WINAPI
joyConfigChanged(
    _In_ DWORD dwFlags
    );

#endif

#endif  /* ifndef MMNOJOY */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _JOYSTICKAPI_H_

