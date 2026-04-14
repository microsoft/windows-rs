//-------------------------------------------------------------------------------------
// DirectXPackedVector.inl -- SIMD C++ Math library
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
//
// http://go.microsoft.com/fwlink/?LinkID=615560
//-------------------------------------------------------------------------------------

#pragma once

/****************************************************************************
 *
 * Data conversion
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline float XMConvertHalfToFloat(HALF Value) noexcept
{
#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    __m128i V1 = _mm_cvtsi32_si128(static_cast<int>(Value));
    __m128 V2 = _mm_cvtph_ps(V1);
    return _mm_cvtss_f32(V2);
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__) && !defined(_XM_NO_INTRINSICS_) && (!defined(__GNUC__) || (__ARM_FP & 2))
    uint16x4_t vHalf = vdup_n_u16(Value);
    float32x4_t vFloat = vcvt_f32_f16(vreinterpret_f16_u16(vHalf));
    return vgetq_lane_f32(vFloat, 0);
#else
    auto Mantissa = static_cast<uint32_t>(Value & 0x03FF);

    uint32_t Exponent = (Value & 0x7C00);
    if (Exponent == 0x7C00) // INF/NAN
    {
        Exponent = 0x8f;
    }
    else if (Exponent != 0)  // The value is normalized
    {
        Exponent = static_cast<uint32_t>((static_cast<int>(Value) >> 10) & 0x1F);
    }
    else if (Mantissa != 0)     // The value is denormalized
    {
        // Normalize the value in the resulting float
        Exponent = 1;

        do
        {
            Exponent--;
            Mantissa <<= 1;
        } while ((Mantissa & 0x0400) == 0);

        Mantissa &= 0x03FF;
    }
    else                        // The value is zero
    {
        Exponent = static_cast<uint32_t>(-112);
    }

    uint32_t Result =
        ((static_cast<uint32_t>(Value) & 0x8000) << 16) // Sign
        | ((Exponent + 112) << 23)                      // Exponent
        | (Mantissa << 13);                             // Mantissa

    return reinterpret_cast<float*>(&Result)[0];
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------
#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable : 26015 26019, "PREfast noise: Esp:1307" )
#endif

_Use_decl_annotations_
inline float* XMConvertHalfToFloatStream
(
    float* pOutputStream,
    size_t      OutputStride,
    const HALF* pInputStream,
    size_t      InputStride,
    size_t      HalfCount
) noexcept
{
    assert(pOutputStream);
    assert(pInputStream);

    assert(InputStride >= sizeof(HALF));
    _Analysis_assume_(InputStride >= sizeof(HALF));

    assert(OutputStride >= sizeof(float));
    _Analysis_assume_(OutputStride >= sizeof(float));

#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    auto pHalf = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pFloat = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = HalfCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(HALF))
        {
            if (OutputStride == sizeof(float))
            {
                if ((reinterpret_cast<uintptr_t>(pFloat) & 0xF) == 0)
                {
                    // Packed input, aligned & packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128i HV = _mm_loadl_epi64(reinterpret_cast<const __m128i*>(pHalf));
                        pHalf += InputStride * 4;

                        __m128 FV = _mm_cvtph_ps(HV);

                        XM_STREAM_PS(reinterpret_cast<float*>(pFloat), FV);
                        pFloat += OutputStride * 4;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128i HV = _mm_loadl_epi64(reinterpret_cast<const __m128i*>(pHalf));
                        pHalf += InputStride * 4;

                        __m128 FV = _mm_cvtph_ps(HV);

                        _mm_storeu_ps(reinterpret_cast<float*>(pFloat), FV);
                        pFloat += OutputStride * 4;
                        i += 4;
                    }
                }
            }
            else
            {
                // Packed input, scattered output
                for (size_t j = 0; j < four; ++j)
                {
                    __m128i HV = _mm_loadl_epi64(reinterpret_cast<const __m128i*>(pHalf));
                    pHalf += InputStride * 4;

                    __m128 FV = _mm_cvtph_ps(HV);

                    _mm_store_ss(reinterpret_cast<float*>(pFloat), FV);
                    pFloat += OutputStride;
                    *reinterpret_cast<int*>(pFloat) = _mm_extract_ps(FV, 1);
                    pFloat += OutputStride;
                    *reinterpret_cast<int*>(pFloat) = _mm_extract_ps(FV, 2);
                    pFloat += OutputStride;
                    *reinterpret_cast<int*>(pFloat) = _mm_extract_ps(FV, 3);
                    pFloat += OutputStride;
                    i += 4;
                }
            }
        }
        else if (OutputStride == sizeof(float))
        {
            if ((reinterpret_cast<uintptr_t>(pFloat) & 0xF) == 0)
            {
                // Scattered input, aligned & packed output
                for (size_t j = 0; j < four; ++j)
                {
                    uint16_t H1 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;
                    uint16_t H2 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;
                    uint16_t H3 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;
                    uint16_t H4 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;

                    __m128i HV = _mm_setzero_si128();
                    HV = _mm_insert_epi16(HV, H1, 0);
                    HV = _mm_insert_epi16(HV, H2, 1);
                    HV = _mm_insert_epi16(HV, H3, 2);
                    HV = _mm_insert_epi16(HV, H4, 3);
                    __m128 FV = _mm_cvtph_ps(HV);

                    XM_STREAM_PS(reinterpret_cast<float*>(pFloat), FV);
                    pFloat += OutputStride * 4;
                    i += 4;
                }
            }
            else
            {
                // Scattered input, packed output
                for (size_t j = 0; j < four; ++j)
                {
                    uint16_t H1 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;
                    uint16_t H2 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;
                    uint16_t H3 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;
                    uint16_t H4 = *reinterpret_cast<const HALF*>(pHalf);
                    pHalf += InputStride;

                    __m128i HV = _mm_setzero_si128();
                    HV = _mm_insert_epi16(HV, H1, 0);
                    HV = _mm_insert_epi16(HV, H2, 1);
                    HV = _mm_insert_epi16(HV, H3, 2);
                    HV = _mm_insert_epi16(HV, H4, 3);
                    __m128 FV = _mm_cvtph_ps(HV);

                    _mm_storeu_ps(reinterpret_cast<float*>(pFloat), FV);
                    pFloat += OutputStride * 4;
                    i += 4;
                }
            }
        }
        else
        {
            // Scattered input, scattered output
            for (size_t j = 0; j < four; ++j)
            {
                uint16_t H1 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H2 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H3 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H4 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;

                __m128i HV = _mm_setzero_si128();
                HV = _mm_insert_epi16(HV, H1, 0);
                HV = _mm_insert_epi16(HV, H2, 1);
                HV = _mm_insert_epi16(HV, H3, 2);
                HV = _mm_insert_epi16(HV, H4, 3);
                __m128 FV = _mm_cvtph_ps(HV);

                _mm_store_ss(reinterpret_cast<float*>(pFloat), FV);
                pFloat += OutputStride;
                *reinterpret_cast<int*>(pFloat) = _mm_extract_ps(FV, 1);
                pFloat += OutputStride;
                *reinterpret_cast<int*>(pFloat) = _mm_extract_ps(FV, 2);
                pFloat += OutputStride;
                *reinterpret_cast<int*>(pFloat) = _mm_extract_ps(FV, 3);
                pFloat += OutputStride;
                i += 4;
            }
        }
    }

    for (; i < HalfCount; ++i)
    {
        *reinterpret_cast<float*>(pFloat) = XMConvertHalfToFloat(reinterpret_cast<const HALF*>(pHalf)[0]);
        pHalf += InputStride;
        pFloat += OutputStride;
    }

    XM_SFENCE();

    return pOutputStream;
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) ||__aarch64__) && !defined(_XM_NO_INTRINSICS_) && (!defined(__GNUC__) || (__ARM_FP & 2))
    auto pHalf = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pFloat = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = HalfCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(HALF))
        {
            if (OutputStride == sizeof(float))
            {
                // Packed input, packed output
                for (size_t j = 0; j < four; ++j)
                {
                    uint16x4_t vHalf = vld1_u16(reinterpret_cast<const uint16_t*>(pHalf));
                    pHalf += InputStride * 4;

                    float32x4_t vFloat = vcvt_f32_f16(vreinterpret_f16_u16(vHalf));

                    vst1q_f32(reinterpret_cast<float*>(pFloat), vFloat);
                    pFloat += OutputStride * 4;
                    i += 4;
                }
            }
            else
            {
                // Packed input, scattered output
                for (size_t j = 0; j < four; ++j)
                {
                    uint16x4_t vHalf = vld1_u16(reinterpret_cast<const uint16_t*>(pHalf));
                    pHalf += InputStride * 4;

                    float32x4_t vFloat = vcvt_f32_f16(vreinterpret_f16_u16(vHalf));

                    vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 0);
                    pFloat += OutputStride;
                    vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 1);
                    pFloat += OutputStride;
                    vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 2);
                    pFloat += OutputStride;
                    vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 3);
                    pFloat += OutputStride;
                    i += 4;
                }
            }
        }
        else if (OutputStride == sizeof(float))
        {
            // Scattered input, packed output
            for (size_t j = 0; j < four; ++j)
            {
                uint16_t H1 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H2 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H3 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H4 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;

                uint64_t iHalf = uint64_t(H1) | (uint64_t(H2) << 16) | (uint64_t(H3) << 32) | (uint64_t(H4) << 48);
                uint16x4_t vHalf = vcreate_u16(iHalf);

                float32x4_t vFloat = vcvt_f32_f16(vreinterpret_f16_u16(vHalf));

                vst1q_f32(reinterpret_cast<float*>(pFloat), vFloat);
                pFloat += OutputStride * 4;
                i += 4;
            }
        }
        else
        {
            // Scattered input, scattered output
            for (size_t j = 0; j < four; ++j)
            {
                uint16_t H1 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H2 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H3 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;
                uint16_t H4 = *reinterpret_cast<const HALF*>(pHalf);
                pHalf += InputStride;

                uint64_t iHalf = uint64_t(H1) | (uint64_t(H2) << 16) | (uint64_t(H3) << 32) | (uint64_t(H4) << 48);
                uint16x4_t vHalf = vcreate_u16(iHalf);

                float32x4_t vFloat = vcvt_f32_f16(vreinterpret_f16_u16(vHalf));

                vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 0);
                pFloat += OutputStride;
                vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 1);
                pFloat += OutputStride;
                vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 2);
                pFloat += OutputStride;
                vst1q_lane_f32(reinterpret_cast<float*>(pFloat), vFloat, 3);
                pFloat += OutputStride;
                i += 4;
            }
        }
    }

    for (; i < HalfCount; ++i)
    {
        *reinterpret_cast<float*>(pFloat) = XMConvertHalfToFloat(reinterpret_cast<const HALF*>(pHalf)[0]);
        pHalf += InputStride;
        pFloat += OutputStride;
    }

    return pOutputStream;
#else
    auto pHalf = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pFloat = reinterpret_cast<uint8_t*>(pOutputStream);

    for (size_t i = 0; i < HalfCount; i++)
    {
        *reinterpret_cast<float*>(pFloat) = XMConvertHalfToFloat(reinterpret_cast<const HALF*>(pHalf)[0]);
        pHalf += InputStride;
        pFloat += OutputStride;
    }

    return pOutputStream;
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------

inline HALF XMConvertFloatToHalf(float Value) noexcept
{
#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    __m128 V1 = _mm_set_ss(Value);
    __m128i V2 = _mm_cvtps_ph(V1, _MM_FROUND_TO_NEAREST_INT);
    return static_cast<HALF>(_mm_extract_epi16(V2, 0));
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__) && !defined(_XM_NO_INTRINSICS_) && (!defined(__GNUC__) || (__ARM_FP & 2))
    float32x4_t vFloat = vdupq_n_f32(Value);
    float16x4_t vHalf = vcvt_f16_f32(vFloat);
    return vget_lane_u16(vreinterpret_u16_f16(vHalf), 0);
#else
    uint32_t Result;

    auto IValue = reinterpret_cast<uint32_t*>(&Value)[0];
    uint32_t Sign = (IValue & 0x80000000U) >> 16U;
    IValue = IValue & 0x7FFFFFFFU;      // Hack off the sign
    if (IValue >= 0x47800000 /*e+16*/)
    {
        // The number is too large to be represented as a half. Return infinity or NaN
        Result = 0x7C00U | ((IValue > 0x7F800000) ? (0x200 | ((IValue >> 13U) & 0x3FFU)) : 0U);
    }
    else if (IValue <= 0x33000000U /*e-25*/)
    {
        Result = 0;
    }
    else if (IValue < 0x38800000U /*e-14*/)
    {
        // The number is too small to be represented as a normalized half.
        // Convert it to a denormalized value.
        uint32_t Shift = 125U - (IValue >> 23U);
        IValue = 0x800000U | (IValue & 0x7FFFFFU);
        Result = IValue >> (Shift + 1);
        uint32_t s = (IValue & ((1U << Shift) - 1)) != 0;
        Result += (Result | s) & ((IValue >> Shift) & 1U);
    }
    else
    {
        // Rebias the exponent to represent the value as a normalized half.
        IValue += 0xC8000000U;
        Result = ((IValue + 0x0FFFU + ((IValue >> 13U) & 1U)) >> 13U) & 0x7FFFU;
    }
    return static_cast<HALF>(Result | Sign);
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline HALF* XMConvertFloatToHalfStream
(
    HALF* pOutputStream,
    size_t       OutputStride,
    const float* pInputStream,
    size_t       InputStride,
    size_t       FloatCount
) noexcept
{
    assert(pOutputStream);
    assert(pInputStream);

    assert(InputStride >= sizeof(float));
    _Analysis_assume_(InputStride >= sizeof(float));

    assert(OutputStride >= sizeof(HALF));
    _Analysis_assume_(OutputStride >= sizeof(HALF));

#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    auto pFloat = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pHalf = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = FloatCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(float))
        {
            if (OutputStride == sizeof(HALF))
            {
                if ((reinterpret_cast<uintptr_t>(pFloat) & 0xF) == 0)
                {
                    // Aligned and packed input, packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 FV = _mm_load_ps(reinterpret_cast<const float*>(pFloat));
                        pFloat += InputStride * 4;

                        __m128i HV = _mm_cvtps_ph(FV, _MM_FROUND_TO_NEAREST_INT);

                        _mm_storel_epi64(reinterpret_cast<__m128i*>(pHalf), HV);
                        pHalf += OutputStride * 4;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, packed output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 FV = _mm_loadu_ps(reinterpret_cast<const float*>(pFloat));
                        pFloat += InputStride * 4;

                        __m128i HV = _mm_cvtps_ph(FV, _MM_FROUND_TO_NEAREST_INT);

                        _mm_storel_epi64(reinterpret_cast<__m128i*>(pHalf), HV);
                        pHalf += OutputStride * 4;
                        i += 4;
                    }
                }
            }
            else
            {
                if ((reinterpret_cast<uintptr_t>(pFloat) & 0xF) == 0)
                {
                    // Aligned & packed input, scattered output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 FV = _mm_load_ps(reinterpret_cast<const float*>(pFloat));
                        pFloat += InputStride * 4;

                        __m128i HV = _mm_cvtps_ph(FV, _MM_FROUND_TO_NEAREST_INT);

                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 0));
                        pHalf += OutputStride;
                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 1));
                        pHalf += OutputStride;
                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 2));
                        pHalf += OutputStride;
                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 3));
                        pHalf += OutputStride;
                        i += 4;
                    }
                }
                else
                {
                    // Packed input, scattered output
                    for (size_t j = 0; j < four; ++j)
                    {
                        __m128 FV = _mm_loadu_ps(reinterpret_cast<const float*>(pFloat));
                        pFloat += InputStride * 4;

                        __m128i HV = _mm_cvtps_ph(FV, _MM_FROUND_TO_NEAREST_INT);

                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 0));
                        pHalf += OutputStride;
                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 1));
                        pHalf += OutputStride;
                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 2));
                        pHalf += OutputStride;
                        *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 3));
                        pHalf += OutputStride;
                        i += 4;
                    }
                }
            }
        }
        else if (OutputStride == sizeof(HALF))
        {
            // Scattered input, packed output
            for (size_t j = 0; j < four; ++j)
            {
                __m128 FV1 = _mm_load_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV2 = _mm_broadcast_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV3 = _mm_broadcast_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV4 = _mm_broadcast_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV = _mm_blend_ps(FV1, FV2, 0x2);
                __m128 FT = _mm_blend_ps(FV3, FV4, 0x8);
                FV = _mm_blend_ps(FV, FT, 0xC);

                __m128i HV = _mm_cvtps_ph(FV, _MM_FROUND_TO_NEAREST_INT);

                _mm_storel_epi64(reinterpret_cast<__m128i*>(pHalf), HV);
                pHalf += OutputStride * 4;
                i += 4;
            }
        }
        else
        {
            // Scattered input, scattered output
            for (size_t j = 0; j < four; ++j)
            {
                __m128 FV1 = _mm_load_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV2 = _mm_broadcast_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV3 = _mm_broadcast_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV4 = _mm_broadcast_ss(reinterpret_cast<const float*>(pFloat));
                pFloat += InputStride;

                __m128 FV = _mm_blend_ps(FV1, FV2, 0x2);
                __m128 FT = _mm_blend_ps(FV3, FV4, 0x8);
                FV = _mm_blend_ps(FV, FT, 0xC);

                __m128i HV = _mm_cvtps_ph(FV, _MM_FROUND_TO_NEAREST_INT);

                *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 0));
                pHalf += OutputStride;
                *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 1));
                pHalf += OutputStride;
                *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 2));
                pHalf += OutputStride;
                *reinterpret_cast<HALF*>(pHalf) = static_cast<HALF>(_mm_extract_epi16(HV, 3));
                pHalf += OutputStride;
                i += 4;
            }
        }
    }

    for (; i < FloatCount; ++i)
    {
        *reinterpret_cast<HALF*>(pHalf) = XMConvertFloatToHalf(reinterpret_cast<const float*>(pFloat)[0]);
        pFloat += InputStride;
        pHalf += OutputStride;
    }

    return pOutputStream;
