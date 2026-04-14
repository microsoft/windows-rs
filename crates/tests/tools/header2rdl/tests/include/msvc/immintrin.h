/*
 *  Copyright (C) 1985-2023 Intel Corporation.
 *
 *  SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/***
* immintrin.h - Meta Header file for Intel(R) Architecture intrinsic functions.
*
*******************************************************************************/

#pragma once

#if !defined(_M_IX86) && !defined(_M_X64) && !(defined(_M_ARM64) && defined(USE_SOFT_INTRINSICS))
#error This header is specific to X86, X64, ARM64, and ARM64EC targets
#endif

#if (defined(_M_ARM64) || defined(_M_ARM64EC)) && !defined(__INTRIN_H_)
#error this header should only be included through <intrin.h>
#endif

#ifndef _INCLUDED_IMM
#define _INCLUDED_IMM
#ifndef __midl

#if defined (_M_CEE_PURE)
        #error ERROR: Intel Architecture intrinsic functions not supported in the pure mode!
#else  /* defined (_M_CEE_PURE) */

#include <wmmintrin.h>

#ifdef __cplusplus
extern "C" {
#endif  /* __cplusplus */

/*
 * Intel(R) AVX compiler intrinsic functions.
 */
typedef union __declspec(intrin_type) __declspec(align(32)) __m256 {
    float m256_f32[8];
} __m256;

typedef struct __declspec(intrin_type) __declspec(align(32)) __m256d {
    double m256d_f64[4];
} __m256d;

typedef union  __declspec(intrin_type) __declspec(align(32)) __m256i {
    __int8              m256i_i8[32];
    __int16             m256i_i16[16];
    __int32             m256i_i32[8];
    __int64             m256i_i64[4];
    unsigned __int8     m256i_u8[32];
    unsigned __int16    m256i_u16[16];
    unsigned __int32    m256i_u32[8];
    unsigned __int64    m256i_u64[4];
} __m256i;


/*
 * Compare predicates for scalar and packed compare intrinsic functions
 */
#define _CMP_EQ_OQ     0x00  /* Equal (ordered, nonsignaling)               */
#define _CMP_LT_OS     0x01  /* Less-than (ordered, signaling)              */
#define _CMP_LE_OS     0x02  /* Less-than-or-equal (ordered, signaling)     */
#define _CMP_UNORD_Q   0x03  /* Unordered (nonsignaling)                    */
#define _CMP_NEQ_UQ    0x04  /* Not-equal (unordered, nonsignaling)         */
#define _CMP_NLT_US    0x05  /* Not-less-than (unordered, signaling)        */
#define _CMP_NLE_US    0x06  /* Not-less-than-or-equal (unordered,
                                                        signaling)          */
#define _CMP_ORD_Q     0x07  /* Ordered (nonsignaling)                      */
#define _CMP_EQ_UQ     0x08  /* Equal (unordered, non-signaling)            */
#define _CMP_NGE_US    0x09  /* Not-greater-than-or-equal (unordered,
                                                           signaling)       */
#define _CMP_NGT_US    0x0A  /* Not-greater-than (unordered, signaling)     */
#define _CMP_FALSE_OQ  0x0B  /* False (ordered, nonsignaling)               */
#define _CMP_NEQ_OQ    0x0C  /* Not-equal (ordered, non-signaling)          */
#define _CMP_GE_OS     0x0D  /* Greater-than-or-equal (ordered, signaling)  */
#define _CMP_GT_OS     0x0E  /* Greater-than (ordered, signaling)           */
#define _CMP_TRUE_UQ   0x0F  /* True (unordered, non-signaling)             */
#define _CMP_EQ_OS     0x10  /* Equal (ordered, signaling)                  */
#define _CMP_LT_OQ     0x11  /* Less-than (ordered, nonsignaling)           */
#define _CMP_LE_OQ     0x12  /* Less-than-or-equal (ordered, nonsignaling)  */
#define _CMP_UNORD_S   0x13  /* Unordered (signaling)                       */
#define _CMP_NEQ_US    0x14  /* Not-equal (unordered, signaling)            */
#define _CMP_NLT_UQ    0x15  /* Not-less-than (unordered, nonsignaling)     */
#define _CMP_NLE_UQ    0x16  /* Not-less-than-or-equal (unordered,
                                                        nonsignaling)       */
#define _CMP_ORD_S     0x17  /* Ordered (signaling)                         */
#define _CMP_EQ_US     0x18  /* Equal (unordered, signaling)                */
#define _CMP_NGE_UQ    0x19  /* Not-greater-than-or-equal (unordered,
                                                           nonsignaling)    */
#define _CMP_NGT_UQ    0x1A  /* Not-greater-than (unordered, nonsignaling)  */
#define _CMP_FALSE_OS  0x1B  /* False (ordered, signaling)                  */
#define _CMP_NEQ_OS    0x1C  /* Not-equal (ordered, signaling)              */
#define _CMP_GE_OQ     0x1D  /* Greater-than-or-equal (ordered,
                                                       nonsignaling)        */
#define _CMP_GT_OQ     0x1E  /* Greater-than (ordered, nonsignaling)        */
#define _CMP_TRUE_US   0x1F  /* True (unordered, signaling)                 */

#if !defined(_M_ARM64) && !defined(_M_ARM64EC)
// The feature bits for these functions are defined in isa_availability.h
// __isa_inverted has cleared bits to signal ISA support
// These intrinsics (__isa_inverted, __avx10_version, __arch_inverted, __arch_avx10ver)
// are not defined for ARM64 and ARM64EC.
extern unsigned long long __isa_inverted;
extern unsigned __avx10_version;
#ifdef __cplusplus
inline bool __check_isa_support(unsigned __x, unsigned __v = 0)
#else
__inline _Bool __check_isa_support(unsigned __x, unsigned __v)
#endif
{ return ((__x & __isa_inverted) == 0) && ((unsigned char)__avx10_version >= __v); }

// Simplified check for 512-bit AVX10 support
#define __IA_AVX10_512         0x00040000
#ifdef __cplusplus
inline bool
#else
__inline _Bool
#endif
__check_isa_avx10_512(unsigned __v) { return (__avx10_version >= (__v | __IA_AVX10_512)); }

// __arch_inverted() returns cleared bits to signal /arch support
// (Because these always return constant values, no need for a simplified 512-bit check.)
extern unsigned __arch_inverted(void);
extern unsigned __arch_avx10ver(void);
#ifdef __cplusplus
inline bool __check_arch_support(unsigned __x, unsigned __v = 0)
#else
__inline _Bool __check_arch_support(unsigned __x, unsigned __v)
#endif
{ return ((__x & __arch_inverted()) == 0) && (__arch_avx10ver() >= __v); }

#if !defined(__IA_SUPPORT_VECTOR128)
#define __IA_SUPPORT_VECTOR128 0x00000001
#define __IA_SUPPORT_VECTOR256 0x00000002
#define __IA_SUPPORT_VECTOR512 0x00000004
#endif

#ifdef __cplusplus
inline unsigned _get_vlen(void)
#else
__inline unsigned _get_vlen(void)
#endif
{
    unsigned arch_inverted = __arch_inverted();
    if ((arch_inverted & __IA_SUPPORT_VECTOR512) == 0)
        return 512;
    else if ((arch_inverted & __IA_SUPPORT_VECTOR256) == 0)
        return 256;
    else if ((arch_inverted & __IA_SUPPORT_VECTOR128) == 0)
        return 128;
    else
        return 0;
}
#endif // !defined(_M_ARM64) && !defined(_M_ARM64EC)

// The consolidated Intel Architecture ISA feature bits

/*
 * Add Packed Double Precision Floating-Point Values
 * **** VADDPD ymm1, ymm2, ymm3/m256
 * Performs an SIMD add of the four packed double-precision floating-point
 * values from the first source operand to the second source operand, and
 * stores the packed double-precision floating-point results in the
 * destination
 */
extern __m256d __cdecl _mm256_add_pd(__m256d, __m256d);

/*
 * Add Packed Single Precision Floating-Point Values
 * **** VADDPS ymm1, ymm2, ymm3/m256
 * Performs an SIMD add of the eight packed single-precision floating-point
 * values from the first source operand to the second source operand, and
 * stores the packed single-precision floating-point results in the
 * destination
 */
extern __m256 __cdecl _mm256_add_ps(__m256, __m256);

/*
 * Add/Subtract Double Precision Floating-Point Values
 * **** VADDSUBPD ymm1, ymm2, ymm3/m256
 * Adds odd-numbered double-precision floating-point values of the first
 * source operand with the corresponding double-precision floating-point
 * values from the second source operand; stores the result in the odd-numbered
 * values of the destination. Subtracts the even-numbered double-precision
 * floating-point values from the second source operand from the corresponding
 * double-precision floating values in the first source operand; stores the
 * result into the even-numbered values of the destination
 */
extern __m256d __cdecl _mm256_addsub_pd(__m256d, __m256d);

/*
 * Add/Subtract Packed Single Precision Floating-Point Values
 * **** VADDSUBPS ymm1, ymm2, ymm3/m256
 * Adds odd-numbered single-precision floating-point values of the first source
 * operand with the corresponding single-precision floating-point values from
 * the second source operand; stores the result in the odd-numbered values of
 * the destination. Subtracts the even-numbered single-precision floating-point
 * values from the second source operand from the corresponding
 * single-precision floating values in the first source operand; stores the
 * result into the even-numbered values of the destination
 */
extern __m256 __cdecl _mm256_addsub_ps(__m256, __m256);

/*
 * Bitwise Logical AND of Packed Double Precision Floating-Point Values
 * **** VANDPD ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical AND of the four packed double-precision
 * floating-point values from the first source operand and the second
 * source operand, and stores the result in the destination
 */
extern __m256d __cdecl _mm256_and_pd(__m256d, __m256d);

/*
 * Bitwise Logical AND of Packed Single Precision Floating-Point Values
 * **** VANDPS ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical AND of the eight packed single-precision
 * floating-point values from the first source operand and the second
 * source operand, and stores the result in the destination
 */
extern __m256 __cdecl _mm256_and_ps(__m256, __m256);

/*
 * Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values
 * **** VANDNPD ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical AND NOT of the four packed double-precision
 * floating-point values from the first source operand and the second source
 * operand, and stores the result in the destination
 */
extern __m256d __cdecl _mm256_andnot_pd(__m256d, __m256d);

/*
 * Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values
 * **** VANDNPS ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical AND NOT of the eight packed single-precision
 * floating-point values from the first source operand and the second source
 * operand, and stores the result in the destination
 */
extern __m256 __cdecl _mm256_andnot_ps(__m256, __m256);

/*
 * Blend Packed Double Precision Floating-Point Values
 * **** VBLENDPD ymm1, ymm2, ymm3/m256, imm8
 * Double-Precision Floating-Point values from the second source operand are
 * conditionally merged with values from the first source operand and written
 * to the destination. The immediate bits [3:0] determine whether the
 * corresponding Double-Precision Floating Point value in the destination is
 * copied from the second source or first source. If a bit in the mask,
 * corresponding to a word, is "1", then the Double-Precision Floating-Point
 * value in the second source operand is copied, else the value in the first
 * source operand is copied
 */
extern __m256d __cdecl _mm256_blend_pd(__m256d, __m256d, const int);

/*
 * Blend Packed Single Precision Floating-Point Values
 * **** VBLENDPS ymm1, ymm2, ymm3/m256, imm8
 * Single precision floating point values from the second source operand are
 * conditionally merged with values from the first source operand and written
 * to the destination. The immediate bits [7:0] determine whether the
 * corresponding single precision floating-point value in the destination is
 * copied from the second source or first source. If a bit in the mask,
 * corresponding to a word, is "1", then the single-precision floating-point
 * value in the second source operand is copied, else the value in the first
 * source operand is copied
 */
extern __m256 __cdecl _mm256_blend_ps(__m256, __m256, const int);

/*
 * Blend Packed Double Precision Floating-Point Values
 * **** VBLENDVPD ymm1, ymm2, ymm3/m256, ymm4
 * Conditionally copy each quadword data element of double-precision
 * floating-point value from the second source operand (third operand) and the
 * first source operand (second operand) depending on mask bits defined in the
 * mask register operand (fourth operand).
 */
extern __m256d __cdecl _mm256_blendv_pd(__m256d, __m256d, __m256d);

/*
 * Blend Packed Single Precision Floating-Point Values
 * **** VBLENDVPS ymm1, ymm2, ymm3/m256, ymm4
 * Conditionally copy each dword data element of single-precision
 * floating-point value from the second source operand (third operand) and the
 * first source operand (second operand) depending on mask bits defined in the
 * mask register operand (fourth operand).
 */
extern __m256 __cdecl _mm256_blendv_ps(__m256, __m256, __m256);

/*
 * Divide Packed Double-Precision Floating-Point Values
 * **** VDIVPD ymm1, ymm2, ymm3/m256
 * Performs an SIMD divide of the four packed double-precision floating-point
 * values in the first source operand by the four packed double-precision
 * floating-point values in the second source operand
 */
extern __m256d __cdecl _mm256_div_pd(__m256d, __m256d);

/*
 * Divide Packed Single-Precision Floating-Point Values
 * **** VDIVPS ymm1, ymm2, ymm3/m256
 * Performs an SIMD divide of the eight packed single-precision
 * floating-point values in the first source operand by the eight packed
 * single-precision floating-point values in the second source operand
 */
extern __m256 __cdecl _mm256_div_ps(__m256, __m256);

/*
 * Dot Product of Packed Single-Precision Floating-Point Values
 * **** VDPPS ymm1, ymm2, ymm3/m256, imm8
 * Multiplies the packed single precision floating point values in the
 * first source operand with the packed single-precision floats in the
 * second source. Each of the four resulting single-precision values is
 * conditionally summed depending on a mask extracted from the high 4 bits
 * of the immediate operand. This sum is broadcast to each of 4 positions
 * in the destination if the corresponding bit of the mask selected from
 * the low 4 bits of the immediate operand is "1". If the corresponding
 * low bit 0-3 of the mask is zero, the destination is set to zero.
 * The process is replicated for the high elements of the destination.
 */
extern __m256 __cdecl _mm256_dp_ps(__m256, __m256, const int);

/*
 * Add Horizontal Double Precision Floating-Point Values
 * **** VHADDPD ymm1, ymm2, ymm3/m256
 * Adds pairs of adjacent double-precision floating-point values in the
 * first source operand and second source operand and stores results in
 * the destination
 */
extern __m256d __cdecl _mm256_hadd_pd(__m256d, __m256d);

/*
 * Add Horizontal Single Precision Floating-Point Values
 * **** VHADDPS ymm1, ymm2, ymm3/m256
 * Adds pairs of adjacent single-precision floating-point values in the
 * first source operand and second source operand and stores results in
 * the destination
 */
extern __m256 __cdecl _mm256_hadd_ps(__m256, __m256);

/*
 * Subtract Horizontal Double Precision Floating-Point Values
 * **** VHSUBPD ymm1, ymm2, ymm3/m256
 * Subtract pairs of adjacent double-precision floating-point values in
 * the first source operand and second source operand and stores results
 * in the destination
 */
extern __m256d __cdecl _mm256_hsub_pd(__m256d, __m256d);

/*
 * Subtract Horizontal Single Precision Floating-Point Values
 * **** VHSUBPS ymm1, ymm2, ymm3/m256
 * Subtract pairs of adjacent single-precision floating-point values in
 * the first source operand and second source operand and stores results
 * in the destination.
 */
extern __m256 __cdecl _mm256_hsub_ps(__m256, __m256);

