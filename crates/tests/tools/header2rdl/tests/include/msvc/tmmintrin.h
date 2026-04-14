/*
 *  Copyright (C) 1985-2015 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#pragma once

#if !defined(_M_IX86) && !defined(_M_X64) && !(defined(_M_ARM64) && defined(USE_SOFT_INTRINSICS))
#error This header is specific to X86, X64, ARM64, and ARM64EC targets
#endif

#if (defined(_M_ARM64) || defined(_M_ARM64EC)) && !defined(__INTRIN_H_)
#error this header should only be included through <intrin.h>
#endif

#ifndef _INCLUDED_TMM
#define _INCLUDED_TMM
#ifndef __midl

#if defined (_M_CEE_PURE)
        #error ERROR: XMM intrinsics not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

#include <pmmintrin.h>

#ifdef _MM2_FUNCTIONALITY
/* support old notation */
#ifndef _MM_FUNCTIONALITY
#define _MM_FUNCTIONALITY
#endif  /* _MM_FUNCTIONALITY */
#endif  /* _MM2_FUNCTIONALITY */

#ifdef __cplusplus
extern "C" {
#endif  /* __cplusplus */

    // Horizontal Add: add pairs of adjacent words or double words.
    // Each field in the result is the sum of two adjacent fields
    // from the arguments, with the lower result fields coming from
    // the first argument and the upper result fields coming from
    // the second argument. The "hadds" forms saturate the signed
    // addition rather than wrapping.

    extern __m128i _mm_hadd_epi16 (__m128i, __m128i);
    extern __m128i _mm_hadd_epi32 (__m128i, __m128i);
    extern __m128i _mm_hadds_epi16 (__m128i, __m128i);

#if defined(_M_IX86)
    extern __m64 _mm_hadd_pi16 (__m64, __m64);
    extern __m64 _mm_hadd_pi32 (__m64, __m64);
    extern __m64 _mm_hadds_pi16 (__m64, __m64);
#endif

    // Horizontal Subtract: subtract pairs of adjacent words or double
    // words. Each field in the result is the difference of two adjacent
    // fields from the arguments, where the upper field is subtracted
    // from the lower field. The lower result fields come from
    // the first argument and the upper result fields come from
    // the second argument. The "hsubs" forms saturate the signed
    // subtraction rather than wrapping.

    extern __m128i _mm_hsub_epi16 (__m128i, __m128i);
    extern __m128i _mm_hsub_epi32 (__m128i, __m128i);
    extern __m128i _mm_hsubs_epi16 (__m128i, __m128i);

#if defined(_M_IX86)
    extern __m64 _mm_hsub_pi16 (__m64, __m64);
    extern __m64 _mm_hsub_pi32 (__m64, __m64);
    extern __m64 _mm_hsubs_pi16 (__m64, __m64);
#endif

    // Multiply unsigned bytes by signed bytes and sum the word
    // results in pairs with saturation. Each byte of the first
    // argument is zero-extended to a word field and each byte
    // of the second argument is sign-extended to a word field,
    // then each pair of words is multiplied together to give
    // signed word intermediate results. Pairs of words from
    // that result are added horizontally with saturation
    // to give the final result.

    extern __m128i _mm_maddubs_epi16 (__m128i, __m128i);

#if defined(_M_IX86)
    extern __m64 _mm_maddubs_pi16 (__m64, __m64);
#endif

    // Packed multiply high integers with round and scaling,
    // {X,}MM2/m{128,64} (b) to {X,}MM1 (a).

    extern __m128i _mm_mulhrs_epi16 (__m128i, __m128i);

#if defined(_M_IX86)
    extern __m64 _mm_mulhrs_pi16 (__m64, __m64);
#endif

    // Packed shuffle bytes
    // {X,}MM2/m{128,64} (b) by {X,}MM1 (a).

    extern __m128i _mm_shuffle_epi8 (__m128i, __m128i);

#if defined(_M_IX86)
    extern __m64 _mm_shuffle_pi8 (__m64, __m64);
#endif

    // Packed byte, word, double word sign, {X,}MM2/m{128,64} (b) to
    // {X,}MM1 (a).

    extern __m128i _mm_sign_epi8 (__m128i, __m128i);
    extern __m128i _mm_sign_epi16 (__m128i, __m128i);
    extern __m128i _mm_sign_epi32 (__m128i, __m128i);

#if defined(_M_IX86)
    extern __m64 _mm_sign_pi8 (__m64, __m64);
    extern __m64 _mm_sign_pi16 (__m64, __m64);
    extern __m64 _mm_sign_pi32 (__m64, __m64);
#endif

    // Packed align and shift right by n*8 bits,
    // {X,}MM2/m{128,64} (b) to {X,}MM1 (a).

    extern __m128i _mm_alignr_epi8 (__m128i, __m128i, int);

#if defined(_M_IX86)
    extern __m64 _mm_alignr_pi8 (__m64, __m64, int);
#endif

    // Packed byte, word, double word absolute value,
    // {X,}MM2/m{128,64} (b) to {X,}MM1 (a).

    extern __m128i _mm_abs_epi8 (__m128i);
    extern __m128i _mm_abs_epi16 (__m128i);
    extern __m128i _mm_abs_epi32 (__m128i);

#if defined(_M_IX86)
    extern __m64 _mm_abs_pi8 (__m64);
    extern __m64 _mm_abs_pi16 (__m64);
    extern __m64 _mm_abs_pi32 (__m64);
#endif

#ifdef __cplusplus
};
#endif  /* __cplusplus */

#endif  /* defined (_M_CEE_PURE) */
#endif  /* __midl */
#endif  /* _INCLUDED_TMM */
