/*++

Copyright (c) 2019 Microsoft Corporation

Module Name:

    softintrin.h

Abstract:

    Portable wrappers for common x64 intrinsics and related types.

Author:

    Darek Mihocka (darekm) 29-Jun-2019

Environment:

    Pure library routines. Any environment.

--*/

#pragma once

#if defined(_M_ARM64EC) || ((defined(_M_IX86) || defined(_M_AMD64) || defined(_M_ARM64)) && defined(USE_SOFT_INTRINSICS))

#include <widemath.h>                // 64-bit and 128-bit math helper functions

#ifdef __cplusplus

extern "C"
{

#endif

//
//  Re-map intrinsic names to inline functions that use WideMath.
//

__forceinline uint64_t __U64(const __m64 V) { uint64_t *T = (uint64_t *)&V; return *T; }

__forceinline __m64 __MI64(const uint64_t U) { __m64 *T = (__m64 *)&U; return *T; }

__forceinline
uint128_t __S128(const __m128  V)
{
    uint128_t T;
    T.Q[0] = V.m128_u64[0];
    T.Q[1] = V.m128_u64[1];
    return T;
}

__forceinline
uint128_t __U128(const __m128i V)
{
    uint128_t T;
    T.Q[0] = V.m128i_u64[0];
    T.Q[1] = V.m128i_u64[1];
    return T;
}

__forceinline
uint128_t __D128(const __m128d V)
{
    uint128_t T;
    T.F64[0] = V.m128d_f64[0];
    T.F64[1] = V.m128d_f64[1];
    return T;
}

__forceinline uint32_t  __S128D0(const __m128  V) { uint128_t *T = (uint128_t *)&V; return T->D[0]; }
__forceinline uint32_t  __S128D1(const __m128  V) { uint128_t *T = (uint128_t *)&V; return T->D[1]; }
__forceinline uint32_t  __S128D2(const __m128  V) { uint128_t *T = (uint128_t *)&V; return T->D[2]; }
__forceinline uint32_t  __S128D3(const __m128  V) { uint128_t *T = (uint128_t *)&V; return T->D[3]; }

__forceinline uint32_t  __U128W0(const __m128i V) { uint128_t *T = (uint128_t *)&V; return T->W[0]; }
__forceinline uint32_t  __U128D0(const __m128i V) { uint128_t *T = (uint128_t *)&V; return T->D[0]; }
__forceinline uint64_t __U128Q0(const __m128i V) { uint128_t *T = (uint128_t *)&V; return T->Q[0]; }
__forceinline uint64_t __U128Q1(const __m128i V) { uint128_t *T = (uint128_t *)&V; return T->Q[1]; }

__forceinline uint64_t __D128Q0(const __m128d V) { uint128_t *T = (uint128_t *)&V; return T->Q[0]; }
__forceinline uint64_t __D128Q1(const __m128d V) { uint128_t *T = (uint128_t *)&V; return T->Q[1]; }
__forceinline uint64_t __S128Q0(const __m128 V) { uint128_t *T = (uint128_t *)&V; return T->Q[0]; }
__forceinline uint64_t __S128Q1(const __m128 V) { uint128_t *T = (uint128_t *)&V; return T->Q[1]; }

__forceinline double __D128F0(const __m128d V) { uint128_t *T = (uint128_t *)&V; return *(double *)&T->Q[0]; }
__forceinline float  __S128F0(const __m128  V) { uint128_t *T = (uint128_t *)&V; return *(float *)&T->D[0]; }

__forceinline uint128_t __IMM8_128(const uint64_t C) { uint128_t T; T.Q[0] = C & 0xff; T.Q[1] = 0; return T; }

__forceinline
__m128  __MM128(const uint128_t U)
{
    __m128  T;
    T.m128_u64[0] = U.Q[0];
    T.m128_u64[1] = U.Q[1];
    return T;
}

__forceinline
__m128i __MI128(const uint128_t U)
{
    __m128i T;
    T.m128i_u64[0] = U.Q[0];
    T.m128i_u64[1] = U.Q[1];
    return T;
}

__forceinline
__m128d __MD128(const uint128_t U)
{
    __m128d T;
    T.m128d_f64[0] = U.F64[0];
    T.m128d_f64[1] = U.F64[1];
    return T;
}

__forceinline uint128_t __MAKEU8_U128(const uint8_t A, const uint8_t B, const uint8_t C, const uint8_t D, const uint8_t E, const uint8_t F, const uint8_t G, const uint8_t H, const uint8_t I, const uint8_t J, const uint8_t K, const uint8_t L, const uint8_t M, const uint8_t N, const uint8_t O, const uint8_t P)
{ uint128_t T; T.B[0] = A; T.B[1] = B; T.B[2] = C; T.B[3] = D; T.B[4] = E; T.B[5] = F; T.B[6] = G; T.B[7] = H; T.B[8] = I; T.B[9] = J; T.B[10] = K; T.B[11] = L; T.B[12] = M; T.B[13] = N; T.B[14] = O; T.B[15] = P; return T; }
__forceinline uint128_t __MAKEU8X16_U128(const uint8_t A)
{ uint128_t T; T.B[0] = A; T.B[1] = A; T.B[2] = A; T.B[3] = A; T.B[4] = A; T.B[5] = A; T.B[6] = A; T.B[7] = A; T.B[8] = A; T.B[9] = A; T.B[10] = A; T.B[11] = A; T.B[12] = A; T.B[13] = A; T.B[14] = A; T.B[15] = A; return T; }
__forceinline uint128_t __MAKEU16_U128(const uint16_t A, const uint16_t B, const uint16_t C, const uint16_t D, const uint16_t E, const uint16_t F, const uint16_t G, const uint16_t H)
{ uint128_t T; T.W[0] = A; T.W[1] = B; T.W[2] = C; T.W[3] = D; T.W[4] = E; T.W[5] = F; T.W[6] = G; T.W[7] = H; return T; }
__forceinline uint128_t __MAKEU16X8_U128(const uint16_t A)
{ uint128_t T; T.W[0] = A; T.W[1] = A; T.W[2] = A; T.W[3] = A; T.W[4] = A; T.W[5] = A; T.W[6] = A; T.W[7] = A; return T; }
__forceinline uint128_t __MAKEU32_U128(const uint32_t A, const uint32_t B, const uint32_t C, const uint32_t D)
{ uint128_t T; T.Q[0] = A | ((uint64_t)B << 32); T.Q[1] = C | ((uint64_t)D << 32); return T; }
__forceinline uint128_t __MAKEU32X4_U128(const uint32_t A)
{ return __MAKEU32_U128(A, A, A, A); }
__forceinline uint128_t __MAKEU64_U128(const uint64_t A, const uint64_t B)
{ uint128_t T; T.Q[0] = A; T.Q[1] = B; return T; }
__forceinline uint128_t __MAKEU64X2_U128(const uint64_t A)
{ uint128_t T; T.Q[0] = A; T.Q[1] = A; return T; }
__forceinline uint128_t __MAKEF32_U128(const float A, const float B, const float C, const float D)
{ uint128_t T; T.D[0] = *(uint32_t *)&A; T.D[1] = *(uint32_t *)&B; T.D[2] = *(uint32_t *)&C; T.D[3] = *(uint32_t *)&D ; return T; }
__forceinline uint128_t __MAKEF64_U128(const double A, const double B)
{ uint128_t T; T.Q[0] = *(uint64_t *)&A; T.Q[1] = *(uint64_t *)&B; return T; }

__forceinline uint128_t __INSERT8_U128(const uint128_t A, const uint8_t B, const int C) { uint128_t T = A; T.B[C & 0b1111] = B; return T; }
__forceinline uint128_t __INSERT16_U128(const uint128_t A, const uint16_t B, const int C) { uint128_t T = A; T.W[C & 0b111] = B; return T; }
__forceinline uint128_t __INSERT32_U128(const uint128_t A, const uint32_t B, const int C) { uint128_t T = A; T.D[C & 0b011] = B; return T; }
__forceinline uint128_t __INSERT64_U128(const uint128_t A, const uint64_t B, const int C) { uint128_t T = A; T.Q[C & 0b001] = B; return T; }

__forceinline void __MASKMOVEU128(const  uint128_t A, const uint128_t M, char *Mem)
{
    for (unsigned i = 0; i < 16; i++)
    {
        if (M.B[i] & 0x80)
        {
            Mem[i] = (char)A.B[i];
        }
    }
}

__forceinline int HasOneBit(const unsigned x) { return (x && !(x & (x - 1))) ? 1 : 0; }

//
// For a complete list of X64 intrinsics, see:
//     https://docs.microsoft.com/en-us/cpp/intrinsics/x64-amd64-intrinsics-list?view=msvc-160
//

// Misc. integer intrinsics not already defined in winnt.h

#if defined(_M_ARM64) || defined(_M_ARM64EC)

#define _udiv128(h,l,d,r)               UDIV128((h),(l),(d),(r))
#define _div128(h,l,d,r)                SDIV128((h),(l),(d),(r))

#if !defined(_umul128)
#define _umul128(x,y,h)                 UMUL128((x),(y),(h))
#endif

#if !defined(_mul128)
#define _mul128(x,y,h)                  SMUL128((x),(y),(h))
#endif

#define __rdtsc()                       RDTSC64()
#define __rdtscp(p)                     RDTSCP64(p)

#if !defined(__popcnt64)
#define __popcnt64(a)                   POPCNT64((uint64_t)a)
#endif

#if !defined(__popcnt)
#define __popcnt(a)                     ((uint32_t) POPCNT64((uint32_t)a))
#endif

#if !defined(__popcnt16)
#define __popcnt16(a)                   ((uint16_t) POPCNT64((uint16_t)a))
#endif

#if !defined(__cpuid)
#define __cpuid(a,b)                    CPUIDEX64((a),(b),0)
#endif

#if !defined(__cpuidex)
#define __cpuidex(a,b,c)                CPUIDEX64((a),(b),(c))
#endif

#if !defined(_fxrstor)
#define _fxrstor(p)                     _FXRSTOR(p)
#endif

#if !defined(_fxrstor64)
#define _fxrstor64(p)                   _FXRSTOR64(p)
#endif

#if !defined(_fxsave)
#define _fxsave(p)                      _FXSAVE(p)
#endif

#if !defined(_fxsave64)
#define _fxsave64(p)                    _FXSAVE64(p)
#endif

#endif

#define _mm_popcnt_u32(a)               (int32_t)POPCNT64((uint32_t)a)
#define _mm_popcnt_u64(a)               (int64_t)POPCNT64((uint64_t)a)

// ADDSS ADDSD ADDPS ADDPD

#define _mm_add_pd(a,b)                 __MD128(ADDPD128(__D128(a),__D128(b)))
#define _mm_add_sd(a,b)                 __MD128(ADDSD128(__D128(a),__D128(b)))
#define _mm_add_ps(a,b)                 __MM128(ADDPS128(__S128(a),__S128(b)))
#define _mm_add_ss(a,b)                 __MM128(ADDSS128(__S128(a),__S128(b)))

// ADDSUBPD ADDSUBPS

#define _mm_addsub_pd(a,b)              __MD128(ADDSUBPD128(__D128(a),__D128(b)))
#define _mm_addsub_ps(a,b)              __MM128(ADDSUBPS128(__S128(a),__S128(b)))

// AES

#if defined(_M_ARM64) || defined(_M_ARM64EC)

#define _mm_aesdec_si128(a,b)           __MI128(AESDEC128(__U128(a),__U128(b)))
#define _mm_aesenc_si128(a,b)           __MI128(AESENC128(__U128(a),__U128(b)))
#define _mm_aesdeclast_si128(a,b)       __MI128(AESDECLAST128(__U128(a),__U128(b)))
#define _mm_aesenclast_si128(a,b)       __MI128(AESENCLAST128(__U128(a),__U128(b)))
#define _mm_aesimc_si128(b)             __MI128(AESIMC128(__U128(b)))
#define _mm_aeskeygenassist_si128(b,i)  __MI128(AESKEYGENASSIST128(__U128(b),(i)))

#endif

// ALIGNR

#define _mm_alignr_epi8(a,b,i)          __MI128(PALIGNR128(__U128(a), __U128(b), (i)))

// BLENDVPS BLENDVPS

#define _mm_blend_epi16(a,b,i)          __MI128(PBLENDW128(__U128(a),__U128(b),(i)))
#define _mm_blend_pd(a,b,i)             __MD128(BLENDPD128(__D128(a),__D128(b),(i)))
#define _mm_blend_ps(a,b,i)             __MM128(BLENDPS128(__S128(a),__S128(b),(i)))

#define _mm_blendv_epi8(a,b,c)          __MI128(PBLENDVB128(__U128(a),__U128(b),__U128(c)))
#define _mm_blendv_pd(a,b,c)            __MD128(BLENDVPD128(__D128(a),__D128(b),__D128(c)))
#define _mm_blendv_ps(a,b,c)            __MM128(BLENDVPS128(__S128(a),__S128(b),__S128(c)))

// CMPEQ CMPLT etc.

#define _mm_cmp_pd(a,b,i)               __MD128(CMPPD128(__D128(a), __D128(b), (i)))
#define _mm_cmp_ps(a,b,i)               __MM128(CMPPS128(__S128(a), __S128(b), (i)))

#define _mm_cmp_sd(a,b,i)               __MD128(__MAKEU64_U128(CMPSD64(__D128Q0(a),__D128Q0(b),(i)),__D128Q1(a)))
#define _mm_cmp_ss(a,b,i)               __MM128(__MAKEU32_U128(CMPSS64(__S128D0(a),__S128D0(b),(i)),__S128D1(a),__S128D2(a),__S128D3(a)))
#define _mm_cmpr_sd(a,b,i)              __MD128(__MAKEU64_U128(CMPSD64(__D128Q0(a),__D128Q0(b),(i)),__D128Q1(b)))
#define _mm_cmpr_ss(a,b,i)              __MM128(__MAKEU32_U128(CMPSS64(__S128D0(a),__S128D0(b),(i)),__S128D1(b),__S128D2(b),__S128D3(b)))

#define _mm_cmpeq_pd(a,b)               _mm_cmp_pd((a),(b),_CMP_EQ_OQ)
#define _mm_cmple_pd(a,b)               _mm_cmp_pd((a),(b),_CMP_LE_OS)
#define _mm_cmplt_pd(a,b)               _mm_cmp_pd((a),(b),_CMP_LT_OS)
#define _mm_cmpunord_pd(a,b)            _mm_cmp_pd((a),(b),_CMP_UNORD_Q)
#define _mm_cmpneq_pd(a,b)              _mm_cmp_pd((a),(b),_CMP_NEQ_UQ)
#define _mm_cmpnle_pd(a,b)              _mm_cmp_pd((a),(b),_CMP_NLE_US)
#define _mm_cmpnlt_pd(a,b)              _mm_cmp_pd((a),(b),_CMP_NLT_US)
#define _mm_cmpord_pd(a,b)              _mm_cmp_pd((a),(b),_CMP_ORD_Q)
#define _mm_cmpge_pd(a,b)               _mm_cmp_pd((b),(a),_CMP_LE_OS)
#define _mm_cmpgt_pd(a,b)               _mm_cmp_pd((b),(a),_CMP_LT_OS)
#define _mm_cmpnge_pd(a,b)              _mm_cmp_pd((b),(a),_CMP_NLE_US)
#define _mm_cmpngt_pd(a,b)              _mm_cmp_pd((b),(a),_CMP_NLT_US)

#define _mm_cmpeq_ps(a,b)               _mm_cmp_ps((a),(b),_CMP_EQ_OQ)
#define _mm_cmple_ps(a,b)               _mm_cmp_ps((a),(b),_CMP_LE_OS)
#define _mm_cmplt_ps(a,b)               _mm_cmp_ps((a),(b),_CMP_LT_OS)
#define _mm_cmpunord_ps(a,b)            _mm_cmp_ps((a),(b),_CMP_UNORD_Q)
#define _mm_cmpneq_ps(a,b)              _mm_cmp_ps((a),(b),_CMP_NEQ_OQ)
#define _mm_cmpnle_ps(a,b)              _mm_cmp_ps((a),(b),_CMP_NLE_US)
#define _mm_cmpnlt_ps(a,b)              _mm_cmp_ps((a),(b),_CMP_NLT_US)
#define _mm_cmpord_ps(a,b)              _mm_cmp_ps((a),(b),_CMP_ORD_Q)
#define _mm_cmpge_ps(a,b)               _mm_cmp_ps((b),(a),_CMP_LE_OS)
#define _mm_cmpgt_ps(a,b)               _mm_cmp_ps((b),(a),_CMP_LT_OS)
#define _mm_cmpnge_ps(a,b)              _mm_cmp_ps((b),(a),_CMP_NLE_US)
#define _mm_cmpngt_ps(a,b)              _mm_cmp_ps((b),(a),_CMP_NLT_US)

#define _mm_cmpeq_sd(a,b)               _mm_cmp_sd((a),(b),_CMP_EQ_OQ)
#define _mm_cmple_sd(a,b)               _mm_cmp_sd((a),(b),_CMP_LE_OS)
#define _mm_cmplt_sd(a,b)               _mm_cmp_sd((a),(b),_CMP_LT_OS)
#define _mm_cmpunord_sd(a,b)            _mm_cmp_sd((a),(b),_CMP_UNORD_Q)
#define _mm_cmpneq_sd(a,b)              _mm_cmp_sd((a),(b),_CMP_NEQ_UQ)
#define _mm_cmpnle_sd(a,b)              _mm_cmp_sd((a),(b),_CMP_NLE_US)
#define _mm_cmpnlt_sd(a,b)              _mm_cmp_sd((a),(b),_CMP_NLT_US)
#define _mm_cmpord_sd(a,b)              _mm_cmp_sd((a),(b),_CMP_ORD_Q)
#define _mm_cmpge_sd(a,b)               _mm_cmpr_sd((b),(a),_CMP_LE_OS)
#define _mm_cmpgt_sd(a,b)               _mm_cmpr_sd((b),(a),_CMP_LT_OS)
#define _mm_cmpnge_sd(a,b)              _mm_cmpr_sd((b),(a),_CMP_NLE_US)
#define _mm_cmpngt_sd(a,b)              _mm_cmpr_sd((b),(a),_CMP_NLT_US)

#define _mm_cmpeq_ss(a,b)               _mm_cmp_ss((a),(b),_CMP_EQ_OQ)
#define _mm_cmple_ss(a,b)               _mm_cmp_ss((a),(b),_CMP_LE_OS)
#define _mm_cmplt_ss(a,b)               _mm_cmp_ss((a),(b),_CMP_LT_OS)
#define _mm_cmpunord_ss(a,b)            _mm_cmp_ss((a),(b),_CMP_UNORD_Q)
#define _mm_cmpneq_ss(a,b)              _mm_cmp_ss((a),(b),_CMP_NEQ_OQ)
#define _mm_cmpnle_ss(a,b)              _mm_cmp_ss((a),(b),_CMP_NLE_US)
#define _mm_cmpnlt_ss(a,b)              _mm_cmp_ss((a),(b),_CMP_NLT_US)
#define _mm_cmpord_ss(a,b)              _mm_cmp_ss((a),(b),_CMP_ORD_Q)
#define _mm_cmpge_ss(a,b)               _mm_cmpr_ss((b),(a),_CMP_LE_OS)
#define _mm_cmpgt_ss(a,b)               _mm_cmpr_ss((b),(a),_CMP_LT_OS)
#define _mm_cmpnge_ss(a,b)              _mm_cmpr_ss((b),(a),_CMP_NLE_US)
#define _mm_cmpngt_ss(a,b)              _mm_cmpr_ss((b),(a),_CMP_NLT_US)

#define _mm_cmpestra(a,m,b,n,i)         (0 == (0x041 & PCMPESTRF128(__U128(a), (m), __U128(b), (n), (i))))
#define _mm_cmpestrc(a,m,b,n,i)         (0 != (0x001 & PCMPESTRF128(__U128(a), (m), __U128(b), (n), (i))))
#define _mm_cmpestro(a,m,b,n,i)         (0 != (0x800 & PCMPESTRF128(__U128(a), (m), __U128(b), (n), (i))))
#define _mm_cmpestrs(a,m,b,n,i)         (0 != (0x080 & PCMPESTRF128(__U128(a), (m), __U128(b), (n), (i))))
#define _mm_cmpestrz(a,m,b,n,i)         (0 != (0x040 & PCMPESTRF128(__U128(a), (m), __U128(b), (n), (i))))
#define _mm_cmpistra(a,b,i)             (0 == (0x041 & PCMPISTRF128(__U128(a), __U128(b), (i))))
#define _mm_cmpistrc(a,b,i)             (0 != (0x001 & PCMPISTRF128(__U128(a), __U128(b), (i))))
#define _mm_cmpistro(a,b,i)             (0 != (0x800 & PCMPISTRF128(__U128(a), __U128(b), (i))))
#define _mm_cmpistrs(a,b,i)             (0 != (0x080 & PCMPISTRF128(__U128(a), __U128(b), (i))))
#define _mm_cmpistrz(a,b,i)             (0 != (0x040 & PCMPISTRF128(__U128(a), __U128(b), (i))))

#define _mm_cmpestri(a,m,b,n,i)         (int)PCMPESTRI128(__U128(a), (m), __U128(b), (n), (i), NULL)
#define _mm_cmpistri(a,b,i)             (int)PCMPISTRI128(__U128(a), __U128(b), (i), NULL)

#define _mm_cmpestrm(a,m,b,n,i)         __MI128(PCMPESTRM128(__U128(a), (m), __U128(b), (n), (i), NULL))
#define _mm_cmpistrm(a,b,i)             __MI128(PCMPISTRM128(__U128(a), __U128(b), (i), NULL))

//
// COMSS COMSD
//
// ZF mask is 0x40
// PF mask is 0x04
// CF mask is 0x01
//
// unordered sets ZF PF CF
// greater-than sets nothing
// less-than sets CF
// equal sets ZF
//

#define _mm_comieq_sd(a,b)              (0x040 == (0x051 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_comieq_ss(a,b)              (0x040 == (0x051 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_comige_sd(a,b)              (0x000 == (0x001 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_comige_ss(a,b)              (0x000 == (0x001 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_comigt_sd(a,b)              (0x000 == (0x041 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_comigt_ss(a,b)              (0x000 == (0x041 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_comile_sd(a,b)              (HasOneBit(0x041 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_comile_ss(a,b)              (HasOneBit(0x041 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_comilt_sd(a,b)              (0x001 == (0x051 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_comilt_ss(a,b)              (0x001 == (0x051 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_comineq_sd(a,b)             (0x040 != (0x051 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_comineq_ss(a,b)             (0x040 != (0x051 & UCOMISS128(__S128(a), __S128(b))))

// CRC32

#define _mm_crc32_u8(a,b)               ((unsigned)_CRC64((uint32_t)a,(uint8_t)b,_CRC32_POLYNOMIAL,_BITS_PER_BYTE))
#define _mm_crc32_u16(a,b)              ((unsigned)_CRC64((uint32_t)a,(uint16_t)b,_CRC32_POLYNOMIAL,_BITS_PER_WORD))
#define _mm_crc32_u32(a,b)              ((unsigned)_CRC64((uint32_t)a,(uint32_t)b,_CRC32_POLYNOMIAL,_BITS_PER_LONG))
#define _mm_crc32_u64(a,b)              ((unsigned)_CRC64((uint32_t)a,(uint64_t)b,_CRC32_POLYNOMIAL,_BITS_PER_QUAD))

// CVTDQ2PD CVTDQ2PS

#define _mm_cvtepi32_ps(a)              __MM128(CVTDQ2PS128(__U128(a)))
#define _mm_cvtepi32_pd(a)              __MD128(CVTDQ2PD128(__U128Q0(a)))

// CVTPD2DQ CVTPS2DQ

#define _mm_cvtps_epi32(a)              __MI128(CVTPS2DQ128(__S128(a), _MM_FROUND_TO_NEAREST_INT))
#define _mm_cvttps_epi32(a)             __MI128(CVTPS2DQ128(__S128(a), _MM_FROUND_TO_ZERO))
#define _mm_cvtpd_epi32(a)              __MI128(CVTPD2DQ128(__D128(a), _MM_FROUND_TO_NEAREST_INT))
#define _mm_cvttpd_epi32(a)             __MI128(CVTPD2DQ128(__D128(a), _MM_FROUND_TO_ZERO))

// CVTSI2SS CVTSS2SI CVTTSS2SI CVTSD2SI CVTTSD2SI

#define _mm_cvt_ss2si(a)                (int)_CVTSS2SI32(__S128D0(a), _MM_FROUND_TO_NEAREST_INT)
#define _mm_cvtt_ss2si(a)               (int)_CVTSS2SI32(__S128D0(a), _MM_FROUND_TO_ZERO)
#define _mm_cvt_si2ss(s,a)              __MM128(CVTSI2SS128(__S128(s),(int32_t)(a)))
#define _mm_cvti64_ss(s,a)              __MM128(CVTSI2SS128(__S128(s),(int64_t)(a)))
#define _mm_cvtsd_si32(a)               (int)_CVTSD2SI32(__D128Q0(a), _MM_FROUND_TO_NEAREST_INT)
#define _mm_cvttsd_si32(a)              (int)_CVTSD2SI32(__D128Q0(a), _MM_FROUND_TO_ZERO)
#define _mm_cvtss_si64x(a)              (int64_t)_CVTSD2SI64(CVTSS2SD64(__S128D0(a)), _MM_FROUND_TO_NEAREST_INT)
#define _mm_cvttss_si64x(a)             (int64_t)_CVTSD2SI64(CVTSS2SD64(__S128D0(a)), _MM_FROUND_TO_ZERO)
#define _mm_cvtsd_si64x(a)              (int64_t)_CVTSD2SI64(__D128Q0(a), _MM_FROUND_TO_NEAREST_INT)
#define _mm_cvttsd_si64x(a)             (int64_t)_CVTSD2SI64(__D128Q0(a), _MM_FROUND_TO_ZERO)

// CVT other...

#define _mm_cvtsd_ss(a,b)               __MM128(__MAKEU32_U128(CVTSD2SS64(__D128Q0(b)),__S128D1(a),__S128D2(a),__S128D3(a)))
#define _mm_cvtss_sd(a,b)               __MD128(__MAKEU64_U128(CVTSS2SD64(__S128D0(b)),__D128Q1(a)))
#define _mm_cvtpd_ps(a)                 __MM128(__MAKEU32_U128(CVTSD2SS64(__D128Q0(a)),CVTSD2SS64(__D128Q1(a)),0,0))
#define _mm_cvtps_pd(a)                 __MD128(__MAKEU64_U128(CVTSS2SD64(__S128D0(a)),CVTSS2SD64(__S128D1(a))))
#define _mm_cvtsi64_ss(a,b)             __MM128(CVTSI2SS128(__S128(a),(int64_t)(b)))
#define _mm_cvtsi64x_ss(a,b)            __MM128(CVTSI2SS128(__S128(a),(int64_t)(b)))
#define _mm_cvtsi32_sd(a,b)             __MD128(CVTSI2SD128(__D128(a),(int32_t)(b)))
#define _mm_cvtsi64_sd(a,b)             __MD128(CVTSI2SD128(__D128(a),(int64_t)(b)))
#define _mm_cvtsi64x_sd(a,b)            __MD128(CVTSI2SD128(__D128(a),(int64_t)(b)))

// DIVSS DIVSD DIVPS DIVPD

#define _mm_div_pd(a,b)                 __MD128(DIVPD128(__D128(a),__D128(b)))
#define _mm_div_sd(a,b)                 __MD128(DIVSD128(__D128(a),__D128(b)))
#define _mm_div_ps(a,b)                 __MM128(DIVPS128(__S128(a),__S128(b)))
#define _mm_div_ss(a,b)                 __MM128(DIVSS128(__S128(a),__S128(b)))

// DPPD DPPS

#define _mm_dp_pd(a,b,i)                __MD128(DPPD128(__D128(a), __D128(b), i))
#define _mm_dp_ps(a,b,i)                __MM128(DPPS128(__S128(a), __S128(b), i))

// EXTRACTPS

#define _mm_extract_ps(a,b)             PEXTRD128(__S128(a), (b))

// HADDPD HADDPS

#define _mm_hadd_pd(a,b)                __MD128(HADDPD128(__D128(a),__D128(b)))
#define _mm_hadd_ps(a,b)                __MM128(HADDPS128(__S128(a),__S128(b)))

// HSUBPD HSUBPS

#define _mm_hsub_pd(a,b)                __MD128(HSUBPD128(__D128(a),__D128(b)))
#define _mm_hsub_ps(a,b)                __MM128(HSUBPS128(__S128(a),__S128(b)))

// INSERTPS

#define _mm_insert_ps(a,b,i)            __MM128(INSERTPS128(__S128(a), __S128(b), (i)))

// LDDQU

#define _mm_lddqu_si128(a)              _mm_loadu_si128(a)

// MASKMOVDQU

#define _mm_maskmoveu_si128(a,m,p)      __MASKMOVEU128(__U128(a), __U128(m), p)

// MOVMSKPD MOVMSKPS PMOVMSKB

#define _mm_movemask_epi8(a)            PMOVMSKB128(__U128(a))
#define _mm_movemask_pd(a)              MOVMSKPD128(__D128(a))
#define _mm_movemask_ps(a)              MOVMSKPS128(__S128(a))

// MOVD

#define _mm_cvtsi128_si32(a)            ((int)__U128D0(a))
#define _mm_cvtsi32_si128(a)            __MI128(__MAKEU32_U128((unsigned)(a), 0, 0, 0))

// MOVDDUP MOVSLDUP

#define _mm_loaddup_pd(a)               _mm_load1_pd(a)
#define _mm_movedup_pd(a)               __MD128(MOVDDUP128(__D128(a)))
#define _mm_moveldup_ps(a)              __MM128(MOVSLDUP128(__S128(a)))

// MOVDQA MOVAPD MOVAPS

#define _mm_load_si128(a)               (*(a))
#define _mm_store_si128(a,b)            (void)(*(a) = (b))

#define _mm_load_pd(a)                  (*((__m128d const*)(a)))
#define _mm_store_pd(a,b)               (void)(*((__m128d*)(a)) = (b))

#define _mm_load_ps(a)                  (*((__m128 const*)(a)))
#define _mm_store_ps(a,b)               (void)(*((__m128*)(a)) = (b))

// MOVDQU MOVUPD MOVUPS

#define _mm_loadu_si128(a)              __MI128(__MAKEU64_U128(*(uint64_t *)(a), ((uint64_t *)(a))[1]))
#define _mm_storeu_si128(a,b)           (void)(*(uint64_t *)(a) = __U128Q0(b), ((uint64_t *)(a))[1] = __U128Q1(b))

#define _mm_loadu_pd(a)                 __MD128(__MAKEU64_U128(*(uint64_t *)(a), ((uint64_t *)(a))[1]))
#define _mm_storeu_pd(a,b)              (void)(*(uint64_t *)(a) = __D128Q0(b), ((uint64_t *)(a))[1] = __D128Q1(b))

#define _mm_loadu_ps(a)                 __MM128(__MAKEU64_U128(*(uint64_t *)(a), ((uint64_t *)(a))[1]))
#define _mm_storeu_ps(a,b)              (void)(*(uint64_t *)(a) = __S128Q0(b), ((uint64_t *)(a))[1] = __S128Q1(b))

// MOVHLPS MOVLHPS

#define _mm_movehl_ps(a,b)              __MM128(__MAKEU32_U128(__S128D2(b), __S128D3(b), __S128D2(a), __S128D3(a)))
#define _mm_movelh_ps(a,b)              __MM128(__MAKEU32_U128(__S128D0(a), __S128D1(a), __S128D0(b), __S128D1(b)))

// MOVLPD MOVHPD

#define _mm_loadl_pd(a,b)               __MD128(__MAKEU64_U128(*(uint64_t *)(b), __D128Q1(a)))
#define _mm_loadh_pd(a,b)               __MD128(__MAKEU64_U128(__D128Q0(a), *(uint64_t *)(b)))

#define _mm_storel_pd(a,b)              (void)(*(uint64_t *)(a) = __D128Q0(b))
#define _mm_storeh_pd(a,b)              (void)(*(uint64_t *)(a) = __D128Q1(b))

// MOVLPS MOVHPS

#define _mm_loadl_pi(a,b)               __MM128(__MAKEU64_U128(*(uint64_t *)(b), __S128Q1(a)))
#define _mm_loadh_pi(a,b)               __MM128(__MAKEU64_U128(__S128Q0(a), *(uint64_t *)(b)))

#define _mm_storel_pi(a,b)              (void)(*(uint64_t *)(a) = __S128Q0(b))
#define _mm_storeh_pi(a,b)              (void)(*(uint64_t *)(a) = __S128Q1(b))

// MOVQ

#define _mm_storel_epi64(a,b)           (void)(*(uint64_t *)(a) = __U128Q0(b))
#define _mm_loadl_epi64(a)              __MI128(__MAKEU64_U128(*(uint64_t *)(a), 0))

#define _mm_cvtsi128_si64(a)            ((int64_t)__U128Q0(a))
#define _mm_cvtsi128_si64x(a)           ((int64_t)__U128Q0(a))
#define _mm_cvtsi64_si128(a)            __MI128(__MAKEU64_U128((uint64_t)(a), 0))
#define _mm_cvtsi64x_si128(a)           __MI128(__MAKEU64_U128((uint64_t)(a), 0))

#define _mm_move_epi64(a)               __MI128(__MAKEU64_U128(__U128Q0(a), 0))

// MOVSHDUP

#define _mm_movehdup_ps(a)              __MM128(MOVSHDUP128(__S128(a)))

// MOVSS MOVSD

#define _mm_load_ss(a)                  __MM128(__MAKEU32_U128(*(uint32_t *)(a), 0, 0, 0))
#define _mm_move_ss(a,b)                __MM128(__MAKEU32_U128(__S128D0(b), __S128D1(a), __S128D2(a), __S128D3(a)))
#define _mm_store_ss(a,b)               (void)(*(uint32_t *)(a) = __S128D0(b))
#define _mm_cvtss_f32(a)                (__S128F0(a))

#define _mm_load_sd(a)                  __MD128(__MAKEU64_U128(*(uint64_t *)(a), 0))
#define _mm_move_sd(a,b)                __MD128(__MAKEU64_U128(__D128Q0(b), __D128Q1(a)))
#define _mm_store_sd(a,b)               (void)(*(uint64_t *)(a) = __D128Q0(b))
#define _mm_cvtsd_f64(a)                (__D128F0(a))

// Map the non-temporals to existing temporals

#define _mm_stream_load_si128(p)        _mm_load_si128(p)
#define _mm_stream_si128(p,b)           _mm_store_si128(p,b)
#define _mm_stream_si32(p,b)            (void)(*(int32_t *)(p) = b)
#define _mm_stream_si64x(p,b)           (void)(*(int64_t *)(p) = b)

#define _mm_stream_pd(p,b)              _mm_store_pd(p,b)
#define _mm_stream_ps(p,b)              _mm_store_ps(p,b)
#define _mm_stream_sd(p,b)              _mm_store_sd(p,b)
#define _mm_stream_ss(p,b)              _mm_store_ss(p,b)

// MAXSS MAXSD MAXPS MAXPD

#define _mm_max_pd(a,b)                 __MD128(MAXPD128(__D128(a),__D128(b)))
#define _mm_max_sd(a,b)                 __MD128(MAXSD128(__D128(a),__D128(b)))
#define _mm_max_ps(a,b)                 __MM128(MAXPS128(__S128(a),__S128(b)))
#define _mm_max_ss(a,b)                 __MM128(MAXSS128(__S128(a),__S128(b)))

// MINSS MINSD MINPS MINPD

#define _mm_min_pd(a,b)                 __MD128(MINPD128(__D128(a),__D128(b)))
#define _mm_min_sd(a,b)                 __MD128(MINSD128(__D128(a),__D128(b)))
#define _mm_min_ps(a,b)                 __MM128(MINPS128(__S128(a),__S128(b)))
#define _mm_min_ss(a,b)                 __MM128(MINSS128(__S128(a),__S128(b)))

// MPSADBW

#define _mm_mpsadbw_epu8(a,b,i)         __MI128(MPSADBW128(__U128(a), __U128(b), (i)))

// MULSS MULSD MULPS MULPD

#define _mm_mul_pd(a,b)                 __MD128(MULPD128(__D128(a),__D128(b)))
#define _mm_mul_sd(a,b)                 __MD128(MULSD128(__D128(a),__D128(b)))
#define _mm_mul_ps(a,b)                 __MM128(MULPS128(__S128(a),__S128(b)))
#define _mm_mul_ss(a,b)                 __MM128(MULSS128(__S128(a),__S128(b)))

// PCLMULQDQ

#define _mm_clmulepi64_si128(a,b,c)     __MI128(PCLMULQDQ128(__U128(a),__U128(b),(c)))

// PHMINPOSUW

#define _mm_minpos_epu16(b)             __MI128(PHMINPOSUW128(__U128(b)))

// PSIGN

#define _mm_sign_epi8(a,b)              __MI128(PSIGN128(__U128(a), __U128(b), _WIDEMASK_BYTE, _BITS_PER_BYTE))
#define _mm_sign_epi16(a,b)             __MI128(PSIGN128(__U128(a), __U128(b), _WIDEMASK_WORD, _BITS_PER_WORD))
#define _mm_sign_epi32(a,b)             __MI128(PSIGN128(__U128(a), __U128(b), _WIDEMASK_LONG, _BITS_PER_LONG))

// Reciprocals

#define _mm_rcp_ps(a)                   __MM128(RCPPS128(__S128(a)))
#define _mm_rsqrt_ps(a)                 __MM128(RSQRTPS128(__S128(a)))
#define _mm_rcp_ss(a)                   __MM128(RCPSS128(__S128(a)))
#define _mm_rsqrt_ss(a)                 __MM128(RSQRTSS128(__S128(a)))

// ROUND

#define _mm_round_pd(b,i)               __MD128(ROUNDPD128(__D128(b), i))
#define _mm_round_ps(b,i)               __MM128(ROUNDPS128(__S128(b), i))
#define _mm_round_sd(a,b,i)             __MD128(ROUNDSD128(__D128(a), __D128(b), i))
#define _mm_round_ss(a,b,i)             __MM128(ROUNDSS128(__S128(a), __S128(b), i))

// SHUFPD SHUFPS

#define _mm_shuffle_ps(a,b,i)           __MM128(SHUFPS128(__S128(a), __S128(b), (i)))
#define _mm_shuffle_pd(a,b,i)           __MD128(SHUFPD128(__D128(a), __D128(b), (i)))

// SQRTSS SQRTSD SQRTPS SQRTPD
// Note: sqrt_sd really is not like the others!

#define _mm_sqrt_pd(a)                  __MD128(SQRTPD128(__D128(a)))
#define _mm_sqrt_sd(a,b)                __MD128(SQRTSD128(__D128(a),__D128(b)))
#define _mm_sqrt_ps(a)                  __MM128(SQRTPS128(__S128(a)))
#define _mm_sqrt_ss(a)                  __MM128(SQRTSS128(__S128(a)))

// SUBSS SUBSD SUBPS SUBPD

#define _mm_sub_pd(a,b)                 __MD128(SUBPD128(__D128(a),__D128(b)))
#define _mm_sub_sd(a,b)                 __MD128(SUBSD128(__D128(a),__D128(b)))
#define _mm_sub_ps(a,b)                 __MM128(SUBPS128(__S128(a),__S128(b)))
#define _mm_sub_ss(a,b)                 __MM128(SUBSS128(__S128(a),__S128(b)))

//
// UCOMSS UCOMSD
//
// ZF mask is 0x40
// PF mask is 0x04
// CF mask is 0x01
//
// unordered sets ZF PF CF
// greater-than sets nothing
// less-than sets CF
// equal sets ZF
//

#define _mm_ucomieq_sd(a,b)             (0x040 == (0x051 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_ucomieq_ss(a,b)             (0x040 == (0x051 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_ucomige_sd(a,b)             (0x000 == (0x001 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_ucomige_ss(a,b)             (0x000 == (0x001 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_ucomigt_sd(a,b)             (0x000 == (0x041 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_ucomigt_ss(a,b)             (0x000 == (0x041 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_ucomile_sd(a,b)             (HasOneBit(0x041 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_ucomile_ss(a,b)             (HasOneBit(0x041 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_ucomilt_sd(a,b)             (0x001 == (0x051 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_ucomilt_ss(a,b)             (0x001 == (0x051 & UCOMISS128(__S128(a), __S128(b))))
#define _mm_ucomineq_sd(a,b)            (0x040 != (0x051 & UCOMISD128(__D128(a), __D128(b))))
#define _mm_ucomineq_ss(a,b)            (0x040 != (0x051 & UCOMISS128(__S128(a), __S128(b))))

// UNPCK

#define _mm_unpackhi_pd(a,b)            __MD128(UNPCKHPD128(__D128(a),__D128(b)))
#define _mm_unpacklo_pd(a,b)            __MD128(UNPCKLPD128(__D128(a),__D128(b)))
#define _mm_unpackhi_ps(a,b)            __MM128(UNPCKHPS128(__S128(a),__S128(b)))
#define _mm_unpacklo_ps(a,b)            __MM128(UNPCKLPS128(__S128(a),__S128(b)))

// PABS[W/D/Q]

#define _mm_abs_epi8(b)                 __MI128(PABS128(__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_abs_epi16(b)                __MI128(PABS128(__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_abs_epi32(b)                __MI128(PABS128(__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

// PACKSSWB

#define _mm_packs_epi16(a,b)            __MI128(PACKSSWB128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))

// PACKSSDW

#define _mm_packs_epi32(a,b)            __MI128(PACKSSDW128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

// PACKUSWB

#define _mm_packus_epi16(a,b)           __MI128(PACKUSWB128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))

// PACKUSDW

#define _mm_packus_epi32(a,b)           __MI128(PACKUSDW128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

// PAND

#define _mm_and_si128(a,b)              __MI128(PAND128(__U128(a),__U128(b)))
#define _mm_and_ps(a,b)                 __MM128(PAND128(__S128(a),__S128(b)))
#define _mm_and_pd(a,b)                 __MD128(PAND128(__D128(a),__D128(b)))

// PANDN

#define _mm_andnot_si128(a,b)           __MI128(PANDN128(__U128(a),__U128(b)))
#define _mm_andnot_ps(a,b)              __MM128(PANDN128(__S128(a),__S128(b)))
#define _mm_andnot_pd(a,b)              __MD128(PANDN128(__D128(a),__D128(b)))

// PADD[B/W/D/Q]

#define _mm_add_epi8(a,b)               __MI128(PADD128(__U128(a),__U128(b),_WIDEMASK_BYTE))
#define _mm_add_epi16(a,b)              __MI128(PADD128(__U128(a),__U128(b),_WIDEMASK_WORD))
#define _mm_add_epi32(a,b)              __MI128(PADD128(__U128(a),__U128(b),_WIDEMASK_LONG))
#define _mm_add_epi64(a,b)              __MI128(PADD128(__U128(a),__U128(b),_WIDEMASK_QUAD))

// PADDS[B/W/D/Q]

#define _mm_adds_epi8(a,b)              __MI128(PADDSI128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_adds_epi16(a,b)             __MI128(PADDSI128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))

// PADDUS[B/W/D/Q]

#define _mm_adds_epu8(a,b)              __MI128(PADDSU128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_adds_epu16(a,b)             __MI128(PADDSU128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))

// PAVG[B/W]

#define _mm_avg_epu8(a,b)               __MI128(PAVG128(__U128(a),__U128(b),_WIDEMASK_BYTE))
#define _mm_avg_epu16(a,b)              __MI128(PAVG128(__U128(a),__U128(b),_WIDEMASK_WORD))

// PCMPEQ[B/W/D]

#define _mm_cmpeq_epi8(a,b)             __MI128(PCMPEQ128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_cmpeq_epi16(a,b)            __MI128(PCMPEQ128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_cmpeq_epi32(a,b)            __MI128(PCMPEQ128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_cmpeq_epi64(a,b)            __MI128(PCMPEQ128(__U128(a),__U128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))

// PCMPGT[B/W/D]

#define _mm_cmpgt_epi8(a,b)             __MI128(PCMPGT128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_cmpgt_epi16(a,b)            __MI128(PCMPGT128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_cmpgt_epi32(a,b)            __MI128(PCMPGT128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_cmpgt_epi64(a,b)            __MI128(PCMPGT128(__U128(a),__U128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))

#define _mm_cmplt_epi8(a,b)             __MI128(PCMPGT128(__U128(b),__U128(a),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_cmplt_epi16(a,b)            __MI128(PCMPGT128(__U128(b),__U128(a),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_cmplt_epi32(a,b)            __MI128(PCMPGT128(__U128(b),__U128(a),_WIDEMASK_LONG,_BITS_PER_LONG))

// PEXTR[B/W/D/Q]

#define _mm_extract_epi8(a,b)           PEXTRB128(__U128(a), (b))
#define _mm_extract_epi16(a,b)          PEXTRW128(__U128(a), (b))
#define _mm_extract_epi32(a,b)          PEXTRD128(__U128(a), (b))
#define _mm_extract_epi64(a,b)          PEXTRQ128(__U128(a), (b))

// PHADD[W/D] PHADDSW

#define _mm_hadd_epi16(a,b)             __MI128(PHADDW128(__U128(a),__U128(b)))
#define _mm_hadd_epi32(a,b)             __MI128(PHADDD128(__U128(a),__U128(b)))
#define _mm_hadds_epi16(a,b)            __MI128(PHADDSW128(__U128(a),__U128(b)))

// PHSUB[W/D] PHSUBSW

#define _mm_hsub_epi16(a,b)             __MI128(PHSUBW128(__U128(a),__U128(b)))
#define _mm_hsub_epi32(a,b)             __MI128(PHSUBD128(__U128(a),__U128(b)))
#define _mm_hsubs_epi16(a,b)            __MI128(PHSUBSW128(__U128(a),__U128(b)))

// PINSR[B/W/D/Q]

#define _mm_insert_epi8(a,b,c)          __MI128(__INSERT8_U128(__U128(a),(uint16_t)(b),(c)))
#define _mm_insert_epi16(a,b,c)         __MI128(__INSERT16_U128(__U128(a),(uint16_t)(b),(c)))
#define _mm_insert_epi32(a,b,c)         __MI128(__INSERT32_U128(__U128(a),(uint32_t)(b),(c)))
#define _mm_insert_epi64(a,b,c)         __MI128(__INSERT64_U128(__U128(a),(uint64_t)(b),(c)))

// PMADDUSBW

#define _mm_maddubs_epi16(a,b)          __MI128(PMADDUBSW128(__U128(a),__U128(b)))

// PMADDWD

#define _mm_madd_epi16(a,b)             __MI128(PMADDWD128(__U128(a),__U128(b)))

// PMOVSX PMOVZX [B/W/D/Q]

#define _mm_cvtepi8_epi16(a)            __MI128(PMOVSXBW128(__U128Q0(a)))
#define _mm_cvtepi8_epi32(a)            __MI128(PMOVSXBD128(__U128D0(a)))
#define _mm_cvtepi8_epi64(a)            __MI128(PMOVSXBQ128(__U128W0(a)))
#define _mm_cvtepi16_epi32(a)           __MI128(PMOVSXWD128(__U128Q0(a)))
#define _mm_cvtepi16_epi64(a)           __MI128(PMOVSXWQ128(__U128D0(a)))
#define _mm_cvtepi32_epi64(a)           __MI128(PMOVSXDQ128(__U128Q0(a)))

#define _mm_cvtepu8_epi16(a)            __MI128(PMOVZXBW128(__U128Q0(a)))
#define _mm_cvtepu8_epi32(a)            __MI128(PMOVZXBD128(__U128D0(a)))
#define _mm_cvtepu8_epi64(a)            __MI128(PMOVZXBQ128(__U128W0(a)))
#define _mm_cvtepu16_epi32(a)           __MI128(PMOVZXWD128(__U128Q0(a)))
#define _mm_cvtepu16_epi64(a)           __MI128(PMOVZXWQ128(__U128D0(a)))
#define _mm_cvtepu32_epi64(a)           __MI128(PMOVZXDQ128(__U128Q0(a)))

// PMAXS PMAXU PMINS PMINU [B/W/D]

#define _mm_max_epi8(a,b)               __MI128(PMAXI128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_max_epi16(a,b)              __MI128(PMAXI128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_max_epi32(a,b)              __MI128(PMAXI128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

#define _mm_max_epu8(a,b)               __MI128(PMAXU128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_max_epu16(a,b)              __MI128(PMAXU128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_max_epu32(a,b)              __MI128(PMAXU128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

#define _mm_min_epi8(a,b)               __MI128(PMINI128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_min_epi16(a,b)              __MI128(PMINI128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_min_epi32(a,b)              __MI128(PMINI128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

#define _mm_min_epu8(a,b)               __MI128(PMINU128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_min_epu16(a,b)              __MI128(PMINU128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_min_epu32(a,b)              __MI128(PMINU128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))

// PMULDQ PMULUDQ

#define _mm_mul_epi32(a,b)              __MI128(PMULDQ128(__U128(a),__U128(b)))
#define _mm_mul_epu32(a,b)              __MI128(PMULUDQ128(__U128(a),__U128(b)))

// PMULLD

#define _mm_mullo_epi32(a,b)            __MI128(PMULLD128(__U128(a),__U128(b)))

// PMULH

#define _mm_mulhi_epi16(a,b)            __MI128(PMULHW128(__U128(a),__U128(b)))
#define _mm_mulhi_epu16(a,b)            __MI128(PMULHUW128(__U128(a),__U128(b)))
#define _mm_mullo_epi16(a,b)            __MI128(PMULLW128(__U128(a),__U128(b)))

// PMULHRSW

#define _mm_mulhrs_epi16(a,b)           __MI128(PMULHRSW128(__U128(a),__U128(b)))

// POR

#define _mm_or_si128(a,b)               __MI128(POR128(__U128(a),__U128(b)))
#define _mm_or_ps(a,b)                  __MM128(POR128(__S128(a),__S128(b)))
#define _mm_or_pd(a,b)                  __MD128(POR128(__D128(a),__D128(b)))

// PREFETCH[0/1/2/NTA]

#define _mm_prefetch(a,b)               __noop((a),(b))

// PSADBW

#define _mm_sad_epu8(a,b)               __MI128(PSADBW128(__U128(a),__U128(b)))

// PSHUFB PSHUFD

#define _mm_shuffle_epi8(a,b)           __MI128(PSHUFB128(__U128(a),__U128(b)))
#define _mm_shuffle_epi32(a,i)          __MI128(PSHUFD128(__U128(a),(i)))

// PSHUFHW PSHUFLW

#define _mm_shufflehi_epi16(a,b)        __MI128(__MAKEU64_U128(__U128Q0(a),PSHUFW64(__U128Q1(a),b)))
#define _mm_shufflelo_epi16(a,b)        __MI128(__MAKEU64_U128(PSHUFW64(__U128Q0(a),b),__U128Q1(a)))

// PSLL[W/L/Q]

#define _mm_sll_epi16(a,b)              __MI128(PSLL128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_sll_epi32(a,b)              __MI128(PSLL128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_sll_epi64(a,b)              __MI128(PSLL128(__U128(a),__U128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))
#define _mm_slli_epi16(a,b)             __MI128(PSLL128(__U128(a),__IMM8_128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_slli_epi32(a,b)             __MI128(PSLL128(__U128(a),__IMM8_128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_slli_epi64(a,b)             __MI128(PSLL128(__U128(a),__IMM8_128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))

// PSLLDQ

#define _mm_slli_si128(a,b)             __MI128(PSLLDQ(__U128(a),(b) & 0xff))

// PSRA[W/L/Q]

#define _mm_sra_epi16(a,b)              __MI128(PSRA128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_sra_epi32(a,b)              __MI128(PSRA128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_sra_epi64(a,b)              __MI128(PSRA128(__U128(a),__U128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))
#define _mm_srai_epi16(a,b)             __MI128(PSRA128(__U128(a),__IMM8_128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_srai_epi32(a,b)             __MI128(PSRA128(__U128(a),__IMM8_128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_srai_epi64(a,b)             __MI128(PSRA128(__U128(a),__IMM8_128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))

// PSRL[W/L/Q]

#define _mm_srl_epi16(a,b)              __MI128(PSRL128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_srl_epi32(a,b)              __MI128(PSRL128(__U128(a),__U128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_srl_epi64(a,b)              __MI128(PSRL128(__U128(a),__U128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))
#define _mm_srli_epi16(a,b)             __MI128(PSRL128(__U128(a),__IMM8_128(b),_WIDEMASK_WORD,_BITS_PER_WORD))
#define _mm_srli_epi32(a,b)             __MI128(PSRL128(__U128(a),__IMM8_128(b),_WIDEMASK_LONG,_BITS_PER_LONG))
#define _mm_srli_epi64(a,b)             __MI128(PSRL128(__U128(a),__IMM8_128(b),_WIDEMASK_QUAD,_BITS_PER_QUAD))

// PSRLDQ

#define _mm_srli_si128(a,b)             __MI128(PSRLDQ(__U128(a), (b) & 0xff))

// PSUB[B/W/D/Q]

#define _mm_sub_epi8(a,b)               __MI128(PSUB128(__U128(a),__U128(b),_WIDEMASK_BYTE))
#define _mm_sub_epi16(a,b)              __MI128(PSUB128(__U128(a),__U128(b),_WIDEMASK_WORD))
#define _mm_sub_epi32(a,b)              __MI128(PSUB128(__U128(a),__U128(b),_WIDEMASK_LONG))
#define _mm_sub_epi64(a,b)              __MI128(PSUB128(__U128(a),__U128(b),_WIDEMASK_QUAD))

// PSUBS[B/W]

#define _mm_subs_epi8(a,b)              __MI128(PSUBSI128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_subs_epi16(a,b)             __MI128(PSUBSI128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))

// PSUBUS[B/W]

#define _mm_subs_epu8(a,b)              __MI128(PSUBSU128(__U128(a),__U128(b),_WIDEMASK_BYTE,_BITS_PER_BYTE))
#define _mm_subs_epu16(a,b)             __MI128(PSUBSU128(__U128(a),__U128(b),_WIDEMASK_WORD,_BITS_PER_WORD))

// PUNPCKH

#define _mm_unpackhi_epi8(a,b)          __MI128(PUNPCKHBW128(__U128(a),__U128(b)))
#define _mm_unpackhi_epi16(a,b)         __MI128(PUNPCKHWD128(__U128(a),__U128(b)))
#define _mm_unpackhi_epi32(a,b)         __MI128(PUNPCKHDQ128(__U128(a),__U128(b)))
#define _mm_unpackhi_epi64(a,b)         __MI128(PUNPCKHQ128(__U128(a),__U128(b)))

// PUNPCKL

#define _mm_unpacklo_epi8(a,b)          __MI128(PUNPCKLBW128(__U128(a),__U128(b)))
#define _mm_unpacklo_epi16(a,b)         __MI128(PUNPCKLWD128(__U128(a),__U128(b)))
#define _mm_unpacklo_epi32(a,b)         __MI128(PUNPCKLDQ128(__U128(a),__U128(b)))
#define _mm_unpacklo_epi64(a,b)         __MI128(PUNPCKQ128(__U128(a),__U128(b)))

// PTEST

#define _mm_testc_si128(a,b)            (int)(PTESTC128(__U128(a),__U128(b)))
#define _mm_testnzc_si128(a,b)          (int)((PTESTZ128(__U128(a),__U128(b)) | PTESTC128(__U128(a),__U128(b))) ? 0 : 1)
#define _mm_testz_si128(a,b)            (int)(PTESTZ128(__U128(a),__U128(b)))

// PXOR

#define _mm_xor_si128(a,b)              __MI128(PXOR128(__U128(a),__U128(b)))
#define _mm_xor_ps(a,b)                 __MM128(PXOR128(__S128(a),__S128(b)))
#define _mm_xor_pd(a,b)                 __MD128(PXOR128(__D128(a),__D128(b)))
#define _mm_setzero_ps()                __MM128(SETZERO())
#define _mm_setzero_pd()                __MD128(SETZERO())
#define _mm_setzero_si128()             __MI128(SETZERO())

//
// Sequence intrinsics perform data swizzling using multiple operations.
//

#define _mm_load1_pd(a)                 __MD128(BROADCAST128(*(uint64_t const *)(a), _BITS_PER_QUAD))
#define _mm_load_ps1(a)                 __MM128(BROADCAST128(*(uint32_t const *)(a), _BITS_PER_LONG))

#define _mm_store1_pd(p,a)              (void)(*(uint128_t *)(p) = PSHUFD128(__D128(a), 0b01000100))
#define _mm_store_ps1(p,a)              (void)(*(uint128_t *)(p) = PSHUFD128(__S128(a), 0b00000000))

#define _mm_loadr_pd(p)                 __MD128(PSHUFD128(*(uint128_t const *)(void *)(p), 0b01001110))
#define _mm_loadr_ps(p)                 __MM128(PSHUFD128(*(uint128_t const *)(void *)(p), 0b00011011))

#define _mm_storer_pd(p,a)              (void)(*(uint128_t *)(p) = PSHUFD128(__D128(a), 0b01001110))
#define _mm_storer_ps(p,a)              (void)(*(uint128_t *)(p) = PSHUFD128(__S128(a), 0b00011011))

#define _mm_set_epi8(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p) \
                                        __MI128(__MAKEU8_U128((uint8_t)(p),(uint8_t)(o),(uint8_t)(n),(uint8_t)(m),(uint8_t)(l),(uint8_t)(k),(uint8_t)(j),(uint8_t)(i), \
                                                              (uint8_t)(h),(uint8_t)(g),(uint8_t)(f),(uint8_t)(e),(uint8_t)(d),(uint8_t)(c),(uint8_t)(b),(uint8_t)(a)))
#define _mm_set_epi16(a,b,c,d,e,f,g,h)  __MI128(__MAKEU16_U128((uint16_t)(h),(uint16_t)(g),(uint16_t)(f),(uint16_t)(e),(uint16_t)(d),(uint16_t)(c),(uint16_t)(b),(uint16_t)(a)))
#define _mm_set_epi32(a,b,c,d)          __MI128(__MAKEU32_U128((uint32_t)(d),(uint32_t)(c),(uint32_t)(b),(uint32_t)(a)))
#define _mm_set_epi64(a,b)              __MI128(__MAKEU64_U128(__U64(b),__U64(a)))
#define _mm_set_epi64x(a,b)             __MI128(__MAKEU64_U128((uint64_t)(b),(uint64_t)(a)))

#define _mm_set_pd(a,b)                 __MD128(__MAKEF64_U128((b),(a)))
#if !defined(_mm_set_pd1)
#define _mm_set_pd1(a)                  __MD128(__MAKEF64_U128((a),(a)))
#endif

#define _mm_set_ps(a,b,c,d)             __MM128(__MAKEF32_U128((d),(c),(b),(a)))
#if !defined(_mm_set_ps1)
#define _mm_set_ps1(a)                  __MM128(__MAKEF32_U128((a),(a),(a),(a)))
#endif

#define _mm_set_sd(a)                   __MD128(__MAKEF64_U128((a),0))
#define _mm_set_ss(a)                   __MM128(__MAKEF32_U128((a),0,0,0))

#define _mm_set1_epi8(a)                __MI128(__MAKEU8X16_U128((uint8_t)(a)))
#define _mm_set1_epi16(a)               __MI128(__MAKEU16X8_U128((uint16_t)(a)))
#define _mm_set1_epi32(a)               __MI128(__MAKEU32X4_U128((uint32_t)(a)))
#define _mm_set1_epi64x(a)              __MI128(__MAKEU64X2_U128((uint64_t)(a)))

#if !defined(_mm_set1_pd)
#define _mm_set1_pd(a)                  __MD128(__MAKEF64_U128((a),(a)))
#endif

#if !defined(_mm_set1_ps)
#define _mm_set1_ps(a)                  __MM128(__MAKEF32_U128((a),(a),(a),(a)))
#endif

#define _mm_setr_epi8(p,o,n,m,l,k,j,i,h,g,f,e,d,c,b,a) \
                                        __MI128(__MAKEU8_U128((uint8_t)(p),(uint8_t)(o),(uint8_t)(n),(uint8_t)(m),(uint8_t)(l),(uint8_t)(k),(uint8_t)(j),(uint8_t)(i), \
                                                              (uint8_t)(h),(uint8_t)(g),(uint8_t)(f),(uint8_t)(e),(uint8_t)(d),(uint8_t)(c),(uint8_t)(b),(uint8_t)(a)))
#define _mm_setr_epi16(h,g,f,e,d,c,b,a) __MI128(__MAKEU16_U128((uint16_t)(h),(uint16_t)(g),(uint16_t)(f),(uint16_t)(e),(uint16_t)(d),(uint16_t)(c),(uint16_t)(b),(uint16_t)(a)))
#define _mm_setr_epi32(d,c,b,a)         __MI128(__MAKEU32_U128((uint32_t)(d),(uint32_t)(c),(uint32_t)(b),(uint32_t)(a)))
#define _mm_setr_epi64(b,a)             __MI128(__MAKEU64_U128(__U64(b),__U64(a)))
#define _mm_setr_epi64x(b,a)            __MI128(__MAKEU64_U128((uint64_t)(b),(uint64_t)(a)))

#define _mm_setr_pd(a,b)                __MD128(__MAKEF64_U128((a),(b)))
#define _mm_setr_ps(a,b,c,d)            __MM128(__MAKEF32_U128((a),(b),(c),(d)))

// None

#define _mm_castpd_ps(a)                __MM128(__D128(a))
#define _mm_castps_pd(a)                __MD128(__S128(a))
#define _mm_castpd_si128(a)             __MI128(__D128(a))
#define _mm_castps_si128(a)             __MI128(__S128(a))
#define _mm_castsi128_pd(a)             __MD128(__U128(a))
#define _mm_castsi128_ps(a)             __MM128(__U128(a))

//
// X64 non-SIMD operations
//

#if defined(_M_ARM64) || defined(_M_ARM64EC)

__forceinline
void __aa64_pause(void)
{
    __dmb(_ARM64_BARRIER_ISHST);
    __yield();
}
#define _mm_pause()                     __aa64_pause()

#define _mm_clflush(p)                  (*(volatile char *)(p))

#define _mm_lfence()                    __dmb(_ARM64_BARRIER_ISHLD);
#define _mm_mfence()                    __dmb(_ARM64_BARRIER_ISH);
#define _mm_sfence()                    __dmb(_ARM64_BARRIER_ISHST);
#define __faststorefence()              __dmb(_ARM64_BARRIER_ISH);

#define __addgsbyte(a,b)                __addx18byte(a, b)
#define __addgsword(a,b)                __addx18word(a, b)
#define __addgsdword(a,b)               __addx18dword(a, b)
#define __addgsqword(a,b)               __addx18qword(a, b)

#define __incgsbyte(a)                  __incx18byte(a)
#define __incgsword(a)                  __incx18word(a)
#define __incgsdword(a)                 __incx18dword(a)
#define __incgsqword(a)                 __incx18qword(a)

#define __readgsbyte(a)                 __readx18byte(a)
#define __readgsword(a)                 __readx18word(a)
#define __readgsdword(a)                __readx18dword(a)
#define __readgsqword(a)                __readx18qword(a)

#define __writegsbyte(a,b)              __writex18byte(a, b)
#define __writegsword(a,b)              __writex18word(a, b)
#define __writegsdword(a,b)             __writex18dword(a, b)
#define __writegsqword(a,b)             __writex18qword(a, b)

__forceinline
unsigned char _addcarry_u8(
   unsigned char Carry,
   unsigned char Source1,
   unsigned char Source2,
   unsigned char* Destination
)
{
    unsigned char Sum = (unsigned char)((Carry != 0) + Source1 + Source2);
    unsigned char CarryVector = (unsigned char)((Source1 & Source2) ^ ((Source1 ^ Source2) & ~Sum));
    *Destination = Sum;
    return (unsigned char)(CarryVector >> 7);
}

__forceinline
unsigned char _addcarry_u16(
   unsigned char Carry,
   unsigned short Source1,
   unsigned short Source2,
   unsigned short* Destination
)
{
    unsigned short Sum = (unsigned short)((Carry != 0) + Source1 + Source2);
    unsigned short CarryVector = (unsigned short)((Source1 & Source2) ^ ((Source1 ^ Source2) & ~Sum));
    *Destination = Sum;
    return (unsigned char)(CarryVector >> 15);
}

__forceinline
unsigned char _addcarry_u32(
   unsigned char Carry,
   unsigned int Source1,
   unsigned int Source2,
   unsigned int* Destination
)
{
    unsigned int Sum = (Carry != 0) + Source1 + Source2;
    unsigned int CarryVector = (Source1 & Source2) ^ ((Source1 ^ Source2) & ~Sum);
    *Destination = Sum;
    return CarryVector >> 31;
}

__forceinline
unsigned char _addcarry_u64(
   unsigned char Carry,
   unsigned __int64 Source1,
   unsigned __int64 Source2,
   unsigned __int64* Destination
)
{
    unsigned __int64 Sum = (Carry != 0) + Source1 + Source2;
    unsigned __int64 CarryVector = (Source1 & Source2) ^ ((Source1 ^ Source2) & ~Sum);
    *Destination = Sum;
    return CarryVector >> 63;
}

__forceinline
unsigned char _subborrow_u8(
   unsigned char Borrow,
   unsigned char Source1,
   unsigned char Source2,
   unsigned char* Destination
)
{
    unsigned char Diff = (unsigned char)(Source1 - Source2 - (Borrow != 0));
    unsigned char CarryVector = (unsigned char)((Diff & Source2) ^ ((Diff ^ Source2) & ~Source1));
    *Destination = Diff;
    return (unsigned char)(CarryVector >> 7);
}

__forceinline
unsigned char _subborrow_u16(
   unsigned char Borrow,
   unsigned short Source1,
   unsigned short Source2,
   unsigned short* Destination
)
{
    unsigned short Diff = (unsigned short)(Source1 - Source2 - (Borrow != 0));
    unsigned short CarryVector = (unsigned short)((Diff & Source2) ^ ((Diff ^ Source2) & ~Source1));
    *Destination = Diff;
    return (unsigned char)(CarryVector >> 15);
}

__forceinline
unsigned char _subborrow_u32(
   unsigned char Borrow,
   unsigned int Source1,
   unsigned int Source2,
   unsigned int* Destination
)
{
    unsigned int Diff = Source1 - Source2 - (Borrow != 0);
    unsigned int CarryVector = (Diff & Source2) ^ ((Diff ^ Source2) & ~Source1);
    *Destination = Diff;
    return CarryVector >> 31;
}

__forceinline
unsigned char _subborrow_u64(
   unsigned char Borrow,
   unsigned __int64 Source1,
   unsigned __int64 Source2,
   unsigned __int64* Destination
)
{
    unsigned __int64 Diff = Source1 - Source2 - (Borrow != 0);
    unsigned __int64 CarryVector = (Diff & Source2) ^ ((Diff ^ Source2) & ~Source1);
    *Destination = Diff;
    return CarryVector >> 63;
}

__forceinline
unsigned int __andn_u32(
    unsigned int _Source1,
    unsigned int _Source2
)
{
    return (~_Source1) & _Source2;
}

__forceinline
unsigned __int64 __andn_u64(
    unsigned __int64 _Source1,
    unsigned __int64 _Source2
)
{
    return (~_Source1) & _Source2;
}

#define _andn_u32(_Source1, _Source2) __andn_u32(_Source1, _Source2)
#define _andn_u64(_Source1, _Source2) __andn_u64(_Source1, _Source2)

__forceinline
unsigned int __bextr_u32(
    unsigned int _Source,
    unsigned int _Start,
    unsigned int _Len
)
{
    return (_Source >> _Start) & ~(~0U << _Len);
}

__forceinline
unsigned __int64 __bextr_u64(
    unsigned __int64 _Source,
    unsigned __int64 _Start,
    unsigned __int64 _Len
)
{
    return (_Source >> _Start) & ~(~0ULL << _Len);
}

#define _bextr_u32(_Source, _Start, _Len) __bextr_u32(_Source, _Start, _Len)
#define _bextr_u64(_Source, _Start, _Len) __bextr_u64(_Source, _Start, _Len)

__forceinline
unsigned int __bextr2_u32(
    unsigned int _Source,
    unsigned int _Control
)
{
    unsigned int _Start = _Control & 0xFF;
    unsigned int _Len = (_Control >> 8) & 0xFF;
    return __bextr_u32(_Source, _Start, _Len);
}

__forceinline
unsigned __int64 __bextr2_u64(
    unsigned __int64 _Source,
    unsigned __int64 _Control
)
{
    unsigned int _Start = _Control & 0xFF;
    unsigned int _Len = (_Control >> 8) & 0xFF;
    return __bextr_u64(_Source, _Start, _Len);
}

#define _bextr2_u32(_Source, _Control) __bextr2_u32(_Source, _Control)
#define _bextr2_u64(_Source, _Control) __bextr2_u64(_Source, _Control)

__forceinline
unsigned int __blsi_u32(
    unsigned int _Source
)
{
    return (0U - _Source) & _Source;
}

__forceinline
unsigned __int64 __blsi_u64(
    unsigned __int64 _Source
)
{
    return (0ULL - _Source) & _Source;
}

#define _blsi_u32(x) __blsi_u32(x)
#define _blsi_u64(x) __blsi_u64(x)

__forceinline
unsigned int __blsmsk_u32(
    unsigned int _Source
)
{
    return (_Source - 1) ^ _Source;
}

__forceinline
unsigned __int64 __blsmsk_u64(
    unsigned __int64 _Source
)
{
    return (_Source - 1) ^ _Source;
}

#define _blsmsk_u32(x) __blsmsk_u32(x)
#define _blsmsk_u64(x) __blsmsk_u64(x)

__forceinline
unsigned int __blsr_u32(
    unsigned int _Source
)
{
    return (_Source - 1) & _Source;
}

__forceinline
unsigned __int64 __blsr_u64(
    unsigned __int64 _Source
)
{
    return (_Source - 1) & _Source;
}

#define _blsr_u32(x) __blsr_u32(x)
#define _blsr_u64(x) __blsr_u64(x)

// __lzcnt __lzcnt16 __lzcnt64 are builtin to clang and providing them here errors.
#if !defined(__clang__)

__forceinline
unsigned int __lzcnt(
   unsigned int Source
)
{
    unsigned long Bit = 0;
    _BitScanReverse(&Bit, Source);
    return Source ? Bit ^ 31 : 32;
}

__forceinline
unsigned short __lzcnt16(
   unsigned short Source
)
{
    return (unsigned short)(__lzcnt(Source) - 16u);
}

__forceinline
unsigned __int64 __lzcnt64(
   unsigned __int64 Source
)
{
    unsigned long Bit = 0;
    _BitScanReverse64(&Bit, Source);
    return Source ? Bit ^ 63 : 64;
}

#endif

#define _lzcnt_u32(x) __lzcnt(x)
#define _lzcnt_u64(x) __lzcnt64(x)

#if defined(__clang__)
__forceinline
unsigned __int16 __tzcnt_u16(
    unsigned __int16 Source
)
{
    unsigned __int16 Result = Source ? (unsigned __int16) __builtin_ctz(Source) : (unsigned __int16) 16;
    return Result <= (unsigned __int16) 16 ? Result : (unsigned __int16) 16;
}

__forceinline
unsigned int __tzcnt_u32(
    unsigned int Source
)
{
    return Source ? __builtin_ctz(Source) : 32;
}

__forceinline
unsigned __int64 __tzcnt_u64(
    unsigned __int64 Source
)
{
    return Source ? (unsigned __int64) __builtin_ctzll(Source) : (unsigned __int64) 64;
}

#else // defined(__clang__)
// The MSVC implementations for _CountTrailingZeros() with 0 parameters are defined on ARM64,
// so we don't need to do the same check like clang does.

__forceinline
unsigned __int16 __tzcnt_u16(
    unsigned __int16 Source
)
{
    unsigned __int16 Result = (unsigned __int16) _CountTrailingZeros(Source);
    return Result <= (unsigned __int16) 16 ? Result : (unsigned __int16) 16;
}

__forceinline
unsigned int __tzcnt_u32(
    unsigned int Source
)
{
    return _CountTrailingZeros((unsigned long)Source);
}

__forceinline
unsigned __int64 __tzcnt_u64(
    unsigned __int64 Source
)
{
    return (unsigned __int64) _CountTrailingZeros64(Source);
}

#endif // !defined(__clang__)

#define _tzcnt_u16(x) __tzcnt_u16(x)
#define _tzcnt_u32(x) __tzcnt_u32(x)
#define _tzcnt_u64(x) __tzcnt_u64(x)

//
// __movs* and __stos* while similar to memcpy and memset intrinsics do not map 1-to-1
// for the word, long, and quad cases.  As it turns out implementing them as inline
// loop is actually fewer instructions than even a function call.
//

__forceinline
void __movsb(
   unsigned char* Destination,
   unsigned char const* Source,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = *Source++;
    }
}

