/*
 *  Copyright (C) 1985-2015 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/*
 * pmmintrin.h
 *
 * Principal header file for Intel(R) Pentium(R) 4 processor SSE3 intrinsics
 */

#pragma once

#if !defined(_M_IX86) && !defined(_M_X64) && !(defined(_M_ARM64) && defined(USE_SOFT_INTRINSICS))
#error This header is specific to X86, X64, ARM64, and ARM64EC targets
#endif

#if (defined(_M_ARM64) || defined(_M_ARM64EC)) && !defined(__INTRIN_H_)
#error this header should only be included through <intrin.h>
#endif

#ifndef _INCLUDED_PMM
#define _INCLUDED_PMM
#ifndef __midl

#if defined (_M_CEE_PURE)
        #error ERROR: EMM intrinsics not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

/*
 * We need emmintrin.h for the basic type declarations.
 */
#include <emmintrin.h>

 /*****************************************************/
 /*     MACROS FOR USE WITH INTRINSICS                */
 /*****************************************************/

/*
 * MACRO functions for setting and reading the DAZ bit in the MXCSR
 */
#define _MM_DENORMALS_ZERO_MASK   0x0040
#define _MM_DENORMALS_ZERO_ON     0x0040
#define _MM_DENORMALS_ZERO_OFF    0x0000

#define _MM_SET_DENORMALS_ZERO_MODE(mode)                                   \
            _mm_setcsr((_mm_getcsr() & ~_MM_DENORMALS_ZERO_MASK) | (mode))
#define _MM_GET_DENORMALS_ZERO_MODE()                                       \
            (_mm_getcsr() & _MM_DENORMALS_ZERO_MASK)


 /*****************************************************/
 /*     INTRINSICS FUNCTION PROTOTYPES START HERE     */
 /*****************************************************/

#if defined __cplusplus
extern "C" { /* Begin "C" */
  /* Intrinsics use C name-mangling. */
#endif  /* defined __cplusplus */

/*
 * New Single precision vector instructions.
 */

extern __m128 _mm_addsub_ps(__m128 /* a */, __m128 /* b */);
extern __m128 _mm_hadd_ps(__m128 /* a */, __m128 /* b */);
extern __m128 _mm_hsub_ps(__m128 /* a */, __m128 /* b */);
extern __m128 _mm_movehdup_ps(__m128 /* a */);
extern __m128 _mm_moveldup_ps(__m128 /* a */);

/*
 * New double precision vector instructions.
 */

extern __m128d _mm_addsub_pd(__m128d /* a */, __m128d /* b */);
extern __m128d _mm_hadd_pd(__m128d /* a */, __m128d /* b */);
extern __m128d _mm_hsub_pd(__m128d /* a */, __m128d /* b */);
extern __m128d _mm_loaddup_pd(double const * /* dp */);
extern __m128d _mm_movedup_pd(__m128d /* a */);

/*
 * New unaligned integer vector load instruction.
 */
extern __m128i _mm_lddqu_si128(__m128i const * /* p */);

/*
 * Miscellaneous new instructions.
 */
/*
 * For _mm_monitor p goes in eax, extensions goes in ecx, hints goes in edx.
 */
extern void _mm_monitor(void const * /* p */, unsigned /* extensions */, unsigned /* hints */);

/*
 * For _mm_mwait, extensions goes in ecx, hints goes in eax.
 */
extern void _mm_mwait(unsigned /* extensions */, unsigned /* hints */);

#if defined __cplusplus
}; /* End "C" */
#endif  /* defined __cplusplus */

#endif  /* defined (_M_CEE_PURE) */
#endif  /* __midl */
#endif  /* _INCLUDED_PMM */
