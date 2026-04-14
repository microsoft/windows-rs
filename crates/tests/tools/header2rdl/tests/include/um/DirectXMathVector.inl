//-------------------------------------------------------------------------------------
// DirectXMathVector.inl -- SIMD C++ Math library
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
//
// http://go.microsoft.com/fwlink/?LinkID=615560
//-------------------------------------------------------------------------------------

#pragma once

#if defined(_XM_NO_INTRINSICS_)
#define XMISNAN(x)  ((*(const uint32_t*)&(x) & 0x7F800000) == 0x7F800000 && (*(const uint32_t*)&(x) & 0x7FFFFF) != 0)
#define XMISINF(x)  ((*(const uint32_t*)&(x) & 0x7FFFFFFF) == 0x7F800000)
#endif

#if defined(_XM_SSE_INTRINSICS_)

#define XM3UNPACK3INTO4(l1, l2, l3) \
    XMVECTOR V3 = _mm_shuffle_ps(l2, l3, _MM_SHUFFLE(0, 0, 3, 2));\
    XMVECTOR V2 = _mm_shuffle_ps(l2, l1, _MM_SHUFFLE(3, 3, 1, 0));\
    V2 = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 0, 2));\
    XMVECTOR V4 = _mm_castsi128_ps(_mm_srli_si128(_mm_castps_si128(L3), 32 / 8))

#define XM3PACK4INTO3(v2x) \
    v2x = _mm_shuffle_ps(V2, V3, _MM_SHUFFLE(1, 0, 2, 1));\
    V2 = _mm_shuffle_ps(V2, V1, _MM_SHUFFLE(2, 2, 0, 0));\
    V1 = _mm_shuffle_ps(V1, V2, _MM_SHUFFLE(0, 2, 1, 0));\
    V3 = _mm_shuffle_ps(V3, V4, _MM_SHUFFLE(0, 0, 2, 2));\
    V3 = _mm_shuffle_ps(V3, V4, _MM_SHUFFLE(2, 1, 2, 0))

#endif

/****************************************************************************
 *
 * General Vector
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Assignment operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------
 // Return a vector with all elements equaling zero
inline XMVECTOR XM_CALLCONV XMVectorZero() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { { 0.0f, 0.0f, 0.0f, 0.0f } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_n_f32(0);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_setzero_ps();
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with four floating point values
inline XMVECTOR XM_CALLCONV XMVectorSet
(
    float x,
    float y,
    float z,
    float w
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { { x, y, z, w } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t V0 = vcreate_f32(
        static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&x))
        | (static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&y)) << 32));
    float32x2_t V1 = vcreate_f32(
        static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&z))
        | (static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&w)) << 32));
    return vcombine_f32(V0, V1);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_set_ps(w, z, y, x);
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with four integer values
inline XMVECTOR XM_CALLCONV XMVectorSetInt
(
    uint32_t x,
    uint32_t y,
    uint32_t z,
    uint32_t w
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult = { { { x, y, z, w } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t V0 = vcreate_u32(static_cast<uint64_t>(x) | (static_cast<uint64_t>(y) << 32));
    uint32x2_t V1 = vcreate_u32(static_cast<uint64_t>(z) | (static_cast<uint64_t>(w) << 32));
    return vreinterpretq_f32_u32(vcombine_u32(V0, V1));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_set_epi32(static_cast<int>(w), static_cast<int>(z), static_cast<int>(y), static_cast<int>(x));
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with a replicated floating point value
inline XMVECTOR XM_CALLCONV XMVectorReplicate(float Value) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = Value;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_n_f32(Value);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_set_ps1(Value);
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with a replicated floating point value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorReplicatePtr(const float* pValue) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float Value = pValue[0];
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = Value;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vld1q_dup_f32(pValue);
#elif defined(_XM_AVX_INTRINSICS_)
    return _mm_broadcast_ss(pValue);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_load_ps1(pValue);
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with a replicated integer value
inline XMVECTOR XM_CALLCONV XMVectorReplicateInt(uint32_t Value) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult;
    vResult.u[0] =
        vResult.u[1] =
        vResult.u[2] =
        vResult.u[3] = Value;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vdupq_n_u32(Value));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_set1_epi32(static_cast<int>(Value));
    return _mm_castsi128_ps(vTemp);
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with a replicated integer value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorReplicateIntPtr(const uint32_t* pValue) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t Value = pValue[0];
    XMVECTORU32 vResult;
    vResult.u[0] =
        vResult.u[1] =
        vResult.u[2] =
        vResult.u[3] = Value;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vld1q_dup_u32(pValue));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_load_ps1(reinterpret_cast<const float*>(pValue));
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with all bits set (true mask)
inline XMVECTOR XM_CALLCONV XMVectorTrueInt() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult = { { { 0xFFFFFFFFU, 0xFFFFFFFFU, 0xFFFFFFFFU, 0xFFFFFFFFU } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_s32(vdupq_n_s32(-1));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_set1_epi32(-1);
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------
// Initialize a vector with all bits clear (false mask)
inline XMVECTOR XM_CALLCONV XMVectorFalseInt() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { { 0.0f, 0.0f, 0.0f, 0.0f } } };
    return vResult;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vdupq_n_u32(0));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_setzero_ps();
#endif
}

//------------------------------------------------------------------------------
// Replicate the x component of the vector
inline XMVECTOR XM_CALLCONV XMVectorSplatX(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = V.vector4_f32[0];
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_lane_f32(vget_low_f32(V), 0);
#elif defined(_XM_AVX2_INTRINSICS_) && defined(_XM_FAVOR_INTEL_)
    return _mm_broadcastss_ps(V);
#elif defined(_XM_SSE_INTRINSICS_)
    return XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));
#endif
}

//------------------------------------------------------------------------------
// Replicate the y component of the vector
inline XMVECTOR XM_CALLCONV XMVectorSplatY(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = V.vector4_f32[1];
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_lane_f32(vget_low_f32(V), 1);
#elif defined(_XM_SSE_INTRINSICS_)
    return XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
#endif
}

//------------------------------------------------------------------------------
// Replicate the z component of the vector
inline XMVECTOR XM_CALLCONV XMVectorSplatZ(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = V.vector4_f32[2];
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_lane_f32(vget_high_f32(V), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    return XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
#endif
}

//------------------------------------------------------------------------------
// Replicate the w component of the vector
inline XMVECTOR XM_CALLCONV XMVectorSplatW(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = V.vector4_f32[3];
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_lane_f32(vget_high_f32(V), 1);
#elif defined(_XM_SSE_INTRINSICS_)
    return XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
#endif
}

//------------------------------------------------------------------------------
// Return a vector of 1.0f,1.0f,1.0f,1.0f
inline XMVECTOR XM_CALLCONV XMVectorSplatOne() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = 1.0f;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vdupq_n_f32(1.0f);
#elif defined(_XM_SSE_INTRINSICS_)
    return g_XMOne;
#endif
}

//------------------------------------------------------------------------------
// Return a vector of INF,INF,INF,INF
inline XMVECTOR XM_CALLCONV XMVectorSplatInfinity() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult;
    vResult.u[0] =
        vResult.u[1] =
        vResult.u[2] =
        vResult.u[3] = 0x7F800000;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vdupq_n_u32(0x7F800000));
#elif defined(_XM_SSE_INTRINSICS_)
    return g_XMInfinity;
#endif
}

//------------------------------------------------------------------------------
// Return a vector of Q_NAN,Q_NAN,Q_NAN,Q_NAN
inline XMVECTOR XM_CALLCONV XMVectorSplatQNaN() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult;
    vResult.u[0] =
        vResult.u[1] =
        vResult.u[2] =
        vResult.u[3] = 0x7FC00000;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vdupq_n_u32(0x7FC00000));
#elif defined(_XM_SSE_INTRINSICS_)
    return g_XMQNaN;
#endif
}

//------------------------------------------------------------------------------
// Return a vector of 1.192092896e-7f,1.192092896e-7f,1.192092896e-7f,1.192092896e-7f
inline XMVECTOR XM_CALLCONV XMVectorSplatEpsilon() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult;
    vResult.u[0] =
        vResult.u[1] =
        vResult.u[2] =
        vResult.u[3] = 0x34000000;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vdupq_n_u32(0x34000000));
#elif defined(_XM_SSE_INTRINSICS_)
    return g_XMEpsilon;
#endif
}

//------------------------------------------------------------------------------
// Return a vector of -0.0f (0x80000000),-0.0f,-0.0f,-0.0f
inline XMVECTOR XM_CALLCONV XMVectorSplatSignMask() noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 vResult;
    vResult.u[0] =
        vResult.u[1] =
        vResult.u[2] =
        vResult.u[3] = 0x80000000U;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vdupq_n_u32(0x80000000U));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_set1_epi32(static_cast<int>(0x80000000));
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------
// Return a floating point value via an index. This is not a recommended
// function to use due to performance loss.
inline float XM_CALLCONV XMVectorGetByIndex(FXMVECTOR V, size_t i) noexcept
{
    assert(i < 4);
    _Analysis_assume_(i < 4);
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_f32[i];
#else
    XMVECTORF32 U;
    U.v = V;
    return U.f[i];
#endif
}

//------------------------------------------------------------------------------
// Return the X component in an FPU register.
inline float XM_CALLCONV XMVectorGetX(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_f32[0];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_f32(V, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cvtss_f32(V);
#endif
}

// Return the Y component in an FPU register.
inline float XM_CALLCONV XMVectorGetY(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_f32[1];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_f32(V, 1);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
    return _mm_cvtss_f32(vTemp);
#endif
}

// Return the Z component in an FPU register.
inline float XM_CALLCONV XMVectorGetZ(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_f32[2];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_f32(V, 2);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
    return _mm_cvtss_f32(vTemp);
#endif
}

// Return the W component in an FPU register.
inline float XM_CALLCONV XMVectorGetW(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_f32[3];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_f32(V, 3);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
    return _mm_cvtss_f32(vTemp);
#endif
}

//------------------------------------------------------------------------------

// Store a component indexed by i into a 32 bit float location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetByIndexPtr(float* f, FXMVECTOR V, size_t i) noexcept
{
    assert(f != nullptr);
    assert(i < 4);
    _Analysis_assume_(i < 4);
#if defined(_XM_NO_INTRINSICS_)
    *f = V.vector4_f32[i];
#else
    XMVECTORF32 U;
    U.v = V;
    *f = U.f[i];
#endif
}

//------------------------------------------------------------------------------

// Store the X component into a 32 bit float location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetXPtr(float* x, FXMVECTOR V) noexcept
{
    assert(x != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *x = V.vector4_f32[0];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_f32(x, V, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    _mm_store_ss(x, V);
#endif
}

// Store the Y component into a 32 bit float location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetYPtr(float* y, FXMVECTOR V) noexcept
{
    assert(y != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *y = V.vector4_f32[1];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_f32(y, V, 1);
#elif defined(_XM_SSE4_INTRINSICS_)
    * (reinterpret_cast<int*>(y)) = _mm_extract_ps(V, 1);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
    _mm_store_ss(y, vResult);
#endif
}

// Store the Z component into a 32 bit float location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetZPtr(float* z, FXMVECTOR V) noexcept
{
    assert(z != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *z = V.vector4_f32[2];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_f32(z, V, 2);
#elif defined(_XM_SSE4_INTRINSICS_)
    * (reinterpret_cast<int*>(z)) = _mm_extract_ps(V, 2);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
    _mm_store_ss(z, vResult);
#endif
}

// Store the W component into a 32 bit float location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetWPtr(float* w, FXMVECTOR V) noexcept
{
    assert(w != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *w = V.vector4_f32[3];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_f32(w, V, 3);
#elif defined(_XM_SSE4_INTRINSICS_)
    * (reinterpret_cast<int*>(w)) = _mm_extract_ps(V, 3);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
    _mm_store_ss(w, vResult);
#endif
}

//------------------------------------------------------------------------------

// Return an integer value via an index. This is not a recommended
// function to use due to performance loss.
inline uint32_t XM_CALLCONV XMVectorGetIntByIndex(FXMVECTOR V, size_t i) noexcept
{
    assert(i < 4);
    _Analysis_assume_(i < 4);
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_u32[i];
#else
    XMVECTORU32 U;
    U.v = V;
    return U.u[i];
#endif
}

//------------------------------------------------------------------------------

// Return the X component in an integer register.
inline uint32_t XM_CALLCONV XMVectorGetIntX(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_u32[0];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_u32(vreinterpretq_u32_f32(V), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    return static_cast<uint32_t>(_mm_cvtsi128_si32(_mm_castps_si128(V)));
#endif
}

// Return the Y component in an integer register.
inline uint32_t XM_CALLCONV XMVectorGetIntY(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_u32[1];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_u32(vreinterpretq_u32_f32(V), 1);
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i V1 = _mm_castps_si128(V);
    return static_cast<uint32_t>(_mm_extract_epi32(V1, 1));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vResulti = _mm_shuffle_epi32(_mm_castps_si128(V), _MM_SHUFFLE(1, 1, 1, 1));
    return static_cast<uint32_t>(_mm_cvtsi128_si32(vResulti));
#endif
}

// Return the Z component in an integer register.
inline uint32_t XM_CALLCONV XMVectorGetIntZ(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_u32[2];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_u32(vreinterpretq_u32_f32(V), 2);
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i V1 = _mm_castps_si128(V);
    return static_cast<uint32_t>(_mm_extract_epi32(V1, 2));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vResulti = _mm_shuffle_epi32(_mm_castps_si128(V), _MM_SHUFFLE(2, 2, 2, 2));
    return static_cast<uint32_t>(_mm_cvtsi128_si32(vResulti));
#endif
}

// Return the W component in an integer register.
inline uint32_t XM_CALLCONV XMVectorGetIntW(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return V.vector4_u32[3];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vgetq_lane_u32(vreinterpretq_u32_f32(V), 3);
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i V1 = _mm_castps_si128(V);
    return static_cast<uint32_t>(_mm_extract_epi32(V1, 3));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vResulti = _mm_shuffle_epi32(_mm_castps_si128(V), _MM_SHUFFLE(3, 3, 3, 3));
    return static_cast<uint32_t>(_mm_cvtsi128_si32(vResulti));
#endif
}

//------------------------------------------------------------------------------

// Store a component indexed by i into a 32 bit integer location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetIntByIndexPtr(uint32_t* x, FXMVECTOR V, size_t i) noexcept
{
    assert(x != nullptr);
    assert(i < 4);
    _Analysis_assume_(i < 4);
#if defined(_XM_NO_INTRINSICS_)
    *x = V.vector4_u32[i];
#else
    XMVECTORU32 U;
    U.v = V;
    *x = U.u[i];
#endif
}

//------------------------------------------------------------------------------

// Store the X component into a 32 bit integer location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetIntXPtr(uint32_t* x, FXMVECTOR V) noexcept
{
    assert(x != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *x = V.vector4_u32[0];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_u32(x, *reinterpret_cast<const uint32x4_t*>(&V), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    _mm_store_ss(reinterpret_cast<float*>(x), V);
#endif
}

// Store the Y component into a 32 bit integer location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetIntYPtr(uint32_t* y, FXMVECTOR V) noexcept
{
    assert(y != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *y = V.vector4_u32[1];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_u32(y, *reinterpret_cast<const uint32x4_t*>(&V), 1);
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i V1 = _mm_castps_si128(V);
    *y = static_cast<uint32_t>(_mm_extract_epi32(V1, 1));
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
    _mm_store_ss(reinterpret_cast<float*>(y), vResult);
#endif
}

// Store the Z component into a 32 bit integer locaCantion in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetIntZPtr(uint32_t* z, FXMVECTOR V) noexcept
{
    assert(z != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *z = V.vector4_u32[2];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_u32(z, *reinterpret_cast<const uint32x4_t*>(&V), 2);
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i V1 = _mm_castps_si128(V);
    *z = static_cast<uint32_t>(_mm_extract_epi32(V1, 2));
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
    _mm_store_ss(reinterpret_cast<float*>(z), vResult);
#endif
}

// Store the W component into a 32 bit integer location in memory.
_Use_decl_annotations_
inline void XM_CALLCONV XMVectorGetIntWPtr(uint32_t* w, FXMVECTOR V) noexcept
{
    assert(w != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    *w = V.vector4_u32[3];
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    vst1q_lane_u32(w, *reinterpret_cast<const uint32x4_t*>(&V), 3);
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i V1 = _mm_castps_si128(V);
    *w = static_cast<uint32_t>(_mm_extract_epi32(V1, 3));
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
    _mm_store_ss(reinterpret_cast<float*>(w), vResult);
#endif
}

//------------------------------------------------------------------------------

// Set a single indexed floating point component
inline XMVECTOR XM_CALLCONV XMVectorSetByIndex(FXMVECTOR V, float f, size_t i) noexcept
{
    assert(i < 4);
    _Analysis_assume_(i < 4);
    XMVECTORF32 U;
    U.v = V;
    U.f[i] = f;
    return U.v;
}

//------------------------------------------------------------------------------

// Sets the X component of a vector to a passed floating point value
inline XMVECTOR XM_CALLCONV XMVectorSetX(FXMVECTOR V, float x) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            x,
            V.vector4_f32[1],
            V.vector4_f32[2],
            V.vector4_f32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vsetq_lane_f32(x, V, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_set_ss(x);
    vResult = _mm_move_ss(V, vResult);
    return vResult;
#endif
}

// Sets the Y component of a vector to a passed floating point value
inline XMVECTOR XM_CALLCONV XMVectorSetY(FXMVECTOR V, float y) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            V.vector4_f32[0],
            y,
            V.vector4_f32[2],
            V.vector4_f32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vsetq_lane_f32(y, V, 1);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vResult = _mm_set_ss(y);
    vResult = _mm_insert_ps(V, vResult, 0x10);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap y and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 2, 0, 1));
    // Convert input to vector
    XMVECTOR vTemp = _mm_set_ss(y);
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap y and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 2, 0, 1));
    return vResult;
#endif
}
// Sets the Z component of a vector to a passed floating point value
inline XMVECTOR XM_CALLCONV XMVectorSetZ(FXMVECTOR V, float z) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            V.vector4_f32[0],
            V.vector4_f32[1],
            z,
            V.vector4_f32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vsetq_lane_f32(z, V, 2);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vResult = _mm_set_ss(z);
    vResult = _mm_insert_ps(V, vResult, 0x20);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap z and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 0, 1, 2));
    // Convert input to vector
    XMVECTOR vTemp = _mm_set_ss(z);
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap z and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 0, 1, 2));
    return vResult;
#endif
}

// Sets the W component of a vector to a passed floating point value
inline XMVECTOR XM_CALLCONV XMVectorSetW(FXMVECTOR V, float w) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            V.vector4_f32[0],
            V.vector4_f32[1],
            V.vector4_f32[2],
            w
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vsetq_lane_f32(w, V, 3);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vResult = _mm_set_ss(w);
    vResult = _mm_insert_ps(V, vResult, 0x30);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap w and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 2, 1, 3));
    // Convert input to vector
    XMVECTOR vTemp = _mm_set_ss(w);
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap w and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 2, 1, 3));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

// Sets a component of a vector to a floating point value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetByIndexPtr(FXMVECTOR V, const float* f, size_t i) noexcept
{
    assert(f != nullptr);
    assert(i < 4);
    _Analysis_assume_(i < 4);
    XMVECTORF32 U;
    U.v = V;
    U.f[i] = *f;
    return U.v;
}

//------------------------------------------------------------------------------

// Sets the X component of a vector to a floating point value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetXPtr(FXMVECTOR V, const float* x) noexcept
{
    assert(x != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            *x,
            V.vector4_f32[1],
            V.vector4_f32[2],
            V.vector4_f32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vld1q_lane_f32(x, V, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_load_ss(x);
    vResult = _mm_move_ss(V, vResult);
    return vResult;
#endif
}

// Sets the Y component of a vector to a floating point value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetYPtr(FXMVECTOR V, const float* y) noexcept
{
    assert(y != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            V.vector4_f32[0],
            *y,
            V.vector4_f32[2],
            V.vector4_f32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vld1q_lane_f32(y, V, 1);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap y and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 2, 0, 1));
    // Convert input to vector
    XMVECTOR vTemp = _mm_load_ss(y);
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap y and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 2, 0, 1));
    return vResult;
#endif
}

// Sets the Z component of a vector to a floating point value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetZPtr(FXMVECTOR V, const float* z) noexcept
{
    assert(z != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            V.vector4_f32[0],
            V.vector4_f32[1],
            *z,
            V.vector4_f32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vld1q_lane_f32(z, V, 2);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap z and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 0, 1, 2));
    // Convert input to vector
    XMVECTOR vTemp = _mm_load_ss(z);
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap z and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 0, 1, 2));
    return vResult;
#endif
}

// Sets the W component of a vector to a floating point value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetWPtr(FXMVECTOR V, const float* w) noexcept
{
    assert(w != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 U = { { {
            V.vector4_f32[0],
            V.vector4_f32[1],
            V.vector4_f32[2],
            *w
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vld1q_lane_f32(w, V, 3);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap w and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 2, 1, 3));
    // Convert input to vector
    XMVECTOR vTemp = _mm_load_ss(w);
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap w and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 2, 1, 3));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

// Sets a component of a vector to an integer passed by value
inline XMVECTOR XM_CALLCONV XMVectorSetIntByIndex(FXMVECTOR V, uint32_t x, size_t i) noexcept
{
    assert(i < 4);
    _Analysis_assume_(i < 4);
    XMVECTORU32 tmp;
    tmp.v = V;
    tmp.u[i] = x;
    return tmp;
}

//------------------------------------------------------------------------------

// Sets the X component of a vector to an integer passed by value
inline XMVECTOR XM_CALLCONV XMVectorSetIntX(FXMVECTOR V, uint32_t x) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            x,
            V.vector4_u32[1],
            V.vector4_u32[2],
            V.vector4_u32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vsetq_lane_u32(x, vreinterpretq_u32_f32(V), 0));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cvtsi32_si128(static_cast<int>(x));
    XMVECTOR vResult = _mm_move_ss(V, _mm_castsi128_ps(vTemp));
    return vResult;
#endif
}

// Sets the Y component of a vector to an integer passed by value
inline XMVECTOR XM_CALLCONV XMVectorSetIntY(FXMVECTOR V, uint32_t y) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            V.vector4_u32[0],
            y,
            V.vector4_u32[2],
            V.vector4_u32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vsetq_lane_u32(y, vreinterpretq_u32_f32(V), 1));
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i vResult = _mm_castps_si128(V);
    vResult = _mm_insert_epi32(vResult, static_cast<int>(y), 1);
    return _mm_castsi128_ps(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap y and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 2, 0, 1));
    // Convert input to vector
    __m128i vTemp = _mm_cvtsi32_si128(static_cast<int>(y));
    // Replace the x component
    vResult = _mm_move_ss(vResult, _mm_castsi128_ps(vTemp));
    // Swap y and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 2, 0, 1));
    return vResult;
#endif
}

// Sets the Z component of a vector to an integer passed by value
inline XMVECTOR XM_CALLCONV XMVectorSetIntZ(FXMVECTOR V, uint32_t z) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            V.vector4_u32[0],
            V.vector4_u32[1],
            z,
            V.vector4_u32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vsetq_lane_u32(z, vreinterpretq_u32_f32(V), 2));
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i vResult = _mm_castps_si128(V);
    vResult = _mm_insert_epi32(vResult, static_cast<int>(z), 2);
    return _mm_castsi128_ps(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap z and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 0, 1, 2));
    // Convert input to vector
    __m128i vTemp = _mm_cvtsi32_si128(static_cast<int>(z));
    // Replace the x component
    vResult = _mm_move_ss(vResult, _mm_castsi128_ps(vTemp));
    // Swap z and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 0, 1, 2));
    return vResult;
#endif
}

// Sets the W component of a vector to an integer passed by value
inline XMVECTOR XM_CALLCONV XMVectorSetIntW(FXMVECTOR V, uint32_t w) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            V.vector4_u32[0],
            V.vector4_u32[1],
            V.vector4_u32[2],
            w
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vsetq_lane_u32(w, vreinterpretq_u32_f32(V), 3));
#elif defined(_XM_SSE4_INTRINSICS_)
    __m128i vResult = _mm_castps_si128(V);
    vResult = _mm_insert_epi32(vResult, static_cast<int>(w), 3);
    return _mm_castsi128_ps(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap w and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 2, 1, 3));
    // Convert input to vector
    __m128i vTemp = _mm_cvtsi32_si128(static_cast<int>(w));
    // Replace the x component
    vResult = _mm_move_ss(vResult, _mm_castsi128_ps(vTemp));
    // Swap w and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 2, 1, 3));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

// Sets a component of a vector to an integer value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetIntByIndexPtr(FXMVECTOR V, const uint32_t* x, size_t i) noexcept
{
    assert(x != nullptr);
    assert(i < 4);
    _Analysis_assume_(i < 4);
    XMVECTORU32 tmp;
    tmp.v = V;
    tmp.u[i] = *x;
    return tmp;
}

//------------------------------------------------------------------------------

// Sets the X component of a vector to an integer value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetIntXPtr(FXMVECTOR V, const uint32_t* x) noexcept
{
    assert(x != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            *x,
            V.vector4_u32[1],
            V.vector4_u32[2],
            V.vector4_u32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vld1q_lane_u32(x, *reinterpret_cast<const uint32x4_t*>(&V), 0));
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_load_ss(reinterpret_cast<const float*>(x));
    XMVECTOR vResult = _mm_move_ss(V, vTemp);
    return vResult;
#endif
}

// Sets the Y component of a vector to an integer value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetIntYPtr(FXMVECTOR V, const uint32_t* y) noexcept
{
    assert(y != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            V.vector4_u32[0],
            *y,
            V.vector4_u32[2],
            V.vector4_u32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vld1q_lane_u32(y, *reinterpret_cast<const uint32x4_t*>(&V), 1));
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap y and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 2, 0, 1));
    // Convert input to vector
    XMVECTOR vTemp = _mm_load_ss(reinterpret_cast<const float*>(y));
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap y and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 2, 0, 1));
    return vResult;
#endif
}

// Sets the Z component of a vector to an integer value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetIntZPtr(FXMVECTOR V, const uint32_t* z) noexcept
{
    assert(z != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            V.vector4_u32[0],
            V.vector4_u32[1],
            *z,
            V.vector4_u32[3]
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vld1q_lane_u32(z, *reinterpret_cast<const uint32x4_t*>(&V), 2));
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap z and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 0, 1, 2));
    // Convert input to vector
    XMVECTOR vTemp = _mm_load_ss(reinterpret_cast<const float*>(z));
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap z and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 0, 1, 2));
    return vResult;
#endif
}

// Sets the W component of a vector to an integer value passed by pointer
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorSetIntWPtr(FXMVECTOR V, const uint32_t* w) noexcept
{
    assert(w != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORU32 U = { { {
            V.vector4_u32[0],
            V.vector4_u32[1],
            V.vector4_u32[2],
            *w
        } } };
    return U.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vld1q_lane_u32(w, *reinterpret_cast<const uint32x4_t*>(&V), 3));
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap w and x
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 2, 1, 3));
    // Convert input to vector
    XMVECTOR vTemp = _mm_load_ss(reinterpret_cast<const float*>(w));
    // Replace the x component
    vResult = _mm_move_ss(vResult, vTemp);
    // Swap w and x again
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 2, 1, 3));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSwizzle
(
    FXMVECTOR V,
    uint32_t E0,
    uint32_t E1,
    uint32_t E2,
    uint32_t E3
) noexcept
{
    assert((E0 < 4) && (E1 < 4) && (E2 < 4) && (E3 < 4));
    _Analysis_assume_((E0 < 4) && (E1 < 4) && (E2 < 4) && (E3 < 4));
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            V.vector4_f32[E0],
            V.vector4_f32[E1],
            V.vector4_f32[E2],
            V.vector4_f32[E3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const uint32_t ControlElement[4] =
    {
        0x03020100, // XM_SWIZZLE_X
        0x07060504, // XM_SWIZZLE_Y
        0x0B0A0908, // XM_SWIZZLE_Z
        0x0F0E0D0C, // XM_SWIZZLE_W
    };

    uint8x8x2_t tbl;
    tbl.val[0] = vreinterpret_u8_f32(vget_low_f32(V));
    tbl.val[1] = vreinterpret_u8_f32(vget_high_f32(V));

    uint32x2_t idx = vcreate_u32(static_cast<uint64_t>(ControlElement[E0]) | (static_cast<uint64_t>(ControlElement[E1]) << 32));
    const uint8x8_t rL = vtbl2_u8(tbl, vreinterpret_u8_u32(idx));

    idx = vcreate_u32(static_cast<uint64_t>(ControlElement[E2]) | (static_cast<uint64_t>(ControlElement[E3]) << 32));
    const uint8x8_t rH = vtbl2_u8(tbl, vreinterpret_u8_u32(idx));

    return vcombine_f32(vreinterpret_f32_u8(rL), vreinterpret_f32_u8(rH));
#elif defined(_XM_AVX_INTRINSICS_)
    unsigned int elem[4] = { E0, E1, E2, E3 };
    __m128i vControl = _mm_loadu_si128(reinterpret_cast<const __m128i*>(&elem[0]));
    return _mm_permutevar_ps(V, vControl);
#else
    auto aPtr = reinterpret_cast<const uint32_t*>(&V);

    XMVECTOR Result;
    auto pWork = reinterpret_cast<uint32_t*>(&Result);

    pWork[0] = aPtr[E0];
    pWork[1] = aPtr[E1];
    pWork[2] = aPtr[E2];
    pWork[3] = aPtr[E3];

    return Result;
#endif
}

//------------------------------------------------------------------------------
inline XMVECTOR XM_CALLCONV XMVectorPermute
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    uint32_t PermuteX,
    uint32_t PermuteY,
    uint32_t PermuteZ,
    uint32_t PermuteW
) noexcept
{
    assert(PermuteX <= 7 && PermuteY <= 7 && PermuteZ <= 7 && PermuteW <= 7);
    _Analysis_assume_(PermuteX <= 7 && PermuteY <= 7 && PermuteZ <= 7 && PermuteW <= 7);

#if defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    static const uint32_t ControlElement[8] =
    {
        0x03020100, // XM_PERMUTE_0X
        0x07060504, // XM_PERMUTE_0Y
        0x0B0A0908, // XM_PERMUTE_0Z
        0x0F0E0D0C, // XM_PERMUTE_0W
        0x13121110, // XM_PERMUTE_1X
        0x17161514, // XM_PERMUTE_1Y
        0x1B1A1918, // XM_PERMUTE_1Z
        0x1F1E1D1C, // XM_PERMUTE_1W
    };

    uint8x8x4_t tbl;
    tbl.val[0] = vreinterpret_u8_f32(vget_low_f32(V1));
    tbl.val[1] = vreinterpret_u8_f32(vget_high_f32(V1));
    tbl.val[2] = vreinterpret_u8_f32(vget_low_f32(V2));
    tbl.val[3] = vreinterpret_u8_f32(vget_high_f32(V2));

    uint32x2_t idx = vcreate_u32(static_cast<uint64_t>(ControlElement[PermuteX]) | (static_cast<uint64_t>(ControlElement[PermuteY]) << 32));
    const uint8x8_t rL = vtbl4_u8(tbl, vreinterpret_u8_u32(idx));

    idx = vcreate_u32(static_cast<uint64_t>(ControlElement[PermuteZ]) | (static_cast<uint64_t>(ControlElement[PermuteW]) << 32));
    const uint8x8_t rH = vtbl4_u8(tbl, vreinterpret_u8_u32(idx));

    return vcombine_f32(vreinterpret_f32_u8(rL), vreinterpret_f32_u8(rH));
#elif defined(_XM_AVX_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    static const XMVECTORU32 three = { { { 3, 3, 3, 3 } } };

    XM_ALIGNED_DATA(16) unsigned int elem[4] = { PermuteX, PermuteY, PermuteZ, PermuteW };
    __m128i vControl = _mm_load_si128(reinterpret_cast<const __m128i*>(&elem[0]));

    __m128i vSelect = _mm_cmpgt_epi32(vControl, three);
    vControl = _mm_castps_si128(_mm_and_ps(_mm_castsi128_ps(vControl), three));

    __m128 shuffled1 = _mm_permutevar_ps(V1, vControl);
    __m128 shuffled2 = _mm_permutevar_ps(V2, vControl);

    __m128 masked1 = _mm_andnot_ps(_mm_castsi128_ps(vSelect), shuffled1);
    __m128 masked2 = _mm_and_ps(_mm_castsi128_ps(vSelect), shuffled2);

    return _mm_or_ps(masked1, masked2);
#else

    const uint32_t* aPtr[2];
    aPtr[0] = reinterpret_cast<const uint32_t*>(&V1);
    aPtr[1] = reinterpret_cast<const uint32_t*>(&V2);

    XMVECTOR Result;
    auto pWork = reinterpret_cast<uint32_t*>(&Result);

    const uint32_t i0 = PermuteX & 3;
    const uint32_t vi0 = PermuteX >> 2;
    pWork[0] = aPtr[vi0][i0];

    const uint32_t i1 = PermuteY & 3;
    const uint32_t vi1 = PermuteY >> 2;
    pWork[1] = aPtr[vi1][i1];

    const uint32_t i2 = PermuteZ & 3;
    const uint32_t vi2 = PermuteZ >> 2;
    pWork[2] = aPtr[vi2][i2];

    const uint32_t i3 = PermuteW & 3;
    const uint32_t vi3 = PermuteW >> 2;
    pWork[3] = aPtr[vi3][i3];

    return Result;
#endif
}

//------------------------------------------------------------------------------
// Define a control vector to be used in XMVectorSelect
// operations.  The four integers specified in XMVectorSelectControl
// serve as indices to select between components in two vectors.
// The first index controls selection for the first component of
// the vectors involved in a select operation, the second index
// controls selection for the second component etc.  A value of
// zero for an index causes the corresponding component from the first
// vector to be selected whereas a one causes the component from the
// second vector to be selected instead.

inline XMVECTOR XM_CALLCONV XMVectorSelectControl
(
    uint32_t VectorIndex0,
    uint32_t VectorIndex1,
    uint32_t VectorIndex2,
    uint32_t VectorIndex3
) noexcept
{
#if defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    // x=Index0,y=Index1,z=Index2,w=Index3
    __m128i vTemp = _mm_set_epi32(static_cast<int>(VectorIndex3), static_cast<int>(VectorIndex2), static_cast<int>(VectorIndex1), static_cast<int>(VectorIndex0));
    // Any non-zero entries become 0xFFFFFFFF else 0
    vTemp = _mm_cmpgt_epi32(vTemp, g_XMZero);
    return _mm_castsi128_ps(vTemp);
#elif defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    int32x2_t V0 = vcreate_s32(static_cast<uint64_t>(VectorIndex0) | (static_cast<uint64_t>(VectorIndex1) << 32));
    int32x2_t V1 = vcreate_s32(static_cast<uint64_t>(VectorIndex2) | (static_cast<uint64_t>(VectorIndex3) << 32));
    int32x4_t vTemp = vcombine_s32(V0, V1);
    // Any non-zero entries become 0xFFFFFFFF else 0
    return vreinterpretq_f32_u32(vcgtq_s32(vTemp, g_XMZero));
#else
    XMVECTOR    ControlVector;
    const uint32_t  ControlElement[] =
    {
        XM_SELECT_0,
        XM_SELECT_1
    };

    assert(VectorIndex0 < 2);
    assert(VectorIndex1 < 2);
    assert(VectorIndex2 < 2);
    assert(VectorIndex3 < 2);
    _Analysis_assume_(VectorIndex0 < 2);
    _Analysis_assume_(VectorIndex1 < 2);
    _Analysis_assume_(VectorIndex2 < 2);
    _Analysis_assume_(VectorIndex3 < 2);

    ControlVector.vector4_u32[0] = ControlElement[VectorIndex0];
    ControlVector.vector4_u32[1] = ControlElement[VectorIndex1];
    ControlVector.vector4_u32[2] = ControlElement[VectorIndex2];
    ControlVector.vector4_u32[3] = ControlElement[VectorIndex3];

    return ControlVector;

#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSelect
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR Control
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            (V1.vector4_u32[0] & ~Control.vector4_u32[0]) | (V2.vector4_u32[0] & Control.vector4_u32[0]),
            (V1.vector4_u32[1] & ~Control.vector4_u32[1]) | (V2.vector4_u32[1] & Control.vector4_u32[1]),
            (V1.vector4_u32[2] & ~Control.vector4_u32[2]) | (V2.vector4_u32[2] & Control.vector4_u32[2]),
            (V1.vector4_u32[3] & ~Control.vector4_u32[3]) | (V2.vector4_u32[3] & Control.vector4_u32[3]),
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vbslq_f32(vreinterpretq_u32_f32(Control), V2, V1);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp1 = _mm_andnot_ps(Control, V1);
    XMVECTOR vTemp2 = _mm_and_ps(V2, Control);
    return _mm_or_ps(vTemp1, vTemp2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMergeXY
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            V1.vector4_u32[0],
            V2.vector4_u32[0],
            V1.vector4_u32[1],
            V2.vector4_u32[1],
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vzipq_f32(V1, V2).val[0];
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_unpacklo_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMergeZW
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            V1.vector4_u32[2],
            V2.vector4_u32[2],
            V1.vector4_u32[3],
            V2.vector4_u32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vzipq_f32(V1, V2).val[1];
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_unpackhi_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorShiftLeft(FXMVECTOR V1, FXMVECTOR V2, uint32_t Elements) noexcept
{
    assert(Elements < 4);
    _Analysis_assume_(Elements < 4);
    return XMVectorPermute(V1, V2, Elements, ((Elements)+1), ((Elements)+2), ((Elements)+3));
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorRotateLeft(FXMVECTOR V, uint32_t Elements) noexcept
{
    assert(Elements < 4);
    _Analysis_assume_(Elements < 4);
    return XMVectorSwizzle(V, Elements & 3, (Elements + 1) & 3, (Elements + 2) & 3, (Elements + 3) & 3);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorRotateRight(FXMVECTOR V, uint32_t Elements) noexcept
{
    assert(Elements < 4);
    _Analysis_assume_(Elements < 4);
    return XMVectorSwizzle(V, (4 - (Elements)) & 3, (5 - (Elements)) & 3, (6 - (Elements)) & 3, (7 - (Elements)) & 3);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorInsert(
    FXMVECTOR VD, FXMVECTOR VS,
    uint32_t VSLeftRotateElements,
    uint32_t Select0, uint32_t Select1, uint32_t Select2, uint32_t Select3) noexcept
{
    XMVECTOR Control = XMVectorSelectControl(Select0 & 1, Select1 & 1, Select2 & 1, Select3 & 1);
    return XMVectorSelect(VD, XMVectorRotateLeft(VS, VSLeftRotateElements), Control);
}

//------------------------------------------------------------------------------
// Comparison operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_f32[0] == V2.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[1] == V2.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[2] == V2.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[3] == V2.vector4_f32[3]) ? 0xFFFFFFFF : 0,
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vceqq_f32(V1, V2));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cmpeq_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorEqualR
(
    uint32_t* pCR,
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    assert(pCR != nullptr);
#if defined(_XM_NO_INTRINSICS_)
    uint32_t ux = (V1.vector4_f32[0] == V2.vector4_f32[0]) ? 0xFFFFFFFFU : 0;
    uint32_t uy = (V1.vector4_f32[1] == V2.vector4_f32[1]) ? 0xFFFFFFFFU : 0;
    uint32_t uz = (V1.vector4_f32[2] == V2.vector4_f32[2]) ? 0xFFFFFFFFU : 0;
    uint32_t uw = (V1.vector4_f32[3] == V2.vector4_f32[3]) ? 0xFFFFFFFFU : 0;
    uint32_t CR = 0;
    if (ux & uy & uz & uw)
    {
        // All elements are greater
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!(ux | uy | uz | uw))
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;

    XMVECTORU32 Control = { { { ux, uy, uz, uw } } };
    return Control;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vreinterpret_u8_u32(vget_low_u32(vResult)), vreinterpret_u8_u32(vget_high_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        // All elements are equal
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        // All elements are not equal
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vreinterpretq_f32_u32(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    uint32_t CR = 0;
    int iTest = _mm_movemask_ps(vTemp);
    if (iTest == 0xf)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
// Treat the components of the vectors as unsigned integers and
// compare individual bits between the two.  This is useful for
// comparing control vectors and result vectors returned from
// other comparison operations.

inline XMVECTOR XM_CALLCONV XMVectorEqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_u32[0] == V2.vector4_u32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_u32[1] == V2.vector4_u32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_u32[2] == V2.vector4_u32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_u32[3] == V2.vector4_u32[3]) ? 0xFFFFFFFF : 0,
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vceqq_s32(vreinterpretq_s32_f32(V1), vreinterpretq_s32_f32(V2)));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorEqualIntR
(
    uint32_t* pCR,
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    assert(pCR != nullptr);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Control = XMVectorEqualInt(V1, V2);

    *pCR = 0;
    if (XMVector4EqualInt(Control, XMVectorTrueInt()))
    {
        // All elements are equal
        *pCR |= XM_CRMASK_CR6TRUE;
    }
    else if (XMVector4EqualInt(Control, XMVectorFalseInt()))
    {
        // All elements are not equal
        *pCR |= XM_CRMASK_CR6FALSE;
    }
    return Control;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        // All elements are equal
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        // All elements are not equal
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vreinterpretq_f32_u32(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    int iTemp = _mm_movemask_ps(_mm_castsi128_ps(V));
    uint32_t CR = 0;
    if (iTemp == 0x0F)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTemp)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorNearEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR Epsilon
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    float fDeltax = V1.vector4_f32[0] - V2.vector4_f32[0];
    float fDeltay = V1.vector4_f32[1] - V2.vector4_f32[1];
    float fDeltaz = V1.vector4_f32[2] - V2.vector4_f32[2];
    float fDeltaw = V1.vector4_f32[3] - V2.vector4_f32[3];

    fDeltax = fabsf(fDeltax);
    fDeltay = fabsf(fDeltay);
    fDeltaz = fabsf(fDeltaz);
    fDeltaw = fabsf(fDeltaw);

    XMVECTORU32 Control = { { {
            (fDeltax <= Epsilon.vector4_f32[0]) ? 0xFFFFFFFFU : 0,
            (fDeltay <= Epsilon.vector4_f32[1]) ? 0xFFFFFFFFU : 0,
            (fDeltaz <= Epsilon.vector4_f32[2]) ? 0xFFFFFFFFU : 0,
            (fDeltaw <= Epsilon.vector4_f32[3]) ? 0xFFFFFFFFU : 0,
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vDelta = vsubq_f32(V1, V2);
#if defined(_MSC_VER) && !defined(__clang__) && !defined(_ARM64_DISTINCT_NEON_TYPES)
    return vacleq_f32(vDelta, Epsilon);
#else
    return vreinterpretq_f32_u32(vcleq_f32(vabsq_f32(vDelta), Epsilon));
#endif
#elif defined(_XM_SSE_INTRINSICS_)
    // Get the difference
    XMVECTOR vDelta = _mm_sub_ps(V1, V2);
    // Get the absolute value of the difference
    XMVECTOR vTemp = _mm_setzero_ps();
    vTemp = _mm_sub_ps(vTemp, vDelta);
    vTemp = _mm_max_ps(vTemp, vDelta);
    vTemp = _mm_cmple_ps(vTemp, Epsilon);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorNotEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_f32[0] != V2.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[1] != V2.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[2] != V2.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[3] != V2.vector4_f32[3]) ? 0xFFFFFFFF : 0,
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vmvnq_u32(vceqq_f32(V1, V2)));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cmpneq_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorNotEqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_u32[0] != V2.vector4_u32[0]) ? 0xFFFFFFFFU : 0,
            (V1.vector4_u32[1] != V2.vector4_u32[1]) ? 0xFFFFFFFFU : 0,
            (V1.vector4_u32[2] != V2.vector4_u32[2]) ? 0xFFFFFFFFU : 0,
            (V1.vector4_u32[3] != V2.vector4_u32[3]) ? 0xFFFFFFFFU : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vmvnq_u32(
            vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2))));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return _mm_xor_ps(_mm_castsi128_ps(V), g_XMNegOneMask);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorGreater
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_f32[0] > V2.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[1] > V2.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[2] > V2.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[3] > V2.vector4_f32[3]) ? 0xFFFFFFFF : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vcgtq_f32(V1, V2));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cmpgt_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorGreaterR
(
    uint32_t* pCR,
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    assert(pCR != nullptr);
#if defined(_XM_NO_INTRINSICS_)

    uint32_t ux = (V1.vector4_f32[0] > V2.vector4_f32[0]) ? 0xFFFFFFFFU : 0;
    uint32_t uy = (V1.vector4_f32[1] > V2.vector4_f32[1]) ? 0xFFFFFFFFU : 0;
    uint32_t uz = (V1.vector4_f32[2] > V2.vector4_f32[2]) ? 0xFFFFFFFFU : 0;
    uint32_t uw = (V1.vector4_f32[3] > V2.vector4_f32[3]) ? 0xFFFFFFFFU : 0;
    uint32_t CR = 0;
    if (ux & uy & uz & uw)
    {
        // All elements are greater
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!(ux | uy | uz | uw))
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;

    XMVECTORU32 Control = { { { ux, uy, uz, uw } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgtq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        // All elements are greater
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vreinterpretq_f32_u32(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    uint32_t CR = 0;
    int iTest = _mm_movemask_ps(vTemp);
    if (iTest == 0xf)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vTemp;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorGreaterOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_f32[0] >= V2.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[1] >= V2.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[2] >= V2.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[3] >= V2.vector4_f32[3]) ? 0xFFFFFFFF : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vcgeq_f32(V1, V2));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cmpge_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorGreaterOrEqualR
(
    uint32_t* pCR,
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    assert(pCR != nullptr);
#if defined(_XM_NO_INTRINSICS_)

    uint32_t ux = (V1.vector4_f32[0] >= V2.vector4_f32[0]) ? 0xFFFFFFFFU : 0;
    uint32_t uy = (V1.vector4_f32[1] >= V2.vector4_f32[1]) ? 0xFFFFFFFFU : 0;
    uint32_t uz = (V1.vector4_f32[2] >= V2.vector4_f32[2]) ? 0xFFFFFFFFU : 0;
    uint32_t uw = (V1.vector4_f32[3] >= V2.vector4_f32[3]) ? 0xFFFFFFFFU : 0;
    uint32_t CR = 0;
    if (ux & uy & uz & uw)
    {
        // All elements are greater
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!(ux | uy | uz | uw))
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;

    XMVECTORU32 Control = { { { ux, uy, uz, uw } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgeq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        // All elements are greater or equal
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        // All elements are not greater or equal
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vreinterpretq_f32_u32(vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    uint32_t CR = 0;
    int iTest = _mm_movemask_ps(vTemp);
    if (iTest == 0xf)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        // All elements are not greater
        CR = XM_CRMASK_CR6FALSE;
    }
    *pCR = CR;
    return vTemp;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLess
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_f32[0] < V2.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[1] < V2.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[2] < V2.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[3] < V2.vector4_f32[3]) ? 0xFFFFFFFF : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vcltq_f32(V1, V2));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cmplt_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLessOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V1.vector4_f32[0] <= V2.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[1] <= V2.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[2] <= V2.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V1.vector4_f32[3] <= V2.vector4_f32[3]) ? 0xFFFFFFFF : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vcleq_f32(V1, V2));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_cmple_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorInBounds
(
    FXMVECTOR V,
    FXMVECTOR Bounds
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            (V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) ? 0xFFFFFFFF : 0,
            (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1]) ? 0xFFFFFFFF : 0,
            (V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2]) ? 0xFFFFFFFF : 0,
            (V.vector4_f32[3] <= Bounds.vector4_f32[3] && V.vector4_f32[3] >= -Bounds.vector4_f32[3]) ? 0xFFFFFFFF : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Test if less than or equal
    uint32x4_t vTemp1 = vcleq_f32(V, Bounds);
    // Negate the bounds
    uint32x4_t vTemp2 = vreinterpretq_u32_f32(vnegq_f32(Bounds));
    // Test if greater or equal (Reversed)
    vTemp2 = vcleq_f32(vreinterpretq_f32_u32(vTemp2), V);
    // Blend answers
    vTemp1 = vandq_u32(vTemp1, vTemp2);
    return vreinterpretq_f32_u32(vTemp1);
#elif defined(_XM_SSE_INTRINSICS_)
    // Test if less than or equal
    XMVECTOR vTemp1 = _mm_cmple_ps(V, Bounds);
    // Negate the bounds
    XMVECTOR vTemp2 = _mm_mul_ps(Bounds, g_XMNegativeOne);
    // Test if greater or equal (Reversed)
    vTemp2 = _mm_cmple_ps(vTemp2, V);
    // Blend answers
    vTemp1 = _mm_and_ps(vTemp1, vTemp2);
    return vTemp1;
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMVectorInBoundsR
(
    uint32_t* pCR,
    FXMVECTOR V,
    FXMVECTOR Bounds
) noexcept
{
    assert(pCR != nullptr);
#if defined(_XM_NO_INTRINSICS_)

    uint32_t ux = (V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) ? 0xFFFFFFFFU : 0;
    uint32_t uy = (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1]) ? 0xFFFFFFFFU : 0;
    uint32_t uz = (V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2]) ? 0xFFFFFFFFU : 0;
    uint32_t uw = (V.vector4_f32[3] <= Bounds.vector4_f32[3] && V.vector4_f32[3] >= -Bounds.vector4_f32[3]) ? 0xFFFFFFFFU : 0;

    uint32_t CR = 0;
    if (ux & uy & uz & uw)
    {
        // All elements are in bounds
        CR = XM_CRMASK_CR6BOUNDS;
    }
    *pCR = CR;

    XMVECTORU32 Control = { { { ux, uy, uz, uw } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Test if less than or equal
    uint32x4_t vTemp1 = vcleq_f32(V, Bounds);
    // Negate the bounds
    uint32x4_t vTemp2 = vreinterpretq_u32_f32(vnegq_f32(Bounds));
    // Test if greater or equal (Reversed)
    vTemp2 = vcleq_f32(vreinterpretq_f32_u32(vTemp2), V);
    // Blend answers
    vTemp1 = vandq_u32(vTemp1, vTemp2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vTemp1)), vget_high_u8(vreinterpretq_u8_u32(vTemp1)));
    uint16x4x2_t vTemp3 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp3.val[1]), 1);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        // All elements are in bounds
        CR = XM_CRMASK_CR6BOUNDS;
    }
    *pCR = CR;
    return vreinterpretq_f32_u32(vTemp1);
#elif defined(_XM_SSE_INTRINSICS_)
    // Test if less than or equal
    XMVECTOR vTemp1 = _mm_cmple_ps(V, Bounds);
    // Negate the bounds
    XMVECTOR vTemp2 = _mm_mul_ps(Bounds, g_XMNegativeOne);
    // Test if greater or equal (Reversed)
    vTemp2 = _mm_cmple_ps(vTemp2, V);
    // Blend answers
    vTemp1 = _mm_and_ps(vTemp1, vTemp2);

    uint32_t CR = 0;
    if (_mm_movemask_ps(vTemp1) == 0xf)
    {
        // All elements are in bounds
        CR = XM_CRMASK_CR6BOUNDS;
    }
    *pCR = CR;
    return vTemp1;
#endif
}

//------------------------------------------------------------------------------

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(push)
#pragma float_control(precise, on)
#endif

inline XMVECTOR XM_CALLCONV XMVectorIsNaN(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            XMISNAN(V.vector4_f32[0]) ? 0xFFFFFFFFU : 0,
            XMISNAN(V.vector4_f32[1]) ? 0xFFFFFFFFU : 0,
            XMISNAN(V.vector4_f32[2]) ? 0xFFFFFFFFU : 0,
            XMISNAN(V.vector4_f32[3]) ? 0xFFFFFFFFU : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    XMVECTORU32 vResult = { { {
        isnan(vgetq_lane_f32(V, 0)) ? 0xFFFFFFFFU : 0,
        isnan(vgetq_lane_f32(V, 1)) ? 0xFFFFFFFFU : 0,
        isnan(vgetq_lane_f32(V, 2)) ? 0xFFFFFFFFU : 0,
        isnan(vgetq_lane_f32(V, 3)) ? 0xFFFFFFFFU : 0 } } };
    return vResult.v;
    #else
    // Test against itself. NaN is always not equal
    uint32x4_t vTempNan = vceqq_f32(V, V);
    // Flip results
    return vreinterpretq_f32_u32(vmvnq_u32(vTempNan));
    #endif
#elif defined(_XM_SSE_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    XM_ALIGNED_DATA(16) float tmp[4];
    _mm_store_ps(tmp, V);
    XMVECTORU32 vResult = { { {
        isnan(tmp[0]) ? 0xFFFFFFFFU : 0,
        isnan(tmp[1]) ? 0xFFFFFFFFU : 0,
        isnan(tmp[2]) ? 0xFFFFFFFFU : 0,
        isnan(tmp[3]) ? 0xFFFFFFFFU : 0 } } };
    return vResult.v;
    #else
    // Test against itself. NaN is always not equal
    return _mm_cmpneq_ps(V, V);
    #endif
#endif
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(pop)
#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorIsInfinite(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Control = { { {
            XMISINF(V.vector4_f32[0]) ? 0xFFFFFFFFU : 0,
            XMISINF(V.vector4_f32[1]) ? 0xFFFFFFFFU : 0,
            XMISINF(V.vector4_f32[2]) ? 0xFFFFFFFFU : 0,
            XMISINF(V.vector4_f32[3]) ? 0xFFFFFFFFU : 0
        } } };
    return Control.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Mask off the sign bit
    uint32x4_t vTemp = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    // Compare to infinity
    vTemp = vceqq_f32(vreinterpretq_f32_u32(vTemp), g_XMInfinity);
    // If any are infinity, the signs are true.
    return vreinterpretq_f32_u32(vTemp);
#elif defined(_XM_SSE_INTRINSICS_)
    // Mask off the sign bit
    __m128 vTemp = _mm_and_ps(V, g_XMAbsMask);
    // Compare to infinity
    vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity);
    // If any are infinity, the signs are true.
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
// Rounding and clamping operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMin
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            (V1.vector4_f32[0] < V2.vector4_f32[0]) ? V1.vector4_f32[0] : V2.vector4_f32[0],
            (V1.vector4_f32[1] < V2.vector4_f32[1]) ? V1.vector4_f32[1] : V2.vector4_f32[1],
            (V1.vector4_f32[2] < V2.vector4_f32[2]) ? V1.vector4_f32[2] : V2.vector4_f32[2],
            (V1.vector4_f32[3] < V2.vector4_f32[3]) ? V1.vector4_f32[3] : V2.vector4_f32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vminq_f32(V1, V2);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_min_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMax
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            (V1.vector4_f32[0] > V2.vector4_f32[0]) ? V1.vector4_f32[0] : V2.vector4_f32[0],
            (V1.vector4_f32[1] > V2.vector4_f32[1]) ? V1.vector4_f32[1] : V2.vector4_f32[1],
            (V1.vector4_f32[2] > V2.vector4_f32[2]) ? V1.vector4_f32[2] : V2.vector4_f32[2],
            (V1.vector4_f32[3] > V2.vector4_f32[3]) ? V1.vector4_f32[3] : V2.vector4_f32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vmaxq_f32(V1, V2);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_max_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

namespace Internal
{
    // Round to nearest (even) a.k.a. banker's rounding
    inline float round_to_nearest(float x) noexcept
    {
        float i = floorf(x);
        x -= i;
        if (x < 0.5f)
            return i;
        if (x > 0.5f)
            return i + 1.f;

        float int_part;
        (void)modff(i / 2.f, &int_part);
        if ((2.f * int_part) == i)
        {
            return i;
        }

        return i + 1.f;
    }
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(push)
#pragma float_control(precise, on)
#endif

inline XMVECTOR XM_CALLCONV XMVectorRound(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            Internal::round_to_nearest(V.vector4_f32[0]),
            Internal::round_to_nearest(V.vector4_f32[1]),
            Internal::round_to_nearest(V.vector4_f32[2]),
            Internal::round_to_nearest(V.vector4_f32[3])
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vrndnq_f32(V);
#else
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(V), g_XMNegativeZero);
    float32x4_t sMagic = vreinterpretq_f32_u32(vorrq_u32(g_XMNoFraction, sign));
    float32x4_t R1 = vaddq_f32(V, sMagic);
    R1 = vsubq_f32(R1, sMagic);
    float32x4_t R2 = vabsq_f32(V);
    uint32x4_t mask = vcleq_f32(R2, g_XMNoFraction);
    return vbslq_f32(mask, R1, V);
#endif
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_round_ps(V, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 sign = _mm_and_ps(V, g_XMNegativeZero);
    __m128 sMagic = _mm_or_ps(g_XMNoFraction, sign);
    __m128 R1 = _mm_add_ps(V, sMagic);
    R1 = _mm_sub_ps(R1, sMagic);
    __m128 R2 = _mm_and_ps(V, g_XMAbsMask);
    __m128 mask = _mm_cmple_ps(R2, g_XMNoFraction);
    R2 = _mm_andnot_ps(mask, V);
    R1 = _mm_and_ps(R1, mask);
    XMVECTOR vResult = _mm_xor_ps(R1, R2);
    return vResult;
#endif
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(pop)
#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorTruncate(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTOR Result;
    uint32_t     i;

    // Avoid C4701
    Result.vector4_f32[0] = 0.0f;

    for (i = 0; i < 4; i++)
    {
        if (XMISNAN(V.vector4_f32[i]))
        {
            Result.vector4_u32[i] = 0x7FC00000;
        }
        else if (fabsf(V.vector4_f32[i]) < 8388608.0f)
        {
            Result.vector4_f32[i] = static_cast<float>(static_cast<int32_t>(V.vector4_f32[i]));
        }
        else
        {
            Result.vector4_f32[i] = V.vector4_f32[i];
        }
    }
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vrndq_f32(V);
#else
    float32x4_t vTest = vabsq_f32(V);
    vTest = vreinterpretq_f32_u32(vcltq_f32(vTest, g_XMNoFraction));

    int32x4_t vInt = vcvtq_s32_f32(V);
    float32x4_t vResult = vcvtq_f32_s32(vInt);

    // All numbers less than 8388608 will use the round to int
    // All others, use the ORIGINAL value
    return vbslq_f32(vreinterpretq_u32_f32(vTest), vResult, V);
#endif
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_round_ps(V, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC);
#elif defined(_XM_SSE_INTRINSICS_)
    // To handle NAN, INF and numbers greater than 8388608, use masking
    // Get the abs value
    __m128i vTest = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    // Test for greater than 8388608 (All floats with NO fractionals, NAN and INF
    vTest = _mm_cmplt_epi32(vTest, g_XMNoFraction);
    // Convert to int and back to float for rounding with truncation
    __m128i vInt = _mm_cvttps_epi32(V);
    // Convert back to floats
    XMVECTOR vResult = _mm_cvtepi32_ps(vInt);
    // All numbers less than 8388608 will use the round to int
    vResult = _mm_and_ps(vResult, _mm_castsi128_ps(vTest));
    // All others, use the ORIGINAL value
    vTest = _mm_andnot_si128(vTest, _mm_castps_si128(V));
    vResult = _mm_or_ps(vResult, _mm_castsi128_ps(vTest));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorFloor(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            floorf(V.vector4_f32[0]),
            floorf(V.vector4_f32[1]),
            floorf(V.vector4_f32[2]),
            floorf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vrndmq_f32(V);
#else
    float32x4_t vTest = vabsq_f32(V);
    vTest = vreinterpretq_f32_u32(vcltq_f32(vTest, g_XMNoFraction));
    // Truncate
    int32x4_t vInt = vcvtq_s32_f32(V);
    float32x4_t vResult = vcvtq_f32_s32(vInt);
    uint32x4_t vLargerMask = vcgtq_f32(vResult, V);
    // 0 -> 0, 0xffffffff -> -1.0f
    float32x4_t vLarger = vcvtq_f32_s32(vreinterpretq_s32_u32(vLargerMask));
    vResult = vaddq_f32(vResult, vLarger);
    // All numbers less than 8388608 will use the round to int
    // All others, use the ORIGINAL value
    return vbslq_f32(vreinterpretq_u32_f32(vTest), vResult, V);
#endif
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_floor_ps(V);
#elif defined(_XM_SSE_INTRINSICS_)
    // To handle NAN, INF and numbers greater than 8388608, use masking
    __m128i vTest = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    vTest = _mm_cmplt_epi32(vTest, g_XMNoFraction);
    // Truncate
    __m128i vInt = _mm_cvttps_epi32(V);
    XMVECTOR vResult = _mm_cvtepi32_ps(vInt);
    __m128 vLarger = _mm_cmpgt_ps(vResult, V);
    // 0 -> 0, 0xffffffff -> -1.0f
    vLarger = _mm_cvtepi32_ps(_mm_castps_si128(vLarger));
    vResult = _mm_add_ps(vResult, vLarger);
    // All numbers less than 8388608 will use the round to int
    vResult = _mm_and_ps(vResult, _mm_castsi128_ps(vTest));
    // All others, use the ORIGINAL value
    vTest = _mm_andnot_si128(vTest, _mm_castps_si128(V));
    vResult = _mm_or_ps(vResult, _mm_castsi128_ps(vTest));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorCeiling(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            ceilf(V.vector4_f32[0]),
            ceilf(V.vector4_f32[1]),
            ceilf(V.vector4_f32[2]),
            ceilf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vrndpq_f32(V);
#else
    float32x4_t vTest = vabsq_f32(V);
    vTest = vreinterpretq_f32_u32(vcltq_f32(vTest, g_XMNoFraction));
    // Truncate
    int32x4_t vInt = vcvtq_s32_f32(V);
    float32x4_t vResult = vcvtq_f32_s32(vInt);
    uint32x4_t vSmallerMask = vcltq_f32(vResult, V);
    // 0 -> 0, 0xffffffff -> -1.0f
    float32x4_t vSmaller = vcvtq_f32_s32(vreinterpretq_s32_u32(vSmallerMask));
    vResult = vsubq_f32(vResult, vSmaller);
    // All numbers less than 8388608 will use the round to int
    // All others, use the ORIGINAL value
    return vbslq_f32(vreinterpretq_u32_f32(vTest), vResult, V);
#endif
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_ceil_ps(V);
#elif defined(_XM_SSE_INTRINSICS_)
    // To handle NAN, INF and numbers greater than 8388608, use masking
    __m128i vTest = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    vTest = _mm_cmplt_epi32(vTest, g_XMNoFraction);
    // Truncate
    __m128i vInt = _mm_cvttps_epi32(V);
    XMVECTOR vResult = _mm_cvtepi32_ps(vInt);
    __m128 vSmaller = _mm_cmplt_ps(vResult, V);
    // 0 -> 0, 0xffffffff -> -1.0f
    vSmaller = _mm_cvtepi32_ps(_mm_castps_si128(vSmaller));
    vResult = _mm_sub_ps(vResult, vSmaller);
    // All numbers less than 8388608 will use the round to int
    vResult = _mm_and_ps(vResult, _mm_castsi128_ps(vTest));
    // All others, use the ORIGINAL value
    vTest = _mm_andnot_si128(vTest, _mm_castps_si128(V));
    vResult = _mm_or_ps(vResult, _mm_castsi128_ps(vTest));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorClamp
(
    FXMVECTOR V,
    FXMVECTOR Min,
    FXMVECTOR Max
) noexcept
{
    assert(XMVector4LessOrEqual(Min, Max));

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVectorMax(Min, V);
    Result = XMVectorMin(Max, Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vResult = vmaxq_f32(Min, V);
    vResult = vminq_f32(Max, vResult);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult;
    vResult = _mm_max_ps(Min, V);
    vResult = _mm_min_ps(Max, vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSaturate(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    const XMVECTOR Zero = XMVectorZero();

    return XMVectorClamp(V, Zero, g_XMOne.v);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Set <0 to 0
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0));
    // Set>1 to 1
    return vminq_f32(vResult, vdupq_n_f32(1.0f));
#elif defined(_XM_SSE_INTRINSICS_)
    // Set <0 to 0
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    // Set>1 to 1
    return _mm_min_ps(vResult, g_XMOne);
#endif
}

//------------------------------------------------------------------------------
// Bitwise logical operations
//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorAndInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            V1.vector4_u32[0] & V2.vector4_u32[0],
            V1.vector4_u32[1] & V2.vector4_u32[1],
            V1.vector4_u32[2] & V2.vector4_u32[2],
            V1.vector4_u32[3] & V2.vector4_u32[3]
        } } };
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2)));
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_and_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorAndCInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            V1.vector4_u32[0] & ~V2.vector4_u32[0],
            V1.vector4_u32[1] & ~V2.vector4_u32[1],
            V1.vector4_u32[2] & ~V2.vector4_u32[2],
            V1.vector4_u32[3] & ~V2.vector4_u32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vbicq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2)));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_andnot_si128(_mm_castps_si128(V2), _mm_castps_si128(V1));
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorOrInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            V1.vector4_u32[0] | V2.vector4_u32[0],
            V1.vector4_u32[1] | V2.vector4_u32[1],
            V1.vector4_u32[2] | V2.vector4_u32[2],
            V1.vector4_u32[3] | V2.vector4_u32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(vorrq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2)));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_or_si128(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorNorInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            ~(V1.vector4_u32[0] | V2.vector4_u32[0]),
            ~(V1.vector4_u32[1] | V2.vector4_u32[1]),
            ~(V1.vector4_u32[2] | V2.vector4_u32[2]),
            ~(V1.vector4_u32[3] | V2.vector4_u32[3])
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t Result = vorrq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    return vreinterpretq_f32_u32(vbicq_u32(g_XMNegOneMask, Result));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i Result;
    Result = _mm_or_si128(_mm_castps_si128(V1), _mm_castps_si128(V2));
    Result = _mm_andnot_si128(Result, g_XMNegOneMask);
    return _mm_castsi128_ps(Result);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorXorInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORU32 Result = { { {
            V1.vector4_u32[0] ^ V2.vector4_u32[0],
            V1.vector4_u32[1] ^ V2.vector4_u32[1],
            V1.vector4_u32[2] ^ V2.vector4_u32[2],
            V1.vector4_u32[3] ^ V2.vector4_u32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vreinterpretq_f32_u32(veorq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2)));
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i V = _mm_xor_si128(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return _mm_castsi128_ps(V);
#endif
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorNegate(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            -V.vector4_f32[0],
            -V.vector4_f32[1],
            -V.vector4_f32[2],
            -V.vector4_f32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vnegq_f32(V);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR Z;

    Z = _mm_setzero_ps();

    return _mm_sub_ps(Z, V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorAdd
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            V1.vector4_f32[0] + V2.vector4_f32[0],
            V1.vector4_f32[1] + V2.vector4_f32[1],
            V1.vector4_f32[2] + V2.vector4_f32[2],
            V1.vector4_f32[3] + V2.vector4_f32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vaddq_f32(V1, V2);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_add_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSum(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result;
    Result.f[0] =
        Result.f[1] =
        Result.f[2] =
        Result.f[3] = V.vector4_f32[0] + V.vector4_f32[1] + V.vector4_f32[2] + V.vector4_f32[3];
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    float32x4_t vTemp = vpaddq_f32(V, V);
    return vpaddq_f32(vTemp, vTemp);
#else
    float32x2_t v1 = vget_low_f32(V);
    float32x2_t v2 = vget_high_f32(V);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    return vcombine_f32(v1, v1);
#endif
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vTemp = _mm_hadd_ps(V, V);
    return _mm_hadd_ps(vTemp, vTemp);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 3, 0, 1));
    XMVECTOR vTemp2 = _mm_add_ps(V, vTemp);
    vTemp = XM_PERMUTE_PS(vTemp2, _MM_SHUFFLE(1, 0, 3, 2));
    return _mm_add_ps(vTemp, vTemp2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorAddAngles
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    const XMVECTOR Zero = XMVectorZero();

    // Add the given angles together.  If the range of V1 is such
    // that -Pi <= V1 < Pi and the range of V2 is such that
    // -2Pi <= V2 <= 2Pi, then the range of the resulting angle
    // will be -Pi <= Result < Pi.
    XMVECTOR Result = XMVectorAdd(V1, V2);

    XMVECTOR Mask = XMVectorLess(Result, g_XMNegativePi.v);
    XMVECTOR Offset = XMVectorSelect(Zero, g_XMTwoPi.v, Mask);

    Mask = XMVectorGreaterOrEqual(Result, g_XMPi.v);
    Offset = XMVectorSelect(Offset, g_XMNegativeTwoPi.v, Mask);

    Result = XMVectorAdd(Result, Offset);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Adjust the angles
    float32x4_t vResult = vaddq_f32(V1, V2);
    // Less than Pi?
    uint32x4_t vOffset = vcltq_f32(vResult, g_XMNegativePi);
    vOffset = vandq_u32(vOffset, g_XMTwoPi);
    // Add 2Pi to all entries less than -Pi
    vResult = vaddq_f32(vResult, vreinterpretq_f32_u32(vOffset));
    // Greater than or equal to Pi?
    vOffset = vcgeq_f32(vResult, g_XMPi);
    vOffset = vandq_u32(vOffset, g_XMTwoPi);
    // Sub 2Pi to all entries greater than Pi
    vResult = vsubq_f32(vResult, vreinterpretq_f32_u32(vOffset));
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Adjust the angles
    XMVECTOR vResult = _mm_add_ps(V1, V2);
    // Less than Pi?
    XMVECTOR vOffset = _mm_cmplt_ps(vResult, g_XMNegativePi);
    vOffset = _mm_and_ps(vOffset, g_XMTwoPi);
    // Add 2Pi to all entries less than -Pi
    vResult = _mm_add_ps(vResult, vOffset);
    // Greater than or equal to Pi?
    vOffset = _mm_cmpge_ps(vResult, g_XMPi);
    vOffset = _mm_and_ps(vOffset, g_XMTwoPi);
    // Sub 2Pi to all entries greater than Pi
    vResult = _mm_sub_ps(vResult, vOffset);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSubtract
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            V1.vector4_f32[0] - V2.vector4_f32[0],
            V1.vector4_f32[1] - V2.vector4_f32[1],
            V1.vector4_f32[2] - V2.vector4_f32[2],
            V1.vector4_f32[3] - V2.vector4_f32[3]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vsubq_f32(V1, V2);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_sub_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSubtractAngles
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    const XMVECTOR Zero = XMVectorZero();

    // Subtract the given angles.  If the range of V1 is such
    // that -Pi <= V1 < Pi and the range of V2 is such that
    // -2Pi <= V2 <= 2Pi, then the range of the resulting angle
    // will be -Pi <= Result < Pi.
    XMVECTOR Result = XMVectorSubtract(V1, V2);

    XMVECTOR Mask = XMVectorLess(Result, g_XMNegativePi.v);
    XMVECTOR Offset = XMVectorSelect(Zero, g_XMTwoPi.v, Mask);

    Mask = XMVectorGreaterOrEqual(Result, g_XMPi.v);
    Offset = XMVectorSelect(Offset, g_XMNegativeTwoPi.v, Mask);

    Result = XMVectorAdd(Result, Offset);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Adjust the angles
    XMVECTOR vResult = vsubq_f32(V1, V2);
    // Less than Pi?
    uint32x4_t vOffset = vcltq_f32(vResult, g_XMNegativePi);
    vOffset = vandq_u32(vOffset, g_XMTwoPi);
    // Add 2Pi to all entries less than -Pi
    vResult = vaddq_f32(vResult, vreinterpretq_f32_u32(vOffset));
    // Greater than or equal to Pi?
    vOffset = vcgeq_f32(vResult, g_XMPi);
    vOffset = vandq_u32(vOffset, g_XMTwoPi);
    // Sub 2Pi to all entries greater than Pi
    vResult = vsubq_f32(vResult, vreinterpretq_f32_u32(vOffset));
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Adjust the angles
    XMVECTOR vResult = _mm_sub_ps(V1, V2);
    // Less than Pi?
    XMVECTOR vOffset = _mm_cmplt_ps(vResult, g_XMNegativePi);
    vOffset = _mm_and_ps(vOffset, g_XMTwoPi);
    // Add 2Pi to all entries less than -Pi
    vResult = _mm_add_ps(vResult, vOffset);
    // Greater than or equal to Pi?
    vOffset = _mm_cmpge_ps(vResult, g_XMPi);
    vOffset = _mm_and_ps(vOffset, g_XMTwoPi);
    // Sub 2Pi to all entries greater than Pi
    vResult = _mm_sub_ps(vResult, vOffset);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMultiply
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            V1.vector4_f32[0] * V2.vector4_f32[0],
            V1.vector4_f32[1] * V2.vector4_f32[1],
            V1.vector4_f32[2] * V2.vector4_f32[2],
            V1.vector4_f32[3] * V2.vector4_f32[3]
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vmulq_f32(V1, V2);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_mul_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMultiplyAdd
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR V3
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            V1.vector4_f32[0] * V2.vector4_f32[0] + V3.vector4_f32[0],
            V1.vector4_f32[1] * V2.vector4_f32[1] + V3.vector4_f32[1],
            V1.vector4_f32[2] * V2.vector4_f32[2] + V3.vector4_f32[2],
            V1.vector4_f32[3] * V2.vector4_f32[3] + V3.vector4_f32[3]
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vfmaq_f32(V3, V1, V2);
#else
    return vmlaq_f32(V3, V1, V2);
#endif
#elif defined(_XM_SSE_INTRINSICS_)
    return XM_FMADD_PS(V1, V2, V3);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorDivide
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            V1.vector4_f32[0] / V2.vector4_f32[0],
            V1.vector4_f32[1] / V2.vector4_f32[1],
            V1.vector4_f32[2] / V2.vector4_f32[2],
            V1.vector4_f32[3] / V2.vector4_f32[3]
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vdivq_f32(V1, V2);
#else
    // 2 iterations of Newton-Raphson refinement of reciprocal
    float32x4_t Reciprocal = vrecpeq_f32(V2);
    float32x4_t S = vrecpsq_f32(Reciprocal, V2);
    Reciprocal = vmulq_f32(S, Reciprocal);
    S = vrecpsq_f32(Reciprocal, V2);
    Reciprocal = vmulq_f32(S, Reciprocal);
    return vmulq_f32(V1, Reciprocal);
#endif
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_div_ps(V1, V2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorNegativeMultiplySubtract
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR V3
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            V3.vector4_f32[0] - (V1.vector4_f32[0] * V2.vector4_f32[0]),
            V3.vector4_f32[1] - (V1.vector4_f32[1] * V2.vector4_f32[1]),
            V3.vector4_f32[2] - (V1.vector4_f32[2] * V2.vector4_f32[2]),
            V3.vector4_f32[3] - (V1.vector4_f32[3] * V2.vector4_f32[3])
        } } };
    return Result;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    return vfmsq_f32(V3, V1, V2);
#else
    return vmlsq_f32(V3, V1, V2);
#endif
#elif defined(_XM_SSE_INTRINSICS_)
    return XM_FNMADD_PS(V1, V2, V3);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorScale
(
    FXMVECTOR V,
    float    ScaleFactor
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            V.vector4_f32[0] * ScaleFactor,
            V.vector4_f32[1] * ScaleFactor,
            V.vector4_f32[2] * ScaleFactor,
            V.vector4_f32[3] * ScaleFactor
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vmulq_n_f32(V, ScaleFactor);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_set_ps1(ScaleFactor);
    return _mm_mul_ps(vResult, V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorReciprocalEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            1.f / V.vector4_f32[0],
            1.f / V.vector4_f32[1],
            1.f / V.vector4_f32[2],
            1.f / V.vector4_f32[3]
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vrecpeq_f32(V);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_rcp_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorReciprocal(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            1.f / V.vector4_f32[0],
            1.f / V.vector4_f32[1],
            1.f / V.vector4_f32[2],
            1.f / V.vector4_f32[3]
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
    float32x4_t one = vdupq_n_f32(1.0f);
    return vdivq_f32(one, V);
#else
    // 2 iterations of Newton-Raphson refinement
    float32x4_t Reciprocal = vrecpeq_f32(V);
    float32x4_t S = vrecpsq_f32(Reciprocal, V);
    Reciprocal = vmulq_f32(S, Reciprocal);
    S = vrecpsq_f32(Reciprocal, V);
    return vmulq_f32(S, Reciprocal);
#endif
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_div_ps(g_XMOne, V);
#endif
}

//------------------------------------------------------------------------------
// Return an estimated square root
inline XMVECTOR XM_CALLCONV XMVectorSqrtEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            sqrtf(V.vector4_f32[0]),
            sqrtf(V.vector4_f32[1]),
            sqrtf(V.vector4_f32[2]),
            sqrtf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // 1 iteration of Newton-Raphson refinment of sqrt
    float32x4_t S0 = vrsqrteq_f32(V);
    float32x4_t P0 = vmulq_f32(V, S0);
    float32x4_t R0 = vrsqrtsq_f32(P0, S0);
    float32x4_t S1 = vmulq_f32(S0, R0);

    XMVECTOR VEqualsInfinity = XMVectorEqualInt(V, g_XMInfinity.v);
    XMVECTOR VEqualsZero = XMVectorEqual(V, vdupq_n_f32(0));
    XMVECTOR Result = vmulq_f32(V, S1);
    XMVECTOR Select = XMVectorEqualInt(VEqualsInfinity, VEqualsZero);
    return XMVectorSelect(V, Result, Select);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_sqrt_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSqrt(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            sqrtf(V.vector4_f32[0]),
            sqrtf(V.vector4_f32[1]),
            sqrtf(V.vector4_f32[2]),
            sqrtf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // 3 iterations of Newton-Raphson refinment of sqrt
    float32x4_t S0 = vrsqrteq_f32(V);
    float32x4_t P0 = vmulq_f32(V, S0);
    float32x4_t R0 = vrsqrtsq_f32(P0, S0);
    float32x4_t S1 = vmulq_f32(S0, R0);
    float32x4_t P1 = vmulq_f32(V, S1);
    float32x4_t R1 = vrsqrtsq_f32(P1, S1);
    float32x4_t S2 = vmulq_f32(S1, R1);
    float32x4_t P2 = vmulq_f32(V, S2);
    float32x4_t R2 = vrsqrtsq_f32(P2, S2);
    float32x4_t S3 = vmulq_f32(S2, R2);

    XMVECTOR VEqualsInfinity = XMVectorEqualInt(V, g_XMInfinity.v);
    XMVECTOR VEqualsZero = XMVectorEqual(V, vdupq_n_f32(0));
    XMVECTOR Result = vmulq_f32(V, S3);
    XMVECTOR Select = XMVectorEqualInt(VEqualsInfinity, VEqualsZero);
    return XMVectorSelect(V, Result, Select);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_sqrt_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorReciprocalSqrtEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            1.f / sqrtf(V.vector4_f32[0]),
            1.f / sqrtf(V.vector4_f32[1]),
            1.f / sqrtf(V.vector4_f32[2]),
            1.f / sqrtf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vrsqrteq_f32(V);
#elif defined(_XM_SSE_INTRINSICS_)
    return _mm_rsqrt_ps(V);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorReciprocalSqrt(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            1.f / sqrtf(V.vector4_f32[0]),
            1.f / sqrtf(V.vector4_f32[1]),
            1.f / sqrtf(V.vector4_f32[2]),
            1.f / sqrtf(V.vector4_f32[3])
        } } };
    return Result;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // 2 iterations of Newton-Raphson refinement of reciprocal
    float32x4_t S0 = vrsqrteq_f32(V);

    float32x4_t P0 = vmulq_f32(V, S0);
    float32x4_t R0 = vrsqrtsq_f32(P0, S0);

    float32x4_t S1 = vmulq_f32(S0, R0);
    float32x4_t P1 = vmulq_f32(V, S1);
    float32x4_t R1 = vrsqrtsq_f32(P1, S1);

    return vmulq_f32(S1, R1);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_sqrt_ps(V);
    vResult = _mm_div_ps(g_XMOne, vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorExp2(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            powf(2.0f, V.vector4_f32[0]),
            powf(2.0f, V.vector4_f32[1]),
            powf(2.0f, V.vector4_f32[2]),
            powf(2.0f, V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    int32x4_t itrunc = vcvtq_s32_f32(V);
    float32x4_t ftrunc = vcvtq_f32_s32(itrunc);
    float32x4_t y = vsubq_f32(V, ftrunc);

    float32x4_t poly = vmlaq_f32(g_XMExpEst6, g_XMExpEst7, y);
    poly = vmlaq_f32(g_XMExpEst5, poly, y);
    poly = vmlaq_f32(g_XMExpEst4, poly, y);
    poly = vmlaq_f32(g_XMExpEst3, poly, y);
    poly = vmlaq_f32(g_XMExpEst2, poly, y);
    poly = vmlaq_f32(g_XMExpEst1, poly, y);
    poly = vmlaq_f32(g_XMOne, poly, y);

    int32x4_t biased = vaddq_s32(itrunc, g_XMExponentBias);
    biased = vshlq_n_s32(biased, 23);
    float32x4_t result0 = XMVectorDivide(vreinterpretq_f32_s32(biased), poly);

    biased = vaddq_s32(itrunc, g_XM253);
    biased = vshlq_n_s32(biased, 23);
    float32x4_t result1 = XMVectorDivide(vreinterpretq_f32_s32(biased), poly);
    result1 = vmulq_f32(g_XMMinNormal.v, result1);

    // Use selection to handle the cases
    //  if (V is NaN) -> QNaN;
    //  else if (V sign bit set)
    //      if (V > -150)
    //         if (V.exponent < -126) -> result1
    //         else -> result0
    //      else -> +0
    //  else
    //      if (V < 128) -> result0
    //      else -> +inf

    uint32x4_t comp = vcltq_s32(vreinterpretq_s32_f32(V), g_XMBin128);
    float32x4_t result2 = vbslq_f32(comp, result0, g_XMInfinity);

    comp = vcltq_s32(itrunc, g_XMSubnormalExponent);
    float32x4_t result3 = vbslq_f32(comp, result1, result0);

    comp = vcltq_s32(vreinterpretq_s32_f32(V), g_XMBinNeg150);
    float32x4_t result4 = vbslq_f32(comp, result3, g_XMZero);

    int32x4_t sign = vandq_s32(vreinterpretq_s32_f32(V), g_XMNegativeZero);
    comp = vceqq_s32(sign, g_XMNegativeZero);
    float32x4_t result5 = vbslq_f32(comp, result4, result2);

    int32x4_t t0 = vandq_s32(vreinterpretq_s32_f32(V), g_XMQNaNTest);
    int32x4_t t1 = vandq_s32(vreinterpretq_s32_f32(V), g_XMInfinity);
    t0 = vreinterpretq_s32_u32(vceqq_s32(t0, g_XMZero));
    t1 = vreinterpretq_s32_u32(vceqq_s32(t1, g_XMInfinity));
    int32x4_t isNaN = vbicq_s32(t1, t0);

    float32x4_t vResult = vbslq_f32(vreinterpretq_u32_s32(isNaN), g_XMQNaN, result5);
    return vResult;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_exp2_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i itrunc = _mm_cvttps_epi32(V);
    __m128 ftrunc = _mm_cvtepi32_ps(itrunc);
    __m128 y = _mm_sub_ps(V, ftrunc);

    __m128 poly = XM_FMADD_PS(g_XMExpEst7, y, g_XMExpEst6);
    poly = XM_FMADD_PS(poly, y, g_XMExpEst5);
    poly = XM_FMADD_PS(poly, y, g_XMExpEst4);
    poly = XM_FMADD_PS(poly, y, g_XMExpEst3);
    poly = XM_FMADD_PS(poly, y, g_XMExpEst2);
    poly = XM_FMADD_PS(poly, y, g_XMExpEst1);
    poly = XM_FMADD_PS(poly, y, g_XMOne);

    __m128i biased = _mm_add_epi32(itrunc, g_XMExponentBias);
    biased = _mm_slli_epi32(biased, 23);
    __m128 result0 = _mm_div_ps(_mm_castsi128_ps(biased), poly);

    biased = _mm_add_epi32(itrunc, g_XM253);
    biased = _mm_slli_epi32(biased, 23);
    __m128 result1 = _mm_div_ps(_mm_castsi128_ps(biased), poly);
    result1 = _mm_mul_ps(g_XMMinNormal.v, result1);

    // Use selection to handle the cases
    //  if (V is NaN) -> QNaN;
    //  else if (V sign bit set)
    //      if (V > -150)
    //         if (V.exponent < -126) -> result1
    //         else -> result0
    //      else -> +0
    //  else
    //      if (V < 128) -> result0
    //      else -> +inf

    __m128i comp = _mm_cmplt_epi32(_mm_castps_si128(V), g_XMBin128);
    __m128i select0 = _mm_and_si128(comp, _mm_castps_si128(result0));
    __m128i select1 = _mm_andnot_si128(comp, g_XMInfinity);
    __m128i result2 = _mm_or_si128(select0, select1);

    comp = _mm_cmplt_epi32(itrunc, g_XMSubnormalExponent);
    select1 = _mm_and_si128(comp, _mm_castps_si128(result1));
    select0 = _mm_andnot_si128(comp, _mm_castps_si128(result0));
    __m128i result3 = _mm_or_si128(select0, select1);

    comp = _mm_cmplt_epi32(_mm_castps_si128(V), g_XMBinNeg150);
    select0 = _mm_and_si128(comp, result3);
    select1 = _mm_andnot_si128(comp, g_XMZero);
    __m128i result4 = _mm_or_si128(select0, select1);

    __m128i sign = _mm_and_si128(_mm_castps_si128(V), g_XMNegativeZero);
    comp = _mm_cmpeq_epi32(sign, g_XMNegativeZero);
    select0 = _mm_and_si128(comp, result4);
    select1 = _mm_andnot_si128(comp, result2);
    __m128i result5 = _mm_or_si128(select0, select1);

    __m128i t0 = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i t1 = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    t0 = _mm_cmpeq_epi32(t0, g_XMZero);
    t1 = _mm_cmpeq_epi32(t1, g_XMInfinity);
    __m128i isNaN = _mm_andnot_si128(t0, t1);

    select0 = _mm_and_si128(isNaN, g_XMQNaN);
    select1 = _mm_andnot_si128(isNaN, result5);
    __m128i vResult = _mm_or_si128(select0, select1);

    return _mm_castsi128_ps(vResult);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorExp10(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            powf(10.0f, V.vector4_f32[0]),
            powf(10.0f, V.vector4_f32[1]),
            powf(10.0f, V.vector4_f32[2]),
            powf(10.0f, V.vector4_f32[3])
        } } };
    return Result.v;

#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_exp10_ps(V);
    return Result;
#else
    // exp10(V) = exp2(vin*log2(10))
    XMVECTOR Vten = XMVectorMultiply(g_XMLg10, V);
    return XMVectorExp2(Vten);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorExpE(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            expf(V.vector4_f32[0]),
            expf(V.vector4_f32[1]),
            expf(V.vector4_f32[2]),
            expf(V.vector4_f32[3])
        } } };
    return Result.v;

#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_exp_ps(V);
    return Result;
#else
    // expE(V) = exp2(vin*log2(e))
    XMVECTOR Ve = XMVectorMultiply(g_XMLgE, V);
    return XMVectorExp2(Ve);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorExp(FXMVECTOR V) noexcept
{
    return XMVectorExp2(V);
}

//------------------------------------------------------------------------------

#if defined(_XM_SSE_INTRINSICS_)

namespace Internal
{
    inline __m128i multi_sll_epi32(__m128i value, __m128i count) noexcept
    {
        __m128i v = _mm_shuffle_epi32(value, _MM_SHUFFLE(0, 0, 0, 0));
        __m128i c = _mm_shuffle_epi32(count, _MM_SHUFFLE(0, 0, 0, 0));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r0 = _mm_sll_epi32(v, c);

        v = _mm_shuffle_epi32(value, _MM_SHUFFLE(1, 1, 1, 1));
        c = _mm_shuffle_epi32(count, _MM_SHUFFLE(1, 1, 1, 1));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r1 = _mm_sll_epi32(v, c);

        v = _mm_shuffle_epi32(value, _MM_SHUFFLE(2, 2, 2, 2));
        c = _mm_shuffle_epi32(count, _MM_SHUFFLE(2, 2, 2, 2));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r2 = _mm_sll_epi32(v, c);

        v = _mm_shuffle_epi32(value, _MM_SHUFFLE(3, 3, 3, 3));
        c = _mm_shuffle_epi32(count, _MM_SHUFFLE(3, 3, 3, 3));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r3 = _mm_sll_epi32(v, c);

        // (r0,r0,r1,r1)
        __m128 r01 = _mm_shuffle_ps(_mm_castsi128_ps(r0), _mm_castsi128_ps(r1), _MM_SHUFFLE(0, 0, 0, 0));
        // (r2,r2,r3,r3)
        __m128 r23 = _mm_shuffle_ps(_mm_castsi128_ps(r2), _mm_castsi128_ps(r3), _MM_SHUFFLE(0, 0, 0, 0));
        // (r0,r1,r2,r3)
        __m128 result = _mm_shuffle_ps(r01, r23, _MM_SHUFFLE(2, 0, 2, 0));
        return _mm_castps_si128(result);
    }

    inline __m128i multi_srl_epi32(__m128i value, __m128i count) noexcept
    {
        __m128i v = _mm_shuffle_epi32(value, _MM_SHUFFLE(0, 0, 0, 0));
        __m128i c = _mm_shuffle_epi32(count, _MM_SHUFFLE(0, 0, 0, 0));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r0 = _mm_srl_epi32(v, c);

        v = _mm_shuffle_epi32(value, _MM_SHUFFLE(1, 1, 1, 1));
        c = _mm_shuffle_epi32(count, _MM_SHUFFLE(1, 1, 1, 1));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r1 = _mm_srl_epi32(v, c);

        v = _mm_shuffle_epi32(value, _MM_SHUFFLE(2, 2, 2, 2));
        c = _mm_shuffle_epi32(count, _MM_SHUFFLE(2, 2, 2, 2));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r2 = _mm_srl_epi32(v, c);

        v = _mm_shuffle_epi32(value, _MM_SHUFFLE(3, 3, 3, 3));
        c = _mm_shuffle_epi32(count, _MM_SHUFFLE(3, 3, 3, 3));
        c = _mm_and_si128(c, g_XMMaskX);
        __m128i r3 = _mm_srl_epi32(v, c);

        // (r0,r0,r1,r1)
        __m128 r01 = _mm_shuffle_ps(_mm_castsi128_ps(r0), _mm_castsi128_ps(r1), _MM_SHUFFLE(0, 0, 0, 0));
        // (r2,r2,r3,r3)
        __m128 r23 = _mm_shuffle_ps(_mm_castsi128_ps(r2), _mm_castsi128_ps(r3), _MM_SHUFFLE(0, 0, 0, 0));
        // (r0,r1,r2,r3)
        __m128 result = _mm_shuffle_ps(r01, r23, _MM_SHUFFLE(2, 0, 2, 0));
        return _mm_castps_si128(result);
    }

    inline __m128i GetLeadingBit(const __m128i value) noexcept
    {
        static const XMVECTORI32 g_XM0000FFFF = { { { 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF } } };
        static const XMVECTORI32 g_XM000000FF = { { { 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF } } };
        static const XMVECTORI32 g_XM0000000F = { { { 0x0000000F, 0x0000000F, 0x0000000F, 0x0000000F } } };
        static const XMVECTORI32 g_XM00000003 = { { { 0x00000003, 0x00000003, 0x00000003, 0x00000003 } } };

        __m128i v = value, r, c, b, s;

        c = _mm_cmpgt_epi32(v, g_XM0000FFFF);   // c = (v > 0xFFFF)
        b = _mm_srli_epi32(c, 31);              // b = (c ? 1 : 0)
        r = _mm_slli_epi32(b, 4);               // r = (b << 4)
        v = multi_srl_epi32(v, r);              // v = (v >> r)

        c = _mm_cmpgt_epi32(v, g_XM000000FF);   // c = (v > 0xFF)
        b = _mm_srli_epi32(c, 31);              // b = (c ? 1 : 0)
        s = _mm_slli_epi32(b, 3);               // s = (b << 3)
        v = multi_srl_epi32(v, s);              // v = (v >> s)
        r = _mm_or_si128(r, s);                 // r = (r | s)

        c = _mm_cmpgt_epi32(v, g_XM0000000F);   // c = (v > 0xF)
        b = _mm_srli_epi32(c, 31);              // b = (c ? 1 : 0)
        s = _mm_slli_epi32(b, 2);               // s = (b << 2)
        v = multi_srl_epi32(v, s);              // v = (v >> s)
        r = _mm_or_si128(r, s);                 // r = (r | s)

        c = _mm_cmpgt_epi32(v, g_XM00000003);   // c = (v > 0x3)
        b = _mm_srli_epi32(c, 31);              // b = (c ? 1 : 0)
        s = _mm_slli_epi32(b, 1);               // s = (b << 1)
        v = multi_srl_epi32(v, s);              // v = (v >> s)
        r = _mm_or_si128(r, s);                 // r = (r | s)

        s = _mm_srli_epi32(v, 1);
        r = _mm_or_si128(r, s);
        return r;
    }
} // namespace Internal

#endif // _XM_SSE_INTRINSICS_

#if defined(_XM_ARM_NEON_INTRINSICS_)

namespace Internal
{
    inline int32x4_t GetLeadingBit(const int32x4_t value) noexcept
    {
        static const XMVECTORI32 g_XM0000FFFF = { { { 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF } } };
        static const XMVECTORI32 g_XM000000FF = { { { 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF } } };
        static const XMVECTORI32 g_XM0000000F = { { { 0x0000000F, 0x0000000F, 0x0000000F, 0x0000000F } } };
        static const XMVECTORI32 g_XM00000003 = { { { 0x00000003, 0x00000003, 0x00000003, 0x00000003 } } };

        uint32x4_t c = vcgtq_s32(value, g_XM0000FFFF);              // c = (v > 0xFFFF)
        int32x4_t b = vshrq_n_s32(vreinterpretq_s32_u32(c), 31);    // b = (c ? 1 : 0)
        int32x4_t r = vshlq_n_s32(b, 4);                            // r = (b << 4)
        r = vnegq_s32(r);
        int32x4_t v = vshlq_s32(value, r);                          // v = (v >> r)

        c = vcgtq_s32(v, g_XM000000FF);                             // c = (v > 0xFF)
        b = vshrq_n_s32(vreinterpretq_s32_u32(c), 31);              // b = (c ? 1 : 0)
        int32x4_t s = vshlq_n_s32(b, 3);                            // s = (b << 3)
        s = vnegq_s32(s);
        v = vshlq_s32(v, s);                                        // v = (v >> s)
        r = vorrq_s32(r, s);                                        // r = (r | s)

        c = vcgtq_s32(v, g_XM0000000F);                             // c = (v > 0xF)
        b = vshrq_n_s32(vreinterpretq_s32_u32(c), 31);              // b = (c ? 1 : 0)
        s = vshlq_n_s32(b, 2);                                      // s = (b << 2)
        s = vnegq_s32(s);
        v = vshlq_s32(v, s);                                        // v = (v >> s)
        r = vorrq_s32(r, s);                                        // r = (r | s)

        c = vcgtq_s32(v, g_XM00000003);                             // c = (v > 0x3)
        b = vshrq_n_s32(vreinterpretq_s32_u32(c), 31);              // b = (c ? 1 : 0)
        s = vshlq_n_s32(b, 1);                                      // s = (b << 1)
        s = vnegq_s32(s);
        v = vshlq_s32(v, s);                                        // v = (v >> s)
        r = vorrq_s32(r, s);                                        // r = (r | s)

        s = vshrq_n_s32(v, 1);
        r = vorrq_s32(r, s);
        return r;
    }

} // namespace Internal

#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLog2(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    const float fScale = 1.4426950f; // (1.0f / logf(2.0f));

    XMVECTORF32 Result = { { {
            logf(V.vector4_f32[0]) * fScale,
            logf(V.vector4_f32[1]) * fScale,
            logf(V.vector4_f32[2]) * fScale,
            logf(V.vector4_f32[3]) * fScale
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    int32x4_t rawBiased = vandq_s32(vreinterpretq_s32_f32(V), g_XMInfinity);
    int32x4_t trailing = vandq_s32(vreinterpretq_s32_f32(V), g_XMQNaNTest);
    uint32x4_t isExponentZero = vceqq_s32(vreinterpretq_s32_f32(g_XMZero), rawBiased);

    // Compute exponent and significand for normals.
    int32x4_t biased = vshrq_n_s32(rawBiased, 23);
    int32x4_t exponentNor = vsubq_s32(biased, g_XMExponentBias);
    int32x4_t trailingNor = trailing;

    // Compute exponent and significand for subnormals.
    int32x4_t leading = Internal::GetLeadingBit(trailing);
    int32x4_t shift = vsubq_s32(g_XMNumTrailing, leading);
    int32x4_t exponentSub = vsubq_s32(g_XMSubnormalExponent, shift);
    int32x4_t trailingSub = vshlq_s32(trailing, shift);
    trailingSub = vandq_s32(trailingSub, g_XMQNaNTest);
    int32x4_t e = vbslq_s32(isExponentZero, exponentSub, exponentNor);
    int32x4_t t = vbslq_s32(isExponentZero, trailingSub, trailingNor);

    // Compute the approximation.
    int32x4_t tmp = vorrq_s32(vreinterpretq_s32_f32(g_XMOne), t);
    float32x4_t y = vsubq_f32(vreinterpretq_f32_s32(tmp), g_XMOne);

    float32x4_t log2 = vmlaq_f32(g_XMLogEst6, g_XMLogEst7, y);
    log2 = vmlaq_f32(g_XMLogEst5, log2, y);
    log2 = vmlaq_f32(g_XMLogEst4, log2, y);
    log2 = vmlaq_f32(g_XMLogEst3, log2, y);
    log2 = vmlaq_f32(g_XMLogEst2, log2, y);
    log2 = vmlaq_f32(g_XMLogEst1, log2, y);
    log2 = vmlaq_f32(g_XMLogEst0, log2, y);
    log2 = vmlaq_f32(vcvtq_f32_s32(e), log2, y);

    //  if (x is NaN) -> QNaN
    //  else if (V is positive)
    //      if (V is infinite) -> +inf
    //      else -> log2(V)
    //  else
    //      if (V is zero) -> -inf
    //      else -> -QNaN

    uint32x4_t isInfinite = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    isInfinite = vceqq_u32(isInfinite, g_XMInfinity);

    uint32x4_t isGreaterZero = vcgtq_f32(V, g_XMZero);
    uint32x4_t isNotFinite = vcgtq_f32(V, g_XMInfinity);
    uint32x4_t isPositive = vbicq_u32(isGreaterZero, isNotFinite);

    uint32x4_t isZero = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    isZero = vceqq_u32(isZero, g_XMZero);

    uint32x4_t t0 = vandq_u32(vreinterpretq_u32_f32(V), g_XMQNaNTest);
    uint32x4_t t1 = vandq_u32(vreinterpretq_u32_f32(V), g_XMInfinity);
    t0 = vceqq_u32(t0, g_XMZero);
    t1 = vceqq_u32(t1, g_XMInfinity);
    uint32x4_t isNaN = vbicq_u32(t1, t0);

    float32x4_t result = vbslq_f32(isInfinite, g_XMInfinity, log2);
    float32x4_t tmp2 = vbslq_f32(isZero, g_XMNegInfinity, g_XMNegQNaN);
    result = vbslq_f32(isPositive, result, tmp2);
    result = vbslq_f32(isNaN, g_XMQNaN, result);
    return result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_log2_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i rawBiased = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    __m128i trailing = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i isExponentZero = _mm_cmpeq_epi32(g_XMZero, rawBiased);

    // Compute exponent and significand for normals.
    __m128i biased = _mm_srli_epi32(rawBiased, 23);
    __m128i exponentNor = _mm_sub_epi32(biased, g_XMExponentBias);
    __m128i trailingNor = trailing;

    // Compute exponent and significand for subnormals.
    __m128i leading = Internal::GetLeadingBit(trailing);
    __m128i shift = _mm_sub_epi32(g_XMNumTrailing, leading);
    __m128i exponentSub = _mm_sub_epi32(g_XMSubnormalExponent, shift);
    __m128i trailingSub = Internal::multi_sll_epi32(trailing, shift);
    trailingSub = _mm_and_si128(trailingSub, g_XMQNaNTest);

    __m128i select0 = _mm_and_si128(isExponentZero, exponentSub);
    __m128i select1 = _mm_andnot_si128(isExponentZero, exponentNor);
    __m128i e = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isExponentZero, trailingSub);
    select1 = _mm_andnot_si128(isExponentZero, trailingNor);
    __m128i t = _mm_or_si128(select0, select1);

    // Compute the approximation.
    __m128i tmp = _mm_or_si128(g_XMOne, t);
    __m128 y = _mm_sub_ps(_mm_castsi128_ps(tmp), g_XMOne);

    __m128 log2 = XM_FMADD_PS(g_XMLogEst7, y, g_XMLogEst6);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst5);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst4);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst3);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst2);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst1);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst0);
    log2 = XM_FMADD_PS(log2, y, _mm_cvtepi32_ps(e));

    //  if (x is NaN) -> QNaN
    //  else if (V is positive)
    //      if (V is infinite) -> +inf
    //      else -> log2(V)
    //  else
    //      if (V is zero) -> -inf
    //      else -> -QNaN

    __m128i isInfinite = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    isInfinite = _mm_cmpeq_epi32(isInfinite, g_XMInfinity);

    __m128i isGreaterZero = _mm_cmpgt_epi32(_mm_castps_si128(V), g_XMZero);
    __m128i isNotFinite = _mm_cmpgt_epi32(_mm_castps_si128(V), g_XMInfinity);
    __m128i isPositive = _mm_andnot_si128(isNotFinite, isGreaterZero);

    __m128i isZero = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    isZero = _mm_cmpeq_epi32(isZero, g_XMZero);

    __m128i t0 = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i t1 = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    t0 = _mm_cmpeq_epi32(t0, g_XMZero);
    t1 = _mm_cmpeq_epi32(t1, g_XMInfinity);
    __m128i isNaN = _mm_andnot_si128(t0, t1);

    select0 = _mm_and_si128(isInfinite, g_XMInfinity);
    select1 = _mm_andnot_si128(isInfinite, _mm_castps_si128(log2));
    __m128i result = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isZero, g_XMNegInfinity);
    select1 = _mm_andnot_si128(isZero, g_XMNegQNaN);
    tmp = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isPositive, result);
    select1 = _mm_andnot_si128(isPositive, tmp);
    result = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isNaN, g_XMQNaN);
    select1 = _mm_andnot_si128(isNaN, result);
    result = _mm_or_si128(select0, select1);

    return _mm_castsi128_ps(result);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLog10(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            log10f(V.vector4_f32[0]),
            log10f(V.vector4_f32[1]),
            log10f(V.vector4_f32[2]),
            log10f(V.vector4_f32[3])
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    int32x4_t rawBiased = vandq_s32(vreinterpretq_s32_f32(V), g_XMInfinity);
    int32x4_t trailing = vandq_s32(vreinterpretq_s32_f32(V), g_XMQNaNTest);
    uint32x4_t isExponentZero = vceqq_s32(g_XMZero, rawBiased);

    // Compute exponent and significand for normals.
    int32x4_t biased = vshrq_n_s32(rawBiased, 23);
    int32x4_t exponentNor = vsubq_s32(biased, g_XMExponentBias);
    int32x4_t trailingNor = trailing;

    // Compute exponent and significand for subnormals.
    int32x4_t leading = Internal::GetLeadingBit(trailing);
    int32x4_t shift = vsubq_s32(g_XMNumTrailing, leading);
    int32x4_t exponentSub = vsubq_s32(g_XMSubnormalExponent, shift);
    int32x4_t trailingSub = vshlq_s32(trailing, shift);
    trailingSub = vandq_s32(trailingSub, g_XMQNaNTest);
    int32x4_t e = vbslq_s32(isExponentZero, exponentSub, exponentNor);
    int32x4_t t = vbslq_s32(isExponentZero, trailingSub, trailingNor);

    // Compute the approximation.
    int32x4_t tmp = vorrq_s32(g_XMOne, t);
    float32x4_t y = vsubq_f32(vreinterpretq_f32_s32(tmp), g_XMOne);

    float32x4_t log2 = vmlaq_f32(g_XMLogEst6, g_XMLogEst7, y);
    log2 = vmlaq_f32(g_XMLogEst5, log2, y);
    log2 = vmlaq_f32(g_XMLogEst4, log2, y);
    log2 = vmlaq_f32(g_XMLogEst3, log2, y);
    log2 = vmlaq_f32(g_XMLogEst2, log2, y);
    log2 = vmlaq_f32(g_XMLogEst1, log2, y);
    log2 = vmlaq_f32(g_XMLogEst0, log2, y);
    log2 = vmlaq_f32(vcvtq_f32_s32(e), log2, y);

    log2 = vmulq_f32(g_XMInvLg10, log2);

    //  if (x is NaN) -> QNaN
    //  else if (V is positive)
    //      if (V is infinite) -> +inf
    //      else -> log2(V)
    //  else
    //      if (V is zero) -> -inf
    //      else -> -QNaN

    uint32x4_t isInfinite = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    isInfinite = vceqq_u32(isInfinite, g_XMInfinity);

    uint32x4_t isGreaterZero = vcgtq_s32(vreinterpretq_s32_f32(V), g_XMZero);
    uint32x4_t isNotFinite = vcgtq_s32(vreinterpretq_s32_f32(V), g_XMInfinity);
    uint32x4_t isPositive = vbicq_u32(isGreaterZero, isNotFinite);

    uint32x4_t isZero = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    isZero = vceqq_u32(isZero, g_XMZero);

    uint32x4_t t0 = vandq_u32(vreinterpretq_u32_f32(V), g_XMQNaNTest);
    uint32x4_t t1 = vandq_u32(vreinterpretq_u32_f32(V), g_XMInfinity);
    t0 = vceqq_u32(t0, g_XMZero);
    t1 = vceqq_u32(t1, g_XMInfinity);
    uint32x4_t isNaN = vbicq_u32(t1, t0);

    float32x4_t result = vbslq_f32(isInfinite, g_XMInfinity, log2);
    float32x4_t tmp2 = vbslq_f32(isZero, g_XMNegInfinity, g_XMNegQNaN);
    result = vbslq_f32(isPositive, result, tmp2);
    result = vbslq_f32(isNaN, g_XMQNaN, result);
    return result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_log10_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i rawBiased = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    __m128i trailing = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i isExponentZero = _mm_cmpeq_epi32(g_XMZero, rawBiased);

    // Compute exponent and significand for normals.
    __m128i biased = _mm_srli_epi32(rawBiased, 23);
    __m128i exponentNor = _mm_sub_epi32(biased, g_XMExponentBias);
    __m128i trailingNor = trailing;

    // Compute exponent and significand for subnormals.
    __m128i leading = Internal::GetLeadingBit(trailing);
    __m128i shift = _mm_sub_epi32(g_XMNumTrailing, leading);
    __m128i exponentSub = _mm_sub_epi32(g_XMSubnormalExponent, shift);
    __m128i trailingSub = Internal::multi_sll_epi32(trailing, shift);
    trailingSub = _mm_and_si128(trailingSub, g_XMQNaNTest);

    __m128i select0 = _mm_and_si128(isExponentZero, exponentSub);
    __m128i select1 = _mm_andnot_si128(isExponentZero, exponentNor);
    __m128i e = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isExponentZero, trailingSub);
    select1 = _mm_andnot_si128(isExponentZero, trailingNor);
    __m128i t = _mm_or_si128(select0, select1);

    // Compute the approximation.
    __m128i tmp = _mm_or_si128(g_XMOne, t);
    __m128 y = _mm_sub_ps(_mm_castsi128_ps(tmp), g_XMOne);

    __m128 log2 = XM_FMADD_PS(g_XMLogEst7, y, g_XMLogEst6);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst5);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst4);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst3);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst2);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst1);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst0);
    log2 = XM_FMADD_PS(log2, y, _mm_cvtepi32_ps(e));

    log2 = _mm_mul_ps(g_XMInvLg10, log2);

    //  if (x is NaN) -> QNaN
    //  else if (V is positive)
    //      if (V is infinite) -> +inf
    //      else -> log2(V)
    //  else
    //      if (V is zero) -> -inf
    //      else -> -QNaN

    __m128i isInfinite = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    isInfinite = _mm_cmpeq_epi32(isInfinite, g_XMInfinity);

    __m128i isGreaterZero = _mm_cmpgt_epi32(_mm_castps_si128(V), g_XMZero);
    __m128i isNotFinite = _mm_cmpgt_epi32(_mm_castps_si128(V), g_XMInfinity);
    __m128i isPositive = _mm_andnot_si128(isNotFinite, isGreaterZero);

    __m128i isZero = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    isZero = _mm_cmpeq_epi32(isZero, g_XMZero);

    __m128i t0 = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i t1 = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    t0 = _mm_cmpeq_epi32(t0, g_XMZero);
    t1 = _mm_cmpeq_epi32(t1, g_XMInfinity);
    __m128i isNaN = _mm_andnot_si128(t0, t1);

    select0 = _mm_and_si128(isInfinite, g_XMInfinity);
    select1 = _mm_andnot_si128(isInfinite, _mm_castps_si128(log2));
    __m128i result = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isZero, g_XMNegInfinity);
    select1 = _mm_andnot_si128(isZero, g_XMNegQNaN);
    tmp = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isPositive, result);
    select1 = _mm_andnot_si128(isPositive, tmp);
    result = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isNaN, g_XMQNaN);
    select1 = _mm_andnot_si128(isNaN, result);
    result = _mm_or_si128(select0, select1);

    return _mm_castsi128_ps(result);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLogE(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            logf(V.vector4_f32[0]),
            logf(V.vector4_f32[1]),
            logf(V.vector4_f32[2]),
            logf(V.vector4_f32[3])
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    int32x4_t rawBiased = vandq_s32(vreinterpretq_s32_f32(V), g_XMInfinity);
    int32x4_t trailing = vandq_s32(vreinterpretq_s32_f32(V), g_XMQNaNTest);
    uint32x4_t isExponentZero = vceqq_s32(g_XMZero, rawBiased);

    // Compute exponent and significand for normals.
    int32x4_t biased = vshrq_n_s32(rawBiased, 23);
    int32x4_t exponentNor = vsubq_s32(biased, g_XMExponentBias);
    int32x4_t trailingNor = trailing;

    // Compute exponent and significand for subnormals.
    int32x4_t leading = Internal::GetLeadingBit(trailing);
    int32x4_t shift = vsubq_s32(g_XMNumTrailing, leading);
    int32x4_t exponentSub = vsubq_s32(g_XMSubnormalExponent, shift);
    int32x4_t trailingSub = vshlq_s32(trailing, shift);
    trailingSub = vandq_s32(trailingSub, g_XMQNaNTest);
    int32x4_t e = vbslq_s32(isExponentZero, exponentSub, exponentNor);
    int32x4_t t = vbslq_s32(isExponentZero, trailingSub, trailingNor);

    // Compute the approximation.
    int32x4_t tmp = vorrq_s32(g_XMOne, t);
    float32x4_t y = vsubq_f32(vreinterpretq_f32_s32(tmp), g_XMOne);

    float32x4_t log2 = vmlaq_f32(g_XMLogEst6, g_XMLogEst7, y);
    log2 = vmlaq_f32(g_XMLogEst5, log2, y);
    log2 = vmlaq_f32(g_XMLogEst4, log2, y);
    log2 = vmlaq_f32(g_XMLogEst3, log2, y);
    log2 = vmlaq_f32(g_XMLogEst2, log2, y);
    log2 = vmlaq_f32(g_XMLogEst1, log2, y);
    log2 = vmlaq_f32(g_XMLogEst0, log2, y);
    log2 = vmlaq_f32(vcvtq_f32_s32(e), log2, y);

    log2 = vmulq_f32(g_XMInvLgE, log2);

    //  if (x is NaN) -> QNaN
    //  else if (V is positive)
    //      if (V is infinite) -> +inf
    //      else -> log2(V)
    //  else
    //      if (V is zero) -> -inf
    //      else -> -QNaN

    uint32x4_t isInfinite = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    isInfinite = vceqq_u32(isInfinite, g_XMInfinity);

    uint32x4_t isGreaterZero = vcgtq_s32(vreinterpretq_s32_f32(V), g_XMZero);
    uint32x4_t isNotFinite = vcgtq_s32(vreinterpretq_s32_f32(V), g_XMInfinity);
    uint32x4_t isPositive = vbicq_u32(isGreaterZero, isNotFinite);

    uint32x4_t isZero = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    isZero = vceqq_u32(isZero, g_XMZero);

    uint32x4_t t0 = vandq_u32(vreinterpretq_u32_f32(V), g_XMQNaNTest);
    uint32x4_t t1 = vandq_u32(vreinterpretq_u32_f32(V), g_XMInfinity);
    t0 = vceqq_u32(t0, g_XMZero);
    t1 = vceqq_u32(t1, g_XMInfinity);
    uint32x4_t isNaN = vbicq_u32(t1, t0);

    float32x4_t result = vbslq_f32(isInfinite, g_XMInfinity, log2);
    float32x4_t tmp2 = vbslq_f32(isZero, g_XMNegInfinity, g_XMNegQNaN);
    result = vbslq_f32(isPositive, result, tmp2);
    result = vbslq_f32(isNaN, g_XMQNaN, result);
    return result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_log_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i rawBiased = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    __m128i trailing = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i isExponentZero = _mm_cmpeq_epi32(g_XMZero, rawBiased);

    // Compute exponent and significand for normals.
    __m128i biased = _mm_srli_epi32(rawBiased, 23);
    __m128i exponentNor = _mm_sub_epi32(biased, g_XMExponentBias);
    __m128i trailingNor = trailing;

    // Compute exponent and significand for subnormals.
    __m128i leading = Internal::GetLeadingBit(trailing);
    __m128i shift = _mm_sub_epi32(g_XMNumTrailing, leading);
    __m128i exponentSub = _mm_sub_epi32(g_XMSubnormalExponent, shift);
    __m128i trailingSub = Internal::multi_sll_epi32(trailing, shift);
    trailingSub = _mm_and_si128(trailingSub, g_XMQNaNTest);

    __m128i select0 = _mm_and_si128(isExponentZero, exponentSub);
    __m128i select1 = _mm_andnot_si128(isExponentZero, exponentNor);
    __m128i e = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isExponentZero, trailingSub);
    select1 = _mm_andnot_si128(isExponentZero, trailingNor);
    __m128i t = _mm_or_si128(select0, select1);

    // Compute the approximation.
    __m128i tmp = _mm_or_si128(g_XMOne, t);
    __m128 y = _mm_sub_ps(_mm_castsi128_ps(tmp), g_XMOne);

    __m128 log2 = XM_FMADD_PS(g_XMLogEst7, y, g_XMLogEst6);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst5);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst4);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst3);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst2);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst1);
    log2 = XM_FMADD_PS(log2, y, g_XMLogEst0);
    log2 = XM_FMADD_PS(log2, y, _mm_cvtepi32_ps(e));

    log2 = _mm_mul_ps(g_XMInvLgE, log2);

    //  if (x is NaN) -> QNaN
    //  else if (V is positive)
    //      if (V is infinite) -> +inf
    //      else -> log2(V)
    //  else
    //      if (V is zero) -> -inf
    //      else -> -QNaN

    __m128i isInfinite = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    isInfinite = _mm_cmpeq_epi32(isInfinite, g_XMInfinity);

    __m128i isGreaterZero = _mm_cmpgt_epi32(_mm_castps_si128(V), g_XMZero);
    __m128i isNotFinite = _mm_cmpgt_epi32(_mm_castps_si128(V), g_XMInfinity);
    __m128i isPositive = _mm_andnot_si128(isNotFinite, isGreaterZero);

    __m128i isZero = _mm_and_si128(_mm_castps_si128(V), g_XMAbsMask);
    isZero = _mm_cmpeq_epi32(isZero, g_XMZero);

    __m128i t0 = _mm_and_si128(_mm_castps_si128(V), g_XMQNaNTest);
    __m128i t1 = _mm_and_si128(_mm_castps_si128(V), g_XMInfinity);
    t0 = _mm_cmpeq_epi32(t0, g_XMZero);
    t1 = _mm_cmpeq_epi32(t1, g_XMInfinity);
    __m128i isNaN = _mm_andnot_si128(t0, t1);

    select0 = _mm_and_si128(isInfinite, g_XMInfinity);
    select1 = _mm_andnot_si128(isInfinite, _mm_castps_si128(log2));
    __m128i result = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isZero, g_XMNegInfinity);
    select1 = _mm_andnot_si128(isZero, g_XMNegQNaN);
    tmp = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isPositive, result);
    select1 = _mm_andnot_si128(isPositive, tmp);
    result = _mm_or_si128(select0, select1);

    select0 = _mm_and_si128(isNaN, g_XMQNaN);
    select1 = _mm_andnot_si128(isNaN, result);
    result = _mm_or_si128(select0, select1);

    return _mm_castsi128_ps(result);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLog(FXMVECTOR V) noexcept
{
    return XMVectorLog2(V);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorPow
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            powf(V1.vector4_f32[0], V2.vector4_f32[0]),
            powf(V1.vector4_f32[1], V2.vector4_f32[1]),
            powf(V1.vector4_f32[2], V2.vector4_f32[2]),
            powf(V1.vector4_f32[3], V2.vector4_f32[3])
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            powf(vgetq_lane_f32(V1, 0), vgetq_lane_f32(V2, 0)),
            powf(vgetq_lane_f32(V1, 1), vgetq_lane_f32(V2, 1)),
            powf(vgetq_lane_f32(V1, 2), vgetq_lane_f32(V2, 2)),
            powf(vgetq_lane_f32(V1, 3), vgetq_lane_f32(V2, 3))
        } } };
    return vResult.v;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_pow_ps(V1, V2);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    XM_ALIGNED_DATA(16) float a[4];
    XM_ALIGNED_DATA(16) float b[4];
    _mm_store_ps(a, V1);
    _mm_store_ps(b, V2);
    XMVECTOR vResult = _mm_setr_ps(
        powf(a[0], b[0]),
        powf(a[1], b[1]),
        powf(a[2], b[2]),
        powf(a[3], b[3]));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorAbs(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            fabsf(V.vector4_f32[0]),
            fabsf(V.vector4_f32[1]),
            fabsf(V.vector4_f32[2]),
            fabsf(V.vector4_f32[3])
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    return vabsq_f32(V);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_setzero_ps();
    vResult = _mm_sub_ps(vResult, V);
    vResult = _mm_max_ps(vResult, V);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorMod
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    // V1 % V2 = V1 - V2 * truncate(V1 / V2)

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Quotient = XMVectorDivide(V1, V2);
    Quotient = XMVectorTruncate(Quotient);
    XMVECTOR Result = XMVectorNegativeMultiplySubtract(V2, Quotient, V1);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR vResult = XMVectorDivide(V1, V2);
    vResult = XMVectorTruncate(vResult);
    return vmlsq_f32(V1, vResult, V2);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_div_ps(V1, V2);
    vResult = XMVectorTruncate(vResult);
    return XM_FNMADD_PS(vResult, V2, V1);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorModAngles(FXMVECTOR Angles) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR V;
    XMVECTOR Result;

    // Modulo the range of the given angles such that -XM_PI <= Angles < XM_PI
    V = XMVectorMultiply(Angles, g_XMReciprocalTwoPi.v);
    V = XMVectorRound(V);
    Result = XMVectorNegativeMultiplySubtract(g_XMTwoPi.v, V, Angles);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Modulo the range of the given angles such that -XM_PI <= Angles < XM_PI
    XMVECTOR vResult = vmulq_f32(Angles, g_XMReciprocalTwoPi);
    // Use the inline function due to complexity for rounding
    vResult = XMVectorRound(vResult);
    return vmlsq_f32(Angles, vResult, g_XMTwoPi);
#elif defined(_XM_SSE_INTRINSICS_)
    // Modulo the range of the given angles such that -XM_PI <= Angles < XM_PI
    XMVECTOR vResult = _mm_mul_ps(Angles, g_XMReciprocalTwoPi);
    // Use the inline function due to complexity for rounding
    vResult = XMVectorRound(vResult);
    return XM_FNMADD_PS(vResult, g_XMTwoPi, Angles);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSin(FXMVECTOR V) noexcept
{
    // 11-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            sinf(V.vector4_f32[0]),
            sinf(V.vector4_f32[1]),
            sinf(V.vector4_f32[2]),
            sinf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(x), g_XMNegativeZero);
    uint32x4_t c = vorrq_u32(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    float32x4_t absx = vabsq_f32(x);
    float32x4_t rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    uint32x4_t comp = vcleq_f32(absx, g_XMHalfPi);
    x = vbslq_f32(comp, x, rflx);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const XMVECTOR SC1 = g_XMSinCoefficients1;
    const XMVECTOR SC0 = g_XMSinCoefficients0;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(SC0), 1);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_low_f32(SC1), 0);

    vConstants = vdupq_lane_f32(vget_high_f32(SC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(SC0), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(SC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    Result = vmulq_f32(Result, x);
    return Result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_sin_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    __m128 sign = _mm_and_ps(x, g_XMNegativeZero);
    __m128 c = _mm_or_ps(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    __m128 absx = _mm_andnot_ps(sign, x);  // |x|
    __m128 rflx = _mm_sub_ps(c, x);
    __m128 comp = _mm_cmple_ps(absx, g_XMHalfPi);
    __m128 select0 = _mm_and_ps(comp, x);
    __m128 select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const XMVECTOR SC1 = g_XMSinCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(SC1, _MM_SHUFFLE(0, 0, 0, 0));
    const XMVECTOR SC0 = g_XMSinCoefficients0;
    __m128 vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(2, 2, 2, 2));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, x);
    return Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorCos(FXMVECTOR V) noexcept
{
    // 10-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            cosf(V.vector4_f32[0]),
            cosf(V.vector4_f32[1]),
            cosf(V.vector4_f32[2]),
            cosf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Map V to x in [-pi,pi].
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(x), g_XMNegativeZero);
    uint32x4_t c = vorrq_u32(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    float32x4_t absx = vabsq_f32(x);
    float32x4_t rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    uint32x4_t comp = vcleq_f32(absx, g_XMHalfPi);
    x = vbslq_f32(comp, x, rflx);
    float32x4_t fsign = vbslq_f32(comp, g_XMOne, g_XMNegativeOne);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const XMVECTOR CC1 = g_XMCosCoefficients1;
    const XMVECTOR CC0 = g_XMCosCoefficients0;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(CC0), 1);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_low_f32(CC1), 0);

    vConstants = vdupq_lane_f32(vget_high_f32(CC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(CC0), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(CC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    Result = vmulq_f32(Result, fsign);
    return Result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_cos_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    // Map V to x in [-pi,pi].
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
    XMVECTOR sign = _mm_and_ps(x, g_XMNegativeZero);
    __m128 c = _mm_or_ps(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    __m128 absx = _mm_andnot_ps(sign, x);  // |x|
    __m128 rflx = _mm_sub_ps(c, x);
    __m128 comp = _mm_cmple_ps(absx, g_XMHalfPi);
    __m128 select0 = _mm_and_ps(comp, x);
    __m128 select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);
    select0 = _mm_and_ps(comp, g_XMOne);
    select1 = _mm_andnot_ps(comp, g_XMNegativeOne);
    sign = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const XMVECTOR CC1 = g_XMCosCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(CC1, _MM_SHUFFLE(0, 0, 0, 0));
    const XMVECTOR CC0 = g_XMCosCoefficients0;
    __m128 vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(2, 2, 2, 2));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, sign);
    return Result;
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline void XM_CALLCONV XMVectorSinCos
(
    XMVECTOR* pSin,
    XMVECTOR* pCos,
    FXMVECTOR V
) noexcept
{
    assert(pSin != nullptr);
    assert(pCos != nullptr);

    // 11/10-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Sin = { { {
            sinf(V.vector4_f32[0]),
            sinf(V.vector4_f32[1]),
            sinf(V.vector4_f32[2]),
            sinf(V.vector4_f32[3])
        } } };

    XMVECTORF32 Cos = { { {
            cosf(V.vector4_f32[0]),
            cosf(V.vector4_f32[1]),
            cosf(V.vector4_f32[2]),
            cosf(V.vector4_f32[3])
        } } };

    *pSin = Sin.v;
    *pCos = Cos.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(x), g_XMNegativeZero);
    uint32x4_t c = vorrq_u32(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    float32x4_t absx = vabsq_f32(x);
    float32x4_t  rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    uint32x4_t comp = vcleq_f32(absx, g_XMHalfPi);
    x = vbslq_f32(comp, x, rflx);
    float32x4_t fsign = vbslq_f32(comp, g_XMOne, g_XMNegativeOne);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation for sine
    const XMVECTOR SC1 = g_XMSinCoefficients1;
    const XMVECTOR SC0 = g_XMSinCoefficients0;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(SC0), 1);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_low_f32(SC1), 0);

    vConstants = vdupq_lane_f32(vget_high_f32(SC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(SC0), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(SC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    *pSin = vmulq_f32(Result, x);

    // Compute polynomial approximation for cosine
    const XMVECTOR CC1 = g_XMCosCoefficients1;
    const XMVECTOR CC0 = g_XMCosCoefficients0;
    vConstants = vdupq_lane_f32(vget_high_f32(CC0), 1);
    Result = vmlaq_lane_f32(vConstants, x2, vget_low_f32(CC1), 0);

    vConstants = vdupq_lane_f32(vget_high_f32(CC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(CC0), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(CC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    *pCos = vmulq_f32(Result, fsign);
#elif defined(_XM_SVML_INTRINSICS_)
    *pSin = _mm_sincos_ps(pCos, V);
#elif defined(_XM_SSE_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x), cos(y) = sign*cos(x).
    XMVECTOR sign = _mm_and_ps(x, g_XMNegativeZero);
    __m128 c = _mm_or_ps(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    __m128 absx = _mm_andnot_ps(sign, x);  // |x|
    __m128 rflx = _mm_sub_ps(c, x);
    __m128 comp = _mm_cmple_ps(absx, g_XMHalfPi);
    __m128 select0 = _mm_and_ps(comp, x);
    __m128 select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);
    select0 = _mm_and_ps(comp, g_XMOne);
    select1 = _mm_andnot_ps(comp, g_XMNegativeOne);
    sign = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation of sine
    const XMVECTOR SC1 = g_XMSinCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(SC1, _MM_SHUFFLE(0, 0, 0, 0));
    const XMVECTOR SC0 = g_XMSinCoefficients0;
    __m128 vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(2, 2, 2, 2));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SC0, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, x);
    *pSin = Result;

    // Compute polynomial approximation of cosine
    const XMVECTOR CC1 = g_XMCosCoefficients1;
    vConstantsB = XM_PERMUTE_PS(CC1, _MM_SHUFFLE(0, 0, 0, 0));
    const XMVECTOR CC0 = g_XMCosCoefficients0;
    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(3, 3, 3, 3));
    Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(2, 2, 2, 2));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CC0, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, sign);
    *pCos = Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorTan(FXMVECTOR V) noexcept
{
    // Cody and Waite algorithm to compute tangent.

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            tanf(V.vector4_f32[0]),
            tanf(V.vector4_f32[1]),
            tanf(V.vector4_f32[2]),
            tanf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_tan_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    static const XMVECTORF32 TanCoefficients0 = { { { 1.0f, -4.667168334e-1f, 2.566383229e-2f, -3.118153191e-4f } } };
    static const XMVECTORF32 TanCoefficients1 = { { { 4.981943399e-7f, -1.333835001e-1f, 3.424887824e-3f, -1.786170734e-5f } } };
    static const XMVECTORF32 TanConstants = { { { 1.570796371f, 6.077100628e-11f, 0.000244140625f, 0.63661977228f /*2 / Pi*/ } } };
    static const XMVECTORU32 Mask = { { { 0x1, 0x1, 0x1, 0x1 } } };

    XMVECTOR TwoDivPi = XMVectorSplatW(TanConstants.v);

    XMVECTOR Zero = XMVectorZero();

    XMVECTOR C0 = XMVectorSplatX(TanConstants.v);
    XMVECTOR C1 = XMVectorSplatY(TanConstants.v);
    XMVECTOR Epsilon = XMVectorSplatZ(TanConstants.v);

    XMVECTOR VA = XMVectorMultiply(V, TwoDivPi);

    VA = XMVectorRound(VA);

    XMVECTOR VC = XMVectorNegativeMultiplySubtract(VA, C0, V);

    XMVECTOR VB = XMVectorAbs(VA);

    VC = XMVectorNegativeMultiplySubtract(VA, C1, VC);

#if defined(_XM_ARM_NEON_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    VB = vreinterpretq_f32_u32(vcvtq_u32_f32(VB));
#elif defined(_XM_SSE_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    reinterpret_cast<__m128i*>(&VB)[0] = _mm_cvttps_epi32(VB);
#else
    for (size_t i = 0; i < 4; i++)
    {
        VB.vector4_u32[i] = static_cast<uint32_t>(VB.vector4_f32[i]);
    }
#endif

    XMVECTOR VC2 = XMVectorMultiply(VC, VC);

    XMVECTOR T7 = XMVectorSplatW(TanCoefficients1.v);
    XMVECTOR T6 = XMVectorSplatZ(TanCoefficients1.v);
    XMVECTOR T4 = XMVectorSplatX(TanCoefficients1.v);
    XMVECTOR T3 = XMVectorSplatW(TanCoefficients0.v);
    XMVECTOR T5 = XMVectorSplatY(TanCoefficients1.v);
    XMVECTOR T2 = XMVectorSplatZ(TanCoefficients0.v);
    XMVECTOR T1 = XMVectorSplatY(TanCoefficients0.v);
    XMVECTOR T0 = XMVectorSplatX(TanCoefficients0.v);

    XMVECTOR VBIsEven = XMVectorAndInt(VB, Mask.v);
    VBIsEven = XMVectorEqualInt(VBIsEven, Zero);

    XMVECTOR N = XMVectorMultiplyAdd(VC2, T7, T6);
    XMVECTOR D = XMVectorMultiplyAdd(VC2, T4, T3);
    N = XMVectorMultiplyAdd(VC2, N, T5);
    D = XMVectorMultiplyAdd(VC2, D, T2);
    N = XMVectorMultiply(VC2, N);
    D = XMVectorMultiplyAdd(VC2, D, T1);
    N = XMVectorMultiplyAdd(VC, N, VC);
    XMVECTOR VCNearZero = XMVectorInBounds(VC, Epsilon);
    D = XMVectorMultiplyAdd(VC2, D, T0);

    N = XMVectorSelect(N, VC, VCNearZero);
    D = XMVectorSelect(D, g_XMOne.v, VCNearZero);

    XMVECTOR R0 = XMVectorNegate(N);
    XMVECTOR R1 = XMVectorDivide(N, D);
    R0 = XMVectorDivide(D, R0);

    XMVECTOR VIsZero = XMVectorEqual(V, Zero);

    XMVECTOR Result = XMVectorSelect(R0, R1, VBIsEven);

    Result = XMVectorSelect(Result, Zero, VIsZero);

    return Result;

#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSinH(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            sinhf(V.vector4_f32[0]),
            sinhf(V.vector4_f32[1]),
            sinhf(V.vector4_f32[2]),
            sinhf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.442695040888963f, 1.442695040888963f, 1.442695040888963f, 1.442695040888963f } } }; // 1.0f / ln(2.0f)

    XMVECTOR V1 = vmlaq_f32(g_XMNegativeOne.v, V, Scale.v);
    XMVECTOR V2 = vmlsq_f32(g_XMNegativeOne.v, V, Scale.v);
    XMVECTOR E1 = XMVectorExp(V1);
    XMVECTOR E2 = XMVectorExp(V2);

    return vsubq_f32(E1, E2);
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_sinh_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.442695040888963f, 1.442695040888963f, 1.442695040888963f, 1.442695040888963f } } }; // 1.0f / ln(2.0f)

    XMVECTOR V1 = XM_FMADD_PS(V, Scale, g_XMNegativeOne);
    XMVECTOR V2 = XM_FNMADD_PS(V, Scale, g_XMNegativeOne);
    XMVECTOR E1 = XMVectorExp(V1);
    XMVECTOR E2 = XMVectorExp(V2);

    return _mm_sub_ps(E1, E2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorCosH(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            coshf(V.vector4_f32[0]),
            coshf(V.vector4_f32[1]),
            coshf(V.vector4_f32[2]),
            coshf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.442695040888963f, 1.442695040888963f, 1.442695040888963f, 1.442695040888963f } } }; // 1.0f / ln(2.0f)

    XMVECTOR V1 = vmlaq_f32(g_XMNegativeOne.v, V, Scale.v);
    XMVECTOR V2 = vmlsq_f32(g_XMNegativeOne.v, V, Scale.v);
    XMVECTOR E1 = XMVectorExp(V1);
    XMVECTOR E2 = XMVectorExp(V2);
    return vaddq_f32(E1, E2);
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_cosh_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.442695040888963f, 1.442695040888963f, 1.442695040888963f, 1.442695040888963f } } }; // 1.0f / ln(2.0f)

    XMVECTOR V1 = XM_FMADD_PS(V, Scale.v, g_XMNegativeOne.v);
    XMVECTOR V2 = XM_FNMADD_PS(V, Scale.v, g_XMNegativeOne.v);
    XMVECTOR E1 = XMVectorExp(V1);
    XMVECTOR E2 = XMVectorExp(V2);
    return _mm_add_ps(E1, E2);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorTanH(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            tanhf(V.vector4_f32[0]),
            tanhf(V.vector4_f32[1]),
            tanhf(V.vector4_f32[2]),
            tanhf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 2.8853900817779268f, 2.8853900817779268f, 2.8853900817779268f, 2.8853900817779268f } } }; // 2.0f / ln(2.0f)

    XMVECTOR E = vmulq_f32(V, Scale.v);
    E = XMVectorExp(E);
    E = vmlaq_f32(g_XMOneHalf.v, E, g_XMOneHalf.v);
    E = XMVectorReciprocal(E);
    return vsubq_f32(g_XMOne.v, E);
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_tanh_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 2.8853900817779268f, 2.8853900817779268f, 2.8853900817779268f, 2.8853900817779268f } } }; // 2.0f / ln(2.0f)

    XMVECTOR E = _mm_mul_ps(V, Scale.v);
    E = XMVectorExp(E);
    E = XM_FMADD_PS(E, g_XMOneHalf.v, g_XMOneHalf.v);
    E = _mm_div_ps(g_XMOne.v, E);
    return _mm_sub_ps(g_XMOne.v, E);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorASin(FXMVECTOR V) noexcept
{
    // 7-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            asinf(V.vector4_f32[0]),
            asinf(V.vector4_f32[1]),
            asinf(V.vector4_f32[2]),
            asinf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t nonnegative = vcgeq_f32(V, g_XMZero);
    float32x4_t x = vabsq_f32(V);

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    float32x4_t oneMValue = vsubq_f32(g_XMOne, x);
    float32x4_t clampOneMValue = vmaxq_f32(g_XMZero, oneMValue);
    float32x4_t root = XMVectorSqrt(clampOneMValue);

    // Compute polynomial approximation
    const XMVECTOR AC1 = g_XMArcCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(AC1), 0);
    XMVECTOR t0 = vmlaq_lane_f32(vConstants, x, vget_high_f32(AC1), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(AC1), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AC1), 0);
    t0 = vmlaq_f32(vConstants, t0, x);

    const XMVECTOR AC0 = g_XMArcCoefficients0;
    vConstants = vdupq_lane_f32(vget_high_f32(AC0), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_high_f32(AC0), 0);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AC0), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AC0), 0);
    t0 = vmlaq_f32(vConstants, t0, x);
    t0 = vmulq_f32(t0, root);

    float32x4_t t1 = vsubq_f32(g_XMPi, t0);
    t0 = vbslq_f32(nonnegative, t0, t1);
    t0 = vsubq_f32(g_XMHalfPi, t0);
    return t0;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_asin_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 nonnegative = _mm_cmpge_ps(V, g_XMZero);
    __m128 mvalue = _mm_sub_ps(g_XMZero, V);
    __m128 x = _mm_max_ps(V, mvalue);  // |V|

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    __m128 oneMValue = _mm_sub_ps(g_XMOne, x);
    __m128 clampOneMValue = _mm_max_ps(g_XMZero, oneMValue);
    __m128 root = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

    // Compute polynomial approximation
    const XMVECTOR AC1 = g_XMArcCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 t0 = XM_FMADD_PS(vConstantsB, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(1, 1, 1, 1));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(0, 0, 0, 0));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    const XMVECTOR AC0 = g_XMArcCoefficients0;
    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(3, 3, 3, 3));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(2, 2, 2, 2));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(1, 1, 1, 1));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(0, 0, 0, 0));
    t0 = XM_FMADD_PS(t0, x, vConstants);
    t0 = _mm_mul_ps(t0, root);

    __m128 t1 = _mm_sub_ps(g_XMPi, t0);
    t0 = _mm_and_ps(nonnegative, t0);
    t1 = _mm_andnot_ps(nonnegative, t1);
    t0 = _mm_or_ps(t0, t1);
    t0 = _mm_sub_ps(g_XMHalfPi, t0);
    return t0;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorACos(FXMVECTOR V) noexcept
{
    // 7-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            acosf(V.vector4_f32[0]),
            acosf(V.vector4_f32[1]),
            acosf(V.vector4_f32[2]),
            acosf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t nonnegative = vcgeq_f32(V, g_XMZero);
    float32x4_t x = vabsq_f32(V);

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    float32x4_t oneMValue = vsubq_f32(g_XMOne, x);
    float32x4_t clampOneMValue = vmaxq_f32(g_XMZero, oneMValue);
    float32x4_t root = XMVectorSqrt(clampOneMValue);

    // Compute polynomial approximation
    const XMVECTOR AC1 = g_XMArcCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(AC1), 0);
    XMVECTOR t0 = vmlaq_lane_f32(vConstants, x, vget_high_f32(AC1), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(AC1), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AC1), 0);
    t0 = vmlaq_f32(vConstants, t0, x);

    const XMVECTOR AC0 = g_XMArcCoefficients0;
    vConstants = vdupq_lane_f32(vget_high_f32(AC0), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_high_f32(AC0), 0);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AC0), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AC0), 0);
    t0 = vmlaq_f32(vConstants, t0, x);
    t0 = vmulq_f32(t0, root);

    float32x4_t t1 = vsubq_f32(g_XMPi, t0);
    t0 = vbslq_f32(nonnegative, t0, t1);
    return t0;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_acos_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 nonnegative = _mm_cmpge_ps(V, g_XMZero);
    __m128 mvalue = _mm_sub_ps(g_XMZero, V);
    __m128 x = _mm_max_ps(V, mvalue);  // |V|

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    __m128 oneMValue = _mm_sub_ps(g_XMOne, x);
    __m128 clampOneMValue = _mm_max_ps(g_XMZero, oneMValue);
    __m128 root = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

    // Compute polynomial approximation
    const XMVECTOR AC1 = g_XMArcCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 t0 = XM_FMADD_PS(vConstantsB, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(1, 1, 1, 1));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC1, _MM_SHUFFLE(0, 0, 0, 0));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    const XMVECTOR AC0 = g_XMArcCoefficients0;
    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(3, 3, 3, 3));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(2, 2, 2, 2));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(1, 1, 1, 1));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AC0, _MM_SHUFFLE(0, 0, 0, 0));
    t0 = XM_FMADD_PS(t0, x, vConstants);
    t0 = _mm_mul_ps(t0, root);

    __m128 t1 = _mm_sub_ps(g_XMPi, t0);
    t0 = _mm_and_ps(nonnegative, t0);
    t1 = _mm_andnot_ps(nonnegative, t1);
    t0 = _mm_or_ps(t0, t1);
    return t0;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorATan(FXMVECTOR V) noexcept
{
    // 17-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            atanf(V.vector4_f32[0]),
            atanf(V.vector4_f32[1]),
            atanf(V.vector4_f32[2]),
            atanf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t absV = vabsq_f32(V);
    float32x4_t invV = XMVectorReciprocal(V);
    uint32x4_t comp = vcgtq_f32(V, g_XMOne);
    float32x4_t sign = vbslq_f32(comp, g_XMOne, g_XMNegativeOne);
    comp = vcleq_f32(absV, g_XMOne);
    sign = vbslq_f32(comp, g_XMZero, sign);
    float32x4_t x = vbslq_f32(comp, V, invV);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const XMVECTOR TC1 = g_XMATanCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(TC1), 0);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_high_f32(TC1), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(TC1), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(TC1), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    const XMVECTOR TC0 = g_XMATanCoefficients0;
    vConstants = vdupq_lane_f32(vget_high_f32(TC0), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_high_f32(TC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(TC0), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(TC0), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    Result = vmulq_f32(Result, x);

    float32x4_t result1 = vmulq_f32(sign, g_XMHalfPi);
    result1 = vsubq_f32(result1, Result);

    comp = vceqq_f32(sign, g_XMZero);
    Result = vbslq_f32(comp, Result, result1);
    return Result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_atan_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 absV = XMVectorAbs(V);
    __m128 invV = _mm_div_ps(g_XMOne, V);
    __m128 comp = _mm_cmpgt_ps(V, g_XMOne);
    __m128 select0 = _mm_and_ps(comp, g_XMOne);
    __m128 select1 = _mm_andnot_ps(comp, g_XMNegativeOne);
    __m128 sign = _mm_or_ps(select0, select1);
    comp = _mm_cmple_ps(absV, g_XMOne);
    select0 = _mm_and_ps(comp, g_XMZero);
    select1 = _mm_andnot_ps(comp, sign);
    sign = _mm_or_ps(select0, select1);
    select0 = _mm_and_ps(comp, V);
    select1 = _mm_andnot_ps(comp, invV);
    __m128 x = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const XMVECTOR TC1 = g_XMATanCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(TC1, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(TC1, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(TC1, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(TC1, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    const XMVECTOR TC0 = g_XMATanCoefficients0;
    vConstants = XM_PERMUTE_PS(TC0, _MM_SHUFFLE(3, 3, 3, 3));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(TC0, _MM_SHUFFLE(2, 2, 2, 2));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(TC0, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(TC0, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    Result = XM_FMADD_PS(Result, x2, g_XMOne);

    Result = _mm_mul_ps(Result, x);
    __m128 result1 = _mm_mul_ps(sign, g_XMHalfPi);
    result1 = _mm_sub_ps(result1, Result);

    comp = _mm_cmpeq_ps(sign, g_XMZero);
    select0 = _mm_and_ps(comp, Result);
    select1 = _mm_andnot_ps(comp, result1);
    Result = _mm_or_ps(select0, select1);
    return Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorATan2
(
    FXMVECTOR Y,
    FXMVECTOR X
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            atan2f(Y.vector4_f32[0], X.vector4_f32[0]),
            atan2f(Y.vector4_f32[1], X.vector4_f32[1]),
            atan2f(Y.vector4_f32[2], X.vector4_f32[2]),
            atan2f(Y.vector4_f32[3], X.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_atan2_ps(Y, X);
    return Result;
#else

    // Return the inverse tangent of Y / X in the range of -Pi to Pi with the following exceptions:

    //     Y == 0 and X is Negative         -> Pi with the sign of Y
    //     y == 0 and x is positive         -> 0 with the sign of y
    //     Y != 0 and X == 0                -> Pi / 2 with the sign of Y
    //     Y != 0 and X is Negative         -> atan(y/x) + (PI with the sign of Y)
    //     X == -Infinity and Finite Y      -> Pi with the sign of Y
    //     X == +Infinity and Finite Y      -> 0 with the sign of Y
    //     Y == Infinity and X is Finite    -> Pi / 2 with the sign of Y
    //     Y == Infinity and X == -Infinity -> 3Pi / 4 with the sign of Y
    //     Y == Infinity and X == +Infinity -> Pi / 4 with the sign of Y

    static const XMVECTORF32 ATan2Constants = { { { XM_PI, XM_PIDIV2, XM_PIDIV4, XM_PI * 3.0f / 4.0f } } };

    XMVECTOR Zero = XMVectorZero();
    XMVECTOR ATanResultValid = XMVectorTrueInt();

    XMVECTOR Pi = XMVectorSplatX(ATan2Constants);
    XMVECTOR PiOverTwo = XMVectorSplatY(ATan2Constants);
    XMVECTOR PiOverFour = XMVectorSplatZ(ATan2Constants);
    XMVECTOR ThreePiOverFour = XMVectorSplatW(ATan2Constants);

    XMVECTOR YEqualsZero = XMVectorEqual(Y, Zero);
    XMVECTOR XEqualsZero = XMVectorEqual(X, Zero);
    XMVECTOR XIsPositive = XMVectorAndInt(X, g_XMNegativeZero.v);
    XIsPositive = XMVectorEqualInt(XIsPositive, Zero);
    XMVECTOR YEqualsInfinity = XMVectorIsInfinite(Y);
    XMVECTOR XEqualsInfinity = XMVectorIsInfinite(X);

    XMVECTOR YSign = XMVectorAndInt(Y, g_XMNegativeZero.v);
    Pi = XMVectorOrInt(Pi, YSign);
    PiOverTwo = XMVectorOrInt(PiOverTwo, YSign);
    PiOverFour = XMVectorOrInt(PiOverFour, YSign);
    ThreePiOverFour = XMVectorOrInt(ThreePiOverFour, YSign);

    XMVECTOR R1 = XMVectorSelect(Pi, YSign, XIsPositive);
    XMVECTOR R2 = XMVectorSelect(ATanResultValid, PiOverTwo, XEqualsZero);
    XMVECTOR R3 = XMVectorSelect(R2, R1, YEqualsZero);
    XMVECTOR R4 = XMVectorSelect(ThreePiOverFour, PiOverFour, XIsPositive);
    XMVECTOR R5 = XMVectorSelect(PiOverTwo, R4, XEqualsInfinity);
    XMVECTOR Result = XMVectorSelect(R3, R5, YEqualsInfinity);
    ATanResultValid = XMVectorEqualInt(Result, ATanResultValid);

    XMVECTOR V = XMVectorDivide(Y, X);

    XMVECTOR R0 = XMVectorATan(V);

    R1 = XMVectorSelect(Pi, g_XMNegativeZero, XIsPositive);
    R2 = XMVectorAdd(R0, R1);

    return XMVectorSelect(Result, R2, ATanResultValid);

#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorSinEst(FXMVECTOR V) noexcept
{
    // 7-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            sinf(V.vector4_f32[0]),
            sinf(V.vector4_f32[1]),
            sinf(V.vector4_f32[2]),
            sinf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(x), g_XMNegativeZero);
    uint32x4_t c = vorrq_u32(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    float32x4_t absx = vabsq_f32(x);
    float32x4_t rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    uint32x4_t comp = vcleq_f32(absx, g_XMHalfPi);
    x = vbslq_f32(comp, x, rflx);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const XMVECTOR SEC = g_XMSinCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(SEC), 0);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_high_f32(SEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(SEC), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    Result = vmulq_f32(Result, x);
    return Result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_sin_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x).
    __m128 sign = _mm_and_ps(x, g_XMNegativeZero);
    __m128 c = _mm_or_ps(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    __m128 absx = _mm_andnot_ps(sign, x);  // |x|
    __m128 rflx = _mm_sub_ps(c, x);
    __m128 comp = _mm_cmple_ps(absx, g_XMHalfPi);
    __m128 select0 = _mm_and_ps(comp, x);
    __m128 select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const XMVECTOR SEC = g_XMSinCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(SEC, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(SEC, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SEC, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);
    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, x);
    return Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorCosEst(FXMVECTOR V) noexcept
{
    // 6-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            cosf(V.vector4_f32[0]),
            cosf(V.vector4_f32[1]),
            cosf(V.vector4_f32[2]),
            cosf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Map V to x in [-pi,pi].
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(x), g_XMNegativeZero);
    uint32x4_t c = vorrq_u32(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    float32x4_t absx = vabsq_f32(x);
    float32x4_t rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    uint32x4_t comp = vcleq_f32(absx, g_XMHalfPi);
    x = vbslq_f32(comp, x, rflx);
    float32x4_t fsign = vbslq_f32(comp, g_XMOne, g_XMNegativeOne);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const XMVECTOR CEC = g_XMCosCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(CEC), 0);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_high_f32(CEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(CEC), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    Result = vmulq_f32(Result, fsign);
    return Result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_cos_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    // Map V to x in [-pi,pi].
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
    XMVECTOR sign = _mm_and_ps(x, g_XMNegativeZero);
    __m128 c = _mm_or_ps(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    __m128 absx = _mm_andnot_ps(sign, x);  // |x|
    __m128 rflx = _mm_sub_ps(c, x);
    __m128 comp = _mm_cmple_ps(absx, g_XMHalfPi);
    __m128 select0 = _mm_and_ps(comp, x);
    __m128 select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);
    select0 = _mm_and_ps(comp, g_XMOne);
    select1 = _mm_andnot_ps(comp, g_XMNegativeOne);
    sign = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const XMVECTOR CEC = g_XMCosCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(CEC, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(CEC, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CEC, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);
    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, sign);
    return Result;
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline void XM_CALLCONV XMVectorSinCosEst
(
    XMVECTOR* pSin,
    XMVECTOR* pCos,
    FXMVECTOR  V
) noexcept
{
    assert(pSin != nullptr);
    assert(pCos != nullptr);

    // 7/6-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Sin = { { {
            sinf(V.vector4_f32[0]),
            sinf(V.vector4_f32[1]),
            sinf(V.vector4_f32[2]),
            sinf(V.vector4_f32[3])
        } } };

    XMVECTORF32 Cos = { { {
            cosf(V.vector4_f32[0]),
            cosf(V.vector4_f32[1]),
            cosf(V.vector4_f32[2]),
            cosf(V.vector4_f32[3])
        } } };

    *pSin = Sin.v;
    *pCos = Cos.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with cos(y) = sign*cos(x).
    uint32x4_t sign = vandq_u32(vreinterpretq_u32_f32(x), g_XMNegativeZero);
    uint32x4_t c = vorrq_u32(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    float32x4_t absx = vabsq_f32(x);
    float32x4_t rflx = vsubq_f32(vreinterpretq_f32_u32(c), x);
    uint32x4_t comp = vcleq_f32(absx, g_XMHalfPi);
    x = vbslq_f32(comp, x, rflx);
    float32x4_t fsign = vbslq_f32(comp, g_XMOne, g_XMNegativeOne);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation for sine
    const XMVECTOR SEC = g_XMSinCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(SEC), 0);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_high_f32(SEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(SEC), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    *pSin = vmulq_f32(Result, x);

    // Compute polynomial approximation
    const XMVECTOR CEC = g_XMCosCoefficients1;
    vConstants = vdupq_lane_f32(vget_high_f32(CEC), 0);
    Result = vmlaq_lane_f32(vConstants, x2, vget_high_f32(CEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(CEC), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    Result = vmlaq_f32(g_XMOne, Result, x2);
    *pCos = vmulq_f32(Result, fsign);
#elif defined(_XM_SSE_INTRINSICS_)
    // Force the value within the bounds of pi
    XMVECTOR x = XMVectorModAngles(V);

    // Map in [-pi/2,pi/2] with sin(y) = sin(x), cos(y) = sign*cos(x).
    XMVECTOR sign = _mm_and_ps(x, g_XMNegativeZero);
    __m128 c = _mm_or_ps(g_XMPi, sign);  // pi when x >= 0, -pi when x < 0
    __m128 absx = _mm_andnot_ps(sign, x);  // |x|
    __m128 rflx = _mm_sub_ps(c, x);
    __m128 comp = _mm_cmple_ps(absx, g_XMHalfPi);
    __m128 select0 = _mm_and_ps(comp, x);
    __m128 select1 = _mm_andnot_ps(comp, rflx);
    x = _mm_or_ps(select0, select1);
    select0 = _mm_and_ps(comp, g_XMOne);
    select1 = _mm_andnot_ps(comp, g_XMNegativeOne);
    sign = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation for sine
    const XMVECTOR SEC = g_XMSinCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(SEC, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(SEC, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(SEC, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);
    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, x);
    *pSin = Result;

    // Compute polynomial approximation for cosine
    const XMVECTOR CEC = g_XMCosCoefficients1;
    vConstantsB = XM_PERMUTE_PS(CEC, _MM_SHUFFLE(3, 3, 3, 3));
    vConstants = XM_PERMUTE_PS(CEC, _MM_SHUFFLE(2, 2, 2, 2));
    Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(CEC, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);
    Result = XM_FMADD_PS(Result, x2, g_XMOne);
    Result = _mm_mul_ps(Result, sign);
    *pCos = Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorTanEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            tanf(V.vector4_f32[0]),
            tanf(V.vector4_f32[1]),
            tanf(V.vector4_f32[2]),
            tanf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_tan_ps(V);
    return Result;
#else

    XMVECTOR OneOverPi = XMVectorSplatW(g_XMTanEstCoefficients.v);

    XMVECTOR V1 = XMVectorMultiply(V, OneOverPi);
    V1 = XMVectorRound(V1);

    V1 = XMVectorNegativeMultiplySubtract(g_XMPi.v, V1, V);

    XMVECTOR T0 = XMVectorSplatX(g_XMTanEstCoefficients.v);
    XMVECTOR T1 = XMVectorSplatY(g_XMTanEstCoefficients.v);
    XMVECTOR T2 = XMVectorSplatZ(g_XMTanEstCoefficients.v);

    XMVECTOR V2T2 = XMVectorNegativeMultiplySubtract(V1, V1, T2);
    XMVECTOR V2 = XMVectorMultiply(V1, V1);
    XMVECTOR V1T0 = XMVectorMultiply(V1, T0);
    XMVECTOR V1T1 = XMVectorMultiply(V1, T1);

    XMVECTOR D = XMVectorReciprocalEst(V2T2);
    XMVECTOR N = XMVectorMultiplyAdd(V2, V1T1, V1T0);

    return XMVectorMultiply(N, D);

#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorASinEst(FXMVECTOR V) noexcept
{
    // 3-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result;
    Result.f[0] = asinf(V.vector4_f32[0]);
    Result.f[1] = asinf(V.vector4_f32[1]);
    Result.f[2] = asinf(V.vector4_f32[2]);
    Result.f[3] = asinf(V.vector4_f32[3]);
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t nonnegative = vcgeq_f32(V, g_XMZero);
    float32x4_t x = vabsq_f32(V);

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    float32x4_t oneMValue = vsubq_f32(g_XMOne, x);
    float32x4_t clampOneMValue = vmaxq_f32(g_XMZero, oneMValue);
    float32x4_t root = XMVectorSqrt(clampOneMValue);

    // Compute polynomial approximation
    const XMVECTOR AEC = g_XMArcEstCoefficients;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(AEC), 0);
    XMVECTOR t0 = vmlaq_lane_f32(vConstants, x, vget_high_f32(AEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(AEC), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AEC), 0);
    t0 = vmlaq_f32(vConstants, t0, x);
    t0 = vmulq_f32(t0, root);

    float32x4_t t1 = vsubq_f32(g_XMPi, t0);
    t0 = vbslq_f32(nonnegative, t0, t1);
    t0 = vsubq_f32(g_XMHalfPi, t0);
    return t0;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_asin_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 nonnegative = _mm_cmpge_ps(V, g_XMZero);
    __m128 mvalue = _mm_sub_ps(g_XMZero, V);
    __m128 x = _mm_max_ps(V, mvalue);  // |V|

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    __m128 oneMValue = _mm_sub_ps(g_XMOne, x);
    __m128 clampOneMValue = _mm_max_ps(g_XMZero, oneMValue);
    __m128 root = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

    // Compute polynomial approximation
    const XMVECTOR AEC = g_XMArcEstCoefficients;
    __m128 vConstantsB = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 t0 = XM_FMADD_PS(vConstantsB, x, vConstants);

    vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(1, 1, 1, 1));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(0, 0, 0, 0));
    t0 = XM_FMADD_PS(t0, x, vConstants);
    t0 = _mm_mul_ps(t0, root);

    __m128 t1 = _mm_sub_ps(g_XMPi, t0);
    t0 = _mm_and_ps(nonnegative, t0);
    t1 = _mm_andnot_ps(nonnegative, t1);
    t0 = _mm_or_ps(t0, t1);
    t0 = _mm_sub_ps(g_XMHalfPi, t0);
    return t0;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorACosEst(FXMVECTOR V) noexcept
{
    // 3-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            acosf(V.vector4_f32[0]),
            acosf(V.vector4_f32[1]),
            acosf(V.vector4_f32[2]),
            acosf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t nonnegative = vcgeq_f32(V, g_XMZero);
    float32x4_t x = vabsq_f32(V);

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    float32x4_t oneMValue = vsubq_f32(g_XMOne, x);
    float32x4_t clampOneMValue = vmaxq_f32(g_XMZero, oneMValue);
    float32x4_t root = XMVectorSqrt(clampOneMValue);

    // Compute polynomial approximation
    const XMVECTOR AEC = g_XMArcEstCoefficients;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(AEC), 0);
    XMVECTOR t0 = vmlaq_lane_f32(vConstants, x, vget_high_f32(AEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(AEC), 1);
    t0 = vmlaq_f32(vConstants, t0, x);

    vConstants = vdupq_lane_f32(vget_low_f32(AEC), 0);
    t0 = vmlaq_f32(vConstants, t0, x);
    t0 = vmulq_f32(t0, root);

    float32x4_t t1 = vsubq_f32(g_XMPi, t0);
    t0 = vbslq_f32(nonnegative, t0, t1);
    return t0;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_acos_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 nonnegative = _mm_cmpge_ps(V, g_XMZero);
    __m128 mvalue = _mm_sub_ps(g_XMZero, V);
    __m128 x = _mm_max_ps(V, mvalue);  // |V|

    // Compute (1-|V|), clamp to zero to avoid sqrt of negative number.
    __m128 oneMValue = _mm_sub_ps(g_XMOne, x);
    __m128 clampOneMValue = _mm_max_ps(g_XMZero, oneMValue);
    __m128 root = _mm_sqrt_ps(clampOneMValue);  // sqrt(1-|V|)

    // Compute polynomial approximation
    const XMVECTOR AEC = g_XMArcEstCoefficients;
    __m128 vConstantsB = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 t0 = XM_FMADD_PS(vConstantsB, x, vConstants);

    vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(1, 1, 1, 1));
    t0 = XM_FMADD_PS(t0, x, vConstants);

    vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(0, 0, 0, 0));
    t0 = XM_FMADD_PS(t0, x, vConstants);
    t0 = _mm_mul_ps(t0, root);

    __m128 t1 = _mm_sub_ps(g_XMPi, t0);
    t0 = _mm_and_ps(nonnegative, t0);
    t1 = _mm_andnot_ps(nonnegative, t1);
    t0 = _mm_or_ps(t0, t1);
    return t0;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorATanEst(FXMVECTOR V) noexcept
{
    // 9-degree minimax approximation

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            atanf(V.vector4_f32[0]),
            atanf(V.vector4_f32[1]),
            atanf(V.vector4_f32[2]),
            atanf(V.vector4_f32[3])
        } } };
    return Result.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t absV = vabsq_f32(V);
    float32x4_t invV = XMVectorReciprocalEst(V);
    uint32x4_t comp = vcgtq_f32(V, g_XMOne);
    float32x4_t sign = vbslq_f32(comp, g_XMOne, g_XMNegativeOne);
    comp = vcleq_f32(absV, g_XMOne);
    sign = vbslq_f32(comp, g_XMZero, sign);
    float32x4_t x = vbslq_f32(comp, V, invV);

    float32x4_t x2 = vmulq_f32(x, x);

    // Compute polynomial approximation
    const XMVECTOR AEC = g_XMATanEstCoefficients1;
    XMVECTOR vConstants = vdupq_lane_f32(vget_high_f32(AEC), 0);
    XMVECTOR Result = vmlaq_lane_f32(vConstants, x2, vget_high_f32(AEC), 1);

    vConstants = vdupq_lane_f32(vget_low_f32(AEC), 1);
    Result = vmlaq_f32(vConstants, Result, x2);

    vConstants = vdupq_lane_f32(vget_low_f32(AEC), 0);
    Result = vmlaq_f32(vConstants, Result, x2);

    // ATanEstCoefficients0 is already splatted
    Result = vmlaq_f32(g_XMATanEstCoefficients0, Result, x2);
    Result = vmulq_f32(Result, x);

    float32x4_t result1 = vmulq_f32(sign, g_XMHalfPi);
    result1 = vsubq_f32(result1, Result);

    comp = vceqq_f32(sign, g_XMZero);
    Result = vbslq_f32(comp, Result, result1);
    return Result;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_atan_ps(V);
    return Result;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128 absV = XMVectorAbs(V);
    __m128 invV = _mm_div_ps(g_XMOne, V);
    __m128 comp = _mm_cmpgt_ps(V, g_XMOne);
    __m128 select0 = _mm_and_ps(comp, g_XMOne);
    __m128 select1 = _mm_andnot_ps(comp, g_XMNegativeOne);
    __m128 sign = _mm_or_ps(select0, select1);
    comp = _mm_cmple_ps(absV, g_XMOne);
    select0 = _mm_and_ps(comp, g_XMZero);
    select1 = _mm_andnot_ps(comp, sign);
    sign = _mm_or_ps(select0, select1);
    select0 = _mm_and_ps(comp, V);
    select1 = _mm_andnot_ps(comp, invV);
    __m128 x = _mm_or_ps(select0, select1);

    __m128 x2 = _mm_mul_ps(x, x);

    // Compute polynomial approximation
    const XMVECTOR AEC = g_XMATanEstCoefficients1;
    __m128 vConstantsB = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(3, 3, 3, 3));
    __m128 vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(2, 2, 2, 2));
    __m128 Result = XM_FMADD_PS(vConstantsB, x2, vConstants);

    vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(1, 1, 1, 1));
    Result = XM_FMADD_PS(Result, x2, vConstants);

    vConstants = XM_PERMUTE_PS(AEC, _MM_SHUFFLE(0, 0, 0, 0));
    Result = XM_FMADD_PS(Result, x2, vConstants);
    // ATanEstCoefficients0 is already splatted
    Result = XM_FMADD_PS(Result, x2, g_XMATanEstCoefficients0);
    Result = _mm_mul_ps(Result, x);
    __m128 result1 = _mm_mul_ps(sign, g_XMHalfPi);
    result1 = _mm_sub_ps(result1, Result);

    comp = _mm_cmpeq_ps(sign, g_XMZero);
    select0 = _mm_and_ps(comp, Result);
    select1 = _mm_andnot_ps(comp, result1);
    Result = _mm_or_ps(select0, select1);
    return Result;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorATan2Est
(
    FXMVECTOR Y,
    FXMVECTOR X
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 Result = { { {
            atan2f(Y.vector4_f32[0], X.vector4_f32[0]),
            atan2f(Y.vector4_f32[1], X.vector4_f32[1]),
            atan2f(Y.vector4_f32[2], X.vector4_f32[2]),
            atan2f(Y.vector4_f32[3], X.vector4_f32[3]),
        } } };
    return Result.v;
#elif defined(_XM_SVML_INTRINSICS_)
    XMVECTOR Result = _mm_atan2_ps(Y, X);
    return Result;
#else

    static const XMVECTORF32 ATan2Constants = { { { XM_PI, XM_PIDIV2, XM_PIDIV4, 2.3561944905f /* Pi*3/4 */ } } };

    const XMVECTOR Zero = XMVectorZero();
    XMVECTOR ATanResultValid = XMVectorTrueInt();

    XMVECTOR Pi = XMVectorSplatX(ATan2Constants);
    XMVECTOR PiOverTwo = XMVectorSplatY(ATan2Constants);
    XMVECTOR PiOverFour = XMVectorSplatZ(ATan2Constants);
    XMVECTOR ThreePiOverFour = XMVectorSplatW(ATan2Constants);

    XMVECTOR YEqualsZero = XMVectorEqual(Y, Zero);
    XMVECTOR XEqualsZero = XMVectorEqual(X, Zero);
    XMVECTOR XIsPositive = XMVectorAndInt(X, g_XMNegativeZero.v);
    XIsPositive = XMVectorEqualInt(XIsPositive, Zero);
    XMVECTOR YEqualsInfinity = XMVectorIsInfinite(Y);
    XMVECTOR XEqualsInfinity = XMVectorIsInfinite(X);

    XMVECTOR YSign = XMVectorAndInt(Y, g_XMNegativeZero.v);
    Pi = XMVectorOrInt(Pi, YSign);
    PiOverTwo = XMVectorOrInt(PiOverTwo, YSign);
    PiOverFour = XMVectorOrInt(PiOverFour, YSign);
    ThreePiOverFour = XMVectorOrInt(ThreePiOverFour, YSign);

    XMVECTOR R1 = XMVectorSelect(Pi, YSign, XIsPositive);
    XMVECTOR R2 = XMVectorSelect(ATanResultValid, PiOverTwo, XEqualsZero);
    XMVECTOR R3 = XMVectorSelect(R2, R1, YEqualsZero);
    XMVECTOR R4 = XMVectorSelect(ThreePiOverFour, PiOverFour, XIsPositive);
    XMVECTOR R5 = XMVectorSelect(PiOverTwo, R4, XEqualsInfinity);
    XMVECTOR Result = XMVectorSelect(R3, R5, YEqualsInfinity);
    ATanResultValid = XMVectorEqualInt(Result, ATanResultValid);

    XMVECTOR Reciprocal = XMVectorReciprocalEst(X);
    XMVECTOR V = XMVectorMultiply(Y, Reciprocal);
    XMVECTOR R0 = XMVectorATanEst(V);

    R1 = XMVectorSelect(Pi, g_XMNegativeZero, XIsPositive);
    R2 = XMVectorAdd(R0, R1);

    Result = XMVectorSelect(Result, R2, ATanResultValid);

    return Result;

#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLerp
(
    FXMVECTOR V0,
    FXMVECTOR V1,
    float    t
) noexcept
{
    // V0 + t * (V1 - V0)

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Scale = XMVectorReplicate(t);
    XMVECTOR Length = XMVectorSubtract(V1, V0);
    return XMVectorMultiplyAdd(Length, Scale, V0);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR L = vsubq_f32(V1, V0);
    return vmlaq_n_f32(V0, L, t);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR L = _mm_sub_ps(V1, V0);
    XMVECTOR S = _mm_set_ps1(t);
    return XM_FMADD_PS(L, S, V0);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorLerpV
(
    FXMVECTOR V0,
    FXMVECTOR V1,
    FXMVECTOR T
) noexcept
{
    // V0 + T * (V1 - V0)

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Length = XMVectorSubtract(V1, V0);
    return XMVectorMultiplyAdd(Length, T, V0);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR L = vsubq_f32(V1, V0);
    return vmlaq_f32(V0, L, T);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR Length = _mm_sub_ps(V1, V0);
    return XM_FMADD_PS(Length, T, V0);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorHermite
(
    FXMVECTOR Position0,
    FXMVECTOR Tangent0,
    FXMVECTOR Position1,
    GXMVECTOR Tangent1,
    float    t
) noexcept
{
    // Result = (2 * t^3 - 3 * t^2 + 1) * Position0 +
    //          (t^3 - 2 * t^2 + t) * Tangent0 +
    //          (-2 * t^3 + 3 * t^2) * Position1 +
    //          (t^3 - t^2) * Tangent1

#if defined(_XM_NO_INTRINSICS_)

    float t2 = t * t;
    float t3 = t * t2;

    XMVECTOR P0 = XMVectorReplicate(2.0f * t3 - 3.0f * t2 + 1.0f);
    XMVECTOR T0 = XMVectorReplicate(t3 - 2.0f * t2 + t);
    XMVECTOR P1 = XMVectorReplicate(-2.0f * t3 + 3.0f * t2);
    XMVECTOR T1 = XMVectorReplicate(t3 - t2);

    XMVECTOR Result = XMVectorMultiply(P0, Position0);
    Result = XMVectorMultiplyAdd(T0, Tangent0, Result);
    Result = XMVectorMultiplyAdd(P1, Position1, Result);
    Result = XMVectorMultiplyAdd(T1, Tangent1, Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float t2 = t * t;
    float t3 = t * t2;

    float p0 = 2.0f * t3 - 3.0f * t2 + 1.0f;
    float t0 = t3 - 2.0f * t2 + t;
    float p1 = -2.0f * t3 + 3.0f * t2;
    float t1 = t3 - t2;

    XMVECTOR vResult = vmulq_n_f32(Position0, p0);
    vResult = vmlaq_n_f32(vResult, Tangent0, t0);
    vResult = vmlaq_n_f32(vResult, Position1, p1);
    vResult = vmlaq_n_f32(vResult, Tangent1, t1);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    float t2 = t * t;
    float t3 = t * t2;

    XMVECTOR P0 = _mm_set_ps1(2.0f * t3 - 3.0f * t2 + 1.0f);
    XMVECTOR T0 = _mm_set_ps1(t3 - 2.0f * t2 + t);
    XMVECTOR P1 = _mm_set_ps1(-2.0f * t3 + 3.0f * t2);
    XMVECTOR T1 = _mm_set_ps1(t3 - t2);

    XMVECTOR vResult = _mm_mul_ps(P0, Position0);
    vResult = XM_FMADD_PS(Tangent0, T0, vResult);
    vResult = XM_FMADD_PS(Position1, P1, vResult);
    vResult = XM_FMADD_PS(Tangent1, T1, vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorHermiteV
(
    FXMVECTOR Position0,
    FXMVECTOR Tangent0,
    FXMVECTOR Position1,
    GXMVECTOR Tangent1,
    HXMVECTOR T
) noexcept
{
    // Result = (2 * t^3 - 3 * t^2 + 1) * Position0 +
    //          (t^3 - 2 * t^2 + t) * Tangent0 +
    //          (-2 * t^3 + 3 * t^2) * Position1 +
    //          (t^3 - t^2) * Tangent1

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR T2 = XMVectorMultiply(T, T);
    XMVECTOR T3 = XMVectorMultiply(T, T2);

    XMVECTOR P0 = XMVectorReplicate(2.0f * T3.vector4_f32[0] - 3.0f * T2.vector4_f32[0] + 1.0f);
    XMVECTOR T0 = XMVectorReplicate(T3.vector4_f32[1] - 2.0f * T2.vector4_f32[1] + T.vector4_f32[1]);
    XMVECTOR P1 = XMVectorReplicate(-2.0f * T3.vector4_f32[2] + 3.0f * T2.vector4_f32[2]);
    XMVECTOR T1 = XMVectorReplicate(T3.vector4_f32[3] - T2.vector4_f32[3]);

    XMVECTOR Result = XMVectorMultiply(P0, Position0);
    Result = XMVectorMultiplyAdd(T0, Tangent0, Result);
    Result = XMVectorMultiplyAdd(P1, Position1, Result);
    Result = XMVectorMultiplyAdd(T1, Tangent1, Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 CatMulT2 = { { { -3.0f, -2.0f, 3.0f, -1.0f } } };
    static const XMVECTORF32 CatMulT3 = { { { 2.0f, 1.0f, -2.0f, 1.0f } } };

    XMVECTOR T2 = vmulq_f32(T, T);
    XMVECTOR T3 = vmulq_f32(T, T2);
    // Mul by the constants against t^2
    T2 = vmulq_f32(T2, CatMulT2);
    // Mul by the constants against t^3
    T3 = vmlaq_f32(T2, T3, CatMulT3);
    // T3 now has the pre-result.
    // I need to add t.y only
    T2 = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(T), g_XMMaskY));
    T3 = vaddq_f32(T3, T2);
    // Add 1.0f to x
    T3 = vaddq_f32(T3, g_XMIdentityR0);
    // Now, I have the constants created
    // Mul the x constant to Position0
    XMVECTOR vResult = vmulq_lane_f32(Position0, vget_low_f32(T3), 0); // T3[0]
    // Mul the y constant to Tangent0
    vResult = vmlaq_lane_f32(vResult, Tangent0, vget_low_f32(T3), 1); // T3[1]
    // Mul the z constant to Position1
    vResult = vmlaq_lane_f32(vResult, Position1, vget_high_f32(T3), 0); // T3[2]
    // Mul the w constant to Tangent1
    vResult = vmlaq_lane_f32(vResult, Tangent1, vget_high_f32(T3), 1); // T3[3]
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 CatMulT2 = { { { -3.0f, -2.0f, 3.0f, -1.0f } } };
    static const XMVECTORF32 CatMulT3 = { { { 2.0f, 1.0f, -2.0f, 1.0f } } };

    XMVECTOR T2 = _mm_mul_ps(T, T);
    XMVECTOR T3 = _mm_mul_ps(T, T2);
    // Mul by the constants against t^2
    T2 = _mm_mul_ps(T2, CatMulT2);
    // Mul by the constants against t^3
    T3 = XM_FMADD_PS(T3, CatMulT3, T2);
    // T3 now has the pre-result.
    // I need to add t.y only
    T2 = _mm_and_ps(T, g_XMMaskY);
    T3 = _mm_add_ps(T3, T2);
    // Add 1.0f to x
    T3 = _mm_add_ps(T3, g_XMIdentityR0);
    // Now, I have the constants created
    // Mul the x constant to Position0
    XMVECTOR vResult = XM_PERMUTE_PS(T3, _MM_SHUFFLE(0, 0, 0, 0));
    vResult = _mm_mul_ps(vResult, Position0);
    // Mul the y constant to Tangent0
    T2 = XM_PERMUTE_PS(T3, _MM_SHUFFLE(1, 1, 1, 1));
    vResult = XM_FMADD_PS(T2, Tangent0, vResult);
    // Mul the z constant to Position1
    T2 = XM_PERMUTE_PS(T3, _MM_SHUFFLE(2, 2, 2, 2));
    vResult = XM_FMADD_PS(T2, Position1, vResult);
    // Mul the w constant to Tangent1
    T3 = XM_PERMUTE_PS(T3, _MM_SHUFFLE(3, 3, 3, 3));
    vResult = XM_FMADD_PS(T3, Tangent1, vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorCatmullRom
(
    FXMVECTOR Position0,
    FXMVECTOR Position1,
    FXMVECTOR Position2,
    GXMVECTOR Position3,
    float    t
) noexcept
{
    // Result = ((-t^3 + 2 * t^2 - t) * Position0 +
    //           (3 * t^3 - 5 * t^2 + 2) * Position1 +
    //           (-3 * t^3 + 4 * t^2 + t) * Position2 +
    //           (t^3 - t^2) * Position3) * 0.5

#if defined(_XM_NO_INTRINSICS_)

    float t2 = t * t;
    float t3 = t * t2;

    XMVECTOR P0 = XMVectorReplicate((-t3 + 2.0f * t2 - t) * 0.5f);
    XMVECTOR P1 = XMVectorReplicate((3.0f * t3 - 5.0f * t2 + 2.0f) * 0.5f);
    XMVECTOR P2 = XMVectorReplicate((-3.0f * t3 + 4.0f * t2 + t) * 0.5f);
    XMVECTOR P3 = XMVectorReplicate((t3 - t2) * 0.5f);

    XMVECTOR Result = XMVectorMultiply(P0, Position0);
    Result = XMVectorMultiplyAdd(P1, Position1, Result);
    Result = XMVectorMultiplyAdd(P2, Position2, Result);
    Result = XMVectorMultiplyAdd(P3, Position3, Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float t2 = t * t;
    float t3 = t * t2;

    float p0 = (-t3 + 2.0f * t2 - t) * 0.5f;
    float p1 = (3.0f * t3 - 5.0f * t2 + 2.0f) * 0.5f;
    float p2 = (-3.0f * t3 + 4.0f * t2 + t) * 0.5f;
    float p3 = (t3 - t2) * 0.5f;

    XMVECTOR P1 = vmulq_n_f32(Position1, p1);
    XMVECTOR P0 = vmlaq_n_f32(P1, Position0, p0);
    XMVECTOR P3 = vmulq_n_f32(Position3, p3);
    XMVECTOR P2 = vmlaq_n_f32(P3, Position2, p2);
    P0 = vaddq_f32(P0, P2);
    return P0;
#elif defined(_XM_SSE_INTRINSICS_)
    float t2 = t * t;
    float t3 = t * t2;

    XMVECTOR P0 = _mm_set_ps1((-t3 + 2.0f * t2 - t) * 0.5f);
    XMVECTOR P1 = _mm_set_ps1((3.0f * t3 - 5.0f * t2 + 2.0f) * 0.5f);
    XMVECTOR P2 = _mm_set_ps1((-3.0f * t3 + 4.0f * t2 + t) * 0.5f);
    XMVECTOR P3 = _mm_set_ps1((t3 - t2) * 0.5f);

    P1 = _mm_mul_ps(Position1, P1);
    P0 = XM_FMADD_PS(Position0, P0, P1);
    P3 = _mm_mul_ps(Position3, P3);
    P2 = XM_FMADD_PS(Position2, P2, P3);
    P0 = _mm_add_ps(P0, P2);
    return P0;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorCatmullRomV
(
    FXMVECTOR Position0,
    FXMVECTOR Position1,
    FXMVECTOR Position2,
    GXMVECTOR Position3,
    HXMVECTOR T
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float fx = T.vector4_f32[0];
    float fy = T.vector4_f32[1];
    float fz = T.vector4_f32[2];
    float fw = T.vector4_f32[3];
    XMVECTORF32 vResult = { { {
            0.5f * ((-fx * fx * fx + 2 * fx * fx - fx) * Position0.vector4_f32[0]
            + (3 * fx * fx * fx - 5 * fx * fx + 2) * Position1.vector4_f32[0]
            + (-3 * fx * fx * fx + 4 * fx * fx + fx) * Position2.vector4_f32[0]
            + (fx * fx * fx - fx * fx) * Position3.vector4_f32[0]),

            0.5f * ((-fy * fy * fy + 2 * fy * fy - fy) * Position0.vector4_f32[1]
            + (3 * fy * fy * fy - 5 * fy * fy + 2) * Position1.vector4_f32[1]
            + (-3 * fy * fy * fy + 4 * fy * fy + fy) * Position2.vector4_f32[1]
            + (fy * fy * fy - fy * fy) * Position3.vector4_f32[1]),

            0.5f * ((-fz * fz * fz + 2 * fz * fz - fz) * Position0.vector4_f32[2]
            + (3 * fz * fz * fz - 5 * fz * fz + 2) * Position1.vector4_f32[2]
            + (-3 * fz * fz * fz + 4 * fz * fz + fz) * Position2.vector4_f32[2]
            + (fz * fz * fz - fz * fz) * Position3.vector4_f32[2]),

            0.5f * ((-fw * fw * fw + 2 * fw * fw - fw) * Position0.vector4_f32[3]
            + (3 * fw * fw * fw - 5 * fw * fw + 2) * Position1.vector4_f32[3]
            + (-3 * fw * fw * fw + 4 * fw * fw + fw) * Position2.vector4_f32[3]
            + (fw * fw * fw - fw * fw) * Position3.vector4_f32[3])
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Catmul2 = { { { 2.0f, 2.0f, 2.0f, 2.0f } } };
    static const XMVECTORF32 Catmul3 = { { { 3.0f, 3.0f, 3.0f, 3.0f } } };
    static const XMVECTORF32 Catmul4 = { { { 4.0f, 4.0f, 4.0f, 4.0f } } };
    static const XMVECTORF32 Catmul5 = { { { 5.0f, 5.0f, 5.0f, 5.0f } } };
    // Cache T^2 and T^3
    XMVECTOR T2 = vmulq_f32(T, T);
    XMVECTOR T3 = vmulq_f32(T, T2);
    // Perform the Position0 term
    XMVECTOR vResult = vaddq_f32(T2, T2);
    vResult = vsubq_f32(vResult, T);
    vResult = vsubq_f32(vResult, T3);
    vResult = vmulq_f32(vResult, Position0);
    // Perform the Position1 term and add
    XMVECTOR vTemp = vmulq_f32(T3, Catmul3);
    vTemp = vmlsq_f32(vTemp, T2, Catmul5);
    vTemp = vaddq_f32(vTemp, Catmul2);
    vResult = vmlaq_f32(vResult, vTemp, Position1);
    // Perform the Position2 term and add
    vTemp = vmulq_f32(T2, Catmul4);
    vTemp = vmlsq_f32(vTemp, T3, Catmul3);
    vTemp = vaddq_f32(vTemp, T);
    vResult = vmlaq_f32(vResult, vTemp, Position2);
    // Position3 is the last term
    T3 = vsubq_f32(T3, T2);
    vResult = vmlaq_f32(vResult, T3, Position3);
    // Multiply by 0.5f and exit
    vResult = vmulq_f32(vResult, g_XMOneHalf);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Catmul2 = { { { 2.0f, 2.0f, 2.0f, 2.0f } } };
    static const XMVECTORF32 Catmul3 = { { { 3.0f, 3.0f, 3.0f, 3.0f } } };
    static const XMVECTORF32 Catmul4 = { { { 4.0f, 4.0f, 4.0f, 4.0f } } };
    static const XMVECTORF32 Catmul5 = { { { 5.0f, 5.0f, 5.0f, 5.0f } } };
    // Cache T^2 and T^3
    XMVECTOR T2 = _mm_mul_ps(T, T);
    XMVECTOR T3 = _mm_mul_ps(T, T2);
    // Perform the Position0 term
    XMVECTOR vResult = _mm_add_ps(T2, T2);
    vResult = _mm_sub_ps(vResult, T);
    vResult = _mm_sub_ps(vResult, T3);
    vResult = _mm_mul_ps(vResult, Position0);
    // Perform the Position1 term and add
    XMVECTOR vTemp = _mm_mul_ps(T3, Catmul3);
    vTemp = XM_FNMADD_PS(T2, Catmul5, vTemp);
    vTemp = _mm_add_ps(vTemp, Catmul2);
    vResult = XM_FMADD_PS(vTemp, Position1, vResult);
    // Perform the Position2 term and add
    vTemp = _mm_mul_ps(T2, Catmul4);
    vTemp = XM_FNMADD_PS(T3, Catmul3, vTemp);
    vTemp = _mm_add_ps(vTemp, T);
    vResult = XM_FMADD_PS(vTemp, Position2, vResult);
    // Position3 is the last term
    T3 = _mm_sub_ps(T3, T2);
    vResult = XM_FMADD_PS(T3, Position3, vResult);
    // Multiply by 0.5f and exit
    vResult = _mm_mul_ps(vResult, g_XMOneHalf);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorBaryCentric
(
    FXMVECTOR Position0,
    FXMVECTOR Position1,
    FXMVECTOR Position2,
    float    f,
    float    g
) noexcept
{
    // Result = Position0 + f * (Position1 - Position0) + g * (Position2 - Position0)

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR P10 = XMVectorSubtract(Position1, Position0);
    XMVECTOR ScaleF = XMVectorReplicate(f);

    XMVECTOR P20 = XMVectorSubtract(Position2, Position0);
    XMVECTOR ScaleG = XMVectorReplicate(g);

    XMVECTOR Result = XMVectorMultiplyAdd(P10, ScaleF, Position0);
    Result = XMVectorMultiplyAdd(P20, ScaleG, Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR R1 = vsubq_f32(Position1, Position0);
    XMVECTOR R2 = vsubq_f32(Position2, Position0);
    R1 = vmlaq_n_f32(Position0, R1, f);
    return vmlaq_n_f32(R1, R2, g);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR R1 = _mm_sub_ps(Position1, Position0);
    XMVECTOR R2 = _mm_sub_ps(Position2, Position0);
    XMVECTOR SF = _mm_set_ps1(f);
    R1 = XM_FMADD_PS(R1, SF, Position0);
    XMVECTOR SG = _mm_set_ps1(g);
    return XM_FMADD_PS(R2, SG, R1);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVectorBaryCentricV
(
    FXMVECTOR Position0,
    FXMVECTOR Position1,
    FXMVECTOR Position2,
    GXMVECTOR F,
    HXMVECTOR G
) noexcept
{
    // Result = Position0 + f * (Position1 - Position0) + g * (Position2 - Position0)

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR P10 = XMVectorSubtract(Position1, Position0);
    XMVECTOR P20 = XMVectorSubtract(Position2, Position0);

    XMVECTOR Result = XMVectorMultiplyAdd(P10, F, Position0);
    Result = XMVectorMultiplyAdd(P20, G, Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR R1 = vsubq_f32(Position1, Position0);
    XMVECTOR R2 = vsubq_f32(Position2, Position0);
    R1 = vmlaq_f32(Position0, R1, F);
    return vmlaq_f32(R1, R2, G);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR R1 = _mm_sub_ps(Position1, Position0);
    XMVECTOR R2 = _mm_sub_ps(Position2, Position0);
    R1 = XM_FMADD_PS(R1, F, Position0);
    return XM_FMADD_PS(R2, G, R1);
#endif
}

/****************************************************************************
 *
 * 2D Vector
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2Equal
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] == V2.vector4_f32[0]) && (V1.vector4_f32[1] == V2.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vceq_f32(vget_low_f32(V1), vget_low_f32(V2));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    // z and w are don't care
    return (((_mm_movemask_ps(vTemp) & 3) == 3) != 0);
#endif
}


//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector2EqualR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    uint32_t CR = 0;
    if ((V1.vector4_f32[0] == V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] == V2.vector4_f32[1]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] != V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] != V2.vector4_f32[1]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vceq_f32(vget_low_f32(V1), vget_low_f32(V2));
    uint64_t r = vget_lane_u64(vreinterpret_u64_u32(vTemp), 0);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    // z and w are don't care
    int iTest = _mm_movemask_ps(vTemp) & 3;
    uint32_t CR = 0;
    if (iTest == 3)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2EqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_u32[0] == V2.vector4_u32[0]) && (V1.vector4_u32[1] == V2.vector4_u32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vceq_u32(vget_low_u32(vreinterpretq_u32_f32(V1)), vget_low_u32(vreinterpretq_u32_f32(V2)));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 3) == 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector2EqualIntR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    uint32_t CR = 0;
    if ((V1.vector4_u32[0] == V2.vector4_u32[0]) &&
        (V1.vector4_u32[1] == V2.vector4_u32[1]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_u32[0] != V2.vector4_u32[0]) &&
        (V1.vector4_u32[1] != V2.vector4_u32[1]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vceq_u32(vget_low_u32(vreinterpretq_u32_f32(V1)), vget_low_u32(vreinterpretq_u32_f32(V2)));
    uint64_t r = vget_lane_u64(vreinterpret_u64_u32(vTemp), 0);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    int iTest = _mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 3;
    uint32_t CR = 0;
    if (iTest == 3)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2NearEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR Epsilon
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float dx = fabsf(V1.vector4_f32[0] - V2.vector4_f32[0]);
    float dy = fabsf(V1.vector4_f32[1] - V2.vector4_f32[1]);
    return ((dx <= Epsilon.vector4_f32[0]) &&
        (dy <= Epsilon.vector4_f32[1]));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t vDelta = vsub_f32(vget_low_f32(V1), vget_low_f32(V2));
#if defined(_MSC_VER) && !defined(__clang__) && !defined(_ARM64_DISTINCT_NEON_TYPES)
    uint32x2_t vTemp = vacle_f32(vDelta, vget_low_u32(Epsilon));
#else
    uint32x2_t vTemp = vcle_f32(vabs_f32(vDelta), vget_low_f32(Epsilon));
#endif
    uint64_t r = vget_lane_u64(vreinterpret_u64_u32(vTemp), 0);
    return (r == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    // Get the difference
    XMVECTOR vDelta = _mm_sub_ps(V1, V2);
    // Get the absolute value of the difference
    XMVECTOR vTemp = _mm_setzero_ps();
    vTemp = _mm_sub_ps(vTemp, vDelta);
    vTemp = _mm_max_ps(vTemp, vDelta);
    vTemp = _mm_cmple_ps(vTemp, Epsilon);
    // z and w are don't care
    return (((_mm_movemask_ps(vTemp) & 3) == 0x3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2NotEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] != V2.vector4_f32[0]) || (V1.vector4_f32[1] != V2.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vceq_f32(vget_low_f32(V1), vget_low_f32(V2));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) != 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    // z and w are don't care
    return (((_mm_movemask_ps(vTemp) & 3) != 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2NotEqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_u32[0] != V2.vector4_u32[0]) || (V1.vector4_u32[1] != V2.vector4_u32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vceq_u32(vget_low_u32(vreinterpretq_u32_f32(V1)), vget_low_u32(vreinterpretq_u32_f32(V2)));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) != 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 3) != 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2Greater
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] > V2.vector4_f32[0]) && (V1.vector4_f32[1] > V2.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vcgt_f32(vget_low_f32(V1), vget_low_f32(V2));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    // z and w are don't care
    return (((_mm_movemask_ps(vTemp) & 3) == 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector2GreaterR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    uint32_t CR = 0;
    if ((V1.vector4_f32[0] > V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] > V2.vector4_f32[1]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] <= V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] <= V2.vector4_f32[1]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vcgt_f32(vget_low_f32(V1), vget_low_f32(V2));
    uint64_t r = vget_lane_u64(vreinterpret_u64_u32(vTemp), 0);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    int iTest = _mm_movemask_ps(vTemp) & 3;
    uint32_t CR = 0;
    if (iTest == 3)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2GreaterOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] >= V2.vector4_f32[0]) && (V1.vector4_f32[1] >= V2.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vcge_f32(vget_low_f32(V1), vget_low_f32(V2));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 3) == 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector2GreaterOrEqualR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    uint32_t CR = 0;
    if ((V1.vector4_f32[0] >= V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] >= V2.vector4_f32[1]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] < V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] < V2.vector4_f32[1]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vcge_f32(vget_low_f32(V1), vget_low_f32(V2));
    uint64_t r = vget_lane_u64(vreinterpret_u64_u32(vTemp), 0);
    uint32_t CR = 0;
    if (r == 0xFFFFFFFFFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    int iTest = _mm_movemask_ps(vTemp) & 3;
    uint32_t CR = 0;
    if (iTest == 3)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2Less
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] < V2.vector4_f32[0]) && (V1.vector4_f32[1] < V2.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vclt_f32(vget_low_f32(V1), vget_low_f32(V2));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmplt_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 3) == 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2LessOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] <= V2.vector4_f32[0]) && (V1.vector4_f32[1] <= V2.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vTemp = vcle_f32(vget_low_f32(V1), vget_low_f32(V2));
    return (vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmple_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 3) == 3) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2InBounds
(
    FXMVECTOR V,
    FXMVECTOR Bounds
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) &&
        (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    float32x2_t B = vget_low_f32(Bounds);
    // Test if less than or equal
    uint32x2_t ivTemp1 = vcle_f32(VL, B);
    // Negate the bounds
    float32x2_t vTemp2 = vneg_f32(B);
    // Test if greater or equal (Reversed)
    uint32x2_t ivTemp2 = vcle_f32(vTemp2, VL);
    // Blend answers
    ivTemp1 = vand_u32(ivTemp1, ivTemp2);
    // x and y in bounds?
    return (vget_lane_u64(vreinterpret_u64_u32(ivTemp1), 0) == 0xFFFFFFFFFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    // Test if less than or equal
    XMVECTOR vTemp1 = _mm_cmple_ps(V, Bounds);
    // Negate the bounds
    XMVECTOR vTemp2 = _mm_mul_ps(Bounds, g_XMNegativeOne);
    // Test if greater or equal (Reversed)
    vTemp2 = _mm_cmple_ps(vTemp2, V);
    // Blend answers
    vTemp1 = _mm_and_ps(vTemp1, vTemp2);
    // x and y in bounds? (z and w are don't care)
    return (((_mm_movemask_ps(vTemp1) & 0x3) == 0x3) != 0);
#endif
}

//------------------------------------------------------------------------------

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(push)
#pragma float_control(precise, on)
#endif

inline bool XM_CALLCONV XMVector2IsNaN(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (XMISNAN(V.vector4_f32[0]) ||
        XMISNAN(V.vector4_f32[1]));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    return isnan(vgetq_lane_f32(V, 0)) || isnan(vgetq_lane_f32(V, 1));
    #else
    float32x2_t VL = vget_low_f32(V);
    // Test against itself. NaN is always not equal
    uint32x2_t vTempNan = vceq_f32(VL, VL);
    // If x or y are NaN, the mask is zero
    return (vget_lane_u64(vreinterpret_u64_u32(vTempNan), 0) != 0xFFFFFFFFFFFFFFFFU);
    #endif
#elif defined(_XM_SSE_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    XM_ALIGNED_DATA(16) float tmp[4];
    _mm_store_ps(tmp, V);
    return isnan(tmp[0]) || isnan(tmp[1]);
    #else
    // Test against itself. NaN is always not equal
    XMVECTOR vTempNan = _mm_cmpneq_ps(V, V);
    // If x or y are NaN, the mask is non-zero
    return ((_mm_movemask_ps(vTempNan) & 3) != 0);
    #endif
#endif
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(pop)
#endif

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector2IsInfinite(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    return (XMISINF(V.vector4_f32[0]) ||
        XMISINF(V.vector4_f32[1]));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Mask off the sign bit
    uint32x2_t vTemp = vand_u32(vget_low_u32(vreinterpretq_u32_f32(V)), vget_low_u32(g_XMAbsMask));
    // Compare to infinity
    vTemp = vceq_f32(vreinterpret_f32_u32(vTemp), vget_low_f32(g_XMInfinity));
    // If any are infinity, the signs are true.
    return vget_lane_u64(vreinterpret_u64_u32(vTemp), 0) != 0;
#elif defined(_XM_SSE_INTRINSICS_)
    // Mask off the sign bit
    __m128 vTemp = _mm_and_ps(V, g_XMAbsMask);
    // Compare to infinity
    vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity);
    // If x or z are infinity, the signs are true.
    return ((_mm_movemask_ps(vTemp) & 3) != 0);
#endif
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Dot
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result;
    Result.f[0] =
        Result.f[1] =
        Result.f[2] =
        Result.f[3] = V1.vector4_f32[0] * V2.vector4_f32[0] + V1.vector4_f32[1] * V2.vector4_f32[1];
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Perform the dot product on x and y
    float32x2_t vTemp = vmul_f32(vget_low_f32(V1), vget_low_f32(V2));
    vTemp = vpadd_f32(vTemp, vTemp);
    return vcombine_f32(vTemp, vTemp);
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_dp_ps(V1, V2, 0x3f);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vDot = _mm_mul_ps(V1, V2);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_moveldup_ps(vDot);
    return vDot;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y
    XMVECTOR vLengthSq = _mm_mul_ps(V1, V2);
    // vTemp has y splatted
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    // x+y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Cross
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    // [ V1.x*V2.y - V1.y*V2.x, V1.x*V2.y - V1.y*V2.x ]

#if defined(_XM_NO_INTRINSICS_)
    float fCross = (V1.vector4_f32[0] * V2.vector4_f32[1]) - (V1.vector4_f32[1] * V2.vector4_f32[0]);
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = fCross;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Negate = { { { 1.f, -1.f, 0, 0 } } };

    float32x2_t vTemp = vmul_f32(vget_low_f32(V1), vrev64_f32(vget_low_f32(V2)));
    vTemp = vmul_f32(vTemp, vget_low_f32(Negate));
    vTemp = vpadd_f32(vTemp, vTemp);
    return vcombine_f32(vTemp, vTemp);
#elif defined(_XM_SSE_INTRINSICS_)
    // Swap x and y
    XMVECTOR vResult = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 1, 0, 1));
    // Perform the muls
    vResult = _mm_mul_ps(vResult, V1);
    // Splat y
    XMVECTOR vTemp = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(1, 1, 1, 1));
    // Sub the values
    vResult = _mm_sub_ss(vResult, vTemp);
    // Splat the cross product
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 0, 0, 0));
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2LengthSq(FXMVECTOR V) noexcept
{
    return XMVector2Dot(V, V);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2ReciprocalLengthEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector2LengthSq(V);
    Result = XMVectorReciprocalSqrtEst(Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    // Dot2
    float32x2_t vTemp = vmul_f32(VL, VL);
    vTemp = vpadd_f32(vTemp, vTemp);
    // Reciprocal sqrt (estimate)
    vTemp = vrsqrte_f32(vTemp);
    return vcombine_f32(vTemp, vTemp);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x3f);
    return _mm_rsqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    XMVECTOR vTemp = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_rsqrt_ss(vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has y splatted
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    // x+y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = _mm_rsqrt_ss(vLengthSq);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2ReciprocalLength(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector2LengthSq(V);
    Result = XMVectorReciprocalSqrt(Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    // Dot2
    float32x2_t vTemp = vmul_f32(VL, VL);
    vTemp = vpadd_f32(vTemp, vTemp);
    // Reciprocal sqrt
    float32x2_t  S0 = vrsqrte_f32(vTemp);
    float32x2_t  P0 = vmul_f32(vTemp, S0);
    float32x2_t  R0 = vrsqrts_f32(P0, S0);
    float32x2_t  S1 = vmul_f32(S0, R0);
    float32x2_t  P1 = vmul_f32(vTemp, S1);
    float32x2_t  R1 = vrsqrts_f32(P1, S1);
    float32x2_t Result = vmul_f32(S1, R1);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x3f);
    XMVECTOR vLengthSq = _mm_sqrt_ps(vTemp);
    return _mm_div_ps(g_XMOne, vLengthSq);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    XMVECTOR vTemp = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ss(vTemp);
    vLengthSq = _mm_div_ss(g_XMOne, vLengthSq);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has y splatted
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    // x+y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = _mm_sqrt_ss(vLengthSq);
    vLengthSq = _mm_div_ss(g_XMOne, vLengthSq);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2LengthEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector2LengthSq(V);
    Result = XMVectorSqrtEst(Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    // Dot2
    float32x2_t vTemp = vmul_f32(VL, VL);
    vTemp = vpadd_f32(vTemp, vTemp);
    const float32x2_t zero = vdup_n_f32(0);
    uint32x2_t VEqualsZero = vceq_f32(vTemp, zero);
    // Sqrt (estimate)
    float32x2_t Result = vrsqrte_f32(vTemp);
    Result = vmul_f32(vTemp, Result);
    Result = vbsl_f32(VEqualsZero, zero, Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x3f);
    return _mm_sqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    XMVECTOR vTemp = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ss(vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has y splatted
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    // x+y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = _mm_sqrt_ss(vLengthSq);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Length(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector2LengthSq(V);
    Result = XMVectorSqrt(Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    // Dot2
    float32x2_t vTemp = vmul_f32(VL, VL);
    vTemp = vpadd_f32(vTemp, vTemp);
    const float32x2_t zero = vdup_n_f32(0);
    uint32x2_t VEqualsZero = vceq_f32(vTemp, zero);
    // Sqrt
    float32x2_t S0 = vrsqrte_f32(vTemp);
    float32x2_t P0 = vmul_f32(vTemp, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(vTemp, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    float32x2_t Result = vmul_f32(S1, R1);
    Result = vmul_f32(vTemp, Result);
    Result = vbsl_f32(VEqualsZero, zero, Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x3f);
    return _mm_sqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    XMVECTOR vTemp = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ss(vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has y splatted
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    // x+y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------
// XMVector2NormalizeEst uses a reciprocal estimate and
// returns QNaN on zero and infinite vectors.

inline XMVECTOR XM_CALLCONV XMVector2NormalizeEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector2ReciprocalLength(V);
    Result = XMVectorMultiply(V, Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    // Dot2
    float32x2_t vTemp = vmul_f32(VL, VL);
    vTemp = vpadd_f32(vTemp, vTemp);
    // Reciprocal sqrt (estimate)
    vTemp = vrsqrte_f32(vTemp);
    // Normalize
    float32x2_t Result = vmul_f32(VL, vTemp);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x3f);
    XMVECTOR vResult = _mm_rsqrt_ps(vTemp);
    return _mm_mul_ps(vResult, V);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_rsqrt_ss(vLengthSq);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    vLengthSq = _mm_mul_ps(vLengthSq, V);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has y splatted
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    // x+y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = _mm_rsqrt_ss(vLengthSq);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    vLengthSq = _mm_mul_ps(vLengthSq, V);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Normalize(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR vResult = XMVector2Length(V);
    float fLength = vResult.vector4_f32[0];

    // Prevent divide by zero
    if (fLength > 0)
    {
        fLength = 1.0f / fLength;
    }

    vResult.vector4_f32[0] = V.vector4_f32[0] * fLength;
    vResult.vector4_f32[1] = V.vector4_f32[1] * fLength;
    vResult.vector4_f32[2] = V.vector4_f32[2] * fLength;
    vResult.vector4_f32[3] = V.vector4_f32[3] * fLength;
    return vResult;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    // Dot2
    float32x2_t vTemp = vmul_f32(VL, VL);
    vTemp = vpadd_f32(vTemp, vTemp);
    uint32x2_t VEqualsZero = vceq_f32(vTemp, vdup_n_f32(0));
    uint32x2_t VEqualsInf = vceq_f32(vTemp, vget_low_f32(g_XMInfinity));
    // Reciprocal sqrt (2 iterations of Newton-Raphson)
    float32x2_t S0 = vrsqrte_f32(vTemp);
    float32x2_t P0 = vmul_f32(vTemp, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(vTemp, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    vTemp = vmul_f32(S1, R1);
    // Normalize
    float32x2_t Result = vmul_f32(VL, vTemp);
    Result = vbsl_f32(VEqualsZero, vdup_n_f32(0), Result);
    Result = vbsl_f32(VEqualsInf, vget_low_f32(g_XMQNaN), Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_dp_ps(V, V, 0x3f);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Reciprocal mul to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#elif defined(_XM_SSE3_INTRINSICS_)
    // Perform the dot product on x and y only
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_moveldup_ps(vLengthSq);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Reciprocal mul to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x and y only
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 1, 1, 1));
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Reciprocal mul to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2ClampLength
(
    FXMVECTOR V,
    float    LengthMin,
    float    LengthMax
) noexcept
{
    XMVECTOR ClampMax = XMVectorReplicate(LengthMax);
    XMVECTOR ClampMin = XMVectorReplicate(LengthMin);
    return XMVector2ClampLengthV(V, ClampMin, ClampMax);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2ClampLengthV
(
    FXMVECTOR V,
    FXMVECTOR LengthMin,
    FXMVECTOR LengthMax
) noexcept
{
    assert((XMVectorGetY(LengthMin) == XMVectorGetX(LengthMin)));
    assert((XMVectorGetY(LengthMax) == XMVectorGetX(LengthMax)));
    assert(XMVector2GreaterOrEqual(LengthMin, g_XMZero));
    assert(XMVector2GreaterOrEqual(LengthMax, g_XMZero));
    assert(XMVector2GreaterOrEqual(LengthMax, LengthMin));

    XMVECTOR LengthSq = XMVector2LengthSq(V);

    const XMVECTOR Zero = XMVectorZero();

    XMVECTOR RcpLength = XMVectorReciprocalSqrt(LengthSq);

    XMVECTOR InfiniteLength = XMVectorEqualInt(LengthSq, g_XMInfinity.v);
    XMVECTOR ZeroLength = XMVectorEqual(LengthSq, Zero);

    XMVECTOR Length = XMVectorMultiply(LengthSq, RcpLength);

    XMVECTOR Normal = XMVectorMultiply(V, RcpLength);

    XMVECTOR Select = XMVectorEqualInt(InfiniteLength, ZeroLength);
    Length = XMVectorSelect(LengthSq, Length, Select);
    Normal = XMVectorSelect(LengthSq, Normal, Select);

    XMVECTOR ControlMax = XMVectorGreater(Length, LengthMax);
    XMVECTOR ControlMin = XMVectorLess(Length, LengthMin);

    XMVECTOR ClampLength = XMVectorSelect(Length, LengthMax, ControlMax);
    ClampLength = XMVectorSelect(ClampLength, LengthMin, ControlMin);

    XMVECTOR Result = XMVectorMultiply(Normal, ClampLength);

    // Preserve the original vector (with no precision loss) if the length falls within the given range
    XMVECTOR Control = XMVectorEqualInt(ControlMax, ControlMin);
    Result = XMVectorSelect(Result, V, Control);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Reflect
(
    FXMVECTOR Incident,
    FXMVECTOR Normal
) noexcept
{
    // Result = Incident - (2 * dot(Incident, Normal)) * Normal

    XMVECTOR Result;
    Result = XMVector2Dot(Incident, Normal);
    Result = XMVectorAdd(Result, Result);
    Result = XMVectorNegativeMultiplySubtract(Result, Normal, Incident);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Refract
(
    FXMVECTOR Incident,
    FXMVECTOR Normal,
    float    RefractionIndex
) noexcept
{
    XMVECTOR Index = XMVectorReplicate(RefractionIndex);
    return XMVector2RefractV(Incident, Normal, Index);
}

//------------------------------------------------------------------------------

// Return the refraction of a 2D vector
inline XMVECTOR XM_CALLCONV XMVector2RefractV
(
    FXMVECTOR Incident,
    FXMVECTOR Normal,
    FXMVECTOR RefractionIndex
) noexcept
{
    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))

#if defined(_XM_NO_INTRINSICS_)

    float IDotN = (Incident.vector4_f32[0] * Normal.vector4_f32[0]) + (Incident.vector4_f32[1] * Normal.vector4_f32[1]);
    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    float RY = 1.0f - (IDotN * IDotN);
    float RX = 1.0f - (RY * RefractionIndex.vector4_f32[0] * RefractionIndex.vector4_f32[0]);
    RY = 1.0f - (RY * RefractionIndex.vector4_f32[1] * RefractionIndex.vector4_f32[1]);
    if (RX >= 0.0f)
    {
        RX = (RefractionIndex.vector4_f32[0] * Incident.vector4_f32[0]) - (Normal.vector4_f32[0] * ((RefractionIndex.vector4_f32[0] * IDotN) + sqrtf(RX)));
    }
    else
    {
        RX = 0.0f;
    }
    if (RY >= 0.0f)
    {
        RY = (RefractionIndex.vector4_f32[1] * Incident.vector4_f32[1]) - (Normal.vector4_f32[1] * ((RefractionIndex.vector4_f32[1] * IDotN) + sqrtf(RY)));
    }
    else
    {
        RY = 0.0f;
    }

    XMVECTOR vResult;
    vResult.vector4_f32[0] = RX;
    vResult.vector4_f32[1] = RY;
    vResult.vector4_f32[2] = 0.0f;
    vResult.vector4_f32[3] = 0.0f;
    return vResult;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t IL = vget_low_f32(Incident);
    float32x2_t NL = vget_low_f32(Normal);
    float32x2_t RIL = vget_low_f32(RefractionIndex);
    // Get the 2D Dot product of Incident-Normal
    float32x2_t vTemp = vmul_f32(IL, NL);
    float32x2_t IDotN = vpadd_f32(vTemp, vTemp);
    // vTemp = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    vTemp = vmls_f32(vget_low_f32(g_XMOne), IDotN, IDotN);
    vTemp = vmul_f32(vTemp, RIL);
    vTemp = vmls_f32(vget_low_f32(g_XMOne), vTemp, RIL);
    // If any terms are <=0, sqrt() will fail, punt to zero
    uint32x2_t vMask = vcgt_f32(vTemp, vget_low_f32(g_XMZero));
    // Sqrt(vTemp)
    float32x2_t S0 = vrsqrte_f32(vTemp);
    float32x2_t P0 = vmul_f32(vTemp, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(vTemp, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    float32x2_t S2 = vmul_f32(S1, R1);
    vTemp = vmul_f32(vTemp, S2);
    // R = RefractionIndex * IDotN + sqrt(R)
    vTemp = vmla_f32(vTemp, RIL, IDotN);
    // Result = RefractionIndex * Incident - Normal * R
    float32x2_t vResult = vmul_f32(RIL, IL);
    vResult = vmls_f32(vResult, vTemp, NL);
    vResult = vreinterpret_f32_u32(vand_u32(vreinterpret_u32_f32(vResult), vMask));
    return vcombine_f32(vResult, vResult);
#elif defined(_XM_SSE_INTRINSICS_)
    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))
    // Get the 2D Dot product of Incident-Normal
    XMVECTOR IDotN = XMVector2Dot(Incident, Normal);
    // vTemp = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    XMVECTOR vTemp = XM_FNMADD_PS(IDotN, IDotN, g_XMOne);
    vTemp = _mm_mul_ps(vTemp, RefractionIndex);
    vTemp = XM_FNMADD_PS(vTemp, RefractionIndex, g_XMOne);
    // If any terms are <=0, sqrt() will fail, punt to zero
    XMVECTOR vMask = _mm_cmpgt_ps(vTemp, g_XMZero);
    // R = RefractionIndex * IDotN + sqrt(R)
    vTemp = _mm_sqrt_ps(vTemp);
    vTemp = XM_FMADD_PS(RefractionIndex, IDotN, vTemp);
    // Result = RefractionIndex * Incident - Normal * R
    XMVECTOR vResult = _mm_mul_ps(RefractionIndex, Incident);
    vResult = XM_FNMADD_PS(vTemp, Normal, vResult);
    vResult = _mm_and_ps(vResult, vMask);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Orthogonal(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            -V.vector4_f32[1],
            V.vector4_f32[0],
            0.f,
            0.f
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Negate = { { { -1.f, 1.f, 0, 0 } } };
    const float32x2_t zero = vdup_n_f32(0);

    float32x2_t VL = vget_low_f32(V);
    float32x2_t Result = vmul_f32(vrev64_f32(VL), vget_low_f32(Negate));
    return vcombine_f32(Result, zero);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 2, 0, 1));
    vResult = _mm_mul_ps(vResult, g_XMNegateX);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2AngleBetweenNormalsEst
(
    FXMVECTOR N1,
    FXMVECTOR N2
) noexcept
{
    XMVECTOR Result = XMVector2Dot(N1, N2);
    Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
    Result = XMVectorACosEst(Result);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2AngleBetweenNormals
(
    FXMVECTOR N1,
    FXMVECTOR N2
) noexcept
{
    XMVECTOR Result = XMVector2Dot(N1, N2);
    Result = XMVectorClamp(Result, g_XMNegativeOne, g_XMOne);
    Result = XMVectorACos(Result);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2AngleBetweenVectors
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    XMVECTOR L1 = XMVector2ReciprocalLength(V1);
    XMVECTOR L2 = XMVector2ReciprocalLength(V2);

    XMVECTOR Dot = XMVector2Dot(V1, V2);

    L1 = XMVectorMultiply(L1, L2);

    XMVECTOR CosAngle = XMVectorMultiply(Dot, L1);
    CosAngle = XMVectorClamp(CosAngle, g_XMNegativeOne.v, g_XMOne.v);

    return XMVectorACos(CosAngle);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2LinePointDistance
(
    FXMVECTOR LinePoint1,
    FXMVECTOR LinePoint2,
    FXMVECTOR Point
) noexcept
{
    // Given a vector PointVector from LinePoint1 to Point and a vector
    // LineVector from LinePoint1 to LinePoint2, the scaled distance
    // PointProjectionScale from LinePoint1 to the perpendicular projection
    // of PointVector onto the line is defined as:
    //
    //     PointProjectionScale = dot(PointVector, LineVector) / LengthSq(LineVector)

    XMVECTOR PointVector = XMVectorSubtract(Point, LinePoint1);
    XMVECTOR LineVector = XMVectorSubtract(LinePoint2, LinePoint1);

    XMVECTOR LengthSq = XMVector2LengthSq(LineVector);

    XMVECTOR PointProjectionScale = XMVector2Dot(PointVector, LineVector);
    PointProjectionScale = XMVectorDivide(PointProjectionScale, LengthSq);

    XMVECTOR DistanceVector = XMVectorMultiply(LineVector, PointProjectionScale);
    DistanceVector = XMVectorSubtract(PointVector, DistanceVector);

    return XMVector2Length(DistanceVector);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2IntersectLine
(
    FXMVECTOR Line1Point1,
    FXMVECTOR Line1Point2,
    FXMVECTOR Line2Point1,
    GXMVECTOR Line2Point2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_) || defined(_XM_ARM_NEON_INTRINSICS_)

    XMVECTOR V1 = XMVectorSubtract(Line1Point2, Line1Point1);
    XMVECTOR V2 = XMVectorSubtract(Line2Point2, Line2Point1);
    XMVECTOR V3 = XMVectorSubtract(Line1Point1, Line2Point1);

    XMVECTOR C1 = XMVector2Cross(V1, V2);
    XMVECTOR C2 = XMVector2Cross(V2, V3);

    XMVECTOR Result;
    const XMVECTOR Zero = XMVectorZero();
    if (XMVector2NearEqual(C1, Zero, g_XMEpsilon.v))
    {
        if (XMVector2NearEqual(C2, Zero, g_XMEpsilon.v))
        {
            // Coincident
            Result = g_XMInfinity.v;
        }
        else
        {
            // Parallel
            Result = g_XMQNaN.v;
        }
    }
    else
    {
        // Intersection point = Line1Point1 + V1 * (C2 / C1)
        XMVECTOR Scale = XMVectorReciprocal(C1);
        Scale = XMVectorMultiply(C2, Scale);
        Result = XMVectorMultiplyAdd(V1, Scale, Line1Point1);
    }

    return Result;

#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR V1 = _mm_sub_ps(Line1Point2, Line1Point1);
    XMVECTOR V2 = _mm_sub_ps(Line2Point2, Line2Point1);
    XMVECTOR V3 = _mm_sub_ps(Line1Point1, Line2Point1);
    // Generate the cross products
    XMVECTOR C1 = XMVector2Cross(V1, V2);
    XMVECTOR C2 = XMVector2Cross(V2, V3);
    // If C1 is not close to epsilon, use the calculated value
    XMVECTOR vResultMask = _mm_setzero_ps();
    vResultMask = _mm_sub_ps(vResultMask, C1);
    vResultMask = _mm_max_ps(vResultMask, C1);
    // 0xFFFFFFFF if the calculated value is to be used
    vResultMask = _mm_cmpgt_ps(vResultMask, g_XMEpsilon);
    // If C1 is close to epsilon, which fail type is it? INFINITY or NAN?
    XMVECTOR vFailMask = _mm_setzero_ps();
    vFailMask = _mm_sub_ps(vFailMask, C2);
    vFailMask = _mm_max_ps(vFailMask, C2);
    vFailMask = _mm_cmple_ps(vFailMask, g_XMEpsilon);
    XMVECTOR vFail = _mm_and_ps(vFailMask, g_XMInfinity);
    vFailMask = _mm_andnot_ps(vFailMask, g_XMQNaN);
    // vFail is NAN or INF
    vFail = _mm_or_ps(vFail, vFailMask);
    // Intersection point = Line1Point1 + V1 * (C2 / C1)
    XMVECTOR vResult = _mm_div_ps(C2, C1);
    vResult = XM_FMADD_PS(vResult, V1, Line1Point1);
    // Use result, or failure value
    vResult = _mm_and_ps(vResult, vResultMask);
    vResultMask = _mm_andnot_ps(vResultMask, vFail);
    vResult = _mm_or_ps(vResult, vResultMask);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2Transform
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Y = XMVectorSplatY(V);
    XMVECTOR X = XMVectorSplatX(V);

    XMVECTOR Result = XMVectorMultiplyAdd(Y, M.r[1], M.r[3]);
    Result = XMVectorMultiplyAdd(X, M.r[0], Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    float32x4_t Result = vmlaq_lane_f32(M.r[3], M.r[1], VL, 1); // Y
    return vmlaq_lane_f32(Result, M.r[0], VL, 0); // X
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
    vResult = XM_FMADD_PS(vResult, M.r[1], M.r[3]);
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
    vResult = XM_FMADD_PS(vTemp, M.r[0], vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMFLOAT4* XM_CALLCONV XMVector2TransformStream
(
    XMFLOAT4* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT2* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT2));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT2));

    assert(OutputStride >= sizeof(XMFLOAT4));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT4));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row3 = M.r[3];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pInputVector));
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiplyAdd(Y, row1, row3);
        Result = XMVectorMultiplyAdd(X, row0, Result);

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015, "PREfast noise: Esp:1307" )
#endif

        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(pOutputVector), Result);

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT2)) && (OutputStride == sizeof(XMFLOAT4)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x2_t V = vld2q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT2) * 4;

                float32x2_t r3 = vget_low_f32(row3);
                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Ax+M
                XMVECTOR vResult1 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Bx+N

                XM_PREFETCH(pInputVector);

                r3 = vget_high_f32(row3);
                r = vget_high_f32(row0);
                XMVECTOR vResult2 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Cx+O
                XMVECTOR vResult3 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Dx+P

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(row1);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[1], r, 0); // Cx+Gy+O
                vResult3 = vmlaq_lane_f32(vResult3, V.val[1], r, 1); // Dx+Hy+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                float32x4x4_t R;
                R.val[0] = vResult0;
                R.val[1] = vResult1;
                R.val[2] = vResult2;
                R.val[3] = vResult3;

                vst4q_f32(reinterpret_cast<float*>(pOutputVector), R);
                pOutputVector += sizeof(XMFLOAT4) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        float32x2_t V = vld1_f32(reinterpret_cast<const float*>(pInputVector));
        pInputVector += InputStride;

        XMVECTOR vResult = vmlaq_lane_f32(row3, row0, V, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, V, 1); // Y

        vst1q_f32(reinterpret_cast<float*>(pOutputVector), vResult);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_AVX2_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        __m256 row0 = _mm256_broadcast_ps(&M.r[0]);
        __m256 row1 = _mm256_broadcast_ps(&M.r[1]);
        __m256 row3 = _mm256_broadcast_ps(&M.r[3]);

        if (InputStride == sizeof(XMFLOAT2))
        {
            if (OutputStride == sizeof(XMFLOAT4))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0x1F))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 4;

                        __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                        __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                        __m256 vTempB = _mm256_fmadd_ps(Y1, row1, row3);
                        __m256 vTempB2 = _mm256_fmadd_ps(Y2, row1, row3);
                        __m256 vTempA = _mm256_mul_ps(X1, row0);
                        __m256 vTempA2 = _mm256_mul_ps(X2, row0);
                        vTempA = _mm256_add_ps(vTempA, vTempB);
                        vTempA2 = _mm256_add_ps(vTempA2, vTempB2);

                        X1 = _mm256_insertf128_ps(vTempA, _mm256_castps256_ps128(vTempA2), 1);
                        XM256_STREAM_PS(reinterpret_cast<float*>(pOutputVector), X1);
                        pOutputVector += sizeof(XMFLOAT4) * 2;

                        X2 = _mm256_insertf128_ps(vTempA2, _mm256_extractf128_ps(vTempA, 1), 0);
                        XM256_STREAM_PS(reinterpret_cast<float*>(pOutputVector), X2);
                        pOutputVector += sizeof(XMFLOAT4) * 2;

                        i += 4;
                    }
                }
                else
                {
                    // Packed input, packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 4;

                        __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                        __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                        __m256 vTempB = _mm256_fmadd_ps(Y1, row1, row3);
                        __m256 vTempB2 = _mm256_fmadd_ps(Y2, row1, row3);
                        __m256 vTempA = _mm256_mul_ps(X1, row0);
                        __m256 vTempA2 = _mm256_mul_ps(X2, row0);
                        vTempA = _mm256_add_ps(vTempA, vTempB);
                        vTempA2 = _mm256_add_ps(vTempA2, vTempB2);

                        X1 = _mm256_insertf128_ps(vTempA, _mm256_castps256_ps128(vTempA2), 1);
                        _mm256_storeu_ps(reinterpret_cast<float*>(pOutputVector), X1);
                        pOutputVector += sizeof(XMFLOAT4) * 2;

                        X2 = _mm256_insertf128_ps(vTempA2, _mm256_extractf128_ps(vTempA, 1), 0);
                        _mm256_storeu_ps(reinterpret_cast<float*>(pOutputVector), X2);
                        pOutputVector += sizeof(XMFLOAT4) * 2;

                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 4;

                    __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                    __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                    __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                    __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                    __m256 vTempB = _mm256_fmadd_ps(Y1, row1, row3);
                    __m256 vTempB2 = _mm256_fmadd_ps(Y2, row1, row3);
                    __m256 vTempA = _mm256_mul_ps(X1, row0);
                    __m256 vTempA2 = _mm256_mul_ps(X2, row0);
                    vTempA = _mm256_add_ps(vTempA, vTempB);
                    vTempA2 = _mm256_add_ps(vTempA2, vTempB2);

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), _mm256_castps256_ps128(vTempA));
                    pOutputVector += OutputStride;

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), _mm256_castps256_ps128(vTempA2));
                    pOutputVector += OutputStride;

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), _mm256_extractf128_ps(vTempA, 1));
                    pOutputVector += OutputStride;

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), _mm256_extractf128_ps(vTempA2, 1));
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    if (i < VectorCount)
    {
        const XMVECTOR row0 = M.r[0];
        const XMVECTOR row1 = M.r[1];
        const XMVECTOR row3 = M.r[3];

        for (; i < VectorCount; i++)
        {
            __m128 xy = _mm_castpd_ps(_mm_load_sd(reinterpret_cast<const double*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(xy, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(xy, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);

            _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t two = VectorCount >> 1;
    if (two > 0)
    {
        if (InputStride == sizeof(XMFLOAT2))
        {
            if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF) && !(OutputStride & 0xF))
            {
                // Packed input, aligned output
                for (size_t j = 0; j < two; ++j)
                {
                    XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 2;

                    XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                    XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);

                    XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                    X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                    vTemp = XM_FMADD_PS(Y, row1, row3);
                    vTemp2 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);

                    XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 2;
                }
            }
            else
            {
                // Packed input, unaligned output
                for (size_t j = 0; j < two; ++j)
                {
                    XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 2;

                    XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                    XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                    X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                    vTemp = XM_FMADD_PS(Y, row1, row3);
                    vTemp2 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 2;
                }
            }
        }
    }

    if (!(reinterpret_cast<uintptr_t>(pInputVector) & 0xF) && !(InputStride & 0xF))
    {
        if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF) && !(OutputStride & 0xF))
        {
            // Aligned input, aligned output
            for (; i < VectorCount; i++)
            {
                XMVECTOR V = _mm_castsi128_ps(_mm_loadl_epi64(reinterpret_cast<const __m128i*>(pInputVector)));
                pInputVector += InputStride;

                XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                vTemp = _mm_add_ps(vTemp, vTemp2);

                XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                pOutputVector += OutputStride;
            }
        }
        else
        {
            // Aligned input, unaligned output
            for (; i < VectorCount; i++)
            {
                XMVECTOR V = _mm_castsi128_ps(_mm_loadl_epi64(reinterpret_cast<const __m128i*>(pInputVector)));
                pInputVector += InputStride;

                XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                vTemp = _mm_add_ps(vTemp, vTemp2);

                _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                pOutputVector += OutputStride;
            }
        }
    }
    else
    {
        // Unaligned input
        for (; i < VectorCount; i++)
        {
            __m128 xy = _mm_castpd_ps(_mm_load_sd(reinterpret_cast<const double*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(xy, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(xy, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);

            _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2TransformCoord
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
    XMVECTOR Y = XMVectorSplatY(V);
    XMVECTOR X = XMVectorSplatX(V);

    XMVECTOR Result = XMVectorMultiplyAdd(Y, M.r[1], M.r[3]);
    Result = XMVectorMultiplyAdd(X, M.r[0], Result);

    XMVECTOR W = XMVectorSplatW(Result);
    return XMVectorDivide(Result, W);
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMFLOAT2* XM_CALLCONV XMVector2TransformCoordStream
(
    XMFLOAT2* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT2* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT2));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT2));

    assert(OutputStride >= sizeof(XMFLOAT2));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT2));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row3 = M.r[3];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pInputVector));
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiplyAdd(Y, row1, row3);
        Result = XMVectorMultiplyAdd(X, row0, Result);

        XMVECTOR W = XMVectorSplatW(Result);

        Result = XMVectorDivide(Result, W);

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015, "PREfast noise: Esp:1307" )
#endif

        XMStoreFloat2(reinterpret_cast<XMFLOAT2*>(pOutputVector), Result);

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT2)) && (OutputStride == sizeof(XMFLOAT2)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x2_t V = vld2q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT2) * 4;

                float32x2_t r3 = vget_low_f32(row3);
                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Ax+M
                XMVECTOR vResult1 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Bx+N

                XM_PREFETCH(pInputVector);

                r3 = vget_high_f32(row3);
                r = vget_high_f32(row0);
                XMVECTOR W = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Dx+P

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(row1);
                W = vmlaq_lane_f32(W, V.val[1], r, 1); // Dx+Hy+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
                V.val[0] = vdivq_f32(vResult0, W);
                V.val[1] = vdivq_f32(vResult1, W);
#else
                // 2 iterations of Newton-Raphson refinement of reciprocal
                float32x4_t Reciprocal = vrecpeq_f32(W);
                float32x4_t S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);
                S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);

                V.val[0] = vmulq_f32(vResult0, Reciprocal);
                V.val[1] = vmulq_f32(vResult1, Reciprocal);
#endif

                vst2q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT2) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        float32x2_t V = vld1_f32(reinterpret_cast<const float*>(pInputVector));
        pInputVector += InputStride;

        XMVECTOR vResult = vmlaq_lane_f32(row3, row0, V, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, V, 1); // Y

        V = vget_high_f32(vResult);
        float32x2_t W = vdup_lane_f32(V, 1);

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
        V = vget_low_f32(vResult);
        V = vdiv_f32(V, W);
#else
        // 2 iterations of Newton-Raphson refinement of reciprocal for W
        float32x2_t Reciprocal = vrecpe_f32(W);
        float32x2_t S = vrecps_f32(Reciprocal, W);
        Reciprocal = vmul_f32(S, Reciprocal);
        S = vrecps_f32(Reciprocal, W);
        Reciprocal = vmul_f32(S, Reciprocal);

        V = vget_low_f32(vResult);
        V = vmul_f32(V, Reciprocal);
#endif

        vst1_f32(reinterpret_cast<float*>(pOutputVector), V);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_AVX2_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        __m256 row0 = _mm256_broadcast_ps(&M.r[0]);
        __m256 row1 = _mm256_broadcast_ps(&M.r[1]);
        __m256 row3 = _mm256_broadcast_ps(&M.r[3]);

        if (InputStride == sizeof(XMFLOAT2))
        {
            if (OutputStride == sizeof(XMFLOAT2))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0x1F))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 4;

                        __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                        __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                        __m256 vTempB = _mm256_fmadd_ps(Y1, row1, row3);
                        __m256 vTempB2 = _mm256_fmadd_ps(Y2, row1, row3);
                        __m256 vTempA = _mm256_mul_ps(X1, row0);
                        __m256 vTempA2 = _mm256_mul_ps(X2, row0);
                        vTempA = _mm256_add_ps(vTempA, vTempB);
                        vTempA2 = _mm256_add_ps(vTempA2, vTempB2);

                        __m256 W = _mm256_shuffle_ps(vTempA, vTempA, _MM_SHUFFLE(3, 3, 3, 3));
                        vTempA = _mm256_div_ps(vTempA, W);

                        W = _mm256_shuffle_ps(vTempA2, vTempA2, _MM_SHUFFLE(3, 3, 3, 3));
                        vTempA2 = _mm256_div_ps(vTempA2, W);

                        X1 = _mm256_shuffle_ps(vTempA, vTempA2, 0x44);
                        XM256_STREAM_PS(reinterpret_cast<float*>(pOutputVector), X1);
                        pOutputVector += sizeof(XMFLOAT2) * 4;

                        i += 4;
                    }
                }
                else
                {
                    // Packed input, packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 4;

                        __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                        __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                        __m256 vTempB = _mm256_fmadd_ps(Y1, row1, row3);
                        __m256 vTempB2 = _mm256_fmadd_ps(Y2, row1, row3);
                        __m256 vTempA = _mm256_mul_ps(X1, row0);
                        __m256 vTempA2 = _mm256_mul_ps(X2, row0);
                        vTempA = _mm256_add_ps(vTempA, vTempB);
                        vTempA2 = _mm256_add_ps(vTempA2, vTempB2);

                        __m256 W = _mm256_shuffle_ps(vTempA, vTempA, _MM_SHUFFLE(3, 3, 3, 3));
                        vTempA = _mm256_div_ps(vTempA, W);

                        W = _mm256_shuffle_ps(vTempA2, vTempA2, _MM_SHUFFLE(3, 3, 3, 3));
                        vTempA2 = _mm256_div_ps(vTempA2, W);

                        X1 = _mm256_shuffle_ps(vTempA, vTempA2, 0x44);
                        _mm256_storeu_ps(reinterpret_cast<float*>(pOutputVector), X1);
                        pOutputVector += sizeof(XMFLOAT2) * 4;

                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 4;

                    __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                    __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                    __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                    __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                    __m256 vTempB = _mm256_fmadd_ps(Y1, row1, row3);
                    __m256 vTempB2 = _mm256_fmadd_ps(Y2, row1, row3);
                    __m256 vTempA = _mm256_mul_ps(X1, row0);
                    __m256 vTempA2 = _mm256_mul_ps(X2, row0);
                    vTempA = _mm256_add_ps(vTempA, vTempB);
                    vTempA2 = _mm256_add_ps(vTempA2, vTempB2);

                    __m256 W = _mm256_shuffle_ps(vTempA, vTempA, _MM_SHUFFLE(3, 3, 3, 3));
                    vTempA = _mm256_div_ps(vTempA, W);

                    W = _mm256_shuffle_ps(vTempA2, vTempA2, _MM_SHUFFLE(3, 3, 3, 3));
                    vTempA2 = _mm256_div_ps(vTempA2, W);

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_castps256_ps128(vTempA)));
                    pOutputVector += OutputStride;

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_castps256_ps128(vTempA2)));
                    pOutputVector += OutputStride;

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_extractf128_ps(vTempA, 1)));
                    pOutputVector += OutputStride;

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_extractf128_ps(vTempA2, 1)));
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    if (i < VectorCount)
    {
        const XMVECTOR row0 = M.r[0];
        const XMVECTOR row1 = M.r[1];
        const XMVECTOR row3 = M.r[3];

        for (; i < VectorCount; i++)
        {
            __m128 xy = _mm_castpd_ps(_mm_load_sd(reinterpret_cast<const double*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(xy, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(xy, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);

            XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
            vTemp = _mm_div_ps(vTemp, W);

            _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t two = VectorCount >> 1;
    if (two > 0)
    {
        if (InputStride == sizeof(XMFLOAT2))
        {
            if (OutputStride == sizeof(XMFLOAT2))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < two; ++j)
                    {
                        XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 2;

                        // Result 1
                        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                        XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        XMVECTOR V1 = _mm_div_ps(vTemp, W);

                        // Result 2
                        Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                        X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                        vTemp = XM_FMADD_PS(Y, row1, row3);
                        vTemp2 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        XMVECTOR V2 = _mm_div_ps(vTemp, W);

                        vTemp = _mm_movelh_ps(V1, V2);

                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                        pOutputVector += sizeof(XMFLOAT2) * 2;

                        i += 2;
                    }
                }
                else
                {
                    // Packed input, unaligned & packed output
                    for (size_t j = 0; j < two; ++j)
                    {
                        XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 2;

                        // Result 1
                        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                        XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        XMVECTOR V1 = _mm_div_ps(vTemp, W);

                        // Result 2
                        Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                        X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                        vTemp = XM_FMADD_PS(Y, row1, row3);
                        vTemp2 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        XMVECTOR V2 = _mm_div_ps(vTemp, W);

                        vTemp = _mm_movelh_ps(V1, V2);

                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                        pOutputVector += sizeof(XMFLOAT2) * 2;

                        i += 2;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < two; ++j)
                {
                    XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 2;

                    // Result 1
                    XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
                    XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);

                    XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                    vTemp = _mm_div_ps(vTemp, W);

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
                    pOutputVector += OutputStride;

                    // Result 2
                    Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                    X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                    vTemp = XM_FMADD_PS(Y, row1, row3);
                    vTemp2 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                    vTemp = _mm_div_ps(vTemp, W);

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
                    pOutputVector += OutputStride;

                    i += 2;
                }
            }
        }
    }

    if (!(reinterpret_cast<uintptr_t>(pInputVector) & 0xF) && !(InputStride & 0xF))
    {
        // Aligned input
        for (; i < VectorCount; i++)
        {
            XMVECTOR V = _mm_castsi128_ps(_mm_loadl_epi64(reinterpret_cast<const __m128i*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);

            XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

            vTemp = _mm_div_ps(vTemp, W);

            _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
            pOutputVector += OutputStride;
        }
    }
    else
    {
        // Unaligned input
        for (; i < VectorCount; i++)
        {
            __m128 xy = _mm_castpd_ps(_mm_load_sd(reinterpret_cast<const double*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(xy, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(xy, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Y, row1, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);

            XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

            vTemp = _mm_div_ps(vTemp, W);

            _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector2TransformNormal
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Y = XMVectorSplatY(V);
    XMVECTOR X = XMVectorSplatX(V);

    XMVECTOR Result = XMVectorMultiply(Y, M.r[1]);
    Result = XMVectorMultiplyAdd(X, M.r[0], Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    float32x4_t Result = vmulq_lane_f32(M.r[1], VL, 1); // Y
    return vmlaq_lane_f32(Result, M.r[0], VL, 0); // X
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
    vResult = _mm_mul_ps(vResult, M.r[1]);
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
    vResult = XM_FMADD_PS(vTemp, M.r[0], vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMFLOAT2* XM_CALLCONV XMVector2TransformNormalStream
(
    XMFLOAT2* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT2* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT2));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT2));

    assert(OutputStride >= sizeof(XMFLOAT2));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT2));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pInputVector));
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiply(Y, row1);
        Result = XMVectorMultiplyAdd(X, row0, Result);

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015, "PREfast noise: Esp:1307" )
#endif

        XMStoreFloat2(reinterpret_cast<XMFLOAT2*>(pOutputVector), Result);

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT2)) && (OutputStride == sizeof(XMFLOAT2)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x2_t V = vld2q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT2) * 4;

                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmulq_lane_f32(V.val[0], r, 0); // Ax
                XMVECTOR vResult1 = vmulq_lane_f32(V.val[0], r, 1); // Bx

                XM_PREFETCH(pInputVector);
                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));
                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                V.val[0] = vResult0;
                V.val[1] = vResult1;

                vst2q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT2) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        float32x2_t V = vld1_f32(reinterpret_cast<const float*>(pInputVector));
        pInputVector += InputStride;

        XMVECTOR vResult = vmulq_lane_f32(row0, V, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, V, 1); // Y

        V = vget_low_f32(vResult);
        vst1_f32(reinterpret_cast<float*>(pOutputVector), V);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_AVX2_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        __m256 row0 = _mm256_broadcast_ps(&M.r[0]);
        __m256 row1 = _mm256_broadcast_ps(&M.r[1]);

        if (InputStride == sizeof(XMFLOAT2))
        {
            if (OutputStride == sizeof(XMFLOAT2))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0x1F))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 4;

                        __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                        __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                        __m256 vTempA = _mm256_mul_ps(Y1, row1);
                        __m256 vTempB = _mm256_mul_ps(Y2, row1);
                        vTempA = _mm256_fmadd_ps(X1, row0, vTempA);
                        vTempB = _mm256_fmadd_ps(X2, row0, vTempB);

                        X1 = _mm256_shuffle_ps(vTempA, vTempB, 0x44);
                        XM256_STREAM_PS(reinterpret_cast<float*>(pOutputVector), X1);
                        pOutputVector += sizeof(XMFLOAT2) * 4;

                        i += 4;
                    }
                }
                else
                {
                    // Packed input, packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 4;

                        __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                        __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                        __m256 vTempA = _mm256_mul_ps(Y1, row1);
                        __m256 vTempB = _mm256_mul_ps(Y2, row1);
                        vTempA = _mm256_fmadd_ps(X1, row0, vTempA);
                        vTempB = _mm256_fmadd_ps(X2, row0, vTempB);

                        X1 = _mm256_shuffle_ps(vTempA, vTempB, 0x44);
                        _mm256_storeu_ps(reinterpret_cast<float*>(pOutputVector), X1);
                        pOutputVector += sizeof(XMFLOAT2) * 4;

                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 4;

                    __m256 Y2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));
                    __m256 X2 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                    __m256 Y1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                    __m256 X1 = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));

                    __m256 vTempA = _mm256_mul_ps(Y1, row1);
                    __m256 vTempB = _mm256_mul_ps(Y2, row1);
                    vTempA = _mm256_fmadd_ps(X1, row0, vTempA);
                    vTempB = _mm256_fmadd_ps(X2, row0, vTempB);

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_castps256_ps128(vTempA)));
                    pOutputVector += OutputStride;

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_castps256_ps128(vTempB)));
                    pOutputVector += OutputStride;

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_extractf128_ps(vTempA, 1)));
                    pOutputVector += OutputStride;

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector),
                        _mm_castps_pd(_mm256_extractf128_ps(vTempB, 1)));
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    if (i < VectorCount)
    {
        const XMVECTOR row0 = M.r[0];
        const XMVECTOR row1 = M.r[1];

        for (; i < VectorCount; i++)
        {
            __m128 xy = _mm_castpd_ps(_mm_load_sd(reinterpret_cast<const double*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(xy, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(xy, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = _mm_mul_ps(Y, row1);
            vTemp = XM_FMADD_PS(X, row0, vTemp);

            _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];

    size_t i = 0;
    size_t two = VectorCount >> 1;
    if (two > 0)
    {
        if (InputStride == sizeof(XMFLOAT2))
        {
            if (OutputStride == sizeof(XMFLOAT2))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < two; ++j)
                    {
                        XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 2;

                        // Result 1
                        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = _mm_mul_ps(Y, row1);
                        XMVECTOR V1 = XM_FMADD_PS(X, row0, vTemp);

                        // Result 2
                        Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                        X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                        vTemp = _mm_mul_ps(Y, row1);
                        XMVECTOR V2 = XM_FMADD_PS(X, row0, vTemp);

                        vTemp = _mm_movelh_ps(V1, V2);

                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                        pOutputVector += sizeof(XMFLOAT2) * 2;

                        i += 2;
                    }
                }
                else
                {
                    // Packed input, unaligned & packed output
                    for (size_t j = 0; j < two; ++j)
                    {
                        XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT2) * 2;

                        // Result 1
                        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = _mm_mul_ps(Y, row1);
                        XMVECTOR V1 = XM_FMADD_PS(X, row0, vTemp);

                        // Result 2
                        Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                        X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                        vTemp = _mm_mul_ps(Y, row1);
                        XMVECTOR V2 = XM_FMADD_PS(X, row0, vTemp);

                        vTemp = _mm_movelh_ps(V1, V2);

                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                        pOutputVector += sizeof(XMFLOAT2) * 2;

                        i += 2;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < two; ++j)
                {
                    XMVECTOR V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT2) * 2;

                    // Result 1
                    XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = _mm_mul_ps(Y, row1);
                    vTemp = XM_FMADD_PS(X, row0, vTemp);

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
                    pOutputVector += OutputStride;

                    // Result 2
                    Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));
                    X = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));

                    vTemp = _mm_mul_ps(Y, row1);
                    vTemp = XM_FMADD_PS(X, row0, vTemp);

                    _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
                    pOutputVector += OutputStride;

                    i += 2;
                }
            }
        }
    }

    if (!(reinterpret_cast<uintptr_t>(pInputVector) & 0xF) && !(InputStride & 0xF))
    {
        // Aligned input
        for (; i < VectorCount; i++)
        {
            XMVECTOR V = _mm_castsi128_ps(_mm_loadl_epi64(reinterpret_cast<const __m128i*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = _mm_mul_ps(Y, row1);
            vTemp = XM_FMADD_PS(X, row0, vTemp);

            _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
            pOutputVector += OutputStride;
        }
    }
    else
    {
        // Unaligned input
        for (; i < VectorCount; i++)
        {
            __m128 xy = _mm_castpd_ps(_mm_load_sd(reinterpret_cast<const double*>(pInputVector)));
            pInputVector += InputStride;

            XMVECTOR Y = XM_PERMUTE_PS(xy, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(xy, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = _mm_mul_ps(Y, row1);
            vTemp = XM_FMADD_PS(X, row0, vTemp);

            _mm_store_sd(reinterpret_cast<double*>(pOutputVector), _mm_castps_pd(vTemp));
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

/****************************************************************************
 *
 * 3D Vector
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3Equal
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] == V2.vector4_f32[0]) && (V1.vector4_f32[1] == V2.vector4_f32[1]) && (V1.vector4_f32[2] == V2.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 7) == 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector3EqualR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t CR = 0;
    if ((V1.vector4_f32[0] == V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] == V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] == V2.vector4_f32[2]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] != V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] != V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] != V2.vector4_f32[2]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU;

    uint32_t CR = 0;
    if (r == 0xFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    int iTest = _mm_movemask_ps(vTemp) & 7;
    uint32_t CR = 0;
    if (iTest == 7)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3EqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_u32[0] == V2.vector4_u32[0]) && (V1.vector4_u32[1] == V2.vector4_u32[1]) && (V1.vector4_u32[2] == V2.vector4_u32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 7) == 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector3EqualIntR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t CR = 0;
    if ((V1.vector4_u32[0] == V2.vector4_u32[0]) &&
        (V1.vector4_u32[1] == V2.vector4_u32[1]) &&
        (V1.vector4_u32[2] == V2.vector4_u32[2]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_u32[0] != V2.vector4_u32[0]) &&
        (V1.vector4_u32[1] != V2.vector4_u32[1]) &&
        (V1.vector4_u32[2] != V2.vector4_u32[2]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU;

    uint32_t CR = 0;
    if (r == 0xFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    int iTemp = _mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 7;
    uint32_t CR = 0;
    if (iTemp == 7)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTemp)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3NearEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR Epsilon
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float dx, dy, dz;

    dx = fabsf(V1.vector4_f32[0] - V2.vector4_f32[0]);
    dy = fabsf(V1.vector4_f32[1] - V2.vector4_f32[1]);
    dz = fabsf(V1.vector4_f32[2] - V2.vector4_f32[2]);
    return (((dx <= Epsilon.vector4_f32[0]) &&
        (dy <= Epsilon.vector4_f32[1]) &&
        (dz <= Epsilon.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vDelta = vsubq_f32(V1, V2);
#if defined(_MSC_VER) && !defined(__clang__) && !defined(_ARM64_DISTINCT_NEON_TYPES)
    uint32x4_t vResult = vacleq_f32(vDelta, Epsilon);
#else
    uint32x4_t vResult = vcleq_f32(vabsq_f32(vDelta), Epsilon);
#endif
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    // Get the difference
    XMVECTOR vDelta = _mm_sub_ps(V1, V2);
    // Get the absolute value of the difference
    XMVECTOR vTemp = _mm_setzero_ps();
    vTemp = _mm_sub_ps(vTemp, vDelta);
    vTemp = _mm_max_ps(vTemp, vDelta);
    vTemp = _mm_cmple_ps(vTemp, Epsilon);
    // w is don't care
    return (((_mm_movemask_ps(vTemp) & 7) == 0x7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3NotEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] != V2.vector4_f32[0]) || (V1.vector4_f32[1] != V2.vector4_f32[1]) || (V1.vector4_f32[2] != V2.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) != 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 7) != 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3NotEqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_u32[0] != V2.vector4_u32[0]) || (V1.vector4_u32[1] != V2.vector4_u32[1]) || (V1.vector4_u32[2] != V2.vector4_u32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) != 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return (((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) & 7) != 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3Greater
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] > V2.vector4_f32[0]) && (V1.vector4_f32[1] > V2.vector4_f32[1]) && (V1.vector4_f32[2] > V2.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgtq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 7) == 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector3GreaterR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t CR = 0;
    if ((V1.vector4_f32[0] > V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] > V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] > V2.vector4_f32[2]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] <= V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] <= V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] <= V2.vector4_f32[2]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgtq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU;

    uint32_t CR = 0;
    if (r == 0xFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    uint32_t CR = 0;
    int iTest = _mm_movemask_ps(vTemp) & 7;
    if (iTest == 7)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3GreaterOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] >= V2.vector4_f32[0]) && (V1.vector4_f32[1] >= V2.vector4_f32[1]) && (V1.vector4_f32[2] >= V2.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgeq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 7) == 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector3GreaterOrEqualR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    uint32_t CR = 0;
    if ((V1.vector4_f32[0] >= V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] >= V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] >= V2.vector4_f32[2]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] < V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] < V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] < V2.vector4_f32[2]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgeq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU;

    uint32_t CR = 0;
    if (r == 0xFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    uint32_t CR = 0;
    int iTest = _mm_movemask_ps(vTemp) & 7;
    if (iTest == 7)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3Less
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] < V2.vector4_f32[0]) && (V1.vector4_f32[1] < V2.vector4_f32[1]) && (V1.vector4_f32[2] < V2.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcltq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmplt_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 7) == 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3LessOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] <= V2.vector4_f32[0]) && (V1.vector4_f32[1] <= V2.vector4_f32[1]) && (V1.vector4_f32[2] <= V2.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcleq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmple_ps(V1, V2);
    return (((_mm_movemask_ps(vTemp) & 7) == 7) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3InBounds
(
    FXMVECTOR V,
    FXMVECTOR Bounds
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) &&
        (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1]) &&
        (V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Test if less than or equal
    uint32x4_t ivTemp1 = vcleq_f32(V, Bounds);
    // Negate the bounds
    float32x4_t vTemp2 = vnegq_f32(Bounds);
    // Test if greater or equal (Reversed)
    uint32x4_t ivTemp2 = vcleq_f32(vTemp2, V);
    // Blend answers
    ivTemp1 = vandq_u32(ivTemp1, ivTemp2);
    // in bounds?
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(ivTemp1)), vget_high_u8(vreinterpretq_u8_u32(ivTemp1)));
    uint16x4x2_t vTemp3 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp3.val[1]), 1) & 0xFFFFFFU) == 0xFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    // Test if less than or equal
    XMVECTOR vTemp1 = _mm_cmple_ps(V, Bounds);
    // Negate the bounds
    XMVECTOR vTemp2 = _mm_mul_ps(Bounds, g_XMNegativeOne);
    // Test if greater or equal (Reversed)
    vTemp2 = _mm_cmple_ps(vTemp2, V);
    // Blend answers
    vTemp1 = _mm_and_ps(vTemp1, vTemp2);
    // x,y and z in bounds? (w is don't care)
    return (((_mm_movemask_ps(vTemp1) & 0x7) == 0x7) != 0);
#else
    return XMComparisonAllInBounds(XMVector3InBoundsR(V, Bounds));
#endif
}

//------------------------------------------------------------------------------

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(push)
#pragma float_control(precise, on)
#endif

inline bool XM_CALLCONV XMVector3IsNaN(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    return (XMISNAN(V.vector4_f32[0]) ||
        XMISNAN(V.vector4_f32[1]) ||
        XMISNAN(V.vector4_f32[2]));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    return isnan(vgetq_lane_f32(V, 0)) || isnan(vgetq_lane_f32(V, 1)) || isnan(vgetq_lane_f32(V, 2));
    #else
    // Test against itself. NaN is always not equal
    uint32x4_t vTempNan = vceqq_f32(V, V);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vTempNan)), vget_high_u8(vreinterpretq_u8_u32(vTempNan)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    // If x or y or z are NaN, the mask is zero
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) != 0xFFFFFFU);
    #endif
#elif defined(_XM_SSE_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    XM_ALIGNED_DATA(16) float tmp[4];
    _mm_store_ps(tmp, V);
    return isnan(tmp[0]) || isnan(tmp[1]) || isnan(tmp[2]);
    #else
    // Test against itself. NaN is always not equal
    XMVECTOR vTempNan = _mm_cmpneq_ps(V, V);
    // If x or y or z are NaN, the mask is non-zero
    return ((_mm_movemask_ps(vTempNan) & 7) != 0);
    #endif
#endif
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(pop)
#endif

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector3IsInfinite(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (XMISINF(V.vector4_f32[0]) ||
        XMISINF(V.vector4_f32[1]) ||
        XMISINF(V.vector4_f32[2]));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Mask off the sign bit
    uint32x4_t vTempInf = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    // Compare to infinity
    vTempInf = vceqq_f32(vreinterpretq_f32_u32(vTempInf), g_XMInfinity);
    // If any are infinity, the signs are true.
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vTempInf)), vget_high_u8(vreinterpretq_u8_u32(vTempInf)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return ((vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) & 0xFFFFFFU) != 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Mask off the sign bit
    __m128 vTemp = _mm_and_ps(V, g_XMAbsMask);
    // Compare to infinity
    vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity);
    // If x,y or z are infinity, the signs are true.
    return ((_mm_movemask_ps(vTemp) & 7) != 0);
#endif
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Dot
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float fValue = V1.vector4_f32[0] * V2.vector4_f32[0] + V1.vector4_f32[1] * V2.vector4_f32[1] + V1.vector4_f32[2] * V2.vector4_f32[2];
    XMVECTORF32 vResult;
    vResult.f[0] =
        vResult.f[1] =
        vResult.f[2] =
        vResult.f[3] = fValue;
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vTemp = vmulq_f32(V1, V2);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    return vcombine_f32(v1, v1);
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_dp_ps(V1, V2, 0x7f);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vTemp = _mm_mul_ps(V1, V2);
    vTemp = _mm_and_ps(vTemp, g_XMMask3);
    vTemp = _mm_hadd_ps(vTemp, vTemp);
    return _mm_hadd_ps(vTemp, vTemp);
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product
    XMVECTOR vDot = _mm_mul_ps(V1, V2);
    // x=Dot.vector4_f32[1], y=Dot.vector4_f32[2]
    XMVECTOR vTemp = XM_PERMUTE_PS(vDot, _MM_SHUFFLE(2, 1, 2, 1));
    // Result.vector4_f32[0] = x+y
    vDot = _mm_add_ss(vDot, vTemp);
    // x=Dot.vector4_f32[2]
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    // Result.vector4_f32[0] = (x+y)+z
    vDot = _mm_add_ss(vDot, vTemp);
    // Splat x
    return XM_PERMUTE_PS(vDot, _MM_SHUFFLE(0, 0, 0, 0));
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Cross
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    // [ V1.y*V2.z - V1.z*V2.y, V1.z*V2.x - V1.x*V2.z, V1.x*V2.y - V1.y*V2.x ]

#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            (V1.vector4_f32[1] * V2.vector4_f32[2]) - (V1.vector4_f32[2] * V2.vector4_f32[1]),
            (V1.vector4_f32[2] * V2.vector4_f32[0]) - (V1.vector4_f32[0] * V2.vector4_f32[2]),
            (V1.vector4_f32[0] * V2.vector4_f32[1]) - (V1.vector4_f32[1] * V2.vector4_f32[0]),
            0.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t v1xy = vget_low_f32(V1);
    float32x2_t v2xy = vget_low_f32(V2);

    float32x2_t v1yx = vrev64_f32(v1xy);
    float32x2_t v2yx = vrev64_f32(v2xy);

    float32x2_t v1zz = vdup_lane_f32(vget_high_f32(V1), 0);
    float32x2_t v2zz = vdup_lane_f32(vget_high_f32(V2), 0);

    XMVECTOR vResult = vmulq_f32(vcombine_f32(v1yx, v1xy), vcombine_f32(v2zz, v2yx));
    vResult = vmlsq_f32(vResult, vcombine_f32(v1zz, v1yx), vcombine_f32(v2yx, v2xy));
    vResult = vreinterpretq_f32_u32(veorq_u32(vreinterpretq_u32_f32(vResult), g_XMFlipY));
    return vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(vResult), g_XMMask3));
#elif defined(_XM_SSE_INTRINSICS_)
    // y1,z1,x1,w1
    XMVECTOR vTemp1 = XM_PERMUTE_PS(V1, _MM_SHUFFLE(3, 0, 2, 1));
    // z2,x2,y2,w2
    XMVECTOR vTemp2 = XM_PERMUTE_PS(V2, _MM_SHUFFLE(3, 1, 0, 2));
    // Perform the left operation
    XMVECTOR vResult = _mm_mul_ps(vTemp1, vTemp2);
    // z1,x1,y1,w1
    vTemp1 = XM_PERMUTE_PS(vTemp1, _MM_SHUFFLE(3, 0, 2, 1));
    // y2,z2,x2,w2
    vTemp2 = XM_PERMUTE_PS(vTemp2, _MM_SHUFFLE(3, 1, 0, 2));
    // Perform the right operation
    vResult = XM_FNMADD_PS(vTemp1, vTemp2, vResult);
    // Set w to zero
    return _mm_and_ps(vResult, g_XMMask3);
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3LengthSq(FXMVECTOR V) noexcept
{
    return XMVector3Dot(V, V);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3ReciprocalLengthEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector3LengthSq(V);
    Result = XMVectorReciprocalSqrtEst(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot3
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    // Reciprocal sqrt (estimate)
    v2 = vrsqrte_f32(v1);
    return vcombine_f32(v2, v2);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x7f);
    return _mm_rsqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_rsqrt_ps(vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y and z
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and y
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 2, 1, 2));
    // x+z, y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    // y,y,y,y
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    // x+z+y,??,??,??
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    // Splat the length squared
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    // Get the reciprocal
    vLengthSq = _mm_rsqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3ReciprocalLength(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector3LengthSq(V);
    Result = XMVectorReciprocalSqrt(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot3
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    // Reciprocal sqrt
    float32x2_t  S0 = vrsqrte_f32(v1);
    float32x2_t  P0 = vmul_f32(v1, S0);
    float32x2_t  R0 = vrsqrts_f32(P0, S0);
    float32x2_t  S1 = vmul_f32(S0, R0);
    float32x2_t  P1 = vmul_f32(v1, S1);
    float32x2_t  R1 = vrsqrts_f32(P1, S1);
    float32x2_t Result = vmul_f32(S1, R1);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x7f);
    XMVECTOR vLengthSq = _mm_sqrt_ps(vTemp);
    return _mm_div_ps(g_XMOne, vLengthSq);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vDot = _mm_mul_ps(V, V);
    vDot = _mm_and_ps(vDot, g_XMMask3);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_sqrt_ps(vDot);
    vDot = _mm_div_ps(g_XMOne, vDot);
    return vDot;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product
    XMVECTOR vDot = _mm_mul_ps(V, V);
    // x=Dot.y, y=Dot.z
    XMVECTOR vTemp = XM_PERMUTE_PS(vDot, _MM_SHUFFLE(2, 1, 2, 1));
    // Result.x = x+y
    vDot = _mm_add_ss(vDot, vTemp);
    // x=Dot.z
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    // Result.x = (x+y)+z
    vDot = _mm_add_ss(vDot, vTemp);
    // Splat x
    vDot = XM_PERMUTE_PS(vDot, _MM_SHUFFLE(0, 0, 0, 0));
    // Get the reciprocal
    vDot = _mm_sqrt_ps(vDot);
    // Get the reciprocal
    vDot = _mm_div_ps(g_XMOne, vDot);
    return vDot;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3LengthEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector3LengthSq(V);
    Result = XMVectorSqrtEst(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot3
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    const float32x2_t zero = vdup_n_f32(0);
    uint32x2_t VEqualsZero = vceq_f32(v1, zero);
    // Sqrt (estimate)
    float32x2_t Result = vrsqrte_f32(v1);
    Result = vmul_f32(v1, Result);
    Result = vbsl_f32(VEqualsZero, zero, Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x7f);
    return _mm_sqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y and z
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and y
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 2, 1, 2));
    // x+z, y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    // y,y,y,y
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    // x+z+y,??,??,??
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    // Splat the length squared
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    // Get the length
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Length(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector3LengthSq(V);
    Result = XMVectorSqrt(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot3
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    const float32x2_t zero = vdup_n_f32(0);
    uint32x2_t VEqualsZero = vceq_f32(v1, zero);
    // Sqrt
    float32x2_t S0 = vrsqrte_f32(v1);
    float32x2_t P0 = vmul_f32(v1, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(v1, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    float32x2_t Result = vmul_f32(S1, R1);
    Result = vmul_f32(v1, Result);
    Result = vbsl_f32(VEqualsZero, zero, Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x7f);
    return _mm_sqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y and z
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and y
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 2, 1, 2));
    // x+z, y
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    // y,y,y,y
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    // x+z+y,??,??,??
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    // Splat the length squared
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    // Get the length
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------
// XMVector3NormalizeEst uses a reciprocal estimate and
// returns QNaN on zero and infinite vectors.

inline XMVECTOR XM_CALLCONV XMVector3NormalizeEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector3ReciprocalLength(V);
    Result = XMVectorMultiply(V, Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot3
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    // Reciprocal sqrt (estimate)
    v2 = vrsqrte_f32(v1);
    // Normalize
    return vmulq_f32(V, vcombine_f32(v2, v2));
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0x7f);
    XMVECTOR vResult = _mm_rsqrt_ps(vTemp);
    return _mm_mul_ps(vResult, V);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vDot = _mm_mul_ps(V, V);
    vDot = _mm_and_ps(vDot, g_XMMask3);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_rsqrt_ps(vDot);
    vDot = _mm_mul_ps(vDot, V);
    return vDot;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product
    XMVECTOR vDot = _mm_mul_ps(V, V);
    // x=Dot.y, y=Dot.z
    XMVECTOR vTemp = XM_PERMUTE_PS(vDot, _MM_SHUFFLE(2, 1, 2, 1));
    // Result.x = x+y
    vDot = _mm_add_ss(vDot, vTemp);
    // x=Dot.z
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    // Result.x = (x+y)+z
    vDot = _mm_add_ss(vDot, vTemp);
    // Splat x
    vDot = XM_PERMUTE_PS(vDot, _MM_SHUFFLE(0, 0, 0, 0));
    // Get the reciprocal
    vDot = _mm_rsqrt_ps(vDot);
    // Perform the normalization
    vDot = _mm_mul_ps(vDot, V);
    return vDot;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Normalize(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float fLength;
    XMVECTOR vResult;

    vResult = XMVector3Length(V);
    fLength = vResult.vector4_f32[0];

    // Prevent divide by zero
    if (fLength > 0)
    {
        fLength = 1.0f / fLength;
    }

    vResult.vector4_f32[0] = V.vector4_f32[0] * fLength;
    vResult.vector4_f32[1] = V.vector4_f32[1] * fLength;
    vResult.vector4_f32[2] = V.vector4_f32[2] * fLength;
    vResult.vector4_f32[3] = V.vector4_f32[3] * fLength;
    return vResult;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot3
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vpadd_f32(v1, v1);
    v2 = vdup_lane_f32(v2, 0);
    v1 = vadd_f32(v1, v2);
    uint32x2_t VEqualsZero = vceq_f32(v1, vdup_n_f32(0));
    uint32x2_t VEqualsInf = vceq_f32(v1, vget_low_f32(g_XMInfinity));
    // Reciprocal sqrt (2 iterations of Newton-Raphson)
    float32x2_t S0 = vrsqrte_f32(v1);
    float32x2_t P0 = vmul_f32(v1, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(v1, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    v2 = vmul_f32(S1, R1);
    // Normalize
    XMVECTOR vResult = vmulq_f32(V, vcombine_f32(v2, v2));
    vResult = vbslq_f32(vcombine_u32(VEqualsZero, VEqualsZero), vdupq_n_f32(0), vResult);
    return vbslq_f32(vcombine_u32(VEqualsInf, VEqualsInf), g_XMQNaN, vResult);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_dp_ps(V, V, 0x7f);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Divide to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#elif defined(_XM_SSE3_INTRINSICS_)
    // Perform the dot product on x,y and z only
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_and_ps(vLengthSq, g_XMMask3);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Divide to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y and z only
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 1, 2, 1));
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(1, 1, 1, 1));
    vLengthSq = _mm_add_ss(vLengthSq, vTemp);
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(0, 0, 0, 0));
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Divide to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3ClampLength
(
    FXMVECTOR V,
    float    LengthMin,
    float    LengthMax
) noexcept
{
    XMVECTOR ClampMax = XMVectorReplicate(LengthMax);
    XMVECTOR ClampMin = XMVectorReplicate(LengthMin);

    return XMVector3ClampLengthV(V, ClampMin, ClampMax);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3ClampLengthV
(
    FXMVECTOR V,
    FXMVECTOR LengthMin,
    FXMVECTOR LengthMax
) noexcept
{
    assert((XMVectorGetY(LengthMin) == XMVectorGetX(LengthMin)) && (XMVectorGetZ(LengthMin) == XMVectorGetX(LengthMin)));
    assert((XMVectorGetY(LengthMax) == XMVectorGetX(LengthMax)) && (XMVectorGetZ(LengthMax) == XMVectorGetX(LengthMax)));
    assert(XMVector3GreaterOrEqual(LengthMin, XMVectorZero()));
    assert(XMVector3GreaterOrEqual(LengthMax, XMVectorZero()));
    assert(XMVector3GreaterOrEqual(LengthMax, LengthMin));

    XMVECTOR LengthSq = XMVector3LengthSq(V);

    const XMVECTOR Zero = XMVectorZero();

    XMVECTOR RcpLength = XMVectorReciprocalSqrt(LengthSq);

    XMVECTOR InfiniteLength = XMVectorEqualInt(LengthSq, g_XMInfinity.v);
    XMVECTOR ZeroLength = XMVectorEqual(LengthSq, Zero);

    XMVECTOR Normal = XMVectorMultiply(V, RcpLength);

    XMVECTOR Length = XMVectorMultiply(LengthSq, RcpLength);

    XMVECTOR Select = XMVectorEqualInt(InfiniteLength, ZeroLength);
    Length = XMVectorSelect(LengthSq, Length, Select);
    Normal = XMVectorSelect(LengthSq, Normal, Select);

    XMVECTOR ControlMax = XMVectorGreater(Length, LengthMax);
    XMVECTOR ControlMin = XMVectorLess(Length, LengthMin);

    XMVECTOR ClampLength = XMVectorSelect(Length, LengthMax, ControlMax);
    ClampLength = XMVectorSelect(ClampLength, LengthMin, ControlMin);

    XMVECTOR Result = XMVectorMultiply(Normal, ClampLength);

    // Preserve the original vector (with no precision loss) if the length falls within the given range
    XMVECTOR Control = XMVectorEqualInt(ControlMax, ControlMin);
    Result = XMVectorSelect(Result, V, Control);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Reflect
(
    FXMVECTOR Incident,
    FXMVECTOR Normal
) noexcept
{
    // Result = Incident - (2 * dot(Incident, Normal)) * Normal

    XMVECTOR Result = XMVector3Dot(Incident, Normal);
    Result = XMVectorAdd(Result, Result);
    Result = XMVectorNegativeMultiplySubtract(Result, Normal, Incident);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Refract
(
    FXMVECTOR Incident,
    FXMVECTOR Normal,
    float    RefractionIndex
) noexcept
{
    XMVECTOR Index = XMVectorReplicate(RefractionIndex);
    return XMVector3RefractV(Incident, Normal, Index);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3RefractV
(
    FXMVECTOR Incident,
    FXMVECTOR Normal,
    FXMVECTOR RefractionIndex
) noexcept
{
    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))

#if defined(_XM_NO_INTRINSICS_)

    const XMVECTOR  Zero = XMVectorZero();

    XMVECTOR IDotN = XMVector3Dot(Incident, Normal);

    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    XMVECTOR R = XMVectorNegativeMultiplySubtract(IDotN, IDotN, g_XMOne.v);
    R = XMVectorMultiply(R, RefractionIndex);
    R = XMVectorNegativeMultiplySubtract(R, RefractionIndex, g_XMOne.v);

    if (XMVector4LessOrEqual(R, Zero))
    {
        // Total internal reflection
        return Zero;
    }
    else
    {
        // R = RefractionIndex * IDotN + sqrt(R)
        R = XMVectorSqrt(R);
        R = XMVectorMultiplyAdd(RefractionIndex, IDotN, R);

        // Result = RefractionIndex * Incident - Normal * R
        XMVECTOR Result = XMVectorMultiply(RefractionIndex, Incident);
        Result = XMVectorNegativeMultiplySubtract(Normal, R, Result);

        return Result;
    }

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR IDotN = XMVector3Dot(Incident, Normal);

    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    float32x4_t R = vmlsq_f32(g_XMOne, IDotN, IDotN);
    R = vmulq_f32(R, RefractionIndex);
    R = vmlsq_f32(g_XMOne, R, RefractionIndex);

    uint32x4_t isrzero = vcleq_f32(R, g_XMZero);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(isrzero)), vget_high_u8(vreinterpretq_u8_u32(isrzero)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));

    float32x4_t vResult;
    if (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU)
    {
        // Total internal reflection
        vResult = g_XMZero;
    }
    else
    {
        // Sqrt(R)
        float32x4_t S0 = vrsqrteq_f32(R);
        float32x4_t P0 = vmulq_f32(R, S0);
        float32x4_t R0 = vrsqrtsq_f32(P0, S0);
        float32x4_t S1 = vmulq_f32(S0, R0);
        float32x4_t P1 = vmulq_f32(R, S1);
        float32x4_t R1 = vrsqrtsq_f32(P1, S1);
        float32x4_t S2 = vmulq_f32(S1, R1);
        R = vmulq_f32(R, S2);
        // R = RefractionIndex * IDotN + sqrt(R)
        R = vmlaq_f32(R, RefractionIndex, IDotN);
        // Result = RefractionIndex * Incident - Normal * R
        vResult = vmulq_f32(RefractionIndex, Incident);
        vResult = vmlsq_f32(vResult, R, Normal);
    }
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))
    XMVECTOR IDotN = XMVector3Dot(Incident, Normal);
    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    XMVECTOR R = XM_FNMADD_PS(IDotN, IDotN, g_XMOne);
    XMVECTOR R2 = _mm_mul_ps(RefractionIndex, RefractionIndex);
    R = XM_FNMADD_PS(R, R2, g_XMOne);

    XMVECTOR vResult = _mm_cmple_ps(R, g_XMZero);
    if (_mm_movemask_ps(vResult) == 0x0f)
    {
        // Total internal reflection
        vResult = g_XMZero;
    }
    else
    {
        // R = RefractionIndex * IDotN + sqrt(R)
        R = _mm_sqrt_ps(R);
        R = XM_FMADD_PS(RefractionIndex, IDotN, R);
        // Result = RefractionIndex * Incident - Normal * R
        vResult = _mm_mul_ps(RefractionIndex, Incident);
        vResult = XM_FNMADD_PS(R, Normal, vResult);
    }
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Orthogonal(FXMVECTOR V) noexcept
{
    XMVECTOR Zero = XMVectorZero();
    XMVECTOR Z = XMVectorSplatZ(V);
    XMVECTOR YZYY = XMVectorSwizzle<XM_SWIZZLE_Y, XM_SWIZZLE_Z, XM_SWIZZLE_Y, XM_SWIZZLE_Y>(V);

    XMVECTOR NegativeV = XMVectorSubtract(Zero, V);

    XMVECTOR ZIsNegative = XMVectorLess(Z, Zero);
    XMVECTOR YZYYIsNegative = XMVectorLess(YZYY, Zero);

    XMVECTOR S = XMVectorAdd(YZYY, Z);
    XMVECTOR D = XMVectorSubtract(YZYY, Z);

    XMVECTOR Select = XMVectorEqualInt(ZIsNegative, YZYYIsNegative);

    XMVECTOR R0 = XMVectorPermute<XM_PERMUTE_1X, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_0X>(NegativeV, S);
    XMVECTOR R1 = XMVectorPermute<XM_PERMUTE_1X, XM_PERMUTE_0X, XM_PERMUTE_0X, XM_PERMUTE_0X>(V, D);

    return XMVectorSelect(R1, R0, Select);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3AngleBetweenNormalsEst
(
    FXMVECTOR N1,
    FXMVECTOR N2
) noexcept
{
    XMVECTOR Result = XMVector3Dot(N1, N2);
    Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
    Result = XMVectorACosEst(Result);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3AngleBetweenNormals
(
    FXMVECTOR N1,
    FXMVECTOR N2
) noexcept
{
    XMVECTOR Result = XMVector3Dot(N1, N2);
    Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
    Result = XMVectorACos(Result);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3AngleBetweenVectors
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    XMVECTOR L1 = XMVector3ReciprocalLength(V1);
    XMVECTOR L2 = XMVector3ReciprocalLength(V2);

    XMVECTOR Dot = XMVector3Dot(V1, V2);

    L1 = XMVectorMultiply(L1, L2);

    XMVECTOR CosAngle = XMVectorMultiply(Dot, L1);
    CosAngle = XMVectorClamp(CosAngle, g_XMNegativeOne.v, g_XMOne.v);

    return XMVectorACos(CosAngle);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3LinePointDistance
(
    FXMVECTOR LinePoint1,
    FXMVECTOR LinePoint2,
    FXMVECTOR Point
) noexcept
{
    // Given a vector PointVector from LinePoint1 to Point and a vector
    // LineVector from LinePoint1 to LinePoint2, the scaled distance
    // PointProjectionScale from LinePoint1 to the perpendicular projection
    // of PointVector onto the line is defined as:
    //
    //     PointProjectionScale = dot(PointVector, LineVector) / LengthSq(LineVector)

    XMVECTOR PointVector = XMVectorSubtract(Point, LinePoint1);
    XMVECTOR LineVector = XMVectorSubtract(LinePoint2, LinePoint1);

    XMVECTOR LengthSq = XMVector3LengthSq(LineVector);

    XMVECTOR PointProjectionScale = XMVector3Dot(PointVector, LineVector);
    PointProjectionScale = XMVectorDivide(PointProjectionScale, LengthSq);

    XMVECTOR DistanceVector = XMVectorMultiply(LineVector, PointProjectionScale);
    DistanceVector = XMVectorSubtract(PointVector, DistanceVector);

    return XMVector3Length(DistanceVector);
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline void XM_CALLCONV XMVector3ComponentsFromNormal
(
    XMVECTOR* pParallel,
    XMVECTOR* pPerpendicular,
    FXMVECTOR  V,
    FXMVECTOR  Normal
) noexcept
{
    assert(pParallel != nullptr);
    assert(pPerpendicular != nullptr);

    XMVECTOR Scale = XMVector3Dot(V, Normal);

    XMVECTOR Parallel = XMVectorMultiply(Normal, Scale);

    *pParallel = Parallel;
    *pPerpendicular = XMVectorSubtract(V, Parallel);
}

//------------------------------------------------------------------------------
// Transform a vector using a rotation expressed as a unit quaternion

inline XMVECTOR XM_CALLCONV XMVector3Rotate
(
    FXMVECTOR V,
    FXMVECTOR RotationQuaternion
) noexcept
{
    XMVECTOR A = XMVectorSelect(g_XMSelect1110.v, V, g_XMSelect1110.v);
    XMVECTOR Q = XMQuaternionConjugate(RotationQuaternion);
    XMVECTOR Result = XMQuaternionMultiply(Q, A);
    return XMQuaternionMultiply(Result, RotationQuaternion);
}

//------------------------------------------------------------------------------
// Transform a vector using the inverse of a rotation expressed as a unit quaternion

inline XMVECTOR XM_CALLCONV XMVector3InverseRotate
(
    FXMVECTOR V,
    FXMVECTOR RotationQuaternion
) noexcept
{
    XMVECTOR A = XMVectorSelect(g_XMSelect1110.v, V, g_XMSelect1110.v);
    XMVECTOR Result = XMQuaternionMultiply(RotationQuaternion, A);
    XMVECTOR Q = XMQuaternionConjugate(RotationQuaternion);
    return XMQuaternionMultiply(Result, Q);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Transform
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Z = XMVectorSplatZ(V);
    XMVECTOR Y = XMVectorSplatY(V);
    XMVECTOR X = XMVectorSplatX(V);

    XMVECTOR Result = XMVectorMultiplyAdd(Z, M.r[2], M.r[3]);
    Result = XMVectorMultiplyAdd(Y, M.r[1], Result);
    Result = XMVectorMultiplyAdd(X, M.r[0], Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    XMVECTOR vResult = vmlaq_lane_f32(M.r[3], M.r[0], VL, 0); // X
    vResult = vmlaq_lane_f32(vResult, M.r[1], VL, 1); // Y
    return vmlaq_lane_f32(vResult, M.r[2], vget_high_f32(V), 0); // Z
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2)); // Z
    vResult = XM_FMADD_PS(vResult, M.r[2], M.r[3]);
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
    vResult = XM_FMADD_PS(vTemp, M.r[1], vResult);
    vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
    vResult = XM_FMADD_PS(vTemp, M.r[0], vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015 26019, "PREfast noise: Esp:1307" )
#endif

_Use_decl_annotations_
inline XMFLOAT4* XM_CALLCONV XMVector3TransformStream
(
    XMFLOAT4* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT3* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT3));

    assert(OutputStride >= sizeof(XMFLOAT4));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT4));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        XMVECTOR Z = XMVectorSplatZ(V);
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiplyAdd(Z, row2, row3);
        Result = XMVectorMultiplyAdd(Y, row1, Result);
        Result = XMVectorMultiplyAdd(X, row0, Result);

        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(pOutputVector), Result);

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT3)) && (OutputStride == sizeof(XMFLOAT4)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x3_t V = vld3q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT3) * 4;

                float32x2_t r3 = vget_low_f32(row3);
                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Ax+M
                XMVECTOR vResult1 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Bx+N

                XM_PREFETCH(pInputVector);

                r3 = vget_high_f32(row3);
                r = vget_high_f32(row0);
                XMVECTOR vResult2 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Cx+O
                XMVECTOR vResult3 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Dx+P

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(row1);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[1], r, 0); // Cx+Gy+O
                vResult3 = vmlaq_lane_f32(vResult3, V.val[1], r, 1); // Dx+Hy+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                r = vget_low_f32(row2);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[2], r, 0); // Ax+Ey+Iz+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[2], r, 1); // Bx+Fy+Jz+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 4));

                r = vget_high_f32(row2);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[2], r, 0); // Cx+Gy+Kz+O
                vResult3 = vmlaq_lane_f32(vResult3, V.val[2], r, 1); // Dx+Hy+Lz+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 5));

                float32x4x4_t R;
                R.val[0] = vResult0;
                R.val[1] = vResult1;
                R.val[2] = vResult2;
                R.val[3] = vResult3;

                vst4q_f32(reinterpret_cast<float*>(pOutputVector), R);
                pOutputVector += sizeof(XMFLOAT4) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        float32x2_t VL = vld1_f32(reinterpret_cast<const float*>(pInputVector));
        float32x2_t zero = vdup_n_f32(0);
        float32x2_t VH = vld1_lane_f32(reinterpret_cast<const float*>(pInputVector) + 2, zero, 0);
        pInputVector += InputStride;

        XMVECTOR vResult = vmlaq_lane_f32(row3, row0, VL, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, VL, 1); // Y
        vResult = vmlaq_lane_f32(vResult, row2, VH, 0); // Z

        vst1q_f32(reinterpret_cast<float*>(pOutputVector), vResult);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(XMFLOAT3))
        {
            if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF) && !(OutputStride & 0xF))
            {
                // Packed input, aligned output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                    __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                    pInputVector += sizeof(XMFLOAT3) * 4;

                    // Unpack the 4 vectors (.w components are junk)
                    XM3UNPACK3INTO4(V1, L2, L3);

                    // Result 1
                    XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                    XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
                    XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                    XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 2
                    Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 3
                    Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 4
                    Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
            else
            {
                // Packed input, unaligned output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                    __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                    pInputVector += sizeof(XMFLOAT3) * 4;

                    // Unpack the 4 vectors (.w components are junk)
                    XM3UNPACK3INTO4(V1, L2, L3);

                    // Result 1
                    XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                    XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
                    XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                    XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 2
                    Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 3
                    Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 4
                    Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);
                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF) && !(OutputStride & 0xF))
    {
        // Aligned output
        for (; i < VectorCount; ++i)
        {
            XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
            pInputVector += InputStride;

            XMVECTOR Z = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
            XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
            XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);
            vTemp = _mm_add_ps(vTemp, vTemp3);

            XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTemp);
            pOutputVector += OutputStride;
        }
    }
    else
    {
        // Unaligned output
        for (; i < VectorCount; ++i)
        {
            XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
            pInputVector += InputStride;

            XMVECTOR Z = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
            XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

            XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
            XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
            XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
            vTemp = _mm_add_ps(vTemp, vTemp2);
            vTemp = _mm_add_ps(vTemp, vTemp3);

            _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTemp);
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3TransformCoord
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
    XMVECTOR Z = XMVectorSplatZ(V);
    XMVECTOR Y = XMVectorSplatY(V);
    XMVECTOR X = XMVectorSplatX(V);

    XMVECTOR Result = XMVectorMultiplyAdd(Z, M.r[2], M.r[3]);
    Result = XMVectorMultiplyAdd(Y, M.r[1], Result);
    Result = XMVectorMultiplyAdd(X, M.r[0], Result);

    XMVECTOR W = XMVectorSplatW(Result);
    return XMVectorDivide(Result, W);
}

//------------------------------------------------------------------------------

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015 26019, "PREfast noise: Esp:1307" )
#endif

_Use_decl_annotations_
inline XMFLOAT3* XM_CALLCONV XMVector3TransformCoordStream
(
    XMFLOAT3* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT3* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT3));

    assert(OutputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT3));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        XMVECTOR Z = XMVectorSplatZ(V);
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiplyAdd(Z, row2, row3);
        Result = XMVectorMultiplyAdd(Y, row1, Result);
        Result = XMVectorMultiplyAdd(X, row0, Result);

        XMVECTOR W = XMVectorSplatW(Result);

        Result = XMVectorDivide(Result, W);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), Result);

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT3)) && (OutputStride == sizeof(XMFLOAT3)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x3_t V = vld3q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT3) * 4;

                float32x2_t r3 = vget_low_f32(row3);
                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Ax+M
                XMVECTOR vResult1 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Bx+N

                XM_PREFETCH(pInputVector);

                r3 = vget_high_f32(row3);
                r = vget_high_f32(row0);
                XMVECTOR vResult2 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Cx+O
                XMVECTOR W = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Dx+P

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(row1);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[1], r, 0); // Cx+Gy+O
                W = vmlaq_lane_f32(W, V.val[1], r, 1); // Dx+Hy+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                r = vget_low_f32(row2);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[2], r, 0); // Ax+Ey+Iz+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[2], r, 1); // Bx+Fy+Jz+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 4));

                r = vget_high_f32(row2);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[2], r, 0); // Cx+Gy+Kz+O
                W = vmlaq_lane_f32(W, V.val[2], r, 1); // Dx+Hy+Lz+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 5));

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
                V.val[0] = vdivq_f32(vResult0, W);
                V.val[1] = vdivq_f32(vResult1, W);
                V.val[2] = vdivq_f32(vResult2, W);
#else
                // 2 iterations of Newton-Raphson refinement of reciprocal
                float32x4_t Reciprocal = vrecpeq_f32(W);
                float32x4_t S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);
                S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);

                V.val[0] = vmulq_f32(vResult0, Reciprocal);
                V.val[1] = vmulq_f32(vResult1, Reciprocal);
                V.val[2] = vmulq_f32(vResult2, Reciprocal);
#endif

                vst3q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT3) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        float32x2_t VL = vld1_f32(reinterpret_cast<const float*>(pInputVector));
        float32x2_t zero = vdup_n_f32(0);
        float32x2_t VH = vld1_lane_f32(reinterpret_cast<const float*>(pInputVector) + 2, zero, 0);
        pInputVector += InputStride;

        XMVECTOR vResult = vmlaq_lane_f32(row3, row0, VL, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, VL, 1); // Y
        vResult = vmlaq_lane_f32(vResult, row2, VH, 0); // Z

        VH = vget_high_f32(vResult);
        XMVECTOR W = vdupq_lane_f32(VH, 1);

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
        vResult = vdivq_f32(vResult, W);
#else
        // 2 iterations of Newton-Raphson refinement of reciprocal for W
        float32x4_t Reciprocal = vrecpeq_f32(W);
        float32x4_t S = vrecpsq_f32(Reciprocal, W);
        Reciprocal = vmulq_f32(S, Reciprocal);
        S = vrecpsq_f32(Reciprocal, W);
        Reciprocal = vmulq_f32(S, Reciprocal);

        vResult = vmulq_f32(vResult, Reciprocal);
#endif

        VL = vget_low_f32(vResult);
        vst1_f32(reinterpret_cast<float*>(pOutputVector), VL);
        vst1q_lane_f32(reinterpret_cast<float*>(pOutputVector) + 2, vResult, 2);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(XMFLOAT3))
        {
            if (OutputStride == sizeof(XMFLOAT3))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V1 = _mm_div_ps(vTemp, W);

                        // Result 2
                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, row2, row3);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V2 = _mm_div_ps(vTemp, W);

                        // Result 3
                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, row2, row3);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V3 = _mm_div_ps(vTemp, W);

                        // Result 4
                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, row2, row3);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V4 = _mm_div_ps(vTemp, W);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), V1);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, unaligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V1 = _mm_div_ps(vTemp, W);

                        // Result 2
                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, row2, row3);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V2 = _mm_div_ps(vTemp, W);

                        // Result 3
                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, row2, row3);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V3 = _mm_div_ps(vTemp, W);

                        // Result 4
                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, row2, row3);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                        V4 = _mm_div_ps(vTemp, W);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), V1);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                    __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                    pInputVector += sizeof(XMFLOAT3) * 4;

                    // Unpack the 4 vectors (.w components are junk)
                    XM3UNPACK3INTO4(V1, L2, L3);

                    // Result 1
                    XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                    XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
                    XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                    XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                    vTemp = _mm_div_ps(vTemp, W);
                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 2
                    Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                    vTemp = _mm_div_ps(vTemp, W);
                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 3
                    Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                    vTemp = _mm_div_ps(vTemp, W);
                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 4
                    Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, row2, row3);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

                    vTemp = _mm_div_ps(vTemp, W);
                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        pInputVector += InputStride;

        XMVECTOR Z = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

        XMVECTOR vTemp = XM_FMADD_PS(Z, row2, row3);
        XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
        XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
        vTemp = _mm_add_ps(vTemp, vTemp2);
        vTemp = _mm_add_ps(vTemp, vTemp3);

        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));

        vTemp = _mm_div_ps(vTemp, W);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
        pOutputVector += OutputStride;
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3TransformNormal
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Z = XMVectorSplatZ(V);
    XMVECTOR Y = XMVectorSplatY(V);
    XMVECTOR X = XMVectorSplatX(V);

    XMVECTOR Result = XMVectorMultiply(Z, M.r[2]);
    Result = XMVectorMultiplyAdd(Y, M.r[1], Result);
    Result = XMVectorMultiplyAdd(X, M.r[0], Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    XMVECTOR vResult = vmulq_lane_f32(M.r[0], VL, 0); // X
    vResult = vmlaq_lane_f32(vResult, M.r[1], VL, 1); // Y
    return vmlaq_lane_f32(vResult, M.r[2], vget_high_f32(V), 0); // Z
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2)); // Z
    vResult = _mm_mul_ps(vResult, M.r[2]);
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
    vResult = XM_FMADD_PS(vTemp, M.r[1], vResult);
    vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
    vResult = XM_FMADD_PS(vTemp, M.r[0], vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015 26019, "PREfast noise: Esp:1307" )
#endif

_Use_decl_annotations_
inline XMFLOAT3* XM_CALLCONV XMVector3TransformNormalStream
(
    XMFLOAT3* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT3* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT3));

    assert(OutputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT3));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        XMVECTOR Z = XMVectorSplatZ(V);
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiply(Z, row2);
        Result = XMVectorMultiplyAdd(Y, row1, Result);
        Result = XMVectorMultiplyAdd(X, row0, Result);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), Result);

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT3)) && (OutputStride == sizeof(XMFLOAT3)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x3_t V = vld3q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT3) * 4;

                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmulq_lane_f32(V.val[0], r, 0); // Ax
                XMVECTOR vResult1 = vmulq_lane_f32(V.val[0], r, 1); // Bx

                XM_PREFETCH(pInputVector);

                r = vget_high_f32(row0);
                XMVECTOR vResult2 = vmulq_lane_f32(V.val[0], r, 0); // Cx

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(row1);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[1], r, 0); // Cx+Gy

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                r = vget_low_f32(row2);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[2], r, 0); // Ax+Ey+Iz
                vResult1 = vmlaq_lane_f32(vResult1, V.val[2], r, 1); // Bx+Fy+Jz

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 4));

                r = vget_high_f32(row2);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[2], r, 0); // Cx+Gy+Kz

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 5));

                V.val[0] = vResult0;
                V.val[1] = vResult1;
                V.val[2] = vResult2;

                vst3q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT3) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        float32x2_t VL = vld1_f32(reinterpret_cast<const float*>(pInputVector));
        float32x2_t zero = vdup_n_f32(0);
        float32x2_t VH = vld1_lane_f32(reinterpret_cast<const float*>(pInputVector) + 2, zero, 0);
        pInputVector += InputStride;

        XMVECTOR vResult = vmulq_lane_f32(row0, VL, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, VL, 1); // Y
        vResult = vmlaq_lane_f32(vResult, row2, VH, 0); // Z

        VL = vget_low_f32(vResult);
        vst1_f32(reinterpret_cast<float*>(pOutputVector), VL);
        vst1q_lane_f32(reinterpret_cast<float*>(pOutputVector) + 2, vResult, 2);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(XMFLOAT3))
        {
            if (OutputStride == sizeof(XMFLOAT3))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = _mm_mul_ps(Z, row2);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V1 = _mm_add_ps(vTemp, vTemp3);

                        // Result 2
                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = _mm_mul_ps(Z, row2);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V2 = _mm_add_ps(vTemp, vTemp3);

                        // Result 3
                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = _mm_mul_ps(Z, row2);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V3 = _mm_add_ps(vTemp, vTemp3);

                        // Result 4
                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = _mm_mul_ps(Z, row2);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V4 = _mm_add_ps(vTemp, vTemp3);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), V1);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, unaligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = _mm_mul_ps(Z, row2);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V1 = _mm_add_ps(vTemp, vTemp3);

                        // Result 2
                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = _mm_mul_ps(Z, row2);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V2 = _mm_add_ps(vTemp, vTemp3);

                        // Result 3
                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = _mm_mul_ps(Z, row2);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V3 = _mm_add_ps(vTemp, vTemp3);

                        // Result 4
                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = _mm_mul_ps(Z, row2);
                        vTemp2 = _mm_mul_ps(Y, row1);
                        vTemp3 = _mm_mul_ps(X, row0);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        V4 = _mm_add_ps(vTemp, vTemp3);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), V1);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                    __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                    pInputVector += sizeof(XMFLOAT3) * 4;

                    // Unpack the 4 vectors (.w components are junk)
                    XM3UNPACK3INTO4(V1, L2, L3);

                    // Result 1
                    XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                    XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = _mm_mul_ps(Z, row2);
                    XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
                    XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 2
                    Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = _mm_mul_ps(Z, row2);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 3
                    Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = _mm_mul_ps(Z, row2);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 4
                    Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = _mm_mul_ps(Z, row2);
                    vTemp2 = _mm_mul_ps(Y, row1);
                    vTemp3 = _mm_mul_ps(X, row0);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        pInputVector += InputStride;

        XMVECTOR Z = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

        XMVECTOR vTemp = _mm_mul_ps(Z, row2);
        XMVECTOR vTemp2 = _mm_mul_ps(Y, row1);
        XMVECTOR vTemp3 = _mm_mul_ps(X, row0);
        vTemp = _mm_add_ps(vTemp, vTemp2);
        vTemp = _mm_add_ps(vTemp, vTemp3);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
        pOutputVector += OutputStride;
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Project
(
    FXMVECTOR V,
    float    ViewportX,
    float    ViewportY,
    float    ViewportWidth,
    float    ViewportHeight,
    float    ViewportMinZ,
    float    ViewportMaxZ,
    FXMMATRIX Projection,
    CXMMATRIX View,
    CXMMATRIX World
) noexcept
{
    const float HalfViewportWidth = ViewportWidth * 0.5f;
    const float HalfViewportHeight = ViewportHeight * 0.5f;

    XMVECTOR Scale = XMVectorSet(HalfViewportWidth, -HalfViewportHeight, ViewportMaxZ - ViewportMinZ, 0.0f);
    XMVECTOR Offset = XMVectorSet(ViewportX + HalfViewportWidth, ViewportY + HalfViewportHeight, ViewportMinZ, 0.0f);

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);

    XMVECTOR Result = XMVector3TransformCoord(V, Transform);

    Result = XMVectorMultiplyAdd(Result, Scale, Offset);

    return Result;
}

//------------------------------------------------------------------------------

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015 26019, "PREfast noise: Esp:1307" )
#endif

_Use_decl_annotations_
inline XMFLOAT3* XM_CALLCONV XMVector3ProjectStream
(
    XMFLOAT3* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT3* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    float           ViewportX,
    float           ViewportY,
    float           ViewportWidth,
    float           ViewportHeight,
    float           ViewportMinZ,
    float           ViewportMaxZ,
    FXMMATRIX     Projection,
    CXMMATRIX     View,
    CXMMATRIX     World
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT3));

    assert(OutputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT3));

#if defined(_XM_NO_INTRINSICS_)

    const float HalfViewportWidth = ViewportWidth * 0.5f;
    const float HalfViewportHeight = ViewportHeight * 0.5f;

    XMVECTOR Scale = XMVectorSet(HalfViewportWidth, -HalfViewportHeight, ViewportMaxZ - ViewportMinZ, 1.0f);
    XMVECTOR Offset = XMVectorSet(ViewportX + HalfViewportWidth, ViewportY + HalfViewportHeight, ViewportMinZ, 0.0f);

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));

        XMVECTOR Result = XMVector3TransformCoord(V, Transform);
        Result = XMVectorMultiplyAdd(Result, Scale, Offset);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), Result);

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    const float HalfViewportWidth = ViewportWidth * 0.5f;
    const float HalfViewportHeight = ViewportHeight * 0.5f;

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT3)) && (OutputStride == sizeof(XMFLOAT3)))
        {
            XMVECTOR ScaleX = vdupq_n_f32(HalfViewportWidth);
            XMVECTOR ScaleY = vdupq_n_f32(-HalfViewportHeight);
            XMVECTOR ScaleZ = vdupq_n_f32(ViewportMaxZ - ViewportMinZ);

            XMVECTOR OffsetX = vdupq_n_f32(ViewportX + HalfViewportWidth);
            XMVECTOR OffsetY = vdupq_n_f32(ViewportY + HalfViewportHeight);
            XMVECTOR OffsetZ = vdupq_n_f32(ViewportMinZ);

            for (size_t j = 0; j < four; ++j)
            {
                float32x4x3_t V = vld3q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT3) * 4;

                float32x2_t r3 = vget_low_f32(Transform.r[3]);
                float32x2_t r = vget_low_f32(Transform.r[0]);
                XMVECTOR vResult0 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Ax+M
                XMVECTOR vResult1 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Bx+N

                XM_PREFETCH(pInputVector);

                r3 = vget_high_f32(Transform.r[3]);
                r = vget_high_f32(Transform.r[0]);
                XMVECTOR vResult2 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), V.val[0], r, 0); // Cx+O
                XMVECTOR W = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), V.val[0], r, 1); // Dx+P

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(Transform.r[1]);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(Transform.r[1]);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[1], r, 0); // Cx+Gy+O
                W = vmlaq_lane_f32(W, V.val[1], r, 1); // Dx+Hy+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                r = vget_low_f32(Transform.r[2]);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[2], r, 0); // Ax+Ey+Iz+M
                vResult1 = vmlaq_lane_f32(vResult1, V.val[2], r, 1); // Bx+Fy+Jz+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 4));

                r = vget_high_f32(Transform.r[2]);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[2], r, 0); // Cx+Gy+Kz+O
                W = vmlaq_lane_f32(W, V.val[2], r, 1); // Dx+Hy+Lz+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 5));

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
                vResult0 = vdivq_f32(vResult0, W);
                vResult1 = vdivq_f32(vResult1, W);
                vResult2 = vdivq_f32(vResult2, W);
#else
                // 2 iterations of Newton-Raphson refinement of reciprocal
                float32x4_t Reciprocal = vrecpeq_f32(W);
                float32x4_t S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);
                S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);

                vResult0 = vmulq_f32(vResult0, Reciprocal);
                vResult1 = vmulq_f32(vResult1, Reciprocal);
                vResult2 = vmulq_f32(vResult2, Reciprocal);
#endif

                V.val[0] = vmlaq_f32(OffsetX, vResult0, ScaleX);
                V.val[1] = vmlaq_f32(OffsetY, vResult1, ScaleY);
                V.val[2] = vmlaq_f32(OffsetZ, vResult2, ScaleZ);

                vst3q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT3) * 4;

                i += 4;
            }
        }
    }

    if (i < VectorCount)
    {
        XMVECTOR Scale = XMVectorSet(HalfViewportWidth, -HalfViewportHeight, ViewportMaxZ - ViewportMinZ, 1.0f);
        XMVECTOR Offset = XMVectorSet(ViewportX + HalfViewportWidth, ViewportY + HalfViewportHeight, ViewportMinZ, 0.0f);

        for (; i < VectorCount; i++)
        {
            float32x2_t VL = vld1_f32(reinterpret_cast<const float*>(pInputVector));
            float32x2_t zero = vdup_n_f32(0);
            float32x2_t VH = vld1_lane_f32(reinterpret_cast<const float*>(pInputVector) + 2, zero, 0);
            pInputVector += InputStride;

            XMVECTOR vResult = vmlaq_lane_f32(Transform.r[3], Transform.r[0], VL, 0); // X
            vResult = vmlaq_lane_f32(vResult, Transform.r[1], VL, 1); // Y
            vResult = vmlaq_lane_f32(vResult, Transform.r[2], VH, 0); // Z

            VH = vget_high_f32(vResult);
            XMVECTOR W = vdupq_lane_f32(VH, 1);

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
            vResult = vdivq_f32(vResult, W);
#else
            // 2 iterations of Newton-Raphson refinement of reciprocal for W
            float32x4_t Reciprocal = vrecpeq_f32(W);
            float32x4_t S = vrecpsq_f32(Reciprocal, W);
            Reciprocal = vmulq_f32(S, Reciprocal);
            S = vrecpsq_f32(Reciprocal, W);
            Reciprocal = vmulq_f32(S, Reciprocal);

            vResult = vmulq_f32(vResult, Reciprocal);
#endif

            vResult = vmlaq_f32(Offset, vResult, Scale);

            VL = vget_low_f32(vResult);
            vst1_f32(reinterpret_cast<float*>(pOutputVector), VL);
            vst1q_lane_f32(reinterpret_cast<float*>(pOutputVector) + 2, vResult, 2);
            pOutputVector += OutputStride;
        }
    }

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    const float HalfViewportWidth = ViewportWidth * 0.5f;
    const float HalfViewportHeight = ViewportHeight * 0.5f;

    XMVECTOR Scale = XMVectorSet(HalfViewportWidth, -HalfViewportHeight, ViewportMaxZ - ViewportMinZ, 1.0f);
    XMVECTOR Offset = XMVectorSet(ViewportX + HalfViewportWidth, ViewportY + HalfViewportHeight, ViewportMinZ, 0.0f);

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(XMFLOAT3))
        {
            if (OutputStride == sizeof(XMFLOAT3))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V1 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Result 2
                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V2 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Result 3
                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V3 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Result 4
                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V4 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), V1);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, unaligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V1 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Result 2
                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V2 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Result 3
                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V3 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Result 4
                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        vTemp = _mm_div_ps(vTemp, W);
                        V4 = XM_FMADD_PS(vTemp, Scale, Offset);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), V1);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                    __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                    pInputVector += sizeof(XMFLOAT3) * 4;

                    // Unpack the 4 vectors (.w components are junk)
                    XM3UNPACK3INTO4(V1, L2, L3);

                    // Result 1
                    XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                    XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);
                    vTemp = XM_FMADD_PS(vTemp, Scale, Offset);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 2
                    Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);
                    vTemp = XM_FMADD_PS(vTemp, Scale, Offset);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 3
                    Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);
                    vTemp = XM_FMADD_PS(vTemp, Scale, Offset);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 4
                    Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);
                    vTemp = XM_FMADD_PS(vTemp, Scale, Offset);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        pInputVector += InputStride;

        XMVECTOR Z = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

        XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
        XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
        XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
        vTemp = _mm_add_ps(vTemp, vTemp2);
        vTemp = _mm_add_ps(vTemp, vTemp3);

        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
        vTemp = _mm_div_ps(vTemp, W);
        vTemp = XM_FMADD_PS(vTemp, Scale, Offset);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
        pOutputVector += OutputStride;
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector3Unproject
(
    FXMVECTOR V,
    float     ViewportX,
    float     ViewportY,
    float     ViewportWidth,
    float     ViewportHeight,
    float     ViewportMinZ,
    float     ViewportMaxZ,
    FXMMATRIX Projection,
    CXMMATRIX View,
    CXMMATRIX World
) noexcept
{
    static const XMVECTORF32 D = { { { -1.0f, 1.0f, 0.0f, 0.0f } } };

    XMVECTOR Scale = XMVectorSet(ViewportWidth * 0.5f, -ViewportHeight * 0.5f, ViewportMaxZ - ViewportMinZ, 1.0f);
    Scale = XMVectorReciprocal(Scale);

    XMVECTOR Offset = XMVectorSet(-ViewportX, -ViewportY, -ViewportMinZ, 0.0f);
    Offset = XMVectorMultiplyAdd(Scale, Offset, D.v);

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);
    Transform = XMMatrixInverse(nullptr, Transform);

    XMVECTOR Result = XMVectorMultiplyAdd(V, Scale, Offset);

    return XMVector3TransformCoord(Result, Transform);
}

//------------------------------------------------------------------------------

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015 26019, "PREfast noise: Esp:1307" )
#endif

_Use_decl_annotations_
inline XMFLOAT3* XM_CALLCONV XMVector3UnprojectStream
(
    XMFLOAT3* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT3* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    float           ViewportX,
    float           ViewportY,
    float           ViewportWidth,
    float           ViewportHeight,
    float           ViewportMinZ,
    float           ViewportMaxZ,
    FXMMATRIX       Projection,
    CXMMATRIX       View,
    CXMMATRIX       World
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT3));

    assert(OutputStride >= sizeof(XMFLOAT3));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT3));

#if defined(_XM_NO_INTRINSICS_)

    static const XMVECTORF32 D = { { { -1.0f, 1.0f, 0.0f, 0.0f } } };

    XMVECTOR Scale = XMVectorSet(ViewportWidth * 0.5f, -ViewportHeight * 0.5f, ViewportMaxZ - ViewportMinZ, 1.0f);
    Scale = XMVectorReciprocal(Scale);

    XMVECTOR Offset = XMVectorSet(-ViewportX, -ViewportY, -ViewportMinZ, 0.0f);
    Offset = XMVectorMultiplyAdd(Scale, Offset, D.v);

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);
    Transform = XMMatrixInverse(nullptr, Transform);

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));

        XMVECTOR Result = XMVectorMultiplyAdd(V, Scale, Offset);

        Result = XMVector3TransformCoord(Result, Transform);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), Result);

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);
    Transform = XMMatrixInverse(nullptr, Transform);

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    float sx = 1.f / (ViewportWidth * 0.5f);
    float sy = 1.f / (-ViewportHeight * 0.5f);
    float sz = 1.f / (ViewportMaxZ - ViewportMinZ);

    float ox = (-ViewportX * sx) - 1.f;
    float oy = (-ViewportY * sy) + 1.f;
    float oz = (-ViewportMinZ * sz);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT3)) && (OutputStride == sizeof(XMFLOAT3)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x3_t V = vld3q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT3) * 4;

                XMVECTOR ScaleX = vdupq_n_f32(sx);
                XMVECTOR OffsetX = vdupq_n_f32(ox);
                XMVECTOR VX = vmlaq_f32(OffsetX, ScaleX, V.val[0]);

                float32x2_t r3 = vget_low_f32(Transform.r[3]);
                float32x2_t r = vget_low_f32(Transform.r[0]);
                XMVECTOR vResult0 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), VX, r, 0); // Ax+M
                XMVECTOR vResult1 = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), VX, r, 1); // Bx+N

                XM_PREFETCH(pInputVector);

                r3 = vget_high_f32(Transform.r[3]);
                r = vget_high_f32(Transform.r[0]);
                XMVECTOR vResult2 = vmlaq_lane_f32(vdupq_lane_f32(r3, 0), VX, r, 0); // Cx+O
                XMVECTOR W = vmlaq_lane_f32(vdupq_lane_f32(r3, 1), VX, r, 1); // Dx+P

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                XMVECTOR ScaleY = vdupq_n_f32(sy);
                XMVECTOR OffsetY = vdupq_n_f32(oy);
                XMVECTOR VY = vmlaq_f32(OffsetY, ScaleY, V.val[1]);

                r = vget_low_f32(Transform.r[1]);
                vResult0 = vmlaq_lane_f32(vResult0, VY, r, 0); // Ax+Ey+M
                vResult1 = vmlaq_lane_f32(vResult1, VY, r, 1); // Bx+Fy+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(Transform.r[1]);
                vResult2 = vmlaq_lane_f32(vResult2, VY, r, 0); // Cx+Gy+O
                W = vmlaq_lane_f32(W, VY, r, 1); // Dx+Hy+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                XMVECTOR ScaleZ = vdupq_n_f32(sz);
                XMVECTOR OffsetZ = vdupq_n_f32(oz);
                XMVECTOR VZ = vmlaq_f32(OffsetZ, ScaleZ, V.val[2]);

                r = vget_low_f32(Transform.r[2]);
                vResult0 = vmlaq_lane_f32(vResult0, VZ, r, 0); // Ax+Ey+Iz+M
                vResult1 = vmlaq_lane_f32(vResult1, VZ, r, 1); // Bx+Fy+Jz+N

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 4));

                r = vget_high_f32(Transform.r[2]);
                vResult2 = vmlaq_lane_f32(vResult2, VZ, r, 0); // Cx+Gy+Kz+O
                W = vmlaq_lane_f32(W, VZ, r, 1); // Dx+Hy+Lz+P

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 5));

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
                V.val[0] = vdivq_f32(vResult0, W);
                V.val[1] = vdivq_f32(vResult1, W);
                V.val[2] = vdivq_f32(vResult2, W);
#else
                // 2 iterations of Newton-Raphson refinement of reciprocal
                float32x4_t Reciprocal = vrecpeq_f32(W);
                float32x4_t S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);
                S = vrecpsq_f32(Reciprocal, W);
                Reciprocal = vmulq_f32(S, Reciprocal);

                V.val[0] = vmulq_f32(vResult0, Reciprocal);
                V.val[1] = vmulq_f32(vResult1, Reciprocal);
                V.val[2] = vmulq_f32(vResult2, Reciprocal);
#endif

                vst3q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT3) * 4;

                i += 4;
            }
        }
    }

    if (i < VectorCount)
    {
        float32x2_t ScaleL = vcreate_f32(
            static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&sx))
            | (static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&sy)) << 32));
        float32x2_t ScaleH = vcreate_f32(static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&sz)));

        float32x2_t OffsetL = vcreate_f32(
            static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&ox))
            | (static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&oy)) << 32));
        float32x2_t OffsetH = vcreate_f32(static_cast<uint64_t>(*reinterpret_cast<const uint32_t*>(&oz)));

        for (; i < VectorCount; i++)
        {
            float32x2_t VL = vld1_f32(reinterpret_cast<const float*>(pInputVector));
            float32x2_t zero = vdup_n_f32(0);
            float32x2_t VH = vld1_lane_f32(reinterpret_cast<const float*>(pInputVector) + 2, zero, 0);
            pInputVector += InputStride;

            VL = vmla_f32(OffsetL, VL, ScaleL);
            VH = vmla_f32(OffsetH, VH, ScaleH);

            XMVECTOR vResult = vmlaq_lane_f32(Transform.r[3], Transform.r[0], VL, 0); // X
            vResult = vmlaq_lane_f32(vResult, Transform.r[1], VL, 1); // Y
            vResult = vmlaq_lane_f32(vResult, Transform.r[2], VH, 0); // Z

            VH = vget_high_f32(vResult);
            XMVECTOR W = vdupq_lane_f32(VH, 1);

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__
            vResult = vdivq_f32(vResult, W);
#else
            // 2 iterations of Newton-Raphson refinement of reciprocal for W
            float32x4_t Reciprocal = vrecpeq_f32(W);
            float32x4_t S = vrecpsq_f32(Reciprocal, W);
            Reciprocal = vmulq_f32(S, Reciprocal);
            S = vrecpsq_f32(Reciprocal, W);
            Reciprocal = vmulq_f32(S, Reciprocal);

            vResult = vmulq_f32(vResult, Reciprocal);
#endif

            VL = vget_low_f32(vResult);
            vst1_f32(reinterpret_cast<float*>(pOutputVector), VL);
            vst1q_lane_f32(reinterpret_cast<float*>(pOutputVector) + 2, vResult, 2);
            pOutputVector += OutputStride;
        }
    }

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 D = { { { -1.0f, 1.0f, 0.0f, 0.0f } } };

    XMVECTOR Scale = XMVectorSet(ViewportWidth * 0.5f, -ViewportHeight * 0.5f, ViewportMaxZ - ViewportMinZ, 1.0f);
    Scale = XMVectorReciprocal(Scale);

    XMVECTOR Offset = XMVectorSet(-ViewportX, -ViewportY, -ViewportMinZ, 0.0f);
    Offset = _mm_mul_ps(Scale, Offset);
    Offset = _mm_add_ps(Offset, D);

    XMMATRIX Transform = XMMatrixMultiply(World, View);
    Transform = XMMatrixMultiply(Transform, Projection);
    Transform = XMMatrixInverse(nullptr, Transform);

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(XMFLOAT3))
        {
            if (OutputStride == sizeof(XMFLOAT3))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        V1 = XM_FMADD_PS(V1, Scale, Offset);

                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V1 = _mm_div_ps(vTemp, W);

                        // Result 2
                        V2 = XM_FMADD_PS(V2, Scale, Offset);

                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V2 = _mm_div_ps(vTemp, W);

                        // Result 3
                        V3 = XM_FMADD_PS(V3, Scale, Offset);

                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V3 = _mm_div_ps(vTemp, W);

                        // Result 4
                        V4 = XM_FMADD_PS(V4, Scale, Offset);

                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V4 = _mm_div_ps(vTemp, W);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), V1);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, unaligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                        __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                        pInputVector += sizeof(XMFLOAT3) * 4;

                        // Unpack the 4 vectors (.w components are junk)
                        XM3UNPACK3INTO4(V1, L2, L3);

                        // Result 1
                        V1 = XM_FMADD_PS(V1, Scale, Offset);

                        XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                        XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                        XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                        XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V1 = _mm_div_ps(vTemp, W);

                        // Result 2
                        V2 = XM_FMADD_PS(V2, Scale, Offset);

                        Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V2 = _mm_div_ps(vTemp, W);

                        // Result 3
                        V3 = XM_FMADD_PS(V3, Scale, Offset);

                        Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V3 = _mm_div_ps(vTemp, W);

                        // Result 4
                        V4 = XM_FMADD_PS(V4, Scale, Offset);

                        Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                        Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                        X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                        vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                        vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                        vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                        vTemp = _mm_add_ps(vTemp, vTemp2);
                        vTemp = _mm_add_ps(vTemp, vTemp3);

                        W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                        V4 = _mm_div_ps(vTemp, W);

                        // Pack and store the vectors
                        XM3PACK4INTO3(vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), V1);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 16), vTemp);
                        _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector + 32), V3);
                        pOutputVector += sizeof(XMFLOAT3) * 4;
                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128 V1 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    __m128 L2 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 16));
                    __m128 L3 = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector + 32));
                    pInputVector += sizeof(XMFLOAT3) * 4;

                    // Unpack the 4 vectors (.w components are junk)
                    XM3UNPACK3INTO4(V1, L2, L3);

                    // Result 1
                    V1 = XM_FMADD_PS(V1, Scale, Offset);

                    XMVECTOR Z = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 2, 2, 2));
                    XMVECTOR Y = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 1, 1));
                    XMVECTOR X = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 0));

                    XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 2
                    V2 = XM_FMADD_PS(V2, Scale, Offset);

                    Z = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V2, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 3
                    V3 = XM_FMADD_PS(V3, Scale, Offset);

                    Z = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    // Result 4
                    V4 = XM_FMADD_PS(V4, Scale, Offset);

                    Z = XM_PERMUTE_PS(V4, _MM_SHUFFLE(2, 2, 2, 2));
                    Y = XM_PERMUTE_PS(V4, _MM_SHUFFLE(1, 1, 1, 1));
                    X = XM_PERMUTE_PS(V4, _MM_SHUFFLE(0, 0, 0, 0));

                    vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
                    vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
                    vTemp3 = _mm_mul_ps(X, Transform.r[0]);
                    vTemp = _mm_add_ps(vTemp, vTemp2);
                    vTemp = _mm_add_ps(vTemp, vTemp3);

                    W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
                    vTemp = _mm_div_ps(vTemp, W);

                    XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
                    pOutputVector += OutputStride;

                    i += 4;
                }
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pInputVector));
        pInputVector += InputStride;

        V = _mm_mul_ps(V, Scale);
        V = _mm_add_ps(V, Offset);

        XMVECTOR Z = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
        XMVECTOR Y = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
        XMVECTOR X = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));

        XMVECTOR vTemp = XM_FMADD_PS(Z, Transform.r[2], Transform.r[3]);
        XMVECTOR vTemp2 = _mm_mul_ps(Y, Transform.r[1]);
        XMVECTOR vTemp3 = _mm_mul_ps(X, Transform.r[0]);
        vTemp = _mm_add_ps(vTemp, vTemp2);
        vTemp = _mm_add_ps(vTemp, vTemp3);

        XMVECTOR W = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 3, 3, 3));
        vTemp = _mm_div_ps(vTemp, W);

        XMStoreFloat3(reinterpret_cast<XMFLOAT3*>(pOutputVector), vTemp);
        pOutputVector += OutputStride;
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

/****************************************************************************
 *
 * 4D Vector
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------
 // Comparison operations
 //------------------------------------------------------------------------------

 //------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4Equal
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] == V2.vector4_f32[0]) && (V1.vector4_f32[1] == V2.vector4_f32[1]) && (V1.vector4_f32[2] == V2.vector4_f32[2]) && (V1.vector4_f32[3] == V2.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    return ((_mm_movemask_ps(vTemp) == 0x0f) != 0);
#else
    return XMComparisonAllTrue(XMVector4EqualR(V1, V2));
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector4EqualR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    uint32_t CR = 0;

    if ((V1.vector4_f32[0] == V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] == V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] == V2.vector4_f32[2]) &&
        (V1.vector4_f32[3] == V2.vector4_f32[3]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] != V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] != V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] != V2.vector4_f32[2]) &&
        (V1.vector4_f32[3] != V2.vector4_f32[3]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);

    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpeq_ps(V1, V2);
    int iTest = _mm_movemask_ps(vTemp);
    uint32_t CR = 0;
    if (iTest == 0xf)     // All equal?
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (iTest == 0)  // All not equal?
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4EqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_u32[0] == V2.vector4_u32[0]) && (V1.vector4_u32[1] == V2.vector4_u32[1]) && (V1.vector4_u32[2] == V2.vector4_u32[2]) && (V1.vector4_u32[3] == V2.vector4_u32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return ((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) == 0xf) != 0);
#else
    return XMComparisonAllTrue(XMVector4EqualIntR(V1, V2));
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector4EqualIntR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t CR = 0;
    if (V1.vector4_u32[0] == V2.vector4_u32[0] &&
        V1.vector4_u32[1] == V2.vector4_u32[1] &&
        V1.vector4_u32[2] == V2.vector4_u32[2] &&
        V1.vector4_u32[3] == V2.vector4_u32[3])
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (V1.vector4_u32[0] != V2.vector4_u32[0] &&
        V1.vector4_u32[1] != V2.vector4_u32[1] &&
        V1.vector4_u32[2] != V2.vector4_u32[2] &&
        V1.vector4_u32[3] != V2.vector4_u32[3])
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);

    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    int iTest = _mm_movemask_ps(_mm_castsi128_ps(vTemp));
    uint32_t CR = 0;
    if (iTest == 0xf)     // All equal?
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (iTest == 0)  // All not equal?
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

inline bool XM_CALLCONV XMVector4NearEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR Epsilon
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float dx, dy, dz, dw;

    dx = fabsf(V1.vector4_f32[0] - V2.vector4_f32[0]);
    dy = fabsf(V1.vector4_f32[1] - V2.vector4_f32[1]);
    dz = fabsf(V1.vector4_f32[2] - V2.vector4_f32[2]);
    dw = fabsf(V1.vector4_f32[3] - V2.vector4_f32[3]);
    return (((dx <= Epsilon.vector4_f32[0]) &&
        (dy <= Epsilon.vector4_f32[1]) &&
        (dz <= Epsilon.vector4_f32[2]) &&
        (dw <= Epsilon.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vDelta = vsubq_f32(V1, V2);
#if defined(_MSC_VER) && !defined(__clang__) && !defined(_ARM64_DISTINCT_NEON_TYPES)
    uint32x4_t vResult = vacleq_f32(vDelta, Epsilon);
#else
    uint32x4_t vResult = vcleq_f32(vabsq_f32(vDelta), Epsilon);
#endif
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    // Get the difference
    XMVECTOR vDelta = _mm_sub_ps(V1, V2);
    // Get the absolute value of the difference
    XMVECTOR vTemp = _mm_setzero_ps();
    vTemp = _mm_sub_ps(vTemp, vDelta);
    vTemp = _mm_max_ps(vTemp, vDelta);
    vTemp = _mm_cmple_ps(vTemp, Epsilon);
    return ((_mm_movemask_ps(vTemp) == 0xf) != 0);
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4NotEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] != V2.vector4_f32[0]) || (V1.vector4_f32[1] != V2.vector4_f32[1]) || (V1.vector4_f32[2] != V2.vector4_f32[2]) || (V1.vector4_f32[3] != V2.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) != 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpneq_ps(V1, V2);
    return ((_mm_movemask_ps(vTemp)) != 0);
#else
    return XMComparisonAnyFalse(XMVector4EqualR(V1, V2));
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4NotEqualInt
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_u32[0] != V2.vector4_u32[0]) || (V1.vector4_u32[1] != V2.vector4_u32[1]) || (V1.vector4_u32[2] != V2.vector4_u32[2]) || (V1.vector4_u32[3] != V2.vector4_u32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vceqq_u32(vreinterpretq_u32_f32(V1), vreinterpretq_u32_f32(V2));
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) != 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    __m128i vTemp = _mm_cmpeq_epi32(_mm_castps_si128(V1), _mm_castps_si128(V2));
    return ((_mm_movemask_ps(_mm_castsi128_ps(vTemp)) != 0xF) != 0);
#else
    return XMComparisonAnyFalse(XMVector4EqualIntR(V1, V2));
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4Greater
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] > V2.vector4_f32[0]) && (V1.vector4_f32[1] > V2.vector4_f32[1]) && (V1.vector4_f32[2] > V2.vector4_f32[2]) && (V1.vector4_f32[3] > V2.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgtq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    return ((_mm_movemask_ps(vTemp) == 0x0f) != 0);
#else
    return XMComparisonAllTrue(XMVector4GreaterR(V1, V2));
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector4GreaterR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t CR = 0;
    if (V1.vector4_f32[0] > V2.vector4_f32[0] &&
        V1.vector4_f32[1] > V2.vector4_f32[1] &&
        V1.vector4_f32[2] > V2.vector4_f32[2] &&
        V1.vector4_f32[3] > V2.vector4_f32[3])
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (V1.vector4_f32[0] <= V2.vector4_f32[0] &&
        V1.vector4_f32[1] <= V2.vector4_f32[1] &&
        V1.vector4_f32[2] <= V2.vector4_f32[2] &&
        V1.vector4_f32[3] <= V2.vector4_f32[3])
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgtq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);

    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    uint32_t CR = 0;
    XMVECTOR vTemp = _mm_cmpgt_ps(V1, V2);
    int iTest = _mm_movemask_ps(vTemp);
    if (iTest == 0xf)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4GreaterOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] >= V2.vector4_f32[0]) && (V1.vector4_f32[1] >= V2.vector4_f32[1]) && (V1.vector4_f32[2] >= V2.vector4_f32[2]) && (V1.vector4_f32[3] >= V2.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgeq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    return ((_mm_movemask_ps(vTemp) == 0x0f) != 0);
#else
    return XMComparisonAllTrue(XMVector4GreaterOrEqualR(V1, V2));
#endif
}

//------------------------------------------------------------------------------

inline uint32_t XM_CALLCONV XMVector4GreaterOrEqualR
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    uint32_t CR = 0;
    if ((V1.vector4_f32[0] >= V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] >= V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] >= V2.vector4_f32[2]) &&
        (V1.vector4_f32[3] >= V2.vector4_f32[3]))
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if ((V1.vector4_f32[0] < V2.vector4_f32[0]) &&
        (V1.vector4_f32[1] < V2.vector4_f32[1]) &&
        (V1.vector4_f32[2] < V2.vector4_f32[2]) &&
        (V1.vector4_f32[3] < V2.vector4_f32[3]))
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcgeq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    uint32_t r = vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1);

    uint32_t CR = 0;
    if (r == 0xFFFFFFFFU)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!r)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#elif defined(_XM_SSE_INTRINSICS_)
    uint32_t CR = 0;
    XMVECTOR vTemp = _mm_cmpge_ps(V1, V2);
    int iTest = _mm_movemask_ps(vTemp);
    if (iTest == 0x0f)
    {
        CR = XM_CRMASK_CR6TRUE;
    }
    else if (!iTest)
    {
        CR = XM_CRMASK_CR6FALSE;
    }
    return CR;
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4Less
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] < V2.vector4_f32[0]) && (V1.vector4_f32[1] < V2.vector4_f32[1]) && (V1.vector4_f32[2] < V2.vector4_f32[2]) && (V1.vector4_f32[3] < V2.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcltq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmplt_ps(V1, V2);
    return ((_mm_movemask_ps(vTemp) == 0x0f) != 0);
#else
    return XMComparisonAllTrue(XMVector4GreaterR(V2, V1));
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4LessOrEqual
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V1.vector4_f32[0] <= V2.vector4_f32[0]) && (V1.vector4_f32[1] <= V2.vector4_f32[1]) && (V1.vector4_f32[2] <= V2.vector4_f32[2]) && (V1.vector4_f32[3] <= V2.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vResult = vcleq_f32(V1, V2);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vResult)), vget_high_u8(vreinterpretq_u8_u32(vResult)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp = _mm_cmple_ps(V1, V2);
    return ((_mm_movemask_ps(vTemp) == 0x0f) != 0);
#else
    return XMComparisonAllTrue(XMVector4GreaterOrEqualR(V2, V1));
#endif
}

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4InBounds
(
    FXMVECTOR V,
    FXMVECTOR Bounds
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (((V.vector4_f32[0] <= Bounds.vector4_f32[0] && V.vector4_f32[0] >= -Bounds.vector4_f32[0]) &&
        (V.vector4_f32[1] <= Bounds.vector4_f32[1] && V.vector4_f32[1] >= -Bounds.vector4_f32[1]) &&
        (V.vector4_f32[2] <= Bounds.vector4_f32[2] && V.vector4_f32[2] >= -Bounds.vector4_f32[2]) &&
        (V.vector4_f32[3] <= Bounds.vector4_f32[3] && V.vector4_f32[3] >= -Bounds.vector4_f32[3])) != 0);
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Test if less than or equal
    uint32x4_t ivTemp1 = vcleq_f32(V, Bounds);
    // Negate the bounds
    float32x4_t vTemp2 = vnegq_f32(Bounds);
    // Test if greater or equal (Reversed)
    uint32x4_t ivTemp2 = vcleq_f32(vTemp2, V);
    // Blend answers
    ivTemp1 = vandq_u32(ivTemp1, ivTemp2);
    // in bounds?
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(ivTemp1)), vget_high_u8(vreinterpretq_u8_u32(ivTemp1)));
    uint16x4x2_t vTemp3 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp3.val[1]), 1) == 0xFFFFFFFFU);
#elif defined(_XM_SSE_INTRINSICS_)
    // Test if less than or equal
    XMVECTOR vTemp1 = _mm_cmple_ps(V, Bounds);
    // Negate the bounds
    XMVECTOR vTemp2 = _mm_mul_ps(Bounds, g_XMNegativeOne);
    // Test if greater or equal (Reversed)
    vTemp2 = _mm_cmple_ps(vTemp2, V);
    // Blend answers
    vTemp1 = _mm_and_ps(vTemp1, vTemp2);
    // All in bounds?
    return ((_mm_movemask_ps(vTemp1) == 0x0f) != 0);
#else
    return XMComparisonAllInBounds(XMVector4InBoundsR(V, Bounds));
#endif
}

//------------------------------------------------------------------------------

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(push)
#pragma float_control(precise, on)
#endif

inline bool XM_CALLCONV XMVector4IsNaN(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    return (XMISNAN(V.vector4_f32[0]) ||
        XMISNAN(V.vector4_f32[1]) ||
        XMISNAN(V.vector4_f32[2]) ||
        XMISNAN(V.vector4_f32[3]));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    return isnan(vgetq_lane_f32(V, 0)) || isnan(vgetq_lane_f32(V, 1)) || isnan(vgetq_lane_f32(V, 2)) || isnan(vgetq_lane_f32(V, 3));
    #else
    // Test against itself. NaN is always not equal
    uint32x4_t vTempNan = vceqq_f32(V, V);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vTempNan)), vget_high_u8(vreinterpretq_u8_u32(vTempNan)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    // If any are NaN, the mask is zero
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) != 0xFFFFFFFFU);
    #endif
#elif defined(_XM_SSE_INTRINSICS_)
    #if defined(__clang__) && defined(__FINITE_MATH_ONLY__)
    XM_ALIGNED_DATA(16) float tmp[4];
    _mm_store_ps(tmp, V);
    return isnan(tmp[0]) || isnan(tmp[1]) || isnan(tmp[2]) || isnan(tmp[3]);
    #else
    // Test against itself. NaN is always not equal
    XMVECTOR vTempNan = _mm_cmpneq_ps(V, V);
    // If any are NaN, the mask is non-zero
    return (_mm_movemask_ps(vTempNan) != 0);
    #endif
#endif
}

#if !defined(_XM_NO_INTRINSICS_) && defined(_MSC_VER) && !defined(__INTEL_COMPILER)
#pragma float_control(pop)
#endif

//------------------------------------------------------------------------------

inline bool XM_CALLCONV XMVector4IsInfinite(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    return (XMISINF(V.vector4_f32[0]) ||
        XMISINF(V.vector4_f32[1]) ||
        XMISINF(V.vector4_f32[2]) ||
        XMISINF(V.vector4_f32[3]));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Mask off the sign bit
    uint32x4_t vTempInf = vandq_u32(vreinterpretq_u32_f32(V), g_XMAbsMask);
    // Compare to infinity
    vTempInf = vceqq_f32(vreinterpretq_f32_u32(vTempInf), g_XMInfinity);
    // If any are infinity, the signs are true.
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(vTempInf)), vget_high_u8(vreinterpretq_u8_u32(vTempInf)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));
    return (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) != 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Mask off the sign bit
    XMVECTOR vTemp = _mm_and_ps(V, g_XMAbsMask);
    // Compare to infinity
    vTemp = _mm_cmpeq_ps(vTemp, g_XMInfinity);
    // If any are infinity, the signs are true.
    return (_mm_movemask_ps(vTemp) != 0);
#endif
}

//------------------------------------------------------------------------------
// Computation operations
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Dot
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result;
    Result.f[0] =
        Result.f[1] =
        Result.f[2] =
        Result.f[3] = V1.vector4_f32[0] * V2.vector4_f32[0] + V1.vector4_f32[1] * V2.vector4_f32[1] + V1.vector4_f32[2] * V2.vector4_f32[2] + V1.vector4_f32[3] * V2.vector4_f32[3];
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vTemp = vmulq_f32(V1, V2);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    return vcombine_f32(v1, v1);
#elif defined(_XM_SSE4_INTRINSICS_)
    return _mm_dp_ps(V1, V2, 0xff);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vTemp = _mm_mul_ps(V1, V2);
    vTemp = _mm_hadd_ps(vTemp, vTemp);
    return _mm_hadd_ps(vTemp, vTemp);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vTemp2 = V2;
    XMVECTOR vTemp = _mm_mul_ps(V1, vTemp2);
    vTemp2 = _mm_shuffle_ps(vTemp2, vTemp, _MM_SHUFFLE(1, 0, 0, 0)); // Copy X to the Z position and Y to the W position
    vTemp2 = _mm_add_ps(vTemp2, vTemp);          // Add Z = X+Z; W = Y+W;
    vTemp = _mm_shuffle_ps(vTemp, vTemp2, _MM_SHUFFLE(0, 3, 0, 0));  // Copy W to the Z position
    vTemp = _mm_add_ps(vTemp, vTemp2);           // Add Z and W together
    return XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(2, 2, 2, 2));    // Splat Z and return
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Cross
(
    FXMVECTOR V1,
    FXMVECTOR V2,
    FXMVECTOR V3
) noexcept
{
    // [ ((v2.z*v3.w-v2.w*v3.z)*v1.y)-((v2.y*v3.w-v2.w*v3.y)*v1.z)+((v2.y*v3.z-v2.z*v3.y)*v1.w),
    //   ((v2.w*v3.z-v2.z*v3.w)*v1.x)-((v2.w*v3.x-v2.x*v3.w)*v1.z)+((v2.z*v3.x-v2.x*v3.z)*v1.w),
    //   ((v2.y*v3.w-v2.w*v3.y)*v1.x)-((v2.x*v3.w-v2.w*v3.x)*v1.y)+((v2.x*v3.y-v2.y*v3.x)*v1.w),
    //   ((v2.z*v3.y-v2.y*v3.z)*v1.x)-((v2.z*v3.x-v2.x*v3.z)*v1.y)+((v2.y*v3.x-v2.x*v3.y)*v1.z) ]

#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            (((V2.vector4_f32[2] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[2])) * V1.vector4_f32[1]) - (((V2.vector4_f32[1] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[1])) * V1.vector4_f32[2]) + (((V2.vector4_f32[1] * V3.vector4_f32[2]) - (V2.vector4_f32[2] * V3.vector4_f32[1])) * V1.vector4_f32[3]),
            (((V2.vector4_f32[3] * V3.vector4_f32[2]) - (V2.vector4_f32[2] * V3.vector4_f32[3])) * V1.vector4_f32[0]) - (((V2.vector4_f32[3] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[3])) * V1.vector4_f32[2]) + (((V2.vector4_f32[2] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[2])) * V1.vector4_f32[3]),
            (((V2.vector4_f32[1] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[1])) * V1.vector4_f32[0]) - (((V2.vector4_f32[0] * V3.vector4_f32[3]) - (V2.vector4_f32[3] * V3.vector4_f32[0])) * V1.vector4_f32[1]) + (((V2.vector4_f32[0] * V3.vector4_f32[1]) - (V2.vector4_f32[1] * V3.vector4_f32[0])) * V1.vector4_f32[3]),
            (((V2.vector4_f32[2] * V3.vector4_f32[1]) - (V2.vector4_f32[1] * V3.vector4_f32[2])) * V1.vector4_f32[0]) - (((V2.vector4_f32[2] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[2])) * V1.vector4_f32[1]) + (((V2.vector4_f32[1] * V3.vector4_f32[0]) - (V2.vector4_f32[0] * V3.vector4_f32[1])) * V1.vector4_f32[2]),
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    const uint32x2_t select = vget_low_u32(g_XMMaskX);

    // Term1: V2zwyz * V3wzwy
    const float32x2_t v2xy = vget_low_f32(V2);
    const float32x2_t v2zw = vget_high_f32(V2);
    const float32x2_t v2yx = vrev64_f32(v2xy);
    const float32x2_t v2wz = vrev64_f32(v2zw);
    const float32x2_t v2yz = vbsl_f32(select, v2yx, v2wz);

    const float32x2_t v3zw = vget_high_f32(V3);
    const float32x2_t v3wz = vrev64_f32(v3zw);
    const float32x2_t v3xy = vget_low_f32(V3);
    const float32x2_t v3wy = vbsl_f32(select, v3wz, v3xy);

    float32x4_t vTemp1 = vcombine_f32(v2zw, v2yz);
    float32x4_t vTemp2 = vcombine_f32(v3wz, v3wy);
    XMVECTOR vResult = vmulq_f32(vTemp1, vTemp2);

    // - V2wzwy * V3zwyz
    const float32x2_t v2wy = vbsl_f32(select, v2wz, v2xy);

    const float32x2_t v3yx = vrev64_f32(v3xy);
    const float32x2_t v3yz = vbsl_f32(select, v3yx, v3wz);

    vTemp1 = vcombine_f32(v2wz, v2wy);
    vTemp2 = vcombine_f32(v3zw, v3yz);
    vResult = vmlsq_f32(vResult, vTemp1, vTemp2);

    // term1 * V1yxxx
    const float32x2_t v1xy = vget_low_f32(V1);
    const float32x2_t v1yx = vrev64_f32(v1xy);

    vTemp1 = vcombine_f32(v1yx, vdup_lane_f32(v1yx, 1));
    vResult = vmulq_f32(vResult, vTemp1);

    // Term2: V2ywxz * V3wxwx
    const float32x2_t v2yw = vrev64_f32(v2wy);
    const float32x2_t v2xz = vbsl_f32(select, v2xy, v2wz);

    const float32x2_t v3wx = vbsl_f32(select, v3wz, v3yx);

    vTemp1 = vcombine_f32(v2yw, v2xz);
    vTemp2 = vcombine_f32(v3wx, v3wx);
    float32x4_t vTerm = vmulq_f32(vTemp1, vTemp2);

    // - V2wxwx * V3ywxz
    const float32x2_t v2wx = vbsl_f32(select, v2wz, v2yx);

    const float32x2_t v3yw = vrev64_f32(v3wy);
    const float32x2_t v3xz = vbsl_f32(select, v3xy, v3wz);

    vTemp1 = vcombine_f32(v2wx, v2wx);
    vTemp2 = vcombine_f32(v3yw, v3xz);
    vTerm = vmlsq_f32(vTerm, vTemp1, vTemp2);

    // vResult - term2 * V1zzyy
    const float32x2_t v1zw = vget_high_f32(V1);

    vTemp1 = vcombine_f32(vdup_lane_f32(v1zw, 0), vdup_lane_f32(v1yx, 0));
    vResult = vmlsq_f32(vResult, vTerm, vTemp1);

    // Term3: V2yzxy * V3zxyx
    const float32x2_t v3zx = vrev64_f32(v3xz);

    vTemp1 = vcombine_f32(v2yz, v2xy);
    vTemp2 = vcombine_f32(v3zx, v3yx);
    vTerm = vmulq_f32(vTemp1, vTemp2);

    // - V2zxyx * V3yzxy
    const float32x2_t v2zx = vrev64_f32(v2xz);

    vTemp1 = vcombine_f32(v2zx, v2yx);
    vTemp2 = vcombine_f32(v3yz, v3xy);
    vTerm = vmlsq_f32(vTerm, vTemp1, vTemp2);

    // vResult + term3 * V1wwwz
    const float32x2_t v1wz = vrev64_f32(v1zw);

    vTemp1 = vcombine_f32(vdup_lane_f32(v1wz, 0), v1wz);
    return vmlaq_f32(vResult, vTerm, vTemp1);
#elif defined(_XM_SSE_INTRINSICS_)
    // V2zwyz * V3wzwy
    XMVECTOR vResult = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 1, 3, 2));
    XMVECTOR vTemp3 = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 3, 2, 3));
    vResult = _mm_mul_ps(vResult, vTemp3);
    // - V2wzwy * V3zwyz
    XMVECTOR vTemp2 = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 3, 2, 3));
    vTemp3 = XM_PERMUTE_PS(vTemp3, _MM_SHUFFLE(1, 3, 0, 1));
    vResult = XM_FNMADD_PS(vTemp2, vTemp3, vResult);
     // term1 * V1yxxx
    XMVECTOR vTemp1 = XM_PERMUTE_PS(V1, _MM_SHUFFLE(0, 0, 0, 1));
    vResult = _mm_mul_ps(vResult, vTemp1);

    // V2ywxz * V3wxwx
    vTemp2 = XM_PERMUTE_PS(V2, _MM_SHUFFLE(2, 0, 3, 1));
    vTemp3 = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 3, 0, 3));
    vTemp3 = _mm_mul_ps(vTemp3, vTemp2);
    // - V2wxwx * V3ywxz
    vTemp2 = XM_PERMUTE_PS(vTemp2, _MM_SHUFFLE(2, 1, 2, 1));
    vTemp1 = XM_PERMUTE_PS(V3, _MM_SHUFFLE(2, 0, 3, 1));
    vTemp3 = XM_FNMADD_PS(vTemp2, vTemp1, vTemp3);
    // vResult - temp * V1zzyy
    vTemp1 = XM_PERMUTE_PS(V1, _MM_SHUFFLE(1, 1, 2, 2));
    vResult = XM_FNMADD_PS(vTemp1, vTemp3, vResult);

    // V2yzxy * V3zxyx
    vTemp2 = XM_PERMUTE_PS(V2, _MM_SHUFFLE(1, 0, 2, 1));
    vTemp3 = XM_PERMUTE_PS(V3, _MM_SHUFFLE(0, 1, 0, 2));
    vTemp3 = _mm_mul_ps(vTemp3, vTemp2);
    // - V2zxyx * V3yzxy
    vTemp2 = XM_PERMUTE_PS(vTemp2, _MM_SHUFFLE(2, 0, 2, 1));
    vTemp1 = XM_PERMUTE_PS(V3, _MM_SHUFFLE(1, 0, 2, 1));
    vTemp3 = XM_FNMADD_PS(vTemp1, vTemp2, vTemp3);
    // vResult + term * V1wwwz
    vTemp1 = XM_PERMUTE_PS(V1, _MM_SHUFFLE(2, 3, 3, 3));
    vResult = XM_FMADD_PS(vTemp3, vTemp1, vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4LengthSq(FXMVECTOR V) noexcept
{
    return XMVector4Dot(V, V);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4ReciprocalLengthEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector4LengthSq(V);
    Result = XMVectorReciprocalSqrtEst(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot4
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    // Reciprocal sqrt (estimate)
    v2 = vrsqrte_f32(v1);
    return vcombine_f32(v2, v2);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0xff);
    return _mm_rsqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_rsqrt_ps(vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and w
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
    // x+z, y+w
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // x+z,x+z,x+z,y+w
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
    // ??,??,y+w,y+w
    vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
    // ??,??,x+z+y+w,??
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // Splat the length
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
    // Get the reciprocal
    vLengthSq = _mm_rsqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4ReciprocalLength(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector4LengthSq(V);
    Result = XMVectorReciprocalSqrt(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot4
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    // Reciprocal sqrt
    float32x2_t  S0 = vrsqrte_f32(v1);
    float32x2_t  P0 = vmul_f32(v1, S0);
    float32x2_t  R0 = vrsqrts_f32(P0, S0);
    float32x2_t  S1 = vmul_f32(S0, R0);
    float32x2_t  P1 = vmul_f32(v1, S1);
    float32x2_t  R1 = vrsqrts_f32(P1, S1);
    float32x2_t Result = vmul_f32(S1, R1);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0xff);
    XMVECTOR vLengthSq = _mm_sqrt_ps(vTemp);
    return _mm_div_ps(g_XMOne, vLengthSq);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    vLengthSq = _mm_div_ps(g_XMOne, vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and w
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
    // x+z, y+w
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // x+z,x+z,x+z,y+w
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
    // ??,??,y+w,y+w
    vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
    // ??,??,x+z+y+w,??
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // Splat the length
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
    // Get the reciprocal
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    // Accurate!
    vLengthSq = _mm_div_ps(g_XMOne, vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4LengthEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector4LengthSq(V);
    Result = XMVectorSqrtEst(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot4
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    const float32x2_t zero = vdup_n_f32(0);
    uint32x2_t VEqualsZero = vceq_f32(v1, zero);
    // Sqrt (estimate)
    float32x2_t Result = vrsqrte_f32(v1);
    Result = vmul_f32(v1, Result);
    Result = vbsl_f32(VEqualsZero, zero, Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0xff);
    return _mm_sqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and w
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
    // x+z, y+w
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // x+z,x+z,x+z,y+w
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
    // ??,??,y+w,y+w
    vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
    // ??,??,x+z+y+w,??
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // Splat the length
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
    // Get the length
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Length(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;

    Result = XMVector4LengthSq(V);
    Result = XMVectorSqrt(Result);

    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot4
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    const float32x2_t zero = vdup_n_f32(0);
    uint32x2_t VEqualsZero = vceq_f32(v1, zero);
    // Sqrt
    float32x2_t S0 = vrsqrte_f32(v1);
    float32x2_t P0 = vmul_f32(v1, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(v1, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    float32x2_t Result = vmul_f32(S1, R1);
    Result = vmul_f32(v1, Result);
    Result = vbsl_f32(VEqualsZero, zero, Result);
    return vcombine_f32(Result, Result);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0xff);
    return _mm_sqrt_ps(vTemp);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and w
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
    // x+z, y+w
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // x+z,x+z,x+z,y+w
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
    // ??,??,y+w,y+w
    vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
    // ??,??,x+z+y+w,??
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // Splat the length
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
    // Get the length
    vLengthSq = _mm_sqrt_ps(vLengthSq);
    return vLengthSq;
#endif
}

//------------------------------------------------------------------------------
// XMVector4NormalizeEst uses a reciprocal estimate and
// returns QNaN on zero and infinite vectors.

inline XMVECTOR XM_CALLCONV XMVector4NormalizeEst(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR Result;
    Result = XMVector4ReciprocalLength(V);
    Result = XMVectorMultiply(V, Result);
    return Result;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot4
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    // Reciprocal sqrt (estimate)
    v2 = vrsqrte_f32(v1);
    // Normalize
    return vmulq_f32(V, vcombine_f32(v2, v2));
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vTemp = _mm_dp_ps(V, V, 0xff);
    XMVECTOR vResult = _mm_rsqrt_ps(vTemp);
    return _mm_mul_ps(vResult, V);
#elif defined(_XM_SSE3_INTRINSICS_)
    XMVECTOR vDot = _mm_mul_ps(V, V);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_hadd_ps(vDot, vDot);
    vDot = _mm_rsqrt_ps(vDot);
    vDot = _mm_mul_ps(vDot, V);
    return vDot;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and w
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
    // x+z, y+w
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // x+z,x+z,x+z,y+w
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
    // ??,??,y+w,y+w
    vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
    // ??,??,x+z+y+w,??
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // Splat the length
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
    // Get the reciprocal
    XMVECTOR vResult = _mm_rsqrt_ps(vLengthSq);
    // Reciprocal mul to perform the normalization
    vResult = _mm_mul_ps(vResult, V);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Normalize(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)
    float fLength;
    XMVECTOR vResult;

    vResult = XMVector4Length(V);
    fLength = vResult.vector4_f32[0];

    // Prevent divide by zero
    if (fLength > 0)
    {
        fLength = 1.0f / fLength;
    }

    vResult.vector4_f32[0] = V.vector4_f32[0] * fLength;
    vResult.vector4_f32[1] = V.vector4_f32[1] * fLength;
    vResult.vector4_f32[2] = V.vector4_f32[2] * fLength;
    vResult.vector4_f32[3] = V.vector4_f32[3] * fLength;
    return vResult;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    // Dot4
    float32x4_t vTemp = vmulq_f32(V, V);
    float32x2_t v1 = vget_low_f32(vTemp);
    float32x2_t v2 = vget_high_f32(vTemp);
    v1 = vadd_f32(v1, v2);
    v1 = vpadd_f32(v1, v1);
    uint32x2_t VEqualsZero = vceq_f32(v1, vdup_n_f32(0));
    uint32x2_t VEqualsInf = vceq_f32(v1, vget_low_f32(g_XMInfinity));
    // Reciprocal sqrt (2 iterations of Newton-Raphson)
    float32x2_t S0 = vrsqrte_f32(v1);
    float32x2_t P0 = vmul_f32(v1, S0);
    float32x2_t R0 = vrsqrts_f32(P0, S0);
    float32x2_t S1 = vmul_f32(S0, R0);
    float32x2_t P1 = vmul_f32(v1, S1);
    float32x2_t R1 = vrsqrts_f32(P1, S1);
    v2 = vmul_f32(S1, R1);
    // Normalize
    XMVECTOR vResult = vmulq_f32(V, vcombine_f32(v2, v2));
    vResult = vbslq_f32(vcombine_u32(VEqualsZero, VEqualsZero), vdupq_n_f32(0), vResult);
    return vbslq_f32(vcombine_u32(VEqualsInf, VEqualsInf), g_XMQNaN, vResult);
#elif defined(_XM_SSE4_INTRINSICS_)
    XMVECTOR vLengthSq = _mm_dp_ps(V, V, 0xff);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Divide to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#elif defined(_XM_SSE3_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    vLengthSq = _mm_hadd_ps(vLengthSq, vLengthSq);
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Divide to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    // Perform the dot product on x,y,z and w
    XMVECTOR vLengthSq = _mm_mul_ps(V, V);
    // vTemp has z and w
    XMVECTOR vTemp = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(3, 2, 3, 2));
    // x+z, y+w
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // x+z,x+z,x+z,y+w
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(1, 0, 0, 0));
    // ??,??,y+w,y+w
    vTemp = _mm_shuffle_ps(vTemp, vLengthSq, _MM_SHUFFLE(3, 3, 0, 0));
    // ??,??,x+z+y+w,??
    vLengthSq = _mm_add_ps(vLengthSq, vTemp);
    // Splat the length
    vLengthSq = XM_PERMUTE_PS(vLengthSq, _MM_SHUFFLE(2, 2, 2, 2));
    // Prepare for the division
    XMVECTOR vResult = _mm_sqrt_ps(vLengthSq);
    // Create zero with a single instruction
    XMVECTOR vZeroMask = _mm_setzero_ps();
    // Test for a divide by zero (Must be FP to detect -0.0)
    vZeroMask = _mm_cmpneq_ps(vZeroMask, vResult);
    // Failsafe on zero (Or epsilon) length planes
    // If the length is infinity, set the elements to zero
    vLengthSq = _mm_cmpneq_ps(vLengthSq, g_XMInfinity);
    // Divide to perform the normalization
    vResult = _mm_div_ps(V, vResult);
    // Any that are infinity, set to zero
    vResult = _mm_and_ps(vResult, vZeroMask);
    // Select qnan or result based on infinite length
    XMVECTOR vTemp1 = _mm_andnot_ps(vLengthSq, g_XMQNaN);
    XMVECTOR vTemp2 = _mm_and_ps(vResult, vLengthSq);
    vResult = _mm_or_ps(vTemp1, vTemp2);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4ClampLength
(
    FXMVECTOR V,
    float    LengthMin,
    float    LengthMax
) noexcept
{
    XMVECTOR ClampMax = XMVectorReplicate(LengthMax);
    XMVECTOR ClampMin = XMVectorReplicate(LengthMin);

    return XMVector4ClampLengthV(V, ClampMin, ClampMax);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4ClampLengthV
(
    FXMVECTOR V,
    FXMVECTOR LengthMin,
    FXMVECTOR LengthMax
) noexcept
{
    assert((XMVectorGetY(LengthMin) == XMVectorGetX(LengthMin)) && (XMVectorGetZ(LengthMin) == XMVectorGetX(LengthMin)) && (XMVectorGetW(LengthMin) == XMVectorGetX(LengthMin)));
    assert((XMVectorGetY(LengthMax) == XMVectorGetX(LengthMax)) && (XMVectorGetZ(LengthMax) == XMVectorGetX(LengthMax)) && (XMVectorGetW(LengthMax) == XMVectorGetX(LengthMax)));
    assert(XMVector4GreaterOrEqual(LengthMin, XMVectorZero()));
    assert(XMVector4GreaterOrEqual(LengthMax, XMVectorZero()));
    assert(XMVector4GreaterOrEqual(LengthMax, LengthMin));

    XMVECTOR LengthSq = XMVector4LengthSq(V);

    const XMVECTOR Zero = XMVectorZero();

    XMVECTOR RcpLength = XMVectorReciprocalSqrt(LengthSq);

    XMVECTOR InfiniteLength = XMVectorEqualInt(LengthSq, g_XMInfinity.v);
    XMVECTOR ZeroLength = XMVectorEqual(LengthSq, Zero);

    XMVECTOR Normal = XMVectorMultiply(V, RcpLength);

    XMVECTOR Length = XMVectorMultiply(LengthSq, RcpLength);

    XMVECTOR Select = XMVectorEqualInt(InfiniteLength, ZeroLength);
    Length = XMVectorSelect(LengthSq, Length, Select);
    Normal = XMVectorSelect(LengthSq, Normal, Select);

    XMVECTOR ControlMax = XMVectorGreater(Length, LengthMax);
    XMVECTOR ControlMin = XMVectorLess(Length, LengthMin);

    XMVECTOR ClampLength = XMVectorSelect(Length, LengthMax, ControlMax);
    ClampLength = XMVectorSelect(ClampLength, LengthMin, ControlMin);

    XMVECTOR Result = XMVectorMultiply(Normal, ClampLength);

    // Preserve the original vector (with no precision loss) if the length falls within the given range
    XMVECTOR Control = XMVectorEqualInt(ControlMax, ControlMin);
    Result = XMVectorSelect(Result, V, Control);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Reflect
(
    FXMVECTOR Incident,
    FXMVECTOR Normal
) noexcept
{
    // Result = Incident - (2 * dot(Incident, Normal)) * Normal

    XMVECTOR Result = XMVector4Dot(Incident, Normal);
    Result = XMVectorAdd(Result, Result);
    Result = XMVectorNegativeMultiplySubtract(Result, Normal, Incident);

    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Refract
(
    FXMVECTOR Incident,
    FXMVECTOR Normal,
    float    RefractionIndex
) noexcept
{
    XMVECTOR Index = XMVectorReplicate(RefractionIndex);
    return XMVector4RefractV(Incident, Normal, Index);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4RefractV
(
    FXMVECTOR Incident,
    FXMVECTOR Normal,
    FXMVECTOR RefractionIndex
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR        IDotN;
    XMVECTOR        R;
    const XMVECTOR  Zero = XMVectorZero();

    // Result = RefractionIndex * Incident - Normal * (RefractionIndex * dot(Incident, Normal) +
    // sqrt(1 - RefractionIndex * RefractionIndex * (1 - dot(Incident, Normal) * dot(Incident, Normal))))

    IDotN = XMVector4Dot(Incident, Normal);

    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    R = XMVectorNegativeMultiplySubtract(IDotN, IDotN, g_XMOne.v);
    R = XMVectorMultiply(R, RefractionIndex);
    R = XMVectorNegativeMultiplySubtract(R, RefractionIndex, g_XMOne.v);

    if (XMVector4LessOrEqual(R, Zero))
    {
        // Total internal reflection
        return Zero;
    }
    else
    {
        XMVECTOR Result;

        // R = RefractionIndex * IDotN + sqrt(R)
        R = XMVectorSqrt(R);
        R = XMVectorMultiplyAdd(RefractionIndex, IDotN, R);

        // Result = RefractionIndex * Incident - Normal * R
        Result = XMVectorMultiply(RefractionIndex, Incident);
        Result = XMVectorNegativeMultiplySubtract(Normal, R, Result);

        return Result;
    }

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    XMVECTOR IDotN = XMVector4Dot(Incident, Normal);

    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    float32x4_t R = vmlsq_f32(g_XMOne, IDotN, IDotN);
    R = vmulq_f32(R, RefractionIndex);
    R = vmlsq_f32(g_XMOne, R, RefractionIndex);

    uint32x4_t isrzero = vcleq_f32(R, g_XMZero);
    uint8x8x2_t vTemp = vzip_u8(vget_low_u8(vreinterpretq_u8_u32(isrzero)), vget_high_u8(vreinterpretq_u8_u32(isrzero)));
    uint16x4x2_t vTemp2 = vzip_u16(vreinterpret_u16_u8(vTemp.val[0]), vreinterpret_u16_u8(vTemp.val[1]));

    float32x4_t vResult;
    if (vget_lane_u32(vreinterpret_u32_u16(vTemp2.val[1]), 1) == 0xFFFFFFFFU)
    {
        // Total internal reflection
        vResult = g_XMZero;
    }
    else
    {
        // Sqrt(R)
        float32x4_t S0 = vrsqrteq_f32(R);
        float32x4_t P0 = vmulq_f32(R, S0);
        float32x4_t R0 = vrsqrtsq_f32(P0, S0);
        float32x4_t S1 = vmulq_f32(S0, R0);
        float32x4_t P1 = vmulq_f32(R, S1);
        float32x4_t R1 = vrsqrtsq_f32(P1, S1);
        float32x4_t S2 = vmulq_f32(S1, R1);
        R = vmulq_f32(R, S2);
        // R = RefractionIndex * IDotN + sqrt(R)
        R = vmlaq_f32(R, RefractionIndex, IDotN);
        // Result = RefractionIndex * Incident - Normal * R
        vResult = vmulq_f32(RefractionIndex, Incident);
        vResult = vmlsq_f32(vResult, R, Normal);
    }
    return vResult;
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR IDotN = XMVector4Dot(Incident, Normal);

    // R = 1.0f - RefractionIndex * RefractionIndex * (1.0f - IDotN * IDotN)
    XMVECTOR R = XM_FNMADD_PS(IDotN, IDotN, g_XMOne);
    XMVECTOR R2 = _mm_mul_ps(RefractionIndex, RefractionIndex);
    R = XM_FNMADD_PS(R, R2, g_XMOne);

    XMVECTOR vResult = _mm_cmple_ps(R, g_XMZero);
    if (_mm_movemask_ps(vResult) == 0x0f)
    {
        // Total internal reflection
        vResult = g_XMZero;
    }
    else
    {
        // R = RefractionIndex * IDotN + sqrt(R)
        R = _mm_sqrt_ps(R);
        R = XM_FMADD_PS(RefractionIndex, IDotN, R);
        // Result = RefractionIndex * Incident - Normal * R
        vResult = _mm_mul_ps(RefractionIndex, Incident);
        vResult = XM_FNMADD_PS(R, Normal, vResult);
    }
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Orthogonal(FXMVECTOR V) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    XMVECTORF32 Result = { { {
            V.vector4_f32[2],
            V.vector4_f32[3],
            -V.vector4_f32[0],
            -V.vector4_f32[1]
        } } };
    return Result.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Negate = { { { 1.f, 1.f, -1.f, -1.f } } };

    float32x4_t Result = vcombine_f32(vget_high_f32(V), vget_low_f32(V));
    return vmulq_f32(Result, Negate);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 FlipZW = { { { 1.0f, 1.0f, -1.0f, -1.0f } } };
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 0, 3, 2));
    vResult = _mm_mul_ps(vResult, FlipZW);
    return vResult;
#endif
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4AngleBetweenNormalsEst
(
    FXMVECTOR N1,
    FXMVECTOR N2
) noexcept
{
    XMVECTOR Result = XMVector4Dot(N1, N2);
    Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
    Result = XMVectorACosEst(Result);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4AngleBetweenNormals
(
    FXMVECTOR N1,
    FXMVECTOR N2
) noexcept
{
    XMVECTOR Result = XMVector4Dot(N1, N2);
    Result = XMVectorClamp(Result, g_XMNegativeOne.v, g_XMOne.v);
    Result = XMVectorACos(Result);
    return Result;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4AngleBetweenVectors
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    XMVECTOR L1 = XMVector4ReciprocalLength(V1);
    XMVECTOR L2 = XMVector4ReciprocalLength(V2);

    XMVECTOR Dot = XMVector4Dot(V1, V2);

    L1 = XMVectorMultiply(L1, L2);

    XMVECTOR CosAngle = XMVectorMultiply(Dot, L1);
    CosAngle = XMVectorClamp(CosAngle, g_XMNegativeOne.v, g_XMOne.v);

    return XMVectorACos(CosAngle);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV XMVector4Transform
(
    FXMVECTOR V,
    FXMMATRIX M
) noexcept
{
#if defined(_XM_NO_INTRINSICS_)

    float fX = (M.m[0][0] * V.vector4_f32[0]) + (M.m[1][0] * V.vector4_f32[1]) + (M.m[2][0] * V.vector4_f32[2]) + (M.m[3][0] * V.vector4_f32[3]);
    float fY = (M.m[0][1] * V.vector4_f32[0]) + (M.m[1][1] * V.vector4_f32[1]) + (M.m[2][1] * V.vector4_f32[2]) + (M.m[3][1] * V.vector4_f32[3]);
    float fZ = (M.m[0][2] * V.vector4_f32[0]) + (M.m[1][2] * V.vector4_f32[1]) + (M.m[2][2] * V.vector4_f32[2]) + (M.m[3][2] * V.vector4_f32[3]);
    float fW = (M.m[0][3] * V.vector4_f32[0]) + (M.m[1][3] * V.vector4_f32[1]) + (M.m[2][3] * V.vector4_f32[2]) + (M.m[3][3] * V.vector4_f32[3]);
    XMVECTORF32 vResult = { { { fX, fY, fZ, fW } } };
    return vResult.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x2_t VL = vget_low_f32(V);
    XMVECTOR vResult = vmulq_lane_f32(M.r[0], VL, 0); // X
    vResult = vmlaq_lane_f32(vResult, M.r[1], VL, 1); // Y
    float32x2_t VH = vget_high_f32(V);
    vResult = vmlaq_lane_f32(vResult, M.r[2], VH, 0); // Z
    return vmlaq_lane_f32(vResult, M.r[3], VH, 1); // W
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3)); // W
    vResult = _mm_mul_ps(vResult, M.r[3]);
    XMVECTOR vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2)); // Z
    vResult = XM_FMADD_PS(vTemp, M.r[2], vResult);
    vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1)); // Y
    vResult = XM_FMADD_PS(vTemp, M.r[1], vResult);
    vTemp = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0)); // X
    vResult = XM_FMADD_PS(vTemp, M.r[0], vResult);
    return vResult;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMFLOAT4* XM_CALLCONV XMVector4TransformStream
(
    XMFLOAT4* pOutputStream,
    size_t          OutputStride,
    const XMFLOAT4* pInputStream,
    size_t          InputStride,
    size_t          VectorCount,
    FXMMATRIX       M
) noexcept
{
    assert(pOutputStream != nullptr);
    assert(pInputStream != nullptr);

    assert(InputStride >= sizeof(XMFLOAT4));
    _Analysis_assume_(InputStride >= sizeof(XMFLOAT4));

    assert(OutputStride >= sizeof(XMFLOAT4));
    _Analysis_assume_(OutputStride >= sizeof(XMFLOAT4));

#if defined(_XM_NO_INTRINSICS_)

    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    for (size_t i = 0; i < VectorCount; i++)
    {
        XMVECTOR V = XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pInputVector));
        XMVECTOR W = XMVectorSplatW(V);
        XMVECTOR Z = XMVectorSplatZ(V);
        XMVECTOR Y = XMVectorSplatY(V);
        XMVECTOR X = XMVectorSplatX(V);

        XMVECTOR Result = XMVectorMultiply(W, row3);
        Result = XMVectorMultiplyAdd(Z, row2, Result);
        Result = XMVectorMultiplyAdd(Y, row1, Result);
        Result = XMVectorMultiplyAdd(X, row0, Result);

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015, "PREfast noise: Esp:1307" )
#endif

        XMStoreFloat4(reinterpret_cast<XMFLOAT4*>(pOutputVector), Result);

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

        pInputVector += InputStride;
        pOutputVector += OutputStride;
    }

    return pOutputStream;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    size_t i = 0;
    size_t four = VectorCount >> 2;
    if (four > 0)
    {
        if ((InputStride == sizeof(XMFLOAT4)) && (OutputStride == sizeof(XMFLOAT4)))
        {
            for (size_t j = 0; j < four; ++j)
            {
                float32x4x4_t V = vld4q_f32(reinterpret_cast<const float*>(pInputVector));
                pInputVector += sizeof(XMFLOAT4) * 4;

                float32x2_t r = vget_low_f32(row0);
                XMVECTOR vResult0 = vmulq_lane_f32(V.val[0], r, 0); // Ax
                XMVECTOR vResult1 = vmulq_lane_f32(V.val[0], r, 1); // Bx

                XM_PREFETCH(pInputVector);

                r = vget_high_f32(row0);
                XMVECTOR vResult2 = vmulq_lane_f32(V.val[0], r, 0); // Cx
                XMVECTOR vResult3 = vmulq_lane_f32(V.val[0], r, 1); // Dx

                XM_PREFETCH(pInputVector + XM_CACHE_LINE_SIZE);

                r = vget_low_f32(row1);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[1], r, 0); // Ax+Ey
                vResult1 = vmlaq_lane_f32(vResult1, V.val[1], r, 1); // Bx+Fy

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 2));

                r = vget_high_f32(row1);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[1], r, 0); // Cx+Gy
                vResult3 = vmlaq_lane_f32(vResult3, V.val[1], r, 1); // Dx+Hy

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 3));

                r = vget_low_f32(row2);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[2], r, 0); // Ax+Ey+Iz
                vResult1 = vmlaq_lane_f32(vResult1, V.val[2], r, 1); // Bx+Fy+Jz

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 4));

                r = vget_high_f32(row2);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[2], r, 0); // Cx+Gy+Kz
                vResult3 = vmlaq_lane_f32(vResult3, V.val[2], r, 1); // Dx+Hy+Lz

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 5));

                r = vget_low_f32(row3);
                vResult0 = vmlaq_lane_f32(vResult0, V.val[3], r, 0); // Ax+Ey+Iz+Mw
                vResult1 = vmlaq_lane_f32(vResult1, V.val[3], r, 1); // Bx+Fy+Jz+Nw

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 6));

                r = vget_high_f32(row3);
                vResult2 = vmlaq_lane_f32(vResult2, V.val[3], r, 0); // Cx+Gy+Kz+Ow
                vResult3 = vmlaq_lane_f32(vResult3, V.val[3], r, 1); // Dx+Hy+Lz+Pw

                XM_PREFETCH(pInputVector + (XM_CACHE_LINE_SIZE * 7));

                V.val[0] = vResult0;
                V.val[1] = vResult1;
                V.val[2] = vResult2;
                V.val[3] = vResult3;

                vst4q_f32(reinterpret_cast<float*>(pOutputVector), V);
                pOutputVector += sizeof(XMFLOAT4) * 4;

                i += 4;
            }
        }
    }

    for (; i < VectorCount; i++)
    {
        XMVECTOR V = vld1q_f32(reinterpret_cast<const float*>(pInputVector));
        pInputVector += InputStride;

        float32x2_t VL = vget_low_f32(V);
        XMVECTOR vResult = vmulq_lane_f32(row0, VL, 0); // X
        vResult = vmlaq_lane_f32(vResult, row1, VL, 1); // Y
        float32x2_t VH = vget_high_f32(V);
        vResult = vmlaq_lane_f32(vResult, row2, VH, 0); // Z
        vResult = vmlaq_lane_f32(vResult, row3, VH, 1); // W

        vst1q_f32(reinterpret_cast<float*>(pOutputVector), vResult);
        pOutputVector += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_AVX2_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t two = VectorCount >> 1;
    if (two > 0)
    {
        __m256 row0 = _mm256_broadcast_ps(&M.r[0]);
        __m256 row1 = _mm256_broadcast_ps(&M.r[1]);
        __m256 row2 = _mm256_broadcast_ps(&M.r[2]);
        __m256 row3 = _mm256_broadcast_ps(&M.r[3]);

        if (InputStride == sizeof(XMFLOAT4))
        {
            if (OutputStride == sizeof(XMFLOAT4))
            {
                if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0x1F))
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < two; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT4) * 2;

                        __m256 vTempX = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));
                        __m256 vTempY = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 vTempZ = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 vTempW = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));

                        vTempX = _mm256_mul_ps(vTempX, row0);
                        vTempY = _mm256_mul_ps(vTempY, row1);
                        vTempZ = _mm256_fmadd_ps(vTempZ, row2, vTempX);
                        vTempW = _mm256_fmadd_ps(vTempW, row3, vTempY);
                        vTempX = _mm256_add_ps(vTempZ, vTempW);

                        XM256_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTempX);
                        pOutputVector += sizeof(XMFLOAT4) * 2;

                        i += 2;
                    }
                }
                else
                {
                    // Packed input, packed output
                    for (size_t j = 0; j < two; ++j)
                    {
                        __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                        pInputVector += sizeof(XMFLOAT4) * 2;

                        __m256 vTempX = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));
                        __m256 vTempY = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                        __m256 vTempZ = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                        __m256 vTempW = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));

                        vTempX = _mm256_mul_ps(vTempX, row0);
                        vTempY = _mm256_mul_ps(vTempY, row1);
                        vTempZ = _mm256_fmadd_ps(vTempZ, row2, vTempX);
                        vTempW = _mm256_fmadd_ps(vTempW, row3, vTempY);
                        vTempX = _mm256_add_ps(vTempZ, vTempW);

                        _mm256_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTempX);
                        pOutputVector += sizeof(XMFLOAT4) * 2;

                        i += 2;
                    }
                }
            }
            else
            {
                // Packed input, unpacked output
                for (size_t j = 0; j < two; ++j)
                {
                    __m256 VV = _mm256_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                    pInputVector += sizeof(XMFLOAT4) * 2;

                    __m256 vTempX = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(0, 0, 0, 0));
                    __m256 vTempY = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(1, 1, 1, 1));
                    __m256 vTempZ = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(2, 2, 2, 2));
                    __m256 vTempW = _mm256_shuffle_ps(VV, VV, _MM_SHUFFLE(3, 3, 3, 3));

                    vTempX = _mm256_mul_ps(vTempX, row0);
                    vTempY = _mm256_mul_ps(vTempY, row1);
                    vTempZ = _mm256_fmadd_ps(vTempZ, row2, vTempX);
                    vTempW = _mm256_fmadd_ps(vTempW, row3, vTempY);
                    vTempX = _mm256_add_ps(vTempZ, vTempW);

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), _mm256_castps256_ps128(vTempX));
                    pOutputVector += OutputStride;

                    _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), _mm256_extractf128_ps(vTempX, 1));
                    pOutputVector += OutputStride;
                    i += 2;
                }
            }
        }
    }

    if (i < VectorCount)
    {
        const XMVECTOR row0 = M.r[0];
        const XMVECTOR row1 = M.r[1];
        const XMVECTOR row2 = M.r[2];
        const XMVECTOR row3 = M.r[3];

        for (; i < VectorCount; i++)
        {
            __m128 V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
            pInputVector += InputStride;

            XMVECTOR vTempX = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));
            XMVECTOR vTempY = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
            XMVECTOR vTempZ = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
            XMVECTOR vTempW = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));

            vTempX = _mm_mul_ps(vTempX, row0);
            vTempY = _mm_mul_ps(vTempY, row1);
            vTempZ = XM_FMADD_PS(vTempZ, row2, vTempX);
            vTempW = XM_FMADD_PS(vTempW, row3, vTempY);
            vTempX = _mm_add_ps(vTempZ, vTempW);

            _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTempX);
            pOutputVector += OutputStride;
        }
    }

    XM_SFENCE();

    return pOutputStream;
#elif defined(_XM_SSE_INTRINSICS_)
    auto pInputVector = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pOutputVector = reinterpret_cast<uint8_t*>(pOutputStream);

    const XMVECTOR row0 = M.r[0];
    const XMVECTOR row1 = M.r[1];
    const XMVECTOR row2 = M.r[2];
    const XMVECTOR row3 = M.r[3];

    if (!(reinterpret_cast<uintptr_t>(pOutputStream) & 0xF) && !(OutputStride & 0xF))
    {
        if (!(reinterpret_cast<uintptr_t>(pInputStream) & 0xF) && !(InputStride & 0xF))
        {
            // Aligned input, aligned output
            for (size_t i = 0; i < VectorCount; i++)
            {
                __m128 V = _mm_load_ps(reinterpret_cast<const float*>(pInputVector));
                pInputVector += InputStride;

                XMVECTOR vTempX = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));
                XMVECTOR vTempY = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                XMVECTOR vTempZ = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
                XMVECTOR vTempW = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));

                vTempX = _mm_mul_ps(vTempX, row0);
                vTempY = _mm_mul_ps(vTempY, row1);
                vTempZ = XM_FMADD_PS(vTempZ, row2, vTempX);
                vTempW = XM_FMADD_PS(vTempW, row3, vTempY);
                vTempX = _mm_add_ps(vTempZ, vTempW);

                XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTempX);
                pOutputVector += OutputStride;
            }
        }
        else
        {
            // Unaligned input, aligned output
            for (size_t i = 0; i < VectorCount; i++)
            {
                __m128 V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                pInputVector += InputStride;

                XMVECTOR vTempX = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));
                XMVECTOR vTempY = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                XMVECTOR vTempZ = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
                XMVECTOR vTempW = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));

                vTempX = _mm_mul_ps(vTempX, row0);
                vTempY = _mm_mul_ps(vTempY, row1);
                vTempZ = XM_FMADD_PS(vTempZ, row2, vTempX);
                vTempW = XM_FMADD_PS(vTempW, row3, vTempY);
                vTempX = _mm_add_ps(vTempZ, vTempW);

                XM_STREAM_PS(reinterpret_cast<float*>(pOutputVector), vTempX);
                pOutputVector += OutputStride;
            }
        }
    }
    else
    {
        if (!(reinterpret_cast<uintptr_t>(pInputStream) & 0xF) && !(InputStride & 0xF))
        {
            // Aligned input, unaligned output
            for (size_t i = 0; i < VectorCount; i++)
            {
                __m128 V = _mm_load_ps(reinterpret_cast<const float*>(pInputVector));
                pInputVector += InputStride;

                XMVECTOR vTempX = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));
                XMVECTOR vTempY = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                XMVECTOR vTempZ = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
                XMVECTOR vTempW = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));

                vTempX = _mm_mul_ps(vTempX, row0);
                vTempY = _mm_mul_ps(vTempY, row1);
                vTempZ = XM_FMADD_PS(vTempZ, row2, vTempX);
                vTempW = XM_FMADD_PS(vTempW, row3, vTempY);
                vTempX = _mm_add_ps(vTempZ, vTempW);

                _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTempX);
                pOutputVector += OutputStride;
            }
        }
        else
        {
            // Unaligned input, unaligned output
            for (size_t i = 0; i < VectorCount; i++)
            {
                __m128 V = _mm_loadu_ps(reinterpret_cast<const float*>(pInputVector));
                pInputVector += InputStride;

                XMVECTOR vTempX = XM_PERMUTE_PS(V, _MM_SHUFFLE(0, 0, 0, 0));
                XMVECTOR vTempY = XM_PERMUTE_PS(V, _MM_SHUFFLE(1, 1, 1, 1));
                XMVECTOR vTempZ = XM_PERMUTE_PS(V, _MM_SHUFFLE(2, 2, 2, 2));
                XMVECTOR vTempW = XM_PERMUTE_PS(V, _MM_SHUFFLE(3, 3, 3, 3));

                vTempX = _mm_mul_ps(vTempX, row0);
                vTempY = _mm_mul_ps(vTempY, row1);
                vTempZ = XM_FMADD_PS(vTempZ, row2, vTempX);
                vTempW = XM_FMADD_PS(vTempW, row3, vTempY);
                vTempX = _mm_add_ps(vTempZ, vTempW);

                _mm_storeu_ps(reinterpret_cast<float*>(pOutputVector), vTempX);
                pOutputVector += OutputStride;
            }
        }
    }

    XM_SFENCE();

    return pOutputStream;
#endif
}

/****************************************************************************
 *
 * XMVECTOR operators
 *
 ****************************************************************************/

#ifndef _XM_NO_XMVECTOR_OVERLOADS_

 //------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator+ (FXMVECTOR V) noexcept
{
    return V;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator- (FXMVECTOR V) noexcept
{
    return XMVectorNegate(V);
}

//------------------------------------------------------------------------------

inline XMVECTOR& XM_CALLCONV operator+=
(
    XMVECTOR& V1,
    FXMVECTOR       V2
) noexcept
{
    V1 = XMVectorAdd(V1, V2);
    return V1;
}

//------------------------------------------------------------------------------

inline XMVECTOR& XM_CALLCONV operator-=
(
    XMVECTOR& V1,
    FXMVECTOR       V2
) noexcept
{
    V1 = XMVectorSubtract(V1, V2);
    return V1;
}

//------------------------------------------------------------------------------

inline XMVECTOR& XM_CALLCONV operator*=
(
    XMVECTOR& V1,
    FXMVECTOR       V2
) noexcept
{
    V1 = XMVectorMultiply(V1, V2);
    return V1;
}

//------------------------------------------------------------------------------

inline XMVECTOR& XM_CALLCONV operator/=
(
    XMVECTOR& V1,
    FXMVECTOR       V2
) noexcept
{
    V1 = XMVectorDivide(V1, V2);
    return V1;
}

//------------------------------------------------------------------------------

inline XMVECTOR& operator*=
(
    XMVECTOR& V,
    const float S
) noexcept
{
    V = XMVectorScale(V, S);
    return V;
}

//------------------------------------------------------------------------------

inline XMVECTOR& operator/=
(
    XMVECTOR& V,
    const float S
) noexcept
{
    XMVECTOR vS = XMVectorReplicate(S);
    V = XMVectorDivide(V, vS);
    return V;
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator+
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    return XMVectorAdd(V1, V2);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator-
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    return XMVectorSubtract(V1, V2);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator*
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    return XMVectorMultiply(V1, V2);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator/
(
    FXMVECTOR V1,
    FXMVECTOR V2
) noexcept
{
    return XMVectorDivide(V1, V2);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator*
(
    FXMVECTOR      V,
    const float    S
) noexcept
{
    return XMVectorScale(V, S);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator/
(
    FXMVECTOR      V,
    const float    S
) noexcept
{
    XMVECTOR vS = XMVectorReplicate(S);
    return XMVectorDivide(V, vS);
}

//------------------------------------------------------------------------------

inline XMVECTOR XM_CALLCONV operator*
(
    float           S,
    FXMVECTOR       V
) noexcept
{
    return XMVectorScale(V, S);
}

#endif /* !_XM_NO_XMVECTOR_OVERLOADS_ */

#if defined(_XM_NO_INTRINSICS_)
#undef XMISNAN
#undef XMISINF
#endif

#if defined(_XM_SSE_INTRINSICS_)
#undef XM3UNPACK3INTO4
#undef XM3PACK4INTO3
#endif