#elif defined(_XM_ARM_NEON_INTRINSICS_) && (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC) || __aarch64__) && !defined(_XM_NO_INTRINSICS_) && (!defined(__GNUC__) || (__ARM_FP & 2))
    auto pFloat = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pHalf = reinterpret_cast<uint8_t*>(pOutputStream);

    size_t i = 0;
    size_t four = FloatCount >> 2;
    if (four > 0)
    {
        if (InputStride == sizeof(float))
        {
            if (OutputStride == sizeof(HALF))
            {
                // Packed input, packed output
                for (size_t j = 0; j < four; ++j)
                {
                    float32x4_t vFloat = vld1q_f32(reinterpret_cast<const float*>(pFloat));
                    pFloat += InputStride * 4;

                    uint16x4_t vHalf = vreinterpret_u16_f16(vcvt_f16_f32(vFloat));

                    vst1_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf);
                    pHalf += OutputStride * 4;
                    i += 4;
                }
            }
            else
            {
                // Packed input, scattered output
                for (size_t j = 0; j < four; ++j)
                {
                    float32x4_t vFloat = vld1q_f32(reinterpret_cast<const float*>(pFloat));
                    pFloat += InputStride * 4;

                    uint16x4_t vHalf = vreinterpret_u16_f16(vcvt_f16_f32(vFloat));

                    vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 0);
                    pHalf += OutputStride;
                    vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 1);
                    pHalf += OutputStride;
                    vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 2);
                    pHalf += OutputStride;
                    vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 3);
                    pHalf += OutputStride;
                    i += 4;
                }
            }
        }
        else if (OutputStride == sizeof(HALF))
        {
            // Scattered input, packed output
            for (size_t j = 0; j < four; ++j)
            {
                float32x4_t vFloat = vdupq_n_f32(0);
                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 0);
                pFloat += InputStride;

                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 1);
                pFloat += InputStride;

                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 2);
                pFloat += InputStride;

                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 3);
                pFloat += InputStride;

                uint16x4_t vHalf = vreinterpret_u16_f16(vcvt_f16_f32(vFloat));

                vst1_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf);
                pHalf += OutputStride * 4;
                i += 4;
            }
        }
        else
        {
            // Scattered input, scattered output
            for (size_t j = 0; j < four; ++j)
            {
                float32x4_t vFloat = vdupq_n_f32(0);
                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 0);
                pFloat += InputStride;

                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 1);
                pFloat += InputStride;

                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 2);
                pFloat += InputStride;

                vFloat = vld1q_lane_f32(reinterpret_cast<const float*>(pFloat), vFloat, 3);
                pFloat += InputStride;

                uint16x4_t vHalf = vreinterpret_u16_f16(vcvt_f16_f32(vFloat));

                vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 0);
                pHalf += OutputStride;
                vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 1);
                pHalf += OutputStride;
                vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 2);
                pHalf += OutputStride;
                vst1_lane_u16(reinterpret_cast<uint16_t*>(pHalf), vHalf, 3);
                pHalf += OutputStride;
                i += 4;
            }
        }
    }

    for (; i < FloatCount; ++i)
    {
        *reinterpret_cast<HALF*>(pHalf) = XMConvertFloatToHalf(reinterpret_cast<const float*>(pFloat)[0]);
        pFloat += InputStride;
        pHalf += OutputStride;
    }

    return pOutputStream;
#else
    auto pFloat = reinterpret_cast<const uint8_t*>(pInputStream);
    auto pHalf = reinterpret_cast<uint8_t*>(pOutputStream);

    for (size_t i = 0; i < FloatCount; i++)
    {
        *reinterpret_cast<HALF*>(pHalf) = XMConvertFloatToHalf(reinterpret_cast<const float*>(pFloat)[0]);
        pFloat += InputStride;
        pHalf += OutputStride;
    }
    return pOutputStream;
#endif // !_XM_F16C_INTRINSICS_
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