/*
 * Maximum of Packed Double Precision Floating-Point Values
 * **** VMAXPD ymm1, ymm2, ymm3/m256
 * Performs an SIMD compare of the packed double-precision floating-point
 * values in the first source operand and the second source operand and
 * returns the maximum value for each pair of values to the destination
 */
extern __m256d __cdecl _mm256_max_pd(__m256d, __m256d);

/*
 * Maximum of Packed Single Precision Floating-Point Values
 * **** VMAXPS ymm1, ymm2, ymm3/m256
 * Performs an SIMD compare of the packed single-precision floating-point
 * values in the first source operand and the second source operand and
 * returns the maximum value for each pair of values to the destination
 */
extern __m256 __cdecl _mm256_max_ps(__m256, __m256);

/*
 * Minimum of Packed Double Precision Floating-Point Values
 * **** VMINPD ymm1, ymm2, ymm3/m256
 * Performs an SIMD compare of the packed double-precision floating-point
 * values in the first source operand and the second source operand and
 * returns the minimum value for each pair of values to the destination
 */
extern __m256d __cdecl _mm256_min_pd(__m256d, __m256d);

/*
 * Minimum of Packed Single Precision Floating-Point Values
 * **** VMINPS ymm1, ymm2, ymm3/m256
 * Performs an SIMD compare of the packed single-precision floating-point
 * values in the first source operand and the second source operand and
 * returns the minimum value for each pair of values to the destination
 */
extern __m256 __cdecl _mm256_min_ps(__m256, __m256);

/*
 * Multiply Packed Double Precision Floating-Point Values
 * **** VMULPD ymm1, ymm2, ymm3/m256
 * Performs a SIMD multiply of the four packed double-precision floating-point
 * values from the first Source operand to the Second Source operand, and
 * stores the packed double-precision floating-point results in the
 * destination
 */
extern __m256d __cdecl _mm256_mul_pd(__m256d, __m256d);

/*
 * Multiply Packed Single Precision Floating-Point Values
 * **** VMULPS ymm1, ymm2, ymm3/m256
 * Performs an SIMD multiply of the eight packed single-precision
 * floating-point values from the first source operand to the second source
 * operand, and stores the packed double-precision floating-point results in
 * the destination
 */
extern __m256 __cdecl _mm256_mul_ps(__m256, __m256);

/*
 * Bitwise Logical OR of Packed Double Precision Floating-Point Values
 * **** VORPD ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical OR of the four packed double-precision
 * floating-point values from the first source operand and the second
 * source operand, and stores the result in the destination
 */
extern __m256d __cdecl _mm256_or_pd(__m256d, __m256d);

/*
 * Bitwise Logical OR of Packed Single Precision Floating-Point Values
 * **** VORPS ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical OR of the eight packed single-precision
 * floating-point values from the first source operand and the second
 * source operand, and stores the result in the destination
 */
extern __m256 __cdecl _mm256_or_ps(__m256, __m256);

/*
 * Shuffle Packed Double Precision Floating-Point Values
 * **** VSHUFPD ymm1, ymm2, ymm3/m256, imm8
 * Moves either of the two packed double-precision floating-point values from
 * each double quadword in the first source operand into the low quadword
 * of each double quadword of the destination; moves either of the two packed
 * double-precision floating-point values from the second source operand into
 * the high quadword of each double quadword of the destination operand.
 * The selector operand determines which values are moved to the destination
 */
extern __m256d __cdecl _mm256_shuffle_pd(__m256d, __m256d, const int);

/*
 * Shuffle Packed Single Precision Floating-Point Values
 * **** VSHUFPS ymm1, ymm2, ymm3/m256, imm8
 * Moves two of the four packed single-precision floating-point values
 * from each double qword of the first source operand into the low
 * quadword of each double qword of the destination; moves two of the four
 * packed single-precision floating-point values from each double qword of
 * the second source operand into to the high quadword of each double qword
 * of the destination. The selector operand determines which values are moved
 * to the destination.
 */
extern __m256 __cdecl _mm256_shuffle_ps(__m256, __m256, const int);

/*
 * Subtract Packed Double Precision Floating-Point Values
 * **** VSUBPD ymm1, ymm2, ymm3/m256
 * Performs an SIMD subtract of the four packed double-precision floating-point
 * values of the second Source operand from the first Source operand, and
 * stores the packed double-precision floating-point results in the destination
 */
extern __m256d __cdecl _mm256_sub_pd(__m256d, __m256d);

/*
 * Subtract Packed Single Precision Floating-Point Values
 * **** VSUBPS ymm1, ymm2, ymm3/m256
 * Performs an SIMD subtract of the eight packed single-precision
 * floating-point values in the second Source operand from the First Source
 * operand, and stores the packed single-precision floating-point results in
 * the destination
 */
extern __m256 __cdecl _mm256_sub_ps(__m256, __m256);

/*
 * Bitwise Logical XOR of Packed Double Precision Floating-Point Values
 * **** VXORPD ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical XOR of the four packed double-precision
 * floating-point values from the first source operand and the second
 * source operand, and stores the result in the destination
 */
extern __m256d __cdecl _mm256_xor_pd(__m256d, __m256d);

/*
 * Bitwise Logical XOR of Packed Single Precision Floating-Point Values
 * **** VXORPS ymm1, ymm2, ymm3/m256
 * Performs a bitwise logical XOR of the eight packed single-precision
 * floating-point values from the first source operand and the second
 * source operand, and stores the result in the destination
 */
extern __m256 __cdecl _mm256_xor_ps(__m256, __m256);

/*
 * Compare Packed Double-Precision Floating-Point Values
 * **** VCMPPD xmm1, xmm2, xmm3/m128, imm8
 * **** VCMPPD ymm1, ymm2, ymm3/m256, imm8
 * Performs an SIMD compare of the four packed double-precision floating-point
 * values in the second source operand (third operand) and the first source
 * operand (second operand) and returns the results of the comparison to the
 * destination operand (first operand). The comparison predicate operand
 * (immediate) specifies the type of comparison performed on each of the pairs
 * of packed values.
 * For 128-bit intrinsic function with compare predicate values in range 0-7
 * compiler may generate SSE2 instructions if it is warranted for performance
 * reasons.
 */
extern __m128d __cdecl _mm_cmp_pd(__m128d, __m128d, const int);
extern __m256d __cdecl _mm256_cmp_pd(__m256d, __m256d, const int);

/*
 * Compare Packed Single-Precision Floating-Point Values
 * **** VCMPPS xmm1, xmm2, xmm3/m256, imm8
 * **** VCMPPS ymm1, ymm2, ymm3/m256, imm8
 * Performs a SIMD compare of the packed single-precision floating-point values
 * in the second source operand (third operand) and the first source operand
 * (second operand) and returns the results of the comparison to the
 * destination operand (first operand). The comparison predicate operand
 * (immediate) specifies the type of comparison performed on each of the pairs
 * of packed values.
 * For 128-bit intrinsic function with compare predicate values in range 0-7
 * compiler may generate SSE2 instructions if it is warranted for performance
 * reasons.
 */
extern __m128 __cdecl _mm_cmp_ps(__m128, __m128, const int);
extern __m256 __cdecl _mm256_cmp_ps(__m256, __m256, const int);

/*
 * Compare Scalar Double-Precision Floating-Point Values
 * **** VCMPSD xmm1, xmm2, xmm3/m64, imm8
 * Compares the low double-precision floating-point values in the second source
 * operand (third operand) and the first source operand (second operand) and
 * returns the results in of the comparison to the destination operand (first
 * operand). The comparison predicate operand (immediate operand) specifies the
 * type of comparison performed.
 * For compare predicate values in range 0-7 compiler may generate SSE2
 * instructions if it is warranted for performance reasons.
 */
extern __m128d __cdecl _mm_cmp_sd(__m128d, __m128d, const int);

/* Compare Scalar Double-Precision Floating-Point Values with Integer Result
 * This is similar to _mm_cmp_sd, except it returns the result as an integer
 * and it supports all predicate values even when AVX support is not available.
 */
extern int __cdecl _mm_comi_sd(__m128d, __m128d, const int);

/*
 * Compare Scalar Single-Precision Floating-Point Values
 * **** VCMPSS xmm1, xmm2, xmm3/m64, imm8
 * Compares the low single-precision floating-point values in the second source
 * operand (third operand) and the first source operand (second operand) and
 * returns the results of the comparison to the destination operand (first
 * operand). The comparison predicate operand (immediate operand) specifies
 * the type of comparison performed.
 * For compare predicate values in range 0-7 compiler may generate SSE2
 * instructions if it is warranted for performance reasons.
 */
extern __m128 __cdecl _mm_cmp_ss(__m128, __m128, const int);

/* Compare Scalar Single-Precision Floating-Point Values with Integer Result
 * This is similar to _mm_cmp_ss, except it returns the result as an integer
 * and it supports all predicate values even when AVX support is not available.
 */
extern int __cdecl _mm_comi_ss(__m128, __m128, const int);

/*
 * Convert Packed Doubleword Integers to
 * Packed Double-Precision Floating-Point Values
 * **** VCVTDQ2PD ymm1, xmm2/m128
 * Converts four packed signed doubleword integers in the source operand to
 * four packed double-precision floating-point values in the destination
 */
extern __m256d __cdecl _mm256_cvtepi32_pd(__m128i);

/*
 * Convert Packed Doubleword Integers to
 * Packed Single-Precision Floating-Point Values
 * **** VCVTDQ2PS ymm1, ymm2/m256
 * Converts eight packed signed doubleword integers in the source operand to
 * eight packed double-precision floating-point values in the destination
 */
extern __m256  __cdecl _mm256_cvtepi32_ps(__m256i);

/*
 * Convert Packed Double-Precision Floating-point values to
 * Packed Single-Precision Floating-Point Values
 * **** VCVTPD2PS xmm1, ymm2/m256
 * Converts four packed double-precision floating-point values in the source
 * operand to four packed single-precision floating-point values in the
 * destination
 */
extern __m128  __cdecl _mm256_cvtpd_ps(__m256d);

/*
 * Convert Packed Single Precision Floating-Point Values to
 * Packed Singed Doubleword Integer Values
 * **** VCVTPS2DQ ymm1, ymm2/m256
 * Converts eight packed single-precision floating-point values in the source
 * operand to eight signed doubleword integers in the destination
 */
extern __m256i __cdecl _mm256_cvtps_epi32(__m256);

/*
 * Convert Packed Single Precision Floating-point values to
 * Packed Double Precision Floating-Point Values
 * **** VCVTPS2PD ymm1, xmm2/m128
 * Converts four packed single-precision floating-point values in the source
 * operand to four packed double-precision floating-point values in the
 * destination
 */
extern __m256d __cdecl _mm256_cvtps_pd(__m128);

/*
 * Convert with Truncation Packed Double-Precision Floating-Point values to
 * Packed Doubleword Integers
 * **** VCVTTPD2DQ xmm1, ymm2/m256
 * Converts four packed double-precision floating-point values in the source
 * operand to four packed signed doubleword integers in the destination.
 * When a conversion is inexact, a truncated (round toward zero) value is
 * returned. If a converted result is larger than the maximum signed doubleword
 * integer, the floating-point invalid exception is raised, and if this
 * exception is masked, the indefinite integer value (80000000H) is returned
*/
extern __m128i __cdecl _mm256_cvttpd_epi32(__m256d);

/*
 * Convert Packed Double-Precision Floating-point values to
 * Packed Doubleword Integers
 * **** VCVTPD2DQ xmm1, ymm2/m256
 * Converts four packed double-precision floating-point values in the source
 * operand to four packed signed doubleword integers in the destination
 */
extern __m128i __cdecl _mm256_cvtpd_epi32(__m256d);

/*
 * Convert with Truncation Packed Single Precision Floating-Point Values to
 * Packed Singed Doubleword Integer Values
 * **** VCVTTPS2DQ ymm1, ymm2/m256
 * Converts eight packed single-precision floating-point values in the source
 * operand to eight signed doubleword integers in the destination.
 * When a conversion is inexact, a truncated (round toward zero) value is
 * returned. If a converted result is larger than the maximum signed doubleword
 * integer, the floating-point invalid exception is raised, and if this
 * exception is masked, the indefinite integer value (80000000H) is returned
 */
extern __m256i __cdecl _mm256_cvttps_epi32(__m256);

/*
 * Convert Scalar Single-Precision Floating-point value in 256-bit vector to
 * equivalent C/C++ float type.
 */
// float _mm256_cvtss_f32 (__m256 a)
#define _mm256_cvtss_f32(a) (_mm_cvtss_f32(_mm256_castps256_ps128(a)))

/*
 * Convert Scalar Double-Precision Floating-point value in 256-bit vector to
 * equivalent C/C++ double type.
 */
// double _mm256_cvtsd_f64 (__m256d a)
#define _mm256_cvtsd_f64(a) (_mm_cvtsd_f64(_mm256_castpd256_pd128(a)))

/*
 * Convert 32-bit Scalar integer in 256-bit vector to equivalent C/C++ int type.
 */
// int _mm256_cvtsi256_si32 (__m256i a)
#define _mm256_cvtsi256_si32(a) (_mm_cvtsi128_si32(_mm256_castsi256_si128(a)))

#if defined (_M_X64)
/*
 * Convert 64-bit Scalar integer in 256-bit vector to equivalent C/C++ __int64 type.
 */
// __int64 _mm256_cvtsi256_si64 (__m256i a)
#define _mm256_cvtsi256_si64(a) (_mm_cvtsi128_si64(_mm256_castsi256_si128(a)))
#endif  /* defined (_M_X64) */


/*
 * Extract packed floating-point values
 * **** VEXTRACTF128 xmm1/m128, ymm2, imm8
 * Extracts 128-bits of packed floating-point values from the source operand
 * at an 128-bit offset from imm8[0] into the destination
 */
extern __m128  __cdecl _mm256_extractf128_ps(__m256, const int);
extern __m128d __cdecl _mm256_extractf128_pd(__m256d, const int);
extern __m128i __cdecl _mm256_extractf128_si256(__m256i, const int);

/*
 * Zero All YMM registers
 * **** VZEROALL
 * Zeros contents of all YMM registers
 */
extern void __cdecl _mm256_zeroall(void);

/*
 * Zero Upper bits of YMM registers
 * **** VZEROUPPER
 * Zeros the upper 128 bits of all YMM registers. The lower 128-bits of the
 * registers (the corresponding XMM registers) are unmodified
 */
extern void __cdecl _mm256_zeroupper(void);

/*
 * Permute Single-Precision Floating-Point Values
 * **** VPERMILPS ymm1, ymm2, ymm3/m256
 * **** VPERMILPS xmm1, xmm2, xmm3/m128
 * Permute Single-Precision Floating-Point values in the first source operand
 * using 8-bit control fields in the low bytes of corresponding elements the
 * shuffle control and store results in the destination
 */
extern __m256  __cdecl _mm256_permutevar_ps(__m256, __m256i);
extern __m128  __cdecl _mm_permutevar_ps(__m128, __m128i);

/*
 * Permute Single-Precision Floating-Point Values
 * **** VPERMILPS ymm1, ymm2/m256, imm8
 * **** VPERMILPS xmm1, xmm2/m128, imm8
 * Permute Single-Precision Floating-Point values in the first source operand
 * using four 2-bit control fields in the 8-bit immediate and store results
 * in the destination
 */
extern __m256  __cdecl _mm256_permute_ps(__m256, int);
extern __m128  __cdecl _mm_permute_ps(__m128, int);

