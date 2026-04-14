//
// vadefs.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// Definitions of macro helpers used by <stdarg.h>.  This is the topmost header
// in the CRT header lattice, and is always the first CRT header to be included,
// explicitly or implicitly.  Therefore, this header also has several definitions
// that are used throughout the CRT.
//
#pragma once
#define _INC_VADEFS

#define _CRT_PACKING 8
#pragma pack(push, _CRT_PACKING)

// C4339: '__type_info_node': use of undefined type detected in CLR meta-data (/Wall)
#ifndef _VCRUNTIME_DISABLED_WARNING_4339
    #ifdef _M_CEE_PURE
        #define _VCRUNTIME_DISABLED_WARNING_4339 4339
    #else
        #define _VCRUNTIME_DISABLED_WARNING_4339
    #endif
#endif

// C4412: function signature contains type '<typename>';
//        C++ objects are unsafe to pass between pure code and mixed or native. (/Wall)
#ifndef _VCRUNTIME_DISABLED_WARNING_4412
    #ifdef _M_CEE_PURE
        #define _VCRUNTIME_DISABLED_WARNING_4412 4412
    #else
        #define _VCRUNTIME_DISABLED_WARNING_4412
    #endif
#endif

// Use _VCRUNTIME_EXTRA_DISABLED_WARNINGS to add additional warning suppressions to VCRuntime headers.
#ifndef _VCRUNTIME_EXTRA_DISABLED_WARNINGS
    #define _VCRUNTIME_EXTRA_DISABLED_WARNINGS
#endif

// C4514: unreferenced inline function has been removed (/Wall)
// C4820: '<typename>' : 'N' bytes padding added after data member (/Wall)
#ifndef _VCRUNTIME_DISABLED_WARNINGS
    #define _VCRUNTIME_DISABLED_WARNINGS _VCRUNTIME_DISABLED_WARNING_4339 _VCRUNTIME_DISABLED_WARNING_4412 4514 4820 _VCRUNTIME_EXTRA_DISABLED_WARNINGS
#endif

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