__forceinline
void __movsw(
   unsigned short* Destination,
   unsigned short const* Source,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = *Source++;
    }
}

__forceinline
void __movsd(
   unsigned long* Destination,
   unsigned long const* Source,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = *Source++;
    }
}

__forceinline
void __movsq(
   unsigned __int64* Destination,
   unsigned __int64 const* Source,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = *Source++;
    }
}

__forceinline
void __stosb(
   unsigned char* Destination,
   unsigned char Data,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = Data;
    }
}

__forceinline
void __stosw(
   unsigned short* Destination,
   unsigned short Data,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = Data;
    }
}

__forceinline
void __stosd(
   unsigned long* Destination,
   unsigned long Data,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = Data;
    }
}

__forceinline
void __stosq(
   unsigned __int64* Destination,
   unsigned __int64 Data,
   size_t Count
)
{
    while (Count--)
    {
        *Destination++ = Data;
    }
}

__forceinline
void __ud2(
    void
)
{
    __emit(0);
}

#define _udiv64(n,d,r)                  _UDIV64((n),(d),(r))
#define _div64(n,d,r)                   _SDIV64((n),(d),(r))

#define __emulu(x,y)                    ((uint64_t)(uint32_t)(x) * (uint64_t)(uint32_t)(y))
#define __emul(x,y)                     ((int64_t)(int32_t)(x) * (int64_t)(int32_t)(y))

