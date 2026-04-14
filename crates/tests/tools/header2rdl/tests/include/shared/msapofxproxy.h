//
// MsApoFxProxy.h -- Copyright (c) Microsoft Corporation. All rights reserved.
//
// Description:
//
//   Public definitions for MsApoFxProxy APO
//

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#ifndef _MSAPOFXPROXY_PUBLIC_HEADERFILE_
#define _MSAPOFXPROXY_PUBLIC_HEADERFILE_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <ks.h>

#define STATIC_KSPROPSETID_AudioEffectsDiscovery\
    0xb217a72, 0x16b8, 0x4a4d, 0xbd, 0xed, 0xf9, 0xd6, 0xbb, 0xed, 0xcd, 0x8f
DEFINE_GUIDSTRUCT("0B217A72-16B8-4A4D-BDED-F9D6BBEDCD8F", KSPROPSETID_AudioEffectsDiscovery);
#define KSPROPSETID_AudioEffectsDiscovery DEFINE_GUIDNAMED(KSPROPSETID_AudioEffectsDiscovery)

// define new property id 
typedef enum {
    KSPROPERTY_AUDIOEFFECTSDISCOVERY_EFFECTSLIST = 1,
} KSPROPERTY_AUDIOEFFECTSDISCOVERY;

// Property data for KSPROPERTY_AUDIOEFFECTSDISCOVERY_EFFECTSLIST
typedef struct tagKSP_PINMODE
{
    KSP_PIN     PinProperty;
    GUID        AudioProcessingMode;
} KSP_PINMODE, *PKSP_PINMODE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_MSAPOFXPROXY_PUBLIC_HEADERFILE_

#endif //NTDDI_VERSION