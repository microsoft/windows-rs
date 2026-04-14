//------------------------------------------------------------------------------
// File: expose.h
//
// Desc: macros to allow the same enum to be exposed to native and managed.  
//
// USAGE:
//
// see comments at top of exposeenums2managed.h
//
// Copyright (c) 2003-2004, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------

// !!! do not pragma once or macro guard this file.
// it gets used multiple times by the same compilation units

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#undef V1_ENUM
#undef V1_ENUMG
#undef ENUM
#undef ENUMG
#undef ENUM16
#undef FLAGS
#undef FLAGS16
#undef TAG
#undef EHRECVR_MGD_NAMESPACE
#undef ANALOG_VIDEO_STANDARD_NAMESPACE
#undef RATLEVEL
#undef RATATTR

// end of file - expose.h

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