//
// Map X64 atomic intrinsics to equivalent ARM64 ones
// Reference starting from here: https://docs.microsoft.com/en-us/cpp/intrinsics/interlockedand-intrinsic-functions
//

#define _InterlockedAnd_np _InterlockedAnd
#define _InterlockedAnd8_np _InterlockedAnd8
#define _InterlockedAnd16_np _InterlockedAnd16
#define _InterlockedAnd64_np _InterlockedAnd64

#define _InterlockedAnd_HLEAcquire _InterlockedAnd_acq
#define _InterlockedAnd_HLERelease _InterlockedAnd_rel
#define _InterlockedAnd64_HLEAcquire _InterlockedAnd64_acq
#define _InterlockedAnd64_HLERelease _InterlockedAnd64_rel

#define _InterlockedOr_np _InterlockedOr
#define _InterlockedOr8_np _InterlockedOr8
#define _InterlockedOr16_np _InterlockedOr16
#define _InterlockedOr64_np _InterlockedOr64

#define _InterlockedOr_HLEAcquire _InterlockedOr_acq
#define _InterlockedOr_HLERelease _InterlockedOr_rel
#define _InterlockedOr64_HLEAcquire _InterlockedOr64_acq
#define _InterlockedOr64_HLERelease _InterlockedOr64_rel