/*
 * Permute Double-Precision Floating-Point Values
 * **** VPERMILPD ymm1, ymm2, ymm3/m256
 * **** VPERMILPD xmm1, xmm2, xmm3/m128
 * Permute Double-Precision Floating-Point values in the first source operand
 * using 8-bit control fields in the low bytes of the second source operand
 * and store results in the destination
 */
extern __m256d __cdecl _mm256_permutevar_pd(__m256d, __m256i);
extern __m128d __cdecl _mm_permutevar_pd(__m128d, __m128i);

/*
 * Permute Double-Precision Floating-Point Values
 * **** VPERMILPD ymm1, ymm2/m256, imm8
 * **** VPERMILPD xmm1, xmm2/m128, imm8
 * Permute Double-Precision Floating-Point values in the first source operand
 * using two, 1-bit control fields in the low 2 bits of the 8-bit immediate
 * and store results in the destination
 */
extern __m256d __cdecl _mm256_permute_pd(__m256d, int);
extern __m128d __cdecl _mm_permute_pd(__m128d, int);

/*
 * Permute Floating-Point Values
 * **** VPERM2F128 ymm1, ymm2, ymm3/m256, imm8
 * Permute 128 bit floating-point-containing fields from the first source
 * operand and second source operand using bits in the 8-bit immediate and
 * store results in the destination
 */
extern __m256  __cdecl _mm256_permute2f128_ps(__m256, __m256, int);
extern __m256d __cdecl _mm256_permute2f128_pd(__m256d, __m256d, int);
extern __m256i __cdecl _mm256_permute2f128_si256(__m256i, __m256i, int);

/*
 * Load with Broadcast
 * **** VBROADCASTSS ymm1, m32
 * **** VBROADCASTSS xmm1, m32
 * Load floating point values from the source operand and broadcast to all
 * elements of the destination
 */
extern __m256  __cdecl _mm256_broadcast_ss(float const *);
extern __m128  __cdecl _mm_broadcast_ss(float const *);

/*
 * Load with Broadcast
 * **** VBROADCASTSD ymm1, m64
 * Load floating point values from the source operand and broadcast to all
 * elements of the destination
 */
extern __m256d __cdecl _mm256_broadcast_sd(double const *);

/*
 * Load with Broadcast
 * **** VBROADCASTF128 ymm1, m128
 * Load floating point values from the source operand and broadcast to all
 * elements of the destination
 */
extern __m256  __cdecl _mm256_broadcast_ps(__m128 const *);
extern __m256d __cdecl _mm256_broadcast_pd(__m128d const *);

/*
 * Insert packed floating-point values
 * **** VINSERTF128 ymm1, ymm2, xmm3/m128, imm8
 * Performs an insertion of 128-bits of packed floating-point values from the
 * second source operand into an the destination at an 128-bit offset from
 * imm8[0]. The remaining portions of the destination are written by the
 * corresponding fields of the first source operand
 */
extern __m256  __cdecl _mm256_insertf128_ps(__m256, __m128, int);
extern __m256d __cdecl _mm256_insertf128_pd(__m256d, __m128d, int);
extern __m256i __cdecl _mm256_insertf128_si256(__m256i, __m128i, int);

/*
 * Move Aligned Packed Double-Precision Floating-Point Values
 * **** VMOVAPD ymm1, m256
 * **** VMOVAPD m256, ymm1
 * Moves 4 double-precision floating-point values from the source operand to
 * the destination
 */
extern __m256d __cdecl _mm256_load_pd(double const *);
extern void    __cdecl _mm256_store_pd(double *, __m256d);

/*
 * Move Aligned Packed Single-Precision Floating-Point Values
 * **** VMOVAPS ymm1, m256
 * **** VMOVAPS m256, ymm1
 * Moves 8 single-precision floating-point values from the source operand to
 * the destination
 */
extern __m256  __cdecl _mm256_load_ps(float const *);
extern void    __cdecl _mm256_store_ps(float *, __m256);

/*
 * Move Unaligned Packed Double-Precision Floating-Point Values
 * **** VMOVUPD ymm1, m256
 * **** VMOVUPD m256, ymm1
 * Moves 256 bits of packed double-precision floating-point values from the
 * source operand to the destination
 */
extern __m256d __cdecl _mm256_loadu_pd(double const *);
extern void    __cdecl _mm256_storeu_pd(double *, __m256d);

/*
 * Move Unaligned Packed Single-Precision Floating-Point Values
 * **** VMOVUPS ymm1, m256
 * **** VMOVUPS m256, ymm1
 * Moves 256 bits of packed single-precision floating-point values from the
 * source operand to the destination
 */
extern __m256  __cdecl _mm256_loadu_ps(float const *);
extern void    __cdecl _mm256_storeu_ps(float *, __m256);

/*
 * Move Aligned Packed Integer Values
 * **** VMOVDQA ymm1, m256
 * **** VMOVDQA m256, ymm1
 * Moves 256 bits of packed integer values from the source operand to the
 * destination
 */
extern __m256i __cdecl _mm256_load_si256(__m256i const *);
extern void    __cdecl _mm256_store_si256(__m256i *, __m256i);

/*
 * Move Unaligned Packed Integer Values
 * **** VMOVDQU ymm1, m256
 * **** VMOVDQU m256, ymm1
 * Moves 256 bits of packed integer values from the source operand to the
 * destination
 */
extern __m256i __cdecl _mm256_loadu_si256(__m256i const *);
extern void    __cdecl _mm256_storeu_si256(__m256i *, __m256i);

/*
 * Load Two Unaligned Packed 128-bit Values
 * Loads two potentially unaligned 128-bit values
 * and combines them into one 256-bit value.
 *
 * The data types here (float const*, double const* and __m128i const*)
 * were chosen for consistency with the underlying _mm_loadu_{ps,pd,si128}
 * intrinsics.
 */

#define _mm256_loadu2_m128(/* float const* */ hiaddr, \
                           /* float const* */ loaddr) \
    _mm256_set_m128(_mm_loadu_ps(hiaddr), _mm_loadu_ps(loaddr))

#define _mm256_loadu2_m128d(/* double const* */ hiaddr, \
                            /* double const* */ loaddr) \
    _mm256_set_m128d(_mm_loadu_pd(hiaddr), _mm_loadu_pd(loaddr))

#define _mm256_loadu2_m128i(/* __m128i const* */ hiaddr, \
                            /* __m128i const* */ loaddr) \
    _mm256_set_m128i(_mm_loadu_si128(hiaddr), _mm_loadu_si128(loaddr))

/*
 * Store 256-bit Value To Two Unaligned 128-bit Locations
 * Stores the high and low 128-bit halves of a 256-bit value
 * to two different potentially unaligned addresses.
 */

#define _mm256_storeu2_m128(/* float* */ hiaddr, /* float* */ loaddr, \
                            /* __m256 */ a) \
    do { \
        __m256 _a = (a); /* reference a only once in macro body */ \
        _mm_storeu_ps((loaddr), _mm256_castps256_ps128(_a)); \
        _mm_storeu_ps((hiaddr), _mm256_extractf128_ps(_a, 0x1)); \
    } while (0)

#define _mm256_storeu2_m128d(/* double* */ hiaddr, /* double* */ loaddr, \
                             /* __m256d */ a) \
    do { \
        __m256d _a = (a); /* reference a only once in macro body */ \
        _mm_storeu_pd((loaddr), _mm256_castpd256_pd128(_a)); \
        _mm_storeu_pd((hiaddr), _mm256_extractf128_pd(_a, 0x1)); \
    } while (0)

#define _mm256_storeu2_m128i(/* __m128i* */ hiaddr, /* __m128i* */ loaddr, \
                             /* __m256i */ a) \
    do { \
        __m256i _a = (a); /* reference a only once in macro body */ \
        _mm_storeu_si128((loaddr), _mm256_castsi256_si128(_a)); \
        _mm_storeu_si128((hiaddr), _mm256_extractf128_si256(_a, 0x1)); \
    } while (0)

/*
 * Conditional SIMD Packed Loads and Stores
 * **** VMASKMOVPD xmm1, xmm2, m128
 * **** VMASKMOVPD ymm1, ymm2, m256
 * **** VMASKMOVPD m128, xmm1, xmm2
 * **** VMASKMOVPD m256, ymm1, ymm2
 *
 * Load forms:
 * Load packed values from the 128-bit (XMM forms) or 256-bit (YMM forms)
 * memory location (third operand) into the destination XMM or YMM register
 * (first operand) using a mask in the first source operand (second operand).
 *
 * Store forms:
 * Stores packed values from the XMM or YMM register in the second source
 * operand (third operand) into the 128-bit (XMM forms) or 256-bit (YMM forms)
 * memory location using a mask in first source operand (second operand).
 * Stores are atomic.
 */
extern __m256d __cdecl _mm256_maskload_pd(double const *, __m256i);
extern void    __cdecl _mm256_maskstore_pd(double *, __m256i, __m256d);
extern __m128d __cdecl _mm_maskload_pd(double const *, __m128i);
extern void    __cdecl _mm_maskstore_pd(double *, __m128i, __m128d);

/*
 * Conditional SIMD Packed Loads and Stores
 * **** VMASKMOVPS xmm1, xmm2, m128
 * **** VMASKMOVPS ymm1, ymm2, m256
 * **** VMASKMOVPS m128, xmm1, xmm2
 * **** VMASKMOVPS m256, ymm1, ymm2
 *
 * Load forms:
 * Load packed values from the 128-bit (XMM forms) or 256-bit (YMM forms)
 * memory location (third operand) into the destination XMM or YMM register
 * (first operand) using a mask in the first source operand (second operand).
 *
 * Store forms:
 * Stores packed values from the XMM or YMM register in the second source
 * operand (third operand) into the 128-bit (XMM forms) or 256-bit (YMM forms)
 * memory location using a mask in first source operand (second operand).
 * Stores are atomic.
 */
extern __m256  __cdecl _mm256_maskload_ps(float const *, __m256i);
extern void    __cdecl _mm256_maskstore_ps(float *, __m256i, __m256);
extern __m128  __cdecl _mm_maskload_ps(float const *, __m128i);
extern void    __cdecl _mm_maskstore_ps(float *, __m128i, __m128);

/*
 * Replicate Single-Precision Floating-Point Values
 * **** VMOVSHDUP ymm1, ymm2/m256
 * Duplicates odd-indexed single-precision floating-point values from the
 * source operand
 */
extern __m256  __cdecl _mm256_movehdup_ps(__m256);

/*
 * Replicate Single-Precision Floating-Point Values
 * **** VMOVSLDUP ymm1, ymm2/m256
 * Duplicates even-indexed single-precision floating-point values from the
 * source operand
 */
extern __m256  __cdecl _mm256_moveldup_ps(__m256);

/*
 * Replicate Double-Precision Floating-Point Values
 * **** VMOVDDUP ymm1, ymm2/m256
 * Duplicates even-indexed double-precision floating-point values from the
 * source operand
 */
extern __m256d __cdecl _mm256_movedup_pd(__m256d);

/*
 * Move Unaligned Integer
 * **** VLDDQU ymm1, m256
 * The instruction is functionally similar to VMOVDQU YMM, m256 for loading
 * from memory. That is: 32 bytes of data starting at an address specified by
 * the source memory operand are fetched from memory and placed in a
 * destination
 */
extern __m256i __cdecl _mm256_lddqu_si256(__m256i const *);

/*
 * Store Packed Integers Using Non-Temporal Hint
 * **** VMOVNTDQ m256, ymm1
 * Moves the packed integers in the source operand to the destination using a
 * non-temporal hint to prevent caching of the data during the write to memory
 */
extern void    __cdecl _mm256_stream_si256(__m256i *, __m256i);

/*
 * Store Packed Double-Precision Floating-Point Values Using Non-Temporal Hint
 * **** VMOVNTPD m256, ymm1
 * Moves the packed double-precision floating-point values in the source
 * operand to the destination operand using a non-temporal hint to prevent
 * caching of the data during the write to memory
 */
extern void    __cdecl _mm256_stream_pd(double *, __m256d);

/*
 * Store Packed Single-Precision Floating-Point Values Using Non-Temporal Hint
 * **** VMOVNTPS m256, ymm1
 * Moves the packed single-precision floating-point values in the source
 * operand to the destination operand using a non-temporal hint to prevent
 * caching of the data during the write to memory
 */
extern void    __cdecl _mm256_stream_ps(float *, __m256);

/*
 * Compute Approximate Reciprocals of Packed Single-Precision Floating-Point
 * Values
 * **** VRCPPS ymm1, ymm2/m256
 * Performs an SIMD computation of the approximate reciprocals of the eight
 * packed single precision floating-point values in the source operand and
 * stores the packed single-precision floating-point results in the destination
 */
extern __m256  __cdecl _mm256_rcp_ps(__m256);

/*
 * Compute Approximate Reciprocals of Square Roots of
 * Packed Single-Precision Floating-point Values
 * **** VRSQRTPS ymm1, ymm2/m256
 * Performs an SIMD computation of the approximate reciprocals of the square
 * roots of the eight packed single precision floating-point values in the
 * source operand and stores the packed single-precision floating-point results
 * in the destination
 */
extern __m256  __cdecl _mm256_rsqrt_ps(__m256);

/*
 * Square Root of Double-Precision Floating-Point Values
 * **** VSQRTPD ymm1, ymm2/m256
 * Performs an SIMD computation of the square roots of the two or four packed
 * double-precision floating-point values in the source operand and stores
 * the packed double-precision floating-point results in the destination
 */
extern __m256d __cdecl _mm256_sqrt_pd(__m256d);

/*
 * Square Root of Single-Precision Floating-Point Values
 * **** VSQRTPS ymm1, ymm2/m256
 * Performs an SIMD computation of the square roots of the eight packed
 * single-precision floating-point values in the source operand stores the
 * packed double-precision floating-point results in the destination
 */
extern __m256  __cdecl _mm256_sqrt_ps(__m256);

/*
 * Round Packed Double-Precision Floating-Point Values
 * **** VROUNDPD ymm1,ymm2/m256,imm8
 * Round the four Double-Precision Floating-Point Values values in the source
 * operand by the rounding mode specified in the immediate operand and place
 * the result in the destination. The rounding process rounds the input to an
 * integral value and returns the result as a double-precision floating-point
 * value. The Precision Floating Point Exception is signaled according to the
 * immediate operand. If any source operand is an SNaN then it will be
 * converted to a QNaN.
 */
extern __m256d __cdecl _mm256_round_pd(__m256d, int);
#define _mm256_ceil_pd(val)   _mm256_round_pd((val), _MM_FROUND_CEIL)
#define _mm256_floor_pd(val)  _mm256_round_pd((val), _MM_FROUND_FLOOR)

/*
 * Round Packed Single-Precision Floating-Point Values
 * **** VROUNDPS ymm1,ymm2/m256,imm8
 * Round the four single-precision floating-point values values in the source
 * operand by the rounding mode specified in the immediate operand and place
 * the result in the destination. The rounding process rounds the input to an
 * integral value and returns the result as a double-precision floating-point
 * value. The Precision Floating Point Exception is signaled according to the
 * immediate operand. If any source operand is an SNaN then it will be
 * converted to a QNaN.
 */
extern __m256  __cdecl _mm256_round_ps(__m256, int);
#define _mm256_ceil_ps(val)   _mm256_round_ps((val), _MM_FROUND_CEIL)
#define _mm256_floor_ps(val)  _mm256_round_ps((val), _MM_FROUND_FLOOR)

