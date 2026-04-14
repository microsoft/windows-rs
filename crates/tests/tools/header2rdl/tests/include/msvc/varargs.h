//
// varargs.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// XENIX-style macros for accessing arguments of a function which takes a
// variable number of arguments.  Use the C Standard <stdarg.h> instead.
//
#pragma once
#define _INC_VARARGS

#include <vcruntime.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

_CRT_BEGIN_C_HEADER



#if __STDC__
    #error varargs.h incompatible with __STDC__ (use stdarg.h)
#endif

#ifndef va_arg
    #if defined _M_CEE
        #error varargs.h not supported when targeting _M_CEE (use stdarg.h)
    #endif

    #define va_dcl va_list va_alist;
    #define va_arg __crt_va_arg
    #define va_end __crt_va_end

    #if defined _M_IX86 || defined _M_ARM
        #define va_start(ap) ((void)(ap = (va_list)&va_alist))
    #elif defined _M_X64 && !defined(_M_ARM64EC)
        #define va_start(ap) ((void)(__va_start(&ap, 0)))
    #elif defined _M_ARM64 || defined _M_ARM64EC
        #define va_start(ap) ((void)(__va_start(&ap, 0, 0, 0, 0)))
    #else
        #error Unsupported architecture
    #endif
#endif // va_arg



_CRT_END_C_HEADER

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