/****************************************************************************
 *
 * Vector and matrix load operations
 *
 ****************************************************************************/

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable:28931, "PREfast noise: Esp:1266")
#endif

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadColor(const XMCOLOR* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    // int32_t -> Float conversions are done in one instruction.
    // uint32_t -> Float calls a runtime function. Keep in int32_t
    auto iColor = static_cast<int32_t>(pSource->c);
    XMVECTORF32 vColor = { { {
            static_cast<float>((iColor >> 16) & 0xFF)* (1.0f / 255.0f),
            static_cast<float>((iColor >> 8) & 0xFF)* (1.0f / 255.0f),
            static_cast<float>(iColor & 0xFF)* (1.0f / 255.0f),
            static_cast<float>((iColor >> 24) & 0xFF)* (1.0f / 255.0f)
        } } };
    return vColor.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32_t bgra = pSource->c;
    uint32_t rgba = (bgra & 0xFF00FF00) | ((bgra >> 16) & 0xFF) | ((bgra << 16) & 0xFF0000);
    uint32x2_t vInt8 = vdup_n_u32(rgba);
    uint16x8_t vInt16 = vmovl_u8(vreinterpret_u8_u32(vInt8));
    uint32x4_t vInt = vmovl_u16(vget_low_u16(vInt16));
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_n_f32(R, 1.0f / 255.0f);
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the color in all four entries
    __m128i vInt = _mm_set1_epi32(static_cast<int>(pSource->c));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vInt = _mm_and_si128(vInt, g_XMMaskA8R8G8B8);
    // a is unsigned! Flip the bit to convert the order to signed
    vInt = _mm_xor_si128(vInt, g_XMFlipA8R8G8B8);
    // Convert to floating point numbers
    XMVECTOR vTemp = _mm_cvtepi32_ps(vInt);
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMFixAA8R8G8B8);
    // Convert 0-255 to 0.0f-1.0f
    return _mm_mul_ps(vTemp, g_XMNormalizeA8R8G8B8);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadHalf2(const XMHALF2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    __m128 V = _mm_load_ss(reinterpret_cast<const float*>(pSource));
    return _mm_cvtph_ps(_mm_castps_si128(V));
#else
    XMVECTORF32 vResult = { { {
            XMConvertHalfToFloat(pSource->x),
            XMConvertHalfToFloat(pSource->y),
            0.0f,
            0.0f
        } } };
    return vResult.v;
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadShortN2(const XMSHORTN2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            (pSource->x == -32768) ? -1.f : (static_cast<float>(pSource->x)* (1.0f / 32767.0f)),
            (pSource->y == -32768) ? -1.f : (static_cast<float>(pSource->y)* (1.0f / 32767.0f)),
            0.0f,
            0.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt16 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    int32x4_t vInt = vmovl_s16(vreinterpret_s16_u32(vInt16));
    vInt = vandq_s32(vInt, g_XMMaskXY);
    float32x4_t R = vcvtq_f32_s32(vInt);
    R = vmulq_n_f32(R, 1.0f / 32767.0f);
    return vmaxq_f32(R, vdupq_n_f32(-1.f));
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the two shorts in all four entries (WORD alignment okay,
    // DWORD alignment preferred)
    __m128 vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0xFFFF, y&0xFFFF0000,z&0,w&0
    vTemp = _mm_and_ps(vTemp, g_XMMaskX16Y16);
    // x needs to be sign extended
    vTemp = _mm_xor_ps(vTemp, g_XMFlipX16Y16);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x - 0x8000 to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMFixX16Y16);
    // Convert -1.0f - 1.0f
    vTemp = _mm_mul_ps(vTemp, g_XMNormalizeX16Y16);
    // Clamp result (for case of -32768)
    return _mm_max_ps(vTemp, g_XMNegativeOne);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadShort2(const XMSHORT2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            0.f,
            0.f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt16 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    int32x4_t vInt = vmovl_s16(vreinterpret_s16_u32(vInt16));
    vInt = vandq_s32(vInt, g_XMMaskXY);
    return vcvtq_f32_s32(vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the two shorts in all four entries (WORD alignment okay,
    // DWORD alignment preferred)
    __m128 vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0xFFFF, y&0xFFFF0000,z&0,w&0
    vTemp = _mm_and_ps(vTemp, g_XMMaskX16Y16);
    // x needs to be sign extended
    vTemp = _mm_xor_ps(vTemp, g_XMFlipX16Y16);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x - 0x8000 to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMFixX16Y16);
    // Y is 65536 too large
    return _mm_mul_ps(vTemp, g_XMFixupY16);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUShortN2(const XMUSHORTN2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x) / 65535.0f,
            static_cast<float>(pSource->y) / 65535.0f,
            0.f,
            0.f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt16 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    uint32x4_t vInt = vmovl_u16(vreinterpret_u16_u32(vInt16));
    vInt = vandq_u32(vInt, g_XMMaskXY);
    float32x4_t R = vcvtq_f32_u32(vInt);
    R = vmulq_n_f32(R, 1.0f / 65535.0f);
    return vmaxq_f32(R, vdupq_n_f32(-1.f));
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 FixupY16 = { { { 1.0f / 65535.0f, 1.0f / (65535.0f * 65536.0f), 0.0f, 0.0f } } };
    static const XMVECTORF32 FixaddY16 = { { { 0, 32768.0f * 65536.0f, 0, 0 } } };
    // Splat the two shorts in all four entries (WORD alignment okay,
    // DWORD alignment preferred)
    __m128 vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0xFFFF, y&0xFFFF0000,z&0,w&0
    vTemp = _mm_and_ps(vTemp, g_XMMaskX16Y16);
    // y needs to be sign flipped
    vTemp = _mm_xor_ps(vTemp, g_XMFlipY);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // y + 0x8000 to undo the signed order.
    vTemp = _mm_add_ps(vTemp, FixaddY16);
    // Y is 65536 times too large
    vTemp = _mm_mul_ps(vTemp, FixupY16);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUShort2(const XMUSHORT2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            0.f,
            0.f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt16 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    uint32x4_t vInt = vmovl_u16(vreinterpret_u16_u32(vInt16));
    vInt = vandq_u32(vInt, g_XMMaskXY);
    return vcvtq_f32_u32(vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 FixaddY16 = { { { 0, 32768.0f, 0, 0 } } };
    // Splat the two shorts in all four entries (WORD alignment okay,
    // DWORD alignment preferred)
    __m128 vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0xFFFF, y&0xFFFF0000,z&0,w&0
    vTemp = _mm_and_ps(vTemp, g_XMMaskX16Y16);
    // y needs to be sign flipped
    vTemp = _mm_xor_ps(vTemp, g_XMFlipY);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // Y is 65536 times too large
    vTemp = _mm_mul_ps(vTemp, g_XMFixupY16);
    // y + 0x8000 to undo the signed order.
    vTemp = _mm_add_ps(vTemp, FixaddY16);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadByteN2(const XMBYTEN2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            (pSource->x == -128) ? -1.f : (static_cast<float>(pSource->x)* (1.0f / 127.0f)),
            (pSource->y == -128) ? -1.f : (static_cast<float>(pSource->y)* (1.0f / 127.0f)),
            0.0f,
            0.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint16x4_t vInt8 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    int16x8_t vInt16 = vmovl_s8(vreinterpret_s8_u16(vInt8));
    int32x4_t vInt = vmovl_s16(vget_low_s16(vInt16));
    vInt = vandq_s32(vInt, g_XMMaskXY);
    float32x4_t R = vcvtq_f32_s32(vInt);
    R = vmulq_n_f32(R, 1.0f / 127.0f);
    return vmaxq_f32(R, vdupq_n_f32(-1.f));
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f / 127.0f, 1.0f / (127.0f * 256.0f), 0, 0 } } };
    static const XMVECTORU32 Mask = { { { 0xFF, 0xFF00, 0, 0 } } };
    // Splat the color in all four entries (x,z,y,w)
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vTemp = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0, 0, 0, 0));
    // Mask
    vTemp = _mm_and_ps(vTemp, Mask);
    // x,y and z are unsigned! Flip the bits to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMXorByte4);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x, y and z - 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddByte4);
    // Fix y, z and w because they are too large
    vTemp = _mm_mul_ps(vTemp, Scale);
    // Clamp result (for case of -128)
    return _mm_max_ps(vTemp, g_XMNegativeOne);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadByte2(const XMBYTE2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            0.0f,
            0.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint16x4_t vInt8 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    int16x8_t vInt16 = vmovl_s8(vreinterpret_s8_u16(vInt8));
    int32x4_t vInt = vmovl_s16(vget_low_s16(vInt16));
    vInt = vandq_s32(vInt, g_XMMaskXY);
    return vcvtq_f32_s32(vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f, 1.0f / 256.0f, 1.0f / 65536.0f, 1.0f / (65536.0f * 256.0f) } } };
    static const XMVECTORU32 Mask = { { { 0xFF, 0xFF00, 0, 0 } } };
    // Splat the color in all four entries (x,z,y,w)
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vTemp = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0, 0, 0, 0));
    // Mask
    vTemp = _mm_and_ps(vTemp, Mask);
    // x,y and z are unsigned! Flip the bits to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMXorByte4);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x, y and z - 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddByte4);
    // Fix y, z and w because they are too large
    return _mm_mul_ps(vTemp, Scale);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUByteN2(const XMUBYTEN2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x)* (1.0f / 255.0f),
            static_cast<float>(pSource->y)* (1.0f / 255.0f),
            0.0f,
            0.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint16x4_t vInt8 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint16x8_t vInt16 = vmovl_u8(vreinterpret_u8_u16(vInt8));
    uint32x4_t vInt = vmovl_u16(vget_low_u16(vInt16));
    vInt = vandq_u32(vInt, g_XMMaskXY);
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_n_f32(R, 1.0f / 255.0f);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f / 255.0f, 1.0f / (255.0f * 256.0f), 0, 0 } } };
    static const XMVECTORU32 Mask = { { { 0xFF, 0xFF00, 0, 0 } } };
    // Splat the color in all four entries (x,z,y,w)
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vTemp = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0, 0, 0, 0));
    // Mask
    vTemp = _mm_and_ps(vTemp, Mask);
    // w is signed! Flip the bits to convert the order to unsigned
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // w + 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Fix y, z and w because they are too large
    return _mm_mul_ps(vTemp, Scale);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUByte2(const XMUBYTE2* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            0.0f,
            0.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint16x4_t vInt8 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint16x8_t vInt16 = vmovl_u8(vreinterpret_u8_u16(vInt8));
    uint32x4_t vInt = vmovl_u16(vget_low_u16(vInt16));
    vInt = vandq_u32(vInt, g_XMMaskXY);
    return vcvtq_f32_u32(vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f, 1.0f / 256.0f, 0, 0 } } };
    static const XMVECTORU32 Mask = { { { 0xFF, 0xFF00, 0, 0 } } };
    // Splat the color in all four entries (x,z,y,w)
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vTemp = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0, 0, 0, 0));
    // Mask
    vTemp = _mm_and_ps(vTemp, Mask);
    // w is signed! Flip the bits to convert the order to unsigned
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // w + 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Fix y, z and w because they are too large
    return _mm_mul_ps(vTemp, Scale);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadU565(const XMU565* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            float(pSource->v & 0x1F),
            float((pSource->v >> 5) & 0x3F),
            float((pSource->v >> 11) & 0x1F),
            0.f,
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORI32 U565And = { { { 0x1F, 0x3F << 5, 0x1F << 11, 0 } } };
    static const XMVECTORF32 U565Mul = { { { 1.0f, 1.0f / 32.0f, 1.0f / 2048.f, 0 } } };
    uint16x4_t vInt16 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint32x4_t vInt = vmovl_u16(vInt16);
    vInt = vandq_u32(vInt, U565And);
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_f32(R, U565Mul);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORI32 U565And = { { { 0x1F, 0x3F << 5, 0x1F << 11, 0 } } };
    static const XMVECTORF32 U565Mul = { { { 1.0f, 1.0f / 32.0f, 1.0f / 2048.f, 0 } } };
    // Get the 16 bit value and splat it
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vResult = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0, 0, 0, 0));
    // Mask off x, y and z
    vResult = _mm_and_ps(vResult, U565And);
    // Convert to float
    vResult = _mm_cvtepi32_ps(_mm_castps_si128(vResult));
    // Normalize x, y, and z
    vResult = _mm_mul_ps(vResult, U565Mul);
    return vResult;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadFloat3PK(const XMFLOAT3PK* pSource) noexcept
{
    assert(pSource);

    XM_ALIGNED_DATA(16) uint32_t Result[4];
    uint32_t Mantissa;
    uint32_t Exponent;

    // X Channel (6-bit mantissa)
    Mantissa = pSource->xm;

    if (pSource->xe == 0x1f) // INF or NAN
    {
        Result[0] = static_cast<uint32_t>(0x7f800000 | (static_cast<int>(pSource->xm) << 17));
    }
    else
    {
        if (pSource->xe != 0) // The value is normalized
        {
            Exponent = pSource->xe;
        }
        else if (Mantissa != 0) // The value is denormalized
        {
            // Normalize the value in the resulting float
            Exponent = 1;

            do
            {
                Exponent--;
                Mantissa <<= 1;
            } while ((Mantissa & 0x40) == 0);

            Mantissa &= 0x3F;
        }
        else // The value is zero
        {
            Exponent = static_cast<uint32_t>(-112);
        }

        Result[0] = ((Exponent + 112) << 23) | (Mantissa << 17);
    }

    // Y Channel (6-bit mantissa)
    Mantissa = pSource->ym;

    if (pSource->ye == 0x1f) // INF or NAN
    {
        Result[1] = static_cast<uint32_t>(0x7f800000 | (static_cast<int>(pSource->ym) << 17));
    }
    else
    {
        if (pSource->ye != 0) // The value is normalized
        {
            Exponent = pSource->ye;
        }
        else if (Mantissa != 0) // The value is denormalized
        {
            // Normalize the value in the resulting float
            Exponent = 1;

            do
            {
                Exponent--;
                Mantissa <<= 1;
            } while ((Mantissa & 0x40) == 0);

            Mantissa &= 0x3F;
        }
        else // The value is zero
        {
            Exponent = static_cast<uint32_t>(-112);
        }

        Result[1] = ((Exponent + 112) << 23) | (Mantissa << 17);
    }

    // Z Channel (5-bit mantissa)
    Mantissa = pSource->zm;

    if (pSource->ze == 0x1f) // INF or NAN
    {
        Result[2] = static_cast<uint32_t>(0x7f800000 | (static_cast<int>(pSource->zm) << 17));
    }
    else
    {
        if (pSource->ze != 0) // The value is normalized
        {
            Exponent = pSource->ze;
        }
        else if (Mantissa != 0) // The value is denormalized
        {
            // Normalize the value in the resulting float
            Exponent = 1;

            do
            {
                Exponent--;
                Mantissa <<= 1;
            } while ((Mantissa & 0x20) == 0);

            Mantissa &= 0x1F;
        }
        else // The value is zero
        {
            Exponent = static_cast<uint32_t>(-112);
        }

        Result[2] = ((Exponent + 112) << 23) | (Mantissa << 18);
    }

    return XMLoadFloat3A(reinterpret_cast<const XMFLOAT3A*>(&Result));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadFloat3SE(const XMFLOAT3SE* pSource) noexcept
{
    assert(pSource);

    union { float f; int32_t i; } fi;
    fi.i = 0x33800000 + (pSource->e << 23);
    float Scale = fi.f;

    XMVECTORF32 v = { { {
            Scale * float(pSource->xm),
            Scale * float(pSource->ym),
            Scale * float(pSource->zm),
            1.0f } } };
    return v;
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadHalf4(const XMHALF4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    __m128i V = _mm_loadl_epi64(reinterpret_cast<const __m128i*>(pSource));
    return _mm_cvtph_ps(V);
#else
    XMVECTORF32 vResult = { { {
            XMConvertHalfToFloat(pSource->x),
            XMConvertHalfToFloat(pSource->y),
            XMConvertHalfToFloat(pSource->z),
            XMConvertHalfToFloat(pSource->w)
        } } };
    return vResult.v;
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadShortN4(const XMSHORTN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            (pSource->x == -32768) ? -1.f : (static_cast<float>(pSource->x)* (1.0f / 32767.0f)),
            (pSource->y == -32768) ? -1.f : (static_cast<float>(pSource->y)* (1.0f / 32767.0f)),
            (pSource->z == -32768) ? -1.f : (static_cast<float>(pSource->z)* (1.0f / 32767.0f)),
            (pSource->w == -32768) ? -1.f : (static_cast<float>(pSource->w)* (1.0f / 32767.0f))
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    int16x4_t vInt = vld1_s16(reinterpret_cast<const int16_t*>(pSource));
    int32x4_t V = vmovl_s16(vInt);
    float32x4_t vResult = vcvtq_f32_s32(V);
    vResult = vmulq_n_f32(vResult, 1.0f / 32767.0f);
    return vmaxq_f32(vResult, vdupq_n_f32(-1.f));
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the color in all four entries (x,z,y,w)
    __m128d vIntd = _mm_load1_pd(reinterpret_cast<const double*>(&pSource->x));
    // Shift x&0ffff,z&0xffff,y&0xffff0000,w&0xffff0000
    __m128 vTemp = _mm_and_ps(_mm_castpd_ps(vIntd), g_XMMaskX16Y16Z16W16);
    // x and z are unsigned! Flip the bits to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMFlipX16Y16Z16W16);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x and z - 0x8000 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMFixX16Y16Z16W16);
    // Convert to -1.0f - 1.0f
    vTemp = _mm_mul_ps(vTemp, g_XMNormalizeX16Y16Z16W16);
    // Very important! The entries are x,z,y,w, flip it to x,y,z,w
    vTemp = XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 1, 2, 0));
    // Clamp result (for case of -32768)
    return _mm_max_ps(vTemp, g_XMNegativeOne);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadShort4(const XMSHORT4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            static_cast<float>(pSource->z),
            static_cast<float>(pSource->w)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    int16x4_t vInt = vld1_s16(reinterpret_cast<const int16_t*>(pSource));
    int32x4_t V = vmovl_s16(vInt);
    return vcvtq_f32_s32(V);
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the color in all four entries (x,z,y,w)
    __m128d vIntd = _mm_load1_pd(reinterpret_cast<const double*>(&pSource->x));
    // Shift x&0ffff,z&0xffff,y&0xffff0000,w&0xffff0000
    __m128 vTemp = _mm_and_ps(_mm_castpd_ps(vIntd), g_XMMaskX16Y16Z16W16);
    // x and z are unsigned! Flip the bits to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMFlipX16Y16Z16W16);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x and z - 0x8000 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMFixX16Y16Z16W16);
    // Fix y and w because they are 65536 too large
    vTemp = _mm_mul_ps(vTemp, g_XMFixupY16W16);
    // Very important! The entries are x,z,y,w, flip it to x,y,z,w
    return XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 1, 2, 0));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUShortN4(const XMUSHORTN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x) / 65535.0f,
            static_cast<float>(pSource->y) / 65535.0f,
            static_cast<float>(pSource->z) / 65535.0f,
            static_cast<float>(pSource->w) / 65535.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint16x4_t vInt = vld1_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint32x4_t V = vmovl_u16(vInt);
    float32x4_t vResult = vcvtq_f32_u32(V);
    return vmulq_n_f32(vResult, 1.0f / 65535.0f);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 FixupY16W16 = { { { 1.0f / 65535.0f, 1.0f / 65535.0f, 1.0f / (65535.0f * 65536.0f), 1.0f / (65535.0f * 65536.0f) } } };
    static const XMVECTORF32 FixaddY16W16 = { { { 0, 0, 32768.0f * 65536.0f, 32768.0f * 65536.0f } } };
    // Splat the color in all four entries (x,z,y,w)
    __m128d vIntd = _mm_load1_pd(reinterpret_cast<const double*>(&pSource->x));
    // Shift x&0ffff,z&0xffff,y&0xffff0000,w&0xffff0000
    __m128 vTemp = _mm_and_ps(_mm_castpd_ps(vIntd), g_XMMaskX16Y16Z16W16);
    // y and w are signed! Flip the bits to convert the order to unsigned
    vTemp = _mm_xor_ps(vTemp, g_XMFlipZW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // y and w + 0x8000 to complete the conversion
    vTemp = _mm_add_ps(vTemp, FixaddY16W16);
    // Fix y and w because they are 65536 too large
    vTemp = _mm_mul_ps(vTemp, FixupY16W16);
    // Very important! The entries are x,z,y,w, flip it to x,y,z,w
    return XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 1, 2, 0));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUShort4(const XMUSHORT4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            static_cast<float>(pSource->z),
            static_cast<float>(pSource->w)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint16x4_t vInt = vld1_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint32x4_t V = vmovl_u16(vInt);
    return vcvtq_f32_u32(V);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 FixaddY16W16 = { { { 0, 0, 32768.0f, 32768.0f } } };
    // Splat the color in all four entries (x,z,y,w)
    __m128d vIntd = _mm_load1_pd(reinterpret_cast<const double*>(&pSource->x));
    // Shift x&0ffff,z&0xffff,y&0xffff0000,w&0xffff0000
    __m128 vTemp = _mm_and_ps(_mm_castpd_ps(vIntd), g_XMMaskX16Y16Z16W16);
    // y and w are signed! Flip the bits to convert the order to unsigned
    vTemp = _mm_xor_ps(vTemp, g_XMFlipZW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // Fix y and w because they are 65536 too large
    vTemp = _mm_mul_ps(vTemp, g_XMFixupY16W16);
    // y and w + 0x8000 to complete the conversion
    vTemp = _mm_add_ps(vTemp, FixaddY16W16);
    // Very important! The entries are x,z,y,w, flip it to x,y,z,w
    return XM_PERMUTE_PS(vTemp, _MM_SHUFFLE(3, 1, 2, 0));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadXDecN4(const XMXDECN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    static const uint32_t SignExtend[] = { 0x00000000, 0xFFFFFC00 };

    uint32_t ElementX = pSource->v & 0x3FF;
    uint32_t ElementY = (pSource->v >> 10) & 0x3FF;
    uint32_t ElementZ = (pSource->v >> 20) & 0x3FF;

    XMVECTORF32 vResult = { { {
            (ElementX == 0x200) ? -1.f : (static_cast<float>(static_cast<int16_t>(ElementX | SignExtend[ElementX >> 9])) / 511.0f),
            (ElementY == 0x200) ? -1.f : (static_cast<float>(static_cast<int16_t>(ElementY | SignExtend[ElementY >> 9])) / 511.0f),
            (ElementZ == 0x200) ? -1.f : (static_cast<float>(static_cast<int16_t>(ElementZ | SignExtend[ElementZ >> 9])) / 511.0f),
            static_cast<float>(pSource->v >> 30) / 3.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskA2B10G10R10);
    vInt = veorq_u32(vInt, g_XMFlipA2B10G10R10);
    float32x4_t R = vcvtq_f32_s32(vreinterpretq_s32_u32(vInt));
    R = vaddq_f32(R, g_XMFixAA2B10G10R10);
    R = vmulq_f32(R, g_XMNormalizeA2B10G10R10);
    return vmaxq_f32(R, vdupq_n_f32(-1.0f));
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the color in all four entries
    __m128 vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskA2B10G10R10);
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMFlipA2B10G10R10);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMFixAA2B10G10R10);
    // Convert 0-255 to 0.0f-1.0f
    vTemp = _mm_mul_ps(vTemp, g_XMNormalizeA2B10G10R10);
    // Clamp result (for case of -512)
    return _mm_max_ps(vTemp, g_XMNegativeOne);
#endif
}

//------------------------------------------------------------------------------
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4996)
// C4996: ignore deprecation warning
#endif

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadXDec4(const XMXDEC4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    static const uint32_t SignExtend[] = { 0x00000000, 0xFFFFFC00 };

    uint32_t ElementX = pSource->v & 0x3FF;
    uint32_t ElementY = (pSource->v >> 10) & 0x3FF;
    uint32_t ElementZ = (pSource->v >> 20) & 0x3FF;

    XMVECTORF32 vResult = { { {
            static_cast<float>(static_cast<int16_t>(ElementX | SignExtend[ElementX >> 9])),
            static_cast<float>(static_cast<int16_t>(ElementY | SignExtend[ElementY >> 9])),
            static_cast<float>(static_cast<int16_t>(ElementZ | SignExtend[ElementZ >> 9])),
            static_cast<float>(pSource->v >> 30)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORU32 XDec4Xor = { { { 0x200, 0x200 << 10, 0x200 << 20, 0x80000000 } } };
    static const XMVECTORF32 XDec4Add = { { { -512.0f, -512.0f * 1024.0f, -512.0f * 1024.0f * 1024.0f, 32768 * 65536.0f } } };
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskDec4);
    vInt = veorq_u32(vInt, XDec4Xor);
    float32x4_t R = vcvtq_f32_s32(vreinterpretq_s32_u32(vInt));
    R = vaddq_f32(R, XDec4Add);
    return vmulq_f32(R, g_XMMulDec4);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORU32 XDec4Xor = { { { 0x200, 0x200 << 10, 0x200 << 20, 0x80000000 } } };
    static const XMVECTORF32 XDec4Add = { { { -512.0f, -512.0f * 1024.0f, -512.0f * 1024.0f * 1024.0f, 32768 * 65536.0f } } };
    // Splat the color in all four entries
    XMVECTOR vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskDec4);
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, XDec4Xor);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, XDec4Add);
    // Convert 0-255 to 0.0f-1.0f
    vTemp = _mm_mul_ps(vTemp, g_XMMulDec4);
    return vTemp;