#define _InterlockedXor_np _InterlockedXor
#define _InterlockedXor8_np _InterlockedXor8
#define _InterlockedXor16_np _InterlockedXor16
#define _InterlockedXor64_np _InterlockedXor64

#define _InterlockedXor_HLEAcquire _InterlockedXor_acq
#define _InterlockedXor_HLERelease _InterlockedXor_rel
#define _InterlockedXor64_HLEAcquire _InterlockedXor64_acq
#define _InterlockedXor64_HLERelease _InterlockedXor64_rel

#define _InterlockedCompareExchange_np _InterlockedCompareExchange
#define _InterlockedCompareExchangePointer_np _InterlockedCompareExchangePointer
#define _InterlockedCompareExchange16_np _InterlockedCompareExchange16
#define _InterlockedCompareExchange64_np _InterlockedCompareExchange64
#define _InterlockedCompareExchange128_np _InterlockedCompareExchange128

#define _InterlockedCompareExchange_HLEAcquire _InterlockedCompareExchange_acq
#define _InterlockedCompareExchange_HLERelease _InterlockedCompareExchange_rel
#define _InterlockedCompareExchange64_HLEAcquire _InterlockedCompareExchange64_acq
#define _InterlockedCompareExchange64_HLERelease _InterlockedCompareExchange64_rel

