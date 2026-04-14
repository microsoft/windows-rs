#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.
*/

#ifndef __TVOUT__
#define __TVOUT__

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef GUID_DEFINED
#include <guiddef.h>
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _VIDEOPARAMETERS {
    GUID  Guid;                         // GUID for this structure
    ULONG dwOffset;                     // leave it 0 for now.
    ULONG dwCommand;                    // VP_COMMAND_*            SET or GET
    ULONG dwFlags;                      // bitfield, defined below SET or GET
    ULONG dwMode;                       // bitfield, defined below SET or GET
    ULONG dwTVStandard;                 // bitfield, defined below SET or GET
    ULONG dwAvailableModes;             // bitfield, defined below GET
    ULONG dwAvailableTVStandard;        // bitfield, defined below GET
    ULONG dwFlickerFilter;              // value                   SET or GET
    ULONG dwOverScanX;                  // value                   SET or GET
    ULONG dwOverScanY;                  //                         SET or GET
    ULONG dwMaxUnscaledX;               // value                   SET or GET
    ULONG dwMaxUnscaledY;               //                         SET or GET
    ULONG dwPositionX;                  // value                   SET or GET
    ULONG dwPositionY;                  //                         SET or GET
    ULONG dwBrightness;                 // value                   SET or GET
    ULONG dwContrast;                   // value                   SET or GET
    ULONG dwCPType;                     // copy protection type    SET or GET
    ULONG dwCPCommand;                  // VP_CP_CMD_
    ULONG dwCPStandard;                 // what TV standards CP is available on. GET
    ULONG dwCPKey;
    ULONG bCP_APSTriggerBits;           // (a dword for alignment) SET(bits 0 and 1 valid).
    UCHAR bOEMCopyProtection[256];      // oem specific copy protection data SET or GET
} VIDEOPARAMETERS, *PVIDEOPARAMETERS, *LPVIDEOPARAMETERS;

#define VP_COMMAND_GET          0x0001  // size set, return caps.
                                        // returned Flags = 0 if not supported.
#define VP_COMMAND_SET          0x0002  // size and params set.

#define VP_FLAGS_TV_MODE        0x0001
#define VP_FLAGS_TV_STANDARD    0x0002
#define VP_FLAGS_FLICKER        0x0004
#define VP_FLAGS_OVERSCAN       0x0008
#define VP_FLAGS_MAX_UNSCALED   0x0010  // do not use on SET
#define VP_FLAGS_POSITION       0x0020
#define VP_FLAGS_BRIGHTNESS     0x0040
#define VP_FLAGS_CONTRAST       0x0080
#define VP_FLAGS_COPYPROTECT    0x0100

#define VP_MODE_WIN_GRAPHICS    0x0001
#define VP_MODE_TV_PLAYBACK     0x0002  // optimize for TV video playback

#define VP_TV_STANDARD_NTSC_M   0x0001  //        75 IRE Setup
#define VP_TV_STANDARD_NTSC_M_J 0x0002  // Japan,  0 IRE Setup
#define VP_TV_STANDARD_PAL_B    0x0004
#define VP_TV_STANDARD_PAL_D    0x0008
#define VP_TV_STANDARD_PAL_H    0x0010
#define VP_TV_STANDARD_PAL_I    0x0020
#define VP_TV_STANDARD_PAL_M    0x0040
#define VP_TV_STANDARD_PAL_N    0x0080
#define VP_TV_STANDARD_SECAM_B  0x0100
#define VP_TV_STANDARD_SECAM_D  0x0200
#define VP_TV_STANDARD_SECAM_G  0x0400
#define VP_TV_STANDARD_SECAM_H  0x0800
#define VP_TV_STANDARD_SECAM_K  0x1000
#define VP_TV_STANDARD_SECAM_K1 0x2000
#define VP_TV_STANDARD_SECAM_L  0x4000
#define VP_TV_STANDARD_WIN_VGA  0x8000
// and the rest
#define VP_TV_STANDARD_NTSC_433 0x00010000
#define VP_TV_STANDARD_PAL_G    0x00020000
#define VP_TV_STANDARD_PAL_60   0x00040000
#define VP_TV_STANDARD_SECAM_L1 0x00080000

#define VP_CP_TYPE_APS_TRIGGER  0x0001  // DVD trigger bits only
#define VP_CP_TYPE_MACROVISION  0x0002  // full macrovision data available

#define VP_CP_CMD_ACTIVATE      0x0001  // CP command type
#define VP_CP_CMD_DEACTIVATE    0x0002
#define VP_CP_CMD_CHANGE        0x0004

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
