/*
 *  Copyright (C) 1985-2015 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/*
 * wmmintrin.h
 *
 * Principal header file for Intel(R) AES and PCLMULQDQ intrinsics.
 */

#pragma once

#if !defined(_M_IX86) && !defined(_M_X64) && !(defined(_M_ARM64) && defined(USE_SOFT_INTRINSICS))
#error This header is specific to X86, X64, ARM64, and ARM64EC targets
#endif

#if (defined(_M_ARM64) || defined(_M_ARM64EC)) && !defined(__INTRIN_H_)
#error this header should only be included through <intrin.h>
#endif

#ifndef _INCLUDED_WMM
#define _INCLUDED_WMM
#ifndef __midl

#if defined (_M_CEE_PURE)
        #error ERROR: EMM intrinsics not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

#include <nmmintrin.h>


#if defined (__cplusplus)
extern "C" {
#endif  /* defined (__cplusplus) */

/*
 * Performs 1 round of AES decryption of the first m128i using
 * the second m128i as a round key.
 */
extern __m128i _mm_aesdec_si128(__m128i /* v */, __m128i /* rkey */);

/*
 * Performs the last round of AES decryption of the first m128i
 * using the second m128i as a round key.
 */
extern __m128i _mm_aesdeclast_si128(__m128i /* v */, __m128i /* rkey */);

/*
 * Performs 1 round of AES encryption of the first m128i using
 * the second m128i as a round key.
 */
extern __m128i _mm_aesenc_si128(__m128i /* v */, __m128i /* rkey */);

/*
 * Performs the last round of AES encryption of the first m128i
 * using the second m128i as a round key.
 */
extern __m128i _mm_aesenclast_si128(__m128i /* v */, __m128i /* rkey */);

/*
 * Performs the InverseMixColumn operation on the source m128i
 * and stores the result into m128i destination.
 */
extern __m128i _mm_aesimc_si128(__m128i /* v */);

/*
 * Generates a m128i round key for the input m128i
 * AES cipher key and byte round constant.
 * The second parameter must be a compile time constant.
 */
extern __m128i _mm_aeskeygenassist_si128(__m128i /* ckey */, const int /* rcon */);

/*
 * Performs carry-less integer multiplication of 64-bit halves
 * of 128-bit input operands.
 * The third parameter indicates which 64-bit halves of the input parameters
 * v1 and v2 should be used. It must be a compile time constant.
 */
extern __m128i _mm_clmulepi64_si128(__m128i /* v1 */, __m128i /* v2 */,
                                            const int /* imm8 */);


#if defined __cplusplus
}; /* End "C" */
#endif  /* defined __cplusplus */

#endif  /* defined (_M_CEE_PURE) */
#endif  /* __midl */
#endif  /* _INCLUDED_WMM */
