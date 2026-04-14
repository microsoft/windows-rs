//-------------------------------------------------------------------------------------
// DirectXMath.h -- SIMD C++ Math library
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
//
// http://go.microsoft.com/fwlink/?LinkID=615560
//-------------------------------------------------------------------------------------

#pragma once

#ifndef __cplusplus
#error DirectX Math requires C++
#endif

#define DIRECTX_MATH_VERSION 319

#if defined(_MSC_VER) && defined(_GAMING_XBOX) && defined(_M_X64) && !defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
#define _XM_F16C_INTRINSICS_
#endif

#if defined(_MSC_VER) && !defined(_M_ARM) && !defined(_M_ARM64) && !defined(_M_HYBRID_X86_ARM64) && !defined(_M_ARM64EC) && (!_MANAGED) && (!_M_CEE) && (!defined(_M_IX86_FP) || (_M_IX86_FP > 1)) && !defined(_XM_NO_INTRINSICS_) && !defined(_XM_VECTORCALL_)
#define _XM_VECTORCALL_ 1
#endif

#if _XM_VECTORCALL_
#define XM_CALLCONV __vectorcall
#elif defined(__GNUC__)
#define XM_CALLCONV
#else
#define XM_CALLCONV __fastcall
#endif

#ifndef XM_DEPRECATED
#ifdef __GNUC__
#define XM_DEPRECATED __attribute__ ((deprecated))
#else
#define XM_DEPRECATED __declspec(deprecated("This is deprecated and will be removed in a future version."))
#endif
#endif

#if !defined(_XM_AVX2_INTRINSICS_) && defined(__AVX2__) && !defined(_XM_NO_INTRINSICS_)
#define _XM_AVX2_INTRINSICS_
#endif

#if !defined(_XM_FMA3_INTRINSICS_) && defined(_XM_AVX2_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
#define _XM_FMA3_INTRINSICS_
#endif

#if !defined(_XM_F16C_INTRINSICS_) && defined(_XM_AVX2_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
#define _XM_F16C_INTRINSICS_
#endif

#if !defined(_XM_F16C_INTRINSICS_) && defined(__F16C__) && !defined(_XM_NO_INTRINSICS_)
#define _XM_F16C_INTRINSICS_
#endif

#if defined(_XM_FMA3_INTRINSICS_) && !defined(_XM_AVX_INTRINSICS_)
#define _XM_AVX_INTRINSICS_
#endif

#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_AVX_INTRINSICS_)
#define _XM_AVX_INTRINSICS_
#endif

#if !defined(_XM_AVX_INTRINSICS_) && defined(__AVX__) && !defined(_XM_NO_INTRINSICS_)
#define _XM_AVX_INTRINSICS_
#endif

#if defined(_XM_AVX_INTRINSICS_) && !defined(_XM_SSE4_INTRINSICS_)
#define _XM_SSE4_INTRINSICS_
#endif

#if defined(_XM_SSE4_INTRINSICS_) && !defined(_XM_SSE3_INTRINSICS_)
#define _XM_SSE3_INTRINSICS_
#endif

#if defined(_XM_SSE3_INTRINSICS_) && !defined(_XM_SSE_INTRINSICS_)
#define _XM_SSE_INTRINSICS_
#endif

#if !defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
#if (defined(_M_IX86) || defined(_M_X64) || __i386__ || __x86_64__) && !defined(_M_HYBRID_X86_ARM64) && !defined(_M_ARM64EC)
#define _XM_SSE_INTRINSICS_
#elif defined(_M_ARM) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __arm__ || __aarch64__
#define _XM_ARM_NEON_INTRINSICS_
#elif !defined(_XM_NO_INTRINSICS_)
#error DirectX Math does not support this target
#endif
#endif // !_XM_ARM_NEON_INTRINSICS_ && !_XM_SSE_INTRINSICS_ && !_XM_NO_INTRINSICS_


#if !defined(_XM_NO_XMVECTOR_OVERLOADS_) && (defined(__clang__) || defined(__GNUC__)) && !defined(_XM_NO_INTRINSICS_)
#define _XM_NO_XMVECTOR_OVERLOADS_
#endif

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable:4514 4820)
// C4514/4820: Off by default noise
#endif
#include <math.h>
#include <float.h>
#ifdef _MSC_VER
#pragma warning(pop)
#endif

#ifndef _XM_NO_INTRINSICS_

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4987)
// C4987: Off by default noise
#endif
#if defined(_MSC_VER) || defined(__MINGW32__)
#include <intrin.h>
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

#if (defined(__clang__) || defined(__GNUC__)) && (__x86_64__ || __i386__) && !defined(__MINGW32__)
#include <cpuid.h>
#endif

#ifdef _XM_SSE_INTRINSICS_
#include <xmmintrin.h>
#include <emmintrin.h>

#ifdef _XM_SSE3_INTRINSICS_
#include <pmmintrin.h>
#endif

#ifdef _XM_SSE4_INTRINSICS_
#include <smmintrin.h>
#endif

#ifdef _XM_AVX_INTRINSICS_
#include <immintrin.h>
#endif

#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_MSC_VER) && !defined(__clang__) && (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC))
#include <arm64_neon.h>
#else
#include <arm_neon.h>
#endif
#endif
#endif // !_XM_NO_INTRINSICS_

#include "sal.h"
#include <assert.h>

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4005 4668)
// C4005/4668: Old header issue
#endif
#include <stdint.h>
#ifdef _MSC_VER
#pragma warning(pop)
#endif

#if __cplusplus >= 201703L
#define XM_ALIGNED_DATA(x) alignas(x)
#define XM_ALIGNED_STRUCT(x) struct alignas(x)
#elif defined(__GNUC__)
#define XM_ALIGNED_DATA(x) __attribute__ ((aligned(x)))
#define XM_ALIGNED_STRUCT(x) struct __attribute__ ((aligned(x)))
#else
#define XM_ALIGNED_DATA(x) __declspec(align(x))
#define XM_ALIGNED_STRUCT(x) __declspec(align(x)) struct
#endif

#if (__cplusplus >= 202002L)
#include <compare>
#endif

/****************************************************************************
 *
 * Conditional intrinsics
 *
 ****************************************************************************/

#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)

#if defined(_XM_NO_MOVNT_)
#define XM_STREAM_PS( p, a ) _mm_store_ps((p), (a))
#define XM256_STREAM_PS( p, a ) _mm256_store_ps((p), (a))
#define XM_SFENCE()
#else
#define XM_STREAM_PS( p, a ) _mm_stream_ps((p), (a))
#define XM256_STREAM_PS( p, a ) _mm256_stream_ps((p), (a))
#define XM_SFENCE() _mm_sfence()
#endif

#if defined(_XM_FMA3_INTRINSICS_)
#define XM_FMADD_PS( a, b, c ) _mm_fmadd_ps((a), (b), (c))
#define XM_FNMADD_PS( a, b, c ) _mm_fnmadd_ps((a), (b), (c))
#else
#define XM_FMADD_PS( a, b, c ) _mm_add_ps(_mm_mul_ps((a), (b)), (c))
#define XM_FNMADD_PS( a, b, c ) _mm_sub_ps((c), _mm_mul_ps((a), (b)))
#endif

#if defined(_XM_AVX_INTRINSICS_) && defined(_XM_FAVOR_INTEL_)
#define XM_PERMUTE_PS( v, c ) _mm_permute_ps((v), c )
#else
#define XM_PERMUTE_PS( v, c ) _mm_shuffle_ps((v), (v), c )
#endif

#define XM_LOADU_SI16( p ) _mm_cvtsi32_si128(*reinterpret_cast<unsigned short const*>(p))

#endif // _XM_SSE_INTRINSICS_ && !_XM_NO_INTRINSICS_

#if defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)

#if defined(__clang__) || defined(__GNUC__)
#define XM_PREFETCH( a ) __builtin_prefetch(a)
#elif defined(_MSC_VER)
#define XM_PREFETCH( a ) __prefetch(a)
#else
#define XM_PREFETCH( a )
#endif

#endif // _XM_ARM_NEON_INTRINSICS_ && !_XM_NO_INTRINSICS_

namespace DirectX
{

    /****************************************************************************
     *
     * Constant definitions
     *
     ****************************************************************************/

#if defined(__XNAMATH_H__) && defined(XM_PI)
#undef XM_PI
#undef XM_2PI
#undef XM_1DIVPI
#undef XM_1DIV2PI
#undef XM_PIDIV2
#undef XM_PIDIV4
#undef XM_SELECT_0
#undef XM_SELECT_1
#undef XM_PERMUTE_0X
#undef XM_PERMUTE_0Y
#undef XM_PERMUTE_0Z
#undef XM_PERMUTE_0W
#undef XM_PERMUTE_1X
#undef XM_PERMUTE_1Y
#undef XM_PERMUTE_1Z
#undef XM_PERMUTE_1W
#undef XM_CRMASK_CR6
#undef XM_CRMASK_CR6TRUE
#undef XM_CRMASK_CR6FALSE
#undef XM_CRMASK_CR6BOUNDS
#undef XM_CACHE_LINE_SIZE
#endif

    constexpr float XM_PI = 3.141592654f;
    constexpr float XM_2PI = 6.283185307f;
    constexpr float XM_1DIVPI = 0.318309886f;
    constexpr float XM_1DIV2PI = 0.159154943f;
    constexpr float XM_PIDIV2 = 1.570796327f;
    constexpr float XM_PIDIV4 = 0.785398163f;

    constexpr uint32_t XM_SELECT_0 = 0x00000000;
    constexpr uint32_t XM_SELECT_1 = 0xFFFFFFFF;

    constexpr uint32_t XM_PERMUTE_0X = 0;
    constexpr uint32_t XM_PERMUTE_0Y = 1;
    constexpr uint32_t XM_PERMUTE_0Z = 2;
    constexpr uint32_t XM_PERMUTE_0W = 3;
    constexpr uint32_t XM_PERMUTE_1X = 4;
    constexpr uint32_t XM_PERMUTE_1Y = 5;
    constexpr uint32_t XM_PERMUTE_1Z = 6;
    constexpr uint32_t XM_PERMUTE_1W = 7;

    constexpr uint32_t XM_SWIZZLE_X = 0;
    constexpr uint32_t XM_SWIZZLE_Y = 1;
    constexpr uint32_t XM_SWIZZLE_Z = 2;
    constexpr uint32_t XM_SWIZZLE_W = 3;

    constexpr uint32_t XM_CRMASK_CR6 = 0x000000F0;
    constexpr uint32_t XM_CRMASK_CR6TRUE = 0x00000080;
    constexpr uint32_t XM_CRMASK_CR6FALSE = 0x00000020;
    constexpr uint32_t XM_CRMASK_CR6BOUNDS = XM_CRMASK_CR6FALSE;

#if defined(_M_ARM) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __arm__ || __aarch64__
    constexpr size_t XM_CACHE_LINE_SIZE = 128;
#else
    constexpr size_t XM_CACHE_LINE_SIZE = 64;
#endif


    /****************************************************************************
     *
     * Macros
     *
     ****************************************************************************/

#if defined(__XNAMATH_H__) && defined(XMComparisonAllTrue)
#undef XMComparisonAllTrue
#undef XMComparisonAnyTrue
#undef XMComparisonAllFalse
#undef XMComparisonAnyFalse
#undef XMComparisonMixed
#undef XMComparisonAllInBounds
#undef XMComparisonAnyOutOfBounds
#endif

    // Unit conversion

    constexpr float XMConvertToRadians(float fDegrees) noexcept { return fDegrees * (XM_PI / 180.0f); }
    constexpr float XMConvertToDegrees(float fRadians) noexcept { return fRadians * (180.0f / XM_PI); }

    // Condition register evaluation proceeding a recording (R) comparison

    constexpr bool XMComparisonAllTrue(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6TRUE) == XM_CRMASK_CR6TRUE; }
    constexpr bool XMComparisonAnyTrue(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6FALSE) != XM_CRMASK_CR6FALSE; }
    constexpr bool XMComparisonAllFalse(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6FALSE) == XM_CRMASK_CR6FALSE; }
    constexpr bool XMComparisonAnyFalse(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6TRUE) != XM_CRMASK_CR6TRUE; }
    constexpr bool XMComparisonMixed(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6) == 0; }
    constexpr bool XMComparisonAllInBounds(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6BOUNDS) == XM_CRMASK_CR6BOUNDS; }
    constexpr bool XMComparisonAnyOutOfBounds(uint32_t CR) noexcept { return (CR & XM_CRMASK_CR6BOUNDS) != XM_CRMASK_CR6BOUNDS; }


    /****************************************************************************
     *
     * Data types
     *
     ****************************************************************************/

#ifdef _MSC_VER    
#pragma warning(push)
#pragma warning(disable:4068 4201 4365 4324 4820)
     // C4068: ignore unknown pragmas
     // C4201: nonstandard extension used : nameless struct/union
     // C4365: Off by default noise
     // C4324/4820: padding warnings
#endif

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 25000, "FXMVECTOR is 16 bytes")
#endif

//------------------------------------------------------------------------------
#if defined(_XM_NO_INTRINSICS_)
    struct __vector4
    {
        union
        {
            float       vector4_f32[4];
            uint32_t    vector4_u32[4];
        };
    };
#endif // _XM_NO_INTRINSICS_

    //------------------------------------------------------------------------------
    // Vector intrinsic: Four 32 bit floating point components aligned on a 16 byte
    // boundary and mapped to hardware vector registers
#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    using XMVECTOR = __m128;
#elif defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    using XMVECTOR = float32x4_t;
#else
    using XMVECTOR = __vector4;
#endif

    // Fix-up for (1st-3rd) XMVECTOR parameters that are pass-in-register for x86, ARM, ARM64, and vector call; by reference otherwise
#if ( defined(_M_IX86) || defined(_M_ARM) || defined(_M_ARM64) || _XM_VECTORCALL_ || __i386__ || __arm__ || __aarch64__ ) && !defined(_XM_NO_INTRINSICS_)
    typedef const XMVECTOR FXMVECTOR;
#else
    typedef const XMVECTOR& FXMVECTOR;
#endif

    // Fix-up for (4th) XMVECTOR parameter to pass in-register for ARM, ARM64, and vector call; by reference otherwise
#if ( defined(_M_ARM) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || _XM_VECTORCALL_ || __arm__ || __aarch64__ ) && !defined(_XM_NO_INTRINSICS_)
    typedef const XMVECTOR GXMVECTOR;
#else
    typedef const XMVECTOR& GXMVECTOR;
#endif

    // Fix-up for (5th & 6th) XMVECTOR parameter to pass in-register for ARM64 and vector call; by reference otherwise
