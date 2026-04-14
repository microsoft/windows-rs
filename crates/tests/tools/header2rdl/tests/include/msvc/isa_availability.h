// Copyright (c) Microsoft Corporation. All rights reserved.
#ifndef __ISA_AVAILABILITY__H__
#define __ISA_AVAILABILITY__H__

/*
 * These are defines for the extern "__isa_available" defined in the CRT,
 * which defines the latest instruction set available for use. The compiler
 * backend reads this file to emit vector code for specific microarchitectures.
 *
 * Additional architectural features are defined for extern "__favor",
 * these defines identify performance features that are enabled in the processor.
 * The compiler backend can use these to enable processor specific optimizations.
 */

enum ISA_AVAILABILITY
{
    __ISA_AVAILABLE_X86   = 0,
    __ISA_AVAILABLE_SSE2  = 1,
    __ISA_AVAILABLE_SSE42 = 2,
    __ISA_AVAILABLE_AVX   = 3,
    __ISA_AVAILABLE_ENFSTRG = 4,
    __ISA_AVAILABLE_AVX2 = 5,
    __ISA_AVAILABLE_AVX512 = 6,

    __ISA_AVAILABLE_ARMNT   = 0,   // minimum Win8 ARM support (but w/o NEON)
    __ISA_AVAILABLE_NEON    = 1,   // support for 128-bit NEON instructions
    __ISA_AVAILABLE_NEON_ARM64 = 2,// support for 128-bit NEON instructions for ARM64. The distinction between ARM32 and
                                   // ARM64 NEON is temporary. They may eventually be merged.
};

#if defined(_M_IX86) || defined(_M_X64)

// Inverted ISA extension bits are defined by macros rather than enum so that types are correct
// The lowest byte has flags that should be especially useful for code that must run
// on legacy processors.
#define __IA_SUPPORT_VECTOR128 0x00000001 // Vector lengths up to 128 bits supported (SSE2    if __avx10_version = 0)
#define __IA_SUPPORT_VECTOR256 0x00000002 // Vector lengths up to 256 bits supported (AVX2    if __avx10_version = 0)
#define __IA_SUPPORT_VECTOR512 0x00000004 // Vector lengths up to 512 bits supported (AVX-512 if __avx10_version = 0)
#define __IA_SUPPORT_AVX10     0x00000008 // AVX10 support
#define __IA_SUPPORT_SSE42     0x00000010 // SSE4.2 support (scalar and 128-bit)
#define __IA_SUPPORT_SV128X    0x00000020 // AVX512 or AVX10.1 support (scalar and 128-bit)
#define __IA_SUPPORT_AVX10_2   0x00000040 // AVX10.2 128-bit support (scalar and 128-bit)
#define __IA_SUPPORT_APX       0x00000080 // APX support
#define __IA_SUPPORT_FP16      0x01000000 // FP16 support (scalar and vector)
#endif

#if defined(_M_IX86)

/* Defines for: "__favor" defined in the CRT */
#define __FAVOR_ATOM    0
#define __FAVOR_ENFSTRG 1 /* Enhanced Fast Strings rep movb/stob */
/* #define reserved     2 */

#elif defined(_M_X64)

/* Defines for: "__favor" defined in the CRT */
#define __FAVOR_ATOM    0
#define __FAVOR_ENFSTRG 1 /* Enhanced Fast Strings rep movb/stob */
/* #define reserved     2 */

#endif

#endif // __ISA_AVAILABILITY__H__
