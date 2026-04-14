#include <winapifamily.h>

/*==========================================================================
 *
 *  mmsystem.h -- Include file for Multimedia API's
 *
 *  Version 4.00
 *
 *  Copyright (C) 1992-1998 Microsoft Corporation.  All Rights Reserved.
 *
 *--------------------------------------------------------------------------
 *
 *  Define:         Prevent inclusion of:
 *  --------------  --------------------------------------------------------
 *  MMNODRV         Installable driver support
 *  MMNOSOUND       Sound support
 *  MMNOWAVE        Waveform support
 *  MMNOMIDI        MIDI support
 *  MMNOAUX         Auxiliary audio support
 *  MMNOMIXER       Mixer support
 *  MMNOTIMER       Timer support
 *  MMNOJOY         Joystick support
 *  MMNOMCI         MCI support
 *  MMNOMMIO        Multimedia file I/O support
 *  MMNOMMSYSTEM    General MMSYSTEM functions
 *
 *==========================================================================
 */

#ifndef _INC_MMSYSTEM
#define _INC_MMSYSTEM   /* #defined if mmsystem.h has been included */


#include <mmsyscom.h>

#ifdef _WIN32
#include <pshpack1.h>
#else
#ifndef RC_INVOKED
#pragma pack(1)
#endif
#endif

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/****************************************************************************

                    Multimedia Extensions Window Messages

****************************************************************************/


#ifndef MMNOMCI

/* MMNOMCI         MCI support */
#include <mciapi.h>

#endif // #ifndef MMNOMCI

/* MMNODRV - Installable driver support */
#include <mmiscapi.h>
#include <mmiscapi2.h>


/* MMNOSOUND  Sound support */
#include <playsoundapi.h>

#include <mmeapi.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef MMNOTIMER
/****************************************************************************

                            Timer support

****************************************************************************/

#include <timeapi.h>


/* timer data types */
#endif  /* ifndef MMNOTIMER */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Joystickapi API Set contract
//
#include <joystickapi.h>



/****************************************************************************

                        DISPLAY Driver extensions

****************************************************************************/

#ifndef NEWTRANSPARENT
    #define NEWTRANSPARENT  3           /* use with SetBkMode() */

    #define QUERYROPSUPPORT 40          /* use to determine ROP support */
#endif  /* ifndef NEWTRANSPARENT */

/****************************************************************************

                        DIB Driver extensions

****************************************************************************/

#define SELECTDIB       41                      /* DIB.DRV select dib escape */
#define DIBINDEX(n)     MAKELONG((n),0x10FF)


/****************************************************************************

                        ScreenSaver support

    The current application will receive a syscommand of SC_SCREENSAVE just
    before the screen saver is invoked.  If the app wishes to prevent a
    screen save, return non-zero value, otherwise call DefWindowProc().

****************************************************************************/

#ifndef SC_SCREENSAVE

    #define SC_SCREENSAVE   0xF140

#endif  /* ifndef SC_SCREENSAVE */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}                       /* End of extern "C" { */
#endif  /* __cplusplus */

#ifdef _WIN32
#include <poppack.h>
#else
#ifndef RC_INVOKED
#pragma pack()
#endif
#endif

#endif  /* _INC_MMSYSTEM */




