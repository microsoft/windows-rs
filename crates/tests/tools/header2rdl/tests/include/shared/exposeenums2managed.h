//------------------------------------------------------------------------------
// File: exposeenums2managed.h
//
// Desc: macros to allow the same enum to be exposed to native and managed
//
// USAGE:
//
// in your whatever.h file that defines the enums use ENUM or FLAGS(for enums defining bitmasks/flags)
// at the top of the file include this .h
// at the bottom of the file include unexposeenums2managed.h(resets the macro state)
//
// in a native client .idl/.h/.cpp file as normal just
// #include <whatever.h>  
// this will include the file normally
//
// in a mgd cpp file 
// #include <whatever.h>
// once normally, this will make the enums available to native
//
// Copyright (c) 2003-2004, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------

// !!! do not pragma once or macro guard this file.
// it gets used multiple times by the same compilation units

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef MANAGED_ENUMS

#ifndef _MANAGED
#error "you can only generate managed enums when compiling managed code"
#endif

#define ENUM typedef public enum class
#define ENUMG(g) ENUM
#define ENUM16 ENUM
#define FLAGS [System::Flags] ENUM
#define FLAGS16 [System::Flags] ENUM16
#define TAG(x) x
#define RATLEVEL(x) EnTvRat_GenericLevel::##x
#define RATATTR(x) BfEnTvRat_GenericAttributes::##x
#ifdef USING_EHRECVR_NAMESPACE
#define EHRECVR_MGD_OUTER_NAMESPACE Microsoft::MediaCenter::TV::Tuners
#define EHRECVR_MGD_NAMESPACE(x) EHRECVR_MGD_OUTER_NAMESPACE##::##x
#define ANALOG_VIDEO_STANDARD_NAMESPACE(x) EHRECVR_MGD_NAMESPACE(AnalogVideoStandard::##x)
#else
#define EHRECVR_MGD_NAMESPACE(x) x
#define ANALOG_VIDEO_STANDARD_NAMESPACE(x) x
#endif


#else // managed

#ifdef __midl
#define V1_ENUM [v1_enum]
#define V1_ENUMG(g) [uuid(g), v1_enum]
#else
#define V1_ENUM
#define V1_ENUMG(g)
#endif
#define RATLEVEL(x) x
#define RATATTR(x) x
#define ENUM typedef V1_ENUM enum
#define ENUMG(g) typedef V1_ENUMG(g) enum
#define ENUM16 typedef enum
#define FLAGS ENUM
#define FLAGS16 ENUM16
#define TAG(x) tag##x
#define EHRECVR_MGD_NAMESPACE(x) x
#define ANALOG_VIDEO_STANDARD_NAMESPACE(x) x

#endif

// end of file - exposeenums2managed.h

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