/*
 * Unpack and Interleave High Packed Double-Precision Floating-Point Values
 * **** VUNPCKHPD ymm1,ymm2,ymm3/m256
 * Performs an interleaved unpack of the high double-precision floating-point
 * values from the first source operand and the second source operand.
 */
extern __m256d __cdecl _mm256_unpackhi_pd(__m256d, __m256d);

/*
 * Unpack and Interleave High Packed Single-Precision Floating-Point Values
 * **** VUNPCKHPS ymm1,ymm2,ymm3
 * Performs an interleaved unpack of the high single-precision floating-point
 * values from the first source operand and the second source operand
 */
extern __m256  __cdecl _mm256_unpackhi_ps(__m256, __m256);

/*
 * Unpack and Interleave Low Packed Double-Precision Floating-Point Values
 * **** VUNPCKLPD ymm1,ymm2,ymm3/m256
 * Performs an interleaved unpack of the low double-precision floating-point
 * values from the first source operand and the second source operand
 */
extern __m256d __cdecl _mm256_unpacklo_pd(__m256d, __m256d);

/*
 * Unpack and Interleave Low Packed Single-Precision Floating-Point Values
 * **** VUNPCKLPS ymm1,ymm2,ymm3
 * Performs an interleaved unpack of the low single-precision floating-point
 * values from the first source operand and the second source operand
 */
extern __m256  __cdecl _mm256_unpacklo_ps(__m256, __m256);

/*
 * Packed Bit Test
 * **** VPTEST ymm1, ymm2/m256
 * VPTEST set the ZF flag if all bits in the result are 0 of the bitwise AND
 * of the first source operand and the second source operand. VPTEST sets the
 * CF flag if all bits in the result are 0 of the bitwise AND of the second
 * source operand and the logical NOT of the first source operand.
 */
extern int     __cdecl _mm256_testz_si256(__m256i, __m256i);
#define _mm256_test_all_zeros(mask, val) \
              _mm256_testz_si256((mask), (val))

extern int     __cdecl _mm256_testc_si256(__m256i, __m256i);
#define _mm256_test_all_ones(val) \
              _mm256_testc_si256((val), _mm256_cmpeq_epi32((val),(val)))

extern int     __cdecl _mm256_testnzc_si256(__m256i, __m256i);
#define _mm256_test_mix_ones_zeros(mask, val) \
              _mm256_testnzc_si256((mask), (val))

/*
 * Packed Bit Test
 * **** VTESTPD ymm1, ymm2/m256
 * **** VTESTPD xmm1, xmm2/m128
 * VTESTPD performs a bitwise comparison of all the sign bits of the
 * double-precision elements in the first source operation and corresponding
 * sign bits in the second source operand. If the AND of the two sets of bits
 * produces all zeros, the ZF is set else the ZF is clear. If the AND NOT of
 * the source sign bits with the dest sign bits produces all zeros the CF is
 * set else the CF is clear
 */
extern int     __cdecl _mm256_testz_pd(__m256d, __m256d);
extern int     __cdecl _mm256_testc_pd(__m256d, __m256d);
extern int     __cdecl _mm256_testnzc_pd(__m256d, __m256d);
extern int     __cdecl _mm_testz_pd(__m128d, __m128d);
extern int     __cdecl _mm_testc_pd(__m128d, __m128d);
extern int     __cdecl _mm_testnzc_pd(__m128d, __m128d);

/*
 * Packed Bit Test
 * **** VTESTPS ymm1, ymm2/m256
 * **** VTESTPS xmm1, xmm2/m128
 * VTESTPS performs a bitwise comparison of all the sign bits of the packed
 * single-precision elements in the first source operation and corresponding
 * sign bits in the second source operand. If the AND of the two sets of bits
 * produces all zeros, the ZF is set else the ZF is clear. If the AND NOT of
 * the source sign bits with the dest sign bits produces all zeros the CF is
 * set else the CF is clear
 */
extern int     __cdecl _mm256_testz_ps(__m256, __m256);
extern int     __cdecl _mm256_testc_ps(__m256, __m256);
extern int     __cdecl _mm256_testnzc_ps(__m256, __m256);
extern int     __cdecl _mm_testz_ps(__m128, __m128);
extern int     __cdecl _mm_testc_ps(__m128, __m128);
extern int     __cdecl _mm_testnzc_ps(__m128, __m128);

/*
 * Extract Double-Precision Floating-Point Sign mask
 * **** VMOVMSKPD r32, ymm2
 * Extracts the sign bits from the packed double-precision floating-point
 * values in the source operand, formats them into a 4-bit mask, and stores
 * the mask in the destination
 */
extern int     __cdecl _mm256_movemask_pd(__m256d);

/*
 * Extract Single-Precision Floating-Point Sign mask
 * **** VMOVMSKPS r32, ymm2
 * Extracts the sign bits from the packed single-precision floating-point
 * values in the source operand, formats them into a 8-bit mask, and stores
 * the mask in the destination
 */
extern int     __cdecl _mm256_movemask_ps(__m256);

/*
 * Return 256-bit vector with all elements set to 0
 */
extern __m256d __cdecl _mm256_setzero_pd(void);
extern __m256  __cdecl _mm256_setzero_ps(void);
extern __m256i __cdecl _mm256_setzero_si256(void);

/*
 * Return 256-bit vector initialized to specified arguments
 */
extern __m256d __cdecl _mm256_set_pd(double, double, double, double);
extern __m256  __cdecl _mm256_set_ps(float, float, float, float,
                                            float, float, float, float);
extern __m256i __cdecl _mm256_set_epi8(char, char, char, char,
                                              char, char, char, char,
                                              char, char, char, char,
                                              char, char, char, char,
                                              char, char, char, char,
                                              char, char, char, char,
                                              char, char, char, char,
                                              char, char, char, char);
extern __m256i __cdecl _mm256_set_epi16(short, short, short, short,
                                               short, short, short, short,
                                               short, short, short, short,
                                               short, short, short, short);
extern __m256i __cdecl _mm256_set_epi32(int, int, int, int,
                                               int, int, int, int);
extern __m256i __cdecl _mm256_set_epi64x(__int64, __int64,
                                                __int64, __int64);

#define _mm256_set_m128(/* __m128 */ hi, /* __m128 */ lo) \
    _mm256_insertf128_ps(_mm256_castps128_ps256(lo), (hi), 0x1)

#define _mm256_set_m128d(/* __m128d */ hi, /* __m128d */ lo) \
    _mm256_insertf128_pd(_mm256_castpd128_pd256(lo), (hi), 0x1)

#define _mm256_set_m128i(/* __m128i */ hi, /* __m128i */ lo) \
    _mm256_insertf128_si256(_mm256_castsi128_si256(lo), (hi), 0x1)

extern __m256d __cdecl _mm256_setr_pd(double, double, double, double);
extern __m256  __cdecl _mm256_setr_ps(float, float, float, float,
                                             float, float, float, float);
extern __m256i __cdecl _mm256_setr_epi8(char, char, char, char,
                                               char, char, char, char,
                                               char, char, char, char,
                                               char, char, char, char,
                                               char, char, char, char,
                                               char, char, char, char,
                                               char, char, char, char,
                                               char, char, char, char);
extern __m256i __cdecl _mm256_setr_epi16(short, short, short, short,
                                                short, short, short, short,
                                                short, short, short, short,
                                                short, short, short, short);
extern __m256i __cdecl _mm256_setr_epi32(int, int, int, int,
                                                int, int, int, int);
extern __m256i __cdecl _mm256_setr_epi64x(__int64, __int64,
                                                 __int64, __int64);
#define _mm256_setr_m128(lo, hi)    _mm256_set_m128((hi), (lo))
#define _mm256_setr_m128d(lo, hi)   _mm256_set_m128d((hi), (lo))
#define _mm256_setr_m128i(lo, hi)   _mm256_set_m128i((hi), (lo))

/*
 * Return 256-bit vector with all elements initialized to specified scalar
 */
extern __m256d __cdecl _mm256_set1_pd(double);
extern __m256  __cdecl _mm256_set1_ps(float);
extern __m256i __cdecl _mm256_set1_epi8(char);
extern __m256i __cdecl _mm256_set1_epi16(short);
extern __m256i __cdecl _mm256_set1_epi32(int);
extern __m256i __cdecl _mm256_set1_epi64x(long long);

/*
 * Support intrinsic functions to do vector type casts. These functions do
 * not introduce extra moves to generated code. When cast is done from a 128
 * to 256-bit type the low 128 bits of the 256-bit result contain source
 * parameter value; the upper 128 bits of the result are undefined.
 */
extern __m256  __cdecl _mm256_castpd_ps(__m256d);
extern __m256d __cdecl _mm256_castps_pd(__m256);
extern __m256i __cdecl _mm256_castps_si256(__m256);
extern __m256i __cdecl _mm256_castpd_si256(__m256d);
extern __m256  __cdecl _mm256_castsi256_ps(__m256i);
extern __m256d __cdecl _mm256_castsi256_pd(__m256i);
extern __m128  __cdecl _mm256_castps256_ps128(__m256);
extern __m128d __cdecl _mm256_castpd256_pd128(__m256d);
extern __m128i __cdecl _mm256_castsi256_si128(__m256i);
extern __m256  __cdecl _mm256_castps128_ps256(__m128);
extern __m256d __cdecl _mm256_castpd128_pd256(__m128d);
extern __m256i __cdecl _mm256_castsi128_si256(__m128i);


/*
 * Support for half-float conversions to/from normal float.
 * Immediate argument is used for special MXCSR overrides.
 */
extern __m128  __cdecl _mm_cvtph_ps(__m128i);
extern __m256  __cdecl _mm256_cvtph_ps(__m128i);
extern __m128i __cdecl _mm_cvtps_ph(__m128 /* m1 */, const int /* imm */);
extern __m128i __cdecl _mm256_cvtps_ph(__m256, int);

/*
 * Return a vector with all elements set to zero. It is recommended to use the
 * result of this intrinsic as an input argument to another intrinsic when the
 * initial value is irrelevant.
 */
#define _mm_undefined_ps       _mm_setzero_ps
#define _mm_undefined_pd       _mm_setzero_pd
#define _mm_undefined_si128    _mm_setzero_si128
#define _mm256_undefined_ps    _mm256_setzero_ps
#define _mm256_undefined_pd    _mm256_setzero_pd
#define _mm256_undefined_si256 _mm256_setzero_si256

/*
 * The list of extended control registers.
 * Currently, the list includes only one register.
 */
#define _XCR_XFEATURE_ENABLED_MASK 0

/* Returns the content of the specified extended control register */
extern unsigned __int64 __cdecl _xgetbv(unsigned int);

/* Writes the value to the specified extended control register */
extern void __cdecl _xsetbv(unsigned int, unsigned __int64);


/*
 * Performs a full or partial save of the enabled processor state components
 * using the specified memory address location and a mask.
 */