#define _interlockedbittestandreset_HLEAcquire _interlockedbittestandreset_acq
#define _interlockedbittestandreset_HLERelease _interlockedbittestandreset_rel
#define _interlockedbittestandreset64_HLEAcquire _interlockedbittestandreset64_acq
#define _interlockedbittestandreset64_HLERelease _interlockedbittestandreset64_rel

#define _interlockedbittestandset_HLEAcquire _interlockedbittestandset_acq
#define _interlockedbittestandset_HLERelease _interlockedbittestandset_rel
#define _interlockedbittestandset64_HLEAcquire _interlockedbittestandset64_acq
#define _interlockedbittestandset64_HLERelease _interlockedbittestandset64_rel

#define _InterlockedCompareExchangePointer_HLEAcquire _InterlockedCompareExchangePointer_acq
#define _InterlockedCompareExchangePointer_HLERelease _InterlockedCompareExchangePointer_rel

#define _InterlockedExchange_HLEAcquire _InterlockedExchange_acq
#define _InterlockedExchange_HLERelease _InterlockedExchange_rel
#define _InterlockedExchange64_HLEAcquire _InterlockedExchange64_acq
#define _InterlockedExchange64_HLERelease _InterlockedExchange64_rel

#define _InterlockedExchangeAdd_HLEAcquire _InterlockedExchangeAdd_acq
#define _InterlockedExchangeAdd_HLERelease _InterlockedExchangeAdd_rel
#define _InterlockedExchangeAdd64_HLEAcquire _InterlockedExchangeAdd64_acq
#define _InterlockedExchangeAdd64_HLERelease _InterlockedExchangeAdd64_rel

#define _InterlockedExchangePointer_HLEAcquire _InterlockedExchangePointer_acq
#define _InterlockedExchangePointer_HLERelease _InterlockedExchangePointer_rel

#endif

#ifdef __cplusplus

}  // extern "C"

#endif

#endif  // defined(_M_ARM64EC) || defined(USE_SOFT_INTRINSICS)