#ifdef __cplusplus
extern "C" {
#endif

#if !defined _W64
#define _W64
#endif

#ifndef _UINTPTR_T_DEFINED
    #define _UINTPTR_T_DEFINED
    #ifdef _WIN64
        typedef unsigned __int64  uintptr_t;
    #else
        typedef unsigned int uintptr_t;
    #endif
#endif

#ifndef _VA_LIST_DEFINED
    #define _VA_LIST_DEFINED
    #ifdef _M_CEE_PURE
        typedef System::ArgIterator va_list;
    #else
        typedef char* va_list;
    #endif
#endif

#ifdef __cplusplus
    #define _ADDRESSOF(v) (&const_cast<char&>(reinterpret_cast<const volatile char&>(v)))
#else
    #define _ADDRESSOF(v) (&(v))
#endif

#if (defined _M_ARM || defined _M_HYBRID_X86_ARM64) && !defined _M_CEE_PURE
    #define _VA_ALIGN       4
    #define _SLOTSIZEOF(t)  ((sizeof(t) + _VA_ALIGN - 1) & ~(_VA_ALIGN - 1))
    #define _APALIGN(t,ap)  (((va_list)0 - (ap)) & (__alignof(t) - 1))
#elif (defined _M_ARM64 || defined _M_ARM64EC) && !defined _M_CEE_PURE
    #define _VA_ALIGN       8
    #define _SLOTSIZEOF(t)  ((sizeof(t) + _VA_ALIGN - 1) & ~(_VA_ALIGN - 1))
    #define _APALIGN(t,ap)  (((va_list)0 - (ap)) & (__alignof(t) - 1))
#else
    #define _SLOTSIZEOF(t)  (sizeof(t))
    #define _APALIGN(t,ap)  (__alignof(t))
#endif

#if defined _M_CEE_PURE || (defined _M_CEE && !defined _M_ARM && !defined _M_ARM64)

    void  __cdecl __va_start(va_list*, ...);
    void* __cdecl __va_arg(va_list*, ...);
    void  __cdecl __va_end(va_list*);

    #define __crt_va_start_a(ap, v) ((void)(__va_start(&ap, _ADDRESSOF(v), _SLOTSIZEOF(v), __alignof(v), _ADDRESSOF(v))))
    #define __crt_va_arg(ap, t)     (*(t *)__va_arg(&ap, _SLOTSIZEOF(t), _APALIGN(t,ap), (t*)0))
    #define __crt_va_end(ap)        ((void)(__va_end(&ap)))

#elif defined _M_IX86 && !defined _M_HYBRID_X86_ARM64

    #define _INTSIZEOF(n)          ((sizeof(n) + sizeof(int) - 1) & ~(sizeof(int) - 1))

    #define __crt_va_start_a(ap, v) ((void)(ap = (va_list)_ADDRESSOF(v) + _INTSIZEOF(v)))
    #define __crt_va_arg(ap, t)     (*(t*)((ap += _INTSIZEOF(t)) - _INTSIZEOF(t)))
    #define __crt_va_end(ap)        ((void)(ap = (va_list)0))

#elif defined _M_ARM

    #ifdef __cplusplus
        void __cdecl __va_start(va_list*, ...);
        #define __crt_va_start_a(ap, v) ((void)(__va_start(&ap, _ADDRESSOF(v), _SLOTSIZEOF(v), _ADDRESSOF(v))))
    #else
        #define __crt_va_start_a(ap, v) ((void)(ap = (va_list)_ADDRESSOF(v) + _SLOTSIZEOF(v)))
    #endif

    #define __crt_va_arg(ap, t) (*(t*)((ap += _SLOTSIZEOF(t) + _APALIGN(t,ap)) - _SLOTSIZEOF(t)))
    #define __crt_va_end(ap)    ((void)(ap = (va_list)0))

#elif defined _M_HYBRID_X86_ARM64
    void __cdecl __va_start(va_list*, ...);
    #define __crt_va_start_a(ap,v) ((void)(__va_start(&ap, _ADDRESSOF(v), _SLOTSIZEOF(v), __alignof(v), _ADDRESSOF(v))))
    #define __crt_va_arg(ap, t)    (*(t*)((ap += _SLOTSIZEOF(t)) - _SLOTSIZEOF(t)))
    #define __crt_va_end(ap)       ((void)(ap = (va_list)0))

#elif defined _M_ARM64

    void __cdecl __va_start(va_list*, ...);

    #define __crt_va_start_a(ap,v) ((void)(__va_start(&ap, _ADDRESSOF(v), _SLOTSIZEOF(v), __alignof(v), _ADDRESSOF(v))))
    #define __crt_va_arg(ap, t)                                                 \
       ((sizeof(t) > (2 * sizeof(__int64)))                                   \
           ? **(t**)((ap += sizeof(__int64)) - sizeof(__int64))               \
           : *(t*)((ap += _SLOTSIZEOF(t) + _APALIGN(t,ap)) - _SLOTSIZEOF(t)))
    #define __crt_va_end(ap)       ((void)(ap = (va_list)0))

#elif defined _M_ARM64EC
    void __cdecl __va_start(va_list*, ...);
    //take the ARM64 va_start (for now)
    #define __crt_va_start_a(ap,v) ((void)(__va_start(&ap, _ADDRESSOF(v), _SLOTSIZEOF(v), __alignof(v), _ADDRESSOF(v))))
    //a hybrid va arg, to account for the shift in calling convention, with the alignment of ARM64
    #define __crt_va_arg(ap, t)                                               \
        ((sizeof(t) > sizeof(__int64) || (sizeof(t) & (sizeof(t) - 1)) != 0) \
            ? **(t**)((ap += sizeof(__int64)) - sizeof(__int64))             \
            : *(t*)((ap += _SLOTSIZEOF(t) + _APALIGN(t,ap)) - _SLOTSIZEOF(t)))
    #define __crt_va_end(ap)        ((void)(ap = (va_list)0))

#elif defined _M_X64

    void __cdecl __va_start(va_list* , ...);

    #define __crt_va_start_a(ap, x) ((void)(__va_start(&ap, x)))
    #define __crt_va_arg(ap, t)                                               \
        ((sizeof(t) > sizeof(__int64) || (sizeof(t) & (sizeof(t) - 1)) != 0) \
            ? **(t**)((ap += sizeof(__int64)) - sizeof(__int64))             \
            :  *(t* )((ap += sizeof(__int64)) - sizeof(__int64)))
    #define __crt_va_end(ap)        ((void)(ap = (va_list)0))

#endif

#ifdef __cplusplus
} // extern "C"
#endif

#if defined __cplusplus && !defined _CRT_NO_VA_START_VALIDATION
    extern "C++"
    {
        template <typename _Ty>
        struct __vcrt_va_list_is_reference
        {
            enum : bool { __the_value = false };
        };

        template <typename _Ty>
        struct __vcrt_va_list_is_reference<_Ty&>
        {
            enum : bool { __the_value = true };
        };

        template <typename _Ty>
        struct __vcrt_va_list_is_reference<_Ty&&>
        {
            enum : bool { __the_value = true };
        };

        template <typename _Ty>
        struct __vcrt_assert_va_start_is_not_reference
        {
            static_assert(!__vcrt_va_list_is_reference<_Ty>::__the_value,
                "va_start argument must not have reference type and must not be parenthesized");
        };
    } // extern "C++"

    #define __crt_va_start(ap, x) ((void)(__vcrt_assert_va_start_is_not_reference<decltype(x)>(), __crt_va_start_a(ap, x)))

#else // ^^^ __cplusplus ^^^ // vvv !__cplusplus vvv //

    #define __crt_va_start(ap, x) __crt_va_start_a(ap, x)

#endif

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
#pragma pack(pop)