#if ( defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || _XM_VECTORCALL_ || __aarch64__ ) && !defined(_XM_NO_INTRINSICS_)
    typedef const XMVECTOR HXMVECTOR;
#else
    typedef const XMVECTOR& HXMVECTOR;
#endif

    // Fix-up for (7th+) XMVECTOR parameters to pass by reference
    typedef const XMVECTOR& CXMVECTOR;

    //------------------------------------------------------------------------------
    // Conversion types for constants
    XM_ALIGNED_STRUCT(16) XMVECTORF32
    {
        union
        {
            float f[4];
            XMVECTOR v;
        };

        inline operator XMVECTOR() const noexcept { return v; }
        inline operator const float* () const noexcept { return f; }
#ifdef _XM_NO_INTRINSICS_
#elif defined(_XM_SSE_INTRINSICS_)
        inline operator __m128i() const noexcept { return _mm_castps_si128(v); }
        inline operator __m128d() const noexcept { return _mm_castps_pd(v); }
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(__GNUC__) || defined(_ARM64_DISTINCT_NEON_TYPES))
        inline operator int32x4_t() const noexcept { return vreinterpretq_s32_f32(v); }
        inline operator uint32x4_t() const noexcept { return vreinterpretq_u32_f32(v); }
#endif
    };

    XM_ALIGNED_STRUCT(16) XMVECTORI32
    {
        union
        {
            int32_t i[4];
            XMVECTOR v;
        };

        inline operator XMVECTOR() const noexcept { return v; }
#ifdef _XM_NO_INTRINSICS_
#elif defined(_XM_SSE_INTRINSICS_)
        inline operator __m128i() const noexcept { return _mm_castps_si128(v); }
        inline operator __m128d() const noexcept { return _mm_castps_pd(v); }
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(__GNUC__) || defined(_ARM64_DISTINCT_NEON_TYPES))
        inline operator int32x4_t() const noexcept { return vreinterpretq_s32_f32(v); }
        inline operator uint32x4_t() const noexcept { return vreinterpretq_u32_f32(v); }
#endif
    };

    XM_ALIGNED_STRUCT(16) XMVECTORU8
    {
        union
        {
            uint8_t u[16];
            XMVECTOR v;
        };

        inline operator XMVECTOR() const noexcept { return v; }
#ifdef _XM_NO_INTRINSICS_
#elif defined(_XM_SSE_INTRINSICS_)
        inline operator __m128i() const noexcept { return _mm_castps_si128(v); }
        inline operator __m128d() const noexcept { return _mm_castps_pd(v); }
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(__GNUC__) || defined(_ARM64_DISTINCT_NEON_TYPES))
        inline operator int32x4_t() const noexcept { return vreinterpretq_s32_f32(v); }
        inline operator uint32x4_t() const noexcept { return vreinterpretq_u32_f32(v); }
#endif
    };

    XM_ALIGNED_STRUCT(16) XMVECTORU32
    {
        union
        {
            uint32_t u[4];
            XMVECTOR v;
        };

        inline operator XMVECTOR() const noexcept { return v; }
#ifdef _XM_NO_INTRINSICS_
#elif defined(_XM_SSE_INTRINSICS_)
        inline operator __m128i() const noexcept { return _mm_castps_si128(v); }
        inline operator __m128d() const noexcept { return _mm_castps_pd(v); }
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(__GNUC__) || defined(_ARM64_DISTINCT_NEON_TYPES))
        inline operator int32x4_t() const noexcept { return vreinterpretq_s32_f32(v); }
        inline operator uint32x4_t() const noexcept { return vreinterpretq_u32_f32(v); }
#endif
    };

    //------------------------------------------------------------------------------
    // Vector operators

