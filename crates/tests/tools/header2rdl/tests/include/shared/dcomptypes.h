
//---------------------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//---------------------------------------------------------------------------
#pragma once

#include <dxgitype.h>   // for DXGI_RATIONAL
#include <dxgi1_5.h>    // for DXGI_ALPHA_MODE, DXGI_HDR_METADATA_TYPE
#include <winapifamily.h>


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)


//
// DirectComposition types
//

enum DCOMPOSITION_BITMAP_INTERPOLATION_MODE
{
    DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0,
    DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR           = 1,

    DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT          = 0xffffffff
};

enum DCOMPOSITION_BORDER_MODE
{
    DCOMPOSITION_BORDER_MODE_SOFT       = 0,
    DCOMPOSITION_BORDER_MODE_HARD       = 1,

    DCOMPOSITION_BORDER_MODE_INHERIT    = 0xffffffff
};

enum DCOMPOSITION_COMPOSITE_MODE
{
    DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER        = 0,
    DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT = 1,
#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
    DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND          = 2,
#endif  // (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

    DCOMPOSITION_COMPOSITE_MODE_INHERIT            = 0xffffffff
};

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
enum DCOMPOSITION_BACKFACE_VISIBILITY
{
    DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE    = 0,
    DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN     = 1,

    DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT    = 0xffffffff
};

enum DCOMPOSITION_OPACITY_MODE
{
    DCOMPOSITION_OPACITY_MODE_LAYER     = 0,
    DCOMPOSITION_OPACITY_MODE_MULTIPLY  = 1,

    DCOMPOSITION_OPACITY_MODE_INHERIT   = 0xffffffff
};
#endif  // (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
enum DCOMPOSITION_DEPTH_MODE
{
    DCOMPOSITION_DEPTH_MODE_TREE    = 0,
    DCOMPOSITION_DEPTH_MODE_SPATIAL = 1,
    DCOMPOSITION_DEPTH_MODE_SORTED  = 3,

    DCOMPOSITION_DEPTH_MODE_INHERIT = 0xffffffff
};
#endif  // (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

typedef struct
{
    LARGE_INTEGER lastFrameTime;
    DXGI_RATIONAL currentCompositionRate;
    LARGE_INTEGER currentTime;
    LARGE_INTEGER timeFrequency;
    LARGE_INTEGER nextEstimatedFrameTime;
} DCOMPOSITION_FRAME_STATISTICS;

//
// Composition object specific access flags
//

#define COMPOSITIONOBJECT_READ          0x0001L
#define COMPOSITIONOBJECT_WRITE         0x0002L

#define COMPOSITIONOBJECT_ALL_ACCESS    (COMPOSITIONOBJECT_READ | COMPOSITIONOBJECT_WRITE)

//
// Composition Stats
//

typedef enum COMPOSITION_FRAME_ID_TYPE
{
    COMPOSITION_FRAME_ID_CREATED = 0,
    COMPOSITION_FRAME_ID_CONFIRMED = 1,
    COMPOSITION_FRAME_ID_COMPLETED = 2
} COMPOSITION_FRAME_ID_TYPE;

typedef ULONG64 COMPOSITION_FRAME_ID;

typedef struct tagCOMPOSITION_FRAME_STATS
{
    UINT64 startTime;
    UINT64 targetTime;
    UINT64 framePeriod;
} COMPOSITION_FRAME_STATS;

typedef struct tagCOMPOSITION_TARGET_ID
{
#if defined(__cplusplus) && !defined(SORTPP_PASS)
    bool operator ==(const tagCOMPOSITION_TARGET_ID& rhs) const
    {
        return ((displayAdapterLuid.LowPart == rhs.displayAdapterLuid.LowPart)      &&
                (displayAdapterLuid.HighPart == rhs.displayAdapterLuid.HighPart)    &&
                (renderAdapterLuid.LowPart == rhs.renderAdapterLuid.LowPart)        &&
                (renderAdapterLuid.HighPart == rhs.renderAdapterLuid.HighPart)      &&
                (vidPnSourceId == rhs.vidPnSourceId)                                &&
                (vidPnTargetId == rhs.vidPnTargetId)                                &&
                ((uniqueId == rhs.uniqueId) || (uniqueId == 0) || (rhs.uniqueId == 0)));
    }

    bool operator !=(const tagCOMPOSITION_TARGET_ID& rhs) const
    {
        return !(*this == rhs);
    }
#endif

    LUID displayAdapterLuid;
    LUID renderAdapterLuid;
    UINT vidPnSourceId;
    UINT vidPnTargetId;
    UINT uniqueId;
} COMPOSITION_TARGET_ID;

typedef struct tagCOMPOSITION_STATS
{
    UINT presentCount;
    UINT refreshCount;
    UINT virtualRefreshCount;
    UINT64 time;
} COMPOSITION_STATS;

typedef struct tagCOMPOSITION_TARGET_STATS
{
    UINT outstandingPresents;
    UINT64 presentTime;
    UINT64 vblankDuration;

    COMPOSITION_STATS presentedStats;
    COMPOSITION_STATS completedStats;
} COMPOSITION_TARGET_STATS;

// The maximum nubmer of objects we allow users to wait on the compositor clock
#define DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS 32

// Maximum number of targets kept per frame
#define COMPOSITION_STATS_MAX_TARGETS 256

#endif // NTDDI_WIN8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