extern void __cdecl _xsave(void *, unsigned __int64);
#if defined (_M_X64)
extern void __cdecl _xsave64(void *, unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Performs a full or partial save of the enabled processor state components
 * using the specified memory address location and a mask.
 * Optimize the state save operation if possible.
 */
extern void __cdecl _xsaveopt(void *, unsigned __int64);
#if defined (_M_X64)
extern void __cdecl _xsaveopt64(void *, unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Performs a full or compressed partial save of the enabled processor state
 * components using the specified memory address location and a mask.
 */
extern void __cdecl _xsavec(void *, unsigned __int64);
#if defined (_M_X64)
extern void __cdecl _xsavec64(void *, unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Performs a full or partial restore of the enabled processor states
 * using the state information stored in the specified memory address location
 * and a mask.
 */
extern void __cdecl _xrstor(void const *, unsigned __int64);
#if defined (_M_X64)
extern void __cdecl _xrstor64(void const *, unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Performs a full or partial save of the enabled processor extended
 * and supervisor state components in compacted form using the
 * specified memory address location and masks in XCR0 and IA32_XSS MSR.
 */
extern void __cdecl _xsaves(void *, unsigned __int64);
#if defined (_M_X64)
extern void __cdecl _xsaves64(void *, unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Performs a full or partial restore of the enabled processor extended
 * and supervisor states using the state information stored in the
 * specified memory address location and masks in XCR0 and IA32_XSS MSR.
 */
extern void __cdecl _xrstors(void const *, unsigned __int64);
#if defined (_M_X64)
extern void __cdecl _xrstors64(void const *, unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Saves the current state of the x87 FPU, MMX technology, XMM,
 * and MXCSR registers to the specified 512-byte memory location.
 */
extern void __cdecl _fxsave(void *);
#if defined (_M_X64)
extern void __cdecl _fxsave64(void *);
#endif  /* defined (_M_X64) */

/*
 * Restore the current state of the x87 FPU, MMX technology, XMM,
 * and MXCSR registers from the specified 512-byte memory location.
 */
extern void __cdecl _fxrstor(void const *);
#if defined (_M_X64)
extern void __cdecl _fxrstor64(void const *);
#endif  /* defined (_M_X64) */

/*
 * Perform one attempt to generate a hardware generated random value.
 * The generated value is written to the given memory location and the success
 * status is returned: 1 if the hardware could generate a valid random number
 * and 0 otherwise.
 */
extern int __cdecl _rdrand16_step(unsigned short *);
extern int __cdecl _rdrand32_step(unsigned int *);
#if defined (_M_X64)
extern int __cdecl _rdrand64_step(unsigned __int64 *);
#endif  /* defined (_M_X64) */

#if defined (_M_X64)
/*
 * Return the value of the FS/GS segment base register.
 */
extern unsigned int     __cdecl _readfsbase_u32(void);
extern unsigned int     __cdecl _readgsbase_u32(void);
extern unsigned __int64 __cdecl _readfsbase_u64(void);
extern unsigned __int64 __cdecl _readgsbase_u64(void);

/*
 * Write the value to the FS/GS segment base register.
 */
extern void __cdecl _writefsbase_u32(unsigned int);
extern void __cdecl _writegsbase_u32(unsigned int);
extern void __cdecl _writefsbase_u64(unsigned __int64);
extern void __cdecl _writegsbase_u64(unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Perform FMA (Fused Multiply-and-Add) operations.
 */
extern __m128  __cdecl _mm_fmadd_ps(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fmadd_pd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fmadd_ss(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fmadd_sd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fmsub_ps(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fmsub_pd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fmsub_ss(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fmsub_sd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fnmadd_ps(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fnmadd_pd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fnmadd_ss(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fnmadd_sd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fnmsub_ps(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fnmsub_pd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fnmsub_ss(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fnmsub_sd(__m128d, __m128d, __m128d);

extern __m256  __cdecl _mm256_fmadd_ps(__m256, __m256, __m256);
extern __m256d __cdecl _mm256_fmadd_pd(__m256d, __m256d, __m256d);
extern __m256  __cdecl _mm256_fmsub_ps(__m256, __m256, __m256);
extern __m256d __cdecl _mm256_fmsub_pd(__m256d, __m256d, __m256d);
extern __m256  __cdecl _mm256_fnmadd_ps(__m256, __m256, __m256);
extern __m256d __cdecl _mm256_fnmadd_pd(__m256d, __m256d, __m256d);
extern __m256  __cdecl _mm256_fnmsub_ps(__m256, __m256, __m256);
extern __m256d __cdecl _mm256_fnmsub_pd(__m256d, __m256d, __m256d);


/*
 * Fused Multiply-and-Add/Subtract__and Multiply-and-Subtract/Add operations.
 */
extern __m128  __cdecl _mm_fmaddsub_ps(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fmaddsub_pd(__m128d, __m128d, __m128d);
extern __m128  __cdecl _mm_fmsubadd_ps(__m128, __m128, __m128);
extern __m128d __cdecl _mm_fmsubadd_pd(__m128d, __m128d, __m128d);

extern __m256  __cdecl _mm256_fmaddsub_ps(__m256, __m256, __m256);
extern __m256d __cdecl _mm256_fmaddsub_pd(__m256d, __m256d, __m256d);
extern __m256  __cdecl _mm256_fmsubadd_ps(__m256, __m256, __m256);
extern __m256d __cdecl _mm256_fmsubadd_pd(__m256d, __m256d, __m256d);

/*
 * Scalar FP intrinsics (with double/float arguments)
 */
extern float  __cdecl __fmadd_ss(float, float, float);
extern double __cdecl __fmadd_sd(double, double, double);
extern float  __cdecl __fmsub_ss(float, float, float);
extern double __cdecl __fmsub_sd(double, double, double);
extern float  __cdecl __fnmadd_ss(float, float, float);
extern double __cdecl __fnmadd_sd(double, double, double);
extern float  __cdecl __fnmsub_ss(float, float, float);
extern double __cdecl __fnmsub_sd(double, double, double);

/*
 * Integer 256-bit vector comparison operations.
 */
extern __m256i __cdecl _mm256_cmpeq_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_cmpeq_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_cmpeq_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_cmpeq_epi64(__m256i, __m256i);

extern __m256i __cdecl _mm256_cmpgt_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_cmpgt_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_cmpgt_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_cmpgt_epi64(__m256i, __m256i);


/*
 * Integer 256-bit vector MIN/MAX operations.
 */
extern __m256i __cdecl _mm256_max_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_max_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_max_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_max_epu8(__m256i, __m256i);
extern __m256i __cdecl _mm256_max_epu16(__m256i, __m256i);
extern __m256i __cdecl _mm256_max_epu32(__m256i, __m256i);

extern __m256i __cdecl _mm256_min_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_min_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_min_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_min_epu8(__m256i, __m256i);
extern __m256i __cdecl _mm256_min_epu16(__m256i, __m256i);
extern __m256i __cdecl _mm256_min_epu32(__m256i, __m256i);


/*
 * Integer 256-bit vector logical operations.
 */
extern __m256i __cdecl _mm256_and_si256(__m256i, __m256i);
extern __m256i __cdecl _mm256_andnot_si256(__m256i, __m256i);
extern __m256i __cdecl _mm256_or_si256(__m256i, __m256i);
extern __m256i __cdecl _mm256_xor_si256(__m256i, __m256i);


/*
 * Integer 256-bit vector arithmetic operations.
 */
extern __m256i __cdecl _mm256_abs_epi8(__m256i);
extern __m256i __cdecl _mm256_abs_epi16(__m256i);
extern __m256i __cdecl _mm256_abs_epi32(__m256i);

extern __m256i __cdecl _mm256_add_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_add_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_add_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_add_epi64(__m256i, __m256i);

extern __m256i __cdecl _mm256_adds_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_adds_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_adds_epu8(__m256i, __m256i);
extern __m256i __cdecl _mm256_adds_epu16(__m256i, __m256i);

extern __m256i __cdecl _mm256_sub_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_sub_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_sub_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_sub_epi64(__m256i, __m256i);

extern __m256i __cdecl _mm256_subs_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_subs_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_subs_epu8(__m256i, __m256i);
extern __m256i __cdecl _mm256_subs_epu16(__m256i, __m256i);

extern __m256i __cdecl _mm256_avg_epu8(__m256i, __m256i);
extern __m256i __cdecl _mm256_avg_epu16(__m256i, __m256i);

extern __m256i __cdecl _mm256_hadd_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_hadd_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_hadds_epi16(__m256i, __m256i);

extern __m256i __cdecl _mm256_hsub_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_hsub_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_hsubs_epi16(__m256i, __m256i);

extern __m256i __cdecl _mm256_madd_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_maddubs_epi16(__m256i, __m256i);

extern __m256i __cdecl _mm256_mulhi_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_mulhi_epu16(__m256i, __m256i);

extern __m256i __cdecl _mm256_mullo_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_mullo_epi32(__m256i, __m256i);

extern __m256i __cdecl _mm256_mul_epu32(__m256i, __m256i);
extern __m256i __cdecl _mm256_mul_epi32(__m256i, __m256i);

extern __m256i __cdecl _mm256_sign_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_sign_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_sign_epi32(__m256i, __m256i);

extern __m256i __cdecl _mm256_mulhrs_epi16(__m256i, __m256i);

extern __m256i __cdecl _mm256_sad_epu8(__m256i, __m256i);
extern __m256i __cdecl _mm256_mpsadbw_epu8(__m256i, __m256i, const int);


/*
 * Integer 256-bit vector arithmetic/logical shift operations.
 */
extern __m256i __cdecl _mm256_slli_si256(__m256i, const int);
#define _mm256_bslli_epi128 _mm256_slli_si256
extern __m256i __cdecl _mm256_srli_si256(__m256i, const int);
#define _mm256_bsrli_epi128 _mm256_srli_si256

extern __m256i __cdecl _mm256_sll_epi16(__m256i, __m128i);
extern __m256i __cdecl _mm256_sll_epi32(__m256i, __m128i);
extern __m256i __cdecl _mm256_sll_epi64(__m256i, __m128i);

extern __m256i __cdecl _mm256_slli_epi16(__m256i, int);
extern __m256i __cdecl _mm256_slli_epi32(__m256i, int);
extern __m256i __cdecl _mm256_slli_epi64(__m256i, int);

extern __m256i __cdecl _mm256_sllv_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_sllv_epi64(__m256i, __m256i);

extern __m128i __cdecl _mm_sllv_epi32(__m128i, __m128i);
extern __m128i __cdecl _mm_sllv_epi64(__m128i, __m128i);

extern __m256i __cdecl _mm256_sra_epi16(__m256i, __m128i);
extern __m256i __cdecl _mm256_sra_epi32(__m256i, __m128i);

extern __m256i __cdecl _mm256_srai_epi16(__m256i, int);
extern __m256i __cdecl _mm256_srai_epi32(__m256i, int);

extern __m256i __cdecl _mm256_srav_epi32(__m256i, __m256i);

extern __m128i __cdecl _mm_srav_epi32(__m128i, __m128i);

extern __m256i __cdecl _mm256_srl_epi16(__m256i, __m128i);
extern __m256i __cdecl _mm256_srl_epi32(__m256i, __m128i);
extern __m256i __cdecl _mm256_srl_epi64(__m256i, __m128i);

extern __m256i __cdecl _mm256_srli_epi16(__m256i, int);
extern __m256i __cdecl _mm256_srli_epi32(__m256i, int);
extern __m256i __cdecl _mm256_srli_epi64(__m256i, int);

extern __m256i __cdecl _mm256_srlv_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_srlv_epi64(__m256i, __m256i);

extern __m128i __cdecl _mm_srlv_epi32(__m128i, __m128i);
extern __m128i __cdecl _mm_srlv_epi64(__m128i, __m128i);


/*
 * Integer 128/256-bit vector pack/blend/shuffle/insert/extract operations.
 */
extern __m128i __cdecl _mm_blend_epi32(__m128i, __m128i, const int);

extern __m256i __cdecl _mm256_blend_epi32(__m256i,__m256i, const int);

extern __m256i __cdecl _mm256_alignr_epi8(__m256i, __m256i, const int);

extern __m256i __cdecl _mm256_blendv_epi8(__m256i, __m256i, __m256i);
extern __m256i __cdecl _mm256_blend_epi16(__m256i, __m256i, const int);

extern __m256i __cdecl _mm256_packs_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_packs_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_packus_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_packus_epi32(__m256i, __m256i);

extern __m256i __cdecl _mm256_unpackhi_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_unpackhi_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_unpackhi_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_unpackhi_epi64(__m256i, __m256i);

extern __m256i __cdecl _mm256_unpacklo_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_unpacklo_epi16(__m256i, __m256i);
extern __m256i __cdecl _mm256_unpacklo_epi32(__m256i, __m256i);
extern __m256i __cdecl _mm256_unpacklo_epi64(__m256i, __m256i);

extern __m256i __cdecl _mm256_shuffle_epi8(__m256i, __m256i);
extern __m256i __cdecl _mm256_shuffle_epi32(__m256i, const int);

extern __m256i __cdecl _mm256_shufflehi_epi16(__m256i, const int);
extern __m256i __cdecl _mm256_shufflelo_epi16(__m256i, const int);

extern __m128i __cdecl _mm256_extracti128_si256(__m256i, const int);
extern __m256i __cdecl _mm256_inserti128_si256(__m256i, __m128i, const int);


/*
 * Scalar to 128/256-bit vector broadcast operations.
 */
extern __m128  __cdecl _mm_broadcastss_ps(__m128);
extern __m128d __cdecl _mm_broadcastsd_pd(__m128d);

extern __m128i __cdecl _mm_broadcastb_epi8(__m128i);
extern __m128i __cdecl _mm_broadcastw_epi16(__m128i);
extern __m128i __cdecl _mm_broadcastd_epi32(__m128i);
extern __m128i __cdecl _mm_broadcastq_epi64(__m128i);

extern __m256  __cdecl _mm256_broadcastss_ps(__m128);
extern __m256d __cdecl _mm256_broadcastsd_pd(__m128d);

extern __m256i __cdecl _mm256_broadcastb_epi8(__m128i);
extern __m256i __cdecl _mm256_broadcastw_epi16(__m128i);
extern __m256i __cdecl _mm256_broadcastd_epi32(__m128i);
extern __m256i __cdecl _mm256_broadcastq_epi64(__m128i);

extern __m256i __cdecl _mm256_broadcastsi128_si256(__m128i);



/*
 * Integer 256-bit vector signed/unsigned extension operations.
 */
extern __m256i __cdecl _mm256_cvtepi8_epi16(__m128i);
extern __m256i __cdecl _mm256_cvtepi8_epi32(__m128i);
extern __m256i __cdecl _mm256_cvtepi8_epi64(__m128i);
extern __m256i __cdecl _mm256_cvtepi16_epi32(__m128i);
extern __m256i __cdecl _mm256_cvtepi16_epi64(__m128i);
extern __m256i __cdecl _mm256_cvtepi32_epi64(__m128i);

extern __m256i __cdecl _mm256_cvtepu8_epi16(__m128i);
extern __m256i __cdecl _mm256_cvtepu8_epi32(__m128i);
extern __m256i __cdecl _mm256_cvtepu8_epi64(__m128i);
extern __m256i __cdecl _mm256_cvtepu16_epi32(__m128i);
extern __m256i __cdecl _mm256_cvtepu16_epi64(__m128i);
extern __m256i __cdecl _mm256_cvtepu32_epi64(__m128i);


/*
 * Returns a 32-bit mask made up of the most significant bit of each byte
 * of the 256-bit vector source operand.
 */
extern int __cdecl _mm256_movemask_epi8(__m256i);


/*
 * Masked load/store operations.
 */
extern __m128i __cdecl _mm_maskload_epi32(int const * /* ptr */,
                                          __m128i     /* vmask */);
extern __m128i __cdecl _mm_maskload_epi64(__int64 const * /* ptr */,
                                          __m128i         /* vmask */);

extern void __cdecl _mm_maskstore_epi32(int *   /* ptr */,
                                        __m128i /* vmask */,
                                        __m128i /* val */);
extern void __cdecl _mm_maskstore_epi64(__int64 * /* ptr */,
                                        __m128i   /* vmask */,
                                        __m128i   /* val */);

extern __m256i __cdecl _mm256_maskload_epi32(int const * /* ptr */,
                                             __m256i     /* vmask */);
extern __m256i __cdecl _mm256_maskload_epi64(__int64 const * /* ptr */,
                                             __m256i         /* vmask */);

extern void __cdecl _mm256_maskstore_epi32(int *   /* ptr */,
                                           __m256i /* vmask */,
                                           __m256i /* val */);
extern void __cdecl _mm256_maskstore_epi64(__int64 * /* ptr */,
                                           __m256i   /* vmask */,
                                           __m256i   /* val */);


/*
 * Permute elements in vector operations.
 */
extern __m256i __cdecl _mm256_permutevar8x32_epi32(__m256i, __m256i);
extern __m256  __cdecl _mm256_permutevar8x32_ps(__m256, __m256i);

extern __m256i __cdecl _mm256_permute4x64_epi64(__m256i, const int);
extern __m256d __cdecl _mm256_permute4x64_pd(__m256d, const int);

extern __m256i __cdecl _mm256_permute2x128_si256(__m256i, __m256i, const int);


/*
 * Load 32-bytes from memory using non-temporal aligned hint.
 */
extern __m256i  __cdecl _mm256_stream_load_si256(__m256i const *);



/*
 * Masked GATHER from memory to vector register operations.
 */
extern __m256d __cdecl _mm256_mask_i32gather_pd(__m256d        /* old_dst */,
                                                double const * /* ptr */,
                                                __m128i        /* vindex */,
                                                __m256d        /* vmask */,
                                                const int      /* scale */);
extern __m256  __cdecl _mm256_mask_i32gather_ps(__m256         /* old_dst */,
                                                float const *  /* ptr */,
                                                __m256i        /* vindex */,
                                                __m256         /* vmask */,
                                                const int      /* scale */);
extern __m256d __cdecl _mm256_mask_i64gather_pd(__m256d        /* old_dst */,
                                                double const * /* ptr */,
                                                __m256i        /* vindex */,
                                                __m256d        /* vmask */,
                                                const int      /* scale */);
extern __m128  __cdecl _mm256_mask_i64gather_ps(__m128         /* old_dst */,
                                                float const *  /* ptr */,
                                                __m256i        /* vindex */,
                                                __m128         /* vmask */,
                                                const int      /* scale */);

extern __m128d __cdecl _mm_mask_i32gather_pd(__m128d        /* old_dst */,
                                             double const * /* ptr */,
                                             __m128i        /* vindex */,
                                             __m128d        /* vmask */,
                                             const int      /* scale */);
extern __m128  __cdecl _mm_mask_i32gather_ps(__m128         /* old_dst */,
                                             float const *  /* ptr */,
                                             __m128i        /* vindex */,
                                             __m128         /* vmask */,
                                             const int      /* scale */);
extern __m128d __cdecl _mm_mask_i64gather_pd(__m128d        /* old_dst */,
                                             double const * /* ptr */,
                                             __m128i        /* vindex */,
                                             __m128d        /* vmask */,
                                             const int      /* scale */);
extern __m128  __cdecl _mm_mask_i64gather_ps(__m128         /* old_dst */,
                                             float const *  /* ptr */,
                                             __m128i        /* vindex */,
                                             __m128         /* vmask */,
                                             const int      /* scale */);


extern __m256i __cdecl _mm256_mask_i32gather_epi32(__m256i     /* old_dst */,
                                                   int const * /* ptr */,
                                                   __m256i     /* vindex */,
                                                   __m256i     /* vmask */,
                                                   const int   /* scale */);
extern __m256i __cdecl _mm256_mask_i32gather_epi64(__m256i     /* old_dst */,
                                                   __int64 const * /* ptr */,
                                                   __m128i     /* vindex */,
                                                   __m256i     /* vmask */,
                                                   const int   /* scale */);
extern __m128i __cdecl _mm256_mask_i64gather_epi32(__m128i     /* old_dst */,
                                                   int     const * /* ptr */,
                                                   __m256i     /* vindex */,
                                                   __m128i     /* vmask */,
                                                   const int   /* scale */);
extern __m256i __cdecl _mm256_mask_i64gather_epi64(__m256i     /* old_dst */,
                                                   __int64 const * /* ptr */,
                                                   __m256i     /* vindex */,
                                                   __m256i     /* vmask */,
                                                   const int   /* scale */);

extern __m128i __cdecl _mm_mask_i32gather_epi32(__m128i         /* old_dst */,
                                                int const *     /* ptr */,
                                                __m128i         /* vindex */,
                                                __m128i         /* vmask */,
                                                const int       /* scale */);
extern __m128i __cdecl _mm_mask_i32gather_epi64(__m128i         /* old_dst */,
                                                __int64 const * /* ptr */,
                                                __m128i         /* vindex */,
                                                __m128i         /* vmask */,
                                                const int       /* scale */);
extern __m128i __cdecl _mm_mask_i64gather_epi32(__m128i         /* old_dst */,
                                                int     const * /* ptr */,
                                                __m128i         /* vindex */,
                                                __m128i         /* vmask */,
                                                const int       /* scale */);
extern __m128i __cdecl _mm_mask_i64gather_epi64(__m128i         /* old_dst */,
                                                __int64 const * /* ptr */,
                                                __m128i         /* vindex */,
                                                __m128i         /* vmask */,
                                                const int       /* scale */);


/*
 * GATHER from memory to vector register operations.
 */
extern __m256d __cdecl _mm256_i32gather_pd(double const * /* ptr */,
                                           __m128i        /* vindex */,
                                           const int      /* index_scale */);
extern __m256  __cdecl _mm256_i32gather_ps(float  const * /* ptr */,
                                           __m256i        /* vindex */,
                                           const int      /* index_scale */);
extern __m256d __cdecl _mm256_i64gather_pd(double const * /* ptr */,
                                           __m256i        /* vindex */,
                                           const int      /* index_scale */);
extern __m128  __cdecl _mm256_i64gather_ps(float  const * /* ptr */,
                                           __m256i        /* vindex */,
                                           const int      /* index_scale */);

extern __m128d __cdecl _mm_i32gather_pd(double const * /* ptr */,
                                        __m128i        /* vindex */,
                                        const int      /* index_scale */);
extern __m128  __cdecl _mm_i32gather_ps(float  const * /* ptr */,
                                        __m128i        /* vindex */,
                                        const int      /* index_scale */);
extern __m128d __cdecl _mm_i64gather_pd(double const * /* ptr */,
                                        __m128i        /* vindex */,
                                        const int      /* index_scale */);
extern __m128  __cdecl _mm_i64gather_ps(float  const * /* ptr */,
                                        __m128i        /* vindex */,
                                        const int      /* index_scale */);

extern __m256i __cdecl _mm256_i32gather_epi32(int const *     /* ptr */,
                                              __m256i         /* vindex */,
                                              const int       /* scale */);
extern __m256i __cdecl _mm256_i32gather_epi64(__int64 const * /* ptr */,
                                              __m128i         /* vindex */,
                                              const int       /* scale */);
extern __m128i __cdecl _mm256_i64gather_epi32(int const *     /* ptr */,
                                              __m256i         /* vindex */,
                                              const int       /* scale */);
extern __m256i __cdecl _mm256_i64gather_epi64(__int64 const * /* ptr */,
                                              __m256i         /* vindex */,
                                              const int       /* scale */);

extern __m128i __cdecl _mm_i32gather_epi32(int const *     /* ptr */,
                                           __m128i         /* vindex */,
                                           const int       /* index_scale */);
extern __m128i __cdecl _mm_i32gather_epi64(__int64 const * /* ptr */,
                                           __m128i         /* vindex */,
                                           const int       /* index_scale */);
extern __m128i __cdecl _mm_i64gather_epi32(int     const * /* ptr */,
                                           __m128i         /* vindex */,
                                           const int       /* index_scale */);
extern __m128i __cdecl _mm_i64gather_epi64(__int64 const * /* ptr */,
                                           __m128i         /* vindex */,
                                           const int       /* index_scale */);


// unaligned load and store functions
#define _mm_loadu_si16(p) _mm_cvtsi32_si128(*(unsigned short const*)(p))
#define _mm_storeu_si16(p, a) (void)(*(short*)(p) = (short)_mm_cvtsi128_si32((a)))
#define _mm_loadu_si32(p) _mm_cvtsi32_si128(*(unsigned int const*)(p))
#define _mm_storeu_si32(p, a) (void)(*(int*)(p) = _mm_cvtsi128_si32((a)))
#define _mm_loadu_si64(p) _mm_loadl_epi64((__m128i const*)(p))
#define _mm_storeu_si64(p, a) (_mm_storel_epi64((__m128i*)(p), (a)))


/*
 * A collection of operations to manipulate integer data at bit-granularity.
 * The names of these functions are formed from the instruction mnemonic and
 * the operand data type used to implement them.
 */
extern unsigned int     _bextr_u32(unsigned int /* src */,
                                   unsigned int /* start_bit */,
                                   unsigned int /* len_in_bits */);
extern unsigned int     _bextr2_u32(unsigned int /* src */,
                                    unsigned int /* start_and_len_in_bits */);
extern unsigned int     _blsi_u32(unsigned int);
extern unsigned int     _blsmsk_u32(unsigned int);
extern unsigned int     _blsr_u32(unsigned int);
extern unsigned int     _bzhi_u32(unsigned int /* src */,
                                  unsigned int /* index */);
extern unsigned int     _mulx_u32(unsigned int /* src1 */,
                                  unsigned int /* src2 */,
                                  unsigned int * /* high_bits */);
extern unsigned int     _pdep_u32(unsigned int /* src */,
                                  unsigned int /* mask */);
extern unsigned int     _pext_u32(unsigned int /* src */,
                                  unsigned int /* mask */);
extern unsigned int     _rorx_u32(unsigned int /* src */,
                                  const unsigned int /* shift_count */);
extern int              _sarx_i32(int /* src */,
                                  unsigned int /* shift_count */);
extern unsigned int     _shlx_u32(unsigned int /* src */,
                                  unsigned int /* shift_count */);
extern unsigned int     _shrx_u32(unsigned int /* src */,
                                          unsigned int /* shift_count */);

#if defined (_M_X64)
extern unsigned __int64 _bextr_u64(unsigned __int64 /* src */,
                                   unsigned int /* start_bit */,
                                   unsigned int /* len_in_bits */);
extern unsigned __int64 _bextr2_u64(unsigned __int64 /* src */,
                                    unsigned __int64 /* start_and_len_in_bits */);
extern unsigned __int64 _blsi_u64(unsigned __int64);
extern unsigned __int64 _blsmsk_u64(unsigned __int64);
extern unsigned __int64 _blsr_u64(unsigned __int64);
extern unsigned __int64 _bzhi_u64(unsigned __int64 /* src */,
                                  unsigned int /* index */);
extern unsigned __int64 _mulx_u64(unsigned __int64 /* src1 */,
                                  unsigned __int64 /* src2 */,
                                  unsigned __int64 * /* high_bits */);
extern unsigned __int64 _pdep_u64(unsigned __int64 /* src */,
                                  unsigned __int64 /* mask */);
extern unsigned __int64 _pext_u64(unsigned __int64 /* src */,
                                  unsigned __int64 /* mask */);
extern unsigned __int64 _rorx_u64(unsigned __int64 /* src */,
                                  const unsigned int /* shift_count */);
extern __int64          _sarx_i64(__int64 /* src */,
                                  unsigned int /* shift_count */);
extern unsigned __int64 _shlx_u64(unsigned __int64 /* src */,
                                  unsigned int /* shift_count */);
extern unsigned __int64 _shrx_u64(unsigned __int64 /* src */,
                                          unsigned int /* shift_count */);
#endif  /* defined (_M_X64) */


/*
 * Leading zero bit count.
 *
 *    Counts the number of leading zero bits in a source operand.
 *    Returns operand size as output when source operand is zero.
 */
extern unsigned int     _lzcnt_u32(unsigned int);
#if defined (_M_X64)
extern unsigned __int64 _lzcnt_u64(unsigned __int64);
#endif  /* defined (_M_X64) */

/*
 * Trailing zero bit count.
 *
 *    Searches the source operand (r2) for the least significant set bit
 *    (1 bit).  If a least significant 1 bit is found, its bit index is
 *    returned, otherwise the result is the number of bits in the operand size.
 */
extern unsigned __int16 _tzcnt_u16(unsigned __int16);
extern unsigned int     _tzcnt_u32(unsigned int);
#if defined (_M_X64)
extern unsigned __int64 _tzcnt_u64(unsigned __int64);
#endif  /* defined (_M_X64) */



/*
 * Operation targeted to system software that manages processor context IDs.
 */
extern void __cdecl _invpcid(unsigned int /* type */, void * /* descriptor */);

// Hardware Lock Elision
extern void _Store_HLERelease(long volatile *,long);
extern void _StorePointer_HLERelease(void * volatile *,void *);

extern long _InterlockedExchange_HLEAcquire(long volatile *,long);
extern long _InterlockedExchange_HLERelease(long volatile *,long);
extern void * _InterlockedExchangePointer_HLEAcquire(void *volatile *,void *);
extern void * _InterlockedExchangePointer_HLERelease(void *volatile *,void *);

extern long _InterlockedCompareExchange_HLEAcquire(long volatile *,long,long);
extern long _InterlockedCompareExchange_HLERelease(long volatile *,long,long);
extern __int64 _InterlockedCompareExchange64_HLEAcquire(__int64 volatile *,__int64,__int64);
extern __int64 _InterlockedCompareExchange64_HLERelease(__int64 volatile *,__int64,__int64);
extern void * _InterlockedCompareExchangePointer_HLEAcquire(void *volatile *,void *,void *);
extern void * _InterlockedCompareExchangePointer_HLERelease(void *volatile *,void *,void *);

extern long _InterlockedExchangeAdd_HLEAcquire(long volatile *,long);
extern long _InterlockedExchangeAdd_HLERelease(long volatile *,long);

extern long _InterlockedAnd_HLEAcquire(long volatile *,long);
extern long _InterlockedAnd_HLERelease(long volatile *,long);
extern long _InterlockedOr_HLEAcquire(long volatile *,long);
extern long _InterlockedOr_HLERelease(long volatile *,long);
extern long _InterlockedXor_HLEAcquire(long volatile *,long);
extern long _InterlockedXor_HLERelease(long volatile *,long);

extern unsigned char _interlockedbittestandset_HLEAcquire(long *,long);
extern unsigned char _interlockedbittestandset_HLERelease(long *,long);
extern unsigned char _interlockedbittestandreset_HLEAcquire(long *,long);
extern unsigned char _interlockedbittestandreset_HLERelease(long *,long);

#if defined(_M_X64)
extern void _Store64_HLERelease(__int64 volatile *,__int64);
extern __int64 _InterlockedExchange64_HLEAcquire(__int64 volatile *,__int64);
extern __int64 _InterlockedExchange64_HLERelease(__int64 volatile *,__int64);

extern __int64 _InterlockedExchangeAdd64_HLEAcquire(__int64 volatile *,__int64);
extern __int64 _InterlockedExchangeAdd64_HLERelease(__int64 volatile *,__int64);

extern __int64 _InterlockedAnd64_HLEAcquire(__int64 volatile *,__int64);
extern __int64 _InterlockedAnd64_HLERelease(__int64 volatile *,__int64);
extern __int64 _InterlockedOr64_HLEAcquire(__int64 volatile *,__int64);
extern __int64 _InterlockedOr64_HLERelease(__int64 volatile *,__int64);
extern __int64 _InterlockedXor64_HLEAcquire(__int64 volatile *,__int64);
extern __int64 _InterlockedXor64_HLERelease(__int64 volatile *,__int64);

extern unsigned char _interlockedbittestandset64_HLEAcquire(__int64 *,__int64);
extern unsigned char _interlockedbittestandset64_HLERelease(__int64 *,__int64);
extern unsigned char _interlockedbittestandreset64_HLEAcquire(__int64 *,__int64);
extern unsigned char _interlockedbittestandreset64_HLERelease(__int64 *,__int64);
#endif  /* defined (_M_X64) */

//  Restricted Transactional Memory
#define _XBEGIN_STARTED          (~0u)
#define _XABORT_EXPLICIT         (1 << 0)
#define _XABORT_RETRY            (1 << 1)
#define _XABORT_CONFLICT         (1 << 2)
#define _XABORT_CAPACITY         (1 << 3)
#define _XABORT_DEBUG            (1 << 4)
#define _XABORT_NESTED           (1 << 5)
#define _XABORT_CODE(x)          ((unsigned char)(((x) >> 24) & 0xFF))

extern unsigned int     __cdecl _xbegin(void);
extern void             __cdecl _xend(void);
extern void             __cdecl _xabort(const unsigned int);
extern unsigned char    __cdecl _xtest(void);

/*
 * Perform one attempt to generate a hardware generated random value
 * accordingly to the NIST SP 800-90B/C standards.
 * The generated value is written to the given memory location and the success
 * status is returned: 1 if the hardware could generate a valid random number
 * and 0 otherwise.
 */
extern int __cdecl _rdseed16_step(unsigned short *);
extern int __cdecl _rdseed32_step(unsigned int *);
#if defined(_M_X64)
extern int __cdecl _rdseed64_step(unsigned __int64 *);
#endif  /* defined (_M_X64) */

/*
 * The _addcarryx... functions generate ADCX and ADOX instructions which
 * use CF and OF (in the flags register) respectively to propagate carry.
 * Because this allows two add-with-carry sequences to be interleaved
 * without having to save and restore the carry flag this is useful in
 * multiprecision multiply for example. These functions return
 * the carry-out, which is convenient for chaining multiple operations.
 * The sum is written using the given reference.
 */
extern unsigned char __cdecl _addcarryx_u32(unsigned char /*c_in*/,
                                                   unsigned int /*src1*/,
                                                   unsigned int /*src2*/,
                                                   unsigned int * /*out*/);


#if defined(_M_X64)
extern unsigned char __cdecl _addcarryx_u64(unsigned char /*c_in*/,
                                                   unsigned __int64 /*src1*/,
                                                   unsigned __int64 /*src2*/,
                                                   unsigned __int64 * /*out*/);
#endif  /* defined (_M_X64) */


/*
 * Perform load a big-endian value from memory.
 */
extern unsigned short   __cdecl _load_be_u16(void const*);
extern unsigned int     __cdecl _load_be_u32(void const*);
extern unsigned __int64 __cdecl _load_be_u64(void const*);
#define _loadbe_i16(be_ptr) ((short)  _load_be_u16(be_ptr))
#define _loadbe_i32(be_ptr) ((int)    _load_be_u32(be_ptr))
#define _loadbe_i64(be_ptr) ((__int64)_load_be_u64(be_ptr))

/*
 * Perform store a value to memory as big-endian.
 */
extern void __cdecl _store_be_u16(void *, unsigned short);
extern void __cdecl _store_be_u32(void *, unsigned int);
extern void __cdecl _store_be_u64(void *, unsigned __int64);
#define _storebe_i16(be_ptr, val) _store_be_u16(be_ptr, (unsigned short)(val))
#define _storebe_i32(be_ptr, val) _store_be_u32(be_ptr, (unsigned int)(val))
#define _storebe_i64(be_ptr, val) _store_be_u64(be_ptr, (unsigned __int64)(__int64)(val))

/*
 * The Secure Hash Algorithm (SHA) New Instructions.
*/
extern __m128i __cdecl _mm_sha1msg1_epu32(__m128i, __m128i);
extern __m128i __cdecl _mm_sha1msg2_epu32(__m128i, __m128i);
extern __m128i __cdecl _mm_sha1nexte_epu32(__m128i, __m128i);
extern __m128i __cdecl _mm_sha1rnds4_epu32(__m128i, __m128i, const int);

extern __m128i __cdecl _mm_sha256msg1_epu32(__m128i, __m128i);
extern __m128i __cdecl _mm_sha256msg2_epu32(__m128i, __m128i);
extern __m128i __cdecl _mm_sha256rnds2_epu32(__m128i, __m128i, __m128i);

/*
 * Intel(R) Memory Protection Extensions (Intel(R) MPX) intrinsic functions
 */
extern void * __cdecl _bnd_set_ptr_bounds(const void *, size_t);
extern void * __cdecl _bnd_narrow_ptr_bounds(const void *, const void *, size_t);
extern void * __cdecl _bnd_copy_ptr_bounds(const void *, const void *);
extern void * __cdecl _bnd_init_ptr_bounds(const void *);
extern void __cdecl _bnd_store_ptr_bounds(const void **, const void *);
extern void __cdecl _bnd_chk_ptr_lbounds(const void *);
extern void __cdecl _bnd_chk_ptr_ubounds(const void *);
extern void __cdecl _bnd_chk_ptr_bounds(const void *, size_t);
extern void * __cdecl _bnd_load_ptr_bounds(const void **, const void *);
extern const void * __cdecl _bnd_get_ptr_lbound(const void *);
extern const void * __cdecl _bnd_get_ptr_ubound(const void *);

// Insert integer into 256-bit packed integer array at element selected by index
extern __m256i __cdecl _mm256_insert_epi8 (__m256i /* dst */, int /* src */, const int /* index */);
extern __m256i __cdecl _mm256_insert_epi16(__m256i /* dst */, int /* src */, const int /* index */);
extern __m256i __cdecl _mm256_insert_epi32(__m256i /* dst */, int /* src */, const int /* index */);
#if defined(_M_X64)
extern __m256i __cdecl _mm256_insert_epi64(__m256i /* dst */, __int64 /* src */, const int /* index */);
#endif  // defined (_M_X64)

// Extract integer element selected by index from 256-bit packed integer array
extern int __cdecl _mm256_extract_epi8 (__m256i /* src */, const int /* index */);
extern int __cdecl _mm256_extract_epi16(__m256i /* src */, const int /* index */);
extern int __cdecl _mm256_extract_epi32(__m256i /* src */, const int /* index */);
#if defined(_M_X64)
extern __int64 __cdecl _mm256_extract_epi64(__m256i /* src */, const int /* index */);
#endif  // defined (_M_X64)

// Zero-extended cast functions
extern __m256d __cdecl _mm256_zextpd128_pd256(__m128d);
extern __m256  __cdecl _mm256_zextps128_ps256(__m128);
extern __m256i __cdecl _mm256_zextsi128_si256(__m128i);

// RDPID
extern unsigned int __cdecl _rdpid_u32(void);
// PTWRITE
extern void         __cdecl _ptwrite32(unsigned int);
#if defined(_M_X64)
extern void         __cdecl _ptwrite64(unsigned __int64);
#endif  // defined (_M_X64)

// AVX_VNNI intrinsics
extern __m128i __cdecl _mm_dpbusd_avx_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbusd_avx_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpbusds_avx_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbusds_avx_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwssd_avx_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwssd_avx_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwssds_avx_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwssds_avx_epi32(__m256i, __m256i, __m256i);

// MKTME
extern unsigned int __cdecl _pconfig_u32(const int, size_t __data[]);
extern void __cdecl _wbnoinvd(void);

// SGX
extern unsigned int __cdecl _encls_u32(const int, size_t __data[]);
extern unsigned int __cdecl _enclu_u32(const int, size_t __data[]);
extern unsigned int __cdecl _enclv_u32(const int, size_t __data[]);

// Div & idiv
#if defined(_M_X64)
// _udiv128 is also declared in <intrin0.inl.h>
extern unsigned __int64 __cdecl _udiv128(unsigned __int64 /* highdividend */, unsigned __int64 /* lowdividend */, unsigned __int64 /* divisor */, unsigned __int64* /* remainder */);
extern __int64          __cdecl _div128(__int64 /* highdividend */, __int64 /* lowdividend */, __int64 /* divisor */, __int64* /* remainder */);
#endif  // defined (_M_X64)
extern unsigned         __cdecl _udiv64(unsigned __int64 /* dividend */, unsigned /* divisor */, unsigned* /* remainder */);
extern int              __cdecl _div64(__int64 /* dividend */, int /* divisor */, int* /* remainder */);

// Keylocker
extern unsigned char _mm_aesdec128kl_u8(__m128i* /* odata */, __m128i /* idata */, const void* /* h */);
extern unsigned char _mm_aesdec256kl_u8(__m128i* /* odata */, __m128i /* idata */, const void* /* h */);
extern unsigned char _mm_aesdecwide128kl_u8(__m128i* /* odata */, const __m128i* /* idata */, const void* /* h */);
extern unsigned char _mm_aesdecwide256kl_u8(__m128i* /* odata */, const __m128i* /* idata */, const void* /* h */);
extern unsigned char _mm_aesenc128kl_u8(__m128i* /* odata */, __m128i /* idata */, const void* /* h */);
extern unsigned char _mm_aesenc256kl_u8(__m128i* /* odata */, __m128i /* idata */, const void* /* h */);
extern unsigned char _mm_aesencwide128kl_u8(__m128i* /* odata */, const __m128i* /* idata */, const void* /* h */);
extern unsigned char _mm_aesencwide256kl_u8(__m128i* /* odata */, const __m128i* /* idata */, const void* /* h */);
extern unsigned int  _mm_encodekey128_u32(unsigned int /* htype */, __m128i /* key */, void* /* h */);
extern unsigned int  _mm_encodekey256_u32(unsigned int /* htype */, __m128i /* key_lo */, __m128i /* key_hi */, void* /* h */);
extern void          _mm_loadiwkey(unsigned int /* ctl */, __m128i /* intkey */, __m128i /* enkey_lo */, __m128i /* enkey_hi */);

// Protection key rights for user pages
extern unsigned int     __cdecl _rdpkru_u32(void);
extern void             __cdecl _wrpkru(unsigned int);

// Enqueue stores
extern int              __cdecl _enqcmd(void * /* dst */, const void * /* src */);
extern int              __cdecl _enqcmds(void * /* dst */, const void * /* src */);

/*
* Intel(R) Control-Flow Enforcement Technology (CET) shadow stack intrinsic functions
*/
extern void             __cdecl _incsspd (unsigned int);
extern unsigned int     __cdecl _rdsspd (void);
extern void             __cdecl _saveprevssp (void);
extern void             __cdecl _rstorssp (void *);
extern void             __cdecl _wrssd (unsigned int, void *);
extern void             __cdecl _wrussd (unsigned int, void *);
extern void             __cdecl _setssbsy (void);
extern void             __cdecl _clrssbsy (void *);
extern void *           __cdecl _switchssp(void *);
#if defined(_M_X64)
extern void             __cdecl _incsspq (unsigned __int64);
extern unsigned __int64 __cdecl _rdsspq (void);
extern void             __cdecl _wrssq (unsigned __int64, void *);
extern void             __cdecl _wrussq(unsigned __int64, void *);
#endif

/*
* Intrinsic functions for Short Vector Math Library (SVML)
*/

// vector integer divide and remainder
extern __m128i _mm_div_epi8(__m128i, __m128i);
extern __m128i _mm_div_epi16(__m128i, __m128i);
extern __m128i _mm_div_epi32(__m128i, __m128i);
extern __m128i _mm_div_epi64(__m128i, __m128i);
extern __m128i _mm_div_epu8(__m128i, __m128i);
extern __m128i _mm_div_epu16(__m128i, __m128i);
extern __m128i _mm_div_epu32(__m128i, __m128i);
extern __m128i _mm_div_epu64(__m128i, __m128i);
extern __m128i _mm_rem_epi8(__m128i, __m128i);
extern __m128i _mm_rem_epi16(__m128i, __m128i);
extern __m128i _mm_rem_epi32(__m128i, __m128i);
extern __m128i _mm_rem_epi64(__m128i, __m128i);
extern __m128i _mm_rem_epu8(__m128i, __m128i);
extern __m128i _mm_rem_epu16(__m128i, __m128i);
extern __m128i _mm_rem_epu32(__m128i, __m128i);
extern __m128i _mm_rem_epu64(__m128i, __m128i);
extern __m256i _mm256_div_epi8(__m256i, __m256i);
extern __m256i _mm256_div_epi16(__m256i, __m256i);
extern __m256i _mm256_div_epi32(__m256i, __m256i);
extern __m256i _mm256_div_epi64(__m256i, __m256i);
extern __m256i _mm256_div_epu8(__m256i, __m256i);
extern __m256i _mm256_div_epu16(__m256i, __m256i);
extern __m256i _mm256_div_epu32(__m256i, __m256i);
extern __m256i _mm256_div_epu64(__m256i, __m256i);
extern __m256i _mm256_rem_epi8(__m256i, __m256i);
extern __m256i _mm256_rem_epi16(__m256i, __m256i);
extern __m256i _mm256_rem_epi32(__m256i, __m256i);
extern __m256i _mm256_rem_epi64(__m256i, __m256i);
extern __m256i _mm256_rem_epu8(__m256i, __m256i);
extern __m256i _mm256_rem_epu16(__m256i, __m256i);
extern __m256i _mm256_rem_epu32(__m256i, __m256i);
extern __m256i _mm256_rem_epu64(__m256i, __m256i);

#define _mm_idiv_epi32 _mm_div_epi32
#define _mm_irem_epi32 _mm_rem_epi32
#define _mm_udiv_epi32 _mm_div_epu32
#define _mm_urem_epi32 _mm_rem_epu32
#define _mm256_idiv_epi32 _mm256_div_epi32
#define _mm256_irem_epi32 _mm256_rem_epi32
#define _mm256_udiv_epi32 _mm256_div_epu32
#define _mm256_urem_epi32 _mm256_rem_epu32

extern __m128i _mm_divrem_epi32(__m128i * /*rem_res*/, __m128i, __m128i);
extern __m128i _mm_divrem_epu32(__m128i * /*rem_res*/, __m128i, __m128i);
extern __m256i _mm256_divrem_epi32(__m256i * /*rem_res*/, __m256i, __m256i);
extern __m256i _mm256_divrem_epu32(__m256i * /*rem_res*/, __m256i, __m256i);

#define _mm_idivrem_epi32    _mm_divrem_epi32
#define _mm_udivrem_epi32    _mm_divrem_epu32
#define _mm256_idivrem_epi32 _mm256_divrem_epi32
#define _mm256_udivrem_epi32 _mm256_divrem_epu32

// Math functions
extern __m128  _mm_sin_ps(__m128);
extern __m128d _mm_sin_pd(__m128d);
extern __m256  _mm256_sin_ps(__m256);
extern __m256d _mm256_sin_pd(__m256d);
extern __m128  _mm_cos_ps(__m128);
extern __m128d _mm_cos_pd(__m128d);
extern __m256  _mm256_cos_ps(__m256);
extern __m256d _mm256_cos_pd(__m256d);
extern __m128  _mm_sincos_ps(__m128  * /*cos_res*/, __m128);
extern __m128d _mm_sincos_pd(__m128d * /*cos_res*/, __m128d);
extern __m256  _mm256_sincos_ps(__m256  * /*cos_res*/, __m256);
extern __m256d _mm256_sincos_pd(__m256d * /*cos_res*/, __m256d);
extern __m128  _mm_tan_ps(__m128);
extern __m128d _mm_tan_pd(__m128d);
extern __m256  _mm256_tan_ps(__m256);
extern __m256d _mm256_tan_pd(__m256d);
extern __m128  _mm_asin_ps(__m128);
extern __m128d _mm_asin_pd(__m128d);
extern __m256  _mm256_asin_ps(__m256);
extern __m256d _mm256_asin_pd(__m256d);
extern __m128  _mm_acos_ps(__m128);
extern __m128d _mm_acos_pd(__m128d);
extern __m256  _mm256_acos_ps(__m256);
extern __m256d _mm256_acos_pd(__m256d);
extern __m128  _mm_atan_ps(__m128);
extern __m128d _mm_atan_pd(__m128d);
extern __m256  _mm256_atan_ps(__m256);
extern __m256d _mm256_atan_pd(__m256d);
extern __m128  _mm_atan2_ps(__m128, __m128);
extern __m128d _mm_atan2_pd(__m128d, __m128d);
extern __m256  _mm256_atan2_ps(__m256, __m256);
extern __m256d _mm256_atan2_pd(__m256d, __m256d);
extern __m128  _mm_sind_ps(__m128);
extern __m128d _mm_sind_pd(__m128d);
extern __m256  _mm256_sind_ps(__m256);
extern __m256d _mm256_sind_pd(__m256d);
extern __m128  _mm_cosd_ps(__m128);
extern __m128d _mm_cosd_pd(__m128d);
extern __m256  _mm256_cosd_ps(__m256);
extern __m256d _mm256_cosd_pd(__m256d);
extern __m128  _mm_tand_ps(__m128);
extern __m128d _mm_tand_pd(__m128d);
extern __m256  _mm256_tand_ps(__m256);
extern __m256d _mm256_tand_pd(__m256d);
extern __m128  _mm_sinh_ps(__m128);
extern __m128d _mm_sinh_pd(__m128d);
extern __m256  _mm256_sinh_ps(__m256);
extern __m256d _mm256_sinh_pd(__m256d);
extern __m128  _mm_cosh_ps(__m128);
extern __m128d _mm_cosh_pd(__m128d);
extern __m256  _mm256_cosh_ps(__m256);
extern __m256d _mm256_cosh_pd(__m256d);
extern __m128  _mm_tanh_ps(__m128);
extern __m128d _mm_tanh_pd(__m128d);
extern __m256  _mm256_tanh_ps(__m256);
extern __m256d _mm256_tanh_pd(__m256d);
extern __m128  _mm_asinh_ps(__m128);
extern __m128d _mm_asinh_pd(__m128d);
extern __m256  _mm256_asinh_ps(__m256);
extern __m256d _mm256_asinh_pd(__m256d);
extern __m128  _mm_acosh_ps(__m128);
extern __m128d _mm_acosh_pd(__m128d);
extern __m256  _mm256_acosh_ps(__m256);
extern __m256d _mm256_acosh_pd(__m256d);
extern __m128  _mm_atanh_ps(__m128);
extern __m128d _mm_atanh_pd(__m128d);
extern __m256  _mm256_atanh_ps(__m256);
extern __m256d _mm256_atanh_pd(__m256d);
extern __m128  _mm_log_ps(__m128);
extern __m128d _mm_log_pd(__m128d);
extern __m256  _mm256_log_ps(__m256);
extern __m256d _mm256_log_pd(__m256d);
extern __m128  _mm_log1p_ps(__m128);
extern __m128d _mm_log1p_pd(__m128d);
extern __m256  _mm256_log1p_ps(__m256);
extern __m256d _mm256_log1p_pd(__m256d);
extern __m128  _mm_log10_ps(__m128);
extern __m128d _mm_log10_pd(__m128d);
extern __m256  _mm256_log10_ps(__m256);
extern __m256d _mm256_log10_pd(__m256d);
extern __m128  _mm_log2_ps(__m128);
extern __m128d _mm_log2_pd(__m128d);
extern __m256  _mm256_log2_ps(__m256);
extern __m256d _mm256_log2_pd(__m256d);
extern __m128  _mm_logb_ps(__m128);
extern __m128d _mm_logb_pd(__m128d);
extern __m256  _mm256_logb_ps(__m256);
extern __m256d _mm256_logb_pd(__m256d);
extern __m128  _mm_exp_ps(__m128);
extern __m128d _mm_exp_pd(__m128d);
extern __m256  _mm256_exp_ps(__m256);
extern __m256d _mm256_exp_pd(__m256d);
extern __m128  _mm_exp10_ps(__m128);
extern __m128d _mm_exp10_pd(__m128d);
extern __m256  _mm256_exp10_ps(__m256);
extern __m256d _mm256_exp10_pd(__m256d);
extern __m128  _mm_exp2_ps(__m128);
extern __m128d _mm_exp2_pd(__m128d);
extern __m256  _mm256_exp2_ps(__m256);
extern __m256d _mm256_exp2_pd(__m256d);
extern __m128  _mm_expm1_ps(__m128);
extern __m128d _mm_expm1_pd(__m128d);
extern __m256  _mm256_expm1_ps(__m256);
extern __m256d _mm256_expm1_pd(__m256d);
extern __m128  _mm_pow_ps(__m128, __m128);
extern __m128d _mm_pow_pd(__m128d, __m128d);
extern __m256  _mm256_pow_ps(__m256, __m256);
extern __m256d _mm256_pow_pd(__m256d, __m256d);
extern __m128  _mm_trunc_ps(__m128);
extern __m128d _mm_trunc_pd(__m128d);
extern __m256  _mm256_trunc_ps(__m256);
extern __m256d _mm256_trunc_pd(__m256d);
extern __m128  _mm_svml_floor_ps(__m128);
extern __m128d _mm_svml_floor_pd(__m128d);
extern __m256  _mm256_svml_floor_ps(__m256);
extern __m256d _mm256_svml_floor_pd(__m256d);
extern __m128  _mm_svml_ceil_ps(__m128);
extern __m128d _mm_svml_ceil_pd(__m128d);
extern __m256  _mm256_svml_ceil_ps(__m256);
extern __m256d _mm256_svml_ceil_pd(__m256d);
extern __m128  _mm_svml_round_ps(__m128);
extern __m128d _mm_svml_round_pd(__m128d);
extern __m256  _mm256_svml_round_ps(__m256);
extern __m256d _mm256_svml_round_pd(__m256d);
extern __m128  _mm_fmod_ps(__m128, __m128);
extern __m128d _mm_fmod_pd(__m128d, __m128d);
extern __m256  _mm256_fmod_ps(__m256, __m256);
extern __m256d _mm256_fmod_pd(__m256d, __m256d);
extern __m128  _mm_svml_sqrt_ps(__m128);
extern __m128d _mm_svml_sqrt_pd(__m128d);
extern __m256  _mm256_svml_sqrt_ps(__m256);
extern __m256d _mm256_svml_sqrt_pd(__m256d);
extern __m128  _mm_invsqrt_ps(__m128);
extern __m128d _mm_invsqrt_pd(__m128d);
extern __m256  _mm256_invsqrt_ps(__m256);
extern __m256d _mm256_invsqrt_pd(__m256d);
extern __m128  _mm_cbrt_ps(__m128);
extern __m128d _mm_cbrt_pd(__m128d);
extern __m256  _mm256_cbrt_ps(__m256);
extern __m256d _mm256_cbrt_pd(__m256d);
extern __m128  _mm_invcbrt_ps(__m128);
extern __m128d _mm_invcbrt_pd(__m128d);
extern __m256  _mm256_invcbrt_ps(__m256);
extern __m256d _mm256_invcbrt_pd(__m256d);
extern __m128  _mm_hypot_ps(__m128, __m128);
extern __m128d _mm_hypot_pd(__m128d, __m128d);
extern __m256  _mm256_hypot_ps(__m256, __m256);
extern __m256d _mm256_hypot_pd(__m256d, __m256d);
extern __m128  _mm_cdfnorm_ps(__m128);
extern __m128d _mm_cdfnorm_pd(__m128d);
extern __m256  _mm256_cdfnorm_ps(__m256);
extern __m256d _mm256_cdfnorm_pd(__m256d);
extern __m128  _mm_cdfnorminv_ps(__m128);
extern __m128d _mm_cdfnorminv_pd(__m128d);
extern __m256  _mm256_cdfnorminv_ps(__m256);
extern __m256d _mm256_cdfnorminv_pd(__m256d);
extern __m128  _mm_cexp_ps(__m128);
extern __m256  _mm256_cexp_ps(__m256);
extern __m128  _mm_clog_ps(__m128);
extern __m256  _mm256_clog_ps(__m256);
extern __m128  _mm_csqrt_ps(__m128);
extern __m256  _mm256_csqrt_ps(__m256);
extern __m128  _mm_erf_ps(__m128);
extern __m128d _mm_erf_pd(__m128d);
extern __m256  _mm256_erf_ps(__m256);
extern __m256d _mm256_erf_pd(__m256d);
extern __m128  _mm_erfc_ps(__m128);
extern __m128d _mm_erfc_pd(__m128d);
extern __m256  _mm256_erfc_ps(__m256);
extern __m256d _mm256_erfc_pd(__m256d);
extern __m128  _mm_erfcinv_ps(__m128);
extern __m128d _mm_erfcinv_pd(__m128d);
extern __m256  _mm256_erfcinv_ps(__m256);
extern __m256d _mm256_erfcinv_pd(__m256d);
extern __m128  _mm_erfinv_ps(__m128);
extern __m128d _mm_erfinv_pd(__m128d);
extern __m256  _mm256_erfinv_ps(__m256);
extern __m256d _mm256_erfinv_pd(__m256d);

/* Cache line demote */
extern void _mm_cldemote(void const *);
#define _cldemote  _mm_cldemote

/* Direct stores */
extern void _directstoreu_u32(void *, unsigned int);
#if defined (_M_X64)
extern void _directstoreu_u64(void *, unsigned __int64);
#endif  /* defined (_M_X64) */
extern void _movdir64b(void *, void const *);

/* serialize and TSX load tracking */
extern void __cdecl _serialize(void);
extern void __cdecl _xsusldtrk(void);
extern void __cdecl _xresldtrk(void);

/* User wait */
extern void _umonitor(void *);
extern unsigned char _umwait(unsigned int, unsigned __int64);
extern unsigned char _tpause(unsigned int, unsigned __int64);

/* user interrupts */
#if defined (_M_X64)
extern void _clui(void);
extern void _stui(void);
extern unsigned char _testui(void);
extern void _senduipi(unsigned __int64);
#endif  /* defined (_M_X64) */

/* Hreset */
extern void _hreset(unsigned __int32);

/* SVML conversions (no AVX-512 support needed) */
extern __m128 _mm_svml_cvtepu32_ps (__m128i);
extern __m256 _mm256_svml_cvtepu32_ps (__m256i);
extern __m128d _mm_svml_cvtepu32_pd (__m128i);
extern __m256d _mm256_svml_cvtepu32_pd (__m128i);
extern __m128d _mm_svml_cvtepi64_pd (__m128i);
extern __m256d _mm256_svml_cvtepi64_pd (__m256i);
extern __m128d _mm_svml_cvtepu64_pd (__m128i);
extern __m256d _mm256_svml_cvtepu64_pd (__m256i);

/* casting between floating-point and representation bit pattern as unsigned */
extern unsigned __int32 _castf32_u32 (float);
extern unsigned __int64 _castf64_u64 (double);
extern float _castu32_f32 (unsigned __int32);
extern double _castu64_f64 (unsigned __int64);

#if defined __cplusplus
}; /* End "C" */
#endif  /* defined __cplusplus */

#include <zmmintrin.h>

#ifdef __cplusplus
extern "C" {
#endif  /* __cplusplus */

// No AVX-512 support needed, but require zmmintrin.h definitions for bfloat16 types.

/* Intel(R) AVX-IFMA */
extern __m128i __cdecl _mm_madd52hi_avx_epu64(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_madd52hi_avx_epu64(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_madd52lo_avx_epu64(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_madd52lo_avx_epu64(__m256i, __m256i, __m256i);

/* Intel(R) AVX-NE-CONVERT */
extern __m128 __cdecl _mm_bcstnebf16_ps(const __bfloat16 *);
extern __m256 __cdecl _mm256_bcstnebf16_ps(const __bfloat16 *);
extern __m128 __cdecl _mm_bcstnesh_ps(const void *);
extern __m256 __cdecl _mm256_bcstnesh_ps(const void *);
extern __m128 __cdecl _mm_cvtneebf16_ps(const __m128bh *);
extern __m256 __cdecl _mm256_cvtneebf16_ps(const __m256bh *);
extern __m128 __cdecl _mm_cvtneeph_ps(const __m128h *);
extern __m256 __cdecl _mm256_cvtneeph_ps(const __m256h *);
extern __m128 __cdecl _mm_cvtneobf16_ps(const __m128bh *);
extern __m256 __cdecl _mm256_cvtneobf16_ps(const __m256bh *);
extern __m128 __cdecl _mm_cvtneoph_ps(const __m128h *);
extern __m256 __cdecl _mm256_cvtneoph_ps(const __m256h *);
extern __m128bh __cdecl _mm_cvtneps_avx_pbh(__m128);
extern __m128bh __cdecl _mm256_cvtneps_avx_pbh(__m256);

/* Intel(R) AVX-VNNI-INT8 */
extern __m128i __cdecl _mm_dpbssd_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbssd_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpbssds_epi32( __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbssds_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpbsud_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbsud_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpbsuds_epi32( __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbsuds_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpbuud_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbuud_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpbuuds_epi32( __m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpbuuds_epi32(__m256i, __m256i, __m256i);

/* RAO-INT */
extern void __cdecl _aadd_i32(int*, int);
extern void __cdecl _aand_i32(int*, int);
extern void __cdecl _aor_i32(int*, int);
extern void __cdecl _axor_i32(int*, int);
#if defined (_M_X64)
extern void __cdecl _aadd_i64(__int64*, __int64);
extern void __cdecl _aand_i64(__int64*, __int64);
extern void __cdecl _aor_i64(__int64*, __int64);
extern void __cdecl _axor_i64(__int64*, __int64);
#endif  /* defined (_M_X64) */

/*
 * Scalar FP intrinsics (with double/float arguments)
 */
extern float  __cdecl __rsqrt14_ss(float);
extern double __cdecl __rsqrt14_sd(double);
extern float  __cdecl __rsqrt_ss(float);
extern float  __cdecl __sqrt_ss(float);
extern double __cdecl __sqrt_sd(double);
extern float  __cdecl __max_ss(float, float);
extern double __cdecl __max_sd(double, double);
extern float  __cdecl __min_ss(float, float);
extern double __cdecl __min_sd(double, double);

#if defined (_M_X64)

/* Intel(R) CMPCCXADD */
typedef enum {
  _CMPCCX_O,   /* Overflow.  */
  _CMPCCX_NO,  /* No overflow.  */
  _CMPCCX_B,   /* Below.  */
  _CMPCCX_NB,  /* Not below.  */
  _CMPCCX_Z,   /* Zero.  */
  _CMPCCX_NZ,  /* Not zero.  */
  _CMPCCX_BE,  /* Below or equal.  */
  _CMPCCX_NBE, /* Neither below nor equal.  */
  _CMPCCX_S,   /* Sign.  */
  _CMPCCX_NS,  /* No sign.  */
  _CMPCCX_P,   /* Parity.  */
  _CMPCCX_NP,  /* No parity.  */
  _CMPCCX_L,   /* Less.  */
  _CMPCCX_NL,  /* Not less.  */
  _CMPCCX_LE,  /* Less or equal.  */
  _CMPCCX_NLE, /* Neither less nor equal.  */
} _CMPCCX_ENUM;
//
extern int __cdecl _cmpccxadd_epi32(void *, int, int, const int);
extern __int64 __cdecl _cmpccxadd_epi64(void *, __int64, __int64, const int);

#endif  /* defined (_M_X64) */

// scalar functions with scalar arguments and return values
extern double __round_sd(double, int);
extern float __round_ss(float, int);
/* Intel(R) AVX-VNNI-INT16 */
extern __m128i __cdecl _mm_dpwsud_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwsud_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwsuds_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwsuds_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwusd_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwusd_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwusds_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwusds_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwuud_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwuud_epi32(__m256i, __m256i, __m256i);
extern __m128i __cdecl _mm_dpwuuds_epi32(__m128i, __m128i, __m128i);
extern __m256i __cdecl _mm256_dpwuuds_epi32(__m256i, __m256i, __m256i);

/* TSE */
#if defined (_M_X64)
extern long long __cdecl _pbndkb(const void*, void*);
#endif  /* defined (_M_X64) */

/* SHA512 */
extern __m256i __cdecl _mm256_sha512msg1_epi64(__m256i, __m128i);
extern __m256i __cdecl _mm256_sha512msg2_epi64(__m256i, __m256i);
extern __m256i __cdecl _mm256_sha512rnds2_epi64(__m256i, __m256i, __m128i);

/* SM3 */
extern __m128i __cdecl _mm_sm3msg1_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_sm3msg2_epi32(__m128i, __m128i, __m128i);
extern __m128i __cdecl _mm_sm3rnds2_epi32(__m128i, __m128i, __m128i, const int);

/* SM4 */
extern __m128i __cdecl _mm_sm4key4_epi32(__m128i, __m128i);
extern __m256i __cdecl _mm256_sm4key4_epi32(__m256i, __m256i);
extern __m128i __cdecl _mm_sm4rnds4_epi32(__m128i, __m128i);
extern __m256i __cdecl _mm256_sm4rnds4_epi32(__m256i, __m256i);

#if defined (_M_X64)
extern unsigned __int64 __cdecl _urdmsr(unsigned __int64);
extern void __cdecl _uwrmsr(unsigned __int64, unsigned __int64);
#endif /* defined (_M_X64) */

__m128i __iso_volatile_ia_load128(const volatile __m128i*);
__m256i __iso_volatile_ia_load256(const volatile __m256i*);
__m512i __iso_volatile_ia_load512(const volatile __m512i*);
void __iso_volatile_ia_store128(volatile __m128i*, __m128i);
void __iso_volatile_ia_store256(volatile __m256i*, __m256i);
void __iso_volatile_ia_store512(volatile __m512i*, __m512i);
__m128i __iso_volatile_ia_nt_load128(const volatile __m128i*);
__m256i __iso_volatile_ia_nt_load256(const volatile __m256i*);
__m512i __iso_volatile_ia_nt_load512(const volatile __m512i*);
void __iso_volatile_ia_nt_store128(volatile __m128i*, __m128i);
void __iso_volatile_ia_nt_store256(volatile __m256i*, __m256i);
void __iso_volatile_ia_nt_store512(volatile __m512i*, __m512i);

// MOVRS scalar instructions.
// Prefetch supported for both x32 and x64
extern void _m_prefetchrs(const void *);

#if defined (_M_X64)
extern char  __cdecl _movrs_i8(const void *);
extern short __cdecl _movrs_i16(const void *);
extern int   __cdecl _movrs_i32(const void *);
extern long long __cdecl _movrs_i64(const void *);
#endif /* defined (_M_X64) */

#if defined __cplusplus
} /* End "C" */
#endif  /* defined __cplusplus */

#endif  /* defined (_M_CEE_PURE) */
#endif  /* __midl */
#endif  /* _INCLUDED_IMM */
