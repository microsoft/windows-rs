/*++ BUILD Version: 0001    // Increment this if a change has global effect

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    storprop.h

Abstract:

    this module contains structures and definitions associated
    with exports from storprop.dll

Revision History:

--*/

#ifndef __STORPROP_H__
#define __STORPROP_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <setupapi.h> // for HDEVINFO and PSP_DEVINFO_DATA

#define REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO_VERSION 1

//
// REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO is a structure which
// defines what the OS believes the abilities of a CD-Rom
// capable drive are with respect to Digital Audio Extraction (DAE)
//
// if Accurate is non-zero, then the drive is able to recover
//     from loss-of-streaming conditions without losing any
//     data.  this means the drive never requires what is commonly
//     referred to as 'stitching' to get a correct audio stream
// if Supported is non-zero, then the drive supports audio
//     extraction.  this field does not suggest any quality.
// AccurateMask0 reports what sized reads (in number of sectors)
//     the OS believes to work.  this field is a bitmask, with the
//     lowest bit indicating single-sector reads, and the high bit
//     indictating 32-sector reads.  if reading a given number of
//     sectors per read gives accurate results, the bit will be a
//     '1'.  if Accurate is non-zero, all the bits should be '1'.
//


typedef struct _REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {

    ULONG Version;
    ULONG Accurate;
    ULONG Supported;
    ULONG AccurateMask0;
    // more data may later be added to the end, but
    // backwards compatibility will be retained.

} REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO,
  *PREDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO;

DWORD
CdromCddaInfo(
    IN     HDEVINFO HDevInfo,
    IN     PSP_DEVINFO_DATA DevInfoData,
       OUT PREDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO CddaInfo,
    IN OUT PULONG BufferSize
    );

BOOL
CdromKnownGoodDigitalPlayback(IN HDEVINFO HDevInfo,
                              IN PSP_DEVINFO_DATA DevInfoData);

LONG
CdromEnableDigitalPlayback(IN HDEVINFO DevInfo,
                           IN PSP_DEVINFO_DATA DevInfoData,
                           IN BOOLEAN ForceUnknown);

LONG
CdromDisableDigitalPlayback( IN HDEVINFO DevInfo,
                             IN PSP_DEVINFO_DATA DevInfoData);

LONG
CdromIsDigitalPlaybackEnabled(
    IN  HDEVINFO DevInfo,
    IN  PSP_DEVINFO_DATA DevInfoData,
    OUT PBOOLEAN Enabled
    );

LONG
CdromSetDefaultDvdRegion(IN GEOID GeoId);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __STORPROP_H__

