/********************************************************************************
*                                                                               *
* playsoundapi.h -- ApiSet Contract for api-ms-win-mm-playsound-l1-1-0          *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _PLAYSOUNDAPI_H_
#define _PLAYSOUNDAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#include <mmsyscom.h> // mm common definitions

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef MMNOSOUND
/****************************************************************************

                            Sound support

****************************************************************************/

#ifdef _WIN32

WINMMAPI
BOOL
WINAPI
sndPlaySoundA(
    _In_opt_ LPCSTR pszSound,
    _In_ UINT fuSound
    );

WINMMAPI
BOOL
WINAPI
sndPlaySoundW(
    _In_opt_ LPCWSTR pszSound,
    _In_ UINT fuSound
    );

#ifdef UNICODE
#define sndPlaySound  sndPlaySoundW
#else
#define sndPlaySound  sndPlaySoundA
#endif // !UNICODE

#else
DLOAD_RET(FALSE)
BOOL WINAPI sndPlaySound(LPCSTR pszSound, UINT fuSound);
#endif

/*
 *  flag values for fuSound and fdwSound arguments on [snd]PlaySound
 */
#define SND_SYNC            0x0000  /* play synchronously (default) */
#define SND_ASYNC           0x0001  /* play asynchronously */
#define SND_NODEFAULT       0x0002  /* silence (!default) if sound not found */
#define SND_MEMORY          0x0004  /* pszSound points to a memory file */
#define SND_LOOP            0x0008  /* loop the sound until next sndPlaySound */
#define SND_NOSTOP          0x0010  /* don't stop any currently playing sound */

#define SND_NOWAIT      0x00002000L /* don't wait if the driver is busy */
#define SND_ALIAS       0x00010000L /* name is a registry alias */
#define SND_ALIAS_ID    0x00110000L /* alias is a predefined ID */
#define SND_FILENAME    0x00020000L /* name is file name */
#define SND_RESOURCE    0x00040004L /* name is resource name or atom */
#if(WINVER >= 0x0400)
#define SND_PURGE           0x0040  /* purge non-static events for task */
#define SND_APPLICATION     0x0080  /* look for application specific association */
#endif /* WINVER >= 0x0400 */
#define SND_SENTRY      0x00080000L /* Generate a SoundSentry event with this sound */
#define SND_RING        0x00100000L /* Treat this as a "ring" from a communications app - don't duck me */
#define SND_SYSTEM      0x00200000L /* Treat this as a system sound */

#define SND_ALIAS_START 0           /* alias base */

#ifdef _WIN32
#define sndAlias(ch0, ch1)      (SND_ALIAS_START + (DWORD)(BYTE)(ch0) | ((DWORD)(BYTE)(ch1) << 8))

#define SND_ALIAS_SYSTEMASTERISK        sndAlias('S', '*')
#define SND_ALIAS_SYSTEMQUESTION        sndAlias('S', '?')
#define SND_ALIAS_SYSTEMHAND            sndAlias('S', 'H')
#define SND_ALIAS_SYSTEMEXIT            sndAlias('S', 'E')
#define SND_ALIAS_SYSTEMSTART           sndAlias('S', 'S')
#define SND_ALIAS_SYSTEMWELCOME         sndAlias('S', 'W')
#define SND_ALIAS_SYSTEMEXCLAMATION     sndAlias('S', '!')
#define SND_ALIAS_SYSTEMDEFAULT         sndAlias('S', 'D')

WINMMAPI
BOOL
WINAPI
PlaySoundA(
    _In_opt_ LPCSTR pszSound,
    _In_opt_ HMODULE hmod,
    _In_ DWORD fdwSound
    );

WINMMAPI
BOOL
WINAPI
PlaySoundW(
    _In_opt_ LPCWSTR pszSound,
    _In_opt_ HMODULE hmod,
    _In_ DWORD fdwSound
    );

#ifdef UNICODE
#define PlaySound  PlaySoundW
#else
#define PlaySound  PlaySoundA
#endif // !UNICODE

#else
DLOAD_RET(FALSE)
BOOL WINAPI PlaySound(LPCSTR pszSound, HMODULE hmod, DWORD fdwSound);
#endif

#endif  /* ifndef MMNOSOUND */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _PLAYSOUNDAPI_H_

