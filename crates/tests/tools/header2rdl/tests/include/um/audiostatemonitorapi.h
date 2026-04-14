/**************************************************************************
 *
 * Copyright (c) Microsoft Corporation.  All rights reserved.
 *
 **************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef __AUDIOSTATEMONITORAPI__
#define __AUDIOSTATEMONITORAPI__

#include <winapifamily.h>
#include <basetyps.h>
#include "AudioSessionTypes.h"

#pragma region Desktop and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

interface IAudioStateMonitor;

typedef void CALLBACK AudioStateMonitorCallback (_In_ IAudioStateMonitor* audioStateMonitor, _In_opt_ void* context);
typedef AudioStateMonitorCallback* PAudioStateMonitorCallback;
typedef __int64 AudioStateMonitorRegistrationHandle;

enum class AudioStateMonitorSoundLevel
{
    Muted = 0, // Audio is muted
    Low,       // Audio is ducked
    Full       // Audio is unchanged
};

/**************************************************************************
 *
 * IAudioStateMonitor interface
 *
 **************************************************************************/

#undef INTERFACE
#define INTERFACE IAudioStateMonitor
DECLARE_INTERFACE_IID_(IAudioStateMonitor, IUnknown, "63BD8738-E30D-4C77-BF5C-834E87C657E2")
{
    // Registers a new callback with the AudioStateMonitor.
    STDMETHOD(RegisterCallback) (_In_ PAudioStateMonitorCallback callback, _In_opt_ void* context, _Out_ AudioStateMonitorRegistrationHandle* registration) PURE;

    // Unregisters an existing callback with the AudioStateMonitor
    // This method will block if any callbacks are in progress, until the callbacks have completed. 
    // This method may be called from within the callback (it will not block in this case)
    STDMETHOD_(void, UnregisterCallback) (_In_ AudioStateMonitorRegistrationHandle registration) PURE;

    // Retrieves the current sound level for the AudioStateMonitor
    STDMETHOD_(AudioStateMonitorSoundLevel, GetSoundLevel) () PURE;
};

/**************************************************************************
 *
 * Functions that create and return an AudioStateMonitor instance.
 *
 * There are four variants each for capture and render streams, to determine 
 * whether a process can 
 *  - capture or render any audio of any category
 *  - capture or render audio of a specific category 
 *  - capture or render audio of a specific category, to a specific endpoint.
 *    The endpoint may be specified using the MMDevice id (obtained using IMMDevice::GetId()), or
 *    by using its SWD id (obtained using Windows.Devices.Enumeration or Windows.Media.Devices.MediaDevice)
 *  - capture or render audio of a specific category to the default endpoint 
 *    for a given role
 **************************************************************************/

STDAPI CreateRenderAudioStateMonitor(_Outptr_ IAudioStateMonitor** audioStateMonitor);
STDAPI CreateRenderAudioStateMonitorForCategory(_In_ AUDIO_STREAM_CATEGORY category, _Outptr_ IAudioStateMonitor** audioStateMonitor);
STDAPI CreateRenderAudioStateMonitorForCategoryAndDeviceRole(_In_ AUDIO_STREAM_CATEGORY category, _In_ ERole role, _Outptr_ IAudioStateMonitor** audioStateMonitor);
STDAPI CreateRenderAudioStateMonitorForCategoryAndDeviceId(_In_ AUDIO_STREAM_CATEGORY category, _In_ PCWSTR deviceId, _Outptr_ IAudioStateMonitor** audioStateMonitor);

STDAPI CreateCaptureAudioStateMonitor(_Outptr_ IAudioStateMonitor** audioStateMonitor);
STDAPI CreateCaptureAudioStateMonitorForCategory(_In_ AUDIO_STREAM_CATEGORY category, _Outptr_ IAudioStateMonitor** audioStateMonitor);
STDAPI CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(_In_ AUDIO_STREAM_CATEGORY category, _In_ ERole role, _Outptr_ IAudioStateMonitor** audioStateMonitor);
STDAPI CreateCaptureAudioStateMonitorForCategoryAndDeviceId(_In_ AUDIO_STREAM_CATEGORY category, _In_ PCWSTR deviceId, _Outptr_ IAudioStateMonitor** audioStateMonitor);

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)"

#endif // __AUDIOSTATEMONITORAPI__
