//
// crtdefs.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// Declarations used across the Visual C++ Libraries.  The lack of #pragma once
// is deliberate.
//
#include <vcruntime.h>
#include <corecrt.h>



//-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// CRT DLL Export/Import Macros
//
//-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// used to annotate symbols exported from msvcp140
#ifndef _CRTIMP2
    #if defined CRTDLL2 && defined _CRTBLD
        #define _CRTIMP2 __declspec(dllexport)
    #else
        #define _CRTIMP2
    #endif
#endif

// used to annotate symbols exported from msvcp140_1
#ifndef _CRT_SATELLITE_1
    #if defined _BUILDING_SATELLITE_1 && defined _CRTBLD
        #define _CRT_SATELLITE_1 __declspec(dllexport)
    #else
        #define _CRT_SATELLITE_1
    #endif
#endif

// used to annotate symbols exported from msvcp140_2
#ifndef _CRT_SATELLITE_2
    #if defined _BUILDING_SATELLITE_2 && defined _CRTBLD
        #define _CRT_SATELLITE_2 __declspec(dllexport)
    #else
        #define _CRT_SATELLITE_2
    #endif
#endif

// Symbols exported from msvcp140_codecvt_ids are annotated with _CRT_SATELLITE_CODECVT_IDS, except for symbols which
// are data members of classes exported from msvcp140 that must themselves be exported from msvcp140_codecvt_ids, which
// are annotated with _CRT_SATELLITE_CODECVT_IDS_NOIMPORT.
#ifndef _CRT_SATELLITE_CODECVT_IDS
    #if defined _BUILDING_SATELLITE_CODECVT_IDS && defined _CRTBLD
        #define _CRT_SATELLITE_CODECVT_IDS __declspec(dllexport)
        #define _CRT_SATELLITE_CODECVT_IDS_NOIMPORT __declspec(dllexport)
    #elif defined(_DLL)
        #define _CRT_SATELLITE_CODECVT_IDS __declspec(dllimport)
        #define _CRT_SATELLITE_CODECVT_IDS_NOIMPORT
    #else
        #define _CRT_SATELLITE_CODECVT_IDS
        #define _CRT_SATELLITE_CODECVT_IDS_NOIMPORT
    #endif
#endif

#ifndef _CONCRTIMP
    #if defined CONCRTDLL && defined _CRTBLD
        #define _CONCRTIMP __declspec(dllexport)
    #else
        #if defined _DLL && !defined _STATIC_CPPLIB
            #define _CONCRTIMP __declspec(dllimport)
        #else
            #define _CONCRTIMP
        #endif
    #endif
#endif

#ifndef _MRTIMP2
    #if defined CRTDLL2 && defined _CRTBLD
        #define _MRTIMP2 __declspec(dllexport)
    #elif defined MRTDLL && defined _CRTBLD
        #define _MRTIMP2 _MRTIMP
    #else
        #define _MRTIMP2
    #endif
#endif