#endif
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUDecN4(const XMUDECN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)

    uint32_t ElementX = pSource->v & 0x3FF;
    uint32_t ElementY = (pSource->v >> 10) & 0x3FF;
    uint32_t ElementZ = (pSource->v >> 20) & 0x3FF;

    XMVECTORF32 vResult = { { {
            static_cast<float>(ElementX) / 1023.0f,
            static_cast<float>(ElementY) / 1023.0f,
            static_cast<float>(ElementZ) / 1023.0f,
            static_cast<float>(pSource->v >> 30) / 3.0f
        } } };
    return vResult.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 UDecN4Mul = { { { 1.0f / 1023.0f, 1.0f / (1023.0f * 1024.0f), 1.0f / (1023.0f * 1024.0f * 1024.0f), 1.0f / (3.0f * 1024.0f * 1024.0f * 1024.0f) } } };
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskDec4);
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_f32(R, UDecN4Mul);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 UDecN4Mul = { { { 1.0f / 1023.0f, 1.0f / (1023.0f * 1024.0f), 1.0f / (1023.0f * 1024.0f * 1024.0f), 1.0f / (3.0f * 1024.0f * 1024.0f * 1024.0f) } } };
    // Splat the color in all four entries
    XMVECTOR vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskDec4);
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Convert 0-255 to 0.0f-1.0f
    vTemp = _mm_mul_ps(vTemp, UDecN4Mul);
    return vTemp;
#endif
}


//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUDecN4_XR(const XMUDECN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)

    int32_t ElementX = pSource->v & 0x3FF;
    int32_t ElementY = (pSource->v >> 10) & 0x3FF;
    int32_t ElementZ = (pSource->v >> 20) & 0x3FF;

    XMVECTORF32 vResult = { { {
            static_cast<float>(ElementX - 0x180) / 510.0f,
            static_cast<float>(ElementY - 0x180) / 510.0f,
            static_cast<float>(ElementZ - 0x180) / 510.0f,
            static_cast<float>(pSource->v >> 30) / 3.0f
        } } };

    return vResult.v;

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 XRMul = { { { 1.0f / 510.0f, 1.0f / (510.0f * 1024.0f), 1.0f / (510.0f * 1024.0f * 1024.0f), 1.0f / (3.0f * 1024.0f * 1024.0f * 1024.0f) } } };
    static const XMVECTORI32 XRBias = { { { 0x180, 0x180 * 1024, 0x180 * 1024 * 1024, 0 } } };
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskDec4);
    int32x4_t vTemp = vsubq_s32(vreinterpretq_s32_u32(vInt), XRBias);
    vTemp = veorq_s32(vTemp, g_XMFlipW);
    float32x4_t R = vcvtq_f32_s32(vTemp);
    R = vaddq_f32(R, g_XMAddUDec4);
    return vmulq_f32(R, XRMul);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 XRMul = { { { 1.0f / 510.0f, 1.0f / (510.0f * 1024.0f), 1.0f / (510.0f * 1024.0f * 1024.0f), 1.0f / (3.0f * 1024.0f * 1024.0f * 1024.0f) } } };
    static const XMVECTORI32 XRBias = { { { 0x180, 0x180 * 1024, 0x180 * 1024 * 1024, 0 } } };
    // Splat the color in all four entries
    XMVECTOR vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Mask channels
    vTemp = _mm_and_ps(vTemp, g_XMMaskDec4);
    // Subtract bias
    vTemp = _mm_castsi128_ps(_mm_sub_epi32(_mm_castps_si128(vTemp), XRBias));
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Convert to 0.0f-1.0f
    return _mm_mul_ps(vTemp, XRMul);
#endif
}


//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUDec4(const XMUDEC4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    uint32_t ElementX = pSource->v & 0x3FF;
    uint32_t ElementY = (pSource->v >> 10) & 0x3FF;
    uint32_t ElementZ = (pSource->v >> 20) & 0x3FF;

    XMVECTORF32 vResult = { { {
            static_cast<float>(ElementX),
            static_cast<float>(ElementY),
            static_cast<float>(ElementZ),
            static_cast<float>(pSource->v >> 30)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskDec4);
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_f32(R, g_XMMulDec4);
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the color in all four entries
    XMVECTOR vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskDec4);
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Convert 0-255 to 0.0f-1.0f
    vTemp = _mm_mul_ps(vTemp, g_XMMulDec4);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4996)
// C4996: ignore deprecation warning
#endif

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif

_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadDecN4(const XMDECN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    static const uint32_t SignExtend[] = { 0x00000000, 0xFFFFFC00 };
    static const uint32_t SignExtendW[] = { 0x00000000, 0xFFFFFFFC };

    uint32_t ElementX = pSource->v & 0x3FF;
    uint32_t ElementY = (pSource->v >> 10) & 0x3FF;
    uint32_t ElementZ = (pSource->v >> 20) & 0x3FF;
    uint32_t ElementW = pSource->v >> 30;

    XMVECTORF32 vResult = { { {
            (ElementX == 0x200) ? -1.f : (static_cast<float>(static_cast<int16_t>(ElementX | SignExtend[ElementX >> 9])) / 511.0f),
            (ElementY == 0x200) ? -1.f : (static_cast<float>(static_cast<int16_t>(ElementY | SignExtend[ElementY >> 9])) / 511.0f),
            (ElementZ == 0x200) ? -1.f : (static_cast<float>(static_cast<int16_t>(ElementZ | SignExtend[ElementZ >> 9])) / 511.0f),
            (ElementW == 0x2) ? -1.f : static_cast<float>(static_cast<int16_t>(ElementW | SignExtendW[(ElementW >> 1) & 1]))
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 DecN4Mul = { { { 1.0f / 511.0f, 1.0f / (511.0f * 1024.0f), 1.0f / (511.0f * 1024.0f * 1024.0f), 1.0f / (1024.0f * 1024.0f * 1024.0f) } } };
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskDec4);
    vInt = veorq_u32(vInt, g_XMXorDec4);
    float32x4_t R = vcvtq_f32_s32(vreinterpretq_s32_u32(vInt));
    R = vaddq_f32(R, g_XMAddDec4);
    R = vmulq_f32(R, DecN4Mul);
    return vmaxq_f32(R, vdupq_n_f32(-1.0f));
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 DecN4Mul = { { { 1.0f / 511.0f, 1.0f / (511.0f * 1024.0f), 1.0f / (511.0f * 1024.0f * 1024.0f), 1.0f / (1024.0f * 1024.0f * 1024.0f) } } };
    // Splat the color in all four entries
    XMVECTOR vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskDec4);
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMXorDec4);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMAddDec4);
    // Convert 0-255 to 0.0f-1.0f
    vTemp = _mm_mul_ps(vTemp, DecN4Mul);
    // Clamp result (for case of -512/-1)
    return _mm_max_ps(vTemp, g_XMNegativeOne);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadDec4(const XMDEC4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    static const uint32_t SignExtend[] = { 0x00000000, 0xFFFFFC00 };
    static const uint32_t SignExtendW[] = { 0x00000000, 0xFFFFFFFC };

    uint32_t ElementX = pSource->v & 0x3FF;
    uint32_t ElementY = (pSource->v >> 10) & 0x3FF;
    uint32_t ElementZ = (pSource->v >> 20) & 0x3FF;
    uint32_t ElementW = pSource->v >> 30;

    XMVECTORF32 vResult = { { {
            static_cast<float>(static_cast<int16_t>(ElementX | SignExtend[ElementX >> 9])),
            static_cast<float>(static_cast<int16_t>(ElementY | SignExtend[ElementY >> 9])),
            static_cast<float>(static_cast<int16_t>(ElementZ | SignExtend[ElementZ >> 9])),
            static_cast<float>(static_cast<int16_t>(ElementW | SignExtendW[ElementW >> 1]))
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x4_t vInt = vld1q_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    vInt = vandq_u32(vInt, g_XMMaskDec4);
    vInt = veorq_u32(vInt, g_XMXorDec4);
    float32x4_t R = vcvtq_f32_s32(vreinterpretq_s32_u32(vInt));
    R = vaddq_f32(R, g_XMAddDec4);
    return vmulq_f32(R, g_XMMulDec4);
#elif defined(_XM_SSE_INTRINSICS_)
    // Splat the color in all four entries
    XMVECTOR vTemp = _mm_load_ps1(reinterpret_cast<const float*>(&pSource->v));
    // Shift R&0xFF0000, G&0xFF00, B&0xFF, A&0xFF000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskDec4);
    // a is unsigned! Flip the bit to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMXorDec4);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // RGB + 0, A + 0x80000000.f to undo the signed order.
    vTemp = _mm_add_ps(vTemp, g_XMAddDec4);
    // Convert 0-255 to 0.0f-1.0f
    vTemp = _mm_mul_ps(vTemp, g_XMMulDec4);
    return vTemp;
#endif
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUByteN4(const XMUBYTEN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x) / 255.0f,
            static_cast<float>(pSource->y) / 255.0f,
            static_cast<float>(pSource->z) / 255.0f,
            static_cast<float>(pSource->w) / 255.0f
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt8 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    uint16x8_t vInt16 = vmovl_u8(vreinterpret_u8_u32(vInt8));
    uint32x4_t vInt = vmovl_u16(vget_low_u16(vInt16));
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_n_f32(R, 1.0f / 255.0f);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 LoadUByteN4Mul = { { { 1.0f / 255.0f, 1.0f / (255.0f * 256.0f), 1.0f / (255.0f * 65536.0f), 1.0f / (255.0f * 65536.0f * 256.0f) } } };
    // Splat the color in all four entries (x,z,y,w)
    XMVECTOR vTemp = _mm_load1_ps(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0ff,y&0xff00,z&0xff0000,w&0xff000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskByte4);
    // w is signed! Flip the bits to convert the order to unsigned
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // w + 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Fix y, z and w because they are too large
    vTemp = _mm_mul_ps(vTemp, LoadUByteN4Mul);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUByte4(const XMUBYTE4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            static_cast<float>(pSource->z),
            static_cast<float>(pSource->w)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt8 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    uint16x8_t vInt16 = vmovl_u8(vreinterpret_u8_u32(vInt8));
    uint32x4_t vInt = vmovl_u16(vget_low_u16(vInt16));
    return vcvtq_f32_u32(vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 LoadUByte4Mul = { { { 1.0f, 1.0f / 256.0f, 1.0f / 65536.0f, 1.0f / (65536.0f * 256.0f) } } };
    // Splat the color in all four entries (x,z,y,w)
    XMVECTOR vTemp = _mm_load1_ps(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0ff,y&0xff00,z&0xff0000,w&0xff000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskByte4);
    // w is signed! Flip the bits to convert the order to unsigned
    vTemp = _mm_xor_ps(vTemp, g_XMFlipW);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // w + 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddUDec4);
    // Fix y, z and w because they are too large
    vTemp = _mm_mul_ps(vTemp, LoadUByte4Mul);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadByteN4(const XMBYTEN4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            (pSource->x == -128) ? -1.f : (static_cast<float>(pSource->x) / 127.0f),
            (pSource->y == -128) ? -1.f : (static_cast<float>(pSource->y) / 127.0f),
            (pSource->z == -128) ? -1.f : (static_cast<float>(pSource->z) / 127.0f),
            (pSource->w == -128) ? -1.f : (static_cast<float>(pSource->w) / 127.0f)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt8 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    int16x8_t vInt16 = vmovl_s8(vreinterpret_s8_u32(vInt8));
    int32x4_t vInt = vmovl_s16(vget_low_s16(vInt16));
    float32x4_t R = vcvtq_f32_s32(vInt);
    R = vmulq_n_f32(R, 1.0f / 127.0f);
    return vmaxq_f32(R, vdupq_n_f32(-1.f));
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 LoadByteN4Mul = { { { 1.0f / 127.0f, 1.0f / (127.0f * 256.0f), 1.0f / (127.0f * 65536.0f), 1.0f / (127.0f * 65536.0f * 256.0f) } } };
    // Splat the color in all four entries (x,z,y,w)
    XMVECTOR vTemp = _mm_load1_ps(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0ff,y&0xff00,z&0xff0000,w&0xff000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskByte4);
    // x,y and z are unsigned! Flip the bits to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMXorByte4);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x, y and z - 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddByte4);
    // Fix y, z and w because they are too large
    vTemp = _mm_mul_ps(vTemp, LoadByteN4Mul);
    // Clamp result (for case of -128)
    return _mm_max_ps(vTemp, g_XMNegativeOne);
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadByte4(const XMBYTE4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            static_cast<float>(pSource->x),
            static_cast<float>(pSource->y),
            static_cast<float>(pSource->z),
            static_cast<float>(pSource->w)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    uint32x2_t vInt8 = vld1_dup_u32(reinterpret_cast<const uint32_t*>(pSource));
    int16x8_t vInt16 = vmovl_s8(vreinterpret_s8_u32(vInt8));
    int32x4_t vInt = vmovl_s16(vget_low_s16(vInt16));
    return vcvtq_f32_s32(vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 LoadByte4Mul = { { { 1.0f, 1.0f / 256.0f, 1.0f / 65536.0f, 1.0f / (65536.0f * 256.0f) } } };
    // Splat the color in all four entries (x,z,y,w)
    XMVECTOR vTemp = _mm_load1_ps(reinterpret_cast<const float*>(&pSource->x));
    // Mask x&0ff,y&0xff00,z&0xff0000,w&0xff000000
    vTemp = _mm_and_ps(vTemp, g_XMMaskByte4);
    // x,y and z are unsigned! Flip the bits to convert the order to signed
    vTemp = _mm_xor_ps(vTemp, g_XMXorByte4);
    // Convert to floating point numbers
    vTemp = _mm_cvtepi32_ps(_mm_castps_si128(vTemp));
    // x, y and z - 0x80 to complete the conversion
    vTemp = _mm_add_ps(vTemp, g_XMAddByte4);
    // Fix y, z and w because they are too large
    vTemp = _mm_mul_ps(vTemp, LoadByte4Mul);
    return vTemp;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadUNibble4(const XMUNIBBLE4* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            float(pSource->v & 0xF),
            float((pSource->v >> 4) & 0xF),
            float((pSource->v >> 8) & 0xF),
            float((pSource->v >> 12) & 0xF)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORI32 UNibble4And = { { { 0xF, 0xF0, 0xF00, 0xF000 } } };
    static const XMVECTORF32 UNibble4Mul = { { { 1.0f, 1.0f / 16.f, 1.0f / 256.f, 1.0f / 4096.f } } };
    uint16x4_t vInt16 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint32x4_t vInt = vmovl_u16(vInt16);
    vInt = vandq_u32(vInt, UNibble4And);
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_f32(R, UNibble4Mul);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORI32 UNibble4And = { { { 0xF, 0xF0, 0xF00, 0xF000 } } };
    static const XMVECTORF32 UNibble4Mul = { { { 1.0f, 1.0f / 16.f, 1.0f / 256.f, 1.0f / 4096.f } } };
    // Get the 16 bit value and splat it
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vResult = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0,0,0,0));
    // Mask off x, y and z
    vResult = _mm_and_ps(vResult, UNibble4And);
    // Convert to float
    vResult = _mm_cvtepi32_ps(_mm_castps_si128(vResult));
    // Normalize x, y, and z
    vResult = _mm_mul_ps(vResult, UNibble4Mul);
    return vResult;
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMVECTOR XM_CALLCONV XMLoadU555(const XMU555* pSource) noexcept
{
    assert(pSource);
#if defined(_XM_NO_INTRINSICS_)
    XMVECTORF32 vResult = { { {
            float(pSource->v & 0x1F),
            float((pSource->v >> 5) & 0x1F),
            float((pSource->v >> 10) & 0x1F),
            float((pSource->v >> 15) & 0x1)
        } } };
    return vResult.v;
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORI32 U555And = { { { 0x1F, 0x1F << 5, 0x1F << 10, 0x8000 } } };
    static const XMVECTORF32 U555Mul = { { { 1.0f, 1.0f / 32.f, 1.0f / 1024.f, 1.0f / 32768.f } } };
    uint16x4_t vInt16 = vld1_dup_u16(reinterpret_cast<const uint16_t*>(pSource));
    uint32x4_t vInt = vmovl_u16(vInt16);
    vInt = vandq_u32(vInt, U555And);
    float32x4_t R = vcvtq_f32_u32(vInt);
    return vmulq_f32(R, U555Mul);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORI32 U555And = { { { 0x1F, 0x1F << 5, 0x1F << 10, 0x8000 } } };
    static const XMVECTORF32 U555Mul = { { { 1.0f, 1.0f / 32.f, 1.0f / 1024.f, 1.0f / 32768.f } } };
    // Get the 16bit value and splat it
    __m128i vInt = XM_LOADU_SI16(&pSource->v);
    XMVECTOR vResult = XM_PERMUTE_PS(_mm_castsi128_ps(vInt), _MM_SHUFFLE(0, 0, 0, 0));
    // Mask off x, y and z
    vResult = _mm_and_ps(vResult, U555And);
    // Convert to float
    vResult = _mm_cvtepi32_ps(_mm_castps_si128(vResult));
    // Normalize x, y, and z
    vResult = _mm_mul_ps(vResult, U555Mul);
    return vResult;
#endif
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

/****************************************************************************
 *
 * Vector and matrix store operations
 *
 ****************************************************************************/
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreColor
(
    XMCOLOR* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorSaturate(V);
    N = XMVectorMultiply(N, g_UByteMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->c = (static_cast<uint32_t>(tmp.w) << 24) |
        (static_cast<uint32_t>(tmp.x) << 16) |
        (static_cast<uint32_t>(tmp.y) << 8) |
        static_cast<uint32_t>(tmp.z);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 255.0f);
    R = XMVectorRound(R);
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    uint8x8_t vInt8 = vqmovn_u16(vcombine_u16(vInt16, vInt16));
    uint32_t rgba = vget_lane_u32(vreinterpret_u32_u8(vInt8), 0);
    pDestination->c = (rgba & 0xFF00FF00) | ((rgba >> 16) & 0xFF) | ((rgba << 16) & 0xFF0000);
#elif defined(_XM_SSE_INTRINSICS_)
    // Set <0 to 0
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    // Set>1 to 1
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Convert to 0-255
    vResult = _mm_mul_ps(vResult, g_UByteMax);
    // Shuffle RGBA to ARGB
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(3, 0, 1, 2));
    // Convert to int
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // Mash to shorts
    vInt = _mm_packs_epi32(vInt, vInt);
    // Mash to bytes
    vInt = _mm_packus_epi16(vInt, vInt);
    // Store the color
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->c), _mm_castsi128_ps(vInt));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreHalf2
(
    XMHALF2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    __m128i V1 = _mm_cvtps_ph(V, _MM_FROUND_TO_NEAREST_INT);
    _mm_store_ss(reinterpret_cast<float*>(pDestination), _mm_castsi128_ps(V1));
#else
    pDestination->x = XMConvertFloatToHalf(XMVectorGetX(V));
    pDestination->y = XMConvertFloatToHalf(XMVectorGetY(V));
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreShortN2
(
    XMSHORTN2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_XMNegativeOne.v, g_XMOne.v);
    N = XMVectorMultiply(N, g_ShortMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int16_t>(tmp.x);
    pDestination->y = static_cast<int16_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(-1.f));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 32767.0f);
    int32x4_t vInt32 = vcvtq_s32_f32(R);
    int16x4_t vInt16 = vqmovn_s32(vInt32);
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_s16(vInt16), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_max_ps(V, g_XMNegativeOne);
    vResult = _mm_min_ps(vResult, g_XMOne);
    vResult = _mm_mul_ps(vResult, g_ShortMax);
    __m128i vResulti = _mm_cvtps_epi32(vResult);
    vResulti = _mm_packs_epi32(vResulti, vResulti);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->x), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreShort2
(
    XMSHORT2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_ShortMin, g_ShortMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int16_t>(tmp.x);
    pDestination->y = static_cast<int16_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(-32767.f));
    R = vminq_f32(R, vdupq_n_f32(32767.0f));
    int32x4_t vInt32 = vcvtq_s32_f32(R);
    int16x4_t vInt16 = vqmovn_s32(vInt32);
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_s16(vInt16), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_ShortMin);
    vResult = _mm_min_ps(vResult, g_ShortMax);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // Pack the ints into shorts
    vInt = _mm_packs_epi32(vInt, vInt);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->x), _mm_castsi128_ps(vInt));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUShortN2
(
    XMUSHORTN2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorSaturate(V);
    N = XMVectorMultiplyAdd(N, g_UShortMax, g_XMOneHalf.v);
    N = XMVectorTruncate(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint16_t>(tmp.x);
    pDestination->y = static_cast<uint16_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0.f));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 65535.0f);
    R = vaddq_f32(R, g_XMOneHalf);
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_u16(vInt16), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_XMOne);
    vResult = _mm_mul_ps(vResult, g_UShortMax);
    vResult = _mm_add_ps(vResult, g_XMOneHalf);
    // Convert to int
    __m128i vInt = _mm_cvttps_epi32(vResult);
    // Since the SSE pack instruction clamps using signed rules,
    // manually extract the values to store them to memory
    pDestination->x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    pDestination->y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUShort2
(
    XMUSHORT2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), g_UShortMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint16_t>(tmp.x);
    pDestination->y = static_cast<uint16_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0.f));
    R = vminq_f32(R, vdupq_n_f32(65535.0f));
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_u16(vInt16), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_UShortMax);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // Since the SSE pack instruction clamps using signed rules,
    // manually extract the values to store them to memory
    pDestination->x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    pDestination->y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreByteN2
(
    XMBYTEN2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_XMNegativeOne.v, g_XMOne.v);
    N = XMVectorMultiply(N, g_ByteMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int8_t>(tmp.x);
    pDestination->y = static_cast<int8_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(-1.f));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 127.0f);
    int32x4_t vInt32 = vcvtq_s32_f32(R);
    int16x4_t vInt16 = vqmovn_s32(vInt32);
    int8x8_t vInt8 = vqmovn_s16(vcombine_s16(vInt16, vInt16));
    vst1_lane_u16(reinterpret_cast<uint16_t*>(pDestination), vreinterpret_u16_s8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMNegativeOne);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, g_ByteMax);
    // Convert to int by rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    pDestination->v = static_cast<uint16_t>(((static_cast<int>(y) & 0xFF) << 8) | (static_cast<int>(x) & 0xFF));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreByte2
(
    XMBYTE2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_ByteMin, g_ByteMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int8_t>(tmp.x);
    pDestination->y = static_cast<int8_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(-127.f));
    R = vminq_f32(R, vdupq_n_f32(127.0f));
    int32x4_t vInt32 = vcvtq_s32_f32(R);
    int16x4_t vInt16 = vqmovn_s32(vInt32);
    int8x8_t vInt8 = vqmovn_s16(vcombine_s16(vInt16, vInt16));
    vst1_lane_u16(reinterpret_cast<uint16_t*>(pDestination), vreinterpret_u16_s8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_ByteMin);
    vResult = _mm_min_ps(vResult, g_ByteMax);
    // Convert to int by rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    pDestination->v = static_cast<uint16_t>(((static_cast<int>(y) & 0xFF) << 8) | (static_cast<int>(x) & 0xFF));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUByteN2
(
    XMUBYTEN2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorSaturate(V);
    N = XMVectorMultiplyAdd(N, g_UByteMax, g_XMOneHalf.v);
    N = XMVectorTruncate(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint8_t>(tmp.x);
    pDestination->y = static_cast<uint8_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0.f));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 255.0f);
    R = vaddq_f32(R, g_XMOneHalf);
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    uint8x8_t vInt8 = vqmovn_u16(vcombine_u16(vInt16, vInt16));
    vst1_lane_u16(reinterpret_cast<uint16_t*>(pDestination), vreinterpret_u16_u8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, g_UByteMax);
    vResult = _mm_add_ps(vResult, g_XMOneHalf);
    // Convert to int
    __m128i vInt = _mm_cvttps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    pDestination->v = static_cast<uint16_t>(((static_cast<int>(y) & 0xFF) << 8) | (static_cast<int>(x) & 0xFF));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUByte2
(
    XMUBYTE2* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), g_UByteMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint8_t>(tmp.x);
    pDestination->y = static_cast<uint8_t>(tmp.y);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0.f));
    R = vminq_f32(R, vdupq_n_f32(255.0f));
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    uint8x8_t vInt8 = vqmovn_u16(vcombine_u16(vInt16, vInt16));
    vst1_lane_u16(reinterpret_cast<uint16_t*>(pDestination), vreinterpret_u16_u8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_UByteMax);
    // Convert to int by rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    pDestination->v = static_cast<uint16_t>(((static_cast<int>(y) & 0xFF) << 8) | (static_cast<int>(x) & 0xFF));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreU565