#ifndef _XM_NO_XMVECTOR_OVERLOADS_
    XMVECTOR    XM_CALLCONV     operator+ (FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     operator- (FXMVECTOR V) noexcept;

    XMVECTOR&   XM_CALLCONV     operator+= (XMVECTOR& V1, FXMVECTOR V2) noexcept;
    XMVECTOR&   XM_CALLCONV     operator-= (XMVECTOR& V1, FXMVECTOR V2) noexcept;
    XMVECTOR&   XM_CALLCONV     operator*= (XMVECTOR& V1, FXMVECTOR V2) noexcept;
    XMVECTOR&   XM_CALLCONV     operator/= (XMVECTOR& V1, FXMVECTOR V2) noexcept;

    XMVECTOR& operator*= (XMVECTOR& V, float S) noexcept;
    XMVECTOR& operator/= (XMVECTOR& V, float S) noexcept;

    XMVECTOR    XM_CALLCONV     operator+ (FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     operator- (FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     operator* (FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     operator/ (FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     operator* (FXMVECTOR V, float S) noexcept;
    XMVECTOR    XM_CALLCONV     operator* (float S, FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     operator/ (FXMVECTOR V, float S) noexcept;
#endif /* !_XM_NO_XMVECTOR_OVERLOADS_ */

    //------------------------------------------------------------------------------
    // Matrix type: Sixteen 32 bit floating point components aligned on a
    // 16 byte boundary and mapped to four hardware vector registers

    struct XMMATRIX;

    // Fix-up for (1st) XMMATRIX parameter to pass in-register for ARM64 and vector call; by reference otherwise
#if ( defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || _XM_VECTORCALL_ || __aarch64__ ) && !defined(_XM_NO_INTRINSICS_)
    typedef const XMMATRIX FXMMATRIX;
#else
    typedef const XMMATRIX& FXMMATRIX;
#endif

    // Fix-up for (2nd+) XMMATRIX parameters to pass by reference
    typedef const XMMATRIX& CXMMATRIX;

#ifdef _XM_NO_INTRINSICS_
    struct XMMATRIX
#else
    XM_ALIGNED_STRUCT(16) XMMATRIX
#endif
    {
#ifdef _XM_NO_INTRINSICS_
        union
        {
            XMVECTOR r[4];
            struct
            {
                float _11, _12, _13, _14;
                float _21, _22, _23, _24;
                float _31, _32, _33, _34;
                float _41, _42, _43, _44;
            };
            float m[4][4];
        };
#else
        XMVECTOR r[4];
#endif

        XMMATRIX() = default;

        XMMATRIX(const XMMATRIX&) = default;

#if defined(_MSC_VER) && (_MSC_FULL_VER < 191426431)
        XMMATRIX& operator= (const XMMATRIX& M) noexcept { r[0] = M.r[0]; r[1] = M.r[1]; r[2] = M.r[2]; r[3] = M.r[3]; return *this; }
#else
        XMMATRIX& operator=(const XMMATRIX&) = default;

        XMMATRIX(XMMATRIX&&) = default;
        XMMATRIX& operator=(XMMATRIX&&) = default;
#endif

        constexpr XMMATRIX(FXMVECTOR R0, FXMVECTOR R1, FXMVECTOR R2, CXMVECTOR R3) noexcept : r{ R0,R1,R2,R3 } {}
        XMMATRIX(float m00, float m01, float m02, float m03,
            float m10, float m11, float m12, float m13,
            float m20, float m21, float m22, float m23,
            float m30, float m31, float m32, float m33) noexcept;
        explicit XMMATRIX(_In_reads_(16) const float* pArray) noexcept;

#ifdef _XM_NO_INTRINSICS_
        float       operator() (size_t Row, size_t Column) const noexcept { return m[Row][Column]; }
        float& operator() (size_t Row, size_t Column) noexcept { return m[Row][Column]; }
#endif

        XMMATRIX    operator+ () const noexcept { return *this; }
        XMMATRIX    operator- () const noexcept;

        XMMATRIX&   XM_CALLCONV     operator+= (FXMMATRIX M) noexcept;
        XMMATRIX&   XM_CALLCONV     operator-= (FXMMATRIX M) noexcept;
        XMMATRIX&   XM_CALLCONV     operator*= (FXMMATRIX M) noexcept;
        XMMATRIX&   operator*= (float S) noexcept;
        XMMATRIX&   operator/= (float S) noexcept;

        XMMATRIX    XM_CALLCONV     operator+ (FXMMATRIX M) const noexcept;
        XMMATRIX    XM_CALLCONV     operator- (FXMMATRIX M) const noexcept;
        XMMATRIX    XM_CALLCONV     operator* (FXMMATRIX M) const noexcept;
        XMMATRIX    operator* (float S) const noexcept;
        XMMATRIX    operator/ (float S) const noexcept;

        friend XMMATRIX     XM_CALLCONV     operator* (float S, FXMMATRIX M) noexcept;
    };

    //------------------------------------------------------------------------------
    // 2D Vector; 32 bit floating point components
    struct XMFLOAT2
    {
        float x;
        float y;

        XMFLOAT2() = default;

        XMFLOAT2(const XMFLOAT2&) = default;
        XMFLOAT2& operator=(const XMFLOAT2&) = default;

        XMFLOAT2(XMFLOAT2&&) = default;
        XMFLOAT2& operator=(XMFLOAT2&&) = default;

        constexpr XMFLOAT2(float _x, float _y) noexcept : x(_x), y(_y) {}
        explicit XMFLOAT2(_In_reads_(2) const float* pArray)  noexcept : x(pArray[0]), y(pArray[1]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMFLOAT2&) const = default;
        auto operator <=> (const XMFLOAT2&) const = default;
#endif
    };

    // 2D Vector; 32 bit floating point components aligned on a 16 byte boundary
    XM_ALIGNED_STRUCT(16) XMFLOAT2A : public XMFLOAT2
    {
        using XMFLOAT2::XMFLOAT2;
    };

    //------------------------------------------------------------------------------
    // 2D Vector; 32 bit signed integer components
    struct XMINT2
    {
        int32_t x;
        int32_t y;

        XMINT2() = default;

        XMINT2(const XMINT2&) = default;
        XMINT2& operator=(const XMINT2&) = default;

        XMINT2(XMINT2&&) = default;
        XMINT2& operator=(XMINT2&&) = default;

        constexpr XMINT2(int32_t _x, int32_t _y) noexcept : x(_x), y(_y) {}
        explicit XMINT2(_In_reads_(2) const int32_t* pArray) noexcept : x(pArray[0]), y(pArray[1]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMINT2&) const = default;
        auto operator <=> (const XMINT2&) const = default;
#endif
    };

    // 2D Vector; 32 bit unsigned integer components
    struct XMUINT2
    {
        uint32_t x;
        uint32_t y;

        XMUINT2() = default;

        XMUINT2(const XMUINT2&) = default;
        XMUINT2& operator=(const XMUINT2&) = default;

        XMUINT2(XMUINT2&&) = default;
        XMUINT2& operator=(XMUINT2&&) = default;

        constexpr XMUINT2(uint32_t _x, uint32_t _y) noexcept : x(_x), y(_y) {}
        explicit XMUINT2(_In_reads_(2) const uint32_t* pArray) noexcept : x(pArray[0]), y(pArray[1]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMUINT2&) const = default;
        auto operator <=> (const XMUINT2&) const = default;
#endif
    };

    //------------------------------------------------------------------------------
    // 3D Vector; 32 bit floating point components
    struct XMFLOAT3
    {
        float x;
        float y;
        float z;

        XMFLOAT3() = default;

        XMFLOAT3(const XMFLOAT3&) = default;
        XMFLOAT3& operator=(const XMFLOAT3&) = default;

        XMFLOAT3(XMFLOAT3&&) = default;
        XMFLOAT3& operator=(XMFLOAT3&&) = default;

        constexpr XMFLOAT3(float _x, float _y, float _z) noexcept : x(_x), y(_y), z(_z) {}
        explicit XMFLOAT3(_In_reads_(3) const float* pArray) noexcept : x(pArray[0]), y(pArray[1]), z(pArray[2]) {}
    };

    // 3D Vector; 32 bit floating point components aligned on a 16 byte boundary
    XM_ALIGNED_STRUCT(16) XMFLOAT3A : public XMFLOAT3
    {
        using XMFLOAT3::XMFLOAT3;
    };

    //------------------------------------------------------------------------------
    // 3D Vector; 32 bit signed integer components
    struct XMINT3
    {
        int32_t x;
        int32_t y;
        int32_t z;

        XMINT3() = default;

        XMINT3(const XMINT3&) = default;
        XMINT3& operator=(const XMINT3&) = default;

        XMINT3(XMINT3&&) = default;
        XMINT3& operator=(XMINT3&&) = default;

        constexpr XMINT3(int32_t _x, int32_t _y, int32_t _z) noexcept : x(_x), y(_y), z(_z) {}
        explicit XMINT3(_In_reads_(3) const int32_t* pArray) noexcept : x(pArray[0]), y(pArray[1]), z(pArray[2]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMINT3&) const = default;
        auto operator <=> (const XMINT3&) const = default;
#endif
    };

    // 3D Vector; 32 bit unsigned integer components
    struct XMUINT3
    {
        uint32_t x;
        uint32_t y;
        uint32_t z;

        XMUINT3() = default;

        XMUINT3(const XMUINT3&) = default;
        XMUINT3& operator=(const XMUINT3&) = default;

        XMUINT3(XMUINT3&&) = default;
        XMUINT3& operator=(XMUINT3&&) = default;

        constexpr XMUINT3(uint32_t _x, uint32_t _y, uint32_t _z) noexcept : x(_x), y(_y), z(_z) {}
        explicit XMUINT3(_In_reads_(3) const uint32_t* pArray) noexcept : x(pArray[0]), y(pArray[1]), z(pArray[2]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMUINT3&) const = default;
        auto operator <=> (const XMUINT3&) const = default;
#endif
    };

    //------------------------------------------------------------------------------
    // 4D Vector; 32 bit floating point components
    struct XMFLOAT4
    {
        float x;
        float y;
        float z;
        float w;

        XMFLOAT4() = default;

        XMFLOAT4(const XMFLOAT4&) = default;
        XMFLOAT4& operator=(const XMFLOAT4&) = default;

        XMFLOAT4(XMFLOAT4&&) = default;
        XMFLOAT4& operator=(XMFLOAT4&&) = default;

        constexpr XMFLOAT4(float _x, float _y, float _z, float _w) noexcept : x(_x), y(_y), z(_z), w(_w) {}
        explicit XMFLOAT4(_In_reads_(4) const float* pArray) noexcept : x(pArray[0]), y(pArray[1]), z(pArray[2]), w(pArray[3]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMFLOAT4&) const = default;
        auto operator <=> (const XMFLOAT4&) const = default;
#endif
    };

    // 4D Vector; 32 bit floating point components aligned on a 16 byte boundary
    XM_ALIGNED_STRUCT(16) XMFLOAT4A : public XMFLOAT4
    {
        using XMFLOAT4::XMFLOAT4;
    };

    //------------------------------------------------------------------------------
    // 4D Vector; 32 bit signed integer components
    struct XMINT4
    {
        int32_t x;
        int32_t y;
        int32_t z;
        int32_t w;

        XMINT4() = default;

        XMINT4(const XMINT4&) = default;
        XMINT4& operator=(const XMINT4&) = default;

        XMINT4(XMINT4&&) = default;
        XMINT4& operator=(XMINT4&&) = default;

        constexpr XMINT4(int32_t _x, int32_t _y, int32_t _z, int32_t _w) noexcept : x(_x), y(_y), z(_z), w(_w) {}
        explicit XMINT4(_In_reads_(4) const int32_t* pArray) noexcept : x(pArray[0]), y(pArray[1]), z(pArray[2]), w(pArray[3]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMINT4&) const = default;
        auto operator <=> (const XMINT4&) const = default;
#endif
    };

    // 4D Vector; 32 bit unsigned integer components
    struct XMUINT4
    {
        uint32_t x;
        uint32_t y;
        uint32_t z;
        uint32_t w;

        XMUINT4() = default;

        XMUINT4(const XMUINT4&) = default;
        XMUINT4& operator=(const XMUINT4&) = default;

        XMUINT4(XMUINT4&&) = default;
        XMUINT4& operator=(XMUINT4&&) = default;

        constexpr XMUINT4(uint32_t _x, uint32_t _y, uint32_t _z, uint32_t _w) noexcept : x(_x), y(_y), z(_z), w(_w) {}
        explicit XMUINT4(_In_reads_(4) const uint32_t* pArray) noexcept : x(pArray[0]), y(pArray[1]), z(pArray[2]), w(pArray[3]) {}

#if (__cplusplus >= 202002L)
        bool operator == (const XMUINT4&) const = default;
        auto operator <=> (const XMUINT4&) const = default;
#endif
    };

#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wgnu-anonymous-struct"
#pragma clang diagnostic ignored "-Wnested-anon-types"
#pragma clang diagnostic ignored "-Wunknown-warning-option"
#pragma clang diagnostic ignored "-Wunsafe-buffer-usage"
#endif

    //------------------------------------------------------------------------------
    // 3x3 Matrix: 32 bit floating point components
    struct XMFLOAT3X3
    {
        union
        {
            struct
            {
                float _11, _12, _13;
                float _21, _22, _23;
                float _31, _32, _33;
            };
            float m[3][3];
        };

        XMFLOAT3X3() = default;

        XMFLOAT3X3(const XMFLOAT3X3&) = default;
        XMFLOAT3X3& operator=(const XMFLOAT3X3&) = default;

        XMFLOAT3X3(XMFLOAT3X3&&) = default;
        XMFLOAT3X3& operator=(XMFLOAT3X3&&) = default;

        constexpr XMFLOAT3X3(float m00, float m01, float m02,
            float m10, float m11, float m12,
            float m20, float m21, float m22) noexcept
            : _11(m00), _12(m01), _13(m02),
            _21(m10), _22(m11), _23(m12),
            _31(m20), _32(m21), _33(m22) {}
        explicit XMFLOAT3X3(_In_reads_(9) const float* pArray) noexcept;

        float  operator() (size_t Row, size_t Column) const  noexcept { return m[Row][Column]; }
        float& operator() (size_t Row, size_t Column) noexcept { return m[Row][Column]; }

#if (__cplusplus >= 202002L)
        bool operator == (const XMFLOAT3X3&) const = default;
        auto operator <=> (const XMFLOAT3X3&) const = default;
#endif
    };

    //------------------------------------------------------------------------------
    // 4x3 Row-major Matrix: 32 bit floating point components
    struct XMFLOAT4X3
    {
        union
        {
            struct
            {
                float _11, _12, _13;
                float _21, _22, _23;
                float _31, _32, _33;
                float _41, _42, _43;
            };
            float m[4][3];
            float f[12];
        };

        XMFLOAT4X3() = default;

        XMFLOAT4X3(const XMFLOAT4X3&) = default;
        XMFLOAT4X3& operator=(const XMFLOAT4X3&) = default;

        XMFLOAT4X3(XMFLOAT4X3&&) = default;
        XMFLOAT4X3& operator=(XMFLOAT4X3&&) = default;

        constexpr XMFLOAT4X3(float m00, float m01, float m02,
            float m10, float m11, float m12,
            float m20, float m21, float m22,
            float m30, float m31, float m32) noexcept
            : _11(m00), _12(m01), _13(m02),
            _21(m10), _22(m11), _23(m12),
            _31(m20), _32(m21), _33(m22),
            _41(m30), _42(m31), _43(m32) {}
        explicit XMFLOAT4X3(_In_reads_(12) const float* pArray) noexcept;

        float  operator() (size_t Row, size_t Column) const  noexcept { return m[Row][Column]; }
        float& operator() (size_t Row, size_t Column) noexcept { return m[Row][Column]; }

#if (__cplusplus >= 202002L)
        bool operator == (const XMFLOAT4X3&) const = default;
        auto operator <=> (const XMFLOAT4X3&) const = default;
#endif
    };

    // 4x3 Row-major Matrix: 32 bit floating point components aligned on a 16 byte boundary
    XM_ALIGNED_STRUCT(16) XMFLOAT4X3A : public XMFLOAT4X3
    {
        using XMFLOAT4X3::XMFLOAT4X3;
    };

    //------------------------------------------------------------------------------
    // 3x4 Column-major Matrix: 32 bit floating point components
    struct XMFLOAT3X4
    {
        union
        {
            struct
            {
                float _11, _12, _13, _14;
                float _21, _22, _23, _24;
                float _31, _32, _33, _34;
            };
            float m[3][4];
            float f[12];
        };

        XMFLOAT3X4() = default;

        XMFLOAT3X4(const XMFLOAT3X4&) = default;
        XMFLOAT3X4& operator=(const XMFLOAT3X4&) = default;

        XMFLOAT3X4(XMFLOAT3X4&&) = default;
        XMFLOAT3X4& operator=(XMFLOAT3X4&&) = default;

        constexpr XMFLOAT3X4(float m00, float m01, float m02, float m03,
            float m10, float m11, float m12, float m13,
            float m20, float m21, float m22, float m23) noexcept
            : _11(m00), _12(m01), _13(m02), _14(m03),
            _21(m10), _22(m11), _23(m12), _24(m13),
            _31(m20), _32(m21), _33(m22), _34(m23) {}
        explicit XMFLOAT3X4(_In_reads_(12) const float* pArray) noexcept;

        float  operator() (size_t Row, size_t Column) const noexcept { return m[Row][Column]; }
        float& operator() (size_t Row, size_t Column) noexcept { return m[Row][Column]; }

#if (__cplusplus >= 202002L)
        bool operator == (const XMFLOAT3X4&) const = default;
        auto operator <=> (const XMFLOAT3X4&) const = default;
#endif
    };

    // 3x4 Column-major Matrix: 32 bit floating point components aligned on a 16 byte boundary
    XM_ALIGNED_STRUCT(16) XMFLOAT3X4A : public XMFLOAT3X4
    {
        using XMFLOAT3X4::XMFLOAT3X4;
    };

    //------------------------------------------------------------------------------
    // 4x4 Matrix: 32 bit floating point components
    struct XMFLOAT4X4
    {
        union
        {
            struct
            {
                float _11, _12, _13, _14;
                float _21, _22, _23, _24;
                float _31, _32, _33, _34;
                float _41, _42, _43, _44;
            };
            float m[4][4];
        };

        XMFLOAT4X4() = default;

        XMFLOAT4X4(const XMFLOAT4X4&) = default;
        XMFLOAT4X4& operator=(const XMFLOAT4X4&) = default;

        XMFLOAT4X4(XMFLOAT4X4&&) = default;
        XMFLOAT4X4& operator=(XMFLOAT4X4&&) = default;

        constexpr XMFLOAT4X4(float m00, float m01, float m02, float m03,
            float m10, float m11, float m12, float m13,
            float m20, float m21, float m22, float m23,
            float m30, float m31, float m32, float m33) noexcept
            : _11(m00), _12(m01), _13(m02), _14(m03),
            _21(m10), _22(m11), _23(m12), _24(m13),
            _31(m20), _32(m21), _33(m22), _34(m23),
            _41(m30), _42(m31), _43(m32), _44(m33) {}
        explicit XMFLOAT4X4(_In_reads_(16) const float* pArray) noexcept;

        float  operator() (size_t Row, size_t Column) const noexcept { return m[Row][Column]; }
        float& operator() (size_t Row, size_t Column) noexcept { return m[Row][Column]; }

#if (__cplusplus >= 202002L)
        bool operator == (const XMFLOAT4X4&) const = default;
        auto operator <=> (const XMFLOAT4X4&) const = default;
#endif
    };

    // 4x4 Matrix: 32 bit floating point components aligned on a 16 byte boundary
    XM_ALIGNED_STRUCT(16) XMFLOAT4X4A : public XMFLOAT4X4
    {
        using XMFLOAT4X4::XMFLOAT4X4;
    };

    ////////////////////////////////////////////////////////////////////////////////

#ifdef __clang__
#pragma clang diagnostic pop
#endif
#ifdef _PREFAST_
#pragma prefast(pop)
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

/****************************************************************************
 *
 * Data conversion operations
 *
 ****************************************************************************/

    XMVECTOR    XM_CALLCONV     XMConvertVectorIntToFloat(FXMVECTOR VInt, uint32_t DivExponent) noexcept;
    XMVECTOR    XM_CALLCONV     XMConvertVectorFloatToInt(FXMVECTOR VFloat, uint32_t MulExponent) noexcept;
    XMVECTOR    XM_CALLCONV     XMConvertVectorUIntToFloat(FXMVECTOR VUInt, uint32_t DivExponent) noexcept;
    XMVECTOR    XM_CALLCONV     XMConvertVectorFloatToUInt(FXMVECTOR VFloat, uint32_t MulExponent) noexcept;

#if defined(__XNAMATH_H__) && defined(XMVectorSetBinaryConstant)
#undef XMVectorSetBinaryConstant
#undef XMVectorSplatConstant
#undef XMVectorSplatConstantInt
#endif

    XMVECTOR    XM_CALLCONV     XMVectorSetBinaryConstant(uint32_t C0, uint32_t C1, uint32_t C2, uint32_t C3) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatConstant(int32_t IntConstant, uint32_t DivExponent) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatConstantInt(int32_t IntConstant) noexcept;

    /****************************************************************************
     *
     * Load operations
     *
     ****************************************************************************/

    XMVECTOR    XM_CALLCONV     XMLoadInt(_In_ const uint32_t* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat(_In_ const float* pSource) noexcept;

    XMVECTOR    XM_CALLCONV     XMLoadInt2(_In_reads_(2) const uint32_t* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadInt2A(_In_reads_(2) const uint32_t* PSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat2(_In_ const XMFLOAT2* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat2A(_In_ const XMFLOAT2A* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadSInt2(_In_ const XMINT2* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadUInt2(_In_ const XMUINT2* pSource) noexcept;

    XMVECTOR    XM_CALLCONV     XMLoadInt3(_In_reads_(3) const uint32_t* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadInt3A(_In_reads_(3) const uint32_t* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat3(_In_ const XMFLOAT3* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat3A(_In_ const XMFLOAT3A* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadSInt3(_In_ const XMINT3* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadUInt3(_In_ const XMUINT3* pSource) noexcept;

    XMVECTOR    XM_CALLCONV     XMLoadInt4(_In_reads_(4) const uint32_t* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadInt4A(_In_reads_(4) const uint32_t* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat4(_In_ const XMFLOAT4* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadFloat4A(_In_ const XMFLOAT4A* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadSInt4(_In_ const XMINT4* pSource) noexcept;
    XMVECTOR    XM_CALLCONV     XMLoadUInt4(_In_ const XMUINT4* pSource) noexcept;

    XMMATRIX    XM_CALLCONV     XMLoadFloat3x3(_In_ const XMFLOAT3X3* pSource) noexcept;
    XMMATRIX    XM_CALLCONV     XMLoadFloat4x3(_In_ const XMFLOAT4X3* pSource) noexcept;
    XMMATRIX    XM_CALLCONV     XMLoadFloat4x3A(_In_ const XMFLOAT4X3A* pSource) noexcept;
    XMMATRIX    XM_CALLCONV     XMLoadFloat3x4(_In_ const XMFLOAT3X4* pSource) noexcept;
    XMMATRIX    XM_CALLCONV     XMLoadFloat3x4A(_In_ const XMFLOAT3X4A* pSource) noexcept;
    XMMATRIX    XM_CALLCONV     XMLoadFloat4x4(_In_ const XMFLOAT4X4* pSource) noexcept;
    XMMATRIX    XM_CALLCONV     XMLoadFloat4x4A(_In_ const XMFLOAT4X4A* pSource) noexcept;

    /****************************************************************************
     *
     * Store operations
     *
     ****************************************************************************/

    void        XM_CALLCONV     XMStoreInt(_Out_ uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat(_Out_ float* pDestination, _In_ FXMVECTOR V) noexcept;

    void        XM_CALLCONV     XMStoreInt2(_Out_writes_(2) uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreInt2A(_Out_writes_(2) uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat2(_Out_ XMFLOAT2* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat2A(_Out_ XMFLOAT2A* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreSInt2(_Out_ XMINT2* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreUInt2(_Out_ XMUINT2* pDestination, _In_ FXMVECTOR V) noexcept;

    void        XM_CALLCONV     XMStoreInt3(_Out_writes_(3) uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreInt3A(_Out_writes_(3) uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat3(_Out_ XMFLOAT3* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat3A(_Out_ XMFLOAT3A* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreSInt3(_Out_ XMINT3* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreUInt3(_Out_ XMUINT3* pDestination, _In_ FXMVECTOR V) noexcept;

    void        XM_CALLCONV     XMStoreInt4(_Out_writes_(4) uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreInt4A(_Out_writes_(4) uint32_t* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat4(_Out_ XMFLOAT4* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreFloat4A(_Out_ XMFLOAT4A* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreSInt4(_Out_ XMINT4* pDestination, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMStoreUInt4(_Out_ XMUINT4* pDestination, _In_ FXMVECTOR V) noexcept;

    void        XM_CALLCONV     XMStoreFloat3x3(_Out_ XMFLOAT3X3* pDestination, _In_ FXMMATRIX M) noexcept;
    void        XM_CALLCONV     XMStoreFloat4x3(_Out_ XMFLOAT4X3* pDestination, _In_ FXMMATRIX M) noexcept;
    void        XM_CALLCONV     XMStoreFloat4x3A(_Out_ XMFLOAT4X3A* pDestination, _In_ FXMMATRIX M) noexcept;
    void        XM_CALLCONV     XMStoreFloat3x4(_Out_ XMFLOAT3X4* pDestination, _In_ FXMMATRIX M) noexcept;
    void        XM_CALLCONV     XMStoreFloat3x4A(_Out_ XMFLOAT3X4A* pDestination, _In_ FXMMATRIX M) noexcept;
    void        XM_CALLCONV     XMStoreFloat4x4(_Out_ XMFLOAT4X4* pDestination, _In_ FXMMATRIX M) noexcept;
    void        XM_CALLCONV     XMStoreFloat4x4A(_Out_ XMFLOAT4X4A* pDestination, _In_ FXMMATRIX M) noexcept;

    /****************************************************************************
     *
     * General vector operations
     *
     ****************************************************************************/

    XMVECTOR    XM_CALLCONV     XMVectorZero() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSet(float x, float y, float z, float w) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetInt(uint32_t x, uint32_t y, uint32_t z, uint32_t w) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReplicate(float Value) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReplicatePtr(_In_ const float* pValue) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReplicateInt(uint32_t Value) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReplicateIntPtr(_In_ const uint32_t* pValue) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorTrueInt() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorFalseInt() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatX(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatY(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatZ(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatW(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatOne() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatInfinity() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatQNaN() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatEpsilon() noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSplatSignMask() noexcept;

    float       XM_CALLCONV     XMVectorGetByIndex(FXMVECTOR V, size_t i) noexcept;
    float       XM_CALLCONV     XMVectorGetX(FXMVECTOR V) noexcept;
    float       XM_CALLCONV     XMVectorGetY(FXMVECTOR V) noexcept;
    float       XM_CALLCONV     XMVectorGetZ(FXMVECTOR V) noexcept;
    float       XM_CALLCONV     XMVectorGetW(FXMVECTOR V) noexcept;

    void        XM_CALLCONV     XMVectorGetByIndexPtr(_Out_ float* f, _In_ FXMVECTOR V, _In_ size_t i) noexcept;
    void        XM_CALLCONV     XMVectorGetXPtr(_Out_ float* x, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorGetYPtr(_Out_ float* y, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorGetZPtr(_Out_ float* z, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorGetWPtr(_Out_ float* w, _In_ FXMVECTOR V) noexcept;

    uint32_t    XM_CALLCONV     XMVectorGetIntByIndex(FXMVECTOR V, size_t i) noexcept;
    uint32_t    XM_CALLCONV     XMVectorGetIntX(FXMVECTOR V) noexcept;
    uint32_t    XM_CALLCONV     XMVectorGetIntY(FXMVECTOR V) noexcept;
    uint32_t    XM_CALLCONV     XMVectorGetIntZ(FXMVECTOR V) noexcept;
    uint32_t    XM_CALLCONV     XMVectorGetIntW(FXMVECTOR V) noexcept;

    void        XM_CALLCONV     XMVectorGetIntByIndexPtr(_Out_ uint32_t* x, _In_ FXMVECTOR V, _In_ size_t i) noexcept;
    void        XM_CALLCONV     XMVectorGetIntXPtr(_Out_ uint32_t* x, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorGetIntYPtr(_Out_ uint32_t* y, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorGetIntZPtr(_Out_ uint32_t* z, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorGetIntWPtr(_Out_ uint32_t* w, _In_ FXMVECTOR V) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorSetByIndex(FXMVECTOR V, float f, size_t i) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetX(FXMVECTOR V, float x) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetY(FXMVECTOR V, float y) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetZ(FXMVECTOR V, float z) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetW(FXMVECTOR V, float w) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorSetByIndexPtr(_In_ FXMVECTOR V, _In_ const float* f, _In_ size_t i) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetXPtr(_In_ FXMVECTOR V, _In_ const float* x) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetYPtr(_In_ FXMVECTOR V, _In_ const float* y) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetZPtr(_In_ FXMVECTOR V, _In_ const float* z) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetWPtr(_In_ FXMVECTOR V, _In_ const float* w) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorSetIntByIndex(FXMVECTOR V, uint32_t x, size_t i) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntX(FXMVECTOR V, uint32_t x) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntY(FXMVECTOR V, uint32_t y) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntZ(FXMVECTOR V, uint32_t z) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntW(FXMVECTOR V, uint32_t w) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorSetIntByIndexPtr(_In_ FXMVECTOR V, _In_ const uint32_t* x, _In_ size_t i) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntXPtr(_In_ FXMVECTOR V, _In_ const uint32_t* x) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntYPtr(_In_ FXMVECTOR V, _In_ const uint32_t* y) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntZPtr(_In_ FXMVECTOR V, _In_ const uint32_t* z) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSetIntWPtr(_In_ FXMVECTOR V, _In_ const uint32_t* w) noexcept;

#if defined(__XNAMATH_H__) && defined(XMVectorSwizzle)
#undef XMVectorSwizzle
#endif

    XMVECTOR    XM_CALLCONV     XMVectorSwizzle(FXMVECTOR V, uint32_t E0, uint32_t E1, uint32_t E2, uint32_t E3) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorPermute(FXMVECTOR V1, FXMVECTOR V2, uint32_t PermuteX, uint32_t PermuteY, uint32_t PermuteZ, uint32_t PermuteW) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSelectControl(uint32_t VectorIndex0, uint32_t VectorIndex1, uint32_t VectorIndex2, uint32_t VectorIndex3) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSelect(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR Control) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorMergeXY(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorMergeZW(FXMVECTOR V1, FXMVECTOR V2) noexcept;

#if defined(__XNAMATH_H__) && defined(XMVectorShiftLeft)
#undef XMVectorShiftLeft
#undef XMVectorRotateLeft
#undef XMVectorRotateRight
#undef XMVectorInsert
#endif

    XMVECTOR    XM_CALLCONV     XMVectorShiftLeft(FXMVECTOR V1, FXMVECTOR V2, uint32_t Elements) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorRotateLeft(FXMVECTOR V, uint32_t Elements) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorRotateRight(FXMVECTOR V, uint32_t Elements) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorInsert(FXMVECTOR VD, FXMVECTOR VS, uint32_t VSLeftRotateElements,
        uint32_t Select0, uint32_t Select1, uint32_t Select2, uint32_t Select3) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorEqualR(_Out_ uint32_t* pCR, _In_ FXMVECTOR V1, _In_ FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorEqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorEqualIntR(_Out_ uint32_t* pCR, _In_ FXMVECTOR V, _In_ FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorNearEqual(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR Epsilon) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorNotEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorNotEqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorGreater(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorGreaterR(_Out_ uint32_t* pCR, _In_ FXMVECTOR V1, _In_ FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorGreaterOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorGreaterOrEqualR(_Out_ uint32_t* pCR, _In_ FXMVECTOR V1, _In_ FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLess(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLessOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorInBounds(FXMVECTOR V, FXMVECTOR Bounds) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorInBoundsR(_Out_ uint32_t* pCR, _In_ FXMVECTOR V, _In_ FXMVECTOR Bounds) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorIsNaN(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorIsInfinite(FXMVECTOR V) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorMin(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorMax(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorRound(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorTruncate(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorFloor(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorCeiling(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorClamp(FXMVECTOR V, FXMVECTOR Min, FXMVECTOR Max) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSaturate(FXMVECTOR V) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorAndInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorAndCInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorOrInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorNorInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorXorInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;

    XMVECTOR    XM_CALLCONV     XMVectorNegate(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorAdd(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSum(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorAddAngles(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSubtract(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSubtractAngles(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorMultiply(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorMultiplyAdd(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR V3) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorDivide(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorNegativeMultiplySubtract(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR V3) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorScale(FXMVECTOR V, float ScaleFactor) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReciprocalEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReciprocal(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSqrtEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSqrt(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReciprocalSqrtEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorReciprocalSqrt(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorExp2(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorExp10(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorExpE(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorExp(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLog2(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLog10(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLogE(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLog(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorPow(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorAbs(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorMod(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorModAngles(FXMVECTOR Angles) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSin(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSinEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorCos(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorCosEst(FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorSinCos(_Out_ XMVECTOR* pSin, _Out_ XMVECTOR* pCos, _In_ FXMVECTOR V) noexcept;
    void        XM_CALLCONV     XMVectorSinCosEst(_Out_ XMVECTOR* pSin, _Out_ XMVECTOR* pCos, _In_ FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorTan(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorTanEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorSinH(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorCosH(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorTanH(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorASin(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorASinEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorACos(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorACosEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorATan(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorATanEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorATan2(FXMVECTOR Y, FXMVECTOR X) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorATan2Est(FXMVECTOR Y, FXMVECTOR X) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLerp(FXMVECTOR V0, FXMVECTOR V1, float t) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorLerpV(FXMVECTOR V0, FXMVECTOR V1, FXMVECTOR T) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorHermite(FXMVECTOR Position0, FXMVECTOR Tangent0, FXMVECTOR Position1, GXMVECTOR Tangent1, float t) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorHermiteV(FXMVECTOR Position0, FXMVECTOR Tangent0, FXMVECTOR Position1, GXMVECTOR Tangent1, HXMVECTOR T) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorCatmullRom(FXMVECTOR Position0, FXMVECTOR Position1, FXMVECTOR Position2, GXMVECTOR Position3, float t) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorCatmullRomV(FXMVECTOR Position0, FXMVECTOR Position1, FXMVECTOR Position2, GXMVECTOR Position3, HXMVECTOR T) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorBaryCentric(FXMVECTOR Position0, FXMVECTOR Position1, FXMVECTOR Position2, float f, float g) noexcept;
    XMVECTOR    XM_CALLCONV     XMVectorBaryCentricV(FXMVECTOR Position0, FXMVECTOR Position1, FXMVECTOR Position2, GXMVECTOR F, HXMVECTOR G) noexcept;

    /****************************************************************************
     *
     * 2D vector operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMVector2Equal(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector2EqualR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2EqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector2EqualIntR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2NearEqual(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR Epsilon) noexcept;
    bool        XM_CALLCONV     XMVector2NotEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2NotEqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2Greater(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector2GreaterR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2GreaterOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector2GreaterOrEqualR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2Less(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2LessOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector2InBounds(FXMVECTOR V, FXMVECTOR Bounds) noexcept;

    bool        XM_CALLCONV     XMVector2IsNaN(FXMVECTOR V) noexcept;
    bool        XM_CALLCONV     XMVector2IsInfinite(FXMVECTOR V) noexcept;

    XMVECTOR    XM_CALLCONV     XMVector2Dot(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Cross(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2LengthSq(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2ReciprocalLengthEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2ReciprocalLength(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2LengthEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Length(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2NormalizeEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Normalize(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2ClampLength(FXMVECTOR V, float LengthMin, float LengthMax) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2ClampLengthV(FXMVECTOR V, FXMVECTOR LengthMin, FXMVECTOR LengthMax) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Reflect(FXMVECTOR Incident, FXMVECTOR Normal) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Refract(FXMVECTOR Incident, FXMVECTOR Normal, float RefractionIndex) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2RefractV(FXMVECTOR Incident, FXMVECTOR Normal, FXMVECTOR RefractionIndex) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Orthogonal(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2AngleBetweenNormalsEst(FXMVECTOR N1, FXMVECTOR N2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2AngleBetweenNormals(FXMVECTOR N1, FXMVECTOR N2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2AngleBetweenVectors(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2LinePointDistance(FXMVECTOR LinePoint1, FXMVECTOR LinePoint2, FXMVECTOR Point) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2IntersectLine(FXMVECTOR Line1Point1, FXMVECTOR Line1Point2, FXMVECTOR Line2Point1, GXMVECTOR Line2Point2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2Transform(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT4*   XM_CALLCONV     XMVector2TransformStream(_Out_writes_bytes_(sizeof(XMFLOAT4) + OutputStride * (VectorCount - 1)) XMFLOAT4* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT2) + InputStride * (VectorCount - 1)) const XMFLOAT2* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2TransformCoord(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT2*   XM_CALLCONV     XMVector2TransformCoordStream(_Out_writes_bytes_(sizeof(XMFLOAT2) + OutputStride * (VectorCount - 1)) XMFLOAT2* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT2) + InputStride * (VectorCount - 1)) const XMFLOAT2* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector2TransformNormal(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT2*   XM_CALLCONV     XMVector2TransformNormalStream(_Out_writes_bytes_(sizeof(XMFLOAT2) + OutputStride * (VectorCount - 1)) XMFLOAT2* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT2) + InputStride * (VectorCount - 1)) const XMFLOAT2* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;

    /****************************************************************************
     *
     * 3D vector operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMVector3Equal(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector3EqualR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3EqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector3EqualIntR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3NearEqual(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR Epsilon) noexcept;
    bool        XM_CALLCONV     XMVector3NotEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3NotEqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3Greater(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector3GreaterR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3GreaterOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector3GreaterOrEqualR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3Less(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3LessOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector3InBounds(FXMVECTOR V, FXMVECTOR Bounds) noexcept;

    bool        XM_CALLCONV     XMVector3IsNaN(FXMVECTOR V) noexcept;
    bool        XM_CALLCONV     XMVector3IsInfinite(FXMVECTOR V) noexcept;

    XMVECTOR    XM_CALLCONV     XMVector3Dot(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Cross(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3LengthSq(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3ReciprocalLengthEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3ReciprocalLength(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3LengthEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Length(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3NormalizeEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Normalize(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3ClampLength(FXMVECTOR V, float LengthMin, float LengthMax) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3ClampLengthV(FXMVECTOR V, FXMVECTOR LengthMin, FXMVECTOR LengthMax) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Reflect(FXMVECTOR Incident, FXMVECTOR Normal) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Refract(FXMVECTOR Incident, FXMVECTOR Normal, float RefractionIndex) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3RefractV(FXMVECTOR Incident, FXMVECTOR Normal, FXMVECTOR RefractionIndex) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Orthogonal(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3AngleBetweenNormalsEst(FXMVECTOR N1, FXMVECTOR N2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3AngleBetweenNormals(FXMVECTOR N1, FXMVECTOR N2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3AngleBetweenVectors(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3LinePointDistance(FXMVECTOR LinePoint1, FXMVECTOR LinePoint2, FXMVECTOR Point) noexcept;
    void        XM_CALLCONV     XMVector3ComponentsFromNormal(_Out_ XMVECTOR* pParallel, _Out_ XMVECTOR* pPerpendicular, _In_ FXMVECTOR V, _In_ FXMVECTOR Normal) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Rotate(FXMVECTOR V, FXMVECTOR RotationQuaternion) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3InverseRotate(FXMVECTOR V, FXMVECTOR RotationQuaternion) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Transform(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT4*   XM_CALLCONV     XMVector3TransformStream(_Out_writes_bytes_(sizeof(XMFLOAT4) + OutputStride * (VectorCount - 1)) XMFLOAT4* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT3) + InputStride * (VectorCount - 1)) const XMFLOAT3* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3TransformCoord(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT3*   XM_CALLCONV     XMVector3TransformCoordStream(_Out_writes_bytes_(sizeof(XMFLOAT3) + OutputStride * (VectorCount - 1)) XMFLOAT3* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT3) + InputStride * (VectorCount - 1)) const XMFLOAT3* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3TransformNormal(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT3*   XM_CALLCONV     XMVector3TransformNormalStream(_Out_writes_bytes_(sizeof(XMFLOAT3) + OutputStride * (VectorCount - 1)) XMFLOAT3* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT3) + InputStride * (VectorCount - 1)) const XMFLOAT3* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Project(FXMVECTOR V, float ViewportX, float ViewportY, float ViewportWidth, float ViewportHeight, float ViewportMinZ, float ViewportMaxZ,
        FXMMATRIX Projection, CXMMATRIX View, CXMMATRIX World) noexcept;
    XMFLOAT3*   XM_CALLCONV     XMVector3ProjectStream(_Out_writes_bytes_(sizeof(XMFLOAT3) + OutputStride * (VectorCount - 1)) XMFLOAT3* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT3) + InputStride * (VectorCount - 1)) const XMFLOAT3* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount,
        _In_ float ViewportX, _In_ float ViewportY, _In_ float ViewportWidth, _In_ float ViewportHeight, _In_ float ViewportMinZ, _In_ float ViewportMaxZ,
        _In_ FXMMATRIX Projection, _In_ CXMMATRIX View, _In_ CXMMATRIX World) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector3Unproject(FXMVECTOR V, float ViewportX, float ViewportY, float ViewportWidth, float ViewportHeight, float ViewportMinZ, float ViewportMaxZ,
        FXMMATRIX Projection, CXMMATRIX View, CXMMATRIX World) noexcept;
    XMFLOAT3*   XM_CALLCONV     XMVector3UnprojectStream(_Out_writes_bytes_(sizeof(XMFLOAT3) + OutputStride * (VectorCount - 1)) XMFLOAT3* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT3) + InputStride * (VectorCount - 1)) const XMFLOAT3* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount,
        _In_ float ViewportX, _In_ float ViewportY, _In_ float ViewportWidth, _In_ float ViewportHeight, _In_ float ViewportMinZ, _In_ float ViewportMaxZ,
        _In_ FXMMATRIX Projection, _In_ CXMMATRIX View, _In_ CXMMATRIX World) noexcept;

    /****************************************************************************
     *
     * 4D vector operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMVector4Equal(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector4EqualR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4EqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector4EqualIntR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4NearEqual(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR Epsilon) noexcept;
    bool        XM_CALLCONV     XMVector4NotEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4NotEqualInt(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4Greater(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector4GreaterR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4GreaterOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    uint32_t    XM_CALLCONV     XMVector4GreaterOrEqualR(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4Less(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4LessOrEqual(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    bool        XM_CALLCONV     XMVector4InBounds(FXMVECTOR V, FXMVECTOR Bounds) noexcept;

    bool        XM_CALLCONV     XMVector4IsNaN(FXMVECTOR V) noexcept;
    bool        XM_CALLCONV     XMVector4IsInfinite(FXMVECTOR V) noexcept;

    XMVECTOR    XM_CALLCONV     XMVector4Dot(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Cross(FXMVECTOR V1, FXMVECTOR V2, FXMVECTOR V3) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4LengthSq(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4ReciprocalLengthEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4ReciprocalLength(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4LengthEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Length(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4NormalizeEst(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Normalize(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4ClampLength(FXMVECTOR V, float LengthMin, float LengthMax) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4ClampLengthV(FXMVECTOR V, FXMVECTOR LengthMin, FXMVECTOR LengthMax) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Reflect(FXMVECTOR Incident, FXMVECTOR Normal) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Refract(FXMVECTOR Incident, FXMVECTOR Normal, float RefractionIndex) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4RefractV(FXMVECTOR Incident, FXMVECTOR Normal, FXMVECTOR RefractionIndex) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Orthogonal(FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4AngleBetweenNormalsEst(FXMVECTOR N1, FXMVECTOR N2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4AngleBetweenNormals(FXMVECTOR N1, FXMVECTOR N2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4AngleBetweenVectors(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMVector4Transform(FXMVECTOR V, FXMMATRIX M) noexcept;
    XMFLOAT4*   XM_CALLCONV     XMVector4TransformStream(_Out_writes_bytes_(sizeof(XMFLOAT4) + OutputStride * (VectorCount - 1)) XMFLOAT4* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT4) + InputStride * (VectorCount - 1)) const XMFLOAT4* pInputStream,
        _In_ size_t InputStride, _In_ size_t VectorCount, _In_ FXMMATRIX M) noexcept;

    /****************************************************************************
     *
     * Matrix operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMMatrixIsNaN(FXMMATRIX M) noexcept;
    bool        XM_CALLCONV     XMMatrixIsInfinite(FXMMATRIX M) noexcept;
    bool        XM_CALLCONV     XMMatrixIsIdentity(FXMMATRIX M) noexcept;

    XMMATRIX    XM_CALLCONV     XMMatrixMultiply(FXMMATRIX M1, CXMMATRIX M2) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixMultiplyTranspose(FXMMATRIX M1, CXMMATRIX M2) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixTranspose(FXMMATRIX M) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixInverse(_Out_opt_ XMVECTOR* pDeterminant, _In_ FXMMATRIX M) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixVectorTensorProduct(FXMVECTOR V1, FXMVECTOR V2) noexcept;
    XMVECTOR    XM_CALLCONV     XMMatrixDeterminant(FXMMATRIX M) noexcept;

    _Success_(return)
    bool        XM_CALLCONV     XMMatrixDecompose(_Out_ XMVECTOR* outScale, _Out_ XMVECTOR* outRotQuat, _Out_ XMVECTOR* outTrans, _In_ FXMMATRIX M) noexcept;

    XMMATRIX    XM_CALLCONV     XMMatrixIdentity() noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixSet(float m00, float m01, float m02, float m03,
        float m10, float m11, float m12, float m13,
        float m20, float m21, float m22, float m23,
        float m30, float m31, float m32, float m33) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixTranslation(float OffsetX, float OffsetY, float OffsetZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixTranslationFromVector(FXMVECTOR Offset) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixScaling(float ScaleX, float ScaleY, float ScaleZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixScalingFromVector(FXMVECTOR Scale) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixRotationX(float Angle) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixRotationY(float Angle) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixRotationZ(float Angle) noexcept;

    // Rotates about y-axis (Yaw), then x-axis (Pitch), then z-axis (Roll)
    XMMATRIX    XM_CALLCONV     XMMatrixRotationRollPitchYaw(float Pitch, float Yaw, float Roll) noexcept;

    // Rotates about y-axis (Angles.y), then x-axis (Angles.x), then z-axis (Angles.z)
    XMMATRIX    XM_CALLCONV     XMMatrixRotationRollPitchYawFromVector(FXMVECTOR Angles) noexcept;

    XMMATRIX    XM_CALLCONV     XMMatrixRotationNormal(FXMVECTOR NormalAxis, float Angle) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixRotationAxis(FXMVECTOR Axis, float Angle) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixRotationQuaternion(FXMVECTOR Quaternion) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixTransformation2D(FXMVECTOR ScalingOrigin, float ScalingOrientation, FXMVECTOR Scaling,
        FXMVECTOR RotationOrigin, float Rotation, GXMVECTOR Translation) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixTransformation(FXMVECTOR ScalingOrigin, FXMVECTOR ScalingOrientationQuaternion, FXMVECTOR Scaling,
        GXMVECTOR RotationOrigin, HXMVECTOR RotationQuaternion, HXMVECTOR Translation) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixAffineTransformation2D(FXMVECTOR Scaling, FXMVECTOR RotationOrigin, float Rotation, FXMVECTOR Translation) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixAffineTransformation(FXMVECTOR Scaling, FXMVECTOR RotationOrigin, FXMVECTOR RotationQuaternion, GXMVECTOR Translation) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixReflect(FXMVECTOR ReflectionPlane) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixShadow(FXMVECTOR ShadowPlane, FXMVECTOR LightPosition) noexcept;

    XMMATRIX    XM_CALLCONV     XMMatrixLookAtLH(FXMVECTOR EyePosition, FXMVECTOR FocusPosition, FXMVECTOR UpDirection) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixLookAtRH(FXMVECTOR EyePosition, FXMVECTOR FocusPosition, FXMVECTOR UpDirection) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixLookToLH(FXMVECTOR EyePosition, FXMVECTOR EyeDirection, FXMVECTOR UpDirection) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixLookToRH(FXMVECTOR EyePosition, FXMVECTOR EyeDirection, FXMVECTOR UpDirection) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixPerspectiveLH(float ViewWidth, float ViewHeight, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixPerspectiveRH(float ViewWidth, float ViewHeight, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixPerspectiveFovLH(float FovAngleY, float AspectRatio, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixPerspectiveFovRH(float FovAngleY, float AspectRatio, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixPerspectiveOffCenterLH(float ViewLeft, float ViewRight, float ViewBottom, float ViewTop, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixPerspectiveOffCenterRH(float ViewLeft, float ViewRight, float ViewBottom, float ViewTop, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixOrthographicLH(float ViewWidth, float ViewHeight, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixOrthographicRH(float ViewWidth, float ViewHeight, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixOrthographicOffCenterLH(float ViewLeft, float ViewRight, float ViewBottom, float ViewTop, float NearZ, float FarZ) noexcept;
    XMMATRIX    XM_CALLCONV     XMMatrixOrthographicOffCenterRH(float ViewLeft, float ViewRight, float ViewBottom, float ViewTop, float NearZ, float FarZ) noexcept;


    /****************************************************************************
     *
     * Quaternion operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMQuaternionEqual(FXMVECTOR Q1, FXMVECTOR Q2) noexcept;
    bool        XM_CALLCONV     XMQuaternionNotEqual(FXMVECTOR Q1, FXMVECTOR Q2) noexcept;

    bool        XM_CALLCONV     XMQuaternionIsNaN(FXMVECTOR Q) noexcept;
    bool        XM_CALLCONV     XMQuaternionIsInfinite(FXMVECTOR Q) noexcept;
    bool        XM_CALLCONV     XMQuaternionIsIdentity(FXMVECTOR Q) noexcept;

    XMVECTOR    XM_CALLCONV     XMQuaternionDot(FXMVECTOR Q1, FXMVECTOR Q2) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionMultiply(FXMVECTOR Q1, FXMVECTOR Q2) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionLengthSq(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionReciprocalLength(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionLength(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionNormalizeEst(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionNormalize(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionConjugate(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionInverse(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionLn(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionExp(FXMVECTOR Q) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionSlerp(FXMVECTOR Q0, FXMVECTOR Q1, float t) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionSlerpV(FXMVECTOR Q0, FXMVECTOR Q1, FXMVECTOR T) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionSquad(FXMVECTOR Q0, FXMVECTOR Q1, FXMVECTOR Q2, GXMVECTOR Q3, float t) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionSquadV(FXMVECTOR Q0, FXMVECTOR Q1, FXMVECTOR Q2, GXMVECTOR Q3, HXMVECTOR T) noexcept;
    void        XM_CALLCONV     XMQuaternionSquadSetup(_Out_ XMVECTOR* pA, _Out_ XMVECTOR* pB, _Out_ XMVECTOR* pC, _In_ FXMVECTOR Q0, _In_ FXMVECTOR Q1, _In_ FXMVECTOR Q2, _In_ GXMVECTOR Q3) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionBaryCentric(FXMVECTOR Q0, FXMVECTOR Q1, FXMVECTOR Q2, float f, float g) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionBaryCentricV(FXMVECTOR Q0, FXMVECTOR Q1, FXMVECTOR Q2, GXMVECTOR F, HXMVECTOR G) noexcept;

    XMVECTOR    XM_CALLCONV     XMQuaternionIdentity() noexcept;

    // Rotates about y-axis (Yaw), then x-axis (Pitch), then z-axis (Roll)
    XMVECTOR    XM_CALLCONV     XMQuaternionRotationRollPitchYaw(float Pitch, float Yaw, float Roll) noexcept;

    // Rotates about y-axis (Angles.y), then x-axis (Angles.x), then z-axis (Angles.z)
    XMVECTOR    XM_CALLCONV     XMQuaternionRotationRollPitchYawFromVector(FXMVECTOR Angles) noexcept;

    XMVECTOR    XM_CALLCONV     XMQuaternionRotationNormal(FXMVECTOR NormalAxis, float Angle) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionRotationAxis(FXMVECTOR Axis, float Angle) noexcept;
    XMVECTOR    XM_CALLCONV     XMQuaternionRotationMatrix(FXMMATRIX M) noexcept;

    void        XM_CALLCONV     XMQuaternionToAxisAngle(_Out_ XMVECTOR* pAxis, _Out_ float* pAngle, _In_ FXMVECTOR Q) noexcept;

    /****************************************************************************
     *
     * Plane operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMPlaneEqual(FXMVECTOR P1, FXMVECTOR P2) noexcept;
    bool        XM_CALLCONV     XMPlaneNearEqual(FXMVECTOR P1, FXMVECTOR P2, FXMVECTOR Epsilon) noexcept;
    bool        XM_CALLCONV     XMPlaneNotEqual(FXMVECTOR P1, FXMVECTOR P2) noexcept;

    bool        XM_CALLCONV     XMPlaneIsNaN(FXMVECTOR P) noexcept;
    bool        XM_CALLCONV     XMPlaneIsInfinite(FXMVECTOR P) noexcept;

    XMVECTOR    XM_CALLCONV     XMPlaneDot(FXMVECTOR P, FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMPlaneDotCoord(FXMVECTOR P, FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMPlaneDotNormal(FXMVECTOR P, FXMVECTOR V) noexcept;
    XMVECTOR    XM_CALLCONV     XMPlaneNormalizeEst(FXMVECTOR P) noexcept;
    XMVECTOR    XM_CALLCONV     XMPlaneNormalize(FXMVECTOR P) noexcept;
    XMVECTOR    XM_CALLCONV     XMPlaneIntersectLine(FXMVECTOR P, FXMVECTOR LinePoint1, FXMVECTOR LinePoint2) noexcept;
    void        XM_CALLCONV     XMPlaneIntersectPlane(_Out_ XMVECTOR* pLinePoint1, _Out_ XMVECTOR* pLinePoint2, _In_ FXMVECTOR P1, _In_ FXMVECTOR P2) noexcept;

    // Transforms a plane given an inverse transpose matrix
    XMVECTOR    XM_CALLCONV     XMPlaneTransform(FXMVECTOR P, FXMMATRIX ITM) noexcept;

    // Transforms an array of planes given an inverse transpose matrix
    XMFLOAT4*   XM_CALLCONV     XMPlaneTransformStream(_Out_writes_bytes_(sizeof(XMFLOAT4) + OutputStride * (PlaneCount - 1)) XMFLOAT4* pOutputStream,
        _In_ size_t OutputStride,
        _In_reads_bytes_(sizeof(XMFLOAT4) + InputStride * (PlaneCount - 1)) const XMFLOAT4* pInputStream,
        _In_ size_t InputStride, _In_ size_t PlaneCount, _In_ FXMMATRIX ITM) noexcept;

    XMVECTOR    XM_CALLCONV     XMPlaneFromPointNormal(FXMVECTOR Point, FXMVECTOR Normal) noexcept;
    XMVECTOR    XM_CALLCONV     XMPlaneFromPoints(FXMVECTOR Point1, FXMVECTOR Point2, FXMVECTOR Point3) noexcept;

    /****************************************************************************
     *
     * Color operations
     *
     ****************************************************************************/

    bool        XM_CALLCONV     XMColorEqual(FXMVECTOR C1, FXMVECTOR C2) noexcept;
    bool        XM_CALLCONV     XMColorNotEqual(FXMVECTOR C1, FXMVECTOR C2) noexcept;
    bool        XM_CALLCONV     XMColorGreater(FXMVECTOR C1, FXMVECTOR C2) noexcept;
    bool        XM_CALLCONV     XMColorGreaterOrEqual(FXMVECTOR C1, FXMVECTOR C2) noexcept;
    bool        XM_CALLCONV     XMColorLess(FXMVECTOR C1, FXMVECTOR C2) noexcept;
    bool        XM_CALLCONV     XMColorLessOrEqual(FXMVECTOR C1, FXMVECTOR C2) noexcept;

    bool        XM_CALLCONV     XMColorIsNaN(FXMVECTOR C) noexcept;
    bool        XM_CALLCONV     XMColorIsInfinite(FXMVECTOR C) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorNegative(FXMVECTOR C) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorModulate(FXMVECTOR C1, FXMVECTOR C2) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorAdjustSaturation(FXMVECTOR C, float Saturation) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorAdjustContrast(FXMVECTOR C, float Contrast) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToHSL(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorHSLToRGB(FXMVECTOR hsl) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToHSV(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorHSVToRGB(FXMVECTOR hsv) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToYUV(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorYUVToRGB(FXMVECTOR yuv) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToYUV_HD(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorYUVToRGB_HD(FXMVECTOR yuv) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToYUV_UHD(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorYUVToRGB_UHD(FXMVECTOR yuv) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToXYZ(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorXYZToRGB(FXMVECTOR xyz) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorXYZToSRGB(FXMVECTOR xyz) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorSRGBToXYZ(FXMVECTOR srgb) noexcept;

    XMVECTOR    XM_CALLCONV     XMColorRGBToSRGB(FXMVECTOR rgb) noexcept;
    XMVECTOR    XM_CALLCONV     XMColorSRGBToRGB(FXMVECTOR srgb) noexcept;


    /****************************************************************************
     *
     * Miscellaneous operations
     *
     ****************************************************************************/

    bool            XMVerifyCPUSupport() noexcept;

    XMVECTOR    XM_CALLCONV     XMFresnelTerm(FXMVECTOR CosIncidentAngle, FXMVECTOR RefractionIndex) noexcept;

    bool            XMScalarNearEqual(float S1, float S2, float Epsilon) noexcept;
    float           XMScalarModAngle(float Value) noexcept;

    float           XMScalarSin(float Value) noexcept;
    float           XMScalarSinEst(float Value) noexcept;

    float           XMScalarCos(float Value) noexcept;
    float           XMScalarCosEst(float Value) noexcept;

    void            XMScalarSinCos(_Out_ float* pSin, _Out_ float* pCos, float Value) noexcept;
    void            XMScalarSinCosEst(_Out_ float* pSin, _Out_ float* pCos, float Value) noexcept;

    float           XMScalarASin(float Value) noexcept;
    float           XMScalarASinEst(float Value) noexcept;

    float           XMScalarACos(float Value) noexcept;
    float           XMScalarACosEst(float Value) noexcept;

    /****************************************************************************
     *
     * Templates
     *
     ****************************************************************************/

#if defined(__XNAMATH_H__) && defined(XMMin)
#undef XMMin
#undef XMMax
#endif

    template<class T> inline T XMMin(T a, T b) noexcept { return (a < b) ? a : b; }
    template<class T> inline T XMMax(T a, T b) noexcept { return (a > b) ? a : b; }

    //------------------------------------------------------------------------------

#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)

// PermuteHelper internal template (SSE only)
    namespace Internal
    {
        // Slow path fallback for permutes that do not map to a single SSE shuffle opcode.
        template<uint32_t Shuffle, bool WhichX, bool WhichY, bool WhichZ, bool WhichW> struct PermuteHelper
        {
            static XMVECTOR     XM_CALLCONV     Permute(FXMVECTOR v1, FXMVECTOR v2) noexcept
            {
                static const XMVECTORU32 selectMask =
                { { {
                        WhichX ? 0xFFFFFFFF : 0,
                        WhichY ? 0xFFFFFFFF : 0,
                        WhichZ ? 0xFFFFFFFF : 0,
                        WhichW ? 0xFFFFFFFF : 0,
                } } };

                XMVECTOR shuffled1 = XM_PERMUTE_PS(v1, Shuffle);
                XMVECTOR shuffled2 = XM_PERMUTE_PS(v2, Shuffle);

                XMVECTOR masked1 = _mm_andnot_ps(selectMask, shuffled1);
                XMVECTOR masked2 = _mm_and_ps(selectMask, shuffled2);

                return _mm_or_ps(masked1, masked2);
            }
        };

        // Fast path for permutes that only read from the first vector.
        template<uint32_t Shuffle> struct PermuteHelper<Shuffle, false, false, false, false>
        {
            static XMVECTOR     XM_CALLCONV     Permute(FXMVECTOR v1, FXMVECTOR) noexcept { return XM_PERMUTE_PS(v1, Shuffle); }
        };

        // Fast path for permutes that only read from the second vector.
        template<uint32_t Shuffle> struct PermuteHelper<Shuffle, true, true, true, true>
        {
            static XMVECTOR     XM_CALLCONV     Permute(FXMVECTOR, FXMVECTOR v2) noexcept { return XM_PERMUTE_PS(v2, Shuffle); }
        };

        // Fast path for permutes that read XY from the first vector, ZW from the second.
        template<uint32_t Shuffle> struct PermuteHelper<Shuffle, false, false, true, true>
        {
            static XMVECTOR     XM_CALLCONV     Permute(FXMVECTOR v1, FXMVECTOR v2) noexcept { return _mm_shuffle_ps(v1, v2, Shuffle); }
        };

        // Fast path for permutes that read XY from the second vector, ZW from the first.
        template<uint32_t Shuffle> struct PermuteHelper<Shuffle, true, true, false, false>
        {
            static XMVECTOR     XM_CALLCONV     Permute(FXMVECTOR v1, FXMVECTOR v2) noexcept { return _mm_shuffle_ps(v2, v1, Shuffle); }
        };
    }

#endif // _XM_SSE_INTRINSICS_ && !_XM_NO_INTRINSICS_

    // General permute template
    template<uint32_t PermuteX, uint32_t PermuteY, uint32_t PermuteZ, uint32_t PermuteW>
    inline XMVECTOR     XM_CALLCONV     XMVectorPermute(FXMVECTOR V1, FXMVECTOR V2) noexcept
    {
        static_assert(PermuteX <= 7, "PermuteX template parameter out of range");
        static_assert(PermuteY <= 7, "PermuteY template parameter out of range");
        static_assert(PermuteZ <= 7, "PermuteZ template parameter out of range");
        static_assert(PermuteW <= 7, "PermuteW template parameter out of range");

#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
        constexpr uint32_t Shuffle = _MM_SHUFFLE(PermuteW & 3, PermuteZ & 3, PermuteY & 3, PermuteX & 3);

        constexpr bool WhichX = PermuteX > 3;
        constexpr bool WhichY = PermuteY > 3;
        constexpr bool WhichZ = PermuteZ > 3;
        constexpr bool WhichW = PermuteW > 3;

        return Internal::PermuteHelper<Shuffle, WhichX, WhichY, WhichZ, WhichW>::Permute(V1, V2);
#else

        return XMVectorPermute(V1, V2, PermuteX, PermuteY, PermuteZ, PermuteW);

#endif
    }

    // Special-case permute templates
    template<> constexpr XMVECTOR XM_CALLCONV     XMVectorPermute<0, 1, 2, 3>(FXMVECTOR V1, FXMVECTOR) noexcept { return V1; }
    template<> constexpr XMVECTOR XM_CALLCONV     XMVectorPermute<4, 5, 6, 7>(FXMVECTOR, FXMVECTOR V2) noexcept { return V2; }

#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 4, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_movelh_ps(V1, V2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<6, 7, 2, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_movehl_ps(V1, V2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 4, 1, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_unpacklo_ps(V1, V2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 6, 3, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_unpackhi_ps(V1, V2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 3, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_castpd_ps(_mm_unpackhi_pd(_mm_castps_pd(V1), _mm_castps_pd(V2))); }
#endif

#if defined(_XM_SSE4_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 1, 2, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x1); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 5, 2, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 5, 2, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x3); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 6, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x4); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 1, 6, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x5); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 5, 6, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x6); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 5, 6, 3>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x7); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 2, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x8); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 1, 2, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0x9); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 5, 2, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0xA); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 5, 2, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0xB); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0xC); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<4, 1, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0xD); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 5, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return _mm_blend_ps(V1, V2, 0xE); }
#endif

#if defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)

    // If the indices are all in the range 0-3 or 4-7, then use XMVectorSwizzle instead
    // The mirror cases are not spelled out here as the programmer can always swap the arguments
    // (i.e. prefer permutes where the X element comes from the V1 vector instead of the V2 vector)

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 4, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_low_f32(V1), vget_low_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 0, 4, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_low_f32(V1)), vget_low_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 5, 4>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_low_f32(V1), vrev64_f32(vget_low_f32(V2))); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 0, 5, 4>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_low_f32(V1)), vrev64_f32(vget_low_f32(V2))); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 3, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_high_f32(V1), vget_high_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<3, 2, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_high_f32(V1)), vget_high_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 3, 7, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_high_f32(V1), vrev64_f32(vget_high_f32(V2))); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<3, 2, 7, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_high_f32(V1)), vrev64_f32(vget_high_f32(V2))); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_low_f32(V1), vget_high_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 0, 6, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_low_f32(V1)), vget_high_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 1, 7, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_low_f32(V1), vrev64_f32(vget_high_f32(V2))); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 0, 7, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_low_f32(V1)), vrev64_f32(vget_high_f32(V2))); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<3, 2, 4, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_high_f32(V1)), vget_low_f32(V2)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 3, 5, 4>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vget_high_f32(V1), vrev64_f32(vget_low_f32(V2))); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<3, 2, 5, 4>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vcombine_f32(vrev64_f32(vget_high_f32(V1)), vrev64_f32(vget_low_f32(V2))); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 4, 2, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vtrnq_f32(V1, V2).val[0]; }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 5, 3, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vtrnq_f32(V1, V2).val[1]; }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 4, 1, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vzipq_f32(V1, V2).val[0]; }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 6, 3, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vzipq_f32(V1, V2).val[1]; }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<0, 2, 4, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vuzpq_f32(V1, V2).val[0]; }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 3, 5, 7>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vuzpq_f32(V1, V2).val[1]; }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<1, 2, 3, 4>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vextq_f32(V1, V2, 1); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<2, 3, 4, 5>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vextq_f32(V1, V2, 2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorPermute<3, 4, 5, 6>(FXMVECTOR V1, FXMVECTOR V2) noexcept { return vextq_f32(V1, V2, 3); }

#endif // _XM_ARM_NEON_INTRINSICS_ && !_XM_NO_INTRINSICS_

    //------------------------------------------------------------------------------

    // General swizzle template
    template<uint32_t SwizzleX, uint32_t SwizzleY, uint32_t SwizzleZ, uint32_t SwizzleW>
    inline XMVECTOR     XM_CALLCONV     XMVectorSwizzle(FXMVECTOR V) noexcept
    {
        static_assert(SwizzleX <= 3, "SwizzleX template parameter out of range");
        static_assert(SwizzleY <= 3, "SwizzleY template parameter out of range");
        static_assert(SwizzleZ <= 3, "SwizzleZ template parameter out of range");
        static_assert(SwizzleW <= 3, "SwizzleW template parameter out of range");

#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
        return XM_PERMUTE_PS(V, _MM_SHUFFLE(SwizzleW, SwizzleZ, SwizzleY, SwizzleX));
#else

        return XMVectorSwizzle(V, SwizzleX, SwizzleY, SwizzleZ, SwizzleW);

#endif
    }

    // Specialized swizzles
    template<> constexpr XMVECTOR XM_CALLCONV XMVectorSwizzle<0, 1, 2, 3>(FXMVECTOR V) noexcept { return V; }

#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 1, 0, 1>(FXMVECTOR V) noexcept { return _mm_movelh_ps(V, V); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 3, 2, 3>(FXMVECTOR V) noexcept { return _mm_movehl_ps(V, V); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 0, 1, 1>(FXMVECTOR V) noexcept { return _mm_unpacklo_ps(V, V); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 2, 3, 3>(FXMVECTOR V) noexcept { return _mm_unpackhi_ps(V, V); }
#endif

#if defined(_XM_SSE3_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 0, 2, 2>(FXMVECTOR V) noexcept { return _mm_moveldup_ps(V); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 1, 3, 3>(FXMVECTOR V) noexcept { return _mm_movehdup_ps(V); }
#endif

#if defined(_XM_AVX2_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_) && defined(_XM_FAVOR_INTEL_)
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 0, 0, 0>(FXMVECTOR V) noexcept { return _mm_broadcastss_ps(V); }
#endif

#if defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 0, 0, 0>(FXMVECTOR V) noexcept { return vdupq_lane_f32(vget_low_f32(V), 0); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 1, 1, 1>(FXMVECTOR V) noexcept { return vdupq_lane_f32(vget_low_f32(V), 1); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 2, 2, 2>(FXMVECTOR V) noexcept { return vdupq_lane_f32(vget_high_f32(V), 0); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<3, 3, 3, 3>(FXMVECTOR V) noexcept { return vdupq_lane_f32(vget_high_f32(V), 1); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 0, 3, 2>(FXMVECTOR V) noexcept { return vrev64q_f32(V); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 1, 0, 1>(FXMVECTOR V) noexcept { float32x2_t vt = vget_low_f32(V); return vcombine_f32(vt, vt); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 3, 2, 3>(FXMVECTOR V) noexcept { float32x2_t vt = vget_high_f32(V); return vcombine_f32(vt, vt); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 0, 1, 0>(FXMVECTOR V) noexcept { float32x2_t vt = vrev64_f32(vget_low_f32(V)); return vcombine_f32(vt, vt); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<3, 2, 3, 2>(FXMVECTOR V) noexcept { float32x2_t vt = vrev64_f32(vget_high_f32(V)); return vcombine_f32(vt, vt); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 1, 3, 2>(FXMVECTOR V) noexcept { return vcombine_f32(vget_low_f32(V), vrev64_f32(vget_high_f32(V))); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 0, 2, 3>(FXMVECTOR V) noexcept { return vcombine_f32(vrev64_f32(vget_low_f32(V)), vget_high_f32(V)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 3, 1, 0>(FXMVECTOR V) noexcept { return vcombine_f32(vget_high_f32(V), vrev64_f32(vget_low_f32(V))); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<3, 2, 0, 1>(FXMVECTOR V) noexcept { return vcombine_f32(vrev64_f32(vget_high_f32(V)), vget_low_f32(V)); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<3, 2, 1, 0>(FXMVECTOR V) noexcept { return vcombine_f32(vrev64_f32(vget_high_f32(V)), vrev64_f32(vget_low_f32(V))); }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 0, 2, 2>(FXMVECTOR V) noexcept { return vtrnq_f32(V, V).val[0]; }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 1, 3, 3>(FXMVECTOR V) noexcept { return vtrnq_f32(V, V).val[1]; }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 0, 1, 1>(FXMVECTOR V) noexcept { return vzipq_f32(V, V).val[0]; }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 2, 3, 3>(FXMVECTOR V) noexcept { return vzipq_f32(V, V).val[1]; }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<0, 2, 0, 2>(FXMVECTOR V) noexcept { return vuzpq_f32(V, V).val[0]; }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 3, 1, 3>(FXMVECTOR V) noexcept { return vuzpq_f32(V, V).val[1]; }

    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<1, 2, 3, 0>(FXMVECTOR V) noexcept { return vextq_f32(V, V, 1); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<2, 3, 0, 1>(FXMVECTOR V) noexcept { return vextq_f32(V, V, 2); }
    template<> inline XMVECTOR      XM_CALLCONV     XMVectorSwizzle<3, 0, 1, 2>(FXMVECTOR V) noexcept { return vextq_f32(V, V, 3); }

#endif // _XM_ARM_NEON_INTRINSICS_ && !_XM_NO_INTRINSICS_

    //------------------------------------------------------------------------------

    template<uint32_t Elements>
    inline XMVECTOR     XM_CALLCONV     XMVectorShiftLeft(FXMVECTOR V1, FXMVECTOR V2) noexcept
    {
        static_assert(Elements < 4, "Elements template parameter out of range");
        return XMVectorPermute<Elements, (Elements + 1), (Elements + 2), (Elements + 3)>(V1, V2);
    }

    template<uint32_t Elements>
    inline XMVECTOR     XM_CALLCONV     XMVectorRotateLeft(FXMVECTOR V) noexcept
    {
        static_assert(Elements < 4, "Elements template parameter out of range");
        return XMVectorSwizzle<Elements & 3, (Elements + 1) & 3, (Elements + 2) & 3, (Elements + 3) & 3>(V);
    }

    template<uint32_t Elements>
    inline XMVECTOR     XM_CALLCONV     XMVectorRotateRight(FXMVECTOR V) noexcept
    {
        static_assert(Elements < 4, "Elements template parameter out of range");
        return XMVectorSwizzle<(4 - Elements) & 3, (5 - Elements) & 3, (6 - Elements) & 3, (7 - Elements) & 3>(V);
    }

    template<uint32_t VSLeftRotateElements, uint32_t Select0, uint32_t Select1, uint32_t Select2, uint32_t Select3>
    inline XMVECTOR     XM_CALLCONV     XMVectorInsert(FXMVECTOR VD, FXMVECTOR VS) noexcept
    {
        XMVECTOR Control = XMVectorSelectControl(Select0 & 1, Select1 & 1, Select2 & 1, Select3 & 1);
        return XMVectorSelect(VD, XMVectorRotateLeft<VSLeftRotateElements>(VS), Control);
    }

    /****************************************************************************
     *
     * Globals
     *
     ****************************************************************************/

     // The purpose of the following global constants is to prevent redundant
     // reloading of the constants when they are referenced by more than one
     // separate inline math routine called within the same function.  Declaring
     // a constant locally within a routine is sufficient to prevent redundant
     // reloads of that constant when that single routine is called multiple
     // times in a function, but if the constant is used (and declared) in a
     // separate math routine it would be reloaded.

#ifndef XMGLOBALCONST
#if defined(__GNUC__) && !defined(__MINGW32__)
#define XMGLOBALCONST extern const __attribute__((weak))
#else
#define XMGLOBALCONST extern const __declspec(selectany)
#endif
#endif

    XMGLOBALCONST XMVECTORF32 g_XMSinCoefficients0 = { { { -0.16666667f, +0.0083333310f, -0.00019840874f, +2.7525562e-06f } } };
    XMGLOBALCONST XMVECTORF32 g_XMSinCoefficients1 = { { { -2.3889859e-08f, -0.16665852f /*Est1*/, +0.0083139502f /*Est2*/, -0.00018524670f /*Est3*/ } } };
    XMGLOBALCONST XMVECTORF32 g_XMCosCoefficients0 = { { { -0.5f, +0.041666638f, -0.0013888378f, +2.4760495e-05f } } };
    XMGLOBALCONST XMVECTORF32 g_XMCosCoefficients1 = { { { -2.6051615e-07f, -0.49992746f /*Est1*/, +0.041493919f /*Est2*/, -0.0012712436f /*Est3*/ } } };
    XMGLOBALCONST XMVECTORF32 g_XMTanCoefficients0 = { { { 1.0f, 0.333333333f, 0.133333333f, 5.396825397e-2f } } };
    XMGLOBALCONST XMVECTORF32 g_XMTanCoefficients1 = { { { 2.186948854e-2f, 8.863235530e-3f, 3.592128167e-3f, 1.455834485e-3f } } };
    XMGLOBALCONST XMVECTORF32 g_XMTanCoefficients2 = { { { 5.900274264e-4f, 2.391290764e-4f, 9.691537707e-5f, 3.927832950e-5f } } };
    XMGLOBALCONST XMVECTORF32 g_XMArcCoefficients0 = { { { +1.5707963050f, -0.2145988016f, +0.0889789874f, -0.0501743046f } } };
    XMGLOBALCONST XMVECTORF32 g_XMArcCoefficients1 = { { { +0.0308918810f, -0.0170881256f, +0.0066700901f, -0.0012624911f } } };
    XMGLOBALCONST XMVECTORF32 g_XMATanCoefficients0 = { { { -0.3333314528f, +0.1999355085f, -0.1420889944f, +0.1065626393f } } };
    XMGLOBALCONST XMVECTORF32 g_XMATanCoefficients1 = { { { -0.0752896400f, +0.0429096138f, -0.0161657367f, +0.0028662257f } } };
    XMGLOBALCONST XMVECTORF32 g_XMATanEstCoefficients0 = { { { +0.999866f, +0.999866f, +0.999866f, +0.999866f } } };
    XMGLOBALCONST XMVECTORF32 g_XMATanEstCoefficients1 = { { { -0.3302995f, +0.180141f, -0.085133f, +0.0208351f } } };
    XMGLOBALCONST XMVECTORF32 g_XMTanEstCoefficients = { { { 2.484f, -1.954923183e-1f, 2.467401101f, XM_1DIVPI } } };
    XMGLOBALCONST XMVECTORF32 g_XMArcEstCoefficients = { { { +1.5707288f, -0.2121144f, +0.0742610f, -0.0187293f } } };
    XMGLOBALCONST XMVECTORF32 g_XMPiConstants0 = { { { XM_PI, XM_2PI, XM_1DIVPI, XM_1DIV2PI } } };
    XMGLOBALCONST XMVECTORF32 g_XMIdentityR0 = { { { 1.0f, 0.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMIdentityR1 = { { { 0.0f, 1.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMIdentityR2 = { { { 0.0f, 0.0f, 1.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMIdentityR3 = { { { 0.0f, 0.0f, 0.0f, 1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegIdentityR0 = { { { -1.0f, 0.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegIdentityR1 = { { { 0.0f, -1.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegIdentityR2 = { { { 0.0f, 0.0f, -1.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegIdentityR3 = { { { 0.0f, 0.0f, 0.0f, -1.0f } } };
    XMGLOBALCONST XMVECTORU32 g_XMNegativeZero = { { { 0x80000000, 0x80000000, 0x80000000, 0x80000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMNegate3 = { { { 0x80000000, 0x80000000, 0x80000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskXY = { { { 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMMask3 = { { { 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskX = { { { 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskY = { { { 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskZ = { { { 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskW = { { { 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF } } };
    XMGLOBALCONST XMVECTORF32 g_XMOne = { { { 1.0f, 1.0f, 1.0f, 1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMOne3 = { { { 1.0f, 1.0f, 1.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMZero = { { { 0.0f, 0.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMTwo = { { { 2.f, 2.f, 2.f, 2.f } } };
    XMGLOBALCONST XMVECTORF32 g_XMFour = { { { 4.f, 4.f, 4.f, 4.f } } };
    XMGLOBALCONST XMVECTORF32 g_XMSix = { { { 6.f, 6.f, 6.f, 6.f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegativeOne = { { { -1.0f, -1.0f, -1.0f, -1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMOneHalf = { { { 0.5f, 0.5f, 0.5f, 0.5f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegativeOneHalf = { { { -0.5f, -0.5f, -0.5f, -0.5f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegativeTwoPi = { { { -XM_2PI, -XM_2PI, -XM_2PI, -XM_2PI } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegativePi = { { { -XM_PI, -XM_PI, -XM_PI, -XM_PI } } };
    XMGLOBALCONST XMVECTORF32 g_XMHalfPi = { { { XM_PIDIV2, XM_PIDIV2, XM_PIDIV2, XM_PIDIV2 } } };
    XMGLOBALCONST XMVECTORF32 g_XMPi = { { { XM_PI, XM_PI, XM_PI, XM_PI } } };
    XMGLOBALCONST XMVECTORF32 g_XMReciprocalPi = { { { XM_1DIVPI, XM_1DIVPI, XM_1DIVPI, XM_1DIVPI } } };
    XMGLOBALCONST XMVECTORF32 g_XMTwoPi = { { { XM_2PI, XM_2PI, XM_2PI, XM_2PI } } };
    XMGLOBALCONST XMVECTORF32 g_XMReciprocalTwoPi = { { { XM_1DIV2PI, XM_1DIV2PI, XM_1DIV2PI, XM_1DIV2PI } } };
    XMGLOBALCONST XMVECTORF32 g_XMEpsilon = { { { 1.192092896e-7f, 1.192092896e-7f, 1.192092896e-7f, 1.192092896e-7f } } };
    XMGLOBALCONST XMVECTORI32 g_XMInfinity = { { { 0x7F800000, 0x7F800000, 0x7F800000, 0x7F800000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMQNaN = { { { 0x7FC00000, 0x7FC00000, 0x7FC00000, 0x7FC00000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMQNaNTest = { { { 0x007FFFFF, 0x007FFFFF, 0x007FFFFF, 0x007FFFFF } } };
    XMGLOBALCONST XMVECTORI32 g_XMAbsMask = { { { 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF } } };
    XMGLOBALCONST XMVECTORI32 g_XMFltMin = { { { 0x00800000, 0x00800000, 0x00800000, 0x00800000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMFltMax = { { { 0x7F7FFFFF, 0x7F7FFFFF, 0x7F7FFFFF, 0x7F7FFFFF } } };
    XMGLOBALCONST XMVECTORU32 g_XMNegOneMask = { { { 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskA8R8G8B8 = { { { 0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipA8R8G8B8 = { { { 0x00000000, 0x00000000, 0x00000000, 0x80000000 } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixAA8R8G8B8 = { { { 0.0f, 0.0f, 0.0f, float(0x80000000U) } } };
    XMGLOBALCONST XMVECTORF32 g_XMNormalizeA8R8G8B8 = { { { 1.0f / (255.0f * float(0x10000)), 1.0f / (255.0f * float(0x100)), 1.0f / 255.0f, 1.0f / (255.0f * float(0x1000000)) } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskA2B10G10R10 = { { { 0x000003FF, 0x000FFC00, 0x3FF00000, 0xC0000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipA2B10G10R10 = { { { 0x00000200, 0x00080000, 0x20000000, 0x80000000 } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixAA2B10G10R10 = { { { -512.0f, -512.0f * float(0x400), -512.0f * float(0x100000), float(0x80000000U) } } };
    XMGLOBALCONST XMVECTORF32 g_XMNormalizeA2B10G10R10 = { { { 1.0f / 511.0f, 1.0f / (511.0f * float(0x400)), 1.0f / (511.0f * float(0x100000)), 1.0f / (3.0f * float(0x40000000)) } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskX16Y16 = { { { 0x0000FFFF, 0xFFFF0000, 0x00000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMFlipX16Y16 = { { { 0x00008000, 0x00000000, 0x00000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixX16Y16 = { { { -32768.0f, 0.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNormalizeX16Y16 = { { { 1.0f / 32767.0f, 1.0f / (32767.0f * 65536.0f), 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskX16Y16Z16W16 = { { { 0x0000FFFF, 0x0000FFFF, 0xFFFF0000, 0xFFFF0000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMFlipX16Y16Z16W16 = { { { 0x00008000, 0x00008000, 0x00000000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixX16Y16Z16W16 = { { { -32768.0f, -32768.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNormalizeX16Y16Z16W16 = { { { 1.0f / 32767.0f, 1.0f / 32767.0f, 1.0f / (32767.0f * 65536.0f), 1.0f / (32767.0f * 65536.0f) } } };
    XMGLOBALCONST XMVECTORF32 g_XMNoFraction = { { { 8388608.0f, 8388608.0f, 8388608.0f, 8388608.0f } } };
    XMGLOBALCONST XMVECTORI32 g_XMMaskByte = { { { 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegateX = { { { -1.0f, 1.0f, 1.0f, 1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegateY = { { { 1.0f, -1.0f, 1.0f, 1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegateZ = { { { 1.0f, 1.0f, -1.0f, 1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMNegateW = { { { 1.0f, 1.0f, 1.0f, -1.0f } } };
    XMGLOBALCONST XMVECTORU32 g_XMSelect0101 = { { { XM_SELECT_0, XM_SELECT_1, XM_SELECT_0, XM_SELECT_1 } } };
    XMGLOBALCONST XMVECTORU32 g_XMSelect1010 = { { { XM_SELECT_1, XM_SELECT_0, XM_SELECT_1, XM_SELECT_0 } } };
    XMGLOBALCONST XMVECTORI32 g_XMOneHalfMinusEpsilon = { { { 0x3EFFFFFD, 0x3EFFFFFD, 0x3EFFFFFD, 0x3EFFFFFD } } };
    XMGLOBALCONST XMVECTORU32 g_XMSelect1000 = { { { XM_SELECT_1, XM_SELECT_0, XM_SELECT_0, XM_SELECT_0 } } };
    XMGLOBALCONST XMVECTORU32 g_XMSelect1100 = { { { XM_SELECT_1, XM_SELECT_1, XM_SELECT_0, XM_SELECT_0 } } };
    XMGLOBALCONST XMVECTORU32 g_XMSelect1110 = { { { XM_SELECT_1, XM_SELECT_1, XM_SELECT_1, XM_SELECT_0 } } };
    XMGLOBALCONST XMVECTORU32 g_XMSelect1011 = { { { XM_SELECT_1, XM_SELECT_0, XM_SELECT_1, XM_SELECT_1 } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixupY16 = { { { 1.0f, 1.0f / 65536.0f, 0.0f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixupY16W16 = { { { 1.0f, 1.0f, 1.0f / 65536.0f, 1.0f / 65536.0f } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipY = { { { 0, 0x80000000, 0, 0 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipZ = { { { 0, 0, 0x80000000, 0 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipW = { { { 0, 0, 0, 0x80000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipYZ = { { { 0, 0x80000000, 0x80000000, 0 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipZW = { { { 0, 0, 0x80000000, 0x80000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMFlipYW = { { { 0, 0x80000000, 0, 0x80000000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMMaskDec4 = { { { 0x3FF, 0x3FF << 10, 0x3FF << 20, static_cast<int>(0xC0000000) } } };
    XMGLOBALCONST XMVECTORI32 g_XMXorDec4 = { { { 0x200, 0x200 << 10, 0x200 << 20, 0 } } };
    XMGLOBALCONST XMVECTORF32 g_XMAddUDec4 = { { { 0, 0, 0, 32768.0f * 65536.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMAddDec4 = { { { -512.0f, -512.0f * 1024.0f, -512.0f * 1024.0f * 1024.0f, 0 } } };
    XMGLOBALCONST XMVECTORF32 g_XMMulDec4 = { { { 1.0f, 1.0f / 1024.0f, 1.0f / (1024.0f * 1024.0f), 1.0f / (1024.0f * 1024.0f * 1024.0f) } } };
    XMGLOBALCONST XMVECTORU32 g_XMMaskByte4 = { { { 0xFF, 0xFF00, 0xFF0000, 0xFF000000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMXorByte4 = { { { 0x80, 0x8000, 0x800000, 0x00000000 } } };
    XMGLOBALCONST XMVECTORF32 g_XMAddByte4 = { { { -128.0f, -128.0f * 256.0f, -128.0f * 65536.0f, 0 } } };
    XMGLOBALCONST XMVECTORF32 g_XMFixUnsigned = { { { 32768.0f * 65536.0f, 32768.0f * 65536.0f, 32768.0f * 65536.0f, 32768.0f * 65536.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMMaxInt = { { { 65536.0f * 32768.0f - 128.0f, 65536.0f * 32768.0f - 128.0f, 65536.0f * 32768.0f - 128.0f, 65536.0f * 32768.0f - 128.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMMaxUInt = { { { 65536.0f * 65536.0f - 256.0f, 65536.0f * 65536.0f - 256.0f, 65536.0f * 65536.0f - 256.0f, 65536.0f * 65536.0f - 256.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMUnsignedFix = { { { 32768.0f * 65536.0f, 32768.0f * 65536.0f, 32768.0f * 65536.0f, 32768.0f * 65536.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMsrgbScale = { { { 12.92f, 12.92f, 12.92f, 1.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMsrgbA = { { { 0.055f, 0.055f, 0.055f, 0.0f } } };
    XMGLOBALCONST XMVECTORF32 g_XMsrgbA1 = { { { 1.055f, 1.055f, 1.055f, 1.0f } } };
    XMGLOBALCONST XMVECTORI32 g_XMExponentBias = { { { 127, 127, 127, 127 } } };
    XMGLOBALCONST XMVECTORI32 g_XMSubnormalExponent = { { { -126, -126, -126, -126 } } };
    XMGLOBALCONST XMVECTORI32 g_XMNumTrailing = { { { 23, 23, 23, 23 } } };
    XMGLOBALCONST XMVECTORI32 g_XMMinNormal = { { { 0x00800000, 0x00800000, 0x00800000, 0x00800000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMNegInfinity = { { { 0xFF800000, 0xFF800000, 0xFF800000, 0xFF800000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMNegQNaN = { { { 0xFFC00000, 0xFFC00000, 0xFFC00000, 0xFFC00000 } } };
    XMGLOBALCONST XMVECTORI32 g_XMBin128 = { { { 0x43000000, 0x43000000, 0x43000000, 0x43000000 } } };
    XMGLOBALCONST XMVECTORU32 g_XMBinNeg150 = { { { 0xC3160000, 0xC3160000, 0xC3160000, 0xC3160000 } } };
    XMGLOBALCONST XMVECTORI32 g_XM253 = { { { 253, 253, 253, 253 } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst1 = { { { -6.93147182e-1f, -6.93147182e-1f, -6.93147182e-1f, -6.93147182e-1f } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst2 = { { { +2.40226462e-1f, +2.40226462e-1f, +2.40226462e-1f, +2.40226462e-1f } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst3 = { { { -5.55036440e-2f, -5.55036440e-2f, -5.55036440e-2f, -5.55036440e-2f } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst4 = { { { +9.61597636e-3f, +9.61597636e-3f, +9.61597636e-3f, +9.61597636e-3f } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst5 = { { { -1.32823968e-3f, -1.32823968e-3f, -1.32823968e-3f, -1.32823968e-3f } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst6 = { { { +1.47491097e-4f, +1.47491097e-4f, +1.47491097e-4f, +1.47491097e-4f } } };
    XMGLOBALCONST XMVECTORF32 g_XMExpEst7 = { { { -1.08635004e-5f, -1.08635004e-5f, -1.08635004e-5f, -1.08635004e-5f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst0 = { { { +1.442693f, +1.442693f, +1.442693f, +1.442693f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst1 = { { { -0.721242f, -0.721242f, -0.721242f, -0.721242f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst2 = { { { +0.479384f, +0.479384f, +0.479384f, +0.479384f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst3 = { { { -0.350295f, -0.350295f, -0.350295f, -0.350295f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst4 = { { { +0.248590f, +0.248590f, +0.248590f, +0.248590f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst5 = { { { -0.145700f, -0.145700f, -0.145700f, -0.145700f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst6 = { { { +0.057148f, +0.057148f, +0.057148f, +0.057148f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLogEst7 = { { { -0.010578f, -0.010578f, -0.010578f, -0.010578f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLgE = { { { +1.442695f, +1.442695f, +1.442695f, +1.442695f } } };
    XMGLOBALCONST XMVECTORF32 g_XMInvLgE = { { { +6.93147182e-1f, +6.93147182e-1f, +6.93147182e-1f, +6.93147182e-1f } } };
    XMGLOBALCONST XMVECTORF32 g_XMLg10 = { { { +3.321928f, +3.321928f, +3.321928f, +3.321928f } } };
    XMGLOBALCONST XMVECTORF32 g_XMInvLg10 = { { { +3.010299956e-1f, +3.010299956e-1f, +3.010299956e-1f, +3.010299956e-1f } } };
    XMGLOBALCONST XMVECTORF32 g_UByteMax = { { { 255.0f, 255.0f, 255.0f, 255.0f } } };
    XMGLOBALCONST XMVECTORF32 g_ByteMin = { { { -127.0f, -127.0f, -127.0f, -127.0f } } };
    XMGLOBALCONST XMVECTORF32 g_ByteMax = { { { 127.0f, 127.0f, 127.0f, 127.0f } } };
    XMGLOBALCONST XMVECTORF32 g_ShortMin = { { { -32767.0f, -32767.0f, -32767.0f, -32767.0f } } };
    XMGLOBALCONST XMVECTORF32 g_ShortMax = { { { 32767.0f, 32767.0f, 32767.0f, 32767.0f } } };
    XMGLOBALCONST XMVECTORF32 g_UShortMax = { { { 65535.0f, 65535.0f, 65535.0f, 65535.0f } } };

    /****************************************************************************
     *
     * Implementation
     *
     ****************************************************************************/

#ifdef _MSC_VER    
#pragma warning(push)
#pragma warning(disable:4068 4214 4204 4365 4616 4640 6001 6101)
     // C4068/4616: ignore unknown pragmas
     // C4214/4204: nonstandard extension used
     // C4365/4640: Off by default noise
     // C6001/6101: False positives
#endif
    
#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 25000, "FXMVECTOR is 16 bytes")
#pragma prefast(disable : 26495, "Union initialization confuses /analyze")
#endif

#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wfloat-equal"
#pragma clang diagnostic ignored "-Wundefined-reinterpret-cast"
#pragma clang diagnostic ignored "-Wunknown-warning-option"
#pragma clang diagnostic ignored "-Wunsafe-buffer-usage"
#endif

//------------------------------------------------------------------------------

    inline XMVECTOR XM_CALLCONV XMVectorSetBinaryConstant(uint32_t C0, uint32_t C1, uint32_t C2, uint32_t C3) noexcept
    {
#if defined(_XM_NO_INTRINSICS_)
        XMVECTORU32 vResult;
        vResult.u[0] = (0 - (C0 & 1)) & 0x3F800000;
        vResult.u[1] = (0 - (C1 & 1)) & 0x3F800000;
        vResult.u[2] = (0 - (C2 & 1)) & 0x3F800000;
        vResult.u[3] = (0 - (C3 & 1)) & 0x3F800000;
        return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
        XMVECTORU32 vResult;
        vResult.u[0] = (0 - (C0 & 1)) & 0x3F800000;
        vResult.u[1] = (0 - (C1 & 1)) & 0x3F800000;
        vResult.u[2] = (0 - (C2 & 1)) & 0x3F800000;
        vResult.u[3] = (0 - (C3 & 1)) & 0x3F800000;
        return vResult.v;
#else // XM_SSE_INTRINSICS_
        static const XMVECTORU32 g_vMask1 = { { { 1, 1, 1, 1 } } };
        // Move the parms to a vector
        __m128i vTemp = _mm_set_epi32(static_cast<int>(C3), static_cast<int>(C2), static_cast<int>(C1), static_cast<int>(C0));
        // Mask off the low bits
        vTemp = _mm_and_si128(vTemp, g_vMask1);
        // 0xFFFFFFFF on true bits
        vTemp = _mm_cmpeq_epi32(vTemp, g_vMask1);
        // 0xFFFFFFFF -> 1.0f, 0x00000000 -> 0.0f
        vTemp = _mm_and_si128(vTemp, g_XMOne);
        return _mm_castsi128_ps(vTemp);
#endif
    }

    //------------------------------------------------------------------------------

    inline XMVECTOR XM_CALLCONV XMVectorSplatConstant(int32_t IntConstant, uint32_t DivExponent) noexcept
    {
        assert(IntConstant >= -16 && IntConstant <= 15);
        assert(DivExponent < 32);
#if defined(_XM_NO_INTRINSICS_)

        using DirectX::XMConvertVectorIntToFloat;

        XMVECTORI32 V = { { { IntConstant, IntConstant, IntConstant, IntConstant } } };
        return XMConvertVectorIntToFloat(V.v, DivExponent);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
        // Splat the int
        int32x4_t vScale = vdupq_n_s32(IntConstant);
        // Convert to a float
        XMVECTOR vResult = vcvtq_f32_s32(vScale);
        // Convert DivExponent into 1.0f/(1<<DivExponent)
        uint32_t uScale = 0x3F800000U - (DivExponent << 23);
        // Splat the scalar value (It's really a float)
        vScale = vreinterpretq_s32_u32(vdupq_n_u32(uScale));
        // Multiply by the reciprocal (Perform a right shift by DivExponent)
        vResult = vmulq_f32(vResult, reinterpret_cast<const float32x4_t*>(&vScale)[0]);
        return vResult;
#else // XM_SSE_INTRINSICS_
        // Splat the int
        __m128i vScale = _mm_set1_epi32(IntConstant);
        // Convert to a float
        XMVECTOR vResult = _mm_cvtepi32_ps(vScale);
        // Convert DivExponent into 1.0f/(1<<DivExponent)
        uint32_t uScale = 0x3F800000U - (DivExponent << 23);
        // Splat the scalar value (It's really a float)
        vScale = _mm_set1_epi32(static_cast<int>(uScale));
        // Multiply by the reciprocal (Perform a right shift by DivExponent)
        vResult = _mm_mul_ps(vResult, _mm_castsi128_ps(vScale));
        return vResult;
#endif
    }

    //------------------------------------------------------------------------------

    inline XMVECTOR XM_CALLCONV XMVectorSplatConstantInt(int32_t IntConstant) noexcept
    {
        assert(IntConstant >= -16 && IntConstant <= 15);
#if defined(_XM_NO_INTRINSICS_)

        XMVECTORI32 V = { { { IntConstant, IntConstant, IntConstant, IntConstant } } };
        return V.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
        int32x4_t V = vdupq_n_s32(IntConstant);
        return reinterpret_cast<float32x4_t*>(&V)[0];
#else // XM_SSE_INTRINSICS_
        __m128i V = _mm_set1_epi32(IntConstant);
        return _mm_castsi128_ps(V);
#endif
    }

#include "DirectXMathConvert.inl"
#include "DirectXMathVector.inl"
#include "DirectXMathMatrix.inl"
#include "DirectXMathMisc.inl"

#ifdef __clang__
#pragma clang diagnostic pop
#endif
#ifdef _PREFAST_
#pragma prefast(pop)
#endif
#ifdef _MSC_VER    
#pragma warning(pop)
#endif

} // namespace DirectX

