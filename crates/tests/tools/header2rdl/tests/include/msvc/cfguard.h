//
// cfguard.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// Declarations of Control Flow Guard instrumentation and support functions.
//
#pragma once
#ifndef _CFGUARD_H
#define _CFGUARD_H

#include <sal.h>
#include <vadefs.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

#pragma pack(push, _CRT_PACKING)

#ifdef __cplusplus
extern "C" {
#endif

#if !defined(__midl) && !defined(MIDL_PASS)

    #ifdef _M_CEE
        // This is a placeholder. Control Flow Guard is not implemented for /clr.
        void __cdecl _guard_check_icall(_In_ uintptr_t _Target);
    #else
        #if defined(_M_ARM64EC)
            uintptr_t __fastcall __os_arm64x_check_icall_thunk(_In_ uintptr_t _Target);
        #elif defined(_M_ARM64)
            void __fastcall __guard_check_icall_thunk(_In_ uintptr_t _Target);
        #else
            void __fastcall _guard_check_icall(_In_ uintptr_t _Target);
        #endif

        #if defined(_CONTROL_FLOW_GUARD) && (_CONTROL_FLOW_GUARD == 1)
            #if defined(_M_ARM64EC)
                #define _GUARD_CHECK_ICALL(_Target) __os_arm64x_check_icall_thunk((uintptr_t)(_Target))
            #elif defined(_M_ARM64)
                #define _GUARD_CHECK_ICALL(_Target) __guard_check_icall_thunk((uintptr_t)(_Target))
            #else
                #define _GUARD_CHECK_ICALL(_Target) _guard_check_icall((uintptr_t)(_Target))
            #endif
        #else
            #define _GUARD_CHECK_ICALL(_Target) ((VOID)0)
        #endif

    #endif

    int __cdecl _guard_icall_checks_enforced(void);

#endif // !defined(__midl) && !defined(MIDL_PASS)

#ifdef __cplusplus
} // extern "C"
#endif

// This is defined in the Windows 10 SDK but not in the Windows 8.1 SDK.
#if !defined DECLSPEC_GUARD_SUPPRESS
    #define DECLSPEC_GUARD_SUPPRESS __declspec(guard(suppress))
#endif

#pragma pack(pop)

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS

#endif // _CFGUARD_H