(
    XMU565* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 Max = { { { 31.0f, 63.0f, 31.0f, 0.0f } } };

#if defined(_XM_NO_INTRINSICS_)
    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), Max.v);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint16_t>(
        ((static_cast<int>(tmp.z) & 0x1F) << 11)
        | ((static_cast<int>(tmp.y) & 0x3F) << 5)
        | ((static_cast<int>(tmp.x) & 0x1F)));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f, 32.f, 32.f * 64.f, 0.f } } };
    static const XMVECTORU32 Mask = { { { 0x1F, 0x3F << 5, 0x1F << 11, 0 } } };
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0));
    vResult = vminq_f32(vResult, Max);
    vResult = vmulq_f32(vResult, Scale);
    uint32x4_t vResulti = vcvtq_u32_f32(vResult);
    vResulti = vandq_u32(vResulti, Mask);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vResulti);
    uint32x2_t vhi = vget_high_u32(vResulti);
    vTemp = vorr_u32(vTemp, vhi);
    vTemp = vpadd_u32(vTemp, vTemp);
    vst1_lane_u16(&pDestination->v, vreinterpret_u16_u32(vTemp), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, Max);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    auto z = static_cast<uint16_t>(_mm_extract_epi16(vInt, 4));
    pDestination->v = static_cast<uint16_t>(
        ((static_cast<int>(z) & 0x1F) << 11)
        | ((static_cast<int>(y) & 0x3F) << 5)
        | ((static_cast<int>(x) & 0x1F)));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreFloat3PK
(
    XMFLOAT3PK* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);

    XM_ALIGNED_DATA(16) uint32_t IValue[4];
    XMStoreFloat3A(reinterpret_cast<XMFLOAT3A*>(&IValue), V);

    uint32_t Result[3];

    // X & Y Channels (5-bit exponent, 6-bit mantissa)
    for (uint32_t j = 0; j < 2; ++j)
    {
        uint32_t Sign = IValue[j] & 0x80000000;
        uint32_t I = IValue[j] & 0x7FFFFFFF;

        if ((I & 0x7F800000) == 0x7F800000)
        {
            // INF or NAN
            Result[j] = 0x7C0U;
            if ((I & 0x7FFFFF) != 0)
            {
                Result[j] = 0x7FFU;
            }
            else if (Sign)
            {
                // -INF is clamped to 0 since 3PK is positive only
                Result[j] = 0;
            }
        }
        else if (Sign || I < 0x35800000)
        {
            // 3PK is positive only, so clamp to zero
            Result[j] = 0;
        }
        else if (I > 0x477E0000U)
        {
            // The number is too large to be represented as a float11, set to max
            Result[j] = 0x7BFU;
        }
        else
        {
            if (I < 0x38800000U)
            {
                // The number is too small to be represented as a normalized float11
                // Convert it to a denormalized value.
                uint32_t Shift = 113U - (I >> 23U);
                I = (0x800000U | (I & 0x7FFFFFU)) >> Shift;
            }
            else
            {
                // Rebias the exponent to represent the value as a normalized float11
                I += 0xC8000000U;
            }

            Result[j] = ((I + 0xFFFFU + ((I >> 17U) & 1U)) >> 17U) & 0x7ffU;
        }
    }

    // Z Channel (5-bit exponent, 5-bit mantissa)
    uint32_t Sign = IValue[2] & 0x80000000;
    uint32_t I = IValue[2] & 0x7FFFFFFF;

    if ((I & 0x7F800000) == 0x7F800000)
    {
        // INF or NAN
        Result[2] = 0x3E0U;
        if (I & 0x7FFFFF)
        {
            Result[2] = 0x3FFU;
        }
        else if (Sign || I < 0x36000000)
        {
            // -INF is clamped to 0 since 3PK is positive only
            Result[2] = 0;
        }
    }
    else if (Sign)
    {
        // 3PK is positive only, so clamp to zero
        Result[2] = 0;
    }
    else if (I > 0x477C0000U)
    {
        // The number is too large to be represented as a float10, set to max
        Result[2] = 0x3DFU;
    }
    else
    {
        if (I < 0x38800000U)
        {
            // The number is too small to be represented as a normalized float10
            // Convert it to a denormalized value.
            uint32_t Shift = 113U - (I >> 23U);
            I = (0x800000U | (I & 0x7FFFFFU)) >> Shift;
        }
        else
        {
            // Rebias the exponent to represent the value as a normalized float10
            I += 0xC8000000U;
        }

        Result[2] = ((I + 0x1FFFFU + ((I >> 18U) & 1U)) >> 18U) & 0x3ffU;
    }

    // Pack Result into memory
    pDestination->v = (Result[0] & 0x7ff)
        | ((Result[1] & 0x7ff) << 11)
        | ((Result[2] & 0x3ff) << 22);
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreFloat3SE
(
    XMFLOAT3SE* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);

    XMFLOAT3A tmp;
    XMStoreFloat3A(&tmp, V);

    static constexpr float maxf9 = float(0x1FF << 7);
    static constexpr float minf9 = float(1.f / (1 << 16));

    float x = (tmp.x >= 0.f) ? ((tmp.x > maxf9) ? maxf9 : tmp.x) : 0.f;
    float y = (tmp.y >= 0.f) ? ((tmp.y > maxf9) ? maxf9 : tmp.y) : 0.f;
    float z = (tmp.z >= 0.f) ? ((tmp.z > maxf9) ? maxf9 : tmp.z) : 0.f;

    const float max_xy = (x > y) ? x : y;
    const float max_xyz = (max_xy > z) ? max_xy : z;

    const float maxColor = (max_xyz > minf9) ? max_xyz : minf9;

    union { float f; int32_t i; } fi;
    fi.f = maxColor;
    fi.i += 0x00004000; // round up leaving 9 bits in fraction (including assumed 1)

    auto exp = static_cast<uint32_t>(fi.i) >> 23;
    pDestination->e = exp - 0x6f;

    fi.i = static_cast<int32_t>(0x83000000 - (exp << 23));
    float ScaleR = fi.f;

    pDestination->xm = static_cast<uint32_t>(Internal::round_to_nearest(x * ScaleR));
    pDestination->ym = static_cast<uint32_t>(Internal::round_to_nearest(y * ScaleR));
    pDestination->zm = static_cast<uint32_t>(Internal::round_to_nearest(z * ScaleR));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreHalf4
(
    XMHALF4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_F16C_INTRINSICS_) && !defined(_XM_NO_INTRINSICS_)
    __m128i V1 = _mm_cvtps_ph(V, _MM_FROUND_TO_NEAREST_INT);
    _mm_storel_epi64(reinterpret_cast<__m128i*>(pDestination), V1);
#else
    XMFLOAT4A t;
    XMStoreFloat4A(&t, V);

    pDestination->x = XMConvertFloatToHalf(t.x);
    pDestination->y = XMConvertFloatToHalf(t.y);
    pDestination->z = XMConvertFloatToHalf(t.z);
    pDestination->w = XMConvertFloatToHalf(t.w);
#endif // !_XM_F16C_INTRINSICS_
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreShortN4
(
    XMSHORTN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_XMNegativeOne.v, g_XMOne.v);
    N = XMVectorMultiply(N, g_ShortMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int16_t>(tmp.x);
    pDestination->y = static_cast<int16_t>(tmp.y);
    pDestination->z = static_cast<int16_t>(tmp.z);
    pDestination->w = static_cast<int16_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(-1.f));
    vResult = vminq_f32(vResult, vdupq_n_f32(1.0f));
    vResult = vmulq_n_f32(vResult, 32767.0f);
    int16x4_t vInt = vmovn_s32(vcvtq_s32_f32(vResult));
    vst1_s16(reinterpret_cast<int16_t*>(pDestination), vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    XMVECTOR vResult = _mm_max_ps(V, g_XMNegativeOne);
    vResult = _mm_min_ps(vResult, g_XMOne);
    vResult = _mm_mul_ps(vResult, g_ShortMax);
    __m128i vResulti = _mm_cvtps_epi32(vResult);
    vResulti = _mm_packs_epi32(vResulti, vResulti);
    _mm_store_sd(reinterpret_cast<double*>(&pDestination->x), _mm_castsi128_pd(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreShort4
(
    XMSHORT4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_ShortMin, g_ShortMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int16_t>(tmp.x);
    pDestination->y = static_cast<int16_t>(tmp.y);
    pDestination->z = static_cast<int16_t>(tmp.z);
    pDestination->w = static_cast<int16_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vResult = vmaxq_f32(V, g_ShortMin);
    vResult = vminq_f32(vResult, g_ShortMax);
    int16x4_t vInt = vmovn_s32(vcvtq_s32_f32(vResult));
    vst1_s16(reinterpret_cast<int16_t*>(pDestination), vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_ShortMin);
    vResult = _mm_min_ps(vResult, g_ShortMax);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // Pack the ints into shorts
    vInt = _mm_packs_epi32(vInt, vInt);
    _mm_store_sd(reinterpret_cast<double*>(&pDestination->x), _mm_castsi128_pd(vInt));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUShortN4
(
    XMUSHORTN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorSaturate(V);
    N = XMVectorMultiplyAdd(N, g_UShortMax, g_XMOneHalf.v);
    N = XMVectorTruncate(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint16_t>(tmp.x);
    pDestination->y = static_cast<uint16_t>(tmp.y);
    pDestination->z = static_cast<uint16_t>(tmp.z);
    pDestination->w = static_cast<uint16_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0));
    vResult = vminq_f32(vResult, vdupq_n_f32(1.0f));
    vResult = vmulq_n_f32(vResult, 65535.0f);
    vResult = vaddq_f32(vResult, g_XMOneHalf);
    uint16x4_t vInt = vmovn_u32(vcvtq_u32_f32(vResult));
    vst1_u16(reinterpret_cast<uint16_t*>(pDestination), vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_XMOne);
    vResult = _mm_mul_ps(vResult, g_UShortMax);
    vResult = _mm_add_ps(vResult, g_XMOneHalf);
    // Convert to int
    __m128i vInt = _mm_cvttps_epi32(vResult);
    // Since the SSE pack instruction clamps using signed rules,
    // manually extract the values to store them to memory
    pDestination->x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    pDestination->y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    pDestination->z = static_cast<uint16_t>(_mm_extract_epi16(vInt, 4));
    pDestination->w = static_cast<uint16_t>(_mm_extract_epi16(vInt, 6));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUShort4
(
    XMUSHORT4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), g_UShortMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint16_t>(tmp.x);
    pDestination->y = static_cast<uint16_t>(tmp.y);
    pDestination->z = static_cast<uint16_t>(tmp.z);
    pDestination->w = static_cast<uint16_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0));
    vResult = vminq_f32(vResult, g_UShortMax);
    uint16x4_t vInt = vmovn_u32(vcvtq_u32_f32(vResult));
    vst1_u16(reinterpret_cast<uint16_t*>(pDestination), vInt);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_UShortMax);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // Since the SSE pack instruction clamps using signed rules,
    // manually extract the values to store them to memory
    pDestination->x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    pDestination->y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    pDestination->z = static_cast<uint16_t>(_mm_extract_epi16(vInt, 4));
    pDestination->w = static_cast<uint16_t>(_mm_extract_epi16(vInt, 6));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreXDecN4
(
    XMXDECN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 Min = { { { -1.0f, -1.0f, -1.0f, 0.0f } } };

#if defined(_XM_NO_INTRINSICS_)

    static const XMVECTORF32  Scale = { { { 511.0f, 511.0f, 511.0f, 3.0f } } };

    XMVECTOR N = XMVectorClamp(V, Min.v, g_XMOne.v);
    N = XMVectorMultiply(N, Scale.v);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | (static_cast<int>(tmp.x) & 0x3FF));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 511.0f, 511.0f * 1024.0f, 511.0f * 1048576.0f, 3.0f * 536870912.0f } } };
    static const XMVECTORI32 ScaleMask = { { { 0x3FF, 0x3FF << 10, 0x3FF << 20, 0x3 << 29 } } };
    float32x4_t vResult = vmaxq_f32(V, Min);
    vResult = vminq_f32(vResult, vdupq_n_f32(1.0f));
    vResult = vmulq_f32(vResult, Scale);
    int32x4_t vResulti = vcvtq_s32_f32(vResult);
    vResulti = vandq_s32(vResulti, ScaleMask);
    int32x4_t vResultw = vandq_s32(vResulti, g_XMMaskW);
    vResulti = vaddq_s32(vResulti, vResultw);
    // Do a horizontal or of all 4 entries
    uint32x2_t vTemp = vget_low_u32(vreinterpretq_u32_s32(vResulti));
    uint32x2_t vhi = vget_high_u32(vreinterpretq_u32_s32(vResulti));
    vTemp = vorr_u32(vTemp, vhi);
    vTemp = vpadd_u32(vTemp, vTemp);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 511.0f, 511.0f * 1024.0f, 511.0f * 1048576.0f, 3.0f * 536870912.0f } } };
    static const XMVECTORI32 ScaleMask = { { { 0x3FF, 0x3FF << 10, 0x3FF << 20, 0x3 << 29 } } };
    XMVECTOR vResult = _mm_max_ps(V, Min);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, Scale);
    // Convert to int (W is unsigned)
    __m128i vResulti = _mm_cvtps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, ScaleMask);
    // To fix W, add itself to shift it up to <<30 instead of <<29
    __m128i vResultw = _mm_and_si128(vResulti, g_XMMaskW);
    vResulti = _mm_add_epi32(vResulti, vResultw);
    // Do a horizontal or of all 4 entries
    vResult = XM_PERMUTE_PS(_mm_castsi128_ps(vResulti), _MM_SHUFFLE(0, 3, 2, 1));
    vResulti = _mm_or_si128(vResulti, _mm_castps_si128(vResult));
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 3, 2, 1));
    vResulti = _mm_or_si128(vResulti, _mm_castps_si128(vResult));
    vResult = XM_PERMUTE_PS(vResult, _MM_SHUFFLE(0, 3, 2, 1));
    vResulti = _mm_or_si128(vResulti, _mm_castps_si128(vResult));
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4996)
// C4996: ignore deprecation warning
#endif

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif

_Use_decl_annotations_
inline void XM_CALLCONV XMStoreXDec4
(
    XMXDEC4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 MinXDec4 = { { { -511.0f, -511.0f, -511.0f, 0.0f } } };
    static const XMVECTORF32 MaxXDec4 = { { { 511.0f, 511.0f, 511.0f, 3.0f } } };

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, MinXDec4, MaxXDec4);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | ((static_cast<int>(tmp.x) & 0x3FF)));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 ScaleXDec4 = { { { 1.0f, 1024.0f / 2.0f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f / 2.0f } } };
    static const XMVECTORI32 MaskXDec4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    float32x4_t vResult = vmaxq_f32(V, MinXDec4);
    vResult = vminq_f32(vResult, MaxXDec4);
    vResult = vmulq_f32(vResult, ScaleXDec4);
    int32x4_t vResulti = vcvtq_s32_f32(vResult);
    vResulti = vandq_s32(vResulti, MaskXDec4);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vreinterpretq_u32_s32(vResulti));
    uint32x2_t vTemp2 = vget_high_u32(vreinterpretq_u32_s32(vResulti));
    vTemp = vorr_u32(vTemp, vTemp2);
    // Perform a single bit left shift on y|w
    vTemp2 = vdup_lane_u32(vTemp, 1);
    vTemp2 = vadd_u32(vTemp2, vTemp2);
    vTemp = vorr_u32(vTemp, vTemp2);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleXDec4 = { { { 1.0f, 1024.0f / 2.0f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f / 2.0f } } };
    static const XMVECTORI32 MaskXDec4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, MinXDec4);
    vResult = _mm_min_ps(vResult, MaxXDec4);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleXDec4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskXDec4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // Perform a single bit left shift on y|w
    vResulti2 = _mm_add_epi32(vResulti2, vResulti2);
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUDecN4
(
    XMUDECN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    static const XMVECTORF32  Scale = { { { 1023.0f, 1023.0f, 1023.0f, 3.0f } } };

    XMVECTOR N = XMVectorSaturate(V);
    N = XMVectorMultiply(N, Scale.v);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | ((static_cast<int>(tmp.x) & 0x3FF)));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 ScaleUDecN4 = { { { 1023.0f, 1023.0f * 1024.0f * 0.5f, 1023.0f * 1024.0f * 1024.0f, 3.0f * 1024.0f * 1024.0f * 1024.0f * 0.5f } } };
    static const XMVECTORI32 MaskUDecN4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0.f));
    vResult = vminq_f32(vResult, vdupq_n_f32(1.f));
    vResult = vmulq_f32(vResult, ScaleUDecN4);
    uint32x4_t vResulti = vcvtq_u32_f32(vResult);
    vResulti = vandq_u32(vResulti, MaskUDecN4);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vResulti);
    uint32x2_t vTemp2 = vget_high_u32(vResulti);
    vTemp = vorr_u32(vTemp, vTemp2);
    // Perform a single bit left shift on y|w
    vTemp2 = vdup_lane_u32(vTemp, 1);
    vTemp2 = vadd_u32(vTemp2, vTemp2);
    vTemp = vorr_u32(vTemp, vTemp2);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleUDecN4 = { { { 1023.0f, 1023.0f * 1024.0f * 0.5f, 1023.0f * 1024.0f * 1024.0f, 3.0f * 1024.0f * 1024.0f * 1024.0f * 0.5f } } };
    static const XMVECTORI32 MaskUDecN4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleUDecN4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskUDecN4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // Perform a left shift by one bit on y|w
    vResulti2 = _mm_add_epi32(vResulti2, vResulti2);
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUDecN4_XR
(
    XMUDECN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 Scale = { { { 510.0f, 510.0f, 510.0f, 3.0f } } };
    static const XMVECTORF32 Bias = { { { 384.0f, 384.0f, 384.0f, 0.0f } } };
    static const XMVECTORF32 C = { { { 1023.f, 1023.f, 1023.f, 3.f } } };

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorMultiplyAdd(V, Scale, Bias);
    N = XMVectorClamp(N, g_XMZero, C);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | ((static_cast<int>(tmp.x) & 0x3FF)));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Shift = { { { 1.0f, 1024.0f * 0.5f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f * 0.5f } } };
    static const XMVECTORU32 MaskUDecN4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    float32x4_t vResult = vmlaq_f32(Bias, V, Scale);
    vResult = vmaxq_f32(vResult, vdupq_n_f32(0.f));
    vResult = vminq_f32(vResult, C);
    vResult = vmulq_f32(vResult, Shift);
    uint32x4_t vResulti = vcvtq_u32_f32(vResult);
    vResulti = vandq_u32(vResulti, MaskUDecN4);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vResulti);
    uint32x2_t vTemp2 = vget_high_u32(vResulti);
    vTemp = vorr_u32(vTemp, vTemp2);
    // Perform a single bit left shift on y|w
    vTemp2 = vdup_lane_u32(vTemp, 1);
    vTemp2 = vadd_u32(vTemp2, vTemp2);
    vTemp = vorr_u32(vTemp, vTemp2);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 Shift = { { { 1.0f, 1024.0f * 0.5f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f * 0.5f } } };
    static const XMVECTORU32 MaskUDecN4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    // Scale & bias
    XMVECTOR vResult = XM_FMADD_PS(V, Scale, Bias);
    // Clamp to bounds
    vResult = _mm_max_ps(vResult, g_XMZero);
    vResult = _mm_min_ps(vResult, C);
    // Scale by shift values
    vResult = _mm_mul_ps(vResult, Shift);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskUDecN4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // Perform a left shift by one bit on y|w
    vResulti2 = _mm_add_epi32(vResulti2, vResulti2);
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUDec4
(
    XMUDEC4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 MaxUDec4 = { { { 1023.0f, 1023.0f, 1023.0f, 3.0f } } };

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), MaxUDec4);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | ((static_cast<int>(tmp.x) & 0x3FF)));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 ScaleUDec4 = { { { 1.0f, 1024.0f / 2.0f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f / 2.0f } } };
    static const XMVECTORI32 MaskUDec4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0.f));
    vResult = vminq_f32(vResult, MaxUDec4);
    vResult = vmulq_f32(vResult, ScaleUDec4);
    uint32x4_t vResulti = vcvtq_u32_f32(vResult);
    vResulti = vandq_u32(vResulti, MaskUDec4);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vResulti);
    uint32x2_t vTemp2 = vget_high_u32(vResulti);
    vTemp = vorr_u32(vTemp, vTemp2);
    // Perform a single bit left shift on y|w
    vTemp2 = vdup_lane_u32(vTemp, 1);
    vTemp2 = vadd_u32(vTemp2, vTemp2);
    vTemp = vorr_u32(vTemp, vTemp2);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleUDec4 = { { { 1.0f, 1024.0f / 2.0f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f / 2.0f } } };
    static const XMVECTORI32 MaskUDec4 = { { { 0x3FF, 0x3FF << (10 - 1), 0x3FF << 20, 0x3 << (30 - 1) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, MaxUDec4);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleUDec4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskUDec4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // Perform a left shift by one bit on y|w
    vResulti2 = _mm_add_epi32(vResulti2, vResulti2);
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4996)
// C4996: ignore deprecation warning
#endif

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif

_Use_decl_annotations_
inline void XM_CALLCONV XMStoreDecN4
(
    XMDECN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    static const XMVECTORF32 Scale = { { { 511.0f, 511.0f, 511.0f, 1.0f } } };

    XMVECTOR N = XMVectorClamp(V, g_XMNegativeOne.v, g_XMOne.v);
    N = XMVectorMultiply(N, Scale.v);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | ((static_cast<int>(tmp.x) & 0x3FF)));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 ScaleDecN4 = { { { 511.0f, 511.0f * 1024.0f, 511.0f * 1024.0f * 1024.0f, 1.0f * 1024.0f * 1024.0f * 1024.0f } } };
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(-1.f));
    vResult = vminq_f32(vResult, vdupq_n_f32(1.f));
    vResult = vmulq_f32(vResult, ScaleDecN4);
    int32x4_t vResulti = vcvtq_s32_f32(vResult);
    vResulti = vandq_s32(vResulti, g_XMMaskDec4);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vreinterpretq_u32_s32(vResulti));
    uint32x2_t vhi = vget_high_u32(vreinterpretq_u32_s32(vResulti));
    vTemp = vorr_u32(vTemp, vhi);
    vTemp = vpadd_u32(vTemp, vTemp);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleDecN4 = { { { 511.0f, 511.0f * 1024.0f, 511.0f * 1024.0f * 1024.0f, 1.0f * 1024.0f * 1024.0f * 1024.0f } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMNegativeOne);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleDecN4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, g_XMMaskDec4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreDec4
(
    XMDEC4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 MinDec4 = { { { -511.0f, -511.0f, -511.0f, -1.0f } } };
    static const XMVECTORF32 MaxDec4 = { { { 511.0f, 511.0f, 511.0f, 1.0f } } };

#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, MinDec4, MaxDec4);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint32_t>(
        (static_cast<int>(tmp.w) << 30)
        | ((static_cast<int>(tmp.z) & 0x3FF) << 20)
        | ((static_cast<int>(tmp.y) & 0x3FF) << 10)
        | ((static_cast<int>(tmp.x) & 0x3FF)));

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 ScaleDec4 = { { { 1.0f, 1024.0f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f } } };
    float32x4_t vResult = vmaxq_f32(V, MinDec4);
    vResult = vminq_f32(vResult, MaxDec4);
    vResult = vmulq_f32(vResult, ScaleDec4);
    int32x4_t vResulti = vcvtq_s32_f32(vResult);
    vResulti = vandq_s32(vResulti, g_XMMaskDec4);
    // Do a horizontal or of all 4 entries
    uint32x2_t vTemp = vget_low_u32(vreinterpretq_u32_s32(vResulti));
    uint32x2_t vhi = vget_high_u32(vreinterpretq_u32_s32(vResulti));
    vTemp = vorr_u32(vTemp, vhi);
    vTemp = vpadd_u32(vTemp, vTemp);
    vst1_lane_u32(&pDestination->v, vTemp, 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleDec4 = { { { 1.0f, 1024.0f, 1024.0f * 1024.0f, 1024.0f * 1024.0f * 1024.0f } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, MinDec4);
    vResult = _mm_min_ps(vResult, MaxDec4);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleDec4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, g_XMMaskDec4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUByteN4
(
    XMUBYTEN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorSaturate(V);
    N = XMVectorMultiply(N, g_UByteMax);
    N = XMVectorTruncate(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint8_t>(tmp.x);
    pDestination->y = static_cast<uint8_t>(tmp.y);
    pDestination->z = static_cast<uint8_t>(tmp.z);
    pDestination->w = static_cast<uint8_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 255.0f);
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    uint8x8_t vInt8 = vqmovn_u16(vcombine_u16(vInt16, vInt16));
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_u8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleUByteN4 = { { { 255.0f, 255.0f * 256.0f * 0.5f, 255.0f * 256.0f * 256.0f, 255.0f * 256.0f * 256.0f * 256.0f * 0.5f } } };
    static const XMVECTORI32 MaskUByteN4 = { { { 0xFF, 0xFF << (8 - 1), 0xFF << 16, 0xFF << (24 - 1) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleUByteN4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskUByteN4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // Perform a single bit left shift to fix y|w
    vResulti2 = _mm_add_epi32(vResulti2, vResulti2);
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUByte4
(
    XMUBYTE4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), g_UByteMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<uint8_t>(tmp.x);
    pDestination->y = static_cast<uint8_t>(tmp.y);
    pDestination->z = static_cast<uint8_t>(tmp.z);
    pDestination->w = static_cast<uint8_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(0));
    R = vminq_f32(R, vdupq_n_f32(255.0f));
    uint32x4_t vInt32 = vcvtq_u32_f32(R);
    uint16x4_t vInt16 = vqmovn_u32(vInt32);
    uint8x8_t vInt8 = vqmovn_u16(vcombine_u16(vInt16, vInt16));
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_u8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleUByte4 = { { { 1.0f, 256.0f * 0.5f, 256.0f * 256.0f, 256.0f * 256.0f * 256.0f * 0.5f } } };
    static const XMVECTORI32 MaskUByte4 = { { { 0xFF, 0xFF << (8 - 1), 0xFF << 16, 0xFF << (24 - 1) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, g_UByteMax);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleUByte4);
    // Convert to int by rounding
    __m128i vResulti = _mm_cvtps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskUByte4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // Perform a single bit left shift to fix y|w
    vResulti2 = _mm_add_epi32(vResulti2, vResulti2);
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreByteN4
(
    XMBYTEN4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_XMNegativeOne.v, g_XMOne.v);
    N = XMVectorMultiply(N, g_ByteMax);
    N = XMVectorTruncate(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int8_t>(tmp.x);
    pDestination->y = static_cast<int8_t>(tmp.y);
    pDestination->z = static_cast<int8_t>(tmp.z);
    pDestination->w = static_cast<int8_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(-1.f));
    R = vminq_f32(R, vdupq_n_f32(1.0f));
    R = vmulq_n_f32(R, 127.0f);
    int32x4_t vInt32 = vcvtq_s32_f32(R);
    int16x4_t vInt16 = vqmovn_s32(vInt32);
    int8x8_t vInt8 = vqmovn_s16(vcombine_s16(vInt16, vInt16));
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_s8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleByteN4 = { { { 127.0f, 127.0f * 256.0f, 127.0f * 256.0f * 256.0f, 127.0f * 256.0f * 256.0f * 256.0f } } };
    static const XMVECTORI32 MaskByteN4 = { { { 0xFF, 0xFF << 8, 0xFF << 16, static_cast<int>(0xFF000000) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_XMNegativeOne);
    vResult = _mm_min_ps(vResult, g_XMOne);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleByteN4);
    // Convert to int
    __m128i vResulti = _mm_cvttps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskByteN4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreByte4
(
    XMBYTE4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, g_ByteMin, g_ByteMax);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->x = static_cast<int8_t>(tmp.x);
    pDestination->y = static_cast<int8_t>(tmp.y);
    pDestination->z = static_cast<int8_t>(tmp.z);
    pDestination->w = static_cast<int8_t>(tmp.w);

#elif defined(_XM_ARM_NEON_INTRINSICS_)
    float32x4_t R = vmaxq_f32(V, vdupq_n_f32(-127.f));
    R = vminq_f32(R, vdupq_n_f32(127.f));
    int32x4_t vInt32 = vcvtq_s32_f32(R);
    int16x4_t vInt16 = vqmovn_s32(vInt32);
    int8x8_t vInt8 = vqmovn_s16(vcombine_s16(vInt16, vInt16));
    vst1_lane_u32(&pDestination->v, vreinterpret_u32_s8(vInt8), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    static const XMVECTORF32 ScaleByte4 = { { { 1.0f, 256.0f, 256.0f * 256.0f, 256.0f * 256.0f * 256.0f } } };
    static const XMVECTORI32 MaskByte4 = { { { 0xFF, 0xFF << 8, 0xFF << 16, static_cast<int>(0xFF000000) } } };
    // Clamp to bounds
    XMVECTOR vResult = _mm_max_ps(V, g_ByteMin);
    vResult = _mm_min_ps(vResult, g_ByteMax);
    // Scale by multiplication
    vResult = _mm_mul_ps(vResult, ScaleByte4);
    // Convert to int by rounding
    __m128i vResulti = _mm_cvtps_epi32(vResult);
    // Mask off any fraction
    vResulti = _mm_and_si128(vResulti, MaskByte4);
    // Do a horizontal or of 4 entries
    __m128i vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(3, 2, 3, 2));
    // x = x|z, y = y|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    // Move Z to the x position
    vResulti2 = _mm_shuffle_epi32(vResulti, _MM_SHUFFLE(1, 1, 1, 1));
    // i = x|y|z|w
    vResulti = _mm_or_si128(vResulti, vResulti2);
    _mm_store_ss(reinterpret_cast<float*>(&pDestination->v), _mm_castsi128_ps(vResulti));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreUNibble4
(
    XMUNIBBLE4* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 Max = { { { 15.0f, 15.0f, 15.0f, 15.0f } } };
#if defined(_XM_NO_INTRINSICS_)

    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), Max.v);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint16_t>(
        ((static_cast<int>(tmp.w) & 0xF) << 12)
        | ((static_cast<int>(tmp.z) & 0xF) << 8)
        | ((static_cast<int>(tmp.y) & 0xF) << 4)
        | (static_cast<int>(tmp.x) & 0xF));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f, 16.f, 16.f * 16.f, 16.f * 16.f * 16.f } } };
    static const XMVECTORU32 Mask = { { { 0xF, 0xF << 4, 0xF << 8, 0xF << 12 } } };
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0));
    vResult = vminq_f32(vResult, Max);
    vResult = vmulq_f32(vResult, Scale);
    uint32x4_t vResulti = vcvtq_u32_f32(vResult);
    vResulti = vandq_u32(vResulti, Mask);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vResulti);
    uint32x2_t vhi = vget_high_u32(vResulti);
    vTemp = vorr_u32(vTemp, vhi);
    vTemp = vpadd_u32(vTemp, vTemp);
    vst1_lane_u16(&pDestination->v, vreinterpret_u16_u32(vTemp), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, Max);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    auto z = static_cast<uint16_t>(_mm_extract_epi16(vInt, 4));
    auto w = static_cast<uint16_t>(_mm_extract_epi16(vInt, 6));
    pDestination->v = static_cast<uint16_t>(
        ((static_cast<int>(w) & 0xF) << 12)
        | ((static_cast<int>(z) & 0xF) << 8)
        | ((static_cast<int>(y) & 0xF) << 4)
        | ((static_cast<int>(x) & 0xF)));
#endif
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline void XM_CALLCONV XMStoreU555
(
    XMU555* pDestination,
    FXMVECTOR V
) noexcept
{
    assert(pDestination);
    static const XMVECTORF32 Max = { { { 31.0f, 31.0f, 31.0f, 1.0f } } };

#if defined(_XM_NO_INTRINSICS_)
    XMVECTOR N = XMVectorClamp(V, XMVectorZero(), Max.v);
    N = XMVectorRound(N);

    XMFLOAT4A tmp;
    XMStoreFloat4A(&tmp, N);

    pDestination->v = static_cast<uint16_t>(
        ((tmp.w > 0.f) ? 0x8000 : 0)
        | ((static_cast<int>(tmp.z) & 0x1F) << 10)
        | ((static_cast<int>(tmp.y) & 0x1F) << 5)
        | (static_cast<int>(tmp.x) & 0x1F));
#elif defined(_XM_ARM_NEON_INTRINSICS_)
    static const XMVECTORF32 Scale = { { { 1.0f, 32.f / 2.f, 32.f * 32.f, 32.f * 32.f * 32.f / 2.f } } };
    static const XMVECTORU32 Mask = { { { 0x1F, 0x1F << (5 - 1), 0x1F << 10, 0x1 << (15 - 1) } } };
    float32x4_t vResult = vmaxq_f32(V, vdupq_n_f32(0));
    vResult = vminq_f32(vResult, Max);
    vResult = vmulq_f32(vResult, Scale);
    uint32x4_t vResulti = vcvtq_u32_f32(vResult);
    vResulti = vandq_u32(vResulti, Mask);
    // Do a horizontal or of 4 entries
    uint32x2_t vTemp = vget_low_u32(vResulti);
    uint32x2_t vTemp2 = vget_high_u32(vResulti);
    vTemp = vorr_u32(vTemp, vTemp2);
    // Perform a single bit left shift on y|w
    vTemp2 = vdup_lane_u32(vTemp, 1);
    vTemp2 = vadd_u32(vTemp2, vTemp2);
    vTemp = vorr_u32(vTemp, vTemp2);
    vst1_lane_u16(&pDestination->v, vreinterpret_u16_u32(vTemp), 0);
#elif defined(_XM_SSE_INTRINSICS_)
    // Bounds check
    XMVECTOR vResult = _mm_max_ps(V, g_XMZero);
    vResult = _mm_min_ps(vResult, Max);
    // Convert to int with rounding
    __m128i vInt = _mm_cvtps_epi32(vResult);
    // No SSE operations will write to 16-bit values, so we have to extract them manually
    auto x = static_cast<uint16_t>(_mm_extract_epi16(vInt, 0));
    auto y = static_cast<uint16_t>(_mm_extract_epi16(vInt, 2));
    auto z = static_cast<uint16_t>(_mm_extract_epi16(vInt, 4));
    auto w = static_cast<uint16_t>(_mm_extract_epi16(vInt, 6));
    pDestination->v = static_cast<uint16_t>(
        (static_cast<int>(w) ? 0x8000 : 0)
        | ((static_cast<int>(z) & 0x1F) << 10)
        | ((static_cast<int>(y) & 0x1F) << 5)
        | ((static_cast<int>(x) & 0x1F)));
#endif
}


/****************************************************************************
 *
 * XMCOLOR operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMCOLOR::XMCOLOR
(
    float _r,
    float _g,
    float _b,
    float _a
) noexcept
{
    XMStoreColor(this, XMVectorSet(_r, _g, _b, _a));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMCOLOR::XMCOLOR(const float* pArray) noexcept
{
    XMStoreColor(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMHALF2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMHALF2::XMHALF2
(
    float _x,
    float _y
) noexcept
{
    x = XMConvertFloatToHalf(_x);
    y = XMConvertFloatToHalf(_y);
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMHALF2::XMHALF2(const float* pArray) noexcept
{
    assert(pArray != nullptr);
    x = XMConvertFloatToHalf(pArray[0]);
    y = XMConvertFloatToHalf(pArray[1]);
}

/****************************************************************************
 *
 * XMSHORTN2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMSHORTN2::XMSHORTN2
(
    float _x,
    float _y
) noexcept
{
    XMStoreShortN2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMSHORTN2::XMSHORTN2(const float* pArray) noexcept
{
    XMStoreShortN2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMSHORT2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMSHORT2::XMSHORT2
(
    float _x,
    float _y
) noexcept
{
    XMStoreShort2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMSHORT2::XMSHORT2(const float* pArray) noexcept
{
    XMStoreShort2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMUSHORTN2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUSHORTN2::XMUSHORTN2
(
    float _x,
    float _y
) noexcept
{
    XMStoreUShortN2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUSHORTN2::XMUSHORTN2(const float* pArray) noexcept
{
    XMStoreUShortN2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMUSHORT2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUSHORT2::XMUSHORT2
(
    float _x,
    float _y
) noexcept
{
    XMStoreUShort2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUSHORT2::XMUSHORT2(const float* pArray) noexcept
{
    XMStoreUShort2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMBYTEN2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMBYTEN2::XMBYTEN2
(
    float _x,
    float _y
) noexcept
{
    XMStoreByteN2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMBYTEN2::XMBYTEN2(const float* pArray) noexcept
{
    XMStoreByteN2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMBYTE2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMBYTE2::XMBYTE2
(
    float _x,
    float _y
) noexcept
{
    XMStoreByte2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMBYTE2::XMBYTE2(const float* pArray) noexcept
{
    XMStoreByte2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMUBYTEN2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUBYTEN2::XMUBYTEN2
(
    float _x,
    float _y
) noexcept
{
    XMStoreUByteN2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUBYTEN2::XMUBYTEN2(const float* pArray) noexcept
{
    XMStoreUByteN2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMUBYTE2 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUBYTE2::XMUBYTE2
(
    float _x,
    float _y
) noexcept
{
    XMStoreUByte2(this, XMVectorSet(_x, _y, 0.0f, 0.0f));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUBYTE2::XMUBYTE2(const float* pArray) noexcept
{
    XMStoreUByte2(this, XMLoadFloat2(reinterpret_cast<const XMFLOAT2*>(pArray)));
}

/****************************************************************************
 *
 * XMU565 operators
 *
 ****************************************************************************/

inline XMU565::XMU565
(
    float _x,
    float _y,
    float _z
) noexcept
{
    XMStoreU565(this, XMVectorSet(_x, _y, _z, 0.0f));
}

_Use_decl_annotations_
inline XMU565::XMU565(const float* pArray) noexcept
{
    XMStoreU565(this, XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pArray)));
}

/****************************************************************************
 *
 * XMFLOAT3PK operators
 *
 ****************************************************************************/

inline XMFLOAT3PK::XMFLOAT3PK
(
    float _x,
    float _y,
    float _z
) noexcept
{
    XMStoreFloat3PK(this, XMVectorSet(_x, _y, _z, 0.0f));
}

_Use_decl_annotations_
inline XMFLOAT3PK::XMFLOAT3PK(const float* pArray) noexcept
{
    XMStoreFloat3PK(this, XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pArray)));
}

/****************************************************************************
 *
 * XMFLOAT3SE operators
 *
 ****************************************************************************/

inline XMFLOAT3SE::XMFLOAT3SE
(
    float _x,
    float _y,
    float _z
) noexcept
{
    XMStoreFloat3SE(this, XMVectorSet(_x, _y, _z, 0.0f));
}

_Use_decl_annotations_
inline XMFLOAT3SE::XMFLOAT3SE(const float* pArray) noexcept
{
    XMStoreFloat3SE(this, XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pArray)));
}

/****************************************************************************
 *
 * XMHALF4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMHALF4::XMHALF4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    x = XMConvertFloatToHalf(_x);
    y = XMConvertFloatToHalf(_y);
    z = XMConvertFloatToHalf(_z);
    w = XMConvertFloatToHalf(_w);
}

//------------------------------------------------------------------------------

_Use_decl_annotations_
inline XMHALF4::XMHALF4(const float* pArray) noexcept
{
    XMConvertFloatToHalfStream(&x, sizeof(HALF), pArray, sizeof(float), 4);
}

/****************************************************************************
 *
 * XMSHORTN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMSHORTN4::XMSHORTN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreShortN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMSHORTN4::XMSHORTN4(const float* pArray) noexcept
{
    XMStoreShortN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMSHORT4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMSHORT4::XMSHORT4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreShort4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMSHORT4::XMSHORT4(const float* pArray) noexcept
{
    XMStoreShort4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMUSHORTN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUSHORTN4::XMUSHORTN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUShortN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUSHORTN4::XMUSHORTN4(const float* pArray) noexcept
{
    XMStoreUShortN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMUSHORT4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUSHORT4::XMUSHORT4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUShort4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUSHORT4::XMUSHORT4(const float* pArray) noexcept
{
    XMStoreUShort4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMXDECN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMXDECN4::XMXDECN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreXDecN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMXDECN4::XMXDECN4(const float* pArray) noexcept
{
    XMStoreXDecN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMXDEC4 operators
 *
 ****************************************************************************/
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4996)
 // C4996: ignore deprecation warning
#endif

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
#endif

 //------------------------------------------------------------------------------

inline XMXDEC4::XMXDEC4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreXDec4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMXDEC4::XMXDEC4(const float* pArray) noexcept
{
    XMStoreXDec4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMDECN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMDECN4::XMDECN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreDecN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMDECN4::XMDECN4(const float* pArray) noexcept
{
    XMStoreDecN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMDEC4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMDEC4::XMDEC4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreDec4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMDEC4::XMDEC4(const float* pArray) noexcept
{
    XMStoreDec4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif
#ifdef _MSC_VER
#pragma warning(pop)
#endif

/****************************************************************************
 *
 * XMUDECN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUDECN4::XMUDECN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUDecN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUDECN4::XMUDECN4(const float* pArray) noexcept
{
    XMStoreUDecN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMUDEC4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUDEC4::XMUDEC4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUDec4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUDEC4::XMUDEC4(const float* pArray) noexcept
{
    XMStoreUDec4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMBYTEN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMBYTEN4::XMBYTEN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreByteN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMBYTEN4::XMBYTEN4(const float* pArray) noexcept
{
    XMStoreByteN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMBYTE4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMBYTE4::XMBYTE4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreByte4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMBYTE4::XMBYTE4(const float* pArray) noexcept
{
    XMStoreByte4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMUBYTEN4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUBYTEN4::XMUBYTEN4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUByteN4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUBYTEN4::XMUBYTEN4(const float* pArray) noexcept
{
    XMStoreUByteN4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMUBYTE4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUBYTE4::XMUBYTE4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUByte4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUBYTE4::XMUBYTE4(const float* pArray) noexcept
{
    XMStoreUByte4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMUNIBBLE4 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMUNIBBLE4::XMUNIBBLE4
(
    float _x,
    float _y,
    float _z,
    float _w
) noexcept
{
    XMStoreUNibble4(this, XMVectorSet(_x, _y, _z, _w));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMUNIBBLE4::XMUNIBBLE4(const float* pArray) noexcept
{
    XMStoreUNibble4(this, XMLoadFloat4(reinterpret_cast<const XMFLOAT4*>(pArray)));
}

/****************************************************************************
 *
 * XMU555 operators
 *
 ****************************************************************************/

 //------------------------------------------------------------------------------

inline XMU555::XMU555
(
    float _x,
    float _y,
    float _z,
    bool _w
) noexcept
{
    XMStoreU555(this, XMVectorSet(_x, _y, _z, ((_w) ? 1.0f : 0.0f)));
}

//------------------------------------------------------------------------------
_Use_decl_annotations_
inline XMU555::XMU555
(
    const float* pArray,
    bool _w
) noexcept
{
    XMVECTOR V = XMLoadFloat3(reinterpret_cast<const XMFLOAT3*>(pArray));
    XMStoreU555(this, XMVectorSetW(V, ((_w) ? 1.0f : 0.0f)));
}

